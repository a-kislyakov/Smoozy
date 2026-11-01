# Reports — cross-sweep questions, second pass (Blender 4.5)

Questions answered: 7, 8 and 9 from the Reports question sheet.

**Sources consulted, and the date.** Blender's 4.5 manual (`Blender Sources/manual_4.5`, including its `_images` folder), Blender's 4.5 Python API documentation (`Blender Sources/api_4.5`), Blender's 4.5 release notes (`Blender Sources/release_notes_4.5`), and Cycles' own files as installed with Blender 4.5 (`Blender Sources/cycles_4.5`). All four were taken on **2026-07-30**, and that is the date read for every finding below.

I read only the question sheet and files inside Blender Sources/. I did not open any other part of this project.

I worked alone. I started no helper agents, so there is nobody else's reading to vouch for.

---

## How I searched

Because two of the three answers below come out as "not in the sources", here is the shape of the search first, so the record is in one place rather than scattered.

I searched all four folders whole, by text, before opening anything. The terms I searched for, across `manual_4.5`, `api_4.5`, `release_notes_4.5` and `cycles_4.5`:

*Progress words:* `progress`, `progress bar`, `percent`, `percentage of render`, `percent complete`, `status printing`, `render statistics`, `render stats`, `stats`, `estimate`, `estimated`, `time estimate`, `time remaining`, `ETA`, `elapsed`, `remaining`.

*Printing words:* `print`, `prints`, `printed`, `stdout`, `standard output`, `stderr`, `console`, `console window`, `System Console`, `terminal`, `in the console`, `shown in the console`, `written to the console`, `see console`, `log`, `verbose`, `quiet`, `debug`.

*Literal line fragments a render is commonly said to write:* `Fra:`, `Mem:`, `Peak:`, `Time:`, `Saved:`, `Append frame`, `Blender quit`, `Rendering [0-9]`, `Sample [0-9]`, `Path Tracing`.

*Size words:* `resolution_x`, `resolution_y`, `resolution_percentage`, `render resolution`, `image size`, `picture size`, `frame size`, `pixel count`, `megapixel`, `maximum resolution`, `max resolution`, `too large`, `high resolution`, `large resolution`, `arbitrary dimensions`, `16384`, `32767`, `65536`, `tile`, `downscale`.

*Ways-of-starting words:* `background mode`, `background-mode`, `bpy.app.background`, `command line`, `Render Image`, `Render Menu`, `Render Animation`, `viewport render`, `Control-C`, `cancel a render`, `abort the render`, `abort it with Esc`, `stop the render`.

Pages I opened and read in full or in the relevant part:

- Manual: `interface/window_system/status_bar`, `interface/window_system/topbar`, `advanced/command_line/index`, `advanced/command_line/arguments`, `advanced/command_line/render`, `advanced/command_line/launch/linux`, `.../windows`, `.../macos`, `advanced/limits`, `advanced/scripting/security`, `troubleshooting/startup`, `render/introduction`, `render/output/introduction`, `render/output/animation`, `render/output/properties/format`, `render/output/properties/output`, `render/output/properties/metadata`, `render/cycles/render_settings/performance`, `render/cycles/render_settings/sampling`, `editors/3dview/viewport_render`, `editors/image/navigating`, `editors/image/introduction`, `editors/info_editor`, `editors/preferences/interface`.
- Manual pictures: `_images/interface_window-system_status-bar_ui-center.png`, `_images/advanced_command-line_launch_windows_console.png`, `_images/advanced_command-line_launch_linux_console.png`, `_images/advanced_command-line_launch_macos_mac.png`, `_images/editors_image_introduction_main.png`.
- API: `bpy.types.RenderEngine`, `bpy.types.HydraRenderEngine`, `bpy.types.RenderSettings`, `bpy.types.RenderResult`, `bpy.types.WindowManager`, `bpy.ops.render`, `bpy.app.handlers`, `bpy.app`, `info_advanced_blender_as_bpy`, `info_tips_and_tricks`.
- Release notes 4.5: `index`, `core`, `cycles`, `rendering`, `python_api`, `user_interface`, plus a text search of all nineteen files.
- Cycles' own files: `__init__.py`, `engine.py`, `operators.py`, `properties.py` (searched whole), `ui.py` (searched whole).

What I ruled out and why:

- **The whole `render/eevee`, `render/workbench`, `render/freestyle` and `render/shader_nodes` trees**, beyond the text searches above. The searches for every progress and printing word returned nothing in them; they describe settings, not what is reported while working.
- **The compositing and video-editing trees**, same reason, with one exception I did open: the status-bar picture happens to show a *Sequence Render* task, so I checked the sequencer release notes too. Nothing on progress there.
- **The 4.5 release notes**, all nineteen files. Searching them for `progress`, `console`, `stdout`, `quiet`, `render_stats` and `handler` returned only unrelated hits (a status-bar drawing fix, a "Saved" report-message wording change, a `--profile-gpu` argument for GPU profiling). Nothing changes render progress reporting in 4.5.
- **Cycles' `shader/` and `source/` folders** (the `.oso` shader objects and the kernel/util headers). These are shading and kernel code, not the session or reporting layer; the reporting-related names I searched for do not appear in the Python files that would expose them, and the folder holds no session, progress or output source file to read. The only Cycles reporting hook that is present is in `engine.py`, quoted below.

---

## 7. What does a render report about its own progress, across the whole range of picture sizes a render will accept?

**Ambiguity, flagged.** I read "picture sizes a render will accept" as the range of output resolutions Blender allows for a render, and the question as asking whether progress reporting changes anywhere across that range. The other reading would be the range of *file* dimensions the chosen output format will accept, which is a different and narrower thing; the sources say almost nothing about that either, beyond one warning quoted below.

**LABEL: mixed.** The accepted range of picture sizes is **documented**. The general form of progress reporting is **documented**. Any connection between the two is **not in the sources**.

### The range of picture sizes

> `resolution_x`
> Number of horizontal pixels in the rendered image
> Type:
> int in [4, 65536], default 1920
>
> `resolution_y`
> Number of vertical pixels in the rendered image
> Type:
> int in [4, 65536], default 1080

> `resolution_percentage`
> Percentage scale for render resolution
> Type:
> int in [1, 32767], default 100

Source: Python API documentation, https://docs.blender.org/api/4.5/bpy.types.RenderSettings.html — read 2026-07-30.

The manual's own description of the same two settings:

> Resolution X, Y
> The number of pixels horizontally and vertically in the image.
>
> Percentage
> Slider to reduce or increase the size of the rendered image relative to the Resolution values.
> This is useful for small test renders that have the same proportions as the final image.

Source: manual, https://docs.blender.org/manual/en/4.5/render/output/properties/format.html — read 2026-07-30.

The manual's "Working Limits" page lists limits for space, time and text fields, and does **not** list any limit on picture size. Source: manual, https://docs.blender.org/manual/en/4.5/advanced/limits.html — read 2026-07-30.

### What the sources say a render reports about progress

Three statements, and one picture.

> Running Task
> Shows the progress of the currently running task (such as rendering or baking).
> Hovering the mouse pointer over the progress bar will display a time estimate.
> The task can be aborted by clicking the cancel button ().

Source: manual, https://docs.blender.org/manual/en/4.5/interface/window_system/status_bar.html — read 2026-07-30.

> `update_progress(progress)`
> Update progress percentage of render
> Parameters:
> progress (float in [0, 1]) – Percentage of render that's done

Source: Python API documentation, https://docs.blender.org/api/4.5/bpy.types.RenderEngine.html — read 2026-07-30.

> `update_stats(stats, info)`
> Update and signal to redraw render status text
> Parameters:
> stats (string, (never None)) – Stats
> info (string, (never None)) – Info

Same page, same date. The API defines the two strings only as "Stats" and "Info"; it never says what a render engine puts in them.

**The picture.** `[image: _images/interface_window-system_status-bar_ui-center.png]`, sitting under "Status Messages" on the status bar page above. What is visible in it: a small film-camera icon, the words **Sequence Render**, a horizontal bar filled a little over nine tenths of the way in blue, the text **94%** near the right end of that bar, and an **X** button to its right. Label for that reading: **documented** (it is part of the manual). Anything about what other tasks put in that label, or how the number is arrived at, is not visible and I do not claim it.

### Whether any of this varies with picture size

Nothing in any of the four sources connects progress reporting to the size of the picture. Not the status bar page, not `update_progress`, not `update_stats`, not `WindowManager.progress_begin` / `progress_update` / `progress_end` (https://docs.blender.org/api/4.5/bpy.types.WindowManager.html, read 2026-07-30), not the 4.5 release notes. The size words I searched for (listed at the top) return settings pages and unrelated hits only. So for the question as asked: **not in the sources**.

The one thing the sources do say changes with picture size is the *method*, not the report:

> Use Tiling
> Render high resolution images in tiles to reduce memory usage.
> Tiles are cached to disk while rendering to save memory.
>
> Tile Size
> This value is used to control the size of the tile used for rendering;
> decreasing the size reduces memory usage.

Source: manual, https://docs.blender.org/manual/en/4.5/render/cycles/render_settings/performance.html — read 2026-07-30.

And from Cycles' own files, the same two settings with their kind, default and range — **observed**:

> ```
>     use_auto_tile: BoolProperty(
>         name="Use Tiling",
>         description="Render high resolution images in tiles to reduce memory usage, using the specified tile size. Tiles are cached to disk while rendering to save memory",
>         default=True,
>     )
>     tile_size: IntProperty(
>         name="Tile Size",
>         default=2048,
>         description="",
>         min=8, max=8192,
>     )
> ```

File: `Blender Sources/cycles_4.5/properties.py`, lines 1051–1061 — read 2026-07-30. So: tiling is on by default, the tile is 2048 pixels by default, and the tile size may be set anywhere from 8 to 8192. That is what the file says. What a render *does* with a picture smaller than one tile, or whether the tiling changes what is reported, is a step past the file and I did not take it.

One warning that bears on the other reading of the question:

> Like containers and codecs are sometimes not compatible with each other, some codecs
> do not work with arbitrary dimensions. So, try to stick with common dimensions
> or research the limitations of the codec you are trying to use.

Source: manual, https://docs.blender.org/manual/en/4.5/render/output/properties/output.html — read 2026-07-30. This is about video codecs refusing certain dimensions, not about progress.

### Observed — what a running Blender 4.5 does (2026-08-11)

Run in Blender 4.5.12 on **the Creator's Mac (a MacBook Pro with an Apple M1 Pro chip, 16 GB of memory, macOS 26.5.2, Apple's own graphics and no second card)** and on **a rented Linux machine (Ubuntu 22.04.5 LTS on kernel 6.8, two NVIDIA RTX 4000 Ada Generation cards of 20 GB each on driver 580.105.08, eleven cores, 49 GB of memory)**, each with a settings folder of its own so nothing touched the Creator's own Blender settings. The scene was built by Blender 4.5 itself: a cube, a sphere, an area light and a camera; Cycles on the processor; 16 samples; adaptive sampling off; output PNG. **LABEL: observed.**

**The progress report does change with picture size, at one tile's worth of area.** The same frame was rendered from a terminal with no window at six sizes, changing only the two resolution numbers. At 4 by 4, 160 by 120, 1920 by 1080 and 2048 by 2048 the capture holds **three** lines carrying a sample count and mentions no tile. At 2049 by 2049 and 4096 by 4096 it holds **thirteen**, all of them mentioning tiles, and the stage at the end of the line becomes `Rendered <n>/<total> Tiles, Sample <s>/<max>`. Two of the thirteen from the 2049 by 2049 run on the Creator's Mac:

```
Fra:1 Mem:248.42M (Peak 248.42M) | Time:00:00.18 | Remaining:00:02.66 | Mem:240.19M, Peak:240.19M | Scene, ViewLayer | Rendered 0/4 Tiles, Sample 1/16
Fra:1 Mem:8.54M (Peak 248.42M) | Time:00:03.25 | Mem:0.19M, Peak:240.19M | Scene, ViewLayer | Rendered 3/4 Tiles, Sample 16/16
```

All six runs exited 0 and wrote a picture of the size asked for. **The rented Linux machine gave the same split at the same sizes, and the same tile counts. The two machines agreed.**

**The settings clamp silently and never refuse.** Set from Python and read back: 0, 3 and 4 all become **4** for the two size settings; 65536, 65537 and 70000 all become **65536**; for the percentage 0 becomes **1** and 32768 and 70000 become **32767**. No error, nothing printed, both machines.

**The percentage cuts down rather than rounds.** At 1920 by 1080 the written picture is **1920 x 1080** at 100 per cent, **960 x 540** at 50 per cent and **633 x 356** at 33 per cent — 633 is the product cut down to a whole number, where rounding would give 634. Both machines. **A picture of no pixels is refused and the run still exits 0**: 4 by 4 at 1 per cent wrote nothing, and the whole capture on the Creator's Mac reads

```
Blender 4.5.12 LTS (hash 84afd5f785f7 built 2026-07-21 01:41:35)
Read blend: "…/scenes/base_4.5.blend"
Error: Image too small

Blender quit
```

**A render at the top of the range starts, tiles, and then fails to save.** At 65536 by 65536 with one sample per pixel, on the rented Linux machine only, the render divided into **1024 tiles** and printed a line for each — 1026 tile lines — reaching `Rendered 1023/1024 Tiles, Sample 0/1`, with the machine's memory use at about 3.5 GB of 49 and Blender's own figure at `Mem:248.54M`. It then stopped at the point of putting the tiles together, printing `Error: Error reading tiles from file` on standard output and, on standard error:

```
OpenImageIO exited with a pending error message that was never
retrieved via OIIO::geterror(). This was the error message:
Uncompressed image size 245760.0 MB exceeds the 81920 MB limit.
Image claimed to be 65536x65536, 15-channel float. Possible corrupt input?
If this is a valid file, raise the OIIO attribute "limits:imagesize_MB".
```

The run **exited 1 and wrote no file**, and all four versions did the same. So a size the settings accept is not necessarily a size a render can finish. It was not run on the Creator's Mac, whose 16 GB is not to be filled on purpose.

**What the status bar shows during a render, photographed with Blender's own screenshot command.** Exactly **one** bar, never two. Beside it: a camera icon, the words **`Render...`**, the bar with a **whole-number percentage** inside it, and an **X** at the right end. No decimal point in any photograph. Readings mid-render: `Render...` beside `17%` on the Creator's Mac, `Render...` beside `26%` on the rented Linux machine. **No frame is named**, for a still or an animation, and during a three-frame animation the percentage restarted at each frame rather than climbing across the range. Picture size made no difference — the same one bar and whole-number percentage at 4 by 4 and at 4096 by 4096. **A sequencer render drives the same bar and is labelled `Rendering sequence...`**, not `Sequence Render` as in the manual's picture. Both machines agreed.

**The two tiling settings, read from the running program.** `use_auto_tile` is named **Use Tiling**, described "Render high resolution images in tiles to reduce memory usage, using the specified tile size. Tiles are cached to disk while rendering to save memory", default True. `tile_size` has **no description at all**, default 2048, range `[8, 8192]`. Searching Blender 4.5's own installed Cycles add-on for the words "May work poorly with render tiling" found nothing. Both machines.

**Not covered.** This does not settle what the time estimate says when the pointer rests on the bar, which needs a hover. It does not settle whether a 65536 by 65536 render would save into a format OpenImageIO's size limit allows, or with that limit raised. It does not settle anything about Windows.

**Outside the interface.** Settled, and it differs. With no window there is no status bar and no percentage; what a render reports instead is the printed lines quoted above, which carry a sample count, a memory figure and a `Remaining:` time estimate. The inferred step in the paragraph above — that the progress bar is absent when there is no window — is borne out: no bar exists to photograph, and the terminal carries the lines instead. The size range is the same either way, the same clamping figures coming back from a windowless run. Both machines agreed.

**Operating system.** Settled for macOS and for Linux, and they agreed on every point above. **Windows is untested** — no Windows machine has run it. **Untried rather than untestable: a Windows machine can be rented, and every one this project has read reported a Server edition — see the equipment section of `Preparation/Blender Research/Method.md`.**

---

## 8. What does a render print over the whole of a run, from its first line to its last?

**LABEL: not in the sources.** No page and no file in the four sources gives the lines a render prints, in order, from first to last. What the sources do give is the frame around them — that there is printed output, roughly what kinds of thing are in it, how to turn it up and down, and a handful of named lines from *startup* rather than from a render. Those parts are documented and are set out below, with their labels.

I searched for the literal fragments a render's output is commonly said to contain — `Fra:`, `Mem:`, `Peak:`, `Time:`, `Saved:`, `Append frame`, `Blender quit`, `Rendering <number>`, `Sample <number>` — across the manual, the API documentation, the release notes and Cycles' files. Every one returned nothing, except `Time:` (only in unrelated pages: a follow-path constraint, a mesh cache modifier, custom properties, video formats, fluid noise settings) and `Blender quit`, which appears nowhere in the text but does appear in a picture, quoted below.

### What the sources do establish

**There is printed output, and it goes to a console.**

> The Console Window, also called a Terminal, is an operating system text window that displays
> messages about Blender's operations, status, and internal errors.
>
> When Blender is manually started from a terminal,
> Blender output is shown in the Console Window it is started from.

Source: manual, https://docs.blender.org/manual/en/4.5/advanced/command_line/index.html — read 2026-07-30.

> The Blender Console Window can display many different types of status and error messages. Some messages simply
> inform the user what Blender is doing, but have no real impact on Blender's ability to function. Other messages can
> indicate serious errors that will most likely prevent Blender carrying out a particular task and may even make Blender
> non-responsive or shut down completely. The Blender Console Window messages can also originate internally from
> within the Blender code or from external sources such as Python scripts.

Source: manual, https://docs.blender.org/manual/en/4.5/troubleshooting/startup.html — read 2026-07-30.

**There is a stream of "status printing" that can be switched off.**

> `-q, --quiet`
> Suppress status printing (warnings & errors are still printed).

Source: manual, https://docs.blender.org/manual/en/4.5/advanced/command_line/arguments.html — read 2026-07-30. So the sources confirm a run prints three separable things: status, warnings, and errors; and that the first can be suppressed while the other two cannot.

**Render statistics are printed, and their content is named.** This is the closest the sources come to saying what a render prints:

> `bpy.app.handlers.render_stats`
> on printing render statistics. Accepts one argument: the render stats (render/saving time plus in background mode frame/used [peak] memory).

Source: Python API documentation, https://docs.blender.org/api/4.5/bpy.app.handlers.html — read 2026-07-30. Read plainly: something is printed called the render statistics; it contains render time and saving time; and when Blender is in background mode it also contains the frame and the used and peak memory. It does not give the wording, the order, or where in a run this lands.

**Cycles can be asked to print more, and where it goes is named.** From the manual:

> `--cycles-print-stats`
> Log statistics about render memory and time usage.

Source: manual, https://docs.blender.org/manual/en/4.5/advanced/command_line/arguments.html — read 2026-07-30.

And from Cycles' own file, which says which stream it goes to — **observed**:

> ```
>     parser.add_argument("--cycles-print-stats",
>                         help="Print rendering statistics to stderr",
>                         action='store_true')
> ```

File: `Blender Sources/cycles_4.5/engine.py`, lines 13–15 (inside `_configure_argument_parser`) — read 2026-07-30. The file says the argument's own help text is "Print rendering statistics to stderr". The switch is wired straight through:

> ```
>     if args.cycles_print_stats:
>         import _cycles
>         _cycles.enable_print_stats()
> ```

Same file, lines 34–36 — **observed**. What `_cycles.enable_print_stats()` then emits is inside a compiled part I was not given, so I stop there.

**More output appears with `-d`, at least while encoding video.**

> Tip
> When you view the System Console,
> you can see some of the output of the encoding process.
> You will see even more output if you execute Blender as `blender -d`.

Source: manual, https://docs.blender.org/manual/en/4.5/render/output/properties/output.html — read 2026-07-30. (`-d, --debug` — "Turn debugging on" — from the arguments page, same date.)

**The amount and destination of logging is adjustable.** From the arguments page (same address and date), the Logging Options: `--log <match>` ("Enable logging categories, taking a single comma separated argument… Use "*" to log everything"), `--log-level <level>` ("Set the logging verbosity level (higher for more details) defaults to 1, use -1 to log all levels"), `--log-show-basename`, `--log-show-backtrace` ("debug builds only"), `--log-show-timestamp` ("Show a timestamp for each log message in seconds since start"), and `--log-file <filepath>` ("Set a file to output the log to"). Also `--verbose <verbose>` — "Set the logging verbosity level for debug messages that support it." None of these names a render category.

### The only named lines, and they are from startup

Two lines are documented by their exact text, and both are startup lines, not render lines:

> `found bundled python: {DIR}`
> This message indicates that Blender was able to find the Python library for the Python
> interpreter embedded within Blender. If this folder is missing or unable to be found, it is likely that an error
> will occur, and this message will not appear.
>
> `Read prefs: {DIR}/userpref.blend`
> The preferences use this path.

Source: manual, https://docs.blender.org/manual/en/4.5/troubleshooting/startup.html — read 2026-07-30.

### The pictures — a whole run, first line to last, but not a render

The manual's command-line launch pages carry screenshots of a console session. They are manual content, so what is visible in them is **documented**.

`[image: _images/advanced_command-line_launch_windows_console.png]`, captioned "Blender's Console Window on Microsoft Windows", on https://docs.blender.org/manual/en/4.5/advanced/command_line/launch/windows.html. What is visible, in order, from the first line to the last:

```
c:\blender>blender
Read new prefs: C:\Users\Aaron\AppData\Roaming\Blender Foundation\Blender\2.77\config\userpref.blend
found bundled python: c:\blender\2.77\python
read blend: C:\tmp\Vincent.blend

Blender quit
c:\blender>
```

The page's own words about it: "The screenshot shows the Blender Console Window on Microsoft Windows directly after starting Blender and then a short while later after opening a file along with the relevant messages."

`[image: _images/advanced_command-line_launch_macos_mac.png]`, captioned "Starting Blender from a macOS console window", on https://docs.blender.org/manual/en/4.5/advanced/command_line/launch/macos.html. Visible:

```
hostname:~ user$ /Applications/blender-2.64/blender.app/Contents/MacOS/blender
ndof: 3Dx driver not found
found bundled python: /Applications/blender-2.64/blender.app/Contents/MacOS/2.64/python
```

`[image: _images/advanced_command-line_launch_linux_console.png]`, captioned "Starting Blender from a Linux terminal window", on https://docs.blender.org/manual/en/4.5/advanced/command_line/launch/linux.html. Visible:

```
% blender
Cannot connect to server socket err = No such file or directory
Cannot connect to server request channel
jack server is not running or cannot be started
JackShmReadWritePtr::~JackShmReadWritePtr - Init not done for 4294967295, skipping unlock
JackShmReadWritePtr::~JackShmReadWritePtr - Init not done for 4294967295, skipping unlock
AL lib: (WW) ALCjackBackendFactory_init: jack_client_open() failed, 0x11
AL lib: (WW) alc_initconfig: Failed to initialize backend "jack"
```

All read 2026-07-30. Two cautions, and they matter. First, **none of these three sessions renders anything** — they start Blender, in one case open a file, and in one case quit. So they show the shape of a session's output, not a render's. Second, the paths inside two of them name **2.77** and **2.64**, so the pictures were taken with much older Blender and carried forward into the 4.5 manual; I report what is visible in them and claim nothing about whether 4.5 prints the same words.

### Observed — the whole of a Blender 4.5 render run (2026-08-11)

Run in Blender 4.5.12 on **the Creator's Mac (a MacBook Pro with an Apple M1 Pro chip, 16 GB of memory, macOS 26.5.2)** and on **a rented Linux machine (Ubuntu 22.04.5 LTS, two NVIDIA RTX 4000 Ada Generation cards of 20 GB each, eleven cores, 49 GB of memory)**, each with a settings folder of its own. The scene: a cube, a sphere, an area light and a camera; Cycles on the processor; 16 samples; adaptive sampling off; 160 by 120; output PNG. **LABEL: observed.**

**The first line and the last line, which the sources do not state.** `blender -b <file>.blend -o <out> -f 1 > out.txt 2> err.txt` exited 0, wrote its picture, and put **nothing at all on standard error**. Standard output held fifty lines. The first three, all three sample lines, and the last four, word for word, from the Creator's Mac:

```
Blender 4.5.12 LTS (hash 84afd5f785f7 built 2026-07-21 01:41:35)
Read blend: "…/scenes/base_4.5.blend"
Fra:1 Mem:8.30M (Peak 9.10M) | Time:00:00.03 | Mem:0.00M, Peak:0.00M | Scene, ViewLayer | Synchronizing object | Cube
…
Fra:1 Mem:8.34M (Peak 9.10M) | Time:00:00.03 | Mem:0.19M, Peak:0.19M | Scene, ViewLayer | Sample 0/16
Fra:1 Mem:9.45M (Peak 9.45M) | Time:00:00.04 | Remaining:00:00.05 | Mem:1.21M, Peak:1.21M | Scene, ViewLayer | Sample 1/16
Fra:1 Mem:10.04M (Peak 10.62M) | Time:00:00.21 | Mem:1.21M, Peak:1.21M | Scene, ViewLayer | Sample 16/16
Fra:1 Mem:10.04M (Peak 10.62M) | Time:00:00.21 | Mem:1.21M, Peak:1.21M | Scene, ViewLayer | Finished
Saved: '…/out/e1_4.5_0001.png'
Time: 00:00.27 (Saving: 00:00.05)


Blender quit
```

So **the first line is the version banner and the last is `Blender quit`, with two blank lines before it.** The thirty-eight lines elided all share that shape, differing only in the stage named at the end. `Fra:` in this version carries no space; the line holds two memory pairs, a `Time:` field and the scene and view-layer names. Only the `Sample 1/16` line carries a time estimate, called `Remaining`. **Exactly three lines carry a sample count for a sixteen-sample frame** — never one per sample. The rented Linux machine gave the same fifty lines in the same order, with nothing on standard error. **The two machines agreed.**

**What "status printing" turns out to be in this version: all of it.** `-q` left one line on standard output — the version banner — and nothing else, on both machines. The picture was still written and the run exited 0.

**Which stream Cycles' statistics go to: standard output, not standard error.** With `-- --cycles-print-stats`, standard error was **empty** on both machines. The statistics arrive in two blocks interleaved among the render lines, not at the start or the end: `Update statistics:` during the Updating stages, `Render statistics:` after the last sample line and before `Saved:`. The second block opens

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
```

and runs on through the kernel, shader and object figures. **No sample count appears anywhere in either block.** Without the bare `--` the switch is taken as a file name and the run **exits 1** with `Error: Cannot read file "…/--cycles-print-stats": No such file or directory`.

**An animation prints no announcement at all in this version.** A three-frame run exited 0, wrote three pictures, and its capture holds **no line beginning `Rendering`**. Each frame repeats the whole single-frame block, the frame showing only inside the render lines as `Fra:1`, `Fra:2`, `Fra:3`, each ending with its own `Saved:` and `Time:` pair. Both machines.

**The render statistics are handed over per stage, not once per frame.** A handler attached to `bpy.app.handlers.render_stats` was called **forty-three times** for one frame with no window, each time with a string and `None`:

```
STATS ('Fra:1 Mem:8.31M (Peak 9.10M) | Time:00:00.00 | Mem:0.00M, Peak:0.00M | Scene, ViewLayer | Synchronizing object | Area', None)
STATS ('Fra:1 Mem:9.45M (Peak 9.45M) | Time:00:00.00 | Remaining:00:00.01 | Mem:1.21M, Peak:1.21M | Scene, ViewLayer | Sample 1/16', None)
STATS ('Time: 00:00.14 (Saving: 00:00.04)', None)
```

**With a window open it fired once**, carrying only `STATS ('Time: 00:00.13 (Saving: 00:00.01)', None)`. Both machines.

**The other two engines.** EEVEE — called `BLENDER_EEVEE_NEXT` in this version — prints `Rendering <n> / <total> samples` with a memory figure and a time, four such lines for a 64-sample frame. Workbench prints **no progress line at all**, going straight from the file-reading line to `Saved:`. Both machines.

**What a saved image records rather than prints.** Read back by a plain Python script rather than by Blender, a PNG carries eleven fields — `File`, `Date`, `Time`, `Frame`, `Camera`, `Scene`, `RenderTime`, `cycles.ViewLayer.samples`, `cycles.ViewLayer.total_time`, `cycles.ViewLayer.render_time`, `cycles.ViewLayer.synchronization_time` — and the sample figure among them is the maximum asked for, not the number reached. Both machines.

**Observed again — the whole statistics block, and where it lands in a run (2026-08-12).** Two later hand tests repeated this capture in all four versions on both machines: one for the whole of the block, one for its position. Run on **the Creator's Mac (a MacBook Pro with an Apple M1 Pro chip, 16 GB of memory, macOS 26.5.2, Apple's own graphics and no second card)** and on **a rented Linux machine (Ubuntu 22.04.5 LTS on kernel 6.8, two NVIDIA RTX 4000 Ada Generation cards of 20 GB each on driver 580.105.08, an AMD Ryzen Threadripper PRO 3945WX with eleven cores, 49 GB of memory)**, each version with a settings folder of its own and its own copy of the scene. The command was `blender -b <file>.blend -o <out> -f 1 -- --cycles-print-stats`, standard output and standard error kept apart. Every run exited 0. **LABEL: observed.**

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

**The rented Linux machine printed the same ten lines above `Kernel statistics:`, figure for figure**; its own kernel line read `  Total render time               : Total 100.00% (0.24s), Self 6.20% (0.01s)`. **The two machines agreed on the byte counts and differed on the times.** The times and percentages below that line changed from one run to the next on the same machine, so they are not set down as fixed. Standard error was empty on both machines.

**These figures belong to this version.** Run the same way on the same day, Blender 5.0 and Blender 5.1 both printed `Total memory: 24.04K (24,615)`, `Sphere 23.55K (24,117)`, `Cube 417 (417)`, `Area 81 (81)`; Blender 5.2 printed `Total memory: 16.36K (16,750)`, `Sphere 16.02K (16,405)`, `Cube 289B (289)`, `Area 56B (56)`, with its image section written as `Image statistics:` then `  Total image memory: 0B (0)`.

**Where the block lands, counted by line number.** The lines of a second run were numbered. On the Creator's Mac standard output held 126 lines: `Update statistics:` at 10, `Sample 16/16` at 76, `Finished` at 77, `Render statistics:` at 78, `Saved:` at 122, `Time:` at 123. On the rented Linux machine it held 127 lines in the same order: 11, 77, 78, 79, 123 and 124. **The two machines agreed, and both bear out what this document says: in Blender 4.5 the block sits after the last sample line and before `Saved:`.** In 5.0, 5.1 and 5.2 it came last of all on both machines, after `Saved:` and `Time:`.

**Not covered.** This answer does not settle what a run prints when it fails rather than succeeds, which is a different question on the sheet. It does not settle what is printed when the output is a video file. It does not settle anything about Windows.

**Outside the interface.** Printed output *is* the outside-the-interface channel, and every control over it named above is a command-line argument, so all of it applies when Blender is driven from outside its window: `-q`, `-d`, `--log`, `--log-level`, `--log-file`, `--log-show-timestamp`, `--verbose`, `--cycles-print-stats`. Two documented differences. First, the render-statistics content differs by mode — "render/saving time plus **in background mode** frame/used [peak] memory" (API, `bpy.app.handlers`, read 2026-07-30). Second, when Blender is used as a Python module rather than as a program, those arguments are gone: "Functionality controlled by command line arguments (shown by calling `blender --help` aren't accessible). Typically this isn't such a limitation although there are some command line arguments that don't have equivalents in Blender's Python API (`--threads` and `--log` for example)." (API, https://docs.blender.org/api/4.5/info_advanced_blender_as_bpy.html, read 2026-07-30.)

**Operating system.** Settled for macOS and for Linux by the tests above, and the printed content agreed line for line: the same fifty lines in the same order, the same first and last line, the same three sample lines, the same one-line quiet mode, the same statistics on standard output with an empty standard error, the same animation with no announcement, and the same handler strings. **One difference between the two machines**: given `--log-list-categories`, which this version does not have, without `-b`, Blender 4.5 on the Linux machine tried to open a window, printed `GHOST: unable to initialize, exiting!` and did not exit, where on the Mac it exited 1 straight away. **Checked again on 2026-08-12 for Cycles' statistics**: the block's byte counts and its place in the run came back the same on both machines, and only the times inside it differed.

**Windows is untested** — no Windows machine has run it — so the manual's Windows behaviour stands unchecked: "When Blender is started on a Microsoft Windows operating system, the Console Window (called the Command Prompt) is first created as a separate window on the desktop. The main Blender window will also appear and the Console Window will then be toggled off. To display the console again, go to Window ‣ Toggle System Console." Also `-con, --start-console` — "Start with the console window open (ignored if `-b` is set), (Windows only)." The macOS line the manual gives is how every macOS run above was made: "In order to run Blender you will have to specify that path to the Blender executable inside this folder, to get all output printed to the terminal." Sources: the three launch pages cited above and the arguments page, all read 2026-07-30.

---

## 9. What does a render report about its progress under each of the ways a render can be started?

**LABEL: not in the sources**, for the question as asked. The sources name several ways to start a render, but they describe progress reporting for only one of them (a render started inside the interface), and give exactly one documented difference for a render started outside it. No page sets out, way by way, what is reported.

Here is what each way is, and what the sources do and do not say about its progress.

### The ways the sources name

**From the Render menu.**

> Render Image F12
> Render the active scene at the current frame.
>
> Render Animation Ctrl-F12
> Render the animation of the active scene.
>
> …
>
> View Render F11
> Show the Render window. (Press again to switch back to the main Blender window.)

Source: manual, https://docs.blender.org/manual/en/4.5/interface/window_system/topbar.html — read 2026-07-30.

**From the 3D Viewport, as a viewport render.** "To render a still image, use 3D Viewport ‣ View ‣ Viewport Render Image." / "To render an animation, use 3D Viewport ‣ View ‣ Viewport Render Animation." / "Viewport Render Keyframes". With the note: "Viewport rendering only works for the Workbench and EEVEE render engines. It's not supported for Cycles." Source: manual, https://docs.blender.org/manual/en/4.5/editors/3dview/viewport_render.html — read 2026-07-30.

**From Python.**

> `bpy.ops.render.render(*, animation=False, write_still=False, use_viewport=False, layer='', scene='')`
> Render active scene

and

> `bpy.ops.render.opengl(*, animation=False, render_keyed_only=False, sequencer=False, write_still=False, view_context=True)`
> Take a snapshot of the active viewport

Source: Python API documentation, https://docs.blender.org/api/4.5/bpy.ops.render.html — read 2026-07-30.

**From the command line.** `blender -b file.blend -f 10` for a single image and `blender -b file.blend -a` for an animation, with `-b` documented as "Render in the background (without UI)". Source: manual, https://docs.blender.org/manual/en/4.5/advanced/command_line/render.html — read 2026-07-30. The same page warns "Arguments are executed in the order they are given!" and "Always position `-f` or `-a` as the last arguments."

**With Blender imported as a Python module.** "Blender supports being built as a Python module, allowing `import bpy` to be added to any Python script"; "For the most part using Blender as a Python module is equivalent to running a script in background-mode (passing the command-line arguments `--background` or `-b`), however there are some differences to be aware of." Source: Python API documentation, https://docs.blender.org/api/4.5/info_advanced_blender_as_bpy.html — read 2026-07-30.

**And a render started in the interface can be sent to four different places.**

> Render In
> When rendering, the user interface can do any of:
> Keep User Interface:
> The user interface does not change and the render is computed in the background.
> Maximize Area:
> A new Image editor is opened as a temporary window in full screen mode.
> Image Editor:
> The area that is the largest on screen is replaced placed by a temporary Image editor.
> New Window:
> A new Image editor is opened as a regularly sized temporary window.

Source: manual, https://docs.blender.org/manual/en/4.5/editors/preferences/interface.html — read 2026-07-30. This is a choice about where the picture appears; the page says nothing about progress.

### What is reported, per way

**Started in the interface, any of the above.** The status bar's Running Task: "Shows the progress of the currently running task (such as rendering or baking). Hovering the mouse pointer over the progress bar will display a time estimate. The task can be aborted by clicking the cancel button ()." (manual, https://docs.blender.org/manual/en/4.5/interface/window_system/status_bar.html, read 2026-07-30), and the picture beside it showing a labelled task, a filled bar, a percentage figure and a cancel button — described in full under question 7 above. The engine feeds this through `update_progress` ("Percentage of render that's done", a float 0 to 1), `update_stats` ("Update and signal to redraw render status text") and `update_memory_stats` ("Update memory usage statistics", with `memory_used` and `memory_peak` "in megabytes") — API, https://docs.blender.org/api/4.5/bpy.types.RenderEngine.html, read 2026-07-30. **Documented**, but note the manual never says the status bar is what a *render* fills in; the picture shows a *Sequence Render* task.

For a viewport render the manual adds only how to stop it: "As with a normal render, you can abort it with Esc." (manual, viewport render page, read 2026-07-30).

**Started from the command line, in background.** One documented difference, and only one:

> `bpy.app.handlers.render_stats`
> on printing render statistics. Accepts one argument: the render stats (render/saving time plus in background mode frame/used [peak] memory).

Source: Python API documentation, https://docs.blender.org/api/4.5/bpy.app.handlers.html — read 2026-07-30. Read plainly: the same statistics are printed either way, and *in background mode they carry three extra things* — the frame, the memory used, and the peak memory. That is the whole of what the sources say distinguishes the two.

Alongside it, `-q, --quiet` — "Suppress status printing (warnings & errors are still printed)" (manual, arguments page, read 2026-07-30) — so the status stream of a command-line run can be turned off while warnings and errors survive.

What is *not* said: what a background render reports moment to moment, whether it counts frames or samples or percentages, whether anything at all appears between the start and the statistics.

**Started with Blender as a Python module.** The sources give differences, none of them about progress:

> Blender's typical signal handlers are not initialized, so there is no special handling for Control-C
> to cancel a render and a crash log is not written in the event of a crash.

and

> Functionality controlled by command line arguments (shown by calling `blender --help` aren't accessible).
> Typically this isn't such a limitation although there are some command line arguments that don't have
> equivalents in Blender's Python API (`--threads` and `--log` for example).

Source: Python API documentation, https://docs.blender.org/api/4.5/info_advanced_blender_as_bpy.html — read 2026-07-30. The first of these is worth reading twice: it tells us, in passing, that in ordinary Blender Control-C *is* handled specially and cancels a render. That is a statement about stopping, not about reporting, and I have not stretched it further.

**Started from Python inside a running Blender, and started from the Rendered viewport shading.** Nothing. No page in any of the four sources describes what either reports about progress.

### The other handlers, for completeness of the record

The API lists these render-related handlers, which are the only other places the sources describe a render signalling anything about where it has got to: `render_init` ("on initialization of a render job"), `render_pre` ("on render (before)"), `render_post` ("on render (after)"), `render_write` ("on writing a render frame (directly after the frame is written)"), `render_stats` (quoted above), `render_complete` ("on completion of render job"), `render_cancel` ("on canceling a render job"). Each of the job-level ones "Accepts one argument: the scene datablock being rendered". Source: https://docs.blender.org/api/4.5/bpy.app.handlers.html — read 2026-07-30. The API does not say whether these fire in every way of starting a render.

### Observed — what Blender 4.5 reports under each way of starting (2026-08-11)

Run in Blender 4.5.12 on **the Creator's Mac (a MacBook Pro with an Apple M1 Pro chip, 16 GB of memory, macOS 26.5.2)** and on **a rented Linux machine (Ubuntu 22.04.5 LTS, two NVIDIA RTX 4000 Ada Generation cards of 20 GB each, eleven cores, 49 GB of memory)**, each with a settings folder of its own. The same frame from the same file — Cycles on the processor, 16 samples, adaptive sampling off, 160 by 120 — was started five ways and the terminal captured each time. **LABEL: observed.**

**The ways do differ, and the split is between having a window and not.** Counting the lines that carry a sample count:

| how the render was started | sample lines printed |
|---|---|
| `blender -b <file>.blend -o <out> -f 1` | 3 |
| `blender -b <file>.blend -o <out> -a` (three frames) | 3 per frame |
| `blender -b <file>.blend -o <out> --python <script>` calling `bpy.ops.render.render(write_still=True)` | **3** |
| a window open, the same call from a script | 0 |
| a window open, started the way the F12 key starts it — `bpy.ops.render.render('INVOKE_DEFAULT', write_still=True)` | 0 |

So in Blender 4.5 a render started from Python with no window prints its full progress, exactly as `-f 1` does. **With a window open, no way of starting prints any progress at all** — the terminal gets only `Saved:` and `Time:`. Both machines agreed, and the Python case was run twice, once with the call in a `--python-expr` and once in a script file, with the same result both times.

**What the operator hands back.** Called plainly it returns `{'FINISHED'}`; started the way the F12 key starts it, `{'RUNNING_MODAL'}`, with the render going on afterwards. A picture was written in every case.

**The one documented difference, checked.** The render-statistics handler was attached and every call written down. With no window it fired **forty-three times**, the strings carrying `Fra:1`, two memory pairs, a `Time:` field and the scene name. With a window it fired **once**, carrying only `STATS ('Time: 00:00.13 (Saving: 00:00.01)', None)` — no frame, no memory. So for this version the documented clause about background mode holds: the frame and the memory figures are there with no window and absent with one.

**What the ways of starting actually are, read from the running program.** The Python render operator in 4.5 takes **five** arguments — `animation`, `write_still`, `use_viewport`, `layer`, `scene` — each with a description. The Render menu draws **two** render commands, `Render Image` and `Render Animation`, then `Render Audio...`, `View Render`, `View Animation` and `Lock Interface`. The 3D viewport's View menu draws `Viewport Render Image`, `Viewport Render Animation` and `Viewport Render Keyframes`. There is **no** `save_output` setting in this version. Identical on both machines.

**With a window open, the status bar shows one bar labelled `Render...`** with a whole-number percentage and an X to cancel; a sequencer render drives the same bar labelled `Rendering sequence...`; an animation names no frame and its percentage restarts at each frame. The full reading is under question 7 above.

**A render engine's own progress calls behave the same either way.** `update_progress`, `update_stats` and `update_memory_stats`, called from a tiny engine registered from a script, all returned without complaint with a window and without, on both machines, and none produced output of its own.

**Observed — a windowed run with the handler attached, and how it was closed (2026-08-12).** A later hand test repeated the windowed run in all four versions on both machines, because no earlier record says how a windowed Blender was made to end, and the exit status of such a run belongs to whoever ended it. Run on **the Creator's Mac (a MacBook Pro with an Apple M1 Pro chip, 16 GB of memory, macOS 26.5.2, Apple's own graphics and no second card)** and on **a rented Linux machine (Ubuntu 22.04.5 LTS on kernel 6.8, two NVIDIA RTX 4000 Ada Generation cards of 20 GB each on driver 580.105.08, an AMD Ryzen Threadripper PRO 3945WX with eleven cores, 49 GB of memory), whose window was opened on its own Xorg display**. `/tmp` was emptied of `*.crash.txt` before every run. **LABEL: observed.**

**How a windowed Blender is closed turns out to be part of the answer.** `bpy.ops.wm.quit_blender()` called straight from the start-up script did not close the run on the rented Linux machine: the script printed `STILL HERE AFTER quit_blender()` and Blender was still going when a 120-second cap ended it, and again when a 240-second cap ended it — both times the status was 124, which is `timeout`'s and not Blender's. On the Creator's Mac the same call did close the run, exiting 0, but 24.7 seconds after the render in one run and 6.3 seconds after it in another. A timer registered with `bpy.app.timers.register`, calling `bpy.ops.wm.quit_blender()` three seconds after the render, closed the run on both machines every time, and that is how the runs below were closed.

**Closed by the timer, Blender 4.5 exited 0 on both machines, with no crash file.** The whole of standard output from the Creator's Mac, long paths shortened:

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

Everything from `BACKGROUND` to `STILL HERE` is the test script's own printing. `Blender quit` followed after a blank line, and standard error was empty. **The rented Linux machine gave the same run, exit 0, handler once, `render_complete` once, standard error empty** — with one line different: where the Mac saved a session recovery file, the Linux machine printed `Error: Cannot open file /tmp/quit.blend@ for writing: Permission denied`.

**So the count this document already gives is borne out: with a window the handler fires once**, carrying only the closing `Time:` line. **No run of this version on either machine ended in a segmentation fault, and no crash file was written in `/tmp`.**

**Not covered.** This does not settle what the time estimate says when the pointer rests on the bar — that needs a hover, and neither machine can move the pointer without a hand on it. It does not settle which key is bound to which command: with no window Blender's key configuration holds no items. It does not settle what a viewport render or the Rendered shading mode reports, nor what a render started from Blender loaded as a Python module reports, neither machine having such a build. It does not settle anything about Windows.

**Outside the interface.** Settled, and it is the hinge of this answer. With no window a render started from the command line **or from Python** prints its full progress — stage lines, three sample lines, a memory figure and a `Remaining:` estimate — and the statistics handler is handed forty-three strings. With a window open none of that reaches the terminal in any way of starting, the handler is handed one string, and what stands in for it is the status bar's one labelled bar with its whole-number percentage. The step marked inferred above, that the status bar report is absent outside the interface, is borne out. Both machines agreed. **Added 2026-08-12:** a windowed run started from a terminal, with the handler attached, ended cleanly on both machines — exit 0, no crash file — when closed by a timer calling `bpy.ops.wm.quit_blender()`; the same call made straight from the start-up script did not close a windowed Blender on the Linux machine at all.

**Operating system.** Settled for macOS and for Linux, and they agreed on every point above: the same counts of printed sample lines for all five ways, the same operator arguments and menu entries, and the same `Render...` label and whole-number percentage on each machine's own screen. **One difference between the machines, found on 2026-08-12**: `bpy.ops.wm.quit_blender()` called from a start-up script closed a windowed Blender on the Creator's Mac, after six seconds in one run and twenty-five in another, and did not close it on the rented Linux machine within four minutes. At quit time the Mac printed `Saved session recovery to "/var/folders/…/T/quit.blend"` and the Linux machine printed `Error: Cannot open file /tmp/quit.blend@ for writing: Permission denied`. Closed by a timer, both machines exited 0. **Windows is untested**, no Windows machine having run it, so the platform notes the sources give — `-con, --start-console` "(Windows only)", `/?` "(Windows only)", `-r, --register` "(Windows & Linux only)", `--debug-ghost` "(Linux only)" — stand unchecked for Windows.

---

## Things a person with Blender 4.5 open could settle by hand in a few minutes

The nine items that stood here have all been run, on both machines, and their results are written into the answers above. What is left open is below.

1. **The time estimate that appears when the pointer rests on the progress bar (questions 7 and 9).** It needs a hover, and neither machine has any way of moving the pointer without a hand on it.

2. **Whether a render at the very top of the size range finishes and writes a file (question 7).** At 65536 by 65536 the render starts and divides into 1024 tiles, but the runs were stopped before the end.

3. **Everything on Windows (questions 7, 8 and 9).** No Windows machine has run it.

4. **What a render started from Blender loaded as a Python module reports (question 9).** Neither machine has such a build.

5. **Which key is bound to which render command (question 9).** With no window Blender's key configuration holds no items.

6. **What a run prints when the output is a video file (question 8).** Not tested; every run above wrote still images.

---

2026-08-11 — Claude Opus 5 (Claude Code) — Reports, Blender 4.5

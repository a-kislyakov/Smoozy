# Settings — Blender 5.0

**Sources consulted, and when.** Blender 5.0 manual (plain-text snapshot plus two of its picture files), Blender 5.0 Python API documentation, Blender 5.0 release notes, and Cycles' own add-on files as shipped with Blender 5.0 — all as supplied in `Blender Sources/`, all taken on 2026-07-30. That snapshot date is the date read for every finding below. I did not use the web.

I read only the question sheet and files inside `Blender Sources/`. I did not open any other part of this project.

One note on a source I did **not** use: `Blender Sources/requirements/` exists but was not on the list of sources I was given, and it carries no version in its address, so I left it closed. Nothing below depends on it.

A note on how I searched: the manual and API text were searched whole, by word, before any file was opened. Where an answer is "not in the sources", the words I searched for are listed.

---

## 1. Which of a render's settings can be settled at the moment a render is started from outside Blender, and which can only be changed by changing the file?

**Label: documented** for the listed command-line options and controls, with one **inferred** part flagged inside. The quoted material does not state that every control without a dedicated option is saved in a file or can only be changed by changing that file.

Blender takes a fixed set of options on the command line. From the manual's Command Line Arguments page, the ones that bear on the seven things asked about:

> "-a, --render-anim
>
> Render frames from start to end (inclusive)."

> "-f, --render-frame <frame>
>
> Render frame <frame> and save it.
>
> +<frame> start frame relative, -<frame> end frame relative.
>
> A comma separated list of frames can also be used (no spaces).
>
> A range of frames can be expressed using .. separator between the first and last frames (inclusive)."

> "-s, --frame-start <frame>
>
> Set start to frame <frame>, supports +/- for relative frames too."

> "-e, --frame-end <frame>
>
> Set end to frame <frame>, supports +/- for relative frames too."

> "-j, --frame-jump <frames>
>
> Set number of frames to step forward after each rendered frame."

> "-E, --engine <engine>
>
> Specify the render engine.
> Use -E help to list available engines."

(Blender 5.0 manual, https://docs.blender.org/manual/en/5.0/advanced/command_line/arguments.html — read 2026-07-30.)

Taking the seven in turn:

- **Frame range** — settable at start. `-s` / `--frame-start` and `-e` / `--frame-end`. Blender also documents Frame Start and End in the Output properties: "Set the Start and End frames for Rendering Animations." (https://docs.blender.org/manual/en/5.0/render/output/properties/frame_range.html)
- **An interval to skip frames by** — settable at start. `-j` / `--frame-jump`. Blender also documents Step: "Controls the number of frames to advance for each frame rendered, a value above 1 skips frames." (same page). In the API it is `frame_step`, "Number of frames to skip forward while rendering/playing back each frame" (https://docs.blender.org/api/5.0/bpy.types.Scene.html).
- **A list of separate frames** — settable at start. `-f` takes "A comma separated list of frames". The sources do not document a matching saved list in the file: the documented frame controls are start, end and step (Frame Range panel above; `frame_start`, `frame_end`, `frame_step` on Scene). Searched for: "frame list", "frames", "frame set", "render frames", "frame range", "step", "jump" across manual and API.
- **A sample count** — **not** settable at start by a dedicated option. There is no `--samples` or equivalent anywhere in the Command Line Arguments page (I read the page end to end and searched the whole manual and API for "samples", "--samples", "sample count", "spp", "max samples"). Question 4 records the documented engine settings.
- **Whether a view layer renders at a given frame** — **not** settable at start by a dedicated option. Blender documents the setting: "Use for Rendering / The active view layer will be used during rendering." And, importantly for anything driven from outside: "Render Single Layer ... Note: This option is ignored when rendering from the command line." (https://docs.blender.org/manual/en/5.0/render/layers/view_layer.html) In the API it is `ViewLayer.use`, "Enable or disable rendering of this View Layer", boolean, default True (https://docs.blender.org/api/5.0/bpy.types.ViewLayer.html). The "at a given frame" part: the manual's picture of that panel, `_images/render_layers_view-layer_panel.png`, shows a small round decorator dot to the right of the Use for Rendering checkbox, and the manual says decorators "may appear next to number fields, menus, and checkboxes to indicate the property can be animated" and "Clicking on the decorator dot icon will add a Keyframe to that property" (https://docs.blender.org/manual/en/5.0/interface/controls/buttons/decorators.html). So this one is animatable — **inferred**, from the manual text plus its own picture: the join between "that row carries a dot" and "a dot means the property can be animated" is mine, and no page says this property is animatable. *[Citation check, 2026-08-05: was labelled documented; the quotes and the picture are all where the finding says, but a reader has to supply the join, so the label now reads inferred.]*
- **A render engine** — settable at start. `-E` / `--engine`. Worked example in the manual: "blender -b file.blend -E CYCLES -s 10 -e 500 -t 2 -a" with "-E CYCLES / Use the 'Cycles Render' engine." (https://docs.blender.org/manual/en/5.0/advanced/command_line/render.html)
- **A resolution percentage** — **not** settable at start by a dedicated option. Blender documents the control: "Percentage / Slider to reduce or increase the size of the rendered image relative to the Resolution values." (https://docs.blender.org/manual/en/5.0/render/output/properties/format.html) In the API, `RenderSettings.resolution_percentage`, "Percentage scale for render resolution", int in [1, 32767], default 100 (https://docs.blender.org/api/5.0/bpy.types.RenderSettings.html). The manual's picture of that panel, `_images/render_output_properties_format_panel.png`, shows it as a "%" field reading 100%.

Other settings that *can* be fixed at start, from the same page, since the question says "at least": output path and filename (`-o`), image format (`-F`), whether the extension is appended (`-x`), thread count (`-t`), the scene to render (`-S`), audio device (`-noaudio`, `-setaudio`), GPU backend (`--gpu-backend`), and for Cycles specifically the render device (`--cycles-device`) and statistics logging (`--cycles-print-stats`).

There is a general escape hatch that the manual documents but does not connect to render settings in so many words: `-P, --python <filepath>`, `--python-text <name>` and `--python-expr <expression>` run a script at launch, and the API's own quickstart shows a render setting being read and written through that same interface — `bpy.data.scenes[0].render.resolution_percentage` (https://docs.blender.org/api/5.0/info_quickstart.html), described as "its attributes can be accessed much like you would change a setting using the graphical interface". **Inferred step:** if a script run at launch can set a scene attribute, then any of the settings above that have no dedicated option — sample count, resolution percentage, view-layer enabling — can still be fixed at the moment of starting, by running a script before the render option. The sources do not say this outright for render settings, so it is inference, not documentation.

One thing the manual is emphatic about, which shapes all of the above:

> "Arguments are executed in the order they are given."
>
> "blender --background test.blend --render-output /tmp --render-frame 1
>
> …works as expected."

(Command Line Arguments page.) And on the same theme: "Always position -f or -a as the last arguments." (Rendering From The Command Line page.)

**Not covered.** This does not settle which controls without a dedicated command-line option are preserved in a file or require a changed file. It does not settle what happens if a command-line option contradicts a file setting in a way the manual does not describe (for example `-E` naming an engine the file has no settings for). It does not settle whether the Python escape hatch reaches settings that a render engine has already read. It does not list every command-line option — only those touching the seven things asked about, plus a few neighbours.

**Outside the interface.** This whole answer *is* the outside-the-interface case: every option above is a command-line option, documented on the command-line pages. Two places where inside and outside differ are documented: "Render Single Layer ... is ignored when rendering from the command line", and the audio device is disabled in background mode by default. The `-a` flag also has a second, unrelated meaning outside background mode — "Instead of showing Blender's user interface, this runs Blender as an animation player ... (ignored if -b is set)".

**Operating system.** The Command Line Arguments page marks operating-system limits explicitly where they exist, and none of the options above carry such a mark. Options that *are* marked include `-con, --start-console` "(Windows only)", `/?` "(Windows only)", `-r, --register` "(Windows & Linux only)", `--debug-ghost` "(Linux only)" and `--qos <level>` "(Windows only)". Environment variables differ: "TEMP: Store temporary files here (MS-Windows). / TMPDIR: Store temporary files here (UNIX Systems)." **Observed 2026-08-05:** every test below was run on macOS only, so the other two operating systems stay untested. **Observed 2026-08-06:** the same tests were run again on a rented Linux machine (see the Linux block at the end of this answer). Every argument this answer names behaves the same way there, so the answer now holds on macOS and on Linux; Windows stays untested. The argument list itself is not identical on the two — Blender's own help text is longer on Linux.

### Observed — hand tests 1, 2, 3, 4, 5 and 10 (2026-08-05)

All were run from a terminal on the Creator's own computer — a MacBook Pro with an M1 Pro chip, macOS 26.5.2, Apple's own graphics and no other graphics card — using Blender 5.0.1 at `/Applications/Blender 5.0.1.app/Contents/MacOS/Blender`, with the `BLENDER_USER_RESOURCES` setting pointed at a fresh empty folder so the Creator's own Blender settings were untouched. Label: **observed**.

**On the conditions of hand tests 1, 2 and 3.** Those three were first run before it was known that Blender falls back to the machine's real settings when that folder does not exist, so the first attempts loaded the Creator's own add-ons. Those attempts were discarded. All three were re-run from scratch with the folder created first — hand test 3 with its files rebuilt as well — and each version was checked before use, reporting its own throwaway settings folder and Blender's eight default add-ons and nothing else. What is written below is the clean re-run.

**Hand test 1 — the engine argument.** `blender -E help` printed:

```
Blender Engine Listing:
	BLENDER_EEVEE
	BLENDER_WORKBENCH
	CYCLES
```

exit code 0. With a name no engine bears:

```
blender -b test.blend -E NOT_A_REAL_ENGINE -o out/e_ -F PNG -f 1
00:00.268  blend            | Read blend: ".../test.blend"

Error: engine not found 'NOT_A_REAL_ENGINE'
Blender 5.0.1 (hash a3db93c5b259 built 2025-12-16 01:51:40)
```

exit code 1, and no image written.

**Hand test 2 — is there really no argument for a sample count, a resolution percentage or a view layer?** `blender --help` was written to a file and searched, without opening any blend-file. The whole output is 466 lines. Searching it for `sampl`, `resolution`, `percent`, `view layer` and `python-use` returned exactly three lines:

```
128:	Do not use native pixel size, for high resolution displays (MacBook 'Retina').
161:--python-use-system-env 
411:	Background mode: Do not systematically build and evaluate ViewLayers' dependency graphs
```

None sets a render setting. This build has `--python-use-system-env` and no `--python-use-user-env`.

**Hand test 3 — a list of frames, a range of frames, and the frame jump.** A file was saved from factory startup with the Workbench engine and a 64 by 64 resolution. Five renders, each into its own empty folder, wrote:

```
blender -b test.blend -o out-list/f_  -F PNG -f 1,5,9        -> f_0001.png f_0005.png f_0009.png
blender -b test.blend -o out-range/f_ -F PNG -f 3..6         -> f_0003.png f_0004.png f_0005.png f_0006.png
blender -b test.blend -o out-mixed/f_ -F PNG -f 1,5..7,12    -> f_0001.png f_0005.png f_0006.png f_0007.png f_0012.png
blender -b test.blend -j 3 -o out-jumpf/f_ -F PNG -f 1..10   -> f_0001.png … f_0010.png (all ten)
blender -b test.blend -j 3 -s 1 -e 10 -o out-jumpa/f_ -F PNG -a -> f_0001.png f_0004.png f_0007.png f_0010.png
```

So `-f` takes a comma list, a `..` range, and the two mixed; `-j` is applied to `-a` and not to `-f`.

*Recorded because it happened:* on the first attempt the `-f 1,5,9` run printed `Writing: /var/folders/.../blender.crash.txt` and wrote nothing. That was caused by the person running the test pointing `BLENDER_USER_RESOURCES` at a folder that did not exist; Blender crashes on start in that case. With the folder created first, the same command ran and wrote the three files listed above. Nothing about the frame arguments was involved.

**Hand test 4 — does a launch argument change the file on disk?** A file was saved with frame start 1, frame end 250, frame step 1, resolution percentage 100 and the engine left at `BLENDER_EEVEE`. Then:

```
blender -b test.blend -E CYCLES -s 5 -e 7 -j 2 -o out/f_ -F PNG -f 5
```

The file's modification time and size were unchanged (`1785890067`, `92393 bytes`, before and after). Reading it back with no arguments printed:

```
READBACK 1 250 1 100 BLENDER_EEVEE
```

So the arguments applied to that run only.

**Hand test 5 — `--python-expr` at launch.** A file was saved with Cycles, 512 samples, two view layers (`ViewLayer` and `Layer2`) and a 200 by 100 resolution at 100 per cent. With the expression after the blend-file argument:

```
blender -b test.blend --python-expr "import bpy; s=bpy.data.scenes[0]; s.cycles.samples=8; s.render.resolution_percentage=25; s.view_layers['Layer2'].use=False; print('SET', s.cycles.samples, s.render.resolution_percentage, s.view_layers['Layer2'].use)" -o out-after/f_ -F PNG -f 1
```

printed `SET 8 25 False` and wrote an image of `pixelWidth: 50   pixelHeight: 25`. With the expression before the blend-file argument it printed `SET 8 25` and wrote `pixelWidth: 200   pixelHeight: 100`. **This settles the inferred step above:** a setting with no argument of its own can be settled at launch by Python, and only when the expression comes after the file.

**Hand test 10 — a view layer's "Use for Rendering", and "Render Single Layer".** From factory startup, `keyframe_insert` on a view layer's `use` returned `True`. A file was saved with two view layers, `Layer2`'s `use` keyframed on at frame 1 and off at frame 2, Workbench, 64 by 64, then rendered:

```
blender -b test.blend -o out/f_ -F OPEN_EXR_MULTILAYER -s 1 -e 2 -a
00:00.271  render           | Rendering frame 1
00:00.271  render           | Start rendering: Scene, ViewLayer
                             Scene, Layer2
00:00.4xx  render           | Rendering frame 2
                             Scene, ViewLayer
```

and the channel names in the two written files were:

```
f_0001.exr : Layer2.Combined  Layer2.CombinedpixelAspectRatio  ViewLayer.Combined  ViewLayer.CombinedpixelAspectRatio   (8411 bytes)
f_0002.exr : ViewLayer.Combined  ViewLayer.CombinedpixelAspectRatio                                                     (4394 bytes)
```

So an animation render driven from outside honours the switch frame by frame. Separately, with `use_single_layer` turned on at launch, the run printed `SINGLE-LAYER ON, frame 1, layers on: ['ViewLayer', 'Layer2']` and the written file held both `Layer2.Combined` and `ViewLayer.Combined`. So the setting is indeed ignored from the command line.

### Observed — hand test 25, run by the Creator (2026-08-05)

Run by the Creator in Blender 5.0's own window, on their own computer — a MacBook Pro with an M1 Pro chip, macOS 26.5.2 — on 2026-08-05. That settles macOS and leaves Windows and Linux untested. Label: **observed**.

They opened the Properties editor's View Layer Properties tab, on a view layer named `LayerA`, and photographed it. What is visible: a **View Layer** panel holding **Use for Rendering**, ticked, with a small round dot at the far right of its row; and **Render Single Layer**, unticked, with no dot. Below it sit the Passes, Cryptomatte, Shader AOV, Filter and Custom Properties panels.

They also wrote, in their own words:

> "Pressing I at Use for Rendering makes the checkbox yellow and the dot next to it a diamond. Consistent across all versions. Render Single Layer is non-animatable in all versions, too. Here's the message: "use_single_layer" property cannot be animated. Also consistent in all versions."

**This settles from the running program what the sources only carried in a picture.** There is a decorator dot beside "Use for Rendering" and none beside "Render Single Layer", and the switch does take a keyframe. It adds one thing the sources did not carry: "Render Single Layer" is refused a keyframe outright, with the message quoted above.

**What the screenshots do not cover.** The entry also asked what the right-click menu on "Use for Rendering" offers. No shot of that menu was among those supplied — the Creator answered by pressing I instead — so what that menu lists is still unrecorded.

### Observed on the rented Linux machine — Linux hand tests 2, 3, 4, 5 and 12 (2026-08-06)

Run over SSH from a terminal on a rented Linux machine — Ubuntu 22.04.5 LTS, NVIDIA GeForce RTX 4090, driver 580.105.08, twelve cores, run over SSH, using Blender 5.0.1 at `/opt/blender/blender-5.0.1-linux-x64/blender`, with `BLENDER_USER_RESOURCES` pointed at `/root/smoozy/Settings/res-5.0`, a folder of this topic's own that already existed. That folder was checked before use and reported its own config directory and Blender's eight default add-ons and nothing else. The machine is a container sharing someone else's hardware, and up to four topics were being worked on it at once, so nothing below claims anything about how long a run took or how much memory was free. Label: **observed**.

These are a second machine's answers to the tests the Creator's Mac answered on 2026-08-05. The two are never merged: where they agree it says so, and where they differ it says that plainly. The full list is `Audits/Blender Research/Settings/Hand tests — Linux.md`.

**Linux hand test 2 — is there really no argument for a sample count, a resolution percentage or a view layer?** `blender --help` was written to a file and searched, with no blend-file opened. The whole help output is **479 lines** on this machine; the same command on the Creator's Mac gave 466, so the argument list is not the same length on the two operating systems. Searching it for `sampl`, `resolution`, `percent`, `view layer` and `python-use` returned only lines that set no render setting — the `--no-native-pixels` description, the Python-environment arguments, and the background-mode note about view layer dependency graphs. There is no `--samples` and no `--resolution` in this build either. `--python-use-system-env` is present and `--python-use-user-env` is not, as on the Mac.

**Which arguments name an operating system.** Searching the same help text for operating-system marks gives exactly these, and no render argument is among them:

```
	Start with the console window open (ignored if '-b' is set), (Windows only).
	Disable all window decorations (Linux only).
	Enable debug messages for Ghost (Linux only).
	Print this help text and exit (Windows only).
	Register blend-file extension for current user, then exit (Windows & Linux only).
	Register blend-file extension for all users, then exit (Windows & Linux only).
	Unregister blend-file extension for current user, then exit (Windows & Linux only).
	Unregister blend-file extension for all users, then exit (Windows & Linux only).
	Set the Quality of Service (QoS) mode for hybrid CPU architectures (Windows only).
```

So the Windows-only arguments really are printed on Linux as well, and the render arguments `-a -f -s -e -j -o -E -t -F -x -S -y -Y -P --python-expr` all appear in this build's help with the same names and short forms the Mac found.

**Linux hand test 3 — a list of frames, a range of frames, and the frame jump.** A file was saved from factory startup with Workbench and 64 by 64. Five renders, each into its own empty folder, all exiting 0 with no error printed:

```
blender -b test.blend -o out-list/f_  -F PNG -f 1,5,9        -> f_0001.png f_0005.png f_0009.png
blender -b test.blend -o out-range/f_ -F PNG -f 3..6         -> f_0003.png f_0004.png f_0005.png f_0006.png
blender -b test.blend -o out-mixed/f_ -F PNG -f 1,5..7,12    -> f_0001.png f_0005.png f_0006.png f_0007.png f_0012.png
blender -b test.blend -j 3 -o out-jumpf/f_ -F PNG -f 1..10   -> f_0001.png f_0002.png f_0003.png f_0004.png f_0005.png f_0006.png f_0007.png f_0008.png f_0009.png f_0010.png
blender -b test.blend -j 3 -s 1 -e 10 -o out-jumpa/f_ -F PNG -a -> f_0001.png f_0004.png f_0007.png f_0010.png
```

The same files in the same folders the Mac's run produced. **The two machines agree.**

**Linux hand test 4 — does a launch argument change the file on disk?** A file was saved with frame start 1, frame end 250, frame step 1, resolution percentage 100 and the engine left at `BLENDER_EEVEE`. Then:

```
blender -b test.blend -E CYCLES -s 5 -e 7 -j 2 -o out/f_ -F PNG -f 5
```

Its modification time and size were unchanged before and after, and reading it back in a fresh Blender with none of those arguments printed:

```
READBACK 1 250 1 100 BLENDER_EEVEE
```

**The two machines agree.**

**Linux hand test 5 — setting a sample count, a resolution percentage and a view layer switch by `--python-expr` at launch.** A file was saved with Cycles, 512 samples, two view layers (`ViewLayer` and `Layer2`) and 200 by 100 at 100 per cent. With the expression after the blend-file argument it printed `SET 8 25 False`, Cycles reported `Sample 8/8`, the progress lines named `Scene, ViewLayer` alone, and the written image measured 50 by 25 pixels. With the expression before the blend-file argument it printed `SET 8 25`, Cycles reported `Sample 512/512`, the progress lines named `Scene, Layer2`, and the image measured 200 by 100. **The two machines agree** — the expression settles all three, and only when it comes after the file.

**Linux hand test 12 — a view layer's switch per frame, and "Render Single Layer" from the command line.** A file was saved with two view layers, `Layer2`'s switch keyframed on at frame 1 and off at frame 2, Workbench, 64 by 64. `blender -b test.blend -o out/f_ -F OPEN_EXR_MULTILAYER -s 1 -e 2 -a` wrote two files, and the names found in their headers were:

```
f_0001.exr : ViewLayer.Combined.R/G/B/A  ViewLayer.CombinedpixelAspectRatio
             Layer2.Combined.R/G/B/A     Layer2.CombinedpixelAspectRatio     (8213 bytes)
f_0002.exr : ViewLayer.Combined.R/G/B/A  ViewLayer.CombinedpixelAspectRatio  (4243 bytes)
```

Then the same file with the setting turned on at launch printed `SINGLE-LAYER ON, frame 1, layers on: ['ViewLayer', 'Layer2']` and wrote a file holding both layers again. **The two machines agree** on both halves — the switch is honoured frame by frame from outside Blender, and "Render Single Layer" is ignored there — and on the extra channel names, which the Mac also found in the newer three versions and not in 4.5.

**Which of these can take a keyframe at all**, asked of a factory-startup Blender:

```
KEY samples True
KEY taa_render_samples True
KEY render_aa ERROR bpy_struct.keyframe_insert() property "render_aa" not animatable
KEY use -> True
KEY use_single_layer ERROR bpy_struct.keyframe_insert() property "use_single_layer" not animatable
```

**The two machines agree**, and the refusal of "Render Single Layer" matches what the Creator saw in Blender's own window on the Mac.

---

## 2. Can a render be set to stop once the picture is clean enough, and can it be set to stop after a length of time rather than after a count?

**Label: documented**, for Cycles. Both exist there.

Stopping when clean enough — Cycles' adaptive sampling:

> "Adaptive Sampling
>
> If the Noise Threshold checkbox is enabled, Cycles will use adaptive sampling,
> cutting short the sampling process in areas that have become less noisy than the
> specified threshold value – in other words, not wasting time further refining
> areas that already look good enough."

> "Noise Threshold
>
> The threshold to decide whether to continue sampling a pixel.
> Typical values are in the range from 0.1 to 0.001, with lower values meaning better
> quality but longer render times.
> Setting this to 0 makes Cycles guess an automatic value based on the total sample count."

> "Min Samples
>
> The minimum number of samples a pixel receives before adaptive sampling is applied.
> When set to 0 (default), Cycles automatically picks a value determined by the Noise Threshold."

Stopping after a length of time:

> "Time Limit
>
> Stops rendering if the time exceeds the limit, even if the desired sample count
> hasn't been reached yet. Setting this value to zero disables the limit."

> "Note
>
> The time limit does not include pre-render processing time, only render time."

(Blender 5.0 manual, https://docs.blender.org/manual/en/5.0/render/cycles/render_settings/sampling.html — read 2026-07-30.)

**Label: observed**, for what these settings actually are in Cycles' own files:

> `    time_limit: FloatProperty(`
> `        name="Time Limit",`
> `        description="Limit the render time (excluding synchronization time). "`
> `        "Zero disables the limit",`
> `        min=0.0,`
> `        default=0.0,`
> `        step=100.0,`
> `        unit='TIME_ABSOLUTE',`
> `    )`

(`Blender Sources/cycles_5.0/properties.py`, lines 514–522, inside `class CyclesRenderSettings`.)

> `    use_adaptive_sampling: BoolProperty(`
> `        name="Use Adaptive Sampling",`
> `        description="Automatically reduce the number of samples per pixel based on estimated noise level",`
> `        default=True,`
> `    )`
> `    adaptive_threshold: FloatProperty(`
> `        name="Adaptive Sampling Threshold",`
> `        description="Noise level step to stop sampling at, lower values reduce noise at the cost of render time. Zero for automatic setting based on number of AA samples",`
> `        min=0.0,`
> `        max=1.0,`
> `        soft_min=0.001,`
> `        default=0.01,`
> `        precision=4,`
> `    )`

(Same file, lines 566–579.) So adaptive sampling is on by default in Cycles, with a threshold of 0.01, and the time limit is off by default.

For the other two engines: **not in the sources.** I searched the whole manual, API and release notes for "Time Limit", "time_limit", "Noise Threshold", "noise_threshold", "adaptive" and "threshold". The only render-stopping hits are the Cycles page above; `time_limit` elsewhere in the API is an argument to a redraw-timing test operator, unrelated. EEVEE's own Sampling page (https://docs.blender.org/manual/en/5.0/render/eevee/render_settings/sampling.html) offers a plain count — "Samples / The number of samples to use in the final render" — with no threshold and no time limit; its only "Light Threshold" is about which lights contribute, not about stopping. Workbench's Sampling page (https://docs.blender.org/manual/en/5.0/render/workbench/sampling.html) offers only fixed anti-aliasing choices: "5, 8, 11, 16, 32". Those findings do not establish that no other stopping setting exists.

**Not covered.** The manual describes adaptive sampling as cutting short sampling *in areas* — per pixel. It does not say in so many words when the render as a whole finishes under adaptive sampling, nor whether the time limit is checked per frame or across a whole animation. It does not say what happens when a time limit and a noise threshold are both in force. It says nothing about the accuracy of the time limit.

**Outside the interface.** Both are documented Cycles settings; there is no command-line option for either (I searched the Command Line Arguments page and found none). The quoted material does not state whether they are saved in the file or behave differently in background mode. **Observed 2026-08-05 (hand tests 11 and 12, below):** both are carried by the file and both take effect in a render started from a terminal with no window.

**Operating system.** The sources do not say. Nothing on the Cycles Sampling page is marked as platform-specific, though neighbouring text notes that "Path guiding is only available when rendering on a CPU", which is a device limit rather than an operating-system one. The tests below were run on macOS only. **Observed 2026-08-06:** adaptive sampling and the EEVEE and Workbench property lists were run again on a rented Linux machine and came back the same, so those parts hold on macOS and on Linux. The Time Limit test was not re-run there, because it can only be read from timing and the machine was shared, so that part still holds for macOS alone. Windows stays untested.

### Observed — hand tests 11, 12 and 13 (2026-08-05)

All three were run from a terminal on the Creator's own computer — a MacBook Pro with an M1 Pro chip, macOS 26.5.2, Apple's own graphics and no other graphics card — using Blender 5.0.1 at `/Applications/Blender 5.0.1.app/Contents/MacOS/Blender`, with the `BLENDER_USER_RESOURCES` setting pointed at a fresh empty folder. Label: **observed**.

**Hand test 11 — is the Time Limit per frame or per job, and is a cut-short frame still written?** A file was saved from factory startup with Cycles on the processor, 400 by 400, Max Samples 16384, adaptive sampling off, denoising off, Time Limit 5 seconds, frame range 1 to 3. Then `blender -b test.blend -o out/f_ -F PNG -a`. The per-frame times printed were `Time: 00:05.28`, `Time: 00:05.21`, `Time: 00:05.24`, and the whole run took 17 seconds by the shell's clock. All three images were written: `f_0001.png f_0002.png f_0003.png`. So the limit is applied to each frame, not to the job, and a frame cut short is still saved.

**Hand test 12 — does adaptive sampling end a frame before the sample count is reached?** Two files were saved from factory startup, both Cycles on the processor, 128 by 128, Max Samples 2048, Time Limit 0, denoising off — one with adaptive sampling on and a Noise Threshold of 0.1, one with it off. The sample lines printed:

```
adaptive on  : Sample 0/2048  Sample 1/2048  Sample 48/2048  Sample 0/2048   Time: 00:00.08
adaptive off : Sample 2048/2048                                              Time: 00:01.12
```

So the whole frame stopped at 48 of 2048 samples with adaptive sampling on.

**Hand test 13 — has EEVEE or Workbench any way to stop a render early?** Every property of `scene.eevee` and `scene.display` was listed from factory startup and searched for `time`, `threshold`, `adaptive`, `noise`, `stop` and `limit`. `scene.eevee` has 52 properties and `scene.display` has 9. Neither block holds a time limit or a rule for stopping when the picture is clean enough. The nearest matches on `scene.eevee` were `bokeh_threshold` ("Brightness threshold for using sprite base depth of field") and `light_threshold` ("Minimum light intensity for a light to contribute to the lighting"); on `scene.display` there was no match. Its nine properties in full:

```
light_direction | Direction of the light for shadows and highlights
shadow_shift | Shadow termination angle
shadow_focus | Shadow factor hardness
matcap_ssao_distance | Distance of object that contribute to the cavity/edge effect
matcap_ssao_attenuation | Attenuation constant
matcap_ssao_samples | Number of samples
render_aa | Method of anti-aliasing when rendering final image
viewport_aa | Method of anti-aliasing when rendering 3d viewport
shading | Shading settings for OpenGL render engine
```

### Observed — hand test 26, run by the Creator (2026-08-05)

Run by the Creator in Blender 5.0's own window, on their own computer — a MacBook Pro with an M1 Pro chip, macOS 26.5.2 — on 2026-08-05. That settles macOS and leaves Windows and Linux untested. Label: **observed**.

With the Render Engine set to **EEVEE**, they opened Render Properties and photographed the Sampling panels with every sub-panel expanded. Every control visible, in order:

```
Sampling
  Viewport   Samples                     16
             Temporal Reprojection       ticked
             Jittered Shadows            unticked
  Render     Samples                     64
  Shadows (panel ticked)
             Rays                        1
             Steps                       6
             Volume Shadows              unticked   Steps 16 (greyed)
             Resolution                  1.000
  Advanced   Light Threshold             0.010
```

With the Render Engine set to **Workbench**, the whole Sampling panel is two rows:

```
Sampling
  Render     8 Samples
  Viewport   Single Pass Anti-Aliasing
```

**This closes the "not in the sources" answer above by eye as well as by property list.** Neither engine's Sampling panel offers a time limit, a noise threshold, or any other control that would end a render early. EEVEE's one threshold is Light Threshold, which its property list describes as the minimum intensity for a light to contribute.

### Observed on the rented Linux machine — Linux hand tests 13 and 14 (2026-08-06)

Run over SSH from a terminal on the rented Linux machine, using Blender 5.0.1 at `/opt/blender/blender-5.0.1-linux-x64/blender`, with `BLENDER_USER_RESOURCES` pointed at `/root/smoozy/Settings/res-5.0`, a folder of this topic's own that already existed. That folder was checked before use and reported its own config directory and Blender's eight default add-ons and nothing else. The machine is a container sharing someone else's hardware, and up to four topics were being worked on it at once, so nothing below claims anything about how long a run took or how much memory was free. Label: **observed**.

These are a second machine's answers to the tests the Creator's Mac answered on 2026-08-05. The two are never merged: where they agree it says so, and where they differ it says that plainly. The full list is `Audits/Blender Research/Settings/Hand tests — Linux.md`.

**Linux hand test 13 — does adaptive sampling end the frame before the sample count is reached?** Two Cycles files at 128 by 128, Max Samples 2048, Time Limit 0, denoising off — one with adaptive sampling on at a noise threshold of 0.1, one with it off. Both rendered one frame with `-- --cycles-print-stats`, both exited 0, both wrote their image. The sample counts reported:

```
adaptive on  : Sample 0/2048 -> Sample 1/2048 -> Sample 48/2048   (stopped there)
adaptive off : Sample 0/2048 -> Sample 1/2048 -> Sample 2048/2048
```

**The two machines agree**, at the same 48 samples out of 2,048. So adaptive sampling ends the whole frame early, not only the sampling of single pixels.

**Linux hand test 14 — has EEVEE or Workbench anything that stops a render?** Every property of the scene's EEVEE block and of its Workbench block was listed with its description. EEVEE holds **52** on this build and Workbench holds **9** — the same two counts the Mac's run gave for this version. Searching both lists for `time`, `threshold`, `adaptive`, `noise`, `stop` and `limit` matched only EEVEE lines, and none of them stops anything:

```
eevee gi_diffuse_bounces | Number of times the light is reinjected inside light grids, 0 disable indirect diffuse light
eevee gi_glossy_clamp | Clamp pixel intensity to reduce noise inside glossy reflections from reflection cubemaps (0 to disable)
eevee taa_samples | Number of samples, unlimited if 0
eevee use_taa_reprojection | Denoise image using temporal reprojection (can leave some ghosting)
eevee clamp_surface_direct | If non-zero, the maximum value for lights contribution on a surface. …
eevee clamp_surface_indirect | If non-zero, the maximum value for indirect lighting on surface. …
eevee clamp_volume_direct | If non-zero, the maximum value for lights contribution in volumes. …
eevee clamp_volume_indirect | If non-zero, the maximum value for indirect lighting in volumes. …
eevee volumetric_light_clamp | Maximum light contribution, reducing noise
eevee bokeh_threshold | Brightness threshold for using sprite base depth of field
eevee motion_blur_steps | Controls accuracy of motion blur, more steps means longer render time
eevee light_threshold | Minimum light intensity for a light to contribute to the lighting
```

No line matched on the Workbench block at all. **The two machines agree**: neither engine has a time limit, a noise threshold or anything else that would end a render early, on either operating system.

**One thing that was not run here.** Whether Cycles' Time Limit is per frame or per job can only be read from how long a run takes, and this machine was shared with three other topics throughout, so a timing claim from it would be worthless. It is written out as a test needing the machine to itself in `Audits/Blender Research/Settings/Hand tests — Linux.md`. What the Mac's run settled — the limit is applied per frame and a cut-short frame is still written — therefore still stands for macOS alone.

---

## 3. Which of a render's settings belong to a particular render engine rather than to every engine?

**Label: documented**, at the level of how the settings are grouped. One conclusion from that grouping is marked as inference below.

The manual splits them by where they sit in the interface:

> "Scene
>
> Tabs related to the active scene:
>
> Render: EEVEE, Cycles, or Workbench settings.
>
> Output
>
> View Layer
>
> Scene
>
> World"

(https://docs.blender.org/manual/en/5.0/editors/properties_editor.html — read 2026-07-30.)

And:

> "More renderers from third-party developers are available as add-ons.
> Each renderer has its own render settings to control render quality and performance."

(https://docs.blender.org/manual/en/5.0/render/introduction.html)

The source calls the Render tab engine settings and places Output, View Layer, Scene and World separately. **Inferred step:** this makes the latter groups shared rather than engine-specific. Reading across the manual's own structure, the pages under `render/cycles/render_settings/` (Sampling, Light Paths, Volumes, Curves/Hair, Subdivision, Motion Blur, Film, Performance, Simplify, Grease Pencil), under `render/eevee/render_settings/` (Sampling, Clamping, Raytracing, Volumes, Curves, Depth of Field, Motion Blur, Film, Performance, Grease Pencil) and under `render/workbench/` (Lighting, Color, Options, Sampling, Display Settings, Performance) are each engine's own. The pages under `render/output/properties/` — Format, Frame Range, Stereoscopy, Output, Metadata, Post Processing — are presented separately.

**Label: documented** for the API properties and **observed** for Cycles' own files:

The API lists separate properties for the engines:

> "cycles
>
> Cycles render settings
>
> Type:
>
> CyclesRenderSettings, (readonly)"

> "display
>
> Scene display settings for 3D viewport
>
> Type:
>
> SceneDisplay, (readonly)"

> "eevee
>
> EEVEE settings for the scene
>
> Type:
>
> SceneEEVEE, (readonly)"

(https://docs.blender.org/api/5.0/bpy.types.Scene.html — read 2026-07-30. `Scene.render`, of type `RenderSettings`, is the shared block on the same page.)

Cycles' own files show the same split from the other side. Its settings are a block bolted onto Scene by the add-on:

> `    @classmethod`
> `    def register(cls):`
> `        bpy.types.Scene.cycles = PointerProperty(`
> `            name="Cycles Render Settings",`
> `            description="Cycles render settings",`
> `            type=cls,`
> `        )`

(`Blender Sources/cycles_5.0/properties.py`, lines 1101–1107.) And its panels are gated on the engine:

> `    COMPAT_ENGINES = {'CYCLES'}`
> `​`
> `    @classmethod`
> `    def poll(cls, context):`
> `        return context.engine in cls.COMPAT_ENGINES`

(`Blender Sources/cycles_5.0/ui.py`, lines 64–68.)

One trap worth naming, because it cuts the other way: a setting can appear inside an engine's panel and still be a shared one. Cycles' Performance page documents "Threads Mode" and "Threads" (https://docs.blender.org/manual/en/5.0/render/cycles/render_settings/performance.html), but `threads` and `threads_mode` are properties of the shared `RenderSettings` block, not of `CyclesRenderSettings` (https://docs.blender.org/api/5.0/bpy.types.RenderSettings.html; they are absent from `cycles_5.0/properties.py`). Likewise `use_persistent_data` sits on `RenderSettings`.

**Not covered.** This gives the rule and the grouping, not an exhaustive list of every property on either side of the line. It does not say which shared settings an engine may quietly ignore — for example whether Workbench honours everything in the Output tab. It says nothing about third-party engines' own settings beyond the fact that they have some.

**Outside the interface.** The sources document the properties and the Properties-editor layout. The quoted material does not state whether every block is saved in the file or whether the grouping changes outside the interface. **Observed 2026-08-05 (hand tests 6 and 7, under questions 4 and 9):** all four blocks — `cycles`, `eevee`, `display` and `render` — were reachable and held their own saved values in a Blender started from a terminal with no window, whichever engine was selected.

**Operating system.** The sources do not say the grouping differs by operating system, and nothing on the pages above is marked platform-specific. The test below was run on macOS only. **Observed 2026-08-06:** the same test was run again on a rented Linux machine and came back the same, so this holds on macOS and on Linux. Windows stays untested.

### Observed — hand test 13 (2026-08-05)

Run from a terminal on the Creator's own computer — a MacBook Pro with an M1 Pro chip, macOS 26.5.2 — using Blender 5.0.1 at `/Applications/Blender 5.0.1.app/Contents/MacOS/Blender`, with the `BLENDER_USER_RESOURCES` setting pointed at a fresh empty folder. Label: **observed**.

Every property of `scene.eevee` and `scene.display` was listed from factory startup. `scene.eevee` holds 52 properties; `scene.display` holds 9, listed in full under question 2. Searching both for `device`, `cpu` and `gpu` returned only two lines, both about memory size rather than a choice:

```
eevee gi_irradiance_pool_size | Size of the irradiance pool, a bigger pool size allows for more irradiance grid in the scene but might not fit into GPU memory and decrease performance
eevee shadow_pool_size | Size of the shadow pool, a bigger pool size allows for more shadows in the scene but might not fit into GPU memory
```

### Observed — hand test 23, run by the Creator (2026-08-05)

Run by the Creator in Blender's own window, on their own computer — a MacBook Pro with an M1 Pro chip, macOS 26.5.2 — on 2026-08-05. That settles macOS and leaves Windows and Linux untested. Label: **observed**.

They opened Render Properties and clicked the **Render Engine** dropdown so its list was showing, and photographed it. The open list holds exactly three entries, in this order:

```
EEVEE
Workbench
Cycles
```

with Cycles the one selected. The row immediately under the dropdown in this version is **Device**.

**One shot was supplied for 5.0, 5.1 and 5.2 together rather than one each**, and it is that shot this finding rests on. It was taken on this machine on 2026-08-05, and the Creator gave it as the answer for all three of those versions.

So the control offers human labels, not the identifiers `-E` takes (hand test 1, under question 13), and the three engines the manual names are the three the control offers.

### Observed on the rented Linux machine — Linux hand tests 6, 14 and 25 (2026-08-06)

Run over SSH from a terminal on the rented Linux machine, using Blender 5.0.1 at `/opt/blender/blender-5.0.1-linux-x64/blender`, with `BLENDER_USER_RESOURCES` pointed at `/root/smoozy/Settings/res-5.0`, a folder of this topic's own that already existed. That folder was checked before use and reported its own config directory and Blender's eight default add-ons and nothing else. The machine is a container sharing someone else's hardware, and up to four topics were being worked on it at once, so nothing below claims anything about how long a run took or how much memory was free. Label: **observed**.

These are a second machine's answers to the tests the Creator's Mac answered on 2026-08-05. The two are never merged: where they agree it says so, and where they differ it says that plainly. The full list is `Audits/Blender Research/Settings/Hand tests — Linux.md`.

**Linux hand tests 6 and 14 — are all the engines' blocks there at once, and what is on them?** A file was saved with Cycles samples 123, EEVEE render samples 45, Workbench final anti-aliasing `32` and the Grease Pencil final-render count 3, then read back in a fresh Blender started from a terminal with no window:

```
BACK as-loaded engine= BLENDER_EEVEE cycles= 123 eevee= 45 workbench= 32 gpencil= 3
```

All three engines' blocks were reachable and each held its own saved value, whichever engine was selected. Listing every property gave 52 on the EEVEE block and 9 on the Workbench block. **The two machines agree.**

**Linux hand test 25 — the Render Properties tab, photographed in Blender's own window on this machine.** Blender was started with a real window on the machine's own display, the engine set to Workbench and the Properties editor pointed at Render, and the window photographed with Blender's own screenshot command. Blender 5.0's tab lists **Curves** above Simplify. The tab carries the shared settings under whichever engine is chosen, exactly as the Creator's photographs of the same tab on the Mac showed. **The two machines agree.**

---

## 4. Does a file carry a separate sample count for each render engine?

**Label: documented.** The sources document a distinct sampling setting for each named engine. They do not directly state what happens to another engine's setting when the engine is switched.

Cycles:

> "Render (Max) Samples
>
> The number of paths to trace per pixel in the final render.
> A higher number results in a cleaner image at the cost of a longer render time."

(https://docs.blender.org/manual/en/5.0/render/cycles/render_settings/sampling.html)

EEVEE, under its own Render heading:

> "Samples
>
> The number of samples to use in the final render."

(https://docs.blender.org/manual/en/5.0/render/eevee/render_settings/sampling.html)

Workbench, which does not have a count as such but a choice of anti-aliasing method — and the manual states plainly where that choice is kept:

> "The setting for the 3D Viewport is a user preference to specify the anti-aliasing method
> that runs best on the used system. The setting for viewport rendering
> and final rendering is saved per scene."

(https://docs.blender.org/manual/en/5.0/render/workbench/sampling.html)

In the API these are three separate properties on three separate blocks of the same scene:

> "taa_render_samples
>
> Number of samples per pixel for rendering
>
> Type:
>
> int in [1, inf], default 64"

(EEVEE — https://docs.blender.org/api/5.0/bpy.types.SceneEEVEE.html)

> "render_aa
>
> Method of anti-aliasing when rendering final image
>
> OFF
> No Anti-Aliasing – Scene will be rendering without any anti-aliasing."

(Workbench — https://docs.blender.org/api/5.0/bpy.types.SceneDisplay.html; its choices are `OFF`, `FXAA`, `5`, `8`, `11`, `16`, `32`.)

**Label: observed**, for Cycles', which has no API page because Cycles is an add-on:

> `    samples: IntProperty(`
> `        name="Samples",`
> `        description="Number of samples to render for each pixel",`
> `        min=1, max=(1 << 24),`
> `        default=4096,`
> `    )`

(`Blender Sources/cycles_5.0/properties.py`, lines 479–484, in `class CyclesRenderSettings`, which is attached to Scene as `scene.cycles` — see question 3.)

So: Cycles 4096 by default, EEVEE 64 by default, Workbench a method rather than a count. The sources document separate properties, but do not state whether changing engine overwrites another engine's figure.

There is also a per-view-layer figure, separate again from the scene one:

> "Samples
>
> Allows the view layer to use a different number of samples than the main scene.
> This is controlled by the Layer Samples
> when using the Cycles renderer. When using EEVEE, the Samples override is always used."

(https://docs.blender.org/manual/en/5.0/render/layers/override.html) In the API: `ViewLayer.samples`, "Override number of render samples for this view layer, 0 will use the scene setting" (https://docs.blender.org/api/5.0/bpy.types.ViewLayer.html). The 5.0 release notes record this as new: "View Layer Overrides (Material, World, Sample) are now supported. Older files using this feature will render differently." (https://developer.blender.org/docs/release_notes/5.0/eevee/)

**Not covered.** This does not settle whether a third-party engine keeps its own count (the sources only say each renderer has its own settings). It does not settle whether the view-layer override is one figure shared by all engines or one per engine — the override page describes a single Samples override interpreted differently by Cycles and EEVEE, which suggests one figure, but does not say so outright. It says nothing about how the two viewport sample counts (`preview_samples`, `taa_samples`) relate to the render ones beyond their names.

**Outside the interface.** No dedicated command-line option sets these values (question 1). The Workbench source says its viewport-rendering and final-rendering setting is saved per scene; the quoted material does not say the same for every Cycles and EEVEE value above. **Observed 2026-08-05 (hand test 6, below):** a Blender started from a terminal with no window read all three counts back from the file at the values they were saved with.

**Operating system.** The sources do not say any of these differ by operating system. The test below was run on macOS only. **Observed 2026-08-06:** the same test was run again on a rented Linux machine and came back the same, value for value, so this holds on macOS and on Linux. Windows stays untested.

### Observed — hand test 6 (2026-08-05)

Run from a terminal on the Creator's own computer — a MacBook Pro with an M1 Pro chip, macOS 26.5.2, Apple's own graphics and no other graphics card — using Blender 5.0.1 at `/Applications/Blender 5.0.1.app/Contents/MacOS/Blender`, with the `BLENDER_USER_RESOURCES` setting pointed at a fresh empty folder. Label: **observed**.

A file was saved from factory startup with Cycles samples 123, EEVEE render samples 45, Workbench final anti-aliasing `32` and the Grease Pencil final-render sample count 3. Blender was closed and a fresh Blender opened the file, reading all four values back and switching the engine between readings:

```
MADE cycles=123 eevee=45 workbench=32 gpencil=3
BACK as-saved               engine=BLENDER_EEVEE        cycles=123 eevee=45 workbench=32 gpencil=3
BACK after switch to CYCLES engine=CYCLES               cycles=123 eevee=45 workbench=32 gpencil=3
BACK after switch to BLENDER_WORKBENCH engine=BLENDER_WORKBENCH    cycles=123 eevee=45 workbench=32 gpencil=3
BACK after switch to EEVEE  engine=BLENDER_EEVEE        cycles=123 eevee=45 workbench=32 gpencil=3
```

**This closes the two gaps named above.** Every one of the three counts is carried by the file and survives a save and a reload, and switching the engine changes none of them. The Grease Pencil count is one shared value, reading 3 under all three engines.

### Observed — hand test 26, run by the Creator (2026-08-05)

Run by the Creator in Blender 5.0's own window, on their own computer — a MacBook Pro with an M1 Pro chip, macOS 26.5.2 — on 2026-08-05. That settles macOS and leaves Windows and Linux untested. Label: **observed**.

Photographed with the Render Engine set to **EEVEE**, the Sampling panel shows a **Viewport ▸ Samples** of 16 and a separate **Render ▸ Samples** of 64 — two counts, not one. Photographed with the Render Engine set to **Workbench**, the whole Sampling panel is **Render — 8 Samples** and **Viewport — Single Pass Anti-Aliasing**, both menus rather than number fields. Cycles' own panel (hand test 24, under question 5) shows **Max Samples 4096** as a number field.

So the three engines' final-render counts are three separate controls in three separate panels, and Workbench's is a menu of fixed choices where the other two are numbers — which is what the sources describe, seen in the program.

### Observed on the rented Linux machine — Linux hand tests 6 and 14 (2026-08-06)

Run over SSH from a terminal on the rented Linux machine, using Blender 5.0.1 at `/opt/blender/blender-5.0.1-linux-x64/blender`, with `BLENDER_USER_RESOURCES` pointed at `/root/smoozy/Settings/res-5.0`, a folder of this topic's own that already existed. That folder was checked before use and reported its own config directory and Blender's eight default add-ons and nothing else. The machine is a container sharing someone else's hardware, and up to four topics were being worked on it at once, so nothing below claims anything about how long a run took or how much memory was free. Label: **observed**.

These are a second machine's answers to the tests the Creator's Mac answered on 2026-08-05. The two are never merged: where they agree it says so, and where they differ it says that plainly. The full list is `Audits/Blender Research/Settings/Hand tests — Linux.md`.

**Linux hand test 6 — does one file hold all three counts at once, and is the Grease Pencil count shared?** A file was saved with Cycles samples 123, EEVEE render samples 45, Workbench final anti-aliasing `32` and the Grease Pencil final-render count 3, closed, and read back in a fresh Blender; then the engine was switched three times and the same four values read after each switch, in one run:

```
BACK as-loaded engine= BLENDER_EEVEE cycles= 123 eevee= 45 workbench= 32 gpencil= 3
BACK after-switch-to-CYCLES engine= CYCLES cycles= 123 eevee= 45 workbench= 32 gpencil= 3
BACK after-switch-to-BLENDER_WORKBENCH engine= BLENDER_WORKBENCH cycles= 123 eevee= 45 workbench= 32 gpencil= 3
BACK after-switch-to-BLENDER_EEVEE engine= BLENDER_EEVEE cycles= 123 eevee= 45 workbench= 32 gpencil= 3
```

So one file carries three separate counts, they survive a save and a reload, switching the engine disturbs none of them, and the Grease Pencil count reads the same under every engine. **The two machines agree**, value for value.

**Linux hand test 14 — what each engine's own block actually holds.** The EEVEE block holds **52** properties on this build and the Workbench block holds **9** — the same two counts the Mac's run gave for this version. **The two machines agree.**

---

## 5. Can a sample count change from frame to frame?

**Label: inferred.** The sources never say "the sample count can be animated" in those words. Here is the chain, and each link is quoted.

Link one — properties in general can be animated, and the interface marks the ones that can:

> "Decorators are small buttons that appear to the right of other buttons and show the state of the property.
> Decorators may appear next to number fields, menus,
> and checkboxes to indicate the property can be animated."

> "Clicking on the decorator dot icon will add a Keyframe to that property."

(https://docs.blender.org/manual/en/5.0/interface/controls/buttons/decorators.html — read 2026-07-30.)

And a keyframe is exactly a per-frame value:

> "A Keyframe is simply a marker of time which stores the value of a property."

(https://docs.blender.org/manual/en/5.0/animation/keyframes/introduction.html)

Link two — a property can also be driven, i.e. computed:

> "Drivers are a way to control values of properties by means of a function,
> or a mathematical expression."

(https://docs.blender.org/manual/en/5.0/animation/drivers/introduction.html) A driver expression may use the current frame directly: the Simple Expressions subset lists under "Globals" the single entry "frame" (https://docs.blender.org/manual/en/5.0/animation/drivers/drivers_panel.html).

Link three — Cycles' sample count is an ordinary property of this kind. **Observed:** it is declared as `samples: IntProperty(...)` with no `options` argument (`Blender Sources/cycles_5.0/properties.py`, lines 479–484, quoted in question 4). **Documented:** the API gives the default for that argument as animatable —

> "bpy.props.IntProperty(*, name='', description='', translation_context='*', default=0, min=-2**31, max=2**31 - 1, soft_min=-2**31, soft_max=2**31 - 1, step=1, options={'ANIMATABLE'}, ...)"

— and, of add-on properties in general:

> "These properties can be animated, accessed by the user interface and Python
> like Blender's existing properties."

(https://docs.blender.org/api/5.0/bpy.props.html)

**The step I am taking:** from "the default for `options` is `{'ANIMATABLE'}`", "Cycles declares `samples` without overriding `options`", and "animated properties take a value per frame", I conclude the Cycles sample count can hold a different value on different frames. The sources do not state the conclusion; a reader has to supply the joins. Hence inferred, not documented.

EEVEE's `taa_render_samples` is a built-in property; its API page (https://docs.blender.org/api/5.0/bpy.types.SceneEEVEE.html) gives its type and default but says nothing about animatability either way, so I cannot make the same chain for it. **Not in the sources** for EEVEE and for Workbench's `render_aa`.

A separate, weaker route that *is* documented: a script can change scene data as each frame comes up.

> "bpy.app.handlers.frame_change_pre
>
> Called after frame change for playback and rendering, before any data is evaluated for the new frame. This makes it possible to change data and relations (for example swap an object to another mesh) for the new frame."

(https://docs.blender.org/api/5.0/bpy.app.handlers.html) That documents a hook that fires per frame during rendering, and documents that data may be changed in it. Whether the render engine picks up a changed sample count from that hook is **not in the sources**.

Where I looked, for the "not in the sources" parts: searched manual, API and release notes for "animatable", "is_animatable", "ANIMATABLE", "keyframe" together with "samples", "driver" together with "samples", "animate render settings", "per frame". The only per-property animatability statement in the whole manual is the Decorators page; no page names the sample count as animatable or as not animatable.

**Not covered.** This does not settle whether an animated sample count is actually re-read by Cycles between frames of a single `--render-anim` run, which is the thing that matters in practice and which no source I was given addresses. It does not settle it for EEVEE or Workbench at all. It says nothing about the per-view-layer Samples override (question 4) being animatable.

**Outside the interface.** The handler quote establishes that a handler runs "for playback and rendering". It does not state whether saved keyframes and drivers are evaluated when a background render starts. Question 11 documents the permission rules for Python drivers. The sources say the interface must sometimes be locked when handlers alter data during a render ("lock the interface (Render → Lock Interface or bpy.types.RenderSettings.use_lock_interface) before starting a render"), which is an interface-side concern. **Observed 2026-08-05 (hand test 9, below):** saved keyframes are evaluated in a background render, and the new value is used for that frame.

**Operating system.** The sources do not say any of this differs by operating system. The test below was run on macOS only. **Observed 2026-08-06:** the same tests were run again on a rented Linux machine and came back the same, so this holds on macOS and on Linux. Windows stays untested.

### Observed — hand test 9 (2026-08-05)

Run from a terminal on the Creator's own computer — a MacBook Pro with an M1 Pro chip, macOS 26.5.2 — using Blender 5.0.1 at `/Applications/Blender 5.0.1.app/Contents/MacOS/Blender`, with the `BLENDER_USER_RESOURCES` setting pointed at a fresh empty folder. Label: **observed**.

From factory startup, `keyframe_insert` was called on each engine's count and on a view layer's `use`:

```
KEY cycles.samples               -> True
KEY eevee.taa_render_samples     -> True
KEY display.render_aa            -> ERROR bpy_struct.keyframe_insert() property "render_aa" not animatable
KEY view_layer.use               -> True
```

**This closes the "not in the sources" answer for EEVEE and Workbench:** EEVEE's count can be keyframed and Workbench's anti-aliasing choice cannot.

A file was then saved with Cycles, 64 by 64, adaptive sampling off, denoising off, and Max Samples keyframed to 4 at frame 1 and 512 at frame 2. Rendering it with `blender -b test.blend -o out/f_ -F PNG -s 1 -e 2 -a` reported these maximum sample counts across the two frames: `4` and `512`. **This closes the crux the finding named:** Cycles does re-read the sample count at each frame of a running animation render.

### Observed — hand test 24, run by the Creator (2026-08-05)

Run by the Creator in Blender 5.0's own window, on their own computer — a MacBook Pro with an M1 Pro chip, macOS 26.5.2 — on 2026-08-05. That settles macOS and leaves Windows and Linux untested. Label: **observed**.

With the Render Engine set to **Cycles**, they opened Render Properties ▸ Sampling ▸ Render, right-clicked the **Max Samples** field, and photographed the open menu. It is headed **Samples** and lists:

```
Insert Keyframe                    I
Add Driver                         ⌘D
Open Drivers Editor
Add to Keying Set
Remove from Keying Set             ⌥K
Reset to Default Value             Backspace
Copy to Selected                   (greyed out)
Copy Data Path                     ⇧⌘C
Copy Full Data Path                ⇧⌥⌘C
Copy as New Driver
Online Manual                      F1
```

A second shot shows the panel with no menu open: **Noise Threshold** (ticked, 0.0100), **Max Samples** 4096, **Min Samples** 0, **Time Limit** 0 s, then the **Denoise** panel. No dot appears to the right of any of those four rows.

So the interface offers **Insert Keyframe** on the sample count directly, with `I` as its shortcut, and offers **Add Driver** and **Copy as New Driver** beside it — while the panel itself carries no decorator dot to click. That is the interface half of the animatable-property chain the finding above reasons through, seen rather than reasoned.

### Observed on the rented Linux machine — Linux hand tests 11 and 12 (2026-08-06)

Run over SSH from a terminal on the rented Linux machine, using Blender 5.0.1 at `/opt/blender/blender-5.0.1-linux-x64/blender`, with `BLENDER_USER_RESOURCES` pointed at `/root/smoozy/Settings/res-5.0`, a folder of this topic's own that already existed. That folder was checked before use and reported its own config directory and Blender's eight default add-ons and nothing else. The machine is a container sharing someone else's hardware, and up to four topics were being worked on it at once, so nothing below claims anything about how long a run took or how much memory was free. Label: **observed**.

These are a second machine's answers to the tests the Creator's Mac answered on 2026-08-05. The two are never merged: where they agree it says so, and where they differ it says that plainly. The full list is `Audits/Blender Research/Settings/Hand tests — Linux.md`.

**Linux hand test 11 — which counts take a keyframe, and does a render honour one frame by frame?** Asked of a factory-startup Blender started from a terminal:

```
KEY samples True
KEY taa_render_samples True
KEY render_aa ERROR bpy_struct.keyframe_insert() property "render_aa" not animatable
KEY use -> True
KEY use_single_layer ERROR bpy_struct.keyframe_insert() property "use_single_layer" not animatable
```

So Cycles' count and EEVEE's count take a keyframe and Workbench's anti-aliasing choice does not, with the same refusal the Mac got.

Then a Cycles file at 64 by 64, denoising off, adaptive sampling off, with Max Samples keyframed to 4 at frame 1 and 512 at frame 2, rendered as an animation from a terminal with `-s 1 -e 2 -a -- --cycles-print-stats`. The build step confirmed the file first — `BUILD samples at f1 = 4` and `BUILD samples at f2 = 512`. The render reported:

```
Fra: 1 … Sample 0/4
Fra: 1 … Sample 4/4
Saved: '…/out/f_0001.png'
Fra: 2 … Sample 0/512
Fra: 2 … Sample 512/512
Saved: '…/out/f_0002.png'
```

Four samples at frame 1 and 512 at frame 2, exit code 0. So the count is read again at each frame rather than once when the job starts. **The two machines agree.**

**Linux hand test 12 — the same for a view layer's switch.** A file with `Layer2`'s switch keyframed on at frame 1 and off at frame 2 wrote a first frame holding both layers and a second holding one. **The two machines agree.**

---

## 6. Does every render engine render on a computer with no screen attached?

**Label: documented.** No — there is at least one documented exception, and one engine is documented as requiring a graphics card at all.

Blender generally does render without a display:

> "One advantage of using the command line is that we do not need a graphical display
> (no need for X server on Linux for example)
> and consequently we can render via a remote shell (typically SSH)."

(https://docs.blender.org/manual/en/5.0/advanced/command_line/render.html — read 2026-07-30.)

But EEVEE carries two limitations that bear directly on this:

> "CPU Rendering
>
> Being a rasterization engine, EEVEE only uses the power of
> the GPU to render.
> There is no plan to support CPU (software) rendering
> as it would be very inefficient. CPU power is still needed to handle high scene complexity
> as the geometry must be prepared by the CPU before rendering each frame."

> "Headless Rendering
>
> Headless rendering is not supported on headless Windows systems."

(https://docs.blender.org/manual/en/5.0/render/eevee/limitations/limitations.html — "Headless Rendering" is also listed as a section in the Limitations index at https://docs.blender.org/manual/en/5.0/render/eevee/limitations/index.html.)

So, read plainly: EEVEE does not render on a headless Windows machine. The manual does not say EEVEE fails on headless Linux or macOS; it names Windows only.

For Cycles: **not in the sources** whether it renders headless. Cycles' own files name a CPU device — "CPU: Use CPU for rendering" (`Blender Sources/cycles_5.0/properties.py`, lines 30–34, **observed**) — but that does not establish headless rendering. I found no headless limitation for Cycles anywhere.

For Workbench: **not in the sources**. Searched the whole manual, API and release notes for "headless", "no display", "X server", "offscreen", "EGL", "surfaceless", "background mode", and searched all Workbench pages for "GPU", "graphics card", "OpenGL", "Vulkan", "hardware". Workbench's own pages (https://docs.blender.org/manual/en/5.0/render/workbench/introduction.html) describe it as "a render engine optimized for fast rendering during modeling and animation preview" and say nothing about what hardware or display it needs.

**Not covered.** This does not settle what "headless Windows system" means precisely (no monitor? no logged-in session? a service account?), nor what happens when you try — an error, a blank image, a crash. It does not settle EEVEE on headless Linux or macOS. It says nothing about third-party engines. It says nothing about virtual displays or remote sessions.

**Outside the interface.** This question is entirely about the outside-the-interface case, and the answer above is drawn from the pages that describe it. The command-line page is explicit that no graphical display is needed in general; EEVEE's limitation is the exception.

**Operating system.** This is the answer's whole shape: the one documented headless failure is named for Windows and only Windows. The general "no need for X server" statement is named for Linux. macOS is not mentioned in either. **Observed 2026-08-05 (hand test 14, below):** all three engines render with no window on macOS. Windows and Linux were not tested — that test was run on the Creator's one MacBook Pro — so the EEVEE limitation named for Windows stays untested, and so does the screenless case on any system. **Observed 2026-08-06:** on a rented Linux machine with one graphics card, all three engines rendered with no window and again with no display reachable at all, EEVEE included — but that machine had a display server running and its runs were only cut off from it. **Observed 2026-08-06 on a second rented Linux machine that has no display server at all** — two NVIDIA GeForce RTX 4090s, driver 580.95.05, sixty-one cores, over SSH — every engine this version has rendered there, on the cards and on the processor, EEVEE included. So **the screenless case is now answered for Linux**, in the sense of a machine with no display server. The headless *Windows* case the manual names stays untested, no Windows machine having run it, and so does a machine with no graphics hardware at all.

### Observed — hand test 14 (2026-08-05)

Run from a terminal on the Creator's own computer — a MacBook Pro with an M1 Pro chip, macOS 26.5.2, Apple's own graphics and no other graphics card — using Blender 5.0.1 at `/Applications/Blender 5.0.1.app/Contents/MacOS/Blender`, with the `BLENDER_USER_RESOURCES` setting pointed at a fresh empty folder. Label: **observed**.

A file was saved from factory startup at 64 by 64, Cycles samples 4, EEVEE render samples 4, and rendered three times, once per engine, each with `-b` so no window opened:

```
CYCLES               exit=0  image=64x64
BLENDER_EEVEE        exit=0  image=64x64
BLENDER_WORKBENCH    exit=0  image=64x64
```

No error was printed by any of the three. **This closes the "not in the sources" answer for Workbench and for Cycles, for macOS with no window.**

**What this does not settle.** This machine has a screen attached. Running with no window is not the same as running on a machine with no screen, and the one hard statement in the sources is about a headless *Windows* machine. That test needs a machine with no display reached over a remote shell, and a Windows machine — neither of which the Creator has. It stays unanswered.

### Observed on the rented Linux machine — Linux hand tests 15 and 16 (2026-08-06)

Run over SSH from a terminal on the rented Linux machine, using Blender 5.0.1 at `/opt/blender/blender-5.0.1-linux-x64/blender`, with `BLENDER_USER_RESOURCES` pointed at `/root/smoozy/Settings/res-5.0`, a folder of this topic's own that already existed. That folder was checked before use and reported its own config directory and Blender's eight default add-ons and nothing else. The machine is a container sharing someone else's hardware, and up to four topics were being worked on it at once, so nothing below claims anything about how long a run took or how much memory was free. Label: **observed**.

These are a second machine's answers to the tests the Creator's Mac answered on 2026-08-05. The two are never merged: where they agree it says so, and where they differ it says that plainly. The full list is `Audits/Blender Research/Settings/Hand tests — Linux.md`.

**Linux hand test 15 — each engine with no window, on a machine whose display is reachable.** A file with the default cube, camera and light at 64 by 64, Cycles samples 4, EEVEE render samples 4, rendered three times with `-b` so no window opened:

```
CYCLES              exit=0  image=64x64
BLENDER_EEVEE         exit=0  image=64x64
BLENDER_WORKBENCH   exit=0  image=64x64
```

No error was printed by any of the three. **The two machines agree** — this is the same result the Mac gave on macOS.

**Linux hand test 16 — each engine with no display at all.** The same three renders were run again with the display taken out of the environment entirely (`env -u DISPLAY -u XAUTHORITY`), so nothing was left for Blender to reach a display through:

```
CYCLES              exit=0  image=64x64
BLENDER_EEVEE         exit=0  image=64x64
BLENDER_WORKBENCH   exit=0  image=64x64
```

All three engines rendered and wrote their image, **EEVEE included**. No error, no warning, and nothing about a display in any of the three runs.

**And what Blender says when asked for a window with no display.** The same removal, but without `-b`, so Blender tried to open its interface. It printed:

```
00:00.024  ghost.system     | Failed to initialize display for back-end(s): ['WAYLAND', 'X11']
                            |   'WAYLAND': unable to connect to display!
                            |   'X11': unable to open a display!
                            | 
00:00.024  ghost.system     | ERROR Unable to initialize GHOST, exiting!
```

and exited with code 1.

**What this settles and what it does not.** It settles that a run which cannot reach a display at all renders on all three engines on this Linux machine, and that Blender says so plainly and stops when a window is asked for instead. It does **not** settle a machine with no screen physically attached: this box has a display running and the run was only cut off from it, which is a narrower thing. And it does not touch the manual's one hard statement, which names a headless *Windows* machine — no Windows machine has run it. **Untried rather than untestable: a Windows machine can be rented, and every one this project has read reported a Server edition — see the equipment section of `Preparation/Blender Research/Method.md`.** So this question is now answered for Linux in the sense of "no display reachable", and stays open for "no screen attached" and for Windows.

### Observed on a machine with no screen — the two-card rented Linux machine (2026-08-06)

Run over SSH on a **second rented Linux machine — Ubuntu 22.04.5 LTS, two NVIDIA GeForce RTX 4090s of 24 GB each on driver 580.95.05, an AMD EPYC 7B13 with sixty-one cores, 98 GB of memory, no display server at all** — using Blender at `/opt/blender/blender-5.0.1-linux-x64/blender`, which `--version` reports as `Blender 5.0.1 (hash a3db93c5b259 built 2025-12-16 01:30:59)`, with `BLENDER_USER_RESOURCES` pointed at `/root/smoozy/no screen/res-5.0`, a folder of its own that already existed and was empty. Nothing else was running on the machine. Label: **observed**. The full list is `Audits/Blender Research/Settings/Hand tests — two cards and no screen.md`.

**This is the question item A of the Linux list could not answer, and it is a different claim from the block above.** The one-card machine had a display server running and its runs were cut off from it by removing `DISPLAY` from the environment. This machine has none at all, and nothing was installed or started to change that:

```
DISPLAY=[] WAYLAND_DISPLAY=[] XDG_SESSION_TYPE=[tty]
--- /tmp/.X11-unix:
total 8
drwxrwxrwt  2 root root 4096 Aug  6 10:11 .
drwxrwxrwt 11 root root 4096 Aug  6 11:38 ..
--- X server processes:
(none)
--- /dev/dri:
crw-rw----  1 root video  226,   0 Aug  6 10:11 card0
crw-rw----  1 root video  226,   1 Aug  6 10:11 card1
crw-rw----  1 root render 226, 128 Aug  6 10:11 renderD128
crw-rw----  1 root render 226, 129 Aug  6 10:11 renderD129
```

The X socket directory is empty, no X or Wayland process is running, and the session type is `tty`. The cards are there as kernel devices; there is simply no display server on top of them.

**Every engine rendered, on the cards and on the processor.** A file with the default cube, camera and light at 64 by 64, Cycles samples 4, EEVEE render samples 4, rendered with `-b` and `-E <engine>`, once with `-- --cycles-device OPTIX` and once with `-- --cycles-device CPU`, each under `timeout 120`:

```
GPU  CYCLES exit=0  png=4992
GPU  BLENDER_EEVEE_NEXT exit=0  png=3358
GPU  BLENDER_EEVEE exit=0  png=3358
GPU  BLENDER_WORKBENCH exit=0  png=3596
CPU  CYCLES exit=0  png=4994
CPU  BLENDER_EEVEE_NEXT exit=0  png=3358
CPU  BLENDER_EEVEE exit=0  png=3358
CPU  BLENDER_WORKBENCH exit=0  png=3596
```

All of them rendered and wrote their image, **EEVEE included**, with no error and nothing about a display in any of them. This version accepts both `BLENDER_EEVEE` and `BLENDER_EEVEE_NEXT` and gives the same file size for each; its own `-E help` lists `BLENDER_EEVEE`, `BLENDER_WORKBENCH` and `CYCLES`, exit 0.

The same runs repeated with `--debug-cycles` showed the work really was where it was asked for:

```
on the card:      Path tracing on: NVIDIA GeForce RTX 4090 (OptiX) [CUDA_NVIDIA GeForce RTX 4090_0000:00:0a_OptiX]
                                   NVIDIA GeForce RTX 4090 (OptiX) [CUDA_NVIDIA GeForce RTX 4090_0000:00:07_OptiX]
on the processor: Path tracing on: AMD EPYC 7B13 64-Core Processor (CPU) (61 threads) [CPU]
```

Both cards are named in the card run. Nothing here says how the work was divided between them, or which cards were switched on in the preferences, and nothing is read into it. *(Restored by the two-card citation check, 2026-08-06: this block had been shortened to one card with an ellipsis, while the record it comes from shows two.)*

**And what Blender says when asked for a window on a machine with no display server.** The same command without `-b`, given `--python-expr "import bpy; print('RAN WITH A WINDOW')"` under `timeout 120`. `RAN WITH A WINDOW` was never printed. It printed:

```
00:00.036  ghost.system     | Failed to initialize display for back-end(s): ['WAYLAND', 'X11']
                            |   'WAYLAND': unable to connect to display!
                            |   'X11': unable to open a display!
                            | 
00:00.036  ghost.system     | ERROR Unable to initialize GHOST, exiting!
Error: Not freed memory blocks: 4556, total unfreed memory 1.272339 MB
```

on **standard output**, and exited **1**. That run also wrote to standard error, fourteen times over:

```
Freeing memory after the leak detector has run. This can happen when using static variables in C++ that are defined outside of functions. To fix this error, use the 'construct on first use' idiom.
```

Word for word what the one-card machine printed when cut off from its display, with the same exit status, plus the memory-leak line that machine never showed.

**What this settles.** **On Linux, every render engine this version has renders on a computer with no display server at all** — not merely a run cut off from a display. That is the half of this question item A of the Linux list left open, and it is now answered for Linux. **It does not settle a machine with no screen physically attached in the sense of no graphics hardware**: the cards are present, they are simply not driving a display. And it does not touch the manual's one hard statement, which names a headless *Windows* machine — no Windows machine has run it.

---

## 7. Does a scene set up for one render engine come out the same in another?

**Label: documented.** No. The manual says the inputs are largely shared but the results are not.

On sharing:

> "What the render looks like is defined by cameras,
> lights and materials.
> These are shared between EEVEE and Cycles, however some features are only supported in one or the other."

(https://docs.blender.org/manual/en/5.0/render/introduction.html — read 2026-07-30.)

On the difference:

> "EEVEE materials are created using the same shader nodes as Cycles, making it easy to render existing
> scenes. For Cycles users, this makes EEVEE work great for previewing materials in realtime."

> "EEVEE is based on rasterization and is not a path tracer.
> Instead of computing each ray of light, rasterization determines what surface is visible from the camera.
> It then estimates the way light interacts with these surfaces and materials using numerous algorithms.
> While EEVEE is designed to use PBR principles,
> it is not perfect and Cycles will always provide more physically accurate renders.
> For these reasons, EEVEE has a set of limitations."

(https://docs.blender.org/manual/en/5.0/render/eevee/introduction.html)

The limitations page then lists them concretely. A few, quoted:

> "Only 14 attributes from Geometry Nodes are supported in a material"

> "Only perspective and orthographic projections are currently supported."

> "Lights can only have one color and do not support light node trees."

> "Unlike in Cycles, the Size
> of spot lights does not change the softness of the cone."

> "Only single scattering is supported."

> "Volumetrics are rendered only for the camera "rays". They don't appear in reflections/refractions and probes."

(https://docs.blender.org/manual/en/5.0/render/eevee/limitations/limitations.html — the page's own framing: "Here is a rather exhaustive list of all the limitations you can expect while working with EEVEE.")

A scene can even be authored to deliberately differ per engine. The material output node carries a target:

> "target
>
> Which renderer and viewport shading types to use the shaders for
>
> ALL
> All – Use shaders for all renderers and viewports, unless there exists a more specific output.
>
> EEVEE
> EEVEE – Use shaders for EEVEE renderer.
>
> CYCLES
> Cycles – Use shaders for Cycles renderer."

(https://docs.blender.org/api/5.0/bpy.types.ShaderNodeOutputMaterial.html; the same enum appears on the world, light and line-style output nodes.)

Workbench is further away again: "Unlike other render engines such as EEVEE or Cycles, the Workbench engine does not use shader nodes." and "It is not intended to be a render engine that will render final images for a project." (https://docs.blender.org/manual/en/5.0/render/workbench/introduction.html)

The 5.0 release notes add a specific case where the two engines are being brought closer, which is itself evidence that they differ: "When using displacement, the Normal Map node now uses the undisplaced normal and tangent. This matches EEVEE behavior" (https://developer.blender.org/docs/release_notes/5.0/cycles/), and "The default material when the object has no material assigned now matches Blender and EEVEE, adding a specular component." (same page).

**Not covered.** This says the results differ; it does not quantify how much, or list every difference, or say which differences are visible at a given setting. It does not cover third-party engines. It does not cover whether settings survive the switch; that is outside this question, which asks about the picture.

**Outside the interface.** Nothing here is interface-dependent. The engine is chosen by a scene setting or by `-E` on the command line, and the differences above are differences in what the engines compute.

**Operating system.** The sources do not say these differences vary by operating system.

---

## 8. Can one scene be set up to take in another scene's rendered result, so that rendering the first renders the second?

**Label: documented.** Yes, in two separate ways.

**By way of the compositor.** The Render Layers node names a scene:

> "Renders a View Layer and reads its
> Passes into the compositing node graph."

> "Scene
>
> The scene for which to render a view layer.
>
> View Layer
>
> The view layer to render. The button next to the dropdown re-renders it immediately."

With a limit stated on the same page:

> "Hint
>
> To use the compositing output from another scene rather than its "raw" render output,
> first render that scene into a series of multi-layered images (using e.g. the OpenEXR format),
> then load those images into the Compositor of the current scene using the
> Image Node."

(https://docs.blender.org/manual/en/5.0/compositing/types/input/scene/render_layers.html — read 2026-07-30.) So the other scene's *raw* render comes in directly; its *composited* output does not, and must go through files.

**By way of the sequencer.** A Scene strip:

> "Scene strips are a way to insert the render output of another scene into your sequence.
> Instead of rendering out a video, then inserting the video file, you can insert the scene directly."

> "Note
>
> Scene strips cannot be used to reference the sequence's own scene; a secondary scene must be used instead."

> "Limitations
>
> Scene strips do not render individual Render Passes;
> only the Combined render pass will be used."

(https://docs.blender.org/manual/en/5.0/video_editing/edit/montage/strips/scene.html)

And the "rendering the first renders the second" part is stated outright on the Post Processing page:

> "Pipeline – Sequencer
>
> Renders the output of the Video Sequence editor, instead of the view from the 3D scene's active camera.
> If the sequence contains Scene strips, these will also be rendered as part of the pipeline.
> If Compositing is also enabled, the Scene strip will be the output of the Compositor."

(https://docs.blender.org/manual/en/5.0/render/output/properties/post_processing.html) The two switches that turn each pipeline on are `use_compositing`, "Process the render result through the compositing pipeline, if a compositing node group is assigned to the scene", and `use_sequencer`, "Process the render (and composited) result through the video sequence editor pipeline, if sequencer strips exist" (https://docs.blender.org/api/5.0/bpy.types.RenderSettings.html).

The manual also describes this as an intended way of working: "each shot of an animation can be stored in its own scene, and then combined in a master scene that uses scene strips to sequence them in order" and "When used as strips, scenes are rendered dynamically during playback or rendering, and they share the same data-blocks unless they were created as full copies." (https://docs.blender.org/manual/en/5.0/scene_layout/scene/introduction.html)

One 5.0-specific wrinkle, from the release notes, because it changes which scene the sequencer belongs to:

> "The sequencer no longer uses the active scene in the window for its context. Instead it now uses the _Sequencer Scene_. This is a reference to a scene stored per workspace."

(https://developer.blender.org/docs/release_notes/5.0/sequencer/) There is a matching setting `use_sequencer_override_scene_strip`, "Use workbench render settings from the sequencer scene, instead of each individual scene used in the strip" (https://docs.blender.org/api/5.0/bpy.types.RenderSettings.html).

**Not covered.** This does not settle how frame numbers line up between the two scenes, how deep the nesting may go, or what happens if the two scenes use different engines, resolutions or frame rates. It does not settle whether the second scene's own frame range or output settings have any effect. It does not say what happens with a cycle (scene A referring to scene B referring to scene A) beyond the note that a scene strip cannot reference its own scene.

**Outside the interface.** The sources document both routes but do not separately state whether either behaves differently in a command-line render. The Sequencer Scene introduced in 5.0 is stored per *workspace*, and `bpy.ops.render.render` has a `use_sequencer_scene` argument — "Render the sequencer scene instead of the active scene" (https://docs.blender.org/api/5.0/bpy.ops.render.html) — but I found **no** matching command-line option (I read the Command Line Arguments page end to end; the only scene option is `-S, --scene`). What a command-line render does when a workspace's sequencer scene differs from the active scene is **not in the sources**. **Observed 2026-08-05 (hand tests 16, 17, 18 and 19, below):** both routes work in a render started from a terminal with no window, and a bare command-line render uses the active scene rather than the Sequencer Scene.

**Operating system.** The sources do not say any of this differs by operating system. The tests below were run on macOS only. **Observed 2026-08-06:** the same tests were run again on a rented Linux machine and came back the same, so this holds on macOS and on Linux. Windows stays untested.

### Observed — hand tests 16, 17, 18 and 19 (2026-08-05)

All were run from a terminal on the Creator's own computer — a MacBook Pro with an M1 Pro chip, macOS 26.5.2 — using Blender 5.0.1 at `/Applications/Blender 5.0.1.app/Contents/MacOS/Blender`, with the `BLENDER_USER_RESOURCES` setting pointed at a fresh empty folder. Label: **observed**.

**Hand test 17 — a Render Layers node pointed at another scene.** A file was saved with two scenes. `Alpha` used Workbench at 64 by 64 with its compositing pipeline on, and its compositing node group held one Render Layers node with Scene set to `Beta`, wired to the group output. `Beta` used Cycles with 8 samples, and its own node group inverted its render. Rendering `Alpha`:

```
blender -b test.blend -S Alpha -o out/c_ -F PNG -f 1
00:00.269  render           | Start rendering: Beta, ViewLayer
00:00.269  render           | Engine: Cycles
00:00.272  render           | Fra: 1 | Mem: 1M | Sample 0/8
00:00.272  render           | Fra: 1 | Remaining: 00:00.00 | Mem: 1M | Sample 1/8
00:00.325  render           | Fra: 1 | Mem: 1M | Sample 8/8
```

So rendering the first scene does render the second, with the second scene's own engine and its own sample count.

Three images built from the same file were then compared — `Alpha`'s output, `Beta` rendered directly with its compositing pipeline on, and `Beta` rendered directly with it off. The first pixel of each:

```
alpha_0001.png    [0.0, 0.0, 0.0, 1.0]
betacomp_0001.png [0.7686, 0.7686, 0.7686, 1.0]
betaraw_0001.png  [0.0, 0.0, 0.0, 1.0]
```

`Alpha`'s output matches `Beta`'s raw render, not `Beta`'s composited one. So the second scene's own compositor does not run when it is pulled in this way.

**Hand test 18 — two scenes' Render Layers nodes pointed at each other.** With `Alpha`'s node pointed at `Beta` and `Beta`'s at `Alpha`, both pipelines on, both Workbench, rendering `Alpha` with a sixty-second cut-off finished in 0.7 seconds, exit code 0, and wrote an image:

```
00:00.299  render           | Start rendering: Beta, ViewLayer
00:00.299  render           | Engine: Workbench
00:00.375  render           | Executing compositor
00:00.375  render           | Fra: 1 | Compositing
Scene switch for render: 'Alpha' in file: '.../circular.blend'
00:00.424  render           | Saved: '.../out/x_0001.png'
```

No error, no hang, and `Beta` was rendered once rather than the chain repeating.

**Hand test 19 — a Scene strip.** A file was saved with `Alpha` on Workbench, its sequencer pipeline on and one Scene strip referencing `Beta` at frame 1; `Beta` used Cycles with 8 samples. Rendering `Alpha` from the command line printed:

```
00:00.274  render           | Start rendering: Beta, ViewLayer
00:00.274  render           | Engine: Cycles
00:00.329  render           | Fra: 1 | Mem: 1M | Sample 8/8
```

and wrote an image. So a Scene strip is rendered from outside Blender too, with the referenced scene's own engine.

**Hand test 16 — a Sequencer Scene against the active scene.** A file was saved with three scenes: `Alpha`, `Beta`, and `Edit` whose sequencer held a Scene strip referencing `Alpha`. The workspace's sequencer scene was set to `Edit` and the window's scene to `Beta`; the build step printed `MADE workspace Layout sequencer_scene= Edit` and `MADE window scene= Beta`. A bare `blender -b test.blend -o out/s_ -F PNG -f 1`, with a handler added at launch that names the scene the render starts on, printed:

```
RENDER_PRE SCENE: Beta
```

So a render started from outside uses the active scene and ignores the Sequencer Scene.

### Observed on the rented Linux machine — Linux hand tests 19, 20 and 21 (2026-08-06)

Run over SSH from a terminal on the rented Linux machine, using Blender 5.0.1 at `/opt/blender/blender-5.0.1-linux-x64/blender`, with `BLENDER_USER_RESOURCES` pointed at `/root/smoozy/Settings/res-5.0`, a folder of this topic's own that already existed. That folder was checked before use and reported its own config directory and Blender's eight default add-ons and nothing else. The machine is a container sharing someone else's hardware, and up to four topics were being worked on it at once, so nothing below claims anything about how long a run took or how much memory was free. Label: **observed**.

These are a second machine's answers to the tests the Creator's Mac answered on 2026-08-05. The two are never merged: where they agree it says so, and where they differ it says that plainly. The full list is `Audits/Blender Research/Settings/Hand tests — Linux.md`.

One thing about the set-up first, because it is not the same in all four versions. The way a scene holds its compositor changed: in Blender 4.5 it is `scene.node_tree`, switched on with `scene.use_nodes`; from 5.0 on it is `scene.compositing_node_group`, a node group with a group output. Building the same test file therefore took two shapes, and every build step printed which shape it used and which nodes it made.

**Linux hand test 19 — rendering scene Alpha with a Render Layers node pointed at scene Beta.** A file with `Alpha` on Workbench at 64 by 64, its compositing pipeline on and one Render Layers node with Scene set to `Beta` wired to the output; `Beta` on Cycles with 8 samples and its own compositor holding an Invert node. Rendering `Alpha`:

```
Start rendering: Beta, ViewLayer
Scene switch for render: 'Alpha' in file: '…/test.blend'
Fra: 1 … Sample 0/8
Fra: 1 … Sample 8/8
Saved: '…/out/alpha_0001.png'
```

So rendering the first scene does start a fresh render of the second, with the second scene's own engine and its own sample count — Cycles at 8, not the host scene's Workbench.

Three images built from the same file, middle pixel of each:

```
alpha_0001.png     middle= [1, 1, 1, 255]
betacomp_0001.png  middle= [197, 197, 197, 255]
betaraw_0001.png   middle= [1, 1, 1, 255]
```

`Alpha`'s output matches `Beta`'s raw render and not `Beta`'s composited one, so the pulled-in scene's own compositor does not run. **The two machines agree** on all of it.

**Linux hand test 20 — two scenes' Render Layers nodes pointed at each other.** With each scene's node aimed at the other, rendering `Alpha` under a sixty-second cut-off finished well inside it, exit code 0, and wrote an image. The output ended:

```
Fra: 1 | Mem: 1M | Sample 8/8
Fra: 1 | Mem: 1M | Finished
Executing compositor
Fra: 1 | Compositing
Saved: '…/out/x_0001.png'
Blender quit
```

No error, no hang, `Beta` rendered once, and no sign of the chain repeating. **The two machines agree.**

**Linux hand test 21 — a Scene strip rendered from the command line.** A file with `Alpha` on Workbench, its sequencer pipeline on and one Scene strip referencing `Beta`, which uses Cycles at 8 samples. Rendering `Alpha` printed `Start rendering: Beta, ViewLayer` and `Sample 8/8`, and wrote an image whose middle pixel is `[1, 1, 1, 255]` — `Beta`'s. **The two machines agree.**

**Still not settled on either machine.** How frame numbers line up between the two scenes — the test scenes remain on the Creator's Mac, and the rented Linux machine they were also on no longer exists — and whether a chain may go deeper than two.

---

## 9. Does a file record whether it renders on the computer's graphics cards or on its processor?

**Label: documented and observed.** The sources document a Cycles choice between processor and graphics card per scene, and a separate preferences block for the available graphics devices. The quoted material does not state which of these are saved in a file.

The manual gives the three-step split plainly:

> "To enable GPU rendering:
>
> Select either CUDA, OptiX, HIP, oneAPI, or Metal
> in the Cycles Render Devices user preference.
>
> Enable the available devices for the chosen backend.
>
> Configure each scene to use the GPU Compute Device."

(https://docs.blender.org/manual/en/5.0/render/cycles/gpu_rendering.html — read 2026-07-30.) Steps one and two are user preferences; step three is per scene. The quoted source does not say which of these settings are saved in the file.

**Observed**, in Cycles' own files. The per-scene choice:

> `enum_devices = (`
> `    ('CPU', "CPU", "Use CPU for rendering"),`
> `    ('GPU', "GPU Compute",`
> `        "Use GPU compute device for rendering, configured in the system tab in the user preferences"),`
> `)`

(`Blender Sources/cycles_5.0/properties.py`, lines 30–34.)

> `    device: EnumProperty(`
> `        name="Device",`
> `        description="Device to use for rendering",`
> `        items=enum_devices,`
> `        default='CPU',`
> `    )`

(Same file, lines 381–386; the declaration also carries `update=update_render_passes`. It sits in `class CyclesRenderSettings`, which is attached to Scene — see question 3.) This establishes a scene setting with CPU and "GPU Compute" choices, defaulting to CPU. It does not establish how that setting is saved.

The machine-specific half is named in an add-on preferences block:

> `class CyclesPreferences(bpy.types.AddonPreferences):`

> `    compute_device_type: EnumProperty(`
> `        name="Compute Device Type",`
> `        description="Device to use for computation (rendering with Cycles)",`
> `        default=CyclesPreferences.default_device(),`
> `        items=CyclesPreferences.get_device_types,`
> `    )`
> `​`
> `    devices: CollectionProperty(type=CyclesDeviceSettings)`

(Same file, lines 1569 and 1599–1606; each entry in `devices` carries `id`, `name`, `use` — "Use device for rendering", default True — and `type`.) The quoted material does not state whether or how this preferences block travels with a file.

And both can be overridden at the moment of starting, which is worth stating alongside:

> "--cycles-device <device>
>
> Set the device used for rendering.
> Valid options are: CPU CUDA OPTIX HIP ONEAPI METAL.
>
> Append +CPU to a GPU device to render on both CPU and GPU."

(https://docs.blender.org/manual/en/5.0/advanced/command_line/arguments.html) Cycles' own file is blunter about what that override beats:

> `    parser.add_argument("--cycles-device",`
> `                        help="Set the device to use for Cycles, overriding user preferences and the scene setting."`
> `                             "Valid options are 'CPU', 'CUDA', 'OPTIX', 'HIP', 'ONEAPI', or 'METAL'."`
> `                             "Additionally, you can append '+CPU' to any GPU type for hybrid rendering.",`
> `                        default=None)`

(`Blender Sources/cycles_5.0/engine.py`, lines 16–20.)

For the other engines, the sources document that EEVEE "only uses the power of the GPU to render" (https://docs.blender.org/manual/en/5.0/render/eevee/limitations/limitations.html). I found no device setting for EEVEE or Workbench anywhere in the API or manual (searched for "device", "compute device", "GPU", "CPU" across `bpy.types.SceneEEVEE`, `bpy.types.SceneDisplay`, `bpy.types.RenderSettings` and the EEVEE and Workbench manual pages). That search does not establish what a file records for either engine.

There is one further GPU-related Cycles setting, for denoising:

> `    denoising_use_gpu: BoolProperty(`
> `        name="Denoise on GPU",`
> `        description="Perform denoising on GPU devices configured in the system tab in the user preferences. This is significantly faster than on CPU, but requires additional GPU memory. When large scenes need more GPU memory, this option can be disabled",`
> `        default=False,`
> `    )`

(`Blender Sources/cycles_5.0/properties.py`, lines 432–436 — **observed**; the manual describes it as "Use GPU / Perform denoising on the GPU. This is significantly faster than on CPU, but requires additional GPU memory." at https://docs.blender.org/manual/en/5.0/render/cycles/render_settings/sampling.html.) Note this is off by default for final renders; the separate viewport one, `preview_denoising_use_gpu` at lines 473–477, defaults to True.

**Not covered.** This does not settle what happens when a file saying "GPU Compute" is opened on a machine whose preferences have no graphics device enabled — the sources do not say whether it falls back to the processor, errors, or something else. It does not cover third-party engines. It does not settle whether the preferences file travels with a render job in any way.

**Outside the interface.** Cycles' own file says `--cycles-device` overrides "user preferences and the scene setting". The quoted material does not state what a file supplies to a render started on another machine. Note the argument order rule from the manual: Cycles' options come after a bare `--`, e.g. "blender -b file.blend -f 20 -- --cycles-device OPTIX".

**Operating system.** The device *types* differ by operating system, documented on the GPU Rendering page: "CUDA is supported on Windows and Linux", "OptiX is supported on Windows and Linux", "HIP is supported on Windows and Linux", "oneAPI ... is supported on Windows and Linux", "Metal is supported on Apple computers with Apple Silicon. macOS 13.0 or newer is required to support all features." The 5.0 release notes list "Support for Intel Macs has been removed" (https://developer.blender.org/docs/release_notes/5.0/). The sources do not say whether the scene setting's file storage differs by operating system. **Observed 2026-08-05:** on this machine Metal is the technology found; CUDA, OptiX, HIP and oneAPI could not be tried at all, so what a file's `GPU` choice means on any other make of card stays untested. **Observed 2026-08-06:** a rented Linux machine with an NVIDIA GeForce RTX 4090 settles most of what was left here. That machine offers `CUDA`, `OPTIX`, `HIP` and `ONEAPI` and not `METAL`; `--cycles-device CUDA` and `OPTIX` render on the card, and `HIP`, `ONEAPI` and `METAL` are all refused with the same message. A file asking for GPU Compute renders on the card under CUDA or OptiX and falls back to the processor under the others — with nothing printed at all, except in the `HIP` case, where one warning line reads `WARNING HIPEW initialization failed: Error opening HIP dynamic library`. What an AMD or an Intel card does stays untested, and so does Windows. *(Changed by the citation check of the Linux hand tests, 2026-08-06: this line said "silently" for every case, which the block below already contradicts for HIP in this version.)*

### Observed — hand tests 7 and 8 (2026-08-05)

Both were run from a terminal on the Creator's own computer — a MacBook Pro with an M1 Pro chip, macOS 26.5.2, Apple's own graphics and no other graphics card — using Blender 5.0.1 at `/Applications/Blender 5.0.1.app/Contents/MacOS/Blender`, with the `BLENDER_USER_RESOURCES` setting pointed at fresh empty folders so the Creator's own Blender settings were untouched. Label: **observed**.

**Hand test 7 — does the choice survive a save, and is it per scene?** A file was saved from factory startup with two scenes: `One` with the Cycles device set to `GPU` and the compositor device set to `GPU`, and `Two` left at its defaults. A fresh Blender opened it and printed:

```
SCENE One cycles_device= GPU compositor_device= GPU
SCENE Two cycles_device= CPU compositor_device= CPU
```

**This closes the gap the finding named.** The file does record the choice, it survives a save and a reload, and it is held one scene at a time. The compositor's device choice behaves the same way.

**Hand test 8 — what `--cycles-device` beats, and the double dash.** A file was saved with Cycles on the processor, 100 by 100, 16 samples. A throwaway settings folder was prepared with Metal chosen; it reported:

```
DEVICE: CPU Apple M1 Pro use= True
DEVICE: METAL Apple M1 Pro (GPU - 14 cores) use= True
SAVED compute_device_type= METAL
```

With the file still saying `CPU`, `blender -b test.blend -o out/a_ -F PNG -f 1 -- --cycles-device METAL --cycles-print-stats` produced statistics carrying the line `Profiling information not available (only works with CPU rendering)`, which the same run without the override did not carry. So the argument does beat the file's own setting, as Cycles' help text claims.

Without the double dash:

```
blender -b test.blend -o out/b_ -F PNG -f 1 --cycles-device METAL
unknown argument, loading as file: --cycles-device
00:00.557  reports          | ERROR Cannot read file ".../--cycles-device": No such file or directory
```

With a name no device bears:

```
blender -b test.blend -o out/c_ -F PNG -f 1 -- --cycles-device NOT_A_DEVICE
00:00.323  cycles           | ERROR NOT_A_DEVICE is not a valid Cycles device.
Error: Not freed memory blocks: 1, total unfreed memory 0.000004 MB
```

and no image was written.

**A file asking for the graphics card on a machine with none enabled.** A second throwaway settings folder was prepared with the compute device type set to `NONE`, and the scene's device forced to `GPU` at launch. The run printed nothing about the device and no warning of any kind; its statistics carried no `Profiling information not available` line, and the image was written. So it renders on the processor and says nothing about it. **This closes the gap the finding named.**

**Workbench.** Hand test 13 (under question 3) listed every property of `scene.display`: nine, none of them a device choice.

### Observed — hand test 27, run by the Creator (2026-08-05)

Run by the Creator in Blender 5.0's own window, on their own computer — a MacBook Pro with an M1 Pro chip, macOS 26.5.2 — on 2026-08-05. That settles macOS and leaves Windows and Linux untested. Label: **observed**.

With the Render Engine set to **Workbench**, they expanded every panel of the Render Properties tab and photographed the whole of it. The panels, in order: Curves, Simplify, Performance, Sampling, Lighting, Object Color, Options, Film, Grease Pencil, Freestyle, Color Management.

**Three controls in that tab name a processor or a graphics card, and all three are in the Performance panel:**

```
Performance
  High Quality Normals        unticked
  Compositor
    Device                    CPU | GPU        — CPU selected
  Denoise Nodes
    Denoising Device          Auto | CPU | GPU — Auto selected
    Preview Quality           Balanced
    Final Quality             High
```

Those are the compositor's and the denoise nodes' devices — the two engine-neutral `RenderSettings` entries — shown here under Workbench because the tab carries them whatever the engine. **No control anywhere in the tab is a Workbench device setting.** The only other control naming a device is Color Management ▸ **Display Device**, which is a colour display device rather than a processor or a graphics card.

**This closes the search recorded above for Workbench.** Workbench itself offers no choice between the processor and the graphics card. The Compositor Device reading `CPU` here also matches what hand test 7 read back from a file in this version.

### Observed on the rented Linux machine — Linux hand tests 7, 8, 9, 10 and 25 (2026-08-06)

Run over SSH from a terminal on the rented Linux machine, using Blender 5.0.1 at `/opt/blender/blender-5.0.1-linux-x64/blender`, with `BLENDER_USER_RESOURCES` pointed at `/root/smoozy/Settings/res-5.0`, a folder of this topic's own that already existed. That folder was checked before use and reported its own config directory and Blender's eight default add-ons and nothing else. The machine is a container sharing someone else's hardware, and up to four topics were being worked on it at once, so nothing below claims anything about how long a run took or how much memory was free. Label: **observed**.

These are a second machine's answers to the tests the Creator's Mac answered on 2026-08-05. The two are never merged: where they agree it says so, and where they differ it says that plainly. The full list is `Audits/Blender Research/Settings/Hand tests — Linux.md`.

This is the question the new machine was most needed for. The Creator's Mac has Apple's own graphics and could reach only `METAL` and `CPU`; this one has an NVIDIA GeForce RTX 4090.

**Linux hand test 7 — does the choice survive a save, is it per scene, and what does a brand-new scene start at?** A brand-new factory scene, read without saving anything:

```
FACTORY engine= BLENDER_EEVEE cycles_device= CPU compositor_device= CPU
```

Then a file saved with two scenes, `One` with the Cycles device set to `GPU` and the compositor device set to `GPU`, `Two` left at its defaults, opened again in a fresh Blender:

```
SCENE One cycles_device= GPU compositor_device= GPU
SCENE Two cycles_device= CPU compositor_device= CPU
```

So the file carries the choice and holds it one scene at a time. **The two machines agree**, including on the compositor device a new scene starts at, which reads `CPU` here as it did on the Mac.

**What technologies this machine offers at all.** Setting Cycles' compute device type to `METAL` was refused, and the refusal names the whole set:

```
SETTYPE ERROR TypeError('bpy_struct: item.attr = val: enum "METAL" not found in (\'NONE\', \'CUDA\', \'OPTIX\', \'HIP\', \'ONEAPI\')')
```

So the choices are `NONE`, `CUDA`, `OPTIX`, `HIP` and `ONEAPI`, and **`METAL` is not among them on this machine**. The devices found were:

```
DEVICE: CUDA NVIDIA GeForce RTX 4090 use= True
DEVICE: CPU AMD Ryzen Threadripper PRO 3975WX 32-Cores use= True
DEVICE: OPTIX NVIDIA GeForce RTX 4090 use= True
```

(The processor entry names 32 cores; the container is given twelve of them. That is the container's doing, not Blender's.)

**Linux hand test 9 — what each `--cycles-device` name does.** A Cycles file whose scene device is `CPU`, 100 by 100, 16 samples, rendered six times from a settings folder with no technology chosen, each run being `blender -b test.blend -o out/<name>_ -F PNG -f 1 -- --cycles-device <NAME> --cycles-print-stats`. The statistics run to well over a hundred lines; the line that carries the answer is `Profiling information not available (only works with CPU rendering)`, which Cycles prints only when the render is not on the processor. The results:

```
CUDA     exit 0   image written   'Profiling information not available (only works with CPU rendering)' present
OPTIX    exit 0   image written   'Profiling information not available (only works with CPU rendering)' present
CPU      exit 0   image written   no such line
HIP      exit 1   no image        stopped with 'Found no Cycles device of the specified type'
ONEAPI   exit 1   no image        stopped with 'Found no Cycles device of the specified type'
METAL    exit 1   no image        stopped with 'Found no Cycles device of the specified type'
```

A refused run is short. The line that carries it, word for word:

```
00:00.261  reports          | ERROR Found no Cycles device of the specified type
```

The `HIP` runs — and only those — also printed one warning first: `WARNING HIPEW initialization failed: Error opening HIP dynamic library`. The `ONEAPI` and `METAL` runs printed no line of any kind about the missing technology.

So on a machine with an NVIDIA card, `CUDA` and `OPTIX` both render; and **`HIP`, `ONEAPI` and `METAL` are all refused in exactly the same words**. A name for a technology no card here can serve behaves the same as a name for a technology this operating system cannot have at all. The refusal stops the render and writes nothing.

**Linux hand test 8 — a device name that names no device, and the missing double dash.** With `-- --cycles-device NOT_A_DEVICE`:

```
00:00.231  cycles           | ERROR NOT_A_DEVICE is not a valid Cycles device.
```

exit code 1 and no image, as on the Mac. Without the double dash, `blender -b test.blend -o out/nodash_ -F PNG -f 1 --cycles-device CUDA` printed:

```
Fra: 1 … Sample 16/16
Saved: '…/out/nodash_0001.png'
unknown argument, loading as file: --cycles-device
Cannot read file "…/--cycles-device": No such file or directory
```

exit code 1. **This is fuller than the Mac's record.** The argument is indeed taken as a file name, but the frame asked for had already been rendered and saved before Blender got that far, so an image was written and the run still ended at exit code 1. The device was ignored — the statistics carried no `Profiling information not available` line, so the render was on the processor, which is what the file said.

**Linux hand test 10 — what a file's "GPU Compute" choice actually means here.** The same file, its scene device forced to `GPU` at launch, run from five settings folders in turn, each reporting its own saved technology and devices before the render:

```
preferences say NONE    -> exit 0, image written, no 'Profiling information not available' line -> rendered on the processor
preferences say CUDA    -> exit 0, image written, 'Profiling information not available' present -> rendered on the card
preferences say OPTIX   -> exit 0, image written, 'Profiling information not available' present -> rendered on the card
preferences say HIP     -> exit 0, image written, no such line                                  -> rendered on the processor
preferences say ONEAPI  -> exit 0, image written, no such line                                  -> rendered on the processor
```

So the two-way choice in the file means "use whatever this machine's preferences chose", and when that choice finds no card the render falls back to the processor, writes the image and exits 0. **The two machines agree on the silence** — the Mac found the same fallback with no graphics device enabled — and this machine shows it happening for a second reason as well, a technology chosen for a card that is not there. What Blender said about it was nothing at all, except in the `HIP` case, where one line reads `WARNING HIPEW initialization failed: Error opening HIP dynamic library`. Anything that needs to know whether a render actually used the graphics card cannot learn it from a warning, on either machine.

**Linux hand test 25 — Workbench's own Render Properties tab, photographed on this machine.** Blender was started with a real window on the machine's display, the engine set to Workbench, the Properties editor pointed at Render, and the window photographed with Blender's own screenshot command. Blender 5.0's tab lists Curves, Simplify, Performance, Sampling, Lighting and Object Color from the top. No control in the tab is a Workbench device setting, matching the Creator's photographs of the same tab on the Mac. **The two machines agree.**

---

## 10. What does Blender state about opening a file that was saved by a different version of Blender?

**Label: documented.** There is a page devoted to exactly this.

> "Blender can open blend-files saved with both older versions of the software (backward compatibility),
> and newer ones (forward compatibility). This comes with some limitations though."

> "Tip
>
> When having issues with opening much older (or newer) blend-files, it can help to use a few
> intermediary Blender releases to perform conversions by smaller steps."

> "Backward Compatibility
>
> Opening older files and converting them for the current version of Blender is usually straight-forward.
> It is expected to give very good and usable results.
>
> There can be major feature changes, for which the backward compatibility will only be ensured for
> a limited amount of time. For example the changes to the animation system that happened during the
> Blender 2.5x project. This will never be less than a full major release cycle (i.e. two years at least)."

> "Forward Compatibility
>
> Loss of Data
>
> Forward compatibility is inherently harder to ensure, and loss of feature should always be expected
> when opening a blend-file saved with a more recent version of Blender.
>
> A warning is shown in the UI when editing a more recent blend-file.
> Trying to overwrite it (with a simple 'Save' operation) will also show a confirmation popup,
> as this could make that loss of data permanent."

> "Complete Incompatibility
>
> When Blender switches to a new major version release (e.g. from 3.x to 4.0), there can also be major
> changes that will make the blend-file fully incompatible with older versions of Blender.
>
> In such cases, older Blender will fail opening (or appending/linking from) the newer blend-file, with a
> message stating which minimal version is needed to open it.
>
> In such cases, the last LTS release of the previous release cycle will be kept compatible with the newer
> file format version, and will be usable as converter between both versions.
>
> For example, Blender 3.6 LTS can open files from Blender 4.x, and will perform the necessary conversion
> such that when re-saved from 3.6, the files become compatible with all 3.x Blender versions."

(All from https://docs.blender.org/manual/en/5.0/files/blend/compatibility.html — read 2026-07-30. The page also points to a fuller developer document, which I did not follow because it is not in my sources.)

The 5.0 release notes name specific breaks, and these are statements of change, not comparisons of my own:

> "Compatibility
>
> * Maximum Data-Blocks name length has been increased to 255 bytes.
> * Removal of (Unsupported) Access to Runtime-Defined Properties Storage Data.
> * Big endian support has been removed.
> * Support for LZMA or LZO compressed point caches has been dropped
> * Support for Intel Macs has been removed
> * Support for pre-2.50 animation has been dropped.
> * Increase Minimum GPU requirements for Blender
> * Cycles: The minimum supported NVIDIA compute capability is sm_50 now (~GeForce 900 cards)
> * Cycles: Increased minimum GPU driver requirements for AMD HIP on Windows
> * Blendfile Compression on saving is now enabled by default
> * Theme API was rewritten and custom themes saved in older versions are not carried over"

(https://developer.blender.org/docs/release_notes/5.0/)

With detail on the name-length one:

> "* Compatibility with Blender 4.5:
>   * Names will be truncated when opening such 5.0 blend-files in Blender 4.5.
>   * Linking data-blocks with longer names is not supported at all in Blender 4.5.
>     This also includes opening existing 5.0 blend files linking such data-blocks."

And on packed data:

> "Compatibility wise, 5.0+ blendfiles containing packed data will open in Blender 4.5LTS, packed data will be converted to plain regular local data then (as if they had been appended)."

And on the file format itself:

> "The .blend file format has been changed to support larger buffers ... This change affects the structure of the file header
> as well as the header of each block in the file. So any code that is parsing .blend files has to be updated."

(All from https://developer.blender.org/docs/release_notes/5.0/core/)

Two render-side breaks are also stated:

> "Material `BackFace Culling > Light Probe Volume` property is now working properly. Its meaning was inverted since 4.2. ... Opening files saved with 5.0 and onward in previous versions of Blender will not result in the same baked lighting."

> "View Layer Overrides (Material, World, Sample) are now supported. Older files using this feature will render differently."

> "EEVEE's engine identifier was changed from `BLENDER_EEVEE_NEXT` to `BLENDER_EEVEE`."

(https://developer.blender.org/docs/release_notes/5.0/eevee/)

**Not covered.** This does not say how to tell, from outside Blender, which version wrote a given file. It does not say what Blender does when it opens an older file that names a render engine or an add-on that no longer exists. It does not quantify "usually straight-forward". The forward-compatibility warnings described are shown "in the UI" — see the next line.

**Outside the interface.** Two of the mechanisms described are explicitly interface-side: "A warning is shown in the UI when editing a more recent blend-file" and the save confirmation popup. The sources do not say what a background render does instead — whether it warns on the console, refuses, or proceeds silently. That gap matters here and is not fillable from what I was given. **Observed 2026-08-05 (hand test 22, below):** it warns on the terminal and then renders.

**Operating system.** The compatibility page names no operating system. Two 5.0 notes are platform-bound: "Support for Intel Macs has been removed" and the longer-names caution "Windows (NTFS) only supports up to 255 chars in its file names." The test below was run on macOS only. **Observed 2026-08-06:** the same test was run again on a rented Linux machine and came back the same, so this holds on macOS and on Linux. Windows stays untested.

### Observed — hand test 22 (2026-08-05)

Run from a terminal on the Creator's own computer — a MacBook Pro with an M1 Pro chip, macOS 26.5.2 — with the `BLENDER_USER_RESOURCES` setting pointed at a fresh empty folder. Label: **observed**.

A file was saved from factory startup by Blender 5.2.0 (Workbench, 64 by 64), then opened and rendered by Blender 5.0.1 with no window:

```
blender -b from-5.2.blend -o out/5.0_ -F PNG -f 1
00:00.269  reports          | WARNING File written by newer Blender binary (502.44), expect loss of data!
Blender 5.0.1 (hash a3db93c5b259 built 2025-12-16 01:51:40)
00:00.269  render           | Rendering single frame
00:00.270  render           | Rendering frame 1
00:00.270  render           | Start rendering: Scene, ViewLayer
00:00.270  render           | Engine: Workbench
00:00.394  render           | Saved: '.../out/5.0_0001.png'
00:00.394  render           | Time: 00:00.12 (Saving: 00:00.04)
```

Exit code 0, image written. **This closes the gap the finding named:** the forward-compatibility warning does reach a terminal, in those words, and the render goes ahead rather than being refused.

### Observed on the rented Linux machine — Linux hand tests 24 (2026-08-06)

Run over SSH from a terminal on the rented Linux machine, using Blender 5.0.1 at `/opt/blender/blender-5.0.1-linux-x64/blender`, with `BLENDER_USER_RESOURCES` pointed at `/root/smoozy/Settings/res-5.0`, a folder of this topic's own that already existed. That folder was checked before use and reported its own config directory and Blender's eight default add-ons and nothing else. The machine is a container sharing someone else's hardware, and up to four topics were being worked on it at once, so nothing below claims anything about how long a run took or how much memory was free. Label: **observed**.

These are a second machine's answers to the tests the Creator's Mac answered on 2026-08-05. The two are never merged: where they agree it says so, and where they differ it says that plainly. The full list is `Audits/Blender Research/Settings/Hand tests — Linux.md`.

**Linux hand test 24 — opening a file across versions with no window.** A factory-startup file saved by Blender 5.2 was opened and rendered by 4.5, 5.0 and 5.1, and a factory-startup file saved by 4.5 was opened and rendered by 5.2. In this version:

```
00:00.237  blend            | Read blend: "…/from-5.2.blend"
00:00.245  reports          | WARNING File written by newer Blender binary (502.44), expect loss of data!
00:00.248  render           | Start rendering: Scene, ViewLayer
00:00.248  render           | Engine: EEVEE
00:19.445  render           | Saved: '…/out/a_0001.png'
Blender quit
```

Exit code 0, and the image was written. Nothing else was printed — no region warnings and no unfreed-memory line, which is the same split the Mac found between 4.5 and the newer two. **The two machines agree.**

So the warning the manual describes as shown "in the interface" reaches a terminal on Linux as it does on macOS, and no older Blender refused the file.

---

## 11. Can a file rely on scripts stored inside it to render correctly, and what decides whether those scripts run when a render is started from outside Blender?

**Label: documented.** Yes it can, and what decides is the auto-execution setting, which two command-line flags override.

That a file can hold scripts it depends on:

> "The ability to include Python scripts within blend-files is valuable for advanced tasks
> such as rigging and automation. However, it poses a security risk since
> Python does not restrict what a script can do.
> Therefore, you should only run scripts from sources you know and trust.
> Automatic execution is disabled by default;
> however, some blend-files need this to function properly."

The two ways they run by themselves:

> "Auto Execution
>
> Here are the different ways blend-files may automatically run scripts.
>
> Registered Text-Blocks
>
> A text data-block can have its Register option enabled which means it will load on start.
>
> Animation Drivers
>
> Python expressions can be used to Drive values and are often used in more advanced rigs and animations."

And one way they run regardless, which bears directly on rendering:

> "Manual Execution
>
> There are other ways scripts in a blend-file may execute that require user
> interaction (therefore will run even when auto execution is off),
> but you should be aware that this is the case since it is not necessarily obvious.
>
> Running a script in the Text editor.
>
> Rendering with Freestyle, because Freestyle uses scripts to control line styles."

What decides, from outside:

> "Command Line
>
> You may want to perform batch rendering or some other task from the command line,
> running Blender without an interface.
> In this case, the Preferences are still used but you may want to override them:
>
> Enable with -y or --enable-autoexec
>
> Disable with -Y or --disable-autoexec"

> "Example
>
> To render an animation in background mode, allowing drivers and other scripts to run:
>
> blender --background --enable-autoexec my_movie.blend --render-anim"

(All from https://docs.blender.org/manual/en/5.0/advanced/scripting/security.html — read 2026-07-30.)

The Command Line Arguments page states which way the default falls:

> "-y, --enable-autoexec
>
> Enable automatic Python script execution.
>
> -Y, --disable-autoexec
>
> Disable automatic Python script execution (Python-drivers & startup scripts), (default)."

(https://docs.blender.org/manual/en/5.0/advanced/command_line/arguments.html)

The preference itself:

> "Auto Run Python Scripts
>
> Python scripts (including driver expressions) are not executed by default for security reasons.
> You may be working on projects where you only load files from trusted sources,
> making it more convenient to allow scripts to be executed automatically.
>
> Excluded Paths
>
> Blend-files in these folders will not automatically run Python scripts.
> This can be used to define where blend-files from untrusted sources are kept."

(https://docs.blender.org/manual/en/5.0/editors/preferences/save_load.html)

One important carve-out: not every driver needs Python permission.

> "Simple expressions are evaluated even when Python script execution is disabled."

(https://docs.blender.org/manual/en/5.0/animation/drivers/drivers_panel.html — the subset is defined on that page: ASCII variable names, floating-point and decimal integer literals, the global `frame`, listed constants and functions including `lerp`, `clamp`, `smoothstep`.) Conversely: "If using a Scripted Expression Driver Type that doesn't follow the Simple Expressions subset, you will have to open the file as Trusted Source, or set Auto Run Python Scripts in Preferences ‣ Save & Load ‣ Blender Files." (https://docs.blender.org/manual/en/5.0/animation/drivers/troubleshooting.html)

Two further command-line levers that bear on this, from the arguments page: `--addons <addon(s)>`, "Comma separated list (no spaces) of add-ons to enable in addition to any default add-ons", and `--python-exit-code <code>`, "Set the exit-code in [0..255] to exit if a Python exception is raised (only for scripts executed from the command line), zero disables."

**Not covered.** This does not settle what a background render *does* when a file needs scripts and auto-execution is off — the manual describes a dialog ("a dialog will appear. In it, you can choose to Allow Execution or to Ignore the scripts"), which is an interface thing; whether the render proceeds with the scripts ignored, warns, or fails is not stated. It does not settle the ordering of `-y` relative to the file argument beyond the general rule that arguments run in order. It does not cover what "Trusted Source" means when no File Browser is involved, other than that the preference then applies.

**Outside the interface.** This is the outside case throughout — the Command Line section above is written for it. The one thing that is inside-only is the Allow Execution / Ignore dialog and the File Browser's Trusted Source checkbox; the manual says the preference covers the case where "you may open a file without going through the File Browser". Note also: "These command-line arguments can be used to start a regular Blender instance and will still override the Preferences." **Observed 2026-08-05 (hand test 20, below):** with neither flag the preference is followed; a blocked driver prints a line and yields zero rather than stopping the render.

**Operating system.** The sources do not say any of this differs by operating system. The flags are not marked platform-specific on the arguments page. The test below was run on macOS only. **Observed 2026-08-06:** the same test was run again on a rented Linux machine and gave the same six results, so this holds on macOS and on Linux. Windows stays untested.

### Observed — hand test 20 (2026-08-05)

Run from a terminal on the Creator's own computer — a MacBook Pro with an M1 Pro chip, macOS 26.5.2 — using Blender 5.0.1 at `/Applications/Blender 5.0.1.app/Contents/MacOS/Blender`, with the `BLENDER_USER_RESOURCES` setting pointed at two fresh empty folders, one with Auto Run Python Scripts off and one with it on, so the Creator's own Blender settings were untouched. Label: **observed**.

A file was saved from factory startup with a scripted-expression driver on the default cube's x position, using the expression `__import__("os").getpid()*0+7`, which is outside the simple-expressions subset. The driven value was read back after `frame_set(1)`:

```
Auto-Run pref=off flag=none -> CUBE X = 0.0   BPY_driver_exec: restricted access disallows name '__import__', enable auto-execution to support
Auto-Run pref=off flag=-y   -> CUBE X = 7.0
Auto-Run pref=off flag=-Y   -> CUBE X = 0.0   BPY_driver_exec: restricted access disallows name '__import__', enable auto-execution to support
Auto-Run pref=on  flag=none -> CUBE X = 7.0
Auto-Run pref=on  flag=-y   -> CUBE X = 7.0
Auto-Run pref=on  flag=-Y   -> CUBE X = 0.0   BPY_driver_exec: restricted access disallows name '__import__', enable auto-execution to support
```

**This closes the gap the finding named.** With neither `-y` nor `-Y`, a render started from outside Blender follows the preference. A blocked driver is not silent — the `BPY_driver_exec:` line above is printed — and the render does not fail; it goes ahead with the driver supplying 0.

Two other expressions were tried on the same file and ran whatever the flag: `len("xxxxx")` gave 5.0 and `sum([3.0,4.0])` gave 7.0 in all three cases. So not every scripted expression needs the permission.

### Observed on the rented Linux machine — Linux hand tests 22 and 2 (2026-08-06)

Run over SSH from a terminal on the rented Linux machine, using Blender 5.0.1 at `/opt/blender/blender-5.0.1-linux-x64/blender`, with `BLENDER_USER_RESOURCES` pointed at `/root/smoozy/Settings/res-5.0`, a folder of this topic's own that already existed. That folder was checked before use and reported its own config directory and Blender's eight default add-ons and nothing else. The machine is a container sharing someone else's hardware, and up to four topics were being worked on it at once, so nothing below claims anything about how long a run took or how much memory was free. Label: **observed**.

These are a second machine's answers to the tests the Creator's Mac answered on 2026-08-05. The two are never merged: where they agree it says so, and where they differ it says that plainly. The full list is `Audits/Blender Research/Settings/Hand tests — Linux.md`.

**Linux hand test 22 — a file whose render depends on a Python driver, with and without permission to run scripts.** A file was saved from factory startup with a scripted-expression driver on the default cube's x position, the expression `__import__("os").getpid()*0+7`, Workbench at 64 by 64. Two settings folders were prepared, one with Auto Run Python Scripts off and one with it on, each reporting its own saved value before use. Six renders, each printing the preference and the driven value after `frame_set(1)`:

```
Auto-Run pref=off flag=none -> CUBE X = 0.0
Auto-Run pref=off flag=-y   -> CUBE X = 7.0
Auto-Run pref=off flag=-Y   -> CUBE X = 0.0
Auto-Run pref=on  flag=none -> CUBE X = 7.0
Auto-Run pref=on  flag=-y   -> CUBE X = 7.0
Auto-Run pref=on  flag=-Y   -> CUBE X = 0.0
```

All six exited 0 and all six wrote their image. **The two machines agree, result for result.** With neither flag, a render started from outside follows the preference.

**What a blocked script printed.** Every blocked run printed two lines, not one:

```
	BPY_driver_exec: restricted access disallows name '__import__', enable auto-execution to support
Error in PyDriver: expression failed: __import__("os").getpid()*0+7
```

The first is the line the Mac's record carries. The second is **not in the Mac's record**; whether the Mac printed it too is not recorded there, so it is not claimed either way. Either way the render did not fail — the driver supplied 0 instead of its value and the image was still written.

**And the Python-environment argument.** Searching this build's own help text on Linux found `--python-use-system-env` and no `--python-use-user-env`. **The two machines agree** — the Mac's search found the same in this version.

---

## 12. Can a render started from outside Blender be pointed at one named scene of a file, and what does it render when it is not told which?

**Label: documented** for the first half. **Inferred** for the second half, with a documented near-equivalent.

Pointing it at a named scene:

> "-S, --scene <name>
>
> Set the active scene <name> for rendering."

(https://docs.blender.org/manual/en/5.0/advanced/command_line/arguments.html — read 2026-07-30.) The rendering page repeats the purpose: "See Command Line Arguments for a full list of arguments (for example to specify which scene to render, the end frame number, etc.)" (https://docs.blender.org/manual/en/5.0/advanced/command_line/render.html).

When not told which: the closest the sources come to stating it outright is the render operator, whose scene argument is documented as —

> "scene (string, (optional, never None)) – Scene, Scene to render, current scene if not specified"

(https://docs.blender.org/api/5.0/bpy.ops.render.html.) And the interface's render commands are documented as acting on the active scene: "Render Image F12 / Render the active scene at the current frame." and "Render Animation Ctrl-F12 / Render the animation of the active scene." (https://docs.blender.org/manual/en/5.0/interface/window_system/topbar.html). A scene is recorded per window: `Window.scene`, "Active scene to be edited in the window" (https://docs.blender.org/api/5.0/bpy.types.Window.html).

**The step I am taking:** `-S` is described as *setting* the active scene, and rendering is described as acting on the active scene, and the operator's default is "current scene if not specified". From these I conclude that a command-line render with no `-S` renders whichever scene was active in the file when it was saved. No source says that in one sentence, so this is inferred. The sources do not say anywhere which scene of a multi-scene file is active on load.

There is a 5.0 complication that I must flag rather than resolve. The sequencer now has its own scene, held per workspace:

> "The sequencer no longer uses the active scene in the window for its context. Instead it now uses the _Sequencer Scene_. This is a reference to a scene stored per workspace."

> "If the sequencer scene differs from the active scene, the Render menu will show new options for "Render Sequencer Image" / "Render Sequencer Animation" on top of the normal scene render operators, allowing the user to specify which scene they'd like to render."

(https://developer.blender.org/docs/release_notes/5.0/sequencer/) The render operator has a matching argument — `use_sequencer_scene`, "Render the sequencer scene instead of the active scene" (https://docs.blender.org/api/5.0/bpy.ops.render.html) — but there is **no** command-line option for it. I read the Command Line Arguments page end to end; `-S, --scene` is the only scene option. What a command-line render does in a file whose workspace sequencer scene differs from its active scene is **not in the sources**. Searched for: "sequencer scene", "Sequencer Scene", "use_sequencer_scene", "--scene", "active scene" across manual, API and release notes.

**Ambiguity to flag:** "what does it render when it is not told which" could be read as "which scene" or as "which frames". I have answered "which scene", which I think is what is meant given the first half of the question. On the other reading: with neither `-a` nor `-f`, no render happens at all — the arguments page describes `-a` as "Render frames from start to end (inclusive)" and `-f` as "Render frame <frame> and save it", and neither is implied by loading a file. With `-a` and no frame options, the file's own Frame Start, End and Step are used: "Render the whole animation using all the settings saved in the blend-file." (https://docs.blender.org/manual/en/5.0/advanced/command_line/render.html)

**Not covered.** This does not settle what happens if `-S` names a scene that does not exist in the file. It does not settle whether `-S` must come before or after `-a`/`-f` (the general ordering rule says arguments run in order, so it presumably must come first, but the page does not give a `-S` example). It does not settle which scene is active on load in a multi-scene file.

**Outside the interface.** This is the outside case. The gap named above — no command-line counterpart to `use_sequencer_scene` — is exactly an outside-the-interface gap and matters if a file uses the 5.0 sequencer scene feature. **Observed 2026-08-05 (hand tests 15 and 16, below):** a bare command-line render uses the scene that was showing when the file was saved, and ignores the Sequencer Scene.

**Operating system.** The sources do not say `-S` differs by operating system, and it is not marked platform-specific. The tests below were run on macOS only. **Observed 2026-08-06:** the same tests were run again on a rented Linux machine and came back the same, so this holds on macOS and on Linux. Windows stays untested.

### Observed — hand tests 15 and 16 (2026-08-05)

Both were run from a terminal on the Creator's own computer — a MacBook Pro with an M1 Pro chip, macOS 26.5.2 — using Blender 5.0.1 at `/Applications/Blender 5.0.1.app/Contents/MacOS/Blender`, with the `BLENDER_USER_RESOURCES` setting pointed at a fresh empty folder. Label: **observed**.

**Hand test 15 — which scene renders when none is named.** A file was saved from factory startup with two scenes, `Alpha` (the default cube, camera and light) and `Beta` (a camera only), both Workbench at 64 by 64, with `Beta` made the window's scene before saving. Each render was run with a handler added at launch that names the scene the render starts on. The results:

```
no -S            : RENDER_PRE SCENE: Beta
-S Alpha         : RENDER_PRE SCENE: Alpha
-S NoSuchScene   : RENDER_PRE SCENE: Beta
```

Blender's own progress lines agreed — `Start rendering: Beta, ViewLayer` for the first, `Start rendering: Alpha, ViewLayer` for the second. The `-S NoSuchScene` run also printed:

```
Can't find scene: 'NoSuchScene' in file: '.../test.blend'
```

then rendered `Beta`, saved the image and exited 0.

**This settles the inferred step above.** A render started from outside with no `-S` does use the scene that was showing when the file was saved. A name no scene bears is reported and then ignored.

**Hand test 16 — the Sequencer Scene.** A third scene `Edit` was added, its sequencer holding a Scene strip referencing `Alpha`; the workspace's sequencer scene was set to `Edit` and the window's scene left as `Beta`. A bare `blender -b test.blend -o out/s_ -F PNG -f 1` printed:

```
RENDER_PRE SCENE: Beta
```

**This closes the "not in the sources" answer above.** A command-line render with no `-S` renders the active scene, not the Sequencer Scene.

**Not settled by these tests.** The files were built and saved by a Blender with no window, using `bpy.context.window.scene` and `workspace.sequencer_scene`. Whether a file saved from the interface records the active scene the same way was not tested.

### Observed on the rented Linux machine — Linux hand tests 17 and 18 (2026-08-06)

Run over SSH from a terminal on the rented Linux machine, using Blender 5.0.1 at `/opt/blender/blender-5.0.1-linux-x64/blender`, with `BLENDER_USER_RESOURCES` pointed at `/root/smoozy/Settings/res-5.0`, a folder of this topic's own that already existed. That folder was checked before use and reported its own config directory and Blender's eight default add-ons and nothing else. The machine is a container sharing someone else's hardware, and up to four topics were being worked on it at once, so nothing below claims anything about how long a run took or how much memory was free. Label: **observed**.

These are a second machine's answers to the tests the Creator's Mac answered on 2026-08-05. The two are never merged: where they agree it says so, and where they differ it says that plainly. The full list is `Audits/Blender Research/Settings/Hand tests — Linux.md`.

**Linux hand test 17 — which scene renders when none is named, and what an unknown name does.** A file was saved with two scenes, `Alpha` (the default cube, camera and light) and `Beta` (a camera only), both Workbench at 64 by 64, with `Beta` made the window's scene before saving; the build step printed `BUILD mode= twoscene scenes= ['Alpha', 'Beta'] window scene= Beta`. Each render carried a handler added at launch that prints the scene the render starts on:

```
--python-expr "import bpy; bpy.app.handlers.render_pre.append(lambda scene: print('RENDER_PRE SCENE:', scene.name))"
```

The results:

```
no -S            : RENDER_PRE SCENE: Beta      exit 0
-S Alpha         : RENDER_PRE SCENE: Alpha     exit 0
-S NoSuchScene   : RENDER_PRE SCENE: Beta      exit 0
```

and the third run also printed:

```
Can't find scene: 'NoSuchScene' in file: '…/test.blend'
```

before rendering the same scene it would have rendered anyway and saving the image. **The two machines agree**, all three results.

**Still not settled on either machine.** The test file was built and saved by a Blender with no window, on both machines. Whether a file saved from the interface records the active scene the same way is untried on either.

**Linux hand test 18 — a Sequencer Scene against the active scene.** A file was saved with three scenes: `Alpha`, `Beta`, and `Edit` whose sequencer holds a Scene strip referencing `Alpha`. The workspace's Sequencer Scene was set to `Edit` — the build step confirmed it with `BUILD sequencer_scene= Edit` — and the window's scene set to `Beta`. A bare render printed `RENDER_PRE SCENE: Beta`, saved the image and exited 0. So a render started from outside ignores the Sequencer Scene and uses the active scene. **The two machines agree.**

---

## 13. What decides the list of render engines a file can choose between, and can that list grow on a given computer?

**Label: documented.** What is installed and enabled decides it, and yes, it can grow.

> "Blender includes three render engines with different strengths:
>
> EEVEE is a physically based realtime renderer.
>
> Cycles is a physically based path tracer.
>
> Workbench is designed for layout, modeling and previews.
>
> More renderers from third-party developers are available as
> add-ons.
> Each renderer has its own render settings to control render quality and performance."

(https://docs.blender.org/manual/en/5.0/render/introduction.html — read 2026-07-30.)

The list is not fixed, and Blender will tell you what it is:

> "-E, --engine <engine>
>
> Specify the render engine.
> Use -E help to list available engines."

(https://docs.blender.org/manual/en/5.0/advanced/command_line/arguments.html; the rendering page repeats it: "For a list of available render engines, run blender -E help.")

That the list is built at run time from what is registered shows in the API documentation itself: the scene's engine property is documented as

> "engine
>
> Engine to use for rendering
>
> Type:
>
> enum in ['BLENDER_EEVEE'], default 'BLENDER_EEVEE'"

(https://docs.blender.org/api/5.0/bpy.types.RenderSettings.html.) **Inferred step:** that page lists only one entry, yet the manual says Blender includes three engines and Cycles' own file registers a fourth name; the only way to square those is that the enum is assembled from whatever is registered at the moment, and the documentation was generated with a minimal set. I am flagging this as inference because the API page does not say the enum is dynamic.

How an engine gets registered — documented in the API, with a worked example:

> "class CustomRenderEngine(bpy.types.RenderEngine):
> # These three members are used by Blender to set up the
> # RenderEngine; define its internal name, visible name and capabilities.
> bl_idname = "CUSTOM"
> bl_label = "Custom"
> bl_use_preview = True"

with, at the end of the example, `bpy.utils.register_class(CustomRenderEngine)` and `bpy.utils.unregister_class(CustomRenderEngine)` (https://docs.blender.org/api/5.0/bpy.types.RenderEngine.html).

**Observed**, that Cycles registers an engine:

> `class CyclesRender(bpy.types.RenderEngine):`
> `    bl_idname = 'CYCLES'`
> `    bl_label = "Cycles"`
> `    bl_use_eevee_viewport = True`
> `    bl_use_preview = True`
> `    bl_use_exclude_layers = True`
> `    bl_use_spherical_stereo = True`
> `    bl_use_custom_freestyle = True`

(`Blender Sources/cycles_5.0/__init__.py`, lines 42–49.)

Growing the list on a given computer — by installing and enabling an add-on:

> "The Add-ons section lets you manage secondary scripts, called "Add-ons" that extends Blender's functionality.
> Most of the time you can get add-ons as part of the Extensions system.
>
> In this section you can search, install, enable and disable Add-ons."

> "Install from Disk
>
> Install an extension from a .zip package.
> This is installed to a Local Repository and no updates will be available."

(https://docs.blender.org/manual/en/5.0/editors/preferences/addons.html)

And it can be grown for one run only, at launch:

> "--addons <addon(s)>
>
> Comma separated list (no spaces) of add-ons to enable in addition to any default add-ons."

(Command Line Arguments page.)

One 5.0 change that touches this directly, quoted from the release notes: "EEVEE's engine identifier was changed from `BLENDER_EEVEE_NEXT` to `BLENDER_EEVEE`." (https://developer.blender.org/docs/release_notes/5.0/eevee/) The rendering notes also mention that outside engines can keep up with new node features: "External render engines can support these and other new features with minimal changes using the inline shader nodes API." (https://developer.blender.org/docs/release_notes/5.0/rendering/)

**Not covered.** This does not settle what happens when a file names an engine that is not registered on the machine opening it — no error message, fallback or warning is described anywhere I searched (I looked for "engine is not", "engine not", "missing engine", "unknown engine", "falls back", "not available" across manual and API, and found nothing). It does not settle whether the file stores the engine's identifier as text or as an index. It does not say whether disabling an add-on while a file using it is open changes the scene's engine.

**Outside the interface.** `-E help` and `--addons` are the command-line handles for the list. The sources do not state how a saved engine selection is recorded or what it does if that engine is missing. The latter gap matters outside Blender's window, where nobody is watching.

**Operating system.** Neither `-E` nor `--addons` is marked platform-specific on the arguments page. The sources do not say the set of available engines differs by operating system, though the *devices* an engine can use plainly do (question 9). The tests below were run on macOS only. **Observed 2026-08-06:** the same tests were run again on a rented Linux machine. The engine identifiers, the unknown-name error and the add-on case all came back the same, so those hold on macOS and on Linux. One thing does **not** match: in Blender 4.5.12, `-E help` and an unknown engine name end abnormally on both machines but not in the same way — `libc++abi: terminating` and exit code 134 on the Mac, and a run that never ends at all on the one-card Linux machine. **Observed 2026-08-06 on a second rented Linux machine — two NVIDIA GeForce RTX 4090s, driver 580.95.05, sixty-one cores, no display server at all:** `-E help` in this version printed `BLENDER_EEVEE`, `BLENDER_WORKBENCH` and `CYCLES` and exited 0. Blender 4.5.12 hung there too, but printed its listing first, where on the one-card machine it printed nothing at all before hanging — so 4.5 ends badly on all three machines and in three different ways, and none of the three is chosen between. Windows stays untested. *(Added by the two-card citation check, 2026-08-06: the third machine's result was in the hand-test record but not here, and the sentence read as though there were only two machines.)*

### Observed — hand tests 1 and 21 (2026-08-05)

Both were run from a terminal on the Creator's own computer — a MacBook Pro with an M1 Pro chip, macOS 26.5.2 — using Blender 5.0.1 at `/Applications/Blender 5.0.1.app/Contents/MacOS/Blender`, with the `BLENDER_USER_RESOURCES` setting pointed at fresh empty folders. Label: **observed**.

**Hand test 1 — what `-E help` prints.** It printed:

```
Blender Engine Listing:
	BLENDER_EEVEE
	BLENDER_WORKBENCH
	CYCLES
```

exit code 0. So it lists identifiers rather than human labels, and Workbench's identifier is `BLENDER_WORKBENCH`.

**Hand test 21 — a file whose engine is not registered.** A file was saved with the engine set to `CYCLES`. Read back with Cycles present:

```
BEFORE engine= CYCLES has_multiple_engines= True
```

A throwaway settings folder was then prepared with the Cycles add-on switched off, leaving these add-ons enabled:

```
['bl_pkg', 'io_anim_bvh', 'io_curve_svg', 'io_mesh_uv_layout', 'io_scene_fbx', 'io_scene_gltf2', 'pose_library']
```

Opening the same file with that settings folder printed:

```
00:00.273  reports          | ERROR Engine 'CYCLES' not available for scene 'Scene' (an add-on may need to be installed or enabled)
AFTER engine= 'BLENDER_EEVEE' has_multiple_engines= True engine choices= ['BLENDER_EEVEE']
```

and `blender -E help` in that state listed only `BLENDER_EEVEE` and `BLENDER_WORKBENCH`. Rendering went ahead anyway, exit code 0:

```
00:00.273  render           | Engine: EEVEE
00:00.747  render           | Fra: 1 | Rendering 50 / 64 samples
00:00.806  render           | Fra: 1 | Rendering 64 / 64 samples
00:00.868  render           | Saved: '.../out/n_0001.png'
```

64 samples is EEVEE's own default, not the file's Cycles setting of 4.

**This closes the gap the finding named.** A missing engine is reported by that exact message, the scene's engine reads back as `BLENDER_EEVEE`, and the render goes ahead on EEVEE rather than stopping. The engine list is what is registered in the running Blender.

**What this does not settle.** No third-party engine add-on is installed, so what `--addons` does for `-E`, and what happens to a file naming a third-party engine, stay untested.

### Observed — hand test 23, run by the Creator (2026-08-05)

Run by the Creator in Blender's own window, on their own computer — a MacBook Pro with an M1 Pro chip, macOS 26.5.2 — on 2026-08-05. One shot was supplied for 5.0, 5.1 and 5.2 together rather than one each, and the Creator gave it as the answer for all three. That settles macOS and leaves Windows and Linux untested. Label: **observed**.

The open **Render Engine** dropdown in Render Properties holds exactly three entries — **EEVEE**, **Workbench**, **Cycles** — with Cycles selected. So on a Blender with no extra add-on installed, the control the file's engine is chosen from offers those three and nothing else, under their human labels rather than the identifiers `-E` takes.

### Observed on the rented Linux machine — Linux hand tests 1, 2, 23 and 25 (2026-08-06)

Run over SSH from a terminal on the rented Linux machine, using Blender 5.0.1 at `/opt/blender/blender-5.0.1-linux-x64/blender`, with `BLENDER_USER_RESOURCES` pointed at `/root/smoozy/Settings/res-5.0`, a folder of this topic's own that already existed. That folder was checked before use and reported its own config directory and Blender's eight default add-ons and nothing else. The machine is a container sharing someone else's hardware, and up to four topics were being worked on it at once, so nothing below claims anything about how long a run took or how much memory was free. Label: **observed**.

These are a second machine's answers to the tests the Creator's Mac answered on 2026-08-05. The two are never merged: where they agree it says so, and where they differ it says that plainly. The full list is `Audits/Blender Research/Settings/Hand tests — Linux.md`.

**Linux hand test 1 — what `-E help` prints, and what `-E` does with a name no engine bears.** `blender -b -E help` printed:

```
Blender 5.0.1 (hash …)
Blender Engine Listing:
	BLENDER_EEVEE
	BLENDER_WORKBENCH
	CYCLES
```

and exited 0; the same command without `-b` printed the listing without the version banner and also exited 0. With a name no engine bears:

```
blender -b test.blend -E NOT_A_REAL_ENGINE -o e_ -F PNG -f 1
00:00.206  blend            | Read blend: "…/test.blend"

Error: engine not found 'NOT_A_REAL_ENGINE'
Blender 5.0.1 (hash …)
```

exit code 1, and no image written. **The two machines agree** on every part of this. (Blender 4.5.12 on this same Linux machine printed the same things and then hung, which is recorded in its own findings.)

**What Blender's own data offers for the engine setting.** Asked for the choices on the engine property, a factory-startup Blender printed exactly one:

```
ENGINEITEM 'BLENDER_EEVEE' 'EEVEE' ''
```

That is the same one-entry list the API page shows, and it is why the list of engines cannot be read that way. What is registered in the running Blender is what decides, and `-E help` is what prints it.

**Linux hand test 23 — a file whose engine is not registered.** A file was saved with the engine set to `CYCLES` at 64 by 64. With the Cycles add-on still on, a fresh Blender printed `BEFORE CYCLES multiple= True` and listed the eight default add-ons. The add-on was then turned off in a settings folder of its own, which reported the seven left. Reading the same file there printed:

```
BEFORE BLENDER_EEVEE multiple= True
```

and rendering it printed:

```
Engine 'CYCLES' not available for scene 'Scene' (an add-on may need to be installed or enabled)
Engine: EEVEE
Saved: '…/out/f_0001.png'
```

exit code 0. So the file's engine name does not survive the add-on being off: it reads back as EEVEE's identifier, the render is reported and then goes ahead on EEVEE. **The two machines agree.**

**Linux hand test 25 — Blender's window on this machine.** A window opened in this version on the machine's own display and was photographed with Blender's own screenshot command, with the Render Engine set to Workbench and the Properties editor showing Render. The engine control is there and reads a human label rather than an identifier, as the Creator's photographs on the Mac showed. **The two machines agree.**

---

## Things a person with Blender open could settle in a few minutes

Everything on this list that a terminal could reach was run on the Creator's Mac on 2026-08-05 and written into the answers above, everything that needed eyes on Blender's window was run by the Creator the same day, and every terminal test was then run again on a rented Linux machine on 2026-08-06 and written in beside it. What is left is one gap in what was photographed, one test that needs a machine to itself, and what no machine the project has can reach.

### Still open — needs somebody looking at Blender's window

1. **What the right-click menu on a view layer's "Use for Rendering" offers.** The dot beside it, and that the switch takes a keyframe, are both settled; the menu itself was not among the shots supplied on the Mac, and driving a right-click on one control is not something the Linux run attempted. Question 1.

### Needs a machine to itself

2. **Whether Cycles' Time Limit is per frame or per job on Linux.** Settled on the Mac by timing a three-frame render against a five-second limit. It could not be re-run on the rented machine, which was shared with three other topics throughout, because the answer can only be read from how long the run takes. The test is written out in `Audits/Blender Research/Settings/Hand tests — Linux.md`. Question 2.

### Cannot be run on any machine the project has

3. **Whether EEVEE fails on a headless Windows machine.** The screenless half of this item is now answered: on a second rented Linux machine with **no display server at all** — two NVIDIA GeForce RTX 4090s, driver 580.95.05, sixty-one cores, over SSH — every engine each version has rendered, on the cards and on the processor, EEVEE included, and that result is written into question 6. What is left is the Windows half, which needs a Windows machine, and a computer with no graphics hardware at all. (Question 6.)

4. **What a file's "GPU Compute" choice means on an AMD or an Intel graphics card.** The NVIDIA case is settled — see question 9 — and asking for `HIP` or `ONEAPI` on a machine with no such card is refused outright. What either does when the card is there needs such a card, and there is none. Question 9.

5. **Whether `--addons` enables an engine early enough for `-E` to see it, and what a file naming a missing third-party engine does.** Needs a third-party render engine add-on; none is installed on either machine, and installing one is not allowed on either. The nearest case — the Cycles add-on switched off in a throwaway settings folder — was run on both and is recorded under question 13. Question 13.

6. **Whether Freestyle's line-style scripts run without `--enable-autoexec`.** The sources do not give the exact settings needed to build a file with a scripted line style, so the test cannot be written out without guessing. A second machine does not help this one. Question 11.

7. **Anything about Windows.** The Linux half of every question's operating-system line is now answered. Windows stays untried: no Windows machine has run it.

---

2026-08-04 — Opus 5 (Claude Code) — Settings, Blender 5.0

# Settings — Blender 5.1

**Sources consulted:** Blender 5.1 manual (plain-text snapshot, including one picture from its `_images` folder), Blender 5.1 Python API documentation, Blender 5.1 release notes, and the Cycles add-on's own files as shipped with Blender 5.1. All were read on **2026-07-30** (the snapshot date), and the answers were written on 2026-08-04.

I read only the question sheet and files inside Blender Sources/. I did not open any other part of this project.

A note on one source I deliberately did not lean on: `Blender Sources/requirements/requirements_2026-08-01.txt` is a single undated-by-version page. It names Blender 4.5 LTS and Blender 5.0 in so many words but never names 5.1, so I have not used it to make any claim about 5.1.

I worked alone. No helper agents were started, so the declaration above covers the whole of the work.

---

## 1. Which of a render's settings can be settled at the moment a render is started from outside Blender, and which can only be changed by changing the file?

**Answer, item by item.**

**Frame range — settable at launch. LABEL: documented.**

> `-s, --frame-start <frame>`
> Set start to frame `<frame>`, supports +/- for relative frames too.
> `-e, --frame-end <frame>`
> Set end to frame `<frame>`, supports +/- for relative frames too.

Source: manual, Command Line Arguments — https://docs.blender.org/manual/en/5.1/advanced/command_line/arguments.html — read 2026-07-30.

**An interval to skip frames by — settable at launch. LABEL: documented.**

> `-j, --frame-jump <frames>`
> Set number of frames to step forward after each rendered frame.

Same page and date. The same thing exists inside the file as the Frame Range panel's **Step**:

> Controls the number of frames to advance for each frame rendered, a value above 1 skips frames.

Source: manual, Frame Range — https://docs.blender.org/manual/en/5.1/render/output/properties/frame_range.html — read 2026-07-30.

**A list of separate frames — settable at launch. LABEL: documented.**

> `-f, --render-frame <frame>`
> Render frame `<frame>` and save it.
> `+<frame>` start frame relative, `-<frame>` end frame relative.
> A comma separated list of frames can also be used (no spaces).
> A range of frames can be expressed using `..` separator between the first and last frames (inclusive).

Source: manual, Command Line Arguments (address above), read 2026-07-30.

**A render engine — settable at launch. LABEL: documented.**

> `-E, --engine <engine>`
> Specify the render engine.
> Use `-E help` to list available engines.

Same page and date.

**A sample count — NOT settable by a dedicated launch option. LABEL: not in the sources (as a dedicated option); see the record of looking below.**

There is no sampling option anywhere in the Render Options, Cycles Render Options or Format Options sections of the Command Line Arguments page. The only two Cycles-specific launch options that exist are:

> `--cycles-device <device>` … `--cycles-print-stats`

Source: manual, Command Line Arguments (address above), read 2026-07-30. The same two, and only those two, appear in Cycles' own argument parser:

> `parser.add_argument("--cycles-print-stats", …)` and `parser.add_argument("--cycles-device", …)`

Source (observed): `Blender Sources/cycles_5.1/engine.py`, function `_configure_argument_parser`, lines 13–20. Read 2026-07-30.

**A resolution percentage — NOT settable by a dedicated launch option. LABEL: not in the sources (as a dedicated option).** It is a file setting:

> **Percentage** — Slider to reduce or increase the size of the rendered image relative to the Resolution values.

Source: manual, Format — https://docs.blender.org/manual/en/5.1/render/output/properties/format.html — read 2026-07-30. I also looked at the picture the page names, `_images/render_output_properties_format_panel.png`, which shows a Format panel with rows "Resolution X 1920 px", "Y 1080 px" and "% 100%". LABEL for that: documented.

**Whether a view layer renders at a given frame — NOT settable by a dedicated launch option.** It is a file setting:

> **Use for Rendering** — The active view layer will be used during rendering.

and, importantly:

> **Render Single Layer** — Only render the active view layer.
> Note: This option is ignored when rendering from the command line.

Source: manual, View Layer — https://docs.blender.org/manual/en/5.1/render/layers/view_layer.html — read 2026-07-30. LABEL: documented. I also opened the picture that page names, `_images/render_layers_view-layer_panel.png`; it shows a "View Layer" panel with a ticked "Use for Rendering" checkbox followed by a small round decorator dot, and an unticked "Render Single Layer" checkbox with no dot. LABEL for the picture: documented.

**The escape hatch that changes the whole picture. LABEL: inferred.** Blender accepts arbitrary Python at launch:

> `--python-expr <expression>`
> Run the given expression as a Python script.
> The expression may be a complete multi-line script; you are limited only by the platform's maximum argument length.

and

> `-P, --python <filepath>` — Run the given Python script file.

Source: manual, Command Line Arguments (address above), read 2026-07-30. The settings above all exist as writable Python properties: `resolution_percentage` ("Percentage scale for render resolution (in [1, 32767], default 100)") on `RenderSettings` (https://docs.blender.org/api/5.1/bpy.types.RenderSettings.html), `use` ("Enable or disable rendering of this View Layer (default True)") and `samples` ("Override number of render samples for this view layer, 0 will use the scene setting") on `ViewLayer` (https://docs.blender.org/api/5.1/bpy.types.ViewLayer.html), `samples` on Cycles' own settings (see below), and `taa_render_samples` ("Number of samples per pixel for rendering (in [1, inf], default 64)") on `SceneEEVEE` (https://docs.blender.org/api/5.1/bpy.types.SceneEEVEE.html). All read 2026-07-30. **The step I took:** the sources say Python expressions can be run at launch, and separately say these settings are Python properties with those names; putting the two together gives "they can be set at launch via Python, without changing the file". The sources nowhere say this in one sentence, so this is inferred, not documented.

**Order matters. LABEL: documented.**

> Arguments are executed in the order they are given.

and the worked example:

> `blender --background --render-output /tmp test.blend --render-frame 1` … will not render to /tmp because loading the blend-file overwrites the render output that was set.

Source: manual, Command Line Arguments (address above), read 2026-07-30. The rendering page repeats it:

> Always position `-f` or `-a` as the last arguments.

Source: manual, Rendering From The Command Line — https://docs.blender.org/manual/en/5.1/advanced/command_line/render.html — read 2026-07-30.

**Where I looked, for the "not in the sources" parts.** I read the whole of the Command Line Arguments page and the whole of the Rendering From The Command Line page. I then searched both pages for `sample`, `resolution`, `percentage`, `view layer`, `view-layer`, `viewlayer` — the only hits were "high resolution displays (MacBook Retina)" under `--no-native-pixels` and "ViewLayers' dependency graphs" under `--disable-depsgraph-on-file-load`, neither of which sets a render setting. I also searched the whole manual, the whole API documentation, the whole of the 5.1 release notes and all of Cycles' own files for the literal strings `--samples`, `--render-samples` and `--resolution`, with no hits at all.

**Not covered.** Whether a setting changed by `--python-expr` before `-a`/`-f` reliably survives to the render — the sources give the ordering rule but not a worked example for sampling or resolution. Also not covered: whether **Use for Rendering** can change from one frame to another; the finding records no search for that question. Any launch option for output colour management, output file format beyond `-F`/`-x`, or per-view-layer selection is also not covered.

**Outside the interface.** This whole answer *is* the outside-the-interface case; the options listed exist only there. Two differences are documented: "Render Single Layer" is "ignored when rendering from the command line", and the audio device "is disabled in background-mode by default". Whether every other setting behaves identically is not stated.

**Operating system.** The Render Options, Format Options and Python Options sections carry no operating-system qualifier, while other options on the same page are explicitly marked ("Windows only", "Linux only"). None of the frame-range, frame-jump, frame-list or engine options carries such a mark. That absence is documented; reading it as "these work on every operating system" is inferred. **Observed 2026-08-05:** every test below was run on macOS only, so Windows and Linux stay untested. **Observed 2026-08-06:** the same tests were run again on a rented Linux machine (see the Linux block at the end of this answer). Every argument this answer names behaves the same way there, so the answer now holds on macOS and on Linux; Windows stays untested. The argument list itself is not identical on the two — Blender's own help text is longer on Linux.

### Observed — hand tests 1, 2, 3, 4, 5 and 10 (2026-08-05)

All were run from a terminal on the Creator's own computer — a MacBook Pro with an M1 Pro chip, macOS 26.5.2, Apple's own graphics and no other graphics card — using Blender 5.1.2 at `/Applications/Blender 5.1.2.app/Contents/MacOS/Blender`, with the `BLENDER_USER_RESOURCES` setting pointed at a fresh empty folder so the Creator's own Blender settings were untouched. Label: **observed**.

**On the conditions of hand tests 1, 2 and 3.** Those three were first run before it was known that Blender falls back to the machine's real settings when that folder does not exist, so the first attempts loaded the Creator's own add-ons. Those attempts were discarded. All three were re-run from scratch with the folder created first — hand test 3 with its files rebuilt as well — and each version was checked before use, reporting its own throwaway settings folder and Blender's eight default add-ons and nothing else. What is written below is the clean re-run.

**Hand test 1 — the engine argument.** `blender -E help` printed:

```
Blender Engine Listing:
	BLENDER_EEVEE
	BLENDER_WORKBENCH
	CYCLES
```

exit code 0. With a name no engine bears, `blender -b test.blend -E NOT_A_REAL_ENGINE -o out/e_ -F PNG -f 1` printed `Error: engine not found 'NOT_A_REAL_ENGINE'`, exit code 1, and no image was written.

**Hand test 2 — is there really no argument for a sample count, a resolution percentage or a view layer?** `blender --help` was written to a file and searched, without opening any blend-file. The whole output is 498 lines. Searching it for `sampl`, `resolution`, `percent`, `view layer` and `python-use` returned:

```
128:	Do not use native pixel size, for high resolution displays (MacBook 'Retina').
164:--python-use-system-env 
166:	This also enables user environment, see: '--python-use-user-env'.
168:--python-use-user-env 
443:	Background mode: Do not systematically build and evaluate ViewLayers' dependency graphs
```

None of them sets a render setting, which is what the manual search above had found. **This also settles the discrepancy flagged under question 11:** this build has both arguments, and their help text reads:

```
--python-use-system-env 
	Allow Python to use system environment variables such as 'PYTHONPATH'.
	This also enables user environment, see: '--python-use-user-env'.

--python-use-user-env 
	Allow Python to use user's site-packages directory.
	This disables full isolation for the Python environment.
```

The manual page and the release notes were each describing one of two arguments that both exist. (For comparison, the same search of Blender 4.5.12's and 5.0.1's help shows `--python-use-system-env` only, with the older wording "…such as 'PYTHONPATH' and the user site-packages directory.")

**Hand test 3 — a list of frames, a range of frames, and the frame jump.** A file was saved from factory startup with the Workbench engine and a 64 by 64 resolution. Five renders, each into its own empty folder, wrote:

```
blender -b test.blend -o out-list/f_  -F PNG -f 1,5,9        -> f_0001.png f_0005.png f_0009.png
blender -b test.blend -o out-range/f_ -F PNG -f 3..6         -> f_0003.png f_0004.png f_0005.png f_0006.png
blender -b test.blend -o out-mixed/f_ -F PNG -f 1,5..7,12    -> f_0001.png f_0005.png f_0006.png f_0007.png f_0012.png
blender -b test.blend -j 3 -o out-jumpf/f_ -F PNG -f 1..10   -> f_0001.png … f_0010.png (all ten)
blender -b test.blend -j 3 -s 1 -e 10 -o out-jumpa/f_ -F PNG -a -> f_0001.png f_0004.png f_0007.png f_0010.png
```

No error was printed by any of the five. So `-f` takes a comma list, a `..` range, and the two mixed in one argument; and `-j` is applied to `-a` and not to `-f`.

**Hand test 4 — does a launch argument change the file on disk?** A file was saved with frame start 1, frame end 250, frame step 1, resolution percentage 100 and the engine left at `BLENDER_EEVEE`. Then:

```
blender -b test.blend -E CYCLES -s 5 -e 7 -j 2 -o out/f_ -F PNG -f 5
```

The file's modification time and size were unchanged (`1785890069`, `95372 bytes`, before and after). Reading it back with no arguments printed:

```
READBACK 1 250 1 100 BLENDER_EEVEE
```

So the arguments applied to that run only and left the file alone.

**Hand test 5 — `--python-expr` at launch.** A file was saved with Cycles, 512 samples, two view layers (`ViewLayer` and `Layer2`) and a 200 by 100 resolution at 100 per cent. With the expression after the blend-file argument:

```
blender -b test.blend --python-expr "import bpy; s=bpy.data.scenes[0]; s.cycles.samples=8; s.render.resolution_percentage=25; s.view_layers['Layer2'].use=False; print('SET', s.cycles.samples, s.render.resolution_percentage, s.view_layers['Layer2'].use)" -o out-after/f_ -F PNG -f 1
```

printed `SET 8 25 False` and wrote an image of `pixelWidth: 50   pixelHeight: 25`. With the expression before the blend-file argument it printed `SET 8 25` and wrote `pixelWidth: 200   pixelHeight: 100`. **This settles the inferred escape hatch above**, and the ordering rule with it.

**Hand test 10 — a view layer's "Use for Rendering", and "Render Single Layer".** From factory startup, `keyframe_insert` on a view layer's `use` returned `True`. A file was saved with two view layers, `Layer2`'s `use` keyframed on at frame 1 and off at frame 2, Workbench, 64 by 64, then rendered with `blender -b test.blend -o out/f_ -F OPEN_EXR_MULTILAYER -s 1 -e 2 -a`. The progress lines named:

```
Rendering frame 1 -> Scene, ViewLayer  and  Scene, Layer2
Rendering frame 2 -> Scene, ViewLayer
```

and the channel names in the two written files were:

```
f_0001.exr : Layer2.Combined  Layer2.CombinedpixelAspectRatio  ViewLayer.Combined  ViewLayer.CombinedpixelAspectRatio   (8411 bytes)
f_0002.exr : ViewLayer.Combined  ViewLayer.CombinedpixelAspectRatio                                                     (4394 bytes)
```

**This closes the "Use for Rendering, at a given frame" gap the finding named:** an animation render driven from outside honours the switch frame by frame.

Separately, with `use_single_layer` turned on at launch, the run printed `SINGLE-LAYER ON, frame 1, layers on: ['ViewLayer', 'Layer2']` and the written file held both `Layer2.Combined` and `ViewLayer.Combined`. So the setting is indeed ignored from the command line.

### Observed — hand test 25, run by the Creator (2026-08-05)

Run by the Creator in Blender 5.1's own window, on their own computer — a MacBook Pro with an M1 Pro chip, macOS 26.5.2 — on 2026-08-05. That settles macOS and leaves Windows and Linux untested. Label: **observed**.

They opened the Properties editor's View Layer Properties tab, on a view layer named `LayerA`, and photographed it. What is visible: a **View Layer** panel holding **Use for Rendering**, ticked, with a small round dot at the far right of its row; and **Render Single Layer**, unticked, with no dot. Below it sit the Passes, Cryptomatte, Shader AOV, Filter and Custom Properties panels.

They also wrote, in their own words:

> "Pressing I at Use for Rendering makes the checkbox yellow and the dot next to it a diamond. Consistent across all versions. Render Single Layer is non-animatable in all versions, too. Here's the message: "use_single_layer" property cannot be animated. Also consistent in all versions."

**This settles from the running program the part of question 1 that the finding above left open** — the "at a given frame" reading, which the finding could only reach through the manual's picture. The dot is there, the switch takes a keyframe, and "Render Single Layer" is refused one outright with the message quoted above.

**What the screenshots do not cover.** The entry also asked what the right-click menu on "Use for Rendering" offers. No shot of that menu was among those supplied — the Creator answered by pressing I instead — so what that menu lists is still unrecorded.

### Observed on the rented Linux machine — Linux hand tests 2, 3, 4, 5 and 12 (2026-08-06)

Run over SSH from a terminal on a rented Linux machine — Ubuntu 22.04.5 LTS, NVIDIA GeForce RTX 4090, driver 580.105.08, twelve cores, run over SSH, using Blender 5.1.2 at `/opt/blender/blender-5.1.2-linux-x64/blender`, with `BLENDER_USER_RESOURCES` pointed at `/root/smoozy/Settings/res-5.1`, a folder of this topic's own that already existed. That folder was checked before use and reported its own config directory and Blender's eight default add-ons and nothing else. The machine is a container sharing someone else's hardware, and up to four topics were being worked on it at once, so nothing below claims anything about how long a run took or how much memory was free. Label: **observed**.

These are a second machine's answers to the tests the Creator's Mac answered on 2026-08-05. The two are never merged: where they agree it says so, and where they differ it says that plainly. The full list is `Audits/Blender Research/Settings/Hand tests — Linux.md`.

**Linux hand test 2 — is there really no argument for a sample count, a resolution percentage or a view layer?** `blender --help` was written to a file and searched, with no blend-file opened. The whole help output is **505 lines** on this machine; the same command on the Creator's Mac gave 498, so the argument list is not the same length on the two operating systems. Searching it for `sampl`, `resolution`, `percent`, `view layer` and `python-use` returned only lines that set no render setting — the `--no-native-pixels` description, the Python-environment arguments, and the background-mode note about view layer dependency graphs. There is no `--samples` and no `--resolution` in this build either. Both `--python-use-system-env` and `--python-use-user-env` are present in this build, as they were on the Mac.

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
  $SPNAV_SOCKET              The socket path to connect to the 3D-mouse daemon (Unix only).
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

**Both yes, in Cycles. LABEL: documented.**

Stopping on cleanliness:

> **Adaptive Sampling** — If the Noise Threshold checkbox is enabled, Cycles will use adaptive sampling, cutting short the sampling process in areas that have become less noisy than the specified threshold value – in other words, not wasting time further refining areas that already look good enough.

> **Noise Threshold** — The threshold to decide whether to continue sampling a pixel. Typical values are in the range from 0.1 to 0.001, with lower values meaning better quality but longer render times. Setting this to 0 makes Cycles guess an automatic value based on the total sample count.

> **Min Samples** — The minimum number of samples a pixel receives before adaptive sampling is applied. When set to 0 (default), Cycles automatically picks a value determined by the Noise Threshold.

Stopping on time:

> **Time Limit** — Stops rendering if the time exceeds the limit, even if the desired sample count hasn't been reached yet. Setting this value to zero disables the limit.
> Note: The time limit does not include pre-render processing time, only render time.

Source: manual, Cycles Sampling — https://docs.blender.org/manual/en/5.1/render/cycles/render_settings/sampling.html — read 2026-07-30.

The same two settings, with their kind, default and range. LABEL: observed.

> ```
> time_limit: FloatProperty(
>     name="Time Limit",
>     description="Limit the render time (excluding synchronization time). "
>     "Zero disables the limit",
>     min=0.0,
>     default=0.0,
>     step=100.0,
>     unit='TIME_ABSOLUTE',
> )
> ```

Source: `Blender Sources/cycles_5.1/properties.py`, class `CyclesRenderSettings`, lines 514–522. Read 2026-07-30.

> ```
> use_adaptive_sampling: BoolProperty(
>     name="Use Adaptive Sampling",
>     description="Automatically reduce the number of samples per pixel based on estimated noise level",
>     default=True,
> )
> adaptive_threshold: FloatProperty(
>     name="Adaptive Sampling Threshold",
>     description="Noise level step to stop sampling at, lower values reduce noise at the cost of render time. Zero for automatic setting based on number of AA samples",
>     min=0.0,
>     max=1.0,
>     soft_min=0.001,
>     default=0.01,
>     precision=4,
> )
> adaptive_min_samples: IntProperty(
>     name="Adaptive Min Samples",
>     description="Minimum AA samples for adaptive sampling, to discover noisy features before stopping sampling. Zero for automatic setting based on noise threshold",
>     min=0,
>     max=4096,
>     default=0,
> )
> ```

Source: `Blender Sources/cycles_5.1/properties.py`, lines 566–586. Read 2026-07-30. So adaptive sampling is on by default, with a threshold of 0.01, and the time limit is off by default.

**Neither exists for EEVEE. LABEL: not in the sources.** The EEVEE Sampling page lists only Viewport Samples, Temporal Reprojection, Jittered Shadows, Render Samples, shadow Rays/Steps, Volumetric Shadows, shadow Resolution and Light Threshold — no time limit and no noise threshold. Source: manual, EEVEE Sampling — https://docs.blender.org/manual/en/5.1/render/eevee/render_settings/sampling.html — read 2026-07-30. I also searched the whole manual for "Time Limit" (one hit only, the Cycles page) and for "noise threshold" (four hits, all on the Cycles page), and searched the `SceneEEVEE` API page for `time_limit` and `threshold` — the only threshold there is `bokeh_threshold` and `light_threshold`, neither of which stops a render.

**Not covered.** Whether the Time Limit applies per frame or to a whole animation is not stated anywhere I found — the wording is "Stops rendering if the time exceeds the limit", with no mention of animations. Also not covered: what happens to the frames already written when a time limit cuts an animation short, and whether an image stopped by the time limit is marked in any way. Adaptive sampling is described as acting per pixel ("whether to continue sampling a pixel"); the sources do not say in so many words what ends the frame as a whole when every pixel has converged.

**Outside the interface.** The sources describe both as render settings rather than interface actions. It is inferred that they take effect wherever a render runs, because the sources do not distinguish how the render was started. Neither has a dedicated launch option (see question 1), so from outside Blender they would need to be in the file or set by a Python expression. The note that the time limit "does not include pre-render processing time" is not qualified by how the render was started. **Observed 2026-08-05 (hand tests 11 and 12, below):** both are carried by the file and both take effect in a render started from a terminal with no window.

**Operating system.** The sources say nothing about either setting differing by operating system. The tests below were run on macOS only. **Observed 2026-08-06:** adaptive sampling and the EEVEE and Workbench property lists were run again on a rented Linux machine and came back the same, so those parts hold on macOS and on Linux. The Time Limit test was not re-run there, because it can only be read from timing and the machine was shared, so that part still holds for macOS alone. Windows stays untested.

### Observed — hand tests 11, 12 and 13 (2026-08-05)

All three were run from a terminal on the Creator's own computer — a MacBook Pro with an M1 Pro chip, macOS 26.5.2, Apple's own graphics and no other graphics card — using Blender 5.1.2 at `/Applications/Blender 5.1.2.app/Contents/MacOS/Blender`, with the `BLENDER_USER_RESOURCES` setting pointed at a fresh empty folder. Label: **observed**.

**Hand test 11 — is the Time Limit per frame or per job, and is a cut-short frame still written?** A file was saved from factory startup with Cycles on the processor, 400 by 400, Max Samples 16384, adaptive sampling off, denoising off, Time Limit 5 seconds, frame range 1 to 3. Then `blender -b test.blend -o out/f_ -F PNG -a`. The per-frame times printed were `Time: 00:05.43`, `Time: 00:05.28`, `Time: 00:05.34`, and the whole run took 17 seconds by the shell's clock. All three images were written: `f_0001.png f_0002.png f_0003.png`.

**This closes both gaps the finding named:** the limit is applied to each frame, not to the whole animation, and a frame cut short is still written to disk.

**Hand test 12 — does adaptive sampling end a frame before the sample count is reached?** Two files were saved from factory startup, both Cycles on the processor, 128 by 128, Max Samples 2048, Time Limit 0, denoising off — one with adaptive sampling on and a Noise Threshold of 0.1, one with it off. The sample lines printed:

```
adaptive on  : Sample 0/2048  Sample 1/2048  Sample 48/2048  Sample 0/2048   Time: 00:00.08
adaptive off : Sample 2048/2048                                              Time: 00:01.16
```

**This closes the other gap the finding named:** the whole frame stops early — at 48 of 2048 samples here — not merely the sampling of individual pixels.

**Hand test 13 — has EEVEE or Workbench any way to stop a render early?** Every property of `scene.eevee` and `scene.display` was listed from factory startup and searched for `time`, `threshold`, `adaptive`, `noise`, `stop` and `limit`. `scene.eevee` has 54 properties and `scene.display` has 9. Neither holds a time limit or a rule for stopping when the picture is clean enough. The nearest matches on `scene.eevee` were `bokeh_threshold` ("Brightness threshold for using sprite base depth of field") and `light_threshold` ("Minimum light intensity for a light to contribute to the lighting"); on `scene.display` there was no match. Its nine properties in full:

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

Run by the Creator in Blender 5.1's own window, on their own computer — a MacBook Pro with an M1 Pro chip, macOS 26.5.2 — on 2026-08-05. That settles macOS and leaves Windows and Linux untested. Label: **observed**.

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

**This closes the "not in the sources" answer above by eye as well as by property list.** Neither engine's Sampling panel offers a time limit, a noise threshold, or any other control that would end a render early. The EEVEE page's list quoted above — Viewport Samples, Temporal Reprojection, Jittered Shadows, Render Samples, shadow Rays, Steps, Volumetric Shadows, shadow Resolution and Light Threshold — is what the panel shows, with nothing further hidden in it.

### Observed on the rented Linux machine — Linux hand tests 13 and 14 (2026-08-06)

Run over SSH from a terminal on the rented Linux machine, using Blender 5.1.2 at `/opt/blender/blender-5.1.2-linux-x64/blender`, with `BLENDER_USER_RESOURCES` pointed at `/root/smoozy/Settings/res-5.1`, a folder of this topic's own that already existed. That folder was checked before use and reported its own config directory and Blender's eight default add-ons and nothing else. The machine is a container sharing someone else's hardware, and up to four topics were being worked on it at once, so nothing below claims anything about how long a run took or how much memory was free. Label: **observed**.

These are a second machine's answers to the tests the Creator's Mac answered on 2026-08-05. The two are never merged: where they agree it says so, and where they differ it says that plainly. The full list is `Audits/Blender Research/Settings/Hand tests — Linux.md`.

**Linux hand test 13 — does adaptive sampling end the frame before the sample count is reached?** Two Cycles files at 128 by 128, Max Samples 2048, Time Limit 0, denoising off — one with adaptive sampling on at a noise threshold of 0.1, one with it off. Both rendered one frame with `-- --cycles-print-stats`, both exited 0, both wrote their image. The sample counts reported:

```
adaptive on  : Sample 0/2048 -> Sample 1/2048 -> Sample 48/2048   (stopped there)
adaptive off : Sample 0/2048 -> Sample 1/2048 -> Sample 2048/2048
```

**The two machines agree**, at the same 48 samples out of 2,048. So adaptive sampling ends the whole frame early, not only the sampling of single pixels.

**Linux hand test 14 — has EEVEE or Workbench anything that stops a render?** Every property of the scene's EEVEE block and of its Workbench block was listed with its description. EEVEE holds **54** on this build and Workbench holds **9** — the same two counts the Mac's run gave for this version. Searching both lists for `time`, `threshold`, `adaptive`, `noise`, `stop` and `limit` matched only EEVEE lines, and none of them stops anything:

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

**LABEL: documented, with one inferred step at the end.**

The manual states the split as a principle:

> Blender includes three render engines with different strengths:
> EEVEE is a physically based realtime renderer.
> Cycles is a physically based path tracer.
> Workbench is designed for layout, modeling and previews.
> More renderers from third-party developers are available as add-ons.
> Each renderer has its own render settings to control render quality and performance.

Source: manual, Rendering — Introduction — https://docs.blender.org/manual/en/5.1/render/introduction.html — read 2026-07-30.

The Python data model draws the same line explicitly. On a scene there are separate, simultaneously present groups:

> `cycles` — Cycles render settings (readonly) — Type: `CyclesRenderSettings`
> `cycles_curves` — Cycles curves rendering settings (readonly) — Type: `CyclesCurveRenderSettings`
> `eevee` — EEVEE settings for the scene (readonly) — Type: `SceneEEVEE`
> `display` — Scene display settings for 3D viewport (readonly) — Type: `SceneDisplay`
> `render` — (readonly, never None) — Type: `RenderSettings`

Source: API, Scene — https://docs.blender.org/api/5.1/bpy.types.Scene.html — read 2026-07-30.

`RenderSettings` is a separate group whose properties include `engine`, `resolution_x`, `resolution_y`, `resolution_percentage`, `pixel_aspect_x`, `pixel_aspect_y`, `fps`, `fps_base`, `filepath`, `file_extension`, `use_file_extension`, `use_overwrite`, `use_placeholder`, `image_settings`, `ffmpeg`, `use_border`, `border_min_x`/`border_max_x`/`border_min_y`/`border_max_y`, `use_crop_to_border`, `threads`, `threads_mode`, `use_compositing`, `use_sequencer`, `use_stamp` and the whole `use_stamp_*` family, `film_transparent`, `filter_size`, `use_motion_blur`, `motion_blur_shutter`, `use_simplify` and the `simplify_*` family, `use_single_layer`, `use_persistent_data`, `use_multiview`, `views_format`, `use_freestyle`, `line_thickness`, `compositor_device`, `compositor_precision`, and the read-only `has_multiple_engines` and `is_movie_format`. Source: API, RenderSettings — https://docs.blender.org/api/5.1/bpy.types.RenderSettings.html — read 2026-07-30. Calling this group engine-neutral is inferred from its name and from the settings listed; the API does not use that label.

Examples of settings that belong to one engine only, quoted from the engine's own manual page:

- Cycles only — **Device** ("Specifies the compute device used for rendering in Cycles") and **Open Shading Language**, on the Cycles Render Settings page — https://docs.blender.org/manual/en/5.1/render/cycles/render_settings/index.html; **Tile Size** and **Acceleration Structure** on Cycles Performance — https://docs.blender.org/manual/en/5.1/render/cycles/render_settings/performance.html; and **Adaptive Sampling**, **Path Guiding**, **Light Tree**, **Scrambling Distance** and **Sample Subset** on Cycles Sampling — https://docs.blender.org/manual/en/5.1/render/cycles/render_settings/sampling.html. All read 2026-07-30. *[Citation check, 2026-08-05: the last five were cited to the index and performance pages, which do not carry them; the Sampling page does, and is now named.]*
- EEVEE only — **Temporal Reprojection**, **Jittered Shadows**, shadow **Rays**/**Steps**/**Resolution**, **Volumetric Shadows**, plus the Raytracing panel and, on the EEVEE Performance page, the **Light Probes Volume Pool**. Source: manual, EEVEE Sampling (address in answer 2) and the EEVEE Render Settings folder, read 2026-07-30. *[Citation check, 2026-08-05: "the Light Probes panels" corrected — that folder has no Light Probes panel; what it carries is Light Probes Volume Pool on the Performance page.]*
- Workbench only — the anti-aliasing method, of which the manual says: "The quality of the renders can be adjusted by changing the Anti-Aliasing method. A different one can be selected for the 3D Viewport, viewport rendering and for final rendering." Source: manual, Workbench Sampling — https://docs.blender.org/manual/en/5.1/render/workbench/sampling.html — read 2026-07-30.

Cycles' own file confirms where its group is attached. LABEL: observed.

> ```
> @classmethod
> def register(cls):
>     bpy.types.Scene.cycles = PointerProperty(
>         name="Cycles Render Settings",
>         description="Cycles render settings",
>         type=cls,
>     )
> ```

Source: `Blender Sources/cycles_5.1/properties.py`, class `CyclesRenderSettings`, lines 1101–1107. Read 2026-07-30. The same file attaches Cycles groups to `Camera`, `Material`, `Light`, `World`, `Mesh`, `Curve`, `MetaBall`, `Object`, `Scene.cycles_curves`, `ViewLayer.cycles` and `View3DShading.cycles` (lines 1118–1119, 1169–1170, 1210–1211, 1271–1272, 1324–1325, 1340–1351, 1438–1439, 1467–1468, 1546–1547, 1975).

**The inferred step.** Because Cycles is an add-on that adds `Scene.cycles` when it registers, and EEVEE's and Workbench's groups (`Scene.eevee`, `Scene.display`) are built in, the engine-specific groups exist side by side on one scene regardless of which engine is currently selected. The sources do not say that sentence outright; I got it by combining the registration code with the Scene API page listing all four groups at once. LABEL for that sentence: inferred.

**Not covered.** This is a sample, not an exhaustive inventory of every engine-specific setting. It also does not settle which settings a *third-party* engine adds, nor whether an engine-neutral setting such as `filter_size` or `use_motion_blur` is honoured identically by all three engines.

**Outside the interface.** It is inferred that the grouping holds when Blender is driven from outside, because the sources describe data groups rather than interface controls. The sources do not establish what happens to `scene.cycles` if a launch disables the Cycles add-on. **Observed 2026-08-05 (hand tests 6, 7 and 21):** all four blocks were reachable and held their own saved values in a Blender started from a terminal with no window, whichever engine was selected; and with the Cycles add-on switched off, the scene's engine reads back as `BLENDER_EEVEE` and the engine list drops to `['BLENDER_EEVEE']` — see question 13.

**Operating system.** Nothing in the sources ties the grouping to an operating system. Individual Cycles device options are operating-system-bound (see answer 9), but the settings themselves are not. The test below was run on macOS only. **Observed 2026-08-06:** the same test was run again on a rented Linux machine and came back the same, so this holds on macOS and on Linux. Windows stays untested.

### Observed — hand test 13 (2026-08-05)

Run from a terminal on the Creator's own computer — a MacBook Pro with an M1 Pro chip, macOS 26.5.2 — using Blender 5.1.2 at `/Applications/Blender 5.1.2.app/Contents/MacOS/Blender`, with the `BLENDER_USER_RESOURCES` setting pointed at a fresh empty folder. Label: **observed**.

Every property of `scene.eevee` and `scene.display` was listed from factory startup. `scene.eevee` holds 54 properties; `scene.display` holds 9, listed in full under question 2. Searching both for `device`, `cpu` and `gpu` returned only two lines, both about memory size rather than a choice:

```
eevee gi_irradiance_pool_size | Size of the irradiance pool, a bigger pool size allows for more irradiance grid in the scene but might not fit into GPU memory and decrease performance
eevee shadow_pool_size | Size of the shadow pool, a bigger pool size allows for more shadows in the scene but might not fit into GPU memory
```

### Observed — hand test 23, run by the Creator (2026-08-05)

Run by the Creator in Blender's own window, on their own computer — a MacBook Pro with an M1 Pro chip, macOS 26.5.2 — on 2026-08-05. **One shot was supplied for 5.0, 5.1 and 5.2 together rather than one each**, and the Creator gave it as the answer for all three; it is that shot this finding rests on. That settles macOS and leaves Windows and Linux untested. Label: **observed**.

They opened Render Properties and clicked the **Render Engine** dropdown so its list was showing. The open list holds exactly three entries, in this order:

```
EEVEE
Workbench
Cycles
```

with Cycles the one selected. The row immediately under the dropdown is **Device**.

So the control offers human labels, not the identifiers `-E` takes (hand test 1, under question 13), and the three engines the manual names are the three the control offers.

### Observed on the rented Linux machine — Linux hand tests 6, 14 and 25 (2026-08-06)

Run over SSH from a terminal on the rented Linux machine, using Blender 5.1.2 at `/opt/blender/blender-5.1.2-linux-x64/blender`, with `BLENDER_USER_RESOURCES` pointed at `/root/smoozy/Settings/res-5.1`, a folder of this topic's own that already existed. That folder was checked before use and reported its own config directory and Blender's eight default add-ons and nothing else. The machine is a container sharing someone else's hardware, and up to four topics were being worked on it at once, so nothing below claims anything about how long a run took or how much memory was free. Label: **observed**.

These are a second machine's answers to the tests the Creator's Mac answered on 2026-08-05. The two are never merged: where they agree it says so, and where they differ it says that plainly. The full list is `Audits/Blender Research/Settings/Hand tests — Linux.md`.

**Linux hand tests 6 and 14 — are all the engines' blocks there at once, and what is on them?** A file was saved with Cycles samples 123, EEVEE render samples 45, Workbench final anti-aliasing `32` and the Grease Pencil final-render count 3, then read back in a fresh Blender started from a terminal with no window:

```
BACK as-loaded engine= BLENDER_EEVEE cycles= 123 eevee= 45 workbench= 32 gpencil= 3
```

All three engines' blocks were reachable and each held its own saved value, whichever engine was selected. Listing every property gave 54 on the EEVEE block and 9 on the Workbench block. **The two machines agree.**

**Linux hand test 25 — the Render Properties tab, photographed in Blender's own window on this machine.** Blender was started with a real window on the machine's own display, the engine set to Workbench and the Properties editor pointed at Render, and the window photographed with Blender's own screenshot command. Blender 5.1's tab lists **Curves** above Simplify. The tab carries the shared settings under whichever engine is chosen, exactly as the Creator's photographs of the same tab on the Mac showed. **The two machines agree.**

---

## 4. Does a file carry a separate sample count for each render engine?

**Yes. LABEL: documented for each engine separately; inferred for "all at once".**

Cycles:

> `samples: IntProperty(name="Samples", description="Number of samples to render for each pixel", min=1, max=(1 << 24), default=4096,)`

Source (observed): `Blender Sources/cycles_5.1/properties.py`, lines 479–484. Read 2026-07-30. And in the manual: "**Render (Max) Samples** — The number of paths to trace per pixel in the final render." Source: manual, Cycles Sampling (address in answer 2), read 2026-07-30.

EEVEE:

> `taa_render_samples` — Number of samples per pixel for rendering (in [1, inf], default 64)

Source: API, SceneEEVEE — https://docs.blender.org/api/5.1/bpy.types.SceneEEVEE.html — read 2026-07-30. Manual: "**Render** — **Samples** — The number of samples to use in the final render." Source: manual, EEVEE Sampling (address in answer 2), read 2026-07-30.

Workbench has no sample count as such, but has a per-scene anti-aliasing choice whose options are sample counts:

> `render_aa` — Method of anti-aliasing when rendering final image (default '8')

Source: API, SceneDisplay — https://docs.blender.org/api/5.1/bpy.types.SceneDisplay.html — read 2026-07-30. The manual says where it lives:

> The setting for the 3D Viewport is a user preference to specify the anti-aliasing method that runs best on the used system. The setting for viewport rendering and final rendering is saved per scene.

and lists the multisample options as "5, 8, 11, 16, 32". Source: manual, Workbench Sampling (address in answer 3), read 2026-07-30.

Each engine also carries its own *viewport* count separately from its render count: Cycles `preview_samples` ("Number of samples to render in the viewport, unlimited if 0", default 1024 — `properties.py` lines 485–492, observed) and EEVEE `taa_samples` ("Number of samples, unlimited if 0 (in [0, inf], default 16)", API page above).

**The inferred step for "all at once".** `Scene` exposes `cycles`, `eevee` and `display` at the same time (quoted in answer 3), and each of those holds its own count; therefore one saved file holds three independent values, whichever engine happens to be selected. The sources do not state that in one sentence.

**Not covered.** Whether a third-party engine's sample count is likewise stored per scene in the file (it would depend on how that add-on registers its properties). Also not covered: whether changing the engine ever rewrites another engine's stored count.

**Outside the interface.** It is inferred that these scene groups travel with the file and are therefore the same whether Blender is opened by a person or launched by a script. No dedicated launch option sets any of them (answer 1). **Observed 2026-08-05 (hand test 6, below):** a Blender started from a terminal with no window read all three counts back from the file at the values they were saved with.

**Operating system.** Nothing in the sources ties these to an operating system. The test below was run on macOS only. **Observed 2026-08-06:** the same test was run again on a rented Linux machine and came back the same, value for value, so this holds on macOS and on Linux. Windows stays untested.

### Observed — hand test 6 (2026-08-05)

Run from a terminal on the Creator's own computer — a MacBook Pro with an M1 Pro chip, macOS 26.5.2, Apple's own graphics and no other graphics card — using Blender 5.1.2 at `/Applications/Blender 5.1.2.app/Contents/MacOS/Blender`, with the `BLENDER_USER_RESOURCES` setting pointed at a fresh empty folder. Label: **observed**.

A file was saved from factory startup with Cycles samples 123, EEVEE render samples 45, Workbench final anti-aliasing `32` and the Grease Pencil final-render sample count 3. Blender was closed and a fresh Blender opened the file, reading all four values back and switching the engine between readings:

```
MADE cycles=123 eevee=45 workbench=32 gpencil=3
BACK as-saved               engine=BLENDER_EEVEE        cycles=123 eevee=45 workbench=32 gpencil=3
BACK after switch to CYCLES engine=CYCLES               cycles=123 eevee=45 workbench=32 gpencil=3
BACK after switch to BLENDER_WORKBENCH engine=BLENDER_WORKBENCH    cycles=123 eevee=45 workbench=32 gpencil=3
BACK after switch to EEVEE  engine=BLENDER_EEVEE        cycles=123 eevee=45 workbench=32 gpencil=3
```

**This settles the inferred step above and the "not covered" note with it.** One file carries all three counts at once, they survive a save and a reload, and switching the engine rewrites none of them. The Grease Pencil count is one shared value, reading 3 under all three engines.

### Observed — hand test 26, run by the Creator (2026-08-05)

Run by the Creator in Blender 5.1's own window, on their own computer — a MacBook Pro with an M1 Pro chip, macOS 26.5.2 — on 2026-08-05. That settles macOS and leaves Windows and Linux untested. Label: **observed**.

Photographed with the Render Engine set to **EEVEE**, the Sampling panel shows a **Viewport ▸ Samples** of 16 and a separate **Render ▸ Samples** of 64 — the two counts the finding above names, side by side in one panel. Photographed with the Render Engine set to **Workbench**, the whole Sampling panel is **Render — 8 Samples** and **Viewport — Single Pass Anti-Aliasing**, both menus rather than number fields. Cycles' own panel (hand test 24, under question 5) shows **Max Samples 4096** as a number field.

So each engine keeps its own render count and its own viewport count in its own panel, and Workbench's pair are fixed menus where the other two are numbers.

### Observed on the rented Linux machine — Linux hand tests 6 and 14 (2026-08-06)

Run over SSH from a terminal on the rented Linux machine, using Blender 5.1.2 at `/opt/blender/blender-5.1.2-linux-x64/blender`, with `BLENDER_USER_RESOURCES` pointed at `/root/smoozy/Settings/res-5.1`, a folder of this topic's own that already existed. That folder was checked before use and reported its own config directory and Blender's eight default add-ons and nothing else. The machine is a container sharing someone else's hardware, and up to four topics were being worked on it at once, so nothing below claims anything about how long a run took or how much memory was free. Label: **observed**.

These are a second machine's answers to the tests the Creator's Mac answered on 2026-08-05. The two are never merged: where they agree it says so, and where they differ it says that plainly. The full list is `Audits/Blender Research/Settings/Hand tests — Linux.md`.

**Linux hand test 6 — does one file hold all three counts at once, and is the Grease Pencil count shared?** A file was saved with Cycles samples 123, EEVEE render samples 45, Workbench final anti-aliasing `32` and the Grease Pencil final-render count 3, closed, and read back in a fresh Blender; then the engine was switched three times and the same four values read after each switch, in one run:

```
BACK as-loaded engine= BLENDER_EEVEE cycles= 123 eevee= 45 workbench= 32 gpencil= 3
BACK after-switch-to-CYCLES engine= CYCLES cycles= 123 eevee= 45 workbench= 32 gpencil= 3
BACK after-switch-to-BLENDER_WORKBENCH engine= BLENDER_WORKBENCH cycles= 123 eevee= 45 workbench= 32 gpencil= 3
BACK after-switch-to-BLENDER_EEVEE engine= BLENDER_EEVEE cycles= 123 eevee= 45 workbench= 32 gpencil= 3
```

So one file carries three separate counts, they survive a save and a reload, switching the engine disturbs none of them, and the Grease Pencil count reads the same under every engine. **The two machines agree**, value for value.

**Linux hand test 14 — what each engine's own block actually holds.** The EEVEE block holds **54** properties on this build and the Workbench block holds **9** — the same two counts the Mac's run gave for this version. **The two machines agree.**

---

## 5. Can a sample count change from frame to frame?

**The sources make this possible to infer, but do not establish that a render uses a changed count for each frame. LABEL: inferred for the possibility; not in the sources for the render result.**

What the sources do say. First, that properties in general carry an "animatable" flag by default: every property-creation function in the Python API is documented with the signature default `options={'ANIMATABLE'}` — for example

> `bpy.props.IntProperty(*, name='', description='', translation_context='*', default=0, min=-2**31, max=2**31 - 1, soft_min=-2**31, soft_max=2**31 - 1, step=1, options={'ANIMATABLE'}, override=set(), tags=set(), subtype='NONE', update=None, get=None, set=None, get_transform=None, set_transform=None)`

Source: API, bpy.props — https://docs.blender.org/api/5.1/bpy.props.html — read 2026-07-30.

Second, that Cycles declares its sample count without overriding that flag:

> `samples: IntProperty(name="Samples", description="Number of samples to render for each pixel", min=1, max=(1 << 24), default=4096,)`

Source (observed): `Blender Sources/cycles_5.1/properties.py`, lines 479–484. I searched the whole of `properties.py` for the string `options=` and found no occurrence anywhere in the file, so no Cycles property removes the animatable flag. Read 2026-07-30.

Third, that an animatable property can be keyed or driven:

> Decorators may appear next to number fields, menus, and checkboxes to indicate the property can be animated. … Clicking on the decorator dot icon will add a Keyframe to that property. … If a property is being driven by another, the decorator shows the driver icon.

Source: manual, Decorators — https://docs.blender.org/manual/en/5.1/interface/controls/buttons/decorators.html — read 2026-07-30.

Fourth, that a script can change data as each frame comes round:

> `bpy.app.handlers.frame_change_pre` — Called when a frame change is triggered for playback and rendering, before any data is evaluated for the new frame. This makes it possible to change data and relations (for example swap an object to another mesh) for the new frame.

Source: API, bpy.app.handlers — https://docs.blender.org/api/5.1/bpy.app.handlers.html — read 2026-07-30.

**The step I took:** the sample count is an `IntProperty` declared with the default `ANIMATABLE` option; the manual says animatable properties can take keyframes and drivers; and a handler runs on every frame change during rendering. Together these support an inference that the value can change from frame to frame. No single source says the sample count in particular can be animated, or that a render uses the changed value for that frame, so the answer is not documented.

**Where I looked, for what is missing.** I searched the whole manual, the whole API documentation and the whole 5.1 release notes for `keyframe the sample`, `animate the sample`, `animated samples` and `samples per frame`. The only `samples per frame` hits are the Graph Editor's F-Curve baking operator and an Alembic frame-rate note — nothing about render sampling. The Cycles Sampling page mentions **Use Animated Seed** ("Changes the seed for each frame") but says nothing about a changing sample count. The manual's Keyframes and Drivers pages describe the mechanism in general and never name a render setting.

**Not covered.** Whether Blender re-reads the sample count at every frame of a running animation render, or reads it once at the start of the job, is not stated anywhere I found. That is the crux of the question and the sources are silent on it. Also not covered: whether a mid-render change made by a `frame_change_pre` handler takes effect on that same frame or the next.

**Outside the interface.** It is inferred that keyframes and drivers travel with the file. A driver written as a Python expression is subject to the script-execution rules in answer 11 ("Python scripts (including driver expressions) are not executed by default for security reasons"), which may make a difference for a render started from outside Blender. The sources cited here do not establish what activates a handler stored in a file. **Observed 2026-08-05 (hand test 9, below):** saved keyframes do travel with the file and are evaluated in a render started from a terminal with no window.

**Operating system.** Nothing in the sources ties any of this to an operating system. The test below was run on macOS only. **Observed 2026-08-06:** the same tests were run again on a rented Linux machine and came back the same, so this holds on macOS and on Linux. Windows stays untested.

### Observed — hand test 9 (2026-08-05)

Run from a terminal on the Creator's own computer — a MacBook Pro with an M1 Pro chip, macOS 26.5.2 — using Blender 5.1.2 at `/Applications/Blender 5.1.2.app/Contents/MacOS/Blender`, with the `BLENDER_USER_RESOURCES` setting pointed at a fresh empty folder. Label: **observed**.

From factory startup, `keyframe_insert` was called on each engine's count and on a view layer's `use`:

```
KEY cycles.samples               -> True
KEY eevee.taa_render_samples     -> True
KEY display.render_aa            -> ERROR bpy_struct.keyframe_insert() property "render_aa" not animatable
KEY view_layer.use               -> True
```

So EEVEE's count can be keyframed and Workbench's anti-aliasing choice cannot.

A file was then saved with Cycles, 64 by 64, adaptive sampling off, denoising off, and Max Samples keyframed to 4 at frame 1 and 512 at frame 2. Rendering it with `blender -b test.blend -o out/f_ -F PNG -s 1 -e 2 -a` reported these maximum sample counts across the two frames: `4` and `512`.

**This closes the crux the finding named.** Cycles does re-read the sample count at each frame of a running animation render; it is not read once when the job starts.

### Observed — hand test 24, run by the Creator (2026-08-05)

Run by the Creator in Blender 5.1's own window, on their own computer — a MacBook Pro with an M1 Pro chip, macOS 26.5.2 — on 2026-08-05. That settles macOS and leaves Windows and Linux untested. Label: **observed**.

With the Render Engine set to **Cycles**, they opened Render Properties ▸ Sampling ▸ Render, right-clicked the **Max Samples** field, and photographed the open menu. It is headed **Samples** and lists:

```
Insert Keyframe                    I
Add Driver...                      ⌘D
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

**This is the interface half of the chain the finding above reasons through, seen rather than reasoned.** The decorators page describes a dot that can be clicked; this panel has none. What the interface offers instead is **Insert Keyframe** in the right-click menu, with `I` as its shortcut, and **Add Driver...** beside it — both routes the finding names.

### Observed on the rented Linux machine — Linux hand tests 11 and 12 (2026-08-06)

Run over SSH from a terminal on the rented Linux machine, using Blender 5.1.2 at `/opt/blender/blender-5.1.2-linux-x64/blender`, with `BLENDER_USER_RESOURCES` pointed at `/root/smoozy/Settings/res-5.1`, a folder of this topic's own that already existed. That folder was checked before use and reported its own config directory and Blender's eight default add-ons and nothing else. The machine is a container sharing someone else's hardware, and up to four topics were being worked on it at once, so nothing below claims anything about how long a run took or how much memory was free. Label: **observed**.

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

**LABEL: partly documented, partly not in the sources.**

Rendering without a graphical display is a stated use of the command line:

> One advantage of using the command line is that we do not need a graphical display (no need for X server on Linux for example) and consequently we can render via a remote shell (typically SSH).

Source: manual, Rendering From The Command Line — https://docs.blender.org/manual/en/5.1/advanced/command_line/render.html — read 2026-07-30. And background mode exists for exactly this:

> `-b, --background` — Run in background (often used for UI-less rendering).

Source: manual, Command Line Arguments (address in answer 1), read 2026-07-30.

For EEVEE there is one explicit limitation, and it is operating-system-specific:

> **Headless Rendering**
> Headless rendering is not supported on headless Windows systems.

Source: manual, EEVEE Limitations — https://docs.blender.org/manual/en/5.1/render/eevee/limitations/limitations.html — read 2026-07-30. The same page also states EEVEE cannot fall back to the processor at all:

> **CPU Rendering**
> Being a rasterization engine, EEVEE only uses the power of the GPU to render. There is no plan to support CPU (software) rendering as it would be very inefficient.

**Cycles: not in the sources.** I found no statement about Cycles and headless machines. What is stated is that Cycles can render on the processor ("**CPU:** Render using the system's central processing unit. This mode is compatible with all Cycles features." — manual, Cycles Render Settings, https://docs.blender.org/manual/en/5.1/render/cycles/render_settings/index.html, read 2026-07-30), and that GPU rendering needs a compute backend and drivers (manual, GPU Rendering — https://docs.blender.org/manual/en/5.1/render/cycles/gpu_rendering.html). Neither passage mentions a screen.

**Workbench: not in the sources.** The Workbench pages say what it is for ("optimized for fast rendering during modeling and animation preview", "By default the 3D Viewport uses Workbench to shade and light objects") but say nothing about running without a display. Source: manual, Workbench Introduction — https://docs.blender.org/manual/en/5.1/render/workbench/introduction.html — read 2026-07-30.

**Where I looked.** I searched the whole 5.1 manual for `headless` — three hits, all the EEVEE limitation just quoted and its two index entries. I searched for `X server`, `xvfb`, `no display` and `display server` — the only relevant hit is the command-line rendering sentence quoted above. I searched the whole 5.1 release notes for `headless` and `background mode` — no relevant hits. I read the whole of the EEVEE Limitations page, the Workbench Introduction, Sampling, Performance and Options pages, the Cycles Introduction and GPU Rendering pages, and both command-line pages.

**Not covered.** Whether Workbench renders on a machine with no display, at all, on any operating system. Whether EEVEE's Windows limitation has an equivalent on Linux or macOS. What "headless Windows systems" means precisely (no monitor plugged in, no interactive session, a service account, or a machine with no GPU) — the manual does not define it.

**Outside the interface.** This question is entirely about outside the interface. The documented limit is that EEVEE headless rendering is not supported on headless Windows systems. Applying that limit to a particular render node is inferred.

**Operating system.** This is where the answer splits. The only documented limitation names Windows explicitly. Nothing is said about macOS or Linux for EEVEE, and nothing at all is said for Cycles or Workbench. **Observed 2026-08-05 (hand test 14, below):** all three engines render with no window on macOS. Windows and Linux were not tested — that test was run on the Creator's one MacBook Pro. **Observed 2026-08-06:** on a rented Linux machine with one graphics card, all three engines rendered with no window and again with no display reachable at all, EEVEE included — but that machine had a display server running and its runs were only cut off from it. **Observed 2026-08-06 on a second rented Linux machine that has no display server at all** — two NVIDIA GeForce RTX 4090s, driver 580.95.05, sixty-one cores, over SSH — every engine this version has rendered there, on the cards and on the processor, EEVEE included. So **the screenless case is now answered for Linux**, in the sense of a machine with no display server. The headless *Windows* case the manual names stays untested because no Windows machine has run it — untried rather than untestable, since this project has a route to a Windows computer, described in the equipment section of `Preparation/Blender Research/Method.md`, and every machine that route has produced booted a **Server** edition, so a Windows answer would have to name its edition, and so does a machine with no graphics hardware at all.

### Observed — hand test 14 (2026-08-05)

Run from a terminal on the Creator's own computer — a MacBook Pro with an M1 Pro chip, macOS 26.5.2, Apple's own graphics and no other graphics card — using Blender 5.1.2 at `/Applications/Blender 5.1.2.app/Contents/MacOS/Blender`, with the `BLENDER_USER_RESOURCES` setting pointed at a fresh empty folder. Label: **observed**.

A file was saved from factory startup at 64 by 64, Cycles samples 4, EEVEE render samples 4, and rendered three times, once per engine, each with `-b` so no window opened:

```
CYCLES               exit=0  image=64x64
BLENDER_EEVEE        exit=0  image=64x64
BLENDER_WORKBENCH    exit=0  image=64x64
```

No error was printed by any of the three. **This closes the "not in the sources" answers for Cycles and Workbench, for macOS with no window.**

**What this does not settle.** This machine has a screen attached. Running with no window is not the same as running on a machine with no screen, and the one hard statement in the sources is about a headless *Windows* machine. That test needs a machine with no display reached over a remote shell, and a Windows machine — neither of which the Creator has. It stays unanswered, and so does what "headless Windows systems" means.

### Observed on the rented Linux machine — Linux hand tests 15 and 16 (2026-08-06)

Run over SSH from a terminal on the rented Linux machine, using Blender 5.1.2 at `/opt/blender/blender-5.1.2-linux-x64/blender`, with `BLENDER_USER_RESOURCES` pointed at `/root/smoozy/Settings/res-5.1`, a folder of this topic's own that already existed. That folder was checked before use and reported its own config directory and Blender's eight default add-ons and nothing else. The machine is a container sharing someone else's hardware, and up to four topics were being worked on it at once, so nothing below claims anything about how long a run took or how much memory was free. Label: **observed**.

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

**What this settles and what it does not.** It settles that a run which cannot reach a display at all renders on all three engines on this Linux machine, and that Blender says so plainly and stops when a window is asked for instead. It does **not** settle a machine with no screen physically attached: this box has a display running and the run was only cut off from it, which is a narrower thing. And it does not touch the manual's one hard statement, which names a headless *Windows* machine — no Windows machine has run it — untried rather than untestable, since this project has a route to a Windows computer, described in the equipment section of `Preparation/Blender Research/Method.md`, and every machine that route has produced booted a **Server** edition, so a Windows answer would have to name its edition. So this question is now answered for Linux in the sense of "no display reachable", and stays open for "no screen attached" and for Windows.

### Observed on a machine with no screen — the two-card rented Linux machine (2026-08-06)

Run over SSH on a **second rented Linux machine — Ubuntu 22.04.5 LTS, two NVIDIA GeForce RTX 4090s of 24 GB each on driver 580.95.05, an AMD EPYC 7B13 with sixty-one cores, 98 GB of memory, no display server at all** — using Blender at `/opt/blender/blender-5.1.2-linux-x64/blender`, which `--version` reports as `Blender 5.1.2 (hash ec6e62d40fa9 built 2026-05-19 02:18:37)`, with `BLENDER_USER_RESOURCES` pointed at `/root/smoozy/no screen/res-5.1`, a folder of its own that already existed and was empty. Nothing else was running on the machine. Label: **observed**. The full list is `Audits/Blender Research/Settings/Hand tests — two cards and no screen.md`.

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
GPU  CYCLES exit=0  png=5065
GPU  BLENDER_EEVEE_NEXT exit=0  png=3431
GPU  BLENDER_EEVEE exit=0  png=3431
GPU  BLENDER_WORKBENCH exit=0  png=3669
CPU  CYCLES exit=0  png=5062
CPU  BLENDER_EEVEE_NEXT exit=0  png=3431
CPU  BLENDER_EEVEE exit=0  png=3431
CPU  BLENDER_WORKBENCH exit=0  png=3669
```

All of them rendered and wrote their image, **EEVEE included**, with no error and nothing about a display in any of them. This version accepts both `BLENDER_EEVEE` and `BLENDER_EEVEE_NEXT` and gives the same file size for each; its own `-E help` lists `BLENDER_EEVEE`, `BLENDER_WORKBENCH` and `CYCLES`, exit 0.

The same runs repeated with `--debug-cycles` showed the work really was where it was asked for:

```
on the card:      Path tracing on: NVIDIA GeForce RTX 4090 (OptiX) [CUDA_NVIDIA GeForce RTX 4090_0000:00:0a_OptiX]
                                   NVIDIA GeForce RTX 4090 (OptiX) [CUDA_NVIDIA GeForce RTX 4090_0000:00:07_OptiX]
on the processor: Path tracing on: AMD EPYC 7B13 64-Core Processor (CPU) (61 threads) [CPU]
```

Both cards are named in the card run, with a line reading `Hardware Ray-Tracing: Off` under each name in this version. Nothing here says how the work was divided between the two cards, or which cards were switched on in the preferences, and nothing is read into it. *(Restored by the two-card citation check, 2026-08-06: this block had been shortened to one card with an ellipsis, while the record it comes from shows two.)*

**And what Blender says when asked for a window on a machine with no display server.** The same command without `-b`, given `--python-expr "import bpy; print('RAN WITH A WINDOW')"` under `timeout 120`. `RAN WITH A WINDOW` was never printed. It printed:

```
00:00.058  ghost.system     | Failed to initialize display for back-end(s): ['WAYLAND', 'X11']
                            |   'WAYLAND': unable to connect to display!
                            |   'X11': unable to open a display!
                            | 
00:00.058  ghost.system     | ERROR Unable to initialize GHOST, exiting!
Error: Not freed memory blocks: 4645, total unfreed memory 1.212318 MB
```

on **standard output**, and exited **1**. Nothing was written to standard error. Word for word what the one-card machine printed when cut off from its display, with the same exit status, plus the memory-leak line that machine never showed.

**What this settles.** **On Linux, every render engine this version has renders on a computer with no display server at all** — not merely a run cut off from a display. That is the half of this question item A of the Linux list left open, and it is now answered for Linux. **It does not settle a machine with no screen physically attached in the sense of no graphics hardware**: the cards are present, they are simply not driving a display. And it does not touch the manual's one hard statement, which names a headless *Windows* machine — no Windows machine has run it — untried rather than untestable, since this project has a route to a Windows computer, described in the equipment section of `Preparation/Blender Research/Method.md`, and every machine that route has produced booted a **Server** edition, so a Windows answer would have to name its edition.

---

## 7. Does a scene set up for one render engine come out the same in another?

**No. LABEL: documented.**

> What the render looks like is defined by cameras, lights and materials. These are shared between EEVEE and Cycles, however some features are only supported in one or the other.

Source: manual, Rendering — Introduction — https://docs.blender.org/manual/en/5.1/render/introduction.html — read 2026-07-30.

> EEVEE materials are created using the same shader nodes as Cycles, making it easy to render existing scenes. … EEVEE is based on rasterization and is not a path tracer. … While EEVEE is designed to use PBR principles, it is not perfect and Cycles will always provide more physically accurate renders. For these reasons, EEVEE has a set of limitations.

Source: manual, EEVEE Introduction — https://docs.blender.org/manual/en/5.1/render/eevee/introduction.html — read 2026-07-30.

The clearest single sentence:

> All BSDF's are using approximations to achieve realtime performance so there will always be small differences between Cycles and EEVEE.

Source: manual, EEVEE Limitations (address in answer 6), read 2026-07-30. The same page lists concrete divergences, among them:

> Only perspective and orthographic projections are currently supported.
> Lights can only have one color and do not support light node trees.
> Unlike in Cycles, the Size of spot lights does not change the softness of the cone.
> The area light Beam spread option is not supported.
> Only single scattering is supported.
> Volumetrics are rendered only for the camera "rays". They don't appear in reflections/refractions and probes.
> Some utility nodes are not yet compatible with EEVEE.
> EEVEE generally performs all calculations using half-precision (16-bit) floating point numbers. … The render output subsequently also only has the precision of half-precision (16-bit) floating point numbers.
> The Combined and Light data passes clamp negative output values to zero.

Workbench differs more fundamentally:

> Unlike other render engines such as EEVEE or Cycles, the Workbench engine does not use shader nodes. Instead, shading settings can be tweaked in the 3D Viewport's Shading popover or the render properties when doing final renders.

and

> The Workbench Engine is a render engine optimized for fast rendering during modeling and animation preview. It is not intended to be a render engine that will render final images for a project.

Source: manual, Workbench Introduction (address in answer 6), read 2026-07-30.

One more, about passes rather than pixels:

> The viewport compositor only supports render passes when using EEVEE. For other engines, the passes will be empty.

Source: manual, Render Layers Node — https://docs.blender.org/manual/en/5.1/compositing/types/input/scene/render_layers.html — read 2026-07-30.

**Not covered.** How large "small differences" are, in any measurable sense. Which specific utility nodes are unsupported — the page points at a separate "Nodes Support" list I did not enumerate. Nothing here covers third-party engines. And nothing here says what happens to Cycles-only *settings* when the engine is switched — only what happens to the picture.

**Outside the interface.** It is inferred that the differences hold for a render launched from outside, because the sources describe engine behaviour rather than interface actions. Combining that with `-E` (answer 1) supports a further inference that a render started from outside can produce a different-looking picture from the one saved in the file.

**Operating system.** Nothing in the sources ties these differences to an operating system.

---

## 8. Can one scene be set up to take in another scene's rendered result, so that rendering the first renders the second?

**The sources document two ways to use another scene's render output. LABEL: documented. Whether rendering the first scene starts a fresh render of the other is not established by the quoted material. LABEL: not in the sources in this finding; further documentation research is needed.**

**Way one — the compositor's Render Layers node.**

> Renders a View Layer and reads its Passes into the compositing node graph.
> **Properties**
> **Scene** — The scene for which to render a view layer.
> **View Layer** — The view layer to render. The button next to the dropdown re-renders it immediately.
> Hint: To use the compositing output from another scene rather than its "raw" render output, first render that scene into a series of multi-layered images (using e.g. the OpenEXR format), then load those images into the Compositor of the current scene using the Image Node.

Source: manual, Render Layers Node (address in answer 7), read 2026-07-30. The API confirms the node holds a scene pointer: "`scene` — Type: `Scene`", on a class described as "Input render passes from a scene render". Source: API, CompositorNodeRLayers — https://docs.blender.org/api/5.1/bpy.types.CompositorNodeRLayers.html — read 2026-07-30.

For the compositor to run at all:

> To make the render engine actually use the compositing node tree (or disable compositing), the Compositing Pipeline option must be enabled in the output's Post Processing properties.

Source: manual, Compositing — Introduction — https://docs.blender.org/manual/en/5.1/compositing/introduction.html — read 2026-07-30.

**Way two — a Scene strip in the sequencer.**

> Scene strips are a way to insert the render output of another scene into your sequence. Instead of rendering out a video, then inserting the video file, you can insert the scene directly.
> The strip length will be determined based on the animation settings in that scene.
> Note: Scene strips cannot be used to reference the sequence's own scene; a secondary scene must be used instead.

with an input choice:

> **Input** — Input type to use for the Scene strip.
> **Camera:** Use the Scene's 3D camera as input.
> **Sequencer:** Use the scene's Sequencer timeline as input, allowing one scene to reuse another scene's edit (instead of taking the render output from the scene).

and a limitation:

> Scene strips do not render individual Render Passes; only the Combined render pass will be used.

Source: manual, Scene Strip — https://docs.blender.org/manual/en/5.1/video_editing/edit/montage/strips/scene.html — read 2026-07-30.

**Not covered.** The Render Layers hint says the *compositing* output of the other scene is not what you get — you get its raw render — but the sources do not spell out what else differs (for instance which scene's frame range, resolution or colour management wins). Nor do they say what happens when the two scenes use different render engines, or whether the second scene's render honours its own sample count. Nothing states how deep the chaining may go (scene A pulling scene B pulling scene C).

**Outside the interface.** It is inferred that the two mechanisms travel with a file and can be used by a render launched from outside. One caveat sits in the sources: the Render Layers node's "button next to the dropdown re-renders it immediately" is an interface action and has no outside-the-interface equivalent given. The Scene strip route also depends on `use_sequencer` being on (`RenderSettings.use_sequencer`, API page in answer 3). **Observed 2026-08-05 (hand tests 17, 18, 19 and 16, below):** both mechanisms do work in a render started from a terminal with no window.

**Operating system.** Nothing in the sources ties either mechanism to an operating system. The tests below were run on macOS only. **Observed 2026-08-06:** the same tests were run again on a rented Linux machine and came back the same, so this holds on macOS and on Linux. Windows stays untested.

### Observed — hand tests 16, 17, 18 and 19 (2026-08-05)

All were run from a terminal on the Creator's own computer — a MacBook Pro with an M1 Pro chip, macOS 26.5.2 — using Blender 5.1.2 at `/Applications/Blender 5.1.2.app/Contents/MacOS/Blender`, with the `BLENDER_USER_RESOURCES` setting pointed at a fresh empty folder. Label: **observed**.

**Hand test 17 — a Render Layers node pointed at another scene.** A file was saved with two scenes. `Alpha` used Workbench at 64 by 64 with its compositing pipeline on, and its compositing node group held one Render Layers node with Scene set to `Beta`, wired to the group output. `Beta` used Cycles with 8 samples, and its own node group inverted its render. Rendering `Alpha`:

```
blender -b test.blend -S Alpha -o out/c_ -F PNG -f 1
00:00.318  render           | Start rendering: Beta, ViewLayer
00:00.318  render           | Engine: Cycles
00:00.322  render           | Fra: 1 | Mem: 1M | Sample 0/8
00:00.322  render           | Fra: 1 | Remaining: 00:00.00 | Mem: 1M | Sample 1/8
00:00.374  render           | Fra: 1 | Mem: 1M | Sample 8/8
```

**This closes the "not in the sources" answer at the head of this finding:** rendering the first scene does start a fresh render of the other, and it uses that scene's own engine and its own sample count.

Three images built from the same file were then compared — `Alpha`'s output, `Beta` rendered directly with its compositing pipeline on, and `Beta` rendered directly with it off. The first pixel of each:

```
alpha_0001.png    [0.0, 0.0, 0.0, 1.0]
betacomp_0001.png [0.7686, 0.7686, 0.7686, 1.0]
betaraw_0001.png  [0.0, 0.0, 0.0, 1.0]
```

`Alpha`'s output matches `Beta`'s raw render, not `Beta`'s composited one. **So the second scene's own compositor does not run** — what the manual's Hint implied is what happens.

**Hand test 18 — two scenes' Render Layers nodes pointed at each other.** With `Alpha`'s node pointed at `Beta` and `Beta`'s at `Alpha`, both pipelines on, both Workbench, rendering `Alpha` with a sixty-second cut-off finished in 0.7 seconds, exit code 0, and wrote an image:

```
00:00.333  render           | Start rendering: Beta, ViewLayer
00:00.333  render           | Engine: Workbench
00:00.418  render           | Executing compositor
00:00.418  render           | Fra: 1 | Compositing
Scene switch for render: 'Alpha' in file: '.../circular.blend'
00:00.468  render           | Saved: '.../out/x_0001.png'
```

No error, no hang, and `Beta` was rendered once rather than the chain repeating.

**Hand test 19 — a Scene strip.** A file was saved with `Alpha` on Workbench, its sequencer pipeline on and one Scene strip referencing `Beta` at frame 1; `Beta` used Cycles with 8 samples. Rendering `Alpha` from the command line printed `Start rendering: Beta, ViewLayer`, `Engine: Cycles` and `Sample 8/8`, and wrote an image.

**Hand test 16 — a Sequencer Scene against the active scene.** A file was saved with three scenes: `Alpha`, `Beta`, and `Edit` whose sequencer held a Scene strip referencing `Alpha`. The workspace's sequencer scene was set to `Edit` and the window's scene to `Beta`. A bare `blender -b test.blend -o out/s_ -F PNG -f 1`, with a handler added at launch that names the scene the render starts on, printed:

```
RENDER_PRE SCENE: Beta
```

So a render started from outside uses the active scene and ignores the Sequencer Scene.

### Observed on the rented Linux machine — Linux hand tests 19, 20 and 21 (2026-08-06)

Run over SSH from a terminal on the rented Linux machine, using Blender 5.1.2 at `/opt/blender/blender-5.1.2-linux-x64/blender`, with `BLENDER_USER_RESOURCES` pointed at `/root/smoozy/Settings/res-5.1`, a folder of this topic's own that already existed. That folder was checked before use and reported its own config directory and Blender's eight default add-ons and nothing else. The machine is a container sharing someone else's hardware, and up to four topics were being worked on it at once, so nothing below claims anything about how long a run took or how much memory was free. Label: **observed**.

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

**For Cycles, the sources show a scene setting and a launch override. LABEL: documented for the setting's purpose; observed for the property and override; inferred for what a saved file records.**

The manual describes a choice between processor and graphics card for Cycles:

> **Device** — Specifies the compute device used for rendering in Cycles. The choice of device can significantly affect render performance, GPU Compute is often faster.
> **CPU:** Render using the system's central processing unit. This mode is compatible with all Cycles features. The number of CPU threads used can be changed in the Performance Properties.
> **GPU Compute:** Render using one or more graphics processing units (GPUs) through the compute backend and devices selected in the Cycles Render Devices. Depending on the backend, it may be possible to use both CPU and GPU devices simultaneously.

Source: manual, Cycles Render Settings — https://docs.blender.org/manual/en/5.1/render/cycles/render_settings/index.html — read 2026-07-30.

The setting itself, with its default. LABEL: observed.

> ```
> device: EnumProperty(
>     name="Device",
>     description="Device to use for rendering",
>     items=enum_devices,
>     default='CPU',
>     update=update_render_passes,
> )
> ```

Source: `Blender Sources/cycles_5.1/properties.py`, class `CyclesRenderSettings`, lines 381–387. Read 2026-07-30. As established in answer 3, `CyclesRenderSettings` is attached to `bpy.types.Scene` as `Scene.cycles`. Treating that scene property as a value saved in the file is inferred here.

**The sources place selection of available graphics cards in Preferences. Treating that as not saved in the file is inferred.**

> To enable GPU rendering:
> Select either CUDA, OptiX, HIP, oneAPI, or Metal in the Cycles Render Devices user preference.
> Enable the available devices for the chosen backend.
> Configure each scene to use the GPU Compute Device.

Source: manual, GPU Rendering — https://docs.blender.org/manual/en/5.1/render/cycles/gpu_rendering.html — read 2026-07-30. And:

> **Cycles Render Devices** — Changes the Computing Device the Cycles render engine uses to render images. Cycles can use either the CPU or certain GPUs to render images…
> **None:** When set to None or when the only option is None: the CPU will be used as the computing device for Cycles.

Source: manual, Preferences — System — https://docs.blender.org/manual/en/5.1/editors/preferences/system.html — read 2026-07-30.

**And it can be overridden at launch, above both.** LABEL: documented, and confirmed observed.

> `--cycles-device <device>` — Set the device used for rendering. Valid options are: CPU CUDA OPTIX HIP ONEAPI METAL. Append `+CPU` to a GPU device to render on both CPU and GPU.
> Example: `blender -b file.blend -f 20 -- --cycles-device OPTIX`

Source: manual, Command Line Arguments (address in answer 1), read 2026-07-30. Cycles' own file is more explicit about what it overrides:

> ```
> parser.add_argument("--cycles-device",
>                     help="Set the device to use for Cycles, overriding user preferences and the scene setting."
>                          "Valid options are 'CPU', 'CUDA', 'OPTIX', 'HIP', 'ONEAPI', or 'METAL'."
>                          "Additionally, you can append '+CPU' to any GPU type for hybrid rendering.",
>                     default=None)
> ```

Source (observed): `Blender Sources/cycles_5.1/engine.py`, lines 16–20. Read 2026-07-30. Note the manual page for the option does not say it overrides both the preference and the scene; Cycles' own file does.

**The inferred step.** Putting the three together: a file is inferred to record *whether* processor or graphics card, but not *which* graphics card or which backend, and the observed launch option overrides both. The sources never state that summary in one place.

**Not covered.** EEVEE and Workbench have no equivalent recorded device choice that I found — see answer 6 for EEVEE's statement that it "only uses the power of the GPU to render". Also not covered: what happens when a file saved with GPU Compute is opened on a machine whose preferences have no compute device enabled. Note also that the compositor and the denoiser have their own separate device choices ("Denoising and Compositing have their own device selection", Cycles Render Settings page), which this answer does not cover.

**Outside the interface.** The observed Cycles argument parser says that `--cycles-device` overrides user preferences and the scene setting. A bare `--` ends Blender option processing; the manual's example places this Cycles option after it. It is inferred that a farm can therefore force the device regardless of what the file says.

**Operating system.** The valid device values are documented as operating-system-bound: "CUDA is supported on Windows and Linux", "HIP is supported on Windows and Linux", "oneAPI … supported on Windows and Linux", "Metal for GPU acceleration is supported on Apple computers with Apple Silicon. macOS 13.0 or newer is required to support all features." Source: manual, GPU Rendering (address above), read 2026-07-30. It is inferred that a file recording GPU Compute is portable while a launch option naming a backend is not. **Observed 2026-08-05:** on this machine Metal is the technology found; CUDA, OptiX, HIP and oneAPI could not be tried at all, so what a file's `GPU` choice means on any other make of card stays untested. **Observed 2026-08-06:** a rented Linux machine with an NVIDIA GeForce RTX 4090 settles most of what was left here. That machine offers `CUDA`, `OPTIX`, `HIP` and `ONEAPI` and not `METAL`; `--cycles-device CUDA` and `OPTIX` render on the card, and `HIP`, `ONEAPI` and `METAL` are all refused with the same message. A file asking for GPU Compute renders on the card under CUDA or OptiX and falls back to the processor under the others — with nothing printed at all, except in the `HIP` case, where one warning line reads `WARNING HIPEW initialization failed: Error opening HIP dynamic library`. What an AMD or an Intel card does stays untested, and so does Windows. *(Changed by the citation check of the Linux hand tests, 2026-08-06: this line said "silently" for every case, which the block below already contradicts for HIP in this version.)*

### Observed — hand tests 7 and 8 (2026-08-05)

Both were run from a terminal on the Creator's own computer — a MacBook Pro with an M1 Pro chip, macOS 26.5.2, Apple's own graphics and no other graphics card — using Blender 5.1.2 at `/Applications/Blender 5.1.2.app/Contents/MacOS/Blender`, with the `BLENDER_USER_RESOURCES` setting pointed at fresh empty folders so the Creator's own Blender settings were untouched. Label: **observed**.

**Hand test 7 — does the value survive a save, and is it per scene?** A file was saved from factory startup with two scenes: `One` with the Cycles device set to `GPU` and the compositor device set to `GPU`, and `Two` left at its defaults. A fresh Blender opened it and printed:

```
SCENE One cycles_device= GPU compositor_device= GPU
SCENE Two cycles_device= CPU compositor_device= CPU
```

**This settles the inferred step above.** The file does record the choice, it survives a save and a reload, and it is held one scene at a time.

**Hand test 8 — what `--cycles-device` beats, and the double dash.** A file was saved with Cycles on the processor, 100 by 100, 16 samples. A throwaway settings folder was prepared with Metal chosen; it reported:

```
DEVICE: CPU Apple M1 Pro use= True
DEVICE: METAL Apple M1 Pro (GPU - 14 cores) use= True
SAVED compute_device_type= METAL
```

With the file still saying `CPU`, `blender -b test.blend -o out/a_ -F PNG -f 1 -- --cycles-device METAL --cycles-print-stats` produced statistics carrying the line `Profiling information not available (only works with CPU rendering)`, which the same run without the override did not carry. So the argument does beat the file's own setting, as Cycles' own help text claims.

Without the double dash:

```
blender -b test.blend -o out/b_ -F PNG -f 1 --cycles-device METAL
unknown argument, loading as file: --cycles-device
00:00.587  reports          | ERROR Cannot read file ".../--cycles-device": No such file or directory
```

With a name no device bears:

```
blender -b test.blend -o out/c_ -F PNG -f 1 -- --cycles-device NOT_A_DEVICE
00:00.343  cycles           | ERROR NOT_A_DEVICE is not a valid Cycles device.
Error: Not freed memory blocks: 1, total unfreed memory 0.000004 MB
```

and no image was written.

**A file asking for the graphics card on a machine with none enabled.** A second throwaway settings folder was prepared with the compute device type set to `NONE`, and the scene's device forced to `GPU` at launch. The run printed nothing about the device and no warning of any kind; its statistics carried no `Profiling information not available` line, and the image was written. **This closes the gap the finding named:** it renders on the processor and says nothing about it.

**Workbench.** Hand test 13 (under question 3) listed every property of `scene.display`: nine, none of them a device choice.

### Observed — hand test 27, run by the Creator (2026-08-05)

Run by the Creator in Blender 5.1's own window, on their own computer — a MacBook Pro with an M1 Pro chip, macOS 26.5.2 — on 2026-08-05. That settles macOS and leaves Windows and Linux untested. Label: **observed**.

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

Those are the compositor's and the denoise nodes' devices — the separate device choices the Cycles Render Settings page points at — shown here under Workbench because the tab carries them whatever the engine. **No control anywhere in the tab is a Workbench device setting.** The only other control naming a device is Color Management ▸ **Display Device**, which is a colour display device.

**So the "not covered" note above is settled for Workbench:** it has no equivalent recorded device choice, and what it does show belongs to compositing and denoising. The Compositor Device reading `CPU` here also matches what hand test 7 read back from a file in this version.

### Observed on the rented Linux machine — Linux hand tests 7, 8, 9, 10 and 25 (2026-08-06)

Run over SSH from a terminal on the rented Linux machine, using Blender 5.1.2 at `/opt/blender/blender-5.1.2-linux-x64/blender`, with `BLENDER_USER_RESOURCES` pointed at `/root/smoozy/Settings/res-5.1`, a folder of this topic's own that already existed. That folder was checked before use and reported its own config directory and Blender's eight default add-ons and nothing else. The machine is a container sharing someone else's hardware, and up to four topics were being worked on it at once, so nothing below claims anything about how long a run took or how much memory was free. Label: **observed**.

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

**Linux hand test 25 — Workbench's own Render Properties tab, photographed on this machine.** Blender was started with a real window on the machine's display, the engine set to Workbench, the Properties editor pointed at Render, and the window photographed with Blender's own screenshot command. Blender 5.1's tab lists Curves, Simplify, Performance, Sampling, Lighting and Object Color from the top. No control in the tab is a Workbench device setting, matching the Creator's photographs of the same tab on the Mac. **The two machines agree.**

---

## 10. What does Blender state about opening a file that was saved by a different version of Blender?

**LABEL: documented.** The manual has a page on exactly this.

> Blender can open blend-files saved with both older versions of the software (backward compatibility), and newer ones (forward compatibility). This comes with some limitations though.
> Tip: When having issues with opening much older (or newer) blend-files, it can help to use a few intermediary Blender releases to perform conversions by smaller steps.

> **Backward Compatibility**
> Opening older files and converting them for the current version of Blender is usually straight-forward. It is expected to give very good and usable results.
> There can be major feature changes, for which the backward compatibility will only be ensured for a limited amount of time. … This will never be less than a full major release cycle (i.e. two years at least).

> **Forward Compatibility**
> **Loss of Data**
> Forward compatibility is inherently harder to ensure, and loss of feature should always be expected when opening a blend-file saved with a more recent version of Blender.
> A warning is shown in the UI when editing a more recent blend-file. Trying to overwrite it (with a simple 'Save' operation) will also show a confirmation popup, as this could make that loss of data permanent.

> **Complete Incompatibility**
> When Blender switches to a new major version release (e.g. from 3.x to 4.0), there can also be major changes that will make the blend-file fully incompatible with older versions of Blender.
> In such cases, older Blender will fail opening (or appending/linking from) the newer blend-file, with a message stating which minimal version is needed to open it.
> In such cases, the last LTS release of the previous release cycle will be kept compatible with the newer file format version, and will be usable as converter between both versions.

Source: manual, Compatibility — https://docs.blender.org/manual/en/5.1/files/blend/compatibility.html — read 2026-07-30.

The file records the version it was saved with, and Blender exposes it:

> `version` — File format version the .blend file was saved with (array of 3 items, in [0, inf], default (0, 0, 0), readonly)

Source: API, BlendData — https://docs.blender.org/api/5.1/bpy.types.BlendData.html — read 2026-07-30. And:

> `bpy.app.version_file` — The Blender File version, as a tuple of 3 numbers (major, minor, file sub-version), that will be used to save a .blend file. … This value is, and should be, used for handling compatibility changes between Blender versions

Source: API, bpy.app — https://docs.blender.org/api/5.1/bpy.app.html — read 2026-07-30.

There is a documented hook for conversion work: `bpy.app.handlers.version_update` — "on ending the versioning code". Source: API, bpy.app.handlers (address in answer 5), read 2026-07-30. Cycles uses it, and its own file shows conversion driven by the file's recorded version. LABEL: observed.

> ```
> # Map of versions used by libraries.
> library_versions = {}
> library_versions[bpy.data.version] = [None]
> for library in bpy.data.libraries:
>     library_versions.setdefault(library.version, []).append(library)
>
> # Do versioning per library, since they might have different versions.
> max_need_versioning = (5, 0, 77)
> for version, libraries in library_versions.items():
>     if version > max_need_versioning:
>         continue
>
>     # Scenes
>     for scene in bpy.data.scenes:
>         if scene.library not in libraries:
>             continue
>
>         # Auto tiling is always enabled now
>         if version <= (5, 0, 77):
>             cscene = scene.cycles
>             if not cscene.use_auto_tile:
>                 cscene.use_auto_tile = True
>                 cscene.tile_size = 8192
> ```

Source: `Blender Sources/cycles_5.1/version_update.py`, function `do_versions`, lines 97–119. Read 2026-07-30. The same function carries branches back to `(2, 70, 0)`, and the file also contains:

> `# We don't modify startup file because it assumes to have all the default values only.`
> `if not bpy.data.is_saved:`
> `    return`

(lines 92–95). The handler is registered as `bpy.app.handlers.version_update.append(version_update.do_versions)` in `Blender Sources/cycles_5.1/__init__.py`, line 155, and is decorated `@persistent`, which the API documents as "Function decorator for callback functions not to be removed when loading new files".

The 5.1 release notes name two version-crossing changes:

> Node Tools now have a global unique idname requirement. This can be set automatically by opening and saving files in 5.1, or chosen manually in the node editor header.

> Grease Pencil fills have been revamped. Blend-files are automatically converted to the new system but some procedural setups using Geometry Nodes and Grease Pencil materials might need manual adjustments.

Source: release notes, https://developer.blender.org/docs/release_notes/5.1/ — read 2026-07-30.

**Not covered.** The compatibility page says a warning is shown "in the UI" and that saving raises "a confirmation popup" — it does not say what happens when the same file is loaded with no interface. That gap matters for automated rendering and I could not close it from these sources. Also not covered: whether opening an older file that needs conversion changes the render result, and what the minimum-version failure message actually says.

**Outside the interface.** Explicitly a gap, as just noted: the two stated consequences of forward incompatibility are both interface events (a warning and a confirmation popup). It is inferred, not documented here, that Cycles' versioning code is not interface-bound because it is attached to a persistent version-update handler. **Observed 2026-08-05 (hand test 22, below):** a warning is printed to the terminal, and the render goes ahead.

**Operating system.** Nothing in the sources ties file compatibility to an operating system. The test below was run on macOS only. **Observed 2026-08-06:** the same test was run again on a rented Linux machine and came back the same, so this holds on macOS and on Linux. Windows stays untested.

### Observed — hand test 22 (2026-08-05)

Run from a terminal on the Creator's own computer — a MacBook Pro with an M1 Pro chip, macOS 26.5.2 — with the `BLENDER_USER_RESOURCES` setting pointed at a fresh empty folder. Label: **observed**.

A file was saved from factory startup by Blender 5.2.0 (Workbench, 64 by 64), then opened and rendered by Blender 5.1.2 with no window:

```
blender -b from-5.2.blend -o out/5.1_ -F PNG -f 1
00:00.328  reports          | WARNING File written by newer Blender binary (502.44), expect loss of data!
Blender 5.1.2 (hash ec6e62d40fa9 built 2026-05-19 01:30:33)
00:00.329  render           | Rendering single frame
00:00.329  render           | Rendering frame 1
00:00.329  render           | Start rendering: Scene, ViewLayer
00:00.329  render           | Engine: Workbench
00:00.464  render           | Saved: '.../out/5.1_0001.png'
00:00.464  render           | Time: 00:00.13 (Saving: 00:00.04)
```

Exit code 0, image written. **This closes the gap the finding named:** the forward-compatibility warning does reach a terminal, in those words, and the render goes ahead rather than being refused.

### Observed on the rented Linux machine — Linux hand tests 24 (2026-08-06)

Run over SSH from a terminal on the rented Linux machine, using Blender 5.1.2 at `/opt/blender/blender-5.1.2-linux-x64/blender`, with `BLENDER_USER_RESOURCES` pointed at `/root/smoozy/Settings/res-5.1`, a folder of this topic's own that already existed. That folder was checked before use and reported its own config directory and Blender's eight default add-ons and nothing else. The machine is a container sharing someone else's hardware, and up to four topics were being worked on it at once, so nothing below claims anything about how long a run took or how much memory was free. Label: **observed**.

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

**Yes it can, and what decides is the auto-execution setting, which defaults to off and can be overridden at launch. LABEL: documented.**

> The ability to include Python scripts within blend-files is valuable for advanced tasks such as rigging and automation. However, it poses a security risk since Python does not restrict what a script can do. Therefore, you should only run scripts from sources you know and trust. Automatic execution is disabled by default; however, some blend-files need this to function properly.

> **Scripts in Blend-Files**
> **Auto Execution**
> Here are the different ways blend-files may automatically run scripts.
> **Registered Text-Blocks** — A text data-block can have its Register option enabled which means it will load on start.
> **Animation Drivers** — Python expressions can be used to Drive values and are often used in more advanced rigs and animations.

> **Manual Execution**
> There are other ways scripts in a blend-file may execute that require user interaction (therefore will run even when auto execution is off), but you should be aware that this is the case since it is not necessarily obvious.
> Running a script in the Text editor.
> Rendering with Freestyle, because Freestyle uses scripts to control line styles.

> **Command Line**
> You may want to perform batch rendering or some other task from the command line, running Blender without an interface. In this case, the Preferences are still used but you may want to override them:
> Enable with `-y` or `--enable-autoexec`
> Disable with `-Y` or `--disable-autoexec`
> Example
> To render an animation in background mode, allowing drivers and other scripts to run:
> `blender --background --enable-autoexec my_movie.blend --render-anim`
> Note: These command-line arguments can be used to start a regular Blender instance and will still override the Preferences.

Source: manual, Scripting & Security — https://docs.blender.org/manual/en/5.1/advanced/scripting/security.html — read 2026-07-30.

The Command Line Arguments page states the default plainly:

> `-y, --enable-autoexec` — Enable automatic Python script execution.
> `-Y, --disable-autoexec` — Disable automatic Python script execution (Python-drivers & startup scripts), (default).

Source: manual, Command Line Arguments (address in answer 1), read 2026-07-30.

The preference it overrides:

> **Auto Run Python Scripts** — Python scripts (including driver expressions) are not executed by default for security reasons. You may be working on projects where you only load files from trusted sources, making it more convenient to allow scripts to be executed automatically.
> **Excluded Paths** — Blend-files in these folders will not automatically run Python scripts. This can be used to define where blend-files from untrusted sources are kept.

Source: manual, Preferences — Save & Load — https://docs.blender.org/manual/en/5.1/editors/preferences/save_load.html — read 2026-07-30.

There is also a per-open interface control that has no launch equivalent given:

> First, the File Browser has the option Trusted Source which you can use on a case-by-case basis to control auto execution.

(Same Scripting & Security page.)

The API also lists render handlers: `render_init` ("on initialization of a render job"), `render_pre`, `render_post`, `render_write` ("on writing a render frame (directly after the frame is written)"), `render_complete`, `render_stats`, and `frame_change_post` ("Called after frame change for playback and rendering") and `frame_change_pre` ("Called when a frame change is triggered for playback and rendering"). Source: API, bpy.app.handlers (address in answer 5), read 2026-07-30. *[Citation check, 2026-08-05: the two handlers were quoted under one sentence that is only `frame_change_post`'s in this version; `frame_change_pre`'s own wording is now given, as answer 5 already had it.]* The quoted material does not establish that a file itself carries these handlers.

**Not covered, plus one discrepancy worth flagging.** The sources do not say whether Freestyle's line-style scripts run in background mode without `--enable-autoexec` — the page says manual-execution routes "will run even when auto execution is off", and lists Freestyle rendering among them, but a background render is not obviously "user interaction". I could not settle that. Also not covered: what happens (error, silent skip, wrong render) when a file needs its scripts and they are not allowed to run. Finally, a discrepancy in the sources themselves: the Command Line Arguments page documents `--python-use-system-env` ("Allow Python to use system environment variables such as PYTHONPATH and the user site-packages directory"), while the 5.1 release notes say "The Python user-site-directory is no longer loaded by default … Use the `--python-use-user-env` command line argument to restore access to user modules" (release notes, https://developer.blender.org/docs/release_notes/5.1/core/, read 2026-07-30). The two name different options; I am reporting both rather than choosing.

**Outside the interface.** Directly answered above: the preference still applies, `-y`/`-Y` override it, the default is off, and the Trusted Source per-file tick has no stated command-line equivalent. **Observed 2026-08-05 (hand test 20, below):** with neither flag the preference is followed, and a blocked driver prints a line and yields zero rather than stopping the render.

**Operating system.** Neither `-y`/`-Y` nor the Auto Run preference carries any operating-system qualifier, on pages where other options are explicitly marked "Windows only" or "Linux only". Reading that absence as "the same everywhere" is inferred. The test below was run on macOS only. **Observed 2026-08-06:** the same test was run again on a rented Linux machine and gave the same six results, so this holds on macOS and on Linux. Windows stays untested.

### Observed — hand tests 20 and 2 (2026-08-05)

Both were run from a terminal on the Creator's own computer — a MacBook Pro with an M1 Pro chip, macOS 26.5.2 — using Blender 5.1.2 at `/Applications/Blender 5.1.2.app/Contents/MacOS/Blender`, with the `BLENDER_USER_RESOURCES` setting pointed at fresh empty folders so the Creator's own Blender settings were untouched. Label: **observed**.

**Hand test 20 — what a render started from outside does with a file's scripts.** A file was saved from factory startup with a scripted-expression driver on the default cube's x position, using `__import__("os").getpid()*0+7`, which is outside the simple-expressions subset. The driven value was read back after `frame_set(1)`, against two settings folders — one with Auto Run Python Scripts off, one with it on:

```
Auto-Run pref=off flag=none -> CUBE X = 0.0   BPY_driver_exec: restricted access disallows name '__import__', enable auto-execution to support
Auto-Run pref=off flag=-y   -> CUBE X = 7.0
Auto-Run pref=off flag=-Y   -> CUBE X = 0.0   BPY_driver_exec: restricted access disallows name '__import__', enable auto-execution to support
Auto-Run pref=on  flag=none -> CUBE X = 7.0
Auto-Run pref=on  flag=-y   -> CUBE X = 7.0
Auto-Run pref=on  flag=-Y   -> CUBE X = 0.0   BPY_driver_exec: restricted access disallows name '__import__', enable auto-execution to support
```

**This closes the gap the finding named.** With neither `-y` nor `-Y`, a render started from outside follows the preference. A blocked driver is not silent — the `BPY_driver_exec:` line is printed — and the render does not fail; it goes ahead with the driver supplying 0.

Two other expressions on the same file ran whatever the flag: `len("xxxxx")` gave 5.0 and `sum([3.0,4.0])` gave 7.0 in all three cases. So not every scripted expression needs the permission.

**Hand test 2 — the two argument spellings.** This test was first run before it was known that Blender falls back to the machine's real settings when the throwaway folder does not exist; that attempt was discarded and the test re-run with the folder created first. What follows is the clean re-run. `blender --help` in this build lists both arguments, with this text:

```
--python-use-system-env 
	Allow Python to use system environment variables such as 'PYTHONPATH'.
	This also enables user environment, see: '--python-use-user-env'.

--python-use-user-env 
	Allow Python to use user's site-packages directory.
	This disables full isolation for the Python environment.
```

**This settles the discrepancy flagged above.** They are two different arguments and both exist; the manual page and the release notes were each naming one of them. (The same search of Blender 4.5.12's and 5.0.1's help shows `--python-use-system-env` only.)

### Observed on the rented Linux machine — Linux hand tests 22 and 2 (2026-08-06)

Run over SSH from a terminal on the rented Linux machine, using Blender 5.1.2 at `/opt/blender/blender-5.1.2-linux-x64/blender`, with `BLENDER_USER_RESOURCES` pointed at `/root/smoozy/Settings/res-5.1`, a folder of this topic's own that already existed. That folder was checked before use and reported its own config directory and Blender's eight default add-ons and nothing else. The machine is a container sharing someone else's hardware, and up to four topics were being worked on it at once, so nothing below claims anything about how long a run took or how much memory was free. Label: **observed**.

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

**And the two Python-environment arguments.** Searching this build's own help text on Linux found both `--python-use-system-env` and `--python-use-user-env`, the first saying it also enables the user environment. **The two machines agree** — the Mac's search found the same two in this version.

---

## 12. Can a render started from outside Blender be pointed at one named scene of a file, and what does it render when it is not told which?

**Yes it can be pointed at a named scene. LABEL: documented.**

> `-S, --scene <name>` — Set the active scene `<name>` for rendering.

Source: manual, Command Line Arguments (address in answer 1), read 2026-07-30.

**What it renders when not told is not stated for the command line. LABEL: not in the sources.** The API operator says it renders the current scene if no scene is supplied, and the interface refers to an active scene. Applying either statement to the command line is inferred. What sets the active scene of a freshly loaded file is not in the sources.

The nearest explicit statement is about the Python operator rather than the launch option:

> `bpy.ops.render.render(*, animation=False, write_still=False, use_viewport=False, use_sequencer_scene=False, layer='', scene='', frame_start=0, frame_end=0)`
> `scene (str)` – Scene, Scene to render, current scene if not specified (optional, never None)

Source: API, bpy.ops.render — https://docs.blender.org/api/5.1/bpy.ops.render.html — read 2026-07-30. The same call also carries `layer (str)` – "Render Layer, Single render layer to re-render (used only when animation is disabled)" and `use_sequencer_scene (bool)` – "Use Sequencer Scene, Render the sequencer scene instead of the active scene".

From inside the interface, the equivalents are described the same way: "**Render Image** F12 — Render the active scene at the current frame" and "**Render Animation** Ctrl-F12 — Render the animation of the active scene", plus "**Render Sequencer Image** Alt-F12 — If a Sequencer Scene exists that differs from the active scene, render that scene instead." Source: manual, Topbar — https://docs.blender.org/manual/en/5.1/interface/window_system/topbar.html — read 2026-07-30. A window's active scene is exposed as `Window.scene` — "Active scene to be edited in the window (never None)". Source: API, Window — https://docs.blender.org/api/5.1/bpy.types.Window.html — read 2026-07-30.

**The gap.** No source I found says which scene of a saved file becomes the active one when that file is loaded in background mode, where there is no window. The obvious reading — that it is whichever scene was active when the file was saved — is not written anywhere in these sources, so I am not asserting it.

**Where I looked.** I read the whole Command Line Arguments page and the whole Rendering From The Command Line page. I searched the whole manual for `active scene` — 20 hits, of which the relevant ones are quoted above; none describes how the active scene is chosen at file load. I read the `bpy.ops.render` page and the `Window` page.

**Not covered.** As above: the default scene of a file loaded without an interface. Also not covered: whether `-S` with a name that does not exist errors, warns, or silently renders something else; and whether `-S` must come before or after `-a`/`-f` (the general ordering rule in answer 1 suggests before, but the page does not say so for `-S` specifically).

**Outside the interface.** `-S` exists only outside the interface. The interface equivalent is switching the active scene by hand. One documented difference in the same area is that "Render Single Layer" is "ignored when rendering from the command line" (manual, View Layer, address in answer 1). The quoted material does not establish here exactly which view layers then render.

**Operating system.** `-S` carries no operating-system qualifier on a page where other options do. Reading that as "works everywhere" is inferred. The tests below were run on macOS only. **Observed 2026-08-06:** the same tests were run again on a rented Linux machine and came back the same, so this holds on macOS and on Linux. Windows stays untested.

### Observed — hand tests 15 and 16 (2026-08-05)

Both were run from a terminal on the Creator's own computer — a MacBook Pro with an M1 Pro chip, macOS 26.5.2 — using Blender 5.1.2 at `/Applications/Blender 5.1.2.app/Contents/MacOS/Blender`, with the `BLENDER_USER_RESOURCES` setting pointed at a fresh empty folder. Label: **observed**.

**Hand test 15 — which scene renders when none is named, and what a wrong name does.** A file was saved from factory startup with two scenes, `Alpha` (the default cube, camera and light) and `Beta` (a camera only), both Workbench at 64 by 64, with `Beta` made the window's scene before saving. Each render was run with a handler added at launch that names the scene the render starts on:

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

**This closes both gaps the finding named.** A render started from outside with no `-S` uses the scene that was showing when the file was saved. A name no scene bears is reported and then ignored — it neither stops the render nor changes which scene is used.

**Hand test 16 — the Sequencer Scene.** A third scene `Edit` was added, its sequencer holding a Scene strip referencing `Alpha`; the workspace's sequencer scene was set to `Edit` and the window's scene left as `Beta`. A bare `blender -b test.blend -o out/s_ -F PNG -f 1` printed `RENDER_PRE SCENE: Beta`. So a command-line render uses the active scene, not the Sequencer Scene.

**Not settled by these tests.** The files were built and saved by a Blender with no window, using `bpy.context.window.scene` and `workspace.sequencer_scene`. Whether a file saved from the interface records the active scene the same way was not tested.

### Observed on the rented Linux machine — Linux hand tests 17 and 18 (2026-08-06)

Run over SSH from a terminal on the rented Linux machine, using Blender 5.1.2 at `/opt/blender/blender-5.1.2-linux-x64/blender`, with `BLENDER_USER_RESOURCES` pointed at `/root/smoozy/Settings/res-5.1`, a folder of this topic's own that already existed. That folder was checked before use and reported its own config directory and Blender's eight default add-ons and nothing else. The machine is a container sharing someone else's hardware, and up to four topics were being worked on it at once, so nothing below claims anything about how long a run took or how much memory was free. Label: **observed**.

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

**Three come with Blender, and third-party renderers are available as add-ons. LABEL: documented. The conclusion that the available list grows when such an add-on is installed and enabled is inferred.**

> Blender includes three render engines with different strengths:
> EEVEE is a physically based realtime renderer.
> Cycles is a physically based path tracer.
> Workbench is designed for layout, modeling and previews.
> More renderers from third-party developers are available as add-ons.

Source: manual, Rendering — Introduction — https://docs.blender.org/manual/en/5.1/render/introduction.html — read 2026-07-30.

Blender will tell you the list at launch:

> `-E, --engine <engine>` — Specify the render engine. Use `-E help` to list available engines.

Source: manual, Command Line Arguments (address in answer 1), read 2026-07-30, and repeated on the rendering page: "For a list of available render engines, run `blender -E help`."

Blender also tracks whether there is more than one:

> `has_multiple_engines` — More than one rendering engine is available (default False, readonly)
> `engine` — Engine to use for rendering (default 'BLENDER_EEVEE') — Type: `Literal['BLENDER_EEVEE']`

Source: API, RenderSettings — https://docs.blender.org/api/5.1/bpy.types.RenderSettings.html — read 2026-07-30. Note what that says literally: the published API documentation lists exactly one permitted value for `engine`. **LABEL for the reading: inferred** — the fact that the documentation shows one value while the manual names three engines means the list is built at run time from whatever engines are registered, and the documentation was generated in a build where only one was. The sources do not say this; I am inferring it from the two passages disagreeing in a way only a dynamic list explains.

An engine is a registered Python class with an identifier and a label:

> ```
> class CustomRenderEngine(bpy.types.RenderEngine):
> # These three members are used by Blender to set up the
> # RenderEngine; define its internal name, visible name and capabilities.
> bl_idname = "CUSTOM"
> bl_label = "Custom"
> bl_use_preview = True
> ```
> …
> `bpy.utils.register_class(CustomRenderEngine)`

Source: API, RenderEngine — https://docs.blender.org/api/5.1/bpy.types.RenderEngine.html — read 2026-07-30. There is a second, USD-based route: "Base class for integrating USD Hydra based renderers", with `bl_idname`, `bl_label` and `bl_delegate_id`. Source: API, HydraRenderEngine — https://docs.blender.org/api/5.1/bpy.types.HydraRenderEngine.html — read 2026-07-30.

Cycles is itself exactly such an add-on. LABEL: observed.

> ```
> bl_info = {
>     "name": "Cycles Render Engine",
>     "author": "",
>     "blender": (2, 80, 0),
>     "description": "Cycles renderer integration",
>     "warning": "",
>     "doc_url": "https://docs.blender.org/manual/en/latest/render/cycles/",
>     "tracker_url": "",
>     "support": 'OFFICIAL',
>     "category": "Render"}
> ```
> ```
> class CyclesRender(bpy.types.RenderEngine):
>     bl_idname = 'CYCLES'
>     bl_label = "Cycles"
>     bl_use_eevee_viewport = True
>     bl_use_preview = True
>     bl_use_exclude_layers = True
>     bl_use_spherical_stereo = True
>     bl_use_custom_freestyle = True
> ```

Source: `Blender Sources/cycles_5.1/__init__.py`, lines 7–16 and 42–49. Read 2026-07-30. It is inferred that `CYCLES` is the identifier `-E` expects, and that the engine only exists when the add-on is registered.

Add-ons can be enabled at launch:

> `--addons <addon(s)>` — Comma separated list (no spaces) of add-ons to enable in addition to any default add-ons.

Source: manual, Command Line Arguments (address in answer 1), read 2026-07-30. And add-ons in general: "Add-ons let you extend Blender's functionality using Python. Most of the time you can get add-ons as part of the Extensions system." Source: manual, Add-ons — https://docs.blender.org/manual/en/5.1/advanced/extensions/addons.html — read 2026-07-30.

**So: yes, the list can grow on a given computer** — by installing and enabling an add-on that registers a `RenderEngine` (or `HydraRenderEngine`) subclass. That sentence is inferred from the passages above rather than stated as such.

**Not covered.** What Blender does when a file names an engine that is not registered on the machine opening it. I searched the manual and the API documentation for wording about a missing or unknown engine and found none. Also not covered: whether the three built-in engines can be removed, and whether `-E help` prints identifiers (`CYCLES`) or labels ("Cycles").

**Outside the interface.** The list is discoverable from outside via `-E help`, and can be grown from outside via `--addons`. Nothing here is interface-only. One caveat: an add-on that ships as an extension may need to be installed first, and the sources describe installation through the interface.

**Operating system.** Neither `-E` nor `--addons` carries an operating-system qualifier. Whether a particular third-party engine runs on every operating system is entirely outside these sources. The tests below were run on macOS only. **Observed 2026-08-06:** the same tests were run again on a rented Linux machine. The engine identifiers, the unknown-name error and the add-on case all came back the same, so those hold on macOS and on Linux. One thing does **not** match: in Blender 4.5.12, `-E help` and an unknown engine name end abnormally on both machines but not in the same way — `libc++abi: terminating` and exit code 134 on the Mac, and a run that never ends at all on the one-card Linux machine. **Observed 2026-08-06 on a second rented Linux machine — two NVIDIA GeForce RTX 4090s, driver 580.95.05, sixty-one cores, no display server at all:** `-E help` in this version printed `BLENDER_EEVEE`, `BLENDER_WORKBENCH` and `CYCLES` and exited 0. Blender 4.5.12 hung there too, but printed its listing first, where on the one-card machine it printed nothing at all before hanging — so 4.5 ends badly on all three machines and in three different ways, and none of the three is chosen between. Windows stays untested. *(Added by the two-card citation check, 2026-08-06: the third machine's result was in the hand-test record but not here, and the sentence read as though there were only two machines.)*

### Observed — hand tests 1 and 21 (2026-08-05)

Both were run from a terminal on the Creator's own computer — a MacBook Pro with an M1 Pro chip, macOS 26.5.2 — using Blender 5.1.2 at `/Applications/Blender 5.1.2.app/Contents/MacOS/Blender`, with the `BLENDER_USER_RESOURCES` setting pointed at fresh empty folders. Label: **observed**.

**Hand test 1 — what `-E help` prints.** It printed:

```
Blender Engine Listing:
	BLENDER_EEVEE
	BLENDER_WORKBENCH
	CYCLES
```

exit code 0. **This settles two of the things the finding left open:** the list is of identifiers rather than human labels, and `CYCLES` is exactly what `-E` expects. With a name no engine bears, `-E NOT_A_REAL_ENGINE` printed `Error: engine not found 'NOT_A_REAL_ENGINE'` and exited 1 without rendering.

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
00:00.322  reports          | ERROR Engine 'CYCLES' not available for scene 'Scene' (an add-on may need to be installed or enabled)
AFTER engine= 'BLENDER_EEVEE' has_multiple_engines= True engine choices= ['BLENDER_EEVEE']
```

and `blender -E help` in that state listed only `BLENDER_EEVEE` and `BLENDER_WORKBENCH`. Rendering went ahead anyway, exit code 0:

```
00:00.322  render           | Engine: EEVEE
00:00.931  render           | Fra: 1 | Rendering 50 / 64 samples
00:00.990  render           | Fra: 1 | Rendering 64 / 64 samples
00:01.055  render           | Saved: '.../out/n_0001.png'
```

64 samples is EEVEE's own default, not the file's Cycles setting of 4.

**This closes the gap the finding named.** A missing engine is reported by that exact message, the scene's engine reads back as `BLENDER_EEVEE`, and the render goes ahead on EEVEE rather than stopping. It also confirms the inferred step above: the list of engines is what is registered in the running Blender, and it shrinks when an add-on is switched off.

**What this does not settle.** No third-party engine add-on is installed, so the third-party half of the question, and whether `--addons` arrives in time for `-E`, stay untested.

### Observed — hand test 23, run by the Creator (2026-08-05)

Run by the Creator in Blender's own window, on their own computer — a MacBook Pro with an M1 Pro chip, macOS 26.5.2 — on 2026-08-05. One shot was supplied for 5.0, 5.1 and 5.2 together rather than one each, and the Creator gave it as the answer for all three. That settles macOS and leaves Windows and Linux untested. Label: **observed**.

The open **Render Engine** dropdown in Render Properties holds exactly three entries — **EEVEE**, **Workbench**, **Cycles** — with Cycles selected. So on a Blender with no extra add-on installed, the control offers those three and nothing else, under their human labels rather than the identifiers `-E` takes. That the printed list and the dropdown hold the same three is one more thing the finding above had to infer from an API page listing only one.

### Observed on the rented Linux machine — Linux hand tests 1, 2, 23 and 25 (2026-08-06)

Run over SSH from a terminal on the rented Linux machine, using Blender 5.1.2 at `/opt/blender/blender-5.1.2-linux-x64/blender`, with `BLENDER_USER_RESOURCES` pointed at `/root/smoozy/Settings/res-5.1`, a folder of this topic's own that already existed. That folder was checked before use and reported its own config directory and Blender's eight default add-ons and nothing else. The machine is a container sharing someone else's hardware, and up to four topics were being worked on it at once, so nothing below claims anything about how long a run took or how much memory was free. Label: **observed**.

These are a second machine's answers to the tests the Creator's Mac answered on 2026-08-05. The two are never merged: where they agree it says so, and where they differ it says that plainly. The full list is `Audits/Blender Research/Settings/Hand tests — Linux.md`.

**Linux hand test 1 — what `-E help` prints, and what `-E` does with a name no engine bears.** `blender -b -E help` printed:

```
Blender 5.1.2 (hash …)
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
Blender 5.1.2 (hash …)
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

## Things a person with Blender 5.1 open could settle by hand in a few minutes

Everything on this list that a terminal could reach was run on the Creator's Mac on 2026-08-05 and written into the answers above, everything that needed eyes on Blender's window was run by the Creator the same day, and every terminal test was then run again on a rented Linux machine on 2026-08-06 and written in beside it. What is left is one gap in what was photographed, one test that needs a machine to itself, and what no machine the project has can reach.

### Still open — needs somebody looking at Blender's window

1. **What the right-click menu on a view layer's "Use for Rendering" offers.** The dot beside it, and that the switch takes a keyframe, are both settled; the menu itself was not among the shots supplied on the Mac, and driving a right-click on one control is not something the Linux run attempted. (Question 1.)

### Needs a machine to itself

2. **Whether Cycles' Time Limit is per frame or per job on Linux.** Settled on the Mac by timing a three-frame render against a five-second limit. It could not be re-run on the rented machine, which was shared with three other topics throughout, because the answer can only be read from how long the run takes. The test is written out in `Audits/Blender Research/Settings/Hand tests — Linux.md`. (Question 2.)

### Cannot be run on any machine the project has

3. **Whether EEVEE fails on a headless Windows machine.** The screenless half of this item is now answered: on a second rented Linux machine with **no display server at all** — two NVIDIA GeForce RTX 4090s, driver 580.95.05, sixty-one cores, over SSH — every engine each version has rendered, on the cards and on the processor, EEVEE included, and that result is written into question 6. What is left is the Windows half, which needs a Windows machine, and a computer with no graphics hardware at all. (Question 6.)

4. **Whether Freestyle's line-style scripts run without `--enable-autoexec`.** The sources do not give the exact settings needed to build a file with a scripted line style, so the test cannot be written out without guessing, and a guessed set-up would answer nothing. A second machine does not help this one. (Question 11.)

5. **What a file naming a missing third-party engine does, and whether `--addons` arrives in time for `-E`.** Needs a third-party render engine add-on; none is installed on either machine, and installing one is not allowed on either. The nearest case — the Cycles add-on switched off in a throwaway settings folder — was run on both and is recorded under question 13. (Question 13.)

6. **What a file's "GPU Compute" choice means on an AMD or an Intel graphics card.** The NVIDIA case is settled — see question 9 — and asking for `HIP` or `ONEAPI` on a machine with no such card is refused outright. What either does when the card is there needs such a card, and there is none. (Question 9.)

7. **Anything about Windows.** The Linux half of every question's operating-system line is now answered. Windows stays untried, because no Windows machine has run it. **It is untried rather than untestable** — this project has a route to a Windows computer, described in the equipment section of `Preparation/Blender Research/Method.md`, and every machine that route has produced booted a **Server** edition, so a Windows answer would have to name its edition.

---

2026-08-04 — Opus 5 (Claude Code) — Settings, Blender 5.1

# Settings — Blender 4.5

**Sources consulted, and when.** Blender 4.5 manual (plain-text copy, including its `_images` pictures), Blender 4.5 Python API documentation, Blender 4.5 release notes, and Cycles' own add-on files as shipped with Blender 4.5 — all as supplied in `Blender Sources/`, snapshot taken 2026-07-30. I also read `Blender Sources/requirements/`, the copy of Blender's requirements page taken 2026-08-01, for the list of operating systems. Date read for everything: 2026-07-30 (2026-08-01 for the requirements page). Work done 2026-08-04.

I read only the question sheet and files inside Blender Sources/. I did not open any other part of this project.

I worked alone. No helper agents were used, so nothing needs vouching for beyond my own reading.

---

## 1. Which of a render's settings can be settled at the moment a render is started from outside Blender, and which can only be changed by changing the file?

**Answer.** Blender 4.5 offers command line arguments for some of these and not others.

**Settled at launch, by a documented argument of their own:**

- **Frame range** — `-s, --frame-start <frame>` / `-e, --frame-end <frame>`
- **Interval to skip frames by** — `-j, --frame-jump <frames>`
- **A list of separate frames** — `-f, --render-frame <frame>`
- **Render engine** — `-E, --engine <engine>`
- (also: which scene, `-S`; output path, `-o`; output format, `-F`; thread count, `-t`; the Cycles device, `--cycles-device`)

The exact words:

> `-a, --render-anim`
> Render frames from start to end (inclusive).
>
> `-f, --render-frame <frame>`
> Render frame <frame> and save it.
> `+<frame>` start frame relative, `-<frame>` end frame relative.
> A comma separated list of frames can also be used (no spaces).
> A range of frames can be expressed using `..` separator between the first and last frames (inclusive).
>
> `-s, --frame-start <frame>`
> Set start to frame <frame>, supports +/- for relative frames too.
>
> `-e, --frame-end <frame>`
> Set end to frame <frame>, supports +/- for relative frames too.
>
> `-j, --frame-jump <frames>`
> Set number of frames to step forward after each rendered frame.
>
> `-E, --engine <engine>`
> Specify the render engine. Use `-E help` to list available engines.

- Source: manual — https://docs.blender.org/manual/en/4.5/advanced/command_line/arguments.html
- Date read: 2026-07-30
- Label: **documented**

The order the arguments are given in matters, and the file overwrites earlier settings:

> Arguments are executed in the order they are given.
>
> `blender --background --render-output /tmp test.blend --render-frame 1`
> …will not render to /tmp because loading the blend-file overwrites the render output that was set.

- Source: manual — https://docs.blender.org/manual/en/4.5/advanced/command_line/arguments.html
- Date read: 2026-07-30
- Label: **documented**

**No argument of their own — a sample count, a resolution percentage, whether a view layer renders.** I searched the whole command line documentation (`arguments`, `render`, `extension_arguments`, `launch/*`) for `sample`, `resolution`, `percentage`, `view layer`, `--scene`. The only hits were `-S, --scene <name>` and the phrase "high resolution displays (MacBook Retina)" under `--no-native-pixels`. There is no argument that sets a sample count, a resolution percentage, or a view layer's on/off state.

- Label for the absence: **not in the sources** (record of looking is above)

These three are exposed as properties in Blender's data:

> `resolution_percentage`
> Percentage scale for render resolution
> Type: int in [1, 32767], default 100

- Source: Python API — https://docs.blender.org/api/4.5/bpy.types.RenderSettings.html
- Date read: 2026-07-30
- Label: **documented**

> `samples`
> Override number of render samples for this view layer, 0 will use the scene setting
> Type: int in [0, inf], default 0
>
> `use`
> Enable or disable rendering of this View Layer
> Type: boolean, default False

- Source: Python API — https://docs.blender.org/api/4.5/bpy.types.ViewLayer.html
- Date read: 2026-07-30
- Label: **documented**

And the Cycles scene-wide sample count:

> ```
> samples: IntProperty(
>     name="Samples",
>     description="Number of samples to render for each pixel",
>     min=1, max=(1 << 24),
>     default=4096,
> )
> ```

- Source: Cycles' own files — `properties.py`, lines 482–487, inside `class CyclesRenderSettings(bpy.types.PropertyGroup)` (which registers itself as `bpy.types.Scene.cycles`, lines 1100–1106)
- Date read: 2026-07-30
- Label: **observed**

**A third route, which is still "from outside Blender".** Blender's command line can run a Python script at launch:

> `-P, --python <filepath>`
> Run the given Python script file.
>
> `--python-expr <expression>`
> Run the given expression as a Python script.

- Source: manual — https://docs.blender.org/manual/en/4.5/advanced/command_line/arguments.html
- Date read: 2026-07-30
- Label: **documented**

That a script run this way can *set* the sample count, resolution percentage or view layer switch is **inferred**. The step: the API documentation marks properties that cannot be written with "(readonly)" — for example `Scene.render` is "RenderSettings, (readonly, never None)" and `RenderSettings.has_multiple_engines` is "boolean, default False, (readonly)". `resolution_percentage`, `ViewLayer.samples` and `ViewLayer.use` carry no such mark, so a script may write them. The sources never say this outright.

**On the view layer question specifically.** Two documented details bear on it. First:

> Render Single Layer
> Only render the active view layer.
>
> Note
> This option is ignored when rendering from the command line.

- Source: manual — https://docs.blender.org/manual/en/4.5/render/layers/view_layer.html
- Date read: 2026-07-30
- Label: **documented**

Second, the picture on that same page (`_images/render_layers_view-layer_panel.png`) shows the View Layer panel with a checkbox labelled "Use for Rendering", ticked, and a small dot to its right; "Render Single Layer" below it has a checkbox but no such dot. Blender's manual says of those dots:

> Decorators may appear next to number fields, menus, and checkboxes to indicate the property can be animated.

- Source: manual — https://docs.blender.org/manual/en/4.5/interface/controls/buttons/decorators.html and the picture on https://docs.blender.org/manual/en/4.5/render/layers/view_layer.html
- Date read: 2026-07-30
- Label: **documented** for what the picture and the decorator page each say; **inferred** for the join between them — that "Use for Rendering" can therefore be keyframed, so a view layer can be on at one frame and off at another. The step is mine: the manual never says "Use for Rendering can be animated".

There is also a per-render way to name one layer, from Python rather than the command line:

> `layer (string, (optional, never None))` – Render Layer, Single render layer to re-render (used only when animation is disabled)

- Source: Python API — https://docs.blender.org/api/4.5/bpy.ops.render.html
- Date read: 2026-07-30
- Label: **documented**

**Not covered.** The question was slightly ambiguous on "whether a view layer renders at a given frame" — I read it as asking both whether that switch can be set at launch (it cannot, no argument exists) and whether it can differ frame by frame (see the decorator finding above). The other reading, "which layers a given frame happens to render", is not addressed anywhere I found. I did not settle what happens when a frame list, a frame range and a frame jump are all given at once; nor what `-E` does with an engine name that is not registered; nor whether `-j` applies to `-f`.

**Outside the interface.** Everything above *is* the outside-the-interface case — these are command line arguments and they exist for exactly that purpose. The one documented difference between inside and outside is the "Render Single Layer" note quoted above: the setting is honoured in the interface and ignored from the command line.

**Operating system.** The arguments page marks the arguments that are not universal — `-con, --start-console` is "(Windows only)", `/?` is "(Windows only)", `-r, --register` and `--unregister` are "(Windows & Linux only)", `--debug-ghost` is "(Linux only)". None of the render arguments above carries such a mark. It is **inferred**, not documented, that they therefore apply everywhere. The requirements page names Windows, macOS and Linux for Blender 4.5. **Observed 2026-08-05:** every test below was run on macOS only, so the other two operating systems stay untested. **Observed 2026-08-06:** the same tests were run again on a rented Linux machine (see the Linux block at the end of this answer). Every argument this answer names behaves the same way there, so the answer now holds on macOS and on Linux; Windows stays untested. The argument list itself is not identical on the two — Blender's own help text is longer on Linux.

### Observed — hand tests 2, 3, 4, 5 and 10 (2026-08-05)

All five were run from a terminal on the Creator's own computer — a MacBook Pro with an M1 Pro chip, macOS 26.5.2, Apple's own graphics and no other graphics card — using Blender 4.5.12 at `/Applications/Blender 4.5.12.app/Contents/MacOS/Blender`, with the `BLENDER_USER_RESOURCES` setting pointed at a fresh empty folder so the Creator's own Blender settings were untouched. Label: **observed**.

**On the conditions of hand tests 1, 2 and 3.** Those three were first run before it was known that Blender falls back to the machine's real settings when that folder does not exist, so the first attempts loaded the Creator's own add-ons. Those attempts were discarded. All three were re-run from scratch with the folder created first — hand test 3 with its files rebuilt as well — and each version was checked before use, reporting its own throwaway settings folder and Blender's eight default add-ons and nothing else. What is written below is the clean re-run.

**Hand test 2 — is there really no argument for a sample count, a resolution percentage or a view layer?** `blender --help` was written to a file and searched, without opening any blend-file. The whole help output is 434 lines. Searching it for `sampl`, `resolution`, `percent`, `view layer` and `python-use` returned exactly three lines:

```
128:	Do not use native pixel size, for high resolution displays (MacBook 'Retina').
158:--python-use-system-env 
386:	Background mode: Do not systematically build and evaluate ViewLayers' dependency graphs
```

None of the three sets a render setting. So the program itself carries no argument for a sample count, a resolution percentage or a view layer, which is what the manual search above had found.

**Hand test 3 — a list of frames, a range of frames, and the frame jump.** A file was saved from factory startup with the Workbench engine and a 64 by 64 resolution. Five renders were run, each into its own empty folder. The files written:

```
blender -b test.blend -o out-list/f_  -F PNG -f 1,5,9        -> f_0001.png f_0005.png f_0009.png
blender -b test.blend -o out-range/f_ -F PNG -f 3..6         -> f_0003.png f_0004.png f_0005.png f_0006.png
blender -b test.blend -o out-mixed/f_ -F PNG -f 1,5..7,12    -> f_0001.png f_0005.png f_0006.png f_0007.png f_0012.png
blender -b test.blend -j 3 -o out-jumpf/f_ -F PNG -f 1..10   -> f_0001.png f_0002.png f_0003.png f_0004.png f_0005.png f_0006.png f_0007.png f_0008.png f_0009.png f_0010.png
blender -b test.blend -j 3 -s 1 -e 10 -o out-jumpa/f_ -F PNG -a -> f_0001.png f_0004.png f_0007.png f_0010.png
```

No error was printed by any of the five. So `-f` takes a comma list, a `..` range, and the two mixed in one argument; and `-j` is applied to `-a` and not to `-f`.

**Hand test 4 — does a launch argument change the file on disk?** A file was saved with frame start 1, frame end 250, frame step 1, resolution percentage 100 and the engine left at `BLENDER_EEVEE_NEXT`. Its size and modification time were noted, then this was run:

```
blender -b test.blend -E CYCLES -s 5 -e 7 -j 2 -o out/f_ -F PNG -f 5
```

The file's modification time and size were unchanged (`1785890065`, `434663 bytes`, before and after). Reading it back in a fresh Blender with no arguments printed:

```
READBACK 1 250 1 100 BLENDER_EEVEE_NEXT
```

So the arguments applied to that run only and left the file alone.

**Hand test 5 — setting a sample count, a resolution percentage and a view layer switch by `--python-expr` at launch.** A file was saved with the Cycles engine, 512 samples, two view layers (`ViewLayer` and `Layer2`) and a 200 by 100 resolution at 100 per cent. With the expression placed after the blend-file argument:

```
blender -b test.blend --python-expr "import bpy; s=bpy.data.scenes[0]; s.cycles.samples=8; s.render.resolution_percentage=25; s.view_layers['Layer2'].use=False; print('SET', s.cycles.samples, s.render.resolution_percentage, s.view_layers['Layer2'].use)" -o out-after/f_ -F PNG -f 1
```

printed `SET 8 25 False` and wrote an image of `pixelWidth: 50   pixelHeight: 25`. With the expression placed before the blend-file argument, it printed `SET 8 25` and wrote an image of `pixelWidth: 200   pixelHeight: 100`. So a setting with no argument of its own can be settled at launch by Python, and only when the expression comes after the file.

**Hand test 10 — a view layer's "Use for Rendering", and "Render Single Layer".** From factory startup, `keyframe_insert` was tried on the view layer's `use` property and returned `True`. A file was then saved with two view layers, `Layer2`'s `use` keyframed on at frame 1 and off at frame 2, Workbench, 64 by 64, and rendered:

```
blender -b test.blend -o out/f_ -F OPEN_EXR_MULTILAYER -s 1 -e 2 -a
```

Reading the channel names out of the two written files gave:

```
f_0001.exr : Layer2.Combined  ViewLayer.Combined      (5301 bytes)
f_0002.exr : ViewLayer.Combined                       (4200 bytes)
```

So an animation render driven from outside honours the switch frame by frame. Separately, the same file was rendered with `use_single_layer` turned on at launch:

```
blender -b test.blend --python-expr "import bpy; s=bpy.data.scenes[0]; s.render.use_single_layer=True; s.frame_set(1); print('SINGLE-LAYER ON, frame 1, layers on:', [l.name for l in s.view_layers if l.use])" -o out2/f_ -F OPEN_EXR_MULTILAYER -f 1
```

It printed `SINGLE-LAYER ON, frame 1, layers on: ['ViewLayer', 'Layer2']` and the written file held `Layer2.Combined ViewLayer.Combined`. So the setting is indeed ignored from the command line.

**Hand test 1 — the engine argument.** `blender -E help` printed:

```
Blender Engine Listing:
	BLENDER_EEVEE_NEXT
	BLENDER_WORKBENCH
	CYCLES
```

and then the process ended with `libc++abi: terminating` and exit code 134. That happened on every attempt, with and without `--factory-startup`. With a name no engine bears:

```
blender -b test.blend -E NOT_A_REAL_ENGINE -o out/e_ -F PNG -f 1
Error: engine not found 'NOT_A_REAL_ENGINE'
libc++abi: terminating
```

exit code 134, and no image written.

### Observed — hand test 25, run by the Creator (2026-08-05)

Run by the Creator in Blender 4.5's own window, on their own computer — a MacBook Pro with an M1 Pro chip, macOS 26.5.2 — on 2026-08-05. That settles macOS and leaves Windows and Linux untested. Label: **observed**.

They opened the Properties editor's View Layer Properties tab and photographed it. What is visible: a **View Layer** panel holding two rows — **Use for Rendering**, ticked, with a small round dot at the far right of its row; and **Render Single Layer**, unticked, with no dot on its row. Below it sit the Passes, Cryptomatte, Shader AOV, Filter and Custom Properties panels.

They also wrote, in their own words:

> "Pressing I at Use for Rendering makes the checkbox yellow and the dot next to it a diamond. Consistent across all versions. Render Single Layer is non-animatable in all versions, too. Here's the message: "use_single_layer" property cannot be animated. Also consistent in all versions."

**This settles what the finding above took from a picture in the manual.** The running program does show a decorator dot beside "Use for Rendering" and none beside "Render Single Layer", and the switch does take a keyframe. It also adds something the sources did not carry: "Render Single Layer" is refused a keyframe outright, with the message quoted above.

**What the screenshots do not cover.** The entry also asked what the right-click menu on "Use for Rendering" offers. No shot of that menu was among those supplied, and the Creator answered by pressing I instead. What that menu lists is therefore still unrecorded.

### Observed on the rented Linux machine — Linux hand tests 2, 3, 4, 5 and 12 (2026-08-06)

Run over SSH from a terminal on a rented Linux machine — Ubuntu 22.04.5 LTS, NVIDIA GeForce RTX 4090, driver 580.105.08, twelve cores, run over SSH, using Blender 4.5.12 at `/opt/blender/blender-4.5.12-linux-x64/blender`, with `BLENDER_USER_RESOURCES` pointed at `/root/smoozy/Settings/res-4.5`, a folder of this topic's own that already existed. That folder was checked before use and reported its own config directory and Blender's eight default add-ons and nothing else. The machine is a container sharing someone else's hardware, and up to four topics were being worked on it at once, so nothing below claims anything about how long a run took or how much memory was free. Label: **observed**.

These are a second machine's answers to the tests the Creator's Mac answered on 2026-08-05. The two are never merged: where they agree it says so, and where they differ it says that plainly. The full list is `Audits/Blender Research/Settings/Hand tests — Linux.md`.

**Linux hand test 2 — is there really no argument for a sample count, a resolution percentage or a view layer?** `blender --help` was written to a file and searched, with no blend-file opened. The whole help output is **447 lines** on this machine; the same command on the Creator's Mac gave 434, so the argument list is not the same length on the two operating systems. Searching it for `sampl`, `resolution`, `percent`, `view layer` and `python-use` returned only lines that set no render setting — the `--no-native-pixels` description, the Python-environment arguments, and the background-mode note about view layer dependency graphs. There is no `--samples` and no `--resolution` in this build either. `--python-use-system-env` is present and `--python-use-user-env` is not, as on the Mac.

**Which arguments name an operating system.** Searching the same help text for operating-system marks gives exactly these, and no render argument is among them:

```
	Start with the console window open (ignored if '-b' is set), (Windows only).
	Enable debug messages for Ghost (Linux only).
	Print this help text and exit (Windows only).
	Register blend-file extension for current user, then exit (Windows & Linux only).
	Register blend-file extension for all users, then exit (Windows & Linux only).
	Unregister blend-file extension for current user, then exit (Windows & Linux only).
	Unregister blend-file extension for all users, then exit (Windows & Linux only).
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

**Linux hand test 4 — does a launch argument change the file on disk?** A file was saved with frame start 1, frame end 250, frame step 1, resolution percentage 100 and the engine left at `BLENDER_EEVEE_NEXT`. Then:

```
blender -b test.blend -E CYCLES -s 5 -e 7 -j 2 -o out/f_ -F PNG -f 5
```

Its modification time and size were unchanged before and after, and reading it back in a fresh Blender with none of those arguments printed:

```
READBACK 1 250 1 100 BLENDER_EEVEE_NEXT
```

**The two machines agree.**

**Linux hand test 5 — setting a sample count, a resolution percentage and a view layer switch by `--python-expr` at launch.** A file was saved with Cycles, 512 samples, two view layers (`ViewLayer` and `Layer2`) and 200 by 100 at 100 per cent. With the expression after the blend-file argument it printed `SET 8 25 False`, Cycles reported `Sample 8/8`, the progress lines named `Scene, ViewLayer` alone, and the written image measured 50 by 25 pixels. With the expression before the blend-file argument it printed `SET 8 25`, Cycles reported `Sample 512/512`, the progress lines named `Scene, Layer2`, and the image measured 200 by 100. **The two machines agree** — the expression settles all three, and only when it comes after the file.

**Linux hand test 12 — a view layer's switch per frame, and "Render Single Layer" from the command line.** A file was saved with two view layers, `Layer2`'s switch keyframed on at frame 1 and off at frame 2, Workbench, 64 by 64. `blender -b test.blend -o out/f_ -F OPEN_EXR_MULTILAYER -s 1 -e 2 -a` wrote two files, and the names found in their headers were:

```
f_0001.exr : Layer2.Combined.R/G/B/A   ViewLayer.Combined.R/G/B/A            (5224 bytes)
f_0002.exr : ViewLayer.Combined.R/G/B/A                                      (4049 bytes)
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

**Answer.** Yes to both, in Cycles. Neither exists for EEVEE or Workbench in these sources.

**Clean enough — Adaptive Sampling / Noise Threshold:**

> Adaptive Sampling
> If the Noise Threshold checkbox is enabled, Cycles will use adaptive sampling, cutting short the sampling process in areas that have become less noisy than the specified threshold value – in other words, not wasting time further refining areas that already look good enough.
>
> Noise Threshold
> The threshold to decide whether to continue sampling a pixel. Typical values are in the range from 0.1 to 0.001, with lower values meaning better quality but longer render times. Setting this to 0 makes Cycles guess an automatic value based on the total sample count.
>
> Min Samples
> The minimum number of samples a pixel receives before adaptive sampling is applied. When set to 0 (default), Cycles automatically picks a value determined by the Noise Threshold.

**A length of time — Time Limit:**

> Time Limit
> Stops rendering if the time exceeds the limit, even if the desired sample count hasn't been reached yet. Setting this value to zero disables the limit.
>
> Note
> The time limit does not include pre-render processing time, only render time.

- Source for both: manual — https://docs.blender.org/manual/en/4.5/render/cycles/render_settings/sampling.html
- Date read: 2026-07-30
- Label: **documented**

The settings themselves, in Cycles' own files:

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
> ```

- Source: Cycles' own files — `properties.py`, `time_limit` at lines 517–525, `use_adaptive_sampling` and `adaptive_threshold` at lines 569–582, both inside `class CyclesRenderSettings`
- Date read: 2026-07-30
- Label: **observed**. Note the defaults: adaptive sampling is on by default with a threshold of 0.01; the time limit is off by default (zero).

**Cycles only.** I searched the whole manual for "Time Limit", "Noise Threshold" and "Adaptive Sampling": every hit is on a Cycles page (`render/cycles/render_settings/sampling.txt`, its index, a passing mention on `render/layers/passes.txt`, and a historical note). I also searched the API pages for EEVEE (`bpy.types.SceneEEVEE`) and Workbench (`bpy.types.SceneDisplay`) for "time_limit" and "threshold": the only threshold-named EEVEE properties are `bokeh_threshold` and `light_threshold`, neither of which is a stopping rule.

- Label for that absence: **not in the sources**

**Not covered.** The manual says adaptive sampling cuts short sampling *in areas* — it does not say what makes the frame as a whole finish, so I cannot say from the sources whether a frame ends early when every pixel is under the threshold. I also cannot say what the render writes out when a time limit cuts it short, whether the limit is per frame or per job, or how the two stopping rules interact with denoising.

**Outside the interface.** The sources do not say. Both properties are declared on Cycles' scene settings, and no command line argument exists for either (see question 1's search record). The sources do not state whether, or how, a command line render applies their values. The manual's note that the limit excludes "pre-render processing time" is not qualified by where the render was started from. **Observed 2026-08-05 (hand tests 11 and 12, below):** a render started from outside Blender does apply both — the time limit stopped each frame and adaptive sampling stopped a frame early.

**Operating system.** The sources do not say; neither the manual page nor the Cycles file mentions any operating system in connection with these two settings. The tests below were run on macOS only, so Windows and Linux stay untested. **Observed 2026-08-06:** adaptive sampling and the EEVEE and Workbench property lists were run again on a rented Linux machine and came back the same, so those parts hold on macOS and on Linux. The Time Limit test was not re-run there, because it can only be read from timing and the machine was shared, so that part still holds for macOS alone. Windows stays untested.

### Observed — hand tests 11, 12 and 13 (2026-08-05)

All three were run from a terminal on the Creator's own computer — a MacBook Pro with an M1 Pro chip, macOS 26.5.2, Apple's own graphics and no other graphics card — using Blender 4.5.12 at `/Applications/Blender 4.5.12.app/Contents/MacOS/Blender`, with the `BLENDER_USER_RESOURCES` setting pointed at a fresh empty folder. Label: **observed**.

**Hand test 11 — is the Time Limit per frame or per job, and is a cut-short frame still written?** A file was saved from factory startup with Cycles on the processor, 400 by 400, Max Samples 16384, adaptive sampling off, denoising off, Time Limit 5 seconds, frame range 1 to 3. Then:

```
blender -b test.blend -o out/f_ -F PNG -a
```

The per-frame times printed were `Time: 00:06.98`, `Time: 00:06.96`, `Time: 00:07.07`, and the whole run took 22 seconds by the shell's clock. All three images were written: `f_0001.png f_0002.png f_0003.png`. So the limit is applied to each frame, not to the job, and a frame cut short is still saved.

**Hand test 12 — does adaptive sampling end a frame before the sample count is reached?** Two files were saved from factory startup, both Cycles on the processor, 128 by 128, Max Samples 2048, Time Limit 0, denoising off — one with adaptive sampling on and a Noise Threshold of 0.1, one with adaptive sampling off. Each was rendered with `blender -b <file> -o out/<tag>_ -F PNG -f 1`. The sample lines printed:

```
adaptive on  : Sample 0/2048  Sample 1/2048  Sample 48/2048  Sample 0/2048   Time: 00:00.08
adaptive off : Sample 2048/2048                                              Time: 00:01.47
```

So the whole frame stopped at 48 of 2048 samples with adaptive sampling on.

**Hand test 13 — has EEVEE or Workbench any way to stop a render early?** Every property of `scene.eevee` and `scene.display` was listed from factory startup and searched for `time`, `threshold`, `adaptive`, `noise`, `stop` and `limit`. `scene.eevee` has 55 properties and `scene.display` has 9. No property on either is a time limit or a rule for stopping when the picture is clean enough. The nearest matches on `scene.eevee` were `bokeh_threshold` ("Brightness threshold for using sprite base depth of field") and `light_threshold` ("Minimum light intensity for a light to contribute to the lighting"); on `scene.display` there was no match at all. Its nine properties in full:

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

Run by the Creator in Blender 4.5's own window, on their own computer — a MacBook Pro with an M1 Pro chip, macOS 26.5.2 — on 2026-08-05. That settles macOS and leaves Windows and Linux untested. Label: **observed**.

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

**This closes the "not in the sources" answer above by eye as well as by property list.** Neither engine's Sampling panel offers a time limit or a noise threshold, or any other control that would stop a render early. EEVEE's one threshold is Light Threshold, which the property list describes as the minimum intensity for a light to contribute.

### Observed on the rented Linux machine — Linux hand tests 13 and 14 (2026-08-06)

Run over SSH from a terminal on the rented Linux machine, using Blender 4.5.12 at `/opt/blender/blender-4.5.12-linux-x64/blender`, with `BLENDER_USER_RESOURCES` pointed at `/root/smoozy/Settings/res-4.5`, a folder of this topic's own that already existed. That folder was checked before use and reported its own config directory and Blender's eight default add-ons and nothing else. The machine is a container sharing someone else's hardware, and up to four topics were being worked on it at once, so nothing below claims anything about how long a run took or how much memory was free. Label: **observed**.

These are a second machine's answers to the tests the Creator's Mac answered on 2026-08-05. The two are never merged: where they agree it says so, and where they differ it says that plainly. The full list is `Audits/Blender Research/Settings/Hand tests — Linux.md`.

**Linux hand test 13 — does adaptive sampling end the frame before the sample count is reached?** Two Cycles files at 128 by 128, Max Samples 2048, Time Limit 0, denoising off — one with adaptive sampling on at a noise threshold of 0.1, one with it off. Both rendered one frame with `-- --cycles-print-stats`, both exited 0, both wrote their image. The sample counts reported:

```
adaptive on  : Sample 0/2048 -> Sample 1/2048 -> Sample 48/2048   (stopped there)
adaptive off : Sample 0/2048 -> Sample 1/2048 -> Sample 2048/2048
```

**The two machines agree**, at the same 48 samples out of 2,048. So adaptive sampling ends the whole frame early, not only the sampling of single pixels.

**Linux hand test 14 — has EEVEE or Workbench anything that stops a render?** Every property of the scene's EEVEE block and of its Workbench block was listed with its description. EEVEE holds **55** on this build and Workbench holds **9** — the same two counts the Mac's run gave for this version. Searching both lists for `time`, `threshold`, `adaptive`, `noise`, `stop` and `limit` matched only EEVEE lines, and none of them stops anything:

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

**Answer.** Blender 4.5 keeps a separate block of settings per engine, alongside a shared block.

The manual states the principle:

> Blender includes three render engines with different strengths:
> EEVEE is a physically based realtime renderer.
> Cycles is a physically based path tracer.
> Workbench is designed for layout, modeling and previews.
> More renderers from third-party developers are available as add-ons.
> Each renderer has its own render settings to control render quality and performance.

- Source: manual — https://docs.blender.org/manual/en/4.5/render/introduction.html
- Date read: 2026-07-30
- Label: **documented**

And repeats it for view layers:

> Additional options shown in this panel are different for each render engine.

- Source: manual — https://docs.blender.org/manual/en/4.5/render/layers/view_layer.html
- Date read: 2026-07-30
- Label: **documented**

The API shows the shape of it. A scene carries several separate settings blocks:

> `cycles` — Cycles render settings — Type: CyclesRenderSettings, (readonly)
> `display` — Scene display settings for 3D viewport — Type: SceneDisplay, (readonly)
> `eevee` — EEVEE settings for the scene — Type: SceneEEVEE, (readonly)
> `render` — Type: RenderSettings, (readonly, never None)

- Source: Python API — https://docs.blender.org/api/4.5/bpy.types.Scene.html
- Date read: 2026-07-30
- Label: **documented**

`RenderSettings` is the shared one — it holds `engine`, `resolution_x`, `resolution_y`, `resolution_percentage`, `fps`, output settings and so on (https://docs.blender.org/api/4.5/bpy.types.RenderSettings.html). The frame range and step are shared too, on the scene itself: `frame_start`, `frame_end`, `frame_step` (https://docs.blender.org/api/4.5/bpy.types.Scene.html).

Cycles' own files show it attaching its settings to many kinds of data, all under names of its own:

> `bpy.types.Scene.cycles`, `bpy.types.Scene.cycles_curves`, `bpy.types.ViewLayer.cycles`, `bpy.types.Material.cycles`, `bpy.types.Light.cycles`, `bpy.types.World.cycles`, `bpy.types.World.cycles_visibility`, `bpy.types.Mesh.cycles`, `bpy.types.Curve.cycles`, `bpy.types.Object.cycles`, `bpy.types.Camera.cycles_custom`

- Source: Cycles' own files — `properties.py`, in the `register()` classmethods at lines 1102, 1483, 1537, 1173, 1213, 1279, 1331, 1345, 1350, 1455, 1117
- Date read: 2026-07-30
- Label: **observed**

Panels also declare which engines they belong to. The API's own worked example says:

> RenderEngines also need to tell UI Panels that they are compatible with. We recommend to enable all panels marked as BLENDER_RENDER, and then exclude any panels that are replaced by custom panels registered by the render engine, or that are not supported.

- Source: Python API — https://docs.blender.org/api/4.5/bpy.types.RenderEngine.html
- Date read: 2026-07-30
- Label: **documented**

One practical consequence, **observed**: there is no `bpy.types.CyclesRenderSettings` page in the 4.5 API documentation at all (I listed every file in `api_4.5` matching "cycles" — only `bpy.ops.cycles.txt` and `bpy.types.FModifierCycles.txt` exist). The listed API files do not provide a `CyclesRenderSettings` page.

**Not covered.** I have not enumerated every single engine-specific setting — that would be the whole of the Cycles, EEVEE and Workbench render-settings sections. Nor do the sources say what happens to a Cycles setting when the engine is switched away and back.

**Outside the interface.** The cited sources describe settings blocks in Blender's data, not a difference between the interface and a command line render. It is **inferred** that the split therefore holds equally. The one documented interface-versus-command-line difference I found anywhere is the "Render Single Layer" note under question 1. **Observed 2026-08-05 (hand test 6, under question 4):** all three engines' settings blocks were reachable and held their own values in a Blender started from a terminal with no window, whichever engine was selected.

**Operating system.** The sources do not say, and nothing in this split mentions an operating system. The test below was run on macOS only. **Observed 2026-08-06:** the same test was run again on a rented Linux machine and came back the same, so this holds on macOS and on Linux. Windows stays untested.

### Observed — hand test 13 (2026-08-05)

Run from a terminal on the Creator's own computer — a MacBook Pro with an M1 Pro chip, macOS 26.5.2 — using Blender 4.5.12 at `/Applications/Blender 4.5.12.app/Contents/MacOS/Blender`, with the `BLENDER_USER_RESOURCES` setting pointed at a fresh empty folder. Label: **observed**.

Every property of `scene.eevee` and `scene.display` was listed from factory startup. `scene.eevee` holds 55 properties; `scene.display` holds 9, listed in full under question 2. Neither block holds anything naming a processor or a graphics card as a choice; the only two properties on either that mention a graphics card at all are EEVEE's `gi_irradiance_pool_size` and `shadow_pool_size`, both about memory size:

```
gi_irradiance_pool_size | Size of the irradiance pool, a bigger pool size allows for more irradiance grid in the scene but might not fit into GPU memory and decrease performance
shadow_pool_size | Size of the shadow pool, a bigger pool size allows for more shadows in the scene but might not fit into GPU memory
```

### Observed — hand test 23, run by the Creator (2026-08-05)

Run by the Creator in Blender 4.5's own window, on their own computer — a MacBook Pro with an M1 Pro chip, macOS 26.5.2 — on 2026-08-05. That settles macOS and leaves Windows and Linux untested. Label: **observed**.

They opened Render Properties and clicked the **Render Engine** dropdown so its list was showing, and photographed it. The open list holds exactly three entries, in this order:

```
EEVEE
Workbench
Cycles
```

with Cycles the one selected. The rows immediately under the dropdown in this version are **Feature Set** and **Device**.

So the control offers human labels, not the identifiers `-E` takes (hand test 1, under question 13), and the three engines the manual names are the three the control offers.

### Observed on the rented Linux machine — Linux hand tests 6, 14 and 25 (2026-08-06)

Run over SSH from a terminal on the rented Linux machine, using Blender 4.5.12 at `/opt/blender/blender-4.5.12-linux-x64/blender`, with `BLENDER_USER_RESOURCES` pointed at `/root/smoozy/Settings/res-4.5`, a folder of this topic's own that already existed. That folder was checked before use and reported its own config directory and Blender's eight default add-ons and nothing else. The machine is a container sharing someone else's hardware, and up to four topics were being worked on it at once, so nothing below claims anything about how long a run took or how much memory was free. Label: **observed**.

These are a second machine's answers to the tests the Creator's Mac answered on 2026-08-05. The two are never merged: where they agree it says so, and where they differ it says that plainly. The full list is `Audits/Blender Research/Settings/Hand tests — Linux.md`.

**Linux hand tests 6 and 14 — are all the engines' blocks there at once, and what is on them?** A file was saved with Cycles samples 123, EEVEE render samples 45, Workbench final anti-aliasing `32` and the Grease Pencil final-render count 3, then read back in a fresh Blender started from a terminal with no window:

```
BACK as-loaded engine= BLENDER_EEVEE_NEXT cycles= 123 eevee= 45 workbench= 32 gpencil= 3
```

All three engines' blocks were reachable and each held its own saved value, whichever engine was selected. Listing every property gave 55 on the EEVEE block and 9 on the Workbench block. **The two machines agree.**

**Linux hand test 25 — the Render Properties tab, photographed in Blender's own window on this machine.** Blender was started with a real window on the machine's own display, the engine set to Workbench and the Properties editor pointed at Render, and the window photographed with Blender's own screenshot command. Blender 4.5's tab has no Curves panel. The tab carries the shared settings under whichever engine is chosen, exactly as the Creator's photographs of the same tab on the Mac showed. **The two machines agree.**

---

## 4. Does a file carry a separate sample count for each render engine?

**Answer.** Yes — there are three separate sample-count settings, one per built-in engine, and they live in three different places on the scene.

Cycles:

> ```
> samples: IntProperty(
>     name="Samples",
>     description="Number of samples to render for each pixel",
>     min=1, max=(1 << 24),
>     default=4096,
> )
> ```

- Source: Cycles' own files — `properties.py`, lines 482–487, in `class CyclesRenderSettings` (registered as `bpy.types.Scene.cycles`)
- Date read: 2026-07-30
- Label: **observed**

EEVEE:

> `taa_render_samples`
> Number of samples per pixel for rendering
> Type: int in [1, inf], default 64

- Source: Python API — https://docs.blender.org/api/4.5/bpy.types.SceneEEVEE.html
- Date read: 2026-07-30
- Label: **documented**

Workbench:

> `render_aa`
> Method of anti-aliasing when rendering final image
> OFF — No Anti-Aliasing … FXAA — Single Pass Anti-Aliasing … 5 — 5 Samples … 8 — 8 Samples … 11 — 11 Samples … 16 …

- Source: Python API — https://docs.blender.org/api/4.5/bpy.types.SceneDisplay.html
- Date read: 2026-07-30
- Label: **documented**

The manual describes each in its own engine's Sampling page: Cycles' "Render (Max) Samples — The number of paths to trace per pixel in the final render" (https://docs.blender.org/manual/en/4.5/render/cycles/render_settings/sampling.html) and EEVEE's "Render / Samples — The number of samples to use in the final render" (https://docs.blender.org/manual/en/4.5/render/eevee/render_settings/sampling.html).

What the quoted sources carry is that three separately named sample-count settings sit on three separate settings blocks of the same scene, none of them marked read-only. That a saved `.blend` holds all three of those values at the same time is **inferred**, and the sources never state what a `.blend` writes to disk; it is settled instead by hand test 6 below. *[Citation check, 2026-08-05: the sentence used to claim the saved file as an inference from the quoted properties alone; the quotes do not reach the saving half, so it now says which finding does.]*

**Not covered.** Whether these counts are saved to the file at all (as opposed to being rebuilt at load) is not stated anywhere I found. Nor is there any statement about a third-party engine's sample count. Cycles also has a separate viewport count (`preview_samples`, default 1024) and EEVEE has `taa_samples` (default 16) — the question asked about the render count, so I have kept the viewport ones out of the answer.

**Outside the interface.** The sources do not say. No command line argument reaches any of the three (question 1's search record), so from outside Blender you get whatever the file holds unless a launch script changes it. **Observed 2026-08-05 (hand test 6, below):** a Blender started from a terminal with no window read all three counts back from the file at the values they were saved with.

**Operating system.** The sources do not say; none of the three property definitions mentions an operating system. The test below was run on macOS only. **Observed 2026-08-06:** the same test was run again on a rented Linux machine and came back the same, value for value, so this holds on macOS and on Linux. Windows stays untested.

### Observed — hand test 6 (2026-08-05)

Run from a terminal on the Creator's own computer — a MacBook Pro with an M1 Pro chip, macOS 26.5.2, Apple's own graphics and no other graphics card — using Blender 4.5.12 at `/Applications/Blender 4.5.12.app/Contents/MacOS/Blender`, with the `BLENDER_USER_RESOURCES` setting pointed at a fresh empty folder. Label: **observed**.

A file was saved from factory startup with Cycles samples 123, EEVEE render samples 45, Workbench final anti-aliasing `32` and the Grease Pencil final-render sample count 3. Blender was then closed and a fresh Blender opened the file and read all four values back, switching the engine between each reading. It printed:

```
MADE cycles=123 eevee=45 workbench=32 gpencil=3
BACK as-saved               engine=BLENDER_EEVEE_NEXT   cycles=123 eevee=45 workbench=32 gpencil=3
BACK after switch to CYCLES engine=CYCLES               cycles=123 eevee=45 workbench=32 gpencil=3
BACK after switch to BLENDER_WORKBENCH engine=BLENDER_WORKBENCH    cycles=123 eevee=45 workbench=32 gpencil=3
BACK after switch to EEVEE  engine=BLENDER_EEVEE_NEXT   cycles=123 eevee=45 workbench=32 gpencil=3
```

So one file carries all three engines' counts at once, they survive a save and a reload, and switching the engine changes none of them. The Grease Pencil count is one shared value and reads the same under all three engines.

### Observed — hand test 26, run by the Creator (2026-08-05)

Run by the Creator in Blender 4.5's own window, on their own computer — a MacBook Pro with an M1 Pro chip, macOS 26.5.2 — on 2026-08-05. That settles macOS and leaves Windows and Linux untested. Label: **observed**.

Photographed with the Render Engine set to **EEVEE**, the Sampling panel shows a **Viewport ▸ Samples** of 16 and a separate **Render ▸ Samples** of 64 — two counts, not one. Photographed with the Render Engine set to **Workbench**, the whole Sampling panel is **Render — 8 Samples** and **Viewport — Single Pass Anti-Aliasing**, both dropdowns rather than number fields. Cycles' own panel (hand test 24, under question 5) shows **Max Samples 4096** as a number field.

So the three engines' final-render counts are three separate controls in three separate panels, and Workbench's is a menu of fixed choices where the other two are numbers — which is what the sources describe, seen in the program.

### Observed on the rented Linux machine — Linux hand tests 6 and 14 (2026-08-06)

Run over SSH from a terminal on the rented Linux machine, using Blender 4.5.12 at `/opt/blender/blender-4.5.12-linux-x64/blender`, with `BLENDER_USER_RESOURCES` pointed at `/root/smoozy/Settings/res-4.5`, a folder of this topic's own that already existed. That folder was checked before use and reported its own config directory and Blender's eight default add-ons and nothing else. The machine is a container sharing someone else's hardware, and up to four topics were being worked on it at once, so nothing below claims anything about how long a run took or how much memory was free. Label: **observed**.

These are a second machine's answers to the tests the Creator's Mac answered on 2026-08-05. The two are never merged: where they agree it says so, and where they differ it says that plainly. The full list is `Audits/Blender Research/Settings/Hand tests — Linux.md`.

**Linux hand test 6 — does one file hold all three counts at once, and is the Grease Pencil count shared?** A file was saved with Cycles samples 123, EEVEE render samples 45, Workbench final anti-aliasing `32` and the Grease Pencil final-render count 3, closed, and read back in a fresh Blender; then the engine was switched three times and the same four values read after each switch, in one run:

```
BACK as-loaded engine= BLENDER_EEVEE_NEXT cycles= 123 eevee= 45 workbench= 32 gpencil= 3
BACK after-switch-to-CYCLES engine= CYCLES cycles= 123 eevee= 45 workbench= 32 gpencil= 3
BACK after-switch-to-BLENDER_WORKBENCH engine= BLENDER_WORKBENCH cycles= 123 eevee= 45 workbench= 32 gpencil= 3
BACK after-switch-to-BLENDER_EEVEE_NEXT engine= BLENDER_EEVEE_NEXT cycles= 123 eevee= 45 workbench= 32 gpencil= 3
```

So one file carries three separate counts, they survive a save and a reload, switching the engine disturbs none of them, and the Grease Pencil count reads the same under every engine. **The two machines agree**, value for value.

**Linux hand test 14 — what each engine's own block actually holds.** The EEVEE block holds **55** properties on this build and the Workbench block holds **9** — the same two counts the Mac's run gave for this version. **The two machines agree.**

---

## 5. Can a sample count change from frame to frame?

**Answer.** Partly inferable, but the sources never say so outright, and one observed detail cuts against it being offered in the ordinary way.

**What the sources do say.** Cycles' `samples` is declared as an `IntProperty` with no `options` argument (quoted in question 4). Blender's Python documentation gives the default for that argument:

> `bpy.props.IntProperty(*, name='', description='', translation_context='*', default=0, min=-2**31, max=2**31 - 1, soft_min=-2**31, soft_max=2**31 - 1, step=1, options={'ANIMATABLE'}, override=set(), tags=set(), subtype='NONE', update=None, get=None, set=None)`

The same `options={'ANIMATABLE'}` default appears on the other property functions on that page, for example:

> `bpy.props.BoolProperty(*, name='', description='', translation_context='*', default=False, options={'ANIMATABLE'}, override=set(), tags=set(), subtype='NONE', update=None, get=None, set=None)`

- Source: Python API — https://docs.blender.org/api/4.5/bpy.props.html
- Date read: 2026-07-30
- Label: **documented** for the default; and the flag's meaning is listed as "ANIMATABLE: Animatable." on https://docs.blender.org/api/4.5/bpy_types_enum_items/property_flag_items.html

From those two together it **follows** that Cycles' render sample count is an animatable property, and therefore could hold different values at different frames. The step is mine: no page says "the sample count can be keyframed".

**What cuts against it.** Cycles' own interface file switches the keyframe buttons off in that panel:

> ```
> class CYCLES_RENDER_PT_sampling_render(CyclesButtonsPanel, Panel):
>     bl_label = "Render"
>     bl_parent_id = "CYCLES_RENDER_PT_sampling"
>     …
>         layout.use_property_split = True
>         layout.use_property_decorate = False
>         …
>         col.prop(cscene, "samples", text="Max Samples")
> ```

- Source: Cycles' own files — `ui.py`, lines 286–315
- Date read: 2026-07-30
- Label: **observed** for what the file says — that this panel's draw sets `layout.use_property_decorate = False`. That the drawn panel therefore carries no decorator dot is a step past the code, so that part is **inferred**; the Creator's photograph under hand test 24 is what settles it by eye. The manual explains what decorators are for: *[Citation check, 2026-08-05: line numbers corrected from 288–316, and the label split — the code shows a flag being set, not what the panel looks like.]* "Clicking on the decorator dot icon will add a Keyframe to that property" (https://docs.blender.org/manual/en/4.5/interface/controls/buttons/decorators.html). So the one-click way of keyframing the sample count is not offered in that panel. Whether it can still be keyframed another way is a step beyond what the file shows, and I have not taken it.

**What is documented nearby, but is not the same thing.** Cycles does have a documented per-frame variation — of the noise pattern, not the count:

> Use Animated Seed (clock icon)
> Changes the seed for each frame. It is a good idea to enable this when rendering animations because a varying noise pattern is less noticeable.

And a documented per-*layer* override, not per-frame:

> `samples` — Override number of render samples for this view layer, 0 will use the scene setting

- Sources: manual — https://docs.blender.org/manual/en/4.5/render/cycles/render_settings/sampling.html ; Python API — https://docs.blender.org/api/4.5/bpy.types.ViewLayer.html
- Date read: 2026-07-30
- Label: **documented**

**Where I looked and found nothing.** I searched the manual's whole `render/` section for "keyframe", "per frame", "per-frame", and the whole manual for "can be animated", "cannot be animated", "animatable". Nothing connects any of these to a sample count. There is no statement anywhere in the manual, the API documentation, or the 4.5 release notes about varying a sample count across frames.

- Label for the direct question: **not in the sources**, with the inference above offered separately and clearly marked.

**Not covered.** I cannot say whether EEVEE's `taa_render_samples` or Workbench's `render_aa` behave the same way — I did not find their declarations (they are built into Blender, not in the Cycles files I was given). I also cannot say whether a value that changes mid-animation is picked up by the renderer at each frame, only that the property exists and is nominally animatable.

**Outside the interface.** The sources do not say. Note the interface finding above (no keyframe decorator) is by definition an interface fact; it says nothing about what a render started from outside would do with an animated value. **Observed 2026-08-05 (hand test 9, below):** a render started from a terminal with no window used the keyframed value at each frame.

**Operating system.** The sources do not say; nothing here mentions an operating system. The test below was run on macOS only. **Observed 2026-08-06:** the same tests were run again on a rented Linux machine and came back the same, so this holds on macOS and on Linux. Windows stays untested.

### Observed — hand tests 9 and 10 (2026-08-05)

Both were run from a terminal on the Creator's own computer — a MacBook Pro with an M1 Pro chip, macOS 26.5.2 — using Blender 4.5.12 at `/Applications/Blender 4.5.12.app/Contents/MacOS/Blender`, with the `BLENDER_USER_RESOURCES` setting pointed at a fresh empty folder. Label: **observed**.

**Hand test 9 — can each engine's count take a keyframe, and does a render honour it?** From factory startup, `keyframe_insert` was called on each of the three counts and on a view layer's `use`. It printed:

```
KEY cycles.samples               -> True
KEY eevee.taa_render_samples     -> True
KEY display.render_aa            -> ERROR bpy_struct.keyframe_insert() property "render_aa" not animatable
KEY view_layer.use               -> True
```

So Cycles' and EEVEE's counts can be keyframed and Workbench's anti-aliasing choice cannot.

A file was then saved with Cycles, 64 by 64, adaptive sampling off, denoising off, and Max Samples keyframed to 4 at frame 1 and 512 at frame 2. Rendering it with `blender -b test.blend -o out/f_ -F PNG -s 1 -e 2 -a` printed:

```
Fra:1 Mem:8.51M (Peak 9.10M) | Time:00:00.00 | Mem:0.15M, Peak:0.15M | Scene, ViewLayer | Sample 4/4
Fra:2 Mem:8.51M (Peak 9.10M) | Time:00:00.09 | Mem:0.15M, Peak:0.15M | Scene, ViewLayer | Sample 512/512
```

So the sample count does change from frame to frame, and the renderer reads the new value for each frame.

**Hand test 10 — the view layer switch per frame.** Written up under question 1; in short, a view layer keyframed on at frame 1 and off at frame 2 produced a frame 1 file holding `Layer2.Combined` and `ViewLayer.Combined` and a frame 2 file holding only `ViewLayer.Combined`.

### Observed — hand test 24, run by the Creator (2026-08-05)

Run by the Creator in Blender 4.5's own window, on their own computer — a MacBook Pro with an M1 Pro chip, macOS 26.5.2 — on 2026-08-05. That settles macOS and leaves Windows and Linux untested. Label: **observed**.

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

A second shot shows the panel itself, with no menu open: **Noise Threshold** (ticked, 0.0100), **Max Samples** 4096, **Min Samples** 0, **Time Limit** 0 s, then the **Denoise** panel. No dot appears to the right of any of those four rows.

**This settles the point the finding above left standing.** The panel is drawn without decorators, exactly as Cycles' own interface file suggested — there is no dot to click. But the one-click route is not the only route: the right-click menu offers **Insert Keyframe** on the same field, with `I` as its shortcut. So the count can be keyframed from the interface, just not from a dot.

### Observed on the rented Linux machine — Linux hand tests 11 and 12 (2026-08-06)

Run over SSH from a terminal on the rented Linux machine, using Blender 4.5.12 at `/opt/blender/blender-4.5.12-linux-x64/blender`, with `BLENDER_USER_RESOURCES` pointed at `/root/smoozy/Settings/res-4.5`, a folder of this topic's own that already existed. That folder was checked before use and reported its own config directory and Blender's eight default add-ons and nothing else. The machine is a container sharing someone else's hardware, and up to four topics were being worked on it at once, so nothing below claims anything about how long a run took or how much memory was free. Label: **observed**.

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

**Answer.** The sources do not establish this for every engine. They say command line rendering does not need a graphical display, while EEVEE has a stated limitation on headless Windows systems. They do not define whether "no screen attached" and "headless" mean the same thing, and they do not settle Workbench.

The general claim:

> One advantage of using the command line is that we do not need a graphical display (no need for X server on Linux for example) and consequently we can render via a remote shell (typically SSH).

- Source: manual — https://docs.blender.org/manual/en/4.5/advanced/command_line/render.html
- Date read: 2026-07-30
- Label: **documented**

The exception, on EEVEE's own limitations page:

> Headless Rendering
> Headless rendering is not supported on headless Windows systems.

And on the same page:

> CPU Rendering
> Being a rasterization engine, EEVEE only uses the power of the GPU to render. There is no plan to support CPU (software) rendering as it would be very inefficient.

- Source: manual — https://docs.blender.org/manual/en/4.5/render/eevee/limitations/limitations.html
- Date read: 2026-07-30
- Label: **documented**

Cycles, by contrast, has a documented processor option:

> ```
> enum_devices = (
>     ('CPU', "CPU", "Use CPU for rendering"),
>     ('GPU', "GPU Compute",
>         "Use GPU compute device for rendering, configured in the system tab in the user preferences"),
> )
> ```

- Source: Cycles' own files — `properties.py`, lines 30–34
- Date read: 2026-07-30
- Label: **observed**

**Workbench: not in the sources.** I searched the whole manual for "headless", "background mode", "no GPU", "without a GPU", "software rendering", "no display", "X server", "display server", and read all nine Workbench pages (`render/workbench/*.txt`) grepping them for "GPU", "OpenGL" and "rasteriz" — no hits at all. Blender's 4.5 release notes were searched for "background", "headless", "EGL" and "offscreen"; the only hits were unrelated bug fixes and a Python note. Nothing in these sources says whether Workbench renders on a screenless machine.

**Not covered.** The word "headless" is used in the EEVEE quote without being defined, and the quote is specific to Windows — the sources do not say what happens on a headless Linux or macOS machine with EEVEE, nor whether "no screen attached" and "headless" mean the same thing (a machine can have no monitor but still run a display server). Nothing says whether a third-party engine add-on would work screenless.

**Outside the interface.** This question *is* the outside-the-interface case. The general claim quoted above is made specifically about the command line. The EEVEE limitation is stated flatly, with no mention of how the render was started, so I cannot say whether it is narrower than it reads.

**Operating system.** This is precisely where it differs. EEVEE's headless limitation is named for Windows only. The command line page's screenless claim names Linux ("no need for X server on Linux for example"). macOS is not addressed either way in this connection. The requirements page names Windows, macOS and Linux as the operating systems Blender 4.5 runs on. **Observed 2026-08-05 (hand test 14, below):** all three engines render with no window on macOS. Windows and Linux were not tested — that test was run on the Creator's one MacBook Pro — so the EEVEE limitation named for Windows remains untested, and so does the screenless case on any system. **Observed 2026-08-06:** on a rented Linux machine with one graphics card, all three engines rendered with no window and again with no display reachable at all, EEVEE included — but that machine had a display server running and its runs were only cut off from it. **Observed 2026-08-06 on a second rented Linux machine that has no display server at all** — two NVIDIA GeForce RTX 4090s, driver 580.95.05, sixty-one cores, over SSH — every engine this version has rendered there, on the cards and on the processor, EEVEE included. So **the screenless case is now answered for Linux**, in the sense of a machine with no display server. The headless *Windows* case the manual names stays untested, no Windows machine having run it, and so does a machine with no graphics hardware at all.

### Observed — hand test 14 (2026-08-05)

Run from a terminal on the Creator's own computer — a MacBook Pro with an M1 Pro chip, macOS 26.5.2, Apple's own graphics and no other graphics card — using Blender 4.5.12 at `/Applications/Blender 4.5.12.app/Contents/MacOS/Blender`, with the `BLENDER_USER_RESOURCES` setting pointed at a fresh empty folder. Label: **observed**.

A file was saved from factory startup at 64 by 64, Cycles samples 4, EEVEE render samples 4. It was rendered three times, once per engine, each time with `-b` so no window opened:

```
CYCLES               exit=0  image=64x64
BLENDER_EEVEE_NEXT   exit=0  image=64x64
BLENDER_WORKBENCH    exit=0  image=64x64
```

No error was printed by any of the three.

**What this does not settle.** This machine has a screen attached. Running with no window is not the same as running on a machine with no screen, and the sources' one hard statement is about a headless *Windows* machine. That test needs a machine with no display, reached over a remote shell, and a Windows machine — neither of which the Creator has. It stays unanswered.

### Observed on the rented Linux machine — Linux hand tests 15 and 16 (2026-08-06)

Run over SSH from a terminal on the rented Linux machine, using Blender 4.5.12 at `/opt/blender/blender-4.5.12-linux-x64/blender`, with `BLENDER_USER_RESOURCES` pointed at `/root/smoozy/Settings/res-4.5`, a folder of this topic's own that already existed. That folder was checked before use and reported its own config directory and Blender's eight default add-ons and nothing else. The machine is a container sharing someone else's hardware, and up to four topics were being worked on it at once, so nothing below claims anything about how long a run took or how much memory was free. Label: **observed**.

These are a second machine's answers to the tests the Creator's Mac answered on 2026-08-05. The two are never merged: where they agree it says so, and where they differ it says that plainly. The full list is `Audits/Blender Research/Settings/Hand tests — Linux.md`.

**Linux hand test 15 — each engine with no window, on a machine whose display is reachable.** A file with the default cube, camera and light at 64 by 64, Cycles samples 4, EEVEE render samples 4, rendered three times with `-b` so no window opened:

```
CYCLES              exit=0  image=64x64
BLENDER_EEVEE_NEXT    exit=0  image=64x64
BLENDER_WORKBENCH   exit=0  image=64x64
```

No error was printed by any of the three. **The two machines agree** — this is the same result the Mac gave on macOS.

**Linux hand test 16 — each engine with no display at all.** The same three renders were run again with the display taken out of the environment entirely (`env -u DISPLAY -u XAUTHORITY`), so nothing was left for Blender to reach a display through:

```
CYCLES              exit=0  image=64x64
BLENDER_EEVEE_NEXT    exit=0  image=64x64
BLENDER_WORKBENCH   exit=0  image=64x64
```

All three engines rendered and wrote their image, **EEVEE included**. No error, no warning, and nothing about a display in any of the three runs.

**And what Blender says when asked for a window with no display.** The same removal, but without `-b`, so Blender tried to open its interface. It printed:

```
GHOST: failed to initialize display for back-end(s): ['WAYLAND', 'X11']
  'WAYLAND': unable to connect to display!
  'X11': unable to open a display!
GHOST: unable to initialize, exiting!
```

and then **did not end**: it was killed by a sixty-second cut-off, exit code 124. Blender 4.5.12 has two other cases on this machine where it prints its answer and then hangs — see question 13.

**What this settles and what it does not.** It settles that a run which cannot reach a display at all renders on all three engines on this Linux machine, and that Blender says so plainly and stops when a window is asked for instead. It does **not** settle a machine with no screen physically attached: this box has a display running and the run was only cut off from it, which is a narrower thing. And it does not touch the manual's one hard statement, which names a headless *Windows* machine — no Windows machine has run it. **Untried rather than untestable: a Windows machine can be rented, and every one this project has read reported a Server edition — see the equipment section of `Preparation/Blender Research/Method.md`.** So this question is now answered for Linux in the sense of "no display reachable", and stays open for "no screen attached" and for Windows.

### Observed on a machine with no screen — the two-card rented Linux machine (2026-08-06)

Run over SSH on a **second rented Linux machine — Ubuntu 22.04.5 LTS, two NVIDIA GeForce RTX 4090s of 24 GB each on driver 580.95.05, an AMD EPYC 7B13 with sixty-one cores, 98 GB of memory, no display server at all** — using Blender at `/opt/blender/blender-4.5.12-linux-x64/blender`, which `--version` reports as `Blender 4.5.12 LTS (hash 84afd5f785f7 built 2026-07-21 01:31:22)`, with `BLENDER_USER_RESOURCES` pointed at `/root/smoozy/no screen/res-4.5`, a folder of its own that already existed and was empty. Nothing else was running on the machine. Label: **observed**. The full list is `Audits/Blender Research/Settings/Hand tests — two cards and no screen.md`.

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
GPU  CYCLES exit=0  png=4957
GPU  BLENDER_EEVEE_NEXT exit=0  png=3400
GPU  BLENDER_EEVEE exit=124  png=no file  Error: engine not found 'BLENDER_EEVEE'
GPU  BLENDER_WORKBENCH exit=0  png=3613
CPU  CYCLES exit=0  png=4957
CPU  BLENDER_EEVEE_NEXT exit=0  png=3400
CPU  BLENDER_EEVEE exit=124  png=no file  Error: engine not found 'BLENDER_EEVEE'
CPU  BLENDER_WORKBENCH exit=0  png=3613
```

`BLENDER_EEVEE` is not an engine this version has — its own `-E help` lists `BLENDER_EEVEE_NEXT`, `BLENDER_WORKBENCH` and `CYCLES` — and asked for it, 4.5 printed `Error: engine not found 'BLENDER_EEVEE'` and then did not end, killed at 120 seconds. The three engines it does have all rendered and wrote their image, **EEVEE included**, with no error and nothing about a display in any of them.

The same runs repeated with `--debug-cycles` showed the work really was where it was asked for:

```
on the card:      Path tracing on: NVIDIA GeForce RTX 4090 (OptiX) [CUDA_NVIDIA GeForce RTX 4090_0000:00:0a_OptiX]
                                   NVIDIA GeForce RTX 4090 (OptiX) [CUDA_NVIDIA GeForce RTX 4090_0000:00:07_OptiX]
on the processor: Path tracing on: AMD EPYC 7B13 64-Core Processor (CPU) (61 threads) [CPU]
```

Both cards are named in the card run. Nothing here says how the work was divided between them, or which cards were switched on in the preferences, and nothing is read into it. *(Restored by the two-card citation check, 2026-08-06: this block had been shortened to one card with an ellipsis, while the record it comes from shows two.)*

**And what Blender says when asked for a window on a machine with no display server.** The same command without `-b`, given `--python-expr "import bpy; print('RAN WITH A WINDOW')"` under `timeout 120`. `RAN WITH A WINDOW` was never printed. It printed:

```
GHOST: failed to initialize display for back-end(s): ['WAYLAND', 'X11']
  'WAYLAND': unable to connect to display!
  'X11': unable to open a display!
GHOST: unable to initialize, exiting!
```

on **standard error**, and then **did not end**: it was killed by a 120-second cut-off, exit status **124**. Word for word what the one-card machine printed when cut off from its display, and the same exit status.

**`blender -b -E help` hung here too**, exit `124` at sixty seconds, but on this machine it printed its listing first:

```
Blender 4.5.12 LTS (hash 84afd5f785f7 built 2026-07-21 01:31:22)
Blender Engine Listing:
	BLENDER_EEVEE_NEXT
	BLENDER_WORKBENCH
	CYCLES
```

On the one-card machine it printed nothing before hanging; on the Creator's Mac it printed and stopped. All three stand and none is chosen between here.

**What this settles.** **On Linux, every render engine this version has renders on a computer with no display server at all** — not merely a run cut off from a display. That is the half of this question item A of the Linux list left open, and it is now answered for Linux. **It does not settle a machine with no screen physically attached in the sense of no graphics hardware**: the cards are present, they are simply not driving a display. And it does not touch the manual's one hard statement, which names a headless *Windows* machine — no Windows machine has run it.

---

## 7. Does a scene set up for one render engine come out the same in another?

**Answer.** Not always. The documented limits show that a scene using some features cannot come out the same in another engine. The sources do not state a general rule for every possible scene.

> What the render looks like is defined by cameras, lights and materials. These are shared between EEVEE and Cycles, however some features are only supported in one or the other.

- Source: manual — https://docs.blender.org/manual/en/4.5/render/introduction.html
- Date read: 2026-07-30
- Label: **documented**

> EEVEE materials are created using the same shader nodes as Cycles, making it easy to render existing scenes. … While EEVEE is designed to use PBR principles, it is not perfect and Cycles will always provide more physically accurate renders. For these reasons, EEVEE has a set of limitations.

- Source: manual — https://docs.blender.org/manual/en/4.5/render/eevee/introduction.html
- Date read: 2026-07-30
- Label: **documented**

The limitations page lists specifics, for example:

> Only 14 attributes from Geometry Nodes are supported in a material
>
> Cameras — Only perspective and orthographic projections are currently supported.
>
> Lights can only have one color and do not support light node trees.
>
> Unlike in Cycles, the Size of spot lights does not change the softness of the cone.
>
> Volumetrics — Only single scattering is supported.

- Source: manual — https://docs.blender.org/manual/en/4.5/render/eevee/limitations/limitations.html
- Date read: 2026-07-30
- Label: **documented**

And the traffic runs both ways — some nodes exist only in EEVEE:

> EEVEE only Nodes
> These nodes are only available if EEVEE is the active render engine. These nodes will not work in Cycles.
> Shader to RGB … Specular BSDF …
>
> In the general case, shader nodes should behave more or less like in Cycles. … Although most BSDFs are supported, many of them are approximations and are not feature complete.

- Source: manual — https://docs.blender.org/manual/en/4.5/render/eevee/limitations/nodes_support.html
- Date read: 2026-07-30
- Label: **documented**

Workbench is further away still:

> Unlike other render engines such as EEVEE or Cycles, the Workbench engine does not use shader nodes. Instead, shading settings can be tweaked in the 3D Viewport's Shading popover or the render properties when doing final renders.
>
> It is not intended to be a render engine that will render final images for a project.

- Source: manual — https://docs.blender.org/manual/en/4.5/render/workbench/introduction.html
- Date read: 2026-07-30
- Label: **documented**

On top of the picture differences, each engine's own settings simply do not carry over — see question 3.

**Not covered.** The sources describe which features are missing or approximated; they do not quantify how different the resulting image is, and they say nothing at all about a scene set up for a third-party engine add-on. The EEVEE limitations page calls itself "a rather exhaustive list", not an exhaustive one.

**Outside the interface.** The sources do not say, and nothing in these statements is framed around the interface. Note the "EEVEE only Nodes" phrasing — "only available if EEVEE is the active render engine" — which is about the engine setting, not about who is driving Blender.

**Operating system.** The sources do not say; none of these statements mentions an operating system.

---

## 8. Can one scene be set up to take in another scene's rendered result, so that rendering the first renders the second?

**Answer.** Yes, two documented ways.

**In the compositor, a Render Layers node pointed at another scene:**

> Render Layers Node
> Renders a View Layer and reads its Passes into the compositing node graph.
>
> Properties
> Scene — The scene for which to render a view layer.
> View Layer — The view layer to render. The button next to the dropdown re-renders it immediately.
>
> Hint
> To use the compositing output from another scene rather than its "raw" render output, first render that scene into a series of multi-layered images (using e.g. the OpenEXR format), then load those images into the Compositor of the current scene using the Image Node.
>
> Note
> The viewport compositor only supports render passes when using EEVEE. For other engines, the passes will be empty.

- Source: manual — https://docs.blender.org/manual/en/4.5/compositing/types/input/scene/render_layers.html
- Date read: 2026-07-30
- Label: **documented**

**In the sequencer, a Scene strip:**

> Scene strips are a way to insert the render output of another scene into your sequence. Instead of rendering out a video, then inserting the video file, you can insert the scene directly.
>
> The strip length will be determined based on the animation settings in that scene.
>
> Note
> Scene strips cannot be used to reference the sequence's own scene; a secondary scene must be used instead.
>
> Input — Input type to use for the Scene strip.
> Camera: Use the Scene's 3D camera as input.
> Sequencer: Use the scene's Sequencer timeline as input, allowing one scene to reuse another scene's edit (instead of taking the render output from the scene).
>
> Limitations
> Scene strips do not render individual Render Passes; only the Combined render pass will be used.

- Source: manual — https://docs.blender.org/manual/en/4.5/video_editing/edit/montage/strips/scene.html
- Date read: 2026-07-30
- Label: **documented**

**Not covered.** The Render Layers node's page says it "Renders a View Layer" but does not say which engine or which sample count the *other* scene renders with, nor what frame of the other scene is used, nor whether the second scene's own compositor runs (the Hint quoted above implies it does not, and offers a workaround — that implication is the manual's own wording, but the manual does not state the rule directly). The Scene strip page does not say what engine or settings the referenced scene renders with either. Neither page says whether nesting can go more than one level deep.

**Outside the interface.** The cited pages do not say whether either mechanism differs in a command line render. The one interface-only phrase is the Render Layers node's "The button next to the dropdown re-renders it immediately", and the page also notes that the viewport compositor is a viewport feature. **Observed 2026-08-05 (hand tests 17, 18 and 19, below):** both mechanisms work in a render started from a terminal with no window.

**Operating system.** The sources do not say; neither page mentions an operating system. The tests below were run on macOS only. **Observed 2026-08-06:** the same tests were run again on a rented Linux machine and came back the same, so this holds on macOS and on Linux. Windows stays untested.

### Observed — hand tests 17, 18 and 19 (2026-08-05)

All three were run from a terminal on the Creator's own computer — a MacBook Pro with an M1 Pro chip, macOS 26.5.2 — using Blender 4.5.12 at `/Applications/Blender 4.5.12.app/Contents/MacOS/Blender`, with the `BLENDER_USER_RESOURCES` setting pointed at a fresh empty folder. Label: **observed**.

**Hand test 17 — a Render Layers node pointed at another scene.** A file was saved with two scenes. `Alpha` used Workbench at 64 by 64 with its compositing pipeline on, and its compositor held one Render Layers node with Scene set to `Beta`, wired to the Composite node. `Beta` used Cycles with 8 samples, and its own compositor inverted its render. Rendering `Alpha`:

```
blender -b test.blend -S Alpha -o out/c_ -F PNG -f 1
```

printed, during `Alpha`'s render:

```
Fra:1 Mem:10.71M (Peak 11.37M) | Time:00:00.00 | Mem:0.09M, Peak:0.09M | Beta, ViewLayer | Sample 0/8
Fra:1 Mem:11.09M (Peak 11.37M) | Time:00:00.00 | Remaining:00:00.00 | Mem:0.38M, Peak:0.38M | Beta, ViewLayer | Sample 1/8
Fra:1 Mem:11.22M (Peak 11.37M) | Time:00:00.06 | Mem:0.38M, Peak:0.38M | Beta, ViewLayer | Sample 8/8
```

So rendering the first scene does render the second, with the second scene's own engine and its own sample count.

Three images were then compared, all built from the same file: `Alpha`'s output, `Beta` rendered directly with its compositing pipeline on, and `Beta` rendered directly with its compositing pipeline off. The first pixel of each:

```
alpha_0001.png    [0.0, 0.0, 0.0, 1.0]
betacomp_0001.png [0.7686, 0.7686, 0.7686, 1.0]
betaraw_0001.png  [0.0, 0.0, 0.0, 1.0]
```

`Alpha`'s output matches `Beta`'s raw render, not `Beta`'s composited one. So the second scene's own compositor does not run when it is pulled in this way.

**Hand test 18 — two scenes' Render Layers nodes pointed at each other.** A file was saved with `Alpha`'s node pointed at `Beta` and `Beta`'s node pointed at `Alpha`, both compositing pipelines on, both Workbench. Rendering `Alpha` with a sixty-second cut-off finished in 0.6 seconds, exit code 0, and wrote an image. The output ended:

```
Scene switch for render: 'Alpha' in file: '.../circular.blend'
Fra:1 Mem:10.48M (Peak 10.48M) | Time:00:00.06 | Compositing
Saved: '.../out/x_0001.png'
Time: 00:00.11 (Saving: 00:00.04)
Blender quit
```

No error, no hang, and no sign of the chain repeating.

**Hand test 19 — a Scene strip.** A file was saved with `Alpha` on Workbench, its sequencer pipeline on and one Scene strip referencing `Beta` at frame 1; `Beta` used Cycles with 8 samples. Rendering `Alpha` from the command line printed lines naming `Beta, ViewLayer` throughout, and wrote an image. So a Scene strip is rendered from outside Blender too, with the referenced scene's own engine.

### Observed on the rented Linux machine — Linux hand tests 19, 20 and 21 (2026-08-06)

Run over SSH from a terminal on the rented Linux machine, using Blender 4.5.12 at `/opt/blender/blender-4.5.12-linux-x64/blender`, with `BLENDER_USER_RESOURCES` pointed at `/root/smoozy/Settings/res-4.5`, a folder of this topic's own that already existed. That folder was checked before use and reported its own config directory and Blender's eight default add-ons and nothing else. The machine is a container sharing someone else's hardware, and up to four topics were being worked on it at once, so nothing below claims anything about how long a run took or how much memory was free. Label: **observed**.

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

**Answer.** For Cycles, yes — a per-scene choice between two values. Which actual card is used is *not* in the file. (Cycles' own files carry that the setting exists on the scene, with those two values; that a saved file keeps it is settled by hand test 7 below, not by the quoted code. *[Citation check, 2026-08-05: added, because the quoted code shows a setting on the scene and not what a file records.]*)

The setting, in Cycles' own files:

> ```
> class CyclesRenderSettings(bpy.types.PropertyGroup):
>
>     device: EnumProperty(
>         name="Device",
>         description="Device to use for rendering",
>         items=enum_devices,
>         default='CPU',
>     )
> ```
> with
> ```
> enum_devices = (
>     ('CPU', "CPU", "Use CPU for rendering"),
>     ('GPU', "GPU Compute",
>         "Use GPU compute device for rendering, configured in the system tab in the user preferences"),
> )
> ```

- Source: Cycles' own files — `properties.py`, `device` at lines 376–383, `enum_devices` at lines 30–34. `CyclesRenderSettings` registers itself as `bpy.types.Scene.cycles` (lines 1100–1106), so the setting hangs off the scene.
- Date read: 2026-07-30
- Label: **observed**

The manual says the same, and draws the line between what is in the scene and what is in the preferences:

> To enable GPU rendering, go into the Preferences ‣ System ‣ Cycles Render Devices, and select either CUDA, OptiX, HIP, oneAPI, or Metal. Next, you must configure each scene to use GPU rendering in Properties ‣ Render ‣ Device.

- Source: manual — https://docs.blender.org/manual/en/4.5/render/cycles/gpu_rendering.html
- Date read: 2026-07-30
- Label: **documented**

So: the scene records "processor" or "graphics card"; the choice of technology (CUDA / OptiX / HIP / oneAPI / Metal) and of which particular cards to use sits in the user preferences of the computer doing the rendering, not in the file. That reading of the split is **inferred** from the two sources above put together — the manual says the technology is chosen in Preferences, and Cycles' own file offers only the two-way `CPU` / `GPU` choice on the scene.

Blender also provides a launch setting for the device:

> `--cycles-device <device>`
> Set the device used for rendering.
> Valid options are: CPU CUDA OPTIX HIP ONEAPI METAL.
> Append +CPU to a GPU device to render on both CPU and GPU.
> Example: `blender -b file.blend -f 20 -- --cycles-device OPTIX`

- Source: manual — https://docs.blender.org/manual/en/4.5/advanced/command_line/arguments.html
- Date read: 2026-07-30
- Label: **documented**

For EEVEE, there is nothing to record, because it has no choice:

> Being a rasterization engine, EEVEE only uses the power of the GPU to render. There is no plan to support CPU (software) rendering…

- Source: manual — https://docs.blender.org/manual/en/4.5/render/eevee/limitations/limitations.html
- Date read: 2026-07-30
- Label: **documented**

For Workbench, **not in the sources** — I read all nine Workbench manual pages and grepped them for "GPU" and "OpenGL" with no hits, and `bpy.types.SceneDisplay` has no device property.

**Not covered.** Whether the `device` value survives a save and reload is not stated anywhere; I am reporting that the property exists on the scene, not that the file writes it. Nor do the sources say what happens when a file says GPU and the rendering computer's preferences have no GPU configured, or how `--cycles-device` interacts with a `+CPU` suffix and the scene's own setting.

**Outside the interface.** The scene setting applies either way, and `--cycles-device` exists specifically for the outside case. Note that the preferences half of the split is the user preferences of whichever computer runs the render — so the same file can render on the processor on one machine and a card on another without the file changing. That last sentence is **inferred** from the manual's split quoted above.

**Operating system.** The available GPU technologies differ by operating system. The manual says:
> CUDA is supported on Windows and Linux…
> OptiX is supported on Windows and Linux…
> HIP is supported on Windows and Linux…
> oneAPI is a computation library that is supported on Windows and Linux…
> Metal is supported on Apple computers with Apple Silicon. macOS 13.0 or newer is required to support all features.
(same page as above). The documented technologies are not available on every operating system. **Observed 2026-08-05:** on this machine, Metal was the graphics technology found. CUDA, OptiX, HIP and oneAPI could not be tried at all — the Creator has Apple's own graphics and no other card — so what a file's `GPU` choice means on any other make of card stays untested. **Observed 2026-08-06:** a rented Linux machine with an NVIDIA GeForce RTX 4090 settles most of what was left here. That machine offers `CUDA`, `OPTIX`, `HIP` and `ONEAPI` and not `METAL`; `--cycles-device CUDA` and `OPTIX` render on the card, and `HIP`, `ONEAPI` and `METAL` are all refused with the same message. A file asking for GPU Compute renders on the card under CUDA or OptiX and falls back silently to the processor under the others. What an AMD or an Intel card does stays untested, and so does Windows.

### Observed — hand tests 7 and 8 (2026-08-05)

Both were run from a terminal on the Creator's own computer — a MacBook Pro with an M1 Pro chip, macOS 26.5.2, Apple's own graphics and no other graphics card — using Blender 4.5.12 at `/Applications/Blender 4.5.12.app/Contents/MacOS/Blender`, with the `BLENDER_USER_RESOURCES` setting pointed at fresh empty folders so the Creator's own Blender settings were untouched. Label: **observed**.

**Hand test 7 — does the choice survive a save, and is it per scene?** A file was saved from factory startup with two scenes: `One` with the Cycles device set to `GPU` and the compositor device set to `GPU`, and `Two` left at its defaults. A fresh Blender opened the file and printed:

```
SCENE One cycles_device= GPU compositor_device= GPU
SCENE Two cycles_device= CPU compositor_device= CPU
```

So the file does carry the choice, and it is held one scene at a time.

**Hand test 8 — what `--cycles-device` beats, and the double dash.** A file was saved with Cycles on the processor, 100 by 100, 16 samples. A throwaway settings folder was prepared in which Metal was chosen and the machine's devices switched on; it reported:

```
DEVICE: CPU Apple M1 Pro use= True
DEVICE: METAL Apple M1 Pro (GPU - 14 cores) use= True
SAVED compute_device_type= METAL
```

With the file still saying `CPU`, running

```
blender -b test.blend -o out/a_ -F PNG -f 1 -- --cycles-device METAL --cycles-print-stats
```

printed `Profiling information not available (only works with CPU rendering)` in its statistics, while the same run without the override did not. So the argument does beat what the file says.

Without the double dash:

```
blender -b test.blend -o out/b_ -F PNG -f 1 --cycles-device METAL
unknown argument, loading as file: --cycles-device
Error: Cannot read file ".../--cycles-device": No such file or directory
```

With a name no device bears:

```
blender -b test.blend -o out/c_ -F PNG -f 1 -- --cycles-device NOT_A_DEVICE
Error: NOT_A_DEVICE is not a valid Cycles device.
```

and no image was written.

Finally, a second throwaway settings folder was prepared with the compute device type set to `NONE`, and the file's own scene device forced to `GPU` at launch. The whole terminal output of that run named no device, printed no warning of any kind, and ended:

```
Fra:1 Mem:8.29M (Peak 9.09M) | Time:00:00.07 | Mem:0.02M, Peak:0.02M | Scene, ViewLayer | Sample 16/16
Saved: '.../out/dd_0001.png'
```

The statistics carried no `Profiling information not available` line, which the override run did carry. So a file asking for the graphics card on a machine with none enabled renders on the processor, writes the image, and says nothing about it.

**Workbench.** Hand test 13 (under question 3) listed every property of `scene.display`: nine, none of them a device choice.

### Observed — hand test 27, run by the Creator (2026-08-05)

Run by the Creator in Blender 4.5's own window, on their own computer — a MacBook Pro with an M1 Pro chip, macOS 26.5.2 — on 2026-08-05. That settles macOS and leaves Windows and Linux untested. Label: **observed**.

With the Render Engine set to **Workbench**, they expanded every panel of the Render Properties tab and photographed the whole of it. The panels, in order: Simplify, Performance, Sampling, Lighting, Object Color, Options, Film, Grease Pencil, Freestyle, Color Management.

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

Those are the compositor's and the denoise nodes' devices — the two engine-neutral settings on `RenderSettings` recorded above — shown here under Workbench because that tab carries them whatever the engine. **No control anywhere in the tab is a Workbench device setting.** The nearest thing to one elsewhere is Color Management ▸ **Display Device**, reading `sRGB`, which is a colour display device rather than a processor or a graphics card.

**This closes the "not in the sources" answer for Workbench.** Workbench itself offers no choice between the processor and the graphics card; what the tab does offer is two shared choices that belong to compositing and denoising.

### Observed on the rented Linux machine — Linux hand tests 7, 8, 9, 10 and 25 (2026-08-06)

Run over SSH from a terminal on the rented Linux machine, using Blender 4.5.12 at `/opt/blender/blender-4.5.12-linux-x64/blender`, with `BLENDER_USER_RESOURCES` pointed at `/root/smoozy/Settings/res-4.5`, a folder of this topic's own that already existed. That folder was checked before use and reported its own config directory and Blender's eight default add-ons and nothing else. The machine is a container sharing someone else's hardware, and up to four topics were being worked on it at once, so nothing below claims anything about how long a run took or how much memory was free. Label: **observed**.

These are a second machine's answers to the tests the Creator's Mac answered on 2026-08-05. The two are never merged: where they agree it says so, and where they differ it says that plainly. The full list is `Audits/Blender Research/Settings/Hand tests — Linux.md`.

This is the question the new machine was most needed for. The Creator's Mac has Apple's own graphics and could reach only `METAL` and `CPU`; this one has an NVIDIA GeForce RTX 4090.

**Linux hand test 7 — does the choice survive a save, is it per scene, and what does a brand-new scene start at?** A brand-new factory scene, read without saving anything:

```
FACTORY engine= BLENDER_EEVEE_NEXT cycles_device= CPU compositor_device= CPU
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
Error: Found no Cycles device of the specified type
```

None of the three printed anything before the error — no warning about a missing library, nothing.

So on a machine with an NVIDIA card, `CUDA` and `OPTIX` both render; and **`HIP`, `ONEAPI` and `METAL` are all refused in exactly the same words**. A name for a technology no card here can serve behaves the same as a name for a technology this operating system cannot have at all. The refusal stops the render and writes nothing.

**Linux hand test 8 — a device name that names no device, and the missing double dash.** With `-- --cycles-device NOT_A_DEVICE`:

```
Error: NOT_A_DEVICE is not a valid Cycles device.
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

So the two-way choice in the file means "use whatever this machine's preferences chose", and when that choice finds no card the render falls back to the processor, writes the image and exits 0. **The two machines agree on the silence** — the Mac found the same fallback with no graphics device enabled — and this machine shows it happening for a second reason as well, a technology chosen for a card that is not there. What Blender said about it was nothing at all — not one line, in any of the three. Anything that needs to know whether a render actually used the graphics card cannot learn it from a warning, on either machine.

**Linux hand test 25 — Workbench's own Render Properties tab, photographed on this machine.** Blender was started with a real window on the machine's display, the engine set to Workbench, the Properties editor pointed at Render, and the window photographed with Blender's own screenshot command. Blender 4.5's tab lists Simplify, Performance, Sampling, Lighting and Object Color from the top, with no Curves panel. No control in the tab is a Workbench device setting, matching the Creator's photographs of the same tab on the Mac. **The two machines agree.**

---

## 10. What does Blender state about opening a file that was saved by a different version of Blender?

**Answer.** The manual has a page on exactly this. Quoting it in full where it bears on the question:

> Blender can open blend-files saved with both older versions of the software (backward compatibility), and newer ones (forward compatibility). This comes with some limitations though.
>
> Tip
> When having issues with opening much older (or newer) blend-files, it can help to use a few intermediary Blender releases to perform conversions by smaller steps.
>
> **Backward Compatibility**
> Opening older files and converting them for the current version of Blender is usually straight-forward. It is expected to give very good and usable results.
> There can be major feature changes, for which the backward compatibility will only be ensured for a limited amount of time. For example the changes to the animation system that happened during the Blender 2.5x project. This will never be less than a full major release cycle (i.e. two years at least).
>
> **Forward Compatibility**
> *Loss of Data* — Forward compatibility is inherently harder to ensure, and loss of feature should always be expected when opening a blend-file saved with a more recent version of Blender.
> A warning is shown in the UI when editing a more recent blend-file. Trying to overwrite it (with a simple 'Save' operation) will also show a confirmation popup, as this could make that loss of data permanent.
> *Complete Incompatibility* — When Blender switches to a new major version release (e.g. from 3.x to 4.0), there can also be major changes that will make the blend-file fully incompatible with older versions of Blender.
> In such cases, older Blender will fail opening (or appending/linking from) the newer blend-file, with a message stating which minimal version is needed to open it.
> In such cases, the last LTS release of the previous release cycle will be kept compatible with the newer file format version, and will be usable as converter between both versions.
> For example, Blender 3.6 LTS can open files from Blender 4.x, and will perform the necessary conversion such that when re-saved from 3.6, the files become compatible with all 3.x Blender versions.

- Source: manual — https://docs.blender.org/manual/en/4.5/files/blend/compatibility.html
- Date read: 2026-07-30
- Label: **documented**
- *[Citation check, 2026-08-05: one word restored to the quote — the page reads "(with a simple 'Save' operation)", not "(with a simple 'Save')".]*

The 4.5 release notes add one statement about files from a particular kind of machine:

> Blendfiles saved by Blender from a Big Endian platform will trigger a warning when opened.

- Source: release notes — https://developer.blender.org/docs/release_notes/4.5/core/
- Date read: 2026-07-30
- Label: **documented**

And, on the release notes index:

> **Big Endian** platform & blendfiles are deprecated and will be removed in Blender 5.0.

- Source: release notes — https://developer.blender.org/docs/release_notes/4.5/
- Date read: 2026-07-30
- Label: **documented**

**Not covered.** The page is about opening a file, and says nothing about *rendering* one — it does not say whether a converted file renders the same, or whether settings that came from a different version keep their values. It also does not say what "a warning is shown in the UI" becomes when there is no UI. The page links to "a more exhaustive documentation about compatibility handling, in the developer's documentation", which is not among my sources.

**Outside the interface.** Not stated, and there is a visible gap: the forward-compatibility protections described are interface things — "A warning is shown in the UI", "will also show a confirmation popup". The sources do not say what a render started from outside Blender does instead. The failure case, "older Blender will fail opening … with a message stating which minimal version is needed", is not tied to the interface. **Observed 2026-08-05 (hand tests 22 and 28, below):** both sides are now settled, and they agree. Driven from outside, a warning is printed to the terminal and the render goes ahead. Inside the window, the same sentence appears as a report box over the viewport and in the status bar, and the file opens.

**Operating system.** The compatibility page names no operating system. The Big Endian note is about the kind of processor the file was saved on, not the operating system. The test below was run on macOS only. **Observed 2026-08-06:** the same test was run again on a rented Linux machine and came back the same, so this holds on macOS and on Linux. Windows stays untested.

### Observed — hand test 22 (2026-08-05)

Run from a terminal on the Creator's own computer — a MacBook Pro with an M1 Pro chip, macOS 26.5.2 — with the `BLENDER_USER_RESOURCES` setting pointed at a fresh empty folder. Label: **observed**.

A file was saved from factory startup by Blender 5.2.0 (Workbench, 64 by 64), then opened and rendered by Blender 4.5.12 with no window:

```
blender -b from-5.2.blend -o out/z_ -F PNG -f 1
Blender 4.5.12 LTS (hash 84afd5f785f7 built 2026-07-21 01:41:35)
Read blend: ".../from-5.2.blend"
Warning: File written by newer Blender binary (502.44), expect loss of data!
Saved: '.../out/z_0001.png'
Time: 00:00.12 (Saving: 00:00.04)
Blender quit
Error: Not freed memory blocks: 4, total unfreed memory 0.000427 MB
```

Alongside those, several lines of the form `Warning: region type 11 missing in space type "Action" (id: 12) - removing region` were printed, and one line reading `regiondata free error`. Exit code 0, and the image was written. Reading the file's own recorded version in 4.5 printed `OPENED file version (5, 2, 44)`.

So the forward-compatibility warning does reach a terminal, in these words, and 4.5 opened and rendered a 5.2 file rather than refusing.

### Observed — hand test 28, run by the Creator (2026-08-05)

Run by the Creator in Blender 4.5.12's own window, on their own computer — a MacBook Pro with an M1 Pro chip, macOS 26.5.2 — on 2026-08-05. That settles macOS and leaves Windows and Linux untested. Label: **observed**.

They opened Blender 4.5.12 normally, chose File ▸ Open, opened the file saved by Blender 5.2 that had been prepared for this test, and photographed the whole window straight afterwards without saving.

What is visible: the file is open and its contents loaded — a cube, a camera and a light in the outliner, the cube selected in the viewport. A **Report: Warning** box floats over the viewport reading:

> File written by newer Blender binary (502.44), expect loss of data!

and the status bar along the bottom of the window carries a warning triangle and the same sentence:

> File written by newer Blender binary (502.44), expect loss of data!

The bottom right of the status bar reads `4.5.12`.

**This settles the inside half of the standing line below.** The warning the manual describes as shown "in the UI" is a report box over the viewport and a line in the status bar, and it uses the same words as the terminal warning quoted above. No dialog blocked the opening, and nothing had to be dismissed before the file was usable.

### Observed on the rented Linux machine — Linux hand tests 24 (2026-08-06)

Run over SSH from a terminal on the rented Linux machine, using Blender 4.5.12 at `/opt/blender/blender-4.5.12-linux-x64/blender`, with `BLENDER_USER_RESOURCES` pointed at `/root/smoozy/Settings/res-4.5`, a folder of this topic's own that already existed. That folder was checked before use and reported its own config directory and Blender's eight default add-ons and nothing else. The machine is a container sharing someone else's hardware, and up to four topics were being worked on it at once, so nothing below claims anything about how long a run took or how much memory was free. Label: **observed**.

These are a second machine's answers to the tests the Creator's Mac answered on 2026-08-05. The two are never merged: where they agree it says so, and where they differ it says that plainly. The full list is `Audits/Blender Research/Settings/Hand tests — Linux.md`.

**Linux hand test 24 — opening a file across versions with no window.** A factory-startup file saved by Blender 5.2 was opened and rendered by 4.5, 5.0 and 5.1, and a factory-startup file saved by 4.5 was opened and rendered by 5.2. In this version:

```
Read blend: "…/from-5.2.blend"
Warning: region type 11 missing in space type "Action" (id: 12) - removing region
Warning: region type 14 missing in space type "Node" (id: 16) - removing region
regiondata free error
Warning: region type 15 missing in space type "Node" (id: 16) - removing region
…(those lines repeat, fourteen of them in all)…
Warning: File written by newer Blender binary (502.44), expect loss of data!
Error: Engine 'BLENDER_EEVEE' not available for scene 'Scene' (an add-on may need to be installed or enabled)
Fra:1 Mem:71.01M (Peak 71.05M) | Time:00:00.79 | Rendering 1 / 64 samples
Fra:1 Mem:71.01M (Peak 71.05M) | Time:00:01.01 | Rendering 64 / 64 samples
Saved: '…/out/a-4.5-_0001.png'
Blender quit
Error: Not freed memory blocks: 4, total unfreed memory 0.000427 MB
```

Exit code 0, and the image was written.

**Two things about that.** The extra region warnings and the `Not freed memory blocks: 4` line are the same ones the Mac's run produced in 4.5 and in no other version, so **the two machines agree** on them. The engine line is **new here and is not in the Mac's record**: the 5.2 file names the engine `BLENDER_EEVEE`, which 4.5 does not have — 4.5's own EEVEE is `BLENDER_EEVEE_NEXT` (question 13). Blender 4.5 reported it, fell back to its own EEVEE and rendered anyway. Whether the Mac's run printed the same is not recorded there, so it is not claimed either way.

So the warning the manual describes as shown "in the interface" reaches a terminal on Linux as it does on macOS, and no older Blender refused the file.

---

## 11. Can a file rely on scripts stored inside it to render correctly, and what decides whether those scripts run when a render is started from outside Blender?

**Answer.** Yes, a file can hold and rely on scripts; whether they run is decided by a preference, a per-file trust flag, and two command line arguments.

**Scripts a file can hold:**

> The ability to include Python scripts within blend-files is valuable for advanced tasks such as rigging and automation. However, it poses a security risk since Python does not restrict what a script can do. … Automatic execution is disabled by default; however, some blend-files need this to function properly.
>
> **Auto Execution** — Here are the different ways blend-files may automatically run scripts.
> *Registered Text-Blocks* — A text data-block can have its Register option enabled which means it will load on start.
> *Animation Drivers* — Python expressions can be used to Drive values and are often used in more advanced rigs and animations.
>
> **Manual Execution** — There are other ways scripts in a blend-file may execute that require user interaction (therefore will run even when auto execution is off), but you should be aware that this is the case since it is not necessarily obvious.
> Running a script in the Text editor.
> Rendering with Freestyle, because Freestyle uses scripts to control line styles.

- Source: manual — https://docs.blender.org/manual/en/4.5/advanced/scripting/security.html
- Date read: 2026-07-30
- Label: **documented**

**What decides whether they run:**

> Blender provides a number of ways to control whether scripts from a blend-file are allowed to automatically execute.
> First, the File Browser has the option Trusted Source which you can use on a case-by-case basis to control auto execution. …
> In the Preferences, there is the toggle to Auto Run Python Scripts. This means the Trusted Source option in the File Browser will be enabled by default, and scripts can run when blend-files are loaded without using the File Browser. Once enabled you have the option to exclude certain directories; a typical configuration would be to trust all paths except for the download directory.
>
> **Command Line**
> You may want to perform batch rendering or some other task from the command line, running Blender without an interface. In this case, the Preferences are still used but you may want to override them:
> Enable with `-y` or `--enable-autoexec`
> Disable with `-Y` or `--disable-autoexec`
>
> Example — To render an animation in background mode, allowing drivers and other scripts to run:
> `blender --background --enable-autoexec my_movie.blend --render-anim`

- Source: manual — https://docs.blender.org/manual/en/4.5/advanced/scripting/security.html
- Date read: 2026-07-30
- Label: **documented**

The preference itself:

> Auto Run Python Scripts
> Python scripts (including driver expressions) are not executed by default for security reasons. You may be working on projects where you only load files from trusted sources, making it more convenient to allow scripts to be executed automatically.
> Excluded Paths — Blend-files in these folders will not automatically run Python scripts. This can be used to define where blend-files from untrusted sources are kept.

- Source: manual — https://docs.blender.org/manual/en/4.5/editors/preferences/save_load.html
- Date read: 2026-07-30
- Label: **documented**

> `use_scripts_auto_execute`
> Allow any .blend file to run scripts automatically (unsafe with blend files from an untrusted source)
> Type: boolean, default False

- Source: Python API — https://docs.blender.org/api/4.5/bpy.types.PreferencesFilePaths.html
- Date read: 2026-07-30
- Label: **documented**

**A conflict in the sources, which I am flagging rather than resolving.** The command line arguments page marks the *disabled* state as the default:

> `-y, --enable-autoexec` — Enable automatic Python script execution.
> `-Y, --disable-autoexec` — Disable automatic Python script execution (Python-drivers & startup scripts), (default).

- Source: manual — https://docs.blender.org/manual/en/4.5/advanced/command_line/arguments.html
- Date read: 2026-07-30

while the security page says "In this case, the Preferences are still used but you may want to override them". Both are in the 4.5 manual. I cannot tell from the sources whether a command line render with neither `-y` nor `-Y` follows the preference or is unconditionally off.

**A concrete case where a scripted file will not render right:**

> By default Blender will restrict execution of Python scripts.
> If using a Scripted Expression Driver Type that doesn't follow the Simple Expressions subset, you will have to open the file as Trusted Source, or set Auto Run Python Scripts in Preferences ‣ Save & Load ‣ Blender Files.

- Source: manual — https://docs.blender.org/manual/en/4.5/animation/drivers/troubleshooting.html
- Date read: 2026-07-30
- Label: **documented**

**Not covered.** The Freestyle case is listed under "Manual Execution … require user interaction (therefore will run even when auto execution is off)", but rendering with Freestyle from the command line involves no user interaction — the sources do not say what happens then. The sources also do not say whether the auto-run preference is read from the rendering computer's preferences when Blender is launched with `--factory-startup`, nor whether the Excluded Paths list applies to a file named on the command line.

**Outside the interface.** This is directly addressed, and quoted above: the Preferences still apply, `-y` / `-Y` override them, and the manual gives a worked background-render example. What does *not* survive the move outside is the interactive escape hatch — the manual says "When a blend-file tries to execute a script and is not allowed, a dialog will appear. In it, you can choose to Allow Execution or to Ignore the scripts", and a dialog has nowhere to appear in a command line render. The sources do not say what happens instead.

**Operating system.** The sources do not say; none of these pages or arguments carries an operating-system note. Note only that the preferences live in the user configuration directory, whose location can be redirected with `BLENDER_USER_CONFIG` (documented on the arguments page) — so which preferences apply depends on the account and machine, not on the operating system as such. The test below was run on macOS only. **Observed 2026-08-06:** the same test was run again on a rented Linux machine and gave the same six results, so this holds on macOS and on Linux. Windows stays untested.

### Observed — hand test 20 (2026-08-05)

Run from a terminal on the Creator's own computer — a MacBook Pro with an M1 Pro chip, macOS 26.5.2 — using Blender 4.5.12 at `/Applications/Blender 4.5.12.app/Contents/MacOS/Blender`, with the `BLENDER_USER_RESOURCES` setting pointed at two fresh empty folders, one with Auto Run Python Scripts off and one with it on, so the Creator's own Blender settings were untouched. Label: **observed**.

A file was saved from factory startup with a scripted-expression driver on the default cube's x position. Three expressions were tried in turn, and the driven value read back after `frame_set(1)`:

```
expression: len("xxxxx")                       no flag -> CUBE X = 5.0   -y -> 5.0   -Y -> 5.0
expression: sum([3.0,4.0])                      no flag -> CUBE X = 7.0   -y -> 7.0   -Y -> 7.0
expression: __import__("os").getpid()*0+7       no flag -> CUBE X = 0.0   -y -> 7.0   -Y -> 0.0
```

The first two ran whatever the flag. The third was blocked, and Blender printed:

```
BPY_driver_exec: restricted access disallows name '__import__', enable auto-execution to support
```

with the driven value falling to 0.0 rather than the render stopping.

The third expression was then run against both settings folders:

```
Auto-Run pref=off flag=none -> CUBE X = 0.0   BPY_driver_exec: restricted access disallows name '__import__', enable auto-execution to support
Auto-Run pref=off flag=-y   -> CUBE X = 7.0
Auto-Run pref=off flag=-Y   -> CUBE X = 0.0   BPY_driver_exec: restricted access disallows name '__import__', enable auto-execution to support
Auto-Run pref=on  flag=none -> CUBE X = 7.0
Auto-Run pref=on  flag=-y   -> CUBE X = 7.0
Auto-Run pref=on  flag=-Y   -> CUBE X = 0.0   BPY_driver_exec: restricted access disallows name '__import__', enable auto-execution to support
```

**This settles the conflict flagged above.** With neither `-y` nor `-Y`, a render started from outside Blender follows the preference. The arguments page's "(default)" on `-Y` describes the factory state of that preference, not an unconditional rule. And a blocked script is not silent: the line above is printed, and the value the driver would have supplied is 0.

### Observed on the rented Linux machine — Linux hand tests 22 and 2 (2026-08-06)

Run over SSH from a terminal on the rented Linux machine, using Blender 4.5.12 at `/opt/blender/blender-4.5.12-linux-x64/blender`, with `BLENDER_USER_RESOURCES` pointed at `/root/smoozy/Settings/res-4.5`, a folder of this topic's own that already existed. That folder was checked before use and reported its own config directory and Blender's eight default add-ons and nothing else. The machine is a container sharing someone else's hardware, and up to four topics were being worked on it at once, so nothing below claims anything about how long a run took or how much memory was free. Label: **observed**.

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

**Answer.** Yes, it can be pointed at a named scene. No source quoted here states which scene a command line render uses when `-S` is absent. The Python render operator says an unspecified scene is the current scene; applying that to the command line is inferred, not documented.

**Pointing at a named scene:**

> `-S, --scene <name>`
> Set the active scene <name> for rendering.

- Source: manual — https://docs.blender.org/manual/en/4.5/advanced/command_line/arguments.html
- Date read: 2026-07-30
- Label: **documented**

The manual also notes, in the general command line arguments guidance:

> See Command Line Arguments for a full list of arguments (for example to specify which scene to render, the end frame number, etc.)

- Source: manual — https://docs.blender.org/manual/en/4.5/advanced/command_line/render.html
- Date read: 2026-07-30
- Label: **documented**

**When not told:**

> `bpy.ops.render.render(*, animation=False, write_still=False, use_viewport=False, layer='', scene='')`
> Render active scene
> …
> `scene (string, (optional, never None))` – Scene, Scene to render, current scene if not specified

- Source: Python API — https://docs.blender.org/api/4.5/bpy.ops.render.html
- Date read: 2026-07-30
- Label: **documented** for the render operator. It is **inferred** that a command line render without `-S` follows this operator's default.

The render operator's default is "the active scene" / "current scene". Supporting that reading, the API describes a window's scene as:

> `scene` — Active scene to be edited in the window — Type: Scene, (never None)

- Source: Python API — https://docs.blender.org/api/4.5/bpy.types.Window.html
- Date read: 2026-07-30
- Label: **documented**

**Where the sources go silent.** Which of a file's scenes is the active one when the file is opened — and whether that is stored in the file — is not stated anywhere I found. I searched the whole manual for "active scene" (twenty-odd hits, all of them using the term rather than defining it: the topbar's Render menu "Render the active scene at the current frame", the properties editor, the scene data-block menu, and the `-S` argument itself), and read the scenes chapter (https://docs.blender.org/manual/en/4.5/scene_layout/scene/introduction.html), which describes creating, copying and deleting scenes but never says which is active on load. I also searched the API documentation for a definition, and searched the 4.5 release notes.

- Label for that gap: **not in the sources**

The scenes page does say one thing worth having:

> Each blend-file can contain multiple scenes, which share other data such as objects and materials.
> (Delete Scene) — Deletes the current scene data-block. Note, there always has to be one scene data-block.

- Source: manual — https://docs.blender.org/manual/en/4.5/scene_layout/scene/introduction.html
- Date read: 2026-07-30
- Label: **documented**. From "there always has to be one scene data-block" it **follows** that "the active scene" is never empty — the step is mine.

**Not covered.** What `-S` does with a name that no scene in the file bears is not stated. Nor is what happens when `-S` is given after `-a` or `-f` (the argument-order rule quoted in question 1 suggests it would be too late, but the manual does not say so for `-S` specifically). Nor whether a scene referenced by a Scene strip or a Render Layers node (question 8) also renders when the file is pointed at a different scene.

**Outside the interface.** This question is the outside-the-interface case throughout; `-S` exists only there.

**Operating system.** The sources do not say. `-S` carries no operating-system note on the arguments page, unlike the several arguments there that do. The test below was run on macOS only. **Observed 2026-08-06:** the same tests were run again on a rented Linux machine and came back the same, so this holds on macOS and on Linux. Windows stays untested.

### Observed — hand test 15 (2026-08-05)

Run from a terminal on the Creator's own computer — a MacBook Pro with an M1 Pro chip, macOS 26.5.2 — using Blender 4.5.12 at `/Applications/Blender 4.5.12.app/Contents/MacOS/Blender`, with the `BLENDER_USER_RESOURCES` setting pointed at a fresh empty folder. Label: **observed**.

A file was saved from factory startup with two scenes, `Alpha` (the default cube, camera and light) and `Beta` (a camera only), both Workbench at 64 by 64. `Beta` was made the window's scene before saving; the build step printed `MADE scenes= ['Alpha', 'Beta'] window scene= Beta`. Each render was run with a handler added at launch that prints the scene the render starts on:

```
--python-expr "import bpy; bpy.app.handlers.render_pre.append(lambda scene: print('RENDER_PRE SCENE:', scene.name))"
```

The results:

```
no -S            : RENDER_PRE SCENE: Beta
-S Alpha         : RENDER_PRE SCENE: Alpha
-S NoSuchScene   : RENDER_PRE SCENE: Beta
```

The `-S NoSuchScene` run also printed:

```
Can't find scene: 'NoSuchScene' in file: '.../test.blend'
```

and then went on to render, save `bad_0001.png` and exit 0.

So a render started from outside with no `-S` uses the scene that was showing when the file was saved; a name no scene bears is reported and then ignored, and the same scene is rendered.

**Not settled by this test.** The file was built and saved by a Blender with no window, using `bpy.context.window.scene`. Whether a file saved from the interface records the active scene the same way was not tested.

### Observed on the rented Linux machine — Linux hand tests 17 and 18 (2026-08-06)

Run over SSH from a terminal on the rented Linux machine, using Blender 4.5.12 at `/opt/blender/blender-4.5.12-linux-x64/blender`, with `BLENDER_USER_RESOURCES` pointed at `/root/smoozy/Settings/res-4.5`, a folder of this topic's own that already existed. That folder was checked before use and reported its own config directory and Blender's eight default add-ons and nothing else. The machine is a container sharing someone else's hardware, and up to four topics were being worked on it at once, so nothing below claims anything about how long a run took or how much memory was free. Label: **observed**.

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

**Linux hand test 18 — a Sequencer Scene against the active scene.** This was run in 4.5 too, so that its silence here is a tested silence rather than an untested one. The build step reported `BUILD sequencer_scene attribute does not exist in this version` — a workspace in Blender 4.5 has no Sequencer Scene at all. The file was still built with three scenes, `Alpha`, `Beta` and an `Edit` scene whose sequencer holds a Scene strip referencing `Alpha`, with `Beta` made the window's scene. A bare render printed `RENDER_PRE SCENE: Beta` — the active scene, as in every other case.

---

## 13. What decides the list of render engines a file can choose between, and can that list grow on a given computer?

**Answer.** The sources document three built-in engines, render engines supplied as add-ons, and add-ons being installed and enabled. It is **inferred** that the list available to a running Blender is whatever engines are registered there. The sources document that the available choices can grow when an add-on supplies another engine and is enabled.

> Blender includes three render engines with different strengths:
> EEVEE is a physically based realtime renderer.
> Cycles is a physically based path tracer.
> Workbench is designed for layout, modeling and previews.
> More renderers from third-party developers are available as add-ons.

- Source: manual — https://docs.blender.org/manual/en/4.5/render/introduction.html
- Date read: 2026-07-30
- Label: **documented**

Engines are Python classes that register themselves:

> ```
> class CustomRenderEngine(bpy.types.RenderEngine):
> # These three members are used by Blender to set up the
> # RenderEngine; define its internal name, visible name and capabilities.
> bl_idname = "CUSTOM"
> bl_label = "Custom"
> bl_use_preview = True
> ```
> …
> ```
> def register():
> # Register the RenderEngine.
> bpy.utils.register_class(CustomRenderEngine)
> ```

- Source: Python API — https://docs.blender.org/api/4.5/bpy.types.RenderEngine.html
- Date read: 2026-07-30
- Label: **documented**

Cycles itself is one of these, shipped as an add-on:

> ```
> bl_info = {
>     "name": "Cycles Render Engine",
>     …
>     "support": 'OFFICIAL',
>     "category": "Render"}
> ```
> ```
> class CyclesRender(bpy.types.RenderEngine):
>     bl_idname = 'CYCLES'
>     bl_label = "Cycles"
> ```

- Source: Cycles' own files — `__init__.py`, `bl_info` at lines 7–16, `CyclesRender` at lines 42–44
- Date read: 2026-07-30
- Label: **observed**

The engine choice on a scene is an enum, and Blender exposes whether more than one exists:

> `engine`
> Engine to use for rendering
> Type: enum in ['BLENDER_EEVEE_NEXT'], default 'BLENDER_EEVEE_NEXT'
>
> `has_multiple_engines`
> More than one rendering engine is available
> Type: boolean, default False, (readonly)

- Source: Python API — https://docs.blender.org/api/4.5/bpy.types.RenderSettings.html
- Date read: 2026-07-30
- Label: **documented** for the words. That the enum's contents are built at run time from whatever engines are registered is **inferred**, from two things put together: the published enum lists only `BLENDER_EEVEE_NEXT` even though the manual says Blender includes three engines and Cycles' own file declares `bl_idname = 'CYCLES'`; and Blender publishes a separate read-only flag meaning "More than one rendering engine is available", which would be pointless if the list were fixed. The step is mine — no page says the enum is dynamic.

**Growing the list on a given computer.** The command line can list what is available and can turn add-ons on at launch:

> `-E, --engine <engine>`
> Specify the render engine. Use `-E help` to list available engines.
>
> `--addons <addon(s)>`
> Comma separated list (no spaces) of add-ons to enable in addition to any default add-ons.

- Source: manual — https://docs.blender.org/manual/en/4.5/advanced/command_line/arguments.html
- Date read: 2026-07-30
- Label: **documented**

And from the interface:

> In this section you can search, install, enable and disable Add-ons.
> Blender comes with some preinstalled Add-ons already, ready to be enabled. But you can also add your own, or any interesting ones you find on the web.
> Install from Disk — Install an extension from a .zip package. This is installed to a Local Repository and no updates will be available.
> To enable or disable an add-on check or uncheck the box to the right of the add-ons. The add-on functionality should be immediately available.

- Source: manual — https://docs.blender.org/manual/en/4.5/editors/preferences/addons.html
- Date read: 2026-07-30
- Label: **documented**

> Extensions are add-ons or themes used to extend the core functionality of Blender. They are shared in online platforms, and can be installed and updated from within Blender.

- Source: manual — https://docs.blender.org/manual/en/4.5/advanced/extensions/index.html
- Date read: 2026-07-30
- Label: **documented**

**Not covered.** The sources do not say what happens to a file whose chosen engine is not registered on the computer opening it — whether it falls back, refuses, or keeps the name. They do not say whether `-E help` lists engines from add-ons that are not yet enabled. They do not give the identifier strings of the three built-in engines together in one place (I found `BLENDER_EEVEE_NEXT` in the API enum and `CYCLES` in Cycles' file; I searched all of the manual, the API documentation and the Cycles files for `BLENDER_WORKBENCH` and found no hits at all, so I cannot state Workbench's identifier). And nothing states whether the engine list is per-computer or per-user.

**Outside the interface.** Directly addressed: `-E help` lists the engines and `--addons` enables extra ones at launch, both from outside. The Preferences route is interface-only, but its effect (an enabled add-on) is stored in the user preferences and so would be in force for a later command line render — that last step is **inferred**, since the manual does not say so.

**Operating system.** The sources do not say for the engine list itself. The 4.5 release notes do carry one add-on note bearing on what add-ons work where: "**Add-ons** that use **built-in shaders** should use `POLYLINES_*` or `POINT_*` shader variants to support Metal and Vulkan" (https://developer.blender.org/docs/release_notes/4.5/). The requirements page names Windows, macOS and Linux for Blender 4.5. The tests below were run on macOS only. **Observed 2026-08-06:** the same tests were run again on a rented Linux machine. The engine identifiers, the unknown-name error and the add-on case all came back the same, so those hold on macOS and on Linux. One thing does **not** match: in Blender 4.5.12, `-E help` and an unknown engine name end abnormally on both machines but not in the same way — `libc++abi: terminating` and exit code 134 on the Mac, and a run that never ends at all on the one-card Linux machine. **Observed 2026-08-06 on a second rented Linux machine — two NVIDIA GeForce RTX 4090s, driver 580.95.05, sixty-one cores, no display server at all:** `-E help` printed the same three identifiers, `BLENDER_EEVEE_NEXT`, `BLENDER_WORKBENCH` and `CYCLES`, and then hung as well, killed at sixty seconds, exit code 124 — but unlike the one-card machine it printed its listing before hanging. So this version ends badly on all three machines and in three different ways, and none of the three is chosen between. Windows stays untested. *(Added by the two-card citation check, 2026-08-06: the third machine's result was in the hand-test record and under question 6 but not here, and the sentence read as though there were only two machines.)*

### Observed — hand tests 1 and 21 (2026-08-05)

Both were run from a terminal on the Creator's own computer — a MacBook Pro with an M1 Pro chip, macOS 26.5.2 — using Blender 4.5.12 at `/Applications/Blender 4.5.12.app/Contents/MacOS/Blender`, with the `BLENDER_USER_RESOURCES` setting pointed at fresh empty folders. Label: **observed**.

**Hand test 1 — what `-E help` prints.** `blender -E help` printed:

```
Blender Engine Listing:
	BLENDER_EEVEE_NEXT
	BLENDER_WORKBENCH
	CYCLES
```

so it lists identifiers, not the human labels, and Workbench's identifier is `BLENDER_WORKBENCH`. The process then ended with `libc++abi: terminating` and exit code 134; that happened on every attempt, with and without `--factory-startup`, while a plain `--version` run exited 0.

**Hand test 21 — a file whose engine is not registered.** A file was saved with the engine set to `CYCLES`. Read back with Cycles present it printed:

```
BEFORE engine= CYCLES has_multiple_engines= True
```

A throwaway settings folder was then prepared with the Cycles add-on switched off, which left these add-ons enabled:

```
['bl_pkg', 'io_anim_bvh', 'io_curve_svg', 'io_mesh_uv_layout', 'io_scene_fbx', 'io_scene_gltf2', 'pose_library']
```

Opening the same file with that settings folder printed:

```
Error: Engine 'CYCLES' not available for scene 'Scene' (an add-on may need to be installed or enabled)
AFTER engine= 'BLENDER_EEVEE_NEXT' has_multiple_engines= True engine choices= ['BLENDER_EEVEE_NEXT']
```

and `blender -E help` in that same state listed only `BLENDER_EEVEE_NEXT` and `BLENDER_WORKBENCH`. Rendering the file went ahead anyway, exit code 0, image written, ending:

```
Fra:1 Mem:40.28M (Peak 40.28M) | Time:00:00.65 | Rendering 50 / 64 samples
Fra:1 Mem:40.28M (Peak 40.28M) | Time:00:00.71 | Rendering 64 / 64 samples
Saved: '.../out/n_0001.png'
```

64 samples is EEVEE's own default, not the file's Cycles setting of 4. So: the engine list is what is registered in the running Blender, a missing engine is reported by that exact message, the scene's engine reads back as EEVEE, and the render goes ahead on EEVEE rather than stopping.

**What this does not settle.** No third-party engine add-on is installed, so what `--addons` does for `-E`, and what happens to a file naming a third-party engine, stay untested.

### Observed — hand test 23, run by the Creator (2026-08-05)

Run by the Creator in Blender 4.5's own window, on their own computer — a MacBook Pro with an M1 Pro chip, macOS 26.5.2 — on 2026-08-05. That settles macOS and leaves Windows and Linux untested. Label: **observed**.

The open **Render Engine** dropdown in Render Properties holds exactly three entries — **EEVEE**, **Workbench**, **Cycles** — with Cycles selected. So the control the file's engine is chosen from offers those three and nothing else on a Blender with no extra add-on installed, and it offers them under their human labels rather than the identifiers `-E` takes.

### Observed on the rented Linux machine — Linux hand tests 1, 2, 23 and 25 (2026-08-06)

Run over SSH from a terminal on the rented Linux machine, using Blender 4.5.12 at `/opt/blender/blender-4.5.12-linux-x64/blender`, with `BLENDER_USER_RESOURCES` pointed at `/root/smoozy/Settings/res-4.5`, a folder of this topic's own that already existed. That folder was checked before use and reported its own config directory and Blender's eight default add-ons and nothing else. The machine is a container sharing someone else's hardware, and up to four topics were being worked on it at once, so nothing below claims anything about how long a run took or how much memory was free. Label: **observed**.

These are a second machine's answers to the tests the Creator's Mac answered on 2026-08-05. The two are never merged: where they agree it says so, and where they differ it says that plainly. The full list is `Audits/Blender Research/Settings/Hand tests — Linux.md`.

**Linux hand test 1 — what `-E help` prints, and what `-E` does with a name no engine bears.** With the machine's display reachable, `blender -E help` printed:

```
Blender Engine Listing:
	BLENDER_EEVEE_NEXT
	BLENDER_WORKBENCH
	CYCLES
```

the same three identifiers the Mac's run printed. **But Blender 4.5.12 then never ended.** The run was killed by a 120-second cut-off, exit code 124. It was repeated with a sixty-second cut-off, in background mode with `-b`, twice each, and with `--factory-startup`, and every one of them hung the same way. Run with its output unbuffered so that nothing could be lost when it was killed, `blender -b -E help` wrote:

```
Blender 4.5.12 LTS (hash 84afd5f785f7 built 2026-07-21 01:31:22)
Blender Engine Listing:
	BLENDER_EEVEE_NEXT
	BLENDER_WORKBENCH
	CYCLES
```

and then hung.

With a name no engine bears:

```
blender -b test.blend -E NOT_A_REAL_ENGINE -o e_ -F PNG -f 1

Error: engine not found 'NOT_A_REAL_ENGINE'
```

and again it hung — exit code 124, twice, with no image written.

**How this compares with the Mac, plainly.** On the Creator's Mac these same two commands printed the same output and then ended abnormally with `libc++abi: terminating` and exit code 134. On this Linux machine they print the same output and then do not end at all. **The two machines differ in how the run ends, and agree that it ends badly.** In both cases the version check and ordinary renders are fine: on this machine `blender --version`, `blender -b test.blend -o ok_ -F PNG -f 1` and `blender -b test.blend -E BLENDER_WORKBENCH …` all exited 0 and wrote what they should. So it is these particular cases, not Blender 4.5 in general.

**What Blender's own data offers for the engine setting.** Asked for the choices on the engine property, a factory-startup Blender printed exactly one:

```
ENGINEITEM 'BLENDER_EEVEE_NEXT' 'EEVEE' ''
```

That is the same one-entry list the API page shows, and it is why the list of engines cannot be read that way. What is registered in the running Blender is what decides, and `-E help` is what prints it.

**Linux hand test 23 — a file whose engine is not registered.** A file was saved with the engine set to `CYCLES` at 64 by 64. With the Cycles add-on still on, a fresh Blender printed `BEFORE CYCLES multiple= True` and listed the eight default add-ons. The add-on was then turned off in a settings folder of its own, which reported the seven left. Reading the same file there printed:

```
BEFORE BLENDER_EEVEE_NEXT multiple= True
```

and rendering it printed:

```
Engine 'CYCLES' not available for scene 'Scene' (an add-on may need to be installed or enabled)
Saved: '…/out/f_0001.png'
```

exit code 0. So the file's engine name does not survive the add-on being off: it reads back as EEVEE's identifier, the render is reported and then goes ahead on EEVEE. **The two machines agree.**

**Linux hand test 25 — Blender's window on this machine.** A window opened in this version on the machine's own display and was photographed with Blender's own screenshot command, with the Render Engine set to Workbench and the Properties editor showing Render. The engine control is there and reads a human label rather than an identifier, as the Creator's photographs on the Mac showed. **The two machines agree.**

---

## Tests that could settle open points

Everything this list once held that a terminal could reach was run on the Creator's Mac on 2026-08-05 and written into the answers above, everything that needed eyes on Blender's window was run by the Creator the same day, and every terminal test was then run again on a rented Linux machine on 2026-08-06 and written in beside it. What is left is one gap in what was photographed, one test that needs a machine to itself, and what no machine the project has can reach.

### Still open — needs somebody looking at Blender's window

1. **What the right-click menu on a view layer's "Use for Rendering" offers.** The dot beside it, and that the switch takes a keyframe, are both settled; the menu itself was not among the shots supplied on the Mac, and driving a right-click on one control is not something the Linux run attempted. Bears on question 1.

### Needs a machine to itself

2. **Whether Cycles' Time Limit is per frame or per job on Linux.** Settled on the Mac by timing a three-frame render against a five-second limit. It could not be re-run on the rented machine, which was shared with three other topics throughout, because the answer can only be read from how long the run takes. The test is written out in `Audits/Blender Research/Settings/Hand tests — Linux.md`. Question 2.

### Cannot be run on any machine the project has

3. **Whether EEVEE fails on a headless Windows machine.** The screenless half of this item is now answered: on a second rented Linux machine with **no display server at all** — two NVIDIA GeForce RTX 4090s, driver 580.95.05, sixty-one cores, over SSH — every engine each version has rendered, on the cards and on the processor, EEVEE included, and that result is written into question 6. What is left is the Windows half, which needs a Windows machine, and a computer with no graphics hardware at all. (Question 6.)

4. **What a file's "GPU Compute" choice means on an AMD or an Intel graphics card.** The NVIDIA case is settled — see question 9 — and asking for `HIP` or `ONEAPI` on a machine with no such card is refused outright. What either does when the card is there needs such a card, and there is none. Question 9.

5. **Whether `--addons` enables an engine early enough for `-E` to see it, and what a file naming a missing third-party engine does.** Needs a third-party render engine add-on; none is installed on either machine, and installing one is not allowed on either. The nearest case — the Cycles add-on switched off in a throwaway settings folder — was run on both and is recorded under question 13. Question 13.

6. **Whether Freestyle's line-style scripts run without `--enable-autoexec`.** The sources do not give the exact settings needed to build a file with a scripted line style, so the test cannot be written out without guessing, and a guessed set-up would answer nothing. A second machine does not help this one. Question 11.

7. **Anything about Windows.** The Linux half of every question's operating-system line is now answered. Windows stays untried: no Windows machine has run it.

---

2026-08-04 — Opus 5 (Claude Code) — Settings, Blender 4.5

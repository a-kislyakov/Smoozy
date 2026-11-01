# Settings — Blender 5.2 — second pass (questions 1, 4, 9)

Sources consulted, all on **2026-07-30** (the snapshot date given for the copies on disk): the Blender 5.2 manual (`Blender Sources/manual_5.2`, including two of its pictures in `_images`), the Blender 5.2 Python API documentation (`Blender Sources/api_5.2`), the Blender 5.2 release notes (`Blender Sources/release_notes_5.2`), and Cycles' own files (`Blender Sources/cycles_5.2`). I also opened `Blender Sources/About this folder.md` and `Blender Sources/requirements/requirements_2026-08-01.txt`, both inside `Blender Sources/`; the requirements page does not name 5.2 in so many words, so I have not used it as evidence for any claim below.

I read only the question sheet and files inside Blender Sources/. I did not open any other part of this project.

One thing to declare, so the line above is not read as more than it is. I did not open `CLAUDE.md`, but the tool I work through placed its text in front of me automatically before I had any say in it. What it contains is a signpost telling agents to read `Preparation/Groundwork/Start Here.md`, and a paragraph telling Blender researchers to do the opposite. I did not follow the signpost and did not open `Start Here.md` or anything it points to. Nothing in what I saw made any claim about how Blender behaves, so nothing below can be an echo of it. I did not use any helper agents on this pass; every search and every reading above is my own, and there is no one else's work I would have to vouch for.

---

## 1. Which of a render's settings can be settled at the moment a render is started from outside Blender, and which can only be changed by changing the file? Cover at least a frame range, an interval to skip frames by, a list of separate frames, a sample count, whether a view layer renders at a given frame, a render engine, and a resolution percentage.

**A note on ambiguity.** "Whether a view layer renders at a given frame" reads two ways. Reading A: can the on/off switch that decides whether a view layer is rendered at all be set from outside? Reading B: can that switch differ from frame to frame? I have answered A as the main reading, because the question is a list of settings and asks which can be settled at start-up, and I cover B underneath it.

### The starting position: without a switch, the file decides everything

> "Render the whole animation using all the settings saved in the blend-file."

— Manual, https://docs.blender.org/manual/en/5.2/advanced/command_line/render.html — read 2026-07-30.

**Label: documented.** So the question reduces to: which settings have a switch on the command line?

### The complete list of switches that change what gets rendered

The manual's Command Line Arguments page carries the whole list. Quoting only the render-affecting ones, exactly:

> "-a, --render-anim
>
> Render frames from start to end (inclusive).
>
> -S, --scene <name>
>
> Set the active scene <name> for rendering.
>
> -f, --render-frame <frame>
>
> Render frame <frame> and save it.
>
> +<frame> start frame relative, -<frame> end frame relative.
>
> A comma separated list of frames can also be used (no spaces).
>
> A range of frames can be expressed using .. separator between the first and last frames (inclusive).
>
> -s, --frame-start <frame>
>
> Set start to frame <frame>, supports +/- for relative frames too.
>
> -e, --frame-end <frame>
>
> Set end to frame <frame>, supports +/- for relative frames too.
>
> -j, --frame-jump <frames>
>
> Set number of frames to step forward after each rendered frame.
>
> -o, --render-output <path>
>
> Set the render path and file name.
>
> -E, --engine <engine>
>
> Specify the render engine.
> Use -E help to list available engines.
>
> -t, --threads <threads>
>
> Use amount of <threads> for rendering and other operations
> [1-1024], 0 to use the systems processor count."

and, under a separate heading "Cycles Render Options", with the note "Cycles add-on options must be specified following a double dash.":

> "--cycles-device <device>
>
> Set the device used for rendering.
> Valid options are: CPU CUDA OPTIX HIP ONEAPI METAL.
>
> Append +CPU to a GPU device to render on both CPU and GPU."

and under "Format Options":

> "-F, --render-format <format>
>
> Set the render format.
> Valid options are:
> TGA RAWTGA JPEG IRIS PNG BMP HDR TIFF.
>
> Formats that can be compiled into Blender, not available on all systems:
> OPEN_EXR OPEN_EXR_MULTILAYER FFMPEG CINEON DPX JP2 WEBP.
>
> -x, --use-extension <bool>
>
> Set option to add the file extension to the end of the file."

— Manual, https://docs.blender.org/manual/en/5.2/advanced/command_line/arguments.html — read 2026-07-30.

**Label: documented.**

### The seven settings the question names

| Setting the question names | Settable at start from outside? | Which switch |
|---|---|---|
| A frame range | Yes | `-s` / `--frame-start` and `-e` / `--frame-end` |
| An interval to skip frames by | Yes | `-j` / `--frame-jump` |
| A list of separate frames | Yes | `-f` / `--render-frame`, "A comma separated list of frames can also be used (no spaces)" |
| A sample count | No switch exists | — |
| Whether a view layer renders (reading A) | No switch exists | — |
| A render engine | Yes | `-E` / `--engine` |
| A resolution percentage | No switch exists | — |

**Label for the four "yes" rows: documented** — each is quoted above.

**Label for the three "no" rows: not in the sources** (in the strong sense: I read the complete argument list and found no such switch, and searched for one everywhere else — record below).

The corresponding file settings, for the three with no switch:

- Sample count. Cycles: `samples`, "Number of samples to render for each pixel", `min=1, max=(1 << 24), default=4096` — Cycles' own files, `cycles_5.2/properties.py`, lines 485–490, inside `class CyclesRenderSettings(bpy.types.PropertyGroup)`. **Label: observed.** EEVEE: "taa_render_samples — Number of samples per pixel for rendering (in [1, inf], default 64)" — API, https://docs.blender.org/api/5.2/bpy.types.SceneEEVEE.html. **Label: documented.**
- View layer on/off. "use — Enable or disable rendering of this View Layer (default True)" — API, https://docs.blender.org/api/5.2/bpy.types.ViewLayer.html. **Label: documented.**
- Resolution percentage. "resolution_percentage — Percentage scale for render resolution (in [1, 32767], default 100)" — API, https://docs.blender.org/api/5.2/bpy.types.RenderSettings.html; described in the manual as "Percentage — Slider to reduce or increase the size of the rendered image relative to the Resolution values." — Manual, https://docs.blender.org/manual/en/5.2/render/output/properties/format.html. Both read 2026-07-30. **Label: documented.**

For completeness on the frame settings, the file's own versions are: "frame_start — First frame of the playback/rendering range (in [0, 1048574], default 1)", "frame_end — Final frame of the playback/rendering range (in [0, 1048574], default 250)", "frame_step — Number of frames to skip forward while rendering/playing back each frame (in [0, 1048574], default 1)" — API, https://docs.blender.org/api/5.2/bpy.types.Scene.html — read 2026-07-30. **Label: documented.**

### "Only by changing the file" is too strong — there is a documented third route

Blender's own Python is available at start-up from outside, from the same command line:

> "-P, --python <filepath>
>
> Run the given Python script file.
>
> --python-text <name>
>
> Run the given Python script text block.
>
> --python-expr <expression>
>
> Run the given expression as a Python script."

— Manual, https://docs.blender.org/manual/en/5.2/advanced/command_line/arguments.html — read 2026-07-30. **Label: documented.**

And the API documentation states the reach of that Python:

> "You can access Blender's data with the Python API in the same way as the animation system or user interface; this implies that any setting that can be changed via a button can also be changed with Python."

— API, https://docs.blender.org/api/5.2/info_quickstart.html — read 2026-07-30. **Label: documented.**

**Inferred** (the step, written out): the three settings with no dedicated switch — sample count, view layer on/off, resolution percentage — are settings changed via buttons in Blender's interface (each is a labelled control on a documented panel: Render ‣ Sampling, Properties ‣ View Layer ‣ View Layer, and the Output Format panel respectively). The quoted sentence says any setting changeable via a button is changeable with Python. `--python-expr` runs Python at start-up from outside. Therefore all three can be settled at the moment a render is started from outside, by expression rather than by switch, without changing the file. The reader has to supply the joining of those three quoted facts, so this is inferred, not documented. The sources nowhere show this being done for these particular settings.

### Two documented traps in how the switches are consumed

Order matters, and the file load wipes earlier settings:

> "Arguments are executed in the order they are given. eg:
>
> blender --background test.blend --render-frame 1 --render-output "/tmp"
>
> …will not render to /tmp because --render-frame 1 renders before the output path is set.
>
> blender --background --render-output /tmp test.blend --render-frame 1
>
> …will not render to /tmp because loading the blend-file overwrites the render output that was set."

— Manual, https://docs.blender.org/manual/en/5.2/advanced/command_line/arguments.html — read 2026-07-30. **Label: documented.**

The render.txt page states the same rule as a habit: "Always position -f or -a as the last arguments." — Manual, https://docs.blender.org/manual/en/5.2/advanced/command_line/render.html — read 2026-07-30. **Label: documented.**

### One setting that is documented to behave differently from outside

> "Render Single Layer
>
> Only render the active view layer.
>
> Note
>
> This option is ignored when rendering from the command line."

— Manual, https://docs.blender.org/manual/en/5.2/render/layers/view_layer.html — read 2026-07-30. The API says the same of the same property: "use_single_layer — Only render the active layer. Only affects rendering from the interface, ignored for rendering from command line. (default False)" — API, https://docs.blender.org/api/5.2/bpy.types.RenderSettings.html — read 2026-07-30. **Label: documented.**

So this one is neither settable from outside nor changeable by changing the file in any way that matters: from outside it has no effect at all.

### Reading B, briefly: can the view layer switch differ from frame to frame?

The manual's View Layer panel picture (`_images/render_layers_view-layer_panel.png`, shown on https://docs.blender.org/manual/en/5.2/render/layers/view_layer.html) shows the "Use for Rendering" checkbox with a small dot to its right. The manual describes such dots:

> "Decorators are small buttons that appear to the right of other buttons and show the state of the property. Decorators may appear next to number fields, menus, and checkboxes to indicate the property can be animated."
>
> "Clicking on the decorator dot icon will add a Keyframe to that property."

— Manual, https://docs.blender.org/manual/en/5.2/interface/controls/buttons/decorators.html — read 2026-07-30. **Label of the quotes: documented. Label of the conclusion: inferred** — the step is: the picture shows a dot beside "Use for Rendering"; the decorator page says such a dot indicates the property can be animated and that clicking it keyframes the property; therefore the view layer's on/off state can hold different values at different frames. I did not find any page that states this about `use` directly.

### Record of where I looked

- Read the whole of `manual_5.2/advanced/command_line/arguments.txt` (691 lines, every option) *[Citation check, 2026-08-05: the count was given as 692; the file is 691 lines]*, `render.txt` and `index.txt` in the same folder, and `extension_arguments.txt` (its sub-commands are package/repository management, nothing about rendering).
- Extracted every distinct long option (`--word`) appearing anywhere in the whole 5.2 manual, then in the whole API documentation and release notes. The manual's complete set contains `--render-anim`, `--render-frame`, `--render-format`, `--render-output`, `--frame-start`, `--frame-end`, `--frame-jump`, `--engine`, `--scene`, `--threads`, `--use-extension`, `--cycles-device`, `--cycles-print-stats` and nothing else touching a render's settings. There is no option containing "sample", "resolution", "percent", "scale", "layer" or "quality" in a render sense. The API documentation and release notes contribute only `--background`, `--colorspace`, `--command`, `--debug*`, `--factory-startup`, `--help`, `--log`, `--python`, `--python-use-system-env`, `--enable-`.
- Searched the command-line folder specifically for: sampl, resolution, percentage, scale, quality, "view layer", viewlayer, layer, noise, denoise, engine, device. The only hits were `--no-native-pixels` ("high resolution displays"), `--qos` (Quality of Service), `OPEN_EXR_MULTILAYER`, `--disable-depsgraph-on-file-load` (mentions ViewLayers' dependency graphs), and the `--cycles-device` block. None is a sample, view layer or resolution-percentage switch.
- Searched the whole of `Blender Sources` for `render-samples`, `--samples`, `-samples <` — no hits anywhere, including in Cycles' own files.
- Read Cycles' argument parser directly (`cycles_5.2/engine.py`, lines 8–41): it registers exactly two options, `--cycles-print-stats` and `--cycles-device`, and nothing else.
- Listed every manual and API page that mentions `--background` or `blender -b` (six pages) and read all six: `advanced/scripting/security.txt`, `advanced/command_line/render.txt`, `advanced/command_line/arguments.txt`, `files/asset_libraries/remote_asset_libraries.txt`, `api_5.2/info_tips_and_tricks.txt`, `api_5.2/info_advanced_blender_as_bpy.txt`. Only the first three concern render settings.
- Searched the 5.2 release notes for "command line", "command-line", "argument": the only render-related hit is the new `--command maketx` texture-cache tool in the Cycles notes, which generates texture files and does not set render settings.
- Ruled out, and why: the Freestyle, compositing, animation, modelling, physics, sculpt and video-editing parts of the manual — none of them documents command-line invocation (they produced no `--` option tokens in the sweep above).

**Not covered.** Whether a switch actually overwrites the file's value in memory as opposed to shadowing it — the manual says "Set start to frame <frame>" and nothing more precise, and I could not settle it. Nor whether `-j`/`--frame-jump` applies to `-f` as well as to `-a`. Nor what `-E` does when given the name of an engine the build does not have. Nor whether a `--python-expr` setting of samples or resolution percentage survives the blend-file load — by the documented ordering rule it would have to come after the file argument, but the sources do not say so for Python specifically. One further caution the sources raise and do not resolve: `-a` appears twice in the argument list, once as `--render-anim` under Render Options and once under Animation Playback Options as "Instead of showing Blender's user interface, this runs Blender as an animation player… (ignored if -b is set)"; the sources do not spell out how the two are told apart other than by `-b`.

**Outside the interface.** This whole answer is about the outside case, so it holds there by construction. Where it differs inside: `use_single_layer` ("Render Single Layer") works from the interface and is documented to be ignored from the command line — that is the one documented difference. `-b, --background` is itself described as "Run in background (often used for UI-less rendering)", and its own note says "The audio device is disabled in background-mode by default and can be re-enabled by passing in -setaudio Default afterwards." The sources do not say whether any of the seven named settings behaves differently in background mode.

**Operating system.** The switches for frames, engine, scene and threads carry no operating-system restriction in the source. Two things on the same page do: the format list says of `OPEN_EXR OPEN_EXR_MULTILAYER FFMPEG CINEON DPX JP2 WEBP` that these are "Formats that can be compiled into Blender, not available on all systems", and several unrelated options are marked "(Windows only)" (`-con`, `/?`, `--qos`), "(Linux only)" (`--no-window-frame`, `--debug-ghost`) or "(Windows & Linux only)" (`-r`, `--register-allusers`, `--unregister*`). `--cycles-device`'s valid values differ in practice by platform — see question 9. How you invoke Blender differs by platform (`./blender` on Linux, `blender` on Windows, `./Blender` inside `/Applications/Blender.app/Contents/MacOS` on macOS — https://docs.blender.org/manual/en/5.2/advanced/command_line/launch/index.html and its three sub-pages), but the arguments themselves are the same. **Observed 2026-08-05:** every test below was run on macOS only, so Windows and Linux stay untested. **Observed 2026-08-06:** the same tests were run again on a rented Linux machine (see the Linux block at the end of this answer). Every argument this answer names behaves the same way there, so the answer now holds on macOS and on Linux; Windows stays untested. The argument list itself is not identical on the two — Blender's own help text is longer on Linux.

### Observed — hand tests 2, 3, 4, 5 and 10 (2026-08-05)

All were run from a terminal on the Creator's own computer — a MacBook Pro with an M1 Pro chip, macOS 26.5.2, Apple's own graphics and no other graphics card — using Blender 5.2.0 at `/Applications/Blender.app/Contents/MacOS/Blender`, with the `BLENDER_USER_RESOURCES` setting pointed at a fresh empty folder so the Creator's own Blender settings were untouched. Label: **observed**.

**On the conditions of hand tests 2 and 3.** Those two were first run before it was known that Blender falls back to the machine's real settings when that folder does not exist, so the first attempts loaded the Creator's own add-ons. Those attempts were discarded. Both were re-run from scratch with the folder created first — hand test 3 with its files rebuilt as well — and each version was checked before use, reporting its own throwaway settings folder and Blender's eight default add-ons and nothing else. What is written below is the clean re-run.

**Hand test 5 — does `--python-expr` really settle the three switch-less settings at launch?** A file was saved with Cycles, 512 samples, two view layers (`ViewLayer` and `Layer2`) and a 200 by 100 resolution at 100 per cent. With the expression after the blend-file argument:

```
blender -b test.blend --python-expr "import bpy; s=bpy.data.scenes[0]; s.cycles.samples=8; s.render.resolution_percentage=25; s.view_layers['Layer2'].use=False; print('SET', s.cycles.samples, s.render.resolution_percentage, s.view_layers['Layer2'].use)" -o out-after/f_ -F PNG -f 1
```

printed `SET 8 25 False` and wrote an image of `pixelWidth: 50   pixelHeight: 25`. With the expression before the blend-file argument it printed `SET 8 25` and wrote `pixelWidth: 200   pixelHeight: 100`.

**This settles the inferred step above, and one of the "not covered" items with it.** All three settings can be settled at launch by Python, and the expression must come after the file argument — the load wipes anything set before it, exactly as the manual says of `--render-output`.

**Hand test 4 — do the switches overwrite the file, or only shadow it?** A file was saved with frame start 1, frame end 250, frame step 1, resolution percentage 100, engine `BLENDER_EEVEE`. After `blender -b test.blend -E CYCLES -s 5 -e 7 -j 2 -o out/f_ -F PNG -f 5`, the file's modification time and size were unchanged (`1785890071`, `95947 bytes`), and reading it back with no arguments printed:

```
READBACK 1 250 1 100 BLENDER_EEVEE
```

**This closes the first "not covered" item above:** the file on disk is untouched.

**Hand test 3 — does `-j` apply to `-f`, and does `-f` take a list and a range together?** A file was saved from factory startup with Workbench at 64 by 64. Five renders, each into its own empty folder, wrote:

```
blender -b test.blend -o out-list/f_  -F PNG -f 1,5,9        -> f_0001.png f_0005.png f_0009.png
blender -b test.blend -o out-range/f_ -F PNG -f 3..6         -> f_0003.png f_0004.png f_0005.png f_0006.png
blender -b test.blend -o out-mixed/f_ -F PNG -f 1,5..7,12    -> f_0001.png f_0005.png f_0006.png f_0007.png f_0012.png
blender -b test.blend -j 3 -o out-jumpf/f_ -F PNG -f 1..10   -> f_0001.png … f_0010.png (all ten)
blender -b test.blend -j 3 -s 1 -e 10 -o out-jumpa/f_ -F PNG -a -> f_0001.png f_0004.png f_0007.png f_0010.png
```

No error was printed by any of the five. **This closes the second "not covered" item:** `-j` is applied to `-a` and not to `-f`. A comma list and a `..` range may also be mixed in one `-f`.

**Hand test 2 — what `-E` does with a name the build does not have.** `blender -b test.blend -E NOT_A_REAL_ENGINE -o out/e_ -F PNG -f 1` printed `Error: engine not found 'NOT_A_REAL_ENGINE'`, exit code 1, and no image was written. **This closes the third "not covered" item.** `blender -E help` prints:

```
Blender Engine Listing:
	BLENDER_EEVEE
	BLENDER_WORKBENCH
	CYCLES
```

The same `blender --help` output, 518 lines in all, was searched for `sampl`, `resolution`, `percent` and `view layer` and carried no argument for any of them — which is the sweep above, done against the program rather than the manual.

**Hand test 10 — reading B, whether the view layer switch can differ per frame.** From factory startup, `keyframe_insert` on a view layer's `use` returned `True`. A file was saved with two view layers, `Layer2`'s `use` keyframed on at frame 1 and off at frame 2, Workbench, 64 by 64, then rendered with `blender -b test.blend -o out/f_ -F OPEN_EXR_MULTILAYER -s 1 -e 2 -a`. The progress lines named:

```
Rendering frame 1 -> Scene, ViewLayer  and  Scene, Layer2
Rendering frame 2 -> Scene, ViewLayer
```

and the channel names in the two written files were:

```
f_0001.exr : Layer2.Combined  Layer2.CombinedpixelAspectRatio  ViewLayer.Combined  ViewLayer.CombinedpixelAspectRatio   (8438 bytes)
f_0002.exr : ViewLayer.Combined  ViewLayer.CombinedpixelAspectRatio                                                     (4398 bytes)
```

**This settles reading B, which was inferred above.** The switch can hold different values at different frames, and an animation render driven from outside honours it.

And on the one setting documented to behave differently from outside: with `use_single_layer` turned on at launch, the run printed `SINGLE-LAYER ON, frame 1, layers on: ['ViewLayer', 'Layer2']` and the written file held both `Layer2.Combined` and `ViewLayer.Combined`. So it is indeed ignored from the command line.

### Observed — hand test 25, run by the Creator (2026-08-05)

Run by the Creator in Blender 5.2's own window, on their own computer — a MacBook Pro with an M1 Pro chip, macOS 26.5.2 — on 2026-08-05. That settles macOS and leaves Windows and Linux untested. Label: **observed**.

They opened the Properties editor's View Layer Properties tab, on a view layer named `LayerA`, and photographed it. What is visible: a **View Layer** panel holding **Use for Rendering**, ticked, with a small round dot at the far right of its row; and **Render Single Layer**, unticked, with no dot.

They also wrote, in their own words:

> "Pressing I at Use for Rendering makes the checkbox yellow and the dot next to it a diamond. Consistent across all versions. Render Single Layer is non-animatable in all versions, too. Here's the message: "use_single_layer" property cannot be animated. Also consistent in all versions."

**This settles the inferred step in Reading B above.** The dot in the manual's picture is a decorator in the sense of the decorators page, and the switch does take a keyframe — the interface inserts one and marks the control as animated. The same test settles the row below it by the opposite result: "Render Single Layer" is refused a keyframe outright, with the message quoted above. That adds something the sources nowhere state, and it sits alongside the documented divergence recorded above, where the same setting is ignored from the command line.

**What the screenshots do not cover.** The right-click menu on "Use for Rendering" was not among the shots supplied; the Creator answered by pressing I instead. What that menu lists is still unrecorded.

### Observed on the rented Linux machine — Linux hand tests 2, 3, 4, 5 and 12 (2026-08-06)

Run over SSH from a terminal on a rented Linux machine — Ubuntu 22.04.5 LTS, NVIDIA GeForce RTX 4090, driver 580.105.08, twelve cores, run over SSH, using Blender 5.2.0 at `/opt/blender/blender-5.2.0-linux-x64/blender`, with `BLENDER_USER_RESOURCES` pointed at `/root/smoozy/Settings/res-5.2`, a folder of this topic's own that already existed. That folder was checked before use and reported its own config directory and Blender's eight default add-ons and nothing else. The machine is a container sharing someone else's hardware, and up to four topics were being worked on it at once, so nothing below claims anything about how long a run took or how much memory was free. Label: **observed**.

These are a second machine's answers to the tests the Creator's Mac answered on 2026-08-05. The two are never merged: where they agree it says so, and where they differ it says that plainly. The full list is `Audits/Blender Research/Settings/Hand tests — Linux.md`.

**Linux hand test 2 — is there really no argument for a sample count, a resolution percentage or a view layer?** `blender --help` was written to a file and searched, with no blend-file opened. The whole help output is **525 lines** on this machine; the same command on the Creator's Mac gave 518, so the argument list is not the same length on the two operating systems. Searching it for `sampl`, `resolution`, `percent`, `view layer` and `python-use` returned only lines that set no render setting — the `--no-native-pixels` description, the Python-environment arguments, and the background-mode note about view layer dependency graphs. There is no `--samples` and no `--resolution` in this build either. Both `--python-use-system-env` and `--python-use-user-env` are present in this build, as they were on the Mac.

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

**One argument this document has not carried before.** Comparing this build's help text with Blender 5.1's, 5.2 alone holds `--gpu-device` and `--gpu-device-no-fallback`. In its own words:

```
--gpu-device <device>
	Select a specific GPU device, overriding the GPU device from user preferences for this
	run only. Accepted forms:

	* '<vendor-hex>/<device-hex>/<index-hex>' (matches the identifier used in user
	  preferences, with vendor PCI ID, device PCI ID and enumeration index all hex-encoded).
	* '<index>' (no slashes) picks the Nth supported device by enumeration order, decimal.
	* 'help' prints supported Vulkan devices and exits.

	Only used with the Vulkan backend. Other backends print a warning and ignore this option.
	If the requested Vulkan device is unavailable, Blender falls back to the saved GPU
	preference unless '--gpu-device-no-fallback' is also set.

--gpu-device-no-fallback 
	Fail instead of falling back when '--gpu-device' does not match a usable Vulkan device.
```

It is a launch argument about the graphics device, so it also bears on question 9. It was **not run**: the tests here use Cycles' own `--cycles-device`, and nothing in this topic asks about the Vulkan backend. Whether the Mac's 5.2 carries it too was not checked — the Mac run only searched its help text for five words.

---

## 4. Does a file carry a separate sample count for each render engine?

**Answer: yes, for the three engines that ship with Blender — each keeps its own, on the scene, and they are separate settings that do not stand in for one another. There is one sample-count-like setting that is shared across all three, and one per-view-layer override that both Cycles and EEVEE read but interpret differently.**

### The three separate scene-level counts

**Cycles.** From Cycles' own files, `cycles_5.2/properties.py`, lines 485–498, inside `class CyclesRenderSettings(bpy.types.PropertyGroup)`:

> ```
>     samples: IntProperty(
>         name="Samples",
>         description="Number of samples to render for each pixel",
>         min=1, max=(1 << 24),
>         default=4096,
>     )
>     preview_samples: IntProperty(
>         name="Viewport Samples",
>         description="Number of samples to render in the viewport, unlimited if 0",
>         min=0,
>         soft_min=1,
>         max=(1 << 24),
>         default=1024,
>     )
> ```

and the same file, lines 1140–1146, showing where that group is attached:

> ```
>     @classmethod
>     def register(cls):
>         bpy.types.Scene.cycles = PointerProperty(
>             name="Cycles Render Settings",
>             description="Cycles render settings",
>             type=cls,
>         )
> ```

**Label: observed** (Cycles' own files, read 2026-07-30).

**EEVEE.**

> "taa_render_samples
>
> Number of samples per pixel for rendering (in [1, inf], default 64)"

> "taa_samples
>
> Number of samples, unlimited if 0 (in [0, inf], default 16)"

— API, https://docs.blender.org/api/5.2/bpy.types.SceneEEVEE.html — read 2026-07-30. **Label: documented.**

**Workbench.**

> "render_aa
>
> Method of anti-aliasing when rendering final image (default '8')
>
> OFF — No Anti-Aliasing – Scene will be rendering without any anti-aliasing.
>
> FXAA — Single Pass Anti-Aliasing – Scene will be rendered using a single pass anti-aliasing method (FXAA).
>
> 5 — 5 Samples – Scene will be rendered using 5 anti-aliasing samples."

— API, https://docs.blender.org/api/5.2/bpy.types.SceneDisplay.html — read 2026-07-30. **Label: documented.**

Workbench's count is of a different shape from the other two: it is a choice from a fixed menu rather than a free number. The manual says so:

> "The number of samples are predefined so it uses the best distribution of the samples."
>
> "5, 8, 11, 16, 32"

— Manual, https://docs.blender.org/manual/en/5.2/render/workbench/sampling.html — read 2026-07-30. **Label: documented.**

### That these are carried by the file

The Workbench page says it outright for its own setting:

> "The setting for the 3D Viewport is a user preference to specify the anti-aliasing method that runs best on the used system. The setting for viewport rendering and final rendering is saved per scene."

— Manual, https://docs.blender.org/manual/en/5.2/render/workbench/sampling.html — read 2026-07-30. **Label: documented.**

For Cycles, `cycles_5.2/version_update.py` reads and rewrites the Cycles sample count of a scene according to the version the loaded data came from — lines 155–162 and 226–251, for example:

> ```
>             if version <= (3, 0, 25):
>                 cscene = scene.cycles
>
>                 # Default changes.
>                 if not cscene.is_property_set("samples"):
>                     cscene.samples = 128
>                 if not cscene.is_property_set("preview_samples"):
>                     cscene.preview_samples = 32
> ```

and, in the same block:

> ```
>                 if use_square_samples:
>                     cscene.samples *= cscene.samples
>                     cscene.preview_samples *= cscene.preview_samples
>                     for layer in scene.view_layers:
>                         layer.samples *= layer.samples
> ```

with the version taken from the loaded data, `cycles_5.2/version_update.py` lines 97–101:

> ```
>     library_versions = {}
>     library_versions[bpy.data.version] = [None]
>     for library in bpy.data.libraries:
>         library_versions.setdefault(library.version, []).append(library)
> ```

**Label: observed.** **Inferred**, one step: a sample count that has to be converted on load according to the version the data was written with is a value that came in with the loaded data — that is, it is carried by the file. The files do not use the word "saved" here; I supplied that step.

### That all three live on the scene at the same time

The Scene page lists all three carriers as attributes of the same data-block:

> "cycles — Cycles render settings (readonly) — Type: CyclesRenderSettings | None"
>
> "display — Scene display settings for 3D viewport (readonly) — Type: SceneDisplay | None"
>
> "eevee — EEVEE settings for the scene (readonly) — Type: SceneEEVEE | None"

— API, https://docs.blender.org/api/5.2/bpy.types.Scene.html — read 2026-07-30. **Label: documented.**

**Inferred**, one step: three attributes listed unconditionally on one Scene, each holding its own engine's sample count, means one scene holds three sample counts at once, whichever engine happens to be active. The page does not say "at once"; I supplied that.

The general principle is stated, though without naming sample counts:

> "More renderers from third-party developers are available as add-ons. Each renderer has its own render settings to control render quality and performance."

— Manual, https://docs.blender.org/manual/en/5.2/render/render_engines.html — read 2026-07-30. **Label: documented.** The same page adds: "Switching render engines may change available settings, materials, and rendering behavior."

### The one that is shared

The Grease Pencil sample count is not per engine. The identical panel is documented on all three engines' pages, word for word:

> "SSAA Samples
>
> Number of samples used for super-sampling anti-aliasing in the final render.
> Higher values produce smoother lines but increase render time."

— Manual, at all three of https://docs.blender.org/manual/en/5.2/render/cycles/render_settings/grease_pencil.html , https://docs.blender.org/manual/en/5.2/render/eevee/render_settings/grease_pencil.html and https://docs.blender.org/manual/en/5.2/render/workbench/grease_pencil.html — read 2026-07-30. Its single carrier is: "aa_samples — Number of supersampling anti-aliasing samples per pixel for final render (in [1, inf], default 8)" — API, https://docs.blender.org/api/5.2/bpy.types.SceneGpencil.html — read 2026-07-30. **Label: documented.**

### The per-view-layer override: one setting, two meanings

> "samples
>
> Override number of render samples for this view layer, 0 will use the scene setting (in [0, inf], default 0)"

— API, https://docs.blender.org/api/5.2/bpy.types.ViewLayer.html — read 2026-07-30.

> "Samples
>
> Allows the view layer to use a different number of samples than the main scene.
> This is controlled by the Layer Samples when using the Cycles renderer. When using EEVEE, the Samples override is always used."

— Manual, https://docs.blender.org/manual/en/5.2/render/layers/override.html — read 2026-07-30.

And the Cycles-side control of it:

> "Layer Samples
>
> If any view layers have Sample Overrides configured, this option specifies how to use them.
>
> Use: Allow view layers to override the scene-level sample count.
>
> Bounded: Allow the overrides as long as they don't exceed the scene-level sample count.
>
> Ignore: Ignore the overrides."

— Manual, https://docs.blender.org/manual/en/5.2/render/cycles/render_settings/sampling.html — read 2026-07-30. Its carrier, from Cycles' own files, `cycles_5.2/properties.py` lines 64–68:

> ```
> enum_use_layer_samples = (
>     ('USE', "Use", "Per render layer number of samples override scene samples"),
>     ('BOUNDED', "Bounded", "Bound per render layer number of samples by global samples"),
>     ('IGNORE', "Ignore", "Ignore per render layer number of samples"),
> )
> ```

**Labels: documented** for the manual and API quotes, **observed** for the Cycles file. So a file carries one view-layer override number, shared between Cycles and EEVEE, plus a Cycles-only rule for how to apply it.

### Record of where I looked

- Searched the whole API documentation for `taa_render_samples`, `render_aa` and `resolution_percentage` to find every page carrying a render sample count; the hits were `bpy.types.SceneEEVEE`, `bpy.types.SceneDisplay`, `bpy.types.RenderSettings`, `bpy.types.RenderEngine`, plus index and quickstart pages.
- Searched `bpy.types.RenderSettings` — the engine-neutral render settings — for "sampl": the only hits are anisotropic filtering ("Use 2 samples…"), the pixel filter width, and `use_motion_blur`'s "multi-sampled". There is no engine-neutral sample count. This is the search that most matters, since a shared count would have to live somewhere like this.
- Read the three engines' Sampling manual pages in full: Cycles (`render/cycles/render_settings/sampling.txt`, 322 lines), EEVEE (`render/eevee/render_settings/sampling.txt`), Workbench (`render/workbench/sampling.txt`).
- Read the three engines' Grease Pencil manual pages and compared them line by line — identical.
- Read `cycles_5.2/properties.py` around every occurrence of "samples" (26 hits), and its registration blocks, to establish which group each count belongs to and where the group attaches.
- Read `cycles_5.2/version_update.py` in full for what it does to sample counts on load.
- Searched the manual and API for "sample override", "samples override" and "Override number of render samples" — three hits, all quoted above.
- Searched manual and API for "saved in the blend-file", "stored in the blend-file", "saved in the file", "saved per scene", "per-scene": four hits in total across the whole manual, of which the Workbench sampling line is the only one about render settings. This is why the "carried by the file" part of this answer leans on Cycles' versioning code rather than on a plain statement — no such plain statement exists for Cycles or EEVEE.
- Searched the 5.2 release notes (all 21 files) for sampling changes: the Cycles notes cover the texture cache, Raycast attributes and shader nodes; the EEVEE notes contain no occurrence of "sampl". Nothing there changes this answer.
- Ruled out: `bpy.types.ViewLayerEEVEE` (read in full — it holds ambient occlusion distance and pass switches, no sample count).

**Not covered.** Whether a third-party engine added as an add-on necessarily keeps its own count — the manual says each renderer has its own render settings, which is weaker than saying each has its own sample count, and no third-party engine is in the sources. Whether the counts survive a round trip through a save and reload when the engine that owns them was never the active one — the sources do not address it. Whether the three counts mean comparable things: they do not obviously, since Workbench's is a fixed menu of anti-aliasing sample counts and Cycles' is paths traced per pixel, but the sources do not compare them. And whether a scene's `cycles` attribute being typed "CyclesRenderSettings | None" means the Cycles count is unreachable, or absent, when the Cycles add-on is switched off — the API page states the type and nothing about the cause.

**Outside the interface.** Nothing in any of the quoted material makes these counts conditional on the interface. The counts are scene data reached through `bpy.types.Scene`, and the API is the same whether Blender was started by a person or not. But that is my reasoning, not the source's: **the sources do not say**, and the one command-line note that bears on this is the opposite case — `use_single_layer`, which is documented to be ignored from the command line, is a view layer setting but not a sample count.

**Operating system.** The sources say nothing about any of these counts differing by operating system, and I found nothing on searching. The one platform-flavoured remark near this area concerns Workbench's viewport (not render) anti-aliasing: "The setting for the 3D Viewport is a user preference to specify the anti-aliasing method that runs best on the used system" — which is about the machine, not about the file's render setting. The test below was run on macOS only. **Observed 2026-08-06:** the same test was run again on a rented Linux machine and came back the same, value for value, so this holds on macOS and on Linux. Windows stays untested.

### Observed — hand test 6 (2026-08-05)

Run from a terminal on the Creator's own computer — a MacBook Pro with an M1 Pro chip, macOS 26.5.2, Apple's own graphics and no other graphics card — using Blender 5.2.0 at `/Applications/Blender.app/Contents/MacOS/Blender`, with the `BLENDER_USER_RESOURCES` setting pointed at a fresh empty folder. Label: **observed**.

A file was saved from factory startup with Cycles samples 123, EEVEE render samples 45, Workbench final anti-aliasing `32` and the Grease Pencil final-render sample count 3. Blender was closed and a fresh Blender opened the file, reading all four values back and switching the engine between readings:

```
MADE cycles=123 eevee=45 workbench=32 gpencil=3
BACK as-saved               engine=BLENDER_EEVEE        cycles=123 eevee=45 workbench=32 gpencil=3
BACK after switch to CYCLES engine=CYCLES               cycles=123 eevee=45 workbench=32 gpencil=3
BACK after switch to BLENDER_WORKBENCH engine=BLENDER_WORKBENCH    cycles=123 eevee=45 workbench=32 gpencil=3
BACK after switch to EEVEE  engine=BLENDER_EEVEE        cycles=123 eevee=45 workbench=32 gpencil=3
```

**This settles both inferred steps above.** The counts are carried by the file — they came back at the values saved, with no plain statement in the sources needed — and the three do live on one scene at once, whichever engine happens to be active. Switching the engine changed none of them. The shared Grease Pencil count read 3 under all three engines, which is the "one that is shared" section watched rather than read.

**What this does not settle.** No third-party engine was involved, so whether one keeps its own count is still open.

### Observed — hand test 26, run by the Creator (2026-08-05)

Run by the Creator in Blender 5.2's own window, on their own computer — a MacBook Pro with an M1 Pro chip, macOS 26.5.2 — on 2026-08-05. That settles macOS and leaves Windows and Linux untested. Label: **observed**.

With the Render Engine set to **EEVEE**, the Sampling panel shows a **Viewport ▸ Samples** of 16 and a separate **Render ▸ Samples** of 64 — the two the finding above names, side by side. With the Render Engine set to **Workbench**, the whole Sampling panel is **Render — 8 Samples** and **Viewport — Single Pass Anti-Aliasing**, both menus. Cycles' own panel shows **Max Samples 4096** as a number field, alongside Noise Threshold, Min Samples and Time Limit.

So each engine's pair of counts is its own control in its own panel, and Workbench's really is "a choice from a fixed menu rather than a free number", as the finding above puts it.

**This also covers the counts through the panels rather than through Python**, which the terminal test above could not.

### Observed on the rented Linux machine — Linux hand tests 6 and 14 (2026-08-06)

Run over SSH from a terminal on the rented Linux machine, using Blender 5.2.0 at `/opt/blender/blender-5.2.0-linux-x64/blender`, with `BLENDER_USER_RESOURCES` pointed at `/root/smoozy/Settings/res-5.2`, a folder of this topic's own that already existed. That folder was checked before use and reported its own config directory and Blender's eight default add-ons and nothing else. The machine is a container sharing someone else's hardware, and up to four topics were being worked on it at once, so nothing below claims anything about how long a run took or how much memory was free. Label: **observed**.

These are a second machine's answers to the tests the Creator's Mac answered on 2026-08-05. The two are never merged: where they agree it says so, and where they differ it says that plainly. The full list is `Audits/Blender Research/Settings/Hand tests — Linux.md`.

**Linux hand test 6 — does one file hold all three counts at once, and is the Grease Pencil count shared?** A file was saved with Cycles samples 123, EEVEE render samples 45, Workbench final anti-aliasing `32` and the Grease Pencil final-render count 3, closed, and read back in a fresh Blender; then the engine was switched three times and the same four values read after each switch, in one run:

```
BACK as-loaded engine= BLENDER_EEVEE cycles= 123 eevee= 45 workbench= 32 gpencil= 3
BACK after-switch-to-CYCLES engine= CYCLES cycles= 123 eevee= 45 workbench= 32 gpencil= 3
BACK after-switch-to-BLENDER_WORKBENCH engine= BLENDER_WORKBENCH cycles= 123 eevee= 45 workbench= 32 gpencil= 3
BACK after-switch-to-BLENDER_EEVEE engine= BLENDER_EEVEE cycles= 123 eevee= 45 workbench= 32 gpencil= 3
```

So one file carries three separate counts, they survive a save and a reload, switching the engine disturbs none of them, and the Grease Pencil count reads the same under every engine. **The two machines agree**, value for value.

**Linux hand test 14 — what each engine's own block actually holds.** The EEVEE block holds **53** properties on this build and the Workbench block holds **9** — the same two counts the Mac's run gave for this version. **The two machines agree.**

---

## 9. Does a file record whether it renders on the computer's graphics cards or on its processor?

**Answer: yes for Cycles, and partly. The file records a two-way choice — processor, or "GPU Compute" — per scene. It does not record which graphics card or which graphics technology; that is a user preference on the machine, outside the file. EEVEE and Workbench have no such setting in the file at all. Separately, the file does record processor-or-graphics-card choices for compositing and for denoising.**

### What the file holds, for Cycles

From Cycles' own files, `cycles_5.2/properties.py`, lines 30–34:

> ```
> enum_devices = (
>     ('CPU', "CPU", "Use CPU for rendering"),
>     ('GPU', "GPU Compute",
>         "Use GPU compute device for rendering, configured in the system tab in the user preferences"),
> )
> ```

and lines 387–393, the first property of `class CyclesRenderSettings(bpy.types.PropertyGroup)`:

> ```
>     device: EnumProperty(
>         name="Device",
>         description="Device to use for rendering",
>         items=enum_devices,
>         default='CPU',
>         update=update_render_passes,
>     )
> ```

That group is attached to the scene, `cycles_5.2/properties.py` lines 1140–1146:

> ```
>     @classmethod
>     def register(cls):
>         bpy.types.Scene.cycles = PointerProperty(
>             name="Cycles Render Settings",
>             description="Cycles render settings",
>             type=cls,
>         )
> ```

**Label: observed** (Cycles' own files, read 2026-07-30). Exactly two values, CPU and GPU, the default being CPU, held per scene.

The manual states the same, and states that it is per scene:

> "To enable GPU rendering:
>
> Select either CUDA, OptiX, HIP, oneAPI, or Metal in the Cycles Render Devices user preference.
>
> Enable the available devices for the chosen backend.
>
> Configure each scene to use the GPU Compute Device."

— Manual, https://docs.blender.org/manual/en/5.2/render/cycles/gpu_rendering.html — read 2026-07-30.

> "Device
>
> Specifies the compute device used for rendering in Cycles.
> The choice of device can significantly affect render performance, GPU Compute is often faster .
>
> CPU:
>
> Render using the system's central processing unit. This mode is compatible with all Cycles features.
> The number of CPU threads used can be changed in the Performance Properties.
>
> GPU Compute:
>
> Render using one or more graphics processing units (GPUs) through the compute backend
> and devices selected in the Cycles Render Devices.
> Depending on the backend, it may be possible to use both CPU and GPU devices simultaneously."

— Manual, https://docs.blender.org/manual/en/5.2/render/cycles/render_settings/index.html — read 2026-07-30. **Label: documented.**

### What the file does not hold: which card, and which technology

> "Cycles Render Devices
>
> Changes the Computing Device the Cycles render engine uses to render images.
> Cycles can use either the CPU or certain GPUs to render images, for more information see the GPU Rendering page.
>
> None:
>
> When set to None or when the only option is None: the CPU will be used as the computing device for Cycles."

— Manual, https://docs.blender.org/manual/en/5.2/editors/preferences/system.html — read 2026-07-30. **Label: documented.**

The picture on that page (`_images/editors_preferences_section_system.webp`, shown at the same address) shows the Preferences window with the System section selected and a "Cycles Render Devices" panel at the top offering three tabs, None (selected), CUDA and HIP. **Label: documented** (a manual picture). What is visible is that the backend choice sits in Preferences, not in a scene.

Preferences are a separate file on the machine:

> "./config/userpref.blend
>
> User preferences."

— Manual, https://docs.blender.org/manual/en/5.2/advanced/blender_directory_layout.html — read 2026-07-30, and "By default changes to preferences are saved on exit" — Manual, https://docs.blender.org/manual/en/5.2/editors/preferences/introduction.html — read 2026-07-30. **Label: documented.**

Cycles' own files make the two-part rule explicit, `cycles_5.2/ui.py`, lines 96–113:

> ```
> def get_device_type(context):
>     return context.preferences.addons[__package__].preferences.compute_device_type
>
>
> def backend_has_active_gpu(context):
>     return context.preferences.addons[__package__].preferences.has_active_device()
>
>
> def use_cpu(context):
>     cscene = context.scene.cycles
>
>     return (get_device_type(context) == 'NONE' or cscene.device == 'CPU' or not backend_has_active_gpu(context))
>
>
> def use_gpu(context):
>     cscene = context.scene.cycles
>
>     return (get_device_type(context) != 'NONE' and cscene.device == 'GPU' and backend_has_active_gpu(context))
> ```

and the same file marks where the device list is kept, `cycles_5.2/properties.py` lines 1614–1622:

> ```
> class CyclesDeviceSettings(bpy.types.PropertyGroup):
>     # Runtime properties
>     __slots__ = ("is_optimized")
>
>     # Properties saved in preferences
>     id: StringProperty(name="ID", description="Unique identifier of the device")
>     name: StringProperty(name="Name", description="Name of the device")
>     use: BoolProperty(name="Use", description="Use device for rendering", default=True)
>     type: EnumProperty(name="Type", items=enum_device_type, default='CUDA')
> ```

**Label: observed.** Note the boundary carefully: `use_cpu` and `use_gpu` are helper functions in Cycles' user-interface file, used to decide what to show and what to grey out. What they show is that Cycles composes the file's `device` with two things from the preferences. Claiming that a render therefore falls back to the processor when the file says GPU but the machine has no enabled device would be a step beyond what the file says — **that step would be inferred**, and I have not taken it, because these particular lines govern the interface rather than the render.

### The switch that beats both

> "--cycles-device <device>
>
> Set the device used for rendering.
> Valid options are: CPU CUDA OPTIX HIP ONEAPI METAL.
>
> Append +CPU to a GPU device to render on both CPU and GPU.
>
> Example:
>
> blender -b file.blend -f 20 -- --cycles-device OPTIX"

— Manual, https://docs.blender.org/manual/en/5.2/advanced/command_line/arguments.html — read 2026-07-30. **Label: documented.**

Cycles' own help text for that switch is more explicit about what it beats, `cycles_5.2/engine.py`, lines 16–20:

> ```
>     parser.add_argument("--cycles-device",
>                         help="Set the device to use for Cycles, overriding user preferences and the scene setting."
>                              "Valid options are 'CPU', 'CUDA', 'OPTIX', 'HIP', 'ONEAPI', or 'METAL'."
>                              "Additionally, you can append '+CPU' to any GPU type for hybrid rendering.",
>                         default=None)
> ```

**Label: observed.** "overriding user preferences and the scene setting" — so what the file records for Cycles is overridable from outside, and the switch takes the finer-grained technology name that the file cannot hold.

The same file shows when that parsing happens and that it needs the `--` separator, `cycles_5.2/engine.py` lines 24–41:

> ```
> def _parse_command_line():
>     import sys
>
>     argv = sys.argv
>     if "--" not in argv:
>         return
>
>     parser = _configure_argument_parser()
>     args, _ = parser.parse_known_args(argv[argv.index("--") + 1:])
> ```

**Label: observed.**

### EEVEE and Workbench: no such setting in the file

I read both engines' Performance pages in full. EEVEE's holds High Quality Normals, Anisotropic Filtering, Shadow Pool, Light Probes Volume Pool, viewport Pixel Size, and then Compositor and Denoise Nodes device settings — no engine device. Workbench's holds only High Quality Normals and Anisotropic Filtering. — Manual, https://docs.blender.org/manual/en/5.2/render/eevee/render_settings/performance.html and https://docs.blender.org/manual/en/5.2/render/workbench/performance.html — read 2026-07-30. **Label for "no engine device setting on those pages": documented** (I am reporting the contents of two reference pages that enumerate their panels). **Label for "the file records nothing about processor-or-graphics-card for EEVEE and Workbench": not in the sources** — see the record below for the searches behind it.

The nearest thing is a machine-wide preference, not a file setting: "Backend — Selects the graphics API used for drawing the interface and rendering display content." with values OpenGL, and "Changing the backend requires restarting Blender for the change to take effect." — Manual, https://docs.blender.org/manual/en/5.2/editors/preferences/system.html — read 2026-07-30. Its carrier is `gpu_backend`, an attribute of `bpy.types.PreferencesSystem` (API index, https://docs.blender.org/api/5.2/bpy.types.PreferencesSystem.html), and its command-line equivalent is "--gpu-backend — Force to use a specific GPU backend. Valid options: vulkan, metal, opengl." — Manual, https://docs.blender.org/manual/en/5.2/advanced/command_line/arguments.html. **Label: documented.** Being on `PreferencesSystem`, it is not scene data.

### Other processor-or-graphics-card choices the file does record

> "compositor_device
>
> Set how compositing is executed (default 'GPU')
>
> Type: Literal['CPU', 'GPU']"

> "compositor_denoise_device
>
> The device to use to process the denoise nodes in the compositor (default 'AUTO')
>
> AUTO — Auto – Use the same device used by the compositor to process the denoise node.
>
> CPU — CPU – Use the CPU to process the denoise node."

— API, https://docs.blender.org/api/5.2/bpy.types.RenderSettings.html — read 2026-07-30. `RenderSettings` is reached as "render — (readonly, never None) — Type: RenderSettings" on https://docs.blender.org/api/5.2/bpy.types.Scene.html, so these are scene settings and engine-neutral. **Label: documented.** The manual describes the same two controls on both the Cycles and the EEVEE Performance pages, under "Compositor ‣ Device" and "Denoise Nodes ‣ Denoising Device".

And Cycles' render denoising has its own, `cycles_5.2/properties.py` lines 438–442 and 479–483:

> ```
>     denoising_use_gpu: BoolProperty(
>         name="Denoise on GPU",
>         description="Perform denoising on GPU devices configured in the system tab in the user preferences. This is significantly faster than on CPU, but requires additional GPU memory. When large scenes need more GPU memory, this option can be disabled",
>         default=False,
>     )
> ```

> ```
>     preview_denoising_use_gpu: BoolProperty(
>         name="Denoise Preview on GPU",
>         description="Perform denoising on GPU devices configured in the system tab in the user preferences. This is significantly faster than on CPU, but requires additional GPU memory. When large scenes need more GPU memory, this option can be disabled",
>         default=True,
>     )
> ```

**Label: observed.** The manual flags the split: "See also — Denoising and Compositing have their own device selection." — Manual, https://docs.blender.org/manual/en/5.2/render/cycles/render_settings/index.html — read 2026-07-30. **Label: documented.**

### Record of where I looked

- Searched the whole 5.2 manual, API documentation and release notes for "GPU Compute": five hits, two of them the Cycles GPU-rendering and Render-Settings pages quoted above, two of them unrelated API pages about `gpu.compute` utilities.
- Searched the manual for "Cycles Render Devices": five hits, all read (`editors/preferences/system.txt`, `render/cycles/gpu_rendering.txt`, `render/cycles/render_settings/index.txt`, `render/cycles/index.txt`, `render/cycles/render_settings/sampling.txt`).
- Read `render/cycles/gpu_rendering.txt` in full (219 lines) and `editors/preferences/system.txt`.
- Opened the manual picture `_images/editors_preferences_section_system.webp` to see where the backend choice sits.
- Searched `bpy.types.RenderSettings`, `bpy.types.Scene`, `bpy.types.SceneEEVEE` and `bpy.types.SceneDisplay` for a property named `device` or ending `_device`. Only two hits, both on `RenderSettings`: `compositor_device` and `compositor_denoise_device`. Neither `SceneEEVEE` nor `SceneDisplay` has any. **This is the search that carries the "EEVEE and Workbench record nothing" claim** — those are the pages that would have to hold such a setting.
- Confirmed there is no `bpy.types.CyclesRenderSettings` page in the API documentation at all (Cycles is an add-on and its settings are not in the API docs), which is why everything Cycles-side here is quoted from Cycles' own files.
- Searched Cycles' own files for "device": 40-odd hits, all reviewed; read the surrounding blocks in `properties.py` (the scene property, the preferences property group, the device-list machinery), `engine.py` (the command-line switch) and `ui.py` (the composition rule).
- Searched the API documentation for `gpu_backend`: it belongs to `bpy.types.PreferencesSystem` only.
- Read the EEVEE, Workbench and Cycles Performance manual pages in full.
- Searched the manual and API for "userpref.blend" to establish where preferences live.
- Searched the 5.2 release notes for any device change: the Cycles notes cover the texture cache, Raycast attributes and shader nodes; the Rendering notes cover render output, colour management and shader nodes ("Saving render output can now be disabled"). Nothing about the device setting.
- Ruled out: `bpy.types.RenderEngine` (the add-on-author side of registering an engine) — it appeared in a `resolution_percentage` search but holds nothing about a stored device choice for a file.

**Not covered.** What happens at render time when the file says GPU and the machine has no enabled graphics device — the `use_gpu` helper suggests a composition rule, but it is interface code and I have not extended it to render behaviour. Whether the CPU/GPU choice differs between view layers or is scene-wide only (it sits on the scene's Cycles settings, so scene-wide, but the sources do not discuss a per-view-layer version). Whether a file saved with GPU selected on one machine opens with GPU selected on another that has no graphics device configured. What `--cycles-device` does when given a technology the machine does not have — `engine.py` shows `sys.exit(1)` when `_cycles.set_device_override` returns false, but what makes it return false is inside a part of Cycles I was not given. And whether "GPU Compute" ever means more than one card at once — the manual says "one or more graphics processing units (GPUs)" and "Depending on the backend, it may be possible to use both CPU and GPU devices simultaneously", but which devices is a preference, not a file setting.

**Outside the interface.** The scene's device setting is the same data whether Blender is driven by a person or not, and the documented switch `--cycles-device` exists precisely for the outside case, with an example given in background mode: "blender -b file.blend -f 20 -- --cycles-device OPTIX". Two differences do arise outside. First, the switch overrides both the preferences and the file — Cycles' own help text says so. Second, the switch is only read when a `--` appears in the arguments: `cycles_5.2/engine.py` returns immediately if `"--" not in argv`. **Label: observed** for the second. Beyond that the sources do not say whether the setting behaves differently in background mode.

**Operating system.** The value stored in the file — CPU or GPU — does not vary by operating system; it has exactly two values everywhere. What that value can mean does vary, and the manual is explicit: "CUDA is supported on Windows and Linux", "OptiX is supported on Windows and Linux", "HIP is supported on Windows and Linux", "oneAPI is a computation library that is supported on Windows and Linux", "Metal for GPU acceleration is supported on Apple computers with Apple Silicon. macOS 13.0 or newer is required to support all features." — Manual, https://docs.blender.org/manual/en/5.2/render/cycles/gpu_rendering.html — read 2026-07-30. **Label: documented.** So a file that says GPU is portable as a file, but which technology satisfies it is decided on each machine, and the available set differs by operating system. The `--cycles-device` switch's value list (`CPU CUDA OPTIX HIP ONEAPI METAL`) is the same everywhere in the documentation, but by the above only some of those values can be met on any one operating system — that last joining step is **inferred**. **Observed 2026-08-05:** on this machine the device list holds a CPU entry and a METAL entry and nothing else; CUDA, OptiX, HIP and oneAPI could not be tried at all, so the other half of that claim stays untested. **Observed 2026-08-06:** a rented Linux machine with an NVIDIA GeForce RTX 4090 settles most of what was left here. That machine offers `CUDA`, `OPTIX`, `HIP` and `ONEAPI` and not `METAL`; `--cycles-device CUDA` and `OPTIX` render on the card, and `HIP`, `ONEAPI` and `METAL` are all refused with the same message. A file asking for GPU Compute renders on the card under CUDA or OptiX and falls back to the processor under the others — with nothing printed at all, except in the `HIP` case, where one warning line reads `WARNING HIPEW initialization failed: Error opening HIP dynamic library`. What an AMD or an Intel card does stays untested, and so does Windows. *(Changed by the citation check of the Linux hand tests, 2026-08-06: this line said "silently" for every case, which the block below already contradicts for HIP in this version.)*

### Observed — hand tests 7 and 8 (2026-08-05)

Both were run from a terminal on the Creator's own computer — a MacBook Pro with an M1 Pro chip, macOS 26.5.2, Apple's own graphics and no other graphics card — using Blender 5.2.0 at `/Applications/Blender.app/Contents/MacOS/Blender`, with the `BLENDER_USER_RESOURCES` setting pointed at fresh empty folders so the Creator's own Blender settings were untouched. Label: **observed**.

**Hand test 7 — the file's own record.** A file was saved from factory startup with two scenes: `One` with the Cycles device set to `GPU` and the compositor device set to `GPU`, and `Two` left at its defaults. A fresh Blender opened it and printed:

```
SCENE One cycles_device= GPU compositor_device= GPU
SCENE Two cycles_device= CPU compositor_device= GPU
```

So the two-way choice survives a save and a reload and is held one scene at a time. (The compositor device on the new scene came back as `GPU`, which is this version's stated default; the same test in 4.5, 5.0 and 5.1 gave `CPU` for a new scene.)

**Hand test 8 — what the switch beats, and the `--` requirement.** A throwaway settings folder was prepared with Metal chosen; preparing it printed:

```
DEVICE: CPU Apple M1 Pro use= True
DEVICE: METAL Apple M1 Pro (GPU - 14 cores) use= True
SAVED compute_device_type= METAL
```

With a file whose scene device was `CPU`, `blender -b test.blend -o out/a_ -F PNG -f 1 -- --cycles-device METAL --cycles-print-stats` produced statistics carrying the line `Profiling information not available (only works with CPU rendering)`, while the same run without the override did not. So the switch does beat the scene setting, as Cycles' own help text says.

Without the double dash:

```
blender -b test.blend -o out/b_ -F PNG -f 1 --cycles-device METAL
unknown argument, loading as file: --cycles-device
00:00.675  reports          | ERROR Cannot read file ".../--cycles-device": No such file or directory
```

**This confirms the observed step above** — the switch is only read when a `--` appears, and Blender treats it as a file name otherwise.

With a name no device bears:

```
blender -b test.blend -o out/c_ -F PNG -f 1 -- --cycles-device NOT_A_DEVICE
00:00.284  cycles           | ERROR NOT_A_DEVICE is not a valid Cycles device.
Error: Not freed memory blocks: 1, total unfreed memory 0.000004 MB
```

and no image was written. **This closes one of the "not covered" items above** for the case of a name that is not a device at all; what happens with a real technology the machine lacks still cannot be tested here.

**A file asking for the graphics card on a machine with none enabled.** A second throwaway settings folder was prepared with the compute device type set to `NONE`, and the scene's device forced to `GPU` at launch. The whole terminal output named no device and carried no warning of any kind, and its statistics carried no `Profiling information not available` line:

```
00:00.486  render           | Fra: 1 | Mem: 1M | Sample 16/16
00:00.487  render           | Fra: 1 | Mem: 1M | Finished
00:00.554  render           | Saved: '.../out/dd2_0001.png'
00:00.554  render           | Time: 00:00.14 (Saving: 00:00.06)
```

**This closes the first "not covered" item above.** At render time, a file saying `GPU` on a machine with no enabled device renders on the processor and says nothing about it. That is the behaviour the `use_gpu` helper hinted at, watched rather than reasoned to.

**And on whether `--cycles-device` changes the file:** hand test 4 (under question 1) showed the launch arguments leave the file on disk untouched.

### Observed — hand test 27, run by the Creator (2026-08-05)

Run by the Creator in Blender 5.2's own window, on their own computer — a MacBook Pro with an M1 Pro chip, macOS 26.5.2 — on 2026-08-05. That settles macOS and leaves Windows and Linux untested. Label: **observed**.

With the Render Engine set to **Workbench**, they expanded every panel of the Render Properties tab and photographed the whole of it: Curves, Simplify, Performance, Sampling, Lighting, Object Color, Options, Film, Grease Pencil, Freestyle, Color Management.

Three controls in that tab name a processor or a graphics card, and all three sit in the Performance panel:

```
Performance
  High Quality Normals        unticked
  Anisotropic Filtering       2×
  Compositor
    Device                    CPU | GPU        — GPU selected
    Precision                 Auto
  Denoise Nodes
    Denoising Device          Auto | CPU | GPU — Auto selected
    Preview Quality           Balanced
    Final Quality             High
```

Those are the compositor's and the denoise nodes' devices, the two engine-neutral entries recorded above, shown here because the tab carries them whatever the engine. **No control anywhere in the tab is a Workbench device setting** — which is the "EEVEE and Workbench: no such setting in the file" reading above, seen in the panel rather than searched for in the property list. The only other control naming a device is Color Management ▸ Advanced ▸ **Display Emulation**, reading `Automatic`, which is about colour.

The Compositor Device showing **GPU** on a factory-startup file matches what hand test 7 read back from a new scene in this version, and differs from Blender 4.5, 5.0 and 5.1, where the same panel shows **CPU**.

### Observed on the rented Linux machine — Linux hand tests 7, 8, 9, 10 and 25 (2026-08-06)

Run over SSH from a terminal on the rented Linux machine, using Blender 5.2.0 at `/opt/blender/blender-5.2.0-linux-x64/blender`, with `BLENDER_USER_RESOURCES` pointed at `/root/smoozy/Settings/res-5.2`, a folder of this topic's own that already existed. That folder was checked before use and reported its own config directory and Blender's eight default add-ons and nothing else. The machine is a container sharing someone else's hardware, and up to four topics were being worked on it at once, so nothing below claims anything about how long a run took or how much memory was free. Label: **observed**.

These are a second machine's answers to the tests the Creator's Mac answered on 2026-08-05. The two are never merged: where they agree it says so, and where they differ it says that plainly. The full list is `Audits/Blender Research/Settings/Hand tests — Linux.md`.

This is the question the new machine was most needed for. The Creator's Mac has Apple's own graphics and could reach only `METAL` and `CPU`; this one has an NVIDIA GeForce RTX 4090.

**Linux hand test 7 — does the choice survive a save, is it per scene, and what does a brand-new scene start at?** A brand-new factory scene, read without saving anything:

```
FACTORY engine= BLENDER_EEVEE cycles_device= CPU compositor_device= GPU
```

Then a file saved with two scenes, `One` with the Cycles device set to `GPU` and the compositor device set to `GPU`, `Two` left at its defaults, opened again in a fresh Blender:

```
SCENE One cycles_device= GPU compositor_device= GPU
SCENE Two cycles_device= CPU compositor_device= GPU
```

So the file carries the choice and holds it one scene at a time. **The two machines agree**, including on the compositor device a new scene starts at, which reads `GPU` here as it did on the Mac.

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

**Linux hand test 25 — Workbench's own Render Properties tab, photographed on this machine.** Blender was started with a real window on the machine's display, the engine set to Workbench, the Properties editor pointed at Render, and the window photographed with Blender's own screenshot command. Blender 5.2's tab lists Curves, Simplify, Performance, Sampling, Lighting and Object Color from the top. No control in the tab is a Workbench device setting, matching the Creator's photographs of the same tab on the Mac. **The two machines agree.**

---

## Things a person with Blender 5.2 open could settle by hand in a few minutes

Everything on this list that a terminal could reach was run on the Creator's Mac on 2026-08-05 and written into the answers above, what needed eyes on Blender's window was run by the Creator the same day, and every terminal test was run again on a rented Linux machine on 2026-08-06. Items 1 to 5, 9 and 10 of the original list are answered from a terminal; items 6, 7 and 8 in part, with the panel side of 6 now answered too. What is left:

1. **What the right-click menu on "Use for Rendering" offers.** The dot beside it, and that the switch takes a keyframe, are both settled; the menu itself was not among the shots supplied on the Mac, and driving a right-click on one control is not something the Linux run attempted. (Question 1.)

2. **Whether the Cycles Device control is greyed out when the preferences hold no device.** What a render does in that state is settled on both machines — it runs on the processor and says nothing — but what the control looks like is a separate question. On the Mac, answering it would mean changing the Creator's real Cycles device preference; on the rented machine it would mean driving the interface to that panel, which the Linux run did not attempt. (Question 9.)

3. **What `--cycles-device OPTIX` does.** *Answered on 2026-08-06* on a rented Linux machine with an NVIDIA GeForce RTX 4090: it renders on the card, as `CUDA` does. `HIP`, `ONEAPI` and `METAL` are all refused there with the same message. Recorded under question 9.

---

2026-08-04 — Opus 5 (Claude Code) — Settings, Blender 5.2

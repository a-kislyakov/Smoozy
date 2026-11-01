# Settings — Blender 5.2

**Sources consulted, and when.** Blender 5.2 manual (`Blender Sources/manual_5.2`, including pictures in its `_images` folder), Blender 5.2 Python API documentation (`Blender Sources/api_5.2`), Blender 5.2 release notes (`Blender Sources/release_notes_5.2`), and Cycles' own files as shipped in Blender 5.2 (`Blender Sources/cycles_5.2`). All four were taken on 2026-07-30, and that is the date read for everything below. I also opened `Blender Sources/About this folder.md` and `Blender Sources/requirements/requirements_2026-08-01.txt`; the requirements page does not name 5.2 anywhere, so I have not used it as a source for any answer.

I read only the question sheet and files inside Blender Sources/. I did not open any other part of this project.

I did no work through helper agents. Everything below I found myself.

---

## 1. Which of a render's settings can be settled at the moment a render is started from outside Blender, and which can only be changed by changing the file?

**Answer.** Blender's command line has a fixed list of arguments. Of the seven things named in the question, three have their own argument, one has an argument in a limited form, and three do not.

Settable at launch, with a dedicated argument:

- **Frame range** — yes.
  > `-s, --frame-start <frame>` / `Set start to frame <frame>, supports +/- for relative frames too.`
  > `-e, --frame-end <frame>` / `Set end to frame <frame>, supports +/- for relative frames too.`
- **An interval to skip frames by** — yes.
  > `-j, --frame-jump <frames>` / `Set number of frames to step forward after each rendered frame.`
- **A list of separate frames** — yes, as part of the "render this frame" argument.
  > `-f, --render-frame <frame>` / `Render frame <frame> and save it.` … `A comma separated list of frames can also be used (no spaces).` `A range of frames can be expressed using .. separator between the first and last frames (inclusive).`
- **A render engine** — yes.
  > `-E, --engine <engine>` / `Specify the render engine.` `Use -E help to list available engines.`

Not settable by any dedicated argument — **a sample count**, **whether a view layer renders**, and **a resolution percentage**. None of these words appears anywhere in the command-line argument list.

Also settable at launch, though not asked about: the scene (`-S, --scene <name>`), the output path (`-o`), the image format (`-F`), whether the extension is appended (`-x`), the thread count (`-t`), and, for Cycles only, the render device:
> `--cycles-device <device>` / `Set the device used for rendering.` `Valid options are: CPU CUDA OPTIX HIP ONEAPI METAL.` `Append +CPU to a GPU device to render on both CPU and GPU.`

There is also a general Python command-line route:
> `--python-expr <expression>` / `Run the given expression as a Python script.` `The expression may be a complete multi-line script; you are limited only by the platform's maximum argument length.`

and `-P, --python <filepath>` / `Run the given Python script file.` The cited API entries identify properties for frame range and step (`Scene.frame_start` / `frame_end` / `frame_step`), resolution percentage (`RenderSettings.resolution_percentage`), engine (`RenderSettings.engine`), and whether a view layer renders (`ViewLayer.use`). The quoted command entries do not themselves establish which settings a script can change during a run, whether this includes a sample count, or whether a change is left in the file.

One thing that bites: the arguments are acted on in order, and loading the file resets what was set before it.
> `Arguments are executed in the order they are given.`
> `blender --background --render-output /tmp test.blend --render-frame 1` … `will not render to /tmp because loading the blend-file overwrites the render output that was set.`
> `blender --background test.blend --render-output /tmp --render-frame 1` … `works as expected.`

**Label.** documented, for every argument that exists and for the ordering rule. **inferred**, for the three that do not exist. The step I took: the page is titled "Command Line Arguments" and is the page Blender's own `blender --help` prints; it lists arguments under headed groups (Render Options, Cycles Render Options, Format Options, …); none of those groups contains an argument naming samples, a view layer, or resolution. I read the whole page and searched the command-line folder for `--samples`, `--resolution`, `resolution-percentage` and `--view-layer` with no hits. From "not present on the complete list" I concluded "no such argument exists". A reader has to accept that the list is complete, so this is inferred, not documented.

**Question was ambiguous.** "Whether a view layer renders **at a given frame**" can be read two ways: (a) whether a named view layer takes part in the render at all, or (b) whether it can be turned on and off frame by frame. I answered (a), because that is the setting that exists — `ViewLayer.use`, "Enable or disable rendering of this View Layer (default True)". Reading (b) is not answered here. The sources recorded here do not settle it.

**Not covered.** Whether an argument given after the file *overrides* what the file holds, as opposed to setting it for the run only, is never stated in those words — the ordering note is the closest the sources come. Whether the file on disk is left unchanged by these arguments is also never stated. The recorded sources also do not establish whether a Python command-line script can set the sample count for that run. Nothing here says what happens if `-f` names frames outside the file's own range.

**Sources.**
- Manual, https://docs.blender.org/manual/en/5.2/advanced/command_line/arguments.html — read 2026-07-30
- Manual, https://docs.blender.org/manual/en/5.2/advanced/command_line/render.html — read 2026-07-30
- API documentation, https://docs.blender.org/api/5.2/bpy.types.Scene.html ("frame_start / First frame of the playback/rendering range (in [0, 1048574], default 1)"; "frame_end / Final frame of the playback/rendering range (in [0, 1048574], default 250)"; "frame_step / Number of frames to skip forward while rendering/playing back each frame (in [0, 1048574], default 1)") — read 2026-07-30
- API documentation, https://docs.blender.org/api/5.2/bpy.types.RenderSettings.html ("resolution_percentage / Percentage scale for render resolution (in [1, 32767], default 100)"; "engine / Engine to use for rendering (default 'BLENDER_EEVEE')") — read 2026-07-30
- API documentation, https://docs.blender.org/api/5.2/bpy.types.ViewLayer.html ("use / Enable or disable rendering of this View Layer (default True)") — read 2026-07-30

**Outside the interface.** This whole answer *is* the outside-the-interface case; the arguments only exist there. The manual pairs them with background mode: `-b, --background` / `Run in background (often used for UI-less rendering).` The one caveat the sources give is that `-a` means something different when `-b` is not set — under Animation Playback Options, "`-a <options> <file(s)>` / Instead of showing Blender's user interface, this runs Blender as an animation player … (ignored if -b is set)".

**Operating system.** The list holds on every operating system except where an argument says otherwise. The ones that name an operating system are unrelated to render settings: `-con, --start-console` "(Windows only)", `--no-window-frame` "(Linux only)", `/?` "(Windows only)", `-r, --register` "(Windows & Linux only)", `--qos <level>` "(Windows only)", `--debug-ghost` "(Linux only)". The format argument does vary by build: "Formats that can be compiled into Blender, not available on all systems: OPEN_EXR OPEN_EXR_MULTILAYER FFMPEG CINEON DPX JP2 WEBP." The Cycles device argument lists backends that the manual elsewhere ties to particular systems (see question 9). **Observed 2026-08-05:** every test below was run on macOS only, so Windows and Linux stay untested. **Observed 2026-08-06:** the same tests were run again on a rented Linux machine (see the Linux block at the end of this answer). Every argument this answer names behaves the same way there, so the answer now holds on macOS and on Linux; Windows stays untested. The argument list itself is not identical on the two — Blender's own help text is longer on Linux.

### Observed — hand tests 1, 2, 3, 4, 5 and 10 (2026-08-05)

All were run from a terminal on the Creator's own computer — a MacBook Pro with an M1 Pro chip, macOS 26.5.2, Apple's own graphics and no other graphics card — using Blender 5.2.0 at `/Applications/Blender.app/Contents/MacOS/Blender`, with the `BLENDER_USER_RESOURCES` setting pointed at a fresh empty folder so the Creator's own Blender settings were untouched. Label: **observed**.

**On the conditions of hand tests 1, 2 and 3.** Those three were first run before it was known that Blender falls back to the machine's real settings when that folder does not exist, so the first attempts loaded the Creator's own add-ons. Those attempts were discarded. All three were re-run from scratch with the folder created first — hand test 3 with its files rebuilt as well — and each version was checked before use, reporting its own throwaway settings folder and Blender's eight default add-ons and nothing else. What is written below is the clean re-run.

**Hand test 2 — is the list of arguments really complete?** `blender --help` was written to a file and searched, without opening any blend-file. The whole output is 518 lines. Searching it for `sampl`, `resolution`, `percent`, `view layer` and `python-use` returned:

```
128:	Do not use native pixel size, for high resolution displays (MacBook 'Retina').
164:--python-use-system-env 
166:	This also enables user environment, see: '--python-use-user-env'.
168:--python-use-user-env 
463:	Background mode: Do not systematically build and evaluate ViewLayers' dependency graphs
```

None sets a render setting. **This turns the inferred step above into something watched:** the program's own list, not just the manual page, carries no argument for a sample count, a resolution percentage or a view layer.

**Hand test 3 — a list of frames, a range of frames, and the frame jump.** A file was saved from factory startup with the Workbench engine and a 64 by 64 resolution. Five renders, each into its own empty folder, wrote:

```
blender -b test.blend -o out-list/f_  -F PNG -f 1,5,9        -> f_0001.png f_0005.png f_0009.png
blender -b test.blend -o out-range/f_ -F PNG -f 3..6         -> f_0003.png f_0004.png f_0005.png f_0006.png
blender -b test.blend -o out-mixed/f_ -F PNG -f 1,5..7,12    -> f_0001.png f_0005.png f_0006.png f_0007.png f_0012.png
blender -b test.blend -j 3 -o out-jumpf/f_ -F PNG -f 1..10   -> f_0001.png … f_0010.png (all ten)
blender -b test.blend -j 3 -s 1 -e 10 -o out-jumpa/f_ -F PNG -a -> f_0001.png f_0004.png f_0007.png f_0010.png
```

No error was printed by any of the five. So `-f` takes a comma list, a `..` range, and the two mixed in one argument; and `-j` is applied to `-a` and not to `-f`.

**Hand test 4 — does an argument leave the file alone?** A file was saved with frame start 1, frame end 250, frame step 1, resolution percentage 100 and the engine left at `BLENDER_EEVEE`. Then:

```
blender -b test.blend -E CYCLES -s 5 -e 7 -j 2 -o out/f_ -F PNG -f 5
```

The file's modification time and size were unchanged (`1785890071`, `95947 bytes`, before and after). Reading it back with no arguments printed:

```
READBACK 1 250 1 100 BLENDER_EEVEE
```

**This closes the "not covered" item above:** the arguments apply to that run only and the file on disk is left unchanged.

**Hand test 5 — can a Python script set the settings that have no argument?** A file was saved with Cycles, 512 samples, two view layers (`ViewLayer` and `Layer2`) and a 200 by 100 resolution at 100 per cent. With the expression after the blend-file argument:

```
blender -b test.blend --python-expr "import bpy; s=bpy.data.scenes[0]; s.cycles.samples=8; s.render.resolution_percentage=25; s.view_layers['Layer2'].use=False; print('SET', s.cycles.samples, s.render.resolution_percentage, s.view_layers['Layer2'].use)" -o out-after/f_ -F PNG -f 1
```

printed `SET 8 25 False` and wrote an image of `pixelWidth: 50   pixelHeight: 25`. With the expression before the blend-file argument it printed `SET 8 25` and wrote `pixelWidth: 200   pixelHeight: 100`.

**This closes the other "not covered" item:** a command-line Python script can set the sample count and the resolution percentage for one run, and only when it comes after the file argument.

**Hand test 10 — the second reading of the view-layer question.** From factory startup, `keyframe_insert` on a view layer's `use` returned `True`. A file was saved with two view layers, `Layer2`'s `use` keyframed on at frame 1 and off at frame 2, Workbench, 64 by 64, then rendered with `blender -b test.blend -o out/f_ -F OPEN_EXR_MULTILAYER -s 1 -e 2 -a`. The progress lines named:

```
Rendering frame 1 -> Scene, ViewLayer  and  Scene, Layer2
Rendering frame 2 -> Scene, ViewLayer
```

and the channel names in the two written files were:

```
f_0001.exr : Layer2.Combined  Layer2.CombinedpixelAspectRatio  ViewLayer.Combined  ViewLayer.CombinedpixelAspectRatio   (8438 bytes)
f_0002.exr : ViewLayer.Combined  ViewLayer.CombinedpixelAspectRatio                                                     (4398 bytes)
```

**This answers reading (b), which this finding left open.** Whether a view layer renders can be turned on and off frame by frame, and an animation render driven from outside honours it.

Separately, with `use_single_layer` turned on at launch, the run printed `SINGLE-LAYER ON, frame 1, layers on: ['ViewLayer', 'Layer2']` and the written file held both `Layer2.Combined` and `ViewLayer.Combined` — so that setting is indeed ignored from the command line.

**Hand test 25, run by the Creator (2026-08-05).** Run by the Creator in Blender 5.2's own window, on their own computer — a MacBook Pro with an M1 Pro chip, macOS 26.5.2. That settles macOS and leaves Windows and Linux untested.

They opened the Properties editor's View Layer Properties tab, on a view layer named `LayerA`, and photographed it. What is visible: a **View Layer** panel holding **Use for Rendering**, ticked, with a small round dot at the far right of its row; and **Render Single Layer**, unticked, with no dot.

They also wrote, in their own words:

> "Pressing I at Use for Rendering makes the checkbox yellow and the dot next to it a diamond. Consistent across all versions. Render Single Layer is non-animatable in all versions, too. Here's the message: "use_single_layer" property cannot be animated. Also consistent in all versions."

**This answers reading (b) at the control as well as at the render.** The switch takes a keyframe and the interface marks it as animated. It also settles the row below it, which the sources nowhere address: "Render Single Layer" is refused a keyframe outright, with the message quoted above.

**What the screenshots do not cover.** The entry also asked what the right-click menu on "Use for Rendering" offers. No shot of that menu was among those supplied — the Creator answered by pressing I instead — so what that menu lists is still unrecorded.

**Hand test 1 — the engine argument.** `blender -E help` printed:

```
Blender Engine Listing:
	BLENDER_EEVEE
	BLENDER_WORKBENCH
	CYCLES
```

exit code 0. With a name no engine bears, `blender -b test.blend -E NOT_A_REAL_ENGINE -o out/e_ -F PNG -f 1` printed `Error: engine not found 'NOT_A_REAL_ENGINE'`, exit code 1, and no image was written.

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

## 2. Can a render be set to stop once the picture is clean enough, and can it be set to stop after a length of time rather than after a count?

**Answer.** Both, in Cycles. Neither, as far as the sources show, in EEVEE or Workbench.

**Clean enough.** Cycles has adaptive sampling, driven by a noise threshold.
> `If the Noise Threshold checkbox is enabled, Cycles will use adaptive sampling, cutting short the sampling process in areas that have become less noisy than the specified threshold value – in other words, not wasting time further refining areas that already look good enough. For example, hair on a character may need many samples, but the background may only need few.`
> `Noise Threshold` / `The threshold to decide whether to continue sampling a pixel. Typical values are in the range from 0.1 to 0.001, with lower values meaning better quality but longer render times. Setting this to 0 makes Cycles guess an automatic value based on the total sample count.`
> `Min Samples` / `The minimum number of samples a pixel receives before adaptive sampling is applied. When set to 0 (default), Cycles automatically picks a value determined by the Noise Threshold.`

Cycles' own file confirms the settings, their kind, defaults and ranges:
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

Note what this does and does not say. It is stated as a *per-pixel* decision — "whether to continue sampling a pixel", "reduce the number of samples per pixel". Nothing in the sources says the render as a whole finishes early once every pixel has converged.

**A length of time.** Cycles has a Time Limit.
> `Time Limit` / `Stops rendering if the time exceeds the limit, even if the desired sample count hasn't been reached yet. Setting this value to zero disables the limit.`
> `The time limit does not include pre-render processing time, only render time.`

In Cycles' own file:
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

**Other engines.** I searched the whole manual for "time limit" and "adaptive sampling" and "noise threshold": every hit is on Cycles pages, plus one on a Cycles-only render pass ("Used to analyze adaptive sampling") and one on the version history page. I searched `time_limit` across the whole API documentation: the only hits are on an unrelated operator, `bpy.ops.wm.redraw_timer`. I read the EEVEE sampling page and the whole `SceneEEVEE` property list and the Workbench sampling page; neither engine offers a noise threshold or a time limit. Label for the other engines: **not in the sources**.

**Label.** documented (manual, both Cycles features); observed (Cycles' own file, for the exact names, kinds, defaults and ranges); not in the sources (for EEVEE and Workbench).

**Not covered.** Whether reaching the Time Limit still writes the partly-rendered frame to disk. Whether adaptive sampling can end the whole render before the sample count is reached. What either does when both are set at once. Whether the time limit is per frame or for a whole animation.

**Sources.**
- Manual, https://docs.blender.org/manual/en/5.2/render/cycles/render_settings/sampling.html — read 2026-07-30
- Manual, https://docs.blender.org/manual/en/5.2/render/eevee/render_settings/sampling.html — read 2026-07-30
- Manual, https://docs.blender.org/manual/en/5.2/render/workbench/sampling.html — read 2026-07-30
- Cycles' own file, `Blender Sources/cycles_5.2/properties.py`, in class `CyclesRenderSettings`: `time_limit` at line 520, `use_adaptive_sampling` at line 572, `adaptive_threshold` at line 577 — read 2026-07-30

**Outside the interface.** The sources do not say whether either behaves differently when Blender is driven from outside its window. Both are ordinary scene settings, and neither the manual pages nor Cycles' own file mentions the interface, background mode, or the viewport in connection with the render-side settings. (Cycles keeps *separate* viewport versions of the adaptive-sampling settings — `use_preview_adaptive_sampling`, `preview_adaptive_threshold`, `preview_adaptive_min_samples` — which is why the render-side ones are not viewport-specific.) **Observed 2026-08-05 (hand tests 11 and 12, below):** both take effect in a render started from a terminal with no window.

**Operating system.** The sources do not say. Nothing on these pages or in these property definitions names an operating system. The tests below were run on macOS only. **Observed 2026-08-06:** adaptive sampling and the EEVEE and Workbench property lists were run again on a rented Linux machine and came back the same, so those parts hold on macOS and on Linux. The Time Limit test was not re-run there, because it can only be read from timing and the machine was shared, so that part still holds for macOS alone. Windows stays untested.

### Observed — hand tests 11, 12 and 13 (2026-08-05)

All three were run from a terminal on the Creator's own computer — a MacBook Pro with an M1 Pro chip, macOS 26.5.2, Apple's own graphics and no other graphics card — using Blender 5.2.0 at `/Applications/Blender.app/Contents/MacOS/Blender`, with the `BLENDER_USER_RESOURCES` setting pointed at a fresh empty folder. Label: **observed**.

**Hand test 11 — is the Time Limit per frame or per job, and is a cut-short frame still written?** A file was saved from factory startup with Cycles on the processor, 400 by 400, Max Samples 16384, adaptive sampling off, denoising off, Time Limit 5 seconds, frame range 1 to 3. Then `blender -b test.blend -o out/f_ -F PNG -a`. The per-frame times printed were `Time: 00:06.64`, `Time: 00:06.68`, `Time: 00:06.59`, and the whole run took 21 seconds by the shell's clock. All three images were written: `f_0001.png f_0002.png f_0003.png`.

**This closes two "not covered" items above:** the limit is applied to each frame, not to a whole animation, and reaching it still writes the partly-rendered frame to disk.

**Hand test 12 — does adaptive sampling end the whole render early?** Two files were saved from factory startup, both Cycles on the processor, 128 by 128, Max Samples 2048, Time Limit 0, denoising off — one with adaptive sampling on and a Noise Threshold of 0.1, one with it off. The sample lines printed:

```
adaptive on  : Sample 0/2048  Sample 1/2048  Sample 48/2048  Sample 0/2048   Time: 00:00.10
adaptive off : Sample 2048/2048                                              Time: 00:01.51
```

**This closes another "not covered" item:** adaptive sampling does end the whole frame before the sample count is reached — here at 48 of 2048 — not merely the sampling of individual pixels.

**Hand test 13 — has EEVEE or Workbench any way to stop a render early?** Every property of `scene.eevee` and `scene.display` was listed from factory startup and searched for `time`, `threshold`, `adaptive`, `noise`, `stop` and `limit`. `scene.eevee` has 53 properties and `scene.display` has 9. Neither holds a time limit or a rule for stopping when the picture is clean enough. The nearest matches on `scene.eevee` were `bokeh_threshold` ("Brightness threshold for using sprite base depth of field") and `light_threshold` ("Minimum light intensity for a light to contribute to the lighting"); on `scene.display` there was no match. Its nine properties in full:

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

Run by the Creator in Blender 5.2's own window, on their own computer — a MacBook Pro with an M1 Pro chip, macOS 26.5.2 — on 2026-08-05. That settles macOS and leaves Windows and Linux untested. Label: **observed**.

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

**This closes the "not in the sources" answer above by eye as well as by property list.** Neither engine's Sampling panel offers a time limit, a noise threshold, or any other control that would end a render early.

### Observed on the rented Linux machine — Linux hand tests 13 and 14 (2026-08-06)

Run over SSH from a terminal on the rented Linux machine, using Blender 5.2.0 at `/opt/blender/blender-5.2.0-linux-x64/blender`, with `BLENDER_USER_RESOURCES` pointed at `/root/smoozy/Settings/res-5.2`, a folder of this topic's own that already existed. That folder was checked before use and reported its own config directory and Blender's eight default add-ons and nothing else. The machine is a container sharing someone else's hardware, and up to four topics were being worked on it at once, so nothing below claims anything about how long a run took or how much memory was free. Label: **observed**.

These are a second machine's answers to the tests the Creator's Mac answered on 2026-08-05. The two are never merged: where they agree it says so, and where they differ it says that plainly. The full list is `Audits/Blender Research/Settings/Hand tests — Linux.md`.

**Linux hand test 13 — does adaptive sampling end the frame before the sample count is reached?** Two Cycles files at 128 by 128, Max Samples 2048, Time Limit 0, denoising off — one with adaptive sampling on at a noise threshold of 0.1, one with it off. Both rendered one frame with `-- --cycles-print-stats`, both exited 0, both wrote their image. The sample counts reported:

```
adaptive on  : Sample 0/2048 -> Sample 1/2048 -> Sample 48/2048   (stopped there)
adaptive off : Sample 0/2048 -> Sample 1/2048 -> Sample 2048/2048
```

**The two machines agree**, at the same 48 samples out of 2,048. So adaptive sampling ends the whole frame early, not only the sampling of single pixels.

**Linux hand test 14 — has EEVEE or Workbench anything that stops a render?** Every property of the scene's EEVEE block and of its Workbench block was listed with its description. EEVEE holds **53** on this build and Workbench holds **9** — the same two counts the Mac's run gave for this version. Searching both lists for `time`, `threshold`, `adaptive`, `noise`, `stop` and `limit` matched only EEVEE lines, and none of them stops anything:

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

**Answer.** Blender splits them in two places, and the split is visible in both the interface and the Python data.

In the interface, the Properties editor has one tab whose contents change with the engine and several that do not:
> `Tabs related to the active scene:` / `Render: EEVEE, Cycles, or Workbench settings.` / `Output` / `View Layer` / `Scene` / `World`

and the manual says so plainly:
> `Each renderer has its own render settings to control render quality and performance.`
> `Switching render engines may change available settings, materials, and rendering behavior.` / `Some features are engine-specific.`

The recorded API entries name engine-specific blocks, including:

- Cycles: `Scene.cycles` — "Cycles render settings (readonly)" — and `Scene.cycles_curves` — "Cycles curves rendering settings (readonly)".
- EEVEE: `Scene.eevee` — "EEVEE settings for the scene (readonly)".
- Workbench: `Scene.display` — "Scene display settings for 3D viewport (readonly)"; its `render_aa` is "Method of anti-aliasing when rendering final image (default '8')".
- Hydra: `SceneHydra` — "Scene Hydra render engine settings".

The recorded quotes do not provide a complete list of the settings in these blocks or establish which engine uses each entry in a shared block.

Cycles' own file shows the mechanism directly — its settings panels only appear when Cycles is the engine:
> ```
> class CyclesButtonsPanel:
>     bl_space_type = "PROPERTIES"
>     bl_region_type = "WINDOW"
>     bl_context = "render"
>     COMPAT_ENGINES = {'CYCLES'}
>
>     @classmethod
>     def poll(cls, context):
>         return context.engine in cls.COMPAT_ENGINES
> ```

and Cycles attaches its whole settings block to the scene when it registers:
> ```
>     @classmethod
>     def register(cls):
>         bpy.types.Scene.cycles = PointerProperty(
>             name="Cycles Render Settings",
>             description="Cycles render settings",
>             type=cls,
>         )
> ```

**Label.** documented (the manual's split and the API's property blocks); observed (Cycles' own file, for how the engine-only settings are attached and shown).

**Not covered.** The sources do not give a line-by-line list of which individual entries in the shared `RenderSettings` block are honoured by which engine — some entries there are plainly narrower than "every engine" (for example `use_freestyle`, `hair_type`, `compositor_denoise_device`), and nothing states which engine reads which. Nor do they say what happens to an engine's settings block when the file is rendered with a different engine.

**Sources.**
- Manual, https://docs.blender.org/manual/en/5.2/editors/properties_editor.html — read 2026-07-30
- Manual, https://docs.blender.org/manual/en/5.2/render/render_engines.html — read 2026-07-30
- Manual, https://docs.blender.org/manual/en/5.2/render/introduction.html — read 2026-07-30
- API documentation, https://docs.blender.org/api/5.2/bpy.types.Scene.html — read 2026-07-30
- API documentation, https://docs.blender.org/api/5.2/bpy.types.RenderSettings.html — read 2026-07-30
- API documentation, https://docs.blender.org/api/5.2/bpy.types.SceneEEVEE.html — read 2026-07-30
- API documentation, https://docs.blender.org/api/5.2/bpy.types.SceneDisplay.html — read 2026-07-30
- API documentation, https://docs.blender.org/api/5.2/bpy.types.SceneHydra.html — read 2026-07-30
- Cycles' own files, `Blender Sources/cycles_5.2/ui.py` lines 60–68 (`CyclesButtonsPanel`), and `Blender Sources/cycles_5.2/properties.py` lines 1141–1147 (`CyclesRenderSettings.register`) — read 2026-07-30

**Outside the interface.** The manual's framing of the split ("tabs") is interface language, while the recorded API entries name the same settings blocks without referring to a window. The sources do not state whether their availability differs when Blender is driven from outside its window, or whether an engine's settings remain readable when its add-on is not enabled. The Cycles file does show that its *panels* are hidden when another engine is active (`poll` returns false); that is a display rule, not a statement about the data. **Observed 2026-08-05 (hand tests 6, 7 and 21):** all four blocks were reachable and held their own saved values in a Blender started from a terminal with no window, whichever engine was selected; and with the Cycles add-on switched off the engine list drops to `['BLENDER_EEVEE']` and the scene's engine reads back as `BLENDER_EEVEE` — see question 13.

**Operating system.** The sources do not say. Nothing in the split names an operating system. The test below was run on macOS only. **Observed 2026-08-06:** the same test was run again on a rented Linux machine and came back the same, so this holds on macOS and on Linux. Windows stays untested.

### Observed — hand test 13 (2026-08-05)

Run from a terminal on the Creator's own computer — a MacBook Pro with an M1 Pro chip, macOS 26.5.2 — using Blender 5.2.0 at `/Applications/Blender.app/Contents/MacOS/Blender`, with the `BLENDER_USER_RESOURCES` setting pointed at a fresh empty folder. Label: **observed**.

Every property of `scene.eevee` and `scene.display` was listed from factory startup, which is the complete list the recorded quotes could not give. `scene.eevee` holds 53 properties; `scene.display` holds 9, listed in full under question 2. Searching both for `device`, `cpu` and `gpu` returned only two lines, both about memory size rather than a choice:

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

So the tab whose contents the manual says change with the engine is chosen from a control offering those three, under their human labels rather than the identifiers `-E` takes.

### Observed on the rented Linux machine — Linux hand tests 6, 14 and 25 (2026-08-06)

Run over SSH from a terminal on the rented Linux machine, using Blender 5.2.0 at `/opt/blender/blender-5.2.0-linux-x64/blender`, with `BLENDER_USER_RESOURCES` pointed at `/root/smoozy/Settings/res-5.2`, a folder of this topic's own that already existed. That folder was checked before use and reported its own config directory and Blender's eight default add-ons and nothing else. The machine is a container sharing someone else's hardware, and up to four topics were being worked on it at once, so nothing below claims anything about how long a run took or how much memory was free. Label: **observed**.

These are a second machine's answers to the tests the Creator's Mac answered on 2026-08-05. The two are never merged: where they agree it says so, and where they differ it says that plainly. The full list is `Audits/Blender Research/Settings/Hand tests — Linux.md`.

**Linux hand tests 6 and 14 — are all the engines' blocks there at once, and what is on them?** A file was saved with Cycles samples 123, EEVEE render samples 45, Workbench final anti-aliasing `32` and the Grease Pencil final-render count 3, then read back in a fresh Blender started from a terminal with no window:

```
BACK as-loaded engine= BLENDER_EEVEE cycles= 123 eevee= 45 workbench= 32 gpencil= 3
```

All three engines' blocks were reachable and each held its own saved value, whichever engine was selected. Listing every property gave 53 on the EEVEE block and 9 on the Workbench block. **The two machines agree.**

**Linux hand test 25 — the Render Properties tab, photographed in Blender's own window on this machine.** Blender was started with a real window on the machine's own display, the engine set to Workbench and the Properties editor pointed at Render, and the window photographed with Blender's own screenshot command. Blender 5.2's tab lists **Curves** above Simplify. The tab carries the shared settings under whichever engine is chosen, exactly as the Creator's photographs of the same tab on the Mac showed. **The two machines agree.**

---

## 4. Does a file carry a separate sample count for each render engine?

**Answer.** The sources identify separate sampling-related settings for Cycles, EEVEE and Workbench. They state directly only that Workbench viewport and final-rendering anti-aliasing are saved per scene. The recorded sources do not establish that a file carries a separate sample count for every engine, or that switching engines leaves the other engines' values unchanged.

- **Cycles** — `Scene.cycles.samples`. From Cycles' own file:
  > ```
  > samples: IntProperty(
  >     name="Samples",
  >     description="Number of samples to render for each pixel",
  >     min=1, max=(1 << 24),
  >     default=4096,
  > )
  > ```
  and separately for the viewport:
  > ```
  > preview_samples: IntProperty(
  >     name="Viewport Samples",
  >     description="Number of samples to render in the viewport, unlimited if 0",
  >     min=0,
  >     soft_min=1,
  >     max=(1 << 24),
  >     default=1024,
  > )
  > ```
- **EEVEE** — `SceneEEVEE.taa_render_samples`: "Number of samples per pixel for rendering (in [1, inf], default 64)", and separately `taa_samples`: "Number of samples, unlimited if 0 (in [0, inf], default 16)".
- **Workbench** — the count is folded into the anti-aliasing choice, `SceneDisplay.render_aa`: "Method of anti-aliasing when rendering final image (default '8')", with options `OFF`, `FXAA`, `5`, `8`, `11`, `16`, `32` — for example "`8` / 8 Samples – Scene will be rendered using 8 anti-aliasing samples." Its viewport twin is `viewport_aa` (default `FXAA`).

That these are saved with the file rather than with the program is stated for Workbench:
> `The setting for the 3D Viewport is a user preference to specify the anti-aliasing method that runs best on the used system. The setting for viewport rendering and final rendering is saved per scene.`

On top of the per-engine scene counts there is one per-view-layer override, shared between engines but obeyed differently by each:
> `ViewLayer.samples` / `Override number of render samples for this view layer, 0 will use the scene setting (in [0, inf], default 0)`
> `Samples` / `Allows the view layer to use a different number of samples than the main scene. This is controlled by the Layer Samples when using the Cycles renderer. When using EEVEE, the Samples override is always used.`

**Label.** documented (the EEVEE, Workbench and view-layer settings, and the Workbench statement that final-rendering anti-aliasing is saved per scene); observed (Cycles' own file, for Cycles' own count and its exact default and range). Whether all of those settings persist in the file is not established by this record.

**Not covered.** The sources do not say in so many words that Cycles' or EEVEE's settings are written into the blend-file. The version-update code mentioned in question 10 is not enough to establish that. Nothing says what a third-party engine's sample count is called or whether it has one. Nothing states what happens to a Workbench file's `render_aa` when Cycles renders it, or vice versa.

**Sources.**
- API documentation, https://docs.blender.org/api/5.2/bpy.types.SceneEEVEE.html — read 2026-07-30
- API documentation, https://docs.blender.org/api/5.2/bpy.types.SceneDisplay.html — read 2026-07-30
- API documentation, https://docs.blender.org/api/5.2/bpy.types.ViewLayer.html — read 2026-07-30
- Manual, https://docs.blender.org/manual/en/5.2/render/workbench/sampling.html — read 2026-07-30
- Manual, https://docs.blender.org/manual/en/5.2/render/layers/override.html — read 2026-07-30
- Cycles' own file, `Blender Sources/cycles_5.2/properties.py`, `samples` at line 485 and `preview_samples` at line 491 — read 2026-07-30

**Outside the interface.** The sources do not distinguish. These are scene data, read by the engine at render time; nothing on these pages ties any of them to a window being open. The one interface-flavoured split that *is* documented is render-versus-viewport (each engine has both), not interface-versus-no-interface. **Observed 2026-08-05 (hand test 6, below):** a Blender started from a terminal with no window read all three counts back from the file at the values they were saved with.

**Operating system.** The sources do not say. The one nearby statement that mentions a system is Workbench's *viewport* anti-aliasing being "a user preference to specify the anti-aliasing method that runs best on the used system" — which is about the preference, not the file's render setting. The test below was run on macOS only. **Observed 2026-08-06:** the same test was run again on a rented Linux machine and came back the same, value for value, so this holds on macOS and on Linux. Windows stays untested.

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

**This closes the two points this record left open.** A file does carry a separate sample count for each of the three engines, all at once; the Cycles and EEVEE counts survive a save and a reload just as the Workbench one is documented to; and switching the engine changes none of them. The Grease Pencil count is one shared value, reading 3 under all three engines.

### Observed — hand test 26, run by the Creator (2026-08-05)

Run by the Creator in Blender 5.2's own window, on their own computer — a MacBook Pro with an M1 Pro chip, macOS 26.5.2 — on 2026-08-05. That settles macOS and leaves Windows and Linux untested. Label: **observed**.

Photographed with the Render Engine set to **EEVEE**, the Sampling panel shows a **Viewport ▸ Samples** of 16 and a separate **Render ▸ Samples** of 64. Photographed with the Render Engine set to **Workbench**, the whole Sampling panel is **Render — 8 Samples** and **Viewport — Single Pass Anti-Aliasing**, both menus rather than number fields. Cycles' own panel (hand test 24, under question 5) shows **Max Samples 4096** as a number field.

So each engine's final-render count is its own control in its own panel, and Workbench's really is a fixed menu where the other two are numbers — the difference in shape the finding above draws from the property lists, seen in the program.

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

## 5. Can a sample count change from frame to frame?

**Answer.** For Cycles, yes — the sample count is an animatable property, so it can be keyed or driven and will differ per frame. For EEVEE and Workbench the sources do not settle it. There is also a mechanism that reaches any engine's settings mid-render, though the sources stop short of confirming the effect.

The Cycles case, step by step. Cycles' sample count is declared as a plain `IntProperty` with no `options` argument:
> ```
> samples: IntProperty(
>     name="Samples",
>     description="Number of samples to render for each pixel",
>     min=1, max=(1 << 24),
>     default=4096,
> )
> ```
The API documentation gives that function's default for `options`:
> `bpy.props.IntProperty(*, name='', description='', translation_context='*', default=0, min=-2**31, max=2**31 - 1, soft_min=-2**31, soft_max=2**31 - 1, step=1, options={'ANIMATABLE'}, override=set(), tags=set(), subtype='NONE', update=None, get=None, set=None, get_transform=None, set_transform=None)`
and says of properties added this way:
> `These properties can be animated, accessed by the user interface and Python like Blender's existing properties.`

So: the property is declared without overriding `options`; the default `options` is `{'ANIMATABLE'}`; properties defined this way "can be animated"; an animated property holds a different value on different frames. The two steps a reader has to supply are that the omitted argument takes its documented default, and that "animated" means the value differs per frame. That makes this **inferred**, not documented.

The general mechanism, which applies whatever the engine: Blender calls a handler on every frame change during a render, and it is explicitly the place to change data for the new frame.
> `bpy.app.handlers.frame_change_pre` / `Called when a frame change is triggered for playback and rendering, before any data is evaluated for the new frame. This makes it possible to change data and relations (for example swap an object to another mesh) for the new frame.`
> `While rendering the frame_change_pre and frame_change_post handlers are called from one thread and the viewport updates from a different thread. If the handler changes data that is accessed by the viewport, this can cause a crash of Blender. In such cases, lock the interface (Render → Lock Interface or bpy.types.RenderSettings.use_lock_interface) before starting a render.`
There are also `render_pre` ("on render (before). Accepts one argument: the scene data-block being rendered") and `render_init` handlers. That a handler *can* set a sample count and that the engine *will* honour the new value for that frame is a step past what is written; I have not taken it.

For EEVEE (`taa_render_samples`) and Workbench (`render_aa`) the API documentation records type, default and range but says nothing about whether they are animatable, and the manual's sampling pages say nothing about animating them. I searched the manual's animation section for "any property", "most properties" and "properties can be animated" with no hits. Label for those two: **not in the sources**.

A related setting that varies the count but *not* by frame, which is worth naming so it is not mistaken for one: the per-view-layer override (question 4), and Cycles' `use_layer_samples` policy —
> ```
> enum_use_layer_samples = (
>     ('USE', "Use", "Per render layer number of samples override scene samples"),
>     ('BOUNDED', "Bounded", "Bound per render layer number of samples by global samples"),
>     ('IGNORE', "Ignore", "Ignore per render layer number of samples"),
> ```

**Label.** inferred (Cycles, with the step written above); documented (the handlers exist and are called during rendering); not in the sources (EEVEE, Workbench).

**Not covered.** Whether Cycles actually re-reads the sample count at the start of each frame of an animation render, as opposed to reading it once when the render job starts. Whether the same holds for EEVEE or Workbench. Whether a driver, as opposed to a keyframe, is evaluated in time to affect the frame (drivers are Python and so are subject to question 11's auto-run rules). Nothing about what a changing sample count does to Cycles' noise seed or to sample subsets.

**Sources.**
- API documentation, https://docs.blender.org/api/5.2/bpy.props.html — read 2026-07-30
- API documentation, https://docs.blender.org/api/5.2/bpy.app.handlers.html — read 2026-07-30
- API documentation, https://docs.blender.org/api/5.2/bpy.types.SceneEEVEE.html — read 2026-07-30
- API documentation, https://docs.blender.org/api/5.2/bpy.types.SceneDisplay.html — read 2026-07-30
- Cycles' own file, `Blender Sources/cycles_5.2/properties.py`, `samples` at line 485, `enum_use_layer_samples` at line 64, `use_layer_samples` at line 556 — read 2026-07-30

**Outside the interface.** The handler documentation is the one place the sources touch this, and it does not distinguish: `frame_change_pre` is "called for playback and rendering". Its warning is about a viewport drawing at the same time, which is an interface concern; the suggested fix, Lock Interface, is itself an interface setting (`RenderSettings.use_lock_interface`). Whether that hazard exists at all with no window open is not stated. **Observed 2026-08-05 (hand test 9, below):** a render started from a terminal with no window used the keyframed value at each frame.

**Operating system.** The sources do not say. Nothing here names an operating system. The test below was run on macOS only. **Observed 2026-08-06:** the same tests were run again on a rented Linux machine and came back the same, so this holds on macOS and on Linux. Windows stays untested.

### Observed — hand test 9 (2026-08-05)

Run from a terminal on the Creator's own computer — a MacBook Pro with an M1 Pro chip, macOS 26.5.2 — using Blender 5.2.0 at `/Applications/Blender.app/Contents/MacOS/Blender`, with the `BLENDER_USER_RESOURCES` setting pointed at a fresh empty folder. Label: **observed**.

From factory startup, `keyframe_insert` was called on each engine's count and on a view layer's `use`:

```
KEY cycles.samples               -> True
KEY eevee.taa_render_samples     -> True
KEY display.render_aa            -> ERROR bpy_struct.keyframe_insert() property "render_aa" not animatable
KEY view_layer.use               -> True
```

**This closes the "not in the sources" answer for EEVEE and Workbench:** EEVEE's `taa_render_samples` can be keyframed and Workbench's `render_aa` cannot.

A file was then saved with Cycles, 64 by 64, adaptive sampling off, denoising off, and Max Samples keyframed to 4 at frame 1 and 512 at frame 2. Rendering it with `blender -b test.blend -o out/f_ -F PNG -s 1 -e 2 -a` printed:

```
00:00.412  render           | Rendering frame 1
00:00.417  render           | Fra: 1 | Mem: 1M | Sample 4/4
00:00.417  render           | Fra: 1 | Mem: 1M | Finished
00:00.485  render           | Rendering frame 2
00:00.571  render           | Fra: 2 | Mem: 1M | Sample 512/512
00:00.571  render           | Fra: 2 | Mem: 1M | Finished
```

**This closes the first "not covered" item above.** Cycles does re-read the sample count at the start of each frame of an animation render, rather than reading it once when the job starts.

### Observed — hand test 24, run by the Creator (2026-08-05)

Run by the Creator in Blender 5.2's own window, on their own computer — a MacBook Pro with an M1 Pro chip, macOS 26.5.2 — on 2026-08-05. That settles macOS and leaves Windows and Linux untested. Label: **observed**.

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

**This is the two steps the finding above supplies, seen rather than reasoned.** The interface treats the sample count as animatable — it offers **Insert Keyframe** with `I`, and **Add Driver...** — even though the panel carries no dot to click.

### Observed on the rented Linux machine — Linux hand tests 11 and 12 (2026-08-06)

Run over SSH from a terminal on the rented Linux machine, using Blender 5.2.0 at `/opt/blender/blender-5.2.0-linux-x64/blender`, with `BLENDER_USER_RESOURCES` pointed at `/root/smoozy/Settings/res-5.2`, a folder of this topic's own that already existed. That folder was checked before use and reported its own config directory and Blender's eight default add-ons and nothing else. The machine is a container sharing someone else's hardware, and up to four topics were being worked on it at once, so nothing below claims anything about how long a run took or how much memory was free. Label: **observed**.

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

**Answer.** No — one engine is documented as failing this on one operating system, and the sources are silent on a second engine.

The general position is that the command line does not need a display:
> `One advantage of using the command line is that we do not need a graphical display (no need for X server on Linux for example) and consequently we can render via a remote shell (typically SSH).`

But EEVEE carries an explicit limitation, under its own heading "Headless Rendering":
> `Headless rendering is not supported on headless Windows systems.`

and EEVEE cannot fall back to the processor:
> `Being a rasterization engine, EEVEE only uses the power of the GPU to render. There is no plan to support CPU (software) rendering as it would be very inefficient. CPU power is still needed to handle high scene complexity as the geometry must be prepared by the CPU before rendering each frame.`

The 5.2 release notes add a related fact about driving the graphics card with no window:
> `Added gpu.init() to initialize gpu backend when running Blender in background mode --background.`
and the API documentation for it:
> `gpu.init()` / `Initializes the GPU module for background use. If the initialization fails, a SystemError will be raised.`

For **Workbench** I found nothing. I searched the whole manual, API documentation and release notes for "headless", "offscreen", "off-screen", "no display", "virtual display", "EGL" and "monitor attached". The only "headless" hits anywhere are the three EEVEE ones above. The Workbench pages (index, performance, sampling, options, lighting, color, display settings, grease pencil) do not mention GPU, OpenGL, background mode or headless operation at all. So for Workbench: **not in the sources**.

For **Cycles** there is no engine-specific statement either; what stands is the general command-line statement above plus the fact that Cycles can be told to render on the processor (question 9).

**Label.** documented (the general command-line statement, and EEVEE's limitation); not in the sources (Workbench; and Cycles beyond the general statement).

**Not covered.** What "headless Windows systems" means exactly — whether it covers a Windows machine with a graphics card but no monitor plugged in, a remote-desktop session, or a server with no graphics card at all. Whether EEVEE renders headless on Linux and macOS is not stated either way; only Windows is named. Whether a third-party engine renders headless. Whether `gpu.init()` is needed for a render, or only for scripts using the `gpu` module directly.

**Sources.**
- Manual, https://docs.blender.org/manual/en/5.2/advanced/command_line/render.html — read 2026-07-30
- Manual, https://docs.blender.org/manual/en/5.2/render/eevee/limitations/limitations.html — read 2026-07-30
- Release notes, https://developer.blender.org/docs/release_notes/5.2/python_api/ — read 2026-07-30
- API documentation, https://docs.blender.org/api/5.2/gpu.html — read 2026-07-30

**Outside the interface.** This question is entirely about outside the interface, and the answer above is that case. Inside the interface the question does not arise, since a window implies a screen.

**Operating system.** It differs by operating system, and that is the whole point of the one hard statement: EEVEE's headless limitation is stated for Windows only. The general "no need for X server" statement names Linux. Neither macOS statement exists. **Observed 2026-08-05 (hand test 14, below):** all three engines render with no window on macOS. Windows and Linux were not tested — that test was run on the Creator's one MacBook Pro. **Observed 2026-08-06:** on a rented Linux machine with one graphics card, all three engines rendered with no window and again with no display reachable at all, EEVEE included — but that machine had a display server running and its runs were only cut off from it. **Observed 2026-08-06 on a second rented Linux machine that has no display server at all** — two NVIDIA GeForce RTX 4090s, driver 580.95.05, sixty-one cores, over SSH — every engine this version has rendered there, on the cards and on the processor, EEVEE included. So **the screenless case is now answered for Linux**, in the sense of a machine with no display server. The headless *Windows* case the manual names stays untested, no Windows machine having run it, and so does a machine with no graphics hardware at all.

### Observed — hand test 14 (2026-08-05)

Run from a terminal on the Creator's own computer — a MacBook Pro with an M1 Pro chip, macOS 26.5.2, Apple's own graphics and no other graphics card — using Blender 5.2.0 at `/Applications/Blender.app/Contents/MacOS/Blender`, with the `BLENDER_USER_RESOURCES` setting pointed at a fresh empty folder. Label: **observed**.

A file was saved from factory startup at 64 by 64, Cycles samples 4, EEVEE render samples 4, and rendered three times, once per engine, each with `-b` so no window opened:

```
CYCLES               exit=0  image=64x64
BLENDER_EEVEE        exit=0  image=64x64
BLENDER_WORKBENCH    exit=0  image=64x64
```

No error was printed by any of the three, and `gpu.init()` was not called. **This closes the "not in the sources" answer for Workbench and for Cycles, for macOS with no window.**

**What this does not settle.** This machine has a screen attached. Running with no window is not the same as running on a machine with no screen, and the one hard statement in the sources is about a headless *Windows* machine. That test needs a machine with no display reached over a remote shell, and a Windows machine — neither of which the Creator has. It stays unanswered, and so does what "headless Windows systems" means.

### Observed on the rented Linux machine — Linux hand tests 15 and 16 (2026-08-06)

Run over SSH from a terminal on the rented Linux machine, using Blender 5.2.0 at `/opt/blender/blender-5.2.0-linux-x64/blender`, with `BLENDER_USER_RESOURCES` pointed at `/root/smoozy/Settings/res-5.2`, a folder of this topic's own that already existed. That folder was checked before use and reported its own config directory and Blender's eight default add-ons and nothing else. The machine is a container sharing someone else's hardware, and up to four topics were being worked on it at once, so nothing below claims anything about how long a run took or how much memory was free. Label: **observed**.

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

Run over SSH on a **second rented Linux machine — Ubuntu 22.04.5 LTS, two NVIDIA GeForce RTX 4090s of 24 GB each on driver 580.95.05, an AMD EPYC 7B13 with sixty-one cores, 98 GB of memory, no display server at all** — using Blender at `/opt/blender/blender-5.2.0-linux-x64/blender`, which `--version` reports as `Blender 5.2.0 LTS (hash fbe6228777e7 built 2026-07-14 01:32:04)`, with `BLENDER_USER_RESOURCES` pointed at `/root/smoozy/no screen/res-5.2`, a folder of its own that already existed and was empty. Nothing else was running on the machine. Label: **observed**. The full list is `Audits/Blender Research/Settings/Hand tests — two cards and no screen.md`.

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
GPU  CYCLES exit=0  png=5067
GPU  BLENDER_EEVEE_NEXT exit=0  png=3478
GPU  BLENDER_EEVEE exit=0  png=3478
GPU  BLENDER_WORKBENCH exit=0  png=3669
CPU  CYCLES exit=0  png=5065
CPU  BLENDER_EEVEE_NEXT exit=0  png=3478
CPU  BLENDER_EEVEE exit=0  png=3478
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
00:00.059  ghost.system     | Failed to initialize display for back-end(s): ['WAYLAND', 'X11']
                            |   'WAYLAND': unable to connect to display!
                            |   'X11': unable to open a display!
                            | 
00:00.059  ghost.system     | ERROR Unable to initialize GHOST, exiting!
Error: Not freed memory blocks: 4890, total unfreed memory 1.260777 MB
```

on **standard output**, and exited **1**. Nothing was written to standard error. Word for word what the one-card machine printed when cut off from its display, with the same exit status, plus the memory-leak line that machine never showed.

**What this settles.** **On Linux, every render engine this version has renders on a computer with no display server at all** — not merely a run cut off from a display. That is the half of this question item A of the Linux list left open, and it is now answered for Linux. **It does not settle a machine with no screen physically attached in the sense of no graphics hardware**: the cards are present, they are simply not driving a display. And it does not touch the manual's one hard statement, which names a headless *Windows* machine — no Windows machine has run it.

---

## 7. Does a scene set up for one render engine come out the same in another?

**Answer.** No. The manual says so directly, in the section headed "Switching Engines":
> `Switching render engines may change available settings, materials, and rendering behavior.`
> `Some features are engine-specific.`
> `Materials may appear differently between engines.`
> `Certain nodes or effects may only be supported in specific engines.`
> `It is recommended to choose a render engine early in production to avoid workflow inconsistencies.`

and again in the rendering introduction, about what defines the look:
> `What the render looks like is defined by cameras, lights and materials. These are shared between EEVEE and Cycles, however some features are only supported in one or the other.`

The engines are described as fundamentally different methods — "Real-time rasterization" for EEVEE, "Path tracing" for Cycles, "Basic rasterization" for Workbench — and Workbench is not meant for final images at all:
> `It is not intended to be a render engine that will render final images for a project. Its primary task is to display a scene in the 3D Viewport when it is being worked on.`
> `Unlike other render engines such as EEVEE or Cycles, the Workbench engine does not use shader nodes.`

EEVEE's limitations page gives concrete examples of scene data that will not carry over, among many:
> `Only perspective and orthographic projections are currently supported.`
> `Lights can only have one color and do not support light node trees.`
> `Unlike in Cycles, the Size of spot lights does not change the softness of the cone.`
> `The area light Beam spread option is not supported.`
> `Only 14 attributes from Geometry Nodes are supported in a material`

And the 5.2 release notes record a feature that renders differently by engine on purpose:
> `Support is currently limited to Cycles. When using EEVEE, a warning is shown on the node and the output is zero.` (of the Raycast node's new attribute outputs)

There is also a documented difference in what the compositor gets:
> `The viewport compositor only supports render passes when using EEVEE. For other engines, the passes will be empty.`

**Label.** documented.

**Not covered.** The sources do not quantify anything — "may appear differently" is as precise as it gets, and there is no list of what *is* guaranteed to match. Nothing about whether output resolution, frame range, colour management or file format come out identical (they are shared settings, but that is not stated as a guarantee of identical output). Nothing about third-party engines.

**Sources.**
- Manual, https://docs.blender.org/manual/en/5.2/render/render_engines.html — read 2026-07-30
- Manual, https://docs.blender.org/manual/en/5.2/render/introduction.html — read 2026-07-30
- Manual, https://docs.blender.org/manual/en/5.2/render/workbench/index.html — read 2026-07-30
- Manual, https://docs.blender.org/manual/en/5.2/render/eevee/limitations/limitations.html — read 2026-07-30
- Manual, https://docs.blender.org/manual/en/5.2/compositing/types/input/scene/render_layers.html — read 2026-07-30
- Release notes, https://developer.blender.org/docs/release_notes/5.2/cycles/ — read 2026-07-30

**Outside the interface.** The sources do not distinguish. These are statements about what the engines can represent, and nothing on these pages ties any of them to a window being open. The one exception is the viewport-compositor passes note, which is about the viewport specifically.

**Operating system.** The sources do not say for this question. (Separately, EEVEE's headless limitation and Cycles' device backends are operating-system-specific — see questions 6 and 9 — but nothing states that the *look* differs by operating system.)

---

## 8. Can one scene be set up to take in another scene's rendered result, so that rendering the first renders the second?

**Answer.** Yes, in two documented ways.

**Through the compositor**, with the Render Layers node:
> `Renders a View Layer and reads its Passes into the compositing node graph.`
> `Scene` / `The scene for which to render a view layer.`
> `View Layer` / `The view layer to render. The button next to the dropdown re-renders it immediately.`

The node names the scene, and the node's own description says it *renders* that view layer, not that it reads a previously saved image. There is a documented limit on this route:
> `To use the compositing output from another scene rather than its "raw" render output, first render that scene into a series of multi-layered images (using e.g. the OpenEXR format), then load those images into the Compositor of the current scene using the Image Node.`
— that is, the Render Layers node brings in the other scene's raw render, not the other scene's own composite.

**Through the video sequencer**, with a Scene strip:
> `Scene strips are a way to insert the render output of another scene into your sequence. Instead of rendering out a video, then inserting the video file, you can insert the scene directly.`
> `The strip length will be determined based on the animation settings in that scene.`
> `Scene strips cannot be used to reference the sequence's own scene; a secondary scene must be used instead.`
> `Scene` / `A Data-Block Menu to select or create the scene to render from.`
> `View Layer` / `View Layer from the selected scene used for rendering.`
> `Input` / `Input type to use for the Scene strip.` / `Camera:` `Use the Scene's 3D camera as input.` / `Sequencer:` `Use the scene's Sequencer timeline as input, allowing one scene to reuse another scene's edit (instead of taking the render output from the scene).`
with a limit:
> `Scene strips do not render individual Render Passes; only the Combined render pass will be used.`

The view-layer selector on scene strips is new in this version:
> `Scene strips now have a view layer selector, initially set to the default view layer.`

Blender 5.2 also formalises a scene that exists to hold the edit:
> `The video sequencer uses a separate scene data-block that contains the edit. This is the Sequencer Scene.`
> `Since the sequencer scene is separate from the active scene in the window it is possible to work with scene strips in the sequencer while also viewing and editing the referenced scenes within the same workspace.`

**Label.** documented.

**Not covered.** Neither page says what happens when the chain is deeper than two scenes, nor whether a cycle is detected (only the sequencer's one-step rule is stated: a scene strip cannot reference its own scene). Nothing says which scene's resolution, frame range, sample count or output settings win when scene A pulls in scene B — the sequencer page says only that the strip's length follows the referenced scene's animation settings. Nothing states whether the pulled-in scene's own engine or the host scene's engine is used.

**Sources.**
- Manual, https://docs.blender.org/manual/en/5.2/compositing/types/input/scene/render_layers.html — read 2026-07-30
- Manual, https://docs.blender.org/manual/en/5.2/video_editing/edit/montage/strips/scene.html — read 2026-07-30
- Manual, https://docs.blender.org/manual/en/5.2/video_editing/sequencer_scene.html — read 2026-07-30
- Release notes, https://developer.blender.org/docs/release_notes/5.2/sequencer/ — read 2026-07-30

**Outside the interface.** The sources do not say directly. Both routes are stored scene data — a node in a node tree, a strip in a sequence — rather than actions a person takes at render time, so nothing about them requires a window. The one caution the sources do give is that the compositor's Render Layers node has a *button* that re-renders immediately, which is plainly an interface action and not part of an unattended render. Note also that the Sequencer Scene can differ from the active scene, which matters for what an unattended render actually renders — see question 12. **Observed 2026-08-05 (hand tests 16, 17, 18 and 19, below):** both routes work in a render started from a terminal with no window, and a bare command-line render uses the active scene rather than the Sequencer Scene.

**Operating system.** The sources do not say. Nothing on these pages names an operating system. The tests below were run on macOS only. **Observed 2026-08-06:** the same tests were run again on a rented Linux machine and came back the same, so this holds on macOS and on Linux. Windows stays untested.

### Observed — hand tests 16, 17, 18 and 19 (2026-08-05)

All were run from a terminal on the Creator's own computer — a MacBook Pro with an M1 Pro chip, macOS 26.5.2 — using Blender 5.2.0 at `/Applications/Blender.app/Contents/MacOS/Blender`, with the `BLENDER_USER_RESOURCES` setting pointed at a fresh empty folder. Label: **observed**.

**Hand test 17 — a Render Layers node pointed at another scene.** A file was saved with two scenes. `Alpha` used Workbench at 64 by 64 with its compositing pipeline on, and its compositing node group held one Render Layers node with Scene set to `Beta`, wired to the group output. `Beta` used Cycles with 8 samples, and its own node group inverted its render. Rendering `Alpha`:

```
blender -b test.blend -S Alpha -o out/c_ -F PNG -f 1
00:00.397  render           | Start rendering: Beta, ViewLayer
00:00.397  render           | Engine: Cycles
00:00.401  render           | Fra: 1 | Mem: 1M | Sample 0/8
00:00.401  render           | Fra: 1 | Remaining: 00:00.00 | Mem: 1M | Sample 1/8
00:00.454  render           | Fra: 1 | Mem: 1M | Sample 8/8
```

Three images built from the same file were then compared — `Alpha`'s output, `Beta` rendered directly with its compositing pipeline on, and `Beta` rendered directly with it off. The first pixel of each:

```
alpha_0001.png    [0.0, 0.0, 0.0, 1.0]
betacomp_0001.png [0.7686, 0.7686, 0.7686, 1.0]
betaraw_0001.png  [0.0, 0.0, 0.0, 1.0]
```

**This closes one of the "not covered" items above:** the pulled-in scene's own engine and its own sample count are used, not the host scene's; and the pulled-in scene's own compositor does not run — its raw render is what crosses.

**Hand test 18 — two scenes' Render Layers nodes pointed at each other.** With `Alpha`'s node pointed at `Beta` and `Beta`'s at `Alpha`, both pipelines on, both Workbench, rendering `Alpha` with a sixty-second cut-off finished in 0.9 seconds, exit code 0, and wrote an image:

```
00:00.398  render           | Start rendering: Beta, ViewLayer
00:00.398  render           | Engine: Workbench
00:00.479  render           | Executing compositor
00:00.479  render           | Fra: 1 | Compositing
Scene switch for render: 'Alpha' in file: '.../circular.blend'
00:00.547  render           | Saved: '.../out/x_0001.png'
```

**This closes another "not covered" item:** a cycle produces no error and no hang; `Beta` is rendered once and the chain stops there.

**Hand test 19 — a Scene strip.** A file was saved with `Alpha` on Workbench, its sequencer pipeline on and one Scene strip referencing `Beta` at frame 1; `Beta` used Cycles with 8 samples. Rendering `Alpha` from the command line printed `Start rendering: Beta, ViewLayer`, `Engine: Cycles` and `Sample 8/8`, and wrote an image. So the referenced scene's own engine is used here too.

**Hand test 16 — a Sequencer Scene against the active scene.** A file was saved with three scenes: `Alpha`, `Beta`, and `Edit` whose sequencer held a Scene strip referencing `Alpha`. The workspace's sequencer scene was set to `Edit` and the window's scene to `Beta`. A bare `blender -b test.blend -o out/s_ -F PNG -f 1`, with a handler added at launch that names the scene the render starts on, printed:

```
RENDER_PRE SCENE: Beta
```

So a render started from outside uses the active scene and the Sequencer Scene has no effect on it.

### Observed on the rented Linux machine — Linux hand tests 19, 20 and 21 (2026-08-06)

Run over SSH from a terminal on the rented Linux machine, using Blender 5.2.0 at `/opt/blender/blender-5.2.0-linux-x64/blender`, with `BLENDER_USER_RESOURCES` pointed at `/root/smoozy/Settings/res-5.2`, a folder of this topic's own that already existed. That folder was checked before use and reported its own config directory and Blender's eight default add-ons and nothing else. The machine is a container sharing someone else's hardware, and up to four topics were being worked on it at once, so nothing below claims anything about how long a run took or how much memory was free. Label: **observed**.

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

**Answer.** The sources establish a Cycles choice between processor and graphics card on each scene, and establish that the selected graphics cards and technology live in the program's preferences on the machine. They do not directly establish whether the scene choice is recorded in the file.

For Cycles, the source record shows a two-way setting on each scene:
> `Device` / `Specifies the compute device used for rendering in Cycles. The choice of device can significantly affect render performance, GPU Compute is often faster .`
> `CPU:` / `Render using the system's central processing unit. This mode is compatible with all Cycles features. The number of CPU threads used can be changed in the Performance Properties.`
> `GPU Compute:` / `Render using one or more graphics processing units (GPUs) through the compute backend and devices selected in the Cycles Render Devices. Depending on the backend, it may be possible to use both CPU and GPU devices simultaneously.`

In Cycles' own file, exactly two choices, defaulting to the processor:
> ```
> enum_devices = (
>     ('CPU', "CPU", "Use CPU for rendering"),
>     ('GPU', "GPU Compute",
>         "Use GPU compute device for rendering, configured in the system tab in the user preferences"),
> )
> ```
> ```
> device: EnumProperty(
>     name="Device",
>     description="Device to use for rendering",
>     items=enum_devices,
>     default='CPU',
>     update=update_render_passes,
> )
> ```

What the file does *not* hold is which cards. The manual's setup sequence separates the two:
> `To enable GPU rendering:` / `Select either CUDA, OptiX, HIP, oneAPI, or Metal in the Cycles Render Devices user preference.` / `Enable the available devices for the chosen backend.` / `Configure each scene to use the GPU Compute Device.`

The first two steps are preferences; only the third is per scene. In Cycles' own file the backend choice and the device list sit on `CyclesPreferences`, an `AddonPreferences` class, not on the scene:
> ```
> compute_device_type: EnumProperty(
>     name="Compute Device Type",
>     description="Device to use for computation (rendering with Cycles)",
>     default=CyclesPreferences.default_device(),
>     items=CyclesPreferences.get_device_types,
> )
>
> devices: CollectionProperty(type=CyclesDeviceSettings)
> ```
and preferences are stored on the machine, in `./config/userpref.blend` — "User preferences."

The scene's choice can be overridden at launch without touching the file, and the override names the technology as well:
> `--cycles-device <device>` / `Set the device used for rendering.` / `Valid options are: CPU CUDA OPTIX HIP ONEAPI METAL.` / `Append +CPU to a GPU device to render on both CPU and GPU.`
> `blender -b file.blend -f 20 -- --cycles-device OPTIX`

For **EEVEE** the question is settled by the engine itself rather than by a setting: "Being a rasterization engine, EEVEE only uses the power of the GPU to render. There is no plan to support CPU (software) rendering". For **Workbench** I found no device setting at all; `SceneDisplay` has no such property and the Workbench pages do not mention one. Label for Workbench: **not in the sources**.

**Label.** documented (the manual's device setting and its split from preferences; EEVEE's GPU-only nature; the command-line override); observed (Cycles' own file, for the exact two-way setting, its default, and where the backend and device list actually live); not in the sources (Workbench). Whether the Cycles scene setting persists in a file is not established by this record.

**Not covered.** Whether the scene's Device setting is written into the blend-file is not stated in those words — it is a scene property, and the manual says "Configure each scene", but "each scene" and "saved in the file" are not the same sentence. Nothing says what happens when a file asking for GPU Compute is opened on a machine whose preferences select no graphics device. Nothing says whether `--cycles-device` changes the file or only the run.

**Sources.**
- Manual, https://docs.blender.org/manual/en/5.2/render/cycles/render_settings/index.html — read 2026-07-30
- Manual, https://docs.blender.org/manual/en/5.2/render/cycles/gpu_rendering.html — read 2026-07-30
- Manual, https://docs.blender.org/manual/en/5.2/editors/preferences/system.html — read 2026-07-30
- Manual, https://docs.blender.org/manual/en/5.2/advanced/blender_directory_layout.html — read 2026-07-30
- Manual, https://docs.blender.org/manual/en/5.2/advanced/command_line/arguments.html — read 2026-07-30
- Manual, https://docs.blender.org/manual/en/5.2/render/eevee/limitations/limitations.html — read 2026-07-30
- Cycles' own file, `Blender Sources/cycles_5.2/properties.py`: `enum_devices` at line 30, `device` at line 387, `compute_device_type` at line 1655, `devices` at line 1662 — read 2026-07-30

**Outside the interface.** The command line adds an override that only exists outside the interface (`--cycles-device`). The sources do not say whether the scene setting itself behaves differently there. One source makes explicit that preferences still apply when there is no window — "In this case the Preferences are still used but you may want to override them" — but it says this about script auto-run, not device selection. That the machine's device preferences apply to an unattended render is a reading of "configured in the system tab in the user preferences", not a direct statement.

**Operating system.** It differs sharply. The manual ties each technology to particular systems: "CUDA is supported on Windows and Linux"; "OptiX is supported on Windows and Linux"; "HIP is supported on Windows and Linux"; "oneAPI is a computation library that is supported on Windows and Linux"; "Metal for GPU acceleration is supported on Apple computers with Apple Silicon. macOS 13.0 or newer is required to support all features." Cycles' own file also picks a different default backend by system: "Default to selecting the Metal compute device on Apple Silicon GPUs (drivers are tightly integrated with macOS so pose no stability risk)", guarded by `platform.system() == 'Darwin'` and `platform.machine() == 'arm64'` (`properties.py`, lines 1628–1634). So a file saying "GPU Compute" means a different technology on different machines. **Observed 2026-08-05:** on this machine Metal is what the device list holds; CUDA, OptiX, HIP and oneAPI could not be tried at all, so what a file's `GPU` choice means on any other make of card stays untested. **Observed 2026-08-06:** a rented Linux machine with an NVIDIA GeForce RTX 4090 settles most of what was left here. That machine offers `CUDA`, `OPTIX`, `HIP` and `ONEAPI` and not `METAL`; `--cycles-device CUDA` and `OPTIX` render on the card, and `HIP`, `ONEAPI` and `METAL` are all refused with the same message. A file asking for GPU Compute renders on the card under CUDA or OptiX and falls back to the processor under the others — with nothing printed at all, except in the `HIP` case, where one warning line reads `WARNING HIPEW initialization failed: Error opening HIP dynamic library`. What an AMD or an Intel card does stays untested, and so does Windows. *(Changed by the citation check of the Linux hand tests, 2026-08-06: this line said "silently" for every case, which the block below already contradicts for HIP in this version.)*

### Observed — hand tests 7 and 8 (2026-08-05)

Both were run from a terminal on the Creator's own computer — a MacBook Pro with an M1 Pro chip, macOS 26.5.2, Apple's own graphics and no other graphics card — using Blender 5.2.0 at `/Applications/Blender.app/Contents/MacOS/Blender`, with the `BLENDER_USER_RESOURCES` setting pointed at fresh empty folders so the Creator's own Blender settings were untouched. Label: **observed**.

**Hand test 7 — is the scene's Device setting written into the file?** A file was saved from factory startup with two scenes: `One` with the Cycles device set to `GPU` and the compositor device set to `GPU`, and `Two` left at its defaults. A fresh Blender opened it and printed:

```
SCENE One cycles_device= GPU compositor_device= GPU
SCENE Two cycles_device= CPU compositor_device= GPU
```

**This closes the point this record left open.** The Cycles device choice is written into the blend-file, survives a save and a reload, and is held one scene at a time — `One` came back as `GPU` while `Two` stayed `CPU`. (Note the compositor device: in this version a new scene starts at `GPU`, which matches the API's stated default. In 4.5, 5.0 and 5.1 the same test gave `CPU` for the new scene.)

**Hand test 8 — what `--cycles-device` beats, and the double dash.** A file was saved with Cycles on the processor, 100 by 100, 16 samples. A throwaway settings folder was prepared with Metal chosen; it reported:

```
DEVICE: CPU Apple M1 Pro use= True
DEVICE: METAL Apple M1 Pro (GPU - 14 cores) use= True
SAVED compute_device_type= METAL
```

With the file still saying `CPU`, `blender -b test.blend -o out/a_ -F PNG -f 1 -- --cycles-device METAL --cycles-print-stats` produced statistics carrying the line `Profiling information not available (only works with CPU rendering)`, which the same run without the override did not carry. So the argument does beat the file's own setting.

Without the double dash:

```
blender -b test.blend -o out/b_ -F PNG -f 1 --cycles-device METAL
unknown argument, loading as file: --cycles-device
00:00.675  reports          | ERROR Cannot read file ".../--cycles-device": No such file or directory
```

With a name no device bears:

```
blender -b test.blend -o out/c_ -F PNG -f 1 -- --cycles-device NOT_A_DEVICE
00:00.284  cycles           | ERROR NOT_A_DEVICE is not a valid Cycles device.
Error: Not freed memory blocks: 1, total unfreed memory 0.000004 MB
```

and no image was written.

**A file asking for the graphics card on a machine with none enabled.** A second throwaway settings folder was prepared with the compute device type set to `NONE`, and the scene's device forced to `GPU` at launch. The whole terminal output named no device and carried no warning of any kind; its statistics carried no `Profiling information not available` line; and it ended:

```
00:00.486  render           | Fra: 1 | Mem: 1M | Sample 16/16
00:00.487  render           | Fra: 1 | Mem: 1M | Finished
00:00.554  render           | Saved: '.../out/dd2_0001.png'
00:00.554  render           | Time: 00:00.14 (Saving: 00:00.06)
```

**This closes another "not covered" item:** it renders on the processor, writes the image, and says nothing about it. And `--cycles-device` changed nothing in the file — hand test 4 showed the launch arguments leave the file on disk alone.

**Workbench.** Hand test 13 (under question 3) listed every property of `scene.display`: nine, none of them a device choice.

### Observed — hand test 27, run by the Creator (2026-08-05)

Run by the Creator in Blender 5.2's own window, on their own computer — a MacBook Pro with an M1 Pro chip, macOS 26.5.2 — on 2026-08-05. That settles macOS and leaves Windows and Linux untested. Label: **observed**.

With the Render Engine set to **Workbench**, they expanded every panel of the Render Properties tab and photographed the whole of it. The panels, in order: Curves, Simplify, Performance, Sampling, Lighting, Object Color, Options, Film, Grease Pencil, Freestyle, Color Management.

**Three controls in that tab name a processor or a graphics card, and all three are in the Performance panel:**

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

Those are the compositor's and the denoise nodes' devices — the two engine-neutral `RenderSettings` entries recorded above — shown here under Workbench because the tab carries them whatever the engine. **No control anywhere in the tab is a Workbench device setting.** The only other control naming a device is Color Management ▸ Advanced ▸ **Display Emulation**, reading `Automatic`, which is about colour rather than about a processor or a graphics card.

**This closes the "not in the sources" answer for Workbench.**

**And it agrees with the difference hand test 7 found.** Here the Compositor Device shows **GPU** selected on a factory-startup file, where the same panel in Blender 4.5, 5.0 and 5.1 shows **CPU** — which is what reading the value back from a new scene gave in each version. This version's panel also carries two rows the older three do not: **Anisotropic Filtering** and the compositor's **Precision**.

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

## 10. What does Blender state about opening a file that was saved by a different version of Blender?

**Answer.** Blender states that it handles both directions, older-to-newer and newer-to-older, with limits, and that the newer-to-older direction should be expected to lose things.

> `Blender can open blend-files saved with both older versions of the software (backward compatibility), and newer ones (forward compatibility). This comes with some limitations though.`

Opening an older file:
> `Opening older files and converting them for the current version of Blender is usually straight-forward. It is expected to give very good and usable results.`
> `There can be major feature changes, for which the backward compatibility will only be ensured for a limited amount of time. For example the changes to the animation system that happened during the Blender 2.5x project. This will never be less than a full major release cycle (i.e. two years at least).`

Opening a newer file:
> `Forward compatibility is inherently harder to ensure, and loss of feature should always be expected when opening a blend-file saved with a more recent version of Blender.`
> `A warning is shown in the UI when editing a more recent blend-file. Trying to overwrite it (with a simple 'Save' operation) will also show a confirmation popup, as this could make that loss of data permanent.`

And the hard stop:
> `When Blender switches to a new major version release (e.g. from 3.x to 4.0), there can also be major changes that will make the blend-file fully incompatible with older versions of Blender.`
> `In such cases, older Blender will fail opening (or appending/linking from) the newer blend-file, with a message stating which minimal version is needed to open it.`
> `In such cases, the last LTS release of the previous release cycle will be kept compatible with the newer file format version, and will be usable as converter between both versions.`
> `For example, Blender 3.6 LTS can open files from Blender 4.x, and will perform the necessary conversion such that when re-saved from 3.6, the files become compatible with all 3.x Blender versions.`

Advice for wide gaps:
> `When having issues with opening much older (or newer) blend-files, it can help to use a few intermediary Blender releases to perform conversions by smaller steps.`

There is a separate, older stumbling block about compression:
> `Changed in version 3.0: Prior to this version, the compression algorithm used was Gzip. This means to open newer blend-files in versions prior to 3.0, blend-files must first be saved without compression in a newer version of Blender`

The file itself carries a version, and Blender exposes it:
> `BlendData.version` / `File format version the .blend file was saved with (array of 3 items, in [0, inf], default (0, 0, 0), readonly)`
> `bpy.app.version_file` / `The Blender File version, as a tuple of 3 numbers (major, minor, file sub-version), that will be used to save a .blend file. The last item in this tuple indicates the file sub-version, which is different from the release micro version (the last item of the bpy.app.version tuple). The file sub-version can be incremented multiple times while a Blender version is under development. This value is, and should be, used for handling compatibility changes between Blender versions`

Cycles shows the machinery in action. Its own file registers a handler that runs on file load and rewrites settings according to the version the file was saved with — and does it per linked library, since different libraries in one file may come from different versions:
> ```
> @persistent
> def do_versions(self):
> ```
> ```
>     # Map of versions used by libraries.
>     library_versions = {}
>     library_versions[bpy.data.version] = [None]
>     for library in bpy.data.libraries:
>         library_versions.setdefault(library.version, []).append(library)
>
>     # Do versioning per library, since they might have different versions.
>     max_need_versioning = (5, 2, 8)
>     for version, libraries in library_versions.items():
>         if version > max_need_versioning:
>             continue
> ```
with, for example, a rule that reaches back to the version before this one:
> ```
>             # Auto tiling is always enabled now
>             if version <= (5, 0, 77):
>                 cscene = scene.cycles
>                 if not cscene.use_auto_tile:
>                     cscene.use_auto_tile = True
>                     cscene.tile_size = 8192
> ```
and rules going back as far as `version <= (2, 70, 0)`. The handler is registered at Cycles' startup (`bpy.app.handlers.version_update.append(version_update.do_versions)`, `__init__.py`).

The 5.2 release notes state specific conversions performed on older files, and specific breaks:
> `The Random Walk radius scaling has been adjusted to match other renderers better (blender/blender!133848). To keep compatibility, older files are automatically converted to use Random Walk (Legacy) mode.`
> `Similar to 5.1, asset files with Geometry Nodes tools must be re-saved to be used properly in 5.2`
> `The python API for paint.eraser_brush and paint.eraser_brush_asset_reference is removed as it is no longer used`
> `Socket identifiers for the Compare and Random Value node changed`

**Label.** documented (the manual and the release notes); observed (Cycles' own file, for how a version-dependent conversion is actually carried out and that the file records a version per library).

**Not covered.** Nothing here says what happens to a *render* when a file is silently converted — whether the converted result matches what the older Blender produced. Nothing says whether the forward-compatibility warning appears at all when there is no window (see below). Nothing gives a version floor for 5.2 specifically; the "minimal version" message is described in general terms only.

**Sources.**
- Manual, https://docs.blender.org/manual/en/5.2/files/blend/compatibility.html — read 2026-07-30
- Manual, https://docs.blender.org/manual/en/5.2/files/blend/open_save.html — read 2026-07-30
- API documentation, https://docs.blender.org/api/5.2/bpy.types.BlendData.html — read 2026-07-30
- API documentation, https://docs.blender.org/api/5.2/bpy.app.html — read 2026-07-30
- Release notes, https://developer.blender.org/docs/release_notes/5.2/rendering/ — read 2026-07-30
- Release notes, https://developer.blender.org/docs/release_notes/5.2/ — read 2026-07-30
- Cycles' own file, `Blender Sources/cycles_5.2/version_update.py`, `do_versions` at line 67, library-version mapping at lines 96–105, first rule at lines 111–116 — read 2026-07-30

**Outside the interface.** One documented difference, and it matters here: the forward-compatibility warning is an interface thing. "A warning is shown in the UI when editing a more recent blend-file. Trying to overwrite it (with a simple 'Save' operation) will also show a confirmation popup". The sources do not say what, if anything, appears when Blender is driven from outside its window — so a newer-than-expected file rendering unattended may lose data with no visible warning. That is the plain reading of "shown in the UI", but the silent-case behaviour is not stated, so I record it as an unanswered edge rather than a finding. The conversion itself is not tied to the interface: Cycles' handler runs on file load and its only interface-related guard is `if not bpy.data.is_saved: return`. **Observed 2026-08-05 (hand test 22, below):** it is not silent — a warning is printed to the terminal, and the render goes ahead.

**Operating system.** The sources do not say, and nothing in the compatibility material names an operating system. (The gzip note is about a Blender version, not a system.) The test below was run on macOS only. **Observed 2026-08-06:** the same test was run again on a rented Linux machine and came back the same, so this holds on macOS and on Linux. Windows stays untested.

### Observed — hand test 22 (2026-08-05)

Run from a terminal on the Creator's own computer — a MacBook Pro with an M1 Pro chip, macOS 26.5.2 — with the `BLENDER_USER_RESOURCES` setting pointed at a fresh empty folder. Label: **observed**.

A file was saved from factory startup by Blender 5.2.0 (Workbench, 64 by 64) and then opened and rendered with no window by each of the three older Blenders on the machine:

- **Blender 5.1.2:** `00:00.328  reports          | WARNING File written by newer Blender binary (502.44), expect loss of data!` — then rendered and saved, exit code 0.
- **Blender 5.0.1:** `00:00.269  reports          | WARNING File written by newer Blender binary (502.44), expect loss of data!` — then rendered and saved, exit code 0.
- **Blender 4.5.12:** `Warning: File written by newer Blender binary (502.44), expect loss of data!`, plus several lines of the form `Warning: region type 11 missing in space type "Action" (id: 12) - removing region` and one reading `regiondata free error` — then rendered and saved, exit code 0.

Going the other way, a file saved by Blender 4.5.12 was opened and rendered by Blender 5.2.0 with no window; no warning of any kind was printed and the image was written, exit code 0.

**This closes the gap this record left open.** The forward-compatibility warning does reach a terminal, in those exact words, and none of the three older Blenders refused the file.

### Observed on the rented Linux machine — Linux hand tests 24 (2026-08-06)

Run over SSH from a terminal on the rented Linux machine, using Blender 5.2.0 at `/opt/blender/blender-5.2.0-linux-x64/blender`, with `BLENDER_USER_RESOURCES` pointed at `/root/smoozy/Settings/res-5.2`, a folder of this topic's own that already existed. That folder was checked before use and reported its own config directory and Blender's eight default add-ons and nothing else. The machine is a container sharing someone else's hardware, and up to four topics were being worked on it at once, so nothing below claims anything about how long a run took or how much memory was free. Label: **observed**.

These are a second machine's answers to the tests the Creator's Mac answered on 2026-08-05. The two are never merged: where they agree it says so, and where they differ it says that plainly. The full list is `Audits/Blender Research/Settings/Hand tests — Linux.md`.

**Linux hand test 24 — opening a file across versions with no window.** A factory-startup file saved by Blender 5.2 was opened and rendered by 4.5, 5.0 and 5.1, and a factory-startup file saved by 4.5 was opened and rendered by 5.2. In this version:

Blender 5.2 was given a file saved by 4.5, which is the older direction:

```
00:00.504  blend            | Read blend: "…/from-4.5.blend"
00:00.523  render           | Rendering single frame
00:00.523  render           | Start rendering: Scene, ViewLayer
00:00.523  render           | Engine: EEVEE
00:02.323  render           | Saved: '…/out/b_0001.png'
Blender quit
```

Exit code 0, image written, and **no warning of any kind**. **The two machines agree.**

Blender 5.2 could not be given a newer file to open, here as on the Mac, because no newer Blender exists to write one.

So the warning the manual describes as shown "in the interface" reaches a terminal on Linux as it does on macOS, and no older Blender refused the file.

---

## 11. Can a file rely on scripts stored inside it to render correctly, and what decides whether those scripts run when a render is started from outside Blender?

**Answer.** Yes, a file can depend on scripts it carries, and the manual names the ways. Whether they run is decided by a preference, overridable per file when opening through the file browser, and overridable at launch by a command-line argument — and the default is off.

What a file can carry:
> `The ability to include Python scripts within blend-files is valuable for advanced tasks such as rigging and automation. However, it poses a security risk since Python does not restrict what a script can do. Therefore, you should only run scripts from sources you know and trust. Automatic execution is disabled by default; however, some blend-files need this to function properly.`

The routes that run automatically:
> `Registered Text-Blocks` / `A text data-block can have its Register option enabled which means it will load on start.`
> `Animation Drivers` / `Python expressions can be used to Drive values and are often used in more advanced rigs and animations.`

And two that run without the automatic-execution permission at all — the second of which is a rendering feature:
> `There are other ways scripts in a blend-file may execute that require user interaction (therefore will run even when auto execution is off), but you should be aware that this is the case since it is not necessarily obvious.`
> `Running a script in the Text editor.`
> `Rendering with Freestyle, because Freestyle uses scripts to control line styles.`

What decides:
> `First, the File Browser has the option Trusted Source which you can use on a case-by-case basis to control auto execution. Since you may forget to set this, or may open a file without going through the File Browser, you can change the default (described next).`
> `In the Preferences, there is the toggle to Auto Run Python Scripts. This means the Trusted Source option in the File Browser will be enabled by default, and scripts can run when blend-files are loaded without using the File Browser. Once enabled you have the option to exclude certain directories; a typical configuration would be to trust all paths except for the download directory.`

And from outside Blender's window:
> `You may want to perform batch rendering or some other task from the command line, running Blender without an interface. In this case, the Preferences are still used but you may want to override them:`
> `Enable with -y or --enable-autoexec`
> `Disable with -Y or --disable-autoexec`
> `To render an animation in background mode, allowing drivers and other scripts to run:`
> `blender --background --enable-autoexec my_movie.blend --render-anim`

The command-line list states the default explicitly:
> `-Y, --disable-autoexec` / `Disable automatic Python script execution (Python-drivers & startup scripts), (default).`

**Label.** documented.

**Not covered.** The manual does not say what a render *looks like* when a needed script is blocked — only that "some blend-files need this to function properly". It does not say whether the blocked-scripts dialog ("a dialog will appear. In it, you can choose to Allow Execution or to Ignore the scripts") has any equivalent with no window open, nor whether such a render fails or silently produces the wrong picture. It does not say whether the directory exclusion list applies when driven from the command line.

**Sources.**
- Manual, https://docs.blender.org/manual/en/5.2/advanced/scripting/security.html — read 2026-07-30
- Manual, https://docs.blender.org/manual/en/5.2/advanced/command_line/arguments.html — read 2026-07-30
- Manual picture, `_images/advanced_scripting_security_autorun-scripts-dialog.png`, named on the security page as "An Auto-run warning in the Info editor's header." I did not need to open it; the surrounding text carries the claim.

**Outside the interface.** This is the heart of the answer and it is documented. Outside the interface the preference still applies, the file-browser "Trusted Source" route is unavailable (there is no file browser), and `-y` / `-Y` are the levers. The manual also notes the arguments are not background-only: "These command-line arguments can be used to start a regular Blender instance and will still override the Preferences." **Observed 2026-08-05 (hand test 20, below):** with neither flag the preference is followed; a blocked driver prints a line and yields zero, and the render still goes ahead.

**Operating system.** The sources do not say; nothing on the security page or in the relevant arguments names an operating system. The test below was run on macOS only. **Observed 2026-08-06:** the same test was run again on a rented Linux machine and gave the same six results, so this holds on macOS and on Linux. Windows stays untested.

### Observed — hand test 20 (2026-08-05)

Run from a terminal on the Creator's own computer — a MacBook Pro with an M1 Pro chip, macOS 26.5.2 — using Blender 5.2.0 at `/Applications/Blender.app/Contents/MacOS/Blender`, with the `BLENDER_USER_RESOURCES` setting pointed at two fresh empty folders, one with Auto Run Python Scripts off and one with it on, so the Creator's own Blender settings were untouched. Label: **observed**.

A file was saved from factory startup with a scripted-expression driver on the default cube's x position. Three expressions were tried in turn and the driven value read back after `frame_set(1)`:

```
expression: len("xxxxx")                       no flag -> CUBE X = 5.0   -y -> 5.0   -Y -> 5.0
expression: sum([3.0,4.0])                      no flag -> CUBE X = 7.0   -y -> 7.0   -Y -> 7.0
expression: __import__("os").getpid()*0+7       no flag -> CUBE X = 0.0   -y -> 7.0   -Y -> 0.0
```

The blocked case printed:

```
BPY_driver_exec: restricted access disallows name '__import__', enable auto-execution to support
```

The third expression was then run against both settings folders:

```
Auto-Run pref=off flag=none -> CUBE X = 0.0   BPY_driver_exec: restricted access disallows name '__import__', enable auto-execution to support
Auto-Run pref=off flag=-y   -> CUBE X = 7.0
Auto-Run pref=off flag=-Y   -> CUBE X = 0.0   BPY_driver_exec: restricted access disallows name '__import__', enable auto-execution to support
Auto-Run pref=on  flag=none -> CUBE X = 7.0
Auto-Run pref=on  flag=-y   -> CUBE X = 7.0
Auto-Run pref=on  flag=-Y   -> CUBE X = 0.0   BPY_driver_exec: restricted access disallows name '__import__', enable auto-execution to support
```

**This closes two of the "not covered" items above.** What a blocked script costs in practice: the driver supplies 0 instead of its value, so the picture comes out wrong; the render does not fail and the image is still written; and it is not silent — the `BPY_driver_exec:` line goes to the terminal, which is where a dialog would have gone if there were a window. With neither `-y` nor `-Y`, the preference is what decides.

### Observed on the rented Linux machine — Linux hand tests 22 and 2 (2026-08-06)

Run over SSH from a terminal on the rented Linux machine, using Blender 5.2.0 at `/opt/blender/blender-5.2.0-linux-x64/blender`, with `BLENDER_USER_RESOURCES` pointed at `/root/smoozy/Settings/res-5.2`, a folder of this topic's own that already existed. That folder was checked before use and reported its own config directory and Blender's eight default add-ons and nothing else. The machine is a container sharing someone else's hardware, and up to four topics were being worked on it at once, so nothing below claims anything about how long a run took or how much memory was free. Label: **observed**.

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

**Answer.** Yes, by name, with a dedicated argument:
> `-S, --scene <name>` / `Set the active scene <name> for rendering.`

The manual's worked examples put it alongside the other render arguments and stress the ordering rule (see question 1) — "See Command Line Arguments for a full list of arguments (for example to specify which scene to render, the end frame number, etc.)".

When it is not told which, the closest the sources come to a direct statement is the render operator, whose scene parameter documents the fallback:
> `bpy.ops.render.render(*, animation=False, write_still=False, use_viewport=False, use_sequencer_scene=False, layer='', scene='', frame_start=0, frame_end=0)`
> `scene (str) – Scene, Scene to render, current scene if not specified (optional, never None)`

and the same operator carries the sibling options:
> `layer (str) – Render Layer, Single render layer to re-render (used only when animation is disabled) (optional, never None)`
> `use_sequencer_scene (bool) – Use Sequencer Scene, Render the sequencer scene instead of the active scene (optional)`
> `frame_start (int) – Start Frame, Frame to start rendering animation at. If not specified, the scene start frame will be assumed.`

The interface has a scene selector beside a view-layer selector (picture `_images/interface_window-system_topbar_scenes-layers.png`, which shows a data-block field reading "Scene" and, beside it, one reading "View Layer"). The Python data describes `Window.scene` as "Active scene to be edited in the window (never None)". Neither record establishes that this is a scene saved in the file or a command-line default.

The sources establish only that the render operator falls back to the current scene when its own scene argument is omitted. I read the whole command-line argument list and its render section, and the cited API operator entry. None states that the command line with no `-S` uses that fallback, or that the current scene is a scene saved in the file. The earlier conclusion that it renders the file's active scene went beyond the recorded words.

One complication the sources do raise. Blender 5.2 has a Sequencer Scene that may differ from the active scene, and in the interface the choice is offered as two separate menu entries:
> `Render Sequencer Image Alt-F12` / `If a Sequencer Scene exists that differs from the active scene, render that scene instead.`
> `Render Sequencer Animation Ctrl-Alt-F12` / `Render the animation of the Sequencer Scene.`
> `When the active scene is different from the Sequencer Scene in the current workspace, the "Render" menu will include separate options to render the Sequencer Scene (and not the active Scene).`

The render operator exposes this as `use_sequencer_scene`. The command-line argument list contains no equivalent — I read the whole page and searched it for "sequencer"; there is no hit. So which of the two an unattended `-a` or `-f` picks is **not in the sources**.

**Label.** documented (that `-S` exists and names a scene; the operator's fallback and its sequencer option); not in the sources (what happens from the command line when no scene is named, including when a Sequencer Scene differs from the active scene).

**Not covered.** Whether `-S` with a name that no scene has fails, warns, or silently renders something else. Whether `-S` accepts more than one name. Whether the scene selected by `-S` is remembered if the file is later saved. Whether the frame range used is the named scene's own.

**Sources.**
- Manual, https://docs.blender.org/manual/en/5.2/advanced/command_line/arguments.html — read 2026-07-30
- Manual, https://docs.blender.org/manual/en/5.2/advanced/command_line/render.html — read 2026-07-30
- Manual, https://docs.blender.org/manual/en/5.2/interface/window_system/topbar.html — read 2026-07-30
- Manual, https://docs.blender.org/manual/en/5.2/video_editing/sequencer_scene.html — read 2026-07-30
- Manual picture, `_images/interface_window-system_topbar_scenes-layers.png`, shown on the topbar page — read 2026-07-30. What is visible: a data-block selector holding the name "Scene" with a clear (×) button, and beside it a second data-block selector holding the name "View Layer" with duplicate and remove buttons. That is all I take from it.
- API documentation, https://docs.blender.org/api/5.2/bpy.ops.render.html — read 2026-07-30
- API documentation, https://docs.blender.org/api/5.2/bpy.types.Window.html — read 2026-07-30

**Outside the interface.** This question is about outside the interface, and `-S` exists only there. The gap is the Sequencer Scene: inside the interface it is offered as separate Render-menu entries and as an operator flag; outside, there is no argument for it and the sources do not say which scene wins.

**Operating system.** The sources do not say; `-S` carries no operating-system note, unlike several arguments on the same page that do. The tests below were run on macOS only. **Observed 2026-08-06:** the same tests were run again on a rented Linux machine and came back the same, so this holds on macOS and on Linux. Windows stays untested.

### Observed — hand tests 15 and 16 (2026-08-05)

Both were run from a terminal on the Creator's own computer — a MacBook Pro with an M1 Pro chip, macOS 26.5.2 — using Blender 5.2.0 at `/Applications/Blender.app/Contents/MacOS/Blender`, with the `BLENDER_USER_RESOURCES` setting pointed at a fresh empty folder. Label: **observed**.

**Hand test 15 — which scene renders when none is named, and what a wrong name does.** A file was saved from factory startup with two scenes, `Alpha` (the default cube, camera and light) and `Beta` (a camera only), both Workbench at 64 by 64, with `Beta` made the window's scene before saving; the build step printed `MADE scenes= ['Alpha', 'Beta'] window scene= Beta`. Each render was run with a handler added at launch that names the scene the render starts on:

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

**This closes the "not in the sources" answer above and two of the "not covered" items.** A render started from outside with no `-S` uses the scene that was showing when the file was saved. A name no scene bears is reported and then ignored — it neither stops the render nor changes which scene is used.

**Hand test 16 — the Sequencer Scene.** A third scene `Edit` was added, its sequencer holding a Scene strip referencing `Alpha`; the workspace's sequencer scene was set to `Edit` and the window's scene left as `Beta`; the build step printed `MADE workspace Layout sequencer_scene= Edit` and `MADE window scene= Beta`. A bare `blender -b test.blend -o out/s_ -F PNG -f 1` printed:

```
RENDER_PRE SCENE: Beta
```

**This closes the other "not in the sources" answer above.** With no `-S`, an unattended render picks the active scene; the Sequencer Scene has no effect on it, and there is no argument that would make it.

**Not settled by these tests.** The files were built and saved by a Blender with no window, using `bpy.context.window.scene` and `workspace.sequencer_scene`. Whether a file saved from the interface records the active scene the same way was not tested.

### Observed on the rented Linux machine — Linux hand tests 17 and 18 (2026-08-06)

Run over SSH from a terminal on the rented Linux machine, using Blender 5.2.0 at `/opt/blender/blender-5.2.0-linux-x64/blender`, with `BLENDER_USER_RESOURCES` pointed at `/root/smoozy/Settings/res-5.2`, a folder of this topic's own that already existed. That folder was checked before use and reported its own config directory and Blender's eight default add-ons and nothing else. The machine is a container sharing someone else's hardware, and up to four topics were being worked on it at once, so nothing below claims anything about how long a run took or how much memory was free. Label: **observed**.

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

**Answer.** Blender documents three engines that come with it and says third-party renderers are available as add-ons. It also offers `-E help` to list available engines. So the documented list can grow on a given computer by installing and enabling an add-on or extension. The further statement that the list is exactly the engines registered in the running copy is an inference from those records.

What comes with Blender:
> `Blender provides the following render engines:` / `EEVEE` / `Cycles` / `Workbench`
> `Blender includes three render engines with different strengths:` / `EEVEE is a physically based realtime renderer.` / `Cycles is a physically based path tracer.` / `Workbench is designed for layout, modeling and previews.`

What can be added:
> `More renderers from third-party developers are available as add-ons. Each renderer has its own render settings to control render quality and performance.`

How an add-on becomes an engine — a Python class registered with Blender, carrying an internal name:
> `class CustomRenderEngine(bpy.types.RenderEngine):` / `# These three members are used by Blender to set up the RenderEngine; define its internal name, visible name and capabilities.` / `bl_idname = "CUSTOM"` / `bl_label = "Custom"` / `bl_use_preview = True`

Cycles itself is exactly that — an add-on doing exactly this:
> ```
> bl_info = {
>     "name": "Cycles Render Engine",
> ...
>     "support": 'OFFICIAL',
>     "category": "Render"}
> ```
> ```
> class CyclesRender(bpy.types.RenderEngine):
>     bl_idname = 'CYCLES'
>     bl_label = "Cycles"
> ```

That the list is discovered at run time rather than fixed shows in three places. The command line offers to print it rather than documenting it: `-E, --engine <engine>` / `Specify the render engine.` / `Use -E help to list available engines.` Blender exposes a read-only flag for whether there is more than one: `RenderSettings.has_multiple_engines` / "More than one rendering engine is available (default False, readonly)". And the API documentation for the engine setting itself cannot enumerate the choices — it prints only `Literal['BLENDER_EEVEE']` beside "Engine to use for rendering (default 'BLENDER_EEVEE')", which is fewer than the three engines the manual names.

How the list grows on a machine: add-ons are installed through the Extensions system ("Most of the time you can get add-ons as part of the Extensions system"), which has its own command-line subcommands — `install:` "Install packages.", `install-file:` "Install package from file.", `repo-add:` "Add repository." — and "Render" is one of the tags an extension may carry. Add-ons can also be turned on for a single run:
> `--addons <addon(s)>` / `Comma separated list (no spaces) of add-ons to enable in addition to any default add-ons.`

There is also a documented route for engines built on Hydra, whose delegates are separate from Blender: `HydraRenderEngine`, and `SceneHydra.export_method` — "How to export the Blender scene to the Hydra render engine". The 5.2 release notes note the plumbing changed: "The implementation was updated to the Hydra 2.0 API. OpenUSD provides an abstraction between different Hydra API versions, so existing render delegates should continue to work without changes."

**Label.** documented (the three built-in engines, that third-party renderers arrive as add-ons, `-E help`, `has_multiple_engines`, `--addons`, the extension install commands, Hydra); observed (Cycles' own file, showing that even a bundled engine is an add-on registering a `RenderEngine` subclass with a `bl_idname`); inferred (that the complete list is exactly the engines registered in the running copy).

**Not covered.** The sources do not say what happens to a file whose saved engine is not registered on the machine that opens it — whether Blender falls back to another engine, refuses, or keeps the name. They do not say whether an engine's own settings survive in a file opened without its add-on. They do not say whether `-E` accepts an add-on engine's name when that add-on is not enabled, nor whether `--addons` runs early enough for `-E` to see the engine it enables. And the API documentation's `Literal['BLENDER_EEVEE']` is not explained anywhere, so I have not read a meaning into it beyond "the documentation does not enumerate the choices".

**Sources.**
- Manual, https://docs.blender.org/manual/en/5.2/render/render_engines.html — read 2026-07-30
- Manual, https://docs.blender.org/manual/en/5.2/render/introduction.html — read 2026-07-30
- Manual, https://docs.blender.org/manual/en/5.2/advanced/command_line/arguments.html — read 2026-07-30
- Manual, https://docs.blender.org/manual/en/5.2/advanced/command_line/extension_arguments.html — read 2026-07-30
- Manual, https://docs.blender.org/manual/en/5.2/advanced/extensions/addons.html — read 2026-07-30
- Manual, https://docs.blender.org/manual/en/5.2/advanced/extensions/tags.html — read 2026-07-30
- API documentation, https://docs.blender.org/api/5.2/bpy.types.RenderEngine.html — read 2026-07-30
- API documentation, https://docs.blender.org/api/5.2/bpy.types.RenderSettings.html — read 2026-07-30
- API documentation, https://docs.blender.org/api/5.2/bpy.types.SceneHydra.html — read 2026-07-30
- Release notes, https://developer.blender.org/docs/release_notes/5.2/python_api/ — read 2026-07-30
- Cycles' own file, `Blender Sources/cycles_5.2/__init__.py`, `bl_info` at lines 7–16 and `class CyclesRender` at lines 43–45 — read 2026-07-30

**Outside the interface.** The list is available and growable from outside: `-E help` prints it, `--addons` enables add-ons for a run, and the extension subcommands install them. The manual notes the extension command "implies --background mode" (that is the general rule for `-c, --command`). What is *not* documented is whether an add-on enabled only for a run registers its engine before `-E` is processed — the arguments-are-executed-in-order rule (question 1) makes that a real question, and the sources do not answer it.

**Operating system.** The sources do not say for the engine list as such. The nearest operating-system-bound facts are elsewhere: EEVEE's headless limitation on Windows (question 6) and Cycles' device backends per system (question 9). Nothing states that an engine is absent on a given operating system. The tests below were run on macOS only. **Observed 2026-08-06:** the same tests were run again on a rented Linux machine. The engine identifiers, the unknown-name error and the add-on case all came back the same, so those hold on macOS and on Linux. One thing does **not** match: in Blender 4.5.12, `-E help` and an unknown engine name end abnormally on both machines but not in the same way — `libc++abi: terminating` and exit code 134 on the Mac, and a run that never ends at all on the one-card Linux machine. **Observed 2026-08-06 on a second rented Linux machine — two NVIDIA GeForce RTX 4090s, driver 580.95.05, sixty-one cores, no display server at all:** `-E help` in this version printed `BLENDER_EEVEE`, `BLENDER_WORKBENCH` and `CYCLES` and exited 0. Blender 4.5.12 hung there too, but printed its listing first, where on the one-card machine it printed nothing at all before hanging — so 4.5 ends badly on all three machines and in three different ways, and none of the three is chosen between. Windows stays untested. *(Added by the two-card citation check, 2026-08-06: the third machine's result was in the hand-test record but not here, and the sentence read as though there were only two machines.)*

### Observed — hand tests 1 and 21 (2026-08-05)

Both were run from a terminal on the Creator's own computer — a MacBook Pro with an M1 Pro chip, macOS 26.5.2 — using Blender 5.2.0 at `/Applications/Blender.app/Contents/MacOS/Blender`, with the `BLENDER_USER_RESOURCES` setting pointed at fresh empty folders. Label: **observed**.

**Hand test 1 — what `-E help` prints.** It printed:

```
Blender Engine Listing:
	BLENDER_EEVEE
	BLENDER_WORKBENCH
	CYCLES
```

exit code 0 — three identifiers, not human labels. With a name no engine bears, `-E NOT_A_REAL_ENGINE` printed `Error: engine not found 'NOT_A_REAL_ENGINE'`, exit code 1, and no image was written.

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
00:00.518  reports          | ERROR Engine 'CYCLES' not available for scene 'Scene' (an add-on may need to be installed or enabled)
AFTER engine= 'BLENDER_EEVEE' has_multiple_engines= True engine choices= ['BLENDER_EEVEE']
```

and `blender -E help` in that state listed only `BLENDER_EEVEE` and `BLENDER_WORKBENCH`. Rendering went ahead anyway, exit code 0:

```
00:00.518  render           | Engine: EEVEE
00:00.947  render           | Fra: 1 | Rendering 50 / 64 samples
00:01.006  render           | Fra: 1 | Rendering 64 / 64 samples
00:01.091  render           | Saved: '.../out/n_0001.png'
```

64 samples is EEVEE's own default, not the file's Cycles setting of 4.

**This closes two of the "not covered" items above, and bears out the inference at the head of this answer.** A file whose saved engine is not registered is reported by that exact message; the engine name is not kept — the scene reads back as `BLENDER_EEVEE` — and the render goes ahead on EEVEE rather than refusing. The engine list did shrink when the add-on was switched off, which is the "exactly the engines registered in the running copy" rule seen working.

**What this does not settle.** No third-party engine add-on is installed, so whether `-E` accepts an add-on engine's name before its add-on is enabled, and whether `--addons` runs early enough for `-E` to see it, stay untested.

### Observed — hand test 23, run by the Creator (2026-08-05)

Run by the Creator in Blender's own window, on their own computer — a MacBook Pro with an M1 Pro chip, macOS 26.5.2 — on 2026-08-05. One shot was supplied for 5.0, 5.1 and 5.2 together rather than one each, and the Creator gave it as the answer for all three. That settles macOS and leaves Windows and Linux untested. Label: **observed**.

The open **Render Engine** dropdown in Render Properties holds exactly three entries — **EEVEE**, **Workbench**, **Cycles** — with Cycles selected. So on a Blender with no extra add-on installed, the control offers those three and nothing else, under their human labels rather than the identifiers `-E` takes. That is the same three the printed list gives, against an API page that could enumerate only `Literal['BLENDER_EEVEE']`.

### Observed on the rented Linux machine — Linux hand tests 1, 2, 23 and 25 (2026-08-06)

Run over SSH from a terminal on the rented Linux machine, using Blender 5.2.0 at `/opt/blender/blender-5.2.0-linux-x64/blender`, with `BLENDER_USER_RESOURCES` pointed at `/root/smoozy/Settings/res-5.2`, a folder of this topic's own that already existed. That folder was checked before use and reported its own config directory and Blender's eight default add-ons and nothing else. The machine is a container sharing someone else's hardware, and up to four topics were being worked on it at once, so nothing below claims anything about how long a run took or how much memory was free. Label: **observed**.

These are a second machine's answers to the tests the Creator's Mac answered on 2026-08-05. The two are never merged: where they agree it says so, and where they differ it says that plainly. The full list is `Audits/Blender Research/Settings/Hand tests — Linux.md`.

**Linux hand test 1 — what `-E help` prints, and what `-E` does with a name no engine bears.** `blender -b -E help` printed:

```
Blender 5.2.0 (hash …)
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
Blender 5.2.0 (hash …)
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

## Things a person with Blender 5.2 open could settle in a few minutes

Everything on this list that a terminal could reach was run on the Creator's Mac on 2026-08-05 and written into the answers above, everything that needed eyes on Blender's window was run by the Creator the same day, and every terminal test was then run again on a rented Linux machine on 2026-08-06 and written in beside it. What is left is one gap in what was photographed, one test that needs a machine to itself, and what no machine the project has can reach.

### Still open — needs somebody looking at Blender's window

1. **What the right-click menu on a view layer's "Use for Rendering" offers.** The dot beside it, and that the switch takes a keyframe, are both settled; the menu itself was not among the shots supplied on the Mac, and driving a right-click on one control is not something the Linux run attempted. (Question 1.)

### Not yet run

2. **What `--gpu-device` does.** Blender 5.2 carries this launch argument and `--gpu-device-no-fallback` beside it, and 5.1 and earlier do not; it is quoted in full under question 1. It was not run — the tests here use Cycles' own `--cycles-device`, and its own help text says `--gpu-device` applies only to the Vulkan backend. (Questions 1 and 9.)

### Needs a machine to itself

3. **Whether Cycles' Time Limit is per frame or per job on Linux.** Settled on the Mac by timing a three-frame render against a five-second limit. It could not be re-run on the rented machine, which was shared with three other topics throughout, because the answer can only be read from how long the run takes. The test is written out in `Audits/Blender Research/Settings/Hand tests — Linux.md`. (Question 2.)

### Cannot be run on any machine the project has

4. **Whether EEVEE fails on a headless Windows machine.** The screenless half of this item is now answered: on a second rented Linux machine with **no display server at all** — two NVIDIA GeForce RTX 4090s, driver 580.95.05, sixty-one cores, over SSH — every engine each version has rendered, on the cards and on the processor, EEVEE included, and that result is written into question 6. What is left is the Windows half, which needs a Windows machine, and a computer with no graphics hardware at all. (Question 6.)

5. **What an unregistered third-party engine does, and whether `--addons` arrives in time for `-E`.** Needs a third-party render engine add-on; none is installed on either machine, and installing one is not allowed on either. The nearest case — the Cycles add-on switched off in a throwaway settings folder — was run on both and is recorded under question 13. (Question 13.)

6. **What a file's "GPU Compute" choice means on an AMD or an Intel graphics card.** The NVIDIA case is settled — see question 9 — and asking for `HIP` or `ONEAPI` on a machine with no such card is refused outright. What either does when the card is there needs such a card, and there is none. (Question 9.)

7. **Whether Freestyle's line-style scripts run without `--enable-autoexec`.** The sources do not give the exact settings needed to build a file with a scripted line style, so the test cannot be written out without guessing. A second machine does not help this one. (Question 11.)

8. **Anything about Windows.** The Linux half of every question's operating-system line is now answered. Windows stays untried: no Windows machine has run it.

---

2026-08-04 — Opus 5 (Claude Code) — Settings, Blender 5.2

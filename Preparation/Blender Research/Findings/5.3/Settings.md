# Settings — findings, Blender 5.3

- **Sources consulted:** the Blender 5.3 manual, the Blender 5.3 Python API documentation and the Blender 5.3 release notes (copies taken 2026-10-11), and Cycles' own files for 5.3 (copy taken 2026-10-16), all inside `Blender Sources/`. Work done on 2026-10-16. Dates read below are the snapshot dates, as instructed.
- I read only the question sheet and files inside Blender Sources/. I did not open any other part of this project.

Note for transparency: the tool I run in placed two things in front of me unasked, before I started: the text of the project's `CLAUDE.md` signpost, and a list of one-line titles of the user's saved notes. I did not open either file, did not follow the signpost, and nothing below comes from them. Inside `Blender Sources/` I also read `About this folder.md` (a description of the folder). I did not use the `requirements/` folder, because it is not one of the four sources the brief allows answers to come from.

Labels used: **documented**, **inferred**, **observed**, **not in the sources**, as defined in the brief. Cycles' own files are cited as `cycles_5.3/<file>, line N`. "1 << 24" in a Cycles file is a number written as a binary shift; it equals 16,777,216 (that conversion is arithmetic, so it is inferred, not quoted).

---

## 1. Which settings can be settled at launch from outside, and which only by changing the file?

**The question is ambiguous.** "Settled at the moment a render is started" can mean (a) by the command-line options Blender offers for rendering, or (b) by anything handed in at launch, including a Python script or one-line Python expression passed on the command line. I answer both. Reading (a) looks like the main intent.

**Answer**

| Setting | Command-line option? | By a Python script or expression given at launch? |
|---|---|---|
| Frame range | Yes: `-s` / `--frame-start`, `-e` / `--frame-end` (documented) | Yes (inferred) |
| Interval to skip frames by | Yes: `-j` / `--frame-jump` (documented) | Yes (inferred) |
| List of separate frames | Yes: `-f` / `--render-frame` takes a comma list and `..` ranges (documented) | Not in the sources (see "Not covered") |
| Sample count | No option in the manual's list (not in the sources); Cycles' own extra options are only `--cycles-device` and `--cycles-print-stats` (observed) | Yes (inferred) |
| Whether a view layer renders | No option in the manual's list (not in the sources). The "Render Single Layer" setting is ignored from the command line (documented) | Yes, for the whole render (inferred). Frame by frame: see below |
| Render engine | Yes: `-E` / `--engine` (documented) | Yes (inferred) |
| Resolution percentage | No option in the manual's list (not in the sources) | Yes (inferred) |

- Other render options quoted from the command-line page or Cycles' file: `-S` / `--scene` (documented, quoted in question 12), `--render-output` (documented, in the command-line example below), and, for Cycles, `--cycles-device` (observed).
- Under reading (a): sample count, view-layer on/off and resolution percentage come only from the file unless a script is used.
- Under reading (b): the sources show no setting in the list that can only be changed by editing and re-saving the file. All of them are ordinary writable settings in the API, and the command line can run a script after the file loads and before the render. That a script's change made this way is used by the render without saving the file is **inferred**. The step: the manual says arguments run in order, and shows that a setting given after the file loads and before the render command is used, while one given before the file loads is overwritten.
- **Whether a view layer renders "at a given frame"**: the sources give no option for turning a view layer on or off per frame (not in the sources). The API documents a "frame change" hook that scripts can use to change data for each frame during rendering. So a per-frame change by a script is possible (**inferred**; the hook's text is general and does not name view layers). Whether the "Use for Rendering" checkbox itself can be keyframed is not stated (not in the sources). In the manual's picture of the View Layer panel, the pass checkboxes have the small "decorator" dot, which the manual says marks a property that can be animated, and "Use for Rendering" has none (documented, visible in `_images/render_layers_view-layer_panel.png`). Taking that to mean it cannot be animated is **inferred** from one picture, and the picture may be older than 5.3.

**Label:** documented (the options and picture), observed (Cycles' options), inferred (the script route, the per-frame route, and the picture's implication), not in the sources (command-line options for sample count, view-layer rendering, and resolution percentage; keyframing the view-layer checkbox; a launch script rendering a list of separate frames).

**Quotes**

- "-s, --frame-start <frame> / Set start to frame <frame>, supports +/- for relative frames too." / "-e, --frame-end <frame> / Set end to frame <frame>, supports +/- for relative frames too." / "-j, --frame-jump <frames> / Set number of frames to step forward after each rendered frame." / "-f, --render-frame <frame> / Render frame <frame> and save it. … A comma separated list of frames can also be used (no spaces). A range of frames can be expressed using .. separator between the first and last frames (inclusive)." / "-E, --engine <engine> / Specify the render engine. Use -E help to list available engines." — manual, https://docs.blender.org/manual/en/5.3/advanced/command_line/arguments.html, read 2026-10-11.
- Same page: "-P, --python <filepath> / Run the given Python script file." / "--python-expr <expression> / Run the given expression as a Python script." / "Arguments are executed in the order they are given." / "blender --background --render-output /tmp test.blend --render-frame 1 / …will not render to /tmp because loading the blend-file overwrites the render output that was set." / "blender --background test.blend --render-output /tmp --render-frame 1 / …works as expected." / "Cycles add-on options must be specified following a double dash."
- "use_single_layer / Only render the active layer. Only affects rendering from the interface, ignored for rendering from command line. (default False)" — API, https://docs.blender.org/api/5.3/bpy.types.RenderSettings.html, read 2026-10-11.
- "Render Single Layer / Only render the active view layer. / Note / This option is ignored when rendering from the command line." — manual, https://docs.blender.org/manual/en/5.3/render/layers/view_layer.html, read 2026-10-11. Picture on that page: `_images/render_layers_view-layer_panel.png` (looked at; described above).
- "use / Enable or disable rendering of this View Layer (default True)" — API, https://docs.blender.org/api/5.3/bpy.types.ViewLayer.html, read 2026-10-11.
- "frame_change_pre / Called when a frame change is triggered for playback and rendering, before any data is evaluated for the new frame. This makes it possible to change data and relations (for example swap an object to another mesh) for the new frame." — API, https://docs.blender.org/api/5.3/bpy.app.handlers.html, read 2026-10-11.
- "Decorators may appear next to number fields, menus, and checkboxes to indicate the property can be animated." — manual, https://docs.blender.org/manual/en/5.3/interface/controls/buttons/decorators.html, read 2026-10-11.
- Cycles' own command-line reader defines exactly two options, `parser.add_argument("--cycles-print-stats", …` and `parser.add_argument("--cycles-device", …` — `cycles_5.3/engine.py`, lines 13–21, read 2026-10-16 (observed).

**Where I looked:** read the pages cited in Quotes above (and `bpy.ops.render.html` in question 12); no further search terms or pages were recorded for the items marked not in the sources.

**Not covered:** whether `-j` applies to a comma list given to `-f`; what happens with a frame outside the file's range; whether a launch script can render a list of separate frames itself (the render operator offers a single frame or a start/end range only, see question 12's quote of it); whether the per-view-layer checkbox can be keyframed.

**Outside the interface:** This whole answer is about starting from outside. The command-line options and launch scripts are the outside route. One documented difference: "Render Single Layer" counts inside the interface and is ignored from the command line. **Observed 2026-10-17 (hand tests below, entries 1 to 6, 10, 14 to 16):** from a terminal with no window, the built program has no switch for a sample count, a resolution percentage or a view layer; a Python line handed in after the file settles all three for that run and leaves the file on disk unchanged, while the same line before the file is overwritten by it; `-f` takes lists and ranges and `-j` applies to `-a` only; a view layer's switch can be keyed frame by frame and an animation render honours it; "Render Single Layer" does nothing from outside; and with every view layer switched off nothing is written, one `ERROR` line is printed and the exit status is 0. In Blender's own window "Render Single Layer" refuses a key with `"use_single_layer" property cannot be animated` (entry 61).

**Operating system:** The sources do not say that any of these options differ by operating system. The options page marks some unrelated options as Windows-only or Linux-only, but none of the render options above. **Observed 2026-10-17:** the built program's own help text marks eleven arguments as Windows-only, Linux-only, Windows-and-Linux, Vulkan-only or debug-only, and no render argument among them (entry 1). Every run in this document was on macOS 26.6.2 on Apple silicon; no Linux or Windows machine was available to this sitting, so the observed answer covers macOS alone.

### Observed in Blender 5.3 — hand tests of 2026-10-17

Everything in this section was got by running Blender, not from a source, and sits beside the answer above rather than over it. **On what computer:** the Creator's Mac — a MacBook Pro with an Apple M1 Pro chip and its own 14-core graphics (Metal), 16 GB of memory, macOS 26.6.2. **What ran:** Blender 5.3.0 Beta, built 2026-10-16, at `/Applications/Blender 5.3b.app/Contents/MacOS/Blender`, started from a terminal by Claude Opus 5.5 on 2026-10-17, with a throwaway settings folder and, unless said otherwise, `--factory-startup`, so none of the Creator's own settings was read. Nothing ran on Linux or Windows. The tests are written out in full, with every command and script, in `Audits/Blender Research/Settings/Hand tests — 5.3.md`; their numbers are given here. Label: **observed**.

**The program's own list of launch arguments (entry 1).** `blender --help` was written to a file with no blend-file opened. It is 521 lines long and exited 0. Searching it for `sampl`, `resolution`, `percent`, `view layer`, `view-layer`, `viewlayer` and `python-use` returned exactly:

```
128:	Do not use native pixel size, for high resolution displays (MacBook 'Retina').
164:--python-use-system-env 
166:	This also enables user environment, see: '--python-use-user-env'.
168:--python-use-user-env 
466:	Background mode: Do not systematically build and evaluate ViewLayers' dependency graphs
```

None of them sets a render setting: there is no switch for a sample count, a resolution percentage or a view layer in the built program either. Its Cycles section reads, word for word:

```
Cycles Render Options:
	Cycles add-on options must be specified following a double dash.

--cycles-device <device>
	Set the device used for rendering.
	Valid options are: 'CPU' 'CUDA' 'OPTIX' 'HIP' 'ONEAPI' 'METAL'.

	Append +CPU to a GPU device to render on both CPU and GPU.

	Example:
	# blender -b file.blend -f 20 -- --cycles-device OPTIX
--cycles-print-stats
	Log statistics about render memory and time usage.
```

The lines that name an operating system are these eleven, and no render argument is among them:

```
125:	Start with the console window open (ignored if '-b' is set), (Windows only).
131:	Disable all window decorations (Linux only).
211:	Show a back trace for each log message (debug builds only).
276:	Enable debug messages for Ghost (Linux only).
291:	Enable shader debug info generation (Vulkan only).
436:	Print this help text and exit (Windows only).
439:	Register blend-file extension for current user, then exit (Windows & Linux only).
442:	Register blend-file extension for all users, then exit (Windows & Linux only).
445:	Unregister blend-file extension for current user, then exit (Windows & Linux only).
448:	Unregister blend-file extension for all users, then exit (Windows & Linux only).
451:	Set the Quality of Service (QoS) mode for hybrid CPU architectures (Windows only).
```

`blender -c help` listed three sub-commands, `asset_listing`, `extension` and `maketx`, none of which names a render setting. The `--gpu-device` argument is present; its text is quoted under question 9.

**The engine argument with a name no engine bears (entry 2).** On Blender's default scene saved on Workbench at 64 by 64, `blender -b --factory-startup wb64.blend -E NOT_A_REAL_ENGINE -o <folder>/e_ -F PNG -f 1` printed:

```
00:00.319  blend            | Read blend: "/private/tmp/claude-501/smoozy-hand-tests-5.3/Settings/files/wb64.blend"

Error: engine not found 'NOT_A_REAL_ENGINE'
Blender 5.3.0 Beta (hash 5d8151335b6b built 2026-10-16 01:36:01)
```

exit status 1, and the output folder stayed empty. The engine listing itself is under question 13.

**A launch switch nobody has (entry 3).** The default scene, Cycles on the processor at 64 by 48 with every sampling setting at its default, saved as `cyc64.blend`. Handed `--render-percentage 50` — the shape a switch for the resolution percentage would take — after the file:

```
00:00.316  blend            | Read blend: "/private/tmp/claude-501/smoozy-hand-tests-5.3/Settings/files/cyc64.blend"
unknown argument, loading as file: --render-percentage
00:00.321  reports          | ERROR Cannot read file "/private/tmp/claude-501/smoozy-hand-tests-5.3/Settings/--render-percentage": No such file or directory
Blender 5.3.0 Beta (hash 5d8151335b6b built 2026-10-16 01:36:01)

Blender quit
```

exit status 1 and no picture: the unknown switch is taken for a file name and the run stops before the frame. A guessed Cycles switch, `-- --cycles-samples 8` placed after `-f 1`, changed nothing and drew no complaint — the render counted towards the default and saved its picture, exit status 0:

```
00:00.330  render           | Fra: 1 | Mem: 1M | Sample 0/4096
00:00.330  render           | Fra: 1 | Remaining: 00:01.08 | Mem: 1M | Sample 1/4096
…
00:00.394  render           | Fra: 1 | Remaining: 00:00.71 | Mem: 1M | Sample 336/4096
00:00.462  render           | Saved: '/private/tmp/claude-501/smoozy-hand-tests-5.3/Settings/out/e03/g_0001.png'
```

(the cut lines are seventeen more sample lines of the same form, 80 to 320). The control shows switches after the double dash are read at all: with `-- --cycles-print-stats` in the same place, 7 lines mentioning statistics were printed. `-- --help` printed no Cycles listing of any kind — only the file-read line, the version line and `Blender quit`.

**A list of frames, a range of frames, and the frame jump (entry 4).** Blender's default scene on Workbench at 64 by 64. Five renders, each into its own empty folder; all five exited 0, and their logs, searched for `error` and `warning`, held neither. What each folder held afterwards:

```
blender -b wb64.blend -o list/f_  -F PNG -f 1,5,9            -> f_0001.png f_0005.png f_0009.png
blender -b wb64.blend -o range/f_ -F PNG -f 3..6             -> f_0003.png f_0004.png f_0005.png f_0006.png
blender -b wb64.blend -o mixed/f_ -F PNG -f 1,5..7,12        -> f_0001.png f_0005.png f_0006.png f_0007.png f_0012.png
blender -b wb64.blend -j 3 -o jumpf/f_ -F PNG -f 1..10       -> f_0001.png … f_0010.png, all ten
blender -b wb64.blend -j 3 -s 1 -e 10 -o jumpa/f_ -F PNG -a  -> f_0001.png f_0004.png f_0007.png f_0010.png
```

(each launch also carried `--factory-startup`, and the output paths were absolute). So `-f` takes a comma list, a `..` range and the two mixed in one argument, and `-j` is applied to `-a` and not to a range given to `-f`.

**Whether launch arguments change the file on disk (entry 5).** Blender's default scene saved with its engine (`BLENDER_EEVEE`), frame range 1 to 250, frame step 1 and 100 per cent left alone, and only the picture size (64 by 48) and Cycles' count (4) made small. Its modification time and size, read with `stat -f "%m %z"`, were `1792196017 95789` before and after this launch, which printed `Engine: Cycles`, saved `f_0005.png` and exited 0:

```
blender -b --factory-startup args.blend -E CYCLES -s 5 -e 7 -j 2 -o <folder>/f_ -F PNG -f 5
```

A fresh Blender then read the file back:

```
READ s.frame_start = 1
READ s.frame_end = 250
READ s.frame_step = 1
READ s.render.resolution_percentage = 100
READ s.render.engine = 'BLENDER_EEVEE'
```

The arguments apply to that run only; the file on disk is left unchanged.

**Settings handed in by a Python line at launch, after the file and before it (entry 6).** A file saved with Cycles on the processor, 512 samples, two view layers (`ViewLayer` and `Layer2`) and 200 by 100 at 100 per cent. The same line was handed in with `--python-expr` twice:

```
import bpy; s=bpy.context.scene; s.cycles.samples=8; s.render.resolution_percentage=25; L=s.view_layers.get('Layer2'); (setattr(L,'use',False) if L else None); print('SET', s.cycles.samples, s.render.resolution_percentage, (L.use if L else 'no Layer2 in this scene'))
```

**After the file** (`blender -b --factory-startup twolayer.blend --python-expr "…" -o … -F PNG -f 1`), exit status 0:

```
00:00.314  render           | Start rendering: Scene, ViewLayer
00:00.320  render           | Fra: 1 | Mem: 1M | Sample 0/8
00:00.320  render           | Fra: 1 | Remaining: 00:00.00 | Mem: 1M | Sample 1/8
00:00.367  render           | Fra: 1 | Mem: 1M | Sample 8/8
00:00.427  render           | Saved: '/private/tmp/claude-501/smoozy-hand-tests-5.3/Settings/out/e06/after_0001.png'
SET 8 25 False
```

and the picture measured `pixelWidth: 50  pixelHeight: 25`. **Before the file** (`blender -b --factory-startup --python-expr "…" twolayer.blend -o … -F PNG -f 1`), exit status 0, the last count reached by each layer kept:

```
00:00.320  render           | Start rendering: Scene, ViewLayer
00:00.503  render           | Fra: 1 | Mem: 2M | Sample 512/512
00:00.503  render           | Start rendering: Scene, Layer2
00:00.668  render           | Fra: 1 | Mem: 2M | Sample 512/512
00:00.730  render           | Saved: '/private/tmp/claude-501/smoozy-hand-tests-5.3/Settings/out/e06/before3_0001.png'
SET 8 25 no Layer2 in this scene
```

and the picture measured `pixelWidth: 200  pixelHeight: 100`. (The `SET` line is printed last because Blender's own lines reach the terminal first; it was printed when the line ran. Before the file, the factory scene has no `Layer2`, which is why that run says so.) So a Python line at launch settles the sample count, the resolution percentage and whether a view layer renders — after the file, and only after it: before it, it reads back the values it set and the loaded file then replaces all three.

**A view layer's switch, and "Render Single Layer", in the file Blender writes (entry 10).** Blender's default scene on Workbench at 64 by 48 with a second view layer, `Second`, rendered three times to multi-layer OpenEXR; the channel names were then read out of each file's header.

```
both on:                 Start rendering: Scene, ViewLayer / Start rendering: Scene, Second
                         READBACK [('ViewLayer', True), ('Second', True)]
                         EXR both_0001.exr parts 2 channels ['ViewLayer.Combined.A', 'ViewLayer.Combined.B', 'ViewLayer.Combined.G', 'ViewLayer.Combined.R', 'Second.Combined.A', 'Second.Combined.B', 'Second.Combined.G', 'Second.Combined.R']
Second.use = False:      Start rendering: Scene, ViewLayer
                         READBACK [('ViewLayer', True), ('Second', False)]
                         EXR off_0001.exr parts 1 channels ['ViewLayer.Combined.A', 'ViewLayer.Combined.B', 'ViewLayer.Combined.G', 'ViewLayer.Combined.R']
use_single_layer = True: Start rendering: Scene, ViewLayer / Start rendering: Scene, Second
                         READBACK use_single_layer True [('ViewLayer', True), ('Second', True)]
                         EXR single_0001.exr parts 2 channels ['ViewLayer.Combined.A', 'ViewLayer.Combined.B', 'ViewLayer.Combined.G', 'ViewLayer.Combined.R', 'Second.Combined.A', 'Second.Combined.B', 'Second.Combined.G', 'Second.Combined.R']
```

So `use` on a view layer decides whether it renders when the render is started from outside, and "Render Single Layer", set to `True` and reading back `True`, does nothing there: both layers still render and both are in the file.

**Which of these take a keyframe (entry 14).** Asked of a factory-startup Blender with no window, `keyframe_insert` on a view layer's `use` returned `True`; on "Render Single Layer" it answered `ERROR TypeError bpy_struct.keyframe_insert() property "use_single_layer" not animatable`. The whole list is under question 5.

**The panel, photographed (entry 60).** In Blender's own window, View Layer Properties ▸ View Layer holds `Use for Rendering`, ticked, with a small round dot at the far right of its row, and `Render Single Layer`, unticked, with no dot (written up under question 17).

**Pressing I over each of the two, in Blender's own window (entry 61).** Blender 5.3 was opened with its window from the terminal (a throwaway settings folder, splash and tooltips off), the Properties editor enlarged and turned to View Layer, and the pointer moved over each checkbox in turn by simulated input, then the I key pressed there by simulated input — what a person does to insert a keyframe — and the window photographed with Blender's own screenshot command. Over **`Use for Rendering`**: the checkbox turned from blue to yellow-olive, still ticked, and the round dot at the right of its row became a diamond. Over **`Render Single Layer`**: the checkbox did not change, and the status bar at the foot of the window showed a warning sign and the words **`"use_single_layer" property cannot be animated`**, still there a second later.

**The right-click menu on "Use for Rendering" (entry 62).** In the same way, with View Layer Properties showing, the right mouse button was pressed over the `Use for Rendering` checkbox; the photographed menu is headed **Use for Rendering** and lists, in order: `Insert Keyframe` (I), `Add Driver...` (⌘ D), `Open Drivers Editor`, `Add to Keying Set`, `Remove from Keying Set` (⌥ K), `Reset to Default Value` (Backspace), `Copy to Selected` (greyed), `Copy Data Path` (⇧ ⌘ C), `Copy Full Data Path` (⇧ ⌥ ⌘ C), `Copy as New Driver`, `Add to Quick Favorites`, `Assign Shortcut...`, `Online Manual` (F1).

(The first attempt pressed I in the same instant the pointer arrived; the key went to the control the pointer had been over before, a pass checkbox lower down, which took a key instead. The run was repeated with the pointer moved first and the key pressed 0.6 seconds later, and that is the run described.)

**A view layer switched on at frame 1 and off at frame 2, rendered from outside (entry 15).** Blender's default scene on Workbench at 64 by 64 with a second view layer `Layer2`, whose `use` was keyed `True` at frame 1 and `False` at frame 2; the build printed `BUILD Layer2.use at f1 = True` and `BUILD Layer2.use at f2 = False`. Rendered with `blender -b --factory-startup keyuse.blend -o …/f_ -F OPEN_EXR_MULTILAYER -s 1 -e 2 -a`:

```
00:00.310  render           | Rendering frame 1
00:00.310  render           | Start rendering: Scene, ViewLayer
00:00.411  render           | Start rendering: Scene, Layer2
00:00.416  render           | Saved: '/private/tmp/claude-501/smoozy-hand-tests-5.3/Settings/out/e15/f_0001.exr'
00:00.416  render           | Rendering frame 2
00:00.416  render           | Start rendering: Scene, ViewLayer
00:00.421  render           | Saved: '/private/tmp/claude-501/smoozy-hand-tests-5.3/Settings/out/e15/f_0002.exr'
```

and the channels in the two files:

```
EXR f_0001.exr parts 2 channels ['ViewLayer.Combined.A', 'ViewLayer.Combined.B', 'ViewLayer.Combined.G', 'ViewLayer.Combined.R', 'Layer2.Combined.A', 'Layer2.Combined.B', 'Layer2.Combined.G', 'Layer2.Combined.R']   (8363 bytes)
EXR f_0002.exr parts 1 channels ['ViewLayer.Combined.A', 'ViewLayer.Combined.B', 'ViewLayer.Combined.G', 'ViewLayer.Combined.R']   (4323 bytes)
```

So whether a view layer renders can be switched frame by frame, and an animation render started from outside honours it. This answers the "at a given frame" reading of the question by exercise.

**Every view layer switched off (entry 16).** Blender's default scene on Workbench at 64 by 64 with a second view layer, both layers' `use` set to `False` and saved; a fresh Blender read them back as `[('ViewLayer', False), ('Layer2', False)]`. Rendering one frame, `blender -b --factory-startup alloff.blend -o …/f_ -F PNG -f 1`, printed in full:

```
00:00.318  blend            | Read blend: "/private/tmp/claude-501/smoozy-hand-tests-5.3/Settings/files/alloff.blend"
00:00.322  reports          | ERROR All render layers are disabled
Blender 5.3.0 Beta (hash 5d8151335b6b built 2026-10-16 01:36:01)

Blender quit
```

The exit status was **0**, and the output folder stayed empty. So with every view layer switched off nothing is rendered or written, Blender says so in one `ERROR` line, and the exit status does not show it.

---

## 2. Can a render stop once the picture is clean enough, or after a length of time instead of a count?

**Answer**

- **Clean enough — Cycles: yes, per area of the picture.** With "Noise Threshold" turned on, Cycles stops sampling areas that have become less noisy than the threshold, deciding whether to continue sampling a pixel (documented). It is on by default (observed: default `True`), and Cycles' own kernel file carries a comment describing a per-pixel convergence check (observed). That the whole frame then finishes before the full count once every pixel has passed is **inferred** from this, not stated.
- **Clean enough — EEVEE and Workbench:** no such setting found (not in the sources).
- **Time limit — Cycles: yes** (documented). **EEVEE: yes** (documented; the API change log lists it as added between 5.2 and 5.3). **Workbench:** none found (not in the sources).
- In both engines the time limit counts render time only, not the processing before it (documented).

**Label:** documented (both settings, both engines, per-pixel check); observed (Cycles defaults, kernel comment); inferred (whole-frame early finish); not in the sources (clean-enough setting in EEVEE and Workbench; time limit in Workbench; time-limit unit and scope).

**Quotes**

- "Time Limit / Stops rendering if the time exceeds the limit, even if the desired sample count hasn’t been reached yet. Setting this value to zero disables the limit. / Note / The time limit does not include pre-render processing time, only render time." / "Adaptive Sampling / If the Noise Threshold checkbox is enabled, Cycles will use adaptive sampling, cutting short the sampling process in areas that have become less noisy than the specified threshold value – in other words, not wasting time further refining areas that already look good enough." / "Noise Threshold / The threshold to decide whether to continue sampling a pixel. Typical values are in the range from 0.1 to 0.001, with lower values meaning better quality but longer render times. Setting this to 0 makes Cycles guess an automatic value based on the total sample count." / "Min Samples / The minimum number of samples a pixel receives before adaptive sampling is applied." — manual, https://docs.blender.org/manual/en/5.3/render/cycles/render_settings/sampling.html, read 2026-10-11.
- EEVEE "Render … Time Limit / Stops rendering if the time exceeds the limit, even if the desired sample count hasn’t been reached yet. Setting this value to zero disables the limit." — manual, https://docs.blender.org/manual/en/5.3/render/eevee/render_settings/sampling.html, read 2026-10-11.
- "time_limit / Limit the render time. Zero disables the limit. (in [0, inf], default 0.0)" — API, https://docs.blender.org/api/5.3/bpy.types.SceneEEVEE.html; "bpy.types.SceneEEVEE / Added / bpy.types.SceneEEVEE.time_limit" under "5.2 to 5.3" — API, https://docs.blender.org/api/5.3/change_log.html; both read 2026-10-11.
- Cycles: `time_limit: FloatProperty(` … `description="Limit the render time (excluding synchronization time). " "Zero disables the limit",` `min=0.0,` `default=0.0,` … `unit='TIME_ABSOLUTE',` — `cycles_5.3/properties.py`, lines 568–575; `use_adaptive_sampling: BoolProperty(` … `default=True,` — lines 620–624; `adaptive_threshold` `min=0.0,` `max=1.0,` `soft_min=0.001,` `default=0.01,` — lines 625–633. Read 2026-10-16 (observed).
- `/* Check whether the pixel has converged and should not be sampled anymore. */` — `cycles_5.3/source/kernel/film/adaptive_sampling.h`, line 11, read 2026-10-16 (observed).

**Where I looked:** only the pages in Quotes above were cited; no search terms or additional pages were recorded for clean-enough stopping in EEVEE and Workbench, a time limit in Workbench, or the unit of `TIME_ABSOLUTE`.

**Not covered:** the unit of the time limit (Cycles' file says `TIME_ABSOLUTE`; no source cited here says seconds or minutes); whether the limit is per frame, per view layer, or for a whole animation; whether a frame that hits the time limit is still denoised and saved.

**Outside the interface:** The sources do not describe any difference. These are scene settings, not interface-only options. No command-line option sets them; a launch script could (inferred, as in question 1). **Observed 2026-10-17 (entries 18 to 23):** in renders started from a terminal with no window, Cycles' adaptive sampling ended a whole frame early, Cycles' Time Limit was spent on each frame and a cut-short frame was still written, and EEVEE's Time Limit ended an EEVEE render early with its picture written — none of them printing a word to say so. In Blender's own window the render's information line shows no mention of the limit either (entry 67).

**Operating system:** The sources do not say. **Observed 2026-10-17:** on macOS 26.6.2 on Apple silicon only, every Cycles render on the processor; Linux, Windows and a render on a graphics card were not tried in this sitting.

### Observed in Blender 5.3 — hand tests of 2026-10-17

Got by running Blender, not from a source; it sits beside the answer above. **On what computer:** the Creator's Mac — a MacBook Pro with an Apple M1 Pro chip and its own 14-core graphics (Metal), 16 GB of memory, macOS 26.6.2. **What ran:** Blender 5.3.0 Beta, built 2026-10-16, started from a terminal by Claude Opus 5.5 on 2026-10-17, with a throwaway settings folder and, unless said otherwise, `--factory-startup`. Every Cycles render here was on the processor. Nothing ran on Linux or Windows. The tests are written out in full in `Audits/Blender Research/Settings/Hand tests — 5.3.md`, by the numbers given here. Label: **observed**.

**Cycles' Time Limit is spent on each frame, and a cut-short frame is still written (entry 18).** Blender's default scene on Cycles at 400 by 400, Max Samples 16384, adaptive sampling off, denoising off, Time Limit 5, frames 1 to 3, rendered with `-a`. The whole run took `real 0m19.700s` by the shell's clock and exited 0; all three pictures were written. Blender's own lines:

```
00:00.305  render           | Rendering frame 1
00:06.598  render           | Fra: 1 | Mem: 3M | Sample 1025/16384
00:06.667  render           | Saved: '/private/tmp/claude-501/smoozy-hand-tests-5.3/Settings/out/e18/f_0001.png'
00:06.667  render           | Time: 00:06.36 (Saving: 00:00.06)
00:06.667  render           | Rendering frame 2
00:12.944  render           | Fra: 2 | Mem: 3M | Sample 1025/16384
00:12.953  render           | Saved: '/private/tmp/claude-501/smoozy-hand-tests-5.3/Settings/out/e18/f_0002.png'
00:12.953  render           | Time: 00:06.28 (Saving: 00:00.00)
00:12.953  render           | Rendering frame 3
00:19.236  render           | Fra: 3 | Mem: 3M | Sample 1025/16384
00:19.245  render           | Saved: '/private/tmp/claude-501/smoozy-hand-tests-5.3/Settings/out/e18/f_0003.png'
00:19.245  render           | Time: 00:06.29 (Saving: 00:00.00)
```

(the sample line shown for each frame is the last one it printed). Searched for "limit", the log holds no line naming the time limit; the only lines holding "time" outside the `Time:` and `Remaining` lines are Cycles' "Loading render kernels (may take a few minutes the first time)", one per frame. So the limit is applied to each frame, each frame stops well short of its count, the picture is still saved, and nothing is printed to say the limit fired.

**Adaptive sampling ends the whole frame early (entry 19).** Two files of Blender's default scene on Cycles at 128 by 128, Max Samples 2048, Time Limit 0, denoising off — one with adaptive sampling on at a Noise Threshold of 0.1, one with it off. The scene is plain and evenly lit and was chosen so the run without adaptive sampling finishes in seconds. Their count and time lines:

```
adaptive on:
00:00.323  render           | Fra: 1 | Mem: 1M | Sample 0/2048
00:00.324  render           | Fra: 1 | Remaining: 00:01.60 | Mem: 2M | Sample 1/2048
00:00.354  render           | Fra: 1 | Remaining: 00:01.30 | Mem: 2M | Sample 48/2048
00:00.416  render           | Time: 00:00.09 (Saving: 00:00.06)
adaptive off:
00:00.325  render           | Fra: 1 | Mem: 1M | Sample 0/2048
00:00.326  render           | Fra: 1 | Remaining: 00:01.42 | Mem: 1M | Sample 1/2048
00:01.597  render           | Fra: 1 | Mem: 1M | Sample 2048/2048
00:01.659  render           | Time: 00:01.33 (Saving: 00:00.06)
```

So with adaptive sampling on, the frame as a whole finished at 48 of 2048 — not only the sampling of single pixels.

**What EEVEE's and Workbench's settings blocks hold (entry 20).** Every property of the scene's EEVEE block and of its display block (the one Workbench's anti-aliasing sits on) was listed from a factory-startup Blender with no window, and each was searched — name, label and description — for `time`, `threshold`, `adaptive`, `noise`, `stop`, `limit`, `device`, `cpu` and `gpu`. The EEVEE block holds **54** properties and the display block **9**. One EEVEE match stops a render:

```
MATCH eevee time_limit | Time Limit | Limit the render time. Zero disables the limit. | words: ['time', 'limit']
```

The other EEVEE matches are about light, clamping, viewport sampling and memory (`gi_diffuse_bounces`, `gi_glossy_clamp`, `gi_irradiance_pool_size`, `taa_samples` "unlimited if 0", `use_taa_reprojection`, the four `clamp_…` settings, `volumetric_light_clamp`, `bokeh_threshold`, `motion_blur_steps`, `shadow_pool_size`, `light_threshold`). Nothing on either block is a noise threshold or adaptive stopping, and the display block matched nothing at all. Its nine properties in full:

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

So in 5.3 EEVEE has a time limit and no rule for stopping when the picture is clean enough, and Workbench has neither. What EEVEE's time limit does in a render is entry 21, below.

**EEVEE's Time Limit ends the render and the picture is still written (entry 21).** Blender's default scene on EEVEE at 400 by 400, Render Samples 4096, Time Limit 2.0. The run took `real 0m2.853s` by the shell's clock and exited 0. Every line it printed except the running count:

```
00:00.303  blend            | Read blend: "/private/tmp/claude-501/smoozy-hand-tests-5.3/Settings/files/ee_tl.blend"
00:00.308  render           | Rendering single frame
00:00.308  render           | Rendering frame 1
00:00.308  render           | Start rendering: Scene, ViewLayer
00:00.308  render           | Engine: EEVEE
Blender 5.3.0 Beta (hash 5d8151335b6b built 2026-10-16 01:36:01)
00:02.385  render           | Saved: '/private/tmp/claude-501/smoozy-hand-tests-5.3/Settings/out/e21/ee_0001.png'
00:02.385  render           | Time: 00:02.07 (Saving: 00:00.07)
Blender quit
```

and its last two count lines:

```
00:02.060  render           | Fra: 1 | Rendering 250 / 4096 samples
00:02.219  render           | Fra: 1 | Rendering 275 / 4096 samples
```

The control is the same file with the limit set to 0 at launch, stopped by a 25-second ceiling (exit status 142, which is the ceiling's): its last count line was `00:24.447  render | Fra: 1 | Rendering 3775 / 4096 samples` and it had written no picture. So the limited run did stop early rather than finish its work: it stopped at 275 of 4096 a little after two seconds, saved its picture, and printed nothing naming the limit.

**Cycles' Time Limit with denoising on (entry 21).** Blender's default scene on Cycles at 400 by 400, Max Samples 16384, adaptive sampling off, denoising on, Time Limit 2. It exited 0 in `real 0m3.143s` and wrote its picture. Its lines other than counts and Cycles' preparation steps:

```
00:00.336  blend            | Read blend: "/private/tmp/claude-501/smoozy-hand-tests-5.3/Settings/files/cy_tl_dn.blend"
00:00.341  render           | Rendering single frame
00:00.341  render           | Rendering frame 1
00:00.341  render           | Start rendering: Scene, ViewLayer
00:00.341  render           | Engine: Cycles
Blender 5.3.0 Beta (hash 5d8151335b6b built 2026-10-16 01:36:01)
00:00.346  render           | Fra: 1 | Mem: 1M | Loading denoising kernels (may take a few minutes the first time)
00:02.596  render           | Fra: 1 | Mem: 10M | Finished
00:02.666  render           | Saved: '/private/tmp/claude-501/smoozy-hand-tests-5.3/Settings/out/e21/cy_0001.png'
00:02.666  render           | Time: 00:02.32 (Saving: 00:00.06)
Blender quit
```

with its last count line `00:02.349  render | Fra: 1 | Mem: 10M | Sample 325/16384`. The same file with denoising switched off at launch also exited 0 and wrote its picture; its last count line was `00:02.381 … Mem: 3M | Sample 328/16384` and `Finished` followed at `00:02.383`. Between the last count line and `Finished` the run with denoising on spent 0.247 seconds and the run without spent 0.002. The only line in either log holding "denois" is the kernel-loading line above, in the run with denoising on; nothing is printed after the limit fires to say whether the cut-short frame was denoised.

**The Sampling panels, photographed in Blender's own window (entry 63).** Blender 5.3 was opened with its window from the terminal, with a throwaway settings folder (splash and tooltips off); a script enlarged the Properties editor and turned it to Render Properties, and closed sub-panels were opened by simulated clicks on their headers. Each picture used carries the engine's name in its Render Engine menu. **EEVEE**, every Sampling sub-panel open, every control in order:

```
Sampling
  Viewport   Samples                  16
             Temporal Reprojection    ticked
             Jittered Shadows         unticked
  Render     Samples                  64
             Time Limit               0 s
  Shadows (panel ticked)
             Rays                     1
             Steps                    6
             Volume Shadows           unticked   Steps 16 (greyed)
             Resolution               1.000
  Advanced   Light Threshold          0.010
```

**Workbench**, the whole Sampling panel:

```
Sampling
  Render     8 Samples
  Viewport   Single Pass Anti-Aliasing
```

both rows menus rather than number fields; above it the tab lists Curves, Simplify and Performance, below it Lighting. **Cycles**, with `time_limit = 5` set by the script (which read back `TIME LIMIT READS 5.0`): the Render group shows `Noise Threshold` ticked `0.0100`, `Max Samples` `4096`, `Min Samples` `0`, and **`Time Limit` reading `5 s`**. So in the window EEVEE's Render sampling has a Time Limit field, Workbench's Sampling panel offers no time or noise control, and Cycles shows its limit in seconds.

**What the render window shows when the Time Limit ends a render (entry 67).** In Blender 5.3's own window, opened from the terminal with a throwaway settings folder, the render was set to show in place of the 3D viewport (`render_display_type = 'AREA'`, a preference in that folder only), Blender's default scene set to Cycles on the processor at 400 by 400, Max Samples 32768, adaptive sampling and denoising off, Time Limit 3 seconds, and a render started from the window as the Render menu's F12 does (`bpy.ops.render.render('INVOKE_DEFAULT')`, which answered `{'RUNNING_MODAL'}`). Two photographs taken some seconds later show two moments, each with its own time printed in the render's information line. **During the render:** `Frame:1 | Time:00:02.89 | Remaining: 00:00.15 | Mem: 3M | Sample 385/32768`, with the status bar reading `Rendering...` beside a progress bar at `94%`. **After it ended:** `Frame:1 | Time:00:03.05 | Mem:3M, Peak: 3M`, the status bar empty. Neither shows a word about the limit; after the end the line shows no sample count. (The two photographs came back in the opposite order to the one they were taken in — the earlier moment in the later file — which is the window-photograph lag described under question 17; each is placed here by the time it shows.)

**On a noisier scene (entry 22, written up under question 16).** At Max Samples 4096 the frame ended at `Sample 32/4096` with adaptive sampling at a threshold of 0.5, and reached `Sample 4096/4096` at the default 0.01 in about a quarter of the time taken with adaptive sampling off; a 2-second Time Limit on a 256-sample subset stopped the frame at `Sample 129/256`; and a Noise Threshold of 0 gave a different picture from 0.01 at the same count.

---

## 3. Which settings belong to a particular render engine rather than to every engine?

**Answer**

The manual says each renderer has its own render settings, and the Properties editor's Render tab shows "EEVEE, Cycles, or Workbench settings" while Output and View Layer are separate tabs (documented). The API and Cycles' files show where each engine's settings live:

- **Cycles** (documented on `Scene` and `ViewLayer`; observed in `cycles_5.3/properties.py` for `Scene.cycles`, with the other property groups below named without verbatim quotes of their registration lines): on the scene (`Scene.cycles`: sampling, time limit, denoising, device, and more — see questions 2, 4, 9 and 19; `Scene.cycles_curves`); on each view layer (`ViewLayer.cycles`: Cycles-only passes and per-layer denoising — see question 19); and on objects, materials, lights, worlds (`World.cycles`, `World.cycles_visibility`), meshes/curves/metaballs, and cameras (`Camera.cycles_custom`).
- **EEVEE** (documented): `Scene.eevee` (SceneEEVEE: samples, time limit, ray-tracing options — see questions 2, 4 and 19) and `ViewLayer.eevee` (ViewLayerEEVEE — see question 19).
- **Workbench** (documented on `SceneDisplay` and in the Workbench manual; inferred that `Scene.display.render_aa` and `Scene.display.shading` are the settings Workbench uses, because `SceneDisplay` does not name Workbench and the manual page does not name `Scene.display`): its anti-aliasing for final renders is `Scene.display.render_aa`; its shading settings are `Scene.display.shading`, described as "Shading settings for OpenGL render engine".
- **Hydra-based engines** (documented): `Scene.hydra`, "Hydra settings for the scene".
- **Stored on the scene or view layer rather than in an engine group** (documented): the general render settings on `Scene.render` (RenderSettings) such as resolution percentage (question 14); and view layers with their core settings such as `use` ("Use for Rendering" in the panel picture, question 1) and the `samples` override (question 4).
- **Caution, inferred:** being stored in the shared group does not mean every engine uses a setting. Cycles' own interface file draws some shared RenderSettings items (texture cache, persistent data) only inside its own Cycles panels, and the manual labels individual passes and filters with the engines that support them (for example "UV Cycles", "Combined Cycles, EEVEE, Workbench").

**Label:** documented and observed (as marked); inferred (the link between Workbench and `Scene.display`, the panel visibility rule below, and the caution).

**Quotes**

- "More renderers from third-party developers are available as add-ons. Each renderer has its own render settings to control render quality and performance." / "Switching render engines may change available settings, materials, and rendering behavior." — manual, https://docs.blender.org/manual/en/5.3/render/render_engines.html, read 2026-10-11.
- "Render: EEVEE, Cycles, or Workbench settings. / Output / View Layer" — manual, https://docs.blender.org/manual/en/5.3/editors/properties_editor.html, read 2026-10-11.
- "cycles / Cycles render settings (readonly)" / "eevee / EEVEE settings for the scene (readonly)" / "display / Scene display settings for 3D viewport (readonly)" / "hydra / Hydra settings for the scene (readonly)" / "render / (readonly, never None) / Type: / RenderSettings" — API, https://docs.blender.org/api/5.3/bpy.types.Scene.html, read 2026-10-11.
- "cycles / Cycles ViewLayer Settings (readonly)" / "eevee / View layer settings for EEVEE (readonly, never None)" — API, https://docs.blender.org/api/5.3/bpy.types.ViewLayer.html, read 2026-10-11.
- "shading / Shading settings for OpenGL render engine (readonly)" — API, https://docs.blender.org/api/5.3/bpy.types.SceneDisplay.html, read 2026-10-11.
- "The setting for viewport rendering and final rendering is saved per scene." — manual, https://docs.blender.org/manual/en/5.3/render/workbench/sampling.html, read 2026-10-11.
- `bpy.types.Scene.cycles = PointerProperty(` (`cycles_5.3/properties.py`, line 1191); `bpy.types.Camera.cycles_custom`, `bpy.types.Material.cycles`, `bpy.types.Light.cycles`, `bpy.types.World.cycles`, `bpy.types.World.cycles_visibility`, `bpy.types.Mesh.cycles`, `bpy.types.Curve.cycles`, `bpy.types.MetaBall.cycles`, `bpy.types.Object.cycles`, `bpy.types.Scene.cycles_curves`, `bpy.types.ViewLayer.cycles` (lines 1207–1658, named without verbatim quotes). Cycles' panels: `COMPAT_ENGINES = {'CYCLES'}` with `return context.engine in cls.COMPAT_ENGINES` (`cycles_5.3/ui.py`, lines 63–71); `col.prop(rd, "use_auto_generate_texture_cache", text="Auto Generate")` (line 926) and `col.prop(rd, "use_persistent_data", text="Persistent Data")` (line 1008), where `rd = scene.render`. Read 2026-10-16 (observed).
- "UV Cycles" / "Combined Cycles, EEVEE, Workbench" — manual, https://docs.blender.org/manual/en/5.3/render/layers/passes.html, read 2026-10-11.

**Not covered:** a complete, setting-by-setting list of which shared RenderSettings items each engine actually uses; what happens to Cycles' settings in a file when the Cycles add-on is not loaded.

**Outside the interface:** The storage places are the same inside and outside the interface. The interface only shows an engine's panels while that engine is chosen (inferred for Cycles from the observed `COMPAT_ENGINES` check in `cycles_5.3/ui.py`). From outside, every group can be reached by a script whatever engine is chosen (inferred). **Observed 2026-10-17 (entries 11, 20, 29, 45):** from a terminal with no window, all three engines' settings were reached and set in one run whose engine was Cycles, each engine's count survived a save and engine switches, and with the Cycles add-on switched off a file saved on Cycles read back as EEVEE and rendered there.

**Operating system:** The sources do not say that this differs by operating system. **Observed 2026-10-17:** on macOS 26.6.2 on Apple silicon only; Linux and Windows were not available to this sitting.

### Observed in Blender 5.3 — hand tests of 2026-10-17

Got by running Blender, not from a source; it sits beside the answer above. **On what computer:** the Creator's Mac — a MacBook Pro with an Apple M1 Pro chip and its own 14-core graphics (Metal), 16 GB of memory, macOS 26.6.2. **What ran:** Blender 5.3.0 Beta, built 2026-10-16, started from a terminal by Claude Opus 5.5 on 2026-10-17, with a throwaway settings folder and, unless said otherwise, `--factory-startup`. Nothing ran on Linux or Windows. The tests are written out in full in `Audits/Blender Research/Settings/Hand tests — 5.3.md`, by the numbers given here. Label: **observed**.

**The complete property lists of EEVEE's and the display block (entry 20).** Listed from a Blender with no window: the scene's EEVEE block holds 54 properties and its display block 9 (the nine are quoted in full under question 2). Searched for `device`, `cpu` and `gpu`, the only hits are two EEVEE memory sizes, neither of them a choice:

```
MATCH eevee gi_irradiance_pool_size | Irradiance Pool Size | Size of the irradiance pool, a bigger pool size allows for more irradiance grid in the scene but might not fit into GPU memory and decrease performance | words: ['gpu']
MATCH eevee shadow_pool_size | Shadow Pool Size | Size of the shadow pool, a bigger pool size allows for more shadows in the scene but might not fit into GPU memory | words: ['gpu']
```

---

## 4. Does a file carry a separate sample count for each render engine?

**Answer:** Yes, per scene (documented and observed for the settings on the scene; inferred that they are stored in the saved file, except for Workbench where the manual states its setting is "saved per scene"). Cycles, EEVEE and Workbench each keep their own final-render sample setting on the scene:
- Cycles: `samples` in the Cycles scene settings (default 4096, observed);
- EEVEE: `taa_render_samples` (default 64, documented);
- Workbench: `render_aa` on `Scene.display`, a choice of anti-aliasing modes and fixed sample counts (default `'8'`, documented on `SceneDisplay`; inferred to be Workbench's setting by matching the sample counts `5, 8, 11, 16, 32` with the Workbench sampling page, quoted in question 18). The manual says the Workbench render setting "is saved per scene" (documented).

Cycles also has a separate viewport count (`preview_samples`, observed in question 18), and the Workbench manual distinguishes the 3D Viewport user preference from the per-scene setting for viewport rendering and final rendering (documented; no EEVEE viewport sample count is quoted here). On top of these, each **view layer** has one sample override (`ViewLayer.samples`, 0 = use the scene's count). This one value is **not** per engine. Cycles uses it according to its "Layer Samples" setting, and EEVEE always uses it (documented). Whether Workbench uses it is not stated (not in the sources).

That these values are kept in the saved file is **inferred**: they are settings of the scene, and the manual says the Workbench one is "saved per scene". Cycles' own version-update code also reads whether `samples` was set in an older saved file (observed), which supports the inference that the Cycles count is stored there.

**Label:** documented (EEVEE, `SceneDisplay.render_aa`, Workbench manual, view layer); observed (Cycles); inferred (that `render_aa` is Workbench's setting, and that Cycles, EEVEE and view-layer counts are stored in the file); not in the sources (whether Workbench uses the view-layer override).

**Quotes**

- "taa_render_samples / Number of samples per pixel for rendering (in [1, inf], default 64)" — API, https://docs.blender.org/api/5.3/bpy.types.SceneEEVEE.html, read 2026-10-11.
- "render_aa / Method of anti-aliasing when rendering final image (default '8')" … "Literal[‘OFF’, ‘FXAA’, ‘5’, ‘8’, ‘11’, ‘16’, ‘32’]" — API, https://docs.blender.org/api/5.3/bpy.types.SceneDisplay.html, read 2026-10-11.
- "The setting for the 3D Viewport is a user preference to specify the anti-aliasing method that runs best on the used system. The setting for viewport rendering and final rendering is saved per scene." — manual, https://docs.blender.org/manual/en/5.3/render/workbench/sampling.html, read 2026-10-11.
- "samples / Override number of render samples for this view layer, 0 will use the scene setting (in [0, inf], default 0)" — API, https://docs.blender.org/api/5.3/bpy.types.ViewLayer.html, read 2026-10-11.
- "Samples / Allows the view layer to use a different number of samples than the main scene. This is controlled by the Layer Samples when using the Cycles renderer. When using EEVEE, the Samples override is always used." — manual, https://docs.blender.org/manual/en/5.3/render/layers/override.html, read 2026-10-11.
- `samples: IntProperty(` `name="Samples",` `description="Number of samples to render for each pixel",` `min=1, max=(1 << 24),` `default=4096,` — `cycles_5.3/properties.py`, lines 533–538; `if not cscene.is_property_set("samples"):` `cscene.samples = 128` — `cycles_5.3/version_update.py`, lines 227–228. Read 2026-10-16 (observed).

**Not covered:** whether Workbench uses the view-layer override; how an add-on engine stores its count (the manual says only that each renderer has its own settings).

**Outside the interface:** The values sit in the file the same way however Blender is started (inferred). No command-line option sets any of them (see question 1). **Observed 2026-10-17 (entries 25, 29):** a Blender started from a terminal with no window read all three engines' counts back from the file at the values saved, and switching the engine changed none of them.

**Operating system:** The sources do not say. **Observed 2026-10-17:** on macOS 26.6.2 on Apple silicon only; Linux and Windows were not available to this sitting.

### Observed in Blender 5.3 — hand tests of 2026-10-17

Got by running Blender, not from a source; it sits beside the answer above. **On what computer:** the Creator's Mac — a MacBook Pro with an Apple M1 Pro chip and its own 14-core graphics (Metal), 16 GB of memory, macOS 26.6.2. **What ran:** Blender 5.3.0 Beta, built 2026-10-16, started from a terminal by Claude Opus 5.5 on 2026-10-17, with a throwaway settings folder and `--factory-startup`. Nothing ran on Linux or Windows. The tests are written out in full in `Audits/Blender Research/Settings/Hand tests — 5.3.md`, by the numbers given here. Label: **observed**.

**One file holds a count for each engine, and they survive a save, a reopen and engine switches (entry 29).** A file was saved from the factory scene with Cycles' Samples 123, EEVEE's Render Samples 45, Workbench's final anti-aliasing `'32'` and the Grease Pencil final-render anti-aliasing count 3 (`MADE cycles=123 eevee=45 workbench=32 gpencil=3`). A fresh Blender with no window opened it, read all four, then switched the engine three times, reading after each switch:

```
BACK as-loaded engine= BLENDER_EEVEE cycles= 123 eevee= 45 workbench= 32 gpencil= 3
BACK after-switch-to-CYCLES engine= CYCLES cycles= 123 eevee= 45 workbench= 32 gpencil= 3
BACK after-switch-to-BLENDER_WORKBENCH engine= BLENDER_WORKBENCH cycles= 123 eevee= 45 workbench= 32 gpencil= 3
BACK after-switch-to-BLENDER_EEVEE engine= BLENDER_EEVEE cycles= 123 eevee= 45 workbench= 32 gpencil= 3
```

So the Cycles and EEVEE counts are carried in the file and come back after a reload, as the Workbench one does, and switching the engine disturbs none of them; the Grease Pencil count is one value whatever the engine.

**Each engine's count as its own control (entry 63, under question 2).** Photographed in Blender's own window: EEVEE's Sampling panel has a Viewport `Samples` of 16 and a separate Render `Samples` of 64; Workbench's is a `Render` menu reading `8 Samples` and a `Viewport` menu; Cycles' Render group has `Max Samples` 4096 as a number field.

**More sampling settings across a save and a reopen (entry 25, under question 16).** Eleven Cycles values, among them Max Samples 137, and EEVEE's Render Samples 77 all came back as saved.

---

## 5. Can a sample count change from frame to frame?

**Answer:** Yes, by the sources' general rules, but no source says so about the sample count specifically. **Inferred.**

The steps:
1. The manual says Blender "lets you animate almost any property" (documented, general).
2. The API says a property created with the add-on property function is animatable by default (`options={'ANIMATABLE'}`), and Cycles creates its `samples` setting with that function without turning the option off (observed). So the Cycles render sample count is animatable (inferred).
3. Scripts can also change data before each frame during rendering, using the documented frame-change hook (documented, general; inferred for the sample count).

Whether EEVEE's `taa_render_samples`, Workbench's `render_aa` and the view-layer override can be animated is not stated (not in the sources). Whether a render actually re-reads the animated value at each frame is not stated either (not in the sources); that it does is inferred from the meaning of animation in step 1.

**Label:** inferred; not in the sources (any direct statement about animating sample counts, whether the EEVEE, Workbench and view-layer counts can be animated, or whether a render re-reads an animated count at each frame).

**Quotes**

- "Blender lets you animate almost any property, going from the X coordinate of an object to the transparency of a material." — manual, https://docs.blender.org/manual/en/5.3/editors/graph_editor/fcurves/introduction.html, read 2026-10-11.
- "bpy.props.IntProperty(*, name='', description='', translation_context='*', default=0, min=-2**31, max=2**31 - 1, soft_min=-2**31, soft_max=2**31 - 1, step=1, options={'ANIMATABLE'}, …)" — API, https://docs.blender.org/api/5.3/bpy.props.html, read 2026-10-11. "ANIMATABLE: / Animatable." — API, https://docs.blender.org/api/5.3/bpy_types_enum_items/property_flag_items.html, read 2026-10-11.
- `from bpy.props import (` … `IntProperty,` (lines 8–16) and `samples: IntProperty(` … `default=4096,` with no `options=` line (lines 533–538) — `cycles_5.3/properties.py`, read 2026-10-16 (observed).
- frame_change_pre quote: see question 1 (https://docs.blender.org/api/5.3/bpy.app.handlers.html).

**Where I looked:** only the pages in Quotes above (and the sampling pages in questions 2 and 4) were cited; no search terms or additional pages were recorded for animating sample counts.

**Not covered:** any explicit statement about animating sample counts; the EEVEE, Workbench and view-layer counts; whether a script or animation that changes the count between frames needs automatic script running (see question 11).

**Outside the interface:** The sources do not describe a difference. Note that a frame-change hook stored inside the file is a script, and whether it runs when started from outside depends on the script-running rules in question 11 (inferred). **Observed 2026-10-17 (entry 17):** an animation render started from a terminal with no window used the keyed count at each frame, in Cycles, in EEVEE and for a view layer's own count. Driving the count by a driver or a frame-change script was not tried.

**Operating system:** The sources do not say. **Observed 2026-10-17:** on macOS 26.6.2 on Apple silicon only; Linux and Windows were not available to this sitting.

### Observed in Blender 5.3 — hand tests of 2026-10-17

Got by running Blender, not from a source; it sits beside the answer above. **On what computer:** the Creator's Mac — a MacBook Pro with an Apple M1 Pro chip and its own 14-core graphics (Metal), 16 GB of memory, macOS 26.6.2. **What ran:** Blender 5.3.0 Beta, built 2026-10-16, started from a terminal by Claude Opus 5.5 on 2026-10-17, with a throwaway settings folder and, unless said otherwise, `--factory-startup`. Nothing ran on Linux or Windows. The tests are written out in full in `Audits/Blender Research/Settings/Hand tests — 5.3.md`, by the numbers given here. Label: **observed**.

**Which counts take a keyframe (entry 14).** A factory-startup Blender with no window was asked for a keyframe on each setting at frame 1:

```
KEY s.cycles.samples -> True
KEY s.eevee.taa_render_samples -> True
KEY s.display.render_aa -> ERROR TypeError bpy_struct.keyframe_insert() property "render_aa" not animatable
KEY s.view_layers[0].use -> True
KEY s.render.use_single_layer -> ERROR TypeError bpy_struct.keyframe_insert() property "use_single_layer" not animatable
KEY s.view_layers[0].samples -> True
KEY s.render.resolution_percentage -> ERROR TypeError bpy_struct.keyframe_insert() property "resolution_percentage" not animatable
```

So Cycles' count, EEVEE's count and a view layer's own count take a keyframe; Workbench's anti-aliasing choice does not.

**A keyed count, rendered frame by frame from outside (entry 17).** Three files, each Blender's default scene at 64 by 64, rendered with `-s 1 -e 2 -a` and no window:

- **Cycles**, on the processor, adaptive sampling and denoising off, Max Samples keyed to 4 at frame 1 and 512 at frame 2 (the build read them back as `4` and `512`):
  ```
  00:00.316  render           | Rendering frame 1
  00:00.321  render           | Fra: 1 | Mem: 1M | Sample 0/4
  00:00.321  render           | Fra: 1 | Remaining: 00:00.00 | Mem: 1M | Sample 1/4
  00:00.322  render           | Fra: 1 | Mem: 1M | Sample 4/4
  00:00.383  render           | Saved: '/private/tmp/claude-501/smoozy-hand-tests-5.3/Settings/out/e17/cycles_0001.png'
  00:00.383  render           | Rendering frame 2
  00:00.384  render           | Fra: 2 | Mem: 1M | Sample 0/512
  00:00.384  render           | Fra: 2 | Remaining: 00:00.11 | Mem: 1M | Sample 1/512
  00:00.463  render           | Fra: 2 | Mem: 1M | Sample 512/512
  00:00.464  render           | Saved: '/private/tmp/claude-501/smoozy-hand-tests-5.3/Settings/out/e17/cycles_0002.png'
  ```
- **EEVEE**, Render Samples keyed to 4 at frame 1 and 64 at frame 2:
  ```
  00:00.318  render           | Rendering frame 1
  00:00.451  render           | Fra: 1 | Rendering 1 / 4 samples
  00:00.557  render           | Fra: 1 | Rendering 4 / 4 samples
  00:00.645  render           | Saved: '/private/tmp/claude-501/smoozy-hand-tests-5.3/Settings/out/e17/eevee_0001.png'
  00:00.645  render           | Rendering frame 2
  00:00.649  render           | Fra: 2 | Rendering 1 / 64 samples
  00:00.826  render           | Fra: 2 | Rendering 25 / 64 samples
  00:00.977  render           | Fra: 2 | Rendering 50 / 64 samples
  00:01.060  render           | Fra: 2 | Rendering 64 / 64 samples
  00:01.079  render           | Saved: '/private/tmp/claude-501/smoozy-hand-tests-5.3/Settings/out/e17/eevee_0002.png'
  ```
- **A view layer's own count**, on Cycles as above with the scene count at 512 and Layer Samples at `USE`, the layer's Samples keyed to 5 at frame 1 and 100 at frame 2:
  ```
  00:00.342  render           | Rendering frame 1
  00:00.348  render           | Fra: 1 | Mem: 1M | Sample 0/5
  00:00.349  render           | Fra: 1 | Remaining: 00:00.00 | Mem: 1M | Sample 1/5
  00:00.350  render           | Fra: 1 | Mem: 1M | Sample 5/5
  00:00.410  render           | Saved: '/private/tmp/claude-501/smoozy-hand-tests-5.3/Settings/out/e17/layer_0001.png'
  00:00.410  render           | Rendering frame 2
  00:00.412  render           | Fra: 2 | Mem: 1M | Sample 0/100
  00:00.412  render           | Fra: 2 | Remaining: 00:00.02 | Mem: 1M | Sample 1/100
  00:00.428  render           | Fra: 2 | Mem: 1M | Sample 100/100
  00:00.429  render           | Saved: '/private/tmp/claude-501/smoozy-hand-tests-5.3/Settings/out/e17/layer_0002.png'
  ```

So the count is read again at each frame of an animation render — in Cycles, in EEVEE and for a view layer's own count — rather than once when the job starts.

**How a keyed Max Samples is drawn in the panel (entry 61).** In Blender 5.3's own window, opened from the terminal with a throwaway settings folder, a script set Cycles, keyed Max Samples to 16 at frame 1 and 64 at frame 20, and moved to frames 1, 10 and 20 in turn; the window was photographed at each (each picture shows the frame number in the timeline at its foot, which is how each was matched to its frame). At **frame 1** the Render group's `Max Samples` field reads **16** and is filled yellow-olive; at **frame 10** it reads **39** and is filled green; at **frame 20** it reads **64** and is filled yellow-olive. The script read the same values back (`SAMPLES at f20 64`, `SAMPLES at f10 39`, `SAMPLES at f1 16`). The Viewport group's `Max Samples` stayed `1024`, unfilled. So the panel marks a keyed frame one colour and a frame between keys another, and between the keys the count takes a value neither key holds.

**The right-click menu on Max Samples (entry 62).** In Blender 5.3's own window, with Cycles chosen and Render Properties showing, the pointer was moved over the Render group's `Max Samples` field by simulated input and the right mouse button pressed there; the photographed menu is headed **Samples** and lists, in order:

```
Insert Keyframe                 I
Add Driver...                   ⌘ D
Open Drivers Editor
Add to Keying Set
Remove from Keying Set          ⌥ K
Reset to Default Value          Backspace
Copy to Selected                (greyed)
Copy Data Path                  ⇧ ⌘ C
Copy Full Data Path             ⇧ ⌥ ⌘ C
Copy as New Driver
Online Manual                   F1
```

So the window offers the sample count a keyframe and a driver.

---

## 6. Does every render engine render on a computer with no screen attached?

**The question is ambiguous.** "No screen attached" may mean no monitor plugged in, or no display system running at all ("headless"). The sources only speak to the second.

**Answer:** Not for every engine on every system.
- Rendering from the command line in general does not need a graphical display (documented; the manual gives Linux without an X server as its example).
- **EEVEE:** "Headless rendering is not supported on headless Windows systems" (documented). EEVEE also renders only on the GPU (documented).
- **Cycles and Workbench:** the sources make no statement about headless rendering for either (not in the sources).

**Label:** documented (the general rule and the EEVEE limit); not in the sources (Cycles and Workbench specifically).

**Quotes**

- "One advantage of using the command line is that we do not need a graphical display (no need for X server on Linux for example) and consequently we can render via a remote shell (typically SSH)." — manual, https://docs.blender.org/manual/en/5.3/advanced/command_line/render.html, read 2026-10-11.
- "Headless Rendering / Headless rendering is not supported on headless Windows systems." / "Being a rasterization engine, EEVEE only uses the power of the GPU to render." — manual, https://docs.blender.org/manual/en/5.3/render/eevee/limitations/limitations.html, read 2026-10-11.

**Where I looked:** searched all three document sets for: headless, no display, without a display, graphical display, X server, xvfb, EGL, display server, without a monitor, no monitor, virtual display, offscreen context, without UI, UI-less, background mode, --background. Read the command-line render, launch (Linux, macOS, Windows) and arguments pages; the EEVEE limitations page; the Workbench index, sampling and performance pages; the GPU troubleshooting pages (they cover virtual machines and OpenGL forwarding, not headless rendering); the API `gpu` module and `RenderEngine` pages. The API mentions that the `gpu` module "can't be imported by default" in background mode and offers `gpu.init()` "for background use"; that is about scripts, not about whether Workbench or EEVEE renders, so I did not use it as an answer.

**Not covered:** Cycles and Workbench without a display; EEVEE without a display on Linux and macOS (the sources limit only Windows); what is reported when it fails.

**Outside the interface:** This question is itself about running from outside. Beyond the quotes above, the sources do not say. **Observed 2026-10-17 (entry 33):** all three engines rendered from a terminal with no Blender window, on a Mac that has a screen.

**Operating system:** Documented difference: EEVEE headless is not supported on headless Windows. Nothing is stated for Linux or macOS. **Observed 2026-10-17:** on macOS 26.6.2 on Apple silicon, with a screen attached, all three engines rendered with no window. **Not tried, and why:** a Linux machine with no display server, a headless Windows machine, and a computer with no graphics hardware were not available to this sitting, so the documented Windows limit and the screenless case stay untested in 5.3.

### Observed in Blender 5.3 — hand tests of 2026-10-17

Got by running Blender, not from a source; it sits beside the answer above. **On what computer:** the Creator's Mac — a MacBook Pro with an Apple M1 Pro chip and its own 14-core graphics (Metal), 16 GB of memory, macOS 26.6.2. **This machine has a screen attached**, so these runs show rendering with no Blender window, not rendering on a computer with no screen. **What ran:** Blender 5.3.0 Beta, built 2026-10-16, started from a terminal by Claude Opus 5.5 on 2026-10-17, with a throwaway settings folder and `--factory-startup`. The tests are written out in full in `Audits/Blender Research/Settings/Hand tests — 5.3.md`, by the numbers given here. Label: **observed**.

**Each engine renders with no window (entry 33).** Blender's default scene at 64 by 64, Cycles' count 4 and EEVEE's 4, rendered three times with `-b` and `-E <engine>`:

```
CYCLES exit=0   pixelWidth: 64   pixelHeight: 64  errors=0
BLENDER_EEVEE exit=0   pixelWidth: 64   pixelHeight: 64  errors=0
BLENDER_WORKBENCH exit=0   pixelWidth: 64   pixelHeight: 64  errors=0
```

(`errors` is the number of lines in each run's output holding "error" or "warning"). `gpu.init()` was not called.

**A scene with no camera, and one whose camera is hidden (entry 34).** Two files of Blender's default scene on Workbench at 64 by 64: one saved with the scene's camera set to none (a fresh Blender read back `s.camera = None`, with the object `Camera` still in the scene), one with the camera object hidden for rendering and in the viewport (read back `(True, True)`). Both rendered with no message, exit status 0, and wrote a picture; the no-camera run's whole output:

```
00:00.320  blend            | Read blend: "/private/tmp/claude-501/smoozy-hand-tests-5.3/Settings/files/nocam.blend"
00:00.325  render           | Rendering single frame
00:00.325  render           | Rendering frame 1
00:00.327  render           | Start rendering: Scene, ViewLayer
00:00.327  render           | Engine: Workbench
00:00.494  render           | Saved: '/private/tmp/claude-501/smoozy-hand-tests-5.3/Settings/out/e34/nocam_0001.png'
00:00.494  render           | Time: 00:00.16 (Saving: 00:00.06)
```

The hidden-camera run printed the same lines. Both pictures were compared channel by channel with an ordinary render of the same scene with its camera (from entry 4): `PIXELS max_difference 0.0 channels_differing 0 of 16384` each time.

What this does not reach: a computer with no screen, and Windows and Linux at all (none was available).

---

## 7. Does a scene set up for one render engine come out the same in another?

**Answer:** No. The manual says materials may look different between engines, some features and nodes work only in one engine, there will always be small differences between Cycles and EEVEE, and Workbench does not use shader nodes at all. Each engine also has its own quality settings (question 3), so those do not carry across.

**Label:** documented.

**Quotes**

- "Switching render engines may change available settings, materials, and rendering behavior. / Some features are engine-specific. / Materials may appear differently between engines. / Certain nodes or effects may only be supported in specific engines." — manual, https://docs.blender.org/manual/en/5.3/render/render_engines.html, read 2026-10-11.
- "These are shared between EEVEE and Cycles, however some features are only supported in one or the other." — manual, https://docs.blender.org/manual/en/5.3/render/introduction.html, read 2026-10-11.
- "All BSDF’s are using approximations to achieve realtime performance so there will always be small differences between Cycles and EEVEE." — manual, https://docs.blender.org/manual/en/5.3/render/eevee/limitations/limitations.html, read 2026-10-11.
- "These nodes are only available if EEVEE is the active render engine. These nodes will not work in Cycles." — manual, https://docs.blender.org/manual/en/5.3/render/eevee/limitations/nodes_support.html, read 2026-10-11.
- "Unlike other render engines such as EEVEE or Cycles, the Workbench engine does not use shader nodes." — manual, https://docs.blender.org/manual/en/5.3/render/workbench/index.html, read 2026-10-11.

**Not covered:** a list of exactly what differs. Note that the EEVEE limitations page (https://docs.blender.org/manual/en/5.3/render/eevee/limitations/limitations.html, read 2026-10-11) still says "Only perspective and orthographic projections are currently supported", while the 5.3 release notes say "EEVEE can now render panoramic cameras" (https://developer.blender.org/docs/release_notes/5.3/eevee/, read 2026-10-11). The sources disagree on this point.

**Outside the interface:** The sources do not describe a difference.

**Operating system:** The sources do not say.

---

## 8. Can one scene take in another scene's rendered result, so that rendering the first renders the second?

**Answer:** Yes, in two documented ways.
1. **Compositor:** the Render Layers node has a "Scene" setting and "renders a View Layer from a scene". In 5.3 the compositor also renders scenes referenced inside node groups (release note). This route gives the other scene's raw render, not its own compositing; the manual says to render that scene to a file first if its composited output is wanted.
2. **Video Sequencer scene strips:** a scene strip inserts "the render output of another scene", and such scenes are "rendered dynamically during playback or rendering". A strip cannot point at its own scene. Only the Combined pass is used. Separately, a render setting sends the result through the sequencer when strips exist.

**Label:** documented; inferred (command-line sequencer rendering).

**Quotes**

- "The Render Layers node renders a View Layer from a scene and outputs its image and any enabled render passes to the compositor." / "Scene / The scene for which to render a view layer." / "To use the composited output of another scene instead of its raw render passes, first render that scene to a multilayer image format such as OpenEXR. The resulting image can then be loaded using the Image node." — manual, https://docs.blender.org/manual/en/5.3/compositing/types/input/scene/render_layers.html, read 2026-10-11.
- "The compositor now renders scenes referenced inside node groups." — release notes, https://developer.blender.org/docs/release_notes/5.3/compositor/, read 2026-10-11.
- "Scene strips are a way to insert the render output of another scene into your sequence." / "Scene strips cannot be used to reference the sequence’s own scene; a secondary scene must be used instead." / "Scene strips do not render individual Render Passes; only the Combined render pass will be used." — manual, https://docs.blender.org/manual/en/5.3/video_editing/edit/montage/strips/scene.html, read 2026-10-11.
- "When used as strips, scenes are rendered dynamically during playback or rendering, and they share the same data-blocks unless they were created as full copies." — manual, https://docs.blender.org/manual/en/5.3/scene_layout/scene/introduction.html, read 2026-10-11.
- "use_sequencer / Process the render (and composited) result through the video sequence editor pipeline, if sequencer strips exist (default True)" and "use_render_cache / Save render cache to EXR files (useful for heavy compositing, Note: affects indirectly rendered scenes)" — API, https://docs.blender.org/api/5.3/bpy.types.RenderSettings.html, read 2026-10-11.
- "(possibly in different scenes through the Compositor)" — manual, https://docs.blender.org/manual/en/5.3/render/freestyle/view_layer/freestyle.html, read 2026-10-11.

**Not covered:** whether the second scene renders with its own engine, sample count and resolution, or with the first scene's; whether a scene can take in a scene that takes in the first (a loop). The "Background Scene" setting on `https://docs.blender.org/manual/en/5.3/scene_layout/scene/properties.html` (read 2026-10-11, not quoted) is about sharing background objects rather than a rendered result, so I did not count it.

**Outside the interface:** The sources do not describe a difference. Whether a command-line render of a scene with strips renders those strips is inferred from `use_sequencer` above, not stated for the command line (the render operator also has a `use_sequencer_scene` argument, quoted in question 12). **Observed 2026-10-17 (entries 36 to 40):** both routes work in a render started from a terminal with no window; a command-line render of a scene whose sequencer holds strips writes the sequencer's result and renders no view layer of its own, and with an empty sequencer it renders the 3D scene; a render started from outside uses the window's scene, not the Sequencer Scene.

**Operating system:** The sources do not say. **Observed 2026-10-17:** on macOS 26.6.2 on Apple silicon only; Linux and Windows were not available to this sitting.

### Observed in Blender 5.3 — hand tests of 2026-10-17

Got by running Blender, not from a source; it sits beside the answer above. **On what computer:** the Creator's Mac — a MacBook Pro with an Apple M1 Pro chip and its own 14-core graphics (Metal), 16 GB of memory, macOS 26.6.2. **What ran:** Blender 5.3.0 Beta, built 2026-10-16, started from a terminal by Claude Opus 5.5 on 2026-10-17, with a throwaway settings folder and `--factory-startup`. Nothing ran on Linux or Windows. The tests are written out in full in `Audits/Blender Research/Settings/Hand tests — 5.3.md`, by the numbers given here. Label: **observed**.

**A Render Layers node pointed at another scene (entry 37).** A file with two scenes. `Alpha`: Blender's default scene on Workbench at 64 by 64, its compositing switched on, its compositor node group holding one Render Layers node with Scene set to `Beta`, wired to the group's output. `Beta`: a copy of `Alpha` sharing its cube, camera and light, on Cycles with 8 samples on the processor, its own compositor holding Render Layers → Invert → output. Rendering `Alpha` with `-S Alpha … -f 1`:

```
00:00.314  render           | Start rendering: Beta, ViewLayer
00:00.314  render           | Engine: Cycles
Scene switch for render: 'Alpha' in file: '/private/tmp/claude-501/smoozy-hand-tests-5.3/Settings/files/rlnode.blend'
00:00.320  render           | Fra: 1 | Mem: 1M | Sample 0/8
00:00.320  render           | Fra: 1 | Remaining: 00:00.00 | Mem: 1M | Sample 1/8
00:00.373  render           | Fra: 1 | Mem: 1M | Sample 8/8
00:00.523  render           | Saved: '/private/tmp/claude-501/smoozy-hand-tests-5.3/Settings/out/e37/alpha_0001.png'
```

Three pictures from the same file — `Alpha`'s, `Beta` rendered directly with its compositing on, and `Beta` rendered directly with its compositing switched off at launch — first pixel and middle pixel of each:

```
PIX alpha_0001.png size (64, 64) first [0.2235, 0.2314, 0.2275, 1.0] middle [0.549, 0.549, 0.549, 1.0]
PIX betacomp_0001.png size (64, 64) first [0.7608, 0.7608, 0.7608, 1.0] middle [0.7255, 0.7255, 0.7255, 1.0]
PIX betaraw_0001.png size (64, 64) first [0.2235, 0.2314, 0.2275, 1.0] middle [0.549, 0.549, 0.549, 1.0]
```

So rendering the first scene starts a fresh render of the second, with the second scene's own engine and own sample count — Cycles at 8, not the host's Workbench — and `Alpha`'s picture matches `Beta`'s raw render, not its inverted composite: the pulled-in scene's own compositor did not run.

**Two scenes' Render Layers nodes pointed at each other (entry 38).** The same two scenes, both on Workbench, `Alpha`'s Render Layers node set to `Beta` and `Beta`'s to `Alpha`, both compositing switched on. Rendering `Alpha` under a sixty-second ceiling finished in 0.977 seconds by the shell's clock, exit status 0, and wrote a picture:

```
00:00.327  render           | Start rendering: Beta, ViewLayer
00:00.327  render           | Engine: Workbench
00:00.433  render           | Executing compositor
00:00.433  render           | Fra: 1 | Compositing
Scene switch for render: 'Alpha' in file: '/private/tmp/claude-501/smoozy-hand-tests-5.3/Settings/files/circular.blend'
00:00.495  render           | Saved: '/private/tmp/claude-501/smoozy-hand-tests-5.3/Settings/out/e38/x_0001.png'
Blender quit
```

So a loop produced no error and no hang; `Beta` was rendered once and the chain stopped there.

**A Scene strip in the sequencer (entry 39).** `Alpha` on Workbench at 64 by 64, its sequencer holding one Scene strip of `Beta` at frame 1 with the sequencer switched on; `Beta` on Cycles with 8 samples. Rendering `Alpha`, exit status 0:

```
00:00.317  render           | Start rendering: Beta, ViewLayer
00:00.317  render           | Engine: Cycles
00:00.323  render           | Fra: 1 | Mem: 1M | Sample 0/8
00:00.323  render           | Fra: 1 | Remaining: 00:00.00 | Mem: 1M | Sample 1/8
00:00.375  render           | Fra: 1 | Mem: 1M | Sample 8/8
00:00.436  render           | Saved: '/private/tmp/claude-501/smoozy-hand-tests-5.3/Settings/out/e39/s_0001.png'
```

So through a Scene strip too, the referenced scene renders with its own engine and its own count.

**The sequencer against the compositor, and a sequencer with no strips (entry 40).** Three files of Blender's default scene on Workbench at 64 by 64, the sequencer switched on in each: (a) a red Color strip from frame 1, compositing off; (b) the same strip, compositing on with a compositor whose output is solid green; (c) a sequencer created but holding no strips. Each rendered frame 1, exit status 0. The lines naming what ran:

```
(a)  00:00.325  render           | Executing sequencer
     00:00.386  render           | Saved: '/private/tmp/claude-501/smoozy-hand-tests-5.3/Settings/out/e40/a_0001.png'
(b)  00:00.326  render           | Executing sequencer
     00:00.388  render           | Saved: '/private/tmp/claude-501/smoozy-hand-tests-5.3/Settings/out/e40/b_0001.png'
(c)  00:00.324  render           | Start rendering: Scene, ViewLayer
     00:00.324  render           | Engine: Workbench
     00:00.486  render           | Saved: '/private/tmp/claude-501/smoozy-hand-tests-5.3/Settings/out/e40/c_0001.png'
```

and the pictures' first and middle pixels:

```
PIX a_0001.png size (64, 64) first [0.8549, 0.2196, 0.1255, 1.0] middle [0.8549, 0.2235, 0.1294, 1.0]
PIX b_0001.png size (64, 64) first [0.8549, 0.2196, 0.1255, 1.0] middle [0.8549, 0.2235, 0.1294, 1.0]
PIX c_0001.png size (64, 64) first [0.2275, 0.2275, 0.2275, 1.0] middle [0.3255, 0.3216, 0.3255, 1.0]
```

So with strips present the sequencer's result is what is written, and no view layer of the scene is rendered: (a) and (b) printed no `Start rendering` line, the green compositor did not reach (b)'s picture, and both pictures are the red strip as the file's colour management shows it. With the sequencer switched on but empty, (c) rendered the 3D scene as an ordinary render.

**A Sequencer Scene that differs from the active scene (entry 36, under question 12).** A render started from outside used the window's scene and the Sequencer Scene had no effect on it.

**Not settled here.** How frame numbers line up between the two scenes.

---

## 9. Does a file record whether it renders on the graphics cards or on the processor?

**Answer:** Partly, and only for Cycles (that scene settings are stored in the saved file is **inferred**, as in question 4).
- **Cycles:** each scene has a "Device" setting, CPU or "GPU Compute" (default CPU) (documented; default observed; stored in the file inferred). The manual says each scene must be set to the GPU Compute device.
- **Which kind of GPU and which cards** are not in that scene setting. They are chosen in the Preferences ("Cycles Render Devices"): Cycles keeps them in its add-on preferences (observed), and the manual says preferences live in `userpref.blend`. So they are recorded on the computer, not in the file (**inferred**). If the preference is "None", the CPU is used (documented).
- **Command line:** Cycles defines a `--cycles-device` option whose help text says it overrides both the preferences and the scene setting (the help text is observed in Cycles' own file; that a render behaves accordingly is **inferred**).
- **EEVEE:** GPU only, so there is no choice to record (documented).
- **Other device choices on the scene** (documented on `RenderSettings`, observed on `Scene.cycles`; stored in the file **inferred**): the compositor device (default GPU) and the compositor's Denoise-node device (documented). Cycles' scene settings also have a setting for whether denoising runs on the GPU (observed).

**Label:** documented, observed, inferred (as marked).

**Quotes**

- "Device / Specifies the compute device used for rendering in Cycles." / "CPU: / Render using the system’s central processing unit. This mode is compatible with all Cycles features." / "GPU Compute: / Render using one or more graphics processing units (GPUs) through the compute backend and devices selected in the Cycles Render Devices." / "Denoising and Compositing have their own device selection." — manual, https://docs.blender.org/manual/en/5.3/render/cycles/render_settings/index.html, read 2026-10-11.
- "To enable GPU rendering: / Select either CUDA, OptiX, HIP, oneAPI, or Metal in the Cycles Render Devices user preference. / Enable the available devices for the chosen backend. / Configure each scene to use the GPU Compute Device." — manual, https://docs.blender.org/manual/en/5.3/render/cycles/gpu_rendering.html, read 2026-10-11.
- "None: / When set to None or when the only option is None: the CPU will be used as the computing device for Cycles." — manual, https://docs.blender.org/manual/en/5.3/editors/preferences/system.html, read 2026-10-11.
- "./config/userpref.blend / User preferences." — manual, https://docs.blender.org/manual/en/5.3/advanced/blender_directory_layout.html, read 2026-10-11.
- `device: EnumProperty(` `name="Device",` `description="Device to use for rendering",` `items=enum_devices,` `default='CPU',` (`cycles_5.3/properties.py`, lines 430–436), with `('GPU', "GPU Compute", "Use GPU compute device for rendering, configured in the system tab in the user preferences"),` (lines 32–36); `class CyclesPreferences(bpy.types.AddonPreferences):` … `compute_device_type: EnumProperty(` … `devices: CollectionProperty(type=CyclesDeviceSettings)` (lines 1680–1717); `denoising_use_gpu: BoolProperty(` … `default=False,` (lines 480–484). `help="Set the device to use for Cycles, overriding user preferences and the scene setting."` — `cycles_5.3/engine.py`, lines 16–17. Read 2026-10-16 (observed).
- "compositor_device / Set how compositing is executed (default 'GPU')" and "compositor_denoise_device / The device to use to process the denoise nodes in the compositor (default 'AUTO')" — API, https://docs.blender.org/api/5.3/bpy.types.RenderSettings.html, read 2026-10-11.

**Not covered:** what a Cycles render does when the scene says GPU but the computer has no usable GPU or the preference is "None". Cycles' interface file has a helper that treats that case as CPU (`return (get_device_type(context) == 'NONE' or cscene.device == 'CPU' or not backend_has_active_gpu(context))`, `cycles_5.3/ui.py`, line 110, observed), but that helper decides what the interface shows; that the render behaves the same way is inferred only. Also not covered: what is reported in that case, and what `--cycles-device` does with a device the computer lacks (Cycles' file calls `sys.exit(1)` in its argument-handling branch — `cycles_5.3/engine.py`, lines 40–41, observed; that Blender exits with code 1 when the override call fails is inferred from that code, and what makes that call fail is not in the files given).

**Outside the interface:** From outside, `--cycles-device` overrides the file and the preferences (inferred from the option's observed help text in `cycles_5.3/engine.py`). The manual's example is "blender -b file.blend -f 20 -- --cycles-device OPTIX" and it says "Append +CPU to a GPU device to render on both CPU and GPU." (manual, https://docs.blender.org/manual/en/5.3/advanced/command_line/arguments.html, read 2026-10-11). The preferences still apply when started from outside unless overridden (inferred; the security page says "the Preferences are still used" in a different context, see question 11). **Observed 2026-10-17 (entries 30 to 32):** the scene's device choice is carried in the file per scene; from a terminal with no window, `--cycles-device METAL` beat the file's `CPU`, names this Mac cannot serve were refused with exit status 1 and no picture, and a file asking for the graphics card rendered and wrote its picture whatever the preferences held — with no warning, and with the statistics carrying the off-the-processor line only when Metal and its devices were switched on in the preferences.

**Operating system:** Yes, it differs (documented): CUDA, OptiX, HIP and oneAPI are listed as supported on Windows and Linux; Metal on Apple computers with Apple Silicon. Cycles' file contains a helper returning `5` when `platform.system() == 'Darwin'` and `platform.machine() == 'arm64'`: `if (platform.system() == 'Darwin') and (platform.machine() == 'arm64'):` `return 5` (`cycles_5.3/properties.py`, lines 1684–1690, observed; which setting calls this helper and which device `5` stands for were not quoted). So the same file's "GPU Compute" setting means different hardware on different computers (inferred). **Observed 2026-10-17:** this Mac offers the device types `NONE` and `METAL` only, and refuses `CUDA`, `OPTIX`, `HIP` and `ONEAPI` (entries 31, 56). **Not tried, and why:** what those names, and a file's "GPU Compute", do on a machine with an NVIDIA, AMD or Intel card, and anything on Linux or Windows — no such machine was available to this sitting.

### Observed in Blender 5.3 — hand tests of 2026-10-17

Got by running Blender, not from a source; it sits beside the answer above. **On what computer:** the Creator's Mac — a MacBook Pro with an Apple M1 Pro chip and its own 14-core graphics (Metal), 16 GB of memory, macOS 26.6.2, with no NVIDIA, AMD or Intel graphics card. **What ran:** Blender 5.3.0 Beta, built 2026-10-16, started from a terminal by Claude Opus 5.5 on 2026-10-17, with a throwaway settings folder and, unless said otherwise, `--factory-startup`. Nothing ran on Linux or Windows. The tests are written out in full in `Audits/Blender Research/Settings/Hand tests — 5.3.md`, by the numbers given here. Label: **observed**.

**A second device argument, Blender's own (entry 1).** The program's help text carries this, word for word:

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

It was not run: it is about the graphics backend Blender draws with, and the tests here use Cycles' own `--cycles-device`.

**The device choice is written into the file, one scene at a time (entry 30).** A brand-new factory scene, read with nothing saved:

```
FACTORY engine= BLENDER_EEVEE cycles_device= CPU compositor_device= GPU
```

Then a file saved with two scenes — `One` with the Cycles device set to `GPU` and the compositor device set to `GPU`, `Two` left as a new scene starts — and opened again in a fresh Blender with no window:

```
SCENE One cycles_device= GPU compositor_device= GPU
SCENE Two cycles_device= CPU compositor_device= GPU
```

So the file carries the Cycles device choice, keeps it across a save and a reload, and holds it per scene; a new scene starts on `CPU` for Cycles and `GPU` for the compositor.

**EEVEE's and Workbench's settings blocks hold no device choice (entry 20, under question 3).** Searched for `device`, `cpu` and `gpu`, the only hits are two EEVEE memory sizes.

**What each `--cycles-device` name does on this Mac (entry 31).** A file with Cycles on the processor (`CPU` saved in the file), 100 by 100, 16 samples, rendered with `blender -b --factory-startup dev.blend -o … -F PNG -f 1 -- --cycles-device <NAME> --cycles-print-stats` from a settings folder in which nothing had been chosen. The line that tells where the render ran is `Profiling information not available (only works with CPU rendering)`, which Cycles' statistics carry only when the render was not on the processor. Each run:

```
no --cycles-device  exit=0 image=1 profiling-line=0
CPU                 exit=0 image=1 profiling-line=0
METAL               exit=0 image=1 profiling-line=1
CUDA                exit=1 image=0 profiling-line=0   00:00.370  reports          | ERROR Found no Cycles device of the specified type
OPTIX               exit=1 image=0 profiling-line=0   00:00.321  reports          | ERROR Found no Cycles device of the specified type
HIP                 exit=1 image=0 profiling-line=0   00:00.331  reports          | ERROR Found no Cycles device of the specified type
ONEAPI              exit=1 image=0 profiling-line=0   00:00.330  reports          | ERROR Found no Cycles device of the specified type
GPU                 exit=1 image=0 profiling-line=0   00:00.296  cycles           | ERROR GPU is not a valid Cycles device.   Error: Not freed memory blocks: 1, total unfreed memory 0.000004 MB
NOT_A_DEVICE        exit=1 image=0 profiling-line=0   00:00.305  cycles           | ERROR NOT_A_DEVICE is not a valid Cycles device.   Error: Not freed memory blocks: 1, total unfreed memory 0.000004 MB
```

One refused run in full (CUDA):

```
00:00.361  blend            | Read blend: "/private/tmp/claude-501/smoozy-hand-tests-5.3/Settings/files/dev.blend"
00:00.366  render           | Rendering single frame
00:00.366  render           | Rendering frame 1
00:00.366  render           | Start rendering: Scene, ViewLayer
00:00.366  render           | Engine: Cycles
00:00.370  render           | Fra: 1 | Mem: 0M | Initializing
Blender 5.3.0 Beta (hash 5d8151335b6b built 2026-10-16 01:36:01)
00:00.370  reports          | ERROR Found no Cycles device of the specified type
Blender quit
```

From a settings folder with Metal chosen and both its devices switched on (`DEVICES [('CPU', 'Apple M1 Pro', True), ('METAL', 'Apple M1 Pro (GPU - 14 cores)', True)]`), `--cycles-device METAL` also exited 0, wrote its picture and carried the profiling line. So on this Mac `METAL` beats the file's `CPU`; the four technologies this Mac cannot have are all refused in the same words, with exit status 1 and no picture; and `GPU`, which is the file's own word for the graphics card, is not a name this argument takes.

**Without the double dash** — `blender -b --factory-startup dev.blend -o … -F PNG -f 1 --cycles-device METAL` — the frame was rendered on the file's own setting and saved before Blender reached the unknown argument, and the run then ended with exit status 1:

```
00:00.404  render           | Fra: 1 | Mem: 2M | Finished
00:00.465  render           | Saved: '/private/tmp/claude-501/smoozy-hand-tests-5.3/Settings/out/e31/nodash2_0001.png'
00:00.465  render           | Time: 00:00.14 (Saving: 00:00.06)
unknown argument, loading as file: --cycles-device
00:00.465  reports          | ERROR Cannot read file "/private/tmp/claude-501/smoozy-hand-tests-5.3/Settings/--cycles-device": No such file or directory
Blender quit
```

**Workbench's whole Render tab, photographed in Blender's own window (entry 65).** Blender 5.3 was opened with its window from the terminal (a throwaway settings folder), Workbench chosen, the Properties editor enlarged and turned to Render, every closed panel and sub-panel opened by simulated clicks on its arrow, and the tab photographed from top to bottom, scrolling by simulated mouse wheel. The panels, in order: **Curves**; **Simplify** (sub-panels Viewport, Render, Grease Pencil); **Performance** (`High Quality Normals` unticked, `Anisotropic Filtering` `2×`, and the sub-panels **Compositor** and **Denoise Nodes**); **Sampling**; **Lighting**; **Object Color**; **Options**; **Film**; **Grease Pencil** (Viewport, Render); **Freestyle**; **Color Management** (sub-panels Curves, White Balance, Working Space, Advanced). The controls in the whole tab that name a processor, a graphics card or a device:

```
Performance ▸ Compositor
   Device              CPU | GPU          — GPU selected
   Precision           Auto
Performance ▸ Denoise Nodes
   Denoising Device    Auto | CPU | GPU   — Auto selected
   Preview Quality     Balanced
   Final Quality       High
Color Management ▸ Advanced
   Display Emulation   Automatic
```

The first two are the compositor's device and the compositor Denoise nodes' device; the third is about colour. No control anywhere in the tab is a device setting for Workbench itself.

**What a file's "GPU Compute" means under three sets of preferences (entry 32).** The same file, its scene device switched to `GPU` at launch and nothing else changed, rendered with `-- --cycles-print-stats` from three settings folders. Each run printed what its preferences held:

```
preferences set to None:            PREFS SAY NONE []                                                                                  exit=0 image=1 profiling-line=0
preferences with Metal and devices: PREFS SAY METAL [('CPU', 'Apple M1 Pro', True), ('METAL', 'Apple M1 Pro (GPU - 14 cores)', True)]  exit=0 image=1 profiling-line=1
factory preferences:                PREFS SAY METAL []                                                                                 exit=0 image=1 profiling-line=0
```

None of the three runs printed a `WARNING` or `ERROR` line. Run again with `--log "cycles" --log-level info`, the lines naming a device were, for the folder set to None:

```
00:00.331  cycles           | Using default CPU kernels.
00:00.332  cycles           | Using default CPU kernels.
00:00.333  cycles           | Number of lights sent to the device: 1
```

for the folder with Metal and its devices switched on:

```
00:00.332  cycles           | Added device "Apple M1 Pro (GPU - 14 cores)" with id "METAL_Apple M1 Pro (GPU - 14 cores)".
00:00.332  cycles           | Device with id "METAL_Apple M1 Pro (GPU - 14 cores)" supports OpenImageDenoise.
00:00.332  cycles           | CPU render threads reduced from 8 to 7, to dedicate to GPU.
00:00.332  cycles           | Using default CPU kernels.
```

and for factory preferences:

```
00:00.327  cycles           | Added device "Apple M1 Pro (GPU - 14 cores)" with id "METAL_Apple M1 Pro (GPU - 14 cores)".
00:00.327  cycles           | Device with id "METAL_Apple M1 Pro (GPU - 14 cores)" supports OpenImageDenoise.
00:00.328  cycles           | Using default CPU kernels.
00:00.328  cycles           | Using default CPU kernels.
```

(lines of the form `Updating … | Copying … to device`, which every render prints, are left out). A factory-startup Blender's own Cycles preferences read `FACTORY PREFS compute_device_type 'METAL' devices 0`. So a file asking for the graphics card renders and writes its picture under every one of the three; with the preferences set to None, or left at factory settings with no device switched on, the statistics do not carry the line that marks a render off the processor, and nothing is printed to say so.

---

## 10. What does Blender state about opening a file saved by a different version?

**Answer (documented):**
- **Older files** open and are converted; this "is expected to give very good and usable results". Support for very old features may be dropped after at least two years.
- **Newer files** open with a risk of lost data: "loss of feature should always be expected". The interface shows a warning, and saving over the file asks for confirmation.
- **Across a major version jump**, an older Blender may refuse to open a newer file, "with a message stating which minimal version is needed".
- A tip suggests converting in steps through in-between releases.
- 5.3 release notes: a breaking change means scenes with a Sky Texture's Sun Disc "will look different from earlier versions"; and Spline IK chains are capped to 255 bones on reading so that 5.3 can read a future file without crashing.
- The API exposes the version a file was saved with (`bpy.data.version`), and a hook that runs when the conversion code ends.

**Observed in Cycles' own files (code) and inferred (runtime conversion):** Cycles registers `version_update.do_versions` on `bpy.app.handlers.version_update` (observed). Combining that registration with the handler's documented purpose ("on ending the versioning code"), it is **inferred** that Cycles runs its own conversion when an older file is opened and changes Cycles settings for files saved up to version 5.2.8. Examples in `do_versions` (observed as code; effect on opening inferred): for files up to 5.0.77 it sets `use_auto_tile = True` and `tile_size = 8192`; for files up to 4.2.52 it sets `sampling_pattern = 'TABULATED_SOBOL'` (whose UI item in `properties.py` is labelled `"Classic"`, match inferred); for files up to 3.0.25 it fills an unset `samples` property with `128` rather than the current default. The code skips files with `version > (5, 2, 8)` (observed), and a comment at line 92 says `# We don't modify startup file because it assumes to` (comment observed; behaviour inferred).

**Inferred from the Cycles code:** some settings of an older file can render differently in 5.3 than in the version that saved it, because 5.3 rewrites them on opening.

**Label:** documented; observed (Cycles `version_update.py` and `__init__.py` code); inferred (that the conversion runs on opening older files and can change how they render); not in the sources (command-line behaviour when opening a newer file).

**Quotes**

- "Blender can open blend-files saved with both older versions of the software (backward compatibility), and newer ones (forward compatibility). This comes with some limitations though." / "Opening older files and converting them for the current version of Blender is usually straight-forward. It is expected to give very good and usable results." / "This will never be less than a full major release cycle (i.e. two years at least)." / "Forward compatibility is inherently harder to ensure, and loss of feature should always be expected when opening a blend-file saved with a more recent version of Blender." / "A warning is shown in the UI when editing a more recent blend-file. Trying to overwrite it (with a simple ‘Save’ operation) will also show a confirmation popup, as this could make that loss of data permanent." / "In such cases, older Blender will fail opening (or appending/linking from) the newer blend-file, with a message stating which minimal version is needed to open it." / "When having issues with opening much older (or newer) blend-files, it can help to use a few intermediary Blender releases to perform conversions by smaller steps." — manual, https://docs.blender.org/manual/en/5.3/files/blend/compatibility.html, read 2026-10-11.
- "Scenes using a Sky Texture with **Sun Disc** enabled will look different from earlier versions, because the sun disc now contributes to the lighting. To restore the previous look, disable Sun Disc." — release notes, https://developer.blender.org/docs/release_notes/5.3/eevee/, read 2026-10-11.
- "Spline IK chains are now gracefully capped to 255 bones on file read … This means that when a future version of Blender increases the limit, Blender 5.3 will be able to read that new file without crashing." — release notes, https://developer.blender.org/docs/release_notes/5.3/animation_rigging/, read 2026-10-11.
- "version / File format version the .blend file was saved with" — API, https://docs.blender.org/api/5.3/bpy.types.BlendData.html; "bpy.app.handlers.version_update / on ending the versioning code" — API, https://docs.blender.org/api/5.3/bpy.app.handlers.html; both read 2026-10-11.
- `bpy.app.handlers.version_update.append(version_update.do_versions)` — `cycles_5.3/__init__.py`, line 156. `max_need_versioning = (5, 2, 8)` / `if version > max_need_versioning:` `continue` (lines 104–107); `# We don't modify startup file because it assumes to` (line 92); `if version <= (5, 0, 77):` … `cscene.use_auto_tile = True` `cscene.tile_size = 8192` (lines 115–119); `if version <= (4, 2, 52):` … `cscene.sampling_pattern = 'TABULATED_SOBOL'` (lines 263–268); `if version <= (3, 0, 25):` … `if not cscene.is_property_set("samples"):` `cscene.samples = 128` (lines 223–228) — `cycles_5.3/version_update.py`. `('TABULATED_SOBOL',` `n_("Classic"),` — `cycles_5.3/properties.py`, lines 107–108. Read 2026-10-16 (observed).

**Where I looked:** only the pages in Quotes above were cited; no search terms or additional pages were recorded for what happens when a newer file is opened from the command line.

**Not covered:** what is printed, if anything, when a newer file is opened from the command line (the warning is described for the interface); whether a render from a newer file is refused or proceeds.

**Outside the interface:** The documented warning and confirmation popup are interface behaviour. The sources do not say what happens instead when Blender is driven from outside (not in the sources). The Cycles conversion is registered on a file-load hook rather than in interface code (observed), so it applies to command-line renders too (inferred). **Observed 2026-10-17 (entry 41):** from a terminal with no window, Blender 5.2.2 given a 5.3 file printed `WARNING File written by newer Blender binary (503.29), expect loss of data!` and rendered it, exit status 0; 5.3 given a 5.2.2 file printed nothing about versions and rendered it. What 5.3 does with a newer file was not tried (no newer Blender exists).

**Operating system:** The sources do not say. **Observed 2026-10-17:** on macOS 26.6.2 on Apple silicon only; Linux and Windows were not available to this sitting.

### Observed in Blender 5.3 — hand tests of 2026-10-17

Got by running Blender, not from a source; it sits beside the answer above. **On what computer:** the Creator's Mac — a MacBook Pro with an Apple M1 Pro chip and its own 14-core graphics (Metal), 16 GB of memory, macOS 26.6.2. **What ran:** Blender 5.3.0 Beta, built 2026-10-16, and — because this test is about a file crossing between two versions — Blender 5.2.2 LTS, both started from a terminal with no window by Claude Opus 5.5 on 2026-10-17, with a throwaway settings folder and `--factory-startup`. Nothing ran on Linux or Windows. The tests are written out in full in `Audits/Blender Research/Settings/Hand tests — 5.3.md`, by the numbers given here. Label: **observed**.

**A file saved by 5.3 opened by 5.2.2, and one saved by 5.2.2 opened by 5.3 (entry 41).** Each Blender saved Blender's default scene on Workbench at 64 by 64; the other then opened it, printed the version the file was saved with, and rendered one frame. **5.2.2 given the 5.3 file**, exit status 0:

```
00:00.379  blend            | Read blend: "/private/tmp/claude-501/smoozy-hand-tests-5.3/Settings/files/from53.blend"
00:00.382  reports          | WARNING File written by newer Blender binary (503.29), expect loss of data!
Blender 5.2.2 LTS (hash d13f752e3b9c built 2026-09-15 01:49:19)
00:00.385  render           | Rendering single frame
00:00.385  render           | Rendering frame 1
00:00.385  render           | Start rendering: Scene, ViewLayer
00:00.385  render           | Engine: Workbench
00:00.535  render           | Saved: '/private/tmp/claude-501/smoozy-hand-tests-5.3/Settings/out/e41/in522_0001.png'
00:00.535  render           | Time: 00:00.14 (Saving: 00:00.06)
FILE VERSION (5, 3, 29)
Blender quit
```

**5.3 given the 5.2.2 file**, exit status 0, no warning of any kind:

```
00:00.312  blend            | Read blend: "/private/tmp/claude-501/smoozy-hand-tests-5.3/Settings/files/from522.blend"
00:00.317  render           | Rendering single frame
00:00.317  render           | Rendering frame 1
00:00.317  render           | Start rendering: Scene, ViewLayer
00:00.317  render           | Engine: Workbench
00:00.479  render           | Saved: '/private/tmp/claude-501/smoozy-hand-tests-5.3/Settings/out/e41/in53_0001.png'
00:00.479  render           | Time: 00:00.16 (Saving: 00:00.06)
Blender 5.3.0 Beta (hash 5d8151335b6b built 2026-10-16 01:36:01)
FILE VERSION (5, 2, 45)
Blender quit
```

So the forward-compatibility warning the manual places in the interface reaches the terminal of a render started from outside, in the words above, and the older Blender rendered the newer file rather than refusing it; the newer Blender opened the older file without a word.

**Not tried, and why.** What 5.3 prints on opening a file *newer* than 5.3: no newer Blender exists to write one, and a file with its version number edited by hand would be a damaged file, not a newer one. Cycles' own conversion of older files: the rules quoted above fire for files saved by Blender 5.0.77 or older, and the only other Blender in this sitting is 5.2.2, whose file reports version (5, 2, 45). How the warning is drawn inside a window: in 5.3 that needs a newer file too.

---

## 11. Can a file rely on scripts stored inside it to render correctly, and what decides whether they run when started from outside?

**Answer:**
- **Yes, a file can rely on them** (documented): "some blend-files need this to function properly". The ways a file runs scripts on its own are registered text blocks (run when the file loads) and Python expressions in drivers.
- **What decides, from outside** (documented): automatic running is off by default. The Preferences' "Auto Run Python Scripts" setting (with its excluded folders) still applies, and the command line overrides it: `-y` / `--enable-autoexec` turns it on, `-Y` / `--disable-autoexec` turns it off. The argument list marks `-Y` as "(default)".
- **Exceptions that run anyway**: rendering with Freestyle runs its scripts even with automatic running off; Freestyle style modules are stored in the file as text blocks (documented). Drivers that do not follow the "Simple Expressions" subset require opening the file as Trusted Source or setting Auto Run Python Scripts in Preferences (documented), so drivers that stay within the "Simple Expressions" subset work without that permission (**inferred**).
- A text block stored in the file can also be run on purpose with `--python-text <name>` (documented); whether that is subject to the auto-run setting is not stated (not in the sources).
- Scripts can check from inside whether running was allowed and whether it failed (documented API values).

**Label:** documented; inferred (that Simple Expressions drivers work without permission); not in the sources (whether `--python-text` is subject to auto-run, whether background mode with no flag follows Preferences or `-Y`, what a blocked script reports in background mode, and whether OSL scripts in Cycles Script nodes are governed by the same setting).

**Quotes**

- "Automatic execution is disabled by default; however, some blend-files need this to function properly." / "Registered Text-Blocks / A text data-block can have its Register option enabled which means it will load on start." / "Animation Drivers / Python expressions can be used to Drive values and are often used in more advanced rigs and animations." / "There are other ways scripts in a blend-file may execute that require user interaction (therefore will run even when auto execution is off)…" … "Rendering with Freestyle, because Freestyle uses scripts to control line styles." / "In the Preferences, there is the toggle to Auto Run Python Scripts. … Once enabled you have the option to exclude certain directories" / "You may want to perform batch rendering or some other task from the command line, running Blender without an interface. In this case, the Preferences are still used but you may want to override them: / Enable with -y or --enable-autoexec / Disable with -Y or --disable-autoexec" / "blender --background --enable-autoexec my_movie.blend --render-anim" — manual, https://docs.blender.org/manual/en/5.3/advanced/scripting/security.html, read 2026-10-11.
- "-Y, --disable-autoexec / Disable automatic Python script execution (Python-drivers & startup scripts), (default)." / "--python-text <name> / Run the given Python script text block." — manual, https://docs.blender.org/manual/en/5.3/advanced/command_line/arguments.html, read 2026-10-11.
- "If using a Scripted Expression Driver Type that doesn’t follow the Simple Expressions subset, you will have to open the file as Trusted Source, or set Auto Run Python Scripts in Preferences ‣ Save & Load ‣ Blender Files." — manual, https://docs.blender.org/manual/en/5.3/animation/drivers/troubleshooting.html, read 2026-10-11.
- "Python style modules are stored within blend-files as text data-blocks." — manual, https://docs.blender.org/manual/en/5.3/render/freestyle/python.html, read 2026-10-11.
- "Excluded Paths / Blend-files in these folders will not automatically run Python scripts." — manual, https://docs.blender.org/manual/en/5.3/editors/preferences/save_load.html, read 2026-10-11.
- "bpy.app.autoexec / Boolean, True when auto-execution is allowed (read-only)." / "bpy.app.autoexec_fail / Boolean, True when auto-execution of scripts failed (read-only)." / "bpy.app.autoexec_fail_message / String, message describing the auto-execution failure (read-only)." / "bpy.app.autoexec_override / The auto-execution set by the command line, None when the preference isn’t overridden (read-only)." — API, https://docs.blender.org/api/5.3/bpy.app.html, read 2026-10-11.

**Where I looked:** only the pages in Quotes above (plus `https://docs.blender.org/manual/en/5.3/render/shader_nodes/utilities/script.html`, read 2026-10-11, not quoted) were cited; no further search terms or pages were recorded.

**Not covered:** there is a tension between "the Preferences are still used" and "-Y … (default)". The sources do not settle whether a background render with no flag follows the Preferences or is always off. Also not covered: what a blocked script reports in background mode (the "dialog" is an interface element); whether OSL shader scripts stored in the file (Cycles Script node, https://docs.blender.org/manual/en/5.3/render/shader_nodes/utilities/script.html, read 2026-10-11) are governed by the same setting. "Scripts" in the question may or may not include OSL; I read it as Python.

**Outside the interface:** Covered above. The command-line flags exist only for outside starts; the "Trusted Source" checkbox and the warning dialog belong to the interface. **Observed 2026-10-17 (entries 42 to 44):** from a terminal with no window and neither switch, the preference decided whether a driver needing scripts and a registered text block ran; `-y` and `-Y` overrode it either way; a blocked driver gave 0, printed two lines, set `bpy.app.autoexec_fail`, and the render still went ahead with exit status 0; a text block named with `--python-text` ran even under `-Y`; and an Open Shading Language script stored in the file coloured the render under `-Y` as well.

**Operating system:** The sources do not say that this differs. **Observed 2026-10-17:** on macOS 26.6.2 on Apple silicon only; Linux and Windows were not available to this sitting.

### Observed in Blender 5.3 — hand tests of 2026-10-17

Got by running Blender, not from a source; it sits beside the answer above. **On what computer:** the Creator's Mac — a MacBook Pro with an Apple M1 Pro chip and its own 14-core graphics (Metal), 16 GB of memory, macOS 26.6.2. **What ran:** Blender 5.3.0 Beta, built 2026-10-16, started from a terminal with no window by Claude Opus 5.5 on 2026-10-17. These runs read preferences on purpose, so they used two throwaway settings folders made for them and **not** `--factory-startup`: one saved with Auto Run Python Scripts off, one with it on. None of the Creator's own settings was read. Nothing ran on Linux or Windows. The tests are written out in full in `Audits/Blender Research/Settings/Hand tests — 5.3.md`, by the numbers given here. Label: **observed**.

**A driver that needs scripts, under each preference and each switch (entry 42).** Three files of Blender's default scene on Workbench at 64 by 64, each with a scripted-expression driver on the cube's X position. After loading, a launch line set frame 1 and printed the cube's X and four values from `bpy.app`: `autoexec`, `autoexec_fail`, `autoexec_fail_message` and `autoexec_override`.

Two expressions inside the simple subset, with Auto Run off, under no switch, `-y` and `-Y`:

```
len("xxxxx")     no switch: CUBE X = 5.0   AUTOEXEC False False '' None
                 -y:        CUBE X = 5.0   AUTOEXEC True False '' True
                 -Y:        CUBE X = 5.0   AUTOEXEC False False '' False
sum([3.0,4.0])   no switch: CUBE X = 7.0   AUTOEXEC False False '' None
                 -y:        CUBE X = 7.0   AUTOEXEC True False '' True
                 -Y:        CUBE X = 7.0   AUTOEXEC False False '' False
```

The expression `__import__("os").getpid()*0+7`, rendered to a picture under each preference and each switch — every one of the six exited 0 and saved its picture:

```
pref off, no switch:  CUBE X = 0.0   AUTOEXEC False True 'Driver \'__import__("os").getpid()*0+7\'' None
pref off, -y:         CUBE X = 7.0   AUTOEXEC True False '' True
pref off, -Y:         CUBE X = 0.0   AUTOEXEC False True 'Driver \'__import__("os").getpid()*0+7\'' False
pref on,  no switch:  CUBE X = 7.0   AUTOEXEC True False '' None
pref on,  -y:         CUBE X = 7.0   AUTOEXEC True False '' True
pref on,  -Y:         CUBE X = 0.0   AUTOEXEC False True 'Driver \'__import__("os").getpid()*0+7\'' False
```

Every blocked run also printed these two lines before saving its picture:

```
	BPY_driver_exec: restricted access disallows name '__import__', enable auto-execution to support
Error in PyDriver: expression failed: __import__("os").getpid()*0+7
```

So with neither switch the preference decides; `-y` and `-Y` override it either way; drivers inside the simple subset gave their value under every combination; and a blocked driver gives 0 instead of its value while the render still goes ahead, the picture is still written and the exit status stays 0 — the run says so only in the two lines above and in `bpy.app.autoexec_fail`, which a script in the same run can read.

**A registered text block, and a text block run on purpose with `--python-text` (entry 43).** A file of Blender's default scene carrying two text blocks: `marker.py`, with Register ticked, printing `MARKER registered text block ran`; and `manual.py`, not registered, printing `MARKER python-text block ran`. Opened with Auto Run off and `--python-text manual.py` after the file, under each switch; every run exited 0:

```
no switch:
00:00.319  blend            | Read blend: "/private/tmp/claude-501/smoozy-hand-tests-5.3/Settings/files/texts.blend"
MARKER python-text block ran
scripts disabled for "/private/tmp/claude-501/smoozy-hand-tests-5.3/Settings/files/texts.blend", skipping 'marker.py'
-y:
00:00.322  blend            | Read blend: "/private/tmp/claude-501/smoozy-hand-tests-5.3/Settings/files/texts.blend"
MARKER registered text block ran
MARKER python-text block ran
-Y:
00:00.324  blend            | Read blend: "/private/tmp/claude-501/smoozy-hand-tests-5.3/Settings/files/texts.blend"
MARKER python-text block ran
scripts disabled for "/private/tmp/claude-501/smoozy-hand-tests-5.3/Settings/files/texts.blend", skipping 'marker.py'
```

So the registered block ran only under `-y` here, and when it did not run Blender printed one line naming it; a text block named with `--python-text` ran under all three, `-Y` included.

**An Open Shading Language script stored in the file (entry 44).** Blender's default scene on Cycles on the processor at 64 by 48, 4 samples, Open Shading Language switched on; the cube's material takes its colour from a Script node in Internal mode, reading a text block in the file that holds `shader red_out(output color Col = color(1, 0, 0)) { }`, through an Emission shader. The build compiled it with Cycles' own script-node update and printed `OSL compiled ok= True messages= [(['INFO'], 'OSL shader compilation succeeded')] outputs= ['Col'] bytecode characters= 335`. Rendered with Auto Run off under no switch, `-y` and `-Y`, each exited 0 and wrote its picture, and none printed an error or warning line. The pictures' first pixel (background) and middle pixel (the cube):

```
PIX oslnone_0001.png size (64, 48) first [0.2275, 0.2314, 0.2314, 1.0] middle [0.851, 0.2118, 0.1176, 1.0]
PIX osl-y_0001.png size (64, 48) first [0.2275, 0.2314, 0.2314, 1.0] middle [0.851, 0.2118, 0.1176, 1.0]
PIX osl-Y_0001.png size (64, 48) first [0.2275, 0.2314, 0.2314, 1.0] middle [0.851, 0.2118, 0.1176, 1.0]
PIX svm_0001.png size (64, 48) first [0.2235, 0.2275, 0.2314, 1.0] middle [0.7686, 0.7686, 0.7686, 1.0]
```

The last line is the control: the same file with Open Shading Language switched off at launch, where the cube comes out grey. So the script stored in the file gave the cube its red under all three, `-Y` included: the switch that governs Python scripts did not stop it.

---

## 12. Can a render started from outside be pointed at one named scene, and what does it render if not told which?

**Answer:**
- **Yes** (documented): `-S` / `--scene <name>` sets the active scene for rendering. A launch script could also use the render operator's `scene` argument, which defaults to the "current scene" (documented).
- **If not told:** the sources do not say which scene a command-line render picks. The wording of `-S` ("Set the active scene … for rendering") suggests the render uses the "active scene" (**inferred**). But no source says which scene is active when a file is opened from the command line. The API describes the active scene as belonging to a window, and a background start has no interface.

**Label:** documented (`-S`, operator); inferred (active scene); not in the sources (which scene that is).

**Quotes**

- "-S, --scene <name> / Set the active scene <name> for rendering." — manual, https://docs.blender.org/manual/en/5.3/advanced/command_line/arguments.html, read 2026-10-11.
- "bpy.ops.render.render(*, animation=False, write_still=False, use_viewport=False, use_sequencer_scene=False, layer='', scene='', frame_start=0, frame_end=0)" / "scene (str) – Scene, Scene to render, current scene if not specified" / "use_sequencer_scene (bool) – Use Sequencer Scene, Render the sequencer scene instead of the active scene" — API, https://docs.blender.org/api/5.3/bpy.ops.render.html, read 2026-10-11.
- "scene / Active scene to be edited in the window (never None)" — API, https://docs.blender.org/api/5.3/bpy.types.Window.html, read 2026-10-11.

**Where I looked:** searched the manual and API for "active scene", "current scene", "-S", "--scene", "window scene", "load_ui"; read the arguments page, command-line render page, scene introduction and properties pages, topbar Render menu, Sequencer Scene page, the render operator, Window and Context pages.

**Not covered:** what happens if the named scene does not exist; whether the separate "Sequencer Scene" (manual, https://docs.blender.org/manual/en/5.3/video_editing/sequencer_scene.html, read 2026-10-11) is involved in a command-line render.

**Outside the interface:** `-S` is the outside route. Inside, the "Render Image/Animation" menu items render "the active scene" (manual, https://docs.blender.org/manual/en/5.3/interface/window_system/topbar.html, read 2026-10-11), and the render operator has `use_sequencer_scene` to render the sequencer scene instead of the active scene (quoted above). **Observed 2026-10-17 (entries 35, 36):** from a terminal with no window, with no `-S` the render used the scene showing in the window when the file was saved, a wrong `-S` name was reported in one line and ignored, and the Sequencer Scene had no effect.

**Operating system:** The sources do not say. **Observed 2026-10-17:** on macOS 26.6.2 on Apple silicon only; Linux and Windows were not available to this sitting.

### Observed in Blender 5.3 — hand tests of 2026-10-17

Got by running Blender, not from a source; it sits beside the answer above. **On what computer:** the Creator's Mac — a MacBook Pro with an Apple M1 Pro chip and its own 14-core graphics (Metal), 16 GB of memory, macOS 26.6.2. **What ran:** Blender 5.3.0 Beta, built 2026-10-16, started from a terminal by Claude Opus 5.5 on 2026-10-17, with a throwaway settings folder and `--factory-startup`. Nothing ran on Linux or Windows. The tests are written out in full in `Audits/Blender Research/Settings/Hand tests — 5.3.md`, by the numbers given here. Label: **observed**.

**Which scene renders when none is named, and what a wrong name does (entry 35).** A file saved with two scenes, `Alpha` (Blender's default cube, camera and light) and `Beta` (a camera only), both Workbench at 64 by 64, with `Beta` made the window's scene before saving; the build printed `MADE scenes= ['Alpha', 'Beta'] window scene= Beta`. Each render carried a handler, added at launch, that prints the scene a render starts on: `import bpy; bpy.app.handlers.render_pre.append(lambda scene, *a: print('RENDER_PRE SCENE:', scene.name))`.

```
no -S:            Start rendering: Beta, ViewLayer    RENDER_PRE SCENE: Beta    exit 0
-S Alpha:         Start rendering: Alpha, ViewLayer   RENDER_PRE SCENE: Alpha   exit 0
-S NoSuchScene:   Start rendering: Beta, ViewLayer    RENDER_PRE SCENE: Beta    exit 0
```

The run with the wrong name also printed `Can't find scene: 'NoSuchScene' in file: '/private/tmp/claude-501/smoozy-hand-tests-5.3/Settings/files/twoscene.blend'`, then saved its picture. So with no `-S`, a render started from outside used the scene that was showing in the window when the file was saved, not the first scene; a name no scene bears is reported in one line and then ignored, and the run neither stops nor changes its exit status.

**A Sequencer Scene that differs from the active scene (entry 36).** A file with three scenes — `Alpha`, `Beta`, and `Edit`, whose sequencer holds a Scene strip of `Alpha` — the workspace's Sequencer Scene set to `Edit` and the window's scene to `Beta` (the build printed `MADE workspace Layout sequencer_scene= Edit strip scene= Alpha` and `MADE scenes= ['Alpha', 'Beta', 'Edit'] window scene= Beta`). A bare render printed `Start rendering: Beta, ViewLayer` and `RENDER_PRE SCENE: Beta`, exit status 0. So the Sequencer Scene had no effect on a render started from outside.

**Not settled here.** Both files were built and saved by a Blender with no window, by setting the window's scene from a script. Whether a file saved from Blender's own window records the active scene the same way was not tried.

---

## 13. What decides the list of render engines a file can choose between, and can that list grow on a given computer?

**Answer:**
- The list is the engines available in the running Blender, not something fixed by the file (**inferred**). Blender comes with EEVEE, Cycles and Workbench (documented). More come as add-ons (documented). Cycles is itself an add-on that adds its engine to the list when it is turned on (observed).
- **Yes, the list can grow** (documented): third-party renderers are installed as add-ons. The API documents how any add-on or script adds an engine. An add-on engine can also be built on USD Hydra.
- **From outside:** `-E help` lists the available engines; `--addons` turns on add-ons for that start.
- **Inferred:** because any registered script can add an engine, a script given at launch could add one too.
- The file records only which engine is chosen (the render setting `engine` on `RenderSettings`, documented; that it is stored in the saved file is **inferred**, as in question 4).

**Label:** documented; observed (Cycles is an add-on engine); inferred (as marked); not in the sources (the Workbench engine identifier, what happens when a file names an unavailable engine, and whether `--addons` suffices for one render).

**Quotes**

- "Blender provides the following render engines: / EEVEE / Cycles / Workbench" / "More renderers from third-party developers are available as add-ons." — manual, https://docs.blender.org/manual/en/5.3/render/render_engines.html, read 2026-10-11.
- "-E, --engine <engine> / Specify the render engine. Use -E help to list available engines." / "--addons <addon(s)> / Comma separated list (no spaces) of add-ons to enable in addition to any default add-ons." — manual, https://docs.blender.org/manual/en/5.3/advanced/command_line/arguments.html, read 2026-10-11.
- "engine / Engine to use for rendering (default 'BLENDER_EEVEE')" … "Literal[‘BLENDER_EEVEE’]" / "has_multiple_engines / More than one rendering engine is available (default False, readonly)" — API, https://docs.blender.org/api/5.3/bpy.types.RenderSettings.html, read 2026-10-11. (This API page lists only one value, while the manual names three built-in engines; so the API page's list is not the full list in a running Blender — inferred.)
- "class CustomRenderEngine(bpy.types.RenderEngine): / # These three members are used by Blender to set up the / # RenderEngine; define its internal name, visible name and capabilities. / bl_idname = "CUSTOM" / bl_label = "Custom"" and "bpy.utils.register_class(CustomRenderEngine)" — API, https://docs.blender.org/api/5.3/bpy.types.RenderEngine.html, read 2026-10-11.
- "Base class for integrating USD Hydra based renderers." — API, https://docs.blender.org/api/5.3/bpy.types.HydraRenderEngine.html, read 2026-10-11.
- "The add-on functionality should be immediately available." — manual, https://docs.blender.org/manual/en/5.3/editors/preferences/addons.html, read 2026-10-11.
- `"support": 'OFFICIAL',` `"category": "Render"}` (lines 15–16); `class CyclesRender(bpy.types.RenderEngine):` `bl_idname = 'CYCLES'` (lines 44–45) — `cycles_5.3/__init__.py`, read 2026-10-16 (observed).

**Where I looked:** only the pages in Quotes above were cited; no search terms or additional pages were recorded for the Workbench engine identifier.

**Not covered:** what happens when a file names an engine that is not available on the computer; the identifier for Workbench (not found in the cited sources); whether turning an add-on on with `--addons` is enough for its engine to appear for that one render.

**Outside the interface:** `-E help`, `-E <name>` and `--addons` are the outside route. Inside, the engine is chosen in Render Properties, and add-ons are managed in Preferences. **Observed 2026-10-17 (entries 2, 13, 45, 46, 64):** `-E help` prints `BLENDER_EEVEE`, `BLENDER_WORKBENCH` and `CYCLES`; it grew by an engine a launch script registered and shrank when the Cycles add-on was switched off; a file naming an engine not registered was reported, read back as EEVEE and rendered on EEVEE. In the window the Render Engine menu offers EEVEE, Workbench and Cycles. Whether `--addons` turns on a real third-party engine early enough was not tried — none is installed.

**Operating system:** The sources do not say that the built-in list differs by operating system. **Observed 2026-10-17:** on macOS 26.6.2 on Apple silicon only; Linux and Windows were not available to this sitting.

### Observed in Blender 5.3 — hand tests of 2026-10-17

Got by running Blender, not from a source; it sits beside the answer above. **On what computer:** the Creator's Mac — a MacBook Pro with an Apple M1 Pro chip and its own 14-core graphics (Metal), 16 GB of memory, macOS 26.6.2. **What ran:** Blender 5.3.0 Beta, built 2026-10-16, started from a terminal by Claude Opus 5.5 on 2026-10-17, with a throwaway settings folder and, unless said otherwise, `--factory-startup`. Nothing ran on Linux or Windows. The tests are written out in full in `Audits/Blender Research/Settings/Hand tests — 5.3.md`, by the numbers given here. Label: **observed**.

**What `-E help` prints (entry 2).** Run three ways — `blender -E help` with no other argument, `blender -b -E help`, and `blender -b --factory-startup -E help` — each exited 0 and printed the same listing; the two with `-b` print the version line first:

```
Blender 5.3.0 Beta (hash 5d8151335b6b built 2026-10-16 01:36:01)
Blender Engine Listing:
	BLENDER_EEVEE
	BLENDER_WORKBENCH
	CYCLES
```

So Workbench's identifier is `BLENDER_WORKBENCH`, and the listing gives identifiers, not the labels a person sees. A name no engine bears is refused with `Error: engine not found 'NOT_A_REAL_ENGINE'`, exit status 1 and no picture (quoted under question 1).

**What the engine setting itself offers, read from the program (entry 13).** Asked for the choices on the engine setting, a factory-startup Blender with no window answered, from the setting's type and from the scene's own setting:

```
ENGINE ENUM (type) = ['BLENDER_EEVEE']
ENGINE ENUM (this scene) = [('BLENDER_EEVEE', 'EEVEE')]
ENGINE CLASSES = ['HydraRenderEngine', 'CYCLES']
```

One entry, where `-E help` lists three; and the separately registered engine classes are Cycles and the Hydra base, with Workbench in neither list. So the list of engines cannot be read off the setting's choices this way; `-E help` is what prints it.

**A file whose engine is not available — the Cycles add-on switched off (entry 45).** A file of Blender's default scene saved with the engine `CYCLES`, 4 samples, 64 by 64. Read back by a factory-startup Blender: `ENGINE 'CYCLES' has_multiple_engines True choices ['BLENDER_EEVEE']`. A throwaway settings folder was then saved with the Cycles add-on switched off, leaving `['bl_pkg', 'io_anim_bvh', 'io_curve_svg', 'io_mesh_uv_layout', 'io_scene_fbx', 'io_scene_gltf2', 'pose_library']`. Opening and rendering the same file there, exit status 0:

```
00:00.313  reports          | ERROR Engine 'CYCLES' not available for scene 'Scene' (an add-on may need to be installed or enabled)
00:00.314  render           | Engine: EEVEE
00:00.446  render           | Fra: 1 | Rendering 1 / 64 samples
00:00.632  render           | Fra: 1 | Rendering 25 / 64 samples
00:00.784  render           | Fra: 1 | Rendering 50 / 64 samples
00:00.869  render           | Fra: 1 | Rendering 64 / 64 samples
00:00.959  render           | Saved: '/private/tmp/claude-501/smoozy-hand-tests-5.3/Settings/out/e45/n_0001.png'
ENGINE 'BLENDER_EEVEE' has_multiple_engines True choices ['BLENDER_EEVEE']
```

and `-E help` in that folder listed:

```
Blender Engine Listing:
	BLENDER_EEVEE
	BLENDER_WORKBENCH
```

So the list of engines is what the running Blender has registered — it shrank when the add-on was switched off — and a file whose saved engine is not registered is reported in one `ERROR` line, read back as `BLENDER_EEVEE`, and rendered on EEVEE at EEVEE's own 64 samples rather than refused; the exit status does not show it.

**An engine added by a script at launch (entry 46).** A script holding only the lines the API page's example is quoted for above — a render engine class with `bl_idname = "CUSTOM"` and `bl_label = "Custom"`, registered with `bpy.utils.register_class` — was handed in with `--python` ahead of `-E help`:

```
REGISTERED CUSTOM; engine choices now: ['BLENDER_EEVEE']
Blender Engine Listing:
	BLENDER_EEVEE
	BLENDER_WORKBENCH
	CYCLES
	CUSTOM
```

So the listing grows by the engine a launch script registers; the setting's own list of choices, read from Python, still shows one entry. A file was saved with the scene's engine set to `CUSTOM` (`SET ENGINE CUSTOM`). Opened **without** the script, it printed:

```
00:00.322  reports          | ERROR Engine 'CUSTOM' not available for scene 'Scene' (an add-on may need to be installed or enabled)
00:00.323  render           | Engine: EEVEE
00:01.449  render           | Saved: '/private/tmp/claude-501/smoozy-hand-tests-5.3/Settings/out/e46/n_0001.png'
ENGINE AS LOADED 'BLENDER_EEVEE'
```

exit status 0. Opened **with** the script registered first (`--python custom_engine.py custom.blend …`), the engine name survived — `ENGINE AS LOADED 'CUSTOM'` — and the render printed `Rendering single frame`, `Rendering frame 1`, then `Saved: '…/out/e46/c_0001.png'`, exit status 0, with no `Start rendering` or `Engine:` line. That engine draws nothing (the class has no drawing of its own), and its picture, 1920 by 1080, is transparent black: first and middle pixel `[0.0, 0.0, 0.0, 0.0]`. In Blender's own window, opened from the terminal with the same script handed in first, the Render Engine menu opened by a simulated click holds four entries: **EEVEE**, **Workbench**, **Cycles**, **Custom**.

What this does not reach: a real third-party engine add-on, and whether `--addons` turns one on early enough for `-E`. None is installed on this Mac.

**The Render Engine menu, opened in Blender's own window (entry 64).** Blender 5.3 was opened with its window from the terminal (a throwaway settings folder), Cycles chosen, Render Properties shown, and the left mouse button pressed over the Render Engine menu by simulated input. The photographed menu holds exactly three entries, in this order: **EEVEE**, **Workbench**, **Cycles** — Cycles highlighted as the one chosen; the row beneath the menu is `Device`. So on a Blender with no extra add-on the control offers those three under their labels, not the identifiers `-E` takes.

---

## 14. What values can a render's resolution percentage be set to?

**Answer:** Whole numbers from 1 to 32767; default 100 (documented). The manual describes it as a slider that scales the rendered size relative to the X and Y resolution. The manual's Format panel picture shows the field as "100%". The API's example engine and the path-template page show it applied as a fraction of the resolution (e.g. 50% of 1000×600 gives 500×300).

**Label:** documented. (That only whole numbers are accepted is read from "Type: int".)

**Quotes**

- "resolution_percentage / Percentage scale for render resolution (in [1, 32767], default 100) / Type: / int" — API, https://docs.blender.org/api/5.3/bpy.types.RenderSettings.html, read 2026-10-11.
- "Percentage / Slider to reduce or increase the size of the rendered image relative to the Resolution values. This is useful for small test renders that have the same proportions as the final image." — manual, https://docs.blender.org/manual/en/5.3/render/output/properties/format.html, read 2026-10-11. Picture `_images/render_output_properties_format_panel.png` looked at: it shows rows "Resolution X 1920 px", "Y 1080 px", "% 100%".
- "This factors in the resolution scale as well, so if the scene resolution is 1000x600 and the scale is 50%, then resolution_x and resolution_y will be 500 and 300, respectively." — manual, https://docs.blender.org/manual/en/5.3/files/file_paths.html, read 2026-10-11.
- "scale = scene.render.resolution_percentage / 100.0 / self.size_x = int(scene.render.resolution_x * scale)" — API, https://docs.blender.org/api/5.3/bpy.types.RenderEngine.html, read 2026-10-11.

**Not covered:** whether the slider in the interface stops at a lower "soft" maximum than 32767 (the API pages do not print soft limits); what happens when the scaled size becomes very small or very large.

**Outside the interface:** The same range applies to the API value a script sets (documented). There is no command-line option for it (question 1). **Observed 2026-10-17 (entries 47, 7):** from a terminal with no window, the setting holds an outright range of 1 to 32767 and a dragging range of 1 to 100, takes whole numbers only, and scales the written picture.

**Operating system:** The sources do not say. **Observed 2026-10-17:** on macOS 26.6.2 on Apple silicon only; Linux and Windows were not available to this sitting.

### Observed in Blender 5.3 — hand tests of 2026-10-17

Got by running Blender, not from a source; it sits beside the answer above. **On what computer:** the Creator's Mac — a MacBook Pro with an Apple M1 Pro chip and its own 14-core graphics (Metal), 16 GB of memory, macOS 26.6.2. **What ran:** Blender 5.3.0 Beta, built 2026-10-16, started from a terminal by Claude Opus 5.5 on 2026-10-17, with a throwaway settings folder and, unless said otherwise, `--factory-startup`. Nothing ran on Linux or Windows. The tests are written out in full in `Audits/Blender Research/Settings/Hand tests — 5.3.md`, by the numbers given here. Label: **observed**.

**What the running program holds for the setting, and what values it takes (entry 47).** A factory-startup Blender with no window printed:

```
PROP resolution_percentage type = 'INT'
PROP resolution_percentage default = 100
PROP resolution_percentage hard_min = 1
PROP resolution_percentage hard_max = 32767
PROP resolution_percentage soft_min = 1
PROP resolution_percentage soft_max = 100
PROP resolution_percentage step = 10
PROP resolution_percentage is_animatable = False
PROP resolution_percentage is_readonly = False
PROP resolution_percentage current value = 100
```

So the pair the API page prints, 1 and 32767, is the outright pair, and the setting carries a narrower pair, 1 and 100, for dragging. A series of values was then assigned and read back:

```
SET 1 -> 1
SET 100 -> 100
SET 32767 -> 32767
SET 0 -> 1
SET -50 -> 1
SET 32768 -> 32767
SET 40000 -> 32767
SET 2147483648 -> RAISED ValueError: bpy_struct: item.attr = val: RenderSettings.resolution_percentage value not in 'int' range ((-2147483647 -1), 2147483647)
SET 50.7 -> RAISED TypeError: bpy_struct: item.attr = val: RenderSettings.resolution_percentage expected an int type, not float
SET 0.4 -> RAISED TypeError: bpy_struct: item.attr = val: RenderSettings.resolution_percentage expected an int type, not float
SET '50' -> RAISED TypeError: bpy_struct: item.attr = val: RenderSettings.resolution_percentage expected an int type, not str
SET 'abc' -> RAISED TypeError: bpy_struct: item.attr = val: RenderSettings.resolution_percentage expected an int type, not str
```

Whole numbers from 1 to 32767 are stored as given; a fraction is refused, not rounded. What happens to the values outside the range is under question 15.

**Where the installed program reads the setting (entry 48).** Every Python file inside the installed Blender 5.3.0 Beta (`/Applications/Blender 5.3b.app/Contents/Resources/5.3/scripts`) was searched for `resolution_percentage`: 23 lines, 18 of them in render preset files, and these five elsewhere:

```
…/scripts/startup/bl_operators/presets.py:378:        "scene.render.resolution_percentage",
…/scripts/startup/bl_ui/properties_output.py:119:        col.prop(rd, "resolution_percentage", text="%")
…/scripts/modules/_rna_manual_reference.py:706:    ("bpy.types.rendersettings.resolution_percentage*", "render/output/properties/format.html#bpy-types-rendersettings-resolution-percentage"),
…/scripts/modules/_bl_previews_utils/bl_previews_render.py:123:        scene.render.resolution_percentage = 100
…/scripts/addons_core/cycles/ui.py:414:            render_scale = scene.render.resolution_percentage / 100.0
```

The one line in Cycles' own files is in its panel-drawing file. The same search for `texture_resolution` — a different setting, Cycles' Simplify texture resolution — returned five lines: a manual-reference entry, two property definitions in `cycles/properties.py` (lines 1070 and 1078) and two panel lines in `cycles/ui.py` (2383 and 2411).

**It cannot take a keyframe (entry 14).** Asked for one at frame 1, Blender answered `ERROR TypeError bpy_struct.keyframe_insert() property "resolution_percentage" not animatable`.

**It scales the picture (entry 7).** The default scene at 1920 by 1080 wrote a picture of 1920 by 1080 at 100 per cent and of 480 by 270 with the percentage set to 25 at launch (written up under question 17).

**How the panel shows it (entry 60).** Photographed in Blender's own window, Output Properties ▸ Format shows the setting on a row labelled only `%`, under `Resolution X` and `Y`, as a filled bar reading `100%`.

---

## 15. What does a render do when asked for a resolution percentage outside what it accepts, and what does it report?

**Answer:** The value is clamped to the nearest end of the range — below 1 becomes 1, above 32767 becomes 32767 — and nothing is reported. **Inferred.**

The step: the API says that for an integer setting, "trying to assign a value below [the hard minimum] will silently assign this minimum instead", and the same for the maximum. The resolution percentage is an integer setting whose range the API prints as [1, 32767]. Taking that printed range to be its hard range is the inference; the API does not say in so many words what its "in [a, b]" notation means.

**Label:** inferred.

**Quotes**

- "hard_min / Hard minimum, trying to assign a value below will silently assign this minimum instead" / "hard_max / Hard maximum, trying to assign a value above will silently assign this maximum instead" — API, https://docs.blender.org/api/5.3/bpy.types.IntProperty.html ("RNA integer number property definition"), read 2026-10-11.
- "resolution_percentage / Percentage scale for render resolution (in [1, 32767], default 100)" — API, https://docs.blender.org/api/5.3/bpy.types.RenderSettings.html, read 2026-10-11.

**Where I looked:** searched the API for "out of range", "outside the range", "clamped", "silently", "hard limit", "soft_min", "hard_min", "TypeError", "ValueError"; read the bpy.props, IntProperty and FloatProperty pages and the API's "gotcha" and quick-start pages; searched the manual's interface-controls pages (number fields) and custom-properties page for limits and clamping.

**Not covered:** what happens with a value of the wrong kind (e.g. a fraction or text) — not in the sources; whether anything is logged; whether the render itself (rather than the setting) ever sees an out-of-range value.

**Outside the interface:** From outside, the value can only be given by a script (question 1). If a launch script raises an error, `--python-exit-code` can turn that into an exit code: "Set the exit-code in [0..255] to exit if a Python exception is raised (only for scripts executed from the command line), zero disables." (manual, https://docs.blender.org/manual/en/5.3/advanced/command_line/arguments.html, read 2026-10-11). Since an out-of-range number is clamped "silently", it would not raise such an error (inferred). **Observed 2026-10-17 (entries 47, 49, 50):** from a terminal with no window, a value outside the range was corrected to the nearest end with nothing printed; `--python-exit-code` stayed at 0 for it against a control that came back 42; and a render then worked to the corrected value — a picture too small for Blender was refused with `ERROR Image too small` and exit status 0, and 40000 per cent began rendering at 32767 per cent without a word. What the `%` field in the window does with a typed value outside the range was not tried.

**Operating system:** The sources do not say. **Observed 2026-10-17:** on macOS 26.6.2 on Apple silicon only; Linux and Windows were not available to this sitting.

### Observed in Blender 5.3 — hand tests of 2026-10-17

Got by running Blender, not from a source; it sits beside the answer above. **On what computer:** the Creator's Mac — a MacBook Pro with an Apple M1 Pro chip and its own 14-core graphics (Metal), 16 GB of memory, macOS 26.6.2. **What ran:** Blender 5.3.0 Beta, built 2026-10-16, started from a terminal with no window by Claude Opus 5.5 on 2026-10-17, with a throwaway settings folder and `--factory-startup`. Nothing ran on Linux or Windows. The tests are written out in full in `Audits/Blender Research/Settings/Hand tests — 5.3.md`, by the numbers given here. Label: **observed**.

**What happens to the value, and whether anything is said (entry 47).** Assigned from a script and read back, `0` and `-50` were stored as `1`, `32768` and `40000` as `32767`, and nothing was printed for any of the four. In the same run, `2147483648` raised `ValueError: bpy_struct: item.attr = val: RenderSettings.resolution_percentage value not in 'int' range ((-2147483647 -1), 2147483647)` and the fractions and text raised `TypeError` (all quoted under question 14). Those raised lines are the control: this channel carries a complaint when there is one, so the silence for the four corrected values is a result, not a channel that never speaks.

**Whether whatever started Blender is told (entry 49).** Two launches with `--python-exit-code 42`, the switch that turns a Python failure into a chosen exit status. The control, a script that prints a line and then raises an error on purpose:

```
Traceback (most recent call last):
  File "/private/tmp/claude-501/smoozy-hand-tests-5.3/Settings/s/control.py", line 2, in <module>
    raise ZeroDivisionError("deliberate control error")
ZeroDivisionError: deliberate control error
CONTROL about to raise
Error: script failed, file: 's/control.py', exiting.
```

exit status **42**. A script that assigns 40000 and reads it back printed `READBACK 32767` and exited **0**. Because the control really did come back with 42, the 0 is a result: a value corrected on the way in is not signalled to whatever started Blender.

**What a render does with each percentage, by two routes (entry 50).** Each case started Blender with no window on the factory scene, set the picture's pixel size and the percentage from a script, and rendered one frame with Cycles on the processor at 1 sample — by route A, Blender's own `-f 1` after the script, and by route B, the render operator called inside the script with `bpy.ops.render.render(write_still=True)`. The size of each written picture was read from the file with `sips`.

```
asked 1920 1080 at 200     stored 200     A: 3840 by 2160   B: 3840 by 2160   (control)
asked 1920 1080 at 33      stored 33      A: 633 by 356     B: 633 by 356
asked 1920 1080 at 0       stored 1       A: 19 by 10       B: 19 by 10
asked 4 4 at 1             stored 1       A: no picture     B: no picture
asked 4 4 at 32767         stored 32767   A: 1310 by 1310   B: 1310 by 1310
asked 1920 1080 at 40000   stored 32767   both: started rendering, stopped by a ceiling, no picture
```

Every case but the last exited 0 by both routes. **The refusal at 4 by 4 and 1 per cent**, route A, in full:

```
ASKED 4 4 1
STORED 4 4 1
00:00.304  render           | Rendering single frame
00:00.304  reports          | ERROR Image too small
Blender 5.3.0 Beta (hash 5d8151335b6b built 2026-10-16 01:36:01)

Blender quit
```

and route B:

```
ASKED 4 4 1
STORED 4 4 1
ROUTE B -- the frame is rendered by calling the render operator from inside this script
Traceback (most recent call last):
  File "/private/tmp/claude-501/smoozy-hand-tests-5.3/Settings/s/e51.py", line 15, in <module>
    bpy.ops.render.render(write_still=True)
    ~~~~~~~~~~~~~~~~~~~~~^^^^^^^^^^^^^^^^^^
RuntimeError: Error: Image too small

Error: Image too small
Blender 5.3.0 Beta (hash 5d8151335b6b built 2026-10-16 01:36:01)

Blender quit
```

both with exit status 0 and no file. **The two routes print differently:** route A printed 54 to 60 lines for a finished render and wrote `A_<case>_0001.png`; route B printed 7 lines — the script's own three, Blender's `Saved:` line, the version line, a blank line and `Blender quit` — with no Cycles progress at all, and wrote `B_<case>_.png` from the same kind of output path. **At 40000 per cent** the value was stored as 32767 (`ASKED 1920 1080 40000`, `STORED 1920 1080 32767`) and the render began with no refusal and no warning. Under route A a 90-second ceiling stopped it (exit status 142, which is the ceiling's); its 531 lines held no line containing `Error`, `ERROR`, `Warning` or `WARNING`, and the last before the ceiling was:

```
01:29.463  render           | Fra: 1 | Remaining: 02:42:45.70 | Mem: 307M | Rendered 481/53284 Tiles, Sample 1/1
```

Under route B a 60-second ceiling stopped it and it printed only the script's own three lines, nothing of Blender's. The 200 per cent case is the control: it came out at 3840 by 2160 by both routes, so the measuring works and a plain percentage does scale the picture.

**Reading a file's settings from outside without rendering it (entry 51).** From an empty folder, a file of Blender's default scene on Cycles at 64 by 48 was opened with no window and a Python line printing its settings, with no `-f` or `-a`. The whole output, exit status 0:

```
00:00.316  blend            | Read blend: "/private/tmp/claude-501/smoozy-hand-tests-5.3/Settings/files/cyc64.blend"
SETTINGS CYCLES 64 48 100 4096 [('ViewLayer', True)]
Blender 5.3.0 Beta (hash 5d8151335b6b built 2026-10-16 01:36:01)
Blender quit
```

No render line appeared and the folder stayed empty.

---

## 16. When a render started from outside is given a sample count of its own, what decides how many samples it takes, and what else has a say in when it stops?

**Answer:** The count handed in is only one input. The sources name these others:

- **Which engine's count it is.** Cycles, EEVEE and Workbench each have their own (question 4). A count put into Cycles' setting does nothing for an EEVEE render (inferred from separate settings).
- **When it is applied.** A setting given before the file loads is overwritten by the file (documented for the output path; inferred for any setting).
- **View-layer overrides.** Each view layer may carry its own count (0 = use the scene's).
  - In Cycles, the scene's "Layer Samples" mode decides: **Use** (the layer's count replaces the scene's — this is the default, observed), **Bounded** (the layer's count is used only up to the scene's), or **Ignore**.
  - In EEVEE, a layer's override is always used.
  - So a scene count handed in can be replaced, layer by layer, by counts stored in the file (inferred).
- **Cycles adaptive sampling** (on by default, observed). The count becomes a maximum ("Max Samples"); areas stop early once below the Noise Threshold, after at least "Min Samples".
- **Time limit** (Cycles and EEVEE). It stops the render even if the count is not reached.
- **Cycles Sample Subset.** When on, it renders only "Length" samples starting at "Offset"; that "overrides (Max) Samples in terms of the samples that will get rendered".
- **Workbench.** The count is one of fixed choices: `5, 8, 11, 16, 32` under Multisample (documented on the Workbench sampling page, quoted in question 18), or `OFF, FXAA, 5, 8, 11, 16, 32` on `Scene.display.render_aa` (documented on `SceneDisplay`; inferred to be Workbench's setting, see question 4).

**Label:** documented; observed (defaults); inferred (as marked).

**Quotes**

- "Layer Samples / If any view layers have Sample Overrides configured, this option specifies how to use them. / Use: / Allow view layers to override the scene-level sample count. / Bounded: / Allow the overrides as long as they don’t exceed the scene-level sample count. / Ignore: / Ignore the overrides." / "Length / The number of samples in the subset. While this overrides (Max) Samples in terms of the samples that will get rendered, it’s still important to set (Max) Samples to the total number of samples that will be rendered across all subsets" — manual, https://docs.blender.org/manual/en/5.3/render/cycles/render_settings/sampling.html, read 2026-10-11 (plus the Time Limit and Adaptive Sampling quotes in question 2).
- "When using EEVEE, the Samples override is always used." — manual, https://docs.blender.org/manual/en/5.3/render/layers/override.html, read 2026-10-11.
- `use_layer_samples: EnumProperty(` … `default='USE',` (`cycles_5.3/properties.py`, lines 604–609); `('USE', "Use", "Per render layer number of samples override scene samples"),` `('BOUNDED', "Bounded", "Bound per render layer number of samples by global samples"),` `('IGNORE', "Ignore", "Ignore per render layer number of samples"),` (lines 66–70); `if cscene.use_adaptive_sampling:` `col.prop(cscene, "samples", text="Max Samples")` (`cycles_5.3/ui.py`, lines 327–328); `if view_layer.samples > 0:` … `layout.row().prop(cscene, "use_layer_samples")` (`cycles_5.3/ui.py`, lines 499–501). Read 2026-10-16 (observed).
- Argument-order quote: see question 1.

**Not covered:** the exact rule Cycles applies in "Bounded" beyond the quoted words; whether the time limit and adaptive sampling apply per view layer; whether path guiding training ("Training Samples") affects the count rendered (the manual says only that it limits training).

**Outside the interface:** "Render Single Layer" is ignored from the command line (question 1), so every enabled view layer renders, each with its own override if it has one (inferred). The interface hides the "Layer Samples" choice unless some view layer has an override (inferred from the observed condition `if view_layer.samples > 0:` in `cycles_5.3/ui.py` lines 499–501); from outside that setting still applies whether shown or not (inferred). **Observed 2026-10-17 (entries 6, 22 to 28, 54):** in renders started from a terminal with no window, a count supplied after the file and before `-f` was used and one supplied after `-f` was not; Layer Samples `USE`, `BOUNDED` and `IGNORE` behaved as written up above, a layer's own number standing, capped or ignored; adaptive sampling, a sample subset and a time limit each ended a frame below the count; the viewport settings changed nothing; "Render Single Layer" did not stop a second layer rendering at its own count; and an interrupt cancelled the frame with no picture.

**Operating system:** The sources do not say. **Observed 2026-10-17:** on macOS 26.6.2 on Apple silicon only, every Cycles render on the processor; Linux, Windows and a graphics card were not tried in this sitting.

### Observed in Blender 5.3 — hand tests of 2026-10-17

Got by running Blender, not from a source; it sits beside the answer above. **On what computer:** the Creator's Mac — a MacBook Pro with an Apple M1 Pro chip and its own 14-core graphics (Metal), 16 GB of memory, macOS 26.6.2. **What ran:** Blender 5.3.0 Beta, built 2026-10-16, started from a terminal by Claude Opus 5.5 on 2026-10-17, with a throwaway settings folder and `--factory-startup`. Every Cycles render was on the processor. Nothing ran on Linux or Windows, and other programs were running on the Mac at the time, so the times below belong to this machine on this day and are not compared with any other run's. The tests are written out in full in `Audits/Blender Research/Settings/Hand tests — 5.3.md`, by the numbers given here. Label: **observed**.

**The test scene.** Built by Blender 5.3 itself from a script, so no file saved by another Blender is involved: 320 by 240, Cycles on the processor, a plane, a rough metal sphere, a glass cube, one large area light and a dark world. The script is the one the earlier versions' hand tests of 2026-08-10 supplied, unchanged, and is written out in full in the appendix of the hand-test list named above. **How a count is read:** the last `Sample n/N` line with a non-zero `n` is the count reached and `N` the count worked towards; EEVEE prints `Rendering n / N samples` instead.

**The count reached under each combination of settings (entry 22).** Each row is one launch, `blender -b --factory-startup scene.blend --python-expr "<settings>" -o … -F PNG -f 1`, with every setting not named left as the scene was saved. Every row exited 0.

| Settings handed in | Last count line | Blender's `Time:` |
|---|---|---|
| Max Samples 4096, adaptive off | `Sample 4096/4096` | `01:22.75` |
| Max Samples 4096, adaptive on at 0.01 | `Sample 4096/4096` | `00:23.00` |
| Max Samples 4096, adaptive on at 0.5 | `Sample 32/4096` | `00:00.98` |
| Max Samples 2048, adaptive off, sample subset off | `Sample 2048/2048` | `00:42.00` |
| Max Samples 2048, adaptive off, subset on at Offset 0, Length 256 | `Sample 256/256` | `00:05.37` |
| the same, adaptive on at 0.01 | `Sample 256/256` | `00:05.02` |
| the same as the subset row, plus a 2-second Time Limit | `Sample 129/256` | `00:02.81` |
| Max Samples 200, adaptive on at 0.01, viewport count 4, viewport adaptive on at 0.5, viewport Min Samples 1 | `Sample 200/200` | `00:03.23` |
| Max Samples 200, adaptive on at 0.01, viewport count 3000, viewport adaptive off at 0.001, viewport Min Samples 512 | `Sample 200/200` | `00:03.23` |
| Max Samples 8, Min Samples 64 | `Sample 8/8` | `00:00.36` |
| Max Samples 8, Min Samples 0 | `Sample 8/8` | `00:00.37` |
| Max Samples 1024, adaptive on at 0.01, Min Samples 0 | `Sample 1024/1024` | `00:09.28` |
| Max Samples 1024, adaptive on at 0, Min Samples 0 | `Sample 1024/1024` | `00:21.00` |
| Max Samples 128, adaptive on at 0, Min Samples 0 | `Sample 128/128` | `00:02.57` |
| Max Samples 4096, adaptive on at 0, Min Samples 0 | `Sample 4096/4096` | `01:16.47` |
| EEVEE, Render Samples 64 (Cycles' 4096 still in the file) | `Rendering 64 / 64 samples` | `00:01.86` |
| EEVEE, Render Samples 64, motion blur on, 5 steps | `Rendering 65 / 65 samples` | `00:00.68` |
| EEVEE, Render Samples 16, Jitter Camera (`use_bokeh_jittered`) on | `Rendering 19 / 19 samples` | `00:00.31` |
| EEVEE, Render Samples 16, Jitter Camera off | `Rendering 16 / 16 samples` | `00:00.30` |
| Workbench | no count line; the whole output holds no line containing "sampl" | `00:00.17` |
| Max Samples 64, with `-- --cycles-print-stats` | `Sample 64/64`, then a `Render statistics:` block | `00:01.65` |

**A count supplied at launch, before `-f` and after it.** On a copy of the scene saved with Max Samples 4096 and adaptive sampling off, `--python-expr "import bpy; bpy.context.scene.cycles.samples=32"` placed before `-f 1` gave `Sample 32/32` and `Time: 00:00.87`; the same line placed after `-f 1` gave `Sample 4096/4096` and `Time: 01:31.35`. Both exited 0. So the line placed after `-f 1` did not change that frame's count, and the frame rendered at the file's own 4096.

**The viewport settings have no part in a render with no window.** The two Max Samples 200 rows differ only in their viewport settings; their pictures compared channel by channel:

```
PIXELS max_difference 0.0 channels_differing 0 of 307200
```

**A Noise Threshold of 0 is not the same as the default 0.01.** The two Max Samples 1024 rows both reached 1024, at `00:09.28` against `00:21.00`, and their pictures differ:

```
PIXELS max_difference 0.027450978755950928 channels_differing 23511 of 307200
```

Nothing printed names the value Cycles uses when the threshold is 0.

**`--cycles-print-stats` reports no sample count.** After `Sample 64/64` the run printed a block beginning:

```
Render statistics:
Mesh statistics:
  Geometry:
    Total memory: 37.24K (38,135)
        Sphere                           36.74K (37,621)
        Cube                             289B (289)
        Plane                            119B (119)
        Area                             56B (56)
        backgroundlight                  50B (50)
Image statistics:
  Total image memory: 0B (0)
Kernel statistics:
  Total render time               : Total 100.00% (11.45s), Self 1.75% (0.20s)
    Shade Surface                   : Total 66.54% (7.62s), Self 0.00% (0.00s)
…
```

and no line of the block contains "ampl".

**The sample-related settings a view layer carries for Cycles**, listed by name:

```
PASS_SAMPLE_ATTRS ['denoising_pass_follow_reflections', 'denoising_pass_use_albedo_roughness_weighting', 'denoising_store_motion_passes', 'denoising_store_passes', 'pass_debug_sample_count', 'use_denoising']
```

**A ten-second Time Limit across three frames, three times over (entry 23).** The test scene with Max Samples 4096, adaptive sampling off and Time Limit 10, rendered as frames 1 to 3 in one launch with `-s 1 -e 3 -a`. All three runs exited 0 and each wrote three pictures. By the shell's clock the runs took 32.269, 32.670 and 32.627 seconds. The first run's own lines (the count line kept is each frame's last):

```
00:00.329  render           | Rendering frame 1
00:10.618  render           | Fra: 1 | Mem: 5M | Sample 513/4096
00:10.830  render           | Saved: '/private/tmp/claude-501/smoozy-hand-tests-5.3/Settings/out/e23/r1_0001.png'
00:10.830  render           | Time: 00:10.50 (Saving: 00:00.06)
00:10.830  render           | Rendering frame 2
00:21.131  render           | Fra: 2 | Mem: 5M | Sample 513/4096
00:21.266  render           | Saved: '/private/tmp/claude-501/smoozy-hand-tests-5.3/Settings/out/e23/r1_0002.png'
00:21.266  render           | Time: 00:10.43 (Saving: 00:00.00)
00:21.266  render           | Rendering frame 3
00:31.664  render           | Fra: 3 | Mem: 5M | Sample 513/4096
00:31.795  render           | Saved: '/private/tmp/claude-501/smoozy-hand-tests-5.3/Settings/out/e23/r1_0003.png'
00:31.795  render           | Time: 00:10.52 (Saving: 00:00.00)
```

The second and third runs gave per-frame times of `00:10.66`, `00:10.69`, `00:10.51` and `00:10.61`, `00:10.65`, `00:10.53`, every frame again at `Sample 513/4096`. No line in any of the three logs contains "limit". So the ten seconds are spent on each frame, not once for the job, and nothing is printed to say the limit fired; the only sign is the count sitting below the maximum.

**Which count each view layer renders to under each Layer Samples policy (entry 24).** The test scene with adaptive sampling off, Time Limit 0, and a second view layer `Second` saved with its own Samples — 64 in one file, 1024 in another. The scene count and the policy were handed in at launch. Each run's lines, the count kept being each layer's last:

```
Second's own 64, scene count 1024:
  USE      Start rendering: Scene, ViewLayer -> Sample 1024/1024;  Start rendering: Scene, Second -> Sample 64/64
  BOUNDED  Start rendering: Scene, ViewLayer -> Sample 1024/1024;  Start rendering: Scene, Second -> Sample 64/64
  IGNORE   Start rendering: Scene, ViewLayer -> Sample 1024/1024;  Start rendering: Scene, Second -> Sample 1024/1024
Second's own 1024, scene count 256:
  USE      Start rendering: Scene, ViewLayer -> Sample 256/256;    Start rendering: Scene, Second -> Sample 1024/1024
  BOUNDED  Start rendering: Scene, ViewLayer -> Sample 256/256;    Start rendering: Scene, Second -> Sample 256/256
  IGNORE   Start rendering: Scene, ViewLayer -> Sample 256/256;    Start rendering: Scene, Second -> Sample 256/256
```

One of the six as printed, `BOUNDED` in the second half:

```
00:00.333  render           | Start rendering: Scene, ViewLayer
00:05.673  render           | Fra: 1 | Mem: 5M | Sample 256/256
00:05.673  render           | Start rendering: Scene, Second
00:10.994  render           | Fra: 1 | Mem: 5M | Sample 256/256
POLICY BOUNDED SCENE 256 LAYERS [('ViewLayer', 0), ('Second', 1024)]
```

So a view layer's own number stands in place of a scene count supplied from outside under `USE`, is capped by it under `BOUNDED`, and is ignored under `IGNORE`; a layer whose own number is 0 renders the scene's count under all three. And on Blender's default scene on Cycles at 64 by 48 with the scene count 64 and adaptive sampling off: with no layer count the render ended at `Sample 64/64` (`READBACK scene 64 layer 0`); with the layer's Samples set to 5 at launch it ended at `Sample 5/5`, and Layer Samples read back as `'USE'` without being set (`READBACK scene 64 layer 5 use_layer_samples 'USE'`).

**The sampling settings across a save and a reopen (entry 25).** On the test scene, eleven Cycles values and EEVEE's render count were set and saved to a new file, which a fresh Blender then opened and read:

```
READBACK CYCLES 137 42.0 False 0.004999999888241291 9 BOUNDED True 3 11 64 77
```

In order: the engine, Max Samples 137, Time Limit 42.0, adaptive sampling off, Noise Threshold 0.005, Min Samples 9, Layer Samples `BOUNDED`, sample subset on, subset Offset 3, subset Length 11, the view layer's own Samples 64, EEVEE's Render Samples 77. Each came back as set; the threshold prints as `0.004999999888241291` because 0.005 has no exact form in that kind of number. Layer Samples keeps its value across a save and a reopen whether or not any panel shows it.

**What else stops a render started from outside: an interrupt and a terminate signal (entry 26).** The test scene with Max Samples 4096 and Time Limit 0 (adaptive sampling as saved, on at 0.01), started with no window, left for a fixed eight seconds, then sent one signal. **An interrupt** (`kill -INT`) ended it with exit status 1 and no picture; the last lines of its output:

```
00:07.681  render           | Fra: 1 | Remaining: 00:33.53 | Mem: 7M | Sample 736/4096

Sent an internal break event. Press ^C again to kill Blender
00:07.682  render           | Fra: 1 | Remaining: 00:33.54 | Mem: 7M | Cancelled

Blender quit
```

**A terminate signal** (`kill -TERM`) ended it with exit status 143 and no picture; Blender printed nothing further, the last line of its output being an ordinary `00:07.469  render | Fra: 1 | Remaining: 00:34.37 | Mem: 7M | Sample 704/4096`. **An interrupt with `--disable-abort-handler`** behaved as the plain interrupt did: `Sent an internal break event. Press ^C again to kill Blender`, `Cancelled` at `Sample 736/4096`, `Blender quit`, exit status 1, no picture.

**EEVEE with a view layer's own count; EEVEE's settings searched; Workbench (entry 27).** The test scene on EEVEE with Render Samples 256 and a second view layer `Second` carrying its own Samples of 8:

```
00:00.325  render           | Start rendering: Scene, ViewLayer
00:01.142  render           | Fra: 1 | Rendering 256 / 256 samples
00:01.151  render           | Start rendering: Scene, Second
00:01.238  render           | Fra: 1 | Rendering 8 / 8 samples
LAYERS [('ViewLayer', 0), ('Second', 8)]
```

So a view layer's own count changes an EEVEE render. EEVEE's settings block, searched for "time" and "limit" in name, label and description, returned:

```
EEVEEPROP gi_diffuse_bounces | Diffuse Bounces | Number of times the light is reinjected inside light grids, 0 disable indirect diffuse light
EEVEEPROP taa_samples | Viewport Samples | Number of samples, unlimited if 0
EEVEEPROP time_limit | Time Limit | Limit the render time. Zero disables the limit.
EEVEEPROP motion_blur_steps | Motion steps | Controls accuracy of motion blur, more steps means longer render time
```

and what EEVEE's `time_limit` does is under question 2 (entry 21). With the engine set to Workbench, the render printed no count line of any kind, and the script printed:

```
RENDER_AA '8'
RENDER_AA ITEMS ['OFF', 'FXAA', '5', '8', '11', '16', '32']
CYCLES SAMPLES STILL 4096
```

**"Render Single Layer" from outside, with a second layer carrying its own count (entry 28).** The test scene with adaptive sampling off and a second view layer `Second` carrying Samples 64; at launch the scene count was set to 256, `use_single_layer` to `True`, and the window's view layer to `ViewLayer` (the line setting it ran without error and printed `ACTIVE LAYER SET`). Both layers rendered:

```
00:00.325  render           | Start rendering: Scene, ViewLayer
00:05.703  render           | Fra: 1 | Mem: 5M | Sample 256/256
00:05.703  render           | Start rendering: Scene, Second
00:07.126  render           | Fra: 1 | Mem: 5M | Sample 64/64
ACTIVE LAYER SET
SINGLE_LAYER True
```

So with "Render Single Layer" on, a render started from outside still renders the second layer, at that layer's own count.

What these runs show, read off the table: adaptive sampling at a loose threshold ends the whole frame early (32 of 4096); at the default threshold the count still reaches the maximum while the time falls to about a quarter; a sample subset overrides Max Samples in the count actually rendered; a time limit cuts the subset short; Min Samples above Max Samples does not raise the count; and two EEVEE features raise EEVEE's count beyond the number given.

---

## 17. When an instruction handed in at launch sets the resolution percentage, the sample count, or whether a view layer renders, what name does it use for each?

**Answer:** There is no command-line option for any of the three (not in the sources — see question 1), so such an instruction is a Python script or expression (inferred). The names are:

| Setting | Name | Where it sits |
|---|---|---|
| Resolution percentage | `resolution_percentage` | the scene's render settings: `scene.render.resolution_percentage` (the API shows `bpy.data.scenes[0].render.resolution_percentage`) |
| Sample count, Cycles | `samples` | Cycles' scene settings: `bpy.context.scene.cycles.samples` (shown in the API) |
| Sample count, EEVEE | `taa_render_samples` | the scene's EEVEE settings (`scene.eevee`) |
| Sample count, Workbench | `render_aa` | the scene's display settings (`scene.display`; that `render_aa` is Workbench's setting is inferred, see question 4) |
| Per-view-layer count | `samples` | on each view layer |
| Whether a view layer renders | `use` | on each view layer, found in the scene's `view_layers` |

The interface labels are "Percentage" (shown as "%" in the Format panel picture, question 14, documented), "Samples" / "Max Samples" (observed in `cycles_5.3/properties.py` and `cycles_5.3/ui.py`, question 16), and "Use for Rendering" (visible in the View Layer panel picture, question 1, documented; that it corresponds to `ViewLayer.use` is inferred).

**Label:** documented (names, descriptions, and manual pictures); observed (Cycles `samples` definition and `"Max Samples"` label); inferred (that `render_aa` is Workbench's setting, that `"Use for Rendering"` corresponds to `ViewLayer.use`, and the full written-out paths for EEVEE, Workbench and view layers, put together from the Scene page's `eevee`, `display` and `view_layers` entries); not in the sources (command-line options for the three settings).

**Quotes**

- "resolution_percentage / Percentage scale for render resolution (in [1, 32767], default 100)" — API, https://docs.blender.org/api/5.3/bpy.types.RenderSettings.html; ">>> bpy.data.scenes[0].render.resolution_percentage / 100" — API, https://docs.blender.org/api/5.3/info_quickstart.html; both read 2026-10-11.
- "# Access the number of samples for the Cycles render engine. / bpy.context.scene.cycles.samples" — API, https://docs.blender.org/api/5.3/info_api_reference.html, read 2026-10-11.
- "taa_render_samples / Number of samples per pixel for rendering (in [1, inf], default 64)" — API, https://docs.blender.org/api/5.3/bpy.types.SceneEEVEE.html; "render_aa / Method of anti-aliasing when rendering final image (default '8')" — API, https://docs.blender.org/api/5.3/bpy.types.SceneDisplay.html; "samples / Override number of render samples for this view layer, 0 will use the scene setting" and "use / Enable or disable rendering of this View Layer (default True)" — API, https://docs.blender.org/api/5.3/bpy.types.ViewLayer.html; "view_layers / (default None, readonly) / Type: / ViewLayers[ViewLayer]" — API, https://docs.blender.org/api/5.3/bpy.types.Scene.html; all read 2026-10-11.
- `samples: IntProperty(` `name="Samples",` (`cycles_5.3/properties.py`, lines 533–534), registered as `bpy.types.Scene.cycles = PointerProperty(` (line 1191). Read 2026-10-16 (observed).

**Not covered:** the question allows "instruction" to mean a command-line option; under that reading, the answer is that no such option exists for these three settings.

**Outside the interface:** These are the names to use from outside; the interface shows labels instead. **Observed 2026-10-17 (entries 3, 6 to 13, 60):** every name was set from a Python line at launch with no window and changed what the render wrote; a wrong name printed an error and still let the frame render with exit status 0; an unknown launch switch stopped the run with exit status 1. In the window, the panels label the three `%`, `Max Samples` and `Use for Rendering`.

**Operating system:** The sources do not say that the names differ. **Observed 2026-10-17:** on macOS 26.6.2 on Apple silicon only; Linux and Windows were not available to this sitting.

### Observed in Blender 5.3 — hand tests of 2026-10-17

Got by running Blender, not from a source; it sits beside the answer above. **On what computer:** the Creator's Mac — a MacBook Pro with an Apple M1 Pro chip and its own 14-core graphics (Metal), 16 GB of memory, macOS 26.6.2. **What ran:** Blender 5.3.0 Beta, built 2026-10-16, started from a terminal by Claude Opus 5.5 on 2026-10-17, with a throwaway settings folder and, unless said otherwise, `--factory-startup`. Nothing ran on Linux or Windows. The tests are written out in full in `Audits/Blender Research/Settings/Hand tests — 5.3.md`, by the numbers given here. Label: **observed**.

**No launch switch of Blender's own names any of the three (entry 1).** The built program's whole help text, 521 lines, was searched for `sampl`, `resolution`, `percent`, `view layer`, `view-layer` and `viewlayer`; the only lines returned are about screen pixels, Python environment switches and how view layers are evaluated on file load — the five lines are quoted under question 1. So an instruction handed in at launch for these settings has to be a Python one, which is how every test below hands it in. A launch switch that does not exist is a different case from a wrong name inside a Python line: `--render-percentage 50` is taken as a file name and stops the run with exit status 1, and `-- --cycles-samples 8` is ignored without a word (entry 3, quoted under question 1). Where the Python line sits matters as much as its words: after the file it changes the render, before the file it reads back the values set and changes nothing (entry 6, quoted under question 1).

**`resolution_percentage` changes the picture Blender writes (entry 7).** Blender's default scene on Workbench at its own 1920 by 1080, 100 per cent. With no instruction the picture measured `pixelWidth: 1920  pixelHeight: 1080`. With `import bpy; r=bpy.context.scene.render; r.resolution_percentage=25; print('READBACK', r.resolution_percentage)` handed in after the file, it printed `READBACK 25` and the picture measured `pixelWidth: 480  pixelHeight: 270`.

**`scene.cycles.samples` and `scene.eevee.taa_render_samples` change the count the render works to; `preview_samples` and `taa_samples` are other settings (entry 8).** The default scene at 64 by 48. On Cycles with every sampling setting at its default, the last two count lines were:

```
00:00.342  render           | Fra: 1 | Remaining: 00:00.16 | Mem: 1M | Sample 320/4096
00:00.391  render           | Fra: 1 | Remaining: 00:00.71 | Mem: 1M | Sample 336/4096
```

With `c.samples=3; c.preview_samples=200` handed in:

```
00:00.326  render           | Fra: 1 | Remaining: 00:00.00 | Mem: 1M | Sample 1/3
00:00.376  render           | Fra: 1 | Mem: 1M | Sample 3/3
READBACK 3 200
```

On EEVEE with nothing set the last count line was `Rendering 64 / 64 samples`; with `e.taa_render_samples=3; e.taa_samples=200` handed in:

```
00:00.476  render           | Fra: 1 | Rendering 1 / 3 samples
00:00.478  render           | Fra: 1 | Rendering 3 / 3 samples
READBACK 3 200
```

**`scene.display.render_aa` reaches Workbench's render, and takes text, not a number (entry 9).** The default scene on Workbench at 480 by 270. Set to `'OFF'` and to `'32'` after the file, it read back `READBACK 'OFF'` and `READBACK '32'`. Assigning the number 8 and then the text `'8'`:

```
NUMBER REFUSED TypeError bpy_struct: item.attr = val: expected a string enum, not int
TEXT ACCEPTED '8'
```

The pictures, both 480 by 270, compared channel by channel:

```
OFF against 32:          PIXELS max_difference 0.24313727021217346 channels_differing 150540 of 518400
OFF against OFF again:   PIXELS max_difference 0.0 channels_differing 0 of 518400
```

The second line is the control: two renders at the same setting are identical, so the first line's difference comes from the setting.

**A view layer's `use` removes it from what is written, and `use_single_layer` does nothing from outside (entry 10).** Written up under question 1: with `Second`'s `use` set to `False` its channels are gone from the multi-layer file; with `use_single_layer` set to `True` both layers still render.

**All six names in one Python line, at one launch (entry 11).** The default scene on Cycles at 64 by 48 with a second view layer `Second`:

```
import bpy; s=bpy.context.scene; s.render.resolution_percentage=25; s.cycles.samples=8; s.eevee.taa_render_samples=9; s.display.render_aa='FXAA'; s.view_layers['ViewLayer'].samples=5; s.view_layers['Second'].use=False; print('SIX', …)
```

printed, exit status 0:

```
00:00.329  render           | Start rendering: Scene, ViewLayer
00:00.335  render           | Fra: 1 | Mem: 1M | Sample 0/5
00:00.335  render           | Fra: 1 | Remaining: 00:00.00 | Mem: 1M | Sample 1/5
00:00.380  render           | Fra: 1 | Mem: 1M | Sample 5/5
SIX 25 8 9 'FXAA' 5 False
```

and wrote a picture of 16 by 12. Every name read back as set and none was refused; all three engines' settings were reached in a run whose engine was Cycles. The render counted to the view layer's own 5, not the scene's 8, and `Second` did not render.

**A wrong name prints an error and costs nothing a program could see (entry 12).** On the default scene on Cycles at 64 by 48, three launches each carried one misspelled name in a Python line placed before `-f 1`, with nothing to catch the error. Each printed a Python error, then rendered and saved the frame, and Blender's own exit status was 0 every time:

```
resolution_percent=25  -> AttributeError: 'RenderSettings' object has no attribute 'resolution_percent'. Did you mean: 'resolution_percentage'?   Saved …/w_0001.png   exit 0
cycles.sample=8        -> AttributeError: 'CyclesRenderSettings' object has no attribute 'sample'. Did you mean: 'samples'?               Saved …/w_0001.png   exit 0
view_layers[0].used    -> AttributeError: 'ViewLayer' object has no attribute 'used'. Did you mean: 'use'?                               Saved …/w_0001.png   exit 0
```

One of them in full, sample lines left out:

```
00:00.320  blend            | Read blend: "/private/tmp/claude-501/smoozy-hand-tests-5.3/Settings/files/cyc64.blend"
Traceback (most recent call last):
  File "<string>", line 1, in <module>
AttributeError: 'CyclesRenderSettings' object has no attribute 'sample'. Did you mean: 'samples'?
00:00.325  render           | Rendering single frame
00:00.325  render           | Rendering frame 1
00:00.325  render           | Start rendering: Scene, ViewLayer
00:00.325  render           | Engine: Cycles
…  (Cycles' own preparation lines)
00:00.401  render           | Fra: 1 | Remaining: 00:00.79 | Mem: 1M | Finished
00:00.462  render           | Saved: '/private/tmp/claude-501/smoozy-hand-tests-5.3/Settings/out/e12/w_0001.png'
00:00.462  render           | Time: 00:00.13 (Saving: 00:00.06)
Blender quit
```

A sweep over six settings groups, each misspelling caught and reported by the script, refused every one:

```
REFUSED  s.cycles.sample = 8 | AttributeError 'CyclesRenderSettings' object has no attribute 'sample'
REFUSED  s.cycles.samples_typo = 8 | AttributeError 'CyclesRenderSettings' object has no attribute 'samples_typo'
REFUSED  s.render.resolution_percent = 25 | AttributeError 'RenderSettings' object has no attribute 'resolution_percent'
REFUSED  s.view_layers[0].used = False | AttributeError 'ViewLayer' object has no attribute 'used'
REFUSED  s.display.render_a = 'OFF' | AttributeError 'SceneDisplay' object has no attribute 'render_a'
REFUSED  s.eevee.taa_render_sample = 3 | AttributeError 'SceneEEVEE' object has no attribute 'taa_render_sample'
```

So a misspelling is refused loudly on the terminal, Cycles' settings included, and the frame is still rendered and Blender still exits 0.

**What the running program calls each setting (entry 13).** Read out of a factory-startup Blender with no window — each setting's name, the label the program holds for it, its description, its kind and its declared default:

```
PROP Scene.render | label='Render Data' | description='' | type=POINTER default=?
PROP Scene.view_layers | label='View Layers' | description='' | type=COLLECTION default=?
PROP Scene.eevee | label='EEVEE' | description='EEVEE settings for the scene' | type=POINTER default=?
PROP Scene.display | label='Scene Display' | description='Scene display settings for 3D viewport' | type=POINTER default=?
PROP RenderSettings.resolution_percentage | label='Resolution Scale' | description='Percentage scale for render resolution' | type=INT default=100
PROP RenderSettings.use_single_layer | label='Render Single Layer' | description='Only render the active layer. Only affects rendering from the interface, ignored for rendering from command line.' | type=BOOLEAN default=False
PROP SceneEEVEE.taa_render_samples | label='Render Samples' | description='Number of samples per pixel for rendering' | type=INT default=64
PROP SceneEEVEE.taa_samples | label='Viewport Samples' | description='Number of samples, unlimited if 0' | type=INT default=16
PROP SceneDisplay.render_aa | label='Render Anti-Aliasing' | description='Method of anti-aliasing when rendering final image' | type=ENUM items=['OFF', 'FXAA', '5', '8', '11', '16', '32'] default='8'
PROP ViewLayer.use | label='Enabled' | description='Enable or disable rendering of this View Layer' | type=BOOLEAN default=True
PROP ViewLayer.samples | label='Samples' | description='Override number of render samples for this view layer, 0 will use the scene setting' | type=INT default=0
PROP Scene.cycles | label='Cycles Render Settings' | description='Cycles render settings' | type=POINTER default=?
PROP CyclesRenderSettings.samples | label='Samples' | description='Number of samples to render for each pixel' | type=INT default=4096
PROP CyclesRenderSettings.preview_samples | label='Viewport Samples' | description='Number of samples to render in the viewport, unlimited if 0' | type=INT default=1024
PROP CyclesRenderSettings.use_layer_samples | label='Layer Samples' | description='How to use per view layer sample settings' | type=ENUM items=['USE', 'BOUNDED', 'IGNORE'] default='USE'
PROP SceneGpencil.aa_samples | label='Anti-Aliasing Samples' | description='Number of supersampling anti-aliasing samples per pixel for final render' | type=INT default=8
LIVE scene.render.resolution_percentage = 100
LIVE scene.render.use_single_layer = False
LIVE scene.eevee.taa_render_samples = 64
LIVE scene.display.render_aa = '8'
LIVE existing view layers = [('ViewLayer', True, 0)]
LIVE new view layer = ('MadeByTest', True, 0)
LIVE scene.cycles.samples = 4096 preview_samples = 1024 use_layer_samples = 'USE'
```

So the name an instruction uses and the label the program holds differ for every one of the three settings asked about: `resolution_percentage` is labelled **Resolution Scale**, `taa_render_samples` **Render Samples**, `render_aa` **Render Anti-Aliasing**, a view layer's `use` **Enabled**; only Cycles' `samples` is labelled with its own name, **Samples**. The one description that marks a setting as interface-only is `use_single_layer`'s. Every near neighbour — `use_single_layer`, `preview_samples`, `taa_samples`, `use_layer_samples`, `aa_samples` — exists under that name. What the panels themselves call these is under the window tests below (entry 60).

**What the panels call them, photographed in Blender's own window (entry 60).** Blender 5.3 was opened with its window from the terminal, with a throwaway settings folder whose only changes are the splash screen and tooltips switched off; a script chose Cycles, enlarged the Properties editor to fill the window, turned it to three tabs in turn, and saved Blender's own screenshot of each (3024 by 1724 pixels). What the pictures show:

- **Output Properties ▸ Format:** rows `Resolution X` reading `1920 px`, `Y` reading `1080 px`, and beneath them a row whose whole label is **`%`**, reading `100%`.
- **Render Properties ▸ Sampling**, Cycles: a `Viewport` group — `Noise Threshold` ticked, `0.1000`; **`Max Samples`** `1024`; `Min Samples` `0` — and a `Render` group — `Noise Threshold` ticked, `0.0100`; **`Max Samples`** `4096`; `Min Samples` `0`; `Time Limit` `0 s`. The one label **Max Samples** is used for both the setting an instruction calls `preview_samples` and the one it calls `samples`; only the group tells them apart.
- **View Layer Properties ▸ View Layer:** two checkboxes, **`Use for Rendering`**, ticked, and **`Render Single Layer`**, unticked. A small round dot sits at the far right of the `Use for Rendering` row; the `Render Single Layer` row has none.

(A note on the method, which applies to every window photograph in this document. On this Mac a photograph taken with Blender's own screenshot command sometimes showed the window as it was one step earlier — the previous tab, or the layout before the editor was enlarged. So every photograph used here was checked to carry a mark of the state just set — the tab's own panels, the value just typed in by the script, the frame number in the timeline — and those that did not were not used.)

So for the resolution percentage the instruction says `resolution_percentage`, the program's label says `Resolution Scale` and the panel says `%`; for the Cycles count the instruction says `samples` and the panel says `Max Samples`; for the view layer the instruction says `use`, the program's label says `Enabled` and the panel says `Use for Rendering`.

**A count set by a launch instruction is what the panel then shows (entry 60).** With `s.cycles.samples=7` set by the script in the same window, the Render group's `Max Samples` read **`7`** in the next photograph, the Viewport group's still `1024`.

---

## 18. What values will a render's sample count be set to, and what does a render do when given a value outside those?

**Answer — accepted values:**
- **Cycles, final render:** whole numbers from 1 to 16,777,216 (`min=1, max=(1 << 24)`), default 4096 (observed; 16,777,216 inferred from `1 << 24`).
- **Cycles, viewport:** 0 to 16,777,216, where 0 means unlimited (observed; 16,777,216 inferred). The Cycles sample-subset settings: offset 0 to 16,777,216, length 1 to 16,777,216 (observed; 16,777,216 inferred).
- **EEVEE, final render:** whole numbers from 1 upward; the API prints the top as "inf" (no upper number given); default 64 (documented).
- **Per-view-layer override:** 0 upward, where 0 means "use the scene setting" (documented).
- **Workbench:** one of `5, 8, 11, 16, 32` under Multisample (documented on the Workbench sampling page), or one of `OFF`, `FXAA`, `5`, `8`, `11`, `16`, `32` on `Scene.display.render_aa`, default `8` (documented on `SceneDisplay`; inferred to be Workbench's setting, see question 4).

**Answer — outside the range:** a number below the minimum is set to the minimum, and one above the maximum is set to the maximum, "silently" — nothing is reported. This is **inferred** for Cycles by putting together the documented rule for these setting definitions and Cycles' observed definition. For EEVEE and the view-layer override it is inferred from the integer-setting rule, as in question 15. For a value that is not one of Workbench's choices, the sources say nothing (not in the sources).

**Label:** observed (Cycles ranges as `1 << 24`); documented (EEVEE, view layer, `SceneDisplay.render_aa` and Workbench lists; the clamping rule); inferred (16,777,216 from `1 << 24`, that `render_aa` is Workbench's setting, and the clamping rule applied to these settings); not in the sources (invalid Workbench choice, real upper limit for EEVEE and view-layer override, and non-number values).

**Quotes**

- `samples: IntProperty(` … `min=1, max=(1 << 24),` `default=4096,` (lines 533–538); `preview_samples: IntProperty(` `name="Viewport Samples",` `description="Number of samples to render in the viewport, unlimited if 0",` `min=0,` `soft_min=1,` `max=(1 << 24),` `default=1024,` (lines 539–546); `sample_offset` `min=0, max=(1 << 24),` (lines 554–559); `sample_subset_length` `min=1, max=(1 << 24),` (lines 561–566) — `cycles_5.3/properties.py`, read 2026-10-16 (observed).
- "min (int) – Hard minimum, trying to assign a value below will silently assign this minimum instead." / "max (int) – Hard maximum, trying to assign a value above will silently assign this maximum instead." / "soft_min (int) – Soft minimum (>= min), user won’t be able to drag the widget below this value in the UI." — API, https://docs.blender.org/api/5.3/bpy.props.html (under `bpy.props.IntProperty`), read 2026-10-11.
- "taa_render_samples / Number of samples per pixel for rendering (in [1, inf], default 64)" — API, https://docs.blender.org/api/5.3/bpy.types.SceneEEVEE.html, read 2026-10-11.
- "samples / Override number of render samples for this view layer, 0 will use the scene setting (in [0, inf], default 0)" — API, https://docs.blender.org/api/5.3/bpy.types.ViewLayer.html, read 2026-10-11.
- "Literal[‘OFF’, ‘FXAA’, ‘5’, ‘8’, ‘11’, ‘16’, ‘32’]" — API, https://docs.blender.org/api/5.3/bpy.types.SceneDisplay.html; "Multisample … The number of samples are predefined so it uses the best distribution of the samples. / 5, 8, 11, 16, 32" — manual, https://docs.blender.org/manual/en/5.3/render/workbench/sampling.html; both read 2026-10-11.

**Not covered:** the real upper limit for EEVEE and the view-layer override (the API writes "inf"); a non-whole or non-number value; whether a clamped value is logged; whether an animated value outside the range behaves the same.

**Outside the interface:** From outside, a count can only be set by a script (question 1). The clamping rule is written for assigning values, which is what a script does (inferred). Inside the interface there is also a "soft" lower limit on dragging (Cycles' viewport count has `soft_min=1`). **Observed 2026-10-17 (entries 52 to 54):** from a terminal with no window, an out-of-range count was corrected silently for Cycles, EEVEE, the view layer and Grease Pencil; a value not a whole number was refused; and a Cycles render then worked to the corrected count. Dragging a field in the window was not tried.

**Operating system:** The sources do not say. **Observed 2026-10-17:** on macOS 26.6.2 on Apple silicon only; Linux and Windows were not available to this sitting.

### Observed in Blender 5.3 — hand tests of 2026-10-17

Got by running Blender, not from a source; it sits beside the answer above. **On what computer:** the Creator's Mac — a MacBook Pro with an Apple M1 Pro chip and its own 14-core graphics (Metal), 16 GB of memory, macOS 26.6.2. **What ran:** Blender 5.3.0 Beta, built 2026-10-16, started from a terminal with no window by Claude Opus 5.5 on 2026-10-17, with a throwaway settings folder and `--factory-startup`. Every Cycles render was on the processor. Nothing ran on Linux or Windows. The tests are written out in full in `Audits/Blender Research/Settings/Hand tests — 5.3.md`, by the numbers given here. Label: **observed**.

**Every sample count's floor, ceiling, dragging bounds and starting value, read from the running program (entry 52):**

```
E1 scene.cycles.samples hard_min=1 hard_max=16777216 soft_min=1 soft_max=16777216 default=4096 current=4096
E1 scene.cycles.preview_samples hard_min=0 hard_max=16777216 soft_min=1 soft_max=16777216 default=1024 current=1024
E1 scene.cycles.sample_offset hard_min=0 hard_max=16777216 soft_min=0 soft_max=16777216 default=0 current=0
E1 scene.cycles.sample_subset_length hard_min=1 hard_max=16777216 soft_min=1 soft_max=16777216 default=2048 current=2048
E1 scene.cycles.adaptive_min_samples hard_min=0 hard_max=4096 soft_min=0 soft_max=4096 default=0 current=0
E1 scene.cycles.preview_adaptive_min_samples hard_min=0 hard_max=4096 soft_min=0 soft_max=4096 default=0 current=0
E1 scene.cycles.guiding_training_samples hard_min=0 hard_max=2147483647 soft_min=1 soft_max=2147483647 default=128 current=128
E1 scene.cycles.preview_denoising_start_sample hard_min=0 hard_max=16777216 soft_min=0 soft_max=16777216 default=1 current=1
E1 scene.eevee.taa_render_samples hard_min=1 hard_max=2147483647 soft_min=1 soft_max=2147483647 default=64 current=64
E1 scene.eevee.taa_samples hard_min=0 hard_max=2147483647 soft_min=0 soft_max=2147483647 default=16 current=16
E1 view_layer.samples hard_min=0 hard_max=2147483647 soft_min=0 soft_max=10000 default=0 current=0
E1 scene.grease_pencil_settings.aa_samples hard_min=1 hard_max=2147483647 soft_min=1 soft_max=256 default=8 current=8
E1 scene.display.render_aa items=['OFF', 'FXAA', '5', '8', '11', '16', '32'] default='8' current='8'
```

So the upper end the API writes as "inf" for EEVEE's count and the view layer's override is 2 147 483 647; Cycles' 16 777 216 is what the running program holds; and the view layer's override cannot be dragged past 10 000.

**Counts assigned out of range and of the wrong kind (entry 53).** One run printed two lines of its own — one to standard output, one to standard error — and attempted a deliberately wrong assignment, so that "nothing was reported" can be told from "nothing was captured"; then assigned values to each count and read them back:

```
E2 CONTROL-STDERR this line proves standard error is captured
E2 CONTROL-STDOUT this line proves standard output is captured
E2 CONTROL-RAISES AttributeError: 'CyclesRenderSettings' object has no attribute 'samples_no_such_setting'
E2 scene.cycles.samples = 0 -> stored 1 (no error, was 4096)
E2 scene.cycles.samples = -5 -> stored 1 (no error, was 4096)
E2 scene.cycles.samples = 16777217 -> stored 16777216 (no error, was 4096)
E2 scene.cycles.samples = 99999999 -> stored 16777216 (no error, was 4096)
E2 scene.cycles.samples = 2147483647 -> stored 16777216 (no error, was 4096)
E2 scene.cycles.samples = 2147483648 -> REFUSED ValueError: bpy_struct: item.attr = val: CyclesRenderSettings.samples value not in 'int' range ((-2147483647 -1), 2147483647) (still 4096)
E2 scene.cycles.samples = 1000000000000 -> REFUSED ValueError: bpy_struct: item.attr = val: CyclesRenderSettings.samples value not in 'int' range ((-2147483647 -1), 2147483647) (still 4096)
E2 scene.cycles.preview_samples = -5 -> stored 0 (no error, was 1024)
E2 scene.cycles.adaptive_min_samples = 99999999 -> stored 4096 (no error, was 0)
E2 scene.cycles.sample_subset_length = 0 -> stored 1 (no error, was 2048)
E2 scene.eevee.taa_render_samples = 0 -> stored 1 (no error, was 64)
E2 scene.eevee.taa_render_samples = 99999999 -> stored 99999999 (no error, was 64)
E2 scene.eevee.taa_render_samples = 2147483648 -> REFUSED ValueError: bpy_struct: item.attr = val: SceneEEVEE.taa_render_samples value not in 'int' range ((-2147483647 -1), 2147483647) (still 64)
E2 view_layer.samples = -5 -> stored 0 (no error, was 0)
E2 view_layer.samples = 99999999 -> stored 99999999 (no error, was 0)
E2 scene.grease_pencil_settings.aa_samples = 0 -> stored 1 (no error, was 8)
E3 scene.cycles.samples = 4.7 -> REFUSED TypeError: bpy_struct: item.attr = val: CyclesRenderSettings.samples expected an int type, not float (still 4096)
E3 scene.cycles.samples = 'many' -> REFUSED TypeError: bpy_struct: item.attr = val: CyclesRenderSettings.samples expected an int type, not str (still 4096)
E3 scene.cycles.samples = '64' -> REFUSED TypeError: bpy_struct: item.attr = val: CyclesRenderSettings.samples expected an int type, not str (still 4096)
E3 scene.cycles.samples = None -> REFUSED TypeError: bpy_struct: item.attr = val: CyclesRenderSettings.samples expected an int type, not NoneType (still 4096)
E3 scene.cycles.samples = True -> stored 1 (no error, was 4096)
E3 scene.eevee.taa_render_samples = 4.7 -> REFUSED TypeError: bpy_struct: item.attr = val: SceneEEVEE.taa_render_samples expected an int type, not float (still 64)
E3 scene.eevee.taa_render_samples = True -> stored 1 (no error, was 64)
E3 scene.display.render_aa = '7' -> REFUSED TypeError: bpy_struct: item.attr = val: enum "7" not found in ('OFF', 'FXAA', '5', '8', '11', '16', '32') (still '8')
E3 scene.display.render_aa = 'OFF' -> stored 'OFF' (no error, was '8')
E3 scene.display.render_aa = '32' -> stored '32' (no error, was 'OFF')
E3 scene.display.render_aa = 'fxaa' -> REFUSED TypeError: bpy_struct: item.attr = val: enum "fxaa" not found in ('OFF', 'FXAA', '5', '8', '11', '16', '32') (still '32')
E3 scene.display.render_aa = 8 -> REFUSED TypeError: bpy_struct: item.attr = val: expected a string enum, not int (still '32')
```

(The run also gave the same `expected an int type` refusals for EEVEE's count given text or nothing, and for the view layer's override and the Grease Pencil count given a fraction, text or nothing.) So a whole number outside a count's range is corrected silently to the nearest end, for EEVEE's count, the view layer's override and the Grease Pencil count as well as for Cycles; a number too large for a whole number is refused with a `ValueError`; a value that is not a whole number is refused, not rounded — except `True`, which is stored as 1; and a Workbench choice outside its list is refused, the refusal naming every real choice, case mattering.

**The count a render works to after an out-of-range value (entry 54).** The noisy test scene (a plane, a metal sphere, a glass cube, an area light) on Cycles at 64 by 48 with adaptive sampling off, the count set at launch, one frame, each run under a 25-second ceiling. Asked for 99 999 999, with the line printing the stored value flushed at once:

```
E6 asked=99999999 stored=16777216
00:00.324  render           | Fra: 1 | Mem: 1M | Sample 0/16777216
00:00.325  render           | Fra: 1 | Remaining: 04:41:12.99 | Mem: 1M | Sample 1/16777216
```

and nothing further of Blender's before the ceiling stopped it (exit status 142, the ceiling's): no error, no warning, no picture. The control, asked for 0:

```
00:00.335  render           | Fra: 1 | Mem: 1M | Sample 0/1
00:00.384  render           | Fra: 1 | Mem: 1M | Sample 1/1
00:00.446  render           | Saved: '/private/tmp/claude-501/smoozy-hand-tests-5.3/Settings/out/e54/c0_0001.png'
E6 asked=0 stored=1
```

exit status 0. On EEVEE, asked for 99 999 999, it printed `E6 asked=99999999 stored=99999999`, worked to that number — `Rendering 1 / 99999999 samples` at the start, `Rendering 14600 / 99999999 samples` at 24.563 seconds — and was stopped by the ceiling. So the corrected number is the number the Cycles render works to, and nothing says the number was corrected except the progress line itself.

---

## 19. What settings does a scene or a view layer carry that govern denoising, and what does each do?

**Answer:** Listed by where they sit. Defaults and option lists are as quoted below unless noted as cited by line range without a verbatim quote.

**Scene — Cycles, final render** (observed in `cycles_5.3/properties.py`, with manual descriptions documented):
- `use_denoising` — "Denoise the rendered image"; default on (`default=True`, observed). Manual: "Denoise (Render) / Whether to perform denoising for the final rendered image." (documented).
- `denoiser` — which denoiser: OptiX (only where configured NVIDIA GPUs exist) or OpenImageDenoise (observed and documented). Default value `4` (observed), which is OpenImageDenoise's item number in the same file (that match is inferred).
- `denoising_input_passes` — controls which render passes the denoiser uses as input (documented), default `'RGB_ALBEDO_NORMAL'` (observed; the item labels None, Albedo, and Albedo and Normal were not quoted verbatim).
- `denoising_prefilter` — visible only when using OpenImageDenoise (documented), default `'ACCURATE'` (observed; the item labels None, Fast, and Accurate were not quoted verbatim).
- `denoising_quality` — default `'HIGH'` (observed; its item labels High, Balanced, and Fast and whether it applies only to OpenImageDenoise were not quoted verbatim).
- `denoising_use_gpu` — perform denoising on the GPU (documented); default off (`default=False`, observed).

**Scene — Cycles, viewport only** (`cycles_5.3/properties.py` lines 486–531; only `preview_denoising_start_sample` and the `'DLSS'` item at lines 287–288 are quoted verbatim below, while the other seven property definitions, defaults and option lists were cited by line range without verbatim quotes):
- `use_preview_denoising` (default off, unquoted);
- `preview_denoiser` (in 5.3 includes `'DLSS'`, `"Use NVIDIA DLSS Ray Reconstruction"`, observed, and documented in the 5.3 Cycles release notes; the other items and default were not quoted verbatim);
- `preview_denoising_input_passes` (default Albedo, unquoted);
- `preview_denoising_prefilter` (default Fast, unquoted);
- `preview_denoising_quality` (default Balanced, unquoted);
- `preview_denoising_start_sample` — "Sample to start denoising the preview at", default `1` (observed), manual: "Sample at which to start denoising in the 3D Viewport" (documented);
- `preview_denoising_use_gpu` (default on, unquoted);
- `preview_denoising_upscale_quality` (default Balanced, unquoted).

**Scene — compositor Denoise nodes** (documented, any engine):
- `compositor_denoise_device` — Auto, CPU or GPU for Denoise nodes;
- `compositor_denoise_final_quality` — quality during final renders for nodes set to "Follow Scene", default High;
- `compositor_denoise_preview_quality` — the same for viewport and interactive compositing, default Balanced.

**Scene — EEVEE** (documented):
- `use_taa_reprojection` — "Denoise image using temporal reprojection (can leave some ghosting)" (default `True`);
- the ray-tracing options group:
  - `use_denoise` — "Enable noise reduction techniques for raytraced effects";
  - `denoise_spatial` — reuse neighbouring pixels' rays;
  - `denoise_temporal` — accumulate samples by reprojecting earlier results;
  - `denoise_bilateral` — blur with a bilateral filter.
  - All default on (`default True`).

**View layer — Cycles** (`cycles_5.3/properties.py` lines 1625–1654; the descriptions below are quoted, while the property definitions and `default=` values were cited by line range without verbatim quotes):
- `use_denoising` — per-layer on/off (default on, unquoted). In `cycles_5.3/ui.py`, `sub.active = scene.cycles.use_denoising` is set on its layout (observed); that the interface greys it out when the scene's denoising is off is **inferred** from that line.
- `denoising_store_passes` — "Store the denoising feature passes and the noisy image. The passes adapt to the denoiser selected for rendering" (default off, unquoted).
- `denoising_store_motion_passes` — "Store the denoising motion passes, for temporal denoisers" (default off, unquoted).
- `denoising_pass_follow_reflections` — "Follow reflections for the denoising feature passes" (default on, unquoted).
- `denoising_pass_use_albedo_roughness_weighting` — "Use roughness-based weighting of the albedo for the denoising feature passes" (default on, unquoted).
- In `cycles_5.3/engine.py`, the pass-registration code has:
  - `if scene.cycles.use_denoising and crl.use_denoising: yield (n_("Noisy Image"), "RGBA", 'COLOR')` (code observed; that a "Noisy Image" pass exists in a render only when both are on is **inferred**);
  - `if crl.denoising_store_passes:` (line 254, condition observed; the `yield` lines under it were not quoted);
  - `if crl.denoising_store_motion_passes: if not (scene.render.use_motion_blur and srl.use_motion_blur):` (lines 260–261, condition observed; the `yield` lines under it were not quoted).

**View layer — EEVEE** (documented; both added between 5.2 and 5.3 per the API change log):
- `denoising_store_passes` — "Deliver denoising passes" (default `False`);
- `denoising_pass_use_albedo_roughness_weighting` — "Use roughness-based weighting of the albedo for the denoising feature passes" (default `True`).

The manual describes the "Denoising Data" pass as available in Cycles and EEVEE, for use with the compositor's Denoise node (documented).

**Label:** observed (quoted Cycles property definitions, `engine.py` conditions, and `ui.py` line); documented (compositor, EEVEE, manual descriptions, and release note); inferred (the default denoiser's name, the runtime effect of `engine.py` and `ui.py` lines, and outside-interface behaviour); not in the sources (Workbench denoising settings, and what a render does when asked for an unavailable denoiser).

**Quotes**

- `use_denoising: BoolProperty(` `name="Use Denoising",` `description="Denoise the rendered image",` `default=True,` (lines 448–453); `denoiser: EnumProperty(` … `default=4,  # Use integer to avoid error in builds without OpenImageDenoise.` (lines 454–461); `return [('OPENIMAGEDENOISE', "OpenImageDenoise",` `n_("Use Intel OpenImageDenoise AI denoiser"), 4)]` (lines 278–279); `"Use the OptiX AI denoiser with GPU acceleration, only available on NVIDIA GPUs when configured in the system tab in the user preferences"), 2)]` (lines 294–295); `denoising_prefilter` `default='ACCURATE',` (462–467); `denoising_quality` `default='HIGH',` (468–473); `denoising_input_passes` `default='RGB_ALBEDO_NORMAL',` (474–479); `denoising_use_gpu` `default=False,` (480–484); viewport settings lines 486–531, including `return [('DLSS', "DLSS",` `n_("Use NVIDIA DLSS Ray Reconstruction"), 8)]` (lines 287–288) and `preview_denoising_start_sample: IntProperty(` `name="Start Denoising",` `description="Sample to start denoising the preview at",` `min=0, max=(1 << 24),` `default=1,` (515–520); view-layer settings lines 1625–1654 — `cycles_5.3/properties.py`. `if scene.cycles.use_denoising and crl.use_denoising:` `yield (n_("Noisy Image"), "RGBA", 'COLOR')` (lines 250–251); `if crl.denoising_store_passes:` (254); `if crl.denoising_store_motion_passes:` `if not (scene.render.use_motion_blur and srl.use_motion_blur):` (260–261) — `cycles_5.3/engine.py`. `sub.prop(view_layer.cycles, "use_denoising", text="Denoising")` `sub.active = scene.cycles.use_denoising` — `cycles_5.3/ui.py`, lines 1054–1055. All read 2026-10-16 (observed).
- "Denoiser / The algorithm to use." … "OptiX: … Only available on NVIDIA GPUs when configured in the Cycles Render Devices user preferences." / "Passes / Controls which Render Passes the denoiser should use as input." / "Prefilter OpenImageDenoise … Visible only when using OpenImageDenoise." / "Start Sample / Sample at which to start denoising in the 3D Viewport." / "Use GPU / Perform denoising on the GPU." — manual, https://docs.blender.org/manual/en/5.3/render/cycles/render_settings/sampling.html, read 2026-10-11.
- "On NVIDIA GPUs, a new DLSS denoiser is available for the viewport." — release notes, https://developer.blender.org/docs/release_notes/5.3/cycles/, read 2026-10-11.
- "compositor_denoise_device / The device to use to process the denoise nodes in the compositor (default 'AUTO')" / "compositor_denoise_final_quality / The quality used by denoise nodes during the compositing of final renders if the nodes’ quality option is set to Follow Scene (default 'HIGH')" / "compositor_denoise_preview_quality / The quality used by denoise nodes during viewport and interactive compositing if the nodes’ quality option is set to Follow Scene (default 'BALANCED')" — API, https://docs.blender.org/api/5.3/bpy.types.RenderSettings.html, read 2026-10-11.
- "use_taa_reprojection / Denoise image using temporal reprojection (can leave some ghosting) (default True)" / "ray_tracing_options / EEVEE settings for tracing reflections (readonly)" — API, https://docs.blender.org/api/5.3/bpy.types.SceneEEVEE.html; "denoise_bilateral / Blur the resolved radiance using a bilateral filter (default True)" / "denoise_spatial / Reuse neighbor pixels’ rays (default True)" / "denoise_temporal / Accumulate samples by reprojecting last tracing results (default True)" / "use_denoise / Enable noise reduction techniques for raytraced effects (default True)" — API, https://docs.blender.org/api/5.3/bpy.types.RaytraceEEVEE.html; all read 2026-10-11.
- "Denoising can be enabled to reduce the amount of noise from the raw ray-traced output. This can help image stability but will also over-blur the final ray-traced output." — manual, https://docs.blender.org/manual/en/5.3/render/eevee/render_settings/raytracing.html, read 2026-10-11.
- "denoising_pass_use_albedo_roughness_weighting / Use roughness-based weighting of the albedo for the denoising feature passes (default True)" / "denoising_store_passes / Deliver denoising passes (default False)" — API, https://docs.blender.org/api/5.3/bpy.types.ViewLayerEEVEE.html; "bpy.types.ViewLayerEEVEE.denoising_pass_use_albedo_roughness_weighting" / "bpy.types.ViewLayerEEVEE.denoising_store_passes" listed under "5.2 to 5.3" — API, https://docs.blender.org/api/5.3/change_log.html; both read 2026-10-11.
- "Denoising Data Cycles, EEVEE / Includes auxiliary passes used for denoising, along with the combined image before denoising. These passes can be used with the Denoise Node as an alternative to automatic denoising." — manual, https://docs.blender.org/manual/en/5.3/render/layers/passes.html, read 2026-10-11.

**Where I looked:** only the pages and files in Quotes above were cited; no search terms or additional pages were recorded for Workbench denoising settings.

**Not covered:** what EEVEE's two view-layer denoising settings change in an EEVEE render beyond the descriptions; what `denoising_pass_follow_reflections` does in detail; Workbench (no denoising setting found in the cited sources).

**Outside the interface:** The settings are stored in the scene and view layer and apply however the render is started (inferred). The viewport settings affect only the 3D viewport, which a background render does not have (inferred from "viewport" in their names and descriptions). From outside, what a render does when the file asks for an unavailable denoiser (such as OptiX without configured NVIDIA GPUs) is not stated (not in the sources). **Observed 2026-10-17 (entries 55 to 59, 66):** from a terminal with no window, the settings exist as listed below; Cycles denoised only with both the scene's and the view layer's switch on; the scene's compositor denoise quality changed the render under Cycles and EEVEE; and a file whose stored denoiser this Mac does not offer rendered, still denoised, with nothing printed unless a script read the setting. In the window, the view layer's Denoising checkbox is drawn dimmed, still ticked, when the scene's Denoise is off.

**Operating system:** Partly. OptiX denoising needs NVIDIA GPUs (documented), and the manual's GPU page lists OptiX and CUDA as Windows and Linux only. GPU-accelerated denoising availability varies by GPU and system: "GPU accelerated denoising is available on discrete Radeon RX 6000 and newer GPUs on Linux, and discrete Radeon RX 7000 and newer GPUs on Windows." and "GPU accelerated ray-tracing and denoising is available on Apple Silicon." (manual, https://docs.blender.org/manual/en/5.3/render/cycles/gpu_rendering.html, read 2026-10-11). **Observed 2026-10-17:** on macOS 26.6.2 on Apple silicon, the final render offers OpenImageDenoise alone and the viewport `AUTO` and OpenImageDenoise (entry 56). **Not tried, and why:** what an NVIDIA, AMD or Intel card offers — OptiX, the DLSS viewport denoiser, whether Prefilter and Quality do anything under OptiX — and anything on Linux or Windows; no such machine was available to this sitting.

### Observed in Blender 5.3 — hand tests of 2026-10-17

Got by running Blender, not from a source; it sits beside the answer above. **On what computer:** the Creator's Mac — a MacBook Pro with an Apple M1 Pro chip and its own 14-core graphics (Metal), 16 GB of memory, macOS 26.6.2, with no NVIDIA, AMD or Intel graphics card. **What ran:** Blender 5.3.0 Beta, built 2026-10-16, started from a terminal by Claude Opus 5.5 on 2026-10-17, with a throwaway settings folder and, unless said otherwise, `--factory-startup` and no window. Every Cycles render was on the processor. Nothing ran on Linux or Windows. The tests are written out in full in `Audits/Blender Research/Settings/Hand tests — 5.3.md`, by the numbers given here. Label: **observed**.

**Every denoising setting a scene or a view layer carries (entry 55).** Every setting whose name contains "denois" on the scene's Cycles settings, its render settings, its EEVEE settings and EEVEE's ray-tracing options, and on the view layer's Cycles and EEVEE settings, with its kind, the label the program holds, its declared default, its current value on a factory scene and its fixed choices (`choices=[]` marks a list Blender builds while running, which this listing cannot show; entry 56 asks for those):

```
E8 scene.cycles denoiser type=ENUM label='Denoiser' default='' current='OPENIMAGEDENOISE' choices=[]
E8 scene.cycles denoising_input_passes type=ENUM label='Denoising Input Passes' default='RGB_ALBEDO_NORMAL' current='RGB_ALBEDO_NORMAL' choices=['RGB', 'RGB_ALBEDO', 'RGB_ALBEDO_NORMAL']
E8 scene.cycles denoising_prefilter type=ENUM label='Denoising Prefilter' default='ACCURATE' current='ACCURATE' choices=['NONE', 'FAST', 'ACCURATE']
E8 scene.cycles denoising_quality type=ENUM label='Denoising Quality' default='HIGH' current='HIGH' choices=['HIGH', 'BALANCED', 'FAST']
E8 scene.cycles denoising_use_gpu type=BOOLEAN label='Denoise on GPU' default=False current=False
E8 scene.cycles preview_denoiser type=ENUM label='Viewport Denoiser' default='' current='AUTO' choices=[]
E8 scene.cycles preview_denoising_input_passes type=ENUM label='Viewport Denoising Input Passes' default='RGB_ALBEDO' current='RGB_ALBEDO' choices=['RGB', 'RGB_ALBEDO', 'RGB_ALBEDO_NORMAL']
E8 scene.cycles preview_denoising_prefilter type=ENUM label='Viewport Denoising Prefilter' default='FAST' current='FAST' choices=['NONE', 'FAST', 'ACCURATE']
E8 scene.cycles preview_denoising_quality type=ENUM label='Viewport Denoising Quality' default='BALANCED' current='BALANCED' choices=['HIGH', 'BALANCED', 'FAST']
E8 scene.cycles preview_denoising_start_sample type=INT label='Start Denoising' default=1 current=1
E8 scene.cycles preview_denoising_upscale_quality type=ENUM label='Viewport Denoising Upscale Quality' default='BALANCED' current='BALANCED' choices=['NONE', 'QUALITY', 'BALANCED', 'PERFORMANCE', 'ULTRA_PERFORMANCE']
E8 scene.cycles preview_denoising_use_gpu type=BOOLEAN label='Denoise Preview on GPU' default=True current=True
E8 scene.cycles use_denoising type=BOOLEAN label='Use Denoising' default=True current=True
E8 scene.cycles use_preview_denoising type=BOOLEAN label='Use Viewport Denoising' default=False current=False
E8 scene.render compositor_denoise_device type=ENUM label='Compositor Denoise Node Device' default='AUTO' current='AUTO' choices=['AUTO', 'CPU', 'GPU']
E8 scene.render compositor_denoise_final_quality type=ENUM label='Compositor Final Denoise Quality' default='HIGH' current='HIGH' choices=['HIGH', 'BALANCED', 'FAST']
E8 scene.render compositor_denoise_preview_quality type=ENUM label='Compositor Preview Denoise Quality' default='BALANCED' current='BALANCED' choices=['HIGH', 'BALANCED', 'FAST']
E8 scene.eevee NONE
E8 scene.eevee.ray_tracing_options denoise_bilateral type=BOOLEAN label='Bilateral Filter' default=True current=True
E8 scene.eevee.ray_tracing_options denoise_spatial type=BOOLEAN label='Spatial Reuse' default=True current=True
E8 scene.eevee.ray_tracing_options denoise_temporal type=BOOLEAN label='Temporal Accumulation' default=True current=True
E8 scene.eevee.ray_tracing_options use_denoise type=BOOLEAN label='Denoise' default=True current=True
E8 view_layer.cycles denoising_pass_follow_reflections type=BOOLEAN label='Denoising Pass Reflections' default=True current=True
E8 view_layer.cycles denoising_pass_use_albedo_roughness_weighting type=BOOLEAN label='Denoising Pass Albedo Roughness Weighting' default=True current=True
E8 view_layer.cycles denoising_store_motion_passes type=BOOLEAN label='Store Denoising Motion Passes' default=False current=False
E8 view_layer.cycles denoising_store_passes type=BOOLEAN label='Store Denoising Passes' default=False current=False
E8 view_layer.cycles use_denoising type=BOOLEAN label='Use Denoising' default=True current=True
E8 view_layer.eevee denoising_pass_use_albedo_roughness_weighting type=BOOLEAN label='Denoising Pass Albedo Roughness Weighting' default=True current=True
E8 view_layer.eevee denoising_store_passes type=BOOLEAN label='Denoising Data' default=False current=False
E8 view_layer.eevee all properties: ['ambient_occlusion_distance', 'denoising_pass_use_albedo_roughness_weighting', 'denoising_store_passes', 'use_pass_bloom', 'use_pass_transparent', 'use_pass_volume_direct']
```

EEVEE's scene-level `use_taa_reprojection` does not contain "denois" in its name and so is not in this list; its label is **Viewport Denoising** and its description "Denoise image using temporal reprojection (can leave some ghosting)" (entry 20, under question 2). The two view-layer settings that Cycles draws only with developer options can be set from outside with those options off:

```
E12 denoising_pass_follow_reflections present; use_cycles_debug=False show_developer_ui=False before=True
E12 denoising_pass_follow_reflections set to False -> now False
E12 denoising_pass_use_albedo_roughness_weighting present; use_cycles_debug=False show_developer_ui=False before=True
E12 denoising_pass_use_albedo_roughness_weighting set to False -> now False
```

Whether they change a picture was not tested.

**Which denoisers this Mac offers, and what decides it (entry 56).** On a factory startup, the final-render and viewport denoiser settings were each asked for a value that cannot exist, so that Blender's refusal names every real choice; then Cycles' preferences were read, device types tried, Metal chosen with its devices switched on, and the question asked again:

```
E9 factory denoiser choices <- bpy_struct: item.attr = val: enum "NO_SUCH_CHOICE_XYZ" not found in ('OPENIMAGEDENOISE')
E9 factory preview_denoiser choices <- bpy_struct: item.attr = val: enum "NO_SUCH_CHOICE_XYZ" not found in ('AUTO', 'OPENIMAGEDENOISE')
E9 compute_device_type='METAL'
E9 devices_for_type CUDA -> []
E9 devices_for_type OPTIX -> []
E9 devices_for_type HIP -> []
E9 devices_for_type ONEAPI -> []
E9 devices_for_type METAL -> [('Apple M1 Pro (GPU - 14 cores)', True), ('Apple M1 Pro', False)]
E9 set compute_device_type=OPTIX REFUSED TypeError: bpy_struct: item.attr = val: enum "OPTIX" not found in ('NONE', 'METAL')
E9 set compute_device_type=CUDA REFUSED TypeError: bpy_struct: item.attr = val: enum "CUDA" not found in ('NONE', 'METAL')
E9 set compute_device_type=NO_SUCH_TYPE REFUSED TypeError: bpy_struct: item.attr = val: enum "NO_SUCH_TYPE" not found in ('NONE', 'METAL')
E9 after compute_device_type=METAL, devices on: [('Apple M1 Pro', 'CPU', True), ('Apple M1 Pro (GPU - 14 cores)', 'METAL', True)]
E9 after METAL chosen denoiser choices <- bpy_struct: item.attr = val: enum "NO_SUCH_CHOICE_XYZ" not found in ('OPENIMAGEDENOISE')
E9 after METAL chosen preview_denoiser choices <- bpy_struct: item.attr = val: enum "NO_SUCH_CHOICE_XYZ" not found in ('AUTO', 'OPENIMAGEDENOISE')
```

So on this Mac the final render offers OpenImageDenoise alone, the viewport offers `AUTO` and OpenImageDenoise, neither OptiX nor the new DLSS denoiser is offered, choosing Metal changes nothing about it, and this Mac's device types are `NONE` and `METAL` only. `AUTO` is a viewport choice and not a final-render one. What a machine with an NVIDIA, AMD or Intel card offers was not tried: there is none.

**Whether the scene's and the view layer's denoising switches must both be on (entry 57).** The noisy test scene (320 by 240 as saved, set to 64 by 48 at launch) on Cycles with 8 samples and adaptive sampling off, rendered once for each combination of the scene's Denoise switch and the view layer's Denoising switch, plus a repeat of one as a control; every render exited 0. The pictures compared channel by channel:

```
PIXELCASE on_on vs off_off: PIXELS max_difference 0.6196078658103943 channels_differing 6350 of 12288
PIXELCASE on_off vs off_off: PIXELS max_difference 0.0 channels_differing 0 of 12288
PIXELCASE off_on vs off_off: PIXELS max_difference 0.0 channels_differing 0 of 12288
PIXELCASE on_on vs on_off: PIXELS max_difference 0.6196078658103943 channels_differing 6350 of 12288
PIXELCASE off_off vs off_off2: PIXELS max_difference 0.0 channels_differing 0 of 12288
```

(`on_off` is the scene's switch on and the layer's off.) The passes a view layer offers through a Render Layers node, in six combinations of the two switches and Store Denoising Passes, read three times over with the same result character for character:

```
E7P scene=off layer=off store=off denoising-related=[]  total_sockets=2
E7P scene=on  layer=on  store=off denoising-related=['Noisy Image']  total_sockets=3
E7P scene=on  layer=off store=off denoising-related=[]  total_sockets=2
E7P scene=off layer=on  store=off denoising-related=[]  total_sockets=2
E7P scene=off layer=off store=on  denoising-related=['Denoising Albedo', 'Denoising Specular Albedo', 'Denoising Normal', 'Denoising Roughness', 'Denoising Depth']  total_sockets=7
E7P scene=on  layer=on  store=on  denoising-related=['Noisy Image', 'Denoising Albedo', 'Denoising Specular Albedo', 'Denoising Normal', 'Denoising Roughness', 'Denoising Depth']  total_sockets=8
E7P blender 5.3.0 Beta
```

So Cycles denoised the view layer only with both switches on: either switch alone left the picture identical to both off, to the last channel. The Noisy Image pass appears under the same condition; Store Denoising Passes brings its five feature passes whatever the two switches say.

**Whether the scene's compositor denoise quality reaches a render, under both engines (entry 58).** The noisy test scene at 96 by 72, 8 samples, adaptive sampling off, Cycles' own denoising off, and a compositor of Render Layers → Denoise → output, so the node was the only thing denoising. The Denoise node, asked for a `quality` of its own, has none (`E9C node_quality='NO-QUALITY-ON-NODE'`); its inputs are `['Image', 'Albedo', 'Normal', 'HDR', 'Prefilter', 'Quality']`, where `Quality` is a menu input reading `'Follow Scene'` on a new node. The frame was rendered with `compositor_denoise_final_quality` at Fast and at High under Cycles and under EEVEE, plus a repeat at High under Cycles as a control, and the pictures compared — twelve rounds in one sitting. Every round gave the same three lines:

```
E9CASE cy_high vs cy_high_repeat : PIXELS max_difference 0.0 channels_differing 0 of 27648
E9CASE cy_fast vs cy_high : PIXELS max_difference 0.05098041892051697 channels_differing 9212 of 27648
E9CASE ee_fast vs ee_high : PIXELS max_difference 0.027450978755950928 channels_differing 3276 of 27648
```

So the scene's setting changes a final render under both engines, the control shows two renders at one setting are identical, and in this version on this Mac the counts did not move across the twelve rounds.

**A file whose stored denoiser is one this Mac does not offer (entry 59).** This Mac refuses the name outright — `SET denoiser='OPTIX' REFUSED TypeError: bpy_struct: item.attr = val: enum "OPTIX" not found in ('OPENIMAGEDENOISE')` — so the file was made by writing the stored value directly as the number 2, the number Cycles' own file gives the OptiX choice (`STORED raw value 2`): the noisy test scene at 96 by 72, 8 samples, adaptive off, denoising on. (In the earlier versions such a file was made on a machine with an NVIDIA card and carried to the Mac; no such machine was available here, so the file is made differently, and that is the one difference in the set-up.) Opened, read and rendered, exit status 0:

```
00:00.323  bpy.rna          | WARNING current value '2' matches no enum in 'CyclesRenderSettings', '', 'denoiser'
00:00.460  render           | Saved: '/private/tmp/claude-501/smoozy-hand-tests-5.3/Settings/out/e59/saved_0001.png'
READDEN denoiser_now='' use_denoising=True
```

The same file rendered with nothing reading the setting printed only its `Saved:` line — no warning. Its picture against the read-and-rendered one: `PIXELS max_difference 0.0 channels_differing 0 of 27648`. Against the same file rendered with denoising switched off: `PIXELS max_difference 0.7215686962008476 channels_differing 13844 of 27648`. Assigning the stored value back to itself:

```
00:00.321  bpy.rna          | WARNING current value '2' matches no enum in 'CyclesRenderSettings', '', 'denoiser'
READDEN re-assigning the value it already holds: REFUSED TypeError: bpy_struct: item.attr = val: enum "" not found in ('OPENIMAGEDENOISE')
```

So the render goes ahead and the picture is still denoised — it differs from denoising off by about fourteen thousand channels — while the setting reads back as an empty string, a warning appears only when a script reads it, and a run that never reads it says nothing.

**Whether the view layer's Denoising checkbox is drawn dimmed when the scene's Denoise is off (entry 66).** Blender 5.3 was opened with its window from the terminal (a throwaway settings folder), Cycles chosen, the Properties editor enlarged and turned to View Layer with its search box set to "Denoising" (the script read back `SEARCH 'Denoising' scene denoise True`), and photographed; then the scene's Denoise was switched off by the script (`scene denoise False layer denoise True`) and the window photographed again. Each state was photographed twice and each pair is identical. The two states differ in **1417 pixels, all inside one box from 1263 to 1395 across and 1629 to 1653 down** of the 3024 by 1724 picture — the `Denoising` row at the foot of the Filter panel, and nothing else on the screen. With the scene's Denoise **on**, that checkbox is bright blue and ticked and its label white; with it **off**, the checkbox is a muted blue, still ticked, and its label grey — drawn like the greyed `Motion Blur` row above it. So the switch is dimmed, not cleared and not hidden.

---

## Things a person at a computer with Blender 5.3 could settle in a few minutes

*(Pruned on 2026-10-17. Nine of the eleven items that stood here were run on the Creator's Mac and are written into the answers above as observed: which scene renders with no `-S` (question 12), the resolution percentage's clamping (questions 14 and 15), the sample counts' clamping (question 18), an animated sample count and pressing I on "Use for Rendering" (questions 5 and 1), a view layer's own count under each Layer Samples policy (question 16), the engine list and an unknown engine (question 13), a file's "GPU Compute" under preferences set to None (question 9), whether scripts run with no switch (question 11), and the Time Limit's display and scope (question 2). The whole record, with every command, is `Audits/Blender Research/Settings/Hand tests — 5.3.md`. What is left is genuinely open.)*

1. **Headless rendering on a machine with no display server (question 6).** Not run: no Linux machine and no Windows machine were available to this sitting, and the Creator's Mac has a screen. On such a machine, run `blender -b file.blend -E CYCLES -f 1`, then the same with `-E BLENDER_EEVEE` and `-E BLENDER_WORKBENCH`, and note which produce an image and what is printed.
2. **Opening a file saved by a newer Blender (question 10).** Not run: no Blender newer than 5.3 exists to write one. When one does, save a file in it, run `blender -b thatfile.blend -f 1` in 5.3, and copy any warning printed.
3. **Anything that needs an NVIDIA, AMD or Intel graphics card (questions 9 and 19).** Not run: the Creator's Mac has Apple's own graphics only. What `--cycles-device CUDA`, `OPTIX`, `HIP` or `ONEAPI` does where such a card exists, what a file's "GPU Compute" means there, which denoisers are offered, and whether Prefilter and Quality do anything under OptiX.

---

2026-10-16 — Claude Opus 5.5 (Claude Code) — Settings, Blender 5.3

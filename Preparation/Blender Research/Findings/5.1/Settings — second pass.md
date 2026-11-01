# Settings — Blender 5.1 — second pass (questions 1 and 8)

**Sources consulted, and the date.** Blender 5.1 manual (`Blender Sources/manual_5.1`, including pictures in its `_images` folder), Blender 5.1 Python API documentation (`Blender Sources/api_5.1`), Blender 5.1 release notes (`Blender Sources/release_notes_5.1`), and Cycles' own files (`Blender Sources/cycles_5.1`). All read on **2026-07-30** (the snapshot date given to me).

**I read only the question sheet and files inside Blender Sources/. I did not open any other part of this project.**

This is a second pass covering questions 1 and 8 only. I did not open the existing findings document. Because this pass exists to leave a record of looking, each answer carries a **Where I looked** section listing the files searched, the terms searched for, the pages opened, and what was ruled out.

---

## Question 1

*Which of a render's settings can be settled at the moment a render is started from outside Blender, and which can only be changed by changing the file? Cover at least a frame range, an interval to skip frames by, a list of separate frames, a sample count, whether a view layer renders at a given frame, a render engine, and a resolution percentage.*

### Reading of the question

I read "settled at the moment a render is started from outside Blender" as: given when Blender is launched to render, without editing and re-saving the blend-file. That is the reading I answer.

The question was mildly ambiguous in one place. "Whether a view layer renders at a given frame" can be read as (a) whether a particular view layer is rendered at all, and (b) whether that can differ from frame to frame within one animation render. I answer both, and mark which is which.

### The short shape of the answer

The sources describe **two** ways to settle something at launch:

1. A fixed list of **dedicated command-line arguments**, listed on one page of the manual.
2. **Python run at launch** (`-P` / `--python-expr` / `--python-text`), which the manual's own rule about argument order places *before* the argument that triggers the render.

Because of (2), the sources do not support a clean list of settings that can *only* be changed by editing the file. What they do support is a clean list of settings that have a **dedicated argument** of their own. I answer on that basis and say so explicitly.

### Item by item

**Frame range — has dedicated arguments. LABEL: documented.**

> `-s, --frame-start <frame>`
>
> Set start to frame `<frame>`, supports +/- for relative frames too.
>
> `-e, --frame-end <frame>`
>
> Set end to frame `<frame>`, supports +/- for relative frames too.

Manual, https://docs.blender.org/manual/en/5.1/advanced/command_line/arguments.html — read 2026-07-30.

The manual's worked example confirms the intent:

> `blender -b file.blend -E CYCLES -s 10 -e 500 -t 2 -a`
>
> `-s 10 -e 500`
>
> Set the start frame to 10 and the end frame to 500.

Manual, https://docs.blender.org/manual/en/5.1/advanced/command_line/render.html — read 2026-07-30.

In the file, the same pair is the Frame Range panel:

> Frame Start, End
>
> Set the Start and End frames for Rendering Animations.

Manual, https://docs.blender.org/manual/en/5.1/render/output/properties/frame_range.html — read 2026-07-30. In the Python API these are `Scene.frame_start` ("First frame of the playback/rendering range (in [0, 1048574], default 1)") and `Scene.frame_end` ("Final frame of the playback/rendering range (in [0, 1048574], default 250)"), https://docs.blender.org/api/5.1/bpy.types.Scene.html — read 2026-07-30.

**An interval to skip frames by — has a dedicated argument. LABEL: documented.**

> `-j, --frame-jump <frames>`
>
> Set number of frames to step forward after each rendered frame.

Manual, https://docs.blender.org/manual/en/5.1/advanced/command_line/arguments.html — read 2026-07-30.

The file's equivalent:

> Step
>
> Controls the number of frames to advance for each frame rendered, a value above 1 skips frames.
> This is also used when viewing rendered animations.

Manual, https://docs.blender.org/manual/en/5.1/render/output/properties/frame_range.html — read 2026-07-30. In the API, `Scene.frame_step`: "Number of frames to skip forward while rendering/playing back each frame (in [0, 1048574], default 1)", https://docs.blender.org/api/5.1/bpy.types.Scene.html — read 2026-07-30.

**A list of separate frames — has a dedicated argument. LABEL: documented.**

> `-f, --render-frame <frame>`
>
> Render frame `<frame>` and save it.
>
> `+<frame>` start frame relative, `-<frame>` end frame relative.
>
> A comma separated list of frames can also be used (no spaces).
>
> A range of frames can be expressed using `..` separator between the first and last frames (inclusive).

Manual, https://docs.blender.org/manual/en/5.1/advanced/command_line/arguments.html — read 2026-07-30.

So a list of separate frames, and a range, are both expressible on the one `-f` argument. I found no equivalent list stored in the file; the file carries a start, an end and a step only (frame_range page above).

**A sample count — no dedicated argument. LABEL: not in the sources (as a launch-time argument) / observed (as a file setting).**

There is no command-line argument for a sample count anywhere in the sources. The manual's full argument page has a section "Cycles Render Options" and it contains exactly two entries:

> Cycles add-on options must be specified following a double dash.
>
> `--cycles-device <device>` … `--cycles-print-stats`

Manual, https://docs.blender.org/manual/en/5.1/advanced/command_line/arguments.html — read 2026-07-30.

Cycles' own files agree, and this is the strongest evidence available because Cycles registers its arguments in Python that I can read directly. **LABEL: observed.**

```
def _configure_argument_parser():
    import argparse
    # No help because it conflicts with general Python scripts argument parsing
    parser = argparse.ArgumentParser(description="Cycles Addon argument parser",
                                     add_help=False)
    parser.add_argument("--cycles-print-stats",
                        help="Print rendering statistics to stderr",
                        action='store_true')
    parser.add_argument("--cycles-device",
                        ...
```

`Blender Sources/cycles_5.1/engine.py`, lines 8–21 (function `_configure_argument_parser`) — read 2026-07-30. The whole of `cycles_5.1` contains only these two `add_argument` calls (I searched the folder for `add_argument`, `argparse`, `sys.argv`; the only hits are in `engine.py` at lines 9, 11, 13, 16 and 27).

*The step from file to behaviour:* the file says these two arguments are the ones the add-on's parser is given. That Cycles therefore accepts no other `--cycles-…` argument at run time is one step beyond the words, so **that part is inferred**, not observed.

The sample count itself is a per-engine file setting. In Cycles' own files:

```
    samples: IntProperty(
        name="Samples",
        description="Number of samples to render for each pixel",
        min=1, max=(1 << 24),
        default=4096,
    )
```

`Blender Sources/cycles_5.1/properties.py`, lines 479–484 — read 2026-07-30. **LABEL: observed.**

And in the manual:

> Render (Max) Samples
>
> The number of paths to trace per pixel in the final render.
> A higher number results in a cleaner image at the cost of a longer render time.

Manual, https://docs.blender.org/manual/en/5.1/render/cycles/render_settings/sampling.html — read 2026-07-30.

**Whether a view layer renders — no dedicated argument. LABEL: documented (for the setting), inferred (for per-frame control).**

There is no command-line argument for it. What the file holds:

> Use for Rendering
>
> The active view layer will be used during rendering.
>
> Render Single Layer
>
> Only render the active view layer.
>
> Note
>
> This option is ignored when rendering from the command line.

Manual, https://docs.blender.org/manual/en/5.1/render/layers/view_layer.html — read 2026-07-30.

The API says the same twice over. `ViewLayer.use`: "Enable or disable rendering of this View Layer (default True)", https://docs.blender.org/api/5.1/bpy.types.ViewLayer.html. And `RenderSettings.use_single_layer`: "Only render the active layer. Only affects rendering from the interface, ignored for rendering from command line. (default False)", https://docs.blender.org/api/5.1/bpy.types.RenderSettings.html — both read 2026-07-30.

That last one is the single place in everything I searched where the sources name a render setting that behaves differently from the command line than inside the interface.

On the "at a given frame" part of the question. I opened the manual's picture of the View Layer panel, `_images/render_layers_view-layer_panel.png` (the picture named in https://docs.blender.org/manual/en/5.1/render/layers/view_layer.html). Enlarged, it shows the row "Use for Rendering" with a ticked checkbox and a **small dot to the right of it**; the row below it, "Render Single Layer", has an unticked checkbox and **no such dot**. **LABEL for what is visible: documented** (it is part of the manual).

The manual explains what that dot is:

> Decorators are small buttons that appear to the right of other buttons and show the state of the property.
> Decorators may appear next to number fields, menus,
> and checkboxes to indicate the property can be animated.
>
> Clicking on the decorator dot icon will add a Keyframe to that property.

Manual, https://docs.blender.org/manual/en/5.1/interface/controls/buttons/decorators.html — read 2026-07-30.

Putting the picture together with that page gives: "Use for Rendering" can be animated, so whether a view layer renders can differ from frame to frame. **LABEL: inferred.** The step a reader has to supply is that the dot in that particular picture is a decorator in the sense of the decorators page, and that "can be animated" means the value can differ per frame. Neither page says this about `ViewLayer.use` in words.

One further mechanism exists but is a Python one, not a launch argument. The render operator takes a layer:

> `bpy.ops.render.render(*, animation=False, write_still=False, use_viewport=False, use_sequencer_scene=False, layer='', scene='', frame_start=0, frame_end=0)`
>
> `layer` (str) – Render Layer, Single render layer to re-render (used only when animation is disabled) (optional, never None)

API, https://docs.blender.org/api/5.1/bpy.ops.render.html — read 2026-07-30. Note the words "used only when animation is disabled" — by the operator's own description this does not apply to an animation render.

**A render engine — has a dedicated argument. LABEL: documented.**

> `-E, --engine <engine>`
>
> Specify the render engine.
> Use `-E help` to list available engines.

Manual, https://docs.blender.org/manual/en/5.1/advanced/command_line/arguments.html — read 2026-07-30. And the worked example:

> `-E CYCLES`
>
> Use the "Cycles Render" engine.
> For a list of available render engines, run `blender -E help`.

Manual, https://docs.blender.org/manual/en/5.1/advanced/command_line/render.html — read 2026-07-30.

In the file it is `RenderSettings.engine`: "Engine to use for rendering (default 'BLENDER_EEVEE')", https://docs.blender.org/api/5.1/bpy.types.RenderSettings.html — read 2026-07-30.

**A resolution percentage — no dedicated argument. LABEL: not in the sources (as a launch-time argument) / documented (as a file setting).**

There is no command-line argument for it. In the file:

> Percentage
>
> Slider to reduce or increase the size of the rendered image relative to the Resolution values.
> This is useful for small test renders that have the same proportions as the final image.

Manual, https://docs.blender.org/manual/en/5.1/render/output/properties/format.html — read 2026-07-30. In the API, `RenderSettings.resolution_percentage`: "Percentage scale for render resolution (in [1, 32767], default 100)", https://docs.blender.org/api/5.1/bpy.types.RenderSettings.html — read 2026-07-30.

### The other render settings that do have a dedicated argument

For completeness of the list the question asks about ("which of a render's settings"), the manual's Render Options, Cycles Render Options and Format Options sections name these and no others. **LABEL: documented**, same page and date as above:

- `-b, --background` — "Run in background (often used for UI-less rendering)."
- `-a, --render-anim` — "Render frames from start to end (inclusive)."
- `-S, --scene <name>` — "Set the active scene `<name>` for rendering."
- `-o, --render-output <path>` — "Set the render path and file name."
- `-t, --threads <threads>` — "Use amount of `<threads>` for rendering and other operations [1-1024], 0 to use the systems processor count."
- `-F, --render-format <format>` — "Set the render format." (TGA RAWTGA JPEG IRIS PNG BMP HDR TIFF, plus OPEN_EXR OPEN_EXR_MULTILAYER FFMPEG CINEON DPX JP2 WEBP where compiled in)
- `-x, --use-extension <bool>` — "Set option to add the file extension to the end of the file."
- `--cycles-device <device>` — "Set the device used for rendering. Valid options are: CPU CUDA OPTIX HIP ONEAPI METAL."
- `--cycles-print-stats` — "Log statistics about render memory and time usage."

### The catch that undoes any clean "only by changing the file" list

**LABEL: documented.** The manual gives Python arguments that run at launch:

> `-P, --python <filepath>`
>
> Run the given Python script file.
>
> `--python-expr <expression>`
>
> Run the given expression as a Python script.
>
> The expression may be a complete multi-line script;
> you are limited only by the platform's maximum argument length.

And the ordering rule that makes them usable for this:

> Argument Order
>
> Arguments are executed in the order they are given. eg:
>
> `blender --background test.blend --render-frame 1 --render-output "/tmp"`
>
> …will not render to /tmp because `--render-frame 1` renders before the output path is set.
>
> `blender --background --render-output /tmp test.blend --render-frame 1`
>
> …will not render to /tmp because loading the blend-file overwrites the render output that was set.
>
> `blender --background test.blend --render-output /tmp --render-frame 1`
>
> …works as expected.

Both from the manual, https://docs.blender.org/manual/en/5.1/advanced/command_line/arguments.html — read 2026-07-30. The manual repeats the point on the rendering page: "Always position `-f` or `-a` as the last arguments." (https://docs.blender.org/manual/en/5.1/advanced/command_line/render.html).

**LABEL: inferred.** Because a Python expression can be run at launch, after the file is loaded and before the render is triggered, a setting with no dedicated argument — a sample count, a resolution percentage, a view layer's Use for Rendering — can still be settled at launch without changing the file. The step I am supplying: the sources do not say in so many words that Python may set these particular properties at launch; they say Python runs when the argument is reached, that arguments run in order, and separately that these properties exist and are writable (the API lists them without a `readonly` note, unlike e.g. `use_spherical_stereo` which is marked "readonly"). I have not seen a page that puts those together.

So the honest answer to the second half of the question is: **the sources do not establish any render setting that can only be changed by changing the file.** What they establish is which settings have an argument of their own — frame start, frame end, frame jump, a frame or list or range of frames, engine, scene, output path, output format, extension flag, thread count, Cycles device, Cycles stats — and which do not — sample count, resolution percentage, whether a view layer renders.

One further wrinkle, **LABEL: documented**: an add-on can add its own launch-time arguments, which is exactly how Cycles' two arrive.

> `bpy.utils.register_cli_command(id, execute)`
>
> Register a command, accessible via the (-c / --command) command-line argument.

API, https://docs.blender.org/api/5.1/bpy.utils.html — read 2026-07-30. So the list of arguments on a given installation is not fixed by Blender alone. The manual's `-c, --command <command>` entry says "Run a command which consumes all remaining arguments. Use `-c help` to list all other commands." and "This implies `--background` mode."

### Not covered

- Whether `-s`, `-e`, `-j`, `-E`, `-F`, `-o`, `-x`, `-t` write their values back into the blend-file, or only apply to that run. The sources do not say. (The ordering rule does say that loading the blend-file overwrites a render output set before it, which speaks to order, not to saving.)
- Whether `-j / --frame-jump` applies to `-f / --render-frame` as well as to `-a / --render-anim`. Not stated.
- Whether a comma-separated `-f` list may be combined with `..` ranges in the same argument, and what happens to duplicate or out-of-order entries. Not stated.
- What `-E help` prints, and whether an id from it is exactly what `-E` accepts. Not stated beyond "list available engines".
- Whether EEVEE or Workbench register any launch-time arguments of their own. They are not add-ons with files in this folder, and I found no `--eevee`-style argument anywhere; but I cannot show their absence the way I can for Cycles.
- The maximum length of `--python-expr` — the manual defers to "the platform's maximum argument length" without giving one.
- Whether animating "Use for Rendering" actually changes which layers appear in an animation render. That is behaviour, and the sources describe the control, not the outcome.

### Outside the interface

This answer is entirely about outside the interface — the command-line arguments page is the source for most of it, and that page describes nothing else. The differences from inside are:

- **Documented divergence.** "Render Single Layer" / `use_single_layer`: "This option is ignored when rendering from the command line" (manual) and "Only affects rendering from the interface, ignored for rendering from command line" (API). This is the only such divergence I found in the render area.
- **Documented.** `-b, --background` exists precisely for this case: "Run in background (often used for UI-less rendering)", and "The audio device is disabled in background-mode by default and can be re-enabled by passing in `-setaudio Default` afterwards."
- **Documented.** One argument changes behaviour only in background mode: `--disable-depsgraph-on-file-load` — "Background mode: Do not systematically build and evaluate ViewLayers' dependency graphs when loading a blend-file in background mode (`-b` or `-c` options)." with the note "this is a temporary option, in the future depsgraph will never be automatically generated on file load in background mode."
- Inside the interface, the same settings are set in the Properties editor panels quoted above rather than by argument.

All from the pages already cited, read 2026-07-30.

### Operating system

**LABEL: inferred**, and the step is small but real. The arguments page marks operating-system limits explicitly wherever they exist — `-con, --start-console` "(Windows only)", `--no-window-frame` "(Linux only)", `--debug-ghost` "(Linux only)", `/?` "(Windows only)", `-r, --register` "(Windows & Linux only)", `--qos <level>` "(Windows only)", `SPNAV_SOCKET` "(Unix only)", `TEMP` "(MS-Windows)", `TMPDIR` "(UNIX Systems)". **None** of the Render Options, Cycles Render Options or Format Options carries such a note. From that I take it these arguments are the same on every operating system; the page does not say so outright.

Two things do differ by operating system, both documented:

- How you invoke Blender at all. The manual gives a page each for Linux (`./blender`), macOS (`./Blender` inside `/Applications/Blender.app/Contents/MacOS`), and Windows (`blender` from the install directory), at https://docs.blender.org/manual/en/5.1/advanced/command_line/launch/linux.html, .../macos.html and .../windows.html — read 2026-07-30.
- Which `--cycles-device` values are meaningful. The listed values are "CPU CUDA OPTIX HIP ONEAPI METAL", and the sources do not claim all six exist everywhere.

**Observed 2026-08-05:** every test below was run on macOS only, so Windows and Linux stay untested; and only `METAL` and `CPU` could be reached, since this machine has Apple's own graphics and no other card.

**Observed 2026-08-06:** the same tests were run again on a rented Linux machine (see the Linux block at the end of this answer). Every argument this answer names behaves the same way there, so the answer now holds on macOS and on Linux; Windows stays untested. The argument list itself is not identical on the two — Blender's own help text is longer on Linux. *(Changed by the citation check of the Linux hand tests, 2026-08-06: this line still said Linux was untested while a Linux result sat below it.)*

### Observed — hand tests 2, 3, 4, 5 and 10 (2026-08-05)

All were run from a terminal on the Creator's own computer — a MacBook Pro with an M1 Pro chip, macOS 26.5.2, Apple's own graphics and no other graphics card — using Blender 5.1.2 at `/Applications/Blender 5.1.2.app/Contents/MacOS/Blender`, with the `BLENDER_USER_RESOURCES` setting pointed at a fresh empty folder so the Creator's own Blender settings were untouched. Label: **observed**.

**On the conditions of hand tests 2 and 3.** Those two were first run before it was known that Blender falls back to the machine's real settings when that folder does not exist, so the first attempts loaded the Creator's own add-ons. Those attempts were discarded. Both were re-run from scratch with the folder created first — hand test 3 with its files rebuilt as well — and each version was checked before use, reporting its own throwaway settings folder and Blender's eight default add-ons and nothing else. What is written below is the clean re-run.

**Hand test 5 — does `--python-expr` really settle a sample count, a resolution percentage and a view layer switch at launch?** A file was saved with Cycles, 512 samples, two view layers (`ViewLayer` and `Layer2`) and a 200 by 100 resolution at 100 per cent. With the expression after the blend-file argument:

```
blender -b test.blend --python-expr "import bpy; s=bpy.data.scenes[0]; s.cycles.samples=8; s.render.resolution_percentage=25; s.view_layers['Layer2'].use=False; print('SET', s.cycles.samples, s.render.resolution_percentage, s.view_layers['Layer2'].use)" -o out-after/f_ -F PNG -f 1
```

printed `SET 8 25 False` and wrote an image of `pixelWidth: 50   pixelHeight: 25`. With the expression before the blend-file argument it printed `SET 8 25` and wrote `pixelWidth: 200   pixelHeight: 100`.

**This settles the inferred step above.** All three settings can be settled at launch by Python without changing the file, and the ordering rule bites exactly as the manual describes it for `--render-output`: the expression must come after the file.

**Hand test 3 — does `-j` apply to `-f`, and does `-f` take a mixed list and range?** A file was saved from factory startup with Workbench at 64 by 64. Five renders, each into its own empty folder, wrote:

```
blender -b test.blend -o out-list/f_  -F PNG -f 1,5,9        -> f_0001.png f_0005.png f_0009.png
blender -b test.blend -o out-range/f_ -F PNG -f 3..6         -> f_0003.png f_0004.png f_0005.png f_0006.png
blender -b test.blend -o out-mixed/f_ -F PNG -f 1,5..7,12    -> f_0001.png f_0005.png f_0006.png f_0007.png f_0012.png
blender -b test.blend -j 3 -o out-jumpf/f_ -F PNG -f 1..10   -> f_0001.png … f_0010.png (all ten)
blender -b test.blend -j 3 -s 1 -e 10 -o out-jumpa/f_ -F PNG -a -> f_0001.png f_0004.png f_0007.png f_0010.png
```

No error was printed by any of the five. **This closes two of the "not covered" items above:** a comma list and a `..` range may be mixed in one `-f`, with no error and no duplicate; and `-j` is applied to `-a` and not to `-f`.

**Hand test 4 — do the launch arguments write their values back into the file?** A file was saved with frame start 1, frame end 250, frame step 1, resolution percentage 100, engine `BLENDER_EEVEE`. After `blender -b test.blend -E CYCLES -s 5 -e 7 -j 2 -o out/f_ -F PNG -f 5`, the file's modification time and size were unchanged (`1785890069`, `95372 bytes`), and reading it back with no arguments printed:

```
READBACK 1 250 1 100 BLENDER_EEVEE
```

**This closes the first "not covered" item above:** the arguments apply to that run only.

**Hand test 2 — what `-E help` prints, and what an unknown id does.** `blender -E help` printed:

```
Blender Engine Listing:
	BLENDER_EEVEE
	BLENDER_WORKBENCH
	CYCLES
```

exit code 0 — identifiers, not human labels. `blender -b test.blend -E NOT_A_REAL_ENGINE -o out/e_ -F PNG -f 1` printed `Error: engine not found 'NOT_A_REAL_ENGINE'`, exit code 1, no image. **This closes another "not covered" item:** an id from that list is exactly what `-E` accepts.

The same run of `blender --help` also showed that this build carries both `--python-use-system-env` and `--python-use-user-env`, and that Blender 4.5.12 and 5.0.1 carry only the first.

**Hand test 10 — does animating "Use for Rendering" actually change which layers appear?** A file was saved with two view layers, `Layer2`'s `use` keyframed on at frame 1 and off at frame 2, Workbench, 64 by 64. `keyframe_insert` on that property returned `True`. Rendering with `blender -b test.blend -o out/f_ -F OPEN_EXR_MULTILAYER -s 1 -e 2 -a` named these layers in its progress lines:

```
Rendering frame 1 -> Scene, ViewLayer  and  Scene, Layer2
Rendering frame 2 -> Scene, ViewLayer
```

and the channel names in the two written files were:

```
f_0001.exr : Layer2.Combined  Layer2.CombinedpixelAspectRatio  ViewLayer.Combined  ViewLayer.CombinedpixelAspectRatio   (8411 bytes)
f_0002.exr : ViewLayer.Combined  ViewLayer.CombinedpixelAspectRatio                                                     (4394 bytes)
```

**This closes the last "not covered" item above.** The behaviour follows the control: an animation render driven from outside honours the switch frame by frame.

Separately, with `use_single_layer` turned on at launch, the run printed `SINGLE-LAYER ON, frame 1, layers on: ['ViewLayer', 'Layer2']` and the written file held both layers — so the documented divergence holds in practice too.

### Observed — hand test 25, run by the Creator (2026-08-05)

Run by the Creator in Blender 5.1's own window, on their own computer — a MacBook Pro with an M1 Pro chip, macOS 26.5.2 — on 2026-08-05. That settles macOS and leaves Windows and Linux untested. Label: **observed**.

They opened the Properties editor's View Layer Properties tab, on a view layer named `LayerA`, and photographed it. What is visible: a **View Layer** panel holding **Use for Rendering**, ticked, with a small round dot at the far right of its row; and **Render Single Layer**, unticked, with no dot.

They also wrote, in their own words:

> "Pressing I at Use for Rendering makes the checkbox yellow and the dot next to it a diamond. Consistent across all versions. Render Single Layer is non-animatable in all versions, too. Here's the message: "use_single_layer" property cannot be animated. Also consistent in all versions."

**This settles the inferred step above.** The dot in the manual's picture is a decorator in the sense of the decorators page, and "can be animated" does mean the value takes a keyframe: the running program inserts one, and marks the control as animated. It also settles the same thing for the row below by the opposite result — "Render Single Layer" is refused a keyframe, with the message quoted above, which the sources nowhere state.

**What the screenshots do not cover.** The right-click menu on "Use for Rendering" was not among the shots supplied; the Creator answered by pressing I instead. What that menu lists is still unrecorded.

**Still not settled.** This is about the control, not the outcome. What an animation render then does with the switch is hand test 10, above.

### Where I looked (question 1)

Searched, across `manual_5.1`, `api_5.1`, `release_notes_5.1` and `cycles_5.1` unless noted:

- Located the argument page by searching all of `manual_5.1` for `--render-frame`, `--frame-start`, `--render-anim`. Two files hit: `advanced/command_line/arguments.txt` and `advanced/scripting/security.txt`. Read `arguments.txt` end to end (all 691 lines).
- Read the whole of `advanced/command_line/render.txt`, `advanced/command_line/index.txt`, and the three `advanced/command_line/launch/*.txt` pages. Read the head of `advanced/command_line/extension_arguments.txt` and ruled it out — it documents `blender --command extension …`, package and repository management, nothing about render settings.
- Searched all four source sets for these argument spellings, **zero hits in every one**: `--samples`, `--sample-count`, `--cycles-samples`, `--max-samples`, `--time-limit`, `--noise-threshold`, `--resolution`, `--render-scale`, `--scale`, `--size`, `--width`, `--height`, `--percentage`, `--quality`, `--view-layer`, `--viewlayer`, `--render-layer`, `--use-layer`, `--frames`, `--frame-list`, `--eevee`.
- Searched `arguments.txt` itself, case-insensitively, for `layer`, `sample`, `percent`, `resolution`, `scene`. The only hits are `-S, --scene`, `OPEN_EXR_MULTILAYER` in the format list, "high resolution displays (MacBook Retina)" under `--no-native-pixels`, "OpenXR API validation layer" under `--debug-xr`, and "ViewLayers' dependency graphs" under `--disable-depsgraph-on-file-load`. None is a render setting.
- Searched all of `cycles_5.1` for `add_argument`, `argparse`, `sys.argv`, `--samples`, `--resolution`. Only `engine.py` hits, only the two arguments. Read `engine.py` lines 1–70 and `properties.py` around the sampling properties (lines 470–520).
- Searched `manual_5.1` for every mention of `command line` / `command-line` outside the `advanced/command_line` folder, and opened the ones that could bear on render settings: `render/layers/view_layer.txt` (the Render Single Layer note), `advanced/scripting/security.txt`, `render/cycles/gpu_rendering.txt` (a CUDA compiler environment variable, ruled out), `editors/preferences/system.txt` (console scroll-back lines, ruled out), `render/output/animation_player.txt` (playback, ruled out), `troubleshooting/*` (ruled out).
- Searched `manual_5.1` and `api_5.1` for every `blender -b` / `blender --background` example, to catch any argument used in an example but missing from the reference page. Found none beyond the reference page's own list.
- Read the file-side pages for each item asked about: `render/output/properties/frame_range.txt`, `render/output/properties/format.txt`, `render/output/properties/post_processing.txt`, `render/cycles/render_settings/sampling.txt`, `render/layers/view_layer.txt`, `render/layers/introduction.txt`, `render/introduction.txt`.
- Read in the API: `bpy.types.RenderSettings.txt` (searched it for `resolution_percentage`, `engine`, `use_single_layer`, `use_compositing`, `use_sequencer`), `bpy.types.Scene.txt` (`frame_start`, `frame_end`, `frame_step`, `frame_current`), `bpy.types.ViewLayer.txt` (`use`), `bpy.ops.render.txt` (the `render` operator's parameters), `bpy.utils.txt` (`register_cli_command`), `bpy.app.handlers.txt` (`frame_change_pre`, `frame_change_post`, `render_init`, `render_pre`, `render_post`, `render_write`, `render_stats`).
- Opened two pictures: `_images/render_layers_view-layer_panel.png` (the View Layer panel) and, to interpret the dot in it, read `interface/controls/buttons/decorators.txt`.
- Searched `release_notes_5.1` (all 21 files, 1611 lines) for the argument spellings above and read `rendering.md` in full — nothing about command-line arguments or launch-time settings.

Ruled out and why: the extensions command-line page (package management only); the animation-playback arguments `-p -m -f -j -s -e -c` (they are the player's, not the renderer's — the manual puts them under "Animation Playback Options" and says the mode is "ignored if `-b` is set"); the window, logging, debug and GPU-debug argument groups (they do not set render settings); the freestyle and stereoscopy pages (not about launch-time settings).

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

## Question 8

*Can one scene be set up to take in another scene's rendered result, so that rendering the first renders the second?*

**Yes. LABEL: documented.** The sources describe two separate mechanisms for it, and they behave differently.

### Mechanism one — the Render Layers node in the Compositor

> Render Layers Node
>
> Renders a View Layer and reads its
> Passes into the compositing node graph.
>
> …
>
> Properties
>
> Scene
>
> The scene for which to render a view layer.
>
> View Layer
>
> The view layer to render. The button next to the dropdown re-renders it immediately.

Manual, https://docs.blender.org/manual/en/5.1/compositing/types/input/scene/render_layers.html — read 2026-07-30.

The same page carries a hint that draws the boundary of what is taken in:

> Hint
>
> To use the compositing output from another scene rather than its "raw" render output,
> first render that scene into a series of multi-layered images (using e.g. the OpenEXR format),
> then load those images into the Compositor of the current scene using the
> Image Node.

Same page, same date. So what crosses the scene boundary is the other scene's raw render output and its passes, not the other scene's own composite.

I opened the picture the page names, `_images/node-types_CompositorNodeRLayers.webp`. It shows the node titled "Render Layers", with outputs Image and Alpha, and two fields below: a data-block field containing "Scene" with an X button, and a dropdown containing "ViewLayer" with a small button beside it. **LABEL: documented** (the picture is part of the manual). This confirms the Scene field is on the node face itself, not buried in a sidebar.

The API says the same in one line:

> class bpy.types.CompositorNodeRLayers(CompositorNode)
>
> Input render passes from a scene render

and lists a property:

> `scene`
>
> Type:
>
> Scene

API, https://docs.blender.org/api/5.1/bpy.types.CompositorNodeRLayers.html — read 2026-07-30.

A second manual page confirms in passing that these nodes routinely point at other scenes:

> The views to render are defined in the current scene views,
> in a similar way as you define the composite output resolution in the current scene render panel,
> regardless of the Image nodes resolutions or Render Layers from different scenes.

Manual, https://docs.blender.org/manual/en/5.1/render/output/properties/stereoscopy/usage.html — read 2026-07-30. That also settles a practical edge: the output resolution comes from the current scene, not from the scene being pulled in.

A third, from the Freestyle documentation, names the arrangement outright:

> This means that if Freestyle is used for two or more view layers
> (possibly in different scenes through the Compositor),
> then the cached view map for one view layer is replaced by a new view map
> for another view layer and hence no performance gain is expected.

Manual, https://docs.blender.org/manual/en/5.1/render/freestyle/view_layer/freestyle.html — read 2026-07-30.

For any of this to affect the final render, the compositing pipeline has to be on:

> Pipeline – Compositing
>
> Renders the output from the compositing node setup,
> and then applies the Composite node tree on all images,
> displaying the image inputted in the Group Output node.

Manual, https://docs.blender.org/manual/en/5.1/render/output/properties/post_processing.html — read 2026-07-30. The compositing introduction repeats it as an "Important" box: "To make the render engine actually use the compositing node tree (or disable compositing), the Compositing Pipeline option must be enabled in the output's Post Processing properties." (https://docs.blender.org/manual/en/5.1/compositing/introduction.html). In the API this is `RenderSettings.use_compositing`: "Process the render result through the compositing pipeline, if a compositing node group is assigned to the scene (default True)", https://docs.blender.org/api/5.1/bpy.types.RenderSettings.html — read 2026-07-30.

### Mechanism two — a Scene strip in the Video Sequencer

> Scene strips are a way to insert the render output of another scene into your sequence.
> Instead of rendering out a video, then inserting the video file, you can insert the scene directly.
>
> The strip length will be determined based on the animation settings in that scene.
>
> Note
>
> Scene strips cannot be used to reference the sequence's own scene; a secondary scene must be used instead.

Manual, https://docs.blender.org/manual/en/5.1/video_editing/edit/montage/strips/scene.html — read 2026-07-30.

That page also gives the choice of what is taken from the other scene:

> Input
>
> Input type to use for the Scene strip.
>
> Camera:
>
> Use the Scene's 3D camera as input.
>
> Sequencer:
>
> Use the scene's Sequencer timeline as input, allowing one scene to reuse
> another scene's edit (instead of taking the render output from the scene).

and a limitation:

> Limitations
>
> Scene strips do not render individual Render Passes;
> only the Combined render pass will be used.

Same page, same date.

The part of the question that says "so that rendering the first renders the second" is answered outright for this mechanism:

> Pipeline – Sequencer
>
> Renders the output of the Video Sequence editor, instead of the view from the 3D scene's active camera.
> If the sequence contains Scene strips, these will also be rendered as part of the pipeline.
> If Compositing is also enabled, the Scene strip will be the output of the Compositor.

Manual, https://docs.blender.org/manual/en/5.1/render/output/properties/post_processing.html — read 2026-07-30. In the API this is `RenderSettings.use_sequencer`: "Process the render (and composited) result through the video sequence editor pipeline, if sequencer strips exist (default True)", https://docs.blender.org/api/5.1/bpy.types.RenderSettings.html — read 2026-07-30.

The scene-organisation page describes the same arrangement as an intended workflow:

> Scenes can also be used as Scene Strips in the
> Video Sequencer.
> This allows you to composite or edit multiple scenes together in one timeline.
>
> For example, each shot of an animation can be stored in its own scene,
> and then combined in a master scene that uses scene strips to sequence them in order.
> This workflow is useful for multi-shot projects, trailers,
> or layout editing without the need to render intermediate files.
>
> When used as strips, scenes are rendered dynamically during playback or rendering,
> and they share the same data-blocks unless they were created as full copies.

Manual, https://docs.blender.org/manual/en/5.1/scene_layout/scene/introduction.html — read 2026-07-30. "Rendered dynamically during … rendering" is the plainest statement in the sources that rendering the first renders the second.

The API description of the strip type:

> class bpy.types.SceneStrip(Strip)
>
> Sequence strip using the rendered image of a scene

with properties `scene` ("Scene that this strip uses", type Scene), `scene_camera` ("Override the scene's active camera", type Object) and `scene_input` ("Input type to use for the Scene strip (default 'CAMERA')", with values CAMERA and SEQUENCER). API, https://docs.blender.org/api/5.1/bpy.types.SceneStrip.html — read 2026-07-30.

### A near neighbour that is not the same thing

**LABEL: documented.** The Scene properties offer a Background Scene:

> Background Scene
>
> Allows you to use a scene as a background,
> this is typically useful when you want to focus on animating the foreground for example,
> without background elements getting in the way.
>
> …
>
> Background Scenes can themselves have a Background Scene (they're recursively included).

Manual, https://docs.blender.org/manual/en/5.1/scene_layout/scene/properties.html — read 2026-07-30.

I flag this because it is easy to mistake for an answer to the question. The words say the other scene is *included* — its contents come into the current scene — not that its rendered result is taken in. The sources do not describe it as producing a separate render.

### Not covered

- Whether the second scene's own compositor runs when it is pulled in by a Render Layers node. The Hint on that page implies not (it tells you to render to files first if you want the other scene's composited output), which is as close as the sources come; they do not say it in so many words.
- Which scene's render engine, sample count, resolution or frame number apply to the pulled-in render. Only the output resolution is settled, and only for the stereoscopy case ("the composite output resolution in the current scene render panel"). Nothing on engine, samples or frame mapping.
- What happens if two scenes point Render Layers nodes at each other. The only self-reference rule I found is the Scene-strip one ("cannot be used to reference the sequence's own scene"); nothing equivalent is stated for the Render Layers node.
- Whether the pulled-in render is repeated per frame of an animation, or cached. The scene-introduction page says "rendered dynamically during playback or rendering" and the "Read View Layers" operator mentions a cache ("Reads all the current scene's render layers from cache, as needed… For this to work, Cache Result must be enabled", https://docs.blender.org/manual/en/5.1/interface/controls/nodes/editing.html), but that operator is described as current-scene only and interface only.
- The order in which multiple pulled-in scenes are rendered, and whether they render in parallel or one after another. Not stated anywhere I searched.
- Whether a scene in a *different blend-file* can be pulled in this way. The compositing introduction mentions node trees being "linked/appended from other blend-files", and the Background Scene page mentions "Linking to a Scene", but neither says a Render Layers node or Scene strip may point across files.
- The 5.1 release notes say nothing about any of this. Their compositor, rendering and sequencer pages carry no change touching cross-scene input; the only hits for "Scene Strip" in the whole of `release_notes_5.1` are two bug-fix lines: "VSE Duplicating Scene Strips does not re-assign camera" and "VSE Camera Doesn't Update when Scene Strip Changes" (https://developer.blender.org/docs/release_notes/5.1/bugfixes/).

### Outside the interface

**LABEL: not in the sources, with one aside.** Both mechanisms are file contents — a node in a scene's compositing node group, or a strip in a scene's sequencer — and both are governed by file settings (`use_compositing`, `use_sequencer`, both defaulting True). The sources nowhere state that either behaves differently when Blender is driven from outside its window, and I searched for such a statement (see below). The one place in the whole render area where the sources *do* name a command-line difference is "Render Single Layer" / `use_single_layer`, which is a different setting.

Two interface-only pieces sit near these mechanisms and do not carry over:

- On the Render Layers node, "The button next to the dropdown re-renders it immediately" — a button press.
- The Sequencer Scene is held on the *workspace* (`WorkSpace.sequencer_scene`, https://docs.blender.org/api/5.1/bpy.types.WorkSpace.html), and the manual says "When the active scene is different from the Sequencer Scene in the current workspace, the 'Render' menu will include separate options to render the Sequencer Scene (and not the active Scene)" (https://docs.blender.org/manual/en/5.1/video_editing/sequencer_scene.html). Both read 2026-07-30. The render operator has a matching `use_sequencer_scene` flag ("Render the sequencer scene instead of the active scene"), but there is no command-line argument for it.

There is also a viewport-versus-final-render difference on the Render Layers page, which is not the same as inside-versus-outside but is worth naming because it is easy to confuse: "The viewport compositor only supports render passes when using EEVEE. For other engines, the passes will be empty."

### Operating system

**LABEL: not in the sources.** Nothing on any of the pages cited for this question mentions an operating system. I searched `manual_5.1` for OS names near these pages and found no qualification on the Render Layers node page, the Scene strip page, the Post Processing page, the Scene properties page or the Sequencer Scene page. **Observed 2026-08-05:** the tests below were run on macOS only. **Observed 2026-08-06:** the same tests were run again on a rented Linux machine and came back the same (see the Linux block at the end of this answer), so this holds on macOS and on Linux. Windows stays untested. *(Changed by the citation check of the Linux hand tests, 2026-08-06: this line still said macOS alone while a Linux result sat below it.)*

### Observed — hand tests 16, 17, 18 and 19 (2026-08-05)

All were run from a terminal on the Creator's own computer — a MacBook Pro with an M1 Pro chip, macOS 26.5.2, Apple's own graphics and no other graphics card — using Blender 5.1.2 at `/Applications/Blender 5.1.2.app/Contents/MacOS/Blender`, with the `BLENDER_USER_RESOURCES` setting pointed at a fresh empty folder. Label: **observed**.

**Hand test 17 — does rendering the first scene render the second, and with whose settings?** A file was saved with two scenes. `Alpha` used Workbench at 64 by 64 with its compositing pipeline on, and its compositing node group held one Render Layers node with Scene set to `Beta`, wired to the group output. `Beta` used Cycles with 8 samples, and its own node group inverted its render. Rendering `Alpha`:

```
blender -b test.blend -S Alpha -o out/c_ -F PNG -f 1
00:00.318  render           | Start rendering: Beta, ViewLayer
00:00.318  render           | Engine: Cycles
00:00.322  render           | Fra: 1 | Mem: 1M | Sample 0/8
00:00.322  render           | Fra: 1 | Remaining: 00:00.00 | Mem: 1M | Sample 1/8
00:00.374  render           | Fra: 1 | Mem: 1M | Sample 8/8
```

Three images built from the same file were then compared — `Alpha`'s output, `Beta` rendered directly with its compositing pipeline on, and `Beta` rendered directly with it off. The first pixel of each:

```
alpha_0001.png    [0.0, 0.0, 0.0, 1.0]
betacomp_0001.png [0.7686, 0.7686, 0.7686, 1.0]
betaraw_0001.png  [0.0, 0.0, 0.0, 1.0]
```

**This closes three of the "not covered" items above.** Rendering the first scene does start a fresh render of the other; it uses that scene's own engine and its own sample count; and the second scene's own compositor does not run — what the Hint implied is what happens.

**Hand test 18 — two scenes pointing at each other.** With `Alpha`'s node pointed at `Beta` and `Beta`'s at `Alpha`, both pipelines on, both Workbench, rendering `Alpha` with a sixty-second cut-off finished in 0.7 seconds, exit code 0, and wrote an image:

```
00:00.333  render           | Start rendering: Beta, ViewLayer
00:00.333  render           | Engine: Workbench
00:00.418  render           | Executing compositor
00:00.418  render           | Fra: 1 | Compositing
Scene switch for render: 'Alpha' in file: '.../circular.blend'
00:00.468  render           | Saved: '.../out/x_0001.png'
```

**This closes another "not covered" item:** two Render Layers nodes pointed at each other produce no error and no hang, and `Beta` is rendered once rather than the chain repeating.

**Hand test 19 — a Scene strip from the command line.** A file was saved with `Alpha` on Workbench, its sequencer pipeline on and one Scene strip referencing `Beta` at frame 1; `Beta` used Cycles with 8 samples. Rendering `Alpha` from the command line printed `Start rendering: Beta, ViewLayer`, `Engine: Cycles` and `Sample 8/8`, and wrote an image.

**Hand test 16 — the Sequencer Scene against the active scene.** A file was saved with three scenes: `Alpha`, `Beta`, and `Edit` whose sequencer held a Scene strip referencing `Alpha`. The workspace's sequencer scene was set to `Edit` and the window's scene to `Beta`. A bare `blender -b test.blend -o out/s_ -F PNG -f 1`, with a handler added at launch that names the scene the render starts on, printed:

```
RENDER_PRE SCENE: Beta
```

**This answers the outside-the-interface gap named above.** With no `-S`, a render started from outside uses the active scene, and the workspace's Sequencer Scene has no effect on it.

### Where I looked (question 8)

- Located the node page by listing `manual_5.1/compositing/types/input/`, then `…/input/scene/`. Read `render_layers.txt` in full, plus `index.txt` for that folder.
- Searched all of `manual_5.1` for `another scene`, `other scene`, `different scene`, `second scene`, `secondary scene`, `multiple scenes`, `from a scene`. Opened every hit that could bear on the question: `compositing/types/input/scene/render_layers.txt`, `video_editing/edit/montage/strips/scene.txt`, `video_editing/sequencer_scene.txt`, `scene_layout/scene/introduction.txt`, `scene_layout/scene/properties.txt`, `render/freestyle/view_layer/freestyle.txt`, `render/output/properties/stereoscopy/usage.txt`, `compositing/introduction.txt`, `compositing/types/utilities/switch_stereo_view.txt`, `editors/compositor.txt`. Ruled out as unrelated: the glTF import page, the drivers page, the markers page, the armatures page, the metaball page, the USD page, the collections page, the storyboarding page, the 3D-cursor page, the line-art modifier page.
- Searched all of `manual_5.1` for `scene strip` and `background scene` / `background set` / `Set Scene`, and read every render-relevant hit. Ruled out `movie_clip/tracking/.../clip.txt` ("Set Scene Frames" is a tracking operator) and `video_editing/edit/montage/strips/movie.txt` ("Set Scene Frame Rate").
- Searched all of `manual_5.1` for `Render Layers` and `render layers node`, and opened `render/layers/passes.txt`, `render/layers/introduction.txt`, `compositing/compositor_system.txt`, `interface/controls/nodes/editing.txt` (the Read View Layers operator), `interface/controls/nodes/node_editors.txt`. Ruled out the Cryptomatte, ID Mask, Vector Blur, Bokeh Blur, Math and Alpha Convert node pages — they mention the node only as an example input.
- Searched `compositing/`, `render/`, `scene_layout/` and `video_editing/` for `recursi`. The only relevant hit is the Background Scene sentence; the Freestyle Python hits are about curve splitting and were ruled out.
- Read in the API: `bpy.types.CompositorNodeRLayers.txt` in full, `bpy.types.SceneStrip.txt` (its description and its `scene`, `scene_camera`, `scene_input` properties), `bpy.types.RenderSettings.txt` (`use_compositing`, `use_sequencer`), `bpy.types.Scene.txt` (`compositing_node_group`, `sequence_editor`, `use_nodes` — the last is marked "Deprecated since version 5.0: removal planned in version 6.0"), `bpy.ops.render.txt` (`use_sequencer_scene`, `scene`). Searched the whole of `api_5.1` for `sequencer_scene`.
- Searched all of `release_notes_5.1` for `render layers`, `scene strip`, `cross-scene`, `other scene`, `another scene`, and read `rendering.md`, `compositor.md` and `sequencer.md` in full.
- Opened one picture: `_images/node-types_CompositorNodeRLayers.webp`.
- I did not need `cycles_5.1` for this question. Cross-scene input is a compositor and sequencer matter, not an engine setting, and nothing in the manual pointed at an engine-side control for it.

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

**Still not settled on either machine.** How frame numbers line up between the two scenes — the test scenes are still on both machines — and whether a chain may go deeper than two.

---

## Things a person with Blender 5.1 open could settle by hand in a few minutes

Everything on this list that a terminal could reach was run on the Creator's Mac on 2026-08-05 and written into the answers above, item 7 was answered by the Creator at Blender's window the same day, and every terminal test was run again on a rented Linux machine on 2026-08-06. What is left:

1. **What the right-click menu on "Use for Rendering" offers.** The dot beside it, and that the switch takes a keyframe, are both settled; the menu itself was not among the shots supplied on the Mac, and driving a right-click on one control is not something the Linux run attempted. (Question 1.)

2. **Whether a `frame_change_pre` handler that sets the sample count takes effect for that same frame.** Not run. A keyframed count is honoured per frame, which is the practical case; a handler is a different route and was not tested. (Question 5.)

3. **Which frame of the second scene is used when it is pulled in.** Not run. It needs a scene whose content changes visibly with the frame, and the tests above used a still scene. (Question 8.)

---

2026-08-04 — Opus 5 (Claude Code) — Settings, Blender 5.1

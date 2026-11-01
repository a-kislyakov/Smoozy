# Settings — Architecture item 24, second pass (Blender 5.1)

Sources consulted, all on **2026-08-29**, all snapshots taken **2026-07-30**:
the Blender 5.1 manual (`Blender Sources/manual_5.1`, including its `_images` folder), the Blender 5.1 Python API reference (`Blender Sources/api_5.1`), the Blender 5.1 release notes (`Blender Sources/release_notes_5.1`, every file in it), and Cycles' own files (`Blender Sources/cycles_5.1`).

Every citation below says "read 2026-08-29" — the day I did the reading. The words quoted are those of the 2026-07-30 snapshots named above, and those snapshots are what anyone checking a quote should open.

I read only the question sheet and files inside Blender Sources/. I did not open any other part of this project.

I used no helper agents. Everything below I read myself.

---

## 17. When a render is started from outside and an instruction handed in at launch sets the resolution percentage, the sample count, or whether a view layer renders, what name does that instruction use for each of those three settings?

### The question was ambiguous, and here is the reading I answered

"An instruction handed in at launch" can be read two ways.

- **Reading A (the one I answered):** an instruction given on the command line that then sets the setting — which, in Blender, means an argument that runs Python. The question presumes the instruction *does* set the three settings, and only this reading supports that. So the name asked for is the name Python uses for each setting.
- **Reading B:** a dedicated command-line switch of Blender's own, like `-E` for the engine. Under this reading the answer is that there is no such switch for any of the three, so there is no name to give. I record the evidence for that below, because it is what forces Reading A.

### Reading B first: no command-line switch for any of the three turned up anywhere I looked

I found no argument for a resolution percentage, a sample count, or a view layer in the manual's list of command-line arguments for 5.1. I read the whole list, and the searches behind that are recorded at the end of this document. The Render Options section lists, in full:

> "-b, --background", "-a, --render-anim", "-S, --scene <name>", "-f, --render-frame <frame>", "-s, --frame-start <frame>", "-e, --frame-end <frame>", "-j, --frame-jump <frames>", "-o, --render-output <path>", "-E, --engine <engine>", "-t, --threads <threads>"

and the Cycles Render Options section lists, in full:

> "--cycles-device <device>"
> "--cycles-print-stats"

Source: manual, https://docs.blender.org/manual/en/5.1/advanced/command_line/arguments.html — read 2026-08-29.

Cycles' own files agree. `Blender Sources/cycles_5.1/engine.py`, function `_configure_argument_parser`, lines 8–21, defines exactly two arguments and no more:

> `parser.add_argument("--cycles-print-stats",`
> `parser.add_argument("--cycles-device",`

Labels for this part, and they are not all the same:

- The two quoted lists are what the manual and the file say. The manual's list is **documented**; the lines from Cycles' file are **observed**.
- That the function defines those two arguments and nothing else is **observed** — it is what those lines of the file say.
- That no such switch exists *anywhere* is not something any source states. It is what reading the whole argument list and searching all four folders turned up, so it is **not in the sources**, on the record of looking at the end of this document.
- That this function is what actually reads Blender's command line when Blender starts is a step beyond the file. The file shows the arguments being defined; it does not show what a running Blender does with them. So I do not claim Cycles' add-on "has" these two launch arguments and no others in any running sense — only that these are the two its file defines.

The argument that *can* carry such an instruction is the Python one:

> "--python-expr <expression>
> Run the given expression as a Python script.
> The expression may be a complete multi-line script; you are limited only by the platform's maximum argument length."

and alongside it "-P, --python <filepath> — Run the given Python script file" and "--python-text <name> — Run the given Python script text block". Same page, same date.

So the names below are the Python property names. That choice — that the name the question asks for is the Python property name — is my own reading of an ambiguous question, not something a source says. Label for the choice itself: **inferred**, and the step is written out above.

### The three names

**A note on the full paths in this section, before the names.** Each property's own name is quoted from a source below. The full dotted path is a second claim, and it is weaker. Only one of these paths is written out by a source: `bpy.data.scenes[0].render.resolution_percentage`, in the reference's own worked example. Every other full path here is joined from two separate quotes — one saying a scene has that thing, one saying that thing has that property. Joining them is a step a reader has to supply, so every full path except that one is **inferred**. The property names themselves keep the labels given beside them.

**1. Resolution percentage → `resolution_percentage`**

It sits on `RenderSettings`, and a scene reaches it through `render`.

> "resolution_percentage
> Percentage scale for render resolution (in [1, 32767], default 100)
> Type:
> int"

Source: Python API reference, https://docs.blender.org/api/5.1/bpy.types.RenderSettings.html — read 2026-08-29.

The path to it, from the same reference, on `Scene`:

> "render
> (readonly, never None)
> Type:
> RenderSettings"

Source: Python API reference, https://docs.blender.org/api/5.1/bpy.types.Scene.html — read 2026-08-29.

And the reference spells the whole path out in its own worked example:

> ">>> bpy.data.scenes[0].render.resolution_percentage
> 100"

Source: Python API reference, https://docs.blender.org/api/5.1/info_quickstart.html — read 2026-08-29.

So the full name an instruction uses is `bpy.data.scenes["…"].render.resolution_percentage`, or `bpy.context.scene.render.resolution_percentage`. The reference uses that second form for render settings elsewhere: "bpy.context.scene.render.image_settings.file_format = 'PNG'" (https://docs.blender.org/api/5.1/info_best_practice.html, read 2026-08-29).

In the interface this same setting is labelled "Percentage" in the Format panel — "Percentage — Slider to reduce or increase the size of the rendered image relative to the Resolution values." (manual, https://docs.blender.org/manual/en/5.1/render/output/properties/format.html, read 2026-08-29). The panel picture `_images/render_output_properties_format_panel.png` on that page shows the field labelled just "%" with the value "100%". That is the interface name, not the name an instruction uses.

Label: **documented**.

**2. Sample count → there is no one name; it depends on the render engine**

- **Cycles:** `samples`, on Cycles' own render settings, reached through `cycles` on the scene — `bpy.context.scene.cycles.samples`.

  From Cycles' own file `Blender Sources/cycles_5.1/properties.py`, inside `class CyclesRenderSettings(bpy.types.PropertyGroup)` (which begins at line 378), at lines 479–484:

  > `    samples: IntProperty(`
  > `        name="Samples",`
  > `        description="Number of samples to render for each pixel",`
  > `        min=1, max=(1 << 24),`
  > `        default=4096,`
  > `    )`

  And the attachment point, same file, lines 1103–1107 *(line range corrected 2026-08-29 by the citation check, which opened the file; the words are unchanged)*:

  > `        bpy.types.Scene.cycles = PointerProperty(`
  > `            name="Cycles Render Settings",`
  > `            description="Cycles render settings",`
  > `            type=cls,`
  > `        )`

  Read 2026-08-29. Label: **observed** (read out of the supplied Cycles file, which is not documentation). The Python API reference has no page for `CyclesRenderSettings`; `Scene` merely points at it — "cycles / Cycles render settings (readonly) / Type: CyclesRenderSettings" (https://docs.blender.org/api/5.1/bpy.types.Scene.html, read 2026-08-29) — so Cycles' own file is the only source that gives the name.

  In the interface the same setting is labelled "Render (Max) Samples — The number of paths to trace per pixel in the final render." (manual, https://docs.blender.org/manual/en/5.1/render/cycles/render_settings/sampling.html, read 2026-08-29). Again, that is the interface name, not the one an instruction uses.

- **EEVEE:** `taa_render_samples`, on `SceneEEVEE`, reached through `eevee` on the scene — `bpy.context.scene.eevee.taa_render_samples`.

  > "taa_render_samples
  > Number of samples per pixel for rendering (in [1, inf], default 64)"

  Source: Python API reference, https://docs.blender.org/api/5.1/bpy.types.SceneEEVEE.html — read 2026-08-29. The path, from `Scene`: "eevee / EEVEE settings for the scene (readonly) / Type: SceneEEVEE" (https://docs.blender.org/api/5.1/bpy.types.Scene.html, read 2026-08-29). Label: **documented**.

- **Per view layer, rather than per scene:** `samples`, on `ViewLayer`.

  > "samples
  > Override number of render samples for this view layer, 0 will use the scene setting (in [0, inf], default 0)"

  Source: Python API reference, https://docs.blender.org/api/5.1/bpy.types.ViewLayer.html — read 2026-08-29. Label: **documented**.

**3. Whether a view layer renders → `use`, on that view layer**

> "use
> Enable or disable rendering of this View Layer (default True)
> Type:
> bool"

Source: Python API reference, https://docs.blender.org/api/5.1/bpy.types.ViewLayer.html — read 2026-08-29.

The view layers of a scene are reached through `view_layers`:

> "view_layers
> (default None, readonly)
> Type:
> ViewLayers[ViewLayer]"

Source: Python API reference, https://docs.blender.org/api/5.1/bpy.types.Scene.html — read 2026-08-29.

So the full name is `bpy.context.scene.view_layers["…"].use`.

In the interface this is the checkbox "Use for Rendering — The active view layer will be used during rendering." (manual, https://docs.blender.org/manual/en/5.1/render/layers/view_layer.html, read 2026-08-29). The panel picture `_images/render_layers_view-layer_panel.png` on that page shows a checkbox labelled "Use for Rendering", ticked, above one labelled "Render Single Layer". Label for what the picture shows: **documented**.

Label: **documented**.

### One adjacent name, so it is not mistaken for the above

The render operator takes an argument that also names a view layer, but it is a different setting:

> "bpy.ops.render.render(*, animation=False, write_still=False, use_viewport=False, use_sequencer_scene=False, layer='', scene='', frame_start=0, frame_end=0)"
> "layer (str) – Render Layer, Single render layer to re-render (used only when animation is disabled) (optional, never None)"

Source: Python API reference, https://docs.blender.org/api/5.1/bpy.ops.render.html — read 2026-08-29. That argument is named `layer` and picks one layer to re-render; it is not the per-layer on/off setting, which is `ViewLayer.use`. Label: **documented**.

### What the 5.1 release notes say about these names

Nothing. I read all twenty-one release-note files for 5.1 in full or searched them in full (list in the record below). The Python API notes for 5.1 do list renames, but none of them touch these three — the renames are of video-sequencer strip properties and of brush properties:

> "Strip (`strip.`) time properties were renamed to improve clarity and make them easier to remember for scripting."

> "A number of `Brush` properties related to how the resulting stroke will behave have been folded into a single `enum` field, `brush.stroke_method`."

Source: release notes, https://developer.blender.org/docs/release_notes/5.1/python_api/ — read 2026-08-29.

The Core notes do add one launch argument, and it is worth recording because it is a launch argument about Python, but it does not name any of the three settings:

> "Use the `--python-use-user-env` command line argument to restore access to user modules."

Source: release notes, https://developer.blender.org/docs/release_notes/5.1/core/ — read 2026-08-29. (Note for whoever checks this: the manual's argument list for 5.1 documents `--python-use-system-env` and does not list `--python-use-user-env`. I am reporting both quotes as they stand and drawing no conclusion.)

Label for this section: **documented** for the quotes; the "no rename touches these three" statement is **not in the sources** in the sense that no source states it — it is what my full-text searches of the release notes turned up, and the searches are listed below.

## What a running Blender 5.1 did — the hand tests of 2026-08-29

**Label: observed.** Nothing in this section comes from a source. It was got by running Blender and watching what it did, and it sits **beside** the answers above rather than over them. Those answers say what the 5.1 sources carry; this says what one machine did on one day. Where an answer above says something is *not in the sources*, that stays true of the sources and is not withdrawn — what follows is a different claim about a different thing.

**What was done, and to what.** Fifteen tests, written out in full before they were run, were run from a terminal on **2026-08-29**. Each ran in all four researched versions; only the Blender 5.1 runs are written here. Every run used the application `/Applications/Blender 5.1.2.app`, which reports itself as **Blender 5.1.2**, build hash `ec6e62d40fa9`, build date 2026-05-19 — read off the application, not taken from any document. Every run used `--factory-startup`, so no add-on, preference or startup file of the Creator's could reach a result, and every run began from Blender's own default scene: a cube, a camera and a light, 1920 by 1080 at 100 per cent, one view layer called `ViewLayer`. Output paths were absolute, into a scratch folder outside this repository. In the listings below the long absolute folder is shortened to `…` and nothing else is changed.

**On what kind of computer.** The Creator's MacBook Pro — an Apple M1 Pro chip, Apple's own graphics with no second card, macOS 26.6.2 (build 25G83, Darwin 25.6.0, arm64), read off the machine on the day. **Nothing was rented, and nothing ran on any other operating system.** Every line below therefore speaks for macOS on Apple silicon and for nothing else.

### The three results that matter more than the names

Together they mean **a builder cannot tell a working instruction from a broken one by anything Blender hands back**.

**1. A wrong name does not stop the render, and does not show in the exit status.** Three names were deliberately misspelled and the frame was rendered anyway:

```
00:00.559  render           | Saved: '…/E9_51_0001.png'
WRONG NAME REFUSED resolution_percent AttributeError 'RenderSettings' object has no attribute 'resolution_percent'
WRONG NAME REFUSED cycles.sample AttributeError 'CyclesRenderSettings' object has no attribute 'sample'
WRONG NAME REFUSED view_layer.used AttributeError 'ViewLayer' object has no attribute 'used'
EXIT STATUS 0
```

Blender wrote the picture after the error and exited with status 0. A wrong name is loud on the terminal and silent everywhere a program would look.

**2. A misspelling is refused loudly everywhere except one place, and that place is Cycles' settings in Blender 4.5.12.** The same sweep, over all five settings groups:

```
REFUSED    s.cycles.sample = 8 | AttributeError 'CyclesRenderSettings' object has no attribute 'sample'
REFUSED    s.cycles.samples_typo = 8 | AttributeError 'CyclesRenderSettings' object has no attribute 'samples_typo'
REFUSED    s.render.resolution_percent = 25 | AttributeError 'RenderSettings' object has no attribute 'resolution_percent'
REFUSED    s.view_layers[0].used = False | AttributeError 'ViewLayer' object has no attribute 'used'
REFUSED    s.display.render_a = 'OFF' | AttributeError 'SceneDisplay' object has no attribute 'render_a'
REFUSED    s.eevee.taa_render_sample = 3 | AttributeError 'SceneEEVEE' object has no attribute 'taa_render_sample'
```

**This version refuses.** Blender 4.5.12 does not: there, and only there, a misspelling on Cycles' settings is accepted with no error at all and the value is gone the moment it is read back. That is a difference between the versions, established by running the same sweep in each.

**3. A correct name handed in *before* the file is loaded prints the right value back and has no effect.** A file was saved by this same version with the resolution percentage at 100, then the same instruction was handed in twice — once after the file on the launch line, once before it:

```
----- make the file -----
SAVED …/E15_51.blend 100
----- instruction AFTER the file -----
00:00.448  render           | Saved: '…/E15_after_51_0001.png'
AFTER 25
----- instruction BEFORE the file -----
00:00.560  render           | Saved: '…/E15_before_51_0001.png'
BEFORE 25
----- sizes -----
E15_after_51_0001.png (480, 270)
E15_before_51_0001.png (1920, 1080)
```

**Both orders printed `25`.** The instruction ran, the name was right, and reading the value straight back gave the value set. **Only one order changed the picture.** With the instruction after the file the written image is 480 by 270; with the same instruction before the file it is 1920 by 1080, because loading the file replaced the whole scene and with it the value just written. So a correct name, correctly set and correctly read back, can still have no effect on the render — and here the readback does not even warn, because at the moment of reading the value really was 25.

**What the three say together.** The exit status is 0 either way. A printed readback is 25 either way. In 4.5.12 even the error is missing. The only thing that separates a working instruction from a broken one is what lands on disk — the size of the picture, the sample count Blender prints while it renders, the channels in the file. Every result below that shows a name works, shows it that way.

### Each name, exercised

Each test below has a control run beside it. The control is what makes the second run mean something.

**`resolution_percentage` changes the picture Blender writes.**

```
----- control, no instruction -----
00:00.566  render           | Saved: '…/E2_ctl_51_0001.png'
----- resolution_percentage = 25 -----
00:00.434  render           | Saved: '…/E2_set_51_0001.png'
READBACK 25
----- pixel sizes -----
E2_ctl_51_0001.png (1920, 1080)
E2_set_51_0001.png (480, 270)
```

1920 by 1080 with no instruction, 480 by 270 with it — a quarter of the width and a quarter of the height.

**`scene.cycles.samples` changes the count Blender says it is working towards, and `preview_samples` is a different setting.**

```
----- control, nothing set -----
00:00.584  render           | Fra: 1 | Remaining: 00:02.66 | Mem: 18M | Sample 0/4096
----- samples=3 preview_samples=200 -----
00:00.305  render           | Fra: 1 | Remaining: 00:00.00 | Mem: 3M | Sample 1/3
00:00.385  render           | Fra: 1 | Mem: 3M | Sample 3/3
READBACK 3 200
```

The control carries `/4096`, the instructed run carries `/3`. Setting `preview_samples` to 200 in the same breath changed nothing about the render.

**`scene.eevee.taa_render_samples` does the same for EEVEE, and `taa_samples` is a different setting.**

```
----- control (BLENDER_EEVEE, nothing set) -----
00:00.901  render           | Fra: 1 | Rendering 64 / 64 samples
----- taa_render_samples=3 taa_samples=200 -----
00:00.461  render           | Fra: 1 | Rendering 1 / 3 samples
00:00.490  render           | Fra: 1 | Rendering 3 / 3 samples
READBACK 3 200
```

`Rendering 64 / 64 samples` with nothing set, `Rendering 3 / 3 samples` with the instruction. The engine's own name is `BLENDER_EEVEE` in this version; the setting's name does not change with it.

**`scene.display.render_aa` is the route to Workbench's anti-aliasing, it takes text rather than a number, and it changes the picture.**

```
----- render_aa='OFF' -----
READBACK 'OFF'
----- render_aa='32' -----
READBACK '32'
----- the number 8 against the text '8' -----
NUMBER REFUSED TypeError bpy_struct: item.attr = val: expected a string enum, not int
TEXT ACCEPTED '8'
----- pixel comparison -----
SIZE (480, 270) (480, 270)
OFF vs 32  MAXDIFF 0.24313727021217346
OFF vs OFF MAXDIFF 0.0
```

Three things. The route from a scene is real. `= 8` is refused and `= '8'` is accepted, so the value is a piece of text. And rendering the same frame with `'OFF'` and with `'32'` gives images differing by up to 0.243 on a nought-to-one channel scale, while rendering twice with `'OFF'` gives images differing by exactly 0.0 — the second number is what makes the first a result rather than noise.

**A view layer's `use` removes that layer from the file Blender writes.** Two view layers, written to a multi-layer EXR, and the channel names read out of both files:

```
----- both layers on -----
READBACK [('ViewLayer', True), ('Second', True)]
channels:
Second.Combined.A
Second.Combined.B
Second.Combined.G
Second.Combined.R
Second.CombinedpixelAspectRatio
ViewLayer.Combined.A
ViewLayer.Combined.B
ViewLayer.Combined.G
ViewLayer.Combined.R
ViewLayer.CombinedpixelAspectRatio
----- Second switched off -----
READBACK [('ViewLayer', True), ('Second', False)]
channels:
ViewLayer.Combined.A
ViewLayer.Combined.B
ViewLayer.Combined.G
ViewLayer.Combined.R
ViewLayer.CombinedpixelAspectRatio
```

With both layers on the file carries `Second.Combined.R`, `.G`, `.B` and `.A`; with the one switched off those four are gone. *(In 5.0, 5.1 and 5.2 the reading tool prints a line ending `CombinedpixelAspectRatio`. That is not a channel — the channel is `…Combined` and `pixelAspectRatio` is the next piece of the file's header sitting against it with nothing between. Nothing about view layers turns on it.)*

**A view layer's own `samples` beats the scene's count.**

```
----- control: scene samples 64, no layer override -----
00:00.437  render           | Fra: 1 | Mem: 3M | Sample 64/64
READBACK scene 64 layer 0
----- scene samples 64, layer override 5 -----
00:00.376  render           | Fra: 1 | Remaining: 00:00.00 | Mem: 3M | Sample 1/5
00:00.460  render           | Fra: 1 | Mem: 3M | Sample 5/5
READBACK scene 64 layer 5 use_layer_samples 'USE'
```

With the scene count at 64 and no override the render finishes at `Sample 64/64`; with the override at 5 it finishes at `Sample 5/5`. So the bare word `samples` names two different settings, and which one is meant depends entirely on what it is written after. Blender reported `use_layer_samples` as `'USE'` without being asked to set it.

**All six names can be handed in together, in one expression, at one launch.** One run, engine Cycles, setting the resolution percentage, the Cycles count, the EEVEE count, the Workbench choice, one layer's override and another layer's on/off:

```
SIX 25 8 9 'FXAA' 5 False
```

Every one read back as the value set, and nothing was printed instead of that line, so nothing was refused. Note that all three engines' settings were reached in a run whose engine was Cycles: the names do not depend on which engine is selected.

### What the running program calls these settings, and what the panel calls them

A listing was read straight out of the running program — every property's name beside the label it carries, its description, its type and its declared default:

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
ENGINE ENUM = ['BLENDER_EEVEE']
ENGINE CLASSES = ['CYCLES', 'HydraRenderEngine']
```

**The name an instruction uses and the label the program shows are different words, for every one of the three settings the question asks about.** `resolution_percentage` is labelled **Resolution Scale**. `taa_render_samples` is labelled **Render Samples**. `render_aa` is labelled **Render Anti-Aliasing**. A view layer's `use` is labelled **Enabled**. The Cycles `samples` is labelled **Samples** — the one case where the label is the name with a capital letter.

**And the panel is a third wording again.** Blender was opened with a real window from the terminal, the Properties editor was turned to three tabs in turn, and the window was photographed with Blender's own screenshot command:

```
WINDOW 1512 862 AREA 532 1321
SHOT OUTPUT …/W13_51_output.png
WINDOW 1512 862 AREA 532 1321
SHOT RENDER …/W13_51_render.png
WINDOW 1512 862 AREA 532 1321
SHOT VIEW_LAYER …/W13_51_view_layer.png
W13_51_output.png (3024, 1724)
W13_51_render.png (3024, 1724)
W13_51_view_layer.png (3024, 1724)
```

What the three pictures show:

- **Output Properties ▸ Format.** Three rows in a column: `Resolution X` reading `1920 px`, `Y` reading `1080 px`, and beneath them a row whose whole label is **`%`**, reading `100%`. The panel gives this setting no other name.
- **Render Properties ▸ Sampling**, with Cycles selected. Two groups, `Viewport` and `Render`. Under `Render` the rows are `Noise Thres…`, **`Max Samples`** reading `4096`, `Min Samples` reading `0`, and `Time Limit`. Under `Viewport` the same `Max Samples` label reads `1024`. So the panel uses the one label **Max Samples** for the two settings an instruction has to call `samples` and `preview_samples`, and tells them apart only by which group they sit in.
- **View Layer Properties ▸ View Layer.** Two checkboxes: **`Use for Rende…`**, ticked, and `Render Single …`, unticked. The panel is too narrow to show either label whole. The first is the setting an instruction calls `use`; the second is `use_single_layer`.

So for the resolution percentage the instruction says `resolution_percentage`, the program's own label says `Resolution Scale`, and the panel says `%`. For the Cycles sample count the instruction says `samples` and the panel says `Max Samples`, a label it shares with a different setting. For the view layer the instruction says `use` and the program's label says `Enabled`.

**Every near neighbour exists here under the same name** — `use_single_layer`, `preview_samples`, `taa_samples`, `use_layer_samples` and `SceneGpencil.aa_samples` are all present, and `render_aa` offers seven choices (`OFF`, `FXAA`, `5`, `8`, `11`, `16`, `32`) and defaults to `'8'`. Where any of the eight findings documents is silent about one of those names, the silence is a gap in what that researcher looked for and **not** a difference between versions.

**On engines this reaches only so far.** The `engine` list came back with one entry, the built-in EEVEE, and the separately registered engines came back as `CYCLES` and `HydraRenderEngine`. Workbench is reachable — the `render_aa` test rendered with it — without appearing in either list. So this listing cannot be used to say how many engines a Blender has.

### No launch switch of Blender's own, checked against the built program

The whole of this version's own help listing was searched for `percent`, `sample`, `resolution`, `view layer`, `view-layer` and `viewlayer`:

```
lines:      498
128:	Do not use native pixel size, for high resolution displays (MacBook 'Retina').
443:	Background mode: Do not systematically build and evaluate ViewLayers' dependency graphs
```

Two hits in 498 lines, and neither is a switch for any of the three: one is `--no-native-pixels`, about how a screen is drawn, the other `--disable-depsgraph-on-file-load`, about what is built when a file is opened. **The built program agrees with the manual page.** `blender -b --help` prints the same listing with one extra line, a banner naming the version and build. Asking the program for its sub-commands with `-c help` lists one sub-command, `extension`, and none of them names any of the three settings.

**And what Blender does with a launch argument it does not know is the opposite of what a wrong name does.** `--render-percentage 50` — the shape a switch for this setting would take — was handed in on the launch line of a file that would otherwise have rendered:

```
EXIT STATUS 1
00:00.302  blend            | Read blend: "…/u_51.blend"
unknown argument, loading as file: --render-percentage
00:00.306  reports          | ERROR Cannot read file "…/--render-percentage": No such file or directory
Blender 5.1.2 (hash ec6e62d40fa9 built 2026-05-19 01:30:33)

Blender quit
--- any picture written? ---
(none)
```

Blender took the unknown argument to be a file name, could not read it, **wrote no picture at all, and exited with status 1**. So an unknown *switch* stops the run and shows in the exit status, while a wrong *name* inside a Python instruction stops nothing and shows in nothing. Those are two completely different behaviours and a builder needs to know which it is looking at.

Cycles was asked the same question. A guessed argument was handed in after `--`, with a documented one as a control that arguments after `--` are read at all:

```
----- guessed -- --cycles-samples 8 -----
00:00.431  render           | Fra: 1 | Remaining: 00:00.95 | Mem: 17M | Sample 304/4096
00:00.444  render           | Fra: 1 | Remaining: 00:01.12 | Mem: 17M | Sample 0/4096
----- control -- --cycles-print-stats (samples set to 2 in Python) -----
lines mentioning "statistics": 7
----- -- --help -----
(no lines above means no Cycles help listing appeared)
```

`-- --cycles-samples 8` changed nothing — the render ran to `/4096`, the default, and Blender neither complained about the argument nor acted on it. **The control is what stops that being an empty result:** `-- --cycles-print-stats` made Cycles print seven blocks of statistics, so arguments after `--` genuinely are read, and `--cycles-samples` is absent rather than unreachable. Asking Cycles for its own help with `-- --help` printed no listing of any kind.

### The near miss: `use_single_layer` does nothing from outside

Two view layers, `use_single_layer` set to `True`, written to a multi-layer EXR:

```
READBACK use_single_layer True [('ViewLayer', True), ('Second', True)]
channels:
Second.Combined.A
Second.Combined.B
Second.Combined.G
Second.Combined.R
Second.CombinedpixelAspectRatio
ViewLayer.Combined.A
ViewLayer.Combined.B
ViewLayer.Combined.G
ViewLayer.Combined.R
ViewLayer.CombinedpixelAspectRatio
```

It was set, it read back as `True`, and **both view layers still came out**. Read that against the `use` test above: same file, same two layers, and only `use` changes what is written. `use` is the name that governs whether a view layer renders when the render is started from outside; `use_single_layer` is the name that looks right and does nothing there.

### What none of this reached

- **Any operating system except macOS.** Every run was on the Creator's Mac. Nothing was rented. Windows and Linux are untouched.
- **Reading a name out of a hovering tooltip, or right-clicking a control for "Copy Full Data Path".** Both need a hand on a mouse and no command stands in for them. What they were wanted for — the name the running program uses and the label the panel shows — is settled above by the listing and the photographs.
- **Any render engine other than Cycles, EEVEE and Workbench.** None is installed on this Mac.

### Where the four versions differed

- **A wrong name on Cycles' settings is refused here and accepted in 4.5.12.** Written out above.
- **`ViewLayer.use` is declared with a default of `True` here**, and `False` in 4.5.12. That is a real difference in Blender, not a difference in how the documentation was written — all four documentation sets say what their own Blender says. **The declared default is not what any real view layer gets either way**: in this version the default scene's `ViewLayer` reads `True`, and a view layer made fresh by `view_layers.new(…)` also reads `True`, and the same was true in 4.5.12 despite its declared `False`.
- **The built-in EEVEE engine is called `BLENDER_EEVEE` here**, and `BLENDER_EEVEE_NEXT` in 4.5.12. The *setting* names do not change with it — `taa_render_samples` is the same word in all four.

### What this settles for the answer above, and what it does not

- **The absence answered *not in the sources* is untouched.** No page states that no such switch exists, and that stays the sources' position. **Observed beside it, as a separate claim:** the built program's own help listing carries no such switch, `blender -b --help` prints the same listing, and asking for sub-commands with `-c help` lists only `extension`. So nothing in the program contradicts the sources.
- The answer is careful that Cycles' file shows arguments being *defined* and not what a running Blender does with them. **Observed beside it:** a guessed `--cycles-samples 8` handed in after `--` changed nothing, while the documented `--cycles-print-stats` made Cycles print seven blocks of statistics — so arguments after `--` really are read, and this one is absent rather than unreachable.
- Every full dotted path here except one is labelled inferred, joined from two quotes. **Observed beside it:** every one of those joined paths was written out in a launch-time instruction and reached its setting.
- The *Not covered* bullet naming `render_aa` stands as a statement about the looking. **Observed beside it:** the name works from a launch instruction in 5.1.2 and changes the picture Workbench writes.
- The answer says the case where a launch instruction hands values to a separate script is not covered either way. **No run touched it**, and it stays uncovered.
- "No rename touches these three" stays where the answer put it — a search result, not a source's sentence. Nothing above bears on it.

---

### Not covered

- This answer gives names only. It does not say whether setting them from a launch instruction takes effect, when it takes effect relative to the render, or what the render then does. Those were not asked.
- It covers Cycles and EEVEE. It does not give a name for any other render engine, because no other engine's sample setting appeared in the sources I searched. **That is a gap in the looking, and one name it missed is `render_aa`.** *(Added 2026-08-29 by the citation check.)* The terms searched below include no Workbench term and no `render_aa`, so nothing here rules that setting out. The 5.1 API documentation carries it: "render_aa / Method of anti-aliasing when rendering final image (default '8')", with the choices `OFF`, `FXAA`, `5`, `8`, `11`, `16`, `32` — https://docs.blender.org/api/5.1/bpy.types.SceneDisplay.html, read 2026-07-30, **documented** — reached from a scene under `display`: "display / Scene display settings for 3D viewport (readonly) / Type: / SceneDisplay" — https://docs.blender.org/api/5.1/bpy.types.Scene.html, read 2026-07-30, **documented**. Whether it counts as a sample count is a judgement this document does not make. Its silence must not be read as 5.1 differing from the other versions.
- "Sample count" has more than one name even inside one engine (scene-level and per-view-layer). I have given both for Cycles-style sampling; I have not checked whether every engine has both.
- The record of looking below lists `argv` among the terms searched, but nothing in this answer rests on that search and its result is not written down. So this answer does not cover the case where a launch instruction hands values to a separate script rather than naming the settings itself. Nothing is claimed about that case either way.
- I did not settle whether an instruction can use the interface labels ("Percentage", "Render Samples", "Use for Rendering") instead of the property names. Nothing in the sources suggests it can, but nothing states it cannot.
- One nearby setting carries an explicit warning that I am reporting but not building on, because it is a different property from the three asked about: `RenderSettings.use_single_layer` — "Only render the active layer. Only affects rendering from the interface, ignored for rendering from command line. (default False)" (https://docs.blender.org/api/5.1/bpy.types.RenderSettings.html, read 2026-08-29), matching the manual's "This option is ignored when rendering from the command line." (https://docs.blender.org/manual/en/5.1/render/layers/view_layer.html, read 2026-08-29). This says nothing about `ViewLayer.use`.

### Outside the interface

The names are the same. Label: **inferred**, and here is the step.

What the sources say outright: the launch argument "--python-expr <expression> — Run the given expression as a Python script" exists, and "-b, --background — Run in background (often used for UI-less rendering)" exists (manual, https://docs.blender.org/manual/en/5.1/advanced/command_line/arguments.html, read 2026-08-29); and the Python API reference gives `resolution_percentage`, `taa_render_samples`, `ViewLayer.samples` and `ViewLayer.use` as properties of data types, with no wording restricting them to a running interface. The step a reader has to supply: that a property named the same way in the reference is named the same way when the expression runs with no window open. The reference does not state that, so this is inferred, not documented.

The one place the sources do draw an inside/outside line near this subject is `use_single_layer`, quoted above, which is explicitly "ignored for rendering from command line". No such wording appears on any of the three settings asked about.

Cycles' `samples` is the same case with the same step, reading from `Blender Sources/cycles_5.1/properties.py` rather than a document: the file registers `bpy.types.Scene.cycles` unconditionally at lines 1100–1106, with nothing in that registration testing for an interface.

**What running Blender 5.1 added, 2026-08-29. Label: observed.** Every run recorded under *What a running Blender 5.1 did* above was made from a terminal with `-b` and no window, and every name in this answer was set and read back that way. So these names are no longer merely *unmarked* as interface-only — they were exercised from outside and they worked. Three things the runs add. **`use_single_layer` really is dead from outside:** set to `True` on the launch line, both view layers still came out in the written file. **Where the instruction sits decides whether it does anything at all:** handed in before the file, a correct name reads back the value set and changes nothing about the render. And **a wrong name costs nothing a program could see:** the frame still renders, the file is still written, and Blender still exits 0. All of that was read off the Creator's Mac, macOS 26.6.2 on Apple silicon, and speaks for no other machine.

### Operating system

The names are the same on every operating system Blender runs on. Label: **inferred**, and here is the step.

The manual's argument list marks operating-system-specific arguments explicitly wherever they exist — "-con, --start-console … (Windows only)", "--no-window-frame — Disable all window decorations (Linux only)", "--qos <level> … (Windows only)", "/? — Print this help text and exit (Windows only)" (https://docs.blender.org/manual/en/5.1/advanced/command_line/arguments.html, read 2026-08-29). The Python arguments carry no such mark. The API reference pages for `RenderSettings`, `SceneEEVEE`, `Scene` and `ViewLayer` carry no operating-system note on any of the properties quoted, and Cycles' `properties.py` guards the `samples` property with no platform test. The step a reader has to supply: that a marking convention which is used where it applies means its absence signals no difference. The sources nowhere state that these names are the same on all operating systems.

**What running Blender 5.1 added, 2026-08-29. Label: observed, and bounded.** Every run recorded above was on **one computer** — the Creator's MacBook Pro, an Apple M1 Pro chip, macOS 26.6.2 (build 25G83, Darwin 25.6.0, arm64). **Nothing was rented, and nothing ran on Windows or on Linux.** So on **macOS on Apple silicon** these names are now exercised rather than reasoned about. On every other operating system nothing has changed: the reading above is still all there is, and **whether these names hold on Windows or on Linux is unknown**. A run on a machine of each is what would settle it, and no such machine was available for this round. Nobody may read the macOS result as covering the others.

---

## Record of where I looked

**Files searched in full text (whole folders, every file):** `Blender Sources/manual_5.1`, `Blender Sources/api_5.1` (1,899 files), `Blender Sources/release_notes_5.1` (all 21 files), `Blender Sources/cycles_5.1`.

**Terms searched, across all four folders:**
`resolution_percentage`, `percentage`, `Resolution %`, `Resolution Percentage`, `samples`, `taa_render_samples`, `max-samples`, `--samples`, `render-samples`, `use_layer_samples`, `view layer`, `view_layer`, `view-layer`, `render-layer`, `--layer`, `use_single_layer`, `Use for Rendering`, `Sample Override`, `render-percentage`, `resolution-percentage`, `--percentage`, `python-expr`, `python_expr`, `argv`, `argparse`, `add_argument`, `--cycles`, `cycles_device`, `command line`, `argument`, `rename`, `Python Tooltips`, `Copy Data Path`, `Copy Full Data Path`, `sub-command`, `subcommand`, `CyclesRenderSettings`.

**Pages I opened and read:**
- Command line: `advanced/command_line/arguments.txt` (whole file, 691 lines), `advanced/command_line/index.txt`, `advanced/command_line/render.txt`, `advanced/command_line/extension_arguments.txt`.
- Output and render settings: `render/output/properties/format.txt`, `render/cycles/render_settings/sampling.txt`, `render/eevee/render_settings/sampling.txt`.
- View layers: `render/layers/view_layer.txt`.
- Interface/scripting background: `getting_started/help.txt`, `editors/preferences/interface.txt`, `interface/controls/buttons/menus.txt`.
- API: `bpy.types.RenderSettings.txt`, `bpy.types.Scene.txt`, `bpy.types.ViewLayer.txt`, `bpy.types.SceneEEVEE.txt`, `bpy.types.Context.txt`, `bpy.context.txt`, `bpy.ops.render.txt`, `info_quickstart.txt`, `info_best_practice.txt`, `genindex-R.txt`, `genindex-T.txt`, `genindex-all.txt`.
- Release notes: `python_api.md`, `core.md`, `rendering.md`, `cycles.md` read in full; all other 5.1 release-note files searched in full text for every term above.
- Cycles' own files: `engine.py` (lines 1–50, the argument parser), `properties.py` (lines 60–70, 375–390, 475–560, 1100–1115, 1479–1500).
- Pictures: `_images/render_output_properties_format_panel.png` and `_images/render_layers_view-layer_panel.png`.

**What I ruled out and why:**
- **A dedicated command-line switch for any of the three.** Ruled out by reading the whole argument list, and by reading the function in Cycles' own file that defines its arguments, which defines only two. Also ruled out by searching for the switch names such a thing would plausibly have (`--samples`, `--render-percentage`, `--resolution-percentage`, `--percentage`, `--layer`, `--view-layer`, `--render-layer`, `--max-samples`) across all four folders: no hits.
- **The extension sub-commands** (`advanced/command_line/extension_arguments.txt`). Read; every sub-command is about installing, listing, updating, removing or building extensions. Nothing about render settings.
- **A Python API page for Cycles' render settings.** Searched `api_5.1` for `CyclesRenderSettings`: it appears only as a type name pointed at from `bpy.types.Scene` and once inside `bpy.types.Object`. There is no page for it, which is why the Cycles sample count had to come from Cycles' own file.
- **Interface labels as the answer.** Found and quoted ("Percentage", "Render (Max) Samples", "Use for Rendering"), but these are what the manual says the interface shows, not what an instruction writes, so I did not offer them as the answer.
- **A rename or removal of any of the three names in 5.1.** Searched every release-note file for `sample`, `resolution`, `percentage`, `view layer`, `view_layer`, `rename`, `argument`, `command line`. The only renames announced are of video-sequencer strip properties and brush properties, both quoted above.

---

## Things a person with Blender open could still settle by hand

**One of the six items that stood here was run from a terminal on 2026-08-29**, and what came back is written up under *What a running Blender 5.1 did* above: Blender's own help agrees there is no switch for these (was item 6), and `blender -b --help` prints the same listing with one extra line naming the version.

The other five all asked for a tooltip to be read or a control right-clicked, which is one job, so they are one item here, beside the two things this Mac cannot reach.

1. **Read the names off a hovering tooltip, or copy them off the controls.** Switch on **Python Tooltips** in Preferences ‣ Interface, then hold the pointer still over the Percentage slider in Output Properties ‣ Format, over the render Samples field in Render Properties ‣ Sampling with Cycles and again with EEVEE, and over the *Use for Rendering* checkbox in View Layer Properties. Or right-click each and choose *Copy Full Data Path* and paste the result. Both need a hand holding a mouse on a control, and no command stands in for either. *What they were wanted for is settled above another way*: the running program's own label and description were read out for every one of these properties, and those are the two pieces of text a tooltip draws. A negative result is a tooltip or a pasted path showing a name other than the ones above.
2. **The same three names on an operating system other than macOS.** Every run above was on one Mac on Apple silicon, so Windows and Linux are untouched. Run the same launch-time instruction on a Windows machine and on a Linux machine and compare what is printed and what is written to disk. A negative result is any difference at all in a name. Needs a machine of each, and none was rented for this round.
3. **A sample-count name for a render engine other than Cycles, EEVEE and Workbench.** No such engine is installed on this Mac. With one installed, open a Python console inside Blender, type `bpy.context.scene.` and see what that engine added, then look through it for a sample count. A negative result — the engine adding nothing that carries one — is itself the answer for that engine.

---

2026-08-29 — Claude Opus 5 (Claude Code) — Settings, Blender 5.1

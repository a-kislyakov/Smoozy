# Settings — Architecture item 24

Blender version researched: 5.0. Question answered: 17 only.

Sources consulted, all on 2026-07-30 (the snapshot date):

- Blender 5.0 manual (`Blender Sources/manual_5.0`), pages under https://docs.blender.org/manual/en/5.0/
- Blender 5.0 Python API documentation (`Blender Sources/api_5.0`), pages under https://docs.blender.org/api/5.0/
- Blender 5.0 release notes (`Blender Sources/release_notes_5.0`), pages under https://developer.blender.org/docs/release_notes/5.0/
- Cycles' own files (`Blender Sources/cycles_5.0`)
- One manual image, `render_layers_view-layer_panel.png`, at https://docs.blender.org/manual/en/5.0/_images/render_layers_view-layer_panel.png — it sits on the View Layer page, https://docs.blender.org/manual/en/5.0/render/layers/view_layer.html

I read only the question sheet and files inside Blender Sources/. I did not open any other part of this project.

I started no helper agents. Everything below I read myself.

---

## 17. When a render is started from outside and an instruction handed in at launch sets the resolution percentage, the sample count, or whether a view layer renders, what name does that instruction use for each of those three settings?

**The question is ambiguous, and I say so before answering.** "An instruction handed in at launch" can mean two different things:

- **Reading A** — a Blender command-line argument, like `-o` or `-E`. What is that argument called for each of the three settings?
- **Reading B** — a Python instruction handed to Blender at launch (Blender's `--python-expr`, `--python` or `--python-text` arguments). What name does that instruction have to use for each of the three settings?

I answer **Reading B** as the one meant, because under Reading A the arguments page lists no such argument for any of the three, which would make the question's premise false. I give the Reading A finding too, in full, immediately below, so nothing is hidden.

### Reading A first: the arguments page lists no command-line argument for any of the three

**Label: documented for the two sections quoted below. Inferred for the rest of that page.**

The manual's Render Options section lists arguments for the frame range, the frame step, the scene, the engine, the output path and the thread count, and its Cycles Render Options section lists two more — and neither section carries an argument for resolution percentage, for sample count, or for switching a view layer on or off. That much is read straight off the words quoted here.

That no argument **anywhere else on that page** sets one of the three is a step beyond the quotes, and it is **inferred**. The step is that I read the whole arguments page and found none; that is my reading, not words I can quote.

The full Render Options section reads:

> "-b, --background … -a, --render-anim … -S, --scene <name> … -f, --render-frame <frame> … -s, --frame-start <frame> … -e, --frame-end <frame> … -j, --frame-jump <frames> … -o, --render-output <path> … -E, --engine <engine> … -t, --threads <threads>"

And the whole of the Cycles Render Options section is:

> "--cycles-device <device>"

> "--cycles-print-stats"

Source: manual, https://docs.blender.org/manual/en/5.0/advanced/command_line/arguments.html — read 2026-07-30.

That the two Cycles arguments are the only ones Cycles adds is also **observed** in Cycles' own files. In `Blender Sources/cycles_5.0/engine.py`, the function `_configure_argument_parser()` (lines 8–21) adds exactly two:

> `parser.add_argument("--cycles-print-stats",`

> `parser.add_argument("--cycles-device",`

Source: `Blender Sources/cycles_5.0/engine.py`, lines 13 and 16 — read 2026-07-30.

So under Reading A there is no name to report from that page, for any of the three.

### Reading B: the names a Python instruction must use

The manual documents the launch arguments that hand Blender a Python instruction:

> "-P, --python <filepath> — Run the given Python script file."

> "--python-text <name> — Run the given Python script text block."

> "--python-expr <expression> — Run the given expression as a Python script."

Source: manual, https://docs.blender.org/manual/en/5.0/advanced/command_line/arguments.html — read 2026-07-30.

Such an instruction names each setting by its Python property name. **Label: inferred.** The step is that those three arguments run Python, and the API documentation's own examples quoted below address a setting by its property name — for instance `bpy.data.scenes[0].render.resolution_percentage`. Nothing quoted here says in so many words that a launch-time Python instruction must use those names. Everything under Reading B rests on that step.

Those names are below.

#### (a) Resolution percentage — `resolution_percentage`

**Label: documented.**

The API documentation lists it on the render settings:

> "resolution_percentage"

> "Percentage scale for render resolution"

> "Type:"

> "int in [1, 32767], default 100"

Source: Python API documentation, https://docs.blender.org/api/5.0/bpy.types.RenderSettings.html — read 2026-07-30.

The settings block it sits in is reached from a scene as `render`:

> "render"

> "Type:"

> "RenderSettings, (readonly, never None)"

Source: Python API documentation, https://docs.blender.org/api/5.0/bpy.types.Scene.html — read 2026-07-30.

The API documentation writes the whole name out in one piece in its own example:

> ">>> bpy.data.scenes[0].render.resolution_percentage"

> "100"

Source: Python API documentation, https://docs.blender.org/api/5.0/info_quickstart.html — read 2026-07-30.

A second example inside the API documentation writes it against the current scene rather than a named one:

> "scale = scene.render.resolution_percentage / 100.0"

Source: Python API documentation, https://docs.blender.org/api/5.0/bpy.types.RenderEngine.html — read 2026-07-30.

The manual's Format page describes an interface control called **Percentage**:

> "Percentage"

> "Slider to reduce or increase the size of the rendered image relative to the Resolution values. This is useful for small test renders that have the same proportions as the final image."

Source: manual, https://docs.blender.org/manual/en/5.0/render/output/properties/format.html — read 2026-07-30.

**That this control and the property `resolution_percentage` are the same setting is inferred**, not stated. The step is that the two descriptions match. The manual never prints the name `resolution_percentage` anywhere — see *Where I looked* below — so no source joins the two.

So: one name, `resolution_percentage`, and the quotes place it on a scene's render settings rather than inside any one engine's own settings block. Whether every engine honours it is not something these quotes say, and is not what was asked.

#### (b) Sample count — a different name per render engine

There is no single name. The name depends on which engine the sample count belongs to.

**Cycles: `samples`, reached through `cycles`.** **Label: documented, and separately observed.**

Documented — the API documentation writes the whole name out as its worked example of a nested property:

> "# Access the number of samples for the Cycles render engine."

> "bpy.context.scene.cycles.samples"

Source: Python API documentation, https://docs.blender.org/api/5.0/info_api_reference.html — read 2026-07-30.

The `cycles` step in that name is itself documented on the scene:

> "cycles"

> "Cycles render settings"

> "Type:"

> "CyclesRenderSettings, (readonly)"

Source: Python API documentation, https://docs.blender.org/api/5.0/bpy.types.Scene.html — read 2026-07-30.

Observed — Cycles' own files define the property under exactly that name, inside the class `CyclesRenderSettings` (declared at line 378):

> `    samples: IntProperty(`

> `        name="Samples",`

> `        description="Number of samples to render for each pixel",`

> `        min=1, max=(1 << 24),`

> `        default=4096,`

> `    )`

Source: `Blender Sources/cycles_5.0/properties.py`, lines 479–484 — read 2026-07-30.

And the same file attaches that whole block to a scene under the name `cycles`:

> `        bpy.types.Scene.cycles = PointerProperty(`

> `            name="Cycles Render Settings",`

> `            description="Cycles render settings",`

> `            type=cls,`

> `        )`

Source: `Blender Sources/cycles_5.0/properties.py`, lines 1103–1107, in the `register` method of `CyclesRenderSettings` — read 2026-07-30. *(Line range corrected 2026-08-29 by the citation check, which opened the file: the five quoted lines run 1103 to 1107. The words are unchanged.)*

Note the separate viewport name, so the two are not confused. It is `preview_samples`:

> `    preview_samples: IntProperty(`

> `        name="Viewport Samples",`

> `        description="Number of samples to render in the viewport, unlimited if 0",`

Source: `Blender Sources/cycles_5.0/properties.py`, lines 485–487 — read 2026-07-30.

The manual's Cycles sampling page carries two interface labels alongside:

> "Viewport (Max) Samples"

> "The number of paths to trace per pixel in the 3D Viewport (when using the Rendered shading mode). Setting this value to zero enables indefinite sampling."

> "Render (Max) Samples"

> "The number of paths to trace per pixel in the final render. A higher number results in a cleaner image at the cost of a longer render time."

Source: manual, https://docs.blender.org/manual/en/5.0/render/cycles/render_settings/sampling.html — read 2026-07-30.

**That "Render (Max) Samples" is the control for `samples`, and "Viewport (Max) Samples" the control for `preview_samples`, is inferred**, not stated. The step is that the descriptions match. The manual does not print either property name.

**EEVEE: `taa_render_samples`, reached through `eevee`.** **Label: documented.**

> "taa_render_samples"

> "Number of samples per pixel for rendering"

> "Type:"

> "int in [1, inf], default 64"

Source: Python API documentation, https://docs.blender.org/api/5.0/bpy.types.SceneEEVEE.html — read 2026-07-30.

Its viewport counterpart on the same page is a different name, `taa_samples`:

> "taa_samples"

> "Number of samples, unlimited if 0"

> "Type:"

> "int in [0, inf], default 16"

Source: Python API documentation, https://docs.blender.org/api/5.0/bpy.types.SceneEEVEE.html — read 2026-07-30.

The `eevee` step is documented on the scene:

> "eevee"

> "EEVEE settings for the scene"

> "Type:"

> "SceneEEVEE, (readonly)"

Source: Python API documentation, https://docs.blender.org/api/5.0/bpy.types.Scene.html — read 2026-07-30.

So the whole EEVEE name, assembled from those two documented pieces, is `scene.eevee.taa_render_samples`. Assembling it is a step the reader has to take, so **that assembled form is inferred**; the two pieces themselves are documented.

**A per-view-layer sample count also exists, under the name `samples`.** **Label: documented** for the API quote directly below; the Cycles material after it carries its own label.

> "samples"

> "Override number of render samples for this view layer, 0 will use the scene setting"

> "Type:"

> "int in [0, inf], default 0"

Source: Python API documentation, https://docs.blender.org/api/5.0/bpy.types.ViewLayer.html — read 2026-07-30.

Cycles' own files carry a nearby setting named `use_layer_samples`, offering three choices. **Label: observed** — it is read out of Cycles' files, not out of any documentation. What the three choices are called and how they are described is all that is read here; **what a render does with any of them is not established by reading the file**, and no claim about that is made.

> `enum_use_layer_samples = (`

> `    ('USE', "Use", "Per render layer number of samples override scene samples"),`

> `    ('BOUNDED', "Bounded", "Bound per render layer number of samples by global samples"),`

> `    ('IGNORE', "Ignore", "Ignore per render layer number of samples"),`

Source: `Blender Sources/cycles_5.0/properties.py`, lines 64–67; the property itself is `use_layer_samples: EnumProperty(` at line 550 — read 2026-07-30.

#### (c) Whether a view layer renders — `use`

**Label: documented.**

> "use"

> "Enable or disable rendering of this View Layer"

> "Type:"

> "boolean, default True"

Source: Python API documentation, https://docs.blender.org/api/5.0/bpy.types.ViewLayer.html — read 2026-07-30.

The view layers of a scene are reached under the name `view_layers`:

> "view_layers"

> "Type:"

> "ViewLayers bpy_prop_collection of ViewLayer, (readonly)"

Source: Python API documentation, https://docs.blender.org/api/5.0/bpy.types.Scene.html — read 2026-07-30.

Both pieces are documented separately, and the sources do not print the whole name in one piece anywhere I searched. **So the assembled form — a view layer reached through a scene's `view_layers`, then `use` — is inferred**, exactly as the assembled EEVEE name above is. The two pieces themselves are documented.

The manual's View Layer page describes an interface control called **Use for Rendering**:

> "Use for Rendering"

> "The active view layer will be used during rendering."

Source: manual, https://docs.blender.org/manual/en/5.0/render/layers/view_layer.html — read 2026-07-30.

**That this control and the property `use` are the same switch is inferred**, not stated. The step is that the two descriptions match. The manual does not print the name `use`.

The manual image beside that text, `_images/render_layers_view-layer_panel.png`, shows a View Layer panel with two checkboxes, labelled "Use for Rendering" (ticked) and "Render Single Layer" (unticked). **Label for what the picture shows: documented** — it is part of the manual. I take nothing further from it.

**One near-neighbour that is not this setting, so it is not mistaken for it.** The render operator has a parameter also concerning a single layer:

> "layer (string, (optional, never None)) – Render Layer, Single render layer to re-render (used only when animation is disabled)"

Source: Python API documentation, https://docs.blender.org/api/5.0/bpy.ops.render.html — read 2026-07-30.

That names one layer to re-render; it is not the on/off switch the question asks about. And the manual says of the neighbouring interface option:

> "Render Single Layer"

> "Only render the active view layer."

> "Note"

> "This option is ignored when rendering from the command line."

Source: manual, https://docs.blender.org/manual/en/5.0/render/layers/view_layer.html — read 2026-07-30.

### Summary of the three names

| Setting | Name the instruction uses | Where it hangs |
|---|---|---|
| Resolution percentage | `resolution_percentage` | on a scene's `render` |
| Sample count, Cycles | `samples` | on a scene's `cycles` |
| Sample count, EEVEE | `taa_render_samples` | on a scene's `eevee` |
| Sample count, one view layer's override | `samples` | on a view layer |
| Whether a view layer renders | `use` | on a view layer |

The first two rows are printed whole in a source. The other three are each two documented pieces, and putting a pair together into one name is **inferred**.

## What a running Blender 5.0 did — the hand tests of 2026-08-29

**Label: observed.** Nothing in this section comes from a source. It was got by running Blender and watching what it did, and it sits **beside** the answers above rather than over them. Those answers say what the 5.0 sources carry; this says what one machine did on one day. Where an answer above says something is *not in the sources*, that stays true of the sources and is not withdrawn — what follows is a different claim about a different thing.

**What was done, and to what.** Fifteen tests, written out in full before they were run, were run from a terminal on **2026-08-29**. Each ran in all four researched versions; only the Blender 5.0 runs are written here. Every run used the application `/Applications/Blender 5.0.1.app`, which reports itself as **Blender 5.0.1**, build hash `a3db93c5b259`, build date 2025-12-16 — read off the application, not taken from any document. Every run used `--factory-startup`, so no add-on, preference or startup file of the Creator's could reach a result, and every run began from Blender's own default scene: a cube, a camera and a light, 1920 by 1080 at 100 per cent, one view layer called `ViewLayer`. Output paths were absolute, into a scratch folder outside this repository. In the listings below the long absolute folder is shortened to `…` and nothing else is changed.

**On what kind of computer.** The Creator's MacBook Pro — an Apple M1 Pro chip, Apple's own graphics with no second card, macOS 26.6.2 (build 25G83, Darwin 25.6.0, arm64), read off the machine on the day. **Nothing was rented, and nothing ran on any other operating system.** Every line below therefore speaks for macOS on Apple silicon and for nothing else.

### The three results that matter more than the names

Together they mean **a builder cannot tell a working instruction from a broken one by anything Blender hands back**.

**1. A wrong name does not stop the render, and does not show in the exit status.** Three names were deliberately misspelled and the frame was rendered anyway:

```
00:00.476  render           | Saved: '…/E9_50_0001.png'
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
SAVED …/E15_50.blend 100
----- instruction AFTER the file -----
00:00.380  render           | Saved: '…/E15_after_50_0001.png'
AFTER 25
----- instruction BEFORE the file -----
00:00.491  render           | Saved: '…/E15_before_50_0001.png'
BEFORE 25
----- sizes -----
E15_after_50_0001.png (480, 270)
E15_before_50_0001.png (1920, 1080)
```

**Both orders printed `25`.** The instruction ran, the name was right, and reading the value straight back gave the value set. **Only one order changed the picture.** With the instruction after the file the written image is 480 by 270; with the same instruction before the file it is 1920 by 1080, because loading the file replaced the whole scene and with it the value just written. So a correct name, correctly set and correctly read back, can still have no effect on the render — and here the readback does not even warn, because at the moment of reading the value really was 25.

**What the three say together.** The exit status is 0 either way. A printed readback is 25 either way. In 4.5.12 even the error is missing. The only thing that separates a working instruction from a broken one is what lands on disk — the size of the picture, the sample count Blender prints while it renders, the channels in the file. Every result below that shows a name works, shows it that way.

### Each name, exercised

Each test below has a control run beside it. The control is what makes the second run mean something.

**`resolution_percentage` changes the picture Blender writes.**

```
----- control, no instruction -----
00:00.495  render           | Saved: '…/E2_ctl_50_0001.png'
----- resolution_percentage = 25 -----
00:00.374  render           | Saved: '…/E2_set_50_0001.png'
READBACK 25
----- pixel sizes -----
E2_ctl_50_0001.png (1920, 1080)
E2_set_50_0001.png (480, 270)
```

1920 by 1080 with no instruction, 480 by 270 with it — a quarter of the width and a quarter of the height.

**`scene.cycles.samples` changes the count Blender says it is working towards, and `preview_samples` is a different setting.**

```
----- control, nothing set -----
00:00.562  render           | Fra: 1 | Remaining: 00:03.00 | Mem: 18M | Sample 0/4096
----- samples=3 preview_samples=200 -----
00:00.257  render           | Fra: 1 | Remaining: 00:00.00 | Mem: 3M | Sample 1/3
00:00.337  render           | Fra: 1 | Mem: 3M | Sample 3/3
READBACK 3 200
```

The control carries `/4096`, the instructed run carries `/3`. Setting `preview_samples` to 200 in the same breath changed nothing about the render.

**`scene.eevee.taa_render_samples` does the same for EEVEE, and `taa_samples` is a different setting.**

```
----- control (BLENDER_EEVEE, nothing set) -----
00:00.833  render           | Fra: 1 | Rendering 64 / 64 samples
----- taa_render_samples=3 taa_samples=200 -----
00:00.381  render           | Fra: 1 | Rendering 1 / 3 samples
00:00.412  render           | Fra: 1 | Rendering 3 / 3 samples
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
00:00.386  render           | Fra: 1 | Mem: 3M | Sample 64/64
READBACK scene 64 layer 0
----- scene samples 64, layer override 5 -----
00:00.256  render           | Fra: 1 | Remaining: 00:00.00 | Mem: 3M | Sample 1/5
00:00.337  render           | Fra: 1 | Mem: 3M | Sample 5/5
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
WINDOW 1512 866 AREA 532 1327
SHOT OUTPUT …/W13_50_output.png
WINDOW 1512 866 AREA 532 1327
SHOT RENDER …/W13_50_render.png
WINDOW 1512 866 AREA 532 1327
SHOT VIEW_LAYER …/W13_50_view_layer.png
W13_50_output.png (3024, 1732)
W13_50_render.png (3024, 1732)
W13_50_view_layer.png (3024, 1732)
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
lines:      466
128:	Do not use native pixel size, for high resolution displays (MacBook 'Retina').
411:	Background mode: Do not systematically build and evaluate ViewLayers' dependency graphs
```

Two hits in 466 lines, and neither is a switch for any of the three: one is `--no-native-pixels`, about how a screen is drawn, the other `--disable-depsgraph-on-file-load`, about what is built when a file is opened. **The built program agrees with the manual page.** `blender -b --help` prints the same listing with one extra line, a banner naming the version and build. Asking the program for its sub-commands with `-c help` lists one sub-command, `extension`, and none of them names any of the three settings.

**And what Blender does with a launch argument it does not know is the opposite of what a wrong name does.** `--render-percentage 50` — the shape a switch for this setting would take — was handed in on the launch line of a file that would otherwise have rendered:

```
EXIT STATUS 1
00:00.249  blend            | Read blend: "…/u_50.blend"
unknown argument, loading as file: --render-percentage
00:00.254  reports          | ERROR Cannot read file "…/--render-percentage": No such file or directory
Blender 5.0.1 (hash a3db93c5b259 built 2025-12-16 01:51:40)

Blender quit
--- any picture written? ---
(none)
```

Blender took the unknown argument to be a file name, could not read it, **wrote no picture at all, and exited with status 1**. So an unknown *switch* stops the run and shows in the exit status, while a wrong *name* inside a Python instruction stops nothing and shows in nothing. Those are two completely different behaviours and a builder needs to know which it is looking at.

Cycles was asked the same question. A guessed argument was handed in after `--`, with a documented one as a control that arguments after `--` are read at all:

```
----- guessed -- --cycles-samples 8 -----
00:00.392  render           | Fra: 1 | Remaining: 00:00.96 | Mem: 17M | Sample 304/4096
00:00.438  render           | Fra: 1 | Remaining: 00:01.54 | Mem: 17M | Sample 0/4096
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

- The answer settles the **names** only and says so. **Beside that, observed:** in Blender 5.0.1 every one of those names takes effect when handed in at launch, and the ordering point the answer could only infer from the manual's warning about argument order holds — an instruction placed before the file reads back correctly and changes nothing.
- The answer joins three interface labels to three property names by nothing but the descriptions matching, and labels each pairing inferred. **The runs do not remove that step, but they narrow it:** the running program reports its own label for each property — `Resolution Scale`, `Samples`, `Enabled` — and a photograph of the panel shows `%`, `Max Samples` and `Use for Rende…`. So there are three wordings in play for each setting, and the answer's caution about joining them was right.
- The *Not covered* bullet about `use_single_layer` stands as a statement about the looking. **Observed beside it:** the name exists in 5.0.1 and does nothing when the render is started from outside.
- The answer notes that Cycles' settings have no page of their own in the API documentation. **Observed beside it:** the running program does carry the class and reports `samples`, `preview_samples` and `use_layer_samples` on it with their labels, descriptions and defaults.
- **Not settled:** whether `render_aa` counts as a sample count, and what any engine other than Cycles, EEVEE and Workbench calls its own.

---

### Not covered

- This answer settles the **names** only. It does not settle whether setting one of these names at launch actually takes effect on the render that follows, nor when in the launch sequence it must be done. The manual's argument-order rules bear on that but I did not chase it, because it is not what was asked.
- It does not cover engines other than Cycles and EEVEE. I found a sample-related setting for the Workbench engine named `render_aa` ("Method of anti-aliasing when rendering final image", https://docs.blender.org/api/5.0/bpy.types.SceneDisplay.html), but it is an anti-aliasing method rather than a sample count, so I did not treat it as an answer.
- It does not cover the Grease Pencil setting `aa_samples`, which the API index lists on `bpy.types.SceneGpencil`. I did not open that page; it is not a render engine's sample count.
- Three of the names are only ever printed in pieces: the assembled EEVEE name `scene.eevee.taa_render_samples`, the view-layer switch reached through a scene's `view_layers` and then `use`, and the per-view-layer `samples` override reached the same way. Each is inferred from two documented pieces, and the sources do not print any of the three whole in one piece anywhere I searched.
- Three times the interface label and the property name are joined by nothing but the descriptions matching: **Percentage** with `resolution_percentage`, **Render (Max) Samples** with `samples`, **Use for Rendering** with `use`. Each of those pairings is inferred. The hand tests below are aimed at all three.
- **This answer never looked for `use_single_layer`.** *(Added 2026-08-29 by the citation check.)* It names the interface control "Render Single Layer" and quotes the manual's note about it, but never gives the Python name, and nothing above searched for one — so that is a gap in the looking, not an absence. The 5.0 API documentation carries it: "use_single_layer / Only render the active layer. Only affects rendering from the interface, ignored for rendering from command line. / Type: / boolean, default False" — Python API documentation, https://docs.blender.org/api/5.0/bpy.types.RenderSettings.html, read 2026-07-30, **documented**.
- Cycles' settings have no page of their own in the Python API documentation: there is no `CyclesRenderSettings` page, so the full list of Cycles' names cannot be looked up there. The two names used above do appear in it — `cycles` on `bpy.types.Scene`, and `bpy.context.scene.cycles.samples` in the API reference's own example — and they rest on those plus Cycles' own files.

### Outside the interface

The API documentation defines all three as properties of scene and view-layer data rather than as interface controls. Blender documents the arguments that hand in a Python instruction — `-P/--python`, `--python-text`, `--python-expr` — in the same command-line arguments page that documents background rendering, and the manual's rendering page says of the command line: "One advantage of using the command line is that we do not need a graphical display (no need for X server on Linux for example) and consequently we can render via a remote shell (typically SSH)" (https://docs.blender.org/manual/en/5.0/advanced/command_line/render.html, read 2026-07-30). **That the same three names apply when Blender is driven from outside is inferred**, not stated: the step I took is that the API documentation defines these as properties of scene and view-layer data and nowhere marks them as interface-only or as behaving differently in background mode.

Two documented differences do exist nearby and are worth stating plainly:

- The neighbouring view-layer option **Render Single Layer** is explicitly different outside the interface: "This option is ignored when rendering from the command line." That is *not* the `use` switch, but it sits beside it in the same panel and is easy to confuse with it.
- Order matters at launch. The manual states: "Arguments are executed in the order they are given" and, of a worked example, "will not render to /tmp because loading the blend-file overwrites the render output that was set" (https://docs.blender.org/manual/en/5.0/advanced/command_line/arguments.html, read 2026-07-30). **Inferred step:** if loading a file overwrites a setting handed in before the load, then a Python instruction using these names must come after the file is loaded to act on that file's scene. The sources say this about the output path, not about these three settings, so this is my extension and not the manual's statement.

**What running Blender 5.0 added, 2026-08-29. Label: observed.** Every run recorded under *What a running Blender 5.0 did* above was made from a terminal with `-b` and no window, and every name in this answer was set and read back that way. So these names are no longer merely *unmarked* as interface-only — they were exercised from outside and they worked. Three things the runs add. **`use_single_layer` really is dead from outside:** set to `True` on the launch line, both view layers still came out in the written file. **Where the instruction sits decides whether it does anything at all:** handed in before the file, a correct name reads back the value set and changes nothing about the render. And **a wrong name costs nothing a program could see:** the frame still renders, the file is still written, and Blender still exits 0. All of that was read off the Creator's Mac, macOS 26.6.2 on Apple silicon, and speaks for no other machine.

### Operating system

The sources do not say that any of these three names differs by operating system, and I found nothing suggesting it. The command-line arguments page does mark certain arguments as tied to one system — "-con, --start-console … (Windows only)", "/?" "(Windows only)", "--debug-ghost … (Linux only)", "--qos <level> … (Windows only)" — and the Python Options section carries no such mark on any of `-P/--python`, `--python-text` or `--python-expr`. **Label: inferred** that the names hold on every system Blender runs on; the step is that Blender's documentation marks system-specific arguments explicitly and did not mark these. The API pages for `RenderSettings`, `SceneEEVEE`, `ViewLayer` and `Scene` make no mention of operating systems at all.

**What running Blender 5.0 added, 2026-08-29. Label: observed, and bounded.** Every run recorded above was on **one computer** — the Creator's MacBook Pro, an Apple M1 Pro chip, macOS 26.6.2 (build 25G83, Darwin 25.6.0, arm64). **Nothing was rented, and nothing ran on Windows or on Linux.** So on **macOS on Apple silicon** these names are now exercised rather than reasoned about. On every other operating system nothing has changed: the reading above is still all there is, and **whether these names hold on Windows or on Linux is unknown**. A run on a machine of each is what would settle it, and no such machine was available for this round. Nobody may read the macOS result as covering the others.

---

## Where I looked

Everything below was searched or read on 2026-07-30, inside `Blender Sources/` only.

**Whole-tree searches** (across `manual_5.0`, `api_5.0`, `release_notes_5.0`, `cycles_5.0` unless noted):

- `resolution_percentage` — hits in `api_5.0/bpy.types.RenderSettings.txt`, `api_5.0/bpy.types.RenderEngine.txt`, `api_5.0/info_quickstart.txt`, the two generated indexes. **No hits anywhere in the manual.**
- `percentage` — in the manual, the Format page only.
- `samples` — in `cycles_5.0/properties.py` (60+ hits, reviewed), in `api_5.0/bpy.types.SceneEEVEE.txt`, in `api_5.0/bpy.types.ViewLayer.txt`. This line lists only where the word turned up as a **property name**. The manual carries the word too, as interface labels — the sampling pages quoted above are the case — and those are not counted here.
- `taa_render_samples`, `taa_samples`, `cycles.samples`, `.taa_render_samples` — the API index and `api_5.0/info_api_reference.txt`.
- `samples (bpy` and `(bpy.types.ViewLayer attribute)` in `api_5.0/genindex-all.txt`, to check nothing was missed.
- `command line`, `command-line` — 37 manual files listed; I opened the four that mattered (see below) and ruled out the rest as unrelated (GPU troubleshooting pages, installation pages, translation-contribution pages, crash/startup pages, image-format page, animation-player page).
- `python-expr` — one hit only, the command-line arguments page. No worked example of a launch-time Python instruction exists anywhere in these sources.
- `argv`, `add_argument`, `ArgumentParser`, `cycles-device`, `cycles-print-stats` — across all of `cycles_5.0`; only `engine.py` matches.
- `CyclesRenderSettings` — across `api_5.0`; appears only as a type name referenced from `bpy.types.Scene.txt`, `bpy.types.Object.txt` and `change_log.txt`. There is no `CyclesRenderSettings` page. This is what tells me the full list of Cycles' names cannot be looked up in the API documentation, though two of them do appear there.
- Across the release notes: `command line`, `command-line`, `argument`, `renamed`, `resolution_percentage`, `taa_render_samples`, `ViewLayer.use`. The searches turned up one piece of command-line news — a table replacing `--debug-cycles`, `--debug-ffmpeg` and `--verbose` with `--log`/`--log-level`, plus a new `--gpu-vsync` — and no rename of any of the three settings and no new argument for them. **This is a record of what the searches returned, not a finding**: no words are quoted here, no release-notes address is given, and nothing above is claimed on the strength of it.

**Pages opened in full:**

- manual: `advanced/command_line/arguments.txt`, `advanced/command_line/render.txt`, `advanced/command_line/index.txt`, `advanced/command_line/extension_arguments.txt`, `render/layers/view_layer.txt`, `render/output/properties/format.txt`, `render/cycles/render_settings/sampling.txt`, `render/eevee/render_settings/sampling.txt`
- API: relevant parts of `bpy.types.RenderSettings.txt`, `bpy.types.SceneEEVEE.txt`, `bpy.types.ViewLayer.txt`, `bpy.types.Scene.txt`, `bpy.types.SceneDisplay.txt`, `bpy.types.RenderEngine.txt`, `bpy.ops.render.txt`, `info_api_reference.txt`, `info_quickstart.txt`
- Cycles' own files: `engine.py` (lines 1–60), `properties.py` (the class list, and lines 64–67, 378, 470–520, 550, 1100–1112, 1479–1560)
- Image: `manual_5.0/_images/render_layers_view-layer_panel.png`

**Ruled out, and why:**

- Blender's sub-command system (`-c, --command`). The manual's command-line contents list only one sub-command, Extensions, and I read its whole page: every one of its sub-commands is about installing, listing, building or validating extensions. Nothing there touches render settings.
- The Cycles argument parser. Read in the source file itself; it accepts two arguments and neither is a sample count.
- Cycles' per-view-layer settings class `CyclesRenderLayerSettings`. Read in full; it holds passes and denoising only, no sample count. The per-layer sample count is Blender's own `ViewLayer.samples`, not a Cycles one.
- Workbench's `render_aa` and Grease Pencil's `aa_samples` — anti-aliasing settings, not the sample count the question asks about.
- I did not need the web and did not use it.

---

## Things a person with Blender 5.0 open could still settle by hand

**Four of the eight items that stood here were run from a terminal on 2026-08-29**, and what came back is written up under *What a running Blender 5.0 did* above: whether any command-line argument exists for the three settings (was item 5), what the guessed Cycles argument `--cycles-samples 8` does (6), whether a launch-time Python instruction reaches the setting and what changes when it is placed before the file instead of after (7), and switching a view layer off from launch (8).

What is left is the four that asked for a control to be right-clicked — one item now, because they are one job — plus the two things this Mac cannot reach.

1. **Read the names off a hovering tooltip, or copy them off the controls.** Switch on **Python Tooltips** in Preferences ‣ Interface, then hold the pointer still over the Percentage slider in Output Properties ‣ Format, over the render Samples field in Render Properties ‣ Sampling with Cycles and again with EEVEE, and over the *Use for Rendering* checkbox in View Layer Properties. Or right-click each and choose *Copy Full Data Path* and paste the result. Both need a hand holding a mouse on a control, and no command stands in for either. *What they were wanted for is settled above another way*: the running program's own label and description were read out for every one of these properties, and those are the two pieces of text a tooltip draws. A negative result is a tooltip or a pasted path showing a name other than the ones above.
2. **The same three names on an operating system other than macOS.** Every run above was on one Mac on Apple silicon, so Windows and Linux are untouched. Run the same launch-time instruction on a Windows machine and on a Linux machine and compare what is printed and what is written to disk. A negative result is any difference at all in a name. Needs a machine of each, and none was rented for this round.
3. **A sample-count name for a render engine other than Cycles, EEVEE and Workbench.** No such engine is installed on this Mac. With one installed, open a Python console inside Blender, type `bpy.context.scene.` and see what that engine added, then look through it for a sample count. A negative result — the engine adding nothing that carries one — is itself the answer for that engine.

---

2026-08-29 — Claude Opus 5 (Claude Code) — Settings, Blender 5.0

# Settings — Architecture item 24, second pass (Blender 4.5)

**Sources consulted, and when.** Blender 4.5 manual (`Blender Sources/manual_4.5`, including its `_images` folder), Blender 4.5 Python API documentation (`Blender Sources/api_4.5`), Blender 4.5 release notes (`Blender Sources/release_notes_4.5`), and the Cycles add-on's own files (`Blender Sources/cycles_4.5`). All four are a snapshot taken on 2026-07-30, which is my date read for everything below. I did the searching and reading on 2026-08-29.

I read only the question sheet and files inside Blender Sources/. I did not open any other part of this project.

Only question 17 is answered, as instructed.

---

## 17. When a render is started from outside and an instruction handed in at launch sets the resolution percentage, the sample count, or whether a view layer renders, what name does that instruction use for each of those three settings?

### The question was ambiguous, and I answered both readings

"An instruction handed in at launch" can mean two things, and I chased both:

- **A switch on the launch line itself** (like `-E` or `-s`). For all three settings, no such switch appears anywhere in these sources. That is finding A below, and the label there is inferred.
- **A piece of Python handed in at launch** (`--python-expr`, `--python`, `--python-text`), which is the only launch-time instruction left that can reach these three settings. Its names are Python property names. That is finding B below, and it is the substance of the answer.

I answered the second reading as the one meant, because the first reading has no answer to give — no such switch appears in these sources. I did not rewrite the question.

### Finding A — the sources show no launch-line switch for any of the three

**Label: inferred.**

The manual's list of launch-line arguments is presented as complete, and none of the three settings appears in it.

The complete "Render Options" group reads:

> "-b, --background", "-a, --render-anim", "-S, --scene <name>", "-f, --render-frame <frame>", "-s, --frame-start <frame>", "-e, --frame-end <frame>", "-j, --frame-jump <frames>", "-o, --render-output <path>", "-E, --engine <engine>", "-t, --threads <threads>"

— Manual, https://docs.blender.org/manual/en/4.5/advanced/command_line/arguments.html, read 2026-07-30.

The Cycles-specific group is only two entries:

> "--cycles-device <device>"
> "--cycles-print-stats"

— same page.

And the manual calls that page the full list:

> "See Command Line Arguments for a full list of arguments (for example to specify which scene to render, the end frame number, etc.)"

— Manual, https://docs.blender.org/manual/en/4.5/advanced/command_line/render.html, read 2026-07-30.

**The step I took:** the page presents itself as the full list of arguments; none of resolution percentage, sample count, or view-layer on/off appears anywhere on it; therefore no launch-line switch sets them. A reader has to accept "full list means full" to get from the quote to the claim, so this is inferred, not documented.

The Cycles side can be checked harder than the manual, because Cycles' own files contain its argument parser, and here are the arguments it defines:

> ```
>     parser.add_argument("--cycles-print-stats",
>                         help="Print rendering statistics to stderr",
>                         action='store_true')
>     parser.add_argument("--cycles-device",
> ```

— Cycles' own files, `Blender Sources/cycles_4.5/engine.py`, in `_configure_argument_parser()`, lines 13–16. Read 2026-07-30. **Label for this piece: observed** — for the two arguments those lines show. That these are the *only* two is a second claim, and the quoted lines cannot carry it, because an excerpt cannot show that no third one follows it. That claim rests on my reading of the whole function, recorded below as lines 1–70. **Label for that: inferred.**

### Finding B — the names a Python instruction uses

A Python instruction can be handed in at launch. The manual documents it:

> "--python-expr <expression>
>
> Run the given expression as a Python script."

— Manual, https://docs.blender.org/manual/en/4.5/advanced/command_line/arguments.html, read 2026-07-30. (The same page also lists `-P, --python <filepath>` — "Run the given Python script file." — and `--python-text <name>`.)

Because it is run as a Python script, the names it uses are the Python property names. Those are, one setting at a time.

**One step runs through all three of them, so it is written out here once.** Each source below gives a property name, and a second source gives the name of the group that property sits in. Joining the two into a whole dotted name is a step the sources do not take — except in one place, where the quickstart writes `bpy.data.scenes[0].render.resolution_percentage` out in full. So each individual name below carries the label stated beside it, and every **whole dotted name except that one is inferred**, by this joining step.

#### 1. Resolution percentage — `resolution_percentage`

**Label: documented.**

> "resolution_percentage
>
> Percentage scale for render resolution
>
> Type:
>
> int in [1, 32767], default 100"

— Python API, https://docs.blender.org/api/4.5/bpy.types.RenderSettings.html, read 2026-07-30.

It hangs off a scene under the name `render`:

> "render
>
> Type:
>
> RenderSettings, (readonly, never None)"

— Python API, https://docs.blender.org/api/4.5/bpy.types.Scene.html, read 2026-07-30.

The API documentation writes the whole name out for exactly this property, as an example of a data path:

> ">>> bpy.data.scenes[0].render.resolution_percentage
> 100"

— Python API, https://docs.blender.org/api/4.5/info_quickstart.html, read 2026-07-30.

So the full name the instruction uses is **`bpy.data.scenes[0].render.resolution_percentage`** — and that whole spelling is the one the quickstart writes out, so it is the one full name here that needs no joining step. The same tail could be hung off whichever scene an instruction reaches, for instance `bpy.context.scene.render.resolution_percentage`, but nothing I read says that second spelling is available when Blender runs without its window; see *Not covered* below.

In Blender's own window this setting is labelled `%` and described as "Percentage". The Format panel picture shows the row labelled `%` reading `100%`, below `Resolution X` and `Y`:

> "Percentage
>
> Slider to reduce or increase the size of the rendered image relative to the Resolution values."

— Manual, https://docs.blender.org/manual/en/4.5/render/output/properties/format.html, and the picture `_images/render_output_properties_format_panel.png` on that page, read 2026-07-30. **Label for the picture: documented** — it shows a `%` field set to `100%`. The manual never gives the Python name; only the API documentation does.

#### 2. Sample count — the name depends on the render engine

There is no one name. Each engine keeps its own, under its own group on the scene.

**Cycles — `samples`. Label: observed** (read out of Cycles' own files, which are not documentation).

> ```
>     samples: IntProperty(
>         name="Samples",
>         description="Number of samples to render for each pixel",
>         min=1, max=(1 << 24),
>         default=4096,
>     )
> ```

— Cycles' own files, `Blender Sources/cycles_4.5/properties.py`, lines 482–487, inside `class CyclesRenderSettings(bpy.types.PropertyGroup)` (which begins at line 376). Read 2026-07-30.

That group is hung on the scene under the name `cycles`:

> ```
>         bpy.types.Scene.cycles = PointerProperty(
>             name="Cycles Render Settings",
>             description="Cycles render settings",
>             type=cls,
>         )
> ```

— same file, lines 1102–1106, in `CyclesRenderSettings.register()`. Read 2026-07-30.

So the full name is **`bpy.data.scenes[0].cycles.samples`**. The API documentation confirms the `cycles` step from the other side:

> "cycles
>
> Cycles render settings
>
> Type:
>
> CyclesRenderSettings, (readonly)"

— Python API, https://docs.blender.org/api/4.5/bpy.types.Scene.html, read 2026-07-30.

The neighbouring name is worth stating so it is not confused with it: `preview_samples`, "Number of samples to render in the viewport, unlimited if 0" (same file, lines 488–495). That one is the viewport, not the render.

**EEVEE — `taa_render_samples`. Label: documented.**

> "taa_render_samples
>
> Number of samples per pixel for rendering
>
> Type:
>
> int in [1, inf], default 64"

— Python API, https://docs.blender.org/api/4.5/bpy.types.SceneEEVEE.html, read 2026-07-30.

Hung on the scene under the name `eevee`:

> "eevee
>
> EEVEE settings for the scene
>
> Type:
>
> SceneEEVEE, (readonly)"

— Python API, https://docs.blender.org/api/4.5/bpy.types.Scene.html, read 2026-07-30.

So: **`bpy.data.scenes[0].eevee.taa_render_samples`**. The neighbour to avoid is `taa_samples`, "Number of samples, unlimited if 0" — same page. Those quoted words do not say what that second one covers. What marks `taa_render_samples` as the render one is its own description, "for rendering".

**Workbench — `render_aa`. Label: documented** for the name, its description and the values it takes. The quoted words say the setting is a method of anti-aliasing used when rendering the final image, and that its values are pieces of text, several of them bare numbers. They do not say those numbers are sample counts, so I am not calling them that. Whether Workbench has a plain sample count under some other name is **not settled here**: the record of looking below names no Workbench page of the manual and no sweep of the rest of `SceneDisplay`, so this is a gap rather than an absence I can show.

> "render_aa
>
> Method of anti-aliasing when rendering final image
> …
> Type:
>
> enum in ['OFF', 'FXAA', '5', '8', '11', '16', '32'], default '8'"

— Python API, https://docs.blender.org/api/4.5/bpy.types.SceneDisplay.html, read 2026-07-30. It hangs on the scene under `display`: "display / Scene display settings for 3D viewport / Type: SceneDisplay, (readonly)" — https://docs.blender.org/api/4.5/bpy.types.Scene.html, read 2026-07-30. So: `bpy.data.scenes[0].display.render_aa`, and its value is a piece of text, not a number.

**One more sample-count name, at a different level. Label: documented.** A view layer carries its own override, named `samples`:

> "samples
>
> Override number of render samples for this view layer, 0 will use the scene setting
>
> Type:
>
> int in [0, inf], default 0"

— Python API, https://docs.blender.org/api/4.5/bpy.types.ViewLayer.html, read 2026-07-30.

I mention it only because it is a second thing called "the sample count" and the names collide; the question asked for names, and this is one.

#### 3. Whether a view layer renders — `use`

**Label: documented.**

> "use
>
> Enable or disable rendering of this View Layer
>
> Type:
>
> boolean, default False"

— Python API, https://docs.blender.org/api/4.5/bpy.types.ViewLayer.html, read 2026-07-30.

View layers hang off a scene under the name `view_layers`:

> "view_layers
>
> Type:
>
> ViewLayers bpy_prop_collection of ViewLayer, (readonly)"

— Python API, https://docs.blender.org/api/4.5/bpy.types.Scene.html, read 2026-07-30.

And a view layer has a `name` — "name / View layer name / Type: string" (https://docs.blender.org/api/4.5/bpy.types.ViewLayer.html), so a named one can be picked out of that collection.

So the full name is **`bpy.data.scenes[0].view_layers["ViewLayer"].use`** — with whatever the layer is actually called in place of `"ViewLayer"`.

In Blender's own window this is the checkbox labelled "Use for Rendering":

> "Use for Rendering
>
> The active view layer will be used during rendering."

— Manual, https://docs.blender.org/manual/en/4.5/render/layers/view_layer.html, read 2026-07-30. The picture on that page, `_images/render_layers_view-layer_panel.png`, shows the View Layer panel with a ticked checkbox labelled "Use for Rendering" and, under it, an unticked "Render Single Layer". **Label for the picture: documented.**

Worth carrying alongside it, because it is on the same panel and is about rendering from outside — the same manual page says of the second checkbox:

> "Render Single Layer
>
> Only render the active view layer."
>
> "Note
>
> This option is ignored when rendering from the command line."

— same page, read 2026-07-30.

#### How the UI name and the Python name are connected

The manual states that Blender itself will tell you the Python name of a button:

> "When Python Tooltips are enabled, a Python expression is displayed for scripting (usually an operator or property)."

— Manual, https://docs.blender.org/manual/en/4.5/getting_started/help.html, read 2026-07-30.

> "Copy Data Path Shift-Ctrl-C
>
> Copies the Python property data path, relative to the data-block. Useful for Python scripting."

— Manual, https://docs.blender.org/manual/en/4.5/interface/controls/buttons/menus.html, read 2026-07-30.

I did not use this; the names above come from the API documentation and Cycles' own files. I record it because it is how somebody at a keyboard could check every name above in a minute.

### Release notes

I searched all nineteen 4.5 release-note files for any change to these three names or to the launch line. Nothing touches them. The only launch-line change recorded anywhere in 4.5 is a debugging one:

> "There's a new `--profile-gpu` command line argument that exports GPU and CPU profiling data"

— Release notes, https://developer.blender.org/docs/release_notes/4.5/eevee/, read 2026-07-30.

The Python API release-note page lists its breaking changes under `gpu`, Video Sequencer, Render Paths, Shaders, Image & Movies and `bpy.props`; none of them renames `resolution_percentage`, `samples`, `taa_render_samples` or `use`. — https://developer.blender.org/docs/release_notes/4.5/python_api/, read 2026-07-30.

## What a running Blender 4.5 did — the hand tests of 2026-08-29

**Label: observed.** Nothing in this section comes from a source. It was got by running Blender and watching what it did, and it sits **beside** the answers above rather than over them. Those answers say what the 4.5 sources carry; this says what one machine did on one day. Where an answer above says something is *not in the sources*, that stays true of the sources and is not withdrawn — what follows is a different claim about a different thing.

**What was done, and to what.** Fifteen tests, written out in full before they were run, were run from a terminal on **2026-08-29**. Each ran in all four researched versions; only the Blender 4.5 runs are written here. Every run used the application `/Applications/Blender 4.5.12.app`, which reports itself as **Blender 4.5.12 LTS**, build hash `84afd5f785f7`, build date 2026-07-21 — read off the application, not taken from any document. Every run used `--factory-startup`, so no add-on, preference or startup file of the Creator's could reach a result, and every run began from Blender's own default scene: a cube, a camera and a light, 1920 by 1080 at 100 per cent, one view layer called `ViewLayer`. Output paths were absolute, into a scratch folder outside this repository. In the listings below the long absolute folder is shortened to `…` and nothing else is changed.

**On what kind of computer.** The Creator's MacBook Pro — an Apple M1 Pro chip, Apple's own graphics with no second card, macOS 26.6.2 (build 25G83, Darwin 25.6.0, arm64), read off the machine on the day. **Nothing was rented, and nothing ran on any other operating system.** Every line below therefore speaks for macOS on Apple silicon and for nothing else.

### The three results that matter more than the names

Together they mean **a builder cannot tell a working instruction from a broken one by anything Blender hands back**.

**1. A wrong name does not stop the render, and does not show in the exit status.** Three names were deliberately misspelled and the frame was rendered anyway:

```
Saved: '…/E9_45_0001.png'
WRONG NAME REFUSED resolution_percent AttributeError 'RenderSettings' object has no attribute 'resolution_percent'
WRONG NAME ACCEPTED cycles.sample
WRONG NAME REFUSED view_layer.used AttributeError 'ViewLayer' object has no attribute 'used'
EXIT STATUS 0
```

Blender wrote the picture after the error and exited with status 0. A wrong name is loud on the terminal and silent everywhere a program would look.

**2. A misspelling is refused loudly everywhere except one place, and that place is Cycles' settings in Blender 4.5.12.** The same sweep, over all five settings groups:

```
ACCEPTED   s.cycles.sample = 8
ACCEPTED   s.cycles.samples_typo = 8
REFUSED    s.render.resolution_percent = 25 | AttributeError 'RenderSettings' object has no attribute 'resolution_percent'
REFUSED    s.view_layers[0].used = False | AttributeError 'ViewLayer' object has no attribute 'used'
REFUSED    s.display.render_a = 'OFF' | AttributeError 'SceneDisplay' object has no attribute 'render_a'
REFUSED    s.eevee.taa_render_sample = 3 | AttributeError 'SceneEEVEE' object has no attribute 'taa_render_sample'
```

**This version is the odd one out, and it is the dangerous one.** In 4.5.12 a misspelling on Cycles' settings is not refused at all — no error of any kind. Every other settings group refuses here, and 5.0.1, 5.1.2 and 5.2.1 refuse on Cycles too.

That result was surprising, so it was run again on its own before being written down, and again a third time inside a wider run. It came back the same every time, and the value is gone the moment it is read back:

```
ACCEPTED, value now 'GONE'
IS sample AN RNA PROPERTY? False
RNA NAMES CONTAINING sample: ['preview_denoising_start_sample', 'samples', 'preview_samples', 'use_sample_subset', 'sample_offset', 'sample_subset_length', 'use_layer_samples', 'adaptive_min_samples', 'preview_adaptive_min_samples', 'guiding_training_samples', 'sample_clamp_direct', 'sample_clamp_indirect']
CLASS <class 'cycles.properties.CyclesRenderSettings'> CyclesRenderSettings
```

So in Blender 4.5.12 a misspelling on the Cycles group is written, no error appears, and nothing is there afterwards. **This is exactly the shape of failure a builder cannot detect**, and it happens here for any misspelling on that one group.

**3. A correct name handed in *before* the file is loaded prints the right value back and has no effect.** A file was saved by this same version with the resolution percentage at 100, then the same instruction was handed in twice — once after the file on the launch line, once before it:

```
----- make the file -----
SAVED …/E15_45.blend 100
----- instruction AFTER the file -----
Saved: '…/E15_after_45_0001.png'
AFTER 25
----- instruction BEFORE the file -----
Saved: '…/E15_before_45_0001.png'
BEFORE 25
----- sizes -----
E15_after_45_0001.png (480, 270)
E15_before_45_0001.png (1920, 1080)
```

**Both orders printed `25`.** The instruction ran, the name was right, and reading the value straight back gave the value set. **Only one order changed the picture.** With the instruction after the file the written image is 480 by 270; with the same instruction before the file it is 1920 by 1080, because loading the file replaced the whole scene and with it the value just written. So a correct name, correctly set and correctly read back, can still have no effect on the render — and here the readback does not even warn, because at the moment of reading the value really was 25.

**What the three say together.** The exit status is 0 either way. A printed readback is 25 either way. In 4.5.12 even the error is missing. The only thing that separates a working instruction from a broken one is what lands on disk — the size of the picture, the sample count Blender prints while it renders, the channels in the file. Every result below that shows a name works, shows it that way.

### Each name, exercised

Each test below has a control run beside it. The control is what makes the second run mean something.

**`resolution_percentage` changes the picture Blender writes.**

```
----- control, no instruction -----
Saved: '…/E2_ctl_45_0001.png'
----- resolution_percentage = 25 -----
Saved: '…/E2_set_45_0001.png'
READBACK 25
----- pixel sizes -----
E2_ctl_45_0001.png (1920, 1080)
E2_set_45_0001.png (480, 270)
```

1920 by 1080 with no instruction, 480 by 270 with it — a quarter of the width and a quarter of the height.

**`scene.cycles.samples` changes the count Blender says it is working towards, and `preview_samples` is a different setting.**

```
----- control, nothing set -----
Fra:1 Mem:10.52M (Peak 11.15M) | Time:00:00.24 | Remaining:00:01.41 | Mem:1.59M, Peak:1.59M | Scene, ViewLayer | Sample 0/4096
----- samples=3 preview_samples=200 -----
Fra:1 Mem:9.88M (Peak 9.88M) | Time:00:00.00 | Remaining:00:00.00 | Mem:1.59M, Peak:1.59M | Scene, ViewLayer | Sample 1/3
Fra:1 Mem:10.52M (Peak 11.15M) | Time:00:00.08 | Mem:1.59M, Peak:1.59M | Scene, ViewLayer | Sample 3/3
READBACK 3 200
```

The control carries `/4096`, the instructed run carries `/3`. Setting `preview_samples` to 200 in the same breath changed nothing about the render.

**`scene.eevee.taa_render_samples` does the same for EEVEE, and `taa_samples` is a different setting.**

```
----- control (BLENDER_EEVEE_NEXT, nothing set) -----
Fra:1 Mem:40.76M (Peak 40.76M) | Time:00:00.54 | Rendering 64 / 64 samples
----- taa_render_samples=3 taa_samples=200 -----
Fra:1 Mem:40.67M (Peak 40.67M) | Time:00:00.12 | Rendering 1 / 3 samples
Fra:1 Mem:40.68M (Peak 40.68M) | Time:00:00.15 | Rendering 3 / 3 samples
READBACK 3 200
```

`Rendering 64 / 64 samples` with nothing set, `Rendering 3 / 3 samples` with the instruction. The engine's own name is `BLENDER_EEVEE_NEXT` in this version; the setting's name does not change with it.

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
ViewLayer.Combined.A
ViewLayer.Combined.B
ViewLayer.Combined.G
ViewLayer.Combined.R
----- Second switched off -----
READBACK [('ViewLayer', True), ('Second', False)]
channels:
ViewLayer.Combined.A
ViewLayer.Combined.B
ViewLayer.Combined.G
ViewLayer.Combined.R
```

With both layers on the file carries `Second.Combined.R`, `.G`, `.B` and `.A`; with the one switched off those four are gone. *(In 5.0, 5.1 and 5.2 the reading tool prints a line ending `CombinedpixelAspectRatio`. That is not a channel — the channel is `…Combined` and `pixelAspectRatio` is the next piece of the file's header sitting against it with nothing between. Nothing about view layers turns on it.)*

**A view layer's own `samples` beats the scene's count.**

```
----- control: scene samples 64, no layer override -----
Fra:1 Mem:10.52M (Peak 11.15M) | Time:00:00.15 | Mem:1.59M, Peak:1.59M | Scene, ViewLayer | Sample 64/64
READBACK scene 64 layer 0
----- scene samples 64, layer override 5 -----
Fra:1 Mem:9.88M (Peak 9.88M) | Time:00:00.00 | Remaining:00:00.00 | Mem:1.59M, Peak:1.59M | Scene, ViewLayer | Sample 1/5
Fra:1 Mem:10.52M (Peak 11.15M) | Time:00:00.08 | Mem:1.59M, Peak:1.59M | Scene, ViewLayer | Sample 5/5
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
PROP ViewLayer.use | label='Enabled' | description='Enable or disable rendering of this View Layer' | type=BOOLEAN default=False
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
ENGINE ENUM = ['BLENDER_EEVEE_NEXT']
ENGINE CLASSES = ['CYCLES', 'HydraRenderEngine']
```

**The name an instruction uses and the label the program shows are different words, for every one of the three settings the question asks about.** `resolution_percentage` is labelled **Resolution Scale**. `taa_render_samples` is labelled **Render Samples**. `render_aa` is labelled **Render Anti-Aliasing**. A view layer's `use` is labelled **Enabled**. The Cycles `samples` is labelled **Samples** — the one case where the label is the name with a capital letter.

**And the panel is a third wording again.** Blender was opened with a real window from the terminal, the Properties editor was turned to three tabs in turn, and the window was photographed with Blender's own screenshot command:

```
WINDOW 1512 866 AREA 522 1103
SHOT OUTPUT …/E13_45_output.png
WINDOW 1512 866 AREA 522 1103
SHOT RENDER …/E13_45_render.png
WINDOW 1512 866 AREA 522 1103
SHOT VIEW_LAYER …/E13_45_view_layer.png
E13_45_output.png (3024, 1732)
E13_45_render.png (3024, 1732)
E13_45_view_layer.png (3024, 1732)
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
lines:      434
128:	Do not use native pixel size, for high resolution displays (MacBook 'Retina').
386:	Background mode: Do not systematically build and evaluate ViewLayers' dependency graphs
```

Two hits in 434 lines, and neither is a switch for any of the three: one is `--no-native-pixels`, about how a screen is drawn, the other `--disable-depsgraph-on-file-load`, about what is built when a file is opened. **The built program agrees with the manual page.** `blender -b --help` prints the same listing with one extra line, a banner naming the version and build. Asking the program for its sub-commands with `-c help` lists one sub-command, `extension`, and none of them names any of the three settings.

**And what Blender does with a launch argument it does not know is the opposite of what a wrong name does.** `--render-percentage 50` — the shape a switch for this setting would take — was handed in on the launch line of a file that would otherwise have rendered:

```
EXIT STATUS 1
unknown argument, loading as file: --render-percentage
Blender 4.5.12 LTS (hash 84afd5f785f7 built 2026-07-21 01:41:35)
Read blend: "…/u_45.blend"
Error: Cannot read file "…/--render-percentage": No such file or directory

Blender quit
--- any picture written? ---
(none)
```

Blender took the unknown argument to be a file name, could not read it, **wrote no picture at all, and exited with status 1**. So an unknown *switch* stops the run and shows in the exit status, while a wrong *name* inside a Python instruction stops nothing and shows in nothing. Those are two completely different behaviours and a builder needs to know which it is looking at.

Cycles was asked the same question. A guessed argument was handed in after `--`, with a documented one as a control that arguments after `--` are read at all:

```
----- guessed -- --cycles-samples 8 -----
Fra:1 Mem:8.76M (Peak 8.76M) | Time:00:00.08 | Remaining:00:01.29 | Mem:0.46M, Peak:0.46M | Scene, ViewLayer | Sample 240/4096
Fra:1 Mem:8.92M (Peak 8.92M) | Time:00:00.12 | Remaining:00:02.01 | Mem:0.46M, Peak:0.46M | Scene, ViewLayer | Sample 0/4096
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
ViewLayer.Combined.A
ViewLayer.Combined.B
ViewLayer.Combined.G
ViewLayer.Combined.R
```

It was set, it read back as `True`, and **both view layers still came out**. Read that against the `use` test above: same file, same two layers, and only `use` changes what is written. `use` is the name that governs whether a view layer renders when the render is started from outside; `use_single_layer` is the name that looks right and does nothing there.

### What none of this reached

- **Any operating system except macOS.** Every run was on the Creator's Mac. Nothing was rented. Windows and Linux are untouched.
- **Reading a name out of a hovering tooltip, or right-clicking a control for "Copy Full Data Path".** Both need a hand on a mouse and no command stands in for them. What they were wanted for — the name the running program uses and the label the panel shows — is settled above by the listing and the photographs.
- **Any render engine other than Cycles, EEVEE and Workbench.** None is installed on this Mac.

### Where the four versions differed

- **A wrong name on Cycles' settings is accepted here and refused in the other three.** Written out above.
- **`ViewLayer.use` is declared with a default of `False` here**, and `True` in 5.0.1, 5.1.2 and 5.2.1. That is a real difference in Blender, not a difference in how the documentation was written — all four documentation sets say what their own Blender says. **But the declared default is not what any real view layer gets**: in this version the default scene's `ViewLayer` reads `True`, and a view layer made fresh by `view_layers.new(…)` also reads `True`.
- **The built-in EEVEE engine is called `BLENDER_EEVEE_NEXT` here**, and `BLENDER_EEVEE` in the other three. The *setting* names do not change with it — `taa_render_samples` is the same word in all four.

### What this settles for the answer above, and what it does not

- The answer's *Not covered* line — that whether setting these names at launch takes effect is not settled here — **now has an observed answer beside it**: in Blender 4.5.12 every one of the names takes effect, except where the instruction is handed in before the file, in which case it reads back correctly and changes nothing. The manual's line about arguments running in the order they are given, which this answer would not extend into a claim, holds.
- The answer says nothing here shows the Cycles name `samples` exists when Cycles is not loaded. **Observed beside it:** at `--factory-startup`, with nothing of the Creator's loaded, `Scene.cycles` was present and `scene.cycles.samples` read back 4096 in every run.
- The answer leaves open whether these names can be reached through `bpy.context.scene` as well as `bpy.data.scenes[...]` when Blender runs without its window. **Observed beside it:** every run above ran with `-b` and no window and used `bpy.context.scene`, and every name reached its setting that way.
- The answer reads the Format panel picture as showing a row labelled `%` at `100%`. A photograph of a running 4.5.12 shows the same three rows.
- **Still not in the sources, and not touched by any run:** a name for a whole scene's sample count belonging to no engine. The runs asked the running program about named settings; they did not sweep it for an engine-neutral one, so that answer stands as the sources left it.

---

### Not covered

- The names only. Whether setting them at launch actually takes effect, when it takes effect relative to loading the file and to rendering, and what happens with a value out of range — none of that is settled here. (The arguments page's own warning that "Arguments are executed in the order they are given" is a hint that order matters, but I am not extending it into a claim.)
- Nothing here says the Cycles name `samples` exists when Cycles is not loaded. Cycles' own files show the name is created at the moment the add-on registers itself (`bpy.types.Scene.cycles = PointerProperty(...)` inside `register()`). What happens if that has not run, the sources do not say.
- Engines beyond Cycles, EEVEE and Workbench. Any engine added to Blender may name its sample count anything; nothing in these sources constrains that.
- A name for setting a *whole scene's* sample count in a way that does not belong to one engine. **Label: not in the sources.** The sweeps for `--samples`, `--sample-count`, `sample` and `samples` across all four sources are recorded below, and what they turned up was a separate name per engine, never a shared one.
- The view-layer answer covers `use`, the on/off for the layer itself. Other things that stop something rendering — collections excluded from a layer, objects hidden in renders — are different settings with different names and were not asked about. *Render Single Layer*, the neighbouring checkbox quoted above, is another one of them, and its Python name is not given here. **That name is `use_single_layer`, and it is in the 4.5 sources.** *(Added 2026-08-29 by the citation check, which opened the page.)* "use_single_layer / Only render the active layer. Only affects rendering from the interface, ignored for rendering from command line. / Type: / boolean, default False" — Python API documentation, https://docs.blender.org/api/4.5/bpy.types.RenderSettings.html, read 2026-07-30, **documented**. This pass did not search for it, so its absence above was a gap in the looking; it must not be read as 4.5 lacking the name.
- Whether these names can be reached through `bpy.context.scene` as well as `bpy.data.scenes[...]` when Blender is run without its window: the API documentation shows both spellings in general, but nothing I read states which is available in that situation.

### Outside the interface

This question is *about* outside the interface, so the answer already holds there — it describes the Python names an instruction handed in at launch would use. Two claims sit in that sentence and they take different labels. **Label: documented** for the fact that Python can be handed in at launch — `--python-expr`, `-P/--python` and `--python-text` are all quoted from the arguments page above. **Label: inferred** for the names being the same whether Blender has a window or not, and here is the step: the API pages describe these properties without mentioning a window at all, so I take the names not to depend on one. No source says it outright. This is the same shape of step as the operating-system line below, and it gets the same label.

Two differences the sources do state, both worth having:

- The manual says of the neighbouring checkbox on the very same panel: "This option is ignored when rendering from the command line." That is about *Render Single Layer*, not about `use`. Nothing similar is said about "Use for Rendering". The sources do not say whether `use` behaves differently from the command line; I am not going to guess.
- The launch line has no switch for any of the three, so from outside the interface Python is the only route the sources show. That is finding A.

Where it does *not* hold: nothing here tells you anything about setting these three settings from outside by any means other than a Python instruction — there is no other means in these sources.

**What running Blender 4.5 added, 2026-08-29. Label: observed.** Every run recorded under *What a running Blender 4.5 did* above was made from a terminal with `-b` and no window, and every name in this answer was set and read back that way. So these names are no longer merely *unmarked* as interface-only — they were exercised from outside and they worked. Three things the runs add. **`use_single_layer` really is dead from outside:** set to `True` on the launch line, both view layers still came out in the written file. **Where the instruction sits decides whether it does anything at all:** handed in before the file, a correct name reads back the value set and changes nothing about the render. And **a wrong name costs nothing a program could see:** the frame still renders, the file is still written, and Blender still exits 0. All of that was read off the Creator's Mac, macOS 26.6.2 on Apple silicon, and speaks for no other machine.

### Operating system

The sources do not say any of these names differ by operating system. **Label: inferred**, and here is the step: the arguments page marks its OS-specific entries explicitly — "-con, --start-console … (Windows only)", "/?  … (Windows only)", "-r, --register … (Windows & Linux only)", "--debug-ghost … (Linux only)" — so the page does flag OS limits where they exist, and it flags none on the Python options. The API documentation pages for `RenderSettings`, `SceneEEVEE`, `SceneDisplay`, `Scene` and `ViewLayer` carry no OS note at all on the properties quoted. Cycles' `properties.py` defines `samples` with no platform condition around it. From that I take it the names are the same everywhere, but no source states it outright.

**What running Blender 4.5 added, 2026-08-29. Label: observed, and bounded.** Every run recorded above was on **one computer** — the Creator's MacBook Pro, an Apple M1 Pro chip, macOS 26.6.2 (build 25G83, Darwin 25.6.0, arm64). **Nothing was rented, and nothing ran on Windows or on Linux.** So on **macOS on Apple silicon** these names are now exercised rather than reasoned about. On every other operating system nothing has changed: the reading above is still all there is, and **whether these names hold on Windows or on Linux is unknown**. A run on a machine of each is what would settle it, and no such machine was available for this round. Nobody may read the macOS result as covering the others.

---

## Where I looked

**Files and folders searched.** The whole of `Blender Sources/manual_4.5` (including `_images`), the whole of `Blender Sources/api_4.5` (1851 files), all nineteen files of `Blender Sources/release_notes_4.5`, and all of `Blender Sources/cycles_4.5` (`__init__.py`, `camera.py`, `engine.py`, `operators.py`, `osl.py`, `presets.py`, `properties.py`, `ui.py`, `version_update.py`, and the `shader` and `source` folders).

**Terms searched, across manual + API + release notes + Cycles unless noted.** `resolution_percentage` · `render_percentage` · `render-percentage` · `--percentage` · `percentage` · `percent` · `resolution-scale` · `render-scale` · `--samples` · `--sample-count` · `--render-samples` · `sample` · `samples` · `taa_render_samples` · `render_aa` · `--view-layer` · `--render-layer` · `--layer` · `--use-layer` · `enable-layer` · `Use for Rendering` · `Enable or disable rendering of this` · `view_layers[` · `scene.render.` · `cycles.samples` · `python-expr` · `python_expr` · `command line` / `command-line` / `commandline` / `argument` (release notes) · `renamed` / `rename` (release notes) · `Python Tooltips` · `Copy Data Path` · `Copy Full Data Path` · `argv` / `add_argument` / `argparse` / `cycles-device` / `cycles-print-stats` / `resumable` (Cycles) · `Resolution %` · `Max Samples` · `Resolution X` · `--command` / `-c help` / `Sub-Command`.

**Pages opened in full or in part.**
- Manual: `advanced/command_line/arguments` (all 628 lines), `advanced/command_line/render`, `advanced/command_line/index`, `advanced/command_line/extension_arguments` (the sub-command list), `render/output/properties/format`, `render/layers/view_layer`, `render/cycles/render_settings/sampling`, `render/eevee/render_settings/sampling`, `getting_started/help`, `interface/controls/buttons/menus`.
- Manual pictures: `_images/render_output_properties_format_panel.png`, `_images/render_layers_view-layer_panel.png`.
- API: `bpy.types.RenderSettings`, `bpy.types.Scene`, `bpy.types.ViewLayer`, `bpy.types.ViewLayers`, `bpy.types.SceneEEVEE`, `bpy.types.SceneDisplay`, `info_quickstart`, plus the index files `genindex-all` and `genindex-R` to confirm which class each name belongs to.
- Release notes: `python_api.md` in full, `eevee.md` (the `--profile-gpu` line), `core.md`, and grep sweeps over all nineteen files.
- Cycles: `engine.py` (the argument parser, lines 1–70), `properties.py` (the `samples` and `preview_samples` definitions, `class CyclesRenderSettings` and its `register()`, `class CyclesRenderLayerSettings` and its `register()`).

**Ruled out, and why.**
- **A launch-line switch for the resolution percentage, the sample count, or a view layer's on/off.** Ruled out. The arguments page is the full list, I read every line of it, and searched it separately for `percent`, `sample`, `resolution-scale`, `render-scale`, `--use-layer`, `enable-layer` — no hits at all.
- **A Cycles-specific launch-line switch for the sample count.** Ruled out on stronger ground than the manual: Cycles' own argument parser in `engine.py` defines exactly two arguments and neither is a sample count.
- **The `--command` sub-commands as a route.** Ruled out. The only sub-commands documented are the extension ones — `list`, `sync`, `update`, `install`, `install-file`, `remove`, `repo-list`, `repo-add`, `repo-remove`, `build`, `validate`, `server-generate`. None renders or sets scene settings.
- **The manual as a source of the Python names.** Ruled out as a source: searching the whole manual for `resolution_percentage`, `cycles.samples`, `taa_render_samples`, `view_layers[` and `scene.render.` returns nothing. The manual gives interface labels only. Every Python name above therefore comes from the API documentation or from Cycles' own files.
- **A rename in 4.5.** Ruled out. The release notes' Python API page lists its breaking changes by area and none concerns these names; sweeping all nineteen files for `rename`/`renamed` turns up only the Video Sequencer keymaps and a bug-fix line about batch-renaming nodes.
- **The wider web.** Not used. I did not need it.

---

## Things a person could still settle by hand

**Six of the seven items that stood here were run from a terminal on 2026-08-29**, and what came back is written up under *What a running Blender 4.5 did* above: whether any launch-line switch exists and what Blender does with one it does not know (was item 2), whether a launch-time Python instruction reaches all three names (3), whether the Cycles name exists with nothing of the Creator's loaded (4), that `use` is per layer and reachable by name (5), that `render_aa` takes the text `'8'` and refuses the number `8` (6, in part), and which way of naming the scene works when Blender has no window (7).

What is left needs a hand on a mouse, another operating system, or an engine that is not installed.

1. **Read the names off a hovering tooltip, or copy them off the controls.** Switch on **Python Tooltips** in Preferences ‣ Interface, then hold the pointer still over the Percentage slider in Output Properties ‣ Format, over the render Samples field in Render Properties ‣ Sampling with Cycles and again with EEVEE, and over the *Use for Rendering* checkbox in View Layer Properties. Or right-click each and choose *Copy Full Data Path* and paste the result. Both need a hand holding a mouse on a control, and no command stands in for either. *What they were wanted for is settled above another way*: the running program's own label and description were read out for every one of these properties, and those are the two pieces of text a tooltip draws. A negative result is a tooltip or a pasted path showing a name other than the ones above.
2. **Read every control in Workbench's own render panel.** With Workbench as the engine, write down each control's label and each tooltip expression, so it is on record whether Workbench offers a plain sample count under some other name. This is the half of item 6 that was not run: the photographs above cover Output Properties, Render Properties with Cycles, and View Layer Properties, and nobody photographed Workbench's own panel in 4.5.12. Reading a tooltip needs a mouse in any case. A negative result is a plain number-of-samples control appearing there.
3. **The same three names on an operating system other than macOS.** Every run above was on one Mac on Apple silicon, so Windows and Linux are untouched. Run the same launch-time instruction on a Windows machine and on a Linux machine and compare what is printed and what is written to disk. A negative result is any difference at all in a name. Needs a machine of each, and none was rented for this round.
4. **A sample-count name for a render engine other than Cycles, EEVEE and Workbench.** No such engine is installed on this Mac. With one installed, open a Python console inside Blender, type `bpy.context.scene.` and see what that engine added, then look through it for a sample count. A negative result — the engine adding nothing that carries one — is itself the answer for that engine.

---

2026-08-29 — Claude Opus 5 (Claude Code) — Settings, Blender 4.5

I used no helper agents. Every search and every file read above was done by me, under the containment rules in my brief.

# Settings — Architecture item 24 (Blender 5.2)

This document answers question 17 of the Settings question sheet, for Blender 5.2, and no other question on that sheet.

Sources consulted, all read on **2026-07-30** (the snapshot date of the copies I was given):

- Blender 5.2 manual — https://docs.blender.org/manual/en/5.2/ (including one image from the manual's `_images` folder)
- Blender 5.2 Python API documentation — https://docs.blender.org/api/5.2/
- Blender 5.2 release notes — https://developer.blender.org/docs/release_notes/5.2/
- Cycles' own add-on files, as supplied in `Blender Sources/cycles_5.2`

I read only the question sheet and files inside Blender Sources/. I did not open any other part of this project.

---

## 17. When a render is started from outside and an instruction handed in at launch sets the resolution percentage, the sample count, or whether a view layer renders, what name does that instruction use for each of those three settings?

### A note on the reading

The question is slightly ambiguous, so I say plainly which reading I answered.

**The reading I answered:** an "instruction handed in at launch" is something given on the command line when Blender is started. I checked first whether Blender has a launch switch of its own for any of these three.

**The manual's published list of launch arguments carries no switch for a resolution percentage, for a sample count, or for turning a view layer on or off. LABEL: documented** — that page was read end to end and searched, and the record of looking below holds what was read and what was searched for.

**That there is therefore no such switch at all is a step past that page. LABEL: inferred** — the step a reader must supply is that the page is the complete list, which is what the manual's other command-line pages call it.

So the instruction left that can be handed in at launch and reach these three is a Python script or expression, which Blender accepts through `-P` / `--python`, `--python-text`, or `--python-expr`. The names asked for are then the names that Python instruction uses, and those are the property names in the Python API. That is what I report below.

**The other reading:** if the question means "which dedicated command-line switch names these settings", the answer is that the argument list names none of the three — on the same two labels as above, documented for what the list carries and inferred for the conclusion drawn from it. See the record of looking below.

---

### Part A — the resolution percentage

**Name: `resolution_percentage`**, on the render settings of a scene. Written out in full from a scene, it is `scene.render.resolution_percentage`.

**LABEL: documented**

Exact words, Python API documentation, page https://docs.blender.org/api/5.2/bpy.types.RenderSettings.html — under `class bpy.types.RenderSettings(bpy_struct)`, described as "Rendering settings for a Scene data-block":

> resolution_percentage
>
> Percentage scale for render resolution (in [1, 32767], default 100)
>
> Type:
>
> int

Read 2026-07-30.

That the settings live on `render` of a scene, exact words, Python API documentation, page https://docs.blender.org/api/5.2/bpy.types.Scene.html:

> render
>
> (readonly, never None)
>
> Type:
>
> RenderSettings

Read 2026-07-30.

The full written-out name also appears as a worked example in the API documentation's quick-start page, https://docs.blender.org/api/5.2/info_quickstart.html:

> Example of a data path that can be quickly found via the console:
>
> \>>> bpy.data.scenes[0].render.resolution_percentage
>
> 100

Read 2026-07-30.

And in the example render engine on https://docs.blender.org/api/5.2/bpy.types.RenderEngine.html:

> scale = scene.render.resolution_percentage / 100.0

Read 2026-07-30.

This is the only property of that name in the whole API documentation. The API documentation's own index lists exactly one entry, on https://docs.blender.org/api/5.2/genindex-all.html:

> resolution_percentage (bpy.types.RenderSettings attribute)

Read 2026-07-30.

The setting a person sees is called "Percentage", and in the panel picture it is labelled just `%`. Manual, https://docs.blender.org/manual/en/5.2/render/output/properties/format.html:

> Percentage
>
> Slider to reduce or increase the size of the rendered image relative to the Resolution values.
> This is useful for small test renders that have the same proportions as the final image.

Read 2026-07-30. The picture on that same page (`_images/render_output_properties_format_panel.png`, part of the manual) shows a Format panel with rows "Resolution X" 1920 px, "Y" 1080 px, and below them a row labelled "%" reading 100%. **documented** from the picture. Read 2026-07-30.

The manual nowhere gives the Python name; only the API documentation does. Connecting the manual's "Percentage" to the API's `resolution_percentage` is a step a reader has to take, so treat that connection as **inferred**; the name itself is documented.

---

### Part B — the sample count

There is no single name. The name depends on which render engine the sample count belongs to, and there is a fourth name for a per-view-layer override.

**B1. Cycles — name: `samples`**, on the Cycles settings of a scene. Written out, `scene.cycles.samples`.

**LABEL: observed** (from Cycles' own files, not from documentation — there is no page for Cycles' render settings in the 5.2 API documentation; see the record of looking)

Exact lines, file `Blender Sources/cycles_5.2/properties.py`, lines 485–490, inside `class CyclesRenderSettings(bpy.types.PropertyGroup)` which begins at line 384:

>     samples: IntProperty(
>         name="Samples",
>         description="Number of samples to render for each pixel",
>         min=1, max=(1 << 24),
>         default=4096,
>     )

That this group is reached as `cycles` on a scene, exact lines, same file, lines 1140–1146:

>     @classmethod
>     def register(cls):
>         bpy.types.Scene.cycles = PointerProperty(
>             name="Cycles Render Settings",
>             description="Cycles render settings",
>             type=cls,
>         )

Read 2026-07-30.

The 5.2 API documentation does confirm that a scene carries a `cycles` attribute of that type, https://docs.blender.org/api/5.2/bpy.types.Scene.html:

> cycles
>
> Cycles render settings (readonly)
>
> Type:
>
> CyclesRenderSettings | None

Read 2026-07-30. (There is no `bpy.types.CyclesRenderSettings` page in the 5.2 API documentation, so the member name `samples` comes only from the file above.)

**One step in the written-out name is a reader's, and it takes its own label. LABEL: inferred** — the file declares `samples` inside a class it calls `CyclesRenderSettings`, and the API page names a scene attribute `cycles` whose type it calls `CyclesRenderSettings`. Joining those two into `scene.cycles.samples` is a step: the reader has to supply that the class in the file is the type the page names, on the strength of the names matching. This is the same kind of step as the manual-to-API join in Part A, and it carries the same label.

**B2. EEVEE — name: `taa_render_samples`**, on the EEVEE settings of a scene. Written out, `scene.eevee.taa_render_samples`.

**LABEL: documented**

Exact words, Python API documentation, https://docs.blender.org/api/5.2/bpy.types.SceneEEVEE.html:

> taa_render_samples
>
> Number of samples per pixel for rendering (in [1, inf], default 64)
>
> Type:
>
> int

Read 2026-07-30. The scene attribute it hangs off, https://docs.blender.org/api/5.2/bpy.types.Scene.html:

> eevee
>
> EEVEE settings for the scene (readonly)
>
> Type:
>
> SceneEEVEE | None

Read 2026-07-30.

**B3. A render-time anti-aliasing setting — name: `render_aa`**, on `bpy.types.SceneDisplay`, and it is not a plain number.

**LABEL: documented** — for the name, the page it sits on, and the words quoted below, and for nothing beyond them.

**Two things here are the reader's step, not the page's words, and each takes its own label. LABEL: inferred** for both.

- **That this is the Workbench engine's sample count.** The quoted words name no render engine at all. A reader has to supply which engine `bpy.types.SceneDisplay` belongs to, and nothing quoted in this document supplies it.
- **How `SceneDisplay` is reached from a scene.** No quote in this document carries a scene attribute for it, and the record of looking below does not cover a search for one. So the written-out name from a scene is not established here, and I give none.

Exact words, Python API documentation, https://docs.blender.org/api/5.2/bpy.types.SceneDisplay.html:

> render_aa
>
> Method of anti-aliasing when rendering final image (default '8')
>
> OFF
> No Anti-Aliasing – Scene will be rendering without any anti-aliasing.
>
> FXAA
> Single Pass Anti-Aliasing – Scene will be rendered using a single pass anti-aliasing method (FXAA).
>
> 5
> 5 Samples – Scene will be rendered using 5 anti-aliasing samples.
>
> 8
> 8 Samples – Scene will be rendered using 8 anti-aliasing samples.

Read 2026-07-30.

**B4. A per-view-layer override — name: `samples`**, on a view layer itself.

**LABEL: documented**

Exact words, Python API documentation, https://docs.blender.org/api/5.2/bpy.types.ViewLayer.html:

> samples
>
> Override number of render samples for this view layer, 0 will use the scene setting (in [0, inf], default 0)
>
> Type:
>
> int

Read 2026-07-30.

Note that the same word, `samples`, therefore names two different things — the Cycles scene sample count and the view-layer override — and they are told apart only by what they hang off.

---

### Part C — whether a view layer renders

**Name: `use`**, on the view layer. Written out from a scene, `scene.view_layers["<name of the layer>"].use`.

**LABEL: documented**

Exact words, Python API documentation, https://docs.blender.org/api/5.2/bpy.types.ViewLayer.html:

> use
>
> Enable or disable rendering of this View Layer (default True)
>
> Type:
>
> bool

Read 2026-07-30.

That the view layers hang off a scene under `view_layers`, exact words, https://docs.blender.org/api/5.2/bpy.types.Scene.html:

> view_layers
>
> (default None, readonly)
>
> Type:
>
> ViewLayers[ViewLayer]

Read 2026-07-30.

---

## What a running Blender 5.2 did — the hand tests of 2026-08-29

**Label: observed.** Nothing in this section comes from a source. It was got by running Blender and watching what it did, and it sits **beside** the answers above rather than over them. Those answers say what the 5.2 sources carry; this says what one machine did on one day. Where an answer above says something is *not in the sources*, that stays true of the sources and is not withdrawn — what follows is a different claim about a different thing.

**What was done, and to what.** Fifteen tests, written out in full before they were run, were run from a terminal on **2026-08-29**. Each ran in all four researched versions; only the Blender 5.2 runs are written here. Every run used the application `/Applications/Blender.app`, which reports itself as **Blender 5.2.1 LTS**, build hash `9e2066aef7ef`, build date 2026-08-25 — read off the application, not taken from any document. Every run used `--factory-startup`, so no add-on, preference or startup file of the Creator's could reach a result, and every run began from Blender's own default scene: a cube, a camera and a light, 1920 by 1080 at 100 per cent, one view layer called `ViewLayer`. Output paths were absolute, into a scratch folder outside this repository. In the listings below the long absolute folder is shortened to `…` and nothing else is changed.

**On what kind of computer.** The Creator's MacBook Pro — an Apple M1 Pro chip, Apple's own graphics with no second card, macOS 26.6.2 (build 25G83, Darwin 25.6.0, arm64), read off the machine on the day. **Nothing was rented, and nothing ran on any other operating system.** Every line below therefore speaks for macOS on Apple silicon and for nothing else.

### The three results that matter more than the names

Together they mean **a builder cannot tell a working instruction from a broken one by anything Blender hands back**.

**1. A wrong name does not stop the render, and does not show in the exit status.** Three names were deliberately misspelled and the frame was rendered anyway:

```
00:00.649  render           | Saved: '…/E9_52_0001.png'
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
SAVED …/E15_52.blend 100
----- instruction AFTER the file -----
00:00.542  render           | Saved: '…/E15_after_52_0001.png'
AFTER 25
----- instruction BEFORE the file -----
00:00.650  render           | Saved: '…/E15_before_52_0001.png'
BEFORE 25
----- sizes -----
E15_after_52_0001.png (480, 270)
E15_before_52_0001.png (1920, 1080)
```

**Both orders printed `25`.** The instruction ran, the name was right, and reading the value straight back gave the value set. **Only one order changed the picture.** With the instruction after the file the written image is 480 by 270; with the same instruction before the file it is 1920 by 1080, because loading the file replaced the whole scene and with it the value just written. So a correct name, correctly set and correctly read back, can still have no effect on the render — and here the readback does not even warn, because at the moment of reading the value really was 25.

**What the three say together.** The exit status is 0 either way. A printed readback is 25 either way. In 4.5.12 even the error is missing. The only thing that separates a working instruction from a broken one is what lands on disk — the size of the picture, the sample count Blender prints while it renders, the channels in the file. Every result below that shows a name works, shows it that way.

### Each name, exercised

Each test below has a control run beside it. The control is what makes the second run mean something.

**`resolution_percentage` changes the picture Blender writes.**

```
----- control, no instruction -----
00:00.652  render           | Saved: '…/E2_ctl_52_0001.png'
----- resolution_percentage = 25 -----
00:00.542  render           | Saved: '…/E2_set_52_0001.png'
READBACK 25
----- pixel sizes -----
E2_ctl_52_0001.png (1920, 1080)
E2_set_52_0001.png (480, 270)
```

1920 by 1080 with no instruction, 480 by 270 with it — a quarter of the width and a quarter of the height.

**`scene.cycles.samples` changes the count Blender says it is working towards, and `preview_samples` is a different setting.**

```
----- control, nothing set -----
00:00.647  render           | Fra: 1 | Remaining: 00:01.55 | Mem: 2M | Sample 0/4096
----- samples=3 preview_samples=200 -----
00:00.387  render           | Fra: 1 | Remaining: 00:00.00 | Mem: 2M | Sample 1/3
00:00.467  render           | Fra: 1 | Mem: 2M | Sample 3/3
READBACK 3 200
```

The control carries `/4096`, the instructed run carries `/3`. Setting `preview_samples` to 200 in the same breath changed nothing about the render.

**`scene.eevee.taa_render_samples` does the same for EEVEE, and `taa_samples` is a different setting.**

```
----- control (BLENDER_EEVEE, nothing set) -----
00:00.837  render           | Fra: 1 | Rendering 64 / 64 samples
----- taa_render_samples=3 taa_samples=200 -----
00:00.537  render           | Fra: 1 | Rendering 1 / 3 samples
00:00.561  render           | Fra: 1 | Rendering 3 / 3 samples
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
00:00.537  render           | Fra: 1 | Mem: 2M | Sample 64/64
READBACK scene 64 layer 0
----- scene samples 64, layer override 5 -----
00:00.389  render           | Fra: 1 | Remaining: 00:00.00 | Mem: 2M | Sample 1/5
00:00.474  render           | Fra: 1 | Mem: 2M | Sample 5/5
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
SHOT OUTPUT …/E13_52_output.png
WINDOW 1512 862 AREA 532 1321
SHOT RENDER …/E13_52_render.png
WINDOW 1512 862 AREA 532 1321
SHOT VIEW_LAYER …/E13_52_view_layer.png
E13_52_output.png (3024, 1724)
E13_52_render.png (3024, 1724)
E13_52_view_layer.png (3024, 1724)
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
lines:      518
128:	Do not use native pixel size, for high resolution displays (MacBook 'Retina').
463:	Background mode: Do not systematically build and evaluate ViewLayers' dependency graphs
```

Two hits in 518 lines, and neither is a switch for any of the three: one is `--no-native-pixels`, about how a screen is drawn, the other `--disable-depsgraph-on-file-load`, about what is built when a file is opened. **The built program agrees with the manual page.** `blender -b --help` prints the same listing with one extra line, a banner naming the version and build. Asking the program for its sub-commands with `-c help` lists three sub-commands — `asset_listing`, `extension` and `maketx`, and none of them names any of the three settings.

**And what Blender does with a launch argument it does not know is the opposite of what a wrong name does.** `--render-percentage 50` — the shape a switch for this setting would take — was handed in on the launch line of a file that would otherwise have rendered:

```
EXIT STATUS 1
00:00.384  blend            | Read blend: "…/u_52.blend"
unknown argument, loading as file: --render-percentage
00:00.388  reports          | ERROR Cannot read file "…/--render-percentage": No such file or directory
Blender 5.2.1 LTS (hash 9e2066aef7ef built 2026-08-25 01:35:57)

Blender quit
--- any picture written? ---
(none)
```

Blender took the unknown argument to be a file name, could not read it, **wrote no picture at all, and exited with status 1**. So an unknown *switch* stops the run and shows in the exit status, while a wrong *name* inside a Python instruction stops nothing and shows in nothing. Those are two completely different behaviours and a builder needs to know which it is looking at.

Cycles was asked the same question. A guessed argument was handed in after `--`, with a documented one as a control that arguments after `--` are read at all:

```
----- guessed -- --cycles-samples 8 -----
00:00.472  render           | Fra: 1 | Remaining: 00:01.29 | Mem: 1M | Sample 240/4096
00:00.487  render           | Fra: 1 | Remaining: 00:01.53 | Mem: 1M | Sample 0/4096
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

- The answer settles the names only, and expressly takes no view on whether an instruction using them takes effect. **Observed beside it:** in Blender 5.2.1 every one of the six names takes effect when handed in at launch — and the ordering warning quoted from the manual bites exactly as the answer's caution implies, because an instruction placed before the file reads back correctly and changes nothing.
- The answer says it did not run Blender, so it did not confirm by exercise that Cycles' `samples` is registered under that name at run time. **Observed beside it:** it is. At `--factory-startup` the running program reports `CyclesRenderSettings.samples`, and setting it from a launch instruction changed the count Blender printed while rendering.
- Part B3 leaves two things unsettled about `render_aa`: which engine it belongs to, and how `SceneDisplay` is reached from a scene. **The second is now settled, observed:** the route is `scene.display`, and `scene.display.render_aa` set from a launch instruction changed the picture Workbench wrote. **The first is not settled by any run** — whether an anti-aliasing menu counts as "the Workbench engine's sample count" is a judgement, and a photograph of Workbench's own Render Properties in 5.2.1 shows one `Sampling` panel with two rows, `Render` reading `8 Samples` and `Viewport` reading `Single Pass Anti-…`, and no plain number-of-samples field beside them.
- The second standing line worries that the record of looking does not cover a search of the `SceneEEVEE` page, the `SceneDisplay` page or Cycles' own file for an interface-only note. **Observed beside it:** the running program's own description was read for all six settings, and only `use_single_layer` carries such a note — "Only affects rendering from the interface, ignored for rendering from command line."
- **Not settled:** whether these names are the only names, and what any engine beyond the three does.

---

### NOT COVERED

- I answer only what the settings are **named**. I do not settle whether setting them from a launch-time instruction actually takes effect, when in the launch sequence such an instruction runs relative to the render, or what the render then does with the value. The sheet's own note says questions 1 and 16 covered those, and I take no view on them.
- Add-on render engines other than Cycles are not covered. Their settings are their own, and I was given no files for them.
- I did not check every possible route to the same setting (a driver, a keyed value, a scene's data path used another way). The question asked for a name and that is what I give.
- For Cycles, the name `samples` is what the add-on's own file declares. I did not run Blender, so I did not confirm by exercise that the property is registered under that name at run time. The registration code quoted is what the file states.
- Cycles has many other sample-related names in the same file (`preview_samples`, `adaptive_min_samples`, `use_layer_samples`, `sample_subset_length` and others). I report only the one that is the render's sample count. The others exist and are not what was asked.
- I did not settle whether an add-on could rename or shadow any of these.
- For `render_aa` I settle the name and the page it sits on, and nothing else. Which render engine it belongs to is not established here, and neither is how `bpy.types.SceneDisplay` is reached from a scene — I did not search for either, so there is no written-out name from a scene for that one.

---

### Outside the interface

The names are the same. They are the Python names of the settings, and Python names do not depend on whether a person is clicking or a script is running. Blender's own documented way of handing an instruction in at launch — `-P`, `--python-text`, `--python-expr` — is a background-and-command-line facility to begin with; the manual lists all three under Python Options on https://docs.blender.org/manual/en/5.2/advanced/command_line/arguments.html, e.g.

> --python-expr <expression>
>
> Run the given expression as a Python script.
>
> The expression may be a complete multi-line script;
> you are limited only by the platform's maximum argument length.

Read 2026-07-30. **LABEL for "the names are the same": inferred** — the sources give one set of names and never give a second set for command-line use, and I found no launch switch that names these settings differently; the step a reader must supply is that no separate naming exists.

Two things from the sources that bear directly on the difference between inside and outside. The quoted words in both are **documented**; where I go past those words I say so and label it separately.

1. Order matters on the command line. Manual, https://docs.blender.org/manual/en/5.2/advanced/command_line/render.html:

   > Arguments are executed in the order they are given!
   >
   > The following command will not work, since the output and extension are set after Blender is told to render:
   >
   > blender -b file.blend -a -x 1 -o //render

   Read 2026-07-30. So an instruction handed in at launch has a place in a sequence; the names it uses do not change, but where it sits does.

2. There is one nearby setting that behaves differently outside the interface, and it is worth naming because it sits next to "whether a view layer renders". Python API documentation, https://docs.blender.org/api/5.2/bpy.types.RenderSettings.html:

   > use_single_layer
   >
   > Only render the active layer. Only affects rendering from the interface, ignored for rendering from command line. (default False)

   Read 2026-07-30. That is `use_single_layer`, not `use`, and it is not one of the three settings asked about. I mention it only because it shows the sources do call out interface-only behaviour where it exists.

   **That they call out no such thing for the names I report is a second claim, and it is not what the quote says. LABEL: inferred.** It rests on my not having met such a note. The record of looking below covers the `RenderSettings` page and the `ViewLayer` page, so it bears on `resolution_percentage`, on the view layer's `samples` override and on the view layer's `use`. It does **not** record a search of the `SceneEEVEE` page, the `SceneDisplay` page, or Cycles' own file for an interface-only note, so for `taa_render_samples`, `render_aa` and the Cycles `samples` the claim rests on less again.

---

**What running Blender 5.2 added, 2026-08-29. Label: observed.** Every run recorded under *What a running Blender 5.2 did* above was made from a terminal with `-b` and no window, and every name in this answer was set and read back that way. So these names are no longer merely *unmarked* as interface-only — they were exercised from outside and they worked. Three things the runs add. **`use_single_layer` really is dead from outside:** set to `True` on the launch line, both view layers still came out in the written file. **Where the instruction sits decides whether it does anything at all:** handed in before the file, a correct name reads back the value set and changes nothing about the render. And **a wrong name costs nothing a program could see:** the frame still renders, the file is still written, and Blender still exits 0. All of that was read off the Creator's Mac, macOS 26.6.2 on Apple silicon, and speaks for no other machine.

### Operating system

The sources say nothing that makes any of the six names above differ by operating system — `resolution_percentage`, the Cycles `samples`, `taa_render_samples`, `render_aa`, the view layer's `samples` override, and the view layer's `use`. The Python API documentation gives one page per version, not per system, and marks nothing here as platform-specific. The manual's list of launch arguments does mark other options as platform-specific where they are — for example, of `-con, --start-console` it says "Start with the console window open (ignored if -b is set), (Windows only)", and of `--no-window-frame` it says "Disable all window decorations (Linux only)" (https://docs.blender.org/manual/en/5.2/advanced/command_line/arguments.html, read 2026-07-30) — and it carries no such mark on the Python options. **LABEL: inferred**, from the sources marking platform-specific items elsewhere and not marking these; a reader has to supply the step that an unmarked item is therefore not platform-specific. That reading also rests on the pages I opened: I did not search each of the six pages in turn for a platform note.

One genuine platform limit does touch how an instruction is handed in, though not what it names. The manual, same page, on `--python-expr`: "you are limited only by the platform's maximum argument length." **documented**, read 2026-07-30. That caps how long a launch-time expression can be, and that cap differs by system; the sources do not say what it is on any system.

Within the six lines quoted above, the Cycles file carries no platform condition on the `samples` property. **observed**, `Blender Sources/cycles_5.2/properties.py`, lines 485–490, read 2026-07-30. I did not read the rest of that file for a platform condition sitting further out, so this covers the lines quoted and nothing wider.

**What running Blender 5.2 added, 2026-08-29. Label: observed, and bounded.** Every run recorded above was on **one computer** — the Creator's MacBook Pro, an Apple M1 Pro chip, macOS 26.6.2 (build 25G83, Darwin 25.6.0, arm64). **Nothing was rented, and nothing ran on Windows or on Linux.** So on **macOS on Apple silicon** these names are now exercised rather than reasoned about. On every other operating system nothing has changed: the reading above is still all there is, and **whether these names hold on Windows or on Linux is unknown**. A run on a machine of each is what would settle it, and no such machine was available for this round. Nobody may read the macOS result as covering the others.

---

### Record of where I looked

**What I searched, and with what.**

- Listed everything under `Blender Sources/`: the 5.2 manual, the 5.2 API documentation (1,982 files), the 5.2 release notes (21 files), and Cycles' 5.2 files.
- Read the whole of the manual's launch-argument page, https://docs.blender.org/manual/en/5.2/advanced/command_line/arguments.html, end to end — all of Render Options, Cycles Render Options, Format Options, Animation Playback Options, Window Options, Python Options, Network Options, Logging Options, Debug Options, GPU Options and Misc Options. I did not sample it; I read every entry. **Nothing in it names a resolution percentage, a sample count, or a view-layer on/off switch.** The render-related switches it does carry are `-b`, `-a`, `-S/--scene`, `-f/--render-frame`, `-s/--frame-start`, `-e/--frame-end`, `-j/--frame-jump`, `-o/--render-output`, `-E/--engine`, `-t/--threads`, plus `-F/--render-format` and `-x/--use-extension`.
- Searched that same page for the terms `percentage`, `samples`, `view layer`, `engine` and `--python`. The only hits were the engine switch, the frame-end switch, and the Python options.
- Read https://docs.blender.org/manual/en/5.2/advanced/command_line/render.html and https://docs.blender.org/manual/en/5.2/advanced/command_line/index.html in full. **The first** points back to the argument list as the complete list — "See Command Line Arguments for a full list of arguments" — and neither adds a switch for these three. *(Corrected 2026-08-29 by the citation check, which opened both pages: only the render page carries that sentence. The index page sends the reader to "Launching from the Command Line" and says nothing about the argument list being full, so "Both" was wrong.)*
- Searched the whole 5.2 manual for `python-expr` and `python_expr`. Two hits: the argument list itself, and one unrelated worked example on the remote asset libraries page.
- Searched the whole 5.2 manual for `resolution_percentage`, `taa_render_samples`, `scene.cycles` and `view_layers[`. **No hits at all.** The manual does not give Python names for these settings. That is why every name above is cited to the API documentation or to Cycles' own file, and not to the manual.
- Searched the whole 5.2 API documentation for `resolution_percentage`: five files — `bpy.types.RenderSettings.txt` (the definition), `bpy.types.RenderEngine.txt` (an example using it), `info_quickstart.txt` (an example using it), and the two index files. Checked the API index (`genindex-all`) and found exactly one attribute of that name.
- Searched the API index for `taa_render_samples`: one entry, on `bpy.types.SceneEEVEE`.
- Opened `bpy.types.ViewLayer.txt` and read the properties around the one I wanted, to be sure I had the right one and not a near neighbour. It carries both `samples` (the override) and `use` (the on/off), quoted above.
- Opened `bpy.types.Scene.txt` and checked the attributes `render`, `view_layers`, `cycles`, `eevee` and `cycles_curves`, to be sure of how each of the three settings is reached from a scene.
- **Ruled out a Cycles page in the API documentation.** Searched the API folder for a file with "cycles" in its name: only `bpy.ops.cycles.txt` and `bpy.types.FModifierCycles.txt` (an unrelated animation modifier). Searched the API documentation's text for `CyclesRenderSettings`: it appears only in `bpy.types.Scene.txt` and `bpy.types.Object.txt` as a type name, and in `change_log.txt` as a list of newly added members. There is no page describing its properties. That is why the Cycles name is labelled observed rather than documented.
- Searched Cycles' own `properties.py` for `samples`: found the scene-level `samples` at line 485 and the many other sample-related names listed under Not Covered. Read the class it sits in and the code that attaches it to a scene.
- **Ruled out a Cycles launch switch for samples.** Read Cycles' own argument parser in `Blender Sources/cycles_5.2/engine.py`, lines 8–41. It declares exactly two arguments — `--cycles-print-stats` and `--cycles-device` — and nothing else. This matches the manual's Cycles Render Options section exactly.
- Searched all 21 release-note files for `command line`, `command-line`, `argument`, and `--`. The only launch-related hits are a `--command maketx` example in the Cycles notes and a `--background` mention in the Python API notes. Neither touches these three settings.
- Searched all 21 release-note files for `resolution_percentage`, `taa_render_samples` and `ViewLayer.use`: **no hits**. Searched the API documentation's own change log for the same three: **no hits**. **That is the search result, and it is all the search result is.** Reading it as "these names were not added, renamed or removed" is a step past it — **LABEL: inferred** — and the step a reader must supply is that such a change would have been written up in one of those two places. I did not search either place for the Cycles `samples`, for the view layer's `samples` override, or for `render_aa`, so nothing here bears on those three at all.
- Opened one picture from the manual, `_images/render_output_properties_format_panel.png`, cited on https://docs.blender.org/manual/en/5.2/render/output/properties/format.html, to see what the resolution percentage is labelled in the panel. It shows `%`.
- Read the manual's EEVEE sampling page, https://docs.blender.org/manual/en/5.2/render/eevee/render_settings/sampling.html, for the person-facing name of the render sample count ("Render" > "Samples", "The number of samples to use in the final render"). Read 2026-07-30.

**What I did not use.** I did not go to the web, and did not need to. I did not open any file outside `Blender Sources/` other than the question sheet.

---

## Things a person with Blender open could still settle by hand

**Five of the six items that stood here were run on 2026-08-29**, and what came back is written up under *What a running Blender 5.2 did* above: that the name in Cycles' own file is the name in a running Blender, and that setting it to 7 from a launch instruction makes the panel's *Max Samples* field read `7` (was item 1); that all three names are accepted from an instruction handed in at launch (2); the EEVEE name (3); that `samples` on a view layer and `samples` on the Cycles settings are two different settings holding different numbers (4); and switching a view layer off and reading back which one changed (6).

One is left, and it is joined by the two things this Mac cannot reach.

1. **Read the names off a hovering tooltip, or copy them off the controls.** Switch on **Python Tooltips** in Preferences ‣ Interface, then hold the pointer still over the Percentage slider in Output Properties ‣ Format, over the render Samples field in Render Properties ‣ Sampling with Cycles and again with EEVEE, and over the *Use for Rendering* checkbox in View Layer Properties. Or right-click each and choose *Copy Full Data Path* and paste the result. Both need a hand holding a mouse on a control, and no command stands in for either. *What they were wanted for is settled above another way*: the running program's own label and description were read out for every one of these properties, and those are the two pieces of text a tooltip draws. A negative result is a tooltip or a pasted path showing a name other than the ones above.
2. **The same three names on an operating system other than macOS.** Every run above was on one Mac on Apple silicon, so Windows and Linux are untouched. Run the same launch-time instruction on a Windows machine and on a Linux machine and compare what is printed and what is written to disk. A negative result is any difference at all in a name. Needs a machine of each, and none was rented for this round.
3. **A sample-count name for a render engine other than Cycles, EEVEE and Workbench.** No such engine is installed on this Mac. With one installed, open a Python console inside Blender, type `bpy.context.scene.` and see what that engine added, then look through it for a sample count. A negative result — the engine adding nothing that carries one — is itself the answer for that engine.

---

2026-08-29 — Claude Opus 5 (Claude Code) — Settings, Blender 5.2

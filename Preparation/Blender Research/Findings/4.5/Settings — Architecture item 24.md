# Settings — Architecture item 24

**Sources consulted:** Blender 4.5 manual (`Blender Sources/manual_4.5`), Blender 4.5 Python API documentation (`Blender Sources/api_4.5`), Blender 4.5 release notes (`Blender Sources/release_notes_4.5`), and Cycles' own files (`Blender Sources/cycles_4.5`). All four were taken on 2026-07-30, and that is the date read for everything quoted below. I did the reading on 2026-08-29.

I read only the question sheet and files inside Blender Sources/. I did not open any other part of this project.

---

## 17. When a render is started from outside and an instruction handed in at launch sets the resolution percentage, the sample count, or whether a view layer renders, what name does that instruction use for each of those three settings?

### The question was ambiguous, and I say so before answering

"An instruction handed in at launch" can be read two ways.

- **Reading A:** a switch on the command line of its own, like `-f` or `-E`.
- **Reading B:** a Python instruction handed to Blender at launch, by one of the launch options that exist for exactly that.

I answer both, because they turn out to fit together: for these three settings there is no switch of the first kind in the sources, so the only launch-time instruction that reaches them is the second kind, and it is that instruction whose names are asked for. If only Reading A was meant, the answer is the short one under "No switch of its own in the argument list" below.

### No switch of its own in the argument list

**Label: inferred.**

The manual's command line page lists Blender's arguments in full, grouped by kind. Its "Render Options" group is, complete:

> "-b, --background", "-a, --render-anim", "-S, --scene <name>", "-f, --render-frame <frame>", "-s, --frame-start <frame>", "-e, --frame-end <frame>", "-j, --frame-jump <frames>", "-o, --render-output <path>", "-E, --engine <engine>", "-t, --threads <threads>"

Its "Cycles Render Options" group is, complete:

> "Cycles add-on options must be specified following a double dash."
> "--cycles-device <device>"
> "--cycles-print-stats"

Source: manual — https://docs.blender.org/manual/en/4.5/advanced/command_line/arguments.html — read 2026-07-30.

None of those names a resolution percentage, a sample count, or whether a view layer renders. I searched the whole of that page for `percent`, `sample`, `resolution`, `view layer`, `view-layer` and `viewlayer`; the only hits are `--no-native-pixels` ("Do not use native pixel size, for high resolution displays") and `--disable-depsgraph-on-file-load` ("Do not systematically build and evaluate ViewLayers' dependency graphs when loading a blend-file in background mode"), and neither sets any of the three.

**The step I took:** the page presents itself as the argument list ("Usage: blender [args ...] [file] [args ...]") and Blender's own `blender --help` is pointed at as the same thing ("See Command Line Arguments for a full list of arguments … or simply run: `blender --help`", manual — https://docs.blender.org/manual/en/4.5/advanced/command_line/render.html — read 2026-07-30). From "the full list does not contain such a switch" I concluded "there is no such switch". A reader has to accept that the list is complete to get there, so this is inferred, not documented.

For the Cycles half the evidence is Cycles' own file. It defines Cycles' command-line arguments, and the part quoted below holds two:

```
    parser.add_argument("--cycles-print-stats",
                        help="Print rendering statistics to stderr",
                        action='store_true')
    parser.add_argument("--cycles-device",
                        help="Set the device to use for Cycles, overriding user preferences and the scene setting."
```

Source: Cycles' own files — `Blender Sources/cycles_4.5/engine.py`, in the function `_configure_argument_parser()`, lines 8 to 21 — read 2026-07-30. **Label for the two arguments themselves: observed.**

That Cycles adds no sample-count argument of its own is a further step, and takes its own label. **Label: inferred.** The step: I read the quoted lines as the whole of that function, and that function as the only place Cycles adds arguments, and from that concluded nothing else is there. A reader has to accept both to get where I got.

### The instruction that can set them, and what it is called

**Label: documented.**

Three launch options hand Blender a Python instruction:

> "-P, --python <filepath>
> Run the given Python script file."
>
> "--python-text <name>
> Run the given Python script text block."
>
> "--python-expr <expression>
> Run the given expression as a Python script."

Source: manual — https://docs.blender.org/manual/en/4.5/advanced/command_line/arguments.html — read 2026-07-30.

The API documentation shows this being used as the ordinary way to drive Blender with no interface:

> "For scripts that are not interactive it can end up being more efficient not to use Blender's interface at all and instead execute the script on the command line."
> `blender --background --python myscript.py`
> `blender myscene.blend --background --python myscript.py`

Source: Python API documentation — https://docs.blender.org/api/4.5/info_tips_and_tricks.html — read 2026-07-30.

So the name that instruction uses for a setting is that setting's Python property name. **Label for that sentence alone: inferred.** The quoted words say these options run Python; they do not say how a setting is named once Python is running. The step is that a Python instruction reaches a setting by naming its property, and I supplied it. Those names follow.

### The names, one setting at a time

#### Resolution percentage → `resolution_percentage`

**Label: documented.**

> "resolution_percentage
> Percentage scale for render resolution
> Type:
> int in [1, 32767], default 100"

Source: Python API documentation — https://docs.blender.org/api/4.5/bpy.types.RenderSettings.html — read 2026-07-30.

It sits on a scene's render settings:

> "render
> Type:
> RenderSettings, (readonly, never None)"

Source: Python API documentation — https://docs.blender.org/api/4.5/bpy.types.Scene.html — read 2026-07-30.

The API documentation writes the whole name out in an example:

> `>>> bpy.data.scenes[0].render.resolution_percentage`
> `100`

Source: Python API documentation — https://docs.blender.org/api/4.5/info_quickstart.html — read 2026-07-30.

The interface appears to call this same setting something else. The manual's Format panel names it:

> "Percentage
> Slider to reduce or increase the size of the rendered image relative to the Resolution values."

Source: manual — https://docs.blender.org/manual/en/4.5/render/output/properties/format.html — read 2026-07-30.

That this "Percentage" slider is the same setting as `resolution_percentage` is a step, not something either quote states. **Label for it: inferred.** The step is that both describe scaling the rendered image against the resolution, and I read them as one thing.

#### Sample count → the name depends on the render engine

There is no single name. Each engine keeps its own, under its own group on the scene.

**Cycles: `samples`, under `cycles`.** **Label: observed** (this comes from Cycles' own file, not from documentation).

> ```
>     samples: IntProperty(
>         name="Samples",
>         description="Number of samples to render for each pixel",
>         min=1, max=(1 << 24),
>         default=4096,
>     )
> ```

Source: Cycles' own files — `Blender Sources/cycles_4.5/properties.py`, lines 482 to 487, inside `class CyclesRenderSettings(bpy.types.PropertyGroup)` which begins at line 376 — read 2026-07-30.

The group is attached to the scene under the name `cycles`:

> ```
>     @classmethod
>     def register(cls):
>         bpy.types.Scene.cycles = PointerProperty(
>             name="Cycles Render Settings",
>             description="Cycles render settings",
>             type=cls,
>         )
> ```

Source: Cycles' own files — `Blender Sources/cycles_4.5/properties.py`, lines 1100 to 1106 — read 2026-07-30. *(Line range corrected 2026-08-29 by the citation check, which opened the file: the seven quoted lines run 1100 to 1106, and 1107 is blank. The words are unchanged.)*

The same attachment is documented on the scene page:

> "cycles
> Cycles render settings
> Type:
> CyclesRenderSettings, (readonly)"

Source: Python API documentation — https://docs.blender.org/api/4.5/bpy.types.Scene.html — read 2026-07-30.

So the full name written out is `scene.cycles.samples`, put together from one file quote and one documentation quote. A separate, differently named property sits beside it, `preview_samples`, described in the file as "Number of samples to render in the viewport, unlimited if 0" (same file, lines 488 to 495) — **observed**. That an instruction naming `preview_samples` is therefore not setting the render count is a step from that description rather than something the file states — **inferred**.

**EEVEE: `taa_render_samples`, under `eevee`.** **Label: documented.**

> "taa_render_samples
> Number of samples per pixel for rendering
> Type:
> int in [1, inf], default 64"

Source: Python API documentation — https://docs.blender.org/api/4.5/bpy.types.SceneEEVEE.html — read 2026-07-30.

> "eevee
> EEVEE settings for the scene
> Type:
> SceneEEVEE, (readonly)"

Source: Python API documentation — https://docs.blender.org/api/4.5/bpy.types.Scene.html — read 2026-07-30.

Full name: `scene.eevee.taa_render_samples`. A second, separately named property sits beside it, `taa_samples`, described as "Number of samples, unlimited if 0" (same page). Those words do not say which count it is, so nothing here settles what it is for.

**Workbench: `render_aa`, under `display`.** **Label: documented.** This is not a plain count — it is a choice from a fixed set, some of whose choices are sample counts.

> "render_aa
> Method of anti-aliasing when rendering final image
> OFF
> No Anti-Aliasing – Scene will be rendering without any anti-aliasing.
> FXAA
> Single Pass Anti-Aliasing – Scene will be rendered using a single pass anti-aliasing method (FXAA).
> 5
> 5 Samples – Scene will be rendered using 5 anti-aliasing samples.
> 8
> 8 Samples – Scene will be rendered using 8 anti-aliasing samples."

Source: Python API documentation — https://docs.blender.org/api/4.5/bpy.types.SceneDisplay.html — read 2026-07-30.

> "display
> Scene display settings for 3D viewport
> Type:
> SceneDisplay, (readonly)"

Source: Python API documentation — https://docs.blender.org/api/4.5/bpy.types.Scene.html — read 2026-07-30.

**A per-view-layer sample count also exists, and it is also called `samples`.** **Label: documented.**

> "samples
> Override number of render samples for this view layer, 0 will use the scene setting
> Type:
> int in [0, inf], default 0"

Source: Python API documentation — https://docs.blender.org/api/4.5/bpy.types.ViewLayer.html — read 2026-07-30.

So the bare word `samples` names two different settings depending on what it is written after: `scene.cycles.samples` is the scene's count, `scene.view_layers["…"].samples` is one view layer's override. Cycles carries a further setting named `use_layer_samples`, offering three choices — `'USE'`, `'BOUNDED'`, `'IGNORE'` (`Blender Sources/cycles_4.5/properties.py`, line 553, with the choices at lines 68 to 71) — **observed**, though no words from the file are quoted for it here. What a render then does with that setting is not established by the file and is not claimed.

#### Whether a view layer renders → `use`

**Label: documented.**

> "use
> Enable or disable rendering of this View Layer
> Type:
> boolean, default False"

Source: Python API documentation — https://docs.blender.org/api/4.5/bpy.types.ViewLayer.html — read 2026-07-30.

A scene's view layers are reached by the name `view_layers`:

> "view_layers
> Type:
> ViewLayers bpy_prop_collection of ViewLayer, (readonly)"

Source: Python API documentation — https://docs.blender.org/api/4.5/bpy.types.Scene.html — read 2026-07-30.

Full name written out: `scene.view_layers["…"].use`, the quoted part being the view layer's own name. **Label for that bracket form: inferred.** The words quoted here give the name `view_layers` and say it holds view layers; they do not show one being picked out by its name, and writing it that way is my step.

### The three names together

| Setting | Name the instruction uses | Written out from a scene |
| --- | --- | --- |
| Resolution percentage | `resolution_percentage` | `scene.render.resolution_percentage` |
| Sample count — Cycles | `samples` | `scene.cycles.samples` |
| Sample count — EEVEE | `taa_render_samples` | `scene.eevee.taa_render_samples` |
| Sample count — Workbench, if it counts as one | `render_aa` | `scene.display.render_aa` |
| Sample count — one view layer's override | `samples` | `scene.view_layers["…"].samples` |
| Whether a view layer renders | `use` | `scene.view_layers["…"].use` |

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

- The answer says plainly that it settles the names only, and takes no view on whether an instruction using them takes effect. **It now has that view, separately labelled**: in Blender 4.5.12 all six names take effect when handed in at launch, and the one thing that can stop them is where the instruction sits relative to the file.
- The answer quotes `ViewLayer.use` as "boolean, default False". That quote is right about 4.5's documentation, and the running program declares the same word. What the running program adds is that **no view layer anybody will meet gets that value** — the default scene's `ViewLayer` reads `True`, and a fresh one made by `view_layers.new(…)` reads `True`.
- The *Not covered* bullet about `use_single_layer` stands exactly as written: it is a statement about what this answer looked for. What the runs add beside it is that the name exists in 4.5.12 and does nothing when the render is started from outside.
- The written-out form `scene.view_layers["…"].use`, labelled inferred here, is the form every run used, and it worked.
- **Not settled by any of this:** whether these names are the *only* names, which scene an instruction should hang them from, and whether `render_aa` counts as "a sample count" — that last one is a judgement and running Blender does not make it.

---

### Not covered

- This settles the names only. It says nothing about whether an instruction using them takes effect, when it runs relative to the render, or whether Blender permits it to run at all. Whether Blender runs a script it is handed is a separate matter and I did not investigate it, since it is not what was asked.
- I did not establish that these names are the *only* names. A property can in principle be reached by more than one route, and I did not look for alternatives beyond the paths quoted above.
- The scene these names hang from is not settled here. The sources show `bpy.data.scenes[...]` and `bpy.context.scene` as ways to reach a scene; which one a launch-time instruction should use, and how that interacts with `-S, --scene <name>`, is outside this answer.
- For the sample count I found no engine-neutral name at all. If a fourth render engine is in play, its name is not covered — question 13's territory, not this one.
- Whether `render_aa` counts as "a sample count" is a judgement. It is a choice from a list, not a number, and I report it as what Workbench offers in that place rather than claiming it is the same kind of setting.
- **This answer never looked for `use_single_layer`, the near-miss name that sits beside a view layer's `use`.** *(Added 2026-08-29 by the citation check.)* Nothing above searched for it, so its absence here is a gap in the looking and not something established. The 4.5 API documentation does carry it: "use_single_layer / Only render the active layer. Only affects rendering from the interface, ignored for rendering from command line. / Type: / boolean, default False" — Python API documentation, https://docs.blender.org/api/4.5/bpy.types.RenderSettings.html, read 2026-07-30, **documented**. Nobody may read this document's silence as 4.5 lacking that name.

### Outside the interface

Yes — and this is in fact the only place the question's premise applies. The `-P / --python`, `--python-text` and `--python-expr` options are quoted above as launch arguments, so an instruction handed in one of those ways is handed in from outside. **Label: inferred**, the step being that an argument given at launch cannot be given any other way.

The property names themselves are not written down anywhere in the sources as depending on whether a person is clicking. **Label for that point: inferred** — the step is that the API documentation states each property flatly, with no clause about interface or background mode, and I read that silence as "the same either way".

The manual does record that the same names are visible from inside the interface, which is how a person finds them:

> "Python Tooltips
> Displays a property's Python information below the tooltip."

Source: manual — https://docs.blender.org/manual/en/4.5/editors/preferences/interface.html — read 2026-07-30.

> "Python
> When Python Tooltips are enabled, a Python expression is displayed for scripting (usually an operator or property)."

Source: manual — https://docs.blender.org/manual/en/4.5/getting_started/help.html — read 2026-07-30.

One difference worth naming: the interface shows these settings under their interface labels ("Percentage", "Samples"), not under these names, unless that preference is switched on.

**What running Blender 4.5 added, 2026-08-29. Label: observed.** Every run recorded under *What a running Blender 4.5 did* above was made from a terminal with `-b` and no window, and every name in this answer was set and read back that way. So these names are no longer merely *unmarked* as interface-only — they were exercised from outside and they worked. Three things the runs add. **`use_single_layer` really is dead from outside:** set to `True` on the launch line, both view layers still came out in the written file. **Where the instruction sits decides whether it does anything at all:** handed in before the file, a correct name reads back the value set and changes nothing about the render. And **a wrong name costs nothing a program could see:** the frame still renders, the file is still written, and Blender still exits 0. All of that was read off the Creator's Mac, macOS 26.6.2 on Apple silicon, and speaks for no other machine.

### Operating system

The same on all of them. **Label: inferred.** The step: the command line page marks every argument that is limited to one operating system, and does so plainly — "-con, --start-console … (Windows only)", "/? … (Windows only)", "-r, --register … (Windows & Linux only)", "--debug-ghost … (Linux only)". None of the Python options carries such a mark, and none of the property pages in the API documentation mentions an operating system at all. From "the sources mark the exceptions and these are not marked" I concluded "these are not exceptions".

The manual does give a different command per operating system for *starting* Blender — `./blender` on Linux, `./Blender.app/Contents/MacOS/Blender` on macOS, `blender.exe` on Windows (manual — https://docs.blender.org/manual/en/4.5/advanced/command_line/launch/linux.html, .../macos.html, .../windows.html — read 2026-07-30). That is the path to the program, not the name of a setting, and it does not change any of the three names.

**What running Blender 4.5 added, 2026-08-29. Label: observed, and bounded.** Every run recorded above was on **one computer** — the Creator's MacBook Pro, an Apple M1 Pro chip, macOS 26.6.2 (build 25G83, Darwin 25.6.0, arm64). **Nothing was rented, and nothing ran on Windows or on Linux.** So on **macOS on Apple silicon** these names are now exercised rather than reasoned about. On every other operating system nothing has changed: the reading above is still all there is, and **whether these names hold on Windows or on Linux is unknown**. A run on a machine of each is what would settle it, and no such machine was available for this round. Nobody may read the macOS result as covering the others.

---

## Record of where I looked

**Files searched, whole-folder:**

- `Blender Sources/manual_4.5` — searched for `--samples`, `--resolution`, `--percentage`, `--view-layer`, `--render-percentage`, `--scale`: no hits. Searched for `--python-expr`: one hit, the arguments page. Searched for `resolution_percentage`, `taa_render_samples`, `cycles.samples`, `view_layers[`: no hits anywhere in the manual, which is why the names in this answer come from the API documentation and Cycles' files rather than the manual.
- `Blender Sources/api_4.5` — searched for `resolution_percentage` (5 hits, listed below), `--samples`/`--resolution`/`--percentage`/`--view-layer`/`--render-percentage` (no hits), `--background` and `--python ` (four pages, one of them quoted above).
- `Blender Sources/release_notes_4.5` — searched for `command line`, `command-line`, `argument`. Three hits, all irrelevant to these three settings: a Python `find` method note (`python_api.md`), a modelling note about manifold arguments (`modeling.md`), and `eevee.md` line 97, "There's a new `--profile-gpu` command line argument that exports GPU and CPU profiling data". Nothing in 4.5's release notes adds or changes a command-line argument for a resolution percentage, a sample count, or a view layer.
- `Blender Sources/cycles_4.5` — searched for `argv` (3 hits, all in `engine.py`), `samples` in `properties.py` (28 hits, worked through).

**Pages opened in full or in part:**

- Manual: `advanced/command_line/index`, `advanced/command_line/arguments`, `advanced/command_line/render`, `advanced/command_line/extension_arguments`, `advanced/command_line/launch/index`, `.../launch/linux`, `.../launch/macos`, `.../launch/windows`, `render/output/properties/format`, `editors/preferences/interface`, `getting_started/help`.
- API documentation: `bpy.types.RenderSettings`, `bpy.types.Scene`, `bpy.types.ViewLayer`, `bpy.types.SceneEEVEE`, `bpy.types.SceneDisplay`, `info_quickstart`, `info_tips_and_tricks`, plus the index files `genindex-all` and `genindex-R` to check that `resolution_percentage` and `use` are not attributes of some other type I had missed.
- Cycles' own files: `engine.py` (lines 1 to 80), `properties.py` (the sampling block around lines 476 to 560, the class declaration at line 376, the registration at lines 1100 to 1112, and the view-layer registration around line 1537).

**Ruled out, and why:**

- **A dedicated command-line switch.** Ruled out by the complete argument list on the manual's command line page and by Cycles' own argument parser, which together offer nothing for these three settings. Terms tried are listed above; I tried the plain words (`percentage`, `resolution`, `samples`), the industry words (`view layer`, `AA`, `anti-alias` via the `render_aa` hit), and the surrounding words.
- **The release notes.** Searched and read for any 4.5 change to command-line arguments; the only such change named is `--profile-gpu`, which is a profiling switch.
- **The manual as a source for the names.** Ruled out by search: the manual nowhere writes `resolution_percentage`, `taa_render_samples`, or a view-layer `use`. It names these settings by their interface labels only. This is why the answer above cites the API documentation and Cycles' files for every name.
- **The extensions command (`--command extension …`).** Read in full; it manages add-on packages and touches none of the three settings.
- **Environment variables.** The list on the arguments page is all directory and file-path settings plus `OCIO`, `TEMP`, `TMPDIR`, and the splash-screen images. None names a render setting.
- **Pictures.** I did not need one. The one picture that sat near this material, `_images/render_output_properties_format_panel.png` on the Format page, shows the interface panel, and the interface label is not what was asked for — the question asks what the *instruction* calls the setting, and a screenshot of a panel cannot show that.

**I did not use the web, and did not need to.**

---

## Things a person with Blender open could still settle by hand

**Five of the six items that stood here were run from a terminal on 2026-08-29**, and what came back is written up under *What a running Blender 4.5 did* above: the resolution-percentage name (was item 1), the Cycles sample-count name (2), the view-layer name (3), the EEVEE and Workbench names, including which of the number `8` and the text `'8'` Blender accepts (4), and whether any launch switch exists, in `blender --help` and in Cycles' own arguments (6). They are gone from this list because they are answered, not because they were dropped.

Three things are left, and none can be reached from a terminal on this Mac.

1. **Read the names off a hovering tooltip, or copy them off the controls.** Switch on **Python Tooltips** in Preferences ‣ Interface, then hold the pointer still over the Percentage slider in Output Properties ‣ Format, over the render Samples field in Render Properties ‣ Sampling with Cycles and again with EEVEE, and over the *Use for Rendering* checkbox in View Layer Properties. Or right-click each and choose *Copy Full Data Path* and paste the result. Both need a hand holding a mouse on a control, and no command stands in for either. *What they were wanted for is settled above another way*: the running program's own label and description were read out for every one of these properties, and those are the two pieces of text a tooltip draws. A negative result is a tooltip or a pasted path showing a name other than the ones above.
2. **The same three names on an operating system other than macOS.** Every run above was on one Mac on Apple silicon, so Windows and Linux are untouched. Run the same launch-time instruction on a Windows machine and on a Linux machine and compare what is printed and what is written to disk. A negative result is any difference at all in a name. Needs a machine of each, and none was rented for this round.
3. **A sample-count name for a render engine other than Cycles, EEVEE and Workbench.** No such engine is installed on this Mac. With one installed, open a Python console inside Blender, type `bpy.context.scene.` and see what that engine added, then look through it for a sample count. A negative result — the engine adding nothing that carries one — is itself the answer for that engine.

---

2026-08-29 — Claude Opus 5 (Claude Code) — Settings, Blender 4.5

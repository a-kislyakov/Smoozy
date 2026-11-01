# Settings — Architecture item 24, second pass (Blender 5.0)

**Sources consulted, and when.** Blender 5.0 manual (`Blender Sources/manual_5.0`, including its `_images` folder), Blender 5.0 Python API documentation (`Blender Sources/api_5.0`), Blender 5.0 release notes (`Blender Sources/release_notes_5.0`), and Cycles' own files (`Blender Sources/cycles_5.0`). All four were taken on 2026-07-30, and that is the date read for everything quoted below. I did the reading on 2026-08-29.

I read only the question sheet and files inside Blender Sources/. I did not open any other part of this project.

Only question 17 is answered, as instructed.

---

## 17. When a render is started from outside and an instruction handed in at launch sets the resolution percentage, the sample count, or whether a view layer renders, what name does that instruction use for each of those three settings?

### The ambiguity, flagged before the answer

The question can be read two ways, and the answer differs, so both are given.

- **Reading A (the one I take to be meant, and the one answered in full below):** the instruction is a Python instruction handed to Blender at launch — with `--python`, `--python-text` or `--python-expr` — and the question asks what each of the three settings is called in that instruction.
- **Reading B:** the instruction is a dedicated command-line switch of Blender's own, like `-f` or `-E`. Answered under "Reading B" further down. The short version is that 5.0's documented list of command-line arguments contains no switch for any of the three.

I take Reading A to be meant because Reading B has no answer at all — no such switch appears anywhere in the sources — and a question asking for a name is unlikely to be asking for three absences.

### Reading A — the names a launch-time Python instruction uses

Read the labels below in two layers. Each name carries its own label for the words that give it. The answer to the question as asked — that these are the names a launch-time instruction uses — joins those names to a separately documented fact, that an instruction can be handed in at launch. **That join is inferred**, and the step is written out at the end of this section.

**1. Resolution percentage — `resolution_percentage`**, on the scene's `render` settings. Written out in full as it would appear in an instruction: `bpy.context.scene.render.resolution_percentage`, or `bpy.data.scenes[0].render.resolution_percentage`.

- LABEL: **documented**
- Exact words, from the Python API documentation, page `https://docs.blender.org/api/5.0/bpy.types.RenderSettings.html`, read 2026-07-30:

  > resolution_percentage
  >
  > Percentage scale for render resolution
  >
  > Type:
  >
  > int in [1, 32767], default 100

- That this property hangs off the scene under the name `render` — same source family, page `https://docs.blender.org/api/5.0/bpy.types.Scene.html`, read 2026-07-30:

  > render
  >
  > Type:
  >
  > RenderSettings, (readonly, never None)

- The full written-out form is also shown verbatim in the API documentation's quick start, page `https://docs.blender.org/api/5.0/info_quickstart.html`, read 2026-07-30:

  > \>\>\> bpy.data.scenes[0].render.resolution_percentage
  >
  > 100

  The same page says why the name matters and where a person can find it:

  > In fact, the tooltip for each button also displays the Python attribute
  > which can help in finding what settings to change in a script.

**2. Sample count — the name depends on the render engine.**

**2a. Cycles — `samples`**, on the scene's `cycles` settings. Written out: `bpy.context.scene.cycles.samples`.

- LABEL: **observed** (the name is taken from Cycles' own files, which are read as files rather than quoted from a documentation page)
- Exact words, from `Blender Sources/cycles_5.0/properties.py`, lines 479–484, inside the class `CyclesRenderSettings` (declared at line 378), read 2026-07-30:

  > ```
  >     samples: IntProperty(
  >         name="Samples",
  >         description="Number of samples to render for each pixel",
  >         min=1, max=(1 << 24),
  >         default=4096,
  >     )
  > ```

- That this group is reached from a scene under the name `cycles` — same file, lines 1101–1107 *(line range corrected 2026-08-29 by the citation check, which opened the file; the words are unchanged)*:

  > ```
  >     @classmethod
  >     def register(cls):
  >         bpy.types.Scene.cycles = PointerProperty(
  >             name="Cycles Render Settings",
  >             description="Cycles render settings",
  >             type=cls,
  >         )
  > ```

- Corroborated in the API documentation, page `https://docs.blender.org/api/5.0/bpy.types.Scene.html`, read 2026-07-30:

  > cycles
  >
  > Cycles render settings
  >
  > Type:
  >
  > CyclesRenderSettings, (readonly)

- Note the split between the two kinds of name. The instruction uses `samples`. Beside that name, Cycles' own interface file carries a different piece of text — `Blender Sources/cycles_5.0/ui.py`, line 313: `col.prop(cscene, "samples", text="Max Samples")`. That line is what the file says; reading it as *what a person sees on screen* is a step past the file, and for that step the label is **inferred**. The manual, which is a document rather than a file, calls the setting "Render (Max) Samples" (`https://docs.blender.org/manual/en/5.0/render/cycles/render_settings/sampling.html`, read 2026-07-30) — that one is **documented**. Either way, an instruction uses `samples` and not that other text.

**2b. EEVEE — `taa_render_samples`**, on the scene's `eevee` settings. Written out: `bpy.context.scene.eevee.taa_render_samples`.

- LABEL: **documented**
- Exact words, from the Python API documentation, page `https://docs.blender.org/api/5.0/bpy.types.SceneEEVEE.html`, read 2026-07-30:

  > taa_render_samples
  >
  > Number of samples per pixel for rendering
  >
  > Type:
  >
  > int in [1, inf], default 64

  The neighbouring property is the viewport one, and is named differently, so the two are easy to confuse:

  > taa_samples
  >
  > Number of samples, unlimited if 0
  >
  > Type:
  >
  > int in [0, inf], default 16

- That this group is reached from a scene under the name `eevee` — `https://docs.blender.org/api/5.0/bpy.types.Scene.html`, read 2026-07-30:

  > eevee
  >
  > EEVEE settings for the scene
  >
  > Type:
  >
  > SceneEEVEE, (readonly)

**3. Whether a view layer renders — `use`**, on the view layer. Written out: `bpy.context.scene.view_layers["<name of the view layer>"].use`. (The sources I read give the name of the collection and the name of the setting; what a view layer itself is called in a given file is not something I have quoted words for, so it is left blank here.)

- LABEL: **documented**
- Exact words, from the Python API documentation, page `https://docs.blender.org/api/5.0/bpy.types.ViewLayer.html`, read 2026-07-30:

  > use
  >
  > Enable or disable rendering of this View Layer
  >
  > Type:
  >
  > boolean, default True

- That view layers are reached from a scene under the name `view_layers` — `https://docs.blender.org/api/5.0/bpy.types.Scene.html`, read 2026-07-30:

  > view_layers
  >
  > Type:
  >
  > ViewLayers bpy_prop_collection of ViewLayer, (readonly)

- Again the interface name is different. The manual calls it "Use for Rendering" (`https://docs.blender.org/manual/en/5.0/render/layers/view_layer.html`, read 2026-07-30):

  > Use for Rendering
  >
  > The active view layer will be used during rendering.

  I also opened the screenshot on that same page (`https://docs.blender.org/manual/en/5.0/render/layers/view_layer.html`, read 2026-07-30), `_images/render_layers_view-layer_panel.png`, which is part of the manual. It shows a "View Layer" panel with a ticked checkbox labelled **Use for Rendering** and, below it, an unticked **Render Single Layer**. That confirms the interface label only; it shows no instruction name. (LABEL for what the picture carries: **documented**.)

**That such an instruction can be handed in at launch at all**, and under what argument names, is documented on the manual's command-line arguments page, `https://docs.blender.org/manual/en/5.0/advanced/command_line/arguments.html`, read 2026-07-30, under "Python Options":

> -P, --python <filepath>
>
> Run the given Python script file.
>
> --python-text <name>
>
> Run the given Python script text block.
>
> --python-expr <expression>
>
> Run the given expression as a Python script.

And in the API documentation, page `https://docs.blender.org/api/5.0/info_tips_and_tricks.html`, read 2026-07-30, showing it used together with running without the interface:

> blender myscene.blend --background --python myscript.py

The step I take, and it is small but real: the sources give the property names, and separately document that a Python script or expression can be handed in at launch. Joining the two — "so the launch-time instruction uses those names" — is mine. For that joined claim the label is **inferred**, even though each half is documented.

### Reading B — a dedicated command-line switch

None appears in the sources, for any of the three.

- LABEL: **not in the sources** for a switch name; **inferred** for the stronger claim that no such switch exists.
- The step: the manual's command-line arguments page presents itself as the list of Blender's arguments ("Usage: blender [args ...] [file] [args ...]", and `-h, --help` is described as "Print this help text and exit"). I read the page in full, all 662 lines, section by section — Render Options, Cycles Render Options, Format Options, Animation Playback Options, Window Options, Python Options, Network Options, Logging Options, Debug Options, GPU Options, Misc Options, Other Options, Argument Parsing, Argument Order, Environment Variables. Nothing on it sets a resolution percentage, a sample count, or a view layer's on/off state. Reading "absent from the list" as "does not exist" is my step, not the source's words.
- The render-related switches that do exist are `-b/--background`, `-a/--render-anim`, `-S/--scene`, `-f/--render-frame`, `-s/--frame-start`, `-e/--frame-end`, `-j/--frame-jump`, `-o/--render-output`, `-E/--engine`, `-t/--threads`, plus `-F/--render-format` and `-x/--use-extension`.
- Cycles' own two launch switches are the only engine-specific ones, and neither concerns samples. From the same manual page:

  > --cycles-device <device>
  >
  > Set the device used for rendering.
  > Valid options are: CPU CUDA OPTIX HIP ONEAPI METAL.

  > --cycles-print-stats
  >
  > Log statistics about render memory and time usage.

  Confirmed **observed** in Cycles' own files: `Blender Sources/cycles_5.0/engine.py`, lines 8–21, the function `_configure_argument_parser`, adds exactly two arguments, `"--cycles-print-stats"` and `"--cycles-device"`, and nothing else.
- The `-c, --command` switch is documented on the same page as "Run a command which consumes all remaining arguments", and the manual's sub-command page (`https://docs.blender.org/manual/en/5.0/advanced/command_line/extension_arguments.html`, read 2026-07-30) documents only extension management sub-commands — list, sync, update, install and so on. Nothing about rendering settings.

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

- The answer does not settle whether setting these names at launch takes effect, and says order plainly matters. **Observed beside it:** in Blender 5.0.1 every name takes effect, and order decides everything — the same instruction placed before the file reads back `25` and leaves the picture at full size.
- The answer reads `col.prop(cscene, "samples", text="Max Samples")` out of Cycles' interface file and labels reading it as *what a person sees* an inferred step. **A photograph of a running 5.0.1 settles that step:** the Render group of the Sampling panel really does show a row labelled `Max Samples`, reading `4096`, with a second `Max Samples` reading `1024` in the Viewport group above it.
- The answer stops at Workbench and says whether any engine other than Cycles and EEVEE has a name a launch-time instruction would use for a sample count is **not settled here**. That stays true of the sources. **Observed beside it:** `scene.display.render_aa` is reachable from a launch instruction in 5.0.1, takes a piece of text rather than a number, and changes the picture Workbench writes. Whether that counts as "a sample count" is still a judgement, and running Blender does not make it.
- The *Not covered* bullet about `use_single_layer` stands. **Observed beside it:** the name exists here and does nothing from outside.
- The answer notes it has not checked what happens when a launch-time instruction meets what is already in the file. **Observed beside it:** loading a file replaces the value an earlier instruction wrote.

---

### Not covered

- The interface name and the instruction name are different things, and this answer gives the instruction name. Where I quote an interface label ("Max Samples", "Use for Rendering", "%"), that is context, not the answer.
- I do not settle whether setting these names at launch actually takes effect in the render that follows — only what the settings are called. Order of arguments plainly matters (the manual warns "Arguments are executed in the order they are given!"), and I have not tested any ordering.
- The Cycles per-view-layer sample override is a fourth, different name — `bpy.types.ViewLayer.samples`, documented as "Override number of render samples for this view layer, 0 will use the scene setting" (`https://docs.blender.org/api/5.0/bpy.types.ViewLayer.html`, read 2026-07-30). It was not asked about and I have not gone into it.
- Workbench I looked at once and no further. What the API documents on the page I opened (`https://docs.blender.org/api/5.0/bpy.types.SceneDisplay.html`, read 2026-07-30) is `render_aa`, "Method of anti-aliasing when rendering final image", an enum rather than a count. Whether Workbench, or any render engine other than Cycles and EEVEE, has a name a launch-time instruction would use for a sample count is **not settled here** — I stopped rather than searched.
- I have not checked what happens if two instructions set the same thing, or how a launch-time instruction interacts with anything already in the file.
- **This pass never looked for `use_single_layer`.** *(Added 2026-08-29 by the citation check.)* It quotes the manual's note about the neighbouring control "Render Single Layer" but gives no Python name for it, and no search above went after one — a gap in the looking, not an absence. The 5.0 API documentation carries it: "use_single_layer / Only render the active layer. Only affects rendering from the interface, ignored for rendering from command line. / Type: / boolean, default False" — Python API documentation, https://docs.blender.org/api/5.0/bpy.types.RenderSettings.html, read 2026-07-30, **documented**.
- I did not look at any Blender version other than 5.0.

### Outside the interface

The answer is about launching from outside the interface, so it holds there by construction. The three names are names of stored data, not of anything in the window, and nothing in the sources gives an alternative name for use inside the interface — the labels that go with these settings are "%", "Max Samples" and "Use for Rendering" instead, which is a labelling difference, not a second name for the instruction. (The first comes from a picture in the manual and the third from the manual's own words, both **documented**; the second comes from Cycles' interface file, and reading that as a label on screen is the **inferred** step noted above.)

One documented difference sits right next to the view layer setting and is worth flagging, because it is about the neighbouring control rather than about `use`. The manual page `https://docs.blender.org/manual/en/5.0/render/layers/view_layer.html`, read 2026-07-30, says of **Render Single Layer**:

> Note
>
> This option is ignored when rendering from the command line.

That note is attached to Render Single Layer, not to Use for Rendering, and the sources say nothing about `use` being ignored from the command line.

The sources do not otherwise state that these names behave differently inside and outside the interface. LABEL for the "no difference in the names" part: **inferred**, from the API documentation giving one name per property with no interface/background qualification.

**What running Blender 5.0 added, 2026-08-29. Label: observed.** Every run recorded under *What a running Blender 5.0 did* above was made from a terminal with `-b` and no window, and every name in this answer was set and read back that way. So these names are no longer merely *unmarked* as interface-only — they were exercised from outside and they worked. Three things the runs add. **`use_single_layer` really is dead from outside:** set to `True` on the launch line, both view layers still came out in the written file. **Where the instruction sits decides whether it does anything at all:** handed in before the file, a correct name reads back the value set and changes nothing about the render. And **a wrong name costs nothing a program could see:** the frame still renders, the file is still written, and Blender still exits 0. All of that was read off the Creator's Mac, macOS 26.6.2 on Apple silicon, and speaks for no other machine.

### Operating system

The names are the same on every operating system, as far as the sources go. The manual's command-line arguments page marks operating-system-specific arguments explicitly where they exist — "(Windows only)" on `-con/--start-console`, `/?` and `--qos`, "(Linux only)" on `--debug-ghost`, "(Windows & Linux only)" on `-r/--register`. None of `-P/--python`, `--python-text` or `--python-expr` carries such a marking, and none of the three property names is qualified by operating system anywhere I looked. LABEL: **inferred**, from the absence of an operating-system marking on entries that are marked elsewhere when they are OS-specific. The sources do not state it outright.

The manual does have three separate launch pages, for Linux, macOS and Windows (`https://docs.blender.org/manual/en/5.0/advanced/command_line/launch/`), but those concern how you reach the Blender executable, not what the settings are called.

**What running Blender 5.0 added, 2026-08-29. Label: observed, and bounded.** Every run recorded above was on **one computer** — the Creator's MacBook Pro, an Apple M1 Pro chip, macOS 26.6.2 (build 25G83, Darwin 25.6.0, arm64). **Nothing was rented, and nothing ran on Windows or on Linux.** So on **macOS on Apple silicon** these names are now exercised rather than reasoned about. On every other operating system nothing has changed: the reading above is still all there is, and **whether these names hold on Windows or on Linux is unknown**. A run on a machine of each is what would settle it, and no such machine was available for this round. Nobody may read the macOS result as covering the others.

---

## Record of where I looked

**Files and pages opened in full or in part**

- `Blender Sources/manual_5.0/advanced/command_line/arguments.txt` — read all 662 lines, every section. → `https://docs.blender.org/manual/en/5.0/advanced/command_line/arguments.html`
- `Blender Sources/manual_5.0/advanced/command_line/index.txt` → `.../advanced/command_line/index.html`
- `Blender Sources/manual_5.0/advanced/command_line/render.txt` → `.../advanced/command_line/render.html`
- `Blender Sources/manual_5.0/advanced/command_line/extension_arguments.txt` → `.../advanced/command_line/extension_arguments.html`
- `Blender Sources/manual_5.0/render/output/properties/format.txt` → `.../render/output/properties/format.html`
- `Blender Sources/manual_5.0/render/layers/view_layer.txt` → `.../render/layers/view_layer.html`
- `Blender Sources/manual_5.0/render/cycles/render_settings/sampling.txt` (searched, plus lines read around the samples entries) → `.../render/cycles/render_settings/sampling.html`
- `Blender Sources/manual_5.0/render/workbench/sampling.txt` → `.../render/workbench/sampling.html`
- `Blender Sources/manual_5.0/interface/controls/buttons/menus.txt` (the Copy Data Path entries) → `.../interface/controls/buttons/menus.html`
- Pictures: `Blender Sources/manual_5.0/_images/render_output_properties_format_panel.png` (the Format panel — shows Resolution X, Y, then a row labelled just `%` set to 100%) and `Blender Sources/manual_5.0/_images/render_layers_view-layer_panel.png` (the View Layer panel — Use for Rendering, Render Single Layer)
- `Blender Sources/api_5.0/bpy.types.RenderSettings.txt` → `https://docs.blender.org/api/5.0/bpy.types.RenderSettings.html`
- `Blender Sources/api_5.0/bpy.types.ViewLayer.txt` → `https://docs.blender.org/api/5.0/bpy.types.ViewLayer.html`
- `Blender Sources/api_5.0/bpy.types.SceneEEVEE.txt` → `https://docs.blender.org/api/5.0/bpy.types.SceneEEVEE.html`
- `Blender Sources/api_5.0/bpy.types.Scene.txt` → `https://docs.blender.org/api/5.0/bpy.types.Scene.html`
- `Blender Sources/api_5.0/bpy.types.SceneDisplay.txt` → `https://docs.blender.org/api/5.0/bpy.types.SceneDisplay.html`
- `Blender Sources/api_5.0/info_quickstart.txt` → `https://docs.blender.org/api/5.0/info_quickstart.html`
- `Blender Sources/api_5.0/info_tips_and_tricks.txt` → `https://docs.blender.org/api/5.0/info_tips_and_tricks.html`
- `Blender Sources/api_5.0/change_log.txt`, the "4.5 to 5.0" sections for `bpy.types.RenderSettings`, `bpy.types.Scene`, `bpy.types.SceneEEVEE`, `bpy.types.ViewLayer`, `bpy.types.CyclesRenderSettings` → `https://docs.blender.org/api/5.0/change_log.html`
- `Blender Sources/api_5.0/genindex-R.txt`, `genindex-T.txt`, `genindex-all.txt` (used as indexes, to confirm each name has exactly one owning type)
- `Blender Sources/release_notes_5.0/core.md` — the "Some command line flags were replaced" table and the "Command Line Argument for GPU VSync" section → `https://developer.blender.org/docs/release_notes/5.0/core/`
- `Blender Sources/release_notes_5.0/python_api.md` (searched for renames) → `https://developer.blender.org/docs/release_notes/5.0/python_api/`
- Every other 5.0 release-note file was searched, not read: `animation_rigging.md`, `assets.md`, `bugfixes.md`, `color_management.md`, `compositor.md`, `corrective_releases.md`, `cycles.md`, `eevee.md`, `geometry_nodes.md`, `grease_pencil.md`, `index.md`, `modeling.md`, `motion_tracking.md`, `navigation.md`, `physics.md`, `pipeline_io.md`, `rendering.md`, `sculpt.md`, `sequencer.md`, `user_interface.md`, `virtual_reality.md`
- `Blender Sources/cycles_5.0/properties.py` — the `CyclesRenderSettings` class, its `samples` and `preview_samples` entries, and its `register` method
- `Blender Sources/cycles_5.0/engine.py` — `_configure_argument_parser` and `_parse_command_line`, lines 1–60
- `Blender Sources/cycles_5.0/ui.py` — the lines that draw `samples` and `preview_samples`

**Terms searched, across the whole of `manual_5.0`, `api_5.0`, `release_notes_5.0` and `cycles_5.0` unless said otherwise**

- Candidate switch names, searched as literal text including the leading dashes: `--samples`, `--render-samples`, `--resolution`, `--scale`, `--view-layer`, `--render-layer`, `--percentage`, `--res-percent`. **No hits anywhere, in any of the four source sets.** This is what rules out Reading B.
- Property names: `resolution_percentage`, `taa_render_samples`, `samples`, `use_layer_samples`, `sample_offset`, `render_aa`, `viewport_aa`
- Written-out paths, searched against the manual: `resolution_percentage`, `taa_render_samples`, `cycles.samples`, `view_layer.use`, `scene.render.resolution`, `use_for_rendering`, `render_percentage`, `percentage_scale`. **No hits in the manual for any of them** — the manual does not print Python names for these settings, which is why the answer above rests on the API documentation and on Cycles' own files rather than on the manual.
- Interface wording: `Resolution %`, `Resolution Percentage`, `Percentage scale`, `Max Samples`, `Use for Rendering`, `Enable or disable rendering of this`
- Around the edges: `command line`, `command-line`, `argument`, `launch`, `renamed`, `rename`, `percent`, `sample`, `view layer`, `--python-expr`, `--background`, `argparse`, `add_argument`, `sys.argv`, `CyclesRenderSettings`
- File-name sweep of the manual for anything about the command line or launching, which is how the four `advanced/command_line/` pages were found.

**What I ruled out, and why**

- **Every other command-line switch in 5.0.** Read the whole arguments page. The render-related ones settle a scene name, a frame or frame range, a frame step, an output path, an engine, a thread count, an image format and a file extension. None touches resolution percentage, samples, or a view layer's on/off state.
- **`-c/--command` and its sub-commands.** Checked the sub-command page. It is extension package and repository management only.
- **Cycles' own launch switches.** Checked both the manual's description and Cycles' own argument parser in `engine.py`. Exactly two switches, neither about samples.
- **Environment variables.** Read the list at the end of the arguments page. They set directories for resources, scripts, configuration, data files, extensions, Python, a splash image, a colour configuration, and temporary files. None sets a render setting.
- **Release notes for 5.0.** Searched all twenty-three files for renames and for command-line changes. *(Count corrected 2026-08-29 by the citation check: the list above this one names twenty-three files, and the folder holds twenty-three. Nothing else changes.)* The only command-line changes recorded for 5.0 are that `--debug-cycles` became `--log cycles`, `--debug-ffmpeg` became `--log video`, and `--verbose` became `--log-level`, plus a new `--gpu-vsync`. None of the three settings appears. Quoting the table from `https://developer.blender.org/docs/release_notes/5.0/core/`, read 2026-07-30: "Some command line flags were replaced."
- **The 4.5-to-5.0 change log in the API documentation.** For `bpy.types.RenderSettings` it lists removals only, all of them bake-related (`bake_bias`, `bake_margin`, `bake_margin_type`, `bake_samples`, `bake_type`, `bake_user_scale`, `use_bake_clear`, `use_bake_lores_mesh`, `use_bake_multires`, `use_bake_selected_to_active`, `use_bake_user_scale`) — `resolution_percentage` is untouched. For `bpy.types.ViewLayer` it lists one addition, `bl_system_properties_get` — `use` is untouched. For `bpy.types.SceneEEVEE` it lists removals `gtao_distance` and `gtao_quality` — `taa_render_samples` is untouched. For `bpy.types.CyclesRenderSettings` it lists one addition, `volume_biased`, and one removal, `feature_set` — `samples` is untouched. So none of the three names changed in 5.0.
- **Other source folders present on disk.** `Blender Sources/` also holds 4.5, 5.1 and 5.2 copies. I did not read them; the question is about 5.0 only.

**I did not need the web, and did not use it.**

---

## Things a person with Blender 5.0 open could still settle by hand

**Two of the six items that stood here were run from a terminal on 2026-08-29**, and what came back is written up under *What a running Blender 5.0 did* above: whether the names work when handed in at launch (was item 5), and whether any dedicated switch exists in `blender --help` (6).

What is left is the four that asked for a tooltip to be read or a control right-clicked — one item now, because they are one job — plus the two things this Mac cannot reach.

1. **Read the names off a hovering tooltip, or copy them off the controls.** Switch on **Python Tooltips** in Preferences ‣ Interface, then hold the pointer still over the Percentage slider in Output Properties ‣ Format, over the render Samples field in Render Properties ‣ Sampling with Cycles and again with EEVEE, and over the *Use for Rendering* checkbox in View Layer Properties. Or right-click each and choose *Copy Full Data Path* and paste the result. Both need a hand holding a mouse on a control, and no command stands in for either. *What they were wanted for is settled above another way*: the running program's own label and description were read out for every one of these properties, and those are the two pieces of text a tooltip draws. A negative result is a tooltip or a pasted path showing a name other than the ones above.
2. **The same three names on an operating system other than macOS.** Every run above was on one Mac on Apple silicon, so Windows and Linux are untouched. Run the same launch-time instruction on a Windows machine and on a Linux machine and compare what is printed and what is written to disk. A negative result is any difference at all in a name. Needs a machine of each, and none was rented for this round.
3. **A sample-count name for a render engine other than Cycles, EEVEE and Workbench.** No such engine is installed on this Mac. With one installed, open a Python console inside Blender, type `bpy.context.scene.` and see what that engine added, then look through it for a sample count. A negative result — the engine adding nothing that carries one — is itself the answer for that engine.

---

2026-08-29 — Claude Opus 5 (Claude Code) — Settings, Blender 5.0

I did not start any helper agents for this pass. All of the reading above is mine, and the containment declaration at the top covers all of it.

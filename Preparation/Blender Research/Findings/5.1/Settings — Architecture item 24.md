# Settings — Architecture item 24 (Blender 5.1)

**Sources consulted, and the date:** Blender 5.1 manual (`Blender Sources/manual_5.1`), Blender 5.1 Python API documentation (`Blender Sources/api_5.1`), Blender 5.1 release notes (`Blender Sources/release_notes_5.1`), and Cycles' own files (`Blender Sources/cycles_5.1`). All four were taken on 2026-07-30, and that is my date read for everything quoted below.

I read only the question sheet and files inside `Blender Sources/`. I did not open any other part of this project.

---

## 17. When a render is started from outside and an instruction handed in at launch sets the resolution percentage, the sample count, or whether a view layer renders, what name does that instruction use for each of those three settings?

### The question was ambiguous, and here is the reading I answered

"An instruction handed in at launch" can mean two things.

- **Reading A (the one I answered):** the instruction is whatever is handed to Blender on the launch line that ends up setting the setting. The only such instruction I found in the sources is a Python one, because — as the record below shows — the documented launch options include no option of their own for any of these three settings. So the names asked for are the names that Python instruction uses. *(That "the only such instruction" step is **inferred**: it is drawn from finding nothing rather than from any sentence saying so.)*
- **Reading B:** the instruction is a dedicated launch option of Blender's own, in the same family as `-E` for the engine or `-s` for the start frame. Under that reading the answer is **not in the sources** for all three: the 5.1 argument list documents no such option, and I searched for one (record below).

Both readings are answered here. Reading A carries the substance.

### LABEL

- **documented** — for the three names themselves, and for the fact that a Python instruction can be handed in at launch.
- **not in the sources** — for a launch option of Blender's own that names any of these three settings (Reading B).
- **observed** — for everything taken from Cycles' own files, marked where it appears.
- **inferred** — for the four steps drawn from finding nothing rather than from a sentence: that a Python instruction is the only route (above), that those are the whole of what Cycles reads from the launch line, that a name after `--` is not Blender's, and the two standing lines below. Each is marked where it appears.

### The answer, Reading A

An instruction handed in at launch is a Python one. Blender 5.1 documents three ways to hand one in:

> `-P, --python <filepath>`
>
> Run the given Python script file.
>
> `--python-text <name>`
>
> Run the given Python script text block.
>
> `--python-expr <expression>`
>
> Run the given expression as a Python script.
>
> The expression may be a complete multi-line script;
> you are limited only by the platform's maximum argument length.

Source: manual, https://docs.blender.org/manual/en/5.1/advanced/command_line/arguments.html — read 2026-07-30.

The names such an instruction uses are these.

**1. Resolution percentage — the name is `resolution_percentage`.** It sits on a scene's render settings.

> `resolution_percentage`
>
> Percentage scale for render resolution (in [1, 32767], default 100)
>
> Type:
>
> int

Source: API documentation, https://docs.blender.org/api/5.1/bpy.types.RenderSettings.html — read 2026-07-30.

The API documentation shows the whole name written out, reached through a scene's `render`:

> `>>> bpy.data.scenes[0].render.resolution_percentage`
> `100`

Source: API documentation, https://docs.blender.org/api/5.1/info_quickstart.html — read 2026-07-30.

The step from a scene to its render settings is documented on the scene itself:

> `render`
>
> (readonly, never None)
>
> Type:
>
> `RenderSettings`

Source: API documentation, https://docs.blender.org/api/5.1/bpy.types.Scene.html — read 2026-07-30.

The same setting is called something different where a person clicks. The manual's Format panel calls it:

> `Percentage`
>
> Slider to reduce or increase the size of the rendered image relative to the Resolution values.
> This is useful for small test renders that have the same proportions as the final image.

Source: manual, https://docs.blender.org/manual/en/5.1/render/output/properties/format.html — read 2026-07-30. So the visible label is "Percentage" and the name the documentation gives the setting is `resolution_percentage`. They are not the same word. (Whether writing that name from a launch-time instruction takes effect is not claimed here — see *Not covered*.)

**2. Sample count — the name depends on the render engine.** There is no single name.

*Cycles:* the name is `samples`, on the scene's `cycles` settings. The API documentation writes the whole name out:

> `# Access the number of samples for the Cycles render engine.`
> `bpy.context.scene.cycles.samples`

Source: API documentation, https://docs.blender.org/api/5.1/info_api_reference.html — read 2026-07-30.

Cycles' own file declares that name, its kind, its default and its range:

> ```
>     samples: IntProperty(
>         name="Samples",
>         description="Number of samples to render for each pixel",
>         min=1, max=(1 << 24),
>         default=4096,
>     )
> ```

Source: `Blender Sources/cycles_5.1/properties.py`, lines 479–484, inside `class CyclesRenderSettings(bpy.types.PropertyGroup)` (which begins at line 378). Read 2026-07-30. *(Line range corrected 2026-08-29 by the citation check, which opened the file: the six quoted lines run 479 to 484, the closing bracket being 484. The words are unchanged.)* Label for this one: **observed** — I read the file, I did not run it.

The same file declares the name `cycles` on a scene, and declares it as pointing at this group:

> ```
>     @classmethod
>     def register(cls):
>         bpy.types.Scene.cycles = PointerProperty(
>             name="Cycles Render Settings",
>             description="Cycles render settings",
>             type=cls,
>         )
> ```

Source: `Blender Sources/cycles_5.1/properties.py`, lines 1101–1107. Read 2026-07-30. Label: **observed** — this is what the file declares. What a running Blender then has on a scene is a step beyond reading it, and I am not taking that step here; the documented route to the same name is the API quote above.

Cycles' preset file uses the same two-part name, which is a second sighting of it:

> ```
>         "cycles = bpy.context.scene.cycles"
> ```
> ```
>         "cycles.samples",
> ```

Source: `Blender Sources/cycles_5.1/presets.py`, lines 46 and 51. Read 2026-07-30. Label: **observed**.

*EEVEE:* the name is `taa_render_samples`, on the scene's `eevee` settings.

> `taa_render_samples`
>
> Number of samples per pixel for rendering (in [1, inf], default 64)
>
> Type:
>
> int

Source: API documentation, https://docs.blender.org/api/5.1/bpy.types.SceneEEVEE.html — read 2026-07-30.

> `eevee`
>
> EEVEE settings for the scene (readonly)
>
> Type:
>
> `SceneEEVEE`

Source: API documentation, https://docs.blender.org/api/5.1/bpy.types.Scene.html — read 2026-07-30.

*A third name, on a view layer rather than a scene:* a view layer carries its own sample count, and it is called `samples` there too.

> `samples`
>
> Override number of render samples for this view layer, 0 will use the scene setting (in [0, inf], default 0)
>
> Type:
>
> int

Source: API documentation, https://docs.blender.org/api/5.1/bpy.types.ViewLayer.html — read 2026-07-30. I include it because the question asks for a name and this is a second thing carrying that name; which of the two an instruction should set is not something I am answering here.

**3. Whether a view layer renders — the name is `use`, on the view layer itself.**

> `use`
>
> Enable or disable rendering of this View Layer (default True)
>
> Type:
>
> bool

Source: API documentation, https://docs.blender.org/api/5.1/bpy.types.ViewLayer.html — read 2026-07-30.

The view layers of a scene are reached through `view_layers`:

> `view_layers`
>
> (default None, readonly)
>
> Type:
>
> `ViewLayers[ViewLayer]`

Source: API documentation, https://docs.blender.org/api/5.1/bpy.types.Scene.html — read 2026-07-30.

Where a person clicks, this same switch is labelled differently again:

> `Use for Rendering`
>
> The active view layer will be used during rendering.

Source: manual, https://docs.blender.org/manual/en/5.1/render/layers/view_layer.html — read 2026-07-30. So the visible label is "Use for Rendering" and the name the documentation gives the setting is `use`.

**A near-miss worth naming, because its name looks right and it is not the one.** There is a separate setting called `use_single_layer`, and the sources say outright that it does nothing when the render comes from the command line:

> `use_single_layer`
>
> Only render the active layer. Only affects rendering from the interface, ignored for rendering from command line. (default False)

Source: API documentation, https://docs.blender.org/api/5.1/bpy.types.RenderSettings.html — read 2026-07-30. The manual says the same of the panel switch:

> `Render Single Layer`
>
> Only render the active view layer.
>
> Note
>
> This option is ignored when rendering from the command line.

Source: manual, https://docs.blender.org/manual/en/5.1/render/layers/view_layer.html — read 2026-07-30.

So of the two, `use_single_layer` is the one the sources state is ignored from the command line, and `use` carries no such note. I am not claiming from that silence that `use` does take effect from outside — only that `use` is the name the documentation gives to switching a view layer's rendering on or off, and that the near-miss is documented as dead there.

### The answer, Reading B

**Not in the sources.** Blender 5.1's list of launch options contains no option that names a resolution percentage, a sample count, or whether a view layer renders. The Render Options it does list are `-b/--background`, `-a/--render-anim`, `-S/--scene`, `-f/--render-frame`, `-s/--frame-start`, `-e/--frame-end`, `-j/--frame-jump`, `-o/--render-output`, `-E/--engine`, `-t/--threads` — that list is my own summary of the page's Render Options section, not words quoted from it; the Cycles options it lists are exactly two, and these are quoted:

> `--cycles-device <device>`
>
> Set the device used for rendering.
> Valid options are: CPU CUDA OPTIX HIP ONEAPI METAL.

> `--cycles-print-stats`
>
> Log statistics about render memory and time usage.

Source: manual, https://docs.blender.org/manual/en/5.1/advanced/command_line/arguments.html — read 2026-07-30.

Cycles' own file declares those same two, and declares no third in the function that sets them up:

> ```
>     parser.add_argument("--cycles-print-stats",
>                         help="Print rendering statistics to stderr",
>                         action='store_true')
>     parser.add_argument("--cycles-device",
> ```

Source: `Blender Sources/cycles_5.1/engine.py`, lines 13–16, inside `_configure_argument_parser()`. Read 2026-07-30. Label: **observed** for the two declarations themselves — I am reading the declared list of arguments, not running the program. (The quote stops part-way through the second declaration.) Going from those two declarations to *those two are the whole of what Cycles reads from the launch line* is a further step, and I label that one **inferred**: it rests on the search below finding no other place in Cycles that reads the launch line, not on any sentence saying so.

One further route exists but supplies no name of Blender's own:

> `--`
>
> End option processing, following arguments passed unchanged. Access via Python's `sys.argv`.

Source: manual, https://docs.blender.org/manual/en/5.1/advanced/command_line/arguments.html — read 2026-07-30. The quoted words say those arguments are passed unchanged and reached from Python. That they therefore carry no name of Blender's own is a step I am supplying, so label it **inferred**.

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

- **Reading B was answered *not in the sources*, and that answer is untouched.** It is a statement about Blender 5.1's documentation, and it stays exactly as written. **What the runs add is a different claim about a different thing:** the built program's own help listing, all 498 lines of it, carries no switch for a resolution percentage, a sample count or a view layer either, and neither does Cycles' own set of launch arguments. So the sources and the program agree, and the answer's inferred step from one to the other now has a second leg under it.
- The answer gives the names and takes no view on whether writing them from a launch-time instruction takes effect. **Observed beside it:** in Blender 5.1.2 all of them do, and the ordering point the answer would not extend into a claim holds — before the file, the instruction reads back correctly and changes nothing.
- The *Not covered* bullet about `render_aa` stands as a statement about the looking. **Observed beside it:** in 5.1.2 `scene.display.render_aa` is reachable from a launch instruction, takes text rather than a number, and changes the picture Workbench writes. Whether it counts as a sample count is still the judgement this answer declines to make.
- The answer's near-miss finding about `use_single_layer` — that the sources say it is ignored from the command line — **was put to the program and holds**: set to `True` from the launch line, both view layers still came out.
- **Still open:** what any render engine other than Cycles, EEVEE and Workbench calls its sample count. None is installed on this Mac.

---

### Not covered

- I have given the names. I have not said whether setting any of them from a launch-time Python instruction actually takes effect on the render that follows, nor when in the launch sequence it would have to be done. The manual does state that "Arguments are executed in the order they are given" (same address as above), but I have not worked through what that means for these three, and I am not claiming it.
- The sample count has no single name. I have given three (`scene.cycles.samples`, `scene.eevee.taa_render_samples`, `view_layer.samples`). Engines other than Cycles and EEVEE are not covered at all. I did not search the sources for which render engines 5.1 documents, and I did not search for a sample count belonging to any engine besides those two — so this is a gap in my looking, not a silence I established. An engine added by an add-on could also name its own sample count anything.
- I have not covered which of a scene's view layers an instruction would be acting on, or how a particular one is picked out by name. I only show that `view_layers` is the collection and `use` is the switch on each member.
- The names are the ones the documentation states for 5.1. I have not checked them against any other version, and take no view on whether they changed.
- **This answer never looked at the Workbench engine's anti-aliasing setting, `render_aa`.** *(Added 2026-08-29 by the citation check.)* Nothing above searched for it, so its absence here is a gap in the looking and not an absence in 5.1. The 5.1 API documentation carries it: "render_aa / Method of anti-aliasing when rendering final image (default '8')" with the choices `OFF`, `FXAA`, `5`, `8`, `11`, `16`, `32` — Python API documentation, https://docs.blender.org/api/5.1/bpy.types.SceneDisplay.html, read 2026-07-30, **documented**. It is reached from a scene under the name `display`: "display / Scene display settings for 3D viewport (readonly) / Type: / SceneDisplay" — https://docs.blender.org/api/5.1/bpy.types.Scene.html, read 2026-07-30, **documented**. Whether that setting counts as "a sample count" is the same judgement the question leaves open elsewhere, and I take no view on it; what matters here is that 5.1 has the name, so this document's silence must not be read as a difference from the other versions.
- I did not open the images folder for this question. The names in question are text in the API documentation, not something a screenshot would settle, and the panel labels I quote are given in words in the manual.

### Outside the interface

**It holds, and the step to that is mine rather than a source's — label this line inferred.** The sources nowhere say that these three names change when Blender is driven from outside. What they do say is where the names come from: the API documentation and Cycles' own files, neither of which mentions a window, and a list of launch options that exists for starting Blender without one. From that I take it that the names are the same either way. No sentence states it.

Two places where inside and outside genuinely differ, both stated by the sources rather than inferred:

- `use_single_layer` is documented as "ignored for rendering from command line" (API documentation, `bpy.types.RenderSettings`) and "ignored when rendering from the command line" (manual, view layer page). So that name is live inside the interface and dead outside it. The name `use` on the view layer carries no such note.
- The manual describes running with no display at all, which is the case where an instruction at launch is the only way in: "One advantage of using the command line is that we do not need a graphical display (no need for X server on Linux for example) and consequently we can render via a remote shell (typically SSH)." Source: manual, https://docs.blender.org/manual/en/5.1/advanced/command_line/render.html — read 2026-07-30.

Beyond those two, the sources do not say the names themselves behave differently inside and outside, and I am not guessing.

**What running Blender 5.1 added, 2026-08-29. Label: observed.** Every run recorded under *What a running Blender 5.1 did* above was made from a terminal with `-b` and no window, and every name in this answer was set and read back that way. So these names are no longer merely *unmarked* as interface-only — they were exercised from outside and they worked. Three things the runs add. **`use_single_layer` really is dead from outside:** set to `True` on the launch line, both view layers still came out in the written file. **Where the instruction sits decides whether it does anything at all:** handed in before the file, a correct name reads back the value set and changes nothing about the render. And **a wrong name costs nothing a program could see:** the frame still renders, the file is still written, and Blender still exits 0. All of that was read off the Creator's Mac, macOS 26.6.2 on Apple silicon, and speaks for no other machine.

### Operating system

**Not stated in the sources**, and the record of looking for it is here. The sources do not say that any of these three names differs by operating system, and they do not say it holds on every one either. The pages I quote them from carry no operating-system note, whereas the same argument list marks options that are limited — `-con, --start-console` is marked "(Windows only)", `--no-window-frame` "(Linux only)", `--qos <level>` "(Windows only)", `--debug-ghost` "(Linux only)", and `/?` "(Windows only)". None of those marks is attached to `-P/--python`, `--python-text` or `--python-expr`, nor to anything in the API pages the three names come from. Source: manual, https://docs.blender.org/manual/en/5.1/advanced/command_line/arguments.html — read 2026-07-30.

One operating-system difference is stated for a neighbouring matter, and I note it only because it touches how a Python instruction is handed in at launch: `--python-expr` is documented as limited by "the platform's maximum argument length" (same address). That is a limit on how long the instruction may be, not on what the settings are called.

The sources do not say more than that.

**What running Blender 5.1 added, 2026-08-29. Label: observed, and bounded.** Every run recorded above was on **one computer** — the Creator's MacBook Pro, an Apple M1 Pro chip, macOS 26.6.2 (build 25G83, Darwin 25.6.0, arm64). **Nothing was rented, and nothing ran on Windows or on Linux.** So on **macOS on Apple silicon** these names are now exercised rather than reasoned about. On every other operating system nothing has changed: the reading above is still all there is, and **whether these names hold on Windows or on Linux is unknown**. A run on a machine of each is what would settle it, and no such machine was available for this round. Nobody may read the macOS result as covering the others.

### Record of where I looked

**Files searched, whole-folder, across `manual_5.1`, `api_5.1`, `release_notes_5.1` and `cycles_5.1`:**

Terms searched for, and what came back:

- `--samples`, `--resolution`, `--view-layer`, `--percentage`, `--render-samples`, `--scale` — **no hits in any of the four folders**: not in the manual, not in the API documentation, not in the release notes, not in Cycles' own files. This is the search that makes Reading B's "not in the sources" a real answer rather than a blank.
- `resolution_percentage` — hits in `api_5.1/info_quickstart.txt`, `api_5.1/bpy.types.RenderSettings.txt`, `api_5.1/bpy.types.RenderEngine.txt`, the API index files, and `cycles_5.1/ui.py`. **No hit anywhere in the manual**, including the command line pages.
- `resolution-percentage` (hyphenated, the shape a launch option would take) — no hits.
- `taa_render_samples` — hits only in `api_5.1/bpy.types.SceneEEVEE.txt` and the API index files.
- `cycles.samples` — hits in `api_5.1/info_api_reference.txt` and `cycles_5.1/presets.py`.
- `samples` within `cycles_5.1/properties.py` — read every hit; the scene-level one at line 479 is the render sample count, the rest are viewport, adaptive, guiding, volume and pass settings.
- `python-expr`, `python-text`, `render-frame` — each hits only `manual_5.1/advanced/command_line/arguments.txt`, confirming that file is the single place launch options are listed.
- `argparse`, `add_argument`, `sys.argv`, `cycles-device`, `print_stats` across `cycles_5.1` — hits only in `engine.py` lines 9–41. Nothing else in Cycles reads the launch line.
- `command line` / `command-line` / `samples` across `release_notes_5.1` — one hit only, `core.md` line 17, about `--python-use-user-env`, which concerns Python's own module search and names none of the three settings.

**Pages opened in full and what they settled:**

- `manual_5.1/advanced/command_line/arguments.txt` → https://docs.blender.org/manual/en/5.1/advanced/command_line/arguments.html — read end to end. It is where I establish that no option of Blender's own names any of the three settings, and where the three Python options come from. *(Corrected 2026-08-29 by the citation check. This line used to read "This is the complete list of launch options", stated flat — an absence established by searching, written as fact, in the one part of the document meant to record looking rather than to claim. The completeness is not mine to assert, but a source does assert it:* "See Command Line Arguments / for a full list of arguments / (for example to specify which scene to render, the end frame number, etc.), or simply run: / blender --help" *— manual, https://docs.blender.org/manual/en/5.1/advanced/command_line/render.html, read 2026-07-30, **documented**. So the step from that page to "no such option exists" is one step, and it keeps the **inferred** label the body already gives it.)*
- `manual_5.1/advanced/command_line/index.txt` → https://docs.blender.org/manual/en/5.1/advanced/command_line/index.html — read for its table of contents, to be sure the argument list has no second half elsewhere. It lists Render, Cycles Render, Format, Animation Playback, Window, Python, Network, Logging, Debug, GPU, Misc, Sub-Commands, Extensions, and the rendering workflow pages. Nothing further.
- `manual_5.1/advanced/command_line/render.txt` → https://docs.blender.org/manual/en/5.1/advanced/command_line/render.html — read end to end. Worked examples of rendering from outside. Uses only `-b`, `-f`, `-o`, `-F`, `-a`, `-E`, `-s`, `-e`, `-t`. None of the three settings appears.
- `manual_5.1/advanced/command_line/extension_arguments.txt` → https://docs.blender.org/manual/en/5.1/advanced/command_line/extension_arguments.html — opened because the argument list mentions `-c, --command <command>` and I could not rule out a render sub-command without looking. Every sub-command there is about extension packages and repositories (`list`, `sync`, `update`, `install`, `install-file`, `remove`, `repo-list`, `repo-add`, `repo-remove`, `build`, `validate`). **Ruled out:** there is no render sub-command, so no setting names hide there.
- `api_5.1/bpy.types.RenderSettings.txt` → https://docs.blender.org/api/5.1/bpy.types.RenderSettings.html — for `resolution_percentage` and for `use_single_layer`.
- `api_5.1/bpy.types.Scene.txt` → https://docs.blender.org/api/5.1/bpy.types.Scene.html — for `render`, `eevee` and `view_layers`, the steps from a scene to each setting.
- `api_5.1/bpy.types.ViewLayer.txt` → https://docs.blender.org/api/5.1/bpy.types.ViewLayer.html — for `use` and for `samples`.
- `api_5.1/bpy.types.SceneEEVEE.txt` → https://docs.blender.org/api/5.1/bpy.types.SceneEEVEE.html — for `taa_render_samples`.
- `api_5.1/info_quickstart.txt` → https://docs.blender.org/api/5.1/info_quickstart.html — for the fully written-out `bpy.data.scenes[0].render.resolution_percentage`.
- `api_5.1/info_api_reference.txt` → https://docs.blender.org/api/5.1/info_api_reference.html — for the fully written-out `bpy.context.scene.cycles.samples`.
- `api_5.1/bpy.types.Context.txt` → https://docs.blender.org/api/5.1/bpy.types.Context.html — checked that `scene` and `view_layer` are documented members, so that `bpy.context.scene…` is a documented way to write the names and not one I supplied. Both are there.
- `api_5.1/bpy.context.txt` → https://docs.blender.org/api/5.1/bpy.context.html — opened, and it is only fourteen lines in this snapshot, describing context access in general and naming no members. **Ruled out** as a source for the names; `bpy.types.Context` carries them instead.
- `manual_5.1/render/output/properties/format.txt` → https://docs.blender.org/manual/en/5.1/render/output/properties/format.html — for the visible label "Percentage".
- `manual_5.1/render/layers/view_layer.txt` → https://docs.blender.org/manual/en/5.1/render/layers/view_layer.html — for the visible labels "Use for Rendering" and "Render Single Layer", and the note about the command line.
- `manual_5.1/render/layers/introduction.txt` → https://docs.blender.org/manual/en/5.1/render/layers/introduction.html — read looking for any other name for switching a view layer off. It covers collections in the Outliner and compositing, and gives no further name.
- `cycles_5.1/properties.py` — lines 378–1112 read around every `samples` hit; lines 1479–1556 read to check whether a view layer's Cycles settings carry their own sample count. **Ruled out:** in 5.1 that group (`CyclesRenderLayerSettings`) holds passes and denoising only, no sample count. The view layer's sample count is the one on `bpy.types.ViewLayer` itself.
- `cycles_5.1/engine.py` lines 1–50 — read in full, to see the whole of Cycles' launch-line handling.
- `cycles_5.1/presets.py` around lines 46–56 — for the second sighting of `cycles.samples`.

**What I did not do:** I did not go to the web, and did not need to.

---

## Things a person with Blender open could still settle by hand

**Six of the eight items that stood here were run from a terminal on 2026-08-29**, and what came back is written up under *What a running Blender 5.1 did* above: that no launch option exists for the three settings (was item 1), that a Python instruction at launch really does change the render (2), the Cycles sample count (3), the EEVEE sample count (4), switching a view layer off (5), and the `use_single_layer` near miss (6). Every one of them came out the way the item expected, and none produced the negative result it named.

Three are left, and none can be reached from a terminal on this Mac.

1. **Read the names off a hovering tooltip, or copy them off the controls.** Switch on **Python Tooltips** in Preferences ‣ Interface, then hold the pointer still over the Percentage slider in Output Properties ‣ Format, over the render Samples field in Render Properties ‣ Sampling with Cycles and again with EEVEE, and over the *Use for Rendering* checkbox in View Layer Properties. Or right-click each and choose *Copy Full Data Path* and paste the result. Both need a hand holding a mouse on a control, and no command stands in for either. *What they were wanted for is settled above another way*: the running program's own label and description were read out for every one of these properties, and those are the two pieces of text a tooltip draws. A negative result is a tooltip or a pasted path showing a name other than the ones above.
2. **A sample-count name for a render engine other than Cycles, EEVEE and Workbench.** No such engine is installed on this Mac. With one installed, open a Python console inside Blender, type `bpy.context.scene.` and see what that engine added, then look through it for a sample count. A negative result — the engine adding nothing that carries one — is itself the answer for that engine.
3. **The same three names on an operating system other than macOS.** Every run above was on one Mac on Apple silicon, so Windows and Linux are untouched. Run the same launch-time instruction on a Windows machine and on a Linux machine and compare what is printed and what is written to disk. A negative result is any difference at all in a name. Needs a machine of each, and none was rented for this round.

---

2026-08-29 — Claude Opus 5 (Claude Code) — Settings, Blender 5.1

I used no helper agents for this pass. Everything above I read myself, from the question sheet and from `Blender Sources/` only.

# Settings — Architecture item 24, second pass (Blender 5.2)

Sources consulted, all on **2026-07-30** (the snapshot date of the copies I was given): Blender's manual for 5.2 (`Blender Sources/manual_5.2`, including its `_images` folder), Blender's Python API documentation for 5.2 (`Blender Sources/api_5.2`), Blender's 5.2 release notes (`Blender Sources/release_notes_5.2`, all twenty-one subject files), and Cycles' own files for 5.2 (`Blender Sources/cycles_5.2`).

I read only the question sheet and files inside `Blender Sources/`. I did not open any other part of this project.

I ran no helper agents. Everything below I read myself.

---

## 17. When a render is started from outside and an instruction handed in at launch sets the resolution percentage, the sample count, or whether a view layer renders, what name does that instruction use for each of those three settings?

### The question was ambiguous, and here is how I read it

"An instruction handed in at launch" can mean two things.

- **Reading A — one of Blender's own launch switches.** The name would then be a switch like `-o` or `--render-output`.
- **Reading B — a Python instruction handed in at launch**, using one of the switches that exist for exactly that (`--python-expr`, `--python`, `--python-text`). The name would then be the name Python uses for the setting.

I answered **Reading B**, because under Reading A my sources give me no name to report: the manual's list of launch switches holds none for any of the three settings. I give that absence in full below, since it is what forces the choice. If Reading A was the one meant, the answer is "the sources name no such switch for any of the three".

The question asks for the names and nothing else. I have not gone into what the settings do, what values they take, or whether the render honours them.

---

### Reading A — Blender's own launch switches: the sources name no such switch for any of the three

**Label: not in the sources** for the absence itself — no page states outright that no such switch exists. The record of looking is below and in the closing section: I read the manual's Command Line Arguments page end to end, searched all four sources for likely switch names, and read Cycles' own argument parser.

*(Corrected 2026-08-29 by the citation check, which opened the page. This paragraph used to say that nothing I read says the arguments page is the complete list of switches, and that its completeness was a step I supplied with nothing behind it. That was wrong, and wrong in the over-careful direction: a page recorded below as read in full does say it —* "See Command Line Arguments / for a full list of arguments / (for example to specify which scene to render, the end frame number, etc.), or simply run: / blender --help" *— manual, https://docs.blender.org/manual/en/5.2/advanced/command_line/render.html, read 2026-07-30, **documented**.)*

So the standing here is the ordinary one, not a weaker one. The manual calls that page a full list of arguments; the page carries no switch for any of the three; going from there to "there is no such switch" is **one** step, which is that a page the manual calls full really is full. **Label for that step: inferred.**

The manual's Command Line Arguments page sets out the switches section by section. Its render-related switches are, in full:

> "-b, --background … -a, --render-anim … -S, --scene <name> … -f, --render-frame <frame> … -s, --frame-start <frame> … -e, --frame-end <frame> … -j, --frame-jump <frames> … -o, --render-output <path> … -E, --engine <engine> … -t, --threads <threads>"

and the only two Cycles adds:

> "--cycles-device <device>"
> "--cycles-print-stats"

Source: Blender manual — https://docs.blender.org/manual/en/5.2/advanced/command_line/arguments.html — read 2026-07-30.

None of those names a resolution percentage, a sample count, or a view layer. There is no `--render-percentage`, no `--samples`, no view-layer switch on that page.

Cycles' own files agree, and here I can see it rather than read about it. **Label for this paragraph: observed.** Cycles builds its launch-time argument parser in one function, and that function declares exactly two arguments. (That it is the only such function is not something the file states — it rests on my search of Cycles' files for `add_argument`, `argv` and `parser`, recorded in the closing section, turning up no other.)

> ```
> parser.add_argument("--cycles-print-stats",
>                     help="Print rendering statistics to stderr",
>                     action='store_true')
> parser.add_argument("--cycles-device",
>                     help="Set the device to use for Cycles, overriding user preferences and the scene setting."
> ```

File: `Blender Sources/cycles_5.2/engine.py`, in the function `_configure_argument_parser`, lines 13–20. Read 2026-07-30.

The two switches that do exist for handing in an instruction are these:

> "-P, --python <filepath> — Run the given Python script file."
> "--python-text <name> — Run the given Python script text block."
> "--python-expr <expression> — Run the given expression as a Python script. The expression may be a complete multi-line script; you are limited only by the platform's maximum argument length."

Source: Blender manual — https://docs.blender.org/manual/en/5.2/advanced/command_line/arguments.html — read 2026-07-30.

---

### Reading B — the names a Python instruction uses

**Label for joining the two halves: inferred.** The step, written out: the manual documents that `--python-expr` "Run[s] the given expression as a Python script"; the API documentation and Cycles' own files document the names Python uses for these settings. Nothing in my sources shows a worked example of a launch-time expression setting one of these three. So the individual names below are documented or observed as stated, but the sentence "an instruction handed in at launch uses *this* name" is me joining two documented facts, and is inferred.

#### (a) The resolution percentage — `resolution_percentage`

**Label: documented.**

> "resolution_percentage
>
> Percentage scale for render resolution (in [1, 32767], default 100)
>
> Type:
>
> int"

Source: Python API documentation — https://docs.blender.org/api/5.2/bpy.types.RenderSettings.html — read 2026-07-30.

It sits on a scene's render settings, which are reached by the name `render`:

> "render
>
> (readonly, never None)
>
> Type:
>
> RenderSettings"

Source: Python API documentation — https://docs.blender.org/api/5.2/bpy.types.Scene.html — read 2026-07-30.

The API documentation spells the whole name out in one line, in its quick start page:

> ">>> bpy.data.scenes[0].render.resolution_percentage
> 100"

Source: Python API documentation — https://docs.blender.org/api/5.2/info_quickstart.html — read 2026-07-30.

The same full name appears in a second place, in the example add-on render engine the API documentation gives:

> "scale = scene.render.resolution_percentage / 100.0"

Source: Python API documentation — https://docs.blender.org/api/5.2/bpy.types.RenderEngine.html — read 2026-07-30.

And Cycles itself uses that exact name. **Label: observed.**

> "render_scale = scene.render.resolution_percentage / 100.0"

File: `Blender Sources/cycles_5.2/ui.py`, line 399. Read 2026-07-30.

For anyone matching this to what they see on screen: the manual calls the control "Percentage", under the Format panel —

> "Percentage
>
> Slider to reduce or increase the size of the rendered image relative to the Resolution values."

Source: Blender manual — https://docs.blender.org/manual/en/5.2/render/output/properties/format.html — read 2026-07-30.

— and the manual's picture of that panel (`_images/render_output_properties_format_panel.png`, shown on the same page) labels the field simply `%`, sitting under "Resolution X" and "Y", showing `100%`. **Label: documented**, from the picture.

#### (b) The sample count — the name belongs to the render engine, so there are three answers

There is no one name. The manual states plainly that the engines each have their own settings:

> "Blender includes three render engines with different strengths: EEVEE is a physically based realtime renderer. Cycles is a physically based path tracer. Workbench is designed for layout, modeling and previews. … Each renderer has its own render settings to control render quality and performance."

Source: Blender manual — https://docs.blender.org/manual/en/5.2/render/introduction.html — read 2026-07-30.

**Cycles — `samples`, on the group named `cycles`.** **Label: observed** (Cycles is an add-on; its settings are not in the API documentation, so this comes from its files).

> ```
>     samples: IntProperty(
>         name="Samples",
>         description="Number of samples to render for each pixel",
>         min=1, max=(1 << 24),
>         default=4096,
>     )
> ```

File: `Blender Sources/cycles_5.2/properties.py`, lines 485–490, inside `class CyclesRenderSettings(bpy.types.PropertyGroup)` (which begins at line 384). Read 2026-07-30.

The group is attached to a scene under the name `cycles`:

> ```
>     @classmethod
>     def register(cls):
>         bpy.types.Scene.cycles = PointerProperty(
>             name="Cycles Render Settings",
>             description="Cycles render settings",
>             type=cls,
>         )
> ```

File: `Blender Sources/cycles_5.2/properties.py`, lines 1140–1146. Read 2026-07-30.

So the full name is `scene.cycles.samples`. Cycles' own interface code confirms the pairing — `cscene` in the panel that draws this control is bound two lines above it, `scene = context.scene` then `cscene = scene.cycles` (`Blender Sources/cycles_5.2/ui.py`, lines 298–299, inside the `draw` method whose body the line below sits in):

> "            col.prop(cscene, \"samples\", text=\"Max Samples\")"

File: `Blender Sources/cycles_5.2/ui.py`, line 313 (and again at line 316 with the label `Samples`). Read 2026-07-30. Note that the on-screen label is "Max Samples" or "Samples" depending on a neighbouring setting, but the name is `samples` either way.

*(Corrected 2026-08-29 by the citation check, which opened the file. The link in this chain used to point at `ui.py` line 105 without quoting it. Line 105 is inside a different function (`use_cpu`), so it did not establish what `cscene` means at line 313. The binding that does is in the same `draw` method as line 313, and here it is, quoted:* `        scene = context.scene` */* `        cscene = scene.cycles` *— `Blender Sources/cycles_5.2/ui.py`, lines 298–299, read 2026-07-30. **Label: observed.** The chain no longer rests on a bare reference.)*

**EEVEE — `taa_render_samples`, on the group named `eevee`.** **Label: documented.**

> "taa_render_samples
>
> Number of samples per pixel for rendering (in [1, inf], default 64)
>
> Type:
>
> int"

Source: Python API documentation — https://docs.blender.org/api/5.2/bpy.types.SceneEEVEE.html — read 2026-07-30.

Reached from a scene under the name `eevee`:

> "eevee
>
> EEVEE settings for the scene (readonly)
>
> Type:
>
> SceneEEVEE | None"

Source: Python API documentation — https://docs.blender.org/api/5.2/bpy.types.Scene.html — read 2026-07-30.

Putting those two together, the full name is `scene.eevee.taa_render_samples`. **Label for that joining: inferred** — each half is documented above, but no page I read spells the whole name out in one line, so joining them is a step I supplied. (For the resolution percentage I did find the whole name spelled out, and for the Cycles one Cycles' own interface code pairs them; here I found neither.)

Beside it sits `taa_samples`, "Number of samples, unlimited if 0 (in [0, inf], default 16)" (same page) — that is the viewport one, not the render one; the render one carries `render` in its name. On screen the manual calls the render one simply "Samples", under a heading "Render": "Samples — The number of samples to use in the final render." (Blender manual — https://docs.blender.org/manual/en/5.2/render/eevee/render_settings/sampling.html — read 2026-07-30).

**Workbench — the nearest name I found is `render_aa`, on the group named `display`, and it is not a number.** **Label: documented** for `render_aa` itself, quoted below.

**That Workbench has no sample count at all is a separate claim, and it carries a weaker label: not in the sources.** The record of looking is the index sweep set out in the closing section — I went through every indexed name containing `samples` that belongs to a scene or a view layer and found none that is Workbench's. That is a search that came up empty, not a page saying there is none.

> "render_aa
>
> Method of anti-aliasing when rendering final image (default '8')
> …
> Type:
>
> Literal['OFF', 'FXAA', '5', '8', '11', '16', '32']"

Source: Python API documentation — https://docs.blender.org/api/5.2/bpy.types.SceneDisplay.html — read 2026-07-30. Reached from a scene under the name `display` ("display — Scene display settings for 3D viewport (readonly) — Type: SceneDisplay | None", https://docs.blender.org/api/5.2/bpy.types.Scene.html). So the full name is `scene.display.render_aa`, and what it takes is one of those seven words, not a count.

**One further sample-count name, since it also bears on a view layer.** A view layer can carry its own override, under the name `samples`:

> "samples
>
> Override number of render samples for this view layer, 0 will use the scene setting (in [0, inf], default 0)
>
> Type:
>
> int"

Source: Python API documentation — https://docs.blender.org/api/5.2/bpy.types.ViewLayer.html — read 2026-07-30.

I report the name only. Whether and how a given engine uses it is not what was asked.

#### (c) Whether a view layer renders — `use`

**Label: documented.**

> "use
>
> Enable or disable rendering of this View Layer (default True)
>
> Type:
>
> bool"

Source: Python API documentation — https://docs.blender.org/api/5.2/bpy.types.ViewLayer.html — read 2026-07-30.

The view layers of a scene are reached under the name `view_layers`:

> "view_layers
>
> (default None, readonly)
>
> Type:
>
> ViewLayers[ViewLayer]"

Source: Python API documentation — https://docs.blender.org/api/5.2/bpy.types.Scene.html — read 2026-07-30.

Putting those two together, the full name is `scene.view_layers["<name of the layer>"].use`. **Label for that joining: inferred** — both halves are documented above, but no page I read spells the whole name out in one line, and picking a layer out of the collection by its name is a further step I supplied.

On screen the manual calls this control "Use for Rendering":

> "Use for Rendering
>
> The active view layer will be used during rendering."

Source: Blender manual — https://docs.blender.org/manual/en/5.2/render/layers/view_layer.html — read 2026-07-30. The manual's picture of that panel (`_images/render_layers_view-layer_panel.png`, shown on the same page) shows a checkbox labelled "Use for Rendering", ticked, with "Render Single Layer" beneath it, unticked. **Label: documented**, from the picture.

**Two neighbouring names I checked and ruled out**, because they are easy to mistake for this one:

- `use_single_layer` — "Only render the active layer. Only affects rendering from the interface, ignored for rendering from command line. (default False)" (Python API documentation — https://docs.blender.org/api/5.2/bpy.types.RenderSettings.html — read 2026-07-30). That is "Render Single Layer" in the panel, a different control. The manual repeats the point: "This option is ignored when rendering from the command line." (https://docs.blender.org/manual/en/5.2/render/layers/view_layer.html).
- `layer`, an option of the render command — "layer (str) – Render Layer, Single render layer to re-render (used only when animation is disabled) (optional, never None)" (Python API documentation — https://docs.blender.org/api/5.2/bpy.ops.render.html — read 2026-07-30). That picks one layer to re-render; it does not turn a layer on or off. The full command there is `bpy.ops.render.render(*, animation=False, write_still=False, use_viewport=False, use_sequencer_scene=False, layer='', scene='', frame_start=0, frame_end=0)` — same page — and it has no option for a resolution percentage or a sample count.

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

- **Standing line 1 keeps its *not in the sources* label.** It is a reading of what Blender's documentation does and does not say, and no run can change that. **The observed answer sits beside it, as a different claim:** every one of the sixty runs above was made with `-b` and no window, and every name in this answer was set and read back that way, so these names were exercised from outside and worked.
- **Reading A's absence keeps its *not in the sources* label too.** **Observed beside it:** the built program's own help listing carries no such switch either, `blender -b --help` prints the same listing with one extra banner line, and `-c help` lists three sub-commands — `asset_listing`, `extension` and `maketx` — whose own help texts name no resolution percentage, sample count or view layer. And a made-up switch is not silently ignored: `--render-percentage 50` was taken for a file name, nothing rendered, and Blender exited 1.
- The answer says **that Workbench has no sample count at all is *not in the sources***, resting on an index sweep. That label stands. **What the runs add is narrower than the claim:** `scene.display.render_aa` works from a launch instruction and changes the picture, and a photograph of Workbench's Render Properties in 5.2.1 shows one `Sampling` panel with `Render` reading `8 Samples` and `Viewport` reading `Single Pass Anti-…`, and no plain number-of-samples field. That is one panel photographed, not a sweep of the running program, so it does not turn the absence into an observed one.
- The `cscene` chain repaired by the citation check ends in the claim that the panel says `Max Samples`. **A photograph of a running 5.2.1 shows it does**, and a launch instruction setting `scene.cycles.samples = 7` made that same field read `7`.
- Every joined dotted path labelled inferred here was written out in a launch-time instruction and reached its setting.
- The answer leaves the autoexec switches unexamined. **No run touched them**, and they stay unexamined; every run above used the ordinary form with no such switch and Python ran.

---

### The two lines that go on every answer

**1. Outside the interface.** The question itself is about being driven from outside, and every name above is quoted from a page that says nothing about how Blender was started. **So whether these names hold the same when Blender is driven from outside its own window is not stated either way in my sources — label: not in the sources**, with the record of looking being the whole search set out in the closing section. Two things sit nearby and are worth stating precisely:

- `use_single_layer` is documented as behaving differently: "Only affects rendering from the interface, ignored for rendering from command line" (https://docs.blender.org/api/5.2/bpy.types.RenderSettings.html, read 2026-07-30). That is a neighbour of the third setting, not the third setting.
- Nothing in my sources says `resolution_percentage`, the engines' sample counts, or `ViewLayer.use` are named differently, or behave differently as regards their names, when Blender is driven from outside. **Label for that: not in the sources** — the documentation states the names once and does not qualify them by how Blender was started.

**2. Operating system.** The sources say nothing that makes any of these names differ by operating system. The API documentation and Cycles' files carry no operating-system qualifier on any of them. The one place the manual does mark switches as operating-system specific — for instance "-con, --start-console … (Windows only)" and "--qos <level> … (Windows only)" (https://docs.blender.org/manual/en/5.2/advanced/command_line/arguments.html, read 2026-07-30) — shows that the manual marks such things when they apply, and it marks none of the three settings here. **Label: not in the sources**, with that supporting observation about how the manual marks things noted as **inferred** (the step: the manual marks operating-system limits where they exist, so its silence here is silence rather than omission — but silence is still silence, and I am not claiming more).

**3. What running Blender 5.2.1 added to both lines.** Both labels above stay exactly as they are — they are readings of what the sources say, and no run can change that. What follows is a separate pair of claims about a separate thing.

**Outside the interface, 2026-08-29. Label: observed.** Every run recorded under *What a running Blender 5.2 did* above was made from a terminal with `-b` and no window, and every name in this answer was set and read back that way. So these names are no longer merely *unmarked* as interface-only — they were exercised from outside and they worked. Three things the runs add. **`use_single_layer` really is dead from outside:** set to `True` on the launch line, both view layers still came out in the written file. **Where the instruction sits decides whether it does anything at all:** handed in before the file, a correct name reads back the value set and changes nothing about the render. And **a wrong name costs nothing a program could see:** the frame still renders, the file is still written, and Blender still exits 0. All of that was read off the Creator's Mac, macOS 26.6.2 on Apple silicon, and speaks for no other machine.

**Operating system, 2026-08-29. Label: observed, and bounded.** Every run recorded above was on **one computer** — the Creator's MacBook Pro, an Apple M1 Pro chip, macOS 26.6.2 (build 25G83, Darwin 25.6.0, arm64). **Nothing was rented, and nothing ran on Windows or on Linux.** So on **macOS on Apple silicon** these names are now exercised rather than reasoned about. On every other operating system nothing has changed: the reading above is still all there is, and **whether these names hold on Windows or on Linux is unknown**. A run on a machine of each is what would settle it, and no such machine was available for this round. Nobody may read the macOS result as covering the others.

### Not covered

- This answer gives names only. It does not say whether the render actually honours a value set by a launch-time instruction, when in the launch sequence such an instruction has to run to take effect, or what happens if it runs too late. The manual's warning that "Arguments are executed in the order they are given" is on the same page, but working out its consequences for these three settings is a different question and I have not done it.
- It does not settle whether a launch-time Python instruction is allowed to run at all in a given setup — the switches `-y / --enable-autoexec` and `-Y / --disable-autoexec` are on the arguments page, and I have not looked into how they bear on `--python-expr`.
- For the sample count it covers the three engines the manual names. The manual also says "More renderers from third-party developers are available as add-ons" (https://docs.blender.org/manual/en/5.2/render/introduction.html); what any of those call a sample count is not in my sources.
- The Grease Pencil scene settings carry `aa_samples`, "Number of supersampling anti-aliasing samples per pixel for final render (in [1, inf], default 8)" (https://docs.blender.org/api/5.2/bpy.types.SceneGpencil.html, read 2026-07-30). I found it while sweeping the index and am naming it only so the record is complete; I have not established that it is "the sample count" of any engine.
- I did not establish where in a blend-file's own structure any of these names live, only what the names are.

---

## Record of where I looked

**Files opened in full or in part**

Manual (5.2):
- `advanced/command_line/arguments.txt` — the whole page, read end to end. It is the manual's page of launch switches, set out section by section; it does not itself claim to be complete.
- `advanced/command_line/render.txt` — the whole page.
- `advanced/command_line/index.txt` — the whole page, to see the section list (Render Options, Cycles Render Options, Format Options, Animation Playback Options, Window Options, Python Options, Network Options, Logging Options, Debug Options, GPU Options, Misc Options, Sub-Commands).
- `advanced/command_line/extension_arguments.txt` — opened its first section to check whether the sub-commands reach render settings. They do not: the sub-commands are package management, repository management and extension creation.
- `render/output/properties/format.txt`
- `render/layers/view_layer.txt`
- `render/introduction.txt`
- `render/eevee/render_settings/sampling.txt`
- `getting_started/help.txt` and `interface/controls/buttons/menus.txt` — for how the manual says a person finds the Python name of a control.
- Pictures: `_images/render_output_properties_format_panel.png`, `_images/render_layers_view-layer_panel.png`.

Python API documentation (5.2):
- `bpy.types.RenderSettings.txt`
- `bpy.types.Scene.txt`
- `bpy.types.ViewLayer.txt`
- `bpy.types.SceneEEVEE.txt`
- `bpy.types.SceneDisplay.txt`
- `bpy.types.SceneGpencil.txt`
- `bpy.types.RenderEngine.txt`
- `bpy.ops.render.txt`
- `info_quickstart.txt`
- `genindex-all.txt`, `genindex-R.txt`, `genindex-S.txt`, `genindex-T.txt`, `genindex-U.txt` — the documentation's own index, used to be sure I had not missed a differently named setting.

Release notes (5.2): I searched all twenty-one files (`animation_rigging`, `assets`, `bugfixes`, `compositor`, `core`, `cycles`, `eevee`, `geometry_nodes`, `grease_pencil`, `index`, `modeling`, `motion_tracking`, `navigation`, `physics`, `pipeline_io`, `python_api`, `rendering`, `sculpt`, `sequencer`, `user_interface`, `virtual_reality`) and read `python_api.md` and `rendering.md` in full.

Cycles' own files (5.2):
- `properties.py` — the sampling block, the `CyclesRenderSettings` class, its `register`, and `CyclesRenderLayerSettings` with its `register`.
- `ui.py` — the sampling panel and the places `resolution_percentage` appears.
- `engine.py` — the whole argument-parsing section.
- I also listed the folder to see what is in it: `__init__.py`, `camera.py`, `engine.py`, `license`, `maketx.py`, `operators.py`, `osl.py`, `presets.py`, `properties.py`, `shader`, `source`, `ui.py`, `version_update.py`.

**Terms I searched for, across the manual, the API documentation, the release notes and Cycles' files together**

`resolution_percentage`, `resolution percentage`, `Percentage scale`, `Resolution %`, `percentage`, `taa_render_samples`, `taa_samples`, `samples`, `aa_samples`, `render_aa`, `Max Samples`, `sample count`, `use_single_layer`, `Use for Rendering`, `Use this View Layer`, `ViewLayer.use`, `view_layer.use`, `--render-percentage`, `--samples`, `--python-expr`, `command line`, `command-line`, `argument`, `background mode`, `subcommand`, `sub-command`, `renamed`, `Copy Data Path`, `Python Tooltips`, `add_argument`, `argv`, `parser`.

**What I ruled out, and why**

- **A dedicated launch switch for any of the three.** Ruled out as far as my sources go, by reading the arguments page end to end rather than searching it — no section of it holds one. Cross-checked by searching all four sources for `--render-percentage` and `--samples` (no hits anywhere) and by reading Cycles' own argument parser, which declares exactly two arguments. What this does not rule out is a switch the manual page never lists — though the manual itself calls that page "a full list of arguments" (https://docs.blender.org/manual/en/5.2/advanced/command_line/render.html, read 2026-07-30), so accepting that sentence is the whole of the step. *(Corrected 2026-08-29 by the citation check; this line used to say nothing states the page is complete.)*
- **A rename or removal in 5.2.** Ruled out by searching all twenty-one release-note files for `renamed`, `resolution_percentage`, `taa_render_samples`, `.samples`, `resolution percentage`, `ViewLayer.use` and `view_layer.use`. The only hits were unrelated: an Asset Browser option renamed, sequencer node-group inputs, two bug-fix lines about renaming, and — the one near miss — a new Cycles Simplify setting: "In the Simplify panel, there is a new Texture Resolution percentage setting, in addition to the existing Texture Limit" (https://developer.blender.org/docs/release_notes/5.2/cycles/, read 2026-07-30). That is a texture setting inside Simplify, not the render's resolution percentage, and I set it aside on that ground. The 5.2 Python API notes list changes to annotations, GPU, geometry nodes, nodes, imbuf, paint, mathutils, arrays, reports, the path iterator, the all-IDs iterator, `libraries.load()`, the window manager, the user interface, Hydra, assets and ID-property nesting — none of them touches these three names.
- **New launch switches in 5.2.** Ruled out by searching the release notes for `command line`, `command-line`, `--python`, `argument` and `background mode`. The only launch-related hit was `gpu.init()` for background mode, which is not a render setting.
- **The render command's own options as the answer.** Ruled out by reading the full option list of `bpy.ops.render.render` — it has `layer` and `scene`, neither of which is "whether a view layer renders", and nothing for a percentage or a sample count.
- **A scene-wide sample count belonging to no engine.** Ruled out from the API documentation's index: every indexed name containing `samples` that belongs to a scene or a view layer is engine-specific or viewport-specific (`taa_render_samples` and `taa_samples` on `SceneEEVEE`, `volumetric_samples` and `volumetric_shadow_samples` on `SceneEEVEE`, `matcap_ssao_samples` on `SceneDisplay`, `aa_samples` on `SceneGpencil`), plus `samples` on `ViewLayer` as an override. There is no `samples` on `RenderSettings` and none on `Scene` itself.

---

## Things a person with Blender open could still settle by hand

**Three of the eight items that stood here were run on 2026-08-29**, and what came back is written up under *What a running Blender 5.2 did* above: what Workbench's own Render Properties actually offers (was item 5 — one `Sampling` panel, `Render` reading `8 Samples`, `Viewport` reading `Single Pass Anti-…`, and no plain number-of-samples field); the launch-switch list taken off the program itself, including `-c help` and each sub-command's own help (6); and every name tried end to end from a launch instruction, with the written picture read back (7).

Item 8 asked for the same runs on Linux and on the Mac, and named Windows as unavailable. **Only the Mac leg was run.** What is left:

1. **Read the names off a hovering tooltip, or copy them off the controls.** Switch on **Python Tooltips** in Preferences ‣ Interface, then hold the pointer still over the Percentage slider in Output Properties ‣ Format, over the render Samples field in Render Properties ‣ Sampling with Cycles and again with EEVEE, and over the *Use for Rendering* checkbox in View Layer Properties. Or right-click each and choose *Copy Full Data Path* and paste the result. Both need a hand holding a mouse on a control, and no command stands in for either. *What they were wanted for is settled above another way*: the running program's own label and description were read out for every one of these properties, and those are the two pieces of text a tooltip draws. A negative result is a tooltip or a pasted path showing a name other than the ones above.
2. **The same runs on Linux, and on Windows.** Item 8 already said the Windows leg was not run, no Windows machine having been rented for it. **Untried rather than untestable: a Windows machine can be rented, and every one this project has read reported a Server edition — see the equipment section of `Preparation/Blender Research/Method.md`.** As things stand the Linux leg cannot be run either: nothing was rented for this round, so every result above is a macOS-on-Apple-silicon result and both other systems stay untested rather than answered.
3. **A sample-count name for a render engine other than Cycles, EEVEE and Workbench.** No such engine is installed on this Mac. With one installed, open a Python console inside Blender, type `bpy.context.scene.` and see what that engine added, then look through it for a sample count. A negative result — the engine adding nothing that carries one — is itself the answer for that engine.

---

2026-08-29 — Claude Opus 5 (Claude Code) — Settings, Blender 5.2

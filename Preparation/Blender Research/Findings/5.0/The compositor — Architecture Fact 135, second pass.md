# The compositor — Blender 5.0 — Question 9 (second pass)

**Sources consulted, and when.** Blender 5.0 manual (`Blender Sources/manual_5.0`, including one picture from its `_images` folder), Blender 5.0 Python API documentation (`Blender Sources/api_5.0`), Blender 5.0 release notes (`Blender Sources/release_notes_5.0`), and Cycles' own add-on files (`Blender Sources/cycles_5.0`). All four were taken on 2026-07-30, and that is the date read for everything quoted below. I also read `Blender Sources/About this folder.md`, which is inside the folder I was allowed to read.

I read only the question sheet and files inside `Blender Sources/`. I did not open any other part of this project.

I started no helper agents. Every line below is my own reading.

---

## 9. When one scene takes in another scene's rendered result, what decides the render engine and the sample count that the second scene is rendered with, and what effect on that does a value for either, given to the render from outside, have?

### The reading I answered, and the one I did not

The question is ambiguous in two places, and I say so rather than choose silently.

**"Takes in another scene's rendered result."** I answered the compositor's Render Layers node with its Scene picker set to a scene other than the one being rendered. That is the reading the topic points at. The other reading is a Scene strip in the video sequence editor, which the manual also describes as taking in another scene's render. I checked that page too and report what it says under "Where I looked".

**"A value for either, given to the render from outside."** I answered this as a value handed to Blender from outside the blend-file when the render is started — that is, a command-line argument. The other reading is a value reaching the second scene's render from outside that scene: from the first scene, from the view layer, or from a script run before the render. I cover what the sources say about that reading too, because it is cheap to do and the gap would otherwise be invisible.

---

### Part one — what decides the render engine

**LABEL: inferred.**

Three documented pieces, then the one step I took.

**Documented piece 1 — the node offers no engine setting at all.** The Render Layers node has exactly two properties:

> "Properties
>
> Scene
>
> The scene for which to render a view layer.
>
> View Layer
>
> The view layer to render. The button next to the dropdown re-renders it immediately."

Source: manual, https://docs.blender.org/manual/en/5.0/compositing/types/input/scene/render_layers.html — read 2026-07-30.

The picture on that same page shows the node itself, and shows the same two controls and nothing else: a Scene field reading "Scene", and a View Layer dropdown reading "ViewLayer" with a small button beside it. Picture looked at: `_images/node-types_CompositorNodeRLayers.webp`, from the page above. This is **documented**, since the picture is part of the manual. What is visible is two pickers and a button; I read nothing further into it.

The Python API says the same thing in its own words. The whole property list for the node is:

> "Input render passes from a scene render
>
> layer
>
> Type:
>
> enum in ['PLACEHOLDER'], default 'PLACEHOLDER'
>
> scene
>
> Type:
>
> Scene"

Source: API documentation, https://docs.blender.org/api/5.0/bpy.types.CompositorNodeRLayers.html — read 2026-07-30.

**Documented piece 2 — the engine is a property of a scene.**

> "engine
>
> Engine to use for rendering
>
> Type:
>
> enum in ['BLENDER_EEVEE'], default 'BLENDER_EEVEE'"

Source: API documentation, https://docs.blender.org/api/5.0/bpy.types.RenderSettings.html — read 2026-07-30.

And `RenderSettings` is reached through a scene:

> "render
>
> Type:
>
> RenderSettings, (readonly, never None)"

Source: API documentation, https://docs.blender.org/api/5.0/bpy.types.Scene.html — read 2026-07-30.

The manual puts it plainly:

> "A scene defines the visible objects, camera view, lighting setup, and render settings that make up a specific part of a project."

Source: manual, https://docs.blender.org/manual/en/5.0/scene_layout/scene/introduction.html — read 2026-07-30.

and

> "Each renderer has its own render settings to control render quality and performance."

Source: manual, https://docs.blender.org/manual/en/5.0/render/introduction.html — read 2026-07-30.

**Documented piece 3 — Cycles is one of these engines, named `CYCLES`, and it is an add-on that registers itself.** This one is **observed**, from Cycles' own files:

> `class CyclesRender(bpy.types.RenderEngine):`
> `    bl_idname = 'CYCLES'`
> `    bl_label = "Cycles"`

File: `Blender Sources/cycles_5.0/__init__.py`, lines 42–44. Read 2026-07-30.

That is why the API's engine list above shows only `BLENDER_EEVEE`: the documented enum is the built-in one, and Cycles adds its own identifier when the add-on registers. I say that as **inferred**, and the step is: the API page lists a fixed enum containing only `BLENDER_EEVEE`, while Cycles' file registers a render engine class whose identifier is `CYCLES`; so the enum as documented is not the complete set of engines available at runtime.

**The step I took.** The node names a scene and a view layer and carries nothing else. The engine is a setting that lives on a scene. Therefore the second scene is rendered with the engine set on that second scene, not on the scene doing the compositing. **No page in the sources says this in so many words.** A reader has to join the two facts, so the label is inferred, not documented.

There is a documented near-miss that I deliberately did **not** stretch into an answer. The stereoscopy page says that some settings do come from the compositing scene regardless of what the Render Layers nodes point at:

> "The views to render are defined in the current scene views, in a similar way as you define the composite output resolution in the current scene render panel, regardless of the Image nodes resolutions or Render Layers from different scenes."

Source: manual, https://docs.blender.org/manual/en/5.0/render/output/properties/stereoscopy/usage.html — read 2026-07-30.

That sentence names views and output resolution. It does not name the engine and it does not name samples. I mention it because it is the closest the sources come to the question and because it cuts the other way — it shows that at least some things are taken from the compositing scene rather than the pulled-in one. It is not evidence about the engine either way.

---

### Part two — what decides the sample count

**LABEL: documented for the mechanism; inferred for its application to a scene pulled in by the node.**

The sample count is not one setting. It is a scene setting, possibly overridden per view layer, with a scene-level policy under Cycles deciding whether that override is honoured.

**The scene-level count, Cycles.** Manual:

> "Render (Max) Samples
>
> The number of paths to trace per pixel in the final render. A higher number results in a cleaner image at the cost of a longer render time."

with the panel named at the top of the page as:

> "Panel:
>
> Render ‣ Sampling"

Source: manual, https://docs.blender.org/manual/en/5.0/render/cycles/render_settings/sampling.html — read 2026-07-30.

The setting behind it is on the scene. **Observed**, from Cycles' own files:

> `    samples: IntProperty(`
> `        name="Samples",`
> `        description="Number of samples to render for each pixel",`
> `        min=1, max=(1 << 24),`
> `        default=4096,`
> `    )`

File: `Blender Sources/cycles_5.0/properties.py`, lines 479–484, inside `class CyclesRenderSettings(bpy.types.PropertyGroup):` which begins at line 378. Read 2026-07-30.

That class is attached to the scene, not to anything smaller:

> `    @classmethod`
> `    def register(cls):`
> `        bpy.types.Scene.cycles = PointerProperty(`
> `            name="Cycles Render Settings",`
> `            description="Cycles render settings",`
> `            type=cls,`
> `        )`

File: `Blender Sources/cycles_5.0/properties.py`, lines 1101–1107. Read 2026-07-30.

And the panel that draws it reads it off the scene:

> `        scene = context.scene`
> `        cscene = scene.cycles`

and, further down the same draw method:

> `            col.prop(cscene, "samples", text="Max Samples")`
> …
> `            col.prop(cscene, "samples", text="Samples")`

File: `Blender Sources/cycles_5.0/ui.py`, lines 298–299 and lines 313 and 316. Read 2026-07-30.

**The scene-level count, EEVEE.** Manual:

> "Render
>
> Samples
>
> The number of samples to use in the final render."

Source: manual, https://docs.blender.org/manual/en/5.0/render/eevee/render_settings/sampling.html — read 2026-07-30.

The property behind it, again on the scene's EEVEE settings:

> "taa_render_samples
>
> Number of samples per pixel for rendering
>
> Type:
>
> int in [1, inf], default 64"

Source: API documentation, https://docs.blender.org/api/5.0/bpy.types.SceneEEVEE.html — read 2026-07-30.

**The per-view-layer override.** This matters here, because the Render Layers node names a view layer as well as a scene.

> "samples
>
> Override number of render samples for this view layer, 0 will use the scene setting
>
> Type:
>
> int in [0, inf], default 0"

Source: API documentation, https://docs.blender.org/api/5.0/bpy.types.ViewLayer.html — read 2026-07-30.

The manual describes the same override and says the two engines treat it differently:

> "Samples
>
> Allows the view layer to use a different number of samples than the main scene. This is controlled by the Layer Samples when using the Cycles renderer. When using EEVEE, the Samples override is always used."

and, above it:

> "Overrides are applied only when rendering the specific view layer and do not affect the scene globally."

Source: manual, https://docs.blender.org/manual/en/5.0/render/layers/override.html — read 2026-07-30.

**The Cycles policy that decides whether the override counts.** Manual:

> "Layer Samples
>
> If any view layers have Sample Overrides configured, this option specifies how to use them.
>
> Use:
>
> Allow view layers to override the scene-level sample count.
>
> Bounded:
>
> Allow the overrides as long as they don’t exceed the scene-level sample count.
>
> Ignore:
>
> Ignore the overrides."

Source: manual, https://docs.blender.org/manual/en/5.0/render/cycles/render_settings/sampling.html — read 2026-07-30.

The setting behind it, **observed**:

> `enum_use_layer_samples = (`
> `    ('USE', "Use", "Per render layer number of samples override scene samples"),`
> `    ('BOUNDED', "Bounded", "Bound per render layer number of samples by global samples"),`
> `    ('IGNORE', "Ignore", "Ignore per render layer number of samples"),`

File: `Blender Sources/cycles_5.0/properties.py`, lines 64–67. Read 2026-07-30.

> `    use_layer_samples: EnumProperty(`
> `        name="Layer Samples",`
> `        description="How to use per view layer sample settings",`
> `        items=enum_use_layer_samples,`
> `        default='USE',`
> `    )`

File: `Blender Sources/cycles_5.0/properties.py`, lines 550–555, inside `CyclesRenderSettings` — that is, on the scene. Read 2026-07-30. The default is `USE`.

Cycles only shows that control when it is relevant, and it looks at the view layers of the same scene:

> `        for view_layer in scene.view_layers:`
> `            if view_layer.samples > 0:`
> `                layout.separator()`
> `                layout.row().prop(cscene, "use_layer_samples")`
> `                break`

File: `Blender Sources/cycles_5.0/ui.py`, lines 471–476. Read 2026-07-30.

**The step I took.** Every one of these settings hangs off a scene or off a view layer of a scene. The node names the second scene and one of its view layers. So the sample count for that render is the second scene's own sample setting, raised or lowered by that view layer's `samples` override if it is non-zero, with Cycles' `Layer Samples` policy on the second scene deciding whether the override is used, bounded by the scene count, or ignored — and with EEVEE always using the override. **No page says this about a scene pulled in through the compositor.** The documents describe the mechanism; joining it to the cross-scene case is my step, so: inferred.

---

### Part three — a value given to the render from outside

**LABEL for the engine: documented that the option exists; not in the sources for what it does to the second scene.**

**LABEL for the sample count: not in the sources — these sources describe no such option.**

*(Narrowed by the citation check, 2026-08-29. The label read "not in the sources — there is no such option." The second half is a statement about Blender and the first half is a statement about the reading; the quotes below carry only the first.)*

**Engine, from the command line.** The option exists and is documented:

> "-E, --engine <engine>
>
> Specify the render engine. Use -E help to list available engines."

Source: manual, https://docs.blender.org/manual/en/5.0/advanced/command_line/arguments.html — read 2026-07-30.

The worked example page says the same, in the context of rendering an animation:

> "-E CYCLES
>
> Use the “Cycles Render” engine. For a list of available render engines, run blender -E help."

Source: manual, https://docs.blender.org/manual/en/5.0/advanced/command_line/render.html — read 2026-07-30.

**What the sources do not say about it.** Neither page says which scene or scenes `-E` applies to. It does not say it sets the engine on every scene in the file, and it does not say it applies only to the scene being rendered. It does not use the word "override" at all, which is notable because the neighbouring format option on the same page does:

> "-F OPEN_EXR
>
> Override the image format specified in the blend-file and save to an OpenEXR image."

Source: manual, https://docs.blender.org/manual/en/5.0/advanced/command_line/render.html — read 2026-07-30.

So whether `-E` reaches a second scene pulled in by a Render Layers node is **not in the sources**. I searched for this specifically; the record is below.

One neighbouring option does say what it applies to, and it is about scene selection rather than engine:

> "-S, --scene <name>
>
> Set the active scene <name> for rendering."

Source: manual, https://docs.blender.org/manual/en/5.0/advanced/command_line/arguments.html — read 2026-07-30.

That names one active scene for the render. It says nothing about scenes reached through the compositor.

**Sample count, from the command line — these sources describe none.** *(Heading narrowed by the citation check, 2026-08-29; it read "there is none".)* The full argument list for 5.0 contains no sample-count option. The only render options are `-b`, `-a`, `-S`, `-f`, `-s`, `-e`, `-j`, `-o`, `-E`, `-t`; the only Cycles options are:

> "--cycles-device <device>
>
> Set the device used for rendering. Valid options are: CPU CUDA OPTIX HIP ONEAPI METAL."

and

> "--cycles-print-stats
>
> Log statistics about render memory and time usage."

Source: manual, https://docs.blender.org/manual/en/5.0/advanced/command_line/arguments.html — read 2026-07-30.

Cycles' own files show that these two are the whole of what the add-on's own argument parser defines. **Observed:** *(Narrowed by the citation check, 2026-08-29; it read "the whole of what the add-on accepts after `--`". What the quoted lines show is what the parser declares. What a run does with an argument it has not declared is a step past the file.)*

> `    parser.add_argument("--cycles-print-stats",`
> `                        help="Print rendering statistics to stderr",`
> `                        action='store_true')`
> `    parser.add_argument("--cycles-device",`
> `                        help="Set the device to use for Cycles, overriding user preferences and the scene setting."`

File: `Blender Sources/cycles_5.0/engine.py`, lines 13–18, in `_configure_argument_parser()`. Read 2026-07-30. The parser is then used on the arguments after `--`:

> `    argv = sys.argv`
> `    if "--" not in argv:`
> `        return`
> …
> `    args, _ = parser.parse_known_args(argv[argv.index("--") + 1:])`

File: `Blender Sources/cycles_5.0/engine.py`, lines 27–32. Read 2026-07-30.

Note the wording Cycles uses for the one thing it does override from outside: `--cycles-device` "overriding user preferences and the scene setting". That phrasing exists for the device and for nothing else. There is no equivalent for samples, and no equivalent for the engine in Cycles' files.

There are also no environment variables for either. The environment-variable section of the arguments page lists only resource directories, a splash image, an OpenColorIO path, and temporary-file locations. Same page and date as above.

**The other reading of "from outside".** If "from outside" means from outside the second scene rather than from outside the file, then what the sources document is: the per-view-layer `samples` override quoted in part two, and nothing for the engine — no per-view-layer engine override appears anywhere I searched. A value could of course be assigned by a script, since both `Scene.render.engine` and `ViewLayer.samples` are writable properties in the API pages quoted above, but the sources describe them as ordinary settings and say nothing about a script-set value being treated differently from a saved one. The render operator itself takes no engine or sample argument:

> "bpy.ops.render.render(*, animation=False, write_still=False, use_viewport=False, use_sequencer_scene=False, layer='', scene='', frame_start=0, frame_end=0)"

with

> "scene (string, (optional, never None)) – Scene, Scene to render, current scene if not specified"

Source: API documentation, https://docs.blender.org/api/5.0/bpy.ops.render.html — read 2026-07-30. The page marks the operator itself "Undocumented, consider contributing."

---

### What running Blender showed — the hand tests of 2026-08-29

**LABEL: observed**, for everything in this section and for nothing above it.

**These are two separate claims and they are kept apart.** Everything above this heading is what the Blender 5.0 sources say, and it stands unchanged — including every *not in the sources*, which is a statement about the documentation and is not undone by a run. What follows is the other claim: what Blender 5.0 actually did. Where the two meet, both are recorded — the reading as a reading, the run as a run.

**On what kind of computer, and when.** The Creator's own MacBook Pro: an Apple M1 Pro chip, Apple's own graphics, no second card. Read off the machine on the day:

```
ProductName:		macOS
ProductVersion:		26.6.2
BuildVersion:		25G83
arm64
Apple M1 Pro
```

The Blender was `/Applications/Blender 5.0.1.app/Contents/MacOS/Blender`, which reports:

```
Blender 5.0.1 (hash a3db93c5b259 built 2025-12-16 01:51:40)
```

Every run was made on **2026-08-29**, and every one was repeated on the same machine the same day by the citation check, with the same result each time. Every run used `--factory-startup`, so no third-party add-on was loaded and no add-on render engine existed in any of them. **Nothing was run on any other operating system** — see the operating-system line below.

**What was run against.** One set of files, built by Blender 5.0.1 itself so that no file crossed a version boundary. Each holds two scenes: **`Comp`**, whose compositor is being run, and **`Sub`**, the scene pulled in. Both hold a camera, one area light, a metallic sphere of roughness 0.35 and a blue floor, with the two spheres in different places so the two scenes are not the same picture. `Comp`'s compositor holds one **Render Layers** node with its Scene set to `Sub` and its View Layer set to `Sub`'s own `ViewLayer`, wired straight to the output and nothing else — so the written image is entirely `Sub`'s render, and any change in it is a change in how `Sub` was rendered. Both scenes: 160 by 120 pixels, one frame, uncompressed 32-bit OpenEXR, compositing on, stamping off, Cycles on the processor with denoising off, adaptive sampling off, no time limit and a fixed seed of 7, so a Cycles render repeats to the bit. Only the engines, sample counts, view-layer overrides and Layer Samples choices differ from file to file, and each finding below names the file it used. The script that builds them is printed in full in `Audits/Blender Research/The compositor/Hand tests — Architecture Fact 135.md`.

Every run took this shape, with the file, the extra arguments and the output path changing:

```
"/Applications/Blender 5.0.1.app/Contents/MacOS/Blender" --factory-startup -b "<file>.blend" --log "render" -o "<output>_" -f 1
```

`-f 1` sits last because Blender's own manual warns that arguments run in the order they are given.

**Two instruments, and no weaker ones.** Blender's own log, turned on with `--log "render"`. And exact pixel comparison: every render was written as an uncompressed 32-bit OpenEXR and pairs of images were compared channel by channel, reported below as `maxdiff` — the largest difference in any one colour channel of any one pixel, across 19,200 pixels. A `maxdiff` of exactly 0 means the two are the same picture to the last bit. Files were never compared byte for byte, because Blender writes times into an image and those are not the picture.

---

**Observed 1 — the second scene is rendered with the engine set on the second scene.**

Four files were rendered, each with a different engine on `Comp` and on `Sub`. Each written image was compared against a **reference** made by rendering `Sub` directly with `-S Sub` added, so that no compositor was involved in the reference at all.

Blender named the engine itself, in a line of its own. From the file with `Comp` on Cycles and `Sub` on Workbench:

```
render           | Start rendering: Sub, ViewLayer
render           | Engine: Workbench
render           | Executing compositor
render           | Fra: 1 | Compositing
```

The engine line across the four files:

| file | `Comp` is | `Sub` is | Blender printed |
| --- | --- | --- | --- |
| `eng_Awb_Bcy` | Workbench | Cycles at 64 | `Engine: Cycles` |
| `eng_Acy_Bwb` | Cycles at 64 | Workbench | `Engine: Workbench` |
| `eng_Aee_Bcy` | EEVEE | Cycles at 64 | `Engine: Cycles` |
| `eng_Acy_Bee` | Cycles at 64 | EEVEE at 16 | `Engine: EEVEE` |

And the pixels say the same thing:

| file | `Comp` is | `Sub` is | vs `Sub`'s own engine's reference | vs `Comp`'s engine's reference |
| --- | --- | --- | --- | --- |
| `eng_Awb_Bcy` | Workbench | Cycles at 64 | Cycles-64: **0** | Workbench: 44.1421 |
| `eng_Acy_Bwb` | Cycles at 64 | Workbench | Workbench: **0** | Cycles-64: 44.1421 |
| `eng_Aee_Bcy` | EEVEE | Cycles at 64 | Cycles-64: **0** | EEVEE-16: 2.1 |
| `eng_Acy_Bee` | Cycles at 64 | EEVEE at 16 | EEVEE-16: **0** | Cycles-64: 2.1 |

The zeros are exact zeros. (The two non-zero figures are recorded as the same across all four researched versions to within the third decimal place; the zeros are Blender 5.0's own.)

**So the scene pulled in through the node is rendered with the engine stored on that scene**, in both directions and for all three engines: Workbench inside a Cycles render, Cycles inside a Workbench render, EEVEE inside a Cycles render and Cycles inside an EEVEE render. The four files were built so that the opposite answer would have shown as each image matching the *other* reference, and none did.

One thing found in passing, worth carrying because it bears on what "the second scene is rendered with" even means. Only **one** `Start rendering:` line appeared in any of these runs, and it always named `Sub`. `Comp`'s own view layer was never rendered at all — nothing in its compositor asks for it. So the render of the compositing scene consisted of rendering the other scene and then running the compositor.

---

**Observed 2 — under Cycles, the sample count is the second scene's own, and the compositing scene's count does not leak into it.**

Two files with the two counts swapped, and two references made with `-S Sub`:

| file | `Comp` at | `Sub` at | last sample line printed | vs `Sub`-at-512 | vs `Sub`-at-1 |
| --- | --- | --- | --- | --- | --- |
| `smp_A1_B512` | 1 | 512 | `Sample 512/512` | **0** | 17.5015 |
| `smp_A512_B1` | 512 | 1 | `Sample 1/1` | 17.5015 | **0** |

The compositing scene's number never appears. Then the sharper form of the same question, with `Sub` held at 128 samples and only `Comp`'s number changed between two files:

```
smp_A4096_B128  vs smp_A1_B128:  maxdiff=0  meandiff=0
```

Both reported `Sample 128/128`, and both matched the `Sub`-at-128 reference at `maxdiff = 0`. **A four-thousand-fold change in the compositing scene's sample count produced no change in any pixel of the written image.** Any leak at all would have been a number greater than zero, and there was none.

---

**Observed 3 — under EEVEE, the sample count is also the second scene's own.**

Two files with the counts swapped, and references made with `-S Sub`:

| file | `Comp` at | `Sub` at | vs `Sub`-at-64 | vs `Sub`-at-4 |
| --- | --- | --- | --- | --- |
| `ee_A4_B64` | 4 | 64 | **0** | 5.41 |
| `ee_A64_B4` | 64 | 4 | 5.41 | **0** |

No findings document had covered EEVEE's sample count through the node by test; the cross-scene case had been left to the same inference as Cycles. It holds.

---

**Observed 4 — under Cycles, the view layer's sample override does reach through the node, and it is the *pulled-in* scene's Layer Samples policy that governs it.**

Five files, both scenes on Cycles, `Comp` at 64 samples, `Sub` at 512, the node pointing at `Sub`'s only view layer, and that view layer given a sample override. References at 512 and at 4, made with `-S Sub`.

The five files:

| file | override | `Sub`'s Layer Samples | Blender printed | matches |
| --- | --- | --- | --- | --- |
| `vl_use` | 4 | Use | `Sample 4/4` | the 4-sample reference, `maxdiff = 0` |
| `vl_ignore` | 4 | Ignore | `Sample 512/512` | the 512-sample reference, `maxdiff = 0` |
| `vl_bounded_lo` | 4 | Bounded | `Sample 4/4` | the 4-sample reference, `maxdiff = 0` |
| `vl_bounded_hi` | 4096 | Bounded | `Sample 512/512` | the 512-sample reference, `maxdiff = 0` |
| `vl_use_hi` | 4096 | Use | `Sample 4096/4096` | neither — 4096 samples is its own picture |

So the override reaches through the node, and the three choices behave as the manual describes them: *Use* takes the override whatever its size, *Ignore* discards it, and *Bounded* takes it when it is below the scene's count and clamps it to the scene's count when it is above. Had the override not been consulted through the node, all five would have reported 512; had the policy not been consulted, all five would have reported the override. All five came out distinct.

**Whose policy is consulted** was then separated out directly, by giving the two scenes opposite choices with `Sub` at 512 and its view layer overridden to 4:

| file | `Sub`'s policy | `Comp`'s policy | Blender printed |
| --- | --- | --- | --- |
| `pol_Bignore_Ause` | Ignore | Use | `Sample 512/512` |
| `pol_Buse_Aignore` | Use | Ignore | `Sample 4/4` |

The count follows `Sub`'s policy in both directions and `Comp`'s in neither. **The Layer Samples policy consulted is the pulled-in scene's own.**

---

**Observed 5 — under EEVEE, whether the view layer's sample override is honoured at all is where the four versions differ.**

One file, `ee_vl`: both scenes on EEVEE, `Comp` at 4 render samples, `Sub` at 64, and `Sub`'s view layer given a sample override of 8, with the node pointing at that view layer. Three references made with `-S Sub`: `Sub` at 64 with no override, `Sub` at 4, and `Sub` at 8 with no override — the last built by handing Blender a one-line script that set `Sub`'s EEVEE render samples to 8 and cleared the override before rendering `Sub` directly.

Blender 5.0.1 printed `Rendering 8 / 8 samples`, and the written image matched the 8-sample reference at `maxdiff = 0` and differed from the 64-sample reference, at `maxdiff` 2.633.

**Blender 5.0.1 honours it. Blender 4.5 does not** — this is the one place in the whole round where the four researched versions differ, and each of the four is carrying the difference so that nobody later mistakes it for something else. The same test in 4.5 printed `Rendering 64 / 64 samples` and matched the 64-sample reference exactly, ignoring the override.

**It is not a difference in what the compositor does, and this was established rather than assumed.** The same file was also rendered with `-S Sub`, so `Sub` was the active scene and no compositor was involved at all. In 5.0.1 that render came out `maxdiff = 0` against the 8-sample reference too, and in 4.5 against the 64-sample one. So what differs is whether EEVEE honours a view layer's sample override at all in a given version; the path through the node reports faithfully whichever way that goes.

**It was re-run before being written down**, because it was the round's only disagreement between versions. Blender 5.0.1 was run again on the same file and printed `Rendering 8 / 8 samples`.

---

**Observed 6 — a scene reached through the node renders exactly as it does when it is the scene being rendered.**

Ten of the files above were rendered twice over: once through the node with `Comp` active, and once directly with `-S Sub` so that no compositor was involved. Those ten — `eng_Awb_Bcy`, `eng_Acy_Bwb`, `eng_Acy_Bee`, `smp_A1_B512`, `smp_A512_B1`, `smp_A1_B128`, `vl_use`, `ee_A4_B64`, `ee_A64_B4` and `ee_vl` — cover all three engines, four Cycles sample counts, two EEVEE sample counts, and a view-layer override under each of the two engines.

**Every one of the ten pairs came out `maxdiff = 0`.** Reported here for the hardest case, the EEVEE override file, because that is the one the versions disagree about:

```
5.0   ee_vl (through node) vs ref_ee_vl_direct   maxdiff=0 meandiff=0
```

Any difference at all, in any pixel, would have meant the compositor path renders the second scene with something other than the settings it has when rendered on its own. There was none. This is the strongest form of the answer, because it does not depend on knowing which setting did what.

---

**Observed 7 — an engine given on the command line does not reach the scene behind the node.**

Three files in which **both** scenes carry the same engine, so that any difference afterwards comes from the argument rather than from what was stored: `E_bothcy` (both Cycles at 64), `E_bothwb` (both Workbench), `E_bothee` (both EEVEE at 16). Four runs, `Comp` active in each, of the form:

```
"/Applications/Blender 5.0.1.app/Contents/MacOS/Blender" --factory-startup -b E_bothcy.blend --log "render" -E BLENDER_WORKBENCH -o "<output>_" -f 1
```

Blender named the engine it used, and it was never the one on the command line:

| stored on both scenes | `-E` given | Blender printed | last sample line |
| --- | --- | --- | --- |
| Cycles at 64 | `BLENDER_WORKBENCH` | `Engine: Cycles` | `Sample 64/64` |
| Cycles at 64 | `BLENDER_EEVEE` | `Engine: Cycles` | `Sample 64/64` |
| Workbench | `CYCLES` | `Engine: Workbench` | — |
| EEVEE at 16 | `CYCLES` | `Engine: EEVEE` | `Rendering 16 / 16 samples` |

The pixels agree:

| run | vs the stored engine's reference | vs the `-E` engine's reference |
| --- | --- | --- |
| Cycles file, `-E BLENDER_WORKBENCH` | Cycles-64: **0** | Workbench: 44.1421 |
| Cycles file, `-E BLENDER_EEVEE` | Cycles-64: **0** | EEVEE-16: 2.1 |
| Workbench file, `-E CYCLES` | Workbench: **0** | Cycles-64: 44.1421 |
| EEVEE file, `-E CYCLES` | EEVEE-16: **0** | Cycles-64: 2.1 |

**`-E` does not reach the scene pulled in behind the compositor.** The scene behind the node was rendered with its own stored engine every time. This was re-run before being written down: the Cycles file with `-E BLENDER_WORKBENCH` was run a second time and again reported the stored engine.

**What `-E` does instead is write the engine onto the active scene, and onto that scene alone.** Three runs of the same file, printing the stored engines, with the argument absent, before the printing step, and after it:

```
$ blender --factory-startup -b E_bothcy.blend --python-expr '<print>'   (no -E, control)
SCENES: [('Comp', 'CYCLES'), ('Sub', 'CYCLES')]

$ blender --factory-startup -b E_bothcy.blend -E BLENDER_WORKBENCH --python-expr '<print>'
SCENES: [('Comp', 'BLENDER_WORKBENCH'), ('Sub', 'CYCLES')]

$ blender --factory-startup -b E_bothcy.blend --python-expr '<print>' -E BLENDER_WORKBENCH
SCENES: [('Comp', 'CYCLES'), ('Sub', 'CYCLES')]
```

`Comp` is the file's active scene. So the other scene in the same file is untouched, and the change happens at the point in the command line where the argument appears — read before it, both scenes still say Cycles; read after it, only the active scene has changed. That is what explains the result above: the scene behind the node keeps its own engine because `-E` never wrote to it.

**It changes nothing on disk.** After a full render run with `-E BLENDER_WORKBENCH`, re-opening the same file with no `-E` and printing the engines gave:

```
SCENES: [('Comp', 'CYCLES'), ('Sub', 'CYCLES')]
```

The plain limit on that: it shows that a render run does not save the file back, not that nothing could ever save it — a run told to save the file would write whatever the running copy held.

**And the active-scene argument decides entirely what `-E` reaches.** Two runs of the same file, both scenes on Cycles:

| run | Blender printed | the written image matches |
| --- | --- | --- |
| `-S Comp -E BLENDER_WORKBENCH` | `Engine: Cycles` | the Cycles-64 reference, `maxdiff = 0` |
| `-S Sub -E BLENDER_WORKBENCH` | `Engine: Workbench` | the Workbench reference, `maxdiff = 0` |

Name the pulled-in scene as the scene to render and `-E` applies to it; leave the compositing scene active and `-E` applies to that one while the pulled-in scene keeps its own engine. Note what the second run costs, though: with `-S Sub`, `Sub` is what renders and `Comp`'s compositor is not used at all — it is not the same picture arriving by another route, it is a different render. **`-E` follows the active scene, not the render.**

**An engine name that does not exist is refused, not ignored.** `-E NO_SUCH_ENGINE` printed:

```
Error: engine not found 'NO_SUCH_ENGINE'
```

and nothing was rendered. Blender 5.0.1 exited **1**.

---

**Observed 8 — a sample count given from outside: no argument sets one, one offered anyway is silently ignored, and a script does reach it.**

**What the program says it accepts.** `--help` in Blender 5.0.1 lists, under *Render Options*, exactly `-b/--background`, `-a/--render-anim`, `-S/--scene`, `-f/--render-frame`, `-s/--frame-start`, `-e/--frame-end`, `-j/--frame-jump`, `-o/--render-output`, `-E/--engine`, `-t/--threads`; and under *Cycles Render Options*, exactly `--cycles-device` and `--cycles-print-stats`. **Nothing in either section sets a sample count.** The program's own account of itself therefore agrees with the reading above, which had rested on absence from a page.

Found in the same place, and worth recording because more than one of these documents told whoever ran their tests to look for such an argument rather than guess one: this build carries a `--log <match>` argument and a `--log-level <level>` argument, under a *Logging Options* heading. `--log "render"` is what produced every engine and sample line quoted in this section.

**What the engine names are.** `-E help` in Blender 5.0.1 printed:

```
Blender Engine Listing:
	BLENDER_EEVEE
	BLENDER_WORKBENCH
	CYCLES
```

It printed the listing and exited **0**.

**What happens when a sampling argument is offered anyway.** Running `smp_A1_B512` — `Comp` at 1 sample, `Sub` at 512 — with `-- --cycles-samples 16` appended produced no complaint of any kind, exit status **0**, a reported count of `Sample 512/512`, and a written image identical to the plain run at `maxdiff = 0`. Three outcomes were possible and all three were distinguishable: accepted and rendered at 16, refused with an error, or taken and ignored. **It is the third: the argument is neither accepted nor refused, and nothing is printed about it.**

**A script, though, does reach the second scene.** Running the same file with `--python-expr 'import bpy; bpy.data.scenes["Sub"].cycles.samples = 7'` before the render gave `Sample 0/7`, `Sample 1/7`, `Sample 7/7`, exit status 0 and the log still read `Start rendering: Sub, ViewLayer` and `Engine: Cycles`, and a written image differing from the 512-sample reference by `maxdiff` 7.36. **A value handed in from outside can reach the sample count of the scene behind the node — not by a sampling argument, which does not exist, but by a script given to Blender on the command line.**

---

**Observed 9 — what the node carries, and whether an engine can live below a scene.**

Asked of the running program rather than read off a page:

```
RLAYERS_OWN_PROPS: ['scene', 'layer']
VIEWLAYER_ENGINE_PROPS: []
VIEWLAYER_SAMPLE_PROPS: ['samples']
RENDERSETTINGS_ENGINE: ['engine', 'has_multiple_engines']
```

The node has two settings of its own and no more — the same two the manual lists, with nothing hidden. **A view layer carries no engine setting anywhere**, and one sample setting. The engine lives on a scene's render settings and nowhere else that a view layer or the node can reach. Either an extra name in the first list, or any name at all in the second, would have undercut the whole of the engine answer above; neither appeared.

---

**Observed 10 — the node's own re-render button is still unsettled, and this says so rather than pretending otherwise.**

The operator behind the button beside the View Layer dropdown was found by asking Blender what operators exist rather than guessing a name, which printed `['cryptomatte_layer_add', 'cryptomatte_layer_remove', 'read_viewlayers', 'render_changed']`. In a Blender 5.0.1 started **with a window** from a terminal, one area was turned into a node editor, its tree type set to the compositor, **its node tree set to `Comp`'s own compositing tree**, and the Render Layers node selected and made active. The editor reported itself ready:

```
AREA: NODE_EDITOR CompositorNodeTree
NODE: CompositorNodeRLayers scene= Sub layer= ViewLayer
poll render_changed: True
poll read_viewlayers: True
```

and the two operators returned:

```
RENDER_CHANGED_RESULT: {'CANCELLED'}
render_changed INVOKE: {'CANCELLED'}
read_viewlayers EXEC: {'FINISHED'}
```

**No render happened** — no scene line, no engine line and no sample line appeared in the log. So this establishes what that operator returns when it is driven this way, and nothing about what the button does when a person presses it. A press is a mouse click on a window, and nothing available for this round could produce one. **What the button renders with is unsettled in Blender 5.0.**

*(This one was re-run on 2026-08-29 rather than copied, because the record of the original run was a step short of repeatable: it did not say that the editor's own node tree must be assigned before the call. Driven exactly as that record described it, both operators refuse — `poll` comes back False and the call raises "context is incorrect". With the tree assigned, the original result reproduces exactly, as printed above. The re-run agreed with the record in every other particular. One detail the record does not carry, seen in the re-run: `read_viewlayers` reports an attempt to read a cached image file for `Sub` and that it cannot read it, and still no render follows.)*

**What this settles in this document, and what it does not.**

The step taken in part one and the step taken in part two are both what the runs show. Those labels stay **inferred**, since they describe what the 5.0 sources support, and the observed answers sit beside them.

Part three's labels stay as they are: the sources still say nothing about which scene `-E` lands on, and these sources still describe no command-line sample count. *Observed 7* and *Observed 8* answer the behaviour as a separate claim.

Of the *Not covered* list above, the runs settle the first (`-E` reaches only the scene that is active when it is read) and the fourth (Cycles' Layer Samples policy is read from the pulled-in scene, established by giving the two scenes opposite choices and watching the count follow the pulled-in one in both directions). Adaptive sampling, the Time Limit and Sample Subset were switched off or set aside in these runs rather than examined, so they remain untouched; so does the viewport compositor, and so does the Scene strip.

---

### Not covered

- Whether `-E` on the command line reaches a scene that is pulled in by a Render Layers node, or only the scene being rendered. The sources do not say, and I did not test it.
- Whether the second scene is re-rendered at all as part of the first scene's render, or read from a cache. That is a different question and I did not answer it; my answer is about which settings would govern such a render, not about whether one happens.
- What happens if `-E` names an engine that the second scene could not otherwise use, or an engine that is not available — the sources describe `-E help` as listing available engines but say nothing about mismatch handling.
- Whether Cycles' `Layer Samples` policy is read from the second scene or from the compositing scene. My inference says the second scene, because the setting lives on the scene whose view layers are being sampled and Cycles' own panel walks `scene.view_layers` of the same scene; but no page states it for the cross-scene case.
- Adaptive sampling, the Time Limit, and Sample Subset all change how many samples actually get taken. I treated "sample count" as the configured count and did not chase how these three interact across scenes.
- Whether any of this differs for the viewport compositor as opposed to a final render. The Render Layers page carries a note that the viewport compositor "only supports render passes when using EEVEE", which hints the two paths differ, but I did not pursue it because the question is about what a scene is rendered with, not about passes.
- The Scene strip reading of the question. I read that page and report below what it does and does not offer; I did not build an answer on it.

---

### Outside the interface

*(**Note added by the hand-test step, 2026-08-29.** The tests this document asked for have been run, and their results are now written in — see *What running Blender showed — the hand tests of 2026-08-29* above. Both standing lines below have been brought up to date with them: each keeps what the sources say and gains, marked separately, what a run showed. The citation check's note that the answers were still owed stood here until this was done.)*

Everything documented above is settings data and a command-line option, so it holds the same whether Blender is driven by a person or by something else. Two specifics:

- The command-line option `-E` exists precisely for driving Blender from outside; the manual's whole page on it is titled "Rendering From The Command Line" and says "One advantage of using the command line is that we do not need a graphical display (no need for X server on Linux for example) and consequently we can render via a remote shell (typically SSH)." Source: manual, https://docs.blender.org/manual/en/5.0/advanced/command_line/render.html — read 2026-07-30.
- One thing genuinely differs outside the interface, and it is worth flagging because it is the only difference the sources name in this area. The re-render button on the Render Layers node — "The button next to the dropdown re-renders it immediately" — is an interface control. There is no stated equivalent for a driven render.

Where the sources are silent, they are silent for both cases equally: the manual does not say what `-E` does to a second scene inside or outside the interface, and there is no sample-count option in either.

One further caution, documented, that bears on any driven render: argument order matters, and a value given before the file is loaded can be discarded. "Arguments are executed in the order they are given!" and "blender --background --render-output /tmp test.blend --render-frame 1 …will not render to /tmp because loading the blend-file overwrites the render output that was set." Source: manual, https://docs.blender.org/manual/en/5.0/advanced/command_line/render.html and https://docs.blender.org/manual/en/5.0/advanced/command_line/arguments.html — read 2026-07-30. Those examples are about the output path, not about `-E`; whether `-E` is likewise overwritten by loading the file is **not in the sources**.

**Observed, 2026-08-29.** This line can now be answered for Blender 5.0 rather than left open, on the one operating system it was answered on.

- **A render started from inside a running Blender with a window is the same picture, to the last bit, as one started from the command line.** Blender 5.0.1 was started **without** `-b` — so with a real window and a full interface loaded — and handed a script that made `Comp` the current scene, rendered it and quit. The script printed `GUI_BACKGROUND_FLAG: False`, so Blender really was not in background mode. The written image was compared with the one the same file wrote from a background run: `maxdiff = 0`, and both matched the 512-sample reference at `maxdiff = 0` while differing from the 1-sample reference by 17.5015. **So the engine and the sample count that the scene behind the node is rendered with are the same whether Blender has an interface or not.**
- **The limit on that, stated plainly.** The render was started by a script running inside that windowed Blender, not by a person pressing a key. What it shows is that the render path is the same when Blender has a window and a full interface loaded. It does not show what a keypress does, and nothing in this round could.
- **The one interface-only control this answer names is still unsettled.** The re-render button beside the node's View Layer dropdown was not settled: the operator behind it, driven from a script with the node editor in the right state, cancelled without rendering (*Observed 10* above). A press is a mouse click on a window, and there was no machine in this round that could produce one.
- **Everything else above was driven from outside in the first place.** Every run in the observed section other than that one was a background run started from a terminal, which is the case this project cares about.

### Operating system

Nothing in the answer is stated to be operating-system specific. `-E` and `-S` carry no platform note, while the same page does mark other options as platform-limited ("-con, --start-console … (Windows only)", "--qos <level> … (Windows only)", "--debug-ghost … (Linux only)") — so the absence of a note on `-E` sits alongside notes that are present elsewhere on the page. Sample-count settings are ordinary scene data and carry no platform note either.

The one place where the operating system does enter this area is the render *device*, not the engine and not the sample count: "Valid options are: CPU CUDA OPTIX HIP ONEAPI METAL." Which of those exist depends on the machine. That is a different setting from the two the question asks about, and I raise it only so the boundary is clear. Source: manual, https://docs.blender.org/manual/en/5.0/advanced/command_line/arguments.html — read 2026-07-30.

**Observed, 2026-08-29 — and it settles one operating system and leaves the others open.** Every run in the observed section above was made on **one** machine: the Creator's MacBook Pro, Apple M1 Pro, Apple's own graphics and no second card, macOS 26.6.2 (build 25G83), arm64. Nothing was run on Linux and nothing on Windows; no machine was rented for this round. **Untried rather than untestable: a Windows machine can be rented, and every one this project has read reported a Server edition — see the equipment section of `Preparation/Blender Research/Method.md`.** **So whether any of the observed findings hold on any other operating system is still unknown**, and saying so is the whole of what this line can carry. A test settles the operating system it was run on.

Three places where that gap is worth naming rather than left implicit:

- **The one difference between the four researched versions** — that 4.5 does not honour a view layer's sample override under EEVEE where 5.0, 5.1 and 5.2 do (*Observed 5* above) — was found on this Mac, whose graphics are Apple's own. It is worth repeating on a machine with a different graphics card before it is treated as settled everywhere.
- **The windowed run** behind the *Outside the interface* line above was made on a machine whose viewport is drawn by a real graphics card. It could come out differently on a machine whose graphics are drawn by the processor.
- **No third-party render engine was involved anywhere.** Every run used `--factory-startup`, so no add-on engine was loaded, and none is among this project's equipment.

---

## Where I looked

Searching came first, over the whole of each source folder, then reading.

**Terms searched across the 5.0 manual, API documentation and release notes** (case-insensitive): `another scene`, `other scene`, `different scene`, `second scene`, `per-scene`, `per scene`, `Render Layers`, `render engine`, `Render Engine`, `--engine`, `-E `, `--samples`, `--cycles-samples`, `--max-samples`, `samples`, `Override number of render samples`, `Samples Override`, `Layer Samples`, `per view layer sample`, `override`, `nested render`, `sub-render`, `re-render`, `renders the scene`, `renders that scene`, `will be rendered`, `rendered with`, `its own render settings`, `own settings`, `engine of the scene`, `render engine of`, `active scene`, `current scene`, `command line`, `all scenes`, `every scene`.

**Searched across Cycles' own files:** `samples`, `argparse`, `add_argument`, `sys.argv`, `cycles-device`, `bl_idname`, `use_layer_samples`, `view_layer.samples`, class and registration lines.

**Pages opened and read in full or in the relevant part:**

- Render Layers node — https://docs.blender.org/manual/en/5.0/compositing/types/input/scene/render_layers.html — the whole page. This is where the answer would live if it were documented. It is not; the page lists two properties and then gives an unrelated hint about using another scene's *composited* output via saved EXR files. It says nothing about engine or samples.
- Compositor System — https://docs.blender.org/manual/en/5.0/compositing/compositor_system.html — read in full. Covers data types, image domains, operation domains and output selection. Nothing on engines or samples.
- Compositing introduction — https://docs.blender.org/manual/en/5.0/compositing/introduction.html — read in full. Nothing.
- Compositing limitations — https://docs.blender.org/manual/en/5.0/compositing/limitations.html — read in full. GPU compositor hardware limits only. Ruled out.
- Compositing contents list — https://docs.blender.org/manual/en/5.0/compositing/index.html — scanned the whole node list to check no other input node takes a scene. Only the Render Layers node does.
- Command Line Arguments — https://docs.blender.org/manual/en/5.0/advanced/command_line/arguments.html — read from the top through the environment variables at the end. This is where a sample-count option would be if one existed. It is not there.
- Rendering From The Command Line — https://docs.blender.org/manual/en/5.0/advanced/command_line/render.html — read in full.
- Using Blender From The Command Line, contents — https://docs.blender.org/manual/en/5.0/advanced/command_line/index.html — read to confirm no further sub-page covers engine selection.
- Cycles Sampling — https://docs.blender.org/manual/en/5.0/render/cycles/render_settings/sampling.html — read the Render Samples section and the Layer Samples section.
- EEVEE Sampling — https://docs.blender.org/manual/en/5.0/render/eevee/render_settings/sampling.html — read the Viewport and Render sections.
- View Layer Override — https://docs.blender.org/manual/en/5.0/render/layers/override.html — read in full.
- View Layers introduction — https://docs.blender.org/manual/en/5.0/render/layers/introduction.html — read the "Rendering & Compositing" section.
- Rendering introduction — https://docs.blender.org/manual/en/5.0/render/introduction.html — read in full.
- Scenes introduction — https://docs.blender.org/manual/en/5.0/scene_layout/scene/introduction.html — read in full, including the "Scenes in the Video Sequencer" section.
- Stereoscopy usage — https://docs.blender.org/manual/en/5.0/render/output/properties/stereoscopy/usage.html — read the Compositor section, because it is the one place the manual talks about what the compositing scene decides "regardless of … Render Layers from different scenes". It names views and resolution only.
- Scene Strip — https://docs.blender.org/manual/en/5.0/video_editing/edit/montage/strips/scene.html — read in full, for the second reading of the question. Its options are Scene, Input (Camera or Sequencer), a Camera override, Show flags and Strip Volume. It offers a camera override and no engine or sample override, and its stated limitation is about passes: "Scene strips do not render individual Render Passes; only the Combined render pass will be used." Nothing about engine or samples.
- Node editors sidebar — https://docs.blender.org/manual/en/5.0/interface/controls/nodes/node_editors.html — checked because its example screenshot is described as "Node tab with a compositing Render Layers node selected", in case the sidebar showed extra per-node settings. The listed sidebar fields are Name, Label and Warning Propagation. Nothing relevant.
- Node editing operators — https://docs.blender.org/manual/en/5.0/interface/controls/nodes/editing.html — read the "Read View Layers" entry, which mentions reading "all the current scene's render layers from cache". It concerns caching, not engine or samples. Ruled out.

**Pictures opened:**

- `_images/node-types_CompositorNodeRLayers.webp`, from the Render Layers node page. It shows the node with a Scene field and a View Layer dropdown and no other controls.

**API pages read:**

- https://docs.blender.org/api/5.0/bpy.types.CompositorNodeRLayers.html — full property list; only `layer` and `scene`.
- https://docs.blender.org/api/5.0/bpy.types.RenderSettings.html — `engine`, and `has_multiple_engines` ("More than one rendering engine is available").
- https://docs.blender.org/api/5.0/bpy.types.Scene.html — `render` and `cycles` pointers.
- https://docs.blender.org/api/5.0/bpy.types.ViewLayer.html — `samples`.
- https://docs.blender.org/api/5.0/bpy.types.SceneEEVEE.html — `taa_render_samples`, `taa_samples`.
- https://docs.blender.org/api/5.0/bpy.ops.render.html — the `render()` operator's parameters.
- https://docs.blender.org/api/5.0/bpy.types.RenderEngine.html — scanned every mention of "scene" in it, looking for anything that says which scene an engine instance serves. Its example code reads `scene = depsgraph.scene`, and its methods take a depsgraph. Nothing states how the engine class is chosen for a given render. Ruled out as not answering the question.

**Release notes read:**

- https://developer.blender.org/docs/release_notes/5.0/python_api/ — the Nodes section, for compositor changes. It records that `scene.node_tree` was removed in favour of `scene.compositing_node_group`, and that "EEVEE's render engine identifier was changed from `BLENDER_EEVEE_NEXT` to `BLENDER_EEVEE`." Neither changes the answer, but the second is worth carrying: the identifier a `-E` argument would use for EEVEE in 5.0 is `BLENDER_EEVEE`.
- https://developer.blender.org/docs/release_notes/5.0/compositor/ — read the whole Changed and Removed sections. Nothing about which engine or sample count a pulled-in scene uses.
- https://developer.blender.org/docs/release_notes/5.0/core/ — the Logging section and the command-line flag table. Records that `--debug-cycles` became `--log cycles`, `--verbose` became `--log-level`, and that `--gpu-vsync` was added. No engine or sample argument added or changed.
- Grepped the remaining 5.0 release-note files for `render layers`, `render engine`, `--engine`, `samples`, `command line`. The only other hits were unrelated (a geometry-nodes tangent note, a Freestyle-style bugfix line, a constraint that samples attributes).

**Cycles files read:**

- `Blender Sources/cycles_5.0/engine.py` — the argument parser, lines 8–41.
- `Blender Sources/cycles_5.0/properties.py` — `enum_use_layer_samples` (64–67), `CyclesRenderSettings` (from 378), `samples` (479–484), `use_layer_samples` (550–555), the scene registration (1101–1107), and `CyclesRenderLayerSettings` in full (1479–1555) to check whether Cycles adds a per-view-layer sample setting of its own. It does not — the per-view-layer `samples` is Blender's own `ViewLayer.samples`; Cycles' view-layer settings are passes and denoising only.
- `Blender Sources/cycles_5.0/ui.py` — the sampling panel draw method (295–318) and the advanced sampling panel (458–476).
- `Blender Sources/cycles_5.0/__init__.py` — the `CyclesRender` class (42–120).
- Checked the shape of `Blender Sources/cycles_5.0` as a whole: it holds the Python add-on plus `source/kernel`, `source/util` and `shader`. There is no Blender-session C++ code in it, so the place where Blender decides which engine to instantiate for a render is not in the files I was given. That is why part one is inferred rather than observed.

**What I ruled out and why.**

- The whole `video_editing/` tree beyond the Scene Strip page — it belongs to the other reading of the question, and the one page that could carry the answer does not.
- The `render/cycles/gpu_rendering` and device material — device is not engine and not sample count.
- The output and post-processing pages — they decide whether the compositor runs and what is written, not what a scene is rendered with.
- Anything under `manual_4.5`, `api_4.5`, `release_notes_4.5`, `cycles_4.5`, and the 5.1 and 5.2 copies — the question is about 5.0 and I read only the 5.0 folders.

**Where I would have to go and did not.** Nothing. I did not need the web and did not use it.

---

## Things a person with Blender 5.0 open could settle in a few minutes

*(**Run, and pruned — 2026-08-29.** Every test this list held was carried out on the Creator's MacBook Pro on 2026-08-29, and repeated on the same machine the same day. The answers are written into *What running Blender showed — the hand tests of 2026-08-29* above, which is where they now live; the answered items have therefore been taken off this list rather than left standing as work for somebody to do. What was set up, what was typed and what the machine printed is recorded in full in `Audits/Blender Research/The compositor/Hand tests — Architecture Fact 135.md`. This list's items were, in that record's numbering, entries 6 and 7 (item 1), 10 (item 2), 11 (item 3), 14 (item 4), 3 (item 5) and 2 (item 6).)*

**Nothing from this list is still owed.** Four things were not run at all, and they are recorded in that same document: a real press of the re-render button on the Render Layers node — the operator behind it was driven and cancelled without rendering, see *Observed 10* above; anything on Windows; everything above on a second operating system; and any third-party render engine, since every run used `--factory-startup`.

---

2026-08-29 — Claude Opus 5 (Claude Code) — The compositor, Blender 5.0.

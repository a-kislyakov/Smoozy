# The compositor — Blender 5.2 — Question 9 (second pass)

**Sources consulted, and on what date.** On 2026-08-29 I read, from the snapshot taken on 2026-07-30: the Blender 5.2 manual (`Blender Sources/manual_5.2`, including one image from its `_images` folder), the Blender 5.2 Python API documentation (`Blender Sources/api_5.2`), the Blender 5.2 release notes (`Blender Sources/release_notes_5.2`), and the Cycles add-on's own files (`Blender Sources/cycles_5.2`). Nothing else.

I read only the question sheet and files inside Blender Sources/. I did not open any other part of this project.

I started no helper agents. Every search and every reading below is my own.

---

## 9. When one scene takes in another scene's rendered result, what decides the render engine and the sample count that the second scene is rendered with, and what effect on that does a value for either, given to the render from outside, have?

### How I read the question

This is the compositor sheet, and question 4 used the same phrase ("takes in another scene's rendered result") about the compositor. So I answer the reading I believe is meant: **scene A's compositor contains a Render Layers node whose Scene field points at scene B**, and the question is what engine and what sample count scene B gets rendered with.

The question is ambiguous in two places, and I flag both rather than choose silently:

- **"takes in another scene's rendered result"** could also mean a Video Sequencer *Scene strip*, which the manual describes as taking in another scene's render. I checked that page too and report what I found, but I treat the compositor reading as the one meant.
- **"given to the render from outside"** I read as *supplied from outside the blend-file when the render is launched* — that is, a command-line argument. The other reading is *from outside the second scene* — i.e. does scene A's own engine or sample count reach scene B. I address both, because the sources land in the same place for each.

### LABEL

**Mixed. The parts, separately:**

- Where the engine setting lives, and where the sample count lives: **documented** (manual and API), with **observed** corroboration from Cycles' own files.
- That the second scene is therefore rendered with *its own* engine and *its own* sample count: **inferred**. No source states this. The step is written out below.
- What a command-line engine value (`-E`) does to a scene pulled in by a Render Layers node: **not in the sources**.
- Whether any sample count can be given from outside at all: **not in the sources** for the manual and API; **observed** in Cycles' own files that Cycles' own argument parser declares no such command-line option. *(Narrowed by the citation check, 2026-08-29; it read "that Cycles accepts no such command-line option". What the quoted lines show is what the parser declares. What a run does with an argument the parser has not declared is a step past the file.)*

### The answer

**1. The Render Layers node carries no engine setting and no sample setting of its own.**

The node has exactly two properties. The manual:

> "Properties
>
> Scene
>
> The scene for which to render a view layer.
>
> View Layer
>
> The view layer to render. The button next to the dropdown re-renders it immediately."

— Manual, *Render Layers Node*, https://docs.blender.org/manual/en/5.2/compositing/types/input/scene/render_layers.html — read 2026-07-30.

The same page's opening line says what the node does: "Renders a View Layer and reads its Passes into the compositing node graph."

The picture on that page (`_images/node-types_CompositorNodeRLayers.webp`, shown on the page above) shows the node with an Image output, an Alpha output, a Scene field reading "Scene" with an X button, and a ViewLayer dropdown with a small button beside it. Nothing else is visible on the node. **Documented** (a manual image): there is no engine control and no samples control on the node face.

The API listing agrees. The whole class has two properties:

> "class bpy.types.CompositorNodeRLayers(CompositorNode)
>
> Input render passes from a scene render
>
> layer
>
> (default 'PLACEHOLDER')
> …
> scene
>
> Type:
>
> Scene | None"

— Python API, *CompositorNodeRLayers*, https://docs.blender.org/api/5.2/bpy.types.CompositorNodeRLayers.html — read 2026-07-30.

So nothing at the node decides either value.

**2. The engine is a property of a scene.**

> "engine
>
> Engine to use for rendering (default 'BLENDER_EEVEE')"

— Python API, *RenderSettings*, https://docs.blender.org/api/5.2/bpy.types.RenderSettings.html — read 2026-07-30.

And `RenderSettings` is reached through a scene:

> "render
>
> (readonly, never None)
>
> Type:
>
> RenderSettings"

— Python API, *Scene*, https://docs.blender.org/api/5.2/bpy.types.Scene.html — read 2026-07-30.

The manual says where a person sets it:

> "The active render engine can be selected in the Render Properties."

— Manual, *Render Engines*, https://docs.blender.org/manual/en/5.2/render/render_engines.html — read 2026-07-30.

and says whose properties those are:

> "Tabs related to the active scene:
>
> Render: EEVEE,
> Cycles, or
> Workbench settings."

— Manual, *Properties Editor*, https://docs.blender.org/manual/en/5.2/editors/properties_editor.html — read 2026-07-30.

A worked example in the API documentation reads the engine off a scene the same way: "return context.scene.render.engine == 'CYCLES'" — Python API, *NodeTree*, https://docs.blender.org/api/5.2/bpy.types.NodeTree.html — read 2026-07-30.

**3. The sample count is a property of a scene, per engine, with a per-view-layer override.**

Cycles. The manual:

> "Reference
>
> Panel:
>
> Render ‣ Sampling"

and

> "Render (Max) Samples
>
> The number of paths to trace per pixel in the final render.
> A higher number results in a cleaner image at the cost of a longer render time."

— Manual, *Sampling* (Cycles), https://docs.blender.org/manual/en/5.2/render/cycles/render_settings/sampling.html — read 2026-07-30.

**Observed** in Cycles' own files, the setting itself:

> `    samples: IntProperty(`
> `        name="Samples",`
> `        description="Number of samples to render for each pixel",`
> `        min=1, max=(1 << 24),`
> `        default=4096,`
> `    )`

— file `Blender Sources/cycles_5.2/properties.py`, lines 485–490, inside `class CyclesRenderSettings(bpy.types.PropertyGroup)` (which begins at line 384) — read 2026-07-30.

That class is attached to the scene:

> `    @classmethod`
> `    def register(cls):`
> `        bpy.types.Scene.cycles = PointerProperty(`
> `            name="Cycles Render Settings",`
> `            description="Cycles render settings",`
> `            type=cls,`
> `        )`

— file `Blender Sources/cycles_5.2/properties.py`, lines 1140–1146 — read 2026-07-30.

And the panel that draws it reads it off the current scene:

> `        scene = context.scene`
> `        cscene = scene.cycles`

— file `Blender Sources/cycles_5.2/ui.py`, lines 298–299, in `class CYCLES_RENDER_PT_sampling_render` (begins line 288); the field itself is drawn at line 313 as `col.prop(cscene, "samples", text="Max Samples")` — read 2026-07-30.

EEVEE. The manual, under a "Render" heading:

> "Samples
>
> The number of samples to use in the final render."

— Manual, *Sampling* (EEVEE), https://docs.blender.org/manual/en/5.2/render/eevee/render_settings/sampling.html — read 2026-07-30. Its reference block also reads "Panel: Render ‣ Sampling".

The API names it and ties it to a scene:

> "taa_render_samples
>
> Number of samples per pixel for rendering (in [1, inf], default 64)"

— Python API, *SceneEEVEE*, https://docs.blender.org/api/5.2/bpy.types.SceneEEVEE.html — read 2026-07-30; and "eevee / EEVEE settings for the scene (readonly) / Type: SceneEEVEE | None" — Python API, *Scene*, https://docs.blender.org/api/5.2/bpy.types.Scene.html — read 2026-07-30.

Workbench. The manual:

> "The setting for viewport rendering
> and final rendering is saved per scene."

— Manual, *Sampling* (Workbench), https://docs.blender.org/manual/en/5.2/render/workbench/sampling.html — read 2026-07-30.

The per-view-layer override. The Render Layers node names a view layer as well as a scene, and a view layer can carry its own sample count:

> "Samples
>
> Allows the view layer to use a different number of samples than the main scene.
> This is controlled by the Layer Samples
> when using the Cycles renderer. When using EEVEE, the Samples override is always used."

and, on the same page:

> "Overrides are applied only when rendering the specific view layer and do not affect the scene globally."

— Manual, *View Layer Override*, https://docs.blender.org/manual/en/5.2/render/layers/override.html — read 2026-07-30.

The API:

> "samples
>
> Override number of render samples for this view layer, 0 will use the scene setting (in [0, inf], default 0)"

— Python API, *ViewLayer*, https://docs.blender.org/api/5.2/bpy.types.ViewLayer.html — read 2026-07-30.

Under Cycles, the scene decides whether that override is honoured:

> "Layer Samples
>
> If any view layers have Sample Overrides configured,
> this option specifies how to use them.
>
> Use:
>
> Allow view layers to override the scene-level sample count.
>
> Bounded:
>
> Allow the overrides as long as they don't exceed the scene-level sample count.
>
> Ignore:
>
> Ignore the overrides."

— Manual, *Sampling* (Cycles), https://docs.blender.org/manual/en/5.2/render/cycles/render_settings/sampling.html — read 2026-07-30.

**Observed**, that switch and its wording in Cycles' own files:

> `enum_use_layer_samples = (`
> `    ('USE', "Use", "Per render layer number of samples override scene samples"),`
> `    ('BOUNDED', "Bounded", "Bound per render layer number of samples by global samples"),`
> `    ('IGNORE', "Ignore", "Ignore per render layer number of samples"),`

— file `Blender Sources/cycles_5.2/properties.py`, lines 64–67 — read 2026-07-30. The property `use_layer_samples` that uses it is at lines 556–560 of the same file, inside the scene-level `CyclesRenderSettings` class.

And it is drawn only when some view layer of *that* scene has an override set:

> `        for view_layer in scene.view_layers:`
> `            if view_layer.samples > 0:`
> `                layout.separator()`
> `                layout.row().prop(cscene, "use_layer_samples")`
> `                break`

— file `Blender Sources/cycles_5.2/ui.py`, lines 478–482 — read 2026-07-30. *(Line numbers corrected by the citation check, 2026-08-29; the document said 477–481, which starts one line early and stops before the `break`. The words are unchanged.)*

**4. The step I take, and it is a step (inferred).**

No source I found says, in so many words, whose engine and whose sample count are used when scene A's compositor pulls in scene B. What the sources say is: the node names scene B and a view layer of scene B and renders that view layer; the engine is a property of a scene; the sample count is a property of a scene (per engine) plus an optional override on a view layer of that scene. From that I infer: **scene B is rendered with the engine set in scene B's own Render Properties, and with the sample count set in scene B's own Render Properties for that engine, further modified by any sample override on the named view layer of scene B, subject to scene B's Layer Samples switch under Cycles.** A reader has to supply the joining step "the settings used are the ones belonging to the scene being rendered", so this is **inferred**, not documented.

**5. What a value given from outside does — reading it as a command-line value.**

There is a command-line argument for the engine. The whole of what the sources say about it:

> "-E, --engine <engine>
>
> Specify the render engine.
> Use -E help to list available engines."

— Manual, *Command Line Arguments*, https://docs.blender.org/manual/en/5.2/advanced/command_line/arguments.html — read 2026-07-30.

and, in the worked example:

> "-E CYCLES
>
> Use the "Cycles Render" engine.
> For a list of available render engines, run blender -E help."

— Manual, *Rendering From The Command Line*, https://docs.blender.org/manual/en/5.2/advanced/command_line/render.html — read 2026-07-30.

Neither passage says which scene or scenes it applies to. It does not say "the active scene", it does not say "all scenes", and it does not mention the compositor or scenes reached through it. **So: whether `-E` reaches a second scene pulled in by a Render Layers node is not in the sources.** I will not guess. Note that the same manual page is explicit about scope for a neighbouring argument when it wants to be — `-F` is described as "Override the image format specified in the blend-file" — and about the active scene for another — "-S, --scene <name> / Set the active scene <name> for rendering" — which makes the silence on `-E` a silence, not an omission I can fill.

These sources name no command-line argument for a sample count. *(Narrowed by the citation check, 2026-08-29; the sentence read "There is no command-line argument for a sample count", which is a statement about Blender rather than about the page quoted under it.)* The Command Line Arguments page lists, for Cycles, only two options, and neither is samples:

> "Cycles Render Options
>
> Cycles add-on options must be specified following a double dash.
>
> --cycles-device <device>
> …
> --cycles-print-stats"

— Manual, *Command Line Arguments*, https://docs.blender.org/manual/en/5.2/advanced/command_line/arguments.html — read 2026-07-30.

**Observed**, in Cycles' own files, that these are the only two options the add-on's own argument parser declares: *(Narrowed by the citation check, 2026-08-29; it read "the only two Cycles command-line options that exist".)*

> `    parser.add_argument("--cycles-print-stats",`
> `                        help="Print rendering statistics to stderr",`
> `                        action='store_true')`
> `    parser.add_argument("--cycles-device",`
> `                        help="Set the device to use for Cycles, overriding user preferences and the scene setting."`

— file `Blender Sources/cycles_5.2/engine.py`, lines 13–18, in `_configure_argument_parser()` (begins line 8); the parser is built there and used in `_parse_command_line()` at lines 24–41, and that function is called once from `init()` at line 53 — read 2026-07-30. *(Line numbers corrected by the citation check, 2026-08-29; the document said 14–18, but the quote begins at `parser.add_argument("--cycles-print-stats",` on line 13. The words are unchanged.)*

Worth noting for the record, because it is the one place the sources *do* state the scope of an outside value: the `--cycles-device` help text says it overrides "user preferences and the scene setting". Nothing comparable is written for the engine, and no samples option exists to write it for.

The Python side has no such value either. The render operator takes a scene and a view layer but no engine and no sample count:

> "bpy.ops.render.render(*, animation=False, write_still=False, use_viewport=False, use_sequencer_scene=False, layer='', scene='', frame_start=0, frame_end=0)"

— Python API, *bpy.ops.render*, https://docs.blender.org/api/5.2/bpy.ops.render.html — read 2026-07-30.

**6. The other reading of "from outside" — from outside the second scene.**

Whether scene A's engine or scene A's sample count has any effect on how scene B is rendered: **not in the sources.** Nothing on the Render Layers page, the compositor pages, the render engine page or the sampling pages says that the calling scene's settings reach the called scene, or that they do not.

---

### What running Blender showed — the hand tests of 2026-08-29

**LABEL: observed**, for everything in this section and for nothing above it.

**These are two separate claims and they are kept apart.** Everything above this heading is what the Blender 5.2 sources say, and it stands unchanged — including every *not in the sources*, which is a statement about the documentation and is not undone by a run. What follows is the other claim: what Blender 5.2 actually did. Where the two meet, both are recorded — the reading as a reading, the run as a run.

**On what kind of computer, and when.** The Creator's own MacBook Pro: an Apple M1 Pro chip, Apple's own graphics, no second card. Read off the machine on the day:

```
ProductName:		macOS
ProductVersion:		26.6.2
BuildVersion:		25G83
arm64
Apple M1 Pro
```

The Blender was `/Applications/Blender.app/Contents/MacOS/Blender`, which reports:

```
Blender 5.2.1 LTS (hash 9e2066aef7ef built 2026-08-25 01:35:57)
```

Every run was made on **2026-08-29**, and every one was repeated on the same machine the same day by the citation check, with the same result each time. Every run used `--factory-startup`, so no third-party add-on was loaded and no add-on render engine existed in any of them. **Nothing was run on any other operating system** — see the operating-system line below.

**What was run against.** One set of files, built by Blender 5.2.1 LTS itself so that no file crossed a version boundary. Each holds two scenes: **`Comp`**, whose compositor is being run, and **`Sub`**, the scene pulled in. Both hold a camera, one area light, a metallic sphere of roughness 0.35 and a blue floor, with the two spheres in different places so the two scenes are not the same picture. `Comp`'s compositor holds one **Render Layers** node with its Scene set to `Sub` and its View Layer set to `Sub`'s own `ViewLayer`, wired straight to the output and nothing else — so the written image is entirely `Sub`'s render, and any change in it is a change in how `Sub` was rendered. Both scenes: 160 by 120 pixels, one frame, uncompressed 32-bit OpenEXR, compositing on, stamping off, Cycles on the processor with denoising off, adaptive sampling off, no time limit and a fixed seed of 7, so a Cycles render repeats to the bit. Only the engines, sample counts, view-layer overrides and Layer Samples choices differ from file to file, and each finding below names the file it used. The script that builds them is printed in full in `Audits/Blender Research/The compositor/Hand tests — Architecture Fact 135.md`.

Every run took this shape, with the file, the extra arguments and the output path changing:

```
"/Applications/Blender.app/Contents/MacOS/Blender" --factory-startup -b "<file>.blend" --log "render" -o "<output>_" -f 1
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

The zeros are exact zeros. (The two non-zero figures are recorded as the same across all four researched versions to within the third decimal place; the zeros are Blender 5.2's own.)

**So the scene pulled in through the node is rendered with the engine stored on that scene**, in both directions and for all three engines: Workbench inside a Cycles render, Cycles inside a Workbench render, EEVEE inside a Cycles render and Cycles inside an EEVEE render. The four files were built so that the opposite answer would have shown as each image matching the *other* reference, and none did.

One thing found in passing, worth carrying because it bears on what "the second scene is rendered with" even means. Only **one** `Start rendering:` line appeared in any of these runs, and it always named `Sub`. `Comp`'s own view layer was never rendered at all — nothing in its compositor asks for it. So the render of the compositing scene consisted of rendering the other scene and then running the compositor.

---

**Observed 2 — under Cycles, the sample count is the second scene's own, and the compositing scene's count does not leak into it.**

Two files with the two counts swapped, and two references made with `-S Sub`:

| file | `Comp` at | `Sub` at | last sample line printed | vs `Sub`-at-512 | vs `Sub`-at-1 |
| --- | --- | --- | --- | --- | --- |
| `smp_A1_B512` | 1 | 512 | `Sample 512/512` | **0** | 17.5005 |
| `smp_A512_B1` | 512 | 1 | `Sample 1/1` | 17.5005 | **0** |

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

Blender 5.2.1 LTS printed `Rendering 8 / 8 samples`, and the written image matched the 8-sample reference at `maxdiff = 0` and differed from the 64-sample reference, at `maxdiff` 2.617.

**Blender 5.2.1 LTS honours it. Blender 4.5 does not** — this is the one place in the whole round where the four researched versions differ, and each of the four is carrying the difference so that nobody later mistakes it for something else. The same test in 4.5 printed `Rendering 64 / 64 samples` and matched the 64-sample reference exactly, ignoring the override.

**It is not a difference in what the compositor does, and this was established rather than assumed.** The same file was also rendered with `-S Sub`, so `Sub` was the active scene and no compositor was involved at all. In 5.2.1 LTS that render came out `maxdiff = 0` against the 8-sample reference too, and in 4.5 against the 64-sample one. So what differs is whether EEVEE honours a view layer's sample override at all in a given version; the path through the node reports faithfully whichever way that goes.

**It was re-run before being written down**, because it was the round's only disagreement between versions. Blender 5.2.1 LTS was run again on the same file and printed `Rendering 8 / 8 samples`.

---

**Observed 6 — a scene reached through the node renders exactly as it does when it is the scene being rendered.**

Ten of the files above were rendered twice over: once through the node with `Comp` active, and once directly with `-S Sub` so that no compositor was involved. Those ten — `eng_Awb_Bcy`, `eng_Acy_Bwb`, `eng_Acy_Bee`, `smp_A1_B512`, `smp_A512_B1`, `smp_A1_B128`, `vl_use`, `ee_A4_B64`, `ee_A64_B4` and `ee_vl` — cover all three engines, four Cycles sample counts, two EEVEE sample counts, and a view-layer override under each of the two engines.

**Every one of the ten pairs came out `maxdiff = 0`.** Reported here for the hardest case, the EEVEE override file, because that is the one the versions disagree about:

```
5.2   ee_vl (through node) vs ref_ee_vl_direct   maxdiff=0 meandiff=0
```

Any difference at all, in any pixel, would have meant the compositor path renders the second scene with something other than the settings it has when rendered on its own. There was none. This is the strongest form of the answer, because it does not depend on knowing which setting did what.

---

**Observed 7 — an engine given on the command line does not reach the scene behind the node.**

Three files in which **both** scenes carry the same engine, so that any difference afterwards comes from the argument rather than from what was stored: `E_bothcy` (both Cycles at 64), `E_bothwb` (both Workbench), `E_bothee` (both EEVEE at 16). Four runs, `Comp` active in each, of the form:

```
"/Applications/Blender.app/Contents/MacOS/Blender" --factory-startup -b E_bothcy.blend --log "render" -E BLENDER_WORKBENCH -o "<output>_" -f 1
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

and nothing was rendered. Blender 5.2.1 LTS exited **1**.

---

**Observed 8 — a sample count given from outside: no argument sets one, one offered anyway is silently ignored, and a script does reach it.**

**What the program says it accepts.** `--help` in Blender 5.2.1 LTS lists, under *Render Options*, exactly `-b/--background`, `-a/--render-anim`, `-S/--scene`, `-f/--render-frame`, `-s/--frame-start`, `-e/--frame-end`, `-j/--frame-jump`, `-o/--render-output`, `-E/--engine`, `-t/--threads`; and under *Cycles Render Options*, exactly `--cycles-device` and `--cycles-print-stats`. The *Cycles Render Options* section, quoted from Blender 5.2.1's own help exactly as it printed:

```
Cycles Render Options:
	Cycles add-on options must be specified following a double dash.

--cycles-device <device>
	Set the device used for rendering.
	Valid options are: 'CPU' 'CUDA' 'OPTIX' 'HIP' 'ONEAPI' 'METAL'.

	Append +CPU to a GPU device to render on both CPU and GPU.

	Example:
	# blender -b file.blend -f 20 -- --cycles-device OPTIX
--cycles-print-stats
	Log statistics about render memory and time usage.
```

**Nothing in either section sets a sample count.** The program's own account of itself therefore agrees with the reading above, which had rested on absence from a page.

Found in the same place, and worth recording because more than one of these documents told whoever ran their tests to look for such an argument rather than guess one: this build carries a `--log <match>` argument and a `--log-level <level>` argument, under a *Logging Options* heading. `--log "render"` is what produced every engine and sample line quoted in this section.

**What the engine names are.** `-E help` in Blender 5.2.1 LTS printed:

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

The operator behind the button beside the View Layer dropdown was found by asking Blender what operators exist rather than guessing a name, which printed `['cryptomatte_layer_add', 'cryptomatte_layer_remove', 'read_viewlayers', 'render_changed']`. In a Blender 5.2.1 LTS started **with a window** from a terminal, one area was turned into a node editor, its tree type set to the compositor, **its node tree set to `Comp`'s own compositing tree**, and the Render Layers node selected and made active. The editor reported itself ready:

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

**No render happened** — no scene line, no engine line and no sample line appeared in the log. So this establishes what that operator returns when it is driven this way, and nothing about what the button does when a person presses it. A press is a mouse click on a window, and nothing available for this round could produce one. **What the button renders with is unsettled in Blender 5.2.**

*(This one was re-run on 2026-08-29 rather than copied, because the record of the original run was a step short of repeatable: it did not say that the editor's own node tree must be assigned before the call. Driven exactly as that record described it, both operators refuse — `poll` comes back False and the call raises "context is incorrect". With the tree assigned, the original result reproduces exactly, as printed above. The re-run agreed with the record in every other particular. One detail the record does not carry, seen in the re-run: `read_viewlayers` reports an attempt to read a cached image file for `Sub` and that it cannot read it, and still no render follows.)*

**What this settles in this document, and what it does not.**

The **inferred** step in the LABEL summary — that the second scene is rendered with its own engine and its own sample count — is what the runs show, and the label stays as it is because it describes what the 5.2 sources support.

The two *not in the sources* labels stay as well. *Observed 7* answers what `-E` does without changing what the sources say about it, and *Observed 8* does the same for the sample count: no such argument exists in the program's own help, one offered anyway is taken and ignored in silence, and a script given on the command line is the route that does reach the scene behind the node.

Item 6 of the hand-test list below — whether scene A's sample count leaks into scene B — is answered flatly: it does not. Changing only the compositing scene's count between 1 and 4096, with the pulled-in scene fixed at 128, produced `maxdiff=0 meandiff=0` between the two written images.

Of the *Not covered* list above, the runs settle the first, and they settle nothing about when the second scene is rendered, about third-party engines, or about the Scene strip. The document's note that "I did not test anything" was true of the reading it describes; this section is the part that did, and it is kept separate from it for that reason.

---

### Not covered

- I could not settle what `-E` applies to. Specifically: whether an engine given on the command line replaces the engine of every scene in the file, only the scene being rendered, or only the active scene, and therefore whether it reaches a scene pulled in through a Render Layers node. The sources do not say.
- I could not settle whether the second scene is rendered at all during a final render of the first, or only when the node's re-render button is pressed. That is question 4's ground and I did not answer it here; my answer describes *what settings would apply*, not *when the render happens*.
- I did not find any statement about what happens if the second scene's engine differs from the first scene's — for instance whether the compositor accepts passes from a scene on a different engine. The one note on the Render Layers page about engines concerns the *viewport* compositor only: "The viewport compositor only supports render passes when using EEVEE. For other engines, the passes will be empty."
- I did not test anything. Nothing here is observed from a running Blender; the "observed" findings are readings of Cycles' supplied files, not of a render.
- Third-party engines supplied as add-ons are outside what I can see. The manual says "More renderers from third-party developers are available as add-ons. Each renderer has its own render settings" — so where a third-party engine keeps its sample count is not something these sources fix.
- The Video Sequencer reading: the *Scene Strip* page lists a Scene field, a View Layer field, an Input choice, and a Camera override — "This can be used to override the scene's camera with any other object" — but no engine control and no samples control (Manual, *Scene Strip*, https://docs.blender.org/manual/en/5.2/video_editing/edit/montage/strips/scene.html, read 2026-07-30). So that reading lands in the same place. I did not pursue it further, since the topic is the compositor.

### Outside the interface

*(**Note added by the hand-test step, 2026-08-29.** The tests this document asked for have been run, and their results are now written in — see *What running Blender showed — the hand tests of 2026-08-29* above. Both standing lines below have been brought up to date with them: each keeps what the sources say and gains, marked separately, what a run showed. The citation check's note that the answers were still owed stood here until this was done.)*

The engine and both sample counts are settings stored in the scene inside the blend-file, so nothing about *where they live* depends on a person clicking. But there is a real difference, and it runs the other way: **the one place the sources describe an engine value arriving from anywhere other than a scene is the command line**, which exists only when Blender is driven from outside its own window. `-E` has no counterpart inside the interface. And what `-E` does to a scene reached through the compositor is exactly the thing the sources do not say — so on this question the outside-the-interface case is the *less* documented one, not the more.

Two further things the sources do say that bear on the outside case. First, argument order matters: "Arguments are executed in the order they are given!" and "Always position -f or -a as the last arguments" (Manual, *Rendering From The Command Line*, https://docs.blender.org/manual/en/5.2/advanced/command_line/render.html, read 2026-07-30) — so an engine given after the render argument would not be in force for that render. Second, the release notes confirm that rendering a second scene through the compositor does happen in background mode, in a fixed-bug line:

> "Cycles crashes while trying to render second scene in GPU compositor (-b mode)"

— Release notes, *Fixes*, https://developer.blender.org/docs/release_notes/5.2/bugfixes/ — read 2026-07-30. That is a statement that the situation exists and was crashing before the fix; it says nothing about engine or samples.

**Observed, 2026-08-29.** This line can now be answered for Blender 5.2 rather than left open, on the one operating system it was answered on.

- **A render started from inside a running Blender with a window is the same picture, to the last bit, as one started from the command line.** Blender 5.2.1 LTS was started **without** `-b` — so with a real window and a full interface loaded — and handed a script that made `Comp` the current scene, rendered it and quit. The script printed `GUI_BACKGROUND_FLAG: False`, so Blender really was not in background mode. The written image was compared with the one the same file wrote from a background run: `maxdiff = 0`, and both matched the 512-sample reference at `maxdiff = 0` while differing from the 1-sample reference by 17.5005. **So the engine and the sample count that the scene behind the node is rendered with are the same whether Blender has an interface or not.**
- **The limit on that, stated plainly.** The render was started by a script running inside that windowed Blender, not by a person pressing a key. What it shows is that the render path is the same when Blender has a window and a full interface loaded. It does not show what a keypress does, and nothing in this round could.
- **The one interface-only control this answer names is still unsettled.** The re-render button beside the node's View Layer dropdown was not settled: the operator behind it, driven from a script with the node editor in the right state, cancelled without rendering (*Observed 10* above). A press is a mouse click on a window, and there was no machine in this round that could produce one.
- **Everything else above was driven from outside in the first place.** Every run in the observed section other than that one was a background run started from a terminal, which is the case this project cares about.

### Operating system

The sources do not tie the choice of engine, or the sample count, to any operating system. I found no OS-conditional wording on the Render Layers page, the Render Engines page, either Sampling page, the View Layer Override page, or the Command Line Arguments page.

The one OS-dependent thing nearby concerns the rendering *device*, not the engine or the samples: "CUDA is supported on Windows and Linux…", "OptiX is supported on Windows and Linux…" (Manual, *GPU Rendering*, https://docs.blender.org/manual/en/5.2/render/cycles/gpu_rendering.html, read 2026-07-30), and the `--cycles-device` option's valid values "CPU CUDA OPTIX HIP ONEAPI METAL" span platform-specific backends. That is a different setting from the two this question asks about, and I do not extend it to them.

**Observed, 2026-08-29 — and it settles one operating system and leaves the others open.** Every run in the observed section above was made on **one** machine: the Creator's MacBook Pro, Apple M1 Pro, Apple's own graphics and no second card, macOS 26.6.2 (build 25G83), arm64. Nothing was run on Linux and nothing on Windows; no machine was rented for this round. **Untried rather than untestable: a Windows machine can be rented, and every one this project has read reported a Server edition — see the equipment section of `Preparation/Blender Research/Method.md`.** **So whether any of the observed findings hold on any other operating system is still unknown**, and saying so is the whole of what this line can carry. A test settles the operating system it was run on.

Three places where that gap is worth naming rather than left implicit:

- **The one difference between the four researched versions** — that 4.5 does not honour a view layer's sample override under EEVEE where 5.0, 5.1 and 5.2 do (*Observed 5* above) — was found on this Mac, whose graphics are Apple's own. It is worth repeating on a machine with a different graphics card before it is treated as settled everywhere.
- **The windowed run** behind the *Outside the interface* line above was made on a machine whose viewport is drawn by a real graphics card. It could come out differently on a machine whose graphics are drawn by the processor.
- **No third-party render engine was involved anywhere.** Every run used `--factory-startup`, so no add-on engine was loaded, and none is among this project's equipment.

---

## Record of where I looked

**Folders searched in full, by text search, not by opening files one at a time:** `Blender Sources/manual_5.2` (all `.txt`), `Blender Sources/api_5.2` (all 1982 `.txt` files), `Blender Sources/release_notes_5.2` (all `.md`), `Blender Sources/cycles_5.2` (all `.py`).

**Terms I searched for, across manual and API together unless noted:**
`--engine`, `-E ` (with trailing space, to catch the argument rather than key-shortcut text), `engine`, `render engine`, `the scene's render engine`, `scene's engine`, `engine override`, `override engine`, `overriding`, `command line`, `command-line`, `samples`, `sample`, `sample override`, `Sample Overrides`, `Layer Samples`, `Max Samples`, `taa_render_samples`, `another scene`, `other scene`, `different scene`, `second scene`, `Render Layers`, `Render Layers node`, `Render Properties`, `all scenes`, `every scene`, `each scene`, `per scene`, `per-scene`, `argv`, `argparse`, `sys.argv`, `command` (last four in the Cycles files only), `camera_override`, `layer_override`, `render.render`.

**Pages I opened and read in full or in the relevant part:**

- Manual, *Render Layers Node* — https://docs.blender.org/manual/en/5.2/compositing/types/input/scene/render_layers.html. The node's whole property list. No engine, no samples.
- Manual image `_images/node-types_CompositorNodeRLayers.webp`, the picture on that page. I converted it to PNG to view it. It shows only Image and Alpha outputs, a Scene field and a ViewLayer field. Ruled out any engine or sample control on the node face.
- Manual, *Scene* input node index, and the other six pages under it (Active Camera, Camera Info, Object Info, Scene Time, Time Curve) — https://docs.blender.org/manual/en/5.2/compositing/types/input/scene/index.html and siblings. Checked in case a second node offers scene-render controls. None does.
- Manual, *Compositor System* — https://docs.blender.org/manual/en/5.2/compositing/compositor_system.html. Read in full. Covers data types, image domains, operation domains, active output, socket inspection. Ruled out: says nothing about how or with what a referenced scene is rendered.
- Manual, *Introduction* (compositing) — https://docs.blender.org/manual/en/5.2/compositing/introduction.html. Read in full. Ruled out.
- Manual, *Using The Compositor* — https://docs.blender.org/manual/en/5.2/compositing/usage.html. Read in full. Ruled out.
- Manual, *Limitations* (compositing) — https://docs.blender.org/manual/en/5.2/compositing/limitations.html. Read in full. Hardware limits of the GPU compositor only. Ruled out.
- Manual, *Compositor* editor — https://docs.blender.org/manual/en/5.2/editors/compositor.html. Read the top. "Each Scene can have its own compositor node tree." Nothing on engine or samples.
- Manual, *Render Engines* — https://docs.blender.org/manual/en/5.2/render/render_engines.html. Read in full. Gives where the engine is chosen; no scope statement about outside values.
- Manual, *Properties Editor* — https://docs.blender.org/manual/en/5.2/editors/properties_editor.html. Read the top, for whose properties the Render tab holds.
- Manual, *Sampling* (Cycles) — https://docs.blender.org/manual/en/5.2/render/cycles/render_settings/sampling.html. Read the sampling section and the Layer Samples / Sample Subset section.
- Manual, *Sampling* (EEVEE) — https://docs.blender.org/manual/en/5.2/render/eevee/render_settings/sampling.html. Read the top.
- Manual, *Sampling* (Workbench) — https://docs.blender.org/manual/en/5.2/render/workbench/sampling.html. Read in full.
- Manual, *View Layer Override* — https://docs.blender.org/manual/en/5.2/render/layers/override.html. Read in full.
- Manual, *Command Line Arguments* — https://docs.blender.org/manual/en/5.2/advanced/command_line/arguments.html. Read the whole Render Options, Cycles Render Options and Format Options sections. This is where `-E`, `-S` and the two Cycles options are, and where a samples option would be if one existed. Searched the file for "sample": no match.
- Manual, *Rendering From The Command Line* — https://docs.blender.org/manual/en/5.2/advanced/command_line/render.html. Read in full.
- Manual, *GPU Rendering* — https://docs.blender.org/manual/en/5.2/render/cycles/gpu_rendering.html. Read the top, for the one "Configure each scene to use the GPU Compute Device" line and the OS-by-backend statements.
- Manual, *Scene Strip* — https://docs.blender.org/manual/en/5.2/video_editing/edit/montage/strips/scene.html. Read in full, to check the second reading of the question. Lists Scene, View Layer, Input, Camera override, Show, Sound, and a Limitations note. No engine, no samples.
- Python API, *CompositorNodeRLayers* — https://docs.blender.org/api/5.2/bpy.types.CompositorNodeRLayers.html. Read in full.
- Python API, *RenderSettings* — https://docs.blender.org/api/5.2/bpy.types.RenderSettings.html. Searched for "engine"; read the `engine` and `has_multiple_engines` entries.
- Python API, *Scene* — https://docs.blender.org/api/5.2/bpy.types.Scene.html. Read the `render` and `eevee` entries and searched for "render engine".
- Python API, *SceneEEVEE* — https://docs.blender.org/api/5.2/bpy.types.SceneEEVEE.html. Read the `taa_render_samples` entry.
- Python API, *ViewLayer* — https://docs.blender.org/api/5.2/bpy.types.ViewLayer.html. Read the `samples` entry; searched for "render engine".
- Python API, *RenderEngine* — https://docs.blender.org/api/5.2/bpy.types.RenderEngine.html. Checked its attributes for anything an engine receives about samples or about which scene. It carries `camera_override`, `layer_override`, `is_animation`, `is_preview`, `resolution_x/y`, `use_highlight_tiles`. Nothing about samples. Ruled out.
- Python API, *bpy.ops.render* — https://docs.blender.org/api/5.2/bpy.ops.render.html. Read the `render()` signature and its parameter descriptions.
- Python API, *NodeTree* — https://docs.blender.org/api/5.2/bpy.types.NodeTree.html. Read the worked example that reads the engine off a scene.
- Release notes, *Compositor* — https://developer.blender.org/docs/release_notes/5.2/compositor/. Read in full. Nothing about cross-scene engine or samples.
- Release notes, *Core* — https://developer.blender.org/docs/release_notes/5.2/core/. Read in full. Auto-save and library override only.
- Release notes, *Rendering* — https://developer.blender.org/docs/release_notes/5.2/rendering/. Read in full.
- Release notes, *Pipeline & I/O* — https://developer.blender.org/docs/release_notes/5.2/pipeline_io/. Read. Format importers and exporters.
- Release notes, *Fixes* — https://developer.blender.org/docs/release_notes/5.2/bugfixes/. Searched for scene, engine, samples. The only line touching this question is the GPU-compositor second-scene crash fix quoted above.
- Release notes, *Sequencer*, *Cycles*, *EEVEE*, *Python API* — https://developer.blender.org/docs/release_notes/5.2/sequencer/, /cycles/, /eevee/, /python_api/. Searched, not read in full. Sequencer notes a new view-layer selector on scene strips; Cycles notes texture cache and a `--command maketx` tool; neither adds an engine or samples control.

**Cycles' own files I read:**

- `Blender Sources/cycles_5.2/engine.py`, lines 1–80. The complete command-line parser. Two options only.
- `Blender Sources/cycles_5.2/__init__.py`, lines 1–135. The `CyclesRender` engine class. It receives a depsgraph and passes it on; nothing in the Python layer chooses a scene or a sample count.
- `Blender Sources/cycles_5.2/properties.py`: the samples block (lines 470–570), the layer-samples enum (lines 64–67), the scene registration (lines 1140–1150), and the whole of `CyclesRenderLayerSettings` (lines 1523–1612). Note for the record: that view-layer class holds passes and denoising settings and **no** samples property — the per-view-layer sample override lives on `ViewLayer.samples` in Blender itself, not in the Cycles add-on.
- `Blender Sources/cycles_5.2/ui.py`: the sampling render panel (lines 288–316) and the layer-samples row (lines 465–481).

**What I ruled out and why:**

- The rest of the compositing node reference (colour, filter, transform, keying, mask, output, utilities). Searched the whole `compositing/` folder for "engine" and "sample"; every hit was a node's own internal sample count (Vector Blur samples, Directional Blur samples, Mask node motion-blur samples, Chromatic Aberration samples) or an unrelated use of the word "sample" meaning eyedropper. None concerns a scene render. Ruled out.
- The Freestyle, colour-management and stereoscopy pages, which mention Render Layers from different scenes in passing. The stereoscopy and Switch Stereo View hits say the output size holds "regardless of the Image nodes resolutions or Render Layers from different scenes" — that is about size, not engine or samples. Ruled out.
- Hydra / `SceneHydra`, which turned up in the "render engine" search. It concerns third-party delegates and says nothing about compositor-driven scene renders. Ruled out.
- The wider web. I did not need it and did not go. There is nothing here I would want to fetch.

---

## Things a person with Blender 5.2 open could settle by hand in a few minutes

*(**Run, and pruned — 2026-08-29.** Every test this list held was carried out on the Creator's MacBook Pro on 2026-08-29, and repeated on the same machine the same day. The answers are written into *What running Blender showed — the hand tests of 2026-08-29* above, which is where they now live; the answered items have therefore been taken off this list rather than left standing as work for somebody to do. What was set up, what was typed and what the machine printed is recorded in full in `Audits/Blender Research/The compositor/Hand tests — Architecture Fact 135.md`. This list's items were, in that record's numbering, entries 6 (item 1), 7 (item 2), 14 (item 3), 17 (item 4), 10 (item 5), 8 (item 6) and 5 (item 7).)*

**Nothing from this list is still owed.** Four things were not run at all, and they are recorded in that same document: a real press of the re-render button on the Render Layers node — the operator behind it was driven and cancelled without rendering, see *Observed 10* above; anything on Windows; everything above on a second operating system; and any third-party render engine, since every run used `--factory-startup`.

---

2026-08-29 — Claude Opus 5 (Claude Code) — The compositor, Blender 5.2.

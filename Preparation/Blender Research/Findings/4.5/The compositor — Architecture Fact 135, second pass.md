# The compositor — Architecture Fact 135, second pass (Blender 4.5)

Sources consulted, and the date: the Blender 4.5 manual, the Blender 4.5 Python API documentation, the Blender 4.5 release notes, and the Cycles add-on files, all as supplied in `Blender Sources/` and all taken on 2026-07-30. I read them on 2026-08-29.

I read only the question sheet and files inside Blender Sources/. I did not open any other part of this project.

One thing to declare, for honesty rather than because it changes anything. Before my first action, my working environment automatically placed the text of this project's `CLAUDE.md` in front of me, unasked. I did not open it and I did not follow it: it tells Blender researchers to stop and read nothing further, which is what I did. Its text contains no claim about how Blender behaves, so nothing in it could have coloured a finding below. I opened no file it points to, and nothing else in the project.

---

## 9. When one scene takes in another scene's rendered result, what decides the render engine and the sample count that the second scene is rendered with, and what effect on that does a value for either, given to the render from outside, have?

### How I read the question

The compositor's way of taking in another scene's rendered result is the Render Layers node, which has a Scene picker on it. I answer for that. I call the scene whose compositor is running the **first scene**, and the scene named in the Render Layers node the **second scene**.

"A value for either, given to the render from outside" I read as: a value for the engine, or for the sample count, handed to Blender when the render is started from outside Blender's own window — a command-line argument or the equivalent. That is the reading I answer.

This is ambiguous and I flag it. The other reading would be "a value fed into the compositor from outside the node tree" — but the compositor has no such channel for an engine or a sample count, and I found nothing in the sources describing one, so I do not think that reading was meant.

---

### Part A — what decides the engine and the sample count

**LABEL: inferred** for the second scene specifically. The pieces it is built from are documented, and I give them all below. The step I take is written out at the end of this part.

**A1. The node itself carries no engine and no sample setting.**

LABEL: documented.

The Render Layers node's entire set of properties is a Scene picker and a View Layer picker.

Exact words:

> "Renders a View Layer and reads its
> Passes into the compositing node graph."

> "Properties
>
> Scene
>
> The scene for which to render a view layer.
>
> View Layer
>
> The view layer to render. The button next to the dropdown re-renders it immediately."

Source: the manual, https://docs.blender.org/manual/en/4.5/compositing/types/input/scene/render_layers.html
Date read: 2026-07-30 (snapshot date).

The same page's own picture shows the node with two dropdowns and nothing else on it. I looked at `_images/node-types_CompositorNodeRLayers.webp`. What is visible: a node headed "Render Layers", outputs "Image" and "Alpha", a field reading "Scene" with a clear button, and a field reading "ViewLayer" with a small button beside it. No engine field, no samples field. LABEL for that: documented (it is part of the manual).

I also looked at `_images/interface_controls_nodes_sidebar_item.png`, the picture the manual captions "Node tab with a compositing Render Layers node selected" (https://docs.blender.org/manual/en/4.5/interface/controls/nodes/sidebar.html). Visible in it: a "Properties" panel for the selected Render Layers node containing exactly two fields, "Scene" and "View Layer". So the node offers nothing further in the sidebar either.

The Python side matches. Exact words:

> "class bpy.types.CompositorNodeRLayers(CompositorNode)
>
> Input render passes from a scene render"

and its only two own properties:

> "layer
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

Source: the API documentation, https://docs.blender.org/api/4.5/bpy.types.CompositorNodeRLayers.html
Date read: 2026-07-30.

**A2. The render engine is a setting held by a scene.**

LABEL: documented.

Exact words:

> "engine
>
> Engine to use for rendering
>
> Type:
>
> enum in ['BLENDER_EEVEE_NEXT'], default 'BLENDER_EEVEE_NEXT'"

Source: the API documentation, https://docs.blender.org/api/4.5/bpy.types.RenderSettings.html
Date read: 2026-07-30.

(The list of names in that quote holds only the built-in engine. **Why it is short is not stated on the page, and no source here says it. LABEL: inferred** — the step is that the manual tells a reader to "Use -E help to list available engines" (quoted in Part B) rather than giving a fixed list, so the enum as printed here is not the whole set of engines a running Blender offers. Why it is short I cannot say from these sources. I take nothing else from the short list. *(Relabelled by the citation check, 2026-08-29. The sentence read "The list of names in that quote is short because the documentation was generated with only the built-in engine registered" — an explanation with no source behind it, offered inside a passage labelled documented. The 5.0 second pass makes the same move and labels it inferred with the step written out; this one did not.)*)

And a scene holds one of those settings blocks:

> "render
>
> Type:
>
> RenderSettings, (readonly, never None)"

Source: the API documentation, https://docs.blender.org/api/4.5/bpy.types.Scene.html
Date read: 2026-07-30.

The manual says the same in interface terms. Exact words:

> "Scene
>
> Tabs related to the active scene:
>
> Render: EEVEE,
> Cycles, or
> Workbench settings."

Source: the manual, https://docs.blender.org/manual/en/4.5/editors/properties_editor.html
Date read: 2026-07-30.

> "To use Cycles, select it as the Render Engine in the Render properties."

Source: the manual, https://docs.blender.org/manual/en/4.5/render/cycles/introduction.html
Date read: 2026-07-30.

The manual also uses the possessive form directly:

> "Render the 3D Viewport using the scene's Render Engine, for interactive rendering."

Source: the manual, https://docs.blender.org/manual/en/4.5/editors/3dview/display/shading.html
Date read: 2026-07-30.

And it states plainly that this kind of setting must be set scene by scene, using the Device setting as its example:

> "Next, you must configure each scene to
> use GPU rendering in Properties ‣ Render ‣ Device."

Source: the manual, https://docs.blender.org/manual/en/4.5/render/cycles/gpu_rendering.html
Date read: 2026-07-30.

**A3. The sample count is a setting held by a scene, with a per-view-layer override.**

LABEL: documented for the API and manual quotes; observed for the Cycles add-on files.

Cycles. In the Cycles add-on's own files:

> "    samples: IntProperty(
>         name="Samples",
>         description="Number of samples to render for each pixel",
>         min=1, max=(1 << 24),
>         default=4096,
>     )"

File: `Blender Sources/cycles_4.5/properties.py`, lines 482–487, inside `class CyclesRenderSettings(bpy.types.PropertyGroup):` which begins at line 376.
LABEL: observed.
Date read: 2026-08-29 (files as supplied, snapshot 2026-07-30).

That block is attached to the Scene:

> "    @classmethod
>     def register(cls):
>         bpy.types.Scene.cycles = PointerProperty(
>             name="Cycles Render Settings",
>             description="Cycles render settings",
>             type=cls,
>         )"

File: `Blender Sources/cycles_4.5/properties.py`, lines 1100–1106.
LABEL: observed.

And the panel that draws it reads it off the scene:

> "        scene = context.scene
>         cscene = scene.cycles"

> "        col = layout.column(align=True)
>         if cscene.use_adaptive_sampling:
>             col.prop(cscene, "samples", text="Max Samples")
>             col.prop(cscene, "adaptive_min_samples", text="Min Samples")
>         else:
>             col.prop(cscene, "samples", text="Samples")"

File: `Blender Sources/cycles_4.5/ui.py`, lines 298–299 and 311–316, inside `class CYCLES_RENDER_PT_sampling_render(CyclesButtonsPanel, Panel):` which begins at line 288.
LABEL: observed.

EEVEE. Exact words:

> "taa_render_samples
>
> Number of samples per pixel for rendering
>
> Type:
>
> int in [1, inf], default 64"

Source: the API documentation, https://docs.blender.org/api/4.5/bpy.types.SceneEEVEE.html
Date read: 2026-07-30.

That block too belongs to a scene:

> "eevee
>
> EEVEE settings for the scene"

Source: the API documentation, https://docs.blender.org/api/4.5/bpy.types.Scene.html
Date read: 2026-07-30.

The EEVEE manual page gives the same in interface terms:

> "Render
>
> Samples
>
> The number of samples to use in the final render."

Source: the manual, https://docs.blender.org/manual/en/4.5/render/eevee/render_settings/sampling.html
Date read: 2026-07-30.

Workbench. Workbench has no free sample count; it has an anti-aliasing choice, and the manual says outright it is stored per scene:

> "The setting for viewport rendering
> and final rendering is saved per scene."

and, for the Multisample option:

> "The number of samples are predefined so it uses the best distribution of the samples.
>
> 5, 8, 11, 16, 32"

Source: the manual, https://docs.blender.org/manual/en/4.5/render/workbench/sampling.html
Date read: 2026-07-30.

The view layer override. A view layer inside a scene can override that scene's sample count:

> "samples
>
> Override number of render samples for this view layer, 0 will use the scene setting
>
> Type:
>
> int in [0, inf], default 0"

Source: the API documentation, https://docs.blender.org/api/4.5/bpy.types.ViewLayer.html
Date read: 2026-07-30.

Under Cycles, whether that override is honoured is itself a scene-level choice:

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

Source: the manual, https://docs.blender.org/manual/en/4.5/render/cycles/render_settings/sampling.html
Date read: 2026-07-30.

The Cycles manual's own view-layer page says the same from the other side:

> "Samples
>
> View layer samples to override the scene samples.
> Controlled by the layer samples in the Sampling panel."

Source: the manual, https://docs.blender.org/manual/en/4.5/render/layers/introduction.html
Date read: 2026-07-30.

And in the add-on files, the Layer Samples control is only shown when a view layer *of that same scene* has a non-zero override:

> "        for view_layer in scene.view_layers:
>             if view_layer.samples > 0:
>                 layout.separator()
>                 layout.row().prop(cscene, "use_layer_samples")
>                 break"

File: `Blender Sources/cycles_4.5/ui.py`, lines 452–456.
LABEL: observed.

Its three choices, in the add-on's own words:

> "enum_use_layer_samples = (
>     ('USE', "Use", "Per render layer number of samples override scene samples"),
>     ('BOUNDED', "Bounded", "Bound per render layer number of samples by global samples"),
>     ('IGNORE', "Ignore", "Ignore per render layer number of samples"),"

File: `Blender Sources/cycles_4.5/properties.py`, lines 68–71.
LABEL: observed.

**A4. The step I take, written out.**

Nowhere in the sources is it said, in so many words, which scene's engine or sample count is used when a Render Layers node names a second scene. What is documented is: (i) the node names a scene and a view layer and offers nothing else; (ii) the node "renders a view layer" of that scene; (iii) the engine, and the sample count, are settings that belong to a scene (and, for the sample count, to a view layer inside a scene).

From those I take one step: since the only thing the node supplies is which scene and which view layer, and since engine and sample count live on scenes and view layers, the second scene is rendered with the engine set on the second scene and the sample count set on the second scene, adjusted by that scene's own view-layer override where one is configured and allowed.

That step is mine, not the sources'. It is the reason the label for Part A is **inferred**.

**A5. One nearby thing the sources do settle, which is not this.**

The manual says that some render-wide settings come from the current (first) scene no matter what the Render Layers nodes point at. It says this about multi-view and about output resolution, not about engine or samples:

> "The views to render are defined in the current scene views,
> in a similar way as you define the composite output resolution in the current scene render panel,
> regardless of the Image nodes resolutions or Render Layers from different scenes."

Source: the manual, https://docs.blender.org/manual/en/4.5/render/output/properties/stereoscopy/usage.html
Date read: 2026-07-30.

I record it because it is the closest the sources come to naming which scene wins for anything, and because it deliberately does not extend to the engine or the sample count. Reading it as covering them would be over-reading.

---

### Part B — the effect of a value given to the render from outside

**LABEL: not in the sources**, for the effect on the second scene. What the outside values *are* is documented; what they do to a scene pulled in by a Render Layers node is not.

**B1. There is an outside value for the engine.**

LABEL: documented, as to its existence and description.

Exact words:

> "-E, --engine <engine>
>
> Specify the render engine.
> Use -E help to list available engines."

Source: the manual, https://docs.blender.org/manual/en/4.5/advanced/command_line/arguments.html
Date read: 2026-07-30.

And in the worked examples:

> "blender -b file.blend -E CYCLES -s 10 -e 500 -t 2 -a"

> "-E CYCLES
>
> Use the "Cycles Render" engine.
> For a list of available render engines, run blender -E help."

Source: the manual, https://docs.blender.org/manual/en/4.5/advanced/command_line/render.html
Date read: 2026-07-30.

That is the whole of what the sources say about it. They do not say which scene it lands on, whether it is written into the file's scenes, whether it reaches a scene named by a Render Layers node, or what happens to a second scene that was set to a different engine. I searched for that and found nothing (searches listed below).

The same page does state a general rule about argument order, which bears on any command line that mixes these:

> "Note
>
> Arguments are executed in the order they are given!"

Source: the manual, https://docs.blender.org/manual/en/4.5/advanced/command_line/render.html
Date read: 2026-07-30.

**B2. These sources describe no outside value for the sample count.**

LABEL: not in the sources.

*(Narrowed by the citation check, 2026-08-29. The heading read "There is no outside value for the sample count." Nothing quoted below carries that: what the quotes carry is that one manual page lists no such option and that Cycles' own parser defines none. "There is none" is a statement about Blender; "these sources describe none" is what the words support, and it is what the label above already says.)*

The command-line argument list has no samples argument. Its Render Options are `-b`, `-a`, `-S`, `-f`, `-s`, `-e`, `-j`, `-o`, `-E`, `-t`; its Cycles Render Options are exactly two:

> "--cycles-device <device>
>
> Set the device used for rendering.
> Valid options are: CPU CUDA OPTIX HIP ONEAPI METAL."

> "--cycles-print-stats
>
> Log statistics about render memory and time usage."

Source: the manual, https://docs.blender.org/manual/en/4.5/advanced/command_line/arguments.html
Date read: 2026-07-30.

The Cycles add-on's own argument parser confirms those two are all it defines:

> "    parser = argparse.ArgumentParser(description="Cycles Addon argument parser",
>                                      add_help=False)
>     parser.add_argument("--cycles-print-stats",
>                         help="Print rendering statistics to stderr",
>                         action='store_true')
>     parser.add_argument("--cycles-device",
>                         help="Set the device to use for Cycles, overriding user preferences and the scene setting."
>                              "Valid options are 'CPU', 'CUDA', 'OPTIX', 'HIP', 'ONEAPI', or 'METAL'."
>                              "Additionally, you can append '+CPU' to any GPU type for hybrid rendering.",
>                         default=None)"

File: `Blender Sources/cycles_4.5/engine.py`, lines 11–20, in `_configure_argument_parser()`.
LABEL: observed.

Worth noting for contrast: `--cycles-device` is described in the add-on's own text as "overriding user preferences and the scene setting" — an outside value that is documented to beat the scene. Nothing equivalent is defined for the sample count, and nothing equivalent is said of `-E`.

The manual's list of environment variables Blender reads (`BLENDER_USER_RESOURCES`, `BLENDER_USER_CONFIG`, `BLENDER_USER_SCRIPTS`, `BLENDER_USER_EXTENSIONS`, `BLENDER_USER_DATAFILES`, `BLENDER_SYSTEM_RESOURCES`, `BLENDER_SYSTEM_SCRIPTS`, `BLENDER_SYSTEM_EXTENSIONS`, `BLENDER_SYSTEM_DATAFILES`, `BLENDER_SYSTEM_PYTHON`, `BLENDER_CUSTOM_SPLASH`, `BLENDER_CUSTOM_SPLASH_BANNER`, `OCIO`, `TEMP`, `TMPDIR`) contains none for the engine and none for samples. Same page and date as above.

**B3. Two outside values the sources do describe, and both name which scene is rendered.**

LABEL: documented.

*(Narrowed by the citation check, 2026-08-29. The heading read "What the outside can name is which scene is rendered — not the engine or samples of a second one." The two quotes below show that these two things name a scene and say nothing about an engine or a sample count. They cannot carry "not the engine or samples of a second one" as a statement about everything the outside can name.)*

> "-S, --scene <name>
>
> Set the active scene <name> for rendering."

Source: the manual, https://docs.blender.org/manual/en/4.5/advanced/command_line/arguments.html
Date read: 2026-07-30.

> "bpy.ops.render.render(*, animation=False, write_still=False, use_viewport=False, layer='', scene='')"

> "scene (string, (optional, never None)) – Scene, Scene to render, current scene if not specified"

Source: the API documentation, https://docs.blender.org/api/4.5/bpy.ops.render.html
Date read: 2026-07-30.

Both of these choose the scene that is rendered. Neither says anything about the engine or the sample count of a scene reached through a Render Layers node.

**B4. The other outside route the sources describe.**

LABEL: inferred.

`Scene.render` (RenderSettings, holding `engine`), `Scene.cycles` and `Scene.eevee` are ordinary Python properties of each Scene data-block, and `engine` and `taa_render_samples` are not marked read-only in the API documentation (quoted in A2 and A3). The step: a script run from outside — for example via Blender's Python argument — can therefore set the engine or the sample count on any named scene, including the second one, before the render is started. The sources describe the properties; they do not describe this use. I mark it inferred and I take it no further.

---

### What running Blender showed — the hand tests of 2026-08-29

**LABEL: observed**, for everything in this section and for nothing above it.

**These are two separate claims and they are kept apart.** Everything above this heading is what the Blender 4.5 sources say, and it stands unchanged — including every *not in the sources*, which is a statement about the documentation and is not undone by a run. What follows is the other claim: what Blender 4.5 actually did. Where the two meet, both are recorded — the reading as a reading, the run as a run.

**On what kind of computer, and when.** The Creator's own MacBook Pro: an Apple M1 Pro chip, Apple's own graphics, no second card. Read off the machine on the day:

```
ProductName:		macOS
ProductVersion:		26.6.2
BuildVersion:		25G83
arm64
Apple M1 Pro
```

The Blender was `/Applications/Blender 4.5.12.app/Contents/MacOS/Blender`, which reports:

```
Blender 4.5.12 LTS (hash 84afd5f785f7 built 2026-07-21 01:41:35)
```

Every run was made on **2026-08-29**, and every one was repeated on the same machine the same day by the citation check, with the same result each time. Every run used `--factory-startup`, so no third-party add-on was loaded and no add-on render engine existed in any of them. **Nothing was run on any other operating system** — see the operating-system line below.

**What was run against.** One set of files, built by Blender 4.5.12 LTS itself so that no file crossed a version boundary. Each holds two scenes: **`Comp`**, whose compositor is being run, and **`Sub`**, the scene pulled in. Both hold a camera, one area light, a metallic sphere of roughness 0.35 and a blue floor, with the two spheres in different places so the two scenes are not the same picture. `Comp`'s compositor holds one **Render Layers** node with its Scene set to `Sub` and its View Layer set to `Sub`'s own `ViewLayer`, wired straight to the output and nothing else — so the written image is entirely `Sub`'s render, and any change in it is a change in how `Sub` was rendered. Both scenes: 160 by 120 pixels, one frame, uncompressed 32-bit OpenEXR, compositing on, stamping off, Cycles on the processor with denoising off, adaptive sampling off, no time limit and a fixed seed of 7, so a Cycles render repeats to the bit. Only the engines, sample counts, view-layer overrides and Layer Samples choices differ from file to file, and each finding below names the file it used. The script that builds them is printed in full in `Audits/Blender Research/The compositor/Hand tests — Architecture Fact 135.md`.

Every run took this shape, with the file, the extra arguments and the output path changing:

```
"/Applications/Blender 4.5.12.app/Contents/MacOS/Blender" --factory-startup -b "<file>.blend" --log "render" -o "<output>_" -f 1
```

`-f 1` sits last because Blender's own manual warns that arguments run in the order they are given.

**Two instruments, and no weaker ones.** Blender's own log, turned on with `--log "render"`. And exact pixel comparison: every render was written as an uncompressed 32-bit OpenEXR and pairs of images were compared channel by channel, reported below as `maxdiff` — the largest difference in any one colour channel of any one pixel, across 19,200 pixels. A `maxdiff` of exactly 0 means the two are the same picture to the last bit. Files were never compared byte for byte, because Blender writes times into an image and those are not the picture.

---

**Observed 1 — the second scene is rendered with the engine set on the second scene.**

Four files were rendered, each with a different engine on `Comp` and on `Sub`. Each written image was compared against a **reference** made by rendering `Sub` directly with `-S Sub` added, so that no compositor was involved in the reference at all.

Blender 4.5 prints no line naming the engine — it prints progress lines that name the scene and view layer being rendered, and nothing that says which engine is running. So in this version the pixels are what answer, and they answer clearly:

| file | `Comp` is | `Sub` is | vs `Sub`'s own engine's reference | vs `Comp`'s engine's reference |
| --- | --- | --- | --- | --- |
| `eng_Awb_Bcy` | Workbench | Cycles at 64 | Cycles-64: **0** | Workbench: 44.1421 |
| `eng_Acy_Bwb` | Cycles at 64 | Workbench | Workbench: **0** | Cycles-64: 44.1421 |
| `eng_Aee_Bcy` | EEVEE | Cycles at 64 | Cycles-64: **0** | EEVEE-16: 2.1 |
| `eng_Acy_Bee` | Cycles at 64 | EEVEE at 16 | EEVEE-16: **0** | Cycles-64: 2.1 |

The zeros are exact zeros. (The two non-zero figures are recorded as the same across all four researched versions to within the third decimal place; the zeros are Blender 4.5's own.)

**So the scene pulled in through the node is rendered with the engine stored on that scene**, in both directions and for all three engines: Workbench inside a Cycles render, Cycles inside a Workbench render, EEVEE inside a Cycles render and Cycles inside an EEVEE render. The four files were built so that the opposite answer would have shown as each image matching the *other* reference, and none did.

One thing found in passing, worth carrying because it bears on what "the second scene is rendered with" even means. Every progress line in every one of these runs named `Sub, ViewLayer`, and not one named `Comp`. `Comp`'s own view layer was never rendered at all — nothing in its compositor asks for it. So the render of the compositing scene consisted of rendering the other scene and then running the compositor.

---

**Observed 2 — under Cycles, the sample count is the second scene's own, and the compositing scene's count does not leak into it.**

Two files with the two counts swapped, and two references made with `-S Sub`. Blender 4.5.12's own lines for `smp_A1_B512`, in which `Comp` is set to 1 sample and `Sub` to 512:

```
Fra:1 Mem:8.92M (Peak 9.10M) | Time:00:00.00 | Mem:0.19M, Peak:0.19M | Sub, ViewLayer | Sample 0/512
Fra:1 Mem:9.25M (Peak 9.25M) | Time:00:00.00 | Remaining:00:03.25 | Mem:0.48M, Peak:0.48M | Sub, ViewLayer | Sample 1/512
Fra:1 Mem:9.55M (Peak 9.55M) | Time:00:03.25 | Mem:0.48M, Peak:0.48M | Sub, ViewLayer | Sample 512/512
Fra:1 Mem:9.05M (Peak 9.55M) | Time:00:03.25 | Compositing
```

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

Blender 4.5.12 prints EEVEE's progress in words, so this version names its own answer. From the run with `Comp` on Cycles and `Sub` on EEVEE at 16 — 16 being `Sub`'s own count, not `Comp`'s:

```
Fra:1 Mem:41.31M (Peak 41.31M) | Time:00:00.07 | Rendering 1 / 16 samples
Fra:1 Mem:41.46M (Peak 41.46M) | Time:00:00.08 | Rendering 16 / 16 samples
```

And the swap test proper — two files with the counts swapped, and references made with `-S Sub`:

| file | `Comp` at | `Sub` at | vs `Sub`-at-64 | vs `Sub`-at-4 |
| --- | --- | --- | --- | --- |
| `ee_A4_B64` | 4 | 64 | **0** | 5.41 |
| `ee_A64_B4` | 64 | 4 | 5.41 | **0** |

No findings document had covered EEVEE's sample count through the node by test; the cross-scene case had been left to the same inference as Cycles. It holds.

---

**Observed 4 — under Cycles, the view layer's sample override does reach through the node, and it is the *pulled-in* scene's Layer Samples policy that governs it.**

Five files, both scenes on Cycles, `Comp` at 64 samples, `Sub` at 512, the node pointing at `Sub`'s only view layer, and that view layer given a sample override. References at 512 and at 4, made with `-S Sub`.

Blender 4.5.12's whole log for `vl_use`, as it came:

```
Blender 4.5.12 LTS (hash 84afd5f785f7 built 2026-07-21 01:41:35)
Read blend: ".../vl_use.blend"
Fra:1 Mem:9.01M (Peak 9.10M) | Time:00:00.00 | Mem:0.00M, Peak:0.00M | Sub, ViewLayer | Initializing
Fra:1 Mem:8.80M (Peak 9.10M) | Time:00:00.00 | Mem:0.00M, Peak:0.00M | Sub, ViewLayer | Waiting for render to start
Fra:1 Mem:8.80M (Peak 9.10M) | Time:00:00.00 | Mem:0.00M, Peak:0.00M | Sub, ViewLayer | Loading render kernels (may take a few minutes the first time)
Fra:1 Mem:8.92M (Peak 9.10M) | Time:00:00.00 | Mem:0.19M, Peak:0.19M | Sub, ViewLayer | Sample 0/4
Fra:1 Mem:9.25M (Peak 9.25M) | Time:00:00.01 | Remaining:00:00.01 | Mem:0.48M, Peak:0.48M | Sub, ViewLayer | Sample 1/4
Fra:1 Mem:9.55M (Peak 9.55M) | Time:00:00.03 | Mem:0.48M, Peak:0.48M | Sub, ViewLayer | Sample 4/4
Fra:1 Mem:9.55M (Peak 9.55M) | Time:00:00.03 | Mem:0.48M, Peak:0.48M | Sub, ViewLayer | Finished
Fra:1 Mem:9.05M (Peak 9.55M) | Time:00:00.03 | Compositing
Saved: '.../vl_use_0001.exr'
Time: 00:00.03 (Saving: 00:00.00)
```

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

Blender 4.5.12 LTS printed `Rendering 64 / 64 samples`, and the written image matched the 64-sample reference at `maxdiff = 0` and differed from the 8-sample reference, at `maxdiff` 2.617.

**In Blender 4.5 the view layer's sample override has no effect under EEVEE.** This is the one place in the whole round where the four researched versions differ, and each of the four is carrying the difference so that nobody later mistakes it for something else. The same test was run in 5.0, 5.1 and 5.2, and in all three the override was honoured — `Rendering 8 / 8 samples`, matching the 8-sample reference exactly. Only 4.5 ignores it.

**It is not a difference in what the compositor does, and this was established rather than assumed.** The same file was also rendered with `-S Sub`, so `Sub` was the active scene and no compositor was involved at all. That render came out `maxdiff = 0` against the 64-sample reference too. So what differs is whether EEVEE honours a view layer's sample override at all in this version; the path through the node reports faithfully whichever way that goes.

**It was re-run before being written down**, because it was the round's only disagreement between versions. Blender 4.5.12 was run twice more on the same file and printed `Rendering 64 / 64 samples` both times.

---

**Observed 6 — a scene reached through the node renders exactly as it does when it is the scene being rendered.**

Ten of the files above were rendered twice over: once through the node with `Comp` active, and once directly with `-S Sub` so that no compositor was involved. Those ten — `eng_Awb_Bcy`, `eng_Acy_Bwb`, `eng_Acy_Bee`, `smp_A1_B512`, `smp_A512_B1`, `smp_A1_B128`, `vl_use`, `ee_A4_B64`, `ee_A64_B4` and `ee_vl` — cover all three engines, four Cycles sample counts, two EEVEE sample counts, and a view-layer override under each of the two engines.

**Every one of the ten pairs came out `maxdiff = 0`.** Reported here for the hardest case, the EEVEE override file, because that is the one the versions disagree about:

```
4.5   ee_vl (through node) vs ref_ee_vl_direct   maxdiff=0 meandiff=0
```

Any difference at all, in any pixel, would have meant the compositor path renders the second scene with something other than the settings it has when rendered on its own. There was none. This is the strongest form of the answer, because it does not depend on knowing which setting did what.

---

**Observed 7 — an engine given on the command line does not reach the scene behind the node.**

Three files in which **both** scenes carry the same engine, so that any difference afterwards comes from the argument rather than from what was stored: `E_bothcy` (both Cycles at 64), `E_bothwb` (both Workbench), `E_bothee` (both EEVEE at 16). Four runs, `Comp` active in each, of the form:

```
"/Applications/Blender 4.5.12.app/Contents/MacOS/Blender" --factory-startup -b E_bothcy.blend --log "render" -E BLENDER_WORKBENCH -o "<output>_" -f 1
```

Blender 4.5 prints no engine line, so the pixels are the whole of the evidence here — and they are unambiguous:

| run | vs the stored engine's reference | vs the `-E` engine's reference |
| --- | --- | --- |
| Cycles file, `-E BLENDER_WORKBENCH` | Cycles-64: **0** | Workbench: 44.1421 |
| Cycles file, `-E BLENDER_EEVEE_NEXT` | Cycles-64: **0** | EEVEE-16: 2.1 |
| Workbench file, `-E CYCLES` | Workbench: **0** | Cycles-64: 44.1421 |
| EEVEE file, `-E CYCLES` | EEVEE-16: **0** | Cycles-64: 2.1 |

**`-E` does not reach the scene pulled in behind the compositor.** The scene behind the node was rendered with its own stored engine every time. This was re-run before being written down: the Cycles file with `-E BLENDER_WORKBENCH` was run a second time and again reported `Sample 64/64` and wrote the Cycles picture.

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

| run | the written image matches |
| --- | --- |
| `-S Comp -E BLENDER_WORKBENCH` | the Cycles-64 reference, `maxdiff = 0` |
| `-S Sub -E BLENDER_WORKBENCH` | the Workbench reference, `maxdiff = 0` |

Name the pulled-in scene as the scene to render and `-E` applies to it; leave the compositing scene active and `-E` applies to that one while the pulled-in scene keeps its own engine. Note what the second run costs, though: with `-S Sub`, `Sub` is what renders and `Comp`'s compositor is not used at all — it is not the same picture arriving by another route, it is a different render. **`-E` follows the active scene, not the render.**

**An engine name that does not exist is refused, not ignored.** `-E NO_SUCH_ENGINE` printed:

```
Error: engine not found 'NO_SUCH_ENGINE'
```

and nothing was rendered. Blender 4.5.12 LTS exited **134**, an abort rather than an ordinary refusal.

---

**Observed 8 — a sample count given from outside: no argument sets one, one offered anyway is silently ignored, and a script does reach it.**

**What the program says it accepts.** `--help` in Blender 4.5.12 LTS lists, under *Render Options*, exactly `-b/--background`, `-a/--render-anim`, `-S/--scene`, `-f/--render-frame`, `-s/--frame-start`, `-e/--frame-end`, `-j/--frame-jump`, `-o/--render-output`, `-E/--engine`, `-t/--threads`; and under *Cycles Render Options*, exactly `--cycles-device` and `--cycles-print-stats`. **Nothing in either section sets a sample count.** The program's own account of itself therefore agrees with the reading above, which had rested on absence from a page.

Found in the same place, and worth recording because more than one of these documents told whoever ran their tests to look for such an argument rather than guess one: this build carries a `--log <match>` argument and a `--log-level <level>` argument, under a *Logging Options* heading. `--log "render"` is what produced every engine and sample line quoted in this section.

**What the engine names are.** `-E help` in Blender 4.5.12 LTS printed:

```
Blender Engine Listing:
	BLENDER_EEVEE_NEXT
	BLENDER_WORKBENCH
	CYCLES
```

It prints the listing and then aborts. Run three times in a row, its standard output held the listing, its error output held the single line `libc++abi: terminating`, and it exited with status **134** every time. That is what this command does on this Mac in 4.5, and it is not caused by add-ons — it happened with them switched off.

**What happens when a sampling argument is offered anyway.** Running `smp_A1_B512` — `Comp` at 1 sample, `Sub` at 512 — with `-- --cycles-samples 16` appended produced no complaint of any kind, exit status **0**, a reported count of `Sample 512/512`, and a written image identical to the plain run at `maxdiff = 0`. Three outcomes were possible and all three were distinguishable: accepted and rendered at 16, refused with an error, or taken and ignored. **It is the third: the argument is neither accepted nor refused, and nothing is printed about it.**

**A script, though, does reach the second scene.** Running the same file with `--python-expr 'import bpy; bpy.data.scenes["Sub"].cycles.samples = 7'` before the render gave `Sample 0/7`, `Sample 1/7`, `Sample 7/7`, exit status 0, and a written image differing from the 512-sample reference by `maxdiff` 7.36. **A value handed in from outside can reach the sample count of the scene behind the node — not by a sampling argument, which does not exist, but by a script given to Blender on the command line.**

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

The operator behind the button beside the View Layer dropdown was found by asking Blender what operators exist rather than guessing a name, which printed `['cryptomatte_layer_add', 'cryptomatte_layer_remove', 'read_viewlayers', 'render_changed']`. In a Blender 4.5.12 LTS started **with a window** from a terminal, one area was turned into a node editor, its tree type set to the compositor, **its node tree set to `Comp`'s own compositing tree**, and the Render Layers node selected and made active. The editor reported itself ready:

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

**No render happened** — no scene line, no engine line and no sample line appeared in the log. So this establishes what that operator returns when it is driven this way, and nothing about what the button does when a person presses it. A press is a mouse click on a window, and nothing available for this round could produce one. **What the button renders with is unsettled in Blender 4.5.**

*(This one was re-run on 2026-08-29 rather than copied, because the record of the original run was a step short of repeatable: it did not say that the editor's own node tree must be assigned before the call. Driven exactly as that record described it, both operators refuse — `poll` comes back False and the call raises "context is incorrect". With the tree assigned, the original result reproduces exactly, as printed above. The re-run agreed with the record in every other particular. One detail the record does not carry, seen in the re-run: `read_viewlayers` reports an attempt to read a cached image file for `Sub` and that it cannot read it, and still no render follows.)*

**What this settles in this document, and what it does not.**

Part A's step — that the second scene is rendered with the engine and sample count set on that second scene, adjusted by its own view layer's override where one is configured and allowed — is exactly what the runs show, in every direction they were pushed. The label stays **inferred**, because it describes what the 4.5 sources support; the observed answer stands beside it.

B1 stays as written: the sources describe `-E` and do not say which scene it lands on. That is still true of the sources. *Observed 7* answers the behaviour separately — it lands on the active scene, and on no other.

B2 stays as written for the same reason: these sources describe no outside value for the sample count, and they still do not. *Observed 8* adds what the program does — no such argument exists in its own help, one offered anyway is taken and ignored without a word, and a script is the door that does open.

B4, the inferred script route, is now **observed to work**: `--python-expr` setting `Sub`'s sample count before the render produced `Sample 7/7` for the scene behind the node.

Of the *Not covered* list above, the runs settle "what `-E` actually touches", "whether `-E` is applied before or after the file's scenes exist" (after — it acts at the point in the command line where it appears, and it does not survive to disk), "precedence when several of these meet at once", and "whether a second scene set to an engine the first is not is rendered at all" — it is, with its own engine. They settle nothing about the device, and nothing about the video sequencer's Scene strips.

---

### Not covered

- The sources nowhere state, in words, which scene's engine or sample count is used for a scene pulled in through a Render Layers node. Part A is my inference from per-scene settings plus a node that carries no such setting of its own. If Blender in fact forces the first scene's engine onto the second, or forces the first scene's samples, the sources I was given would not show it.
- What `-E` actually touches — one scene, the active scene only, every scene in the file, or something outside the scenes — is not stated anywhere I could find. I therefore cannot say what it does to a second scene that a Render Layers node names.
- Whether `-E` is applied before or after the file's scenes exist, and whether it survives a scene switch by `-S`, is not stated.
- Precedence when several of these meet at once (an `-E` on the command line, a different engine saved on the second scene, a view-layer sample override, and Cycles' Layer Samples set to Bounded or Ignore) is not stated.
- Whether a second scene set to an engine the first is not — say Cycles inside an EEVEE render — is rendered at all, refused, or rendered with its own engine, is not stated.
- I did not settle anything about the *device* beyond quoting `--cycles-device`; the question did not ask, and device is not engine.
- Nothing here is about the video sequence editor's Scene strips, which are a different way for one scene to take in another and are documented separately.

---

### Outside the interface

*(**Note added by the hand-test step, 2026-08-29.** The tests this document asked for have been run, and their results are now written in — see *What running Blender showed — the hand tests of 2026-08-29* above. Both standing lines below have been brought up to date with them: each keeps what the sources say and gains, marked separately, what a run showed. The citation check's note that the answers were still owed stood here until this was done.)*

Does this hold when Blender is driven from outside its own window?

The settings in Part A are file data — properties of Scene and ViewLayer data-blocks — so nothing in the sources ties them to a person clicking. The values themselves are the same whoever starts the render.

The whole of Part B is about being outside the interface: `-E`, `-S`, `--cycles-device`, `--cycles-print-stats` and `bpy.ops.render.render(scene=...)` exist precisely for that case. The manual frames command-line rendering that way:

> "One advantage of using the command line is that we do not need a graphical display
> (no need for X server on Linux for example)
> and consequently we can render via a remote shell (typically SSH)."

Source: the manual, https://docs.blender.org/manual/en/4.5/advanced/command_line/render.html
Date read: 2026-07-30.

Two documented differences that touch this answer, both interface-side only:

- The Render Layers node's re-render button — "The button next to the dropdown re-renders it immediately" — is a button. There is nobody to press it when Blender is driven from outside. (Manual, render_layers page, cited above.)
- The note on that same page, "The viewport compositor only supports render passes when using EEVEE. For other engines, the passes will be empty", is about the *viewport* compositor, which is a thing shown in a window. The sources do not say it applies to a render started from outside.

Beyond those two, the sources do not say whether anything about engine or sample-count selection differs when Blender is driven from outside. I will not guess. This is the part of the answer I am least able to close from documents, and it is where the hands-on checks below are aimed.

**Observed, 2026-08-29.** This line can now be answered for Blender 4.5 rather than left open, on the one operating system it was answered on.

- **A render started from inside a running Blender with a window is the same picture, to the last bit, as one started from the command line.** Blender 4.5.12 LTS was started **without** `-b` — so with a real window and a full interface loaded — and handed a script that made `Comp` the current scene, rendered it and quit. The script printed `GUI_BACKGROUND_FLAG: False`, so Blender really was not in background mode. The written image was compared with the one the same file wrote from a background run: `maxdiff = 0`, and both matched the 512-sample reference at `maxdiff = 0` while differing from the 1-sample reference by 17.5015. **So the engine and the sample count that the scene behind the node is rendered with are the same whether Blender has an interface or not.**
- **The limit on that, stated plainly.** The render was started by a script running inside that windowed Blender, not by a person pressing a key. What it shows is that the render path is the same when Blender has a window and a full interface loaded. It does not show what a keypress does, and nothing in this round could.
- **The one interface-only control this answer names is still unsettled.** The re-render button beside the node's View Layer dropdown was not settled: the operator behind it, driven from a script with the node editor in the right state, cancelled without rendering (*Observed 10* above). A press is a mouse click on a window, and there was no machine in this round that could produce one.
- **Everything else above was driven from outside in the first place.** Every run in the observed section other than that one was a background run started from a terminal, which is the case this project cares about.

### Operating system

Does this hold on every operating system Blender runs on?

The sources do not say that the engine setting, the sample-count setting, the view-layer override, the Layer Samples policy, or the `-E` argument behave differently on different operating systems. Nothing I read draws any such distinction.

Where the sources *do* draw operating-system lines is elsewhere, on the rendering *device*, which is not what this question asks:

> "CUDA is supported on Windows and Linux and requires a"

and the surrounding section is headed:

> "Blender supports different technologies to render on the GPU depending on the particular GPU manufacturer
> and operating system."

Source: the manual, https://docs.blender.org/manual/en/4.5/render/cycles/gpu_rendering.html
Date read: 2026-07-30.

The temporary-directory environment variables also differ by operating system (`TEMP` on Windows, `TMPDIR` on UNIX systems — command-line arguments page, cited above), which again is not engine or samples.

So: not stated to differ for what this question asks; documented to differ for devices, which is a neighbouring matter.

**Observed, 2026-08-29 — and it settles one operating system and leaves the others open.** Every run in the observed section above was made on **one** machine: the Creator's MacBook Pro, Apple M1 Pro, Apple's own graphics and no second card, macOS 26.6.2 (build 25G83), arm64. Nothing was run on Linux and nothing on Windows; no machine was rented for this round. **Untried rather than untestable: a Windows machine can be rented, and every one this project has read reported a Server edition — see the equipment section of `Preparation/Blender Research/Method.md`.** **So whether any of the observed findings hold on any other operating system is still unknown**, and saying so is the whole of what this line can carry. A test settles the operating system it was run on.

Three places where that gap is worth naming rather than left implicit:

- **The one difference between the four researched versions** — that 4.5 does not honour a view layer's sample override under EEVEE where 5.0, 5.1 and 5.2 do (*Observed 5* above) — was found on this Mac, whose graphics are Apple's own. It is worth repeating on a machine with a different graphics card before it is treated as settled everywhere.
- **The windowed run** behind the *Outside the interface* line above was made on a machine whose viewport is drawn by a real graphics card. It could come out differently on a machine whose graphics are drawn by the processor.
- **No third-party render engine was involved anywhere.** Every run used `--factory-startup`, so no add-on engine was loaded, and none is among this project's equipment.

---

### Where I looked

**Searched across all of `Blender Sources/manual_4.5`, `Blender Sources/api_4.5` and `Blender Sources/release_notes_4.5` (whole-tree text search), for these terms:**

`another scene`, `other scene`, `different scene`, `second scene`, `external scene`, `multiple scenes`, `cross-scene`, `each scene`, `per-scene`, `per scene`, `own render settings`, `its own settings`, `active scene`, `scene setting`, `Render Layers`, `render engine`, `scene's render engine`, `scene.render.engine`, `engine used`, `which engine`, `override engine` / `engine override`, `--engine`, `-E `, `sample count`, `number of samples`, `layer samples`, `samples override`, `override … samples`, `environment variable`, `command line` / `command-line`.

**What those turned up, and what I ruled out:**

- `another scene` — ten hits. Only two mattered: the Render Layers node's Hint (quoted above, and it is about loading pre-rendered EXR files, not about engines or samples) and the video sequencer's Scene Strip page (https://docs.blender.org/manual/en/4.5/video_editing/edit/montage/strips/scene.html), which I read in full. The Scene Strip page lists Scene, Input (Camera or Sequencer), Camera, Show, Sound and a Limitations note about passes. It says nothing about the engine or samples of the referenced scene, and it is not the compositor, so I ruled it out.
- `Render Layers` — nineteen hits. I opened every one that was not an index or an image filename: the compositor system page, the passes page, the view layers introduction, the Cryptomatte, ID Mask, Bokeh Blur and Math node pages, the node-editing page ("Read Render Layers Ctrl-R"), the node sidebar page, and the stereoscopy usage page. None mentions an engine or a sample count for the referenced scene. The Cryptomatte, ID Mask, Bokeh Blur and Math pages use the node only as a stand-in for "an image arrives here".
- I searched the entire `compositing/` folder of the manual for `engine`: **one** hit in the whole folder, the viewport-compositor note on the Render Layers page. I searched the same folder for `sample`: every hit is a node's own sampling control (Mask node samples, Vector Blur samples, Directional Blur samples, Plane Track Deform samples, Denoise wording, texture-node "sample texture at"). None is about rendering a scene. This is the core of my "not in the sources" for Part B: the compositor documentation does not discuss the engine or sample count of the scene it pulls in, anywhere.
- `override engine` / `engine override` across manual and API — no relevant hits. The only API hits are `RenderEngine.camera_override` and `RenderEngine.layer_override`, which are attributes an engine add-on reads, not a way to override which engine runs.
- Release notes for 4.5: I read `compositor.md` in full — it lists new nodes, new options, changed and removed options, and compatibility notes, and says nothing about scenes, engines or samples. I searched `cycles.md`, `rendering.md`, `core.md`, `eevee.md`, `python_api.md` and `pipeline_assets_io.md` for `sample`, `scene`, `engine` and `command line`: the only hits were an Alembic "Cache sampler output" line, a `scene.render.frame_path()` note, a Workbench thumbnail fix, and EEVEE's new `--profile-gpu` argument. Nothing bears on this question. So the release notes do not change or add anything here.
- Environment variables: I read the full list on the command-line arguments page. Sixteen variables, none for engine or samples.
- Command-line arguments: I read the Render Options and Cycles Render Options sections in full, and the whole worked-examples page. `-E` and `-t` are the only engine-adjacent ones; there is no samples argument.

**Files I opened and read, in full or in the relevant part:**

Manual — `compositing/types/input/scene/render_layers.txt`, `compositing/types/input/scene/index.txt`, `compositing/types/input/scene/scene_time.txt`, `compositing/types/input/scene/time_curve.txt`, `compositing/compositor_system.txt`, `compositing/limitations.txt`, `advanced/command_line/arguments.txt`, `advanced/command_line/render.txt`, `render/introduction.txt`, `render/cycles/introduction.txt`, `render/cycles/gpu_rendering.txt`, `render/cycles/render_settings/sampling.txt`, `render/eevee/render_settings/sampling.txt`, `render/workbench/introduction.txt`, `render/workbench/sampling.txt`, `render/layers/introduction.txt`, `render/layers/passes.txt`, `render/output/properties/post_processing.txt`, `render/output/properties/stereoscopy/usage.txt`, `render/freestyle/view_layer/freestyle.txt`, `scene_layout/scene/introduction.txt`, `scene_layout/view_layers/introduction.txt`, `editors/properties_editor.txt`, `editors/3dview/display/shading.txt`, `interface/controls/nodes/sidebar.txt`, `interface/controls/nodes/editing.txt`, `video_editing/edit/montage/strips/scene.txt`.

Manual pictures — `_images/node-types_CompositorNodeRLayers.webp`, `_images/interface_controls_nodes_sidebar_item.png`.

API — `bpy.types.CompositorNodeRLayers.txt`, `bpy.types.RenderSettings.txt`, `bpy.types.Scene.txt`, `bpy.types.SceneEEVEE.txt`, `bpy.types.ViewLayer.txt`, `bpy.types.RenderEngine.txt`, `bpy.ops.render.txt`.

Release notes — `compositor.md` in full; `cycles.md`, `rendering.md`, `core.md`, `eevee.md`, `python_api.md`, `pipeline_assets_io.md`, `bugfixes.md`, `user_interface.md` searched.

Cycles add-on files — `engine.py` read in full, `properties.py` (sampling section, the Scene registration, the view-layer settings class, and a whole-file search for `samples` and for `argparse`), `ui.py` (the sampling panels and a search for `use_layer_samples`). I also listed the whole `cycles_4.5` tree: it holds Python files, OSL shader files, licence texts, and `source/util` plus `source/kernel` headers. There is no Blender-to-Cycles synchronisation code in it, so the point at which Cycles reads a scene's sample count for a render is not present in the files I was given. That is why Part A stops at "inferred" rather than becoming "observed".

**What I did not do:** I did not go to the web, and I did not need to. I did not open any other folder of this project.

---

### Things a person with Blender open could settle in a few minutes

*(**Run, and pruned — 2026-08-29.** Every test this list held was carried out on the Creator's MacBook Pro on 2026-08-29, and repeated on the same machine the same day. The answers are written into *What running Blender showed — the hand tests of 2026-08-29* above, which is where they now live; the answered items have therefore been taken off this list rather than left standing as work for somebody to do. What was set up, what was typed and what the machine printed is recorded in full in `Audits/Blender Research/The compositor/Hand tests — Architecture Fact 135.md`. This list's items were, in that record's numbering, entries 6 (items 1 and 7), 7 (item 2), 14 (item 3), 15 (item 4), 10 (item 5) and 20 (item 6).)*

**Nothing from this list is still owed.** Four things were not run at all, and they are recorded in that same document: a real press of the re-render button on the Render Layers node — the operator behind it was driven and cancelled without rendering, see *Observed 10* above; anything on Windows; everything above on a second operating system; and any third-party render engine, since every run used `--factory-startup`.

---

2026-08-29 — Claude Opus 5 (Claude Code) — The compositor, Blender 4.5

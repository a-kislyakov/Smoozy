# The compositor — Blender 5.2 — Question 9

Sources consulted: Blender 5.2 manual, Blender 5.2 Python API documentation, Blender 5.2 release notes, and Cycles' own files, all from `Blender Sources/`, read on 2026-07-30 (the snapshot date).

I read only the question sheet and files inside `Blender Sources/`. I did not open any other part of this project.

---

## 9. When one scene takes in another scene's rendered result, what decides the render engine and the sample count that the second scene is rendered with, and what effect on that does a value for either, given to the render from outside, have?

### The reading I answered

The question was ambiguous in one place: **"given to the render from outside"**. I answered the reading that a value is handed to the render from outside Blender when the render is started — that is, on the command line. The other reading is "from outside the second scene", meaning the scene doing the compositing imposes a value on the scene it pulls in. I checked that reading too and report on it below; both end in the same place.

I took "one scene takes in another scene's rendered result" to mean a Render Layers node in one scene whose Scene field names a different scene, since this is the compositor sheet.

---

### Part 1 — what the sources do settle: where the engine and the sample count live

**LABEL: documented** — the Render Layers node names a scene and a view layer. The manual's Properties list for the node holds those two and nothing more.

Exact words:

> "Renders a View Layer and reads its Passes into the compositing node graph."
>
> "Properties
>
> Scene
>
> The scene for which to render a view layer.
>
> View Layer
>
> The view layer to render. The button next to the dropdown re-renders it immediately."

Source: manual, https://docs.blender.org/manual/en/5.2/compositing/types/input/scene/render_layers.html — read 2026-07-30.

The picture on that page (`[image: _images/node-types_CompositorNodeRLayers.webp]`) shows the node with exactly two controls: a Scene data-block field and a ViewLayer dropdown, plus Image and Alpha outputs. No engine control and no sample control appear on it. **Documented**, from the manual's own picture; I take from it only what is visible.

One limit on that. What is visible on the node body is not the same as a full list of the node's properties — a control could sit in the node editor's sidebar instead and never show on the body. The manual's Properties list, quoted above, is what carries "those two and nothing more"; the picture only agrees with it for the body. Test 7 at the end of this document settles the sidebar.

The Python API says the same:

> "Input render passes from a scene render"
>
> "scene
>
> Type:
>
> Scene | None"

Source: API documentation, https://docs.blender.org/api/5.2/bpy.types.CompositorNodeRLayers.html — read 2026-07-30.

**LABEL: documented** — render settings, including the engine, belong to a scene.

> "A scene defines the visible objects, camera view, lighting setup, and render settings that make up a specific part of a project."

Source: manual, https://docs.blender.org/manual/en/5.2/scene_layout/scene/introduction.html — read 2026-07-30.

> "class bpy.types.RenderSettings(bpy_struct)
>
> Rendering settings for a Scene data-block"

and, in the same page:

> "engine
>
> Engine to use for rendering (default 'BLENDER_EEVEE')"

Source: API documentation, https://docs.blender.org/api/5.2/bpy.types.RenderSettings.html — read 2026-07-30.

Scene carries one of these:

> "render
>
> (readonly, never None)
>
> Type:
>
> RenderSettings"

Source: API documentation, https://docs.blender.org/api/5.2/bpy.types.Scene.html — read 2026-07-30.

> "The active render engine can be selected in the Render Properties."

Source: manual, https://docs.blender.org/manual/en/5.2/render/render_engines.html — read 2026-07-30.

> "Tabs related to the active scene:
>
> Render: EEVEE, Cycles, or Workbench settings."

Source: manual, https://docs.blender.org/manual/en/5.2/editors/properties_editor.html — read 2026-07-30.

**LABEL: documented** — each engine has its own final-render sample count, and for two of the three the quoted words also place it on the scene. For EEVEE and Workbench they do. For Cycles the manual quote below says what the setting is but not where it lives; the scene-level home for Cycles comes from Cycles' own files, quoted straight after.

EEVEE:

> "Reference
>
> Panel:
>
> Render ‣ Sampling"
>
> "Render
>
> Samples
>
> The number of samples to use in the final render."

Source: manual, https://docs.blender.org/manual/en/5.2/render/eevee/render_settings/sampling.html — read 2026-07-30.

> "taa_render_samples
>
> Number of samples per pixel for rendering (in [1, inf], default 64)"

Source: API documentation, https://docs.blender.org/api/5.2/bpy.types.SceneEEVEE.html — read 2026-07-30. Scene carries this too: "eevee — EEVEE settings for the scene (readonly) — Type: SceneEEVEE | None" (https://docs.blender.org/api/5.2/bpy.types.Scene.html).

Cycles:

> "Render (Max) Samples
>
> The number of paths to trace per pixel in the final render.
> A higher number results in a cleaner image at the cost of a longer render time."

Source: manual, https://docs.blender.org/manual/en/5.2/render/cycles/render_settings/sampling.html — read 2026-07-30.

Workbench:

> "The setting for viewport rendering and final rendering is saved per scene."

Source: manual, https://docs.blender.org/manual/en/5.2/render/workbench/sampling.html — read 2026-07-30.

**LABEL: observed** — in Cycles' own files, the sample count is a property of a per-scene settings block.

> ```
>     samples: IntProperty(
>         name="Samples",
>         description="Number of samples to render for each pixel",
>         min=1, max=(1 << 24),
>         default=4096,
>     )
> ```

File: `Blender Sources/cycles_5.2/properties.py`, lines 485–490, inside `class CyclesRenderSettings(bpy.types.PropertyGroup):` (which begins at line 384). Read 2026-07-30.

That class attaches itself to the Scene type:

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

**LABEL: documented** — the chosen view layer can carry its own sample count, and in Cycles a scene-level switch says how that override is treated.

> "samples
>
> Override number of render samples for this view layer, 0 will use the scene setting (in [0, inf], default 0)"

Source: API documentation, https://docs.blender.org/api/5.2/bpy.types.ViewLayer.html — read 2026-07-30.

> "Samples
>
> Allows the view layer to use a different number of samples than the main scene.
> This is controlled by the Layer Samples
> when using the Cycles renderer. When using EEVEE, the Samples override is always used."

Source: manual, https://docs.blender.org/manual/en/5.2/render/layers/override.html — read 2026-07-30. The same page says: "Overrides are applied only when rendering the specific view layer and do not affect the scene globally."

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

Source: manual, https://docs.blender.org/manual/en/5.2/render/cycles/render_settings/sampling.html — read 2026-07-30.

**LABEL: observed** — the same switch in Cycles' own files, with its default:

> ```
> enum_use_layer_samples = (
>     ('USE', "Use", "Per render layer number of samples override scene samples"),
>     ('BOUNDED', "Bounded", "Bound per render layer number of samples by global samples"),
>     ('IGNORE', "Ignore", "Ignore per render layer number of samples"),
> )
> ```

File: `Blender Sources/cycles_5.2/properties.py`, lines 64–68. And:

> ```
>     use_layer_samples: EnumProperty(
>         name="Layer Samples",
>         description="How to use per view layer sample settings",
>         items=enum_use_layer_samples,
>         default='USE',
>     )
> ```

File: `Blender Sources/cycles_5.2/properties.py`, lines 556–561. Read 2026-07-30.

### Part 2 — what decides them for the *second* scene

**LABEL: inferred.**

The claim: the second scene is rendered with the engine set in that second scene's own render settings, and with that second scene's own sample count for that engine — modified, for the view layer the node names, by that view layer's Samples override where one is set (in Cycles subject to the scene's Layer Samples switch; in EEVEE always).

The step a reader has to supply: the sources say the node names **a Scene** and **a View Layer** of that scene, and that the engine and the sample count are settings held **by a scene** (and by a view layer for the override). No page says outright "the scene a Render Layers node points at is rendered with its own engine and its own samples". I joined those two documented facts. That join is the inference, and nothing in the sources contradicts or confirms it directly.

Also **documented**, and worth keeping separate from the inference: the manual's Properties list for the node holds only Scene and View Layer, and the picture shows only those two controls on the node body — so no engine setting and no sample setting is documented on the node. That much is on the page and in the picture, quoted above. Whether some control sits in the node editor's sidebar without being listed is not settled by the sources; test 7 covers it.

### Part 3 — a value given to the render from outside

**Engine — the option exists. LABEL: documented.**

> "-E, --engine <engine>
>
> Specify the render engine.
> Use -E help to list available engines."

Source: manual, https://docs.blender.org/manual/en/5.2/advanced/command_line/arguments.html — read 2026-07-30.

> "blender -b file.blend -E CYCLES -s 10 -e 500 -t 2 -a"
>
> "-E CYCLES
>
> Use the "Cycles Render" engine.
> For a list of available render engines, run blender -E help."

Source: manual, https://docs.blender.org/manual/en/5.2/advanced/command_line/render.html — read 2026-07-30.

The nearby option that picks which scene is rendered:

> "-S, --scene <name>
>
> Set the active scene <name> for rendering."

Source: manual, https://docs.blender.org/manual/en/5.2/advanced/command_line/arguments.html — read 2026-07-30.

**Effect of that engine value on the second scene — LABEL: not in the sources.**

Neither page, nor anywhere else in the manual, the API documentation, the release notes, or Cycles' own files, says what `-E` applies to: whether it sets the engine of the one scene being rendered, of every scene in the file, or of whatever scene a Render Layers node happens to pull in. The words are only "Specify the render engine." I found exactly two mentions of `-E` in the whole of the 5.2 manual (the two quoted above) and none in the API documentation or the release notes. Nothing anywhere ties a command-line engine value to a scene pulled in by a Render Layers node. My record of looking is below.

**Sample count from outside — LABEL: not in the sources.**

There is no command-line option and no environment variable for a sample count anywhere in the 5.2 sources. The command-line arguments page has no occurrence of the word "sample" at all. The only Cycles-specific command-line options documented are two:

> "--cycles-device <device>
>
> Set the device used for rendering.
> Valid options are: CPU CUDA OPTIX HIP ONEAPI METAL."
>
> "--cycles-print-stats
>
> Log statistics about render memory and time usage."

Source: manual, https://docs.blender.org/manual/en/5.2/advanced/command_line/arguments.html — read 2026-07-30. Cycles' own argument parser declares the same two and no others. **LABEL: observed.**

> ```
> def _configure_argument_parser():
>     import argparse
>     # No help because it conflicts with general Python scripts argument parsing
>     parser = argparse.ArgumentParser(description="Cycles Addon argument parser",
>                                      add_help=False)
>     parser.add_argument("--cycles-print-stats",
>                         help="Print rendering statistics to stderr",
>                         action='store_true')
>     parser.add_argument("--cycles-device",
>                         help="Set the device to use for Cycles, overriding user preferences and the scene setting."
>                              "Valid options are 'CPU', 'CUDA', 'OPTIX', 'HIP', 'ONEAPI', or 'METAL'."
>                              "Additionally, you can append '+CPU' to any GPU type for hybrid rendering.",
>                         default=None)
>     return parser
> ```

File: `Blender Sources/cycles_5.2/engine.py`, lines 8–21, `_configure_argument_parser`. Read 2026-07-30. Neither of the two is a sample count. I mention them only to record that I checked and ruled them out.

*(Quote supplied by the citation check, 2026-08-29. The audit of this document had marked this claim as resting on a file reference with none of the words quoted, and said plainly that it was being put in front of this check for that reason. The lines are as given, and they are what the claim rests on. The wording was also narrowed at the same time: it read "Cycles' own files carry the same two and no others", and what the lines show is what the parser declares — what a run does with an option the parser never declared is a step past reading the file.)*

For the second reading of the question — a value handed in from outside the second scene, by the scene doing the compositing — the sources are likewise silent. **LABEL: not in the sources.** The Render Layers node page lists only Scene and View Layer; there is no documented way for the compositing scene to hand the pulled-in scene an engine or a sample count, and no page describes one scene's render settings standing in for another's.

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

Part 1's documented ground is unchanged, and the inference built on it — that the second scene is rendered with its own engine and its own sample count — is what the runs show.

The two *not in the sources* labels stay exactly as they are. The 5.2 sources still do not say what `-E` applies to, and they still name no outside sample count. *Observed 7* and *Observed 8* answer the behaviour as a separate claim, which is what an observed finding is.

Of the *Not covered* list above, the runs settle the first (`-E` reaches the active scene and nothing else). They do **not** settle the fourth — whether the node's re-render button uses the same engine and sample count as a full render. The operator behind that button was driven and cancelled without rendering, and a press is a mouse click nobody in this round could make; *Observed 10* records that as run and unsettled rather than as an answer. Third-party engines, Sample Subset, and whether the second scene renders at all remain untouched.

One item on the hand-test list below asked to check the node's sidebar by eye for properties not shown on the node body. It was answered without a window, by asking the running program for the node's own property list: two, `scene` and `layer`, and nothing else (*Observed 9*).

---

### Not covered

- Whether `-E` reaches a scene other than the one being rendered. This is the heart of the "from outside" half of the question and the sources do not settle it either way — I am reporting silence, not absence of the behaviour.
- Whether the scene named on a Render Layers node is rendered at all in a given situation, and when. That is a different question and I did not go near it.
- What happens if the second scene's engine differs from the first scene's — e.g. whether passes, or anything else, behave differently. Not asked, not answered.
- Whether the "re-renders it immediately" button beside the View Layer dropdown uses the same engine and sample count as a full render. The page says only that it re-renders.
- Third-party engines registered as add-ons. The sources say "More renderers from third-party developers are available as add-ons. Each renderer has its own render settings to control render quality and performance." (https://docs.blender.org/manual/en/5.2/render/render_engines.html) — nothing about how such an engine stores a sample count or responds to `-E`.
- Sample Subset (Offset/Length) is documented on the Cycles sampling page and can change how many samples actually get rendered; I did not chase whether it interacts with a scene pulled in by a Render Layers node.

### Outside the interface

*(**Note added by the hand-test step, 2026-08-29.** The tests this document asked for have been run, and their results are now written in — see *What running Blender showed — the hand tests of 2026-08-29* above. Both standing lines below have been brought up to date with them: each keeps what the sources say and gains, marked separately, what a run showed. The citation check's note that the answers were still owed stood here until this was done.)*

The command-line arguments page is exactly the "driven from outside its own window" case, and `-E` is documented there, so a way to give the engine from outside does exist. What the sources do **not** say is whether it reaches a second scene pulled in through a Render Layers node — see above. Where things stand:

- The engine and sample settings themselves are stored in the file, on the scene and the view layer — that much is quoted above and **documented**. That they are therefore not interface-only is a step I supplied: **inferred**.
- One documented inside/outside difference sits next door, and it is not about engine or samples: "use_single_layer — Only render the active layer. Only affects rendering from the interface, ignored for rendering from command line. (default False)" (API documentation, https://docs.blender.org/api/5.2/bpy.types.RenderSettings.html, read 2026-07-30). That concerns which view layers render, not which engine or how many samples. I record it because it shows the documentation does mark such differences when they exist — and it marks none for the engine or the sample count.
- The manual's own warning about ordering applies to any outside-driven render: "Arguments are executed in the order they are given!" and "Always position -f or -a as the last arguments." (https://docs.blender.org/manual/en/5.2/advanced/command_line/render.html). The sources do not say where `-E` must sit relative to a render trigger beyond that general rule.

Beyond these, the sources do not say whether the answer differs when Blender is driven from outside.

**Observed, 2026-08-29.** This line can now be answered for Blender 5.2 rather than left open, on the one operating system it was answered on.

- **A render started from inside a running Blender with a window is the same picture, to the last bit, as one started from the command line.** Blender 5.2.1 LTS was started **without** `-b` — so with a real window and a full interface loaded — and handed a script that made `Comp` the current scene, rendered it and quit. The script printed `GUI_BACKGROUND_FLAG: False`, so Blender really was not in background mode. The written image was compared with the one the same file wrote from a background run: `maxdiff = 0`, and both matched the 512-sample reference at `maxdiff = 0` while differing from the 1-sample reference by 17.5005. **So the engine and the sample count that the scene behind the node is rendered with are the same whether Blender has an interface or not.**
- **The limit on that, stated plainly.** The render was started by a script running inside that windowed Blender, not by a person pressing a key. What it shows is that the render path is the same when Blender has a window and a full interface loaded. It does not show what a keypress does, and nothing in this round could.
- **The one interface-only control this answer names is still unsettled.** The re-render button beside the node's View Layer dropdown was not settled: the operator behind it, driven from a script with the node editor in the right state, cancelled without rendering (*Observed 10* above). A press is a mouse click on a window, and there was no machine in this round that could produce one.
- **Everything else above was driven from outside in the first place.** Every run in the observed section other than that one was a background run started from a terminal, which is the case this project cares about.

### Operating system

The sources say nothing that makes any of the above depend on the operating system. The engine and sample settings are scene data in the file; `-E` is listed in the general Render Options of the command-line arguments page, with no platform note, unlike other entries on that same page which do carry one (for example `-con, --start-console` is marked "(Windows only)", and `SPNAV_SOCKET` "(Unix only)"). The one platform-shaped thing I found nearby is the list of valid Cycles **devices** (`CPU CUDA OPTIX HIP ONEAPI METAL`), which are hardware- and platform-specific — but that is the device, not the engine and not the sample count. The sources do not say the answer differs by operating system.

**Observed, 2026-08-29 — and it settles one operating system and leaves the others open.** Every run in the observed section above was made on **one** machine: the Creator's MacBook Pro, Apple M1 Pro, Apple's own graphics and no second card, macOS 26.6.2 (build 25G83), arm64. Nothing was run on Linux and nothing on Windows; no machine was rented for this round. **Untried rather than untestable: a Windows machine can be rented, and every one this project has read reported a Server edition — see the equipment section of `Preparation/Blender Research/Method.md`.** **So whether any of the observed findings hold on any other operating system is still unknown**, and saying so is the whole of what this line can carry. A test settles the operating system it was run on.

Three places where that gap is worth naming rather than left implicit:

- **The one difference between the four researched versions** — that 4.5 does not honour a view layer's sample override under EEVEE where 5.0, 5.1 and 5.2 do (*Observed 5* above) — was found on this Mac, whose graphics are Apple's own. It is worth repeating on a machine with a different graphics card before it is treated as settled everywhere.
- **The windowed run** behind the *Outside the interface* line above was made on a machine whose viewport is drawn by a real graphics card. It could come out differently on a machine whose graphics are drawn by the processor.
- **No third-party render engine was involved anywhere.** Every run used `--factory-startup`, so no add-on engine was loaded, and none is among this project's equipment.

---

## Where I looked

**Files and folders searched (all inside `Blender Sources/`):**

- `manual_5.2/` — whole-tree text searches, plus these pages opened in full: `compositing/types/input/scene/render_layers.txt`, `compositing/types/input/scene/index.txt`, `compositing/compositor_system.txt`, `compositing/introduction.txt`, `advanced/command_line/arguments.txt` (Render Options, Cycles Render Options, Format Options, Python options, Environment Variables), `advanced/command_line/render.txt`, `render/render_engines.txt`, `render/cycles/render_settings/sampling.txt`, `render/eevee/render_settings/sampling.txt`, `render/workbench/sampling.txt`, `render/layers/override.txt`, `scene_layout/scene/introduction.txt`, `scene_layout/scene/properties.txt` (Scene panel, including Background Scene), `editors/properties_editor.txt`, `video_editing/edit/montage/strips/scene.txt`, `interface/controls/nodes/editing.txt` (Read View Layers).
- `manual_5.2/_images/node-types_CompositorNodeRLayers.webp` — the Render Layers node picture, opened and looked at.
- `api_5.2/` — whole-folder searches, plus `bpy.types.CompositorNodeRLayers.txt`, `bpy.types.RenderSettings.txt`, `bpy.types.Scene.txt`, `bpy.types.SceneEEVEE.txt`, `bpy.types.ViewLayer.txt`, `bpy.ops.render.txt`.
- `release_notes_5.2/` — `index.md` and `compositor.md` read in full; `rendering.md`, `core.md`, `cycles.md`, `eevee.md`, `pipeline_io.md`, `python_api.md` searched for engine / samples / scene / command line.
- `cycles_5.2/` — `properties.py` (samples, preview_samples, use_layer_samples, sample subset, the `CyclesRenderSettings` class and its registration onto `bpy.types.Scene`), `engine.py` (the whole command-line parser and `init`), `ui.py` (the sampling panels, which draw from `context.scene`), `operators.py` and `__init__.py` searched for command-line handling. `cycles_5.2/source/` holds only kernel and util headers — no place where scene settings are read — so I ruled it out for this question.

**Terms searched:** "render layers", "another scene", "other scene", "different scene", "second scene", "from a scene", "scene's render", "render engine", "per scene", "per-scene", "engine", "samples", "sample count", "override", "BLENDER_EEVEE", "CYCLES", "-E ", "--engine", "command line", "command-line", "argv", "background", "Environment Variables", "python-expr".

**What I ruled out and why:**

- **The Video Sequencer's Scene strip page.** It is the other way one scene takes in another's render, so I read it in case it stated the rule. It says only "Scene strips are a way to insert the render output of another scene into your sequence" and lists Scene, View Layer, Input, Camera options (manual, https://docs.blender.org/manual/en/5.2/video_editing/edit/montage/strips/scene.html — read 2026-07-30) — nothing about an engine or a sample count. Ruled out as silent, and in any case it is a different mechanism from the compositor's Render Layers node.
- **Background Scene** (Scene Properties). Read in case it was the "takes in another scene" mechanism meant; it describes a scene included in another, not a rendered result taken in, and says nothing about engines or samples.
- **`--cycles-device` and `--cycles-print-stats`.** The only Cycles command-line options that exist in these sources. The device one is described in Cycles' own files as "overriding user preferences and the scene setting" (`Blender Sources/cycles_5.2/engine.py`, lines 16–18) — but it sets the device, which is neither the engine nor the sample count. Ruled out as answering a neighbouring question, not this one.
- **`--python-expr`, `--python`, `--python-text`.** These exist ("Run the given expression as a Python script"), and Python can reach any scene's settings, but no page in these sources describes using them as an engine or sample override, so I do not report them as an answer.
- **Environment variables.** The full list on the command-line arguments page is directories, splash images, an OpenColorIO path, a 3D-mouse socket, and temporary-file locations. None touches the engine or the sample count.
- **Release notes for 5.2.** Searched all subject files; the compositor notes list node and evaluation changes with nothing about which engine a pulled-in scene renders with, and no note anywhere changes `-E` or adds a sample-count argument.
- **`bpy.ops.render.render`.** Its `scene` parameter is documented as "Scene to render, current scene if not specified" and its `layer` parameter as "Single render layer to re-render" (API documentation, https://docs.blender.org/api/5.2/bpy.ops.render.html — read 2026-07-30). That says which scene an operator call renders; it says nothing about the engine or samples used for a scene reached through a Render Layers node, and the sources never connect that operator to the node's re-render button. Ruled out.
- **`cycles_5.2/source/`.** Kernel and util headers only; no Blender-facing settings code. Nothing there could state which scene's settings are used.

---

## Things a person with Blender open could settle in a few minutes

*(**Run, and pruned — 2026-08-29.** Every test this list held was carried out on the Creator's MacBook Pro on 2026-08-29, and repeated on the same machine the same day. The answers are written into *What running Blender showed — the hand tests of 2026-08-29* above, which is where they now live; the answered items have therefore been taken off this list rather than left standing as work for somebody to do. What was set up, what was typed and what the machine printed is recorded in full in `Audits/Blender Research/The compositor/Hand tests — Architecture Fact 135.md`. This list's items were, in that record's numbering, entries 6 (item 1), 7 (item 2), 10 (item 3), 14 (item 4), 15 (item 5), 3 (item 6) and 5 (item 7).)*

**Nothing from this list is still owed.** Four things were not run at all, and they are recorded in that same document: a real press of the re-render button on the Render Layers node — the operator behind it was driven and cancelled without rendering, see *Observed 10* above; anything on Windows; everything above on a second operating system; and any third-party render engine, since every run used `--factory-startup`.

---

2026-08-29 — Claude Opus 5 (Claude Code) — The compositor, Blender 5.2.

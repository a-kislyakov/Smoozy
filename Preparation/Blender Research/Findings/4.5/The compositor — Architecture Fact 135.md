# The compositor — Blender 4.5 — Question 9

**Sources consulted, and when.** Blender 4.5 manual (`Blender Sources/manual_4.5`, including one image from its `_images` folder), Blender 4.5 Python API reference (`Blender Sources/api_4.5`), Blender 4.5 release notes (`Blender Sources/release_notes_4.5`), and the Cycles add-on's own files (`Blender Sources/cycles_4.5`). All four are the snapshot taken on 2026-07-30, and that is the date read for everything cited below. I did the reading on 2026-08-29.

I read only the question sheet and files inside Blender Sources/. I did not open any other part of this project.

I used no helper agents. Everything below I searched and read myself.

---

## Question 9

*When one scene takes in another scene's rendered result, what decides the render engine and the sample count that the second scene is rendered with, and what effect on that does a value for either, given to the render from outside, have?*

### How I read the question

I read "one scene takes in another scene's rendered result" as the Render Layers node in scene A's compositor, with its Scene field set to a different scene B. That is the only way the sources describe one scene taking in another scene's render output inside the compositor.

I read "given to the render from outside" as a value handed to Blender when it is started from the command line, rather than stored in the file. That is the reading I answer. **It is ambiguous.** The other reading is a value written into the file by a script just before the render runs (for example a start-up script setting the engine or the sample count through the Python API). The sources settle neither reading about what reaches the second scene; I say where each one runs out below.

---

### Part 1 — What decides the engine and the sample count

**LABEL: inferred** for the claim that the second scene's own settings are what apply. The pieces underneath it are documented; the joining step is mine, and I write it out.

#### What the sources say outright

The node names a scene and a view layer, and nothing else:

> "Renders a View Layer and reads its
> Passes into the compositing node graph."

> "Scene
>
> The scene for which to render a view layer.
>
> View Layer
>
> The view layer to render. The button next to the dropdown re-renders it immediately."

Source: manual, https://docs.blender.org/manual/en/4.5/compositing/types/input/scene/render_layers.html — read 2026-07-30.

The picture on that same page is a screenshot of the node. It shows the header "Render Layers", the outputs "Image" and "Alpha", a scene field reading "Scene", and a dropdown reading "ViewLayer". There is no engine field and no samples field on the node. Image file `_images/node-types_CompositorNodeRLayers.webp`, on the page https://docs.blender.org/manual/en/4.5/compositing/types/input/scene/render_layers.html — read 2026-07-30. **LABEL for the picture: documented** (it is part of the manual). What is visible is what I state — the node shows two fields; I do not claim from the picture that no such setting exists anywhere else.

The API listing for the node agrees. Its whole set of own properties is:

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

Source: API reference, https://docs.blender.org/api/4.5/bpy.types.CompositorNodeRLayers.html — read 2026-07-30.

**The engine is a property of a scene.** The API puts it on the scene's render settings:

> "engine
>
> Engine to use for rendering"

Source: API reference, https://docs.blender.org/api/4.5/bpy.types.RenderSettings.html — read 2026-07-30. (The same page shows the type as a list of choices holding only `'BLENDER_EEVEE_NEXT'`. Why it holds only that one is not stated on the page, so I take from it only that the property exists and where it lives, not the set of possible values.)

Those render settings hang off a scene, one set per scene:

> "render
>
> Type:
>
> RenderSettings, (readonly, never None)"

Source: API reference, https://docs.blender.org/api/4.5/bpy.types.Scene.html — read 2026-07-30.

The manual speaks of it the same way:

> "Render the 3D Viewport using the scene's Render Engine, for interactive rendering."

Source: manual, https://docs.blender.org/manual/en/4.5/editors/3dview/display/shading.html — read 2026-07-30.

> "To use Cycles, select it as the Render Engine in the Render properties."

Source: manual, https://docs.blender.org/manual/en/4.5/render/cycles/introduction.html — read 2026-07-30.

And the Render properties are per scene:

> "The Properties editor displays settings for the active scene, object, material, and other data types."

> "Tabs related to the active scene:
>
> Render: EEVEE,
> Cycles, or
> Workbench settings."

Source: manual, https://docs.blender.org/manual/en/4.5/editors/properties_editor.html — read 2026-07-30.

> "Each renderer has its own render settings to control render quality and performance."

Source: manual, https://docs.blender.org/manual/en/4.5/render/introduction.html — read 2026-07-30.

Scenes are separate carriers of these settings, to the point that creating one offers to copy them:

> "Copy Settings
>
> Creates an empty scene, but also copies
> the settings from the active scene into the new one."

Source: manual, https://docs.blender.org/manual/en/4.5/scene_layout/scene/introduction.html — read 2026-07-30.

**The sample count is also a property of a scene.** For Cycles this is visible in the add-on's own file. **LABEL: observed.**

> ```
>     samples: IntProperty(
>         name="Samples",
>         description="Number of samples to render for each pixel",
>         min=1, max=(1 << 24),
>         default=4096,
>     )
> ```

File `Blender Sources/cycles_4.5/properties.py`, lines 482–487, inside `class CyclesRenderSettings(bpy.types.PropertyGroup)` which begins at line 376. That class is attached to the scene by its own registration:

> ```
>     @classmethod
>     def register(cls):
>         bpy.types.Scene.cycles = PointerProperty(
>             name="Cycles Render Settings",
>             description="Cycles render settings",
>             type=cls,
>         )
> ```

File `Blender Sources/cycles_4.5/properties.py`, lines 1100–1107. Read 2026-07-30.

The manual's matching wording:

> "Render (Max) Samples
>
> The number of paths to trace per pixel in the final render.
> A higher number results in a cleaner image at the cost of a longer render time."

with the page's own reference line "Panel: Render ‣ Sampling". Source: manual, https://docs.blender.org/manual/en/4.5/render/cycles/render_settings/sampling.html — read 2026-07-30.

For EEVEE the same shape holds:

> "taa_render_samples
>
> Number of samples per pixel for rendering"

Source: API reference, https://docs.blender.org/api/4.5/bpy.types.SceneEEVEE.html — read 2026-07-30 — reached from the scene by:

> "eevee
>
> EEVEE settings for the scene"

Source: API reference, https://docs.blender.org/api/4.5/bpy.types.Scene.html — read 2026-07-30.

**A second thing can decide the sample count: the view layer.** This matters here because the Render Layers node names a view layer, not just a scene.

> "samples
>
> Override number of render samples for this view layer, 0 will use the scene setting"

Source: API reference, https://docs.blender.org/api/4.5/bpy.types.ViewLayer.html — read 2026-07-30.

Cycles carries a scene-level setting of its own about that override, with three choices. **LABEL: observed** — the file shows the setting exists, under this name, with these three choices and this default. What each choice does to a render is the manual's, quoted after it, and is **documented**; the file alone cannot carry it.

> ```
> enum_use_layer_samples = (
>     ('USE', "Use", "Per render layer number of samples override scene samples"),
>     ('BOUNDED', "Bounded", "Bound per render layer number of samples by global samples"),
>     ('IGNORE', "Ignore", "Ignore per render layer number of samples"),
> )
> ```

File `Blender Sources/cycles_4.5/properties.py`, lines 68–72.

> ```
>     use_layer_samples: EnumProperty(
>         name="Layer Samples",
>         description="How to use per view layer sample settings",
>         items=enum_use_layer_samples,
>         default='USE',
>     )
> ```

File `Blender Sources/cycles_4.5/properties.py`, lines 553–558, in the same scene-attached class. Read 2026-07-30.

The manual describes the same control:

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

Source: manual, https://docs.blender.org/manual/en/4.5/render/cycles/render_settings/sampling.html — read 2026-07-30.

> "Samples
>
> View layer samples to override the scene samples.
> Controlled by the layer samples in the Sampling panel."

Source: manual, https://docs.blender.org/manual/en/4.5/render/layers/introduction.html — read 2026-07-30.

#### The step I take, and why it is inferred

No page in these sources says "the scene named on a Render Layers node is rendered with that scene's own engine and sample count". I searched for such a statement and did not find it (record below). What I have is: the node names a scene and a view layer and carries no engine or samples setting of its own; the engine is a property of a scene; the sample count is a property of a scene, optionally overridden by a property of a view layer within that scene, under a scene-level rule. From that I conclude the engine and sample count applied to the second scene are the ones stored on that second scene, and on the named view layer inside it. **A reader has to supply the step "Blender reads these settings from the scene being rendered, not from the scene doing the compositing", so this is inferred, not documented.**

#### One documented reason not to over-trust that step

For at least one other render-wide setting, the manual states the opposite arrangement: the compositing scene wins, and the referenced scene is explicitly disregarded.

> "The views to render are defined in the current scene views,
> in a similar way as you define the composite output resolution in the current scene render panel,
> regardless of the Image nodes resolutions or Render Layers from different scenes."

Source: manual, https://docs.blender.org/manual/en/4.5/render/output/properties/stereoscopy/usage.html — read 2026-07-30. (The same sentence appears at https://docs.blender.org/manual/en/4.5/compositing/types/utilities/switch_stereo_view.html.)

**LABEL: documented** — but documented about multi-view views and output resolution, not about the engine or the sample count. I record it because it shows that "the referenced scene's own settings apply" is not a rule the manual states generally, and that at least one class of setting is taken from the compositing scene instead. It does not, on its own, tell us which way the engine and sample count go.

The sources do confirm that rendering across scenes through the compositor is a real arrangement, in passing:

> "Although the View Map Cache checkbox is a view layer option,
> the cache memory is shared by all view layers and scenes.
> This means that if Freestyle is used for two or more view layers
> (possibly in different scenes through the Compositor),"

Source: manual, https://docs.blender.org/manual/en/4.5/render/freestyle/view_layer/freestyle.html — read 2026-07-30.

---

### Part 2 — A value for either, given to the render from outside

#### The engine: a command-line value exists; its reach is not stated

**LABEL: documented** that the option exists. **LABEL: not in the sources** for what it does to the second scene.

> "-E, --engine <engine>
>
> Specify the render engine.
> Use -E help to list available engines."

Source: manual, https://docs.blender.org/manual/en/4.5/advanced/command_line/arguments.html — read 2026-07-30.

> "-E CYCLES
>
> Use the "Cycles Render" engine.
> For a list of available render engines, run blender -E help."

Source: manual, https://docs.blender.org/manual/en/4.5/advanced/command_line/render.html — read 2026-07-30.

The identifier that option takes for Cycles comes from the add-on itself. **LABEL: observed.**

> ```
> class CyclesRender(bpy.types.RenderEngine):
>     bl_idname = 'CYCLES'
>     bl_label = "Cycles"
> ```

File `Blender Sources/cycles_4.5/__init__.py`, lines 42–44. Read 2026-07-30.

Neither page says which scene or scenes `-E` applies to. Nowhere in the manual, the API reference or the release notes did I find a statement that it sets the engine on the active scene only, on every scene in the file, or on the scene that a compositor node happens to reference. The nearest thing on the same page is a separate option for choosing the scene:

> "-S, --scene <name>
>
> Set the active scene <name> for rendering."

Source: manual, https://docs.blender.org/manual/en/4.5/advanced/command_line/arguments.html — read 2026-07-30. That tells us there is a notion of one active scene for a command-line render, and nothing about how `-E` relates to scenes other than it.

So: whether an engine given from outside reaches the second scene is **not in the sources**.

#### The sample count: the sources list no command-line option that sets one

**LABEL: inferred.** The two readings underneath are documented and observed and are given below. The claim that no command-line option sets a sample count rests on absence — a list that reads as complete, plus the Cycles add-on's own list of the arguments it accepts. No sentence anywhere in these sources says there is no such option, so a reader has to supply the step "these two lists are the whole of it". That step is mine, and it is why this is inferred rather than documented.

The manual's list of render-related command-line options, as that page gives it, is `-b/--background`, `-a/--render-anim`, `-S/--scene`, `-f/--render-frame`, `-s/--frame-start`, `-e/--frame-end`, `-j/--frame-jump`, `-o/--render-output`, `-E/--engine`, `-t/--threads`, followed by a separate heading:

> "Cycles Render Options
>
> Cycles add-on options must be specified following a double dash.
>
> --cycles-device <device>
>
> Set the device used for rendering.
> Valid options are: CPU CUDA OPTIX HIP ONEAPI METAL.
>
> Append +CPU to a GPU device to render on both CPU and GPU.
>
> Example:
>
> blender -b file.blend -f 20 -- --cycles-device OPTIX
>
> --cycles-print-stats
>
> Log statistics about render memory and time usage."

Source: manual, https://docs.blender.org/manual/en/4.5/advanced/command_line/arguments.html — read 2026-07-30. Nothing in that list sets samples.

The Cycles add-on's own argument parser confirms the same two options and no more. **LABEL: observed.**

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

File `Blender Sources/cycles_4.5/engine.py`, lines 8–21. The parser is used by `_parse_command_line()` at lines 24–41 of the same file, which is called from `init()` at line 53. Read 2026-07-30. A search of all four source folders for `cycles-samples`, `cycles_samples`, `--samples` and `samples-override` returned nothing.

Note what the `--cycles-device` help text does say, since it is the one place in these files where an outside value is described as beating a stored one: **"overriding user preferences and the scene setting"**. That is about the device. There is no comparable sentence anywhere in the sources for the engine or for a sample count, and I draw no conclusion from the contrast beyond recording it.

**So, for the sample count: no command-line option that sets one appears anywhere in these sources, so on this reading there is nothing for a value given from outside to act through. LABEL: inferred**, on the step written at the head of this part. If that holds, the sample count comes from the file — the second scene's own setting, or the named view layer's override of it — and which of those two applies is the **inferred** conclusion of Part 1, not a documented one.

#### The other reading of "from outside"

If "given to the render from outside" instead means a value written by a script before the render, the sources tell us the properties are writable in principle (they are ordinary scene and view-layer properties in the API, and `bpy.ops.render.render` takes a scene: "scene (string, (optional, never None)) – Scene, Scene to render, current scene if not specified", https://docs.blender.org/api/4.5/bpy.ops.render.html, read 2026-07-30). But nothing in the sources says how a value set that way on one scene relates to a second scene reached through a Render Layers node. **Not in the sources** for that reading too.

**That silence is thinner than the others here, and is marked as such rather than left looking equal to them.** The record of where I looked, below, does not cover the route by which a script reaches a render started from outside: it holds no search for the arguments that hand Blender a script or an expression to run at start-up, and none for start-up scripts. So this reading is unsearched at its own first step, and the silence should not be treated as settled until a fresh pass has searched it.

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

Part 1's conclusion — that the engine and sample count applied to the second scene are the ones stored on that second scene and on the named view layer inside it — was reached here as **inferred**, and the runs show the inference was right. That label stays as it is: it describes what the 4.5 sources support, and the observed answer sits beside it rather than replacing it.

Part 2's *not in the sources* for what an engine given from outside does to the second scene **also stays as it is**. The 4.5 manual, API documentation and release notes still say nothing about it; that silence is a fact about the documentation and no run can fill it. What the runs add is the separate answer: `-E` does not reach the scene behind the node, because it writes onto the active scene alone (*Observed 7*).

Part 2's **inferred** claim that no command-line option sets a sample count now has a second witness: the program's own `--help` says the same, in all the sections where such an option would sit (*Observed 8*). The label stays inferred for what it was — a reading of a page that reads as complete.

The second reading of "from outside" — a value written by a script before the render — which this document recorded as a silence it had **not searched**: the record of where I looked is untouched and that silence is still unsearched in these sources. But the route itself was run, and it works: a script given on the command line reaches the sample count of the scene behind the node (*Observed 8*).

Of the *NOT COVERED* list above, the runs settle the first (what `-E` writes: the active scene alone, for the run only, at the point in the command line where it appears) and the fourth (the view-layer override is consulted through the node, under the pulled-in scene's own Layer Samples policy). They settle nothing about the viewport compositor, about third-party engines, or about whether the second scene renders at all — that last is question 4's ground, not this one's.

---

### NOT COVERED

- Whether `-E` writes the engine onto the active scene alone, onto every scene in the file, or onto whatever scene is being rendered at the moment. Not stated anywhere I searched.
- What happens when the two scenes have different engines set — whether both engines run in one render, and whether that is supported at all.
- Whether a Render Layers node pointing at another scene causes that scene to render at all in a command-line render. This answer assumes the case where it does and asks only what settings would then apply; it does not establish the triggering.
- Whether the second scene's view-layer sample override is consulted when the view layer is reached through the node rather than through the scene's own render. The override and its scene-level rule are documented; their behaviour in this specific path is not.
- Anything about a sample count outside Cycles and EEVEE — Workbench sampling was not examined, and no other engine's settings are in these sources.
- Whether the engine or sample count in force differ between a final render and the viewport compositor.
- Whether any of the above changed in 4.5; the release notes carry nothing on it (see record below).
- The second reading of "from outside" — a value written by a script before the render — at its own first step. The record below holds no search for how a script is handed to a render started from outside, so the silence recorded for that reading is unsearched rather than established.

---

### Outside the interface

*(**Note added by the hand-test step, 2026-08-29.** The tests this document asked for have been run, and their results are now written in — see *What running Blender showed — the hand tests of 2026-08-29* above. Both standing lines below have been brought up to date with them: each keeps what the sources say and gains, marked separately, what a run showed. The citation check's note that the answers were still owed stood here until this was done.)*

The engine part of this answer is *about* the outside-the-interface case: `-E`/`--engine` exists only as a command-line argument, given when Blender is started by something other than a person clicking. The manual presents it under "Render Options" alongside `-b, --background` ("Run in background (often used for UI-less rendering)"), and the worked examples on the rendering-from-the-command-line page all use `-b`. So the option is documented as usable in exactly the driven-from-outside case. What it does to a scene other than the active one is still not stated, inside or outside.

For the sample count, the absence of a command-line option is itself an outside-the-interface fact, and it is **inferred** from absence rather than documented. It does not amount to "there is no way at all to hand a sample count to a render from outside": the other reading above — a script run before the render — is a from-outside route that these sources neither rule out nor settle, and it was not searched for. So what can be said is narrower: no command-line option that sets a sample count appears in these sources, and where a driven render then reads the sample count from is the **inferred** answer of Part 1.

The one thing on the Render Layers node page that is interface-only is the re-render button: "The button next to the dropdown re-renders it immediately." That is a click, and has no counterpart described for a driven render.

Beyond that, the sources do not say whether where the engine and sample count are read from differs between an interface render and a driven one.

**Observed, 2026-08-29.** This line can now be answered for Blender 4.5 rather than left open, on the one operating system it was answered on.

- **A render started from inside a running Blender with a window is the same picture, to the last bit, as one started from the command line.** Blender 4.5.12 LTS was started **without** `-b` — so with a real window and a full interface loaded — and handed a script that made `Comp` the current scene, rendered it and quit. The script printed `GUI_BACKGROUND_FLAG: False`, so Blender really was not in background mode. The written image was compared with the one the same file wrote from a background run: `maxdiff = 0`, and both matched the 512-sample reference at `maxdiff = 0` while differing from the 1-sample reference by 17.5015. **So the engine and the sample count that the scene behind the node is rendered with are the same whether Blender has an interface or not.**
- **The limit on that, stated plainly.** The render was started by a script running inside that windowed Blender, not by a person pressing a key. What it shows is that the render path is the same when Blender has a window and a full interface loaded. It does not show what a keypress does, and nothing in this round could.
- **The one interface-only control this answer names is still unsettled.** The re-render button beside the node's View Layer dropdown was not settled: the operator behind it, driven from a script with the node editor in the right state, cancelled without rendering (*Observed 10* above). A press is a mouse click on a window, and there was no machine in this round that could produce one.
- **Everything else above was driven from outside in the first place.** Every run in the observed section other than that one was a background run started from a terminal, which is the case this project cares about.

### Operating system

The sources do not say. The command-line arguments page draws no operating-system distinction for `-E`, `-S` or the Cycles options; the only place it distinguishes platforms in this area is the list of valid `--cycles-device` values (CPU CUDA OPTIX HIP ONEAPI METAL), which is about hardware devices, not about the engine or the sample count. The manual's launch pages are split per operating system (Linux, macOS, Windows) but concern how to invoke Blender, not what the arguments do. Nothing in the release notes ties engine choice or sample count to an operating system.

**Observed, 2026-08-29 — and it settles one operating system and leaves the others open.** Every run in the observed section above was made on **one** machine: the Creator's MacBook Pro, Apple M1 Pro, Apple's own graphics and no second card, macOS 26.6.2 (build 25G83), arm64. Nothing was run on Linux and nothing on Windows; no machine was rented for this round. **Untried rather than untestable: a Windows machine can be rented, and every one this project has read reported a Server edition — see the equipment section of `Preparation/Blender Research/Method.md`.** **So whether any of the observed findings hold on any other operating system is still unknown**, and saying so is the whole of what this line can carry. A test settles the operating system it was run on.

Three places where that gap is worth naming rather than left implicit:

- **The one difference between the four researched versions** — that 4.5 does not honour a view layer's sample override under EEVEE where 5.0, 5.1 and 5.2 do (*Observed 5* above) — was found on this Mac, whose graphics are Apple's own. It is worth repeating on a machine with a different graphics card before it is treated as settled everywhere.
- **The windowed run** behind the *Outside the interface* line above was made on a machine whose viewport is drawn by a real graphics card. It could come out differently on a machine whose graphics are drawn by the processor.
- **No third-party render engine was involved anywhere.** Every run used `--factory-startup`, so no add-on engine was loaded, and none is among this project's equipment.

---

## Record of where I looked

**Folders searched, whole-folder text searches:** `Blender Sources/manual_4.5`, `Blender Sources/api_4.5`, `Blender Sources/release_notes_4.5`, `Blender Sources/cycles_4.5`.

**Terms searched (case-insensitive where useful):**
`render layers`, `another scene`, `other scene`, `other scenes`, `different scene`, `different scenes`, `second scene`, `multiple scenes`, `each scene`, `own render settings`, `per-scene`, `each scene has`, `render settings of`, `rendered using`, `will be rendered with`, `Render Engine`, `active scene`, `--engine`, `-E `, `cycles-samples`, `cycles_samples`, `--samples`, `samples-override`, `samples`, `Layer Samples`, `per view layer`, `Sample Override`, `argv`, `argparse`, `cycles-device`, `print-stats`, `engine`, `command line` / `command-line` / `argument` (release notes), `PointerProperty` / `bpy.types.Scene` / `bpy.types.ViewLayer` (Cycles files).

**Pages and files opened:**
- Manual: `compositing/types/input/scene/render_layers.txt`; `compositing/compositor_system.txt`; `compositing/introduction.txt`; `compositing/limitations.txt`; `compositing/sidebar.txt`; `compositing/types/input/scene/` listing; `advanced/command_line/arguments.txt`; `advanced/command_line/render.txt`; `advanced/command_line/` listing including `launch/`; `render/introduction.txt`; `render/cycles/render_settings/sampling.txt`; `render/layers/introduction.txt`; `render/freestyle/view_layer/freestyle.txt`; `render/output/properties/stereoscopy/usage.txt`; `editors/properties_editor.txt`; `editors/3dview/display/shading.txt` (via search hits); `render/cycles/introduction.txt` (via search hits); `scene_layout/scene/introduction.txt`; `video_editing/edit/montage/strips/scene.txt`; `interface/controls/nodes/editing.txt`.
- Manual image: `_images/node-types_CompositorNodeRLayers.webp`.
- API: `bpy.types.CompositorNodeRLayers.txt`; `bpy.types.RenderSettings.txt`; `bpy.types.Scene.txt`; `bpy.types.ViewLayer.txt`; `bpy.types.SceneEEVEE.txt`; `bpy.types.Depsgraph.txt`; `bpy.types.RenderEngine.txt`; `bpy.ops.render.txt`.
- Release notes: `compositor.md`; `cycles.md`; searched all files in the folder for engine, command-line and argument terms.
- Cycles files: `__init__.py`; `engine.py`; `properties.py`; folder listing of `source/` (only `kernel` and `util` are present — there is no Blender-integration code in this snapshot, so nothing there can show how a scene's settings are read).

**What I ruled out and why:**
- **The VSE scene strip page** (`video_editing/edit/montage/strips/scene.txt`). It is the other way one scene takes in another's render output, so I checked it in case it documented engine or sample handling. It documents a camera override and an Input choice (Camera or Sequencer), and states "Scene strips do not render individual Render Passes; only the Combined render pass will be used." It says nothing about engine or samples. It is also not the compositor, so it does not answer the question either way.
- **Cycles `source/`**. Only kernel and utility code; no code that reads a scene's settings. Ruled out as unable to bear on the question.
- **Release notes.** Searched all subjects; the only related lines were a bug-fix entry "Crash when compositor references another scene" (https://developer.blender.org/docs/release_notes/4.5/bugfixes/) and a new `--profile-gpu` argument under EEVEE. Neither touches which engine or sample count a referenced scene is rendered with.
- **`bpy.types.RenderEngine`**. Its worked example reads settings from the dependency graph's scene (`scene = depsgraph.scene`, then `scene.render.resolution_percentage`, `scene.render.resolution_x`), and `Depsgraph.scene` is documented as "Original scene dependency graph is built for". I considered building the inference on this, but the example is a template for writing a *custom* engine, not a statement about how Blender picks an engine or reads samples for a compositor-referenced scene, so I did not lean on it and it does not change the label above.

---

## Things a person could settle by hand in a few minutes

*(**Run, and pruned — 2026-08-29.** Every test this list held was carried out on the Creator's MacBook Pro on 2026-08-29, and repeated on the same machine the same day. The answers are written into *What running Blender showed — the hand tests of 2026-08-29* above, which is where they now live; the answered items have therefore been taken off this list rather than left standing as work for somebody to do. What was set up, what was typed and what the machine printed is recorded in full in `Audits/Blender Research/The compositor/Hand tests — Architecture Fact 135.md`. This list's items were, in that record's numbering, entries 6 (items 1 and 2), 7 (item 3), 10 (item 4), 14 (item 5), 15 (item 6), 3 and 4 (item 7) and 2 (item 8).)*

**Nothing from this list is still owed.** Four things were not run at all, and they are recorded in that same document: a real press of the re-render button on the Render Layers node — the operator behind it was driven and cancelled without rendering, see *Observed 10* above; anything on Windows; everything above on a second operating system; and any third-party render engine, since every run used `--factory-startup`.

---

2026-08-29 — Claude Opus 5 (Claude Code) — The compositor, Blender 4.5.

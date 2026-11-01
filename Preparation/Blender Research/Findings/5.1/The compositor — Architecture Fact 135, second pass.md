# The compositor — Blender 5.1 — Question 9 (second pass)

Sources consulted: the Blender 5.1 manual (`Blender Sources/manual_5.1`, including its `_images` folder), the Blender 5.1 Python API documentation (`Blender Sources/api_5.1`), the Blender 5.1 release notes (`Blender Sources/release_notes_5.1`), and Cycles' own files (`Blender Sources/cycles_5.1`). All were read on 2026-07-30 (the snapshot date given to me).

I read only the question sheet and files inside Blender Sources/. I did not open any other part of this project.

---

## 9. When one scene takes in another scene's rendered result, what decides the render engine and the sample count that the second scene is rendered with, and what effect on that does a value for either, given to the render from outside, have?

### The reading I answered, and the other reading

The question is ambiguous in one phrase: "given to the render from outside".

I answered this reading: **a value for the engine or for the sample count that is handed to the render from outside the blend-file — on the command line, in the environment, or as an argument to a render call — rather than being stored in the file.**

The other reading is: **"outside" meaning the outer scene** — that is, does the first scene's own engine or sample count impose itself on the second scene. I answer that reading too, briefly, at the end of part C, because it is short and it would be worse to leave it hanging.

I read "one scene takes in another scene's rendered result" as the Render Layers node in scene A's compositor with its Scene field set to scene B. That is the compositor's only way of taking in another scene's render, and this sheet is about the compositor. (The Video Sequencer's Scene strip is a second, separate way; I note what I found about it in the record of looking, and it changes nothing here.)

---

### A. What decides the render engine

**LABEL: inferred.**

No page in the 5.1 manual or API documentation says outright which engine is used when a Render Layers node points at a second scene. The claim below is assembled from three documented facts, and the step is written out.

The three documented facts:

1. The node renders the scene named in its own Scene field.

   > "Renders a View Layer and reads its Passes into the compositing node graph."

   and, under Properties:

   > "Scene
   >
   > The scene for which to render a view layer.
   >
   > View Layer
   >
   > The view layer to render. The button next to the dropdown re-renders it immediately."

   Source: the manual, https://docs.blender.org/manual/en/5.1/compositing/types/input/scene/render_layers.html — read 2026-07-30.

2. Render settings belong to a scene.

   > "A scene defines the visible objects, camera view, lighting setup, and render settings that make up a specific part of a project."

   Source: the manual, https://docs.blender.org/manual/en/5.1/scene_layout/scene/introduction.html — read 2026-07-30.

3. The engine is one of those per-scene render settings. In the API documentation, `RenderSettings` carries the engine, and every scene has its own `RenderSettings`:

   > "engine
   >
   > Engine to use for rendering (default 'BLENDER_EEVEE')"

   Source: the API documentation, https://docs.blender.org/api/5.1/bpy.types.RenderSettings.html — read 2026-07-30.

   > "render
   >
   > (readonly, never None)
   >
   > Type:
   >
   > RenderSettings"

   Source: the API documentation, https://docs.blender.org/api/5.1/bpy.types.Scene.html — read 2026-07-30.

   And the manual points to where a person sets it:

   > "To use Cycles, select it as the Render Engine in the Render properties."

   Source: the manual, https://docs.blender.org/manual/en/5.1/render/cycles/introduction.html — read 2026-07-30.

**The step I took:** the node renders a view layer of scene B (fact 1); the engine is a setting held by a scene (facts 2 and 3); therefore the engine used is scene B's own engine setting, not scene A's. A reader has to supply that joining step, so this is inferred, not documented.

Supporting picture, and what it does and does not show: the manual's screenshot of the node, `_images/node-types_CompositorNodeRLayers.webp` (shown on the Render Layers page above), read 2026-07-30. It shows the node with two outputs (Image, Alpha) and exactly two fields, a Scene field and a ViewLayer field. **Documented, from the picture:** the node itself offers no engine control and no sample control. What engine it then uses is not visible in the picture and is not something the picture settles.

### B. What decides the sample count

**LABEL: inferred for the main claim; documented for the modifiers listed under it.**

The main claim — that scene B is rendered with scene B's own sample setting — is inferred by exactly the same step as in part A, from the same three facts, plus the following documented facts that the sample count is a per-scene render setting.

Cycles, in the manual, under "Panel: Render ‣ Sampling":

> "Render (Max) Samples
>
> The number of paths to trace per pixel in the final render.
> A higher number results in a cleaner image at the cost of a longer render time."

Source: the manual, https://docs.blender.org/manual/en/5.1/render/cycles/render_settings/sampling.html — read 2026-07-30.

EEVEE, in the manual, also under "Panel: Render ‣ Sampling", in its "Render" section:

> "Samples
>
> The number of samples to use in the final render."

Source: the manual, https://docs.blender.org/manual/en/5.1/render/eevee/render_settings/sampling.html — read 2026-07-30.

Workbench, in the manual:

> "The setting for viewport rendering
> and final rendering is saved per scene."

Source: the manual, https://docs.blender.org/manual/en/5.1/render/workbench/sampling.html — read 2026-07-30.

That the Cycles sample count hangs off the scene is also **observed** in Cycles' own files. In `Blender Sources/cycles_5.1/properties.py`, inside `class CyclesRenderSettings` (lines 479–484):

```
    samples: IntProperty(
        name="Samples",
        description="Number of samples to render for each pixel",
        min=1, max=(1 << 24),
        default=4096,
    )
```

and that class attaches itself to the Scene type (same file, lines 1101–1107):

```
    @classmethod
    def register(cls):
        bpy.types.Scene.cycles = PointerProperty(
            name="Cycles Render Settings",
            description="Cycles render settings",
            type=cls,
        )
```

The API documentation says the same for EEVEE, in so many words:

> "eevee
>
> EEVEE settings for the scene (readonly)"

Source: the API documentation, https://docs.blender.org/api/5.1/bpy.types.Scene.html — read 2026-07-30.

Four further things inside the second scene are documented to change the number of samples that scene is actually rendered with. All are **documented**, and all are settings of the scene or of its view layer — none of them come from outside.

1. A per-view-layer sample override, and the scene-level policy that governs it:

   > "Samples
   >
   > Allows the view layer to use a different number of samples than the main scene.
   > This is controlled by the Layer Samples
   > when using the Cycles renderer. When using EEVEE, the Samples override is always used."

   and on the same page:

   > "Overrides are applied only when rendering the specific view layer and do not affect the scene globally."

   Source: the manual, https://docs.blender.org/manual/en/5.1/render/layers/override.html — read 2026-07-30.

   The API documentation gives the same override:

   > "samples
   >
   > Override number of render samples for this view layer, 0 will use the scene setting (in [0, inf], default 0)"

   Source: the API documentation, https://docs.blender.org/api/5.1/bpy.types.ViewLayer.html — read 2026-07-30.

   The governing policy, in Cycles' render settings:

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

   Source: the manual, https://docs.blender.org/manual/en/5.1/render/cycles/render_settings/sampling.html — read 2026-07-30. (The three choices are also **observed** in `Blender Sources/cycles_5.1/properties.py`, lines 64–68, as `enum_use_layer_samples`, with default `'USE'` at lines 550–557.)

   This matters here because a Render Layers node names a view layer as well as a scene, so which view layer is named can itself change the sample count used.

2. Cycles can stop before the sample count is reached:

   > "Stops rendering if the time exceeds the limit, even if the desired sample count
   > hasn't been reached yet. Setting this value to zero disables the limit."

   and:

   > "If the Noise Threshold checkbox is enabled, Cycles will use adaptive sampling,
   > cutting short the sampling process in areas that have become less noisy than the
   > specified threshold value"

   Source: the manual, https://docs.blender.org/manual/en/5.1/render/cycles/render_settings/sampling.html — read 2026-07-30.

3. EEVEE's jittered depth of field can change the count:

   > "Randomize the camera position for every scene render sample to increase precision.
   > Enabling this option can change the scene's actual sample count."

   Source: the manual, https://docs.blender.org/manual/en/5.1/render/eevee/render_settings/depth_of_field.html — read 2026-07-30.

4. EEVEE's motion blur rounds the count up:

   > "When using multiple time steps, the render sample count is rounded up to the next multiple
   > of steps to ensure even distribution of samples across steps."

   Source: the manual, https://docs.blender.org/manual/en/5.1/render/eevee/render_settings/motion_blur.html — read 2026-07-30.

### C. What a value given from outside does to that

Two separate answers, because the engine and the sample count are not in the same position.

**C1. A sample count given from outside: these sources describe none. LABEL: inferred, on a documented absence and an observed one.**

*(Heading and label changed by the citation check, 2026-08-29. They read "there is none. LABEL: documented (that no such input exists), plus observed." Two faults in one line. Nothing quoted below says there is no such option; what is quoted is one manual page's two lists and the two arguments Cycles' own parser declares. Getting from those to "there is none" needs a reader to supply the step that these lists are the whole of it — so the label is inferred, and the step is written out here rather than left to be worked out. The pieces underneath keep their own labels, documented for the manual and observed for the file, and are unchanged.)*

The manual's full list of command-line arguments has a "Render Options" section and a "Cycles Render Options" section. Nothing in either sets a sample count. The Cycles section reads in full:

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

Source: the manual, https://docs.blender.org/manual/en/5.1/advanced/command_line/arguments.html — read 2026-07-30.

The same page's "Environment Variables" section lists these variables and no others (`BLENDER_USER_RESOURCES`, `BLENDER_USER_CONFIG`, `BLENDER_USER_SCRIPTS`, `BLENDER_USER_EXTENSIONS`, `BLENDER_USER_DATAFILES`, `BLENDER_SYSTEM_RESOURCES`, `BLENDER_SYSTEM_SCRIPTS`, `BLENDER_SYSTEM_EXTENSIONS`, `BLENDER_SYSTEM_DATAFILES`, `BLENDER_SYSTEM_PYTHON`, `BLENDER_CUSTOM_SPLASH`, `BLENDER_CUSTOM_SPLASH_BANNER`, `BLENDER_OCIO`, `SPNAV_SOCKET`, `TEMP`, `TMPDIR`). None concerns samples or the engine.

This is confirmed **observed** in Cycles' own files. `Blender Sources/cycles_5.1/engine.py`, lines 8–21, is the whole of the Cycles argument parser, and it defines exactly two arguments:

```
def _configure_argument_parser():
    import argparse
    # No help because it conflicts with general Python scripts argument parsing
    parser = argparse.ArgumentParser(description="Cycles Addon argument parser",
                                     add_help=False)
    parser.add_argument("--cycles-print-stats",
                        help="Print rendering statistics to stderr",
                        action='store_true')
    parser.add_argument("--cycles-device",
                        help="Set the device to use for Cycles, overriding user preferences and the scene setting."
                             "Valid options are 'CPU', 'CUDA', 'OPTIX', 'HIP', 'ONEAPI', or 'METAL'."
                             "Additionally, you can append '+CPU' to any GPU type for hybrid rendering.",
                        default=None)
    return parser
```

So: **no argument in these sources sets a sample count**, for the second scene or for any scene, and the sources therefore say nothing about what such a value would do to the second scene. *(Narrowed by the citation check, 2026-08-29. The sentence read "no sample count can be given to a render from outside in 5.1, and the question of its effect on the second scene does not arise." That is a closed door stated as fact, and it is contradicted by the next sentence of this same paragraph, which names a route by which an outside caller reaches the setting. What the quotes carry is a silence, not a denial.)* One route the sources do describe: an outside caller can change the stored setting — for instance by running a Python script with `-P`, which the manual lists as "Run the given Python script file" on the same command-line arguments page. What such a script does depends entirely on the script, and the sources say nothing about it.

The render call itself carries no sample count either. The API documentation gives the render operator's complete signature:

> "bpy.ops.render.render(*, animation=False, write_still=False, use_viewport=False, use_sequencer_scene=False, layer='', scene='', frame_start=0, frame_end=0)"

Source: the API documentation, https://docs.blender.org/api/5.1/bpy.ops.render.html — read 2026-07-30. There is a `scene` parameter ("Scene, Scene to render, current scene if not specified") and a `layer` parameter, but no engine and no samples.

**C2. An engine given from outside: the input exists, but its effect on the second scene is not in the sources. LABEL: documented that the input exists; not in the sources for its effect on the second scene.**

The input exists:

> "-E, --engine <engine>
>
> Specify the render engine.
> Use -E help to list available engines."

Source: the manual, https://docs.blender.org/manual/en/5.1/advanced/command_line/arguments.html — read 2026-07-30.

And, worked through as an example:

> "blender -b file.blend -E CYCLES -s 10 -e 500 -t 2 -a"
>
> "-E CYCLES
>
> Use the "Cycles Render" engine.
> For a list of available render engines, run blender -E help."

Source: the manual, https://docs.blender.org/manual/en/5.1/advanced/command_line/render.html — read 2026-07-30.

That is the entire documented content of `-E` in 5.1. **The sources never say whose engine it sets** — whether it changes the render engine of every scene in the file, only of the scene that is being rendered, or only of the scene that is active when the argument is processed. The neighbouring argument is scene-specific in its wording —

> "-S, --scene <name>
>
> Set the active scene <name> for rendering."

— but that is about which scene is rendered, not about which scenes `-E` reaches, and reading a scope into `-E` from it would be my invention. So: **what `-E` does to a second scene pulled in by a Render Layers node is not in the sources.** Where I looked for it is recorded below.

One further thing the sources do say about outside values and rendering order, which bears on using `-E` at all:

> "Note
>
> Arguments are executed in the order they are given!"

Source: the manual, https://docs.blender.org/manual/en/5.1/advanced/command_line/render.html — read 2026-07-30.

**C3. The other reading — a value coming from the outer scene. LABEL: inferred, same step as parts A and B.**

If "from outside" means the first scene, then by the same reasoning: the engine and sample count are settings held by each scene separately, so scene A's engine and sample count are scene A's, and the sources give no mechanism by which they reach scene B. Nothing in the manual, the API documentation, the release notes or Cycles' files describes a scene imposing its engine or its sample count on a scene it pulls in. That absence is part of the record below.

---

### What running Blender showed — the hand tests of 2026-08-29

**LABEL: observed**, for everything in this section and for nothing above it.

**These are two separate claims and they are kept apart.** Everything above this heading is what the Blender 5.1 sources say, and it stands unchanged — including every *not in the sources*, which is a statement about the documentation and is not undone by a run. What follows is the other claim: what Blender 5.1 actually did. Where the two meet, both are recorded — the reading as a reading, the run as a run.

**On what kind of computer, and when.** The Creator's own MacBook Pro: an Apple M1 Pro chip, Apple's own graphics, no second card. Read off the machine on the day:

```
ProductName:		macOS
ProductVersion:		26.6.2
BuildVersion:		25G83
arm64
Apple M1 Pro
```

The Blender was `/Applications/Blender 5.1.2.app/Contents/MacOS/Blender`, which reports:

```
Blender 5.1.2 (hash ec6e62d40fa9 built 2026-05-19 01:30:33)
```

Every run was made on **2026-08-29**, and every one was repeated on the same machine the same day by the citation check, with the same result each time. Every run used `--factory-startup`, so no third-party add-on was loaded and no add-on render engine existed in any of them. **Nothing was run on any other operating system** — see the operating-system line below.

**What was run against.** One set of files, built by Blender 5.1.2 itself so that no file crossed a version boundary. Each holds two scenes: **`Comp`**, whose compositor is being run, and **`Sub`**, the scene pulled in. Both hold a camera, one area light, a metallic sphere of roughness 0.35 and a blue floor, with the two spheres in different places so the two scenes are not the same picture. `Comp`'s compositor holds one **Render Layers** node with its Scene set to `Sub` and its View Layer set to `Sub`'s own `ViewLayer`, wired straight to the output and nothing else — so the written image is entirely `Sub`'s render, and any change in it is a change in how `Sub` was rendered. Both scenes: 160 by 120 pixels, one frame, uncompressed 32-bit OpenEXR, compositing on, stamping off, Cycles on the processor with denoising off, adaptive sampling off, no time limit and a fixed seed of 7, so a Cycles render repeats to the bit. Only the engines, sample counts, view-layer overrides and Layer Samples choices differ from file to file, and each finding below names the file it used. The script that builds them is printed in full in `Audits/Blender Research/The compositor/Hand tests — Architecture Fact 135.md`.

Every run took this shape, with the file, the extra arguments and the output path changing:

```
"/Applications/Blender 5.1.2.app/Contents/MacOS/Blender" --factory-startup -b "<file>.blend" --log "render" -o "<output>_" -f 1
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

The zeros are exact zeros. (The two non-zero figures are recorded as the same across all four researched versions to within the third decimal place; the zeros are Blender 5.1's own.)

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

Blender 5.1.2 printed `Rendering 8 / 8 samples`, and the written image matched the 8-sample reference at `maxdiff = 0` and differed from the 64-sample reference, at `maxdiff` 2.633.

**Blender 5.1.2 honours it. Blender 4.5 does not** — this is the one place in the whole round where the four researched versions differ, and each of the four is carrying the difference so that nobody later mistakes it for something else. The same test in 4.5 printed `Rendering 64 / 64 samples` and matched the 64-sample reference exactly, ignoring the override.

**It is not a difference in what the compositor does, and this was established rather than assumed.** The same file was also rendered with `-S Sub`, so `Sub` was the active scene and no compositor was involved at all. In 5.1.2 that render came out `maxdiff = 0` against the 8-sample reference too, and in 4.5 against the 64-sample one. So what differs is whether EEVEE honours a view layer's sample override at all in a given version; the path through the node reports faithfully whichever way that goes.

**It was re-run before being written down**, because it was the round's only disagreement between versions. Blender 5.1.2 was run again on the same file and printed `Rendering 8 / 8 samples`.

---

**Observed 6 — a scene reached through the node renders exactly as it does when it is the scene being rendered.**

Ten of the files above were rendered twice over: once through the node with `Comp` active, and once directly with `-S Sub` so that no compositor was involved. Those ten — `eng_Awb_Bcy`, `eng_Acy_Bwb`, `eng_Acy_Bee`, `smp_A1_B512`, `smp_A512_B1`, `smp_A1_B128`, `vl_use`, `ee_A4_B64`, `ee_A64_B4` and `ee_vl` — cover all three engines, four Cycles sample counts, two EEVEE sample counts, and a view-layer override under each of the two engines.

**Every one of the ten pairs came out `maxdiff = 0`.** Reported here for the hardest case, the EEVEE override file, because that is the one the versions disagree about:

```
5.1   ee_vl (through node) vs ref_ee_vl_direct   maxdiff=0 meandiff=0
```

Any difference at all, in any pixel, would have meant the compositor path renders the second scene with something other than the settings it has when rendered on its own. There was none. This is the strongest form of the answer, because it does not depend on knowing which setting did what.

---

**Observed 7 — an engine given on the command line does not reach the scene behind the node.**

Three files in which **both** scenes carry the same engine, so that any difference afterwards comes from the argument rather than from what was stored: `E_bothcy` (both Cycles at 64), `E_bothwb` (both Workbench), `E_bothee` (both EEVEE at 16). Four runs, `Comp` active in each, of the form:

```
"/Applications/Blender 5.1.2.app/Contents/MacOS/Blender" --factory-startup -b E_bothcy.blend --log "render" -E BLENDER_WORKBENCH -o "<output>_" -f 1
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

and nothing was rendered. Blender 5.1.2 exited **1**.

---

**Observed 8 — a sample count given from outside: no argument sets one, one offered anyway is silently ignored, and a script does reach it.**

**What the program says it accepts.** `--help` in Blender 5.1.2 lists, under *Render Options*, exactly `-b/--background`, `-a/--render-anim`, `-S/--scene`, `-f/--render-frame`, `-s/--frame-start`, `-e/--frame-end`, `-j/--frame-jump`, `-o/--render-output`, `-E/--engine`, `-t/--threads`; and under *Cycles Render Options*, exactly `--cycles-device` and `--cycles-print-stats`. **Nothing in either section sets a sample count.** The program's own account of itself therefore agrees with the reading above, which had rested on absence from a page.

Found in the same place, and worth recording because more than one of these documents told whoever ran their tests to look for such an argument rather than guess one: this build carries a `--log <match>` argument and a `--log-level <level>` argument, under a *Logging Options* heading. `--log "render"` is what produced every engine and sample line quoted in this section.

**What the engine names are.** `-E help` in Blender 5.1.2 printed:

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

The operator behind the button beside the View Layer dropdown was found by asking Blender what operators exist rather than guessing a name, which printed `['cryptomatte_layer_add', 'cryptomatte_layer_remove', 'read_viewlayers', 'render_changed']`. In a Blender 5.1.2 started **with a window** from a terminal, one area was turned into a node editor, its tree type set to the compositor, **its node tree set to `Comp`'s own compositing tree**, and the Render Layers node selected and made active. The editor reported itself ready:

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

**No render happened** — no scene line, no engine line and no sample line appeared in the log. So this establishes what that operator returns when it is driven this way, and nothing about what the button does when a person presses it. A press is a mouse click on a window, and nothing available for this round could produce one. **What the button renders with is unsettled in Blender 5.1.**

*(This one was re-run on 2026-08-29 rather than copied, because the record of the original run was a step short of repeatable: it did not say that the editor's own node tree must be assigned before the call. Driven exactly as that record described it, both operators refuse — `poll` comes back False and the call raises "context is incorrect". With the tree assigned, the original result reproduces exactly, as printed above. The re-run agreed with the record in every other particular. One detail the record does not carry, seen in the re-run: `read_viewlayers` reports an attempt to read a cached image file for `Sub` and that it cannot read it, and still no render follows.)*

**What this settles in this document, and what it does not.**

Parts A and B were **inferred**, and both hold. C1 stays as written — these sources name no command-line sample count, and that is a silence in the documentation, not a closed door; *Observed 8* shows the program agreeing about the argument and shows the script route working. C2's *not in the sources* stays as written, and *Observed 7* answers the behaviour beside it: `-E` writes onto the active scene alone and never reaches the scene behind the node. C3's inference — that scene A's engine and sample count do not reach scene B — is what the runs show, most flatly in the leak test, where changing the compositing scene's sample count by a factor of four thousand moved no pixel.

Of the *Not covered* list above, the runs settle the first (what is actually used at render time), the second (`-E` reaches no scene other than the active one) and the fifth (an engine name that does not exist is refused outright, with `Error: engine not found`, and nothing is rendered). Chains of scenes, the Scene strip and third-party engines remain untouched. The one documented operating-system exception this document names — EEVEE headless on Windows — was **not** tested; see the operating-system line above and the one item left on the hand-test list below.

---

### Not covered

- Which engine and sample count are actually used at render time. Everything above is what the settings are and where they live. Nothing in the sources describes the run itself, so the main answer is a joined-up inference, not a documented statement.
- Whether `-E` reaches scenes other than the one being rendered. Completely unaddressed by the sources.
- Whether the second scene is re-rendered at all, or a previous result reused, on repeat renders — outside this question and not settled here.
- What a `-P` script or any other outside script does to either setting. That is the script's business and the sources say nothing.
- Whether an engine name given to `-E` that the second scene's own settings do not use produces any warning, error, or fallback. Not addressed anywhere I looked.
- The behaviour when scene B's engine is a third-party add-on engine rather than EEVEE, Cycles or Workbench.
- Chains: scene B's own compositor pulling in a scene C. Not addressed.
- The Video Sequencer's Scene strip route into another scene. Its manual page says nothing about engine or samples either, but I did not pursue it, since this question sits in the compositor topic.

### Outside the interface

*(**Note added by the hand-test step, 2026-08-29.** The tests this document asked for have been run, and their results are now written in — see *What running Blender showed — the hand tests of 2026-08-29* above. Both standing lines below have been brought up to date with them: each keeps what the sources say and gains, marked separately, what a run showed. The citation check's note that the answers were still owed stood here until this was done.)*

Everything the answer rests on is stored in the blend-file — the node's Scene and View Layer fields, the scene's engine, the scene's sample count, the view layer's sample override, and the Layer Samples policy. Nothing in the sources ties any of them to a window being open, so on the documented facts the answer holds when Blender is driven from outside its own window. **Inferred**, from the fact that these are all scene and view-layer settings (parts A and B) and the manual's own instruction that renders are driven from the command line with the settings saved in the file: "Render the whole animation using all the settings saved in the blend-file" (the manual, https://docs.blender.org/manual/en/5.1/advanced/command_line/render.html, read 2026-07-30).

Three differences I can name:

1. One control on the node is interface-only: "The button next to the dropdown re-renders it immediately" (Render Layers page, cited above). There is no such thing when Blender is driven from outside.
2. The engine input `-E` exists **only** outside the interface — it is a command-line argument. Its effect is therefore a question that only arises there, and, as recorded in C2, the sources do not answer it.
3. If the second scene's engine is EEVEE, the manual records a hard limit that applies precisely to unattended running: "Headless rendering is not supported on headless Windows systems." Source: the manual, https://docs.blender.org/manual/en/5.1/render/eevee/limitations/limitations.html — read 2026-07-30. **Documented.**

Beyond those three, the sources do not say where inside and outside differ.

**Observed, 2026-08-29.** This line can now be answered for Blender 5.1 rather than left open, on the one operating system it was answered on.

- **A render started from inside a running Blender with a window is the same picture, to the last bit, as one started from the command line.** Blender 5.1.2 was started **without** `-b` — so with a real window and a full interface loaded — and handed a script that made `Comp` the current scene, rendered it and quit. The script printed `GUI_BACKGROUND_FLAG: False`, so Blender really was not in background mode. The written image was compared with the one the same file wrote from a background run: `maxdiff = 0`, and both matched the 512-sample reference at `maxdiff = 0` while differing from the 1-sample reference by 17.5005. **So the engine and the sample count that the scene behind the node is rendered with are the same whether Blender has an interface or not.**
- **The limit on that, stated plainly.** The render was started by a script running inside that windowed Blender, not by a person pressing a key. What it shows is that the render path is the same when Blender has a window and a full interface loaded. It does not show what a keypress does, and nothing in this round could.
- **The one interface-only control this answer names is still unsettled.** The re-render button beside the node's View Layer dropdown was not settled: the operator behind it, driven from a script with the node editor in the right state, cancelled without rendering (*Observed 10* above). A press is a mouse click on a window, and there was no machine in this round that could produce one.
- **Everything else above was driven from outside in the first place.** Every run in the observed section other than that one was a background run started from a terminal, which is the case this project cares about.

### Operating system

The sources state nothing operating-system-dependent about which engine or which sample count a scene is rendered with. The engine setting, the sample settings, the view-layer override and the Layer Samples policy are described without any OS condition on all the pages quoted above.

Two OS-specific facts I did find, both about the surroundings rather than about the choice itself, both **documented**:

- EEVEE: "Headless rendering is not supported on headless Windows systems." (the manual, https://docs.blender.org/manual/en/5.1/render/eevee/limitations/limitations.html — read 2026-07-30). So if the second scene's engine is EEVEE, a headless Windows machine is a documented exception to the render happening at all.
- Cycles' device list is plainly platform-shaped — "Valid options are: CPU CUDA OPTIX HIP ONEAPI METAL" (the manual, https://docs.blender.org/manual/en/5.1/advanced/command_line/arguments.html — read 2026-07-30) — but that is the device, not the engine and not the sample count, and I make no claim from it about either.

Whether the choice of engine or sample count for a second scene behaves the same on every operating system: the sources do not say.

**Observed, 2026-08-29 — and it settles one operating system and leaves the others open.** Every run in the observed section above was made on **one** machine: the Creator's MacBook Pro, Apple M1 Pro, Apple's own graphics and no second card, macOS 26.6.2 (build 25G83), arm64. Nothing was run on Linux and nothing on Windows; no machine was rented for this round. **Untried rather than untestable: a Windows machine can be rented, and every one this project has read reported a Server edition — see the equipment section of `Preparation/Blender Research/Method.md`.** **So whether any of the observed findings hold on any other operating system is still unknown**, and saying so is the whole of what this line can carry. A test settles the operating system it was run on.

Four places where that gap is worth naming rather than left implicit:

- **The one difference between the four researched versions** — that 4.5 does not honour a view layer's sample override under EEVEE where 5.0, 5.1 and 5.2 do (*Observed 5* above) — was found on this Mac, whose graphics are Apple's own. It is worth repeating on a machine with a different graphics card before it is treated as settled everywhere.
- **The windowed run** behind the *Outside the interface* line above was made on a machine whose viewport is drawn by a real graphics card. It could come out differently on a machine whose graphics are drawn by the processor.
- **No third-party render engine was involved anywhere.** Every run used `--factory-startup`, so no add-on engine was loaded, and none is among this project's equipment.
- **The one documented operating-system exception named above was not tested.** The manual's line that headless rendering is not supported on headless Windows systems still stands untested: this project's one Windows machine was rented and destroyed on 2026-08-26, and nothing was rented for this round.

---

## Record of where I looked

Everything below is inside `Blender Sources/`. Searches were run over whole folders, not single files.

**Folders searched in full:** `manual_5.1`, `api_5.1` (1,899 files), `release_notes_5.1`, `cycles_5.1`.

**Terms searched, and what came back**

| Term | Where | Result |
| --- | --- | --- |
| `render layers` (case-insensitive) | whole manual | 20 files; the ones that mattered were `compositing/types/input/scene/render_layers.txt`, `render/layers/introduction.txt`, `interface/controls/nodes/node_editors.txt` |
| `another scene` | whole manual | 4 hits: the Render Layers page, the VSE Scene strip page (twice), and a markers page. No third mechanism exists in the manual's wording |
| `engine` | `compositing/` only | 2 hits, neither about which engine a second scene uses ("To make the render engine actually use the compositing node tree…" and "For other engines, the passes will be empty") |
| `sample` / `samples` | `compositing/` only | 20 hits, all about node-local sample counts (Mask node, Chromatic Aberration, Plane Track Deform) or colour sampling. Nothing about render sampling |
| `--engine` | manual, API, release notes, Cycles files | 1 hit, the command-line arguments page. Nothing anywhere else |
| `-E help`, `-E CYCLES` | whole manual | 2 hits, both on the command-line render page |
| `samples`, `sample` | `advanced/command_line/arguments.txt` and `advanced/command_line/render.txt` | no hits at all — this is the basis for saying no sample count can be given from outside |
| `argv`, `argparse`, `add_argument` | whole of `cycles_5.1` | 6 hits, all in `engine.py` lines 9–32, defining exactly two arguments |
| `Render Engine` | whole manual | 5 hits: two viewport-shading pages, the Cycles introduction, the Workbench introduction, one shader-node page. All say the engine is chosen in Render properties; none discusses a second scene |
| `render settings`, `per scene`, `per-scene`, `each scene` | whole manual | 30 hits; the useful ones are the Scenes introduction, the Rendering introduction, the Workbench sampling page and the Compositor editor page ("Each Scene can have its own compositor node tree") |
| `sample count` | whole manual | 14 hits; the useful ones are the Cycles sampling page, the EEVEE depth of field page and the EEVEE motion blur page |
| `layer samples` | whole manual | 2 hits: the Cycles sampling page and the View Layer override page |
| `active scene` | whole manual | 20 hits; only `-S, --scene` on the arguments page is about rendering scope, and it is about which scene is rendered |
| `all scenes` | manual and API | 6 hits, all about deleting or listing data-blocks. Nothing about an argument applying to all scenes |
| `propagat` | whole manual | 10 hits, all about node warning propagation, sculpting and cloth. Nothing about settings propagating between scenes |
| `override` next to `engine` | manual, API, Cycles files | only `RenderEngine.camera_override` and `RenderEngine.layer_override` in the API, which are a camera and a layer, not an engine or a sample count |
| `environment variable` | whole manual | 19 hits; the arguments page's list and the OpenColorIO and CUDA pages. None concerns engine or samples |
| `re-render`, `rerender` | whole API | 2 hits, both unrelated (`use_persistent_data`, and the render operator's `layer` parameter) |
| `engine`, `samples`, `command line` | whole of release notes 5.1 | 2 hits in all of the 5.1 release notes, neither relevant (a glTF note about game engines, and `--python-use-user-env`). The 5.1 release notes say nothing about this question |

**Pages I opened and read in full**

- `compositing/types/input/scene/render_layers.txt` → https://docs.blender.org/manual/en/5.1/compositing/types/input/scene/render_layers.html
- `compositing/compositor_system.txt` → https://docs.blender.org/manual/en/5.1/compositing/compositor_system.html
- `compositing/introduction.txt` → https://docs.blender.org/manual/en/5.1/compositing/introduction.html
- `editors/compositor.txt` (header section) → https://docs.blender.org/manual/en/5.1/editors/compositor.html
- `advanced/command_line/arguments.txt` (render, format, Python, environment sections) → https://docs.blender.org/manual/en/5.1/advanced/command_line/arguments.html
- `advanced/command_line/render.txt` → https://docs.blender.org/manual/en/5.1/advanced/command_line/render.html
- `advanced/command_line/index.txt` → https://docs.blender.org/manual/en/5.1/advanced/command_line/index.html
- `render/introduction.txt` → https://docs.blender.org/manual/en/5.1/render/introduction.html
- `render/cycles/render_settings/sampling.txt` → https://docs.blender.org/manual/en/5.1/render/cycles/render_settings/sampling.html
- `render/eevee/render_settings/sampling.txt` → https://docs.blender.org/manual/en/5.1/render/eevee/render_settings/sampling.html
- `render/eevee/render_settings/depth_of_field.txt` → https://docs.blender.org/manual/en/5.1/render/eevee/render_settings/depth_of_field.html
- `render/eevee/render_settings/motion_blur.txt` → https://docs.blender.org/manual/en/5.1/render/eevee/render_settings/motion_blur.html
- `render/eevee/limitations/limitations.txt` → https://docs.blender.org/manual/en/5.1/render/eevee/limitations/limitations.html
- `render/workbench/sampling.txt` → https://docs.blender.org/manual/en/5.1/render/workbench/sampling.html
- `render/layers/introduction.txt` → https://docs.blender.org/manual/en/5.1/render/layers/introduction.html
- `render/layers/override.txt` → https://docs.blender.org/manual/en/5.1/render/layers/override.html
- `scene_layout/scene/introduction.txt` → https://docs.blender.org/manual/en/5.1/scene_layout/scene/introduction.html
- `scene_layout/scene/properties.txt` → https://docs.blender.org/manual/en/5.1/scene_layout/scene/properties.html
- `video_editing/edit/montage/strips/scene.txt` → https://docs.blender.org/manual/en/5.1/video_editing/edit/montage/strips/scene.html
- API: `bpy.types.CompositorNodeRLayers.txt` → https://docs.blender.org/api/5.1/bpy.types.CompositorNodeRLayers.html
- API: `bpy.types.RenderSettings.txt` (engine section) → https://docs.blender.org/api/5.1/bpy.types.RenderSettings.html
- API: `bpy.types.Scene.txt` (render, eevee, cycles sections) → https://docs.blender.org/api/5.1/bpy.types.Scene.html
- API: `bpy.types.ViewLayer.txt` (samples section) → https://docs.blender.org/api/5.1/bpy.types.ViewLayer.html
- API: `bpy.types.SceneEEVEE.txt` (sample sections) → https://docs.blender.org/api/5.1/bpy.types.SceneEEVEE.html
- API: `bpy.ops.render.txt` (render operator) → https://docs.blender.org/api/5.1/bpy.ops.render.html
- API: `bpy.types.RenderEngine.txt` → https://docs.blender.org/api/5.1/bpy.types.RenderEngine.html
- Release notes: `compositor.md` → https://developer.blender.org/docs/release_notes/5.1/compositor/ ; `cycles.md` → https://developer.blender.org/docs/release_notes/5.1/cycles/ ; `rendering.md` → https://developer.blender.org/docs/release_notes/5.1/rendering/ ; `core.md` → https://developer.blender.org/docs/release_notes/5.1/core/
- Cycles' own files: `Blender Sources/cycles_5.1/engine.py` (lines 1–80) and `Blender Sources/cycles_5.1/properties.py` (lines 64–68, 378, 470–560, 1098–1112, 1479–1556, 1961–1994)

**Picture opened**

- `manual_5.1/_images/node-types_CompositorNodeRLayers.webp`, the screenshot on the Render Layers page. It shows the node with Image and Alpha outputs and two fields, Scene and ViewLayer, and no engine or sample control. Read 2026-07-30.

**What I ruled out, and why**

- **The Video Sequencer Scene strip page.** It is the file's other way of taking in another scene's render, so I read it in full to be sure it did not carry the answer. It describes strip length, input type, camera override, transparency and volume, and ends with "Scene strips do not render individual Render Passes; only the Combined render pass will be used." Not one word about engine or samples. Ruled out.
- **Background Scene** (Scene properties). It reads "Allows you to use a scene as a background", and the scene it names is included into the current scene rather than rendered separately, so it is not "taking in another scene's rendered result". Ruled out as a different mechanism.
- **`--cycles-device`.** This is the one documented case of a command-line value reaching into scene settings — its own help text in `Blender Sources/cycles_5.1/engine.py` says "overriding user preferences and the scene setting". But it sets the **device**, not the engine and not the sample count, so I have not carried anything from it into the answer. Ruled out as the wrong setting.
- **`RenderEngine.camera_override` and `RenderEngine.layer_override`** in the API. Named "override", but one is an Object and one is a 20-element array of booleans, i.e. a camera and a layer mask. Neither is an engine or a sample count. Ruled out.
- **The Cycles source tree in `Blender Sources/cycles_5.1/source`.** It contains only `kernel` and `util` — the rendering kernel and helper code. There is no Blender-session or scene-synchronisation code in it, so it cannot show how a scene's engine or sample count is picked up. Ruled out as not containing the relevant material.
- **All twenty-one 5.1 release-note files.** Searched as a whole for engine, samples and command line; two irrelevant hits. Ruled out.

**Did I need anything I was not given?** No. I did not use the web, and did not need to.

---

## Things a person with Blender 5.1 open could settle by hand in a few minutes

*(**Run, and pruned — 2026-08-29.** Every test this list held was carried out on the Creator's MacBook Pro on 2026-08-29, and repeated on the same machine the same day. The answers are written into *What running Blender showed — the hand tests of 2026-08-29* above, which is where they now live; the answered items have therefore been taken off this list rather than left standing as work for somebody to do. What was set up, what was typed and what the machine printed is recorded in full in `Audits/Blender Research/The compositor/Hand tests — Architecture Fact 135.md`. This list's items were, in that record's numbering, entries 6 (item 1), 7 (item 2), 14 (item 3), 3 and 4 (item 4) and 10 (item 5).)*

**Everything else on this list has been answered.** Four things were not run at all, and they are recorded in that same document: a real press of the re-render button on the Render Layers node — the operator behind it was driven and cancelled without rendering, see *Observed 10* above; anything on Windows; everything above on a second operating system; and any third-party render engine, since every run used `--factory-startup`.

**One item stays on this list, because it could not be run.**

6. **What happens with EEVEE headless on Windows?** On a Windows machine with no display attached, run a background render of a file whose second scene uses EEVEE, and record the exact error or output. This checks the one documented operating-system exception.

   *Still owed as of 2026-08-29.* This project's one Windows machine was rented on 2026-08-26 and destroyed the same day, and nothing was rented for this round. Every run recorded above was on macOS on Apple silicon.

---

2026-08-29 — Claude Opus 5 (Claude Code) — The compositor, Blender 5.1

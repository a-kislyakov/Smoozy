# The compositor — Blender 5.0 — Question 9

**Sources consulted:** the Blender 5.0 manual, the Blender 5.0 Python API documentation, the Blender 5.0 release notes, and the Cycles add-on's own files, all as supplied in `Blender Sources/` and all taken on **2026-07-30**. That snapshot date is my date read for everything quoted below.

I read only the question sheet and files inside Blender Sources/. I did not open any other part of this project.

---

## Question 9

> When one scene takes in another scene's rendered result, what decides the render engine and the sample count that the second scene is rendered with, and what effect on that does a value for either, given to the render from outside, have?

### The reading I answered, and the other reading

The question is ambiguous about *how* one scene takes in another's rendered result. Two things in Blender do this:

- a **Render Layers node** in scene A's compositor with its Scene set to scene B, and
- a **Scene strip** in scene A's video sequencer pointing at scene B.

Because the topic is the compositor, I answered the first reading. The other reading is the Scene strip; I note what I found about it at the end of the record of looking, but I have not answered it.

I also ruled out a third thing that looks similar and is not: **Background Scene** (Scene Properties). That pulls another scene's *contents* into this scene's render, not its rendered result, so there is only one render and one set of render settings involved. (This is a note about what I chose to answer, not a sourced finding. I read the page it comes from — listed in the record of looking below — but quote no words from it, so nothing here rests on it.)

---

## Answer

The answer has three parts. They carry different labels, so I give them separately.

### Part A — what decides the render engine for the second scene

**LABEL: inferred.**

The sources never state, in words, which scene's engine setting is used when a Render Layers node points at a different scene. What they do state is that the engine is a setting held by each scene individually, and that this node renders a view layer belonging to the scene named on the node. Putting those together, the engine used for the second scene is the engine set in **that second scene's own render settings**.

The step a reader has to take, written out plainly: the sources say (1) the node's Scene property picks "the scene for which to render a view layer", and (2) `engine` is a property of `RenderSettings`, and `RenderSettings` is reached through `Scene.render`, so every scene carries its own engine choice. Nothing in the sources describes any mechanism that substitutes one scene's engine for another's. From (1) and (2) I conclude that rendering scene B's view layer uses scene B's engine. That last move — from "each scene has its own engine setting" to "scene B's setting is the one that governs when scene A asks for the render" — is mine, not the source's. It is not documented.

Supporting evidence, quoted:

- "Scene
  
  The scene for which to render a view layer.
  
  View Layer
  
  The view layer to render. The button next to the dropdown re-renders it immediately."
  — Manual, Render Layers Node, https://docs.blender.org/manual/en/5.0/compositing/types/input/scene/render_layers.html — read 2026-07-30

- "engine
  
  Engine to use for rendering
  
  Type:
  
  enum in ['BLENDER_EEVEE'], default 'BLENDER_EEVEE'"
  — Python API, `bpy.types.RenderSettings`, https://docs.blender.org/api/5.0/bpy.types.RenderSettings.html — read 2026-07-30

- "render
  
  Type:
  
  RenderSettings, (readonly, never None)"
  — Python API, `bpy.types.Scene`, https://docs.blender.org/api/5.0/bpy.types.Scene.html — read 2026-07-30

- "Render the 3D Viewport using the scene's Render Engine, for interactive rendering."
  — Manual, Viewport Shading, https://docs.blender.org/manual/en/5.0/editors/3dview/display/shading.html — read 2026-07-30
  (This is about the viewport, not about compositing. I quote it only because it is the one place in the manual that says in words that the render engine belongs to *the scene*.)

- **documented, from a picture.** Nothing on the face of the Render Layers node is an engine control. The picture `_images/node-types_CompositorNodeRLayers.webp`, on the Render Layers Node page, https://docs.blender.org/manual/en/5.0/compositing/types/input/scene/render_layers.html — read 2026-07-30 — shows a header reading "Render Layers", two outputs "Image" and "Alpha", a data-block field holding "Scene", and a dropdown holding "ViewLayer". No engine field and no samples field is visible on it.

  Two limits on that, both mine. What is visible is visible; that the node has no *hidden* engine setting is not something a picture can show. And I take the picture to show the whole node rather than part of it, which the picture cannot tell me either. The API page for `CompositorNodeRLayers` — https://docs.blender.org/api/5.0/bpy.types.CompositorNodeRLayers.html, read 2026-07-30 — points the same way, in that the only properties I found listed as the node's own are `layer` and `scene`; but that is me reading the shape of a listing, with no sentence to quote, so it is a second look rather than a second source.

### Part B — what decides the sample count for the second scene

**LABEL: inferred**, resting on **observed** and **documented** pieces.

Same shape of answer. The sample count is a per-scene setting, plus a per-view-layer override with a per-scene policy governing it. So what decides the sample count for the second scene is that second scene's own sampling setting, as possibly overridden by the particular view layer the node names. The inference step is the same one as in Part A: the sources establish where these settings live, and that the node renders a named scene's named view layer, but never say outright that the named scene's settings are the ones consulted.

The pieces:

**Cycles — the scene-level count. Observed**, from the Cycles add-on's own file.

```
    samples: IntProperty(
        name="Samples",
        description="Number of samples to render for each pixel",
        min=1, max=(1 << 24),
        default=4096,
    )
```
— `Blender Sources/cycles_5.0/properties.py`, lines 479–484, inside `class CyclesRenderSettings(bpy.types.PropertyGroup)` which begins at line 378. Read 2026-07-30.

That property group is attached to the scene:

```
    @classmethod
    def register(cls):
        bpy.types.Scene.cycles = PointerProperty(
            name="Cycles Render Settings",
            description="Cycles render settings",
            type=cls,
        )
```
— `Blender Sources/cycles_5.0/properties.py`, lines 1101–1107. Read 2026-07-30. *(Line numbers corrected by the citation check, 2026-08-29; the document said 1102–1109. The quoted block runs from `@classmethod` at 1101 to the closing bracket at 1107. The words are unchanged.)*

And the Cycles user interface reads it off the scene:

```
        scene = context.scene
        cscene = scene.cycles
...
            col.prop(cscene, "samples", text="Max Samples")
```
— `Blender Sources/cycles_5.0/ui.py`, lines 298–299 and 313, inside `class CYCLES_RENDER_PT_sampling_render` which begins at line 288. Read 2026-07-30.

**EEVEE — the scene-level count. Documented.**

- "taa_render_samples
  
  Number of samples per pixel for rendering
  
  Type:
  
  int in [1, inf], default 64"
  — Python API, `bpy.types.SceneEEVEE`, https://docs.blender.org/api/5.0/bpy.types.SceneEEVEE.html — read 2026-07-30

- "eevee
  
  EEVEE settings for the scene"
  — Python API, `bpy.types.Scene`, https://docs.blender.org/api/5.0/bpy.types.Scene.html — read 2026-07-30

**The view-layer override. Documented.** This matters here because the Render Layers node names a view layer, not just a scene.

- "samples
  
  Override number of render samples for this view layer, 0 will use the scene setting
  
  Type:
  
  int in [0, inf], default 0"
  — Python API, `bpy.types.ViewLayer`, https://docs.blender.org/api/5.0/bpy.types.ViewLayer.html — read 2026-07-30

- "Samples
  
  Allows the view layer to use a different number of samples than the main scene.
  This is controlled by the Layer Samples
  when using the Cycles renderer. When using EEVEE, the Samples override is always used."
  — Manual, View Layer Override, https://docs.blender.org/manual/en/5.0/render/layers/override.html — read 2026-07-30

- "Layer Samples
  
  If any view layers have Sample Overrides configured,
  this option specifies how to use them.
  
  Use:
  
  Allow view layers to override the scene-level sample count.
  
  Bounded:
  
  Allow the overrides as long as they don't exceed the scene-level sample count.
  
  Ignore:
  
  Ignore the overrides."
  — Manual, Cycles Sampling, https://docs.blender.org/manual/en/5.0/render/cycles/render_settings/sampling.html — read 2026-07-30

  The matching setting in Cycles' own file, **observed**:
```
    use_layer_samples: EnumProperty(
        name="Layer Samples",
        description="How to use per view layer sample settings",
        items=enum_use_layer_samples,
        default='USE',
    )
```
  — `Blender Sources/cycles_5.0/properties.py`, lines 550–555; the choices it points at are at lines 64–67:
```
enum_use_layer_samples = (
    ('USE', "Use", "Per render layer number of samples override scene samples"),
    ('BOUNDED', "Bounded", "Bound per render layer number of samples by global samples"),
    ('IGNORE', "Ignore", "Ignore per render layer number of samples"),
```
  Read 2026-07-30. Note that `use_layer_samples` sits in `CyclesRenderSettings`, which is attached to the **scene** — so the policy that decides whether a view layer's override counts is itself a per-scene setting.

So the chain for the second scene, inferred: that scene's own Cycles or EEVEE render sample count, unless the view layer the node names carries its own samples override, in which case — under Cycles — that scene's own Layer Samples policy decides whether the override is used, bounded by the scene count, or ignored; under EEVEE the override is always used. Every setting in that chain belongs to the second scene or to a view layer inside it.

### Part C — what a value given to the render from outside does to any of that

**LABEL for the engine half: documented that an outside value exists; not in the sources what it does to the second scene.**
**LABEL for the sample-count half: not in the sources** — nothing I read hands a sample count to a render from outside, so nothing I read says what such a value would do to the second scene.

**The engine can be given from outside.** The command line takes an engine:

- "-E, --engine <engine>
  
  Specify the render engine.
  Use -E help to list available engines."
  — Manual, Command Line Arguments, https://docs.blender.org/manual/en/5.0/advanced/command_line/arguments.html — read 2026-07-30

- "-E CYCLES
  
  Use the "Cycles Render" engine.
  For a list of available render engines, run blender -E help."
  — Manual, Rendering From The Command Line, https://docs.blender.org/manual/en/5.0/advanced/command_line/render.html — read 2026-07-30

What the sources do **not** say is which scene or scenes that value lands on. Neither page says whether `-E` sets the engine on the active scene only, on every scene in the file, or on whatever scene a render happens to touch. The neighbouring argument makes the ambiguity sharper rather than settling it:

- "-S, --scene <name>
  
  Set the active scene <name> for rendering."
  — same Command Line Arguments page, read 2026-07-30

That names one scene as active. Whether `-E` then applies to that one scene or to all of them is not stated anywhere I could find. So: **whether an engine given from outside changes what the second scene is rendered with is not in the sources.**

The one thing that is stated about ordering, and which applies to `-E` as much as to anything else:

- "Arguments are executed in the order they are given."
  — same page, read 2026-07-30. And, from the rendering page: "Always position -f or -a as the last arguments." — https://docs.blender.org/manual/en/5.0/advanced/command_line/render.html, read 2026-07-30.

  **Inferred**, and the step is mine: neither quoted line mentions `-E`. Putting them together — arguments run in the order given, and the render argument goes last — an engine given after `-f` or `-a` would arrive after the render had started, so it must precede them. The sources do not say that about `-E`.

**No way of giving the sample count from outside is described in the sources.** I found no command-line argument and no environment variable for it, in Blender's own arguments or in Cycles'.

- The full argument list on the Command Line Arguments page contains no sampling option. Under "Render Options" it has only `-b`, `-a`, `-S`, `-f`, `-s`, `-e`, `-j`, `-o`, `-E`, `-t`. Under "Cycles Render Options" it has only `--cycles-device` and `--cycles-print-stats`. Its "Environment Variables" section lists directory, splash and colour-management variables (`BLENDER_USER_*`, `BLENDER_SYSTEM_*`, `BLENDER_CUSTOM_SPLASH*`, `BLENDER_OCIO`, `TEMP`, `TMPDIR`) and nothing about sampling. — https://docs.blender.org/manual/en/5.0/advanced/command_line/arguments.html, read 2026-07-30

- **Observed**, from Cycles' own file: Cycles defines exactly two of its own command-line options, and neither is samples.
```
    parser.add_argument("--cycles-print-stats",
                        help="Print rendering statistics to stderr",
                        action='store_true')
    parser.add_argument("--cycles-device",
                        help="Set the device to use for Cycles, overriding user preferences and the scene setting."
                             "Valid options are 'CPU', 'CUDA', 'OPTIX', 'HIP', 'ONEAPI', or 'METAL'."
                             "Additionally, you can append '+CPU' to any GPU type for hybrid rendering.",
                        default=None)
```
  — `Blender Sources/cycles_5.0/engine.py`, lines 13–20, in `_configure_argument_parser()`. Read 2026-07-30.
  Worth noting the wording of the device one: it says in so many words that it overrides "the scene setting". Nothing equivalent is written for samples, because no samples option exists here at all.

So the sources describe no sample count handed to the render from outside, and therefore describe no effect of one on the second scene. **That is a silence, not a denial.** The pages and files I searched are named in the record of looking and go no further than that; absence from them is not a statement by Blender that no such value exists. I am not claiming a sample count cannot be given from outside — only that nothing I read says it can, or says what it would do.

One route the sources do describe. The manual documents `-P, --python <filepath>`, `--python-text <name>` and `--python-expr <expression>` ("Run the given expression as a Python script") on the same Command Line Arguments page, and the settings in Part B are ordinary properties on a scene. So a value can reach a scene's sample count from outside by that door. But what a script does is whatever its author wrote; the sources say nothing about scenes reached through a compositor node in particular, so this is not an answer to the question so much as an acknowledgement that the door exists.

---

## What running Blender showed — the hand tests of 2026-08-29

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

Parts A and B were both **inferred**, and both inferences hold: the engine is the second scene's own, and so is the sample count, with the view layer's override reaching through the node under that same scene's Layer Samples policy. Those labels stay as they are — they describe what the 5.0 sources support — and the observed answers sit beside them.

Part C's two labels stay too. The sources still do not say which scene `-E` lands on, and they still describe no outside value for samples. *Observed 7* and *Observed 8* answer the behaviour without touching that: `-E` writes onto the active scene alone and never reaches the scene behind the node, and there is no sampling argument to give, though a script reaches the setting.

Of the *Not covered* list above, the runs settle the first (`-E` reaches only the active scene), the second (the compositing scene's settings reach the second scene in no respect that was measurable — a four-thousand-fold change in its sample count moved no pixel), and the seventh (the Layer Samples policy consulted is the pulled-in scene's, in both directions). The Scene strip, engine availability, resolution and colour management are untouched, as is what happens when `-E` names an engine that is not available — though an engine name that does not exist at all was tried, and Blender refuses it outright.

---

## Not covered

- Whether `-E` reaches only the active scene or every scene in the file. This is the crux of the second half of the question and the sources are silent on it.
- Whether the second scene's render is affected by the *first* scene's settings in any respect at all. I found no source that discusses a cross-scene render's settings, in either direction.
- Whether the second scene is rendered at all in every situation, and when. That is a different question and I have not gone near it.
- Anything about the Scene strip in the video sequencer. Different mechanism, not answered here.
- What happens if the second scene's engine is one Blender cannot provide (a missing add-on engine), or if `-E` names an engine that is not available.
- Resolution, frame range, colour management and every other render setting. The question asked about engine and samples only.
- Whether the second scene's *own* Layer Samples policy or the first scene's applies, in the case where both scenes have differing policies. I inferred the second scene's, because the setting sits on `Scene.cycles` and the scene being rendered is the second one — but the sources do not confirm it.
- Third-party engines. `bl_idname`/`bl_label` are per-engine class properties; nothing in the sources says how a third-party engine handles a render requested from another scene's compositor.

---

## Outside the interface

*(**Note added by the hand-test step, 2026-08-29.** The tests this document asked for have been run, and their results are now written in — see *What running Blender showed — the hand tests of 2026-08-29* above. Both standing lines below have been brought up to date with them: each keeps what the sources say and gains, marked separately, what a run showed. The citation check's note that the answers were still owed stood here until this was done.)*

Does this hold when Blender is driven from outside its own window?

Partly, and the part that does not is the important part.

- The settings themselves — engine on `Scene.render`, samples on `Scene.cycles` / `Scene.eevee`, override on `ViewLayer.samples` — are stored in the file and reachable through the Python API. Nothing in the sources ties any of them to the user interface. So Parts A and B hold as stated whether Blender is driven by a person or by a command line. **Inferred**, the step being that a property listed on a data-block in the API documentation is a stored value, not a display state.
- The Render Layers node's re-render button — "The button next to the dropdown re-renders it immediately" — is a button, so it belongs to the interface. From outside, the equivalent is the render operator, which is documented to take both a scene and a layer: "scene (string, (optional, never None)) – Scene, Scene to render, current scene if not specified" and "layer (string, (optional, never None)) – Render Layer, Single render layer to re-render (used only when animation is disabled)" — Python API, `bpy.ops.render.render`, https://docs.blender.org/api/5.0/bpy.ops.render.html, read 2026-07-30. **Documented** that the operator exists and takes those arguments; **not in the sources** whether it behaves identically to the button.
- Part C is specifically about driving Blender from outside, and its answer is that the sources do not say what `-E` does to a second scene, and that these sources describe no outside value for samples. That gap is a gap in exactly the case that matters here. *(Narrowed by the citation check, 2026-08-29: the line read "and that no outside value for samples exists", which is a statement about Blender. Part C's own label is "not in the sources", and the quotes under it carry only that the sources describe none.)*

**Observed, 2026-08-29.** This line can now be answered for Blender 5.0 rather than left open, on the one operating system it was answered on.

- **A render started from inside a running Blender with a window is the same picture, to the last bit, as one started from the command line.** Blender 5.0.1 was started **without** `-b` — so with a real window and a full interface loaded — and handed a script that made `Comp` the current scene, rendered it and quit. The script printed `GUI_BACKGROUND_FLAG: False`, so Blender really was not in background mode. The written image was compared with the one the same file wrote from a background run: `maxdiff = 0`, and both matched the 512-sample reference at `maxdiff = 0` while differing from the 1-sample reference by 17.5015. **So the engine and the sample count that the scene behind the node is rendered with are the same whether Blender has an interface or not.**
- **The limit on that, stated plainly.** The render was started by a script running inside that windowed Blender, not by a person pressing a key. What it shows is that the render path is the same when Blender has a window and a full interface loaded. It does not show what a keypress does, and nothing in this round could.
- **The one interface-only control this answer names is still unsettled.** The re-render button beside the node's View Layer dropdown was not settled: the operator behind it, driven from a script with the node editor in the right state, cancelled without rendering (*Observed 10* above). A press is a mouse click on a window, and there was no machine in this round that could produce one.
- **Everything else above was driven from outside in the first place.** Every run in the observed section other than that one was a background run started from a terminal, which is the case this project cares about.

## Operating system

The sources say nothing that makes any of this differ by operating system.

- The command-line argument list is presented as one list, not split by system. The only operating-system-specific items on that page are the temporary-directory environment variables — "TEMP: Store temporary files here (MS-Windows)." — which have nothing to do with engine or samples. Read 2026-07-30.
- `--cycles-device` names METAL, CUDA, OPTIX, HIP and ONEAPI, which are tied to particular hardware and systems, but that is the device, not the engine or the sample count.
- I found no statement anywhere in the manual, the API documentation, the release notes or Cycles' own files that the engine setting, the sample count, the view-layer samples override, or the Layer Samples policy behaves differently on one system than another. **Not in the sources** that any difference exists; I am not asserting that none does.

**Observed, 2026-08-29 — and it settles one operating system and leaves the others open.** Every run in the observed section above was made on **one** machine: the Creator's MacBook Pro, Apple M1 Pro, Apple's own graphics and no second card, macOS 26.6.2 (build 25G83), arm64. Nothing was run on Linux and nothing on Windows; no machine was rented for this round. **Untried rather than untestable: a Windows machine can be rented, and every one this project has read reported a Server edition — see the equipment section of `Preparation/Blender Research/Method.md`.** **So whether any of the observed findings hold on any other operating system is still unknown**, and saying so is the whole of what this line can carry. A test settles the operating system it was run on.

Three places where that gap is worth naming rather than left implicit:

- **The one difference between the four researched versions** — that 4.5 does not honour a view layer's sample override under EEVEE where 5.0, 5.1 and 5.2 do (*Observed 5* above) — was found on this Mac, whose graphics are Apple's own. It is worth repeating on a machine with a different graphics card before it is treated as settled everywhere.
- **The windowed run** behind the *Outside the interface* line above was made on a machine whose viewport is drawn by a real graphics card. It could come out differently on a machine whose graphics are drawn by the processor.
- **No third-party render engine was involved anywhere.** Every run used `--factory-startup`, so no add-on engine was loaded, and none is among this project's equipment.

---

## Record of where I looked

**Files opened in full or in part**

Manual (`Blender Sources/manual_5.0`):
- `compositing/types/input/scene/render_layers.txt`, `scene_time.txt`, `time_curve.txt`, `index.txt`
- `compositing/compositor_system.txt`, `compositing/introduction.txt`, `compositing/limitations.txt`
- `compositing/types/filter/blur/defocus.txt` (it has a Scene selector too — "To select the linked scene." — but it selects a scene to take camera and Z-buffer settings from, not to render; ruled out)
- `advanced/command_line/arguments.txt` (whole file, including Render Options, Cycles Render Options, Argument Order and Environment Variables)
- `advanced/command_line/render.txt`, `advanced/command_line/launch/index.txt`
- `render/introduction.txt`, `render/layers/introduction.txt`, `render/layers/override.txt`
- `render/cycles/render_settings/sampling.txt`, `render/eevee/render_settings/sampling.txt`
- `scene_layout/scene/properties.txt` (Background Scene — ruled out, see the reading note)
- `video_editing/edit/montage/strips/scene.txt` (Scene strip — the other reading; see below)

Picture: `_images/node-types_CompositorNodeRLayers.webp`.

Python API (`Blender Sources/api_5.0`):
- `bpy.types.CompositorNodeRLayers.txt`, `bpy.types.RenderSettings.txt`, `bpy.types.Scene.txt`, `bpy.types.SceneEEVEE.txt`, `bpy.types.ViewLayer.txt`, `bpy.types.RenderEngine.txt`, `bpy.ops.render.txt`

Release notes (`Blender Sources/release_notes_5.0`):
- `compositor.md`, `core.md`, `python_api.md`, `eevee.md`, `bugfixes.md` (searched; the first three read in the relevant parts)

Cycles' own files (`Blender Sources/cycles_5.0`):
- `properties.py` (sampling properties, `use_layer_samples`, `CyclesRenderLayerSettings`, the `register` methods that attach these groups to `Scene` and `ViewLayer`)
- `engine.py` (its command-line argument parser)
- `__init__.py` (the `CyclesRender` engine class)
- `ui.py` (the sampling panel)
- `source/` contains only `kernel` and `util`. The Blender-side synchronisation code that would actually read a scene's settings is **not** in this folder, so it could not be consulted. This is why Parts A and B could not be raised from inferred to observed.

**Searches run, across the manual, the API documentation and the release notes unless noted**

`--engine`, `-E`, `Render Layers`, `another scene`, `other scene`, `each scene`, `per-scene`, `per scene`, `Render Engine`, `referenced scene`, `scene being rendered`, `its own render settings`, `own render engine`, `scene's render settings`, `scene's engine`, `cross-scene`, `linked scene`, `rendered with`, `which engine`, `engine used`, `uses the engine`, `re-render`, `renders it`, `override` (then narrowed to lines also containing engine, sample or scene), `Background Scene`, `Set Scene`, `sample` (within `compositing/` alone, and within the command-line pages alone), `engine` (within `compositing/` alone), `compositing_node_group`, `scene node`, `rlayers`. In Cycles' files: `samples`, `add_argument`, `argparse`, `sys.argv`, `parse_known_args`, `getenv`, `environ`, `bl_idname`, `bl_use`, `PointerProperty`.

**What those searches ruled out, and why**

- The compositing section of the manual mentions an engine exactly twice: once about the Compositing Pipeline option, once saying the viewport compositor only supports passes under EEVEE. Neither concerns which engine renders a scene named on a node. The word "sample" inside `compositing/` occurs only in node settings (mask motion-blur samples, chromatic-aberration samples, plane-track samples) — none is a render sample count. So the compositing section does not answer this.
- The Render Layers node's own documentation page names only two properties, Scene and View Layer, and its hint concerns loading a *previously saved* multi-layer image instead: "To use the compositing output from another scene rather than its 'raw' render output, first render that scene into a series of multi-layered images (using e.g. the OpenEXR format), then load those images into the Compositor of the current scene using the Image Node." (read 2026-07-30). That tells us the node gives the raw render, not the composite, but says nothing about engine or samples.
- The view-layer documentation (`render/layers/introduction.txt`) describes the compositing relationship — "When a scene is rendered, each View Layer is available in the Compositor using they Render Layers Node" (the wording is the manual's) — but only within one scene, and says nothing about engine or samples.
- The release notes for 5.0 contain no entry about the Render Layers node's scene handling, about `-E`, or about sampling being settable from outside. The compositor notes for 5.0 are about node changes (Composite node replaced by Group Output, File Output redesign, Alpha Over input names, and so on). The only engine-related note is a rename: "EEVEE's render engine identifier was changed from `BLENDER_EEVEE_NEXT` to `BLENDER_EEVEE`." — Release notes, Python API, https://developer.blender.org/docs/release_notes/5.0/python_api/, read 2026-07-30. That matters if an engine name is being passed in from outside, but it does not answer the question.
- The core release notes show the new logging format, and its example includes lines naming the scene, the view layer and the engine at the start of a render: "00:00.245  render           | Start rendering: Scene, ViewLayer" and "00:00.245  render           | Engine: Cycles" — Release notes, Core, https://developer.blender.org/docs/release_notes/5.0/core/, read 2026-07-30. This is a sample of log output, not a statement about cross-scene rendering, so I have not drawn a finding from it. It does give a person a way to check the answer by hand — see below.
- Background Scene and the sequencer Scene strip were both examined and set aside as different mechanisms. For the record, the Scene strip page documents a camera override — "This can be used to override the scene's camera with any other object." — and documents no engine or samples override, and states one limitation: "Scene strips do not render individual Render Passes; only the Combined render pass will be used." — Manual, Scene Strip, https://docs.blender.org/manual/en/5.0/video_editing/edit/montage/strips/scene.html, read 2026-07-30. I did not pursue the strip further, since I answered the compositor reading.

---

## Things a person with Blender open could settle in a few minutes

*(**Run, and pruned — 2026-08-29.** Every test this list held was carried out on the Creator's MacBook Pro on 2026-08-29, and repeated on the same machine the same day. The answers are written into *What running Blender showed — the hand tests of 2026-08-29* above, which is where they now live; the answered items have therefore been taken off this list rather than left standing as work for somebody to do. What was set up, what was typed and what the machine printed is recorded in full in `Audits/Blender Research/The compositor/Hand tests — Architecture Fact 135.md`. This list's items were, in that record's numbering, entries 6 (item 1), 7 (item 2), 14 (item 3), 17 (item 4), 10 and 11 (item 5), 19 (item 6) and 2 (item 7).)*

**Nothing from this list is still owed.** Four things were not run at all, and they are recorded in that same document: a real press of the re-render button on the Render Layers node — the operator behind it was driven and cancelled without rendering, see *Observed 10* above; anything on Windows; everything above on a second operating system; and any third-party render engine, since every run used `--factory-startup`.

---

2026-08-29 — Claude Opus 5 (Claude Code) — The compositor, Blender 5.0.

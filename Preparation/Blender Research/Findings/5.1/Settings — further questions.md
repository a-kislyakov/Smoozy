# Settings — further questions (Blender 5.1)

**Sources consulted, and when.** Blender 5.1 manual (`Blender Sources/manual_5.1`), Blender 5.1 Python API documentation (`Blender Sources/api_5.1`), Blender 5.1 release notes (`Blender Sources/release_notes_5.1`), and Cycles' own files (`Blender Sources/cycles_5.1`). All four were taken on 2026-07-30, and that is the date read for everything cited below.

I read only the question sheet and files inside Blender Sources/. I did not open any other part of this project.

I used no helper agents, so this declaration covers all the work behind this file.

---

## 16. When a render is started from outside Blender and given a sample count of its own, what decides how many samples it actually takes, and what else has a say in when it stops?

### A note on the wording, before the answer

The question does not say which sample count is given. I answer the reading I think is meant: the count given from outside is the scene-level **render** sample count — the one the manual calls "Render (Max) Samples" for Cycles and "Samples" under **Render** for EEVEE. The other reading is that the count given is a **per-view-layer** one; I cover that case too, in point 2 below, because in Blender the two are separate settings and the answer differs.

Per the sheet's own note, I take no view on how a count is supplied from outside. I only answer what governs the render once it has one.

### The short version

In Cycles the count given acts as a **ceiling rather than a quantity**: the six points below each stand between it and the number of samples a pixel actually receives. In EEVEE two settings push the number the other way, above the count given — see the EEVEE section near the end. Separately from all six, a Time Limit can stop a Cycles render before the count is reached.

---

### 1. Where the count lands, and what that setting will hold

**Cycles.** The setting is named `samples`.

> ```
>     samples: IntProperty(
>         name="Samples",
>         description="Number of samples to render for each pixel",
>         min=1, max=(1 << 24),
>         default=4096,
>     )
> ```

Label: **observed**. File: `Blender Sources/cycles_5.1/properties.py`, lines 479–484, inside `class CyclesRenderSettings`. Date read: 2026-07-30.

In that file the same setting is drawn under two different labels, depending on one other switch:

> ```
>         col = layout.column(align=True)
>         if cscene.use_adaptive_sampling:
>             col.prop(cscene, "samples", text="Max Samples")
>             col.prop(cscene, "adaptive_min_samples", text="Min Samples")
>         else:
>             col.prop(cscene, "samples", text="Samples")
>         col.prop(cscene, "time_limit")
> ```

Label: **observed**. File: `Blender Sources/cycles_5.1/ui.py`, lines 311–317. Date read: 2026-07-30.

**EEVEE.** The setting is `taa_render_samples`:

> "Number of samples per pixel for rendering (in [1, inf], default 64)"

Label: **documented**. Source: Python API documentation, https://docs.blender.org/api/5.1/bpy.types.SceneEEVEE.html. Date read: 2026-07-30.

**The render call itself carries no sample count.** The full signature is:

> "bpy.ops.render.render(*, animation=False, write_still=False, use_viewport=False, use_sequencer_scene=False, layer='', scene='', frame_start=0, frame_end=0)"

Label: **documented**. Source: Python API documentation, https://docs.blender.org/api/5.1/bpy.ops.render.html. Date read: 2026-07-30.

What follows from that: a count given from outside cannot be carried by the render call itself, so it has to be sitting in a setting already by the time the render starts. Label for that sentence: **inferred** — the step is that the quoted signature lists every argument the render call accepts, and none of them is a sample count. Which setting it lands in is not settled by that quote; the scene and view-layer settings above are the ones the sources name, not the ones the sources rule everything else out in favour of.

---

### 2. A view layer can replace the count entirely

Each view layer carries its own sample number:

> "samples
>
> Override number of render samples for this view layer, 0 will use the scene setting (in [0, inf], default 0)"

Label: **documented**. Source: Python API documentation, https://docs.blender.org/api/5.1/bpy.types.ViewLayer.html. Date read: 2026-07-30.

> "Samples
>
> Allows the view layer to use a different number of samples than the main scene.
> This is controlled by the Layer Samples
> when using the Cycles renderer. When using EEVEE, the Samples override is always used."

Label: **documented**. Source: manual, https://docs.blender.org/manual/en/5.1/render/layers/override.html. Date read: 2026-07-30.

In Cycles, one scene-level switch decides whether those overrides count at all:

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

Label: **documented**. Source: manual, https://docs.blender.org/manual/en/5.1/render/cycles/render_settings/sampling.html. Date read: 2026-07-30.

Its default is "Use":

> ```
> enum_use_layer_samples = (
>     ('USE', "Use", "Per render layer number of samples override scene samples"),
>     ('BOUNDED', "Bounded", "Bound per render layer number of samples by global samples"),
>     ('IGNORE', "Ignore", "Ignore per render layer number of samples"),
> ```
>
> ```
>     use_layer_samples: EnumProperty(
>         name="Layer Samples",
>         description="How to use per view layer sample settings",
>         items=enum_use_layer_samples,
>         default='USE',
>     )
> ```

Label: **observed**. File: `Blender Sources/cycles_5.1/properties.py`, lines 64–67 and 550–555. Date read: 2026-07-30.

On the default setting, a view layer with a non-zero `samples` value uses that value instead of the scene one — so a count given from outside at scene level does nothing for that layer. "Bounded" makes the scene count a cap on the override. "Ignore" makes the scene count win. That switch is a Cycles one; the manual line quoted above says that in EEVEE the override is always used.

---

### 3. Adaptive sampling turns the count into a maximum

> "Adaptive Sampling
>
> If the Noise Threshold checkbox is enabled, Cycles will use adaptive sampling,
> cutting short the sampling process in areas that have become less noisy than the
> specified threshold value – in other words, not wasting time further refining
> areas that already look good enough. For example, hair on a character may need many samples,
> but the background may only need few.
>
> Noise Threshold
>
> The threshold to decide whether to continue sampling a pixel.
> Typical values are in the range from 0.1 to 0.001, with lower values meaning better
> quality but longer render times.
> Setting this to 0 makes Cycles guess an automatic value based on the total sample count.
>
> Min Samples
>
> The minimum number of samples a pixel receives before adaptive sampling is applied.
> When set to 0 (default), Cycles automatically picks a value determined by the Noise Threshold."

Label: **documented**. Source: manual, https://docs.blender.org/manual/en/5.1/render/cycles/render_settings/sampling.html. Date read: 2026-07-30.

Adaptive sampling is on by default. Of the two settings that have an automatic mode, only Min Samples sits at the value the manual calls automatic; the Noise Threshold's default is 0.01, not 0:

> ```
>     use_adaptive_sampling: BoolProperty(
>         name="Use Adaptive Sampling",
>         description="Automatically reduce the number of samples per pixel based on estimated noise level",
>         default=True,
>     )
>     adaptive_threshold: FloatProperty(
>         name="Adaptive Sampling Threshold",
>         description="Noise level step to stop sampling at, lower values reduce noise at the cost of render time. Zero for automatic setting based on number of AA samples",
>         min=0.0,
>         max=1.0,
>         soft_min=0.001,
>         default=0.01,
>         precision=4,
>     )
>     adaptive_min_samples: IntProperty(
>         name="Adaptive Min Samples",
>         description="Minimum AA samples for adaptive sampling, to discover noisy features before stopping sampling. Zero for automatic setting based on noise threshold",
>         min=0,
>         max=4096,
>         default=0,
>     )
> ```

Label: **observed**. File: `Blender Sources/cycles_5.1/properties.py`, lines 566–586. Date read: 2026-07-30.

Two things fall out of the quoted text. First, the decision is made **per pixel**, so different pixels in the same image get different numbers of samples. Second, the given count can feed back into the decision, but only where the automatic values are chosen: with Noise Threshold at 0 the threshold itself is guessed "based on the total sample count", and with Min Samples at 0 the floor is picked from the threshold. At the defaults quoted just above, that applies to Min Samples and not to the threshold. Label for all of these: **documented** — they are restatements of the quoted lines, not additions to them.

The Cycles files shipped here contain the per-pixel test itself:

> ```
> /* Determines whether to continue sampling a given pixel or if it has sufficiently converged. */
> ```
>
> ```
>   const float error = error_difference / (0.0001f + error_normalize);
>   const bool did_converge = (error < threshold);
> ```

Label: **observed**. File: `Blender Sources/cycles_5.1/source/kernel/film/adaptive_sampling.h`, the comment at line 27 and the two lines at 83–84, inside `film_adaptive_sampling_convergence_check`. Date read: 2026-07-30.

And the point where a converged pixel is skipped rather than sampled again:

> ```
>   /* Check whether the pixel has converged and should not be sampled anymore. */
>   if (!film_need_sample_pixel(kg, state, render_buffer)) {
>     return false;
>   }
> ```

Label: **observed**. File: `Blender Sources/cycles_5.1/source/kernel/integrator/init_from_camera.h`, lines 69–72, inside `integrator_init_from_camera`. Date read: 2026-07-30.

I am reporting what these two files say, not what a render does: on their own they show the test and the skip, not the outcome of any particular render. The claim that pixels end up with different sample counts rests on the manual quoted above, not on these two files.

---

### 4. Sample Subset overrides the count for the samples that get rendered

> "Sample Subset
>
> Only render a subset of the samples."

and, of the Length setting:

> "The number of samples in the subset. While this overrides (Max) Samples in terms of the
> samples that will get rendered, it's still important to set (Max) Samples to the total number
> of samples that will be rendered across all subsets – otherwise, the subsets will have
> incompatible noise and combining them will give a worse result."

Label: **documented**. Source: manual, https://docs.blender.org/manual/en/5.1/render/cycles/render_settings/sampling.html. Date read: 2026-07-30.

The settings behind it, with their defaults:

> ```
>     use_sample_subset: BoolProperty(
>         name="Use Sample Subset",
>         description="Render a subset of the specified max samples. Typically used for distributed rendering across multiple devices",
>         default=False,
>     )
>
>     sample_offset: IntProperty(
>         name="Sample Subset Offset",
>         description="0-based index of sample to start rendering from",
>         min=0, max=(1 << 24),
>         default=0,
>     )
>
>     sample_subset_length: IntProperty(
>         name="Sample Subset Length",
>         description="The number of samples to render in this subset",
>         min=1, max=(1 << 24),
>         default=2048,
>     )
> ```

Label: **observed**. File: `Blender Sources/cycles_5.1/properties.py`, lines 494–512. Date read: 2026-07-30.

It is off by default. When it is on, the manual says that Length, not Max Samples, sets how many samples get rendered — while Max Samples still has to be set to the total across all subsets, or, in the manual's words, "the subsets will have incompatible noise and combining them will give a worse result".

---

### 5. The setting's own bounds

A Cycles render sample count is bounded at `min=1, max=(1 << 24)`, i.e. 1 to 16,777,216 (observed, quoted in point 1). EEVEE's is documented as "in [1, inf]" (documented, quoted in point 1). Those are the bounds the setting declares. What Blender does when it is given a value outside them, and what it reports, is question 15's ground and I do not answer it here.

---

### 6. Cycles carries version-conversion code that can rewrite the stored count

Cycles adds its conversion function to one of Blender's handler lists:

> ```
>     bpy.app.handlers.version_update.append(version_update.do_versions)
> ```

Label: **observed**. File: `Blender Sources/cycles_5.1/__init__.py`, line 155. Date read: 2026-07-30.

That function contains, among others:

> ```
>             if version <= (3, 0, 25):
>                 cscene = scene.cycles
>
>                 # Default changes.
>                 if not cscene.is_property_set("samples"):
>                     cscene.samples = 128
> ```

and

> ```
>                 # Removal of square samples.
>                 cscene = scene.cycles
>                 use_square_samples = cscene.get("use_square_samples", False)
>
>                 if use_square_samples:
>                     cscene.samples *= cscene.samples
>                     cscene.preview_samples *= cscene.preview_samples
>                     for layer in scene.view_layers:
>                         layer.samples *= layer.samples
>                     cscene["use_square_samples"] = False
> ```

Label: **observed**. File: `Blender Sources/cycles_5.1/version_update.py`, lines 223–228 and 242–251. Date read: 2026-07-30.

So for an old enough file, the scene's and every view layer's sample number can be set or squared by this code. That it runs while a file is being loaded is **inferred** — the step is the name of the handler list the function is added to; nothing quoted here states when Blender calls that list. I include this only because it changes the very number a count given from outside is competing with. I make no claim here about the order in which a version conversion and an outside-supplied count take effect — the files I have do not settle that.

---

### What else has a say in when it stops

**Time Limit — the only stop condition other than reaching the count that the searches below turned up.**

> "Time Limit
>
> Stops rendering if the time exceeds the limit, even if the desired sample count
> hasn't been reached yet. Setting this value to zero disables the limit."
>
> "Note
>
> The time limit does not include pre-render processing time, only render time."

Label: **documented**. Source: manual, https://docs.blender.org/manual/en/5.1/render/cycles/render_settings/sampling.html. Date read: 2026-07-30.

> ```
>     time_limit: FloatProperty(
>         name="Time Limit",
>         description="Limit the render time (excluding synchronization time). "
>         "Zero disables the limit",
>         min=0.0,
>         default=0.0,
>         step=100.0,
>         unit='TIME_ABSOLUTE',
>     )
> ```

Label: **observed**. File: `Blender Sources/cycles_5.1/properties.py`, lines 514–522. Date read: 2026-07-30. It is off by default (0.0), and it is a Cycles setting.

**A corroborating list.** Cycles' own sampling preset saves exactly this group of settings together:

> ```
>     preset_values = [
>         "cycles.use_adaptive_sampling",
>         "cycles.samples",
>         "cycles.adaptive_threshold",
>         "cycles.adaptive_min_samples",
>         "cycles.time_limit",
> ```

Label: **observed**. File: `Blender Sources/cycles_5.1/presets.py`, lines 49–54, inside `class AddPresetSampling`. Date read: 2026-07-30. This is what Cycles itself treats as the sampling group; it is not proof that nothing else affects the outcome.

**Nothing else was found.** See the record of looking below.

---

### EEVEE is a different picture

EEVEE's sampling page lists a Render Samples number and nothing resembling a time limit, a noise threshold or adaptive sampling:

> "Render
>
> Samples
>
> The number of samples to use in the final render."

Label: **documented**. Source: manual, https://docs.blender.org/manual/en/5.1/render/eevee/render_settings/sampling.html. Date read: 2026-07-30.

But two other EEVEE settings change the number actually taken, upward:

> "Jitter Camera
>
> Randomize the camera position for every scene render sample to increase precision.
> Enabling this option can change the scene's actual sample count.
>
> Note
>
> Be aware that the actual sample count can grow quite rapidly.
>
> Hint
>
> The actual number of samples is computed by the following formula:
>
> \[sample\_count = (ring\_count^{2} + ring\_count) * 3 + 1\]
>
> where \(ring\_count\) is the number of ring in the hexaweb pattern.
> The \(ring\_count\) is chosen so that the entire pattern contains at least the number of
> samples set in the Render Settings."

Label: **documented**. Source: manual, https://docs.blender.org/manual/en/5.1/render/eevee/render_settings/depth_of_field.html. Date read: 2026-07-30.

> "Note
>
> When using multiple time steps, the render sample count is rounded up to the next multiple
> of steps to ensure even distribution of samples across steps.
> This can lower the quality of jitter camera depth of field.
> Increasing the number of viewport or render samples will get the
> quality back."

Label: **documented**. Source: manual, https://docs.blender.org/manual/en/5.1/render/eevee/render_settings/motion_blur.html. Date read: 2026-07-30.

So in EEVEE the given count can be treated as a floor rather than a ceiling: with Jitter Camera on, the pattern is grown until it holds *at least* the requested number; with more than one motion-blur step, the count is rounded *up* to a multiple of the step count. The motion blur step count is documented as "in [1, inf], default 1" (Python API documentation, https://docs.blender.org/api/5.1/bpy.types.SceneEEVEE.html, `motion_blur_steps`, read 2026-07-30), so with the default of 1 the rounding has no effect.

---

### LABEL

**Documented**, with the mechanism parts **observed** in Cycles' own files, as marked point by point above. No part of this answer is inferred except where the word "inferred" appears.

### NOT COVERED

- **How the mechanisms combine.** Each quoted piece is clear on its own. The sources do not state the order in which they are applied, or what happens when several apply at once — for instance whether the Time Limit is checked before or after adaptive sampling has retired most pixels, or how Sample Subset and a view-layer override interact. I could not settle this from the files: the shipped Cycles files contain the per-pixel convergence kernel but not the render scheduler. I searched all 271 files under `Blender Sources/cycles_5.1/source` for `time_limit`, `sample_offset`, `sample_subset` and `num_samples`; `time_limit` and `sample_subset` do not appear at all, and `sample_offset` appears only as a field of the work tile that the kernel is handed (`Blender Sources/cycles_5.1/source/kernel/types.h`, line 1624, in `struct KernelWorkTile`). The code that fills that tile in is not here.
- **Whether the Time Limit is per frame or for a whole animation.** The manual says only "Stops rendering if the time exceeds the limit". It does not say what "a render" means for an animation. Nothing in the manual, the API documentation, the release notes or the Cycles files settles it.
- **What the automatic values actually are.** Noise Threshold 0 gives "an automatic value based on the total sample count" and Min Samples 0 gives one "determined by the Noise Threshold". Neither manual page gives a formula. The record of looking below does not cover a search of the shipped Cycles files under `source/` for the code that works these out, so this is a gap in the looking rather than a settled silence.
- **Stopping a running render from outside.** Beyond Time Limit, I found nothing in the manual describing any way to stop a render early, and nothing about signals or interruption of a background render. What the same words return in the API documentation and the Cycles files is not recorded. See the record of looking.
- **Engines other than Cycles and EEVEE.** The sample-count settings above belong to those two. I did not look for others, and the question is about the render that runs, whichever engine that is.
- **Questions 1, 14 and 15.** How a count may be supplied from outside, what values the resolution percentage takes, and what happens on an out-of-range value are all deliberately left to those answers.

### OUTSIDE THE INTERFACE

Everything above lives in scene and view-layer data rather than in the render command, and the render call takes no sample argument (documented, point 1). The sources state no difference in how the sample count is decided when Blender is driven from outside its own window. Three related facts the sources do state:

- The count that applies is the **render** one, not the viewport one. "Viewport (Max) Samples: The number of paths to trace per pixel in the 3D Viewport (when using the Rendered shading mode)" versus "Render (Max) Samples: The number of paths to trace per pixel in the final render" (documented, manual, https://docs.blender.org/manual/en/5.1/render/cycles/render_settings/sampling.html, read 2026-07-30). All the viewport-only twins in the Cycles files — `preview_samples`, `use_preview_adaptive_sampling`, `preview_adaptive_threshold`, `preview_adaptive_min_samples` — are separate settings from the render ones (observed, `Blender Sources/cycles_5.1/properties.py`, lines 485–492 and 588–606).
- Which view layers render, and therefore which per-layer sample overrides are in play, is affected by the command line: "Render Single Layer: Only render the active view layer. Note: This option is ignored when rendering from the command line." (documented, manual, https://docs.blender.org/manual/en/5.1/render/layers/view_layer.html, read 2026-07-30).
- Cycles' own command-line parser accepts two arguments and no more: `--cycles-print-stats` and `--cycles-device` (observed, `Blender Sources/cycles_5.1/engine.py`, lines 13–20, in `_configure_argument_parser`; also documented in the manual under "Cycles Render Options", https://docs.blender.org/manual/en/5.1/advanced/command_line/arguments.html). I state this as a fact about that parser only. It is not an answer to how a sample count may be supplied, which this question takes no view on.

**Settled by test on 2026-08-10. LABEL: observed.** Every one of the fifteen tests recorded under *Hand-test results* below was run from a terminal with no window at all, on two computers, and each of the settings above did what this answer describes in that setting: the sample count, adaptive sampling, the time limit, the sample subset and the Layer Samples setting all took effect in a render started from outside Blender. Nothing behaved differently for being driven from outside. The viewport copies of these settings have no bearing on such a render — changing all four of them left the reported count and every pixel of the picture unchanged. And these settings do travel with the file: eleven of them were saved and read back unchanged. What is still untested is the comparison against a render started inside Blender's own window, which neither machine could run this round.

### OPERATING SYSTEM

The sources state no operating-system difference in any of the settings above, or in how the sample count is decided. I found one neighbouring OS fact worth naming, because it bears on whether an EEVEE render is supported at all:

> "Headless Rendering
>
> Headless rendering is not supported on headless Windows systems."

Label: **documented**. Source: manual, https://docs.blender.org/manual/en/5.1/render/eevee/limitations/limitations.html. Date read: 2026-07-30.

Other OS-touching items near this subject — the device list `CPU CUDA OPTIX HIP ONEAPI METAL`, and the denoiser choices — concern which hardware does the work and how the image is cleaned, not how many samples are taken. The sources do not connect either to the sample count.

**Settled by test on 2026-08-10, for two operating systems out of three. LABEL: observed.** All fifteen tests recorded under *Hand-test results* below were run on macOS 26.5.2 on Apple silicon and on Ubuntu 22.04.5 LTS on an AMD processor, and the two machines returned the same sample count in every single test. Where a figure differed between them it was a time, or a count reached under a clock, never a count reached under a sample setting. **Windows is untested and stays untested** — no Windows machine has run it — so nothing here speaks for it.

---

## RECORD OF LOOKING — for the "nothing else stops it" and "no combining order" answers

Searches run over the whole of `Blender Sources/manual_5.1`, `Blender Sources/api_5.1`, `Blender Sources/release_notes_5.1` and `Blender Sources/cycles_5.1`, all case-insensitive, on 2026-07-30 sources:

- **Stop conditions:** `time limit`, `timeout`, `deadline`, `budget`, `quota`, `sample limit`, `render limit`, `stop rendering`, `stop sampling`, `stops render`, `terminat`, `early exit`, `converge`, `cancel`, `abort`, `interrupt`, `Ctrl-C`.
  - `time limit` appears in the manual only on the Cycles Sampling page (two lines, both quoted above). Nowhere else in the manual, at all.
  - `timeout` in the manual hits only unrelated pages: GPU driver Timeout Detection Recovery on the compositor limitations page, and Pie Menu timings in the Interface preferences. `deadline`, `budget`, `quota`, `sample limit`, `render limit` return nothing in the manual.
  - `terminat` and `converge` hit only shading and light-path topics — shadow terminator artefacts, Russian Roulette path termination, Light Threshold, portals — none of which is a stop condition for the render as a whole.
  - `cancel` / `abort` / `interrupt` across `render/` and `advanced/command_line/` return four hits, all unrelated to stopping a render early: advice to render animations as image sequences so a cancelled job can be resumed by changing the frame range (https://docs.blender.org/manual/en/5.1/render/output/introduction.html and .../render/output/animation.html), and the `--disable-abort-handler` and `--disable-crash-handler` arguments (https://docs.blender.org/manual/en/5.1/advanced/command_line/arguments.html). Within those manual pages I found nothing describing a signal, key or command that stops a background render. What those three words return in the API documentation and in the Cycles files is not recorded here.
- **Sample count:** `samples`, `sample count`, `max samples`, `sample override`, `samples_override`, `spp`, `adaptive`, `noise threshold`, `subset`.
  - Outside the pages quoted above, `sample count` in the manual hits only the EEVEE depth-of-field and motion-blur notes (both quoted), an EEVEE material note about evaluation cost, the Cycles Sample Count render pass, and an Alembic export note — none of which adds a stop condition.
  - `samples` across `Blender Sources/manual_5.1/advanced/` — which holds the whole command-line section — returns **no hits at all**. The command-line pages `arguments.txt`, `render.txt`, `extension_arguments.txt`, `index.txt` and the three `launch/` pages were searched together; the only Cycles-specific arguments documented are the two named above.
  - `time_limit`, `adaptive_threshold` and `use_adaptive_sampling` across the whole of `Blender Sources/api_5.1` hit only one file, `bpy.ops.wm.txt`, and there only as generic operator text — the Cycles settings are not in the Python API documentation, which is why the Cycles files are cited for them.
- **Release notes:** `sampl`, `time limit`, `adaptive` across all 21 files of `Blender Sources/release_notes_5.1`. No hit concerns render sampling. `cycles.md` for 5.1 lists performance improvements and shading features only; `eevee.md` lists the Raycast node, light path intensity, planar reflections, an AOV maximum raised to 128, and compilation and memory work. Neither mentions sample counts, the time limit, or adaptive sampling. So nothing in 5.1 is recorded as having changed in this area.
- **Pictures:** the Cycles Sampling page, the EEVEE Sampling page and the View Layer Override page carry no `[image: …]` marker in 5.1 — there is no screenshot of the sampling panel to consult. The only file in `manual_5.1/_images` whose name contains "sampling" is `render_materials_legacy-textures_types_image-movie_image-sampling-panel.png`, which is a texture panel, not a render sampling panel. No picture was used for this answer.

---

## Hand-test results — 2026-08-10

**LABEL: observed**, throughout this section.

Fifteen tests were run in Blender 5.1.2 on two computers, on 2026-08-10. **The Creator's own Mac** — a MacBook Pro with an M1 Pro chip, macOS 26.5.2, eight cores, 16 GB of memory, Apple's own graphics and no second card. **A rented Linux machine** — Ubuntu 22.04.5 LTS on kernel 6.8, one NVIDIA GeForce RTX 4060 Ti with 16 GB on driver 580.95.05, twenty-three cores of an AMD EPYC 7K62, 98 GB of memory, and no display server at all. Every test was run on both, and the two machines are two results, never one.

**How they were run.** Each Blender was pointed at a throwaway settings folder of its own, made before use, so no real preferences were involved. Every render was on the processor rather than the graphics card, because a fresh settings folder switches no graphics device on and the scene asks for `CPU`. The rented machine is somebody else's hardware and a share of a bigger chip, so its times are its own and are never read against the Mac's; it was idle when anything was timed (`load average: 0.00, 0.02, 1.30`, nothing above 4% of one core, and `0MiB / 16380MiB` used on the card).

**The scene**, built by Blender 5.1.2 itself from a script so that no file saved by another Blender is involved: 320 by 240, Cycles on the processor, a plane, a rough metal sphere, a glass cube, one large area light and a dark world — noisy enough that sampling shows in the clock. Renders were started as `blender -b <file> --python-expr "<settings>" -o <folder> -F PNG -f 1`. *(Citation check, 2026-08-10: the script itself is not here, and without it this scene cannot be rebuilt exactly. Repeating these tests on a scene built afresh to this description returned every count that does not depend on how noisy the scene is — 4096/4096, 32/4096, 256/256, 8/8, the six view-layer results, the EEVEE counts — and different figures for everything reached under a clock. A test that turns on a clock needs its scene supplied, not described.)* *(Second batch of hand tests, 2026-08-10: the script is now supplied in full, under “Second batch of hand tests” further down this document, and the four figures that turn on a clock were run again on it.)*

**How the answer was read.** Blender prints a running `Sample n/N` line for each view layer of each frame; the last one with a non-zero `n` is the count that run reached and `N` is the count it was counting towards. One line reading `Sample 0/N` is printed just before `Finished` and is not a result. EEVEE prints `Rendering n / N samples` instead.

### The results, test by test

The Mac and the rented Linux machine returned **the same sample count in every one of these tests**. Only the times differ, and both are given.

| What was set | What Blender reported (both machines) | Mac time | Linux time |
|---|---|---|---|
| Max Samples 4096, adaptive sampling off | `Sample 4096/4096` | `01:15.26` | `00:36.01` |
| Max Samples 4096, adaptive on at 0.01 | `Sample 4096/4096` | `00:21.65` | `00:10.49` |
| Max Samples 4096, adaptive on at 0.5 | `Sample 32/4096` | `00:00.98` | `00:00.82` |
| Max Samples 2048, sample subset off | `Sample 2048/2048` | `00:40.18` | `00:17.72` |
| Max Samples 2048, subset on, Length 256 | `Sample 256/256` | `00:05.07` | `00:02.61` |
| the same plus adaptive on at 0.01 | `Sample 256/256` | `00:04.84` | `00:02.46` |
| the same plus a 2-second Time Limit | `Sample 129/256` (Mac), `Sample 233/256` (Linux) | `00:02.46` | `00:02.40` |
| Max Samples 200, viewport numbers set low | `Sample 200/200` | `00:03.18` | `00:01.74` |
| Max Samples 200, viewport numbers set high | `Sample 200/200` | `00:02.96` | `00:01.69` |
| Max Samples 8 with Min Samples 64 | `Sample 8/8` | `00:00.38` | `00:00.49` |
| Max Samples 8 with Min Samples 0 | `Sample 8/8` | `00:00.37` | `00:00.52` |
| Max Samples 1024, adaptive on at 0.01 | `Sample 1024/1024` | `00:08.82` | `00:04.26` |
| Max Samples 1024, adaptive on at 0 | `Sample 1024/1024` | `00:19.50` | `00:09.01` |
| Max Samples 128, adaptive on at 0 | `Sample 128/128` | `00:02.32` | `00:01.38` |
| Max Samples 4096, adaptive on at 0 | `Sample 4096/4096` | `01:08.98` | `00:32.48` |
| stored count 4096, 32 supplied at launch before `-f` | `Sample 32/32` | `00:00.88` | `00:00.69` |
| stored count 4096, 32 supplied at launch after `-f` | `Sample 4096/4096` | `01:19.14` | `00:36.04` |
| EEVEE Render Samples 64, Cycles 4096 in the same file | `Rendering 64 / 64 samples` | `00:02.37` | `00:29.94` |
| EEVEE Render Samples 64, motion blur on, 5 steps | `Rendering 65 / 65 samples` | `00:00.65` | `00:10.90` |
| EEVEE Render Samples 16, Jitter Camera on | `Rendering 19 / 19 samples` | `00:00.30` | `00:06.87` |
| EEVEE Render Samples 16, Jitter Camera off | `Rendering 16 / 16 samples` | `00:00.30` | `00:03.98` |
| Workbench | no sample line of any kind | `00:00.14` | `00:00.94` |
| Max Samples 64 with `-- --cycles-print-stats` | `Sample 64/64`, then a `Render statistics:` block | `00:01.47` | `00:00.95` |

### A frame does end before Max Samples when its pixels converge, and the count is decided pixel by pixel

The first three rows above are the same frame three ways. With adaptive sampling on at a loose threshold of 0.5 the whole frame ended at `Sample 32/4096`, so a frame does stop once its pixels have converged rather than only individual pixels stopping. With adaptive sampling on at the default 0.01 the reported count did not move at all while the time fell to about a quarter, which is the per-pixel skipping showing in the clock rather than in the count.

The Cycles view-layer settings carry a sample-count pass, printed on both machines as:

```
PASS_SAMPLE_ATTRS ['denoising_store_passes', 'pass_debug_sample_count', 'use_denoising']
```

### The Cycles Time Limit is spent on each frame, not across the whole job

Three frames in one launch with `-s 1 -e 3 -a`, Time Limit 10 seconds, Max Samples 4096, adaptive sampling off, three repeats on each machine. **Every one of the six runs wrote three PNG files.**

On the Mac the whole run took 32.09, 31.15 and 31.17 seconds. The per-frame `Time:` lines in the first repeat were `00:10.31`, `00:10.38` and `00:10.14`, and the three frames reached `Sample 550/4096`, `Sample 577/4096` and `Sample 553/4096`.

On the rented Linux machine, idle, the whole run took 33.36, 32.16 and 32.25 seconds. The per-frame times in the first repeat were `00:11.62`, `00:10.59` and `00:10.33`, and the frames reached `Sample 1281/4096`, `Sample 1153/4096` and `Sample 1153/4096`.

Not one of the six runs stopped the job after ten seconds. **Nothing is printed to say the limit fired**: the frame reports its count, says `Finished`, saves the image, and moves on. The only sign is the count sitting below the maximum.

### Which number a render counts to when a view layer carries its own

Two halves, because an override below the scene count cannot tell `Bounded` from `Use`. Adaptive sampling off, Time Limit 0, and the scene count supplied at launch by `--python-expr` rather than saved in the file. Both machines gave these six results identically:

| | second layer's own Samples 64, scene count 1024 | second layer's own Samples 1024, scene count 256 |
|---|---|---|
| `USE` | first layer `Sample 1024/1024`, second `Sample 64/64` | first `Sample 256/256`, second `Sample 1024/1024` |
| `BOUNDED` | first `Sample 1024/1024`, second `Sample 64/64` | first `Sample 256/256`, second `Sample 256/256` |
| `IGNORE` | first `Sample 1024/1024`, second `Sample 1024/1024` | first `Sample 256/256`, second `Sample 256/256` |

So a view layer's own number stands in place of a scene count supplied from outside under `USE`, is capped by that count under `BOUNDED`, and is thrown away under `IGNORE`. Blender 5.1.2 on the Mac, second half, `BOUNDED`, as printed:

```
00:00.353  render           | Start rendering: Scene, ViewLayer
00:05.273  render           | Fra: 1 | Mem: 6M | Sample 256/256
00:05.274  render           | Start rendering: Scene, Second
00:09.958  render           | Fra: 1 | Mem: 6M | Sample 256/256
POLICY BOUNDED SCENE 256 LAYERS [('ViewLayer', 0), ('Second', 1024)]
```

**One thing about the second half on the Mac.** Those three runs happened between 15:56 and 15:59 on 2026-08-10, and the Creator's own Blender 5.2 was open on that machine from 15:55:59 onwards — so for that one entry, and no other in this set, a second Blender was running beside the test. It changes nothing this entry rests on: the answer is the counts, the counts do not depend on how busy the machine is, and the same six counts came back from the rented Linux machine, where nothing else was running. The times in the excerpt above were taken under that load, and nothing here is built on them.

### The sampling settings survive a save and a reopen unchanged

Eleven Cycles values and EEVEE's render sample count were set, saved to a new file, and read back by opening that file. Both machines answered:

```
READBACK CYCLES 137 42.0 False 0.004999999888241291 9 BOUNDED True 3 11 64 77
```

In order: the engine, Max Samples 137, Time Limit 42.0, adaptive sampling off, adaptive threshold 0.005, Min Samples 9, Layer Samples `BOUNDED`, sample subset on, subset offset 3, subset length 11, the view layer's own Samples 64, EEVEE's Render Samples 77. Nothing was rewritten as the file loaded, and the Layer Samples setting keeps its value across a save and a reopen whatever the panel does or does not show. The threshold reads back as `0.004999999888241291` because 0.005 has no exact form in that kind of number; it is the same value.

### The viewport sampling settings have no part in a render with no window

Render sampling held at Max Samples 200 with adaptive sampling on at 0.01, and the viewport numbers changed between two runs from `preview_samples` 4, viewport adaptive on at threshold 0.5 and viewport Min Samples 1, to `preview_samples` 3000, viewport adaptive off at 0.001 and viewport Min Samples 512. Both runs reported `Sample 200/200`, and the two pictures were compared pixel by pixel rather than byte by byte:

```
PIXELS max_difference 0.0 channels_differing 0 of 307200
```

Identical on both machines.

### What else can stop a render started from outside

A 4096-sample frame with the Time Limit off was started with no window, left for eight seconds, then interrupted, three ways, on both machines.

An interrupt signal stopped it at the next sample report, wrote no file, and exited 1. On the Mac:

```
00:20.321  render           | Fra: 1 | Remaining: 00:59.35 | Mem: 21M | Sample 1025/4096
Sent an internal break event. Press ^C again to kill Blender
00:20.322  render           | Fra: 1 | Remaining: 00:59.35 | Mem: 21M | Cancelled
Blender quit
```

On the rented Linux machine the same three lines came back, with the stop at `00:09.398` and also at `Sample 1025/4096`. On both machines the stop landed on the same sample number rather than at the same moment.

A terminate signal printed nothing further, wrote no file, and gave exit status 143; the last line of its log is an ordinary `Sample 1/4096`. Adding `--disable-abort-handler` to the interrupt run changed nothing at all — the same break line, the same `Cancelled`, exit 1, no file. All the same on both machines.

### EEVEE and Workbench

The engine identifier came from Blender itself: `blender -b --factory-startup -E help` printed `BLENDER_EEVEE`, alongside `BLENDER_WORKBENCH` and `CYCLES`.

EEVEE took exactly the number it was given, and Cycles' Max Samples of 4096 sitting in the same file changed nothing. The two features that raise the count did so exactly as the table shows: five motion-blur steps turned 64 into 65, and Jitter Camera turned 16 into 19. Printing the whole EEVEE settings block turned up no time limit. Searched for "time" and "limit", the only entries were:

```
EEVEEPROP gi_diffuse_bounces | Diffuse Bounces | Number of times the light is reinjected inside light grids, 0 disable indirect diffuse light
EEVEEPROP motion_blur_steps | Motion steps | Controls accuracy of motion blur, more steps means longer render time
EEVEEPROP taa_samples | Viewport Samples | Number of samples, unlimited if 0
```

**A view layer's sample override does change an EEVEE render in 5.1.** With Render Samples 256 and a second view layer carrying its own Samples of 8, the first layer reported `Rendering 256 / 256 samples` and the second `Rendering 8 / 8 samples` — `00:01.50` on the Mac and `00:33.34` on the Linux machine, with `LAYERS [('ViewLayer', 0), ('Second', 8)]` printed alongside. The same on both machines.

On the rented Linux machine, which has no display server at all, every EEVEE run worked and wrote its image, printing three copies of this line first:

```
EGL Error (0x3009): EGL_BAD_MATCH: Arguments are inconsistent (for example, a valid context requires buffers not supplied by a valid surface).
```

*(Citation check, 2026-08-10: this line did not come back. Repeating the EEVEE runs on the same rented machine today — now on graphics driver 580.173.02, where these tests were recorded at 580.95.05 — printed no EGL line at all, while every EEVEE run still worked and still wrote its image. Both results stand; the question goes back as a test.)* *(Second batch of hand tests, 2026-08-10: run a third time on that machine, still on driver 580.173.02 — again no EGL line, in any of the four versions, and again every EEVEE render worked and wrote its image. All three runs stand; see “Second batch of hand tests” below for the whole log.)*

With the engine set to Workbench the run printed no sample line at all, and the setting printed:

```
RENDER_AA '8'
RENDER_AA ITEMS ['OFF', 'FXAA', '5', '8', '11', '16', '32']
CYCLES SAMPLES STILL 4096
```

### A command-line render ignores "Render Single Layer", so a second layer's own count is in play

With `use_single_layer` set to `True`, the first layer active and a second view layer carrying Samples 64 against a scene count of 256, both layers rendered — `Sample 256/256` and `Sample 64/64`, `00:06.28` on the Mac and `00:03.35` on the Linux machine — with `SINGLE_LAYER True` printed alongside. In background mode there is no window, so the instruction meant to make the first layer active had no window to act on; it printed nothing and did not error, and both layers rendered regardless.

### The Noise Threshold's default is not the automatic value it uses at zero

Adaptive sampling on, Min Samples 0, Time Limit 0. Every one of these runs reached its maximum, as the table shows; what separates them is the time and the picture. At Max Samples 1024 a threshold of 0 took `00:19.50` against `00:08.82` for the default 0.01 on the Mac, and `00:09.01` against `00:04.26` on the Linux machine. The two pictures are different pictures on both machines:

```
PIXELS max_difference 0.0313725471496582 channels_differing 25107 of 307200      (the Mac)
PIXELS max_difference 0.0313725471496582 channels_differing 25195 of 307200      (the rented Linux machine)
```

Two things this does not settle. It does not give the automatic value — nothing the run prints names it. And it does not settle whether that value moves with the sample count, because the cost of one sample changes with the count on its own, so the three threshold-zero runs cannot be read against each other.

### `--cycles-print-stats` reports no sample count

The run reported `Sample 64/64` as usual and then printed a block beginning `Render statistics:`, holding mesh geometry memory, a per-kernel timing breakdown, and shader and object statistics. Its opening, from the rented Linux machine:

```
Render statistics:
Mesh statistics:
  Geometry:
    Total memory: 37.24K (38,135)
        Sphere                           36.74K (37,621)
        Cube                             289B (289)
        Plane                            119B (119)
```

No figure anywhere in that block names a sample count. The same on both machines.

### Second batch of hand tests — 2026-08-10: the scene supplied, and three more tests

**LABEL: observed**, throughout this section.

The citation check of these findings on 2026-08-10 found that the section above **describes** the test scene without **supplying** it, so four of its figures — the ones a clock decides — could not be got again by anybody who was not there. A second, short batch of hand tests was run the same day to close that, along with two other things about the record. Everything above stands unchanged; nothing here replaces it.

**The scene, supplied.** This is the script that built the test scene, in full. Blender 5.1.2 ran it itself, so no file saved by another Blender is involved. Save it as `make_scene.py` and run it as `blender -b --factory-startup --python make_scene.py -- <somewhere>/scene.blend`:

```python
import bpy, sys, math

out = sys.argv[-1]

# Start from an empty scene
bpy.ops.wm.read_factory_settings(use_empty=True)
scene = bpy.context.scene
scene.render.engine = 'CYCLES'
scene.cycles.device = 'CPU'
scene.render.resolution_x = 320
scene.render.resolution_y = 240
scene.render.resolution_percentage = 100
scene.render.image_settings.file_format = 'PNG'

# Floor
bpy.ops.mesh.primitive_plane_add(size=20, location=(0, 0, 0))
floor = bpy.context.active_object
mfloor = bpy.data.materials.new("Floor")
mfloor.use_nodes = True
bsdf = mfloor.node_tree.nodes["Principled BSDF"]
bsdf.inputs["Base Color"].default_value = (0.55, 0.55, 0.6, 1.0)
bsdf.inputs["Roughness"].default_value = 0.35
floor.data.materials.append(mfloor)

# A rough metal sphere: reliable source of sampling noise
bpy.ops.mesh.primitive_uv_sphere_add(radius=1.4, location=(0, 0, 1.4), segments=48, ring_count=24)
sphere = bpy.context.active_object
bpy.ops.object.shade_smooth()
msph = bpy.data.materials.new("Metal")
msph.use_nodes = True
b2 = msph.node_tree.nodes["Principled BSDF"]
b2.inputs["Base Color"].default_value = (0.9, 0.6, 0.25, 1.0)
b2.inputs["Metallic"].default_value = 1.0
b2.inputs["Roughness"].default_value = 0.28
sphere.data.materials.append(msph)

# A glass cube next to it, more noise
bpy.ops.mesh.primitive_cube_add(size=1.6, location=(2.6, -0.6, 0.8))
cube = bpy.context.active_object
mg = bpy.data.materials.new("Glass")
mg.use_nodes = True
b3 = mg.node_tree.nodes["Principled BSDF"]
b3.inputs["Base Color"].default_value = (0.85, 0.9, 1.0, 1.0)
b3.inputs["Roughness"].default_value = 0.05
if "Transmission Weight" in b3.inputs:
    b3.inputs["Transmission Weight"].default_value = 1.0
elif "Transmission" in b3.inputs:
    b3.inputs["Transmission"].default_value = 1.0
cube.data.materials.append(mg)

# Big soft area light: soft shadows are noisy
bpy.ops.object.light_add(type='AREA', location=(3.5, -4.0, 6.0))
light = bpy.context.active_object
light.data.energy = 3000.0
light.data.size = 5.0
light.rotation_euler = (math.radians(40), 0.0, math.radians(40))

# World
world = bpy.data.worlds.new("World")
scene.world = world
world.use_nodes = True
world.node_tree.nodes["Background"].inputs["Color"].default_value = (0.05, 0.06, 0.09, 1.0)
world.node_tree.nodes["Background"].inputs["Strength"].default_value = 1.0

# Camera
bpy.ops.object.camera_add(location=(7.5, -7.5, 4.6))
cam = bpy.context.active_object
cam.rotation_euler = (math.radians(66), 0.0, math.radians(45))
scene.camera = cam

# Something that moves, so frames are not identical
sphere.location = (0.0, 0.0, 1.4)
sphere.keyframe_insert(data_path="location", frame=1)
sphere.location = (0.0, 0.0, 1.9)
sphere.keyframe_insert(data_path="location", frame=10)

scene.frame_start = 1
scene.frame_end = 10

bpy.ops.wm.save_as_mainfile(filepath=out)
print("SCENE WRITTEN", out)
```

**The machines, and one thing about each.** The runs were made on the same two computers as the first batch. Two differences from the first batch are recorded here rather than left to be discovered:

- **On the Creator's Mac** (MacBook Pro, M1 Pro, macOS 26.5.2, Apple's own graphics, no second card) the Creator's own Blender 5.2 was open and working throughout, along with a background helper of its own and a BlenderKit client. The load average read `2.54` when the first version started and `23.77` when the last finished. The first batch's Mac figures were taken with nothing else on the machine. So every Mac figure below was got on a busier computer than the one it is being compared with, and that is a difference which is not the scene.
- **On the rented Linux machine** (Ubuntu 22.04.5 LTS, kernel 6.8, AMD EPYC 7K62, 98 GB, no display server) the graphics driver now reads `580.173.02` on both halves, where the first batch recorded `580.95.05`. That machine was idle throughout: `load average: 0.01, 0.04, 0.07` at the start, nobody logged in, `0 MiB` of graphics memory in use.

Every render below was on the processor on both machines.

#### The four figures that turn on a clock, run again on the supplied scene

**The count reached under a 2-second time limit on a 256-sample subset.** Max Samples 2048, adaptive sampling off, subset on at Offset 0 and Length 256, Time Limit 2 seconds. On the Mac the record gives `Sample 129/256` and this run gave `Sample 129/256` — the same. On the rented Linux machine the record gives `Sample 233/256` and this run gave `Sample 233/256` — the same.

**The per-frame counts under a 10-second limit across three frames.** Max Samples 4096, adaptive sampling off, Time Limit 10 seconds, frames 1 to 3 in one launch with `-s 1 -e 3 -a`. On the Mac the record gives `550`, `577`, `553` and this run gave `555`, `561`, `556` — different — in 31.080 s of wall clock, with three PNG files written. On the rented Linux machine the record gives `1281`, `1153`, `1153` and this run gave `1281`, `1153`, `1153` — the same — in 33.317 s, again with three files written.

**The channels differing between a noise threshold of 0.01 and one of 0.** Max Samples 1024, adaptive sampling on, Min Samples 0, Time Limit 0; both renders reached `Sample 1024/1024` and the two pictures were compared channel by channel. On the Mac:

```
PIXELS max_difference 0.0313725471496582 channels_differing 25107 of 307200
```

against a recorded `channels_differing 25107 of 307200`. On the rented Linux machine:

```
PIXELS max_difference 0.0313725471496582 channels_differing 25195 of 307200
```

against a recorded `channels_differing 25195 of 307200`.

**The sample number an interrupt lands on after eight seconds.** Max Samples 4096, Time Limit 0, the render started in the background, left for eight seconds, then sent an interrupt. On the Mac it stopped at `Sample 1025/4096` at `00:21.582`; on the rented Linux machine at `Sample 1025/4096` at `00:09.597`. The record gives `Sample 1025/4096` on both machines. Both runs wrote no file and exited 1, as recorded. This run reached the same sample number as the record on the Mac.

**What the four figures together say.** Supplying the scene was most of what was missing and it was not all of it. Across all four versions and both machines — thirty-two figures in all — the picture comparison came back within six channels of the record everywhere and exactly right in three cases, where the citation check's repeat on a scene built from the written description had been out by more than a thousand; the interrupt landed on the recorded sample number in seven cases out of eight; the two-second subset count came back exactly in five cases out of eight; and the ten-second per-frame counts came back exactly in two cases out of eight. The looser a figure's grip on the clock, the better it repeats. **No claim these figures were quoted in support of changed.** A time limit still cut the subset short, ten seconds was still spent on each frame rather than once for the job, a threshold of 0.01 and a threshold of 0 still made different pictures, and an interrupt still stopped the render without writing a file.

#### The EGL line on the rented Linux machine, a third time

One EEVEE render in Blender 5.1.2 on that machine, engine `BLENDER_EEVEE`, Render Samples 64, with the whole log kept and the driver read at the same time. The driver answered:

```
580.173.02
NVRM version: NVIDIA UNIX Open Kernel Module for x86_64  580.173.02  Release Build  (dvs-builder@U22-I3-AK02-24-4)  Tue Jun 23 08:17:01 UTC 2026
```

and the render's whole log, word for word, was:

```
00:00.371  blend            | Read blend: "/root/q16b/scene5.1.blend"
00:00.382  render           | Rendering single frame
00:00.382  render           | Rendering frame 1
00:00.382  render           | Start rendering: Scene, ViewLayer
00:00.382  render           | Engine: EEVEE
00:02.528  render           | Fra: 1 | Rendering 1 / 64 samples
Blender 5.1.2 (hash ec6e62d40fa9 built 2026-05-19 02:18:37)
00:02.555  render           | Fra: 1 | Rendering 25 / 64 samples
00:02.576  render           | Fra: 1 | Rendering 50 / 64 samples
00:02.588  render           | Fra: 1 | Rendering 64 / 64 samples
00:02.730  render           | Saved: '/root/q16b/out/e17_0001.png'
00:02.730  render           | Time: 00:02.34 (Saving: 00:00.13)

Blender quit
```

`grep -c EGL` on that log returned `0`. The render finished, exited 0 and wrote its image.

**Three runs now stand on this, and none is chosen between.** The first batch, on driver 580.95.05, recorded three copies of `EGL Error (0x3009): EGL_BAD_MATCH: …` before every EEVEE render, with every render working anyway. The citation check, on 580.173.02, found no EGL line, with every render working. This run, on 580.173.02, found no EGL line, with every render working. All three are recorded and none is withdrawn. What every one of the three agrees on is the thing the answer rests on: an EEVEE render works on a machine with no display server and writes its image.

#### Whether `-E help` answers on the rented Linux machine

The engine identifier used above and in the first batch was read off `blender -b --factory-startup -E help`. The citation check found that command returning nothing at all from the 4.5 binary on the rented machine. Run again with each of the four Blenders on that machine, each with a sixty-second ceiling, Blender 5.1.2 printed:

```
=== 5.1 ===
Blender 5.1.2 (hash ec6e62d40fa9 built 2026-05-19 02:18:37)
Blender Engine Listing:
	BLENDER_EEVEE
	BLENDER_WORKBENCH
	CYCLES

real	0m0.711s
user	0m0.484s
sys	0m0.272s
EXIT STATUS 0
```

Across the four, **Blender 4.5 printed nothing at all and was stopped by the ceiling** — exit status 124, which is what `timeout` reports when it had to stop the command itself, and a third of a second of processor time used in a whole minute, so it was waiting rather than working. Blender 5.0, 5.1 and 5.2 each printed their engine list in under a second. This repeats the citation check's result exactly. Nothing in the answer to question 16 rests on it: the identifiers were read off the Mac, where the command answers in every version, and the 4.5 binary on the rented machine renders perfectly well in both Cycles and EEVEE, as the tests above show.

### What these tests could not settle, and why

- **Anything on Windows.** The runs used a Mac and a rented Linux machine; no Windows machine has run it, so nothing about Windows was tested and nothing here speaks for it. **Untried rather than untestable: a Windows machine can be rented, and every one this project has read reported a Server edition — see the equipment section of `Preparation/Blender Research/Method.md`.**
- **The automatic values Cycles picks when the Noise Threshold or Min Samples is zero.** Nothing a terminal can read prints either number; Cycles works them out inside the render and does not report them.
- **Whether the Cycles Time Limit is counted the same way when the render is on a graphics card.** Part-way through this round the rented machine's own automatic upgrade replaced its NVIDIA libraries with version 580.173 while the loaded driver stayed at 580.95.05, after which Cycles answered `ERROR CUDA cuInit: Unknown error` and offered only the processor. Putting that right needs the machine restarted, which was not done for one extra run. All twenty-four time-limit runs above were on the processor.
- **Whether the Time Limit applies per view layer when a scene has several.** Not tested; the timing runs used one view layer.
- **Whether the Layer Samples control appears in the Sampling panel only when some view layer carries an override.** Settling that needs eyes on a Blender window. The Creator was given nothing this round, and the rented Linux machine has no display server, so no window could be opened on either machine. Whether the *setting* keeps its value is a different question and is answered above.
- **Whether a sample count stored by a Blender old enough to have used squared samples survives being opened here.** The project has four Blenders, from 4.5 up, and no file saved by any older one. Only the part that could be run was run: a count this Blender saved itself comes back unchanged.

## THINGS A PERSON WITH BLENDER OPEN COULD SETTLE IN A FEW MINUTES

Pruned on 2026-08-10: everything the hand tests above answered has been removed. What is left:

1. **How many samples does each pixel really get with adaptive sampling on?** In View Layer
   properties, switch on the Cycles "Sample Count" debug pass (Passes ‣ Data ‣ Sample Count).
   Render with Max Samples 1024 and the Noise Threshold at its default 0.01, save as multilayer
   OpenEXR, and read that pass — it holds samples taken divided by the maximum. The hand tests
   above showed the per-pixel skipping in the clock, not in the pass, so the spread of counts
   across one picture is still unmeasured.

2. **Does a sample count stored by a much older Blender survive being opened here?** Item 6 of this
   list asked for a file saved by Blender 3.0 or earlier. The project has four Blenders, from 4.5 up,
   and no file saved by any older one, so this cannot be run with the equipment there is. The part
   that could be run was: a count this Blender saved itself comes back unchanged.

3. **What the render window shows when the time limit ends a render early.** The terminal's wording
   is recorded above and names the limit nowhere. What the interface's own status line says needs
   eyes on a window.

---

2026-08-10 — Claude Opus 5 (Claude Code) — Settings, Blender 5.1

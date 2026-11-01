# Settings — further questions (Blender 4.5)

Sources consulted, all read on **2026-07-30** (the snapshot date): Blender 4.5 manual (`manual_4.5`), Blender 4.5 Python API documentation (`api_4.5`), Blender 4.5 release notes (`release_notes_4.5`), and Cycles' own files (`cycles_4.5`).

I read only the question sheet and files inside Blender Sources/. I did not open any other part of this project.

---

## 16. When a render is started from outside Blender and given a sample count of its own, what decides how many samples it actually takes, and what else has a say in when it stops?

**How I read the question.** It was ambiguous in two small ways, and I answer the reading I think is meant.

- "Given a sample count of its own" — I read this as: the run that is starting has a sample count supplied to it from outside, rather than simply using whatever number the file happens to hold. The other reading would be "given a sample count that differs from the file's". Both readings lead to the same answer below, because in either case what matters is what happens to that number once the render begins. As the sheet instructs, I take no view on whether a sample count can be supplied from outside at all.
- "When it stops" — I read "it" as the render of a frame, i.e. what ends the sampling. The other reading would be the whole job across many frames; the sources say much less about that, and I flag it under Not covered.

**LABEL: mixed — see each part below.** The overall shape of the answer is: the number is a ceiling, not a target; several other settings can lower it, replace it, or end the render before it is reached.

---

### Part A — the manual's own name for the Cycles setting calls it a maximum

**LABEL: documented** for what the manual calls the setting; **inferred** for the step from that name to fewer samples actually being taken.

The manual's heading for the Cycles setting carries the word "Max". The description under it calls the number a count of paths per pixel and does not itself say the number can come out lower.

> "Render (Max) Samples
>
> The number of paths to trace per pixel in the final render.
> A higher number results in a cleaner image at the cost of a longer render time."

Source: manual, https://docs.blender.org/manual/en/4.5/render/cycles/render_settings/sampling.html — read 2026-07-30.

Going from that heading to "so fewer samples may actually be taken" is a step the reader supplies, so it is **inferred** here. What carries it is Parts B and D below, where the manual says in plain words that sampling can be cut short and that a subset can be rendered instead.

---

### Part B — adaptive sampling can stop each pixel below that number, and it is on by default

**LABEL: documented (what it does) plus observed (that it is on by default).**

The manual:

> "Adaptive Sampling
>
> If the Noise Threshold checkbox is enabled, Cycles will use adaptive sampling,
> cutting short the sampling process in areas that have become less noisy than the
> specified threshold value – in other words, not wasting time further refining
> areas that already look good enough."

> "Noise Threshold
>
> The threshold to decide whether to continue sampling a pixel.
> Typical values are in the range from 0.1 to 0.001, with lower values meaning better
> quality but longer render times.
> Setting this to 0 makes Cycles guess an automatic value based on the total sample count."

> "Min Samples
>
> The minimum number of samples a pixel receives before adaptive sampling is applied.
> When set to 0 (default), Cycles automatically picks a value determined by the Noise Threshold."

Source: manual, https://docs.blender.org/manual/en/4.5/render/cycles/render_settings/sampling.html — read 2026-07-30.

That this is switched on unless somebody turns it off is **observed** in Cycles' own files:

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

File: `Blender Sources/cycles_4.5/properties.py`, inside `class CyclesRenderSettings`, lines 569–590 — read 2026-07-30.

So the count is decided pixel by pixel, not for the picture as a whole. Cycles' own kernel files say the same in as many words:

> `/* Check whether the pixel has converged and should not be sampled anymore. */`

> `/* Determines whether to continue sampling a given pixel or if it has sufficiently converged. */`

File: `Blender Sources/cycles_4.5/source/kernel/film/adaptive_sampling.h`, lines 11 and 27 — read 2026-07-30. **Observed.**

Taken together: with adaptive sampling on, a pixel receives at least the Min Samples number and at most the given sample count, and the noise threshold decides where in between it lands. The step from the quoted words to "at least Min Samples, at most the given count" is small but it is a step, so treat that sentence as **inferred**.

There is also a second, viewport-only copy of all three of these settings (`use_preview_adaptive_sampling`, `preview_adaptive_threshold`, `preview_adaptive_min_samples`, same file, lines 591–612), whose descriptions end "for viewport renders". That those three exist under those names, with those descriptions, is **observed**. That they have no say in a final render is a step past what the file says — the file names them and does not say what a render does with them — so treat that as **inferred**. The same step is marked inferred again under Outside the interface below.

---

### Part C — a view layer can override the number, and a scene setting decides whether the override is honoured

**LABEL: documented.**

The API documentation:

> "samples
>
> Override number of render samples for this view layer, 0 will use the scene setting
>
> Type:
>
> int in [0, inf], default 0"

Source: API documentation, https://docs.blender.org/api/4.5/bpy.types.ViewLayer.html — read 2026-07-30.

The manual, on the scene-level setting that governs those overrides:

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
> Allow the overrides as long as they don’t exceed the scene-level sample count.
>
> Ignore:
>
> Ignore the overrides."

Source: manual, https://docs.blender.org/manual/en/4.5/render/cycles/render_settings/sampling.html — read 2026-07-30.

And on the per-layer control itself:

> "Samples
>
> View layer samples to override the scene samples.
> Controlled by the layer samples in the Sampling panel."

Source: manual, https://docs.blender.org/manual/en/4.5/render/layers/introduction.html — read 2026-07-30.

Which of the three modes is in force by default is **observed**:

> ```
> enum_use_layer_samples = (
>     ('USE', "Use", "Per render layer number of samples override scene samples"),
>     ('BOUNDED', "Bounded", "Bound per render layer number of samples by global samples"),
>     ('IGNORE', "Ignore", "Ignore per render layer number of samples"),
> ```

> ```
>     use_layer_samples: EnumProperty(
>         name="Layer Samples",
>         description="How to use per view layer sample settings",
>         items=enum_use_layer_samples,
>         default='USE',
>     )
> ```

File: `Blender Sources/cycles_4.5/properties.py`, lines 68–72 and 553–558 — read 2026-07-30.

Putting the two together: with the default mode in force, a view layer carrying a non-zero override stands in place of the scene number. That joins two sources with a step between them, so treat it as **inferred**. What number a render then actually uses is on the hand-test list below.

---

### Part D — Sample Subset replaces the number of samples actually rendered

**LABEL: documented.**

> "Sample Subset
>
> Only render a subset of the samples."

> "Length
>
> The number of samples in the subset. While this overrides (Max) Samples in terms of the
> samples that will get rendered, it’s still important to set (Max) Samples to the total number
> of samples that will be rendered across all subsets – otherwise, the subsets will have
> incompatible noise and combining them will give a worse result."

> "Offset
>
> The 0-based index of the first sample in the subset."

Source: manual, https://docs.blender.org/manual/en/4.5/render/cycles/render_settings/sampling.html — read 2026-07-30.

**Observed** — it is off unless switched on, and its defaults:

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

File: `Blender Sources/cycles_4.5/properties.py`, lines 497–515 — read 2026-07-30.

---

### Part E — the accepted range of the setting itself

**LABEL: mixed — observed for Cycles, documented for EEVEE.**

> ```
>     samples: IntProperty(
>         name="Samples",
>         description="Number of samples to render for each pixel",
>         min=1, max=(1 << 24),
>         default=4096,
>     )
> ```

File: `Blender Sources/cycles_4.5/properties.py`, lines 482–487 — read 2026-07-30. `1 << 24` is 16,777,216.

For EEVEE the API documentation states the range instead:

> "taa_render_samples
>
> Number of samples per pixel for rendering
>
> Type:
>
> int in [1, inf], default 64"

Source: API documentation, https://docs.blender.org/api/4.5/bpy.types.SceneEEVEE.html — read 2026-07-30. **Documented.**

This part overlaps question 15, which asks directly what happens to an out-of-range value; I record the ranges here only because they bound what number can end up in the setting at all, and say nothing here about what Blender does with a value outside them.

---

### Part F — the engine decides what a sample count even means

**LABEL: documented** for what each engine's page says; **not in the sources** for what EEVEE's page does not mention.

EEVEE has a plain render sample count:

> "Render
>
> Samples
>
> The number of samples to use in the final render."

Source: manual, https://docs.blender.org/manual/en/4.5/render/eevee/render_settings/sampling.html — read 2026-07-30.

That page names no time limit and no adaptive sampling. I searched the whole of it for "time limit", "adaptive", "threshold" and "noise"; the only threshold on it is "Light Threshold", described as "Minimum light intensity for a light to contribute to the lighting." Most of those terms were also searched across the whole manual snapshot — see Where I looked. That is an absence rather than a statement, so it is **not in the sources** that EEVEE has neither, not documented.

Workbench does not take a free number at all — it offers a fixed list:

> "Multisample
>
> The scene will be rendered multiple times with a slight offset.
> The anti-aliasing will be gathered from the multiple renders.
> The number of samples are predefined so it uses the best distribution of the samples.
>
> 5, 8, 11, 16, 32"

Source: manual, https://docs.blender.org/manual/en/4.5/render/workbench/sampling.html — read 2026-07-30.

---

### Part G — Cycles' own files hold a versioning pass that can write the sample count

**LABEL: observed for what the files say, with two inference steps marked.**

Cycles' own files contain a versioning pass that, for old enough files, writes the sample count itself. When that pass runs is the first inferred step below.

> ```
> @persistent
> def do_versions(self):
> ```

> ```
>     # We don't modify startup file because it assumes to
>     # have all the default values only.
>     if not bpy.data.is_saved:
>         return
> ```

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

File: `Blender Sources/cycles_4.5/version_update.py`, lines 66–67, 92–95 and 235–244 — read 2026-07-30. Other branches in the same file set `cscene.samples` outright (lines 150–151 and 220–221) when the file does not carry the property.

The function is added to a list by this line, and that the line exists is **observed**:

> `    bpy.app.handlers.version_update.append(version_update.do_versions)`

File: `Blender Sources/cycles_4.5/__init__.py`, line 156 — read 2026-07-30.

First inferred step. Going from that line to "this runs when a file is loaded" is a step past the words: the line shows the function being added to a list carrying that name, and nothing I read says when the list is called. Treat it as **inferred**.

Second inferred step. If it does run as the file is loaded, then a sample count supplied to the scene *after* the file is open is not touched by it, while one already in the file may be. The sources do not spell that ordering out either, so this is **inferred** and it rests on the step above.

---

### Part H — what else has a say in when it stops

**Time Limit. LABEL: documented.**

> "Time Limit
>
> Stops rendering if the time exceeds the limit, even if the desired sample count
> hasn’t been reached yet. Setting this value to zero disables the limit."

> "Note
>
> The time limit does not include pre-render processing time, only render time."

Source: manual, https://docs.blender.org/manual/en/4.5/render/cycles/render_settings/sampling.html — read 2026-07-30.

It is off unless set — **observed**:

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

File: `Blender Sources/cycles_4.5/properties.py`, lines 517–525 — read 2026-07-30.

**Adaptive sampling and the sample subset**, as in Parts B and D, can each mean fewer samples are rendered in one run than the given count.

**A render job can be cancelled.** The manual mentions this once, and in the context of animation frames rather than samples:

> "For example, when rendering animations to image file formats the render job can be canceled
> and resumed at the last rendered frame by changing the frame range."

Source: manual, https://docs.blender.org/manual/en/4.5/render/output/introduction.html — read 2026-07-30. **LABEL: documented**, but note it says nothing about how a render is cancelled, nor whether that is possible when Blender is driven from outside its own window.

---

### Not covered

- What ends a whole job across many frames, as against what ends the sampling of one frame. That is the second reading of the question set out at the top, and it is the one I did not answer. What I found bearing on it is the cancellation note in Part H, plus the two items below about the Time Limit and about when the sample count is read. Nothing else.
- Whether the render as a whole ends early once every pixel has converged. The manual says only that sampling is cut short "in areas"; it does not say what happens when there are no areas left to refine. I did not find that stated anywhere.
- Whether the Time Limit applies to each frame or to a whole animation. The manual says "Stops rendering if the time exceeds the limit" and does not say which.
- What "pre-render processing time" covers, beyond the note that it is excluded from the time limit.
- How the automatic values are picked when Noise Threshold or Min Samples is left at zero. The manual says Cycles "guess[es] an automatic value" and "automatically picks a value"; it does not give the rule, and the part of Cycles that would contain it is not among the files supplied (the folder holds the kernel and utility files, not the session code).
- Whether the sample count is read once at the start of a job or afresh at each frame.
- Grease Pencil's separate SSAA sample count, and path guiding's Training Samples, are sample counts of their own; neither is the render's sample count and I did not pursue them.
- What happens if a number outside the accepted range is supplied. That is question 15.

### Outside the interface

**LABEL: mixed.**

Every setting named above is a scene or view-layer setting held in the file, and the manual states that a command-line animation render uses the file's settings:

> "-a
>
> Render the whole animation using all the settings saved in the blend-file."

Source: manual, https://docs.blender.org/manual/en/4.5/advanced/command_line/render.html — read 2026-07-30. **Documented.**

Two differences I can point to:

1. The viewport copies of these settings have no say. "Viewport (Max) Samples" is described as "The number of paths to trace per pixel in the 3D Viewport (when using the Rendered shading mode)", and Denoising's "Start Sample" as "Sample at which to start denoising in the 3D Viewport" (manual, sampling page, read 2026-07-30). Since a render driven from outside Blender's window has no 3D Viewport, these do not apply — that last clause is **inferred**, one step from the quoted words.
2. Cycles' own command-line options do not include a sample count. The manual's "Cycles Render Options" section holds two entries, `--cycles-device` and `--cycles-print-stats` (https://docs.blender.org/manual/en/4.5/advanced/command_line/arguments.html, read 2026-07-30), and Cycles' own argument parser defines exactly those two and nothing else — **observed**, `Blender Sources/cycles_4.5/engine.py`, lines 8–21. The same page also carries `--debug-cycles` ("Enable debug messages from Cycles"), under Debug Options rather than Cycles Render Options; it sets no sample count either. I report this as what the sources show and draw no conclusion from it here. *(Citation check, 2026-08-10: narrowed from "the manual's full list of them is two entries" and added the `--debug-cycles` entry, which sits elsewhere on the same page.)*

Beyond those, the sources do not say that any of the deciding factors behave differently when Blender is driven from outside. I found nothing stating that adaptive sampling, the time limit, the sample subset or the layer-samples mode behave differently in a background render.

**Settled by test on 2026-08-10. LABEL: observed.** Every one of the fifteen tests recorded under *Hand-test results* below was run from a terminal with no window at all, on two computers, and each of these settings did what this answer describes: the sample count, adaptive sampling, the time limit, the sample subset and the Layer Samples setting all took effect in a render started from outside Blender. Two of the inferred steps above are now observed rather than inferred. The viewport copies of these settings have no bearing on such a render — changing all four of them left the reported count and every pixel of the picture unchanged. And these settings do travel with the file: eleven of them were saved and read back unchanged. What is still not tested is the comparison with a render started from inside Blender's own window, which neither machine could run this round.

### Operating system

**LABEL: not in the sources** for what the documentation says; **observed** for macOS and Linux since 2026-08-10.

Nothing I read ties any of this to an operating system. I searched the manual, the API documentation and the release notes for "Windows", "macOS" and "Linux" alongside the sampling terms and found no statement that sample counts, adaptive sampling, the time limit or the sample subset differ by platform. The nearest platform-dependent thing on the same manual page is the choice of denoiser ("Only available on NVIDIA GPUs when configured in the Cycles Render Device user preferences") and, on the command-line page, the list of valid devices, which includes `METAL`. Both are about the device, not about how many samples are taken. The 4.5 Cycles release notes list platform-specific issues (NVIDIA driver minimum, AMD stability, Intel packages) and none of them concerns sampling (https://developer.blender.org/docs/release_notes/4.5/cycles/, read 2026-07-30).

**Settled by test on 2026-08-10, for two operating systems out of three. LABEL: observed.** All fifteen tests below were run on macOS 26.5.2 on Apple silicon and on Ubuntu 22.04.5 LTS on an AMD processor, and the two machines returned the same sample count in every single test. Where a figure differed between them it was a time or a count reached under a clock, never a count reached under a sample setting. **Windows is untested and stays untested** — no Windows machine has run it — so nothing here speaks for it.

---

## Where I looked

Searches were run across the whole of `manual_4.5`, `api_4.5` and `release_notes_4.5` at once, and across all of `cycles_4.5`.

Terms searched: `samples`, `sample count`, `max samples`, `Max Samples`, `Adaptive Sampling`, `adaptive`, `noise threshold`, `Noise Threshold`, `stop sampling`, `converge`, `Time Limit`, `time limit`, `Sample Subset`, `resumable`, `sample override`, `Sample Overrides`, `override the scene samples`, `cancel`, `Esc`, `--samples`, `cycles-samples`, `cycles_samples`, `--cycles`, `argv`, `time_limit`, `start_sample`, `num_samples`, `sample_offset`, `subset`, `use_layer_samples`.

Pages read in full:

- https://docs.blender.org/manual/en/4.5/render/cycles/render_settings/sampling.html — the one page that carries most of this answer. It has no images.
- https://docs.blender.org/manual/en/4.5/render/eevee/render_settings/sampling.html
- https://docs.blender.org/manual/en/4.5/render/workbench/sampling.html
- https://docs.blender.org/manual/en/4.5/render/layers/introduction.html (has two images, `_images/render_layers_introduction_list.png` and `_images/render_layers_introduction_viewlayer-collection.png`; neither shows the Sampling panel, so I did not rely on them)
- https://docs.blender.org/manual/en/4.5/advanced/command_line/render.html
- https://docs.blender.org/manual/en/4.5/advanced/command_line/arguments.html — the Render Options and Cycles Render Options sections in full
- https://docs.blender.org/manual/en/4.5/render/output/introduction.html
- https://developer.blender.org/docs/release_notes/4.5/cycles/ — in full; it says nothing about sampling

API pages checked: `bpy.types.ViewLayer`, `bpy.types.SceneEEVEE`, `bpy.ops.render`, `bpy.ops.wm`. Ruled out: I searched every API page for top-level entries named `samples`, `time_limit`, `use_adaptive_sampling`, `adaptive_threshold` and `sample_offset`. The only `samples` entry that concerns a render's sample count is `bpy.types.ViewLayer.samples`; the others are unrelated (blur nodes, ambient occlusion, bevel, shader effects). Cycles' scene-level sampling settings do not appear in the API documentation at all, which is why Parts A–E lean on Cycles' own files.

Cycles files read: `engine.py` (argument parser, lines 8–41), `properties.py` (`CyclesRenderSettings`, lines 376–620, and `CyclesRenderLayerSettings`, lines 1494 onwards), `ui.py` (sampling panels, lines 225–485), `version_update.py` (whole file), `__init__.py` (registration, line 156), `operators.py` (scanned; only `merge_images` is relevant and only as the tool named in the Sample Subset instructions), and `source/kernel/film/adaptive_sampling.h`. The `source` folder holds only kernel and utility files; the part of Cycles that runs a render session, which is where the time limit and the automatic threshold rules would live, is not present.

Screenshots of the Sampling panel: I searched `manual_4.5/_images` for filenames containing "sampling" and found only `render_materials_legacy-textures_types_image-movie_image-sampling-panel.png`, which is about image textures and not about render sampling. So there is no picture of the Cycles Sampling panel in the manual snapshot to check the wording against.

---

## Hand-test results — 2026-08-10

**LABEL: observed**, throughout this section.

Fifteen tests were run in Blender 4.5.12 on two computers, on 2026-08-10. **The Creator's own Mac** — a MacBook Pro with an M1 Pro chip, macOS 26.5.2, eight cores, 16 GB of memory, Apple's own graphics and no second card. **A rented Linux machine** — Ubuntu 22.04.5 LTS on kernel 6.8, one NVIDIA GeForce RTX 4060 Ti with 16 GB on driver 580.95.05, twenty-three cores of an AMD EPYC 7K62, 98 GB of memory, and no display server at all. Every test was run on both, and the two machines are two results: where they agreed, this says so; nothing is folded into one sentence.

**How they were run.** Each Blender was pointed at a throwaway settings folder of its own, made before use, so no real preferences were involved. Every render was on the processor rather than the graphics card, because a fresh settings folder switches no graphics device on and the scene asks for `CPU`. The rented machine is somebody else's hardware and a share of a bigger chip, so its times are its own and are never read against the Mac's.

**The scene**, built by Blender 4.5.12 itself from a script so that no file saved by another Blender is involved: 320 by 240, Cycles on the processor, a plane, a rough metal sphere, a glass cube, one large area light and a dark world — noisy enough that sampling is visible in the clock. Renders were started as `blender -b <file> --python-expr "<settings>" -o <folder> -F PNG -f 1`. *(Citation check, 2026-08-10: the script itself is not here, and without it this scene cannot be rebuilt exactly. Repeating these tests on a scene built afresh to this description returned every count that does not depend on how noisy the scene is — 4096/4096, 32/4096, 256/256, 8/8, the six view-layer results, the EEVEE counts — and different figures for everything reached under a clock. A test that turns on a clock needs its scene supplied, not described.)* *(Second batch of hand tests, 2026-08-10: the script is now supplied in full, under “Second batch of hand tests” further down this document, and the four figures that turn on a clock were run again on it.)*

**How the answer was read.** Blender prints a running `Sample n/N` line for each view layer of each frame. The last such line with a non-zero `n` is the number of samples that run counted to, and `N` is the number it was counting towards. In 4.5 the line reads, for example, `Fra:1 Mem:16.86M (Peak 20.38M) | Time:00:25.47 | Mem:5.88M, Peak:5.88M | Scene, ViewLayer | Sample 4096/4096`. One line reading `Sample 0/N` is printed immediately before `Finished` and is not a result. EEVEE prints `Rendering n / N samples` instead.

### A frame does end before Max Samples when its pixels converge, and the count is decided pixel by pixel

Three renders of one frame at Max Samples 4096 with Time Limit 0, changing only the sampling.

On the Mac: with adaptive sampling off, `Sample 4096/4096` and `Time: 01:35.95`. With adaptive sampling on at the default threshold 0.01, `Sample 4096/4096` and `Time: 00:25.52`. With adaptive sampling on at a threshold of 0.5, the run ended at `Sample 32/4096` and `Time: 00:01.03`, printed in full as:

```
Fra:1 Mem:8.87M (Peak 9.10M) | Time:00:00.00 | Mem:0.32M, Peak:0.32M | Scene, ViewLayer | Sample 0/4096
Fra:1 Mem:14.51M (Peak 14.51M) | Time:00:00.02 | Remaining:01:28.70 | Mem:5.88M, Peak:5.88M | Scene, ViewLayer | Sample 1/4096
Fra:1 Mem:14.51M (Peak 16.27M) | Time:00:00.85 | Remaining:01:47.69 | Mem:5.88M, Peak:5.88M | Scene, ViewLayer | Sample 32/4096
Fra:1 Mem:16.86M (Peak 20.38M) | Time:00:00.98 | Remaining:02:04.15 | Mem:5.88M, Peak:5.88M | Scene, ViewLayer | Sample 0/4096
Fra:1 Mem:16.86M (Peak 20.38M) | Time:00:00.98 | Remaining:02:04.15 | Mem:5.88M, Peak:5.88M | Scene, ViewLayer | Finished
Time: 00:01.03 (Saving: 00:00.05)
```

On the rented Linux machine the same three runs gave the same three counts — `4096/4096`, `4096/4096`, `32/4096` — at `Time: 00:43.72`, `Time: 00:12.46` and `Time: 00:00.90`. The two machines agreed on every count.

This answers two things this document left open. The whole frame does end before the maximum once its pixels have converged, rather than only individual pixels stopping. And with adaptive sampling on at the default threshold the reported count is unchanged while the time falls to about a quarter, which is the per-pixel skipping showing up in the clock rather than in the count.

Blender 4.5 does carry a Cycles sample-count pass, which this document never established: printing the Cycles view-layer settings gave `PASS_SAMPLE_ATTRS ['denoising_store_passes', 'pass_debug_sample_count', 'use_denoising']` on both machines.

### The Cycles Time Limit is spent on each frame, not across the whole job

Three frames rendered in one launch with `-s 1 -e 3 -a`, Time Limit 10 seconds, Max Samples 4096, adaptive sampling off. Three repeats on each machine.

On the Mac the whole run took 34.12, 33.11 and 33.10 seconds, and each run wrote three PNG files. The per-frame `Time:` lines in the first repeat were `00:10.75`, `00:11.50` and `00:10.81`, and each of the three frames reached `Sample 513/4096`.

On the rented Linux machine, which was idle when this was timed — `load average: 0.00, 0.02, 1.30`, no user process above 4% of one core, and the graphics card reporting `0MiB / 16380MiB` used — the whole run took 32.91, 32.44 and 33.14 seconds and also wrote three files each time. The per-frame times in the first repeat were `00:10.31`, `00:11.04` and `00:11.03`, and the frames reached `Sample 946/4096`, `Sample 1025/4096` and `Sample 1025/4096`.

Six runs, three on each machine, and not one stopped the job after ten seconds. The item under *Not covered* saying this could not be settled no longer holds for either operating system.

### Which number a render counts to when a view layer carries its own

Two halves, because an override below the scene count cannot tell `Bounded` from `Use`. Adaptive sampling off and Time Limit 0 throughout, and the scene's count supplied at launch by `--python-expr` rather than saved in the file.

With a second view layer whose Samples is 64 against a scene count of 1024: under `USE` the first layer reported `Sample 1024/1024` and the second `Sample 64/64`; under `BOUNDED`, `1024/1024` and `64/64`; under `IGNORE`, `1024/1024` and `1024/1024`.

With a second view layer whose Samples is 1024 against a scene count of 256: under `USE`, `Sample 256/256` and `Sample 1024/1024`; under `BOUNDED`, `256/256` and `256/256`; under `IGNORE`, `256/256` and `256/256`.

Both machines gave these six results identically. So a view layer's own number does stand in place of a scene count supplied from outside under `USE`, is capped by it under `BOUNDED`, and is thrown away under `IGNORE`.

**One thing about the second half on the Mac.** Those three runs happened between 15:56 and 15:59 on 2026-08-10, and the Creator's own Blender 5.2 was open on that machine from 15:55:59 onwards — so for that one entry, and no other in this set, a second Blender was running beside the test. It changes nothing this entry rests on: the answer is the counts, the counts do not depend on how busy the machine is, and the same six counts came back from the rented Linux machine, where nothing else was running. The times in the excerpt above were taken under that load, and nothing here is built on them.

### Sample Subset decides the count that is rendered, and a time limit still cuts it short

Max Samples 2048 throughout. On the Mac: with the subset off, `Sample 2048/2048` at `Time: 00:43.22`. With the subset on at Offset 0 and Length 256, `Sample 256/256` at `Time: 00:05.64` — the progress line counts against 256, not against 2048. With adaptive sampling also on at 0.01, `Sample 256/256` at `Time: 00:05.20`. With a 2-second Time Limit added instead, `Sample 97/256` at `Time: 00:02.20`.

On the rented Linux machine the same four runs gave `2048/2048` at `00:21.92`, `256/256` at `00:03.05`, `256/256` at `00:02.91` and `Sample 193/256` at `00:02.43`. The first three counts matched the Mac's; the fourth is a clock reading and differs with the machine.

### The Cycles viewport sampling settings have no part in a render with no window

Render sampling held at Max Samples 200 with adaptive sampling on at 0.01, and the viewport numbers changed between two runs from `preview_samples` 4, viewport adaptive on at threshold 0.5 and viewport Min Samples 1, to `preview_samples` 3000, viewport adaptive off at 0.001 and viewport Min Samples 512.

Both runs reported `Sample 200/200`, on both machines. The two pictures were compared pixel by pixel rather than byte by byte, and came back identical on both machines:

```
PIXELS max_difference 0.0 channels_differing 0 of 307200
```

This settles the inferred step in *Part B* and in *Outside the interface* above: on these two computers the viewport copies of these settings have no bearing on a render started with no window.

### The sampling settings survive a save and a reopen unchanged

Eleven Cycles values and EEVEE's render sample count were set, saved to a new file, and read back by opening that file in the same Blender. The reply was the same on both machines:

```
READBACK CYCLES 137 42.0 False 0.004999999888241291 9 BOUNDED True 3 11 64 77
```

In order: the engine, Max Samples 137, Time Limit 42.0, adaptive sampling off, adaptive threshold 0.005, Min Samples 9, Layer Samples `BOUNDED`, sample subset on, subset offset 3, subset length 11, the view layer's own Samples 64, EEVEE's Render Samples 77. Nothing was rewritten as the file loaded. The threshold reads back as `0.004999999888241291` because 0.005 has no exact form in that kind of number; it is the same value.

This bears on *Part G*. It does not test a file saved by a Blender old enough to have used squared samples — no such Blender is available to this project — but it does show that for a file this Blender saved itself, the count comes back as it went in.

### A count supplied at launch is the number the render counts to, if the argument comes first

A file whose stored Max Samples is 4096, with 32 supplied at launch. With `--python-expr` placed before `-f 1`, the run reported `Sample 32/32`. With the same `--python-expr` placed after `-f 1`, it reported `Sample 4096/4096` and took `Time: 01:35.42`. The same on both machines.

### What else can stop a render started from outside

A 4096-sample frame with the Time Limit off was started with no window, left for eight seconds, then interrupted. Three ways, on both machines.

An interrupt signal: Blender printed one extra line and stopped at the next sample report, wrote no file, and exited 1.

```
Fra:1 Mem:13.05M (Peak 13.05M) | Time:00:00.03 | Remaining:01:31.59 | Mem:4.42M, Peak:4.42M | Scene, ViewLayer | Sample 1/4096
Sent an internal break event. Press ^C again to kill Blender
Fra:1 Mem:13.05M (Peak 13.05M) | Time:00:22.61 | Remaining:01:07.73 | Mem:4.42M, Peak:4.42M | Scene, ViewLayer | Sample 1025/4096
Fra:1 Mem:15.39M (Peak 18.91M) | Time:00:22.62 | Remaining:01:07.73 | Mem:4.42M, Peak:4.42M | Scene, ViewLayer | Sample 0/4096
Fra:1 Mem:15.39M (Peak 18.91M) | Time:00:22.62 | Remaining:01:07.73 | Mem:4.42M, Peak:4.42M | Scene, ViewLayer | Cancelled
Blender quit
```

The output folder was empty afterwards. On the rented Linux machine the same three lines came back, with the stop at `Time:00:10.88` and also at `Sample 1025/4096`. On both machines the stop landed on the same sample number rather than at the same moment.

A terminate signal: nothing further was printed, no file was written, and the shell reported exit status 143. The last line of the log is an ordinary `Sample 1/4096`. The same on both machines.

`--disable-abort-handler` with an interrupt signal: identical to the plain interrupt — the same break line, the same `Cancelled`, exit status 1, no file. The same on both machines. So this document's note that `--disable-abort-handler` has nothing said about it stands: it changed nothing here.

### EEVEE takes exactly the number it is given, except where two features round it up

The engine identifier came from Blender itself: `blender -b --factory-startup -E help` printed `BLENDER_EEVEE_NEXT` in 4.5, alongside `BLENDER_WORKBENCH` and `CYCLES`.

With Render Samples 64 and Cycles' Max Samples left at 4096 in the same file, the run reported `Rendering 64 / 64 samples`. So the Cycles count is not read when EEVEE renders.

With Render Samples 64 and motion blur on with 5 steps, it reported `Rendering 65 / 65 samples`. With Render Samples 16 and depth of field with Jitter Camera on, `Rendering 19 / 19 samples`; with Jitter Camera off, `Rendering 16 / 16 samples`. **Both of these features exist and behave this way in 4.5**, which this document does not mention at all.

Printing the whole EEVEE settings block turned up no time limit in 4.5. Searched for "time" and "limit", the only entries were:

```
EEVEEPROP gi_diffuse_bounces | Diffuse Bounces | Number of times the light is reinjected inside light grids, 0 disable indirect diffuse light
EEVEEPROP motion_blur_steps | Motion steps | Controls accuracy of motion blur, more steps means longer render time
EEVEEPROP taa_samples | Viewport Samples | Number of samples, unlimited if 0
```

All four EEVEE runs came out the same on both machines. On the rented Linux machine, which has no display server at all, every EEVEE run worked and wrote its image, printing three copies of this line first:

```
EGL Error (0x3009): EGL_BAD_MATCH: Arguments are inconsistent (for example, a valid context requires buffers not supplied by a valid surface).
```

*(Citation check, 2026-08-10: this line did not come back. Repeating the EEVEE runs on the same rented machine today — now on graphics driver 580.173.02, where these tests were recorded at 580.95.05 — printed no EGL line at all, while every EEVEE run still worked and still wrote its image. Both results stand; the question goes back as a test.)* *(Second batch of hand tests, 2026-08-10: run a third time on that machine, still on driver 580.173.02 — again no EGL line, in any of the four versions, and again every EEVEE render worked and wrote its image. All three runs stand; see “Second batch of hand tests” below for the whole log.)*

### A view layer's sample override does not change an EEVEE render in 4.5

With EEVEE's Render Samples at 256 and a second view layer carrying its own Samples of 8, **both layers rendered 256 samples**:

```
Fra:1 Mem:42.43M (Peak 42.49M) | Time:00:01.27 | Rendering 256 / 256 samples
Fra:1 Mem:44.91M (Peak 44.91M) | Time:00:01.46 | Rendering 1 / 256 samples
Fra:1 Mem:44.85M (Peak 44.91M) | Time:00:02.78 | Rendering 256 / 256 samples
LAYERS [('ViewLayer', 0), ('Second', 8)]
```

The same on both machines. This is recorded here as what Blender 4.5.12 did, and nothing about other versions is stated in this document.

### Workbench reports no sample count at all

With the engine set to Workbench, the run printed no `Sample n/N` line and no `Rendering n / N samples` line — only `Start rendering`, `Engine: Workbench`, `Saved:` and `Time: 00:00.13`. The setting printed:

```
RENDER_AA '8'
RENDER_AA ITEMS ['OFF', 'FXAA', '5', '8', '11', '16', '32']
CYCLES SAMPLES STILL 4096
```

The Cycles count of 4096 sitting in the same file changed nothing. The same on both machines.

### A command-line render ignores "Render Single Layer", so a second layer's own count is in play

With `use_single_layer` set to `True`, the first layer active and a second view layer carrying Samples 64 against a scene count of 256, both layers rendered: `Sample 256/256` and `Sample 64/64`, with `SINGLE_LAYER True` printed alongside. The same on both machines. In background mode there is no window, so the instruction meant to make the first layer active had no window to act on; it printed nothing and did not error, and both layers rendered regardless.

### The Noise Threshold's default is not the automatic value it uses at zero

Adaptive sampling on, Min Samples 0, Time Limit 0. Every one of these runs reached its maximum; what separates them is the time and the picture.

On the Mac: threshold 0.01 at Max Samples 1024 gave `Sample 1024/1024` at `Time: 00:09.90`; threshold 0 at the same maximum gave `Sample 1024/1024` at `Time: 00:22.63`. Threshold 0 at Max Samples 128 gave `Sample 128/128` at `Time: 00:02.78`, and at 4096 gave `Sample 4096/4096` at `Time: 01:17.79`.

On the rented Linux machine the same four runs gave the same four counts at `Time: 00:05.17`, `Time: 00:11.17`, `Time: 00:01.59` and `Time: 00:39.32`.

The two pictures at Max Samples 1024, threshold 0.01 against threshold 0, are different pictures on both machines — `PIXELS max_difference 0.027450978755950928 channels_differing 24778 of 307200` on the Mac and `channels_differing 24906 of 307200` on the Linux machine.

Two things this does not settle. It does not give the automatic value — nothing the run prints names it. And it does not settle whether that value moves with the sample count, because the cost of one sample changes with the count on its own, so the three threshold-zero runs cannot be read against each other.

### A Min Samples above Max Samples changes nothing about the count reached

With Max Samples 8 and Min Samples 64, both values were accepted as set — the run printed `SET 8 64` — and the render reported `Sample 8/8` at `Time: 00:00.36`. The control run with Min Samples 0 printed `SET 8 0` and also reported `Sample 8/8` at `Time: 00:00.36`. The same on both machines.

### `--cycles-print-stats` reports no sample count

The run reported `Sample 64/64` as usual and then printed a block beginning `Render statistics:`, holding mesh geometry memory, a per-kernel timing breakdown, and shader and object statistics. Its closing lines, from the Mac:

```
Shader statistics:
  Floor                           : 3.83s (Relative cost: 1.02)
  Glass                           : 2.20s (Relative cost: 0.99)
  Metal                           : 0.71s (Relative cost: 0.95)
Object statistics:
  Plane                           : 3.83s (Relative cost: 1.02)
  Cube                            : 2.20s (Relative cost: 0.99)
  Sphere                          : 0.71s (Relative cost: 0.95)
```

No figure in that block names a sample count. The same on both machines.

### Second batch of hand tests — 2026-08-10: the scene supplied, and three more tests

**LABEL: observed**, throughout this section.

The citation check of these findings on 2026-08-10 found that the section above **describes** the test scene without **supplying** it, so four of its figures — the ones a clock decides — could not be got again by anybody who was not there. A second, short batch of hand tests was run the same day to close that, along with two other things about the record. Everything above stands unchanged; nothing here replaces it.

**The scene, supplied.** This is the script that built the test scene, in full. Blender 4.5.12 ran it itself, so no file saved by another Blender is involved. Save it as `make_scene.py` and run it as `blender -b --factory-startup --python make_scene.py -- <somewhere>/scene.blend`:

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

**The count reached under a 2-second time limit on a 256-sample subset.** Max Samples 2048, adaptive sampling off, subset on at Offset 0 and Length 256, Time Limit 2 seconds. On the Mac the record gives `Sample 97/256` and this run gave `Sample 97/256` — the same. On the rented Linux machine the record gives `Sample 193/256` and this run gave `Sample 193/256` — the same.

**The per-frame counts under a 10-second limit across three frames.** Max Samples 4096, adaptive sampling off, Time Limit 10 seconds, frames 1 to 3 in one launch with `-s 1 -e 3 -a`. On the Mac the record gives `513`, `513`, `513` and this run gave `513`, `513`, `513` — the same — in 32.780 s of wall clock, with three PNG files written. On the rented Linux machine the record gives `946`, `1025`, `1025` and this run gave `1025`, `949`, `1025` — different — in 33.033 s, again with three files written.

**The channels differing between a noise threshold of 0.01 and one of 0.** Max Samples 1024, adaptive sampling on, Min Samples 0, Time Limit 0; both renders reached `Sample 1024/1024` and the two pictures were compared channel by channel. On the Mac:

```
PIXELS max_difference 0.027450978755950928 channels_differing 24784 of 307200
```

against a recorded `channels_differing 24778 of 307200`. On the rented Linux machine:

```
PIXELS max_difference 0.027450978755950928 channels_differing 24907 of 307200
```

against a recorded `channels_differing 24906 of 307200`.

**The sample number an interrupt lands on after eight seconds.** Max Samples 4096, Time Limit 0, the render started in the background, left for eight seconds, then sent an interrupt. On the Mac it stopped at `Sample 1025/4096` at `00:22.48`; on the rented Linux machine at `Sample 1025/4096` at `00:10.76`. The record gives `Sample 1025/4096` on both machines. Both runs wrote no file and exited 1, as recorded. This run reached the same sample number as the record on the Mac.

**What the four figures together say.** Supplying the scene was most of what was missing and it was not all of it. Across all four versions and both machines — thirty-two figures in all — the picture comparison came back within six channels of the record everywhere and exactly right in three cases, where the citation check's repeat on a scene built from the written description had been out by more than a thousand; the interrupt landed on the recorded sample number in seven cases out of eight; the two-second subset count came back exactly in five cases out of eight; and the ten-second per-frame counts came back exactly in two cases out of eight. The looser a figure's grip on the clock, the better it repeats. **No claim these figures were quoted in support of changed.** A time limit still cut the subset short, ten seconds was still spent on each frame rather than once for the job, a threshold of 0.01 and a threshold of 0 still made different pictures, and an interrupt still stopped the render without writing a file.

#### The EGL line on the rented Linux machine, a third time

One EEVEE render in Blender 4.5.12 on that machine, engine `BLENDER_EEVEE_NEXT`, Render Samples 64, with the whole log kept and the driver read at the same time. The driver answered:

```
580.173.02
NVRM version: NVIDIA UNIX Open Kernel Module for x86_64  580.173.02  Release Build  (dvs-builder@U22-I3-AK02-24-4)  Tue Jun 23 08:17:01 UTC 2026
```

and the render's whole log, word for word, was:

```
Blender 4.5.12 LTS (hash 84afd5f785f7 built 2026-07-21 01:31:22)
Read blend: "/root/q16b/scene4.5.blend"
Fra:1 Mem:40.99M (Peak 40.99M) | Time:00:02.96 | Rendering 1 / 64 samples
Fra:1 Mem:40.99M (Peak 40.99M) | Time:00:02.98 | Rendering 25 / 64 samples
Fra:1 Mem:40.99M (Peak 40.99M) | Time:00:03.00 | Rendering 50 / 64 samples
Fra:1 Mem:40.99M (Peak 40.99M) | Time:00:03.01 | Rendering 64 / 64 samples
Saved: '/root/q16b/out/e17_0001.png'
Time: 00:03.16 (Saving: 00:00.14)


Blender quit
```

`grep -c EGL` on that log returned `0`. The render finished, exited 0 and wrote its image.

**Three runs now stand on this, and none is chosen between.** The first batch, on driver 580.95.05, recorded three copies of `EGL Error (0x3009): EGL_BAD_MATCH: …` before every EEVEE render, with every render working anyway. The citation check, on 580.173.02, found no EGL line, with every render working. This run, on 580.173.02, found no EGL line, with every render working. All three are recorded and none is withdrawn. What every one of the three agrees on is the thing the answer rests on: an EEVEE render works on a machine with no display server and writes its image.

#### Whether `-E help` answers on the rented Linux machine

The engine identifier used above and in the first batch was read off `blender -b --factory-startup -E help`. The citation check found that command returning nothing at all from the 4.5 binary on the rented machine. Run again with each of the four Blenders on that machine, each with a sixty-second ceiling, Blender 4.5.12 printed:

```
=== 4.5 ===

real	1m0.035s
user	0m0.353s
sys	0m0.142s
EXIT STATUS 124
```

Across the four, **Blender 4.5 printed nothing at all and was stopped by the ceiling** — exit status 124, which is what `timeout` reports when it had to stop the command itself, and a third of a second of processor time used in a whole minute, so it was waiting rather than working. Blender 5.0, 5.1 and 5.2 each printed their engine list in under a second. This repeats the citation check's result exactly. Nothing in the answer to question 16 rests on it: the identifiers were read off the Mac, where the command answers in every version, and the 4.5 binary on the rented machine renders perfectly well in both Cycles and EEVEE, as the tests above show.

### What these tests could not settle, and why

- **Anything on Windows.** The runs used a Mac and a rented Linux machine; no Windows machine has run it, so nothing about Windows was tested and nothing here speaks for it. **Untried rather than untestable: a Windows machine can be rented, and every one this project has read reported a Server edition — see the equipment section of `Preparation/Blender Research/Method.md`.**
- **Whether a version-conversion pass rewrites a sample count stored by a much older Blender.** *Part G* and hand-test item 4 both need a file saved by Blender 3.0 or earlier, from the days of squared samples. The project has four Blenders, from 4.5 up, and no file saved by any older one. Only the part that could be run was run, above.
- **The automatic values Cycles picks when the Noise Threshold or Min Samples is zero.** Nothing a terminal can read prints either number; Cycles works them out inside the render and does not report them. The test above shows the automatic threshold differs from 0.01 and stops there.
- **Whether the Cycles Time Limit is counted the same way when the render is on a graphics card.** Part-way through this round the rented machine's own automatic upgrade replaced its NVIDIA libraries with version 580.173 while the loaded driver stayed at 580.95.05, after which Cycles answered `ERROR CUDA cuInit: Unknown error` and offered only the processor. Putting that right needs the machine restarted, which was not done for one extra run. All twenty-four time-limit runs above were on the processor.
- **Whether the Layer Samples control appears in the Sampling panel only when some view layer carries an override.** Settling that needs eyes on a Blender window. The Creator was given nothing this round, and the rented Linux machine has no display server, so no window could be opened on either machine. Whether the *setting* keeps its value is a different question and is answered above: it survives a save and a reopen.

---

## Things a person with Blender open could settle in a few minutes

Pruned on 2026-08-10: items 1, 2, 3, 5, 6 and 7 were answered by the hand tests above and are removed. What is left:

1. **Does a sample count stored by a much older Blender survive being opened here?** Take a file saved by a Blender old enough to have used squared samples — 3.0 or earlier — open it in 4.5, and read Render properties ‣ Sampling ‣ Max Samples. Compare against what the older Blender showed. This cannot be run with the Blenders this project has; it needs such a file from somewhere else.
2. **What does the render *window* say when the time limit ends a render early?** The terminal's wording is recorded above; what the interface shows in its status line is not, and needs eyes on a window.

---

2026-08-10 — Claude Opus 5 (Claude Code) — Settings, Blender 4.5

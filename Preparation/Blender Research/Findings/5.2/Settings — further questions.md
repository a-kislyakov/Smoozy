# Settings — further questions (Blender 5.2)

Sources consulted, all inside `Blender Sources/`, all read on **2026-07-30** (the snapshot date):
the Blender 5.2 manual (`manual_5.2`, including its `_images` folder), the Blender 5.2 Python API
documentation (`api_5.2`), the Blender 5.2 release notes (`release_notes_5.2`), and Cycles' own
files (`cycles_5.2`).

I read only the question sheet and files inside Blender Sources/. I did not open any other part of this project.

I used no helper agents. Everything below is my own reading.

---

## 16. When a render is started from outside Blender and given a sample count of its own, what decides how many samples it actually takes, and what else has a say in when it stops?

### How I read the question

The question was ambiguous, so I say plainly which reading I answered.

I answered this reading: **a render is started from outside Blender, and by the time it runs, a
sample count has been put in place for it. What then governs how many samples are actually taken,
and what else can end the render?**

The other reading is: **the sample count is handed to Blender as a command-line argument.** I did
not take that reading, because the sources describe no such argument (see the note at the end of
this answer). Per the sheet's own note, this answer takes no view on whether a sample count can be
settled from outside; it only describes what governs the render once it is running.

I answer for the three render engines the manual documents a sample count for: Cycles, EEVEE, and
Workbench.

---

### LABEL: **documented** for what each setting is and does; **observed** for the names, kinds, defaults and ranges taken from Cycles' own files; **inferred** where a step has to be supplied on top of those, each marked where it happens; **not in the sources** for how several of these settings combine.

---

### A. Cycles — the scene sample count, and when it is shown as a maximum

The scene sample count is the number of paths traced per pixel in the final render. It is labelled
"Max Samples" in the interface whenever adaptive sampling is on, and "Samples" when it is off.
Whether it acts as a ceiling rather than a target is a separate matter, and depends on adaptive
sampling — section B.

Exact words, manual:

> "Render (Max) Samples
>
> The number of paths to trace per pixel in the final render.
> A higher number results in a cleaner image at the cost of a longer render time."

Source: manual — https://docs.blender.org/manual/en/5.2/render/cycles/render_settings/sampling.html
Date read: 2026-07-30

The setting itself, from Cycles' own files. **observed**:

> ```
>     samples: IntProperty(
>         name="Samples",
>         description="Number of samples to render for each pixel",
>         min=1, max=(1 << 24),
>         default=4096,
>     )
> ```

File: `Blender Sources/cycles_5.2/properties.py`, lines 485–490, in class `CyclesRenderSettings`.
Date read: 2026-07-30

That the same property is shown as "Max Samples" or as "Samples" depending on adaptive sampling is
also in Cycles' own files. **observed**:

> ```
>         col = layout.column(align=True)
>         if cscene.use_adaptive_sampling:
>             col.prop(cscene, "samples", text="Max Samples")
>             col.prop(cscene, "adaptive_min_samples", text="Min Samples")
>         else:
>             col.prop(cscene, "samples", text="Samples")
>         col.prop(cscene, "time_limit")
> ```

File: `Blender Sources/cycles_5.2/ui.py`, lines 311–317, in the render sampling panel.
Date read: 2026-07-30

---

### B. Cycles — adaptive sampling can stop a pixel early, and it is on by default

Exact words, manual:

> "If the Noise Threshold checkbox is enabled, Cycles will use adaptive sampling,
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

Source: manual — https://docs.blender.org/manual/en/5.2/render/cycles/render_settings/sampling.html
Date read: 2026-07-30

That the count varies from pixel to pixel, and is measured against the maximum, is stated on the
passes page. Exact words:

> "Sample Count Cycles
>
> Number of samples calculated for each pixel, divided by the maximum number of samples.
> Used to analyze adaptive sampling."

Source: manual — https://docs.blender.org/manual/en/5.2/render/layers/passes.html
Date read: 2026-07-30

The three settings behind this, from Cycles' own files. **observed**:

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

File: `Blender Sources/cycles_5.2/properties.py`, lines 572–592, in class `CyclesRenderSettings`.
Date read: 2026-07-30

Note the default of `use_adaptive_sampling` is `True`. **observed** from the line above.

---

### C. Cycles — a time limit can end the render before the samples are done

Exact words, manual:

> "Time Limit
>
> Stops rendering if the time exceeds the limit, even if the desired sample count
> hasn't been reached yet. Setting this value to zero disables the limit."

> "Note
>
> The time limit does not include pre-render processing time, only render time."

Source: manual — https://docs.blender.org/manual/en/5.2/render/cycles/render_settings/sampling.html
Date read: 2026-07-30

The setting, from Cycles' own files. **observed**:

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

File: `Blender Sources/cycles_5.2/properties.py`, lines 520–528, in class `CyclesRenderSettings`.
Date read: 2026-07-30

The default value is zero, and the manual says zero disables the limit — so the limit is off by
default. **inferred**, one step: joining the quoted default to the quoted meaning of zero.

Whether this setting travels with the file is not something either quote says. See "Outside the
interface" below, where that step is written out.

---

### D. Cycles — a view layer can replace the scene's number, and a policy decides whether it may

Exact words, manual (view layer page):

> "Samples
>
> Allows the view layer to use a different number of samples than the main scene.
> This is controlled by the Layer Samples
> when using the Cycles renderer. When using EEVEE, the Samples override is always used."

Source: manual — https://docs.blender.org/manual/en/5.2/render/layers/override.html
Date read: 2026-07-30

Exact words, manual (Cycles sampling page):

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

Source: manual — https://docs.blender.org/manual/en/5.2/render/cycles/render_settings/sampling.html
Date read: 2026-07-30

The override itself, from the API documentation. Exact words:

> "samples
>
> Override number of render samples for this view layer, 0 will use the scene setting (in [0, inf], default 0)"

Source: Python API documentation — https://docs.blender.org/api/5.2/bpy.types.ViewLayer.html
Date read: 2026-07-30

The policy setting, from Cycles' own files, including its default. **observed**:

> ```
> enum_use_layer_samples = (
>     ('USE', "Use", "Per render layer number of samples override scene samples"),
>     ('BOUNDED', "Bounded", "Bound per render layer number of samples by global samples"),
>     ('IGNORE', "Ignore", "Ignore per render layer number of samples"),
> ```

File: `Blender Sources/cycles_5.2/properties.py`, lines 64–67.
Date read: 2026-07-30

> ```
>     use_layer_samples: EnumProperty(
>         name="Layer Samples",
>         description="How to use per view layer sample settings",
>         items=enum_use_layer_samples,
>         default='USE',
>     )
> ```

File: `Blender Sources/cycles_5.2/properties.py`, lines 556–561, in class `CyclesRenderSettings`.
Date read: 2026-07-30

So the default policy is `USE`. **inferred**, one step: joining that observed default to the
manual's own wording for `Use` — "Allow view layers to override the scene-level sample count" —
gives a view layer's own number winning over the scene's. The API documentation quoted just above
adds that a view layer whose `samples` is 0 uses the scene setting, so this only bites where a view
layer has a number of its own.

---

### E. Cycles — Sample Subset overrides the number of samples that are actually rendered

Exact words, manual:

> "Sample Subset
>
> Only render a subset of the samples."

> "Offset
>
> The 0-based index of the first sample in the subset.
>
> Length
>
> The number of samples in the subset. While this overrides (Max) Samples in terms of the
> samples that will get rendered, it's still important to set (Max) Samples to the total number
> of samples that will be rendered across all subsets – otherwise, the subsets will have
> incompatible noise and combining them will give a worse result."

Source: manual — https://docs.blender.org/manual/en/5.2/render/cycles/render_settings/sampling.html
Date read: 2026-07-30

The settings, from Cycles' own files. **observed**:

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

File: `Blender Sources/cycles_5.2/properties.py`, lines 500–518, in class `CyclesRenderSettings`.
Date read: 2026-07-30

The manual's worked example for this feature also states the advice to switch denoising off when
splitting a frame across machines. Exact words:

> "On both machines, set the (Max) Samples to 2048, disable Denoise, and enable Sample Subset."

Source: manual — https://docs.blender.org/manual/en/5.2/render/cycles/render_settings/sampling.html
Date read: 2026-07-30

---

### F. EEVEE — the sample count, and two features that change it

Exact words, manual:

> "Render
>
> Samples
>
> The number of samples to use in the final render."

Source: manual — https://docs.blender.org/manual/en/5.2/render/eevee/render_settings/sampling.html
Date read: 2026-07-30

The setting, from the API documentation. Exact words:

> "taa_render_samples
>
> Number of samples per pixel for rendering (in [1, inf], default 64)"

Source: Python API documentation — https://docs.blender.org/api/5.2/bpy.types.SceneEEVEE.html
Date read: 2026-07-30

Two documented things change the count EEVEE actually uses.

First, Jitter Camera in depth of field. Exact words, manual:

> "Jitter Camera
>
> Randomize the camera position for every scene render sample to increase precision.
> Enabling this option can change the scene's actual sample count."

> "Be aware that the actual sample count can grow quite rapidly."

> "The actual number of samples is computed by the following formula:
>
> \[sample\_count = (ring\_count^{2} + ring\_count) * 3 + 1\]
>
> where \(ring\_count\) is the number of ring in the hexaweb pattern.
> The \(ring\_count\) is chosen so that the entire pattern contains at least the number of
> samples set in the Render Settings."

Source: manual — https://docs.blender.org/manual/en/5.2/render/eevee/render_settings/depth_of_field.html
Date read: 2026-07-30

Second, motion blur steps. Exact words, manual:

> "When using multiple time steps, the render sample count is rounded up to the next multiple
> of steps to ensure even distribution of samples across steps."

Source: manual — https://docs.blender.org/manual/en/5.2/render/eevee/render_settings/motion_blur.html
Date read: 2026-07-30

The steps setting, from the API documentation. Exact words:

> "motion_blur_steps
>
> Controls accuracy of motion blur, more steps means longer render time (in [1, inf], default 1)"

Source: Python API documentation — https://docs.blender.org/api/5.2/bpy.types.SceneEEVEE.html
Date read: 2026-07-30

And, as quoted in section D above, for EEVEE a view layer's Samples override "is always used" —
there is no policy setting for it.

I found no time limit and no adaptive-sampling early stop for EEVEE. **not in the sources** — see
"Where I looked" below. Note what that is and is not: the sources describe no early stop, which is
not the same as the sources saying EEVEE has none.

---

### G. Workbench — the sample count is chosen from a fixed set of numbers

Exact words, manual:

> "Multisample
>
> The scene will be rendered multiple times with a slight offset.
> The anti-aliasing will be gathered from the multiple renders.
> The number of samples are predefined so it uses the best distribution of the samples.
>
> 5, 8, 11, 16, 32"

Source: manual — https://docs.blender.org/manual/en/5.2/render/workbench/sampling.html
Date read: 2026-07-30

---

### H. Short answer, pulled together

This is a list of the separate things each source names. It is **not** an order of operations: the
sources do not say what happens when several of them apply at once, and nothing below should be read
as saying they run in this sequence. See the first item under NOT COVERED.

For Cycles, what decides how many samples are actually taken:

1. The sample count in force for the view layer being rendered — the view layer's own override if it
   has one and the Layer Samples policy allows it (default `USE`), otherwise the scene's number.
2. Whether adaptive sampling is on (default on). If it is, that number is a maximum, and each pixel
   may stop earlier once its estimated noise falls below the Noise Threshold, but never before Min
   Samples.
3. Whether Sample Subset is on. If it is, the Length is what gets rendered, not the maximum.

What else has a say in when it stops:

4. The Time Limit, which ends the render even if the sample count has not been reached. It counts
   render time only, not pre-render processing.

For EEVEE, the manual calls the setting the number of samples to use in the final render, and names
two things that change it: Jitter Camera, where the count grows to fit the hexaweb ring pattern, and
motion blur steps, which round it up to the next multiple of the step count. A view layer's Samples
override always replaces the scene number. Nothing in the sources describes an early stop or a time
limit for EEVEE, which is silence rather than a statement that there is none.

For Workbench, the sources describe a fixed set of predefined sample counts — 5, 8, 11, 16, 32 —
rather than a number typed in freely.

---

### NOT COVERED

- **How these combine.** The sources never state what happens when several of them apply at once —
  for example, a Time Limit plus Sample Subset, or a view layer override plus `BOUNDED` plus
  adaptive sampling. Only the pairing of adaptive sampling with Max Samples and Min Samples is
  spelled out. I did not guess at the rest.
- **What the automatic values are.** Noise Threshold 0 makes Cycles "guess an automatic value based
  on the total sample count", and Min Samples 0 makes it pick "a value determined by the Noise
  Threshold". The sources give neither formula.
- **Whether a whole render, as opposed to a pixel, stops early.** The manual describes adaptive
  sampling as cutting short sampling per pixel and in areas. It does not say what ends the frame
  once every pixel has converged. I found nothing on that.
- **Sample counts for particular features.** Cycles Grease Pencil has its own "SSAA Samples", and
  EEVEE has separate shadow-ray, shadow-step and volumetric sample counts. Those are separate
  settings, not the render sample count, and I did not chase them.
- **Anything about the count being reported back.** I did not look into what a render prints about
  samples taken, beyond noting that `--cycles-print-stats` exists.
- **Values outside the accepted range.** That is questions 14 and 15; I did not answer it here.
- **The other reading of the question.** If "given a sample count of its own" means a count passed
  as a command-line argument, then the answer is that the manual's command line arguments page lists
  no such argument, and Cycles' own command-line options are only `--cycles-print-stats` and
  `--cycles-device` (**observed**, `Blender Sources/cycles_5.2/engine.py`, lines 13–20, in
  `_configure_argument_parser`; date read 2026-07-30). Whether a count can be supplied some other
  way is question 1's ground, not mine.
- **5.2 release notes.** I searched them for changes to sampling, noise thresholds, time limits and
  denoising and found none that bear on this question. So everything above describes 5.2 but is not
  described by the release notes as new in 5.2.

---

### Outside the interface

The manual says a command-line render uses the settings saved in the blend-file. Exact words:

> "blender -b file.blend -a
>
> -a
>
> Render the whole animation using all the settings saved in the blend-file."

Source: manual — https://docs.blender.org/manual/en/5.2/advanced/command_line/render.html
Date read: 2026-07-30

From there to "everything in sections A to G applies to a render started from outside in the same
way" takes one step a reader has to supply, so it is **inferred**, not documented. The step: the
settings in sections A to G sit on scene and view layer settings blocks, and nothing quoted above
says those blocks are among the settings saved in the blend-file. Nothing in the sources contradicts
it either. Only a running Blender settles it.

Two differences I can point to from the sources:

1. Some of the sampling settings carry "Viewport" or "Preview" in their names and sit beside the
   final-render ones: Viewport (Max) Samples, Denoise (Viewport), Start Sample,
   `use_preview_adaptive_sampling` and its threshold and min samples, and Scrambling Distance's
   Viewport option. The final-render Cycles `samples` property has `min=1`, so it will not take a
   zero. **observed**, `Blender Sources/cycles_5.2/properties.py` lines 485–497. Date read:
   2026-07-30. What a zero does in any of the viewport properties is not quoted anywhere in this
   answer, and I make no claim about it.
2. Path Guiding is CPU-only. Exact words: "Path guiding is only available when rendering on a CPU".
   Source: manual — https://docs.blender.org/manual/en/5.2/render/cycles/render_settings/sampling.html
   Date read: 2026-07-30. Those words say where the feature is available. They do not say what it
   does to the sample count or to how fast noise falls, and I do not claim it either way — only that
   a render set to a graphics card does not have this feature and one set to the processor does.

Beyond that, the sources do not say whether any of this behaves differently in background mode. In
particular, I found nothing about how a render started from outside can be stopped by anything other
than the settings above — no documented signal handling, no documented external stop. The manual's
only mention of cancelling a render is about a render job in the interface
(https://docs.blender.org/manual/en/5.2/render/output/introduction.html: "the render job can be
canceled"; date read 2026-07-30). The sources do not say.

**Settled by test on 2026-08-10. LABEL: observed.** Every one of the fifteen tests recorded under *Hand-test results* below was run from a terminal with no window at all, on two computers, and each of the settings above did what this answer describes in that setting: the sample count, adaptive sampling, the time limit, the sample subset and the Layer Samples setting all took effect in a render started from outside Blender. Nothing behaved differently for being driven from outside. The viewport copies of these settings have no bearing on such a render — changing all four of them left the reported count and every pixel of the picture unchanged. And these settings do travel with the file: eleven of them were saved and read back unchanged. What is still untested is the comparison against a render started inside Blender's own window, which neither machine could run this round.

### Operating system

The sources do not tie any of the sample-count or stopping settings to an operating system. They are
scene data and are described without any platform condition.

Two platform-dependent things sit nearby. Neither is a sample-count setting, and neither quote says
anything about how many samples are taken.

> "Only available on NVIDIA GPUs when configured in the Cycles Render Devices user preferences"

This is the OptiX denoiser. Source: manual —
https://docs.blender.org/manual/en/5.2/render/cycles/render_settings/sampling.html, under Denoising ‣
Denoiser ‣ OptiX. Date read: 2026-07-30. *(Citation check, 2026-08-10: filled in the address, which
was marked PENDING. The sentence is on that page and reads in full "Only available on NVIDIA GPUs
when configured in the Cycles Render Devices user preferences.")*

> "CPU CUDA OPTIX HIP ONEAPI METAL"

These are the values `--cycles-device` accepts. Source: manual —
https://docs.blender.org/manual/en/5.2/advanced/command_line/arguments.html, under Cycles Render
Options, where the line reads in full "Valid options are: CPU CUDA OPTIX HIP ONEAPI METAL." Date
read: 2026-07-30. *(Citation check, 2026-08-10: filled in the address, which was marked PENDING.
`Blender Sources/cycles_5.2/engine.py` carries the same list in its own words —
"Valid options are 'CPU', 'CUDA', 'OPTIX', 'HIP', 'ONEAPI', or 'METAL'." — but the wording quoted
here is the manual's.)*

Whether the automatic Noise Threshold or Min Samples values differ by platform is not stated
anywhere I looked; the sources do not say.

**Settled by test on 2026-08-10, for two operating systems out of three. LABEL: observed.** All fifteen tests recorded under *Hand-test results* below were run on macOS 26.5.2 on Apple silicon and on Ubuntu 22.04.5 LTS on an AMD processor, and the two machines returned the same sample count in every single test. Where a figure differed between them it was a time, or a count reached under a clock, never a count reached under a sample setting. **Windows is untested and stays untested** — no Windows machine has run it — so nothing here speaks for it.

---

## Where I looked

Because parts of this answer are "not in the sources", here is the record.

Searches run across the whole of `manual_5.2`, `api_5.2`, `release_notes_5.2` and `cycles_5.2`
(case-insensitive), all on 2026-07-30:

- `cycles-samples`, `cycles_samples`, `--cycles` — only `--cycles-device` and `--cycles-print-stats`
  turn up, on the command line arguments page and in `cycles_5.2/engine.py`. No sample argument.
- `sample` restricted to `manual_5.2/advanced/command_line/*` — no hits at all. The command line
  pages say nothing about samples.
- `time limit` — hits only in `manual_5.2/render/cycles/render_settings/sampling.txt`,
  `cycles_5.2/properties.py`, and an unrelated `bpy.ops.wm.redraw_timer` argument in the API docs.
  Nothing for EEVEE, nothing for Workbench, nothing general to all engines.
- `sample_subset`, `sample_offset`, `use_sample_subset`, `time_limit`, `use_layer_samples`,
  `adaptive_min_samples`, `adaptive_threshold`, `use_adaptive_sampling` across `api_5.2` — no hits.
  The Cycles settings are add-on properties and are not in the bundled API documentation, which is
  why I cite Cycles' own files for them.
- `adaptive sampling` across the manual — Cycles sampling page, the Cycles render settings index, the
  passes page, and a history page. Nothing for EEVEE.
- `max samples`, `maximum number of samples`, `sample count` across the manual — the Cycles sampling
  page, the EEVEE depth of field and motion blur pages, the EEVEE material settings page, the passes
  page, and an unrelated Alembic page.
- `stop sampling`, `stops rendering`, `terminate`, `converge`, `clean enough`, `early exit`,
  `samples per pixel` across the manual — nothing beyond the pages already quoted. "terminate" in a
  render context appears only for Russian Roulette path termination and the Light Threshold, which
  are about paths within a sample, not about how many samples are taken.
- `sampl`, `noise threshold`, `time limit`, `denois` across `release_notes_5.2/cycles.md`,
  `eevee.md`, `rendering.md`, `python_api.md`, `index.md`, `core.md` — nothing on sampling limits or
  stopping. The only Cycles hit is an unrelated "Sample Attributes panel" for a node.
- `cancel`, `abort`, `Ctrl-C` across the command line pages, `render/introduction.txt` and
  `render/output/` — only `--disable-abort-handler` on the arguments page, whose entire description
  is "Disable the abort handler." with nothing said about what that handler does, plus one sentence
  about cancelling a render job in the interface.

Pages read in full: the Cycles sampling page, the EEVEE sampling page, the Workbench sampling page,
the view layer override page, the command line arguments page, the command line render page. Parts
read: the EEVEE depth of field page, the EEVEE motion blur page, the passes page, the Cycles render
settings index.

Files read in `cycles_5.2`: `properties.py` (the sampling parts of `CyclesRenderSettings`, the
`enum_use_layer_samples` list, and `CyclesRenderLayerSettings`), `ui.py` (the sampling panels), and
`engine.py` (the command-line argument parser). I also listed `cycles_5.2/source`, which holds only
kernel headers — there is no Blender-to-Cycles synchronisation code there, so I could not see how
these settings are read at render time, and I have not guessed at it.

Images: I looked for a screenshot of the Cycles sampling panel in `manual_5.2/_images` by listing
every image file whose name contains "sampl". None of them is a render sampling panel; they are all
for unrelated features (rigging, the graph editor, geometry nodes, grease pencil). The Cycles
sampling page carries no `[image: ...]` marker at all. So no picture was available to settle
anything here.

I did not use the web.

---

## Hand-test results — 2026-08-10

**LABEL: observed**, throughout this section.

Fifteen tests were run in Blender 5.2.0 on two computers, on 2026-08-10. **The Creator's own Mac** — a MacBook Pro with an M1 Pro chip, macOS 26.5.2, eight cores, 16 GB of memory, Apple's own graphics and no second card. **A rented Linux machine** — Ubuntu 22.04.5 LTS on kernel 6.8, one NVIDIA GeForce RTX 4060 Ti with 16 GB on driver 580.95.05, twenty-three cores of an AMD EPYC 7K62, 98 GB of memory, and no display server at all. Every test was run on both, and the two machines are two results, never one.

**How they were run.** Each Blender was pointed at a throwaway settings folder of its own, made before use, so no real preferences were involved. Every render was on the processor rather than the graphics card, because a fresh settings folder switches no graphics device on and the scene asks for `CPU`. The rented machine is somebody else's hardware and a share of a bigger chip, so its times are its own and are never read against the Mac's; it was idle when anything was timed (`load average: 0.00, 0.02, 1.30`, nothing above 4% of one core, and `0MiB / 16380MiB` used on the card).

**The scene**, built by Blender 5.2.0 itself from a script so that no file saved by another Blender is involved: 320 by 240, Cycles on the processor, a plane, a rough metal sphere, a glass cube, one large area light and a dark world — noisy enough that sampling shows in the clock. Renders were started as `blender -b <file> --python-expr "<settings>" -o <folder> -F PNG -f 1`. *(Citation check, 2026-08-10: the script itself is not here, and without it this scene cannot be rebuilt exactly. Repeating these tests on a scene built afresh to this description returned every count that does not depend on how noisy the scene is — 4096/4096, 32/4096, 256/256, 8/8, the six view-layer results, the EEVEE counts — and different figures for everything reached under a clock. A test that turns on a clock needs its scene supplied, not described.)* *(Second batch of hand tests, 2026-08-10: the script is now supplied in full, under “Second batch of hand tests” further down this document, and the four figures that turn on a clock were run again on it.)*

**How the answer was read.** Blender prints a running `Sample n/N` line for each view layer of each frame; the last one with a non-zero `n` is the count that run reached and `N` is the count it was counting towards. One line reading `Sample 0/N` is printed just before `Finished` and is not a result. EEVEE prints `Rendering n / N samples` instead.

### The results, test by test

The Mac and the rented Linux machine returned **the same sample count in every one of these tests**. Only the times differ, and both are given.

| What was set | What Blender reported (both machines) | Mac time | Linux time |
|---|---|---|---|
| Max Samples 4096, adaptive sampling off | `Sample 4096/4096` | `01:28.26` | `00:37.00` |
| Max Samples 4096, adaptive on at 0.01 | `Sample 4096/4096` | `00:22.55` | `00:10.71` |
| Max Samples 4096, adaptive on at 0.5 | `Sample 32/4096` | `00:00.95` | `00:00.98` |
| Max Samples 2048, sample subset off | `Sample 2048/2048` | `00:43.22` | `00:18.71` |
| Max Samples 2048, subset on, Length 256 | `Sample 256/256` | `00:05.68` | `00:02.76` |
| the same plus adaptive on at 0.01 | `Sample 256/256` | `00:05.32` | `00:02.63` |
| the same plus a 2-second Time Limit | `Sample 89/256` (Mac), `Sample 225/256` (Linux) | `00:02.25` | `00:02.48` |
| Max Samples 200, viewport numbers set low | `Sample 200/200` | `00:03.68` | `00:01.82` |
| Max Samples 200, viewport numbers set high | `Sample 200/200` | `00:03.48` | `00:01.82` |
| Max Samples 8 with Min Samples 64 | `Sample 8/8` | `00:00.40` | `00:00.55` |
| Max Samples 8 with Min Samples 0 | `Sample 8/8` | `00:00.39` | `00:00.54` |
| Max Samples 1024, adaptive on at 0.01 | `Sample 1024/1024` | `00:09.24` | `00:04.55` |
| Max Samples 1024, adaptive on at 0 | `Sample 1024/1024` | `00:21.18` | `00:09.68` |
| Max Samples 128, adaptive on at 0 | `Sample 128/128` | `00:03.38` | `00:01.53` |
| Max Samples 4096, adaptive on at 0 | `Sample 4096/4096` | `01:20.80` | `00:33.82` |
| stored count 4096, 32 supplied at launch before `-f` | `Sample 32/32` | `00:00.90` | `00:00.76` |
| stored count 4096, 32 supplied at launch after `-f` | `Sample 4096/4096` | `01:26.19` | `00:36.88` |
| EEVEE Render Samples 64, Cycles 4096 in the same file | `Rendering 64 / 64 samples` | `00:01.18` | `00:09.29` |
| EEVEE Render Samples 64, motion blur on, 5 steps | `Rendering 65 / 65 samples` | `00:00.74` | `00:10.55` |
| EEVEE Render Samples 16, Jitter Camera on | `Rendering 19 / 19 samples` | `00:00.34` | `00:06.70` |
| EEVEE Render Samples 16, Jitter Camera off | `Rendering 16 / 16 samples` | `00:00.31` | `00:04.07` |
| Workbench | no sample line of any kind | `00:00.15` | `00:01.03` |
| Max Samples 64 with `-- --cycles-print-stats` | `Sample 64/64`, then a `Render statistics:` block | `00:01.63` | `00:01.06` |

### A frame does end before Max Samples when its pixels converge, and the count is decided pixel by pixel

The first three rows above are the same frame three ways. With adaptive sampling on at a loose threshold of 0.5 the whole frame ended at `Sample 32/4096`, so a frame does stop once its pixels have converged rather than only individual pixels stopping. With adaptive sampling on at the default 0.01 the reported count did not move at all while the time fell to about a quarter, which is the per-pixel skipping showing in the clock rather than in the count.

The Cycles view-layer settings carry a sample-count pass, printed on both machines as:

```
PASS_SAMPLE_ATTRS ['denoising_pass_follow_reflections', 'denoising_pass_use_albedo_roughness_weighting', 'denoising_store_passes', 'pass_debug_sample_count', 'use_denoising']
```

### The Cycles Time Limit is spent on each frame, not across the whole job

Three frames in one launch with `-s 1 -e 3 -a`, Time Limit 10 seconds, Max Samples 4096, adaptive sampling off, three repeats on each machine. **Every one of the six runs wrote three PNG files.**

On the Mac the whole run took 33.26, 32.38 and 32.39 seconds. The per-frame `Time:` lines in the first repeat were `00:10.52`, `00:10.46` and `00:10.68`, and the three frames reached `Sample 513/4096`, `Sample 513/4096` and `Sample 513/4096`.

On the rented Linux machine, idle, the whole run took 31.95, 32.05 and 32.00 seconds. The per-frame times in the first repeat were `00:10.46`, `00:10.28` and `00:10.27`, and the frames reached `Sample 1116/4096`, `Sample 1120/4096` and `Sample 1112/4096`.

Not one of the six runs stopped the job after ten seconds. **Nothing is printed to say the limit fired**: the frame reports its count, says `Finished`, saves the image, and moves on. The only sign is the count sitting below the maximum.

### Which number a render counts to when a view layer carries its own

Two halves, because an override below the scene count cannot tell `Bounded` from `Use`. Adaptive sampling off, Time Limit 0, and the scene count supplied at launch by `--python-expr` rather than saved in the file. Both machines gave these six results identically:

| | second layer's own Samples 64, scene count 1024 | second layer's own Samples 1024, scene count 256 |
|---|---|---|
| `USE` | first layer `Sample 1024/1024`, second `Sample 64/64` | first `Sample 256/256`, second `Sample 1024/1024` |
| `BOUNDED` | first `Sample 1024/1024`, second `Sample 64/64` | first `Sample 256/256`, second `Sample 256/256` |
| `IGNORE` | first `Sample 1024/1024`, second `Sample 1024/1024` | first `Sample 256/256`, second `Sample 256/256` |

So a view layer's own number stands in place of a scene count supplied from outside under `USE`, is capped by that count under `BOUNDED`, and is thrown away under `IGNORE`. Blender 5.2.0 on the Mac, second half, `BOUNDED`, as printed:

```
00:00.446  render           | Start rendering: Scene, ViewLayer
00:05.900  render           | Fra: 1 | Mem: 5M | Sample 256/256
00:05.900  render           | Start rendering: Scene, Second
00:11.516  render           | Fra: 1 | Mem: 5M | Sample 256/256
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
00:21.363  render           | Fra: 1 | Remaining: 01:02.09 | Mem: 5M | Sample 1025/4096
Sent an internal break event. Press ^C again to kill Blender
00:21.364  render           | Fra: 1 | Remaining: 01:02.09 | Mem: 5M | Cancelled
Blender quit
```

On the rented Linux machine the same three lines came back, with the stop at `00:09.725` and also at `Sample 1025/4096`. On both machines the stop landed on the same sample number rather than at the same moment.

A terminate signal printed nothing further, wrote no file, and gave exit status 143; the last line of its log is an ordinary `Sample 1/4096`. Adding `--disable-abort-handler` to the interrupt run changed nothing at all — the same break line, the same `Cancelled`, exit 1, no file. All the same on both machines.

### EEVEE and Workbench

The engine identifier came from Blender itself: `blender -b --factory-startup -E help` printed `BLENDER_EEVEE`, alongside `BLENDER_WORKBENCH` and `CYCLES`.

EEVEE took exactly the number it was given, and Cycles' Max Samples of 4096 sitting in the same file changed nothing. The two features that raise the count did so exactly as the table shows: five motion-blur steps turned 64 into 65, and Jitter Camera turned 16 into 19. Printing the whole EEVEE settings block turned up no time limit. Searched for "time" and "limit", the only entries were:

```
EEVEEPROP gi_diffuse_bounces | Diffuse Bounces | Number of times the light is reinjected inside light grids, 0 disable indirect diffuse light
EEVEEPROP motion_blur_steps | Motion steps | Controls accuracy of motion blur, more steps means longer render time
EEVEEPROP taa_samples | Viewport Samples | Number of samples, unlimited if 0
```

**A view layer's sample override does change an EEVEE render in 5.2.** With Render Samples 256 and a second view layer carrying its own Samples of 8, the first layer reported `Rendering 256 / 256 samples` and the second `Rendering 8 / 8 samples` — `00:01.60` on the Mac and `00:33.27` on the Linux machine, with `LAYERS [('ViewLayer', 0), ('Second', 8)]` printed alongside. The same on both machines.

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

With `use_single_layer` set to `True`, the first layer active and a second view layer carrying Samples 64 against a scene count of 256, both layers rendered — `Sample 256/256` and `Sample 64/64`, `00:07.01` on the Mac and `00:03.54` on the Linux machine — with `SINGLE_LAYER True` printed alongside. In background mode there is no window, so the instruction meant to make the first layer active had no window to act on; it printed nothing and did not error, and both layers rendered regardless.

### The Noise Threshold's default is not the automatic value it uses at zero

Adaptive sampling on, Min Samples 0, Time Limit 0. Every one of these runs reached its maximum, as the table shows; what separates them is the time and the picture. At Max Samples 1024 a threshold of 0 took `00:21.18` against `00:09.24` for the default 0.01 on the Mac, and `00:09.68` against `00:04.55` on the Linux machine. The two pictures are different pictures on both machines:

```
PIXELS max_difference 0.027450978755950928 channels_differing 23558 of 307200      (the Mac)
PIXELS max_difference 0.027450978755950928 channels_differing 23630 of 307200      (the rented Linux machine)
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

**The scene, supplied.** This is the script that built the test scene, in full. Blender 5.2.0 ran it itself, so no file saved by another Blender is involved. Save it as `make_scene.py` and run it as `blender -b --factory-startup --python make_scene.py -- <somewhere>/scene.blend`:

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

**The count reached under a 2-second time limit on a 256-sample subset.** Max Samples 2048, adaptive sampling off, subset on at Offset 0 and Length 256, Time Limit 2 seconds. On the Mac the record gives `Sample 89/256` and this run gave `Sample 97/256` — different. On the rented Linux machine the record gives `Sample 225/256` and this run gave `Sample 221/256` — different.

**The per-frame counts under a 10-second limit across three frames.** Max Samples 4096, adaptive sampling off, Time Limit 10 seconds, frames 1 to 3 in one launch with `-s 1 -e 3 -a`. On the Mac the record gives `513`, `513`, `513` and this run gave `398`, `513`, `513` — different — in 35.127 s of wall clock, with three PNG files written. On the rented Linux machine the record gives `1116`, `1120`, `1112` and this run gave `1153`, `1121`, `1119` — different — in 32.242 s, again with three files written.

**The channels differing between a noise threshold of 0.01 and one of 0.** Max Samples 1024, adaptive sampling on, Min Samples 0, Time Limit 0; both renders reached `Sample 1024/1024` and the two pictures were compared channel by channel. On the Mac:

```
PIXELS max_difference 0.027450978755950928 channels_differing 23561 of 307200
```

against a recorded `channels_differing 23558 of 307200`. On the rented Linux machine:

```
PIXELS max_difference 0.027450978755950928 channels_differing 23630 of 307200
```

against a recorded `channels_differing 23630 of 307200`.

**The sample number an interrupt lands on after eight seconds.** Max Samples 4096, Time Limit 0, the render started in the background, left for eight seconds, then sent an interrupt. On the Mac it stopped at `Sample 1025/4096` at `00:23.120`; on the rented Linux machine at `Sample 1025/4096` at `00:09.719`. The record gives `Sample 1025/4096` on both machines. Both runs wrote no file and exited 1, as recorded. This run reached the same sample number as the record on the Mac.

**What the four figures together say.** Supplying the scene was most of what was missing and it was not all of it. Across all four versions and both machines — thirty-two figures in all — the picture comparison came back within six channels of the record everywhere and exactly right in three cases, where the citation check's repeat on a scene built from the written description had been out by more than a thousand; the interrupt landed on the recorded sample number in seven cases out of eight; the two-second subset count came back exactly in five cases out of eight; and the ten-second per-frame counts came back exactly in two cases out of eight. The looser a figure's grip on the clock, the better it repeats. **No claim these figures were quoted in support of changed.** A time limit still cut the subset short, ten seconds was still spent on each frame rather than once for the job, a threshold of 0.01 and a threshold of 0 still made different pictures, and an interrupt still stopped the render without writing a file.

#### The EGL line on the rented Linux machine, a third time

One EEVEE render in Blender 5.2.0 on that machine, engine `BLENDER_EEVEE`, Render Samples 64, with the whole log kept and the driver read at the same time. The driver answered:

```
580.173.02
NVRM version: NVIDIA UNIX Open Kernel Module for x86_64  580.173.02  Release Build  (dvs-builder@U22-I3-AK02-24-4)  Tue Jun 23 08:17:01 UTC 2026
```

and the render's whole log, word for word, was:

```
00:00.506  blend            | Read blend: "/root/q16b/scene5.2.blend"
00:00.519  render           | Rendering single frame
00:00.519  render           | Rendering frame 1
00:00.519  render           | Start rendering: Scene, ViewLayer
00:00.519  render           | Engine: EEVEE
00:03.005  render           | Fra: 1 | Rendering 1 / 64 samples
Blender 5.2.0 LTS (hash fbe6228777e7 built 2026-07-14 01:32:04)
00:03.036  render           | Fra: 1 | Rendering 25 / 64 samples
00:03.058  render           | Fra: 1 | Rendering 50 / 64 samples
00:03.071  render           | Fra: 1 | Rendering 64 / 64 samples
00:03.254  render           | Saved: '/root/q16b/out/e17_0001.png'
00:03.254  render           | Time: 00:02.73 (Saving: 00:00.17)

Blender quit
```

`grep -c EGL` on that log returned `0`. The render finished, exited 0 and wrote its image.

**Three runs now stand on this, and none is chosen between.** The first batch, on driver 580.95.05, recorded three copies of `EGL Error (0x3009): EGL_BAD_MATCH: …` before every EEVEE render, with every render working anyway. The citation check, on 580.173.02, found no EGL line, with every render working. This run, on 580.173.02, found no EGL line, with every render working. All three are recorded and none is withdrawn. What every one of the three agrees on is the thing the answer rests on: an EEVEE render works on a machine with no display server and writes its image.

#### Whether `-E help` answers on the rented Linux machine

The engine identifier used above and in the first batch was read off `blender -b --factory-startup -E help`. The citation check found that command returning nothing at all from the 4.5 binary on the rented machine. Run again with each of the four Blenders on that machine, each with a sixty-second ceiling, Blender 5.2.0 printed:

```
=== 5.2 ===
Blender 5.2.0 LTS (hash fbe6228777e7 built 2026-07-14 01:32:04)
Blender Engine Listing:
	BLENDER_EEVEE
	BLENDER_WORKBENCH
	CYCLES

real	0m0.779s
user	0m0.566s
sys	0m0.264s
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

## Things a person with Blender open could settle in a few minutes

Pruned on 2026-08-10: everything the hand tests above answered has been removed. What is left:

1. **How many samples does each pixel really get with adaptive sampling on?** In View Layer
   properties, switch on the Cycles "Sample Count" debug pass (Passes ‣ Data ‣ Sample Count).
   Render with Max Samples 1024 and the Noise Threshold at its default 0.01, save as multilayer
   OpenEXR, and read that pass — it holds samples taken divided by the maximum. The hand tests
   above showed the per-pixel skipping in the clock, not in the pass, so the spread of counts
   across one picture is still unmeasured.

2. **What automatic Noise Threshold does a zero produce, and what automatic Min Samples?** The hand
   tests showed the automatic value is not 0.01 — the same frame took roughly twice as long with the
   threshold at 0 and came out a different picture — but nothing a terminal prints names the value.

3. **What the render window shows when the time limit ends a render early.** The terminal prints
   nothing naming the limit; what the interface's own status line says needs eyes on a window.

---

2026-08-10 — Claude Opus 5 (Claude Code) — Settings, Blender 5.2

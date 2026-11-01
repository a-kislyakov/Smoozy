# Settings — further questions, second pass (Blender 4.5)

Sources consulted, all read on **2026-07-30** (the snapshot date given to me):
the Blender 4.5 manual (`Blender Sources/manual_4.5`), the Blender 4.5 Python API documentation
(`Blender Sources/api_4.5`), the Blender 4.5 release notes (`Blender Sources/release_notes_4.5`),
and Cycles' own files (`Blender Sources/cycles_4.5`).

I read only the question sheet and files inside Blender Sources/. I did not open any other part of this project.

Answering question **16** only.

---

## Record of where I looked

Everything below was done with whole-tree text searches over the source folders, then by opening
the pages the searches pointed at. Searches were case-insensitive.

**Terms searched across the whole manual (`manual_4.5`):**
`sample`, `sample count`, `max samples`, `samples per pixel`, `time limit`, `adaptive sampling`,
`noise threshold`, `sample override`, `override the scene-level`, `override samples`, `converge`,
`convergence`, `budget`, `deadline`, `early exit`, `terminat*`, `stop rendering`, `stops rendering`,
`abort`, `cancel the render`, `Ctrl-C`, `Ctrl-Break`, `clean enough`, `clamp…sample`, `background`,
`headless`, `command line`.

Of these, `budget`, `deadline`, `early exit`, `clean enough` and `clamp…sample` returned nothing
anywhere in the manual. `abort` returned only `--disable-abort-handler` on the command line page,
which is about a crash/abort handler and not about stopping a render, so I ruled it out.
`Ctrl-C` / `Ctrl-Break` returned nothing.

**Pages I opened in full:**

- `render/cycles/render_settings/sampling.txt` — the main page for this question.
- `render/eevee/render_settings/sampling.txt`.
- `render/workbench/sampling.txt`.
- `render/layers/introduction.txt` (the per-view-layer Samples override).
- `render/cycles/render_settings/simplify.txt` — checked whether Simplify reduces samples for a
  render. It does not; it covers subdivision, child particles, texture size and culling. Ruled out.
- `advanced/command_line/render.txt` and `advanced/command_line/arguments.txt` — read the whole
  Render Options and Cycles Render Options sections.

**Pages I checked and ruled out** (searched them for `samples` and found nothing that decides a
render's sample count): `render/cycles/render_settings/performance.txt`,
`.../film.txt`, `.../light_paths.txt`, `render/cycles/optimizations/reducing_noise.txt`
(mentions samples only as advice about noise, not as a stopping rule),
`render/layers/passes.txt` (the Sample Count debug pass), `render/cycles/gpu_rendering.txt`.

**Images.** I looked for `[image: …]` markers on the Cycles sampling page, the EEVEE sampling page
and the view layer introduction page. The two sampling pages carry no images at all, so there was
no panel screenshot to inspect. I opened no image files.

**API documentation (`api_4.5`) searched for:** `number of samples`, `samples per pixel`,
`sample count`, `time_limit`, `samples` within `bpy.types.ViewLayer`, `bpy.types.SceneEEVEE`,
`bpy.types.SceneDisplay`, `bpy.types.SceneGpencil`, `bpy.types.RenderSettings`.
`time_limit` appears in the API documentation only on `bpy.ops.wm.redraw_timer`, an unrelated
test operator — ruled out. There is **no** API page for the Cycles scene settings; the API
documentation only carries `bpy.ops.cycles` and mentions `bpy.context.scene.cycles.samples` as an
example. So the Cycles sampling settings had to come from Cycles' own files.

**Release notes (`release_notes_4.5`) searched for:** `sample`, `time limit`, `noise threshold`.
The only hits are Grease Pencil super-sampling (new in 4.5) and unrelated bug-fix titles. I read
`cycles.md` and `rendering.md` in full: neither records any change to sample counts, adaptive
sampling, the time limit or the sample subset in 4.5.

**Cycles' own files (`cycles_4.5`) searched for:** `samples`, `sample_offset`, `sample_subset`,
`time_limit`, `adaptive`, `use_layer`, `argparse`, `add_argument`, `min_samples`, `start_sample`.
I read the sampling block of `properties.py`, the sampling panels in `ui.py`, the command line
parser in `engine.py`, and in `source/kernel`: `film/adaptive_sampling.h`,
`film/light_passes.h` (the `film_write_sample` function), `integrator/init_from_camera.h`,
and the `KernelWorkTile` struct in `types.h`.

**A gap in what I was given, which limits the answer.** The `cycles_4.5/source` folder contains
only `kernel` and `util`. The host-side code that schedules samples — the part that would decide
how max samples, min samples, the adaptive check and the time limit are combined during a render —
is not in the folder. So I can say what each setting is declared to be, and what the per-pixel
convergence test computes, but I cannot show the rule that combines them. Where that matters I have
said so rather than guessed.

---

## 16. When a render is started from outside Blender and given a sample count of its own, what decides how many samples it actually takes, and what else has a say in when it stops?

**A note on the question first.** "When it stops" can be read two ways: when sampling of the picture
stops, or when the whole render job stops. I answer the first reading, which is the one the phrase
"how many samples it actually takes" sits next to. Under the second reading the answer would also
have to cover things like the frame range and the output settings, which I have not investigated
here.

**Short answer.** A sample count given from outside is a ceiling for the engine, not a promise. In
Cycles four other settings sit between that ceiling and the number of samples a pixel really
receives: adaptive sampling (with its noise threshold and minimum sample count), the time limit,
the sample subset, and the per-view-layer sample override. In EEVEE and Workbench the sources show
no such machinery to lower the count — though the hand tests below found two EEVEE features that
raise it. Details, with the exact words, follow. *(Citation check, 2026-08-10: this ended "so the
number set is the number used", which this document's own hand tests disprove.)*

### Cycles — the ceiling itself

**LABEL: documented.**

> "Render (Max) Samples
>
> The number of paths to trace per pixel in the final render.
> A higher number results in a cleaner image at the cost of a longer render time."

Source: manual, https://docs.blender.org/manual/en/4.5/render/cycles/render_settings/sampling.html
Date read: 2026-07-30.

Note that this is the *final render* number. The page carries a separate "Viewport (Max) Samples"
for the 3D Viewport. A render started from outside Blender is a final render, so the viewport
number and the viewport-only adaptive settings do not apply to it. **That last sentence is
inferred, not documented** — the page gives the Viewport/Render split and nothing more; the step to
"so they do not apply" is the reader's. It is now settled by the hand test below, which changed all
four viewport numbers and got the same count and the same picture. *(Citation check, 2026-08-10:
marked the step, which sat under a "documented" label.)*

**LABEL: observed.** The setting behind it, with its kind, range and default:

> ```
>     samples: IntProperty(
>         name="Samples",
>         description="Number of samples to render for each pixel",
>         min=1, max=(1 << 24),
>         default=4096,
>     )
> ```

File: `Blender Sources/cycles_4.5/properties.py`, lines 482–487.
Date read: 2026-07-30.

`1 << 24` is 16,777,216. The name by which this is reached is documented in the API introduction:

> "# Access the number of samples for the Cycles render engine.
> bpy.context.scene.cycles.samples"

Source: Python API documentation, https://docs.blender.org/api/4.5/info_api_reference.html
Date read: 2026-07-30.

### Cycles — first thing that changes the real count: adaptive sampling

**LABEL: documented.**

> "Adaptive Sampling
>
> If the Noise Threshold checkbox is enabled, Cycles will use adaptive sampling,
> cutting short the sampling process in areas that have become less noisy than the
> specified threshold value – in other words, not wasting time further refining
> areas that already look good enough. For example, hair on a character may need many samples,
> but the background may only need few."

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

Source: manual, https://docs.blender.org/manual/en/4.5/render/cycles/render_settings/sampling.html
Date read: 2026-07-30.

So with adaptive sampling on, different pixels of the same picture take different numbers of
samples: at least the minimum, at most the maximum, and somewhere in between wherever the pixel
gets quiet enough first.

**LABEL: observed.** Adaptive sampling is on by default for final renders, and the three settings
that drive it are these:

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

File: `Blender Sources/cycles_4.5/properties.py`, lines 569–589.
Date read: 2026-07-30.

That `default=True` is worth holding on to: unless the file says otherwise, a sample count given
from outside is a maximum being handed to a mechanism that is already switched on and already
allowed to stop short of it.

**LABEL: observed.** The test a pixel has to fail before it keeps being sampled is written out in
Cycles' kernel:

> ```
> /* Determines whether to continue sampling a given pixel or if it has sufficiently converged. */
> ```
> …
> ```
>   const float error = error_difference / (0.0001f + error_normalize);
>   const bool did_converge = (error < threshold);
> ```

File: `Blender Sources/cycles_4.5/source/kernel/film/adaptive_sampling.h`, lines 27 and 83–84.

And a converged pixel next to an unconverged one is put back to work:

> ```
> /* This is a simple box filter in two passes.
>  * When a pixel demands more adaptive samples, let its neighboring pixels draw more samples too. */
> ```

File: `Blender Sources/cycles_4.5/source/kernel/film/adaptive_sampling.h`, lines 92–93.

And the count is kept per pixel, not per picture:

> ```
>   /* Check whether the pixel has converged and should not be sampled anymore. */
>   if (!film_need_sample_pixel(kg, state, render_buffer)) {
>     return false;
>   }
>
>   /* Count the sample and get an effective sample for this pixel.
>    *
>    * This logic allows to both count actual number of samples per pixel, and to add samples to this
>    * pixel after it was converged and samples were added somewhere else (in which case the
>    * `scheduled_sample` will be different from actual number of samples in this pixel). */
> ```

File: `Blender Sources/cycles_4.5/source/kernel/integrator/init_from_camera.h`, lines 69–78.
Date read for all three: 2026-07-30.

These quotes are what the files say. What a running render does with them is one step further out,
and I have not taken that step: the code that decides *when* the convergence test is run, and how
often, is not in the folder I was given.

### Cycles — second thing: the time limit

**LABEL: documented.**

> "Time Limit
>
> Stops rendering if the time exceeds the limit, even if the desired sample count
> hasn't been reached yet. Setting this value to zero disables the limit."

> "Note
>
> The time limit does not include pre-render processing time, only render time."

Source: manual, https://docs.blender.org/manual/en/4.5/render/cycles/render_settings/sampling.html
Date read: 2026-07-30.

**LABEL: observed.**

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

File: `Blender Sources/cycles_4.5/properties.py`, lines 517–525.
Date read: 2026-07-30.

Default zero, so off unless the file turns it on. When it is on, it can end sampling before the
sample count is reached, whatever that count was set to.

### Cycles — third thing: the sample subset

**LABEL: documented.** This one can override the maximum outright.

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

Source: manual, https://docs.blender.org/manual/en/4.5/render/cycles/render_settings/sampling.html
Date read: 2026-07-30.

**LABEL: observed.**

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

File: `Blender Sources/cycles_4.5/properties.py`, lines 497–515.
Date read: 2026-07-30.

Off by default. When a file has it on, the number of samples actually rendered is the subset
length, not the maximum — the manual says that in as many words.

### Cycles — fourth thing: a view layer's own sample count

**LABEL: documented.** A view layer can carry its own number, and a scene-level switch says how
that number is treated.

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

Source: manual, https://docs.blender.org/manual/en/4.5/render/cycles/render_settings/sampling.html

> "Samples
>
> View layer samples to override the scene samples.
> Controlled by the layer samples in the Sampling panel."

Source: manual, https://docs.blender.org/manual/en/4.5/render/layers/introduction.html

> "samples
>
> Override number of render samples for this view layer, 0 will use the scene setting
>
> Type:
>
> int in [0, inf], default 0"

Source: Python API documentation, https://docs.blender.org/api/4.5/bpy.types.ViewLayer.html
Date read for all three: 2026-07-30.

**LABEL: observed.** The scene-level switch, and its default:

> ```
> enum_use_layer_samples = (
>     ('USE', "Use", "Per render layer number of samples override scene samples"),
>     ('BOUNDED', "Bounded", "Bound per render layer number of samples by global samples"),
>     ('IGNORE', "Ignore", "Ignore per render layer number of samples"),
> ```

File: `Blender Sources/cycles_4.5/properties.py`, lines 68–71.

> ```
>     use_layer_samples: EnumProperty(
>         name="Layer Samples",
>         description="How to use per view layer sample settings",
>         items=enum_use_layer_samples,
>         default='USE',
>     )
> ```

File: `Blender Sources/cycles_4.5/properties.py`, lines 553–558.
Date read: 2026-07-30.

The default is `USE`, meaning a view layer's own number wins over the scene's. This one matters
most for the question as asked: a sample count set on the scene from outside can be silently
beaten by a number stored on a view layer inside the file, unless that file has the switch on
`Bounded` or `Ignore`.

**LABEL: observed.** The Layer Samples control is only drawn when some view layer actually carries
an override:

> ```
>         for view_layer in scene.view_layers:
>             if view_layer.samples > 0:
>                 layout.separator()
>                 layout.row().prop(cscene, "use_layer_samples")
>                 break
> ```

File: `Blender Sources/cycles_4.5/ui.py`, lines 452–456.
Date read: 2026-07-30.

That is a statement about the panel being drawn. Whether the setting still takes effect when it is
not drawn is not something these files show, and I have not assumed either way.

### Things that end a path rather than a sample

**LABEL: documented.** Two more settings on the same page end work early, but they end individual
light paths inside a sample, not the sampling of the picture. I record them because they sit under
the same "Sampling" heading and could be mistaken for stopping rules.

> "Light Threshold
>
> Probabilistically terminates light samples when the light contribution is below the threshold.
> This avoids wasting time on lights that contribute little to the image.
> Zero disables the test."

> "Min Light Bounces
>
> Minimum number of light bounces for each path,
> after which the integrator uses Russian Roulette to terminate paths that contribute less to the image."

Source: manual, https://docs.blender.org/manual/en/4.5/render/cycles/render_settings/sampling.html
Date read: 2026-07-30.

These change how long a sample takes, not how many samples are taken. **Inferred**, one step: the
quoted words say these terminate *light samples* and *paths*, and reading a light sample as something
other than a render sample is the reader's step. *(Citation check, 2026-08-10: marked the step, which
sat under a "documented" label.)*

### EEVEE

**LABEL: documented** for the quoted setting; **not in the sources** for the absence. The page names
a final-render sample count. It carries no time limit, no adaptive sampling and no early stop of any
kind — but that is an absence, not the page saying EEVEE has none. *(Citation check, 2026-08-10:
this read "The number set is the number used. The page has no time limit, no adaptive sampling and
no early-stop of any kind", all under a "documented" label. The first sentence is not carried by the
quote and is contradicted by this document's own hand tests further down, where five motion-blur
steps turned 64 into `Rendering 65 / 65 samples` and Jitter Camera turned 16 into `Rendering 19 / 19
samples`; the second is an absence and is relabelled.)*

> "Render
>
> Samples
>
> The number of samples to use in the final render."

Source: manual, https://docs.blender.org/manual/en/4.5/render/eevee/render_settings/sampling.html

> "taa_render_samples
>
> Number of samples per pixel for rendering
>
> Type:
>
> int in [1, inf], default 64"

Source: Python API documentation, https://docs.blender.org/api/4.5/bpy.types.SceneEEVEE.html
Date read: 2026-07-30.

I searched the whole EEVEE section of the manual and the whole `SceneEEVEE` API page for `limit`,
`threshold` and `samples`. The only limit-like entries are `light_threshold`, `bokeh_threshold`,
and volumetric sample counts, none of which stop the render. On the same page, the viewport
Samples value is the one that can be set to zero for endless resampling — that is the viewport,
not the final render.

### Workbench

**LABEL: documented.** Workbench does not take an arbitrary number at all; the final-render setting
is a short list of fixed choices.

> "Multisample
>
> The scene will be rendered multiple times with a slight offset.
> The anti-aliasing will be gathered from the multiple renders.
> The number of samples are predefined so it uses the best distribution of the samples.
>
> 5, 8, 11, 16, 32"

Source: manual, https://docs.blender.org/manual/en/4.5/render/workbench/sampling.html

> "render_aa
>
> Method of anti-aliasing when rendering final image"
> …
> "Type:
>
> enum in ['OFF', 'FXAA', '5', '8', '11', '16', '32'], default '8'"

Source: Python API documentation, https://docs.blender.org/api/4.5/bpy.types.SceneDisplay.html
Date read: 2026-07-30.

### One more sample count that exists alongside

**LABEL: documented.** Grease Pencil carries its own final-render sample count, separate from the
engine's, and it is new in 4.5.

> "The SSAA sample count can be controlled with the `Grease Pencil` > `Render` > `SSAA Samples` render setting."

Source: release notes, https://developer.blender.org/docs/release_notes/4.5/grease_pencil/

> "aa_samples
>
> Number of supersampling anti-aliasing samples per pixel for final render
>
> Type:
>
> int in [1, inf], default 8"

Source: Python API documentation, https://docs.blender.org/api/4.5/bpy.types.SceneGpencil.html
Date read: 2026-07-30.

Setting an engine sample count does not touch this one. **Inferred**, one step: the two quotes show
two separate settings exist, and what one does to the other is past what they say. *(Citation check,
2026-08-10: marked the step, which sat under a "documented" label.)*

### What 4.5 changed here

**LABEL: documented (absence of change).** I read the 4.5 Cycles and Rendering release notes in
full and searched all 4.5 release notes for `sample`, `time limit` and `noise threshold`. Nothing
records a change to sample counts, adaptive sampling, the time limit, the sample subset or the
view-layer override in 4.5. The only sampling change recorded anywhere in the 4.5 notes is the
Grease Pencil super-sampling quoted above.
Source: release notes, https://developer.blender.org/docs/release_notes/4.5/cycles/ and
https://developer.blender.org/docs/release_notes/4.5/rendering/. Date read: 2026-07-30.

### Anything else that could stop it — searched for and not found

**LABEL: not in the sources.** I found no documentation of any other thing that ends a render early:
no documented way to signal a running render to stop from outside, no documented per-frame or
per-job time budget beyond the Cycles Time Limit, and nothing that reduces or caps the sample count
based on the machine, the device, the thread count or the memory available. The searches behind
this are listed in the record above; the terms that returned nothing at all across the manual were
`budget`, `deadline`, `early exit`, `clean enough` and `clamp…sample`, and `Ctrl-C` / `Ctrl-Break`
returned nothing. `abort` returned only `--disable-abort-handler`, described as "Disable the abort
handler", which the page places among debug options and does not connect to rendering.
Source searched: manual, https://docs.blender.org/manual/en/4.5/advanced/command_line/arguments.html
Date read: 2026-07-30.

### Not covered

- Whether the Cycles Time Limit applies to each frame separately or to a whole animation. The
  manual says only "Stops rendering if the time exceeds the limit"; neither the manual nor the
  property description mentions frames or animation. I could not settle this.
- Whether the Time Limit applies per view layer when a scene has several.
- The exact rule that combines maximum samples, minimum samples, the adaptive check, the time limit
  and the sample subset during a render. The scheduling code is not in the Cycles folder I was
  given, only the kernel. I can show what each setting is declared to be and what the per-pixel
  test computes; I cannot show the order of precedence at run time.
- What automatic value Cycles picks when the Noise Threshold is 0, or when Min Samples is 0. Both
  are described as automatic and "based on" the other value, but no formula is given anywhere I
  searched.
- How often the convergence test runs — every sample, or at intervals. Not shown in the files given.
- Whether the view-layer override still applies when the Layer Samples control is hidden.
- What is reported or written when the time limit cuts a render short.

### Outside the interface

**LABEL: partly documented, partly not in the sources.**

Documented: everything above is scene data, and the manual describes the whole final-render
sampling behaviour without reference to how Blender was started. One thing does clearly differ
between a final render and the interface, and it works in favour of the outside case: the
viewport-only settings do not apply. The Cycles page separates "Viewport (Max) Samples" from
"Render (Max) Samples" and has a separate viewport adaptive threshold and a viewport-only Denoising
"Start Sample"; the EEVEE page separates a "Viewport" Samples from a "Render" Samples and notes
"Jittered shadows are always enabled for final renders." A render started from outside is a final
render, so it uses the Render values.

Not in the sources: I found no statement anywhere that adaptive sampling, the time limit, the
sample subset or the view-layer override behave differently in background mode. I searched the
whole Cycles section of the manual for `background`, `headless` and `command line`; the only hit
about launching is a GPU-configuration example, and none of the sampling settings are mentioned as
differing. The command line pages list no argument that sets, caps or overrides a sample count —
the documented Cycles arguments are `--cycles-device` and `--cycles-print-stats` only.
Source: manual, https://docs.blender.org/manual/en/4.5/advanced/command_line/arguments.html and
https://docs.blender.org/manual/en/4.5/advanced/command_line/render.html. Date read: 2026-07-30.

**LABEL: observed.** Cycles' own argument parser accepts exactly two arguments, and neither is a
sample count or a time limit:

> ```
>     parser.add_argument("--cycles-print-stats",
>                         help="Print rendering statistics to stderr",
>                         action='store_true')
>     parser.add_argument("--cycles-device",
> ```

File: `Blender Sources/cycles_4.5/engine.py`, lines 13–16. Date read: 2026-07-30.

I record this as a fact about that file, not as an answer to whether a sample count can be given
from outside by some other route — the sheet says question 16 takes no view on that.

**Settled by test on 2026-08-10. LABEL: observed.** Every one of the fifteen tests recorded under
*Hand-test results* below was run from a terminal with no window at all, on two computers, and each
of the settings above did what this answer describes in that setting: the sample count, adaptive
sampling, the time limit, the sample subset and the Layer Samples setting all took effect. The
"not in the sources" above becomes an observed answer for these two operating systems: nothing
behaved differently for being driven from outside. The viewport copies of these settings have no
bearing on such a render — changing all four left the reported count and every pixel unchanged.
And these settings do travel with the file: eleven of them were saved and read back unchanged.
What is still untested is the comparison against a render started inside Blender's own window,
which neither machine could run this round.

### Operating system

**LABEL: not in the sources** for what the documentation says; **observed** for macOS and Linux
since 2026-08-10. Nothing I read ties any of this to an operating system. The sampling
page, the EEVEE sampling page, the Workbench sampling page and the Cycles settings file state no
platform conditions on sample counts, adaptive sampling, the time limit, the sample subset or the
view-layer override. The platform-dependent things sitting nearest are about hardware rather than
counting: the denoiser choices ("Only available on NVIDIA GPUs when configured in the Cycles Render
Device user preferences") and path guiding ("Path guiding is only available when rendering on a
CPU"), both from
https://docs.blender.org/manual/en/4.5/render/cycles/render_settings/sampling.html, read 2026-07-30.
Neither changes how many samples are taken. I did not find any per-operating-system statement about
sampling anywhere in the pages I searched.

**Settled by test on 2026-08-10, for two operating systems out of three. LABEL: observed.** All
fifteen tests below were run on macOS 26.5.2 on Apple silicon and on Ubuntu 22.04.5 LTS on an AMD
processor, and the two machines returned the same sample count in every single test. Where a figure
differed between them it was a time, or a count reached under a clock, never a count reached under
a sample setting. **Windows is untested and stays untested** — no Windows machine has
run it — so nothing here speaks for it.

---

## Hand-test results — 2026-08-10

**LABEL: observed**, throughout this section.

Fifteen tests were run in Blender 4.5.12 on two computers, on 2026-08-10. **The Creator's own Mac** — a MacBook Pro with an M1 Pro chip, macOS 26.5.2, eight cores, 16 GB of memory, Apple's own graphics and no second card. **A rented Linux machine** — Ubuntu 22.04.5 LTS on kernel 6.8, one NVIDIA GeForce RTX 4060 Ti with 16 GB on driver 580.95.05, twenty-three cores of an AMD EPYC 7K62, 98 GB of memory, and no display server at all. Every test was run on both, and the two machines are two results, never one.

**How they were run.** Each Blender was pointed at a throwaway settings folder of its own, made before use, so no real preferences were involved. Every render was on the processor rather than the graphics card, because a fresh settings folder switches no graphics device on and the scene asks for `CPU`. The rented machine is somebody else's hardware and a share of a bigger chip, so its times are its own and are never read against the Mac's; it was idle when anything was timed (`load average: 0.00, 0.02, 1.30`, nothing above 4% of one core, and `0MiB / 16380MiB` used on the card).

**The scene**, built by Blender 4.5.12 itself from a script so that no file saved by another Blender is involved: 320 by 240, Cycles on the processor, a plane, a rough metal sphere, a glass cube, one large area light and a dark world. Renders were started as `blender -b <file> --python-expr "<settings>" -o <folder> -F PNG -f 1`, and the answer read off Blender's own running `Sample n/N` line — the last one with a non-zero `n` is the count the run reached, `N` the count it was counting towards. One line reading `Sample 0/N` is printed just before `Finished` and is not a result. EEVEE prints `Rendering n / N samples` instead. *(Citation check, 2026-08-10: the script itself is not here, and without it this scene cannot be rebuilt exactly. Repeating these tests on a scene built afresh to this description returned every count that does not depend on how noisy the scene is — 4096/4096, 32/4096, 256/256, 8/8, the six view-layer results, the EEVEE counts — and different figures for everything reached under a clock. A test that turns on a clock needs its scene supplied, not described.)* *(Second batch of hand tests, 2026-08-10: the script is now supplied in full, under “Second batch of hand tests” further down this document, and the four figures that turn on a clock were run again on it.)*

### The results, test by test

The Mac and the rented Linux machine returned **the same sample count in every one of these tests**. Only the times differ, and both are given.

| What was set | What Blender reported (both machines) | Mac time | Linux time |
|---|---|---|---|
| Max Samples 4096, adaptive sampling off | `Sample 4096/4096` | `01:35.95` | `00:43.72` |
| Max Samples 4096, adaptive on at 0.01 | `Sample 4096/4096` | `00:25.52` | `00:12.46` |
| Max Samples 4096, adaptive on at 0.5 | `Sample 32/4096` | `00:01.03` | `00:00.90` |
| Max Samples 2048, subset off | `Sample 2048/2048` | `00:43.22` | `00:21.92` |
| Max Samples 2048, subset on, Length 256 | `Sample 256/256` | `00:05.64` | `00:03.05` |
| the same plus adaptive on at 0.01 | `Sample 256/256` | `00:05.20` | `00:02.91` |
| the same plus a 2-second Time Limit | `Sample 97/256` (Mac), `Sample 193/256` (Linux) | `00:02.20` | `00:02.43` |
| Max Samples 200, viewport numbers set low | `Sample 200/200` | `00:03.37` | `00:01.96` |
| Max Samples 200, viewport numbers set high | `Sample 200/200` | `00:03.30` | `00:01.97` |
| Max Samples 8 with Min Samples 64 | `Sample 8/8` | `00:00.36` | `00:00.49` |
| Max Samples 8 with Min Samples 0 | `Sample 8/8` | `00:00.36` | `00:00.49` |
| Max Samples 1024, adaptive on at 0.01 | `Sample 1024/1024` | `00:09.90` | `00:05.17` |
| Max Samples 1024, adaptive on at 0 | `Sample 1024/1024` | `00:22.63` | `00:11.17` |
| Max Samples 128, adaptive on at 0 | `Sample 128/128` | `00:02.78` | `00:01.59` |
| Max Samples 4096, adaptive on at 0 | `Sample 4096/4096` | `01:17.79` | `00:39.32` |
| stored count 4096, 32 supplied at launch before `-f` | `Sample 32/32` | `00:00.86` | `00:00.73` |
| stored count 4096, 32 supplied at launch after `-f` | `Sample 4096/4096` | `01:35.42` | `00:43.38` |
| EEVEE Render Samples 64, Cycles 4096 in the same file | `Rendering 64 / 64 samples` | `00:00.80` | `00:35.63` |
| EEVEE Render Samples 64, motion blur on, 5 steps | `Rendering 65 / 65 samples` | `00:00.87` | `00:10.74` |
| EEVEE Render Samples 16, Jitter Camera on | `Rendering 19 / 19 samples` | `00:00.29` | `00:06.42` |
| EEVEE Render Samples 16, Jitter Camera off | `Rendering 16 / 16 samples` | `00:00.27` | `00:03.30` |
| Workbench | no sample line of any kind | `00:00.13` | `00:00.99` |
| Max Samples 64 with `-- --cycles-print-stats` | `Sample 64/64`, then a `Render statistics:` block | `00:01.57` | `00:01.04` |

### The Time Limit is spent on each frame, and nothing is printed to say it fired

Three frames in one launch with `-s 1 -e 3 -a`, Time Limit 10 seconds, Max Samples 4096, adaptive sampling off, three repeats on each machine. On the Mac the whole run took 34.12, 33.11 and 33.10 seconds; on the Linux machine 32.91, 32.44 and 33.14. Every one of the six runs wrote three PNG files. Per-frame times in the first repeat were `00:10.75`, `00:11.50`, `00:10.81` on the Mac and `00:10.31`, `00:11.04`, `00:11.03` on the Linux machine.

What the frame printed when the limit ended it, on the Mac, in full:

```
Fra:1 Mem:8.87M (Peak 9.13M) | Time:00:00.01 | Mem:0.32M, Peak:0.32M | Scene, ViewLayer | Sample 0/4096
Fra:1 Mem:13.05M (Peak 13.05M) | Time:00:10.48 | Mem:4.42M, Peak:4.42M | Scene, ViewLayer | Sample 513/4096
Fra:1 Mem:15.39M (Peak 18.91M) | Time:00:10.69 | Mem:4.42M, Peak:4.42M | Scene, ViewLayer | Sample 0/4096
Fra:1 Mem:15.39M (Peak 18.91M) | Time:00:10.69 | Mem:4.42M, Peak:4.42M | Scene, ViewLayer | Finished
Saved: '…/f_0001.png'
Time: 00:10.75 (Saving: 00:00.05)
```

Nothing there names the time limit. The run says `Finished`, saves the frame, and moves on; the only sign the limit fired is the count sitting below the maximum. That answers the *Not covered* item asking what is reported when the limit cuts a render short: nothing is.

### Which number a render counts to when a view layer carries its own

Two halves, because an override below the scene count cannot tell `Bounded` from `Use`. Adaptive sampling off, Time Limit 0, and the scene count supplied at launch rather than saved in the file. Both machines gave these six results identically:

| | second layer's own Samples 64, scene count 1024 | second layer's own Samples 1024, scene count 256 |
|---|---|---|
| `USE` | first layer `Sample 1024/1024`, second `Sample 64/64` | first `Sample 256/256`, second `Sample 1024/1024` |
| `BOUNDED` | first `Sample 1024/1024`, second `Sample 64/64` | first `Sample 256/256`, second `Sample 256/256` |
| `IGNORE` | first `Sample 1024/1024`, second `Sample 1024/1024` | first `Sample 256/256`, second `Sample 256/256` |

So a view layer's own number does stand in place of a scene count supplied from outside under `USE`, is capped by that count under `BOUNDED`, and is thrown away under `IGNORE`. This is *Finding 3* settled by measurement.

**One thing about the second half on the Mac.** Those three runs happened between 15:56 and 15:59 on 2026-08-10, and the Creator's own Blender 5.2 was open on that machine from 15:55:59 onwards — so for that one entry, and no other in this set, a second Blender was running beside the test. It changes nothing this entry rests on: the answer is the counts, the counts do not depend on how busy the machine is, and the same six counts came back from the rented Linux machine, where nothing else was running. The times in the excerpt above were taken under that load, and nothing here is built on them.

### The sampling settings survive a save and a reopen unchanged

Eleven Cycles values and EEVEE's render sample count were set, saved to a new file, and read back by opening that file. Both machines answered:

```
READBACK CYCLES 137 42.0 False 0.004999999888241291 9 BOUNDED True 3 11 64 77
```

In order: the engine, Max Samples 137, Time Limit 42.0, adaptive sampling off, adaptive threshold 0.005, Min Samples 9, Layer Samples `BOUNDED`, sample subset on, subset offset 3, subset length 11, the view layer's own Samples 64, EEVEE's Render Samples 77. So the Layer Samples setting keeps its value across a save and a reopen, whatever the panel does or does not show — which is as far as a terminal reaches on the *Not covered* item about the hidden control.

### The viewport settings have no part in a render with no window

Both runs in the table above reported `Sample 200/200` with the viewport numbers set four different ways, and the two pictures were compared pixel by pixel rather than byte by byte:

```
PIXELS max_difference 0.0 channels_differing 0 of 307200
```

The same on both machines.

### What else can stop a render started from outside

A 4096-sample frame with the Time Limit off was started with no window, left for eight seconds, then interrupted, three ways, on both machines.

An interrupt signal stopped it at the next sample report, wrote no file, and exited 1:

```
Sent an internal break event. Press ^C again to kill Blender
Fra:1 Mem:13.05M (Peak 13.05M) | Time:00:22.61 | Remaining:01:07.73 | Mem:4.42M, Peak:4.42M | Scene, ViewLayer | Sample 1025/4096
Fra:1 Mem:15.39M (Peak 18.91M) | Time:00:22.62 | Remaining:01:07.73 | Mem:4.42M, Peak:4.42M | Scene, ViewLayer | Cancelled
Blender quit
```

On the Linux machine the same three lines came back, with the stop at `Time:00:10.88` and also at `Sample 1025/4096`. A terminate signal printed nothing further, wrote no file, and gave exit status 143. Adding `--disable-abort-handler` to the interrupt run changed nothing at all — the same break line, the same `Cancelled`, exit 1, no file. All the same on both machines.

### EEVEE, and where it differs from what this document records

The engine identifier came from Blender itself: `blender -b --factory-startup -E help` printed `BLENDER_EEVEE_NEXT` in 4.5, alongside `BLENDER_WORKBENCH` and `CYCLES`.

Two EEVEE features that this document does not mention **do exist in 4.5 and do raise the count**: motion blur with five steps turned 64 into `Rendering 65 / 65 samples`, and Jitter Camera depth of field turned 16 into `Rendering 19 / 19 samples`. Both on both machines. The section above saying EEVEE's page has no early stop stands as a statement about the page; these two are additions to the count rather than stops, and were not in this document at all.

Printing the whole EEVEE settings block turned up no time limit. Searched for "time" and "limit", the only entries were:

```
EEVEEPROP gi_diffuse_bounces | Diffuse Bounces | Number of times the light is reinjected inside light grids, 0 disable indirect diffuse light
EEVEEPROP motion_blur_steps | Motion steps | Controls accuracy of motion blur, more steps means longer render time
EEVEEPROP taa_samples | Viewport Samples | Number of samples, unlimited if 0
```

With EEVEE's Render Samples at 256 and a second view layer carrying its own Samples of 8, **both layers rendered 256 samples in 4.5** — `Rendering 256 / 256 samples` twice, with `LAYERS [('ViewLayer', 0), ('Second', 8)]` printed alongside. The same on both machines. Recorded here as what Blender 4.5.12 did; nothing about other versions is stated in this document.

On the rented Linux machine, which has no display server at all, every EEVEE run worked and wrote its image, printing three copies of this line first:

```
EGL Error (0x3009): EGL_BAD_MATCH: Arguments are inconsistent (for example, a valid context requires buffers not supplied by a valid surface).
```

*(Citation check, 2026-08-10: this line did not come back. Repeating the EEVEE runs on the same rented machine today — now on graphics driver 580.173.02, where these tests were recorded at 580.95.05 — printed no EGL line at all, while every EEVEE run still worked and still wrote its image. Both results stand; the question goes back as a test.)* *(Second batch of hand tests, 2026-08-10: run a third time on that machine, still on driver 580.173.02 — again no EGL line, in any of the four versions, and again every EEVEE render worked and wrote its image. All three runs stand; see “Second batch of hand tests” below for the whole log.)*

### Workbench, and the command-line view layer switch

With the engine set to Workbench the run printed no sample line at all, and the setting printed:

```
RENDER_AA '8'
RENDER_AA ITEMS ['OFF', 'FXAA', '5', '8', '11', '16', '32']
CYCLES SAMPLES STILL 4096
```

With `use_single_layer` set to `True` and the first layer active, both view layers rendered anyway — `Sample 256/256` and `Sample 64/64` — so a second layer's own sample count is in play in a command-line render that a person clicking Render would not have reached. The same on both machines.

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

- **Anything on Windows.** The runs used a Mac and a rented Linux machine; no Windows machine has run it. **Untried rather than untestable: a Windows machine can be rented, and every one this project has read reported a Server edition — see the equipment section of `Preparation/Blender Research/Method.md`.**
- **What automatic value Cycles picks when the Noise Threshold or Min Samples is zero.** Nothing a terminal can read prints either number. The tests show the automatic threshold is *not* 0.01 — at Max Samples 1024 a threshold of 0 took `00:22.63` against `00:09.90` for 0.01 on the Mac, and the two pictures differ (`PIXELS max_difference 0.027450978755950928 channels_differing 24778 of 307200`) — and stop there. Whether that value moves with the sample count is also not settled, because the cost of one sample changes with the count on its own.
- **Whether the Time Limit applies per view layer when a scene has several.** Not tested; the timing runs used one view layer.
- **How often the convergence test runs.** Nothing printed reaches it.
- **Whether the Layer Samples control appears only when a view layer carries an override.** That needs eyes on a Blender window. The Creator was given nothing this round and the rented machine has no display server, so no window could be opened on either machine. Whether the setting keeps its value is a different question and is answered above.
- **Whether the Time Limit is counted the same way when the render is on a graphics card.** Part-way through this round the rented machine's own automatic upgrade replaced its NVIDIA libraries with version 580.173 while the loaded driver stayed at 580.95.05, after which Cycles answered `ERROR CUDA cuInit: Unknown error` and offered only the processor. All twenty-four time-limit runs above were on the processor.

---

## Things a person with Blender 4.5 open could settle by hand in a few minutes

Pruned on 2026-08-10: items 1, 2, 4, 5, 6 and 7 were answered by the hand tests above and are removed, and item 3 is narrowed to the part a terminal could not reach. What is left:

1. **How many samples does each pixel really get with adaptive sampling on?** In the View Layer
   properties, enable the "Sample Count" debug pass (Passes ‣ Debug ‣ Sample Count). Render with
   Max Samples 1024 and Noise Threshold at the default 0.01, save as multilayer OpenEXR, and look
   at that pass. It shows samples taken divided by the maximum. The hand test above showed the
   per-pixel skipping in the clock — adaptive sampling on cut the same 4096-sample frame from
   `Time: 01:35.95` to `Time: 00:25.52` — but it did not read the pass, so the spread of counts
   across one picture is still unmeasured.

2. **What does the render window show when the time limit ends a render early?** The terminal's
   wording is recorded above and names the limit nowhere. What the interface's own status line says
   is not recorded, and needs eyes on a window.

---

2026-08-10 — Claude Opus 5 (Claude Code) — Settings, Blender 4.5

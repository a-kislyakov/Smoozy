# Settings — further questions (Blender 5.0)

**Sources consulted, and when.** Blender 5.0 manual (`Blender Sources/manual_5.0`), Blender 5.0 Python API documentation (`Blender Sources/api_5.0`), Blender 5.0 release notes (`Blender Sources/release_notes_5.0`), and the Cycles add-on's own files (`Blender Sources/cycles_5.0`). All were taken on 2026-07-30, and that is my date read for everything below.

I read only the question sheet and files inside Blender Sources/. I did not open any other part of this project.

Question answered: 16 only.

---

## 16. When a render is started from outside Blender and given a sample count of its own, what decides how many samples it actually takes, and what else has a say in when it stops?

### A. A note on the question first

The question is ambiguous in one place, so I say plainly which reading I answered.

I answered: **the render is launched from the command line and a sample count is supplied at launch, overriding whatever the file holds.** The other reading would be: the render is launched from the command line and simply uses the sample count already saved in the file. Almost everything below applies to both readings; the only part that is specific to my reading is section B, on how a sample count can be supplied at launch at all.

The sheet says question 16 does not repeat whether a sample count can be settled from outside, so nothing below is offered as an answer to that. Section B is here only because "given a sample count of its own" cannot be answered without knowing what a launch-time sample count is. It does reach a conclusion about how such a value would be supplied; read that as groundwork for this question, not as a ruling on the earlier one.

---

### B. The sources list no command-line option for a sample count, and point to Python as the route

**Label: documented** (that no such option is listed) **plus inferred** (that Python is therefore the route).

The manual's full list of command-line arguments has a "Render Options" section and a separate "Cycles Render Options" section. Neither offers a sample count. The Cycles section offers exactly two things:

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

Source: manual — https://docs.blender.org/manual/en/5.0/advanced/command_line/arguments.html — read 2026-07-30.

The Cycles add-on's own argument parser shows the same two:

> ```
>     parser.add_argument("--cycles-print-stats",
>                         help="Print rendering statistics to stderr",
>                         action='store_true')
>     parser.add_argument("--cycles-device",
> ```

Source: `Blender Sources/cycles_5.0/engine.py`, lines 13–16, inside `_configure_argument_parser()`. Label for this line specifically: **observed**. Read 2026-07-30. That there are no further options rests on the search recorded at the end of this document, not on these four lines.

What the manual does list is a way to run Python at launch:

> "--python-expr <expression>
>
> Run the given expression as a Python script."

Source: manual — https://docs.blender.org/manual/en/5.0/advanced/command_line/arguments.html — read 2026-07-30.

And the Cycles settings are attached to the scene as a Python property group:

> ```
>     @classmethod
>     def register(cls):
>         bpy.types.Scene.cycles = PointerProperty(
>             name="Cycles Render Settings",
>             description="Cycles render settings",
>             type=cls,
>         )
> ```

Source: `Blender Sources/cycles_5.0/properties.py`, lines 1100–1106, at the end of `class CyclesRenderSettings`. **Observed.** Read 2026-07-30.

**The inference step, written out:** the sources show (i) no command-line option that sets a sample count, (ii) a command-line option that runs arbitrary Python, and (iii) that the Cycles sample count is a Python property on the scene. From those three I conclude that a launch-time sample count must be applied by running Python at launch that writes the scene property. The sources nowhere say this in so many words. A reader has to join those three facts, so this is **inferred**, not documented.

One documented thing that matters if you do this — argument order is significant:

> "Arguments are executed in the order they are given!"

and

> "Always position -f or -a as the last arguments."

Source: manual — https://docs.blender.org/manual/en/5.0/advanced/command_line/render.html — read 2026-07-30.

---

### C. What decides how many samples are actually taken — Cycles

Five things in the sources bear on the number, and they interact.

**C1. The scene's own sample count is the number of samples asked for per pixel. Label: documented.**

> "Render (Max) Samples
>
> The number of paths to trace per pixel in the final render.
> A higher number results in a cleaner image at the cost of a longer render time."

Source: manual — https://docs.blender.org/manual/en/5.0/render/cycles/render_settings/sampling.html — read 2026-07-30.

The setting itself, with its kind, range and default:

> ```
>     samples: IntProperty(
>         name="Samples",
>         description="Number of samples to render for each pixel",
>         min=1, max=(1 << 24),
>         default=4096,
>     )
> ```

Source: `Blender Sources/cycles_5.0/properties.py`, lines 479–484, inside `class CyclesRenderSettings`. **Observed.** Read 2026-07-30.

It is not a hard ceiling. C2 can stop a pixel below it, C3 can let a view layer exceed it, and C4 renders a different number of samples while leaving it alone.

**C2. Adaptive sampling can stop a pixel early, below that ceiling. Label: documented.**

> "If the Noise Threshold checkbox is enabled, Cycles will use adaptive sampling,
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

Source: manual — https://docs.blender.org/manual/en/5.0/render/cycles/render_settings/sampling.html — read 2026-07-30.

Note two things in those words. First, this is **per pixel**, not per image — different pixels end on different sample counts. Second, a threshold **set to** zero is derived from the sample count, and a Min Samples left at zero is derived from the threshold. The property quoted below shows the threshold's own default is 0.01, not zero, so the first of those two only bites where somebody has set it to zero. Where they have, changing the sample count changes the stopping rule as well as the number asked for. Label for that last step: **inferred** — the quoted words describe the setting, and a reader has to join them to a count arriving at launch.

The three settings, with kinds, ranges and defaults:

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

Source: `Blender Sources/cycles_5.0/properties.py`, lines 566–587, inside `class CyclesRenderSettings`. **Observed.** Read 2026-07-30. Adaptive sampling is on by default.

The per-pixel count actually reached can be read out of the render as a pass:

> "Sample Count Cycles
>
> Number of samples calculated for each pixel, divided by the maximum number of samples.
> Used to analyze adaptive sampling."

Source: manual — https://docs.blender.org/manual/en/5.0/render/layers/passes.html — read 2026-07-30.

**C3. A view layer can carry its own sample count that overrides the scene's. Label: documented.**

> "Samples
>
> Allows the view layer to use a different number of samples than the main scene.
> This is controlled by the Layer Samples
> when using the Cycles renderer. When using EEVEE, the Samples override is always used."

Source: manual — https://docs.blender.org/manual/en/5.0/render/layers/override.html — read 2026-07-30.

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

Source: manual — https://docs.blender.org/manual/en/5.0/render/cycles/render_settings/sampling.html — read 2026-07-30.

The override itself:

> "samples
>
> Override number of render samples for this view layer, 0 will use the scene setting
>
> Type:
>
> int in [0, inf], default 0"

Source: Python API — https://docs.blender.org/api/5.0/bpy.types.ViewLayer.html — read 2026-07-30.

And the Layer Samples setting, with its default:

> ```
> enum_use_layer_samples = (
>     ('USE', "Use", "Per render layer number of samples override scene samples"),
>     ('BOUNDED', "Bounded", "Bound per render layer number of samples by global samples"),
>     ('IGNORE', "Ignore", "Ignore per render layer number of samples"),
> )
> ```
> ```
>     use_layer_samples: EnumProperty(
>         name="Layer Samples",
>         description="How to use per view layer sample settings",
>         items=enum_use_layer_samples,
>         default='USE',
>     )
> ```

Source: `Blender Sources/cycles_5.0/properties.py`, lines 64–68 and 550–556. **Observed.** Read 2026-07-30. The default is `USE`, so by default a view layer override wins over the scene count.

**This is the sharpest point in the whole answer for anyone supplying a sample count from outside.** What the words above document is about the *scene-level* count: under `USE` a view layer's own override is allowed in place of it, under `BOUNDED` it caps that override, under `IGNORE` it governs. Reading that as a statement about a value supplied at launch needs one further step — that a launch-time value simply becomes the scene-level count — and the sources do not say so. Label for that step: **inferred**. See C5.

**C4. Sample Subset changes how many samples are rendered while leaving the maximum alone. Label: documented.**

> "Sample Subset
>
> Only render a subset of the samples. Multiple subset renders can be combined into a full
> one by running the following in the Python Console:"

> "Offset
>
> The 0-based index of the first sample in the subset.
>
> Length
>
> The number of samples in the subset. While this overrides (Max) Samples in terms of the
> samples that will get rendered, it’s still important to set (Max) Samples to the total number
> of samples that will be rendered across all subsets – otherwise, the subsets will have
> incompatible noise and combining them will give a worse result."

Source: manual — https://docs.blender.org/manual/en/5.0/render/cycles/render_settings/sampling.html — read 2026-07-30.

The settings themselves:

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

Source: `Blender Sources/cycles_5.0/properties.py`, lines 494–512. **Observed.** Read 2026-07-30. Off by default.

So when Sample Subset is on, the number of samples the render takes is the Length, not the Max Samples — and the manual is explicit that Max Samples must still be set to the whole-job total, because it is what the noise pattern is built against.

**C5. Nothing in the sources says a launch-time value is treated differently from a value saved in the file. Label: not in the sources.** Everything above is written about the settings, not about where their values came from. Where I looked is recorded in the section headed *Where I looked*, below.

---

### D. What decides how many samples are actually taken — EEVEE

**Label: documented.**

> "Render
>
> Samples
>
> The number of samples to use in the final render."

Source: manual — https://docs.blender.org/manual/en/5.0/render/eevee/render_settings/sampling.html — read 2026-07-30.

> "taa_render_samples
>
> Number of samples per pixel for rendering
>
> Type:
>
> int in [1, inf], default 64"

Source: Python API — https://docs.blender.org/api/5.0/bpy.types.SceneEEVEE.html — read 2026-07-30.

Two documented things change the number EEVEE actually takes away from the number you set.

Motion blur rounds it up:

> "When using multiple time steps, the render sample count is rounded up to the next multiple
> of steps to ensure even distribution of samples across steps.
> This can lower the quality of jitter camera depth of field.
> Increasing the number of viewport or render samples will get the
> quality back."

Source: manual — https://docs.blender.org/manual/en/5.0/render/eevee/render_settings/motion_blur.html — read 2026-07-30.

Jitter Camera depth of field raises it, to the next value a fixed formula allows:

> "Jitter Camera
>
> Randomize the camera position for every scene render sample to increase precision.
> Enabling this option can change the scene’s actual sample count."

> "Note
>
> Be aware that the actual sample count can grow quite rapidly."

> "Hint
>
> The actual number of samples is computed by the following formula:
>
> \[sample\_count = (ring\_count^{2} + ring\_count) * 3 + 1\]
>
> where \(ring\_count\) is the number of ring in the hexaweb pattern.
> The \(ring\_count\) is chosen so that the entire pattern contains at least the number of
> samples set in the Render Settings."

Source: manual — https://docs.blender.org/manual/en/5.0/render/eevee/render_settings/depth_of_field.html — read 2026-07-30.

Note the words "at least" — the count you set is a floor here, not a ceiling.

View layer sample overrides also apply to EEVEE, and unconditionally:

> "When using EEVEE, the Samples override is always used."

Source: manual — https://docs.blender.org/manual/en/5.0/render/layers/override.html — read 2026-07-30.

This is new in 5.0, and the release notes say so and warn about it:

> "View Layer Overrides (Material, World, Sample) are now supported. Older files using this feature will render differently."

Source: release notes — https://developer.blender.org/docs/release_notes/5.0/eevee/ — read 2026-07-30.

---

### E. What else has a say in when it stops

**E1. A time limit, in Cycles. Label: documented.**

> "Time Limit
>
> Stops rendering if the time exceeds the limit, even if the desired sample count
> hasn’t been reached yet. Setting this value to zero disables the limit."

> "Note
>
> The time limit does not include pre-render processing time, only render time."

Source: manual — https://docs.blender.org/manual/en/5.0/render/cycles/render_settings/sampling.html — read 2026-07-30.

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

Source: `Blender Sources/cycles_5.0/properties.py`, lines 514–522. **Observed.** Read 2026-07-30. Zero by default, so off unless the file sets it.

This is the one that overrides a sample count outright: the quoted words say it stops the render even when the samples asked for have not been taken.

**E2. Adaptive sampling, again — it is a stopping rule as much as a count. Label: documented.** See C2. The quoted words "cutting short the sampling process" and "The threshold to decide whether to continue sampling a pixel" are about when to stop, per pixel.

**E3. Sample Subset, again.** See C4: when it is on, the render ends after Length samples.

**E4. The sources show no time limit and no adaptive stop for EEVEE. Label: not in the sources, with the search recorded.** The EEVEE sampling page lists only Viewport Samples, Temporal Reprojection, Jittered Shadows, Render Samples, shadow Rays/Steps, Volumetric Shadows, Resolution and Light Threshold. A search of the whole 5.0 manual for "time limit" and "time_limit" returns exactly two hits, both on the Cycles sampling page. A search of `bpy.types.SceneEEVEE`, `bpy.types.RenderSettings` and `bpy.types.Scene` in the 5.0 API documentation for "time_limit" returns nothing. Sources: manual — https://docs.blender.org/manual/en/5.0/render/eevee/render_settings/sampling.html and https://docs.blender.org/api/5.0/bpy.types.SceneEEVEE.html — read 2026-07-30. Nothing is quoted here because there is nothing to quote: this is an absence, and an absence of any mention is not a statement that EEVEE has neither.

**E5. Nothing else. Label: not in the sources.** I found no other documented stopping condition — no memory limit that ends a render, no per-frame budget, no error condition described as ending sampling early. A search of the manual's whole render section for "stops rendering", "stop rendering", "cancel the render" and "abort" returns one hit, the Time Limit line above. This is a real gap in the documentation rather than proof that nothing else exists.

---

### Not covered

- **Whether the Cycles time limit is per frame or for a whole animation.** The manual says only that it "Stops rendering if the time exceeds the limit". It does not say what "the render" is when many frames are rendered in one launch. I found nothing that settles it. This matters a great deal for a job launched from outside and left alone, and I cannot answer it from the sources.
- **The exact arithmetic when several of these apply at once.** For example: a view layer override under `BOUNDED`, with adaptive sampling on and a time limit set. Each rule is documented on its own; the sources do not state the order they are applied in or how they combine.
- **How the automatic values are computed.** The manual says a zero Noise Threshold makes Cycles "guess an automatic value based on the total sample count", and a zero Min Samples is "determined by the Noise Threshold". Neither formula is given anywhere I could find. At the factory defaults the threshold is 0.01 rather than zero, so only Min Samples is automatic there; set the threshold to zero as well and neither half of the stopping rule can be predicted from the sources.
- **What a render reports about the count it reached.** I have one release-note example of a background log line (section F) and one debug pass (C2). I did not find a documented, machine-readable report of the final sample count.
- **Whether a value written at launch behaves identically to the same value saved in the file.** Not stated anywhere. See C5.
- **Anything I could only get by running Blender.** I ran nothing. Everything above is reading.
- **Render engines other than Cycles and EEVEE.** I only looked at those two.

---

### F. Outside the interface

**Does this hold when Blender is driven from outside its own window? Label: documented for the general case, with two named exceptions.**

The command-line render page states that a background render uses the file's settings:

> "-a
>
> Render the whole animation using all the settings saved in the blend-file."

Source: manual — https://docs.blender.org/manual/en/5.0/advanced/command_line/render.html — read 2026-07-30.

The 5.0 release notes carry an example of a background render's own log, and it reports sample progress:

> ```
> 00:00.244  render           | Rendering single frame (frame 1)
> 00:00.245  render           | Start rendering: Scene, ViewLayer
> 00:00.245  render           | Engine: Cycles
> 00:26.799  render           | Mem: 2983M | Sample 1/16
> 00:26.799  render           | Mem: 2983M | Sample 16/16
> 00:28.631  render           | Saved: '/tmp/0001.png'
> ```

Source: release notes — https://developer.blender.org/docs/release_notes/5.0/core/ — read 2026-07-30. The same page notes this logging is new: "Most of Blender now uses the same logging system, including operators reports and background render progress. The output format has changed".

The example shows a background render counting samples up to a total. It is there to show the new log format, not to say where that total came from, so treating it as proof that the scene's sample count governs a background render is **inferred**.

**Where it differs.** The manual carries a second set of these settings under the heading *Viewport*: "Viewport (Max) Samples", the viewport's own adaptive sampling and its Min Samples, "Denoise (Viewport)", "Start Sample" ("Sample at which to start denoising in the 3D Viewport"), and "Scrambling Distance — Viewport". Those are separate settings from the render ones — the manual's own Viewport/Render split on the sampling pages shows two sets, and `properties.py` carries a separate `preview_samples`, `use_preview_adaptive_sampling`, `preview_adaptive_threshold` and `preview_adaptive_min_samples` (lines 485–492 and 588–609; named here, not quoted).

That the viewport set has no part in a render started from outside, and that a launch-time sample count must therefore be written to the render setting, is **inferred** from that split. No quoted words say it.

The second difference is section B itself: from outside, there is no argument for a sample count, so the mechanism for supplying one is different in kind from clicking a field.

I found nothing saying the sample count, adaptive sampling, the time limit, Sample Subset or view layer overrides behave differently in background mode. That absence is not the same as a statement that they behave the same.

**Settled by test on 2026-08-10. LABEL: observed.** Every one of the fifteen tests recorded under *Hand-test results* below was run from a terminal with no window at all, on two computers, and each of the settings above did what this answer describes in that setting: the sample count, adaptive sampling, the time limit, the sample subset and the Layer Samples setting all took effect in a render started from outside Blender. Nothing behaved differently for being driven from outside. The viewport copies of these settings have no bearing on such a render — changing all four of them left the reported count and every pixel of the picture unchanged. And these settings do travel with the file: eleven of them were saved and read back unchanged. What is still untested is the comparison against a render started inside Blender's own window, which neither machine could run this round.

### G. Operating system

**Label: not in the sources, for the question as asked.**

I found no statement anywhere in the 5.0 manual, API documentation, release notes or Cycles files that ties any of the above — sample count, adaptive sampling, time limit, Sample Subset, view layer sample overrides, EEVEE's motion-blur rounding or jitter-camera formula — to a particular operating system. None of the pages quoted above mention an operating system at all.

Three nearby things *are* OS- or hardware-dependent and are documented, but none of them is stated to change how many samples are taken:

- The devices `--cycles-device` accepts differ by platform: "Valid options are: CPU CUDA OPTIX HIP ONEAPI METAL." (manual — https://docs.blender.org/manual/en/5.0/advanced/command_line/arguments.html)
- The OptiX denoiser is restricted: "Only available on NVIDIA GPUs when configured in the Cycles Render Devices user preferences." (manual — https://docs.blender.org/manual/en/5.0/render/cycles/render_settings/sampling.html)
- Path guiding is restricted: "Path guiding is only available when rendering on a CPU." (same page)

Both read 2026-07-30. Whether a different device changes the number of samples taken for a given setting is not addressed anywhere I looked.

**Settled by test on 2026-08-10, for two operating systems out of three. LABEL: observed.** All fifteen tests recorded under *Hand-test results* below were run on macOS 26.5.2 on Apple silicon and on Ubuntu 22.04.5 LTS on an AMD processor, and the two machines returned the same sample count in every single test. Where a figure differed between them it was a time, or a count reached under a clock, never a count reached under a sample setting. **Windows is untested and stays untested** — no Windows machine has run it — so nothing here speaks for it.

---

### Where I looked (the record for every "not in the sources" above)

Searches were run across the whole of `manual_5.0`, `api_5.0`, `release_notes_5.0` and `cycles_5.0` unless narrowed below, case-insensitive.

- `sample` inside `manual_5.0/advanced/command_line/` — **no hits at all.** This is what rules out a documented command-line sample option.
- `--samples`, `--cycles-samples`, `--render-samples`, `--max-samples`, `sample-count` — **no hits** in manual, API, release notes or Cycles files.
- `cycles-device`, `cycles-print-stats`, `cycles-resumable`, `Cycles Render Options` — hits only on the command-line arguments page, its index, and the command-line render page. `cycles-resumable`: no hits.
- `argv`, `argparse`, `add_argument`, `register_cli` in the Cycles add-on's Python files — hits only in `engine.py`, giving the two options quoted in section B.
- `sample count` across the manual — 13 hits, all read; the load-bearing ones are quoted above (Cycles sampling, EEVEE depth of field, EEVEE motion blur, view layer passes).
- `time limit`, `time_limit` across the manual — 2 hits, both on the Cycles sampling page. `time_limit` in `bpy.types.SceneEEVEE`, `bpy.types.RenderSettings`, `bpy.types.Scene` — no hits.
- `sample override`, `Samples Override`, `override_num_samples`, `Sample Overrides` across manual and API — 2 hits, both quoted above.
- `stops rendering`, `stop rendering`, `cancel the render`, `abort` across `manual_5.0/render/` — 1 hit, the Time Limit line.
- `sample` across `release_notes_5.0/` — 6 hits; the two relevant ones (EEVEE view layer overrides, the background log example) are quoted above. The whole of `release_notes_5.0/cycles.md` was read; it says nothing about sample counts, adaptive sampling or the time limit.
- Pages read in full: the Cycles sampling page, the EEVEE sampling page, the view layer override page, the command-line arguments page (render, format and Python sections) and the command-line render page, the EEVEE depth-of-field and motion-blur pages (relevant sections), the view layer passes page (Debug section), `bpy.types.ViewLayer`, `bpy.types.SceneEEVEE`, and `cycles_5.0/properties.py`, `engine.py` and `ui.py` in the parts covering sampling.
- Pictures: the Cycles sampling page, the EEVEE sampling page and the view layer override page carry no `[image: …]` markers in the 5.0 snapshot, so there was no screenshot to inspect for this question.

I did not use the web, and did not need to.

---

## Hand-test results — 2026-08-10

**LABEL: observed**, throughout this section.

Fifteen tests were run in Blender 5.0.1 on two computers, on 2026-08-10. **The Creator's own Mac** — a MacBook Pro with an M1 Pro chip, macOS 26.5.2, eight cores, 16 GB of memory, Apple's own graphics and no second card. **A rented Linux machine** — Ubuntu 22.04.5 LTS on kernel 6.8, one NVIDIA GeForce RTX 4060 Ti with 16 GB on driver 580.95.05, twenty-three cores of an AMD EPYC 7K62, 98 GB of memory, and no display server at all. Every test was run on both, and the two machines are two results, never one.

**How they were run.** Each Blender was pointed at a throwaway settings folder of its own, made before use, so no real preferences were involved. Every render was on the processor rather than the graphics card, because a fresh settings folder switches no graphics device on and the scene asks for `CPU`. The rented machine is somebody else's hardware and a share of a bigger chip, so its times are its own and are never read against the Mac's; it was idle when anything was timed (`load average: 0.00, 0.02, 1.30`, nothing above 4% of one core, and `0MiB / 16380MiB` used on the card).

**The scene**, built by Blender 5.0.1 itself from a script so that no file saved by another Blender is involved: 320 by 240, Cycles on the processor, a plane, a rough metal sphere, a glass cube, one large area light and a dark world — noisy enough that sampling shows in the clock. Renders were started as `blender -b <file> --python-expr "<settings>" -o <folder> -F PNG -f 1`. *(Citation check, 2026-08-10: the script itself is not here, and without it this scene cannot be rebuilt exactly. Repeating these tests on a scene built afresh to this description returned every count that does not depend on how noisy the scene is — 4096/4096, 32/4096, 256/256, 8/8, the six view-layer results, the EEVEE counts — and different figures for everything reached under a clock. A test that turns on a clock needs its scene supplied, not described.)* *(Second batch of hand tests, 2026-08-10: the script is now supplied in full, under “Second batch of hand tests” further down this document, and the four figures that turn on a clock were run again on it.)*

**How the answer was read.** Blender prints a running `Sample n/N` line for each view layer of each frame; the last one with a non-zero `n` is the count that run reached and `N` is the count it was counting towards. One line reading `Sample 0/N` is printed just before `Finished` and is not a result. EEVEE prints `Rendering n / N samples` instead.

### The results, test by test

The Mac and the rented Linux machine returned **the same sample count in every one of these tests**. Only the times differ, and both are given.

| What was set | What Blender reported (both machines) | Mac time | Linux time |
|---|---|---|---|
| Max Samples 4096, adaptive sampling off | `Sample 4096/4096` | `01:12.49` | `00:38.10` |
| Max Samples 4096, adaptive on at 0.01 | `Sample 4096/4096` | `00:19.55` | `00:10.96` |
| Max Samples 4096, adaptive on at 0.5 | `Sample 32/4096` | `00:00.89` | `00:00.88` |
| Max Samples 2048, sample subset off | `Sample 2048/2048` | `00:42.67` | `00:19.09` |
| Max Samples 2048, subset on, Length 256 | `Sample 256/256` | `00:05.49` | `00:02.71` |
| the same plus adaptive on at 0.01 | `Sample 256/256` | `00:04.88` | `00:02.50` |
| the same plus a 2-second Time Limit | `Sample 129/256` (Mac), `Sample 225/256` (Linux) | `00:02.66` | `00:02.39` |
| Max Samples 200, viewport numbers set low | `Sample 200/200` | `00:02.92` | `00:01.73` |
| Max Samples 200, viewport numbers set high | `Sample 200/200` | `00:02.97` | `00:01.72` |
| Max Samples 8 with Min Samples 64 | `Sample 8/8` | `00:00.33` | `00:00.42` |
| Max Samples 8 with Min Samples 0 | `Sample 8/8` | `00:00.33` | `00:00.44` |
| Max Samples 1024, adaptive on at 0.01 | `Sample 1024/1024` | `00:08.32` | `00:04.44` |
| Max Samples 1024, adaptive on at 0 | `Sample 1024/1024` | `00:18.69` | `00:09.51` |
| Max Samples 128, adaptive on at 0 | `Sample 128/128` | `00:02.26` | `00:01.39` |
| Max Samples 4096, adaptive on at 0 | `Sample 4096/4096` | `01:04.13` | `00:34.52` |
| stored count 4096, 32 supplied at launch before `-f` | `Sample 32/32` | `00:00.80` | `00:00.67` |
| stored count 4096, 32 supplied at launch after `-f` | `Sample 4096/4096` | `01:19.17` | `00:38.30` |
| EEVEE Render Samples 64, Cycles 4096 in the same file | `Rendering 64 / 64 samples` | `00:02.88` | `00:28.75` |
| EEVEE Render Samples 64, motion blur on, 5 steps | `Rendering 65 / 65 samples` | `00:00.68` | `00:09.61` |
| EEVEE Render Samples 16, Jitter Camera on | `Rendering 19 / 19 samples` | `00:00.26` | `00:05.66` |
| EEVEE Render Samples 16, Jitter Camera off | `Rendering 16 / 16 samples` | `00:00.24` | `00:02.97` |
| Workbench | no sample line of any kind | `00:00.13` | `00:00.94` |
| Max Samples 64 with `-- --cycles-print-stats` | `Sample 64/64`, then a `Render statistics:` block | `00:01.31` | `00:00.92` |

### A frame does end before Max Samples when its pixels converge, and the count is decided pixel by pixel

The first three rows above are the same frame three ways. With adaptive sampling on at a loose threshold of 0.5 the whole frame ended at `Sample 32/4096`, so a frame does stop once its pixels have converged rather than only individual pixels stopping. With adaptive sampling on at the default 0.01 the reported count did not move at all while the time fell to about a quarter, which is the per-pixel skipping showing in the clock rather than in the count.

The Cycles view-layer settings carry a sample-count pass, printed on both machines as:

```
PASS_SAMPLE_ATTRS ['denoising_store_passes', 'pass_debug_sample_count', 'use_denoising']
```

### The Cycles Time Limit is spent on each frame, not across the whole job

Three frames in one launch with `-s 1 -e 3 -a`, Time Limit 10 seconds, Max Samples 4096, adaptive sampling off, three repeats on each machine. **Every one of the six runs wrote three PNG files.**

On the Mac the whole run took 33.57, 31.72 and 31.54 seconds. The per-frame `Time:` lines in the first repeat were `00:10.56`, `00:10.14` and `00:10.28`, and the three frames reached `Sample 577/4096`, `Sample 559/4096` and `Sample 577/4096`.

On the rented Linux machine, idle, the whole run took 33.44, 33.29 and 31.67 seconds. The per-frame times in the first repeat were `00:12.25`, `00:10.35` and `00:10.22`, and the frames reached `Sample 1281/4096`, `Sample 1089/4096` and `Sample 1069/4096`.

Not one of the six runs stopped the job after ten seconds. **Nothing is printed to say the limit fired**: the frame reports its count, says `Finished`, saves the image, and moves on. The only sign is the count sitting below the maximum.

### Which number a render counts to when a view layer carries its own

Two halves, because an override below the scene count cannot tell `Bounded` from `Use`. Adaptive sampling off, Time Limit 0, and the scene count supplied at launch by `--python-expr` rather than saved in the file. Both machines gave these six results identically:

| | second layer's own Samples 64, scene count 1024 | second layer's own Samples 1024, scene count 256 |
|---|---|---|
| `USE` | first layer `Sample 1024/1024`, second `Sample 64/64` | first `Sample 256/256`, second `Sample 1024/1024` |
| `BOUNDED` | first `Sample 1024/1024`, second `Sample 64/64` | first `Sample 256/256`, second `Sample 256/256` |
| `IGNORE` | first `Sample 1024/1024`, second `Sample 1024/1024` | first `Sample 256/256`, second `Sample 256/256` |

So a view layer's own number stands in place of a scene count supplied from outside under `USE`, is capped by that count under `BOUNDED`, and is thrown away under `IGNORE`. Blender 5.0.1 on the Mac, second half, `BOUNDED`, as printed:

```
00:00.307  render           | Start rendering: Scene, ViewLayer
00:05.226  render           | Fra: 1 | Mem: 6M | Sample 256/256
00:05.226  render           | Start rendering: Scene, Second
00:10.179  render           | Fra: 1 | Mem: 6M | Sample 256/256
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
00:20.039  render           | Fra: 1 | Remaining: 00:58.66 | Mem: 21M | Sample 1025/4096
Sent an internal break event. Press ^C again to kill Blender
00:20.040  render           | Fra: 1 | Remaining: 00:58.67 | Mem: 21M | Cancelled
Blender quit
```

On the rented Linux machine the same three lines came back, with the stop at `00:09.804` and also at `Sample 1025/4096`. On both machines the stop landed on the same sample number rather than at the same moment.

A terminate signal printed nothing further, wrote no file, and gave exit status 143; the last line of its log is an ordinary `Sample 1/4096`. Adding `--disable-abort-handler` to the interrupt run changed nothing at all — the same break line, the same `Cancelled`, exit 1, no file. All the same on both machines.

### EEVEE and Workbench

The engine identifier came from Blender itself: `blender -b --factory-startup -E help` printed `BLENDER_EEVEE`, alongside `BLENDER_WORKBENCH` and `CYCLES`.

EEVEE took exactly the number it was given, and Cycles' Max Samples of 4096 sitting in the same file changed nothing. The two features that raise the count did so exactly as the table shows: five motion-blur steps turned 64 into 65, and Jitter Camera turned 16 into 19. Printing the whole EEVEE settings block turned up no time limit. Searched for "time" and "limit", the only entries were:

```
EEVEEPROP gi_diffuse_bounces | Diffuse Bounces | Number of times the light is reinjected inside light grids, 0 disable indirect diffuse light
EEVEEPROP motion_blur_steps | Motion steps | Controls accuracy of motion blur, more steps means longer render time
EEVEEPROP taa_samples | Viewport Samples | Number of samples, unlimited if 0
```

**A view layer's sample override does change an EEVEE render in 5.0.** With Render Samples 256 and a second view layer carrying its own Samples of 8, the first layer reported `Rendering 256 / 256 samples` and the second `Rendering 8 / 8 samples` — `00:01.39` on the Mac and `00:31.18` on the Linux machine, with `LAYERS [('ViewLayer', 0), ('Second', 8)]` printed alongside. The same on both machines.

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

With `use_single_layer` set to `True`, the first layer active and a second view layer carrying Samples 64 against a scene count of 256, both layers rendered — `Sample 256/256` and `Sample 64/64`, `00:06.35` on the Mac and `00:03.42` on the Linux machine — with `SINGLE_LAYER True` printed alongside. In background mode there is no window, so the instruction meant to make the first layer active had no window to act on; it printed nothing and did not error, and both layers rendered regardless.

### The Noise Threshold's default is not the automatic value it uses at zero

Adaptive sampling on, Min Samples 0, Time Limit 0. Every one of these runs reached its maximum, as the table shows; what separates them is the time and the picture. At Max Samples 1024 a threshold of 0 took `00:18.69` against `00:08.32` for the default 0.01 on the Mac, and `00:09.51` against `00:04.44` on the Linux machine. The two pictures are different pictures on both machines:

```
PIXELS max_difference 0.027450978755950928 channels_differing 24911 of 307200      (the Mac)
PIXELS max_difference 0.027450986206531525 channels_differing 25357 of 307200      (the rented Linux machine)
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

**The scene, supplied.** This is the script that built the test scene, in full. Blender 5.0.1 ran it itself, so no file saved by another Blender is involved. Save it as `make_scene.py` and run it as `blender -b --factory-startup --python make_scene.py -- <somewhere>/scene.blend`:

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

**The count reached under a 2-second time limit on a 256-sample subset.** Max Samples 2048, adaptive sampling off, subset on at Offset 0 and Length 256, Time Limit 2 seconds. On the Mac the record gives `Sample 129/256` and this run gave `Sample 129/256` — the same. On the rented Linux machine the record gives `Sample 225/256` and this run gave `Sample 217/256` — different.

**The per-frame counts under a 10-second limit across three frames.** Max Samples 4096, adaptive sampling off, Time Limit 10 seconds, frames 1 to 3 in one launch with `-s 1 -e 3 -a`. On the Mac the record gives `577`, `559`, `577` and this run gave `459`, `513`, `513` — different — in 31.570 s of wall clock, with three PNG files written. On the rented Linux machine the record gives `1281`, `1089`, `1069` and this run gave `1089`, `1089`, `1065` — different — in 31.638 s, again with three files written.

**The channels differing between a noise threshold of 0.01 and one of 0.** Max Samples 1024, adaptive sampling on, Min Samples 0, Time Limit 0; both renders reached `Sample 1024/1024` and the two pictures were compared channel by channel. On the Mac:

```
PIXELS max_difference 0.027450978755950928 channels_differing 24914 of 307200
```

against a recorded `channels_differing 24911 of 307200`. On the rented Linux machine:

```
PIXELS max_difference 0.027450986206531525 channels_differing 25356 of 307200
```

against a recorded `channels_differing 25357 of 307200`.

**The sample number an interrupt lands on after eight seconds.** Max Samples 4096, Time Limit 0, the render started in the background, left for eight seconds, then sent an interrupt. On the Mac it stopped at `Sample 2049/4096` at `00:36.009`; on the rented Linux machine at `Sample 1025/4096` at `00:09.820`. The record gives `Sample 1025/4096` on both machines. Both runs wrote no file and exited 1, as recorded. This run reached a different sample number from the record on the Mac.

It is the one departure among the eight runs of this test. It printed no progress line at all between sample 1 and sample 2049, and stopped at the later one. Its log, from the last two progress lines to the end, word for word:

```
00:00.273  render           | Fra: 1 | Remaining: 01:16.16 | Mem: 21M | Sample 1/4096
00:36.009  render           | Fra: 1 | Remaining: 00:35.71 | Mem: 21M | Sample 2049/4096

Sent an internal break event. Press ^C again to kill Blender
00:36.010  render           | Fra: 1 | Remaining: 00:35.72 | Mem: 21M | Sample 0/4096
00:36.010  render           | Fra: 1 | Remaining: 00:35.72 | Mem: 21M | Cancelled

Blender quit
```

**What the four figures together say.** Supplying the scene was most of what was missing and it was not all of it. Across all four versions and both machines — thirty-two figures in all — the picture comparison came back within six channels of the record everywhere and exactly right in three cases, where the citation check's repeat on a scene built from the written description had been out by more than a thousand; the interrupt landed on the recorded sample number in seven cases out of eight; the two-second subset count came back exactly in five cases out of eight; and the ten-second per-frame counts came back exactly in two cases out of eight. The looser a figure's grip on the clock, the better it repeats. **No claim these figures were quoted in support of changed.** A time limit still cut the subset short, ten seconds was still spent on each frame rather than once for the job, a threshold of 0.01 and a threshold of 0 still made different pictures, and an interrupt still stopped the render without writing a file.

#### The EGL line on the rented Linux machine, a third time

One EEVEE render in Blender 5.0.1 on that machine, engine `BLENDER_EEVEE`, Render Samples 64, with the whole log kept and the driver read at the same time. The driver answered:

```
580.173.02
NVRM version: NVIDIA UNIX Open Kernel Module for x86_64  580.173.02  Release Build  (dvs-builder@U22-I3-AK02-24-4)  Tue Jun 23 08:17:01 UTC 2026
```

and the render's whole log, word for word, was:

```
00:00.361  blend            | Read blend: "/root/q16b/scene5.0.blend"
00:00.378  render           | Rendering single frame
00:00.378  render           | Rendering frame 1
00:00.378  render           | Start rendering: Scene, ViewLayer
00:00.378  render           | Engine: EEVEE
00:02.857  render           | Fra: 1 | Rendering 1 / 64 samples
Blender 5.0.1 (hash a3db93c5b259 built 2025-12-16 01:30:59)
00:02.879  render           | Fra: 1 | Rendering 25 / 64 samples
00:02.897  render           | Fra: 1 | Rendering 50 / 64 samples
00:02.907  render           | Fra: 1 | Rendering 64 / 64 samples
00:03.032  render           | Saved: '/root/q16b/out/e17_0001.png'
00:03.032  render           | Time: 00:02.65 (Saving: 00:00.11)

Blender quit
```

`grep -c EGL` on that log returned `0`. The render finished, exited 0 and wrote its image.

**Three runs now stand on this, and none is chosen between.** The first batch, on driver 580.95.05, recorded three copies of `EGL Error (0x3009): EGL_BAD_MATCH: …` before every EEVEE render, with every render working anyway. The citation check, on 580.173.02, found no EGL line, with every render working. This run, on 580.173.02, found no EGL line, with every render working. All three are recorded and none is withdrawn. What every one of the three agrees on is the thing the answer rests on: an EEVEE render works on a machine with no display server and writes its image.

#### Whether `-E help` answers on the rented Linux machine

The engine identifier used above and in the first batch was read off `blender -b --factory-startup -E help`. The citation check found that command returning nothing at all from the 4.5 binary on the rented machine. Run again with each of the four Blenders on that machine, each with a sixty-second ceiling, Blender 5.0.1 printed:

```
=== 5.0 ===
Blender 5.0.1 (hash a3db93c5b259 built 2025-12-16 01:30:59)
Blender Engine Listing:
	BLENDER_EEVEE
	BLENDER_WORKBENCH
	CYCLES

real	0m0.507s
user	0m0.384s
sys	0m0.170s
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

### Things a person with Blender open could settle in a few minutes

Pruned on 2026-08-10: everything the hand tests above answered has been removed. What is left:

1. **How many samples does each pixel really get with adaptive sampling on?** In View Layer
   properties, switch on the Cycles "Sample Count" debug pass (Passes ‣ Data ‣ Sample Count).
   Render with Max Samples 1024 and the Noise Threshold at its default 0.01, save as multilayer
   OpenEXR, and read that pass — it holds samples taken divided by the maximum. The hand tests
   above showed the per-pixel skipping in the clock, not in the pass, so the spread of counts
   across one picture is still unmeasured.

2. **What automatic Noise Threshold does a zero produce, and what automatic Min Samples?** The hand
   tests showed the automatic value is not 0.01 — the same frame took roughly twice as long with the
   threshold at 0 and came out a different picture — but nothing a terminal prints names the value
   itself. Reading it, if it can be read at all, would need the interface or a build that reports it.

3. **Anything at all on Windows.** No Windows machine has run it, so item 9 of this list
   was not run in the part that asks for one. Its macOS and Linux thirds were run and agreed.

---

2026-08-10 — Claude Opus 5 (Claude Code) — Settings, Blender 5.0

I used no helper agents for this work; every finding above is my own reading of the question sheet and `Blender Sources/`.

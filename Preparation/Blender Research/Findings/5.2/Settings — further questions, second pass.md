# Settings — further questions, second pass (Blender 5.2)

- **Sources consulted, and when.** Blender 5.2 manual (`Blender Sources/manual_5.2`), Blender 5.2 Python API documentation (`Blender Sources/api_5.2`), Blender 5.2 release notes (`Blender Sources/release_notes_5.2`), and Cycles' own files for 5.2 (`Blender Sources/cycles_5.2`). All four are a snapshot taken 2026-07-30, and that is my date read for everything quoted below. Work done 2026-08-10.
- I read only the question sheet and files inside `Blender Sources/`. I did not open any other part of this project.
- One point of full disclosure, so nobody has to wonder. The text of the project's `CLAUDE.md` was placed in front of me automatically at the start, without my asking for it. I did not open it and I did not follow it. For what it is worth, it makes no claim about how Blender behaves — it says only that Blender researchers should stop reading and follow their brief, which is what I did. Nothing else in the project was placed in front of me or opened. I used no helper agents; this answer is entirely my own reading.

---

## 16. When a render is started from outside Blender and given a sample count of its own, what decides how many samples it actually takes, and what else has a say in when it stops?

### How I read the question

I read "given a sample count of its own" as: the run is started from outside Blender and a sample count is supplied to that run, meant to stand in place of whatever the file holds. The question then asks what governs the number of samples that actually get taken, and what else can end the render.

The question is ambiguous in one place, and I say so rather than choose silently. "When it stops" could mean (a) when the sampling of a frame ends, or (b) when an individual light path is cut off inside the renderer. I answer reading (a), because the question pairs "stops" with "how many samples it actually takes". Reading (b) would be about Russian Roulette and light-sampling thresholds, which the manual describes as terminating *paths* and *light samples*, not the render.

The sheet says question 16 "takes no view on" whether a sample count can be settled from outside at all. I therefore do not answer that. But I cannot describe what governs the number without saying where the supplied number lands, so I record, as part of the record of looking, what the launch arguments do and do not contain. That is a statement about the argument list, not a verdict on question 1.

### LABEL

Mixed. Each finding below carries its own label: **documented**, **observed**, **inferred**, or **not in the sources**. The overall shape of the answer is documented for the settings involved, observed for their exact names, kinds, defaults and ranges, and **not in the sources** for several of the "when does it stop" behaviours.

### NOT COVERED (the edge of this answer)

- The sources do not state whether a Cycles time limit applies to each frame separately or to a whole animation run. I found no sentence either way.
- The sources do not state what happens to the render as a whole when every pixel has converged before Max Samples is reached — only that sampling is cut short "in areas".
- The sources do not describe cancelling or interrupting a render that was started from outside Blender (signals, Ctrl-C, a wall-clock kill), nor what is written out if that happens.
- The sources do not give the automatic value Cycles picks when Noise Threshold or Min Samples is left at zero, only that it picks one.
- I did not investigate what happens when a supplied number is outside the accepted range; the sheet places that in question 15.
- I did not examine engines beyond Cycles, EEVEE and Workbench, because those are the ones the 5.2 manual documents under `render/`.

---

### A. Where a supplied sample count can land

**Finding A1 — the documented launch arguments for rendering contain no sample-count option. Label: documented.**

The complete "Render Options" list on the Command Line Arguments page is: `-b/--background`, `-a/--render-anim`, `-S/--scene`, `-f/--render-frame`, `-s/--frame-start`, `-e/--frame-end`, `-j/--frame-jump`, `-o/--render-output`, `-E/--engine`, `-t/--threads`. None concerns samples. The Cycles section that follows it reads in full:

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

Source: manual, https://docs.blender.org/manual/en/5.2/advanced/command_line/arguments.html — read 2026-07-30.

I read the whole of that page (691 lines), including the Environment Variables section at its end. No environment variable listed there concerns samples.

**Finding A2 — Cycles' own argument parser accepts exactly two options, neither of them a sample count. Label: observed.**

File `Blender Sources/cycles_5.2/engine.py`, function `_configure_argument_parser`, lines 8–21:

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

Read 2026-07-30. `--cycles-device` and `--cycles-print-stats` are the only two arguments defined. That is what the file says. What a running Blender then does with any other text after `--` is a step beyond the file; the parser is called with `parse_known_args` (line 32 of the same file), which by its name and by the surrounding code discards what it does not recognise — that last part is **inferred** from the call, not stated.

**Finding A3 — Python is a documented way to run instructions at launch. Label: documented.**

> "-P, --python <filepath>
>
> Run the given Python script file."

> "--python-expr <expression>
>
> Run the given expression as a Python script.
>
> The expression may be a complete multi-line script;
> you are limited only by the platform's maximum argument length."

Source: manual, https://docs.blender.org/manual/en/5.2/advanced/command_line/arguments.html — read 2026-07-30.

**Finding A4 — order of arguments decides whether a supplied value is in place before the render runs. Label: documented.**

> "Arguments are executed in the order they are given!"
>
> "The following command will not work, since the output and extension are set after Blender is told to render"
>
> "Always position -f or -a as the last arguments."

Source: manual, https://docs.blender.org/manual/en/5.2/advanced/command_line/render.html — read 2026-07-30.

And on the arguments page, under "Argument Order":

> "blender --background --render-output /tmp test.blend --render-frame 1
>
> …will not render to /tmp because loading the blend-file overwrites the render output that was set."

Source: manual, https://docs.blender.org/manual/en/5.2/advanced/command_line/arguments.html — read 2026-07-30.

**Inferred step, written out:** the manual states these ordering rules for the render output path and for `-f`/`-a` generally, not for a sample count. To get from those words to "a sample count supplied by a Python argument only has effect if that argument comes after the file is loaded and before `-f` or `-a`", a reader has to apply the general rule to this particular setting. That is my step, so the claim is **inferred**, not documented.

---

### B. What decides how many samples are actually taken — Cycles

**Finding B1 — the number set is a ceiling, not a quota, whenever adaptive sampling is on. Label: documented.**

> "Render (Max) Samples
>
> The number of paths to trace per pixel in the final render.
> A higher number results in a cleaner image at the cost of a longer render time."

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

Source: manual, https://docs.blender.org/manual/en/5.2/render/cycles/render_settings/sampling.html — read 2026-07-30.

So the count actually taken is per pixel, and lies between Min Samples and Max Samples.

**Finding B2 — the exact settings, their kinds, defaults and ranges. Label: observed.**

File `Blender Sources/cycles_5.2/properties.py`, class `CyclesRenderSettings` (which begins at line 384):

Lines 485–498:
```
    samples: IntProperty(
        name="Samples",
        description="Number of samples to render for each pixel",
        min=1, max=(1 << 24),
        default=4096,
    )
    preview_samples: IntProperty(
        name="Viewport Samples",
        description="Number of samples to render in the viewport, unlimited if 0",
        min=0,
        soft_min=1,
        max=(1 << 24),
        default=1024,
    )
```

Lines 572–592:
```
    use_adaptive_sampling: BoolProperty(
        name="Use Adaptive Sampling",
        description="Automatically reduce the number of samples per pixel based on estimated noise level",
        default=True,
    )
    adaptive_threshold: FloatProperty(
        name="Adaptive Sampling Threshold",
        description="Noise level step to stop sampling at, lower values reduce noise at the cost of render time. Zero for automatic setting based on number of AA samples",
        min=0.0,
        max=1.0,
        soft_min=0.001,
        default=0.01,
        precision=4,
    )
    adaptive_min_samples: IntProperty(
        name="Adaptive Min Samples",
        description="Minimum AA samples for adaptive sampling, to discover noisy features before stopping sampling. Zero for automatic setting based on noise threshold",
        min=0,
        max=4096,
        default=0,
    )
```

Read 2026-07-30. Two things this settles: adaptive sampling is **on by default** in this file, so a supplied Max Samples is by default a ceiling rather than a fixed count; and the render count and the viewport count are two separate settings. That a number written into the viewport one therefore has no bearing on a final render is **inferred**, one step — the file shows two properties, not what a render does with either. It is settled by the hand test below, which changed all four viewport numbers and got the same count and the same picture. *(Citation check, 2026-08-10: marked the step, which sat under an "observed" label.)*

**Finding B3 — Cycles decides convergence per pixel, and unconverged pixels drag their neighbours along. Label: observed for what the file says; inferred for the effect on the count.**

File `Blender Sources/cycles_5.2/source/kernel/film/adaptive_sampling.h`. Lines 27–36 and 84–89:

```
/* Determines whether to continue sampling a given pixel or if it has sufficiently converged. */

ccl_device bool film_adaptive_sampling_convergence_check(KernelGlobals kg,
                                                         ccl_global float *render_buffer,
                                                         const int x,
                                                         const int y,
                                                         const float threshold,
                                                         const int reset,
                                                         const int offset,
                                                         const int stride)
```
```
  const bool did_converge = (error < threshold);
```

Lines 92–93:
```
/* This is a simple box filter in two passes.
 * When a pixel demands more adaptive samples, let its neighboring pixels draw more samples too. */
```

Read 2026-07-30.

**Inferred step, written out:** from "when a pixel demands more adaptive samples, let its neighboring pixels draw more samples too", it follows that a pixel already judged converged can still receive further samples because of a noisy neighbour. So the per-pixel count is not settled by that pixel's own error alone. The file describes the mechanism; the consequence for the final count is my step.

**Finding B4 — a view layer can carry its own sample count, and a scene-level setting decides whether it is honoured. Label: documented (manual and API) plus observed (Cycles file).**

Manual:

> "Samples
>
> Allows the view layer to use a different number of samples than the main scene.
> This is controlled by the Layer Samples
> when using the Cycles renderer. When using EEVEE, the Samples override is always used.
>
> Lowering the number of samples for background or utility passes can speed up render times,
> while still using higher samples for beauty passes."

Source: manual, https://docs.blender.org/manual/en/5.2/render/layers/override.html — read 2026-07-30.

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

API, on the view layer's own setting:

> "samples
>
> Override number of render samples for this view layer, 0 will use the scene setting (in [0, inf], default 0)"

Source: Python API, https://docs.blender.org/api/5.2/bpy.types.ViewLayer.html — read 2026-07-30.

Observed, `Blender Sources/cycles_5.2/properties.py`, lines 64–67 and 556–561:

```
enum_use_layer_samples = (
    ('USE', "Use", "Per render layer number of samples override scene samples"),
    ('BOUNDED', "Bounded", "Bound per render layer number of samples by global samples"),
    ('IGNORE', "Ignore", "Ignore per render layer number of samples"),
```
```
    use_layer_samples: EnumProperty(
        name="Layer Samples",
        description="How to use per view layer sample settings",
        items=enum_use_layer_samples,
        default='USE',
    )
```

Read 2026-07-30. The default is `USE`.

**This is the sharpest thing in the answer for the question as asked.** A sample count supplied from outside and written to the scene is not automatically the number used: if any view layer in the file carries a non-zero `samples` override, then with the default `USE` that layer renders at its own number instead. With `BOUNDED` the override is capped by the scene number. Only `IGNORE` makes the scene number authoritative everywhere. **Inferred step, written out:** the manual and the file each describe the setting; putting them together to say "a supplied scene number can be silently displaced by a file's own view-layer override" is my step, so that sentence is inferred.

**Finding B5 — Sample Subset displaces Max Samples for the count actually rendered. Label: documented plus observed.**

> "Sample Subset
>
> Only render a subset of the samples. Multiple subset renders can be combined into a full
> one by running the following in the Python Console:
>
> bpy.ops.cycles.merge_images(input_filepath1=r"1.exr", input_filepath2=r"2.exr",
> output_filepath=r"combined.exr")"

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

Source: manual, https://docs.blender.org/manual/en/5.2/render/cycles/render_settings/sampling.html — read 2026-07-30.

Observed, `Blender Sources/cycles_5.2/properties.py`, lines 500–518:

```
    use_sample_subset: BoolProperty(
        name="Use Sample Subset",
        description="Render a subset of the specified max samples. Typically used for distributed rendering across multiple devices",
        default=False,
    )

    sample_offset: IntProperty(
        name="Sample Subset Offset",
        description="0-based index of sample to start rendering from",
        min=0, max=(1 << 24),
        default=0,
    )

    sample_subset_length: IntProperty(
        name="Sample Subset Length",
        description="The number of samples to render in this subset",
        min=1, max=(1 << 24),
        default=2048,
    )
```

Read 2026-07-30. So if a file has `use_sample_subset` on, a supplied Max Samples does not decide how many samples get rendered; Length does, starting at Offset.

**Finding B6 — the engine chosen decides which sample setting is even consulted. Label: documented.**

> "-E, --engine <engine>
>
> Specify the render engine.
> Use -E help to list available engines."

Source: manual, https://docs.blender.org/manual/en/5.2/advanced/command_line/arguments.html — read 2026-07-30.

EEVEE's own render sample count is a different setting:

> "taa_render_samples
>
> Number of samples per pixel for rendering (in [1, inf], default 64)"

> "taa_samples
>
> Number of samples, unlimited if 0 (in [0, inf], default 16)"

Source: Python API, https://docs.blender.org/api/5.2/bpy.types.SceneEEVEE.html — read 2026-07-30.

Manual, EEVEE:

> "EEVEE uses a process called Temporal Anti-Aliasing (TAA) which reduces Aliasing.
> TAA is sample based so the more samples the more aliasing is reduced at the cost of performance."

> "Render
>
> Samples
>
> The number of samples to use in the final render."

Source: manual, https://docs.blender.org/manual/en/5.2/render/eevee/render_settings/sampling.html — read 2026-07-30.

Workbench does not take a free number at all for final renders:

> "Multisample
>
> The scene will be rendered multiple times with a slight offset.
> The anti-aliasing will be gathered from the multiple renders.
> The number of samples are predefined so it uses the best distribution of the samples.
>
> 5, 8, 11, 16, 32"

Source: manual, https://docs.blender.org/manual/en/5.2/render/workbench/sampling.html — read 2026-07-30.

**Inferred step, written out:** from these three pages together, a sample count supplied from outside has to be written into the setting belonging to the engine that will actually run, and for Workbench there is no arbitrary number to write. The manual never says this in one sentence; assembling it is my step.

---

### C. What else has a say in when it stops

**Finding C1 — a time limit can end a Cycles render before the samples are done. Label: documented plus observed.**

> "Time Limit
>
> Stops rendering if the time exceeds the limit, even if the desired sample count
> hasn't been reached yet. Setting this value to zero disables the limit."

> "Note
>
> The time limit does not include pre-render processing time, only render time."

Source: manual, https://docs.blender.org/manual/en/5.2/render/cycles/render_settings/sampling.html — read 2026-07-30.

Observed, `Blender Sources/cycles_5.2/properties.py`, lines 520–528:

```
    time_limit: FloatProperty(
        name="Time Limit",
        description="Limit the render time (excluding synchronization time). "
        "Zero disables the limit",
        min=0.0,
        default=0.0,
        step=100.0,
        unit='TIME_ABSOLUTE',
    )
```

Read 2026-07-30. Default is zero, that is, off. Note that the manual says "pre-render processing time" and the file says "synchronization time"; I quote both rather than merge them.

Observed, `Blender Sources/cycles_5.2/ui.py`: the time limit appears only in the final-render sampling panel, at line 317 (`col.prop(cscene, "time_limit")`, inside `CYCLES_RENDER_PT_sampling_render`), and not in the viewport panel `CYCLES_RENDER_PT_sampling_viewport` at lines 214–242. Read 2026-07-30.

**Finding C2 — EEVEE has no equivalent time limit in the sources. Label: not in the sources.**

I searched the whole 5.2 manual for "time limit" and "Time Limit": the only file matched was `render/cycles/render_settings/sampling.txt`. I searched the API page `bpy.types.SceneEEVEE` for "time" and "limit": the only matches were an irrelevant light-grid reinjection count, a motion blur step count, and the viewport sample count. Nothing describes an EEVEE render stopping on elapsed time.

**Finding C3 — whether a Cycles frame finishes early once everything has converged is not stated. Label: not in the sources.**

The manual says adaptive sampling cuts short "the sampling process in areas". It does not say that a frame ends before Max Samples once all areas have converged. The Cycles kernel file I was given decides convergence per pixel and says nothing about ending the render; the code that would schedule and end a render session is not among the files in `Blender Sources/cycles_5.2` (that folder holds the add-on's Python files, plus `source/kernel` and `source/util` only). So I cannot settle it and do not guess.

**Finding C4 — cancelling or interrupting a render started from outside Blender is not described. Label: not in the sources.**

Searches for "abort", "cancel", "interrupt", "stop rendering" in the manual return only interface things: Esc cancels a viewport render, the status bar has a cancel button, Esc aborts a fluid bake. One nearby line concerns picking up where you left off, but by hand:

> "May need to interrupt rendering to use the computer, and want to be able to resume rendering where you left off."

Source: manual, https://docs.blender.org/manual/en/5.2/render/output/animation.html — read 2026-07-30. That passage is about choosing to render to image files rather than to a movie file, not about a stop condition.

**Finding C5 — nothing in the 5.2 release notes changes any of this. Label: documented (by absence, with the searches recorded).**

I searched all 21 release-note files for 5.2 for "sample", "time limit", "command line", "--cycles" and "background". The Cycles page for 5.2 is about a texture cache, raycast attributes and shader nodes; the Rendering page is about render output, colour management and shader nodes. Neither mentions sampling limits. The only sampling-adjacent line in the whole set is a bugfix entry:

> " * Blender 5.1.1: Is Cycles render sample progress still accessible from Python without using -f? [[c065cc46602](https://projects.blender.org/blender/blender/commit/c065cc46602ad3d2dbe13814e1ef27cc8f8b5142)]"

Source: release notes, https://developer.blender.org/docs/release_notes/5.2/bugfixes/ — read 2026-07-30. That concerns reading progress from Python, not deciding how many samples are taken. I record it because it is the only hit, not because it answers the question.

---

### Outside the interface

Does this hold when Blender is driven from outside its own window?

- The settings themselves are scene data, not interface state. The Cycles ones live on `CyclesRenderSettings`, registered on the scene (`Blender Sources/cycles_5.2/properties.py`, class beginning line 384), and the per-view-layer one is a documented view-layer property. Nothing in those definitions is conditioned on there being a window. **Observed** for the definitions; the conclusion that they therefore apply identically in a windowless run is **inferred**.
- One documented difference does bear directly on this answer. The API says of the "render single layer" setting:

  > "Only render the active layer. Only affects rendering from the interface, ignored for rendering from command line. (default False)"

  Source: Python API, https://docs.blender.org/api/5.2/bpy.types.RenderSettings.html — read 2026-07-30.

  **Inferred step, written out:** since that switch is ignored from the command line, a run started from outside renders the enabled view layers rather than only the active one, so every view-layer sample override in the file (Finding B4) is in play, including ones a person clicking in the window would never have hit. The API states the switch's behaviour; joining it to the sample overrides is my step.
- The time limit and adaptive sampling are described on the manual's render-settings pages with no mention of the interface, and the Cycles file defines them unconditionally. I found no statement that either behaves differently in a windowless run. **Not in the sources** for any explicit confirmation.
- Two things are specific to being driven from outside: argument order decides whether a supplied value is set before the render starts (Finding A4), and the sample count has to be supplied by some route other than a render argument, because none exists (Findings A1, A2).
- The manual is explicit that command-line rendering needs no display:

  > "One advantage of using the command line is that we do not need a graphical display
  > (no need for X server on Linux for example)
  > and consequently we can render via a remote shell (typically SSH)."

  Source: manual, https://docs.blender.org/manual/en/5.2/advanced/command_line/render.html — read 2026-07-30.

**Settled by test on 2026-08-10. LABEL: observed.** Every one of the fifteen tests recorded under *Hand-test results* below was run from a terminal with no window at all, on two computers, and each of the settings above did what this answer describes in that setting: the sample count, adaptive sampling, the time limit, the sample subset and the Layer Samples setting all took effect in a render started from outside Blender. Nothing behaved differently for being driven from outside. The viewport copies of these settings have no bearing on such a render — changing all four of them left the reported count and every pixel of the picture unchanged. And these settings do travel with the file: eleven of them were saved and read back unchanged, which is the inferred step in the first item above turned into an observed one. The "render single layer" claim was tested too and holds: with that switch on, a command-line render still rendered both view layers. What is still untested is the comparison against a render started inside Blender's own window, which neither machine could run this round.

### Operating system

- The sample settings are defined once in `Blender Sources/cycles_5.2/properties.py` with no operating-system conditions anywhere in `CyclesRenderSettings` (lines 384–1150). The only platform checks in that file are far below, inside the device preferences (lines 1630, 1633, 1881, 1892, 1906, 1923, 1985, 1988), and concern which compute devices are offered. **Observed.** That the sample counts therefore behave the same on every operating system is **inferred** from the absence of any branch.
- The manual states no operating-system difference for Max Samples, Min Samples, Noise Threshold, Time Limit, Sample Subset or the view-layer override. **Not in the sources** for any explicit statement either way.
- Things nearby that are documented as operating-system or hardware bound do not change the sample count, only the speed or the device: `--cycles-device` lists CUDA, OPTIX, HIP, ONEAPI and METAL; the OptiX denoiser is "Only available on NVIDIA GPUs"; and path guiding "is only available when rendering on a CPU" (manual, https://docs.blender.org/manual/en/5.2/render/cycles/render_settings/sampling.html — read 2026-07-30). Whether any of these change the number of samples taken is not stated, and I do not assume it does.

**Settled by test on 2026-08-10, for two operating systems out of three. LABEL: observed.** All fifteen tests recorded under *Hand-test results* below were run on macOS 26.5.2 on Apple silicon and on Ubuntu 22.04.5 LTS on an AMD processor, and the two machines returned the same sample count in every single test. Where a figure differed between them it was a time, or a count reached under a clock, never a count reached under a sample setting. **Windows is untested and stays untested** — no Windows machine has run it — so nothing here speaks for it.

---

### Record of where I looked

Everything below is inside `Blender Sources/`. I opened nothing else in this project.

**Files read in full or in substantial part**

- `manual_5.2/advanced/command_line/arguments.txt` — the whole page, all 691 lines, every section from Render Options through Environment Variables.
- `manual_5.2/advanced/command_line/render.txt` — whole file.
- `manual_5.2/advanced/command_line/index.txt` — whole file, to confirm the section list of the command-line chapter.
- `manual_5.2/render/cycles/render_settings/sampling.txt` — whole file, 322 lines.
- `manual_5.2/render/eevee/render_settings/sampling.txt` — whole file.
- `manual_5.2/render/workbench/sampling.txt` — whole file.
- `manual_5.2/render/layers/override.txt` — whole file.
- `manual_5.2/render/cycles/render_settings/simplify.txt` — whole file, to rule out a sampling control hiding in Simplify. It has Max Subdivision, Child Particles, Texture Resolution, Texture Size Limit, Volume Resolution, Normals, Camera Cull, Distance Cull and Grease Pencil options. No sample count.
- `manual_5.2/render/layers/passes.txt` — the Debug passes section, for the Sample Count pass.
- `cycles_5.2/engine.py` — lines 1–60, the whole argument-parsing part.
- `cycles_5.2/properties.py` — lines 60–70, 384, 480–620, 1523–1610, plus a platform-conditional check across the file.
- `cycles_5.2/ui.py` — lines 210–340 (viewport and render sampling panels), 465–510 (advanced panel and Sample Subset panel).
- `cycles_5.2/source/kernel/film/adaptive_sampling.h` — whole file, 161 lines.
- `api_5.2/bpy.types.ViewLayer.txt` — the `samples` entry.
- `api_5.2/bpy.types.SceneEEVEE.txt` — the sample and time-related entries.
- `api_5.2/bpy.types.RenderSettings.txt` — searched for sample- and background-related entries; found `use_single_layer`.
- `api_5.2/bpy.ops.cycles.txt` — whole file, to see whether any Cycles operator takes a sample count. It does not: the three operators are `denoise_animation`, `merge_images` and `use_shading_nodes`.
- `api_5.2/bpy.ops.render.txt` — the `render.render` signature, to see whether it takes a sample count. It does not: `animation`, `write_still`, `use_viewport`, `use_sequencer_scene`, `layer`, `scene`, `frame_start`, `frame_end`.
- `release_notes_5.2/cycles.md` and `release_notes_5.2/rendering.md` — both in full.

**Searches run, and over what**

Over the whole of `manual_5.2` unless stated:

- `sample` (case-insensitive) across `advanced/command_line/` — no hits at all. This is the basis of Finding A1.
- `--samples`, `--render-samples`, `-samples` across `manual_5.2`, `api_5.2` and `cycles_5.2` — the only hits are unrelated image filenames and a material note about multi-samples.
- `cycles-samples`, `cycles_samples`, `Cycles Render Options` — the last matches three files, all of them pointers to the same section.
- `noise threshold` — one file: `render/cycles/render_settings/sampling.txt`.
- `adaptive sampling` — three files: `render/cycles/render_settings/index.txt`, `render/cycles/render_settings/sampling.txt`, `render/layers/passes.txt`, plus a history page.
- `time limit` and `Time Limit` — one file only: `render/cycles/render_settings/sampling.txt`.
- `stops rendering`, `stop rendering`, `cut short`, `converge`, `terminate`, `abort` — the results are listed under Findings C3 and C4. `stop rendering` has no hits at all. `terminate` hits only path and light-sample termination.
- `progressive`, `resume`, `checkpoint`, `early`, `budget`, `deadline`, `sample count` across `render/` and `advanced/` — `budget`, `deadline` and `checkpoint` have no hits; `progressive` hits only viewport progressive rendering; `resume` hits the two animation-output lines quoted above.
- `environment variable` across `advanced/` — the hits are all about file and resource directories, none about samples.
- `single layer`, `active layer` across `render/layers/` and `interface/` — one hit, `render/layers/view_layer.txt`, "Render Single Layer".
- `sample` across `render/layers/` — two hits, the view-layer override and the Sample Count pass.
- `sample` across `render/cycles/baking.txt` — one line, "Cycles uses the render settings (samples, bounces, …) for baking." I note it and go no further; baking is not what question 16 asks about.
- `getenv` and `environment variable` across the whole of `cycles_5.2` — one hit, a comment about a Metal logging variable. No environment variable sets a sample count.
- `argparse`, `add_argument`, `sys.argv` across `cycles_5.2` — hits only in `maketx.py` (a separate sub-command for texture files) and `engine.py`.
- `samples`, `time_limit`, `adaptive`, `sample_offset`, `sample_subset`, `threshold` across `cycles_5.2/properties.py` — the full list of hits informed Findings B2, B4, B5 and C1.
- `time_limit` across the whole of `cycles_5.2/source` — no hits. The scheduling code that would apply a time limit is not in the files I was given; only `source/kernel` and `source/util` are present.
- `sample`, `time limit`, `command line`, `--cycles`, `background` across all of `release_notes_5.2` — results under Finding C5.

**Pictures**

I searched `render/cycles/render_settings/sampling.txt`, `render/eevee/render_settings/sampling.txt` and `render/layers/override.txt` for `[image:` markers. None of the three carries a picture, so there is no screenshot of the sampling panel in the 5.2 manual to open. I searched the `_images` folder for filenames containing "sampling"; the only match is a legacy image-texture panel, unrelated. So no picture contributed to this answer.

**What I ruled out and why**

- Baking (`render/cycles/baking.txt`): it reuses the render settings, but a bake is not the render the question asks about.
- Light Threshold, Min Light Bounces, Min Transparent Bounces, Russian Roulette (`render/cycles/render_settings/sampling.txt`, `light_paths.txt`): these terminate light paths and light samples, not the render. Under my stated reading of "stops", they are outside the question.
- Denoising, Path Guiding, Scrambling Distance, Sampling Pattern, Seed (same page): these change what a sample is worth or what it looks like, not how many are taken or when the render ends. Scrambling Distance's "Automatic" mode reads the sample count rather than setting it.
- Workbench anisotropic filtering and Grease Pencil SSAA samples: sample counts for texture filtering and line anti-aliasing, not for the render's sampling loop.
- Viewport and preview settings (`preview_samples`, `taa_samples`, Denoise Start Sample): they govern the 3D viewport, not a final render. I mention them only to warn that a supplied number written into the wrong one has no effect.
- The manual, API and release notes for 4.5, 5.0 and 5.1 are on disk beside the 5.2 ones. I did not read them. Every question on the sheet is about 5.2.

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

### Things a person at a computer with Blender 5.2 open could settle in a few minutes

Pruned on 2026-08-10: everything the hand tests above answered has been removed. What is left:

1. **How many samples does each pixel really get with adaptive sampling on?** In View Layer
   properties, switch on the Cycles "Sample Count" debug pass (Passes ‣ Data ‣ Sample Count).
   Render with Max Samples 1024 and the Noise Threshold at its default 0.01, save as multilayer
   OpenEXR, and read that pass — it holds samples taken divided by the maximum. The hand tests
   above showed the per-pixel skipping in the clock, not in the pass, so the spread of counts
   across one picture is still unmeasured.

2. **What automatic Noise Threshold does a zero produce, and what automatic Min Samples?** The hand
   tests showed the automatic value is not 0.01, and stopped there; nothing a terminal prints names
   the value.

3. **Whether the Layer Samples control is drawn only when a view layer carries an override.** That
   needs eyes on a Blender window, which neither machine could give this round. Whether the setting
   keeps its value is a different question and was answered: it survives a save and a reopen.

---

2026-08-10 — Claude Opus 5 (Claude Code) — Settings, Blender 5.2

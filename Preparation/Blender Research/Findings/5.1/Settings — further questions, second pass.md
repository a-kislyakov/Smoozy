# Settings — further questions, second pass (Blender 5.1)

Sources consulted, all on 2026-07-30 (the snapshot date given to me, which I use as the date read throughout): the Blender 5.1 manual, the Blender 5.1 Python API documentation, the Blender 5.1 release notes, and Cycles' own files for 5.1.

I read only the question sheet and files inside Blender Sources/. I did not open any other part of this project.

Question answered here: 16 only.

---

## 16. When a render is started from outside Blender and given a sample count of its own, what decides how many samples it actually takes, and what else has a say in when it stops?

### The reading I took

The question is ambiguous in one place, so I say plainly which way I read it.

I read "given a sample count of its own" as: *for this run, a sample count is in place that is not necessarily the one the file was saved with* — and the question then asks what happens to that number once the render begins. I do not take a view on how the number got there; the sheet says question 16 does not repeat question 1, so I have not tried to settle whether a sample count can be set from outside at all.

The other reading would be: "the render is handed a sample count by some mechanism, and the question is which mechanism". I have not answered that reading, because it is question 1's ground.

One piece of context that shaped my reading, offered as context rather than as an answer to question 1: I searched the whole of the manual's `advanced/` section, which contains every command-line page, for any word beginning "sampl" and found **zero** occurrences. The only Cycles-specific command-line options that exist are two, and neither is a sample count. So the count, however it arrives, does not arrive as a plain command-line switch that the manual documents.

### Short version of the answer

Six things decide how many samples are actually taken, and three of them can stop the render before the count is reached:

1. **Which render engine runs.** Each engine has its own, separate sample setting; a count parked in the wrong engine's setting is simply not read.
2. **The setting's own accepted range**, which clamps the number before anything renders.
3. **A per-view-layer override**, which can replace the scene-level count for that layer.
4. **Cycles' Layer Samples policy** (Use / Bounded / Ignore), which decides whether that override is honoured, capped, or thrown away. EEVEE has no such policy — the override is always used.
5. **Cycles' Sample Subset**, which, when on, decides how many samples are actually rendered regardless of the max.
6. **Cycles' adaptive sampling**, on by default, which turns the count into a *ceiling* rather than a target — each pixel stops when it is judged clean enough.

And separately, **Cycles' Time Limit** stops the render on the clock, whatever the sample count says.

Below, each of these with its evidence.

---

### 1. Which engine runs decides which sample setting is even read

**Label: inferred.**

The manual documents a separate Sampling panel per engine, each with its own sample setting, on separate pages:

- Cycles — "Render (Max) Samples / The number of paths to trace per pixel in the final render. A higher number results in a cleaner image at the cost of a longer render time."
  Source: manual, https://docs.blender.org/manual/en/5.1/render/cycles/render_settings/sampling.html — read 2026-07-30.

- EEVEE — under the heading "Render", "Samples / The number of samples to use in the final render."
  Source: manual, https://docs.blender.org/manual/en/5.1/render/eevee/render_settings/sampling.html — read 2026-07-30.

- Workbench has no free-form sample count at all. It offers a choice of anti-aliasing method, and where it does take multiple samples the counts are a fixed list: "The scene will be rendered multiple times with a slight offset. The anti-aliasing will be gathered from the multiple renders. The number of samples are predefined so it uses the best distribution of the samples." followed by "5, 8, 11, 16, 32".
  Source: manual, https://docs.blender.org/manual/en/5.1/render/workbench/sampling.html — read 2026-07-30.

The step I took: the sources describe three separate settings on three separate engine pages, and the settings live on different data (Cycles' own add-on settings, EEVEE's scene settings, Workbench's anti-aliasing choice). From that I conclude that a count placed in one engine's setting has no effect when a different engine renders. The sources do not state that outright.

**Not covered:** I did not test this, and the sources do not say what happens with third-party engines added as add-ons.

### 2. The setting's accepted range clamps the number

**Label: observed** (Cycles) **and documented** (EEVEE).

Cycles' render sample count, exact lines:

```
    samples: IntProperty(
        name="Samples",
        description="Number of samples to render for each pixel",
        min=1, max=(1 << 24),
        default=4096,
    )
```
Source: Cycles' own files, `Blender Sources/cycles_5.1/properties.py`, lines 479–484, in class `CyclesRenderSettings` — read 2026-07-30.

So the accepted range is 1 to 16777216, and the default is 4096.

EEVEE's render sample count: "Number of samples per pixel for rendering (in [1, inf], default 64)" — the entry for `taa_render_samples`.
Source: Python API documentation, https://docs.blender.org/api/5.1/bpy.types.SceneEEVEE.html — read 2026-07-30.

**Label for the clamping itself: inferred.** The step: the file states a minimum and a maximum for the property, and the API documentation states a range for EEVEE's. I take "min" and "max" to mean the value is held inside those bounds rather than merely advised. Neither source says in so many words what happens to an out-of-range value. (What Blender reports in that case is question 15's ground and I have not gone there.)

**Not covered:** whether an out-of-range value is clamped silently, rejected, or refused, and whether the answer differs between setting it and rendering with it.

### 3. A per-view-layer override can replace the count

**Label: documented.**

"Override number of render samples for this view layer, 0 will use the scene setting (in [0, inf], default 0)" — the `samples` entry on the View Layer type.
Source: Python API documentation, https://docs.blender.org/api/5.1/bpy.types.ViewLayer.html — read 2026-07-30.

And in the manual, under the View Layer Override panel:

"Allows the view layer to use a different number of samples than the main scene. This is controlled by the Layer Samples when using the Cycles renderer. When using EEVEE, the Samples override is always used."
Source: manual, https://docs.blender.org/manual/en/5.1/render/layers/override.html — read 2026-07-30.

Also on that page: "Overrides are applied only when rendering the specific view layer and do not affect the scene globally."

So under EEVEE, a view layer carrying a non-zero override wins over whatever scene-level count is in place, with no policy switch to stop it.

**Not covered:** the sources do not say what happens if the override exceeds EEVEE's own accepted range, nor whether a view layer override interacts with Cycles' Sample Subset.

### 4. Cycles' Layer Samples policy decides whether the override is honoured

**Label: documented,** with the default **observed**.

"Layer Samples / If any view layers have Sample Overrides configured, this option specifies how to use them." with three choices:

- "Use: Allow view layers to override the scene-level sample count."
- "Bounded: Allow the overrides as long as they don't exceed the scene-level sample count."
- "Ignore: Ignore the overrides."

Source: manual, https://docs.blender.org/manual/en/5.1/render/cycles/render_settings/sampling.html — read 2026-07-30.

The exact setting and its default, in Cycles' own files:

```
    use_layer_samples: EnumProperty(
        name="Layer Samples",
        description="How to use per view layer sample settings",
        items=enum_use_layer_samples,
        default='USE',
    )
```
Source: `Blender Sources/cycles_5.1/properties.py`, lines 550–555 — read 2026-07-30.

And the three choices, with their own wording:

```
enum_use_layer_samples = (
    ('USE', "Use", "Per render layer number of samples override scene samples"),
    ('BOUNDED', "Bounded", "Bound per render layer number of samples by global samples"),
    ('IGNORE', "Ignore", "Ignore per render layer number of samples"),
```
Source: `Blender Sources/cycles_5.1/properties.py`, lines 64–67 — read 2026-07-30.

So the default is "Use": under Cycles, out of the box, a view layer's own count wins over the scene-level count.

One more thing worth flagging, because it is easy to miss. The panel that exposes this policy is only drawn when some view layer actually has an override:

```
        for view_layer in scene.view_layers:
            if view_layer.samples > 0:
                layout.separator()
                layout.row().prop(cscene, "use_layer_samples")
                break
```
Source: `Blender Sources/cycles_5.1/ui.py`, lines 471–475 — read 2026-07-30.

**Label: observed** for the lines. **Inferred**, with the step written out: this is code that decides whether to *draw* a control, so what it shows is that the policy can be invisible in the interface while still being set in the file; it does not show that the policy stops applying. I did not find anything stating the policy is skipped when no override exists.

**Not covered:** what "Bounded" does with an override that is below the scene count (the wording implies it is allowed, but the source does not spell out the resulting number), and how any of the three interact with adaptive sampling.

### 5. Cycles' Sample Subset overrides the count outright

**Label: documented,** with the settings **observed**.

"Sample Subset / Only render a subset of the samples." and then, for Length:

"The number of samples in the subset. While this overrides (Max) Samples in terms of the samples that will get rendered, it's still important to set (Max) Samples to the total number of samples that will be rendered across all subsets – otherwise, the subsets will have incompatible noise and combining them will give a worse result."

and for Offset: "The 0-based index of the first sample in the subset."

Source: manual, https://docs.blender.org/manual/en/5.1/render/cycles/render_settings/sampling.html — read 2026-07-30.

The same page gives a worked example: "On both machines, set the (Max) Samples to 2048, disable Denoise, and enable Sample Subset." / "On the first machine, set Offset to 0 and Length to 1024." / "On the second machine, set Offset to 1024 and Length to 1024."

The three settings, exact lines:

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
Source: `Blender Sources/cycles_5.1/properties.py`, lines 494–512 — read 2026-07-30.

So: off by default, but when it is on, the Length is what gets rendered and the given max is used for something else entirely (keeping the noise of the pieces compatible).

**Not covered:** the sources do not say what happens if the subset runs past the max, nor whether the subset is honoured when adaptive sampling is on, nor what happens if denoising is left enabled despite the manual's instruction to disable it.

### 6. Adaptive sampling turns the count into a ceiling, not a target

This is the biggest single factor, and it is on by default.

**Label: documented** for the behaviour, **observed** for the defaults and the mechanism.

The manual:

"If the Noise Threshold checkbox is enabled, Cycles will use adaptive sampling, cutting short the sampling process in areas that have become less noisy than the specified threshold value – in other words, not wasting time further refining areas that already look good enough. For example, hair on a character may need many samples, but the background may only need few."

"Noise Threshold / The threshold to decide whether to continue sampling a pixel. Typical values are in the range from 0.1 to 0.001, with lower values meaning better quality but longer render times. Setting this to 0 makes Cycles guess an automatic value based on the total sample count."

"Min Samples / The minimum number of samples a pixel receives before adaptive sampling is applied. When set to 0 (default), Cycles automatically picks a value determined by the Noise Threshold."

Source: manual, https://docs.blender.org/manual/en/5.1/render/cycles/render_settings/sampling.html — read 2026-07-30.

Note the loop that creates: a Noise Threshold of 0 is worked out from the sample count, and a Min Samples of 0 is worked out from the Noise Threshold. So the given sample count feeds into both ends of the stopping rule when those two are left at their defaults.

The defaults, exact lines:

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
Source: `Blender Sources/cycles_5.1/properties.py`, lines 566–586 — read 2026-07-30.

So adaptive sampling is **on** by default, with a threshold of 0.01 and Min Samples left on automatic.

The interface confirms that the same number is labelled differently depending on whether adaptive sampling is on:

```
        col = layout.column(align=True)
        if cscene.use_adaptive_sampling:
            col.prop(cscene, "samples", text="Max Samples")
            col.prop(cscene, "adaptive_min_samples", text="Min Samples")
        else:
            col.prop(cscene, "samples", text="Samples")
        col.prop(cscene, "time_limit")
```
Source: `Blender Sources/cycles_5.1/ui.py`, lines 311–317 — read 2026-07-30.

**Label: observed.** The same setting is shown as "Max Samples" when adaptive sampling is on and plain "Samples" when it is off.

The mechanism, in Cycles' kernel files. **Label: observed** for the words; the effect on a real render is **inferred** where I say so.

The convergence test itself, with its comment:

```
/* Determines whether to continue sampling a given pixel or if it has sufficiently converged. */
```
and, at the end of that function:

```
  const bool did_converge = (error < threshold);
```
Source: `Blender Sources/cycles_5.1/source/kernel/film/adaptive_sampling.h`, the comment at line 27 and the line at line 84, inside `film_adaptive_sampling_convergence_check` — read 2026-07-30.

Converged pixels are then skipped when new paths are started:

```
  /* Check whether the pixel has converged and should not be sampled anymore. */
  if (!film_need_sample_pixel(kg, state, render_buffer)) {
    return false;
  }
```
Source: `Blender Sources/cycles_5.1/source/kernel/integrator/init_from_camera.h`, lines 69–72, inside `integrator_init_from_camera` — read 2026-07-30.

And a converged pixel's neighbours are kept alive:

```
/* This is a simple box filter in two passes.
 * When a pixel demands more adaptive samples, let its neighboring pixels draw more samples too. */
```
Source: `Blender Sources/cycles_5.1/source/kernel/film/adaptive_sampling.h`, lines 92–93 — read 2026-07-30.

**The inferred step:** from those three quotes together — a per-pixel convergence flag, a skip at path-generation time, and a filter that spreads "keep sampling" to neighbours — I conclude that with adaptive sampling on, different pixels in the same frame take different numbers of samples, and few or none may reach the given count. The files say the code does this; they do not say what any particular render ends up doing.

Two further supports for that conclusion, both **documented**:

"Sample Count / Number of samples calculated for each pixel, divided by the maximum number of samples. Used to analyze adaptive sampling." — a render pass exists precisely to show the per-pixel counts.
Source: manual, https://docs.blender.org/manual/en/5.1/render/layers/passes.html — read 2026-07-30.

And a comment in Cycles' own files, **observed**, which describes the same thing in passing: "meaningful value when adaptive sampler stopped rendering image way before the maximum number of samples was reached".
Source: `Blender Sources/cycles_5.1/source/kernel/film/read.h`, lines 143–146, in a `TODO` comment inside `film_get_pass_pixel_sample_count` — read 2026-07-30.

**Not covered:** the sources do not give the automatic Noise Threshold that a 0 produces, nor the automatic Min Samples, nor how often the convergence check runs, nor the size of the neighbour filter.

### The other thing that has a say in when it stops: the Time Limit

**Label: documented,** with the setting **observed**.

"Time Limit / Stops rendering if the time exceeds the limit, even if the desired sample count hasn't been reached yet. Setting this value to zero disables the limit."

And immediately after, as a Note: "The time limit does not include pre-render processing time, only render time."

Source: manual, https://docs.blender.org/manual/en/5.1/render/cycles/render_settings/sampling.html — read 2026-07-30.

The setting itself:

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
Source: `Blender Sources/cycles_5.1/properties.py`, lines 514–522 — read 2026-07-30.

So it is off by default, and when on it overrides the sample count as the thing that ends the render.

**Not covered, and this is a real gap:** the sources do not say whether the limit applies to each frame separately or to a whole animation job; they do not say what is written out when the limit fires; and they do not say whether denoising still runs on a render cut short this way. I searched every source for "time limit" and found only the two places quoted above plus one unrelated setting on a redraw-timing operator.

### What I looked for and did not find

**Label for all of these: not in the sources.** The record of where I looked is in the section after this one.

- **No documented time limit for EEVEE.** The whole of the 5.1 manual, API documentation, release notes and Cycles files contain exactly two mentions of a render "time limit", both Cycles'. EEVEE's sampling page has no stopping rule of any kind beyond its sample count.
- **No documented adaptive sampling or noise-based stopping for EEVEE.** The word "adaptive" appears nowhere on EEVEE's pages; its only sampling-related appearances in the render section are Cycles' adaptive sampling and, unrelatedly, adaptive subdivision.
- **Nothing in the sources about a sample count on the command line.** Every command-line page in the manual contains no word beginning "sampl". The only Cycles-specific command-line options are `--cycles-device` and `--cycles-print-stats`.
- **Nothing about Esc, Ctrl-C or an interrupt stopping a background render.** The manual mentions cancelling a render job only in general terms for the interface ("the render job can be canceled and resumed at the last rendered frame by changing the frame range", on the output introduction page) and Esc only for a viewport render ("As with a normal render, you can abort it with Esc.").
- **Nothing in the 5.1 release notes about sampling, sample counts, adaptive sampling or stopping rules.** The Cycles and Rendering release-note pages for 5.1 cover performance, a Normal Map node option, a Raycast node, denoising quality with textured transparency, and OpenColorIO — nothing that changes what decides the sample count.

### One extra factor, for files saved by older versions of Blender

**Label: observed,** with an **inferred** step marked.

Cycles' own files contain a versioning handler that changes sampling settings when an old file is loaded. The relevant lines:

```
            if version <= (3, 0, 25):
                cscene = scene.cycles

                # Default changes.
                if not cscene.is_property_set("samples"):
                    cscene.samples = 128
                if not cscene.is_property_set("preview_samples"):
                    cscene.preview_samples = 32
                if not cscene.is_property_set("use_adaptive_sampling"):
                    cscene.use_adaptive_sampling = False
                    cscene.use_preview_adaptive_sampling = False
```
and, a little below:

```
                # Removal of square samples.
                cscene = scene.cycles
                use_square_samples = cscene.get("use_square_samples", False)

                if use_square_samples:
                    cscene.samples *= cscene.samples
                    cscene.preview_samples *= cscene.preview_samples
                    for layer in scene.view_layers:
                        layer.samples *= layer.samples
                    cscene["use_square_samples"] = False
```
Source: `Blender Sources/cycles_5.1/version_update.py`, lines 223–251, inside `do_versions` — read 2026-07-30.

That function is attached as a handler:

```
    bpy.app.handlers.version_update.append(version_update.do_versions)
```
Source: `Blender Sources/cycles_5.1/__init__.py`, line 155 — read 2026-07-30.

and the API documents that handler as running "on ending the versioning code".
Source: Python API documentation, https://docs.blender.org/api/5.1/bpy.app.handlers.html — read 2026-07-30.

**The inferred step:** the manual states "Arguments are executed in the order they are given!" (manual, https://docs.blender.org/manual/en/5.1/advanced/command_line/render.html, read 2026-07-30) and the usage line is "Usage: blender [args ...] [file] [args ...]" (manual, https://docs.blender.org/manual/en/5.1/advanced/command_line/arguments.html, read 2026-07-30). From those, a file named before later arguments is loaded before they run, so this versioning has already happened by the time anything else sets a sample count. What it can still change underneath a freshly-set count is **whether adaptive sampling is on**: for a file saved by a version at or below 3.0.25 that never explicitly set it, adaptive sampling is switched off. A count given to such a file is therefore taken in full rather than treated as a ceiling.

**Not covered:** I did not verify the ordering by running anything, and the sources do not describe the versioning handler's effect anywhere in the documentation — I found it only by reading Cycles' files.

---

### Outside the interface

Every setting above is scene data or view-layer data saved in the file, so it reads the same whether a person clicks Render or something else starts Blender. Three differences I can support:

1. **The viewport counterparts do not apply.** Cycles carries a second, separate set — `preview_samples` ("Number of samples to render in the viewport, unlimited if 0", `properties.py` lines 485–492), `use_preview_adaptive_sampling`, `preview_adaptive_threshold`, `preview_adaptive_min_samples` — and EEVEE likewise carries `taa_samples` ("Number of samples, unlimited if 0 (in [0, inf], default 16)") separately from `taa_render_samples`. The manual describes the viewport ones as applying "in the 3D Viewport (when using the Rendered shading mode)". **Label: inferred** — the step is that a render driven from outside with no window does not use the 3D Viewport, so the viewport settings, including the "unlimited if 0" behaviour, are not the ones in play. The sources do not say this outright. This matters because "0 means unlimited" is true of the viewport counts and *not* of the render counts, whose minimum is 1.

2. **Cycles' own command-line options exist and neither touches samples.** **Label: observed.**
   ```
       parser.add_argument("--cycles-print-stats",
                           help="Print rendering statistics to stderr",
                           action='store_true')
       parser.add_argument("--cycles-device",
   ```
   Source: `Blender Sources/cycles_5.1/engine.py`, lines 13–16, in `_configure_argument_parser` — read 2026-07-30. The manual documents the same two and notes "Cycles add-on options must be specified following a double dash." (manual, https://docs.blender.org/manual/en/5.1/advanced/command_line/arguments.html, read 2026-07-30).

3. **One documented behaviour genuinely differs from the command line, and it touches this answer.** Of the View Layer panel's "Render Single Layer" option, the manual says: "This option is ignored when rendering from the command line."
   Source: manual, https://docs.blender.org/manual/en/5.1/render/layers/view_layer.html — read 2026-07-30.
   **Label: documented** for the quote; **inferred** for the consequence, and the step is this: since each view layer can carry its own sample override (section 3 above), a run started from the command line can render view layers — each with its own sample count — that a person clicking Render with "Render Single Layer" on would not have rendered. So the number of samples taken across the job can differ between the two ways of starting, without any setting having changed.

**Settled by test on 2026-08-10. LABEL: observed.** Every one of the fifteen tests recorded under *Hand-test results* below was run from a terminal with no window at all, on two computers, and each of the settings above did what this answer describes in that setting: the sample count, adaptive sampling, the time limit, the sample subset and the Layer Samples setting all took effect in a render started from outside Blender. Nothing behaved differently for being driven from outside. The viewport copies of these settings have no bearing on such a render — changing all four of them left the reported count and every pixel of the picture unchanged. And these settings do travel with the file: eleven of them were saved and read back unchanged. What is still untested is the comparison against a render started inside Blender's own window, which neither machine could run this round.

**Not covered:** the sources do not say whether the Time Limit, adaptive sampling or Sample Subset behave any differently in a windowless run.

### Operating system

Nothing in the sources ties any of the above to an operating system. I found no statement anywhere in the 5.1 manual, API documentation, release notes or Cycles files that a sample count, a noise threshold, a Min Samples value, a Sample Subset, a Layer Samples policy or a Time Limit behaves differently on one operating system than another.

Two nearby restrictions I did find, neither of which is about the operating system and neither of which changes the sample count:

- "Path guiding is only available when rendering on a CPU." (manual, https://docs.blender.org/manual/en/5.1/render/cycles/render_settings/sampling.html, read 2026-07-30). This restricts a feature by processing device, not by operating system. Its own Training Samples setting is documented as "The maximum number of samples to use for training. A value of 0 will keep training until the end of the render" — that is a count of samples used for training, not a change to how many samples the render takes.
- The list of Cycles devices, "Valid options are: CPU CUDA OPTIX HIP ONEAPI METAL" (manual, https://docs.blender.org/manual/en/5.1/advanced/command_line/arguments.html, read 2026-07-30). Which of those exist plainly depends on the hardware and system, but the sources do not say the device changes how many samples are taken.

**Settled by test on 2026-08-10, for two operating systems out of three. LABEL: observed.** All fifteen tests recorded under *Hand-test results* below were run on macOS 26.5.2 on Apple silicon and on Ubuntu 22.04.5 LTS on an AMD processor, and the two machines returned the same sample count in every single test. Where a figure differed between them it was a time, or a count reached under a clock, never a count reached under a sample setting. **Windows is untested and stays untested** — no Windows machine has run it — so nothing here speaks for it.

**Label: not in the sources**, for the operating-system question itself.

---

## Where I looked

Searches run across the whole of `Blender Sources/manual_5.1`, `Blender Sources/api_5.1`, `Blender Sources/release_notes_5.1` and `Blender Sources/cycles_5.1` unless noted. Case-insensitive throughout.

**Terms searched, and what came back:**

- `render-samples`, `--samples`, `render_samples` — only `taa_render_samples` (EEVEE) and its two index entries. No command-line option.
- `sampl` restricted to `manual_5.1/advanced/` (every command-line page lives there) — **zero hits**. This is the basis for saying no command-line page mentions samples.
- `sample count` — 22 hits, all on the Cycles sampling page, the passes page, and the layers pages already quoted.
- `max samples` — 4 hits: two interface labels in `ui.py`, one description in `properties.py`, one unrelated comment about Sobol sample tables in `kernel/types.h`.
- `noise threshold` — 7 hits, all on the Cycles sampling page and in `properties.py`/`ui.py`.
- `adaptive` — 30-odd hits across the manual; the only sampling-relevant ones are the Cycles sampling page, the Cycles render-settings index, and the Sample Count pass. Everything else is adaptive subdivision, adaptive sculpting resolution, adaptive fluid time steps, an adaptive Grease Pencil simplify mode, adaptive sync in the graphics options.
- `time limit` — 3 hits total: the Cycles sampling page (twice) and `properties.py`. Plus one unrelated `time_limit` argument on `bpy.ops.wm.redraw_timer`.
- `sample subset` — 6 hits: the Cycles sampling page, `ui.py`, `properties.py`.
- `sample offset` — 0 hits as a phrase; found instead as `sample_offset` in `properties.py` and in the kernel work-tile struct.
- `converge` / `convergence` — 103 hits, overwhelmingly about camera perspective and sculpt brushes; the relevant ones are in `kernel/film/adaptive_sampling.h`, `kernel/integrator/init_from_camera.h`, and the device-kernel list in `kernel/types.h`.
- `progressive refine` — 0 hits.
- `timeout` — 19 hits, none about rendering (interface tap-key timeouts, a graphics-driver recovery note, a documentation-server note).
- `deadline` — 0 hits.
- `resumable` — 4 hits, all about fluid simulation caches.
- `square samples` — 1 hit, the versioning code quoted above.
- `stops rendering` / `stop sampling` / `stop rendering` — one hit total, the Time Limit line.
- `cancel`, `abort`, `Ctrl-C` — many hits, none about stopping a render started from outside Blender. The nearest are the output introduction page ("the render job can be canceled and resumed at the last rendered frame by changing the frame range") and the viewport render page ("As with a normal render, you can abort it with Esc.").
- `sample` in the API index (`genindex-all.txt`), scanning every indexed property whose name contains "samples" — turned up `taa_render_samples` and `taa_samples` on SceneEEVEE, `samples` on ViewLayer, `aa_samples` on SceneGpencil, plus shader-effect, brush, light-probe and node properties that are not render sample counts.

**Pages and files opened in full or in part:**

Manual — `render/cycles/render_settings/sampling.txt` (whole), `render/eevee/render_settings/sampling.txt`, `render/workbench/sampling.txt` (whole), `render/layers/override.txt` (whole), `render/layers/view_layer.txt`, `render/layers/passes.txt` (part), `render/output/introduction.txt` (whole), `advanced/command_line/arguments.txt` (whole render/format/Cycles sections, plus the full option list), `advanced/command_line/render.txt` (whole), `render/cycles/render_settings/index.txt` (part).

API documentation — `bpy.types.SceneEEVEE.txt` (sampling entries), `bpy.types.ViewLayer.txt` (`samples` entry), `bpy.ops.render.txt` (the `render` operator's parameter list, to see whether it takes a sample count — it does not: `bpy.ops.render.render(*, animation=False, write_still=False, use_viewport=False, use_sequencer_scene=False, layer='', scene='', frame_start=0, frame_end=0)`), `bpy.app.handlers.txt` (`version_update`), `bpy.ops.wm.txt` (to rule out the `time_limit` hit), `bpy.types.SceneGpencil.txt`, the general index files.

Release notes 5.1 — `cycles.md` (whole), `rendering.md` (whole), plus a search of every file in the folder for "sample", "time limit" and "noise threshold".

Cycles' own files — `properties.py` (the sampling block and the view-layer settings class), `ui.py` (the sampling panels), `engine.py` (whole, for its command-line parsing), `__init__.py` (handler registration), `version_update.py` (the sampling-related versioning), `operators.py` and `presets.py` (searched for "sampl"), and in `source/`: `kernel/film/adaptive_sampling.h` (whole), `kernel/integrator/init_from_camera.h` (part), `kernel/film/read.h` (part), `kernel/film/light_passes.h` (part), `kernel/types.h` (the work-tile struct and the Sobol table constants).

**Pictures.** I checked whether any of the pages that carry this answer have an image, using the manual's own `[image: …]` markers. The Cycles sampling page, the EEVEE sampling page, the Workbench sampling page and the View Layer Override page carry **no images at all**. The View Layer page does (`_images/render_layers_view-layer_panel.png`); I opened it, and it shows only the "Use for Rendering" and "Render Single Layer" checkboxes and a Passes list — no sample setting. So no picture in the 5.1 manual shows the Override panel's Samples field or the Cycles Sampling panel.

**Ruled out, and why.** I did not pursue Cycles' light-path settings (Min Light Bounces, Min Transparent Bounces, Light Threshold, Russian Roulette), because they govern when a single light path is cut short, not how many samples a pixel takes; the question asks about the sample count. I did not pursue denoising beyond noting that the Sample Subset instructions say to disable it, because denoising is described as running on the result rather than deciding when sampling stops. I did not pursue Simplify (searched `render/cycles/render_settings/simplify.txt` and `performance.txt` for "sampl" — zero hits).

**Did I need the web?** No. I did not use it and did not need to.

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

## Things a person with Blender 5.1 open could settle in a few minutes

Pruned on 2026-08-10: everything the hand tests above answered has been removed. What is left:

1. **How many samples does each pixel really get with adaptive sampling on?** In View Layer
   properties, switch on the Cycles "Sample Count" debug pass (Passes ‣ Data ‣ Sample Count).
   Render with Max Samples 1024 and the Noise Threshold at its default 0.01, save as multilayer
   OpenEXR, and read that pass — it holds samples taken divided by the maximum. The hand tests
   above showed the per-pixel skipping in the clock, not in the pass, so the spread of counts
   across one picture is still unmeasured.

2. **Is the image still denoised when the Time Limit fires?** Render one frame with a Time Limit of
   5 seconds, Max Samples 4096 and Denoise (Render) on, then the same with Denoise off, and compare
   the two pictures pixel by pixel. If they differ, denoising ran on the shortened render. The hand
   tests above did not cover this.

3. **Whether the Layer Samples control is drawn only when a view layer carries an override.** That
   needs eyes on a Blender window, which neither machine could give this round. Whether the setting
   keeps its value is a different question and was answered: it survives a save and a reopen.

4. **What automatic Noise Threshold does a zero produce, and what automatic Min Samples?** The hand
   tests showed the automatic value is not 0.01, and stopped there; nothing a terminal prints names
   the value.

---

2026-08-10 — Claude Opus 5 (Claude Code) — Settings, Blender 5.1

Declaration: I ran no helper agents; all of the above is my own reading. I read only the question sheet at `Preparation/Blender Research/Questions/Settings.md` and files inside `Blender Sources/`. I did not open the existing findings document for this version, `Start Here.md`, `CLAUDE.md`, or any other part of this project. I ran no git commands.

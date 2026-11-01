# Settings — further questions, second pass (Blender 5.0)

**Sources consulted, and when.** The Blender 5.0 manual, the Blender 5.0 Python API documentation, the Blender 5.0 release notes, and the Cycles add-on's own files, all as supplied in `Blender Sources/` (`manual_5.0`, `api_5.0`, `release_notes_5.0`, `cycles_5.0`). Snapshot taken 2026-07-30; that is my date read for every citation below. I did the reading and searching on 2026-08-10.

I read only the question sheet and files inside Blender Sources/. I did not open any other part of this project.

One disclosure so that line is exactly true. I did not open the project's `CLAUDE.md`, but its text was placed into my working context automatically before I was given anything to do. I did not act on it and I did not open `Preparation/Groundwork/Start Here.md` or anything it points to. For the record, that file's whole content is a signpost telling agents to go and read the Groundwork folder; it contains no claim about Blender, so there is nothing in it that could have coloured what follows. I also saw, in the same automatic preamble, the names of two changed files under `Preparation/Groundwork/` — names only, no content, and neither is about Blender's behaviour. Nothing else in this project was opened, searched or listed. I used no helper agents.

Question answered: 16 only.

---

## Where I looked

This is the record of the search, not a footnote to it.

**Searches run across the whole of `manual_5.0`, `api_5.0`, `release_notes_5.0` and `cycles_5.0` at once** (case-insensitive unless noted):

`cycles-samples`, `cycles_samples`, `--samples`, `cycles-device`, `taa_render_samples`, `sample`, `samples`, `sample count`, `max samples`, `stop sampling`, `stops rendering`, `converge` / `convergence` / `converged`, `unlimited`, `indefinite`, `time limit` / `time_limit`, `noise threshold`, `adaptive`, `adaptive_threshold`, `adaptive_min_samples`, `sample override` / `override … samples`, `sample_offset`, `sample_subset`, `use_layer_samples`, `cancel`, `abort`, `halt`, `interrupt`, `stop the render` / `cancel the render` / `cancel rendering`, `SIGINT`, `Ctrl-C`, `Esc`, `deadline`, `budget`, `progressive refine`, `resume` / `resumable`, `background`, `python`, `Time Limit`.

**Pages I opened and read in full or in the relevant part:**

- Manual, Cycles sampling: `render/cycles/render_settings/sampling.txt` — the whole page. No `[image: …]` markers on it, so there is no screenshot to open; I checked with a search for `image:` and got nothing.
- Manual, View Layer Override: `render/layers/override.txt` — whole page. No images.
- Manual, EEVEE sampling: `render/eevee/render_settings/sampling.txt` — whole page. No images.
- Manual, EEVEE depth of field: `render/eevee/render_settings/depth_of_field.txt` — the Jitter Camera part.
- Manual, EEVEE motion blur: `render/eevee/render_settings/motion_blur.txt` — the Steps part.
- Manual, Workbench sampling: `render/workbench/sampling.txt` — whole page.
- Manual, Command Line Arguments: `advanced/command_line/arguments.txt` — I listed every one of the 662 lines' worth of option headings to be sure the list of options is complete.
- Manual, Rendering From The Command Line: `advanced/command_line/render.txt` — whole page.
- Manual, Render Passes: `render/layers/passes.txt` — the Debug passes part.
- Manual, Output introduction and Animation workflow: `render/output/introduction.txt`, `render/output/animation.txt`.
- Manual, Cycles Performance and Simplify: `render/cycles/render_settings/performance.txt`, `render/cycles/render_settings/simplify.txt`.
- API: `bpy.types.ViewLayer.txt`, `bpy.types.SceneEEVEE.txt`, `bpy.types.RenderSettings.txt`, `bpy.types.Scene.txt`, `bpy.ops.cycles.txt`.
- Release notes: `core.md` (Logging), `eevee.md` (Breaking Changes), plus searches of `cycles.md`, `rendering.md`, `pipeline_io.md`, `python_api.md`, `bugfixes.md`, `corrective_releases.md`, `index.md`.
- Cycles' own files: `engine.py` (the whole command-line parser), `properties.py` (the sampling block and the view-layer block, and the class register lines), `ui.py` (the sampling panels), `presets.py`, `version_update.py`, and `source/kernel/film/adaptive_sampling.h`.

**What I ruled out, and why:**

- *A samples option on the command line.* There is none in 5.0. The manual's full argument list contains no sample-count option, and the Cycles add-on's own parser defines exactly two options and neither is a sample count (see the quotes below). Searching all four source folders for `cycles-samples`, `cycles_samples` and `--samples` returned one hit only — `--cycles-device` — so this is an absence I checked for by name as well as by reading the list.
- *Cycles Simplify.* Searched and read; it changes subdivision, child particles, textures and culling. It says nothing about samples.
- *Cycles Performance.* Read; it covers threads, tile size and acceleration structures. Nothing about samples or stopping.
- *Cancelling / resuming.* The only thing the manual says about stopping a render part-way is at the level of whole frames in an animation, not samples within a frame. Searches for `SIGINT`, `Ctrl-C`, `abort`, `halt` and `interrupt` in a render sense found nothing about a running render's sample loop.
- *The core API.* `bpy.types.RenderSettings` and `bpy.types.Scene` carry no sample count at all; the only sample counts in the core API are `ViewLayer.samples` and the EEVEE ones. So there is no engine-neutral "samples" setting to talk about — the count belongs to the engine.
- *Cycles' C++ session code.* Not supplied. `cycles_5.0/source` contains only `kernel` and `util`. So the code that owns the sample loop, the time limit and the automatic threshold values is not in front of me, and I have not guessed at it.

---

## 16. When a render is started from outside Blender and given a sample count of its own, what decides how many samples it actually takes, and what else has a say in when it stops?

**A note on the wording first.** "Given a sample count of its own" is ambiguous. I read it as: the run has been handed a sample count that is not simply whatever the file had. The other reading is that the sample count is given as a command-line option of its own; on that reading the answer is short, because Blender 5.0 has no such option (see the first finding). I answer the first reading, and cover the second as a finding rather than by rewriting the question. As instructed, I take no view on whether a sample count can be settled from outside at all.

**LABEL: mixed — documented, observed, and one clearly marked inference. The parts are labelled individually below.**

### The short answer

The count you set is a **ceiling, not a quantity**. What is actually taken is decided by, in this order of effect:

1. a per-view-layer override, which can replace the scene count entirely;
2. adaptive sampling, which stops each pixel early once it is quiet enough;
3. a sample subset, which renders only a slice of the ceiling;
4. a time limit, which stops the render regardless of how many samples have been taken;
5. for EEVEE, two features that round the count *up* rather than down.

Each is below with the words that carry it.

---

### Finding 1 — Blender 5.0 has no command-line option for a sample count

**Label: documented (the manual's list) and observed (the Cycles add-on's own parser).**

The manual's Render Options and Cycles Render Options list every option; the Cycles section is complete in two entries:

> "Cycles add-on options must be specified following a double dash."
> "--cycles-device <device>"
> "--cycles-print-stats"

Source: manual, https://docs.blender.org/manual/en/5.0/advanced/command_line/arguments.html — date read 2026-07-30.

The add-on's own parser agrees, and it is the whole of it:

> `parser.add_argument("--cycles-print-stats",`
> `                    help="Print rendering statistics to stderr",`
> `                    action='store_true')`
> `parser.add_argument("--cycles-device",`
> `                    help="Set the device to use for Cycles, overriding user preferences and the scene setting."`

Source: Cycles' own files, `Blender Sources/cycles_5.0/engine.py`, inside `_configure_argument_parser()`, lines 13–17 (the whole of that function is lines 8–21, and it defines no other option) — date read 2026-07-30.

So a sample count handed to a run from outside has to arrive by some other route. The manual lists Python options including:

> "--python-expr <expression>"
> "Run the given expression as a Python script."

Source: manual, https://docs.blender.org/manual/en/5.0/advanced/command_line/arguments.html — date read 2026-07-30.

And the order in which options are given matters:

> "Arguments are executed in the order they are given!"
> "Always position -f or -a as the last arguments."

Source: manual, https://docs.blender.org/manual/en/5.0/advanced/command_line/render.html — date read 2026-07-30.

**Inferred, one step:** if the sample count is set by something that runs as an argument, it must be given before the argument that starts the render, or the render will already have been done with the file's own count. The step I took: the manual says arguments run in order and that the render arguments must come last; a setting made after the render has run cannot affect that render.

### Finding 2 — The scene count is a maximum, and it lives on the engine, not on the scene generally

**Label: documented and observed.**

Cycles:

> "Render (Max) Samples
> The number of paths to trace per pixel in the final render.
> A higher number results in a cleaner image at the cost of a longer render time."

Source: manual, https://docs.blender.org/manual/en/5.0/render/cycles/render_settings/sampling.html — date read 2026-07-30.

Observed, with its kind, range and default:

> `samples: IntProperty(`
> `    name="Samples",`
> `    description="Number of samples to render for each pixel",`
> `    min=1, max=(1 << 24),`
> `    default=4096,`
> `)`

Source: Cycles' own files, `Blender Sources/cycles_5.0/properties.py`, lines 479–484, in class `CyclesRenderSettings` — date read 2026-07-30. That class attaches to the scene:

> `bpy.types.Scene.cycles = PointerProperty(`

Source: same file, line 1103 — date read 2026-07-30.

The name shown for it changes with whether adaptive sampling is on — observed in the panel code:

> `if cscene.use_adaptive_sampling:`
> `    col.prop(cscene, "samples", text="Max Samples")`
> `    col.prop(cscene, "adaptive_min_samples", text="Min Samples")`
> `else:`
> `    col.prop(cscene, "samples", text="Samples")`
> `col.prop(cscene, "time_limit")`

Source: Cycles' own files, `Blender Sources/cycles_5.0/ui.py`, lines 311–317 — date read 2026-07-30.

EEVEE has its own separate count:

> "taa_render_samples
> Number of samples per pixel for rendering
> Type:
> int in [1, inf], default 64"

Source: Python API, https://docs.blender.org/api/5.0/bpy.types.SceneEEVEE.html — date read 2026-07-30.

For a final render neither engine offers "no limit": Cycles' minimum is 1 and EEVEE's range starts at 1. Only the viewport counts allow zero, and there zero means keep going:

> "Viewport (Max) Samples … Setting this value to zero enables indefinite sampling."

Source: manual, https://docs.blender.org/manual/en/5.0/render/cycles/render_settings/sampling.html — date read 2026-07-30.

Workbench does not work in samples at all for a final render; it offers anti-aliasing methods, with fixed multisample counts:

> "Multisample
> The scene will be rendered multiple times with a slight offset. …
> The number of samples are predefined so it uses the best distribution of the samples.
> 5, 8, 11, 16, 32"

Source: manual, https://docs.blender.org/manual/en/5.0/render/workbench/sampling.html — date read 2026-07-30. So a sample count handed to a Workbench render has nothing to attach to. **Inferred**, one step: the page says the Workbench counts are predefined, not that a number written into some other setting has no effect. Settled by the hand test below, where a Workbench render with Cycles' 4096 in the same file printed no sample line at all. *(Citation check, 2026-08-10: marked the step, which sat under a "documented and observed" label.)*

### Finding 3 — A view layer can override the count, and in Cycles a scene setting decides whether the override is honoured

**Label: documented and observed.** This is the biggest single thing that can make the actual count differ from the count you gave.

> "samples
> Override number of render samples for this view layer, 0 will use the scene setting
> Type:
> int in [0, inf], default 0"

Source: Python API, https://docs.blender.org/api/5.0/bpy.types.ViewLayer.html — date read 2026-07-30.

> "Samples
> Allows the view layer to use a different number of samples than the main scene.
> This is controlled by the Layer Samples
> when using the Cycles renderer. When using EEVEE, the Samples override is always used."

Source: manual, https://docs.blender.org/manual/en/5.0/render/layers/override.html — date read 2026-07-30.

The Cycles control has three settings:

> "Layer Samples
> If any view layers have Sample Overrides configured, this option specifies how to use them.
> Use:
> Allow view layers to override the scene-level sample count.
> Bounded:
> Allow the overrides as long as they don't exceed the scene-level sample count.
> Ignore:
> Ignore the overrides."

Source: manual, https://docs.blender.org/manual/en/5.0/render/cycles/render_settings/sampling.html — date read 2026-07-30.

Observed, with the default:

> `('USE', "Use", "Per render layer number of samples override scene samples"),`
> `('BOUNDED', "Bounded", "Bound per render layer number of samples by global samples"),`
> `('IGNORE', "Ignore", "Ignore per render layer number of samples"),`

and

> `use_layer_samples: EnumProperty(`
> `    name="Layer Samples",`
> `    description="How to use per view layer sample settings",`
> `    items=enum_use_layer_samples,`
> `    default='USE',`
> `)`

Source: Cycles' own files, `Blender Sources/cycles_5.0/properties.py`, lines 64–67 and 550–555 — date read 2026-07-30.

So with Cycles at its default, a view layer that carries its own count wins over the scene count — including over a count handed in from outside, if that count was written to the scene. "Bounded" makes the scene count a cap on the layer's; "Ignore" throws the layer's away.

That the Cycles panel only shows this control when some layer has an override is observed here:

> `for view_layer in scene.view_layers:`
> `    if view_layer.samples > 0:`
> `        layout.separator()`
> `        layout.row().prop(cscene, "use_layer_samples")`
> `        break`

Source: Cycles' own files, `Blender Sources/cycles_5.0/ui.py`, lines 471–475 — date read 2026-07-30. The setting exists whether or not it is shown; only its visibility is conditional.

EEVEE honouring this override is new in 5.0, and the release notes say so plainly:

> "View Layer Overrides (Material, World, Sample) are now supported. Older files using this feature will render differently."

Source: release notes, https://developer.blender.org/docs/release_notes/5.0/eevee/ — date read 2026-07-30.

### Finding 4 — Adaptive sampling stops pixels early, so most pixels never reach the ceiling

**Label: documented, plus one observed detail from Cycles' own code.** Cycles only; EEVEE has no equivalent in the sources.

> "Adaptive Sampling
> If the Noise Threshold checkbox is enabled, Cycles will use adaptive sampling,
> cutting short the sampling process in areas that have become less noisy than the
> specified threshold value – in other words, not wasting time further refining
> areas that already look good enough."

> "Noise Threshold
> The threshold to decide whether to continue sampling a pixel.
> Typical values are in the range from 0.1 to 0.001, with lower values meaning better
> quality but longer render times.
> Setting this to 0 makes Cycles guess an automatic value based on the total sample count."

> "Min Samples
> The minimum number of samples a pixel receives before adaptive sampling is applied.
> When set to 0 (default), Cycles automatically picks a value determined by the Noise Threshold."

Source: manual, https://docs.blender.org/manual/en/5.0/render/cycles/render_settings/sampling.html — date read 2026-07-30.

Note the loop in that: with the threshold at zero, the count you gave decides the threshold, and the threshold decides how early pixels stop. With Min Samples at zero, the threshold decides the floor.

Observed — adaptive sampling is on by default, and both automatic values are the default:

> `use_adaptive_sampling: BoolProperty(`
> `    name="Use Adaptive Sampling",`
> `    description="Automatically reduce the number of samples per pixel based on estimated noise level",`
> `    default=True,`
> `)`

> `adaptive_threshold: FloatProperty(` … `min=0.0,` `max=1.0,` `soft_min=0.001,` `default=0.01,` `precision=4,` `)`

> `adaptive_min_samples: IntProperty(` … `min=0,` `max=4096,` `default=0,` `)`

Source: Cycles' own files, `Blender Sources/cycles_5.0/properties.py`, lines 566–586 — date read 2026-07-30. (Note the ceiling on Min Samples is 4096, far below the ceiling of 16,777,216 on Samples.)

Observed, and worth knowing because it means the stopping is not purely per-pixel — a comment in the kernel:

> `/* This is a simple box filter in two passes.`
> ` * When a pixel demands more adaptive samples, let its neighboring pixels draw more samples too. */`

Source: Cycles' own files, `Blender Sources/cycles_5.0/source/kernel/film/adaptive_sampling.h`, lines 92–93, above `film_adaptive_sampling_filter_x` — date read 2026-07-30. A noisy pixel therefore drags its neighbours along.

The convergence test itself is in the same file: a per-pixel error is compared with the threshold, and

> `  const bool did_converge = (error < threshold);`

Source: same file, line 84, inside `film_adaptive_sampling_convergence_check` — date read 2026-07-30. **This is a file reading, not a render observation.** What the file says is that the comparison exists and takes that form; what a given scene does with it is a step beyond, and I have not taken that step.

### Finding 5 — A sample subset renders only a slice of the ceiling

**Label: documented and observed.** Cycles only.

> "Sample Subset
> Only render a subset of the samples."

> "Offset
> The 0-based index of the first sample in the subset.
> Length
> The number of samples in the subset. While this overrides (Max) Samples in terms of the
> samples that will get rendered, it's still important to set (Max) Samples to the total number
> of samples that will be rendered across all subsets – otherwise, the subsets will have
> incompatible noise and combining them will give a worse result."

Source: manual, https://docs.blender.org/manual/en/5.0/render/cycles/render_settings/sampling.html — date read 2026-07-30.

The same page spells out the intended use and tells you to turn denoising off for it:

> "A typical use case is to distribute the rendering of a single frame over multiple machines."
> "On both machines, set the (Max) Samples to 2048, disable Denoise, and enable Sample Subset."

Source: same page — date read 2026-07-30.

Observed, with kinds, ranges and defaults:

> `use_sample_subset: BoolProperty(` … `description="Render a subset of the specified max samples. Typically used for distributed rendering across multiple devices",` `default=False,` `)`
> `sample_offset: IntProperty(` … `description="0-based index of sample to start rendering from",` `min=0, max=(1 << 24),` `default=0,` `)`
> `sample_subset_length: IntProperty(` … `description="The number of samples to render in this subset",` `min=1, max=(1 << 24),` `default=2048,` `)`

Source: Cycles' own files, `Blender Sources/cycles_5.0/properties.py`, lines 494–512 — date read 2026-07-30.

So when a subset is switched on, the count that decides how many samples are taken is the subset's Length, and the count you gave is only there to keep the noise pattern compatible with the other machines' slices.

### Finding 6 — A time limit stops the render whatever the count says

**Label: documented and observed.** Cycles only; I found no time limit for EEVEE or Workbench.

> "Time Limit
> Stops rendering if the time exceeds the limit, even if the desired sample count
> hasn't been reached yet. Setting this value to zero disables the limit."

> "Note
> The time limit does not include pre-render processing time, only render time."

Source: manual, https://docs.blender.org/manual/en/5.0/render/cycles/render_settings/sampling.html — date read 2026-07-30.

Observed, including that it is off by default:

> `time_limit: FloatProperty(`
> `    name="Time Limit",`
> `    description="Limit the render time (excluding synchronization time). "`
> `    "Zero disables the limit",`
> `    min=0.0,`
> `    default=0.0,`
> `    step=100.0,`
> `    unit='TIME_ABSOLUTE',`
> `)`

Source: Cycles' own files, `Blender Sources/cycles_5.0/properties.py`, lines 514–522 — date read 2026-07-30. Note the description in the file says "excluding synchronization time" where the manual says "does not include pre-render processing time"; they are the same claim in different words.

### Finding 7 — When a time limit is in force, anything that changes render speed changes the count taken

**Label: inferred. Here is the step.** The manual says the time limit stops the render even if the sample count has not been reached. The same manual documents settings that make rendering faster or slower — the number of threads, the device, and on Windows which kind of processor core Blender's threads are scheduled onto:

> "-t, --threads <threads>
> Use amount of <threads> for rendering and other operations
> [1-1024], 0 to use the systems processor count."

> "--cycles-device <device>
> Set the device used for rendering."

> "--qos <level>
> Set the Quality of Service (QoS) mode for hybrid CPU architectures (Windows only).
> default: Uses the default behavior of the OS.
> high: Always makes use of performance cores.
> eco: Schedules Blender threads exclusively to efficiency cores."

Source: manual, https://docs.blender.org/manual/en/5.0/advanced/command_line/arguments.html — date read 2026-07-30.

The step a reader has to supply: if a render is cut off by elapsed time, then how fast it renders determines how far it got. The sources do not say this in so many words, so it is inferred, not documented. Nothing in the sources says how many samples any of these choices buys you.

### Finding 8 — EEVEE has two features that round the count *up*

**Label: documented.** These are the only places in the sources where the actual number of samples exceeds the number set.

Jitter Camera, in depth of field:

> "Jitter Camera
> Randomize the camera position for every scene render sample to increase precision.
> Enabling this option can change the scene's actual sample count."
> "Note
> Be aware that the actual sample count can grow quite rapidly."
> "Hint
> The actual number of samples is computed by the following formula:
> \[sample\_count = (ring\_count^{2} + ring\_count) * 3 + 1\]
> where \(ring\_count\) is the number of ring in the hexaweb pattern.
> The \(ring\_count\) is chosen so that the entire pattern contains at least the number of
> samples set in the Render Settings."

Source: manual, https://docs.blender.org/manual/en/5.0/render/eevee/render_settings/depth_of_field.html — date read 2026-07-30.

That this applies to the final render and not the viewport is documented in the API:

> "use_bokeh_jittered
> Jitter camera position to create accurate blurring using render samples (only for final render)"

Source: Python API, https://docs.blender.org/api/5.0/bpy.types.SceneEEVEE.html — date read 2026-07-30.

Motion blur steps:

> "Note
> When using multiple time steps, the render sample count is rounded up to the next multiple
> of steps to ensure even distribution of samples across steps.
> This can lower the quality of jitter camera depth of field.
> Increasing the number of viewport or render samples will get the
> quality back."

Source: manual, https://docs.blender.org/manual/en/5.0/render/eevee/render_settings/motion_blur.html — date read 2026-07-30.

So for EEVEE, a count handed in from outside is a floor as much as a target: with either of these on, the number actually taken is the next value up that the feature's pattern permits.

### Finding 9 — What the sources do not cover about stopping

**Label: not in the sources.** Searched as recorded above and found nothing on:

- Whether the Cycles time limit applies to each frame separately or to a whole animation run. The manual says only "Stops rendering if the time exceeds the limit". The setting sits on the scene, which is per scene, not per frame — but the sources do not say which way the limit is counted, and I will not guess.
- Whether adaptive sampling and a sample subset interact, and if so how. Nothing gates one on the other in the panel code, and neither page mentions the other. The subset page says to disable *denoising*, and says nothing about the noise threshold.
- Whether Min Samples being set above Max Samples is possible or what happens then. The ranges allow it (Min Samples goes to 4096, and Max Samples can be as low as 1).
- Any way to stop a render part-way by hand once it is running from outside Blender, or any signal that does so. The only stopping the manual discusses is at the level of whole frames: "when rendering animations to image file formats the render job can be canceled and resumed at the last rendered frame by changing the frame range" (manual, https://docs.blender.org/manual/en/5.0/render/output/introduction.html, date read 2026-07-30). That is about frames, not samples.
- What Blender does if a render runs out of memory or the device fails part-way.
- The exact automatic values Cycles picks when the noise threshold or Min Samples is left at zero. The manual says it guesses; the code that does the guessing is not among the Cycles files supplied — those are only the add-on's Python and the render kernel, not the session.

**Not covered.** This answer settles what the sources say governs the number of samples taken and the stopping of a render. It does not settle: how many samples any particular scene will actually take, since that depends on noise in that scene; the arithmetic of the automatic threshold and automatic minimum; whether the time limit is per frame or per run; the order in which the several caps and overrides are applied when more than one is in force at once (for instance a view layer override together with a sample subset — nothing in the sources describes that combination); or anything about a fourth render engine added by an add-on, since the sources describe only Cycles, EEVEE and Workbench.

**Outside the interface.** Everything above is a setting stored in the file or, in the case of the two Cycles options, an option given on the command line, so it applies the same whether a person is clicking or not. Three real differences:

1. There is no way to hand a sample count in as a command-line option — Finding 1. From outside, the count has to be written into the scene by a script, and it must be written before the argument that starts the render, because "Arguments are executed in the order they are given!" (manual, https://docs.blender.org/manual/en/5.0/advanced/command_line/render.html, date read 2026-07-30).
2. The viewport counts and viewport adaptive-sampling settings do not apply. Cycles keeps a separate set for the 3D Viewport — observed as `preview_samples`, `use_preview_adaptive_sampling`, `preview_adaptive_threshold`, `preview_adaptive_min_samples` in `Blender Sources/cycles_5.0/properties.py`, lines 485–492 and 588–608, each described as being for the viewport, for example "Number of samples to render in the viewport, unlimited if 0" (line 487) — date read 2026-07-30. Only the render ones matter for a render run from outside. The one about the viewport that does *not* carry over as you might expect is the EEVEE jitter camera, which the API says is "only for final render".
3. The progress against the ceiling is reported on the way past. The 5.0 release notes give an example of the new logging output for a background render, and the sample line is in it:

> `00:00.244  render           | Rendering single frame (frame 1)`
> `00:00.245  render           | Start rendering: Scene, ViewLayer`
> `00:00.245  render           | Engine: Cycles`
> `00:26.799  render           | Mem: 2983M | Sample 1/16 `
> `00:26.799  render           | Mem: 2983M | Sample 16/16`
> `00:28.631  render           | Saved: '/tmp/0001.png'`

Source: release notes, https://developer.blender.org/docs/release_notes/5.0/core/ — date read 2026-07-30. The same page says "Most of Blender now uses the same logging system, including operators reports and background render progress. The output format has changed" and lists the replacements, including `--verbose` becoming `--log-level [info|debug|trace]`. So a caller outside Blender can read how far the sample count got, but the shape of that text changed in 5.0.

**Settled by test on 2026-08-10. LABEL: observed.** Every one of the fifteen tests recorded under *Hand-test results* below was run from a terminal with no window at all, on two computers, and each of the settings above did what this answer describes in that setting: the sample count, adaptive sampling, the time limit, the sample subset and the Layer Samples setting all took effect in a render started from outside Blender. Nothing behaved differently for being driven from outside. The viewport copies of these settings have no bearing on such a render — changing all four of them left the reported count and every pixel of the picture unchanged. And these settings do travel with the file: eleven of them were saved and read back unchanged. What is still untested is the comparison against a render started inside Blender's own window, which neither machine could run this round.

**Operating system.** The sources state no operating-system difference for any of the sample ceilings, the view layer override, adaptive sampling, the sample subset, the time limit, or the EEVEE rounding rules. Three related things in the sources are not the same everywhere, and each touches this answer only at the edge:

- `--qos` is "(Windows only)" (manual, command line arguments page).
- Which devices `--cycles-device` will accept differs by machine — "Valid options are: CPU CUDA OPTIX HIP ONEAPI METAL" — and the OptiX denoiser is "Only available on NVIDIA GPUs" (manual, command line arguments page; manual, Cycles sampling page).
- Path guiding, whose Training Samples setting is counted against the render's samples — "A value of 0 will keep training until the end of the render" — is limited: "Path guiding is only available when rendering on a CPU" (manual, Cycles sampling page). This changes render speed, not the sample ceiling.

All three bear on how *fast* a render goes, which under a time limit bears on how many samples fit — the inference in Finding 7.

**Settled by test on 2026-08-10, for two operating systems out of three. LABEL: observed.** All fifteen tests recorded under *Hand-test results* below were run on macOS 26.5.2 on Apple silicon and on Ubuntu 22.04.5 LTS on an AMD processor, and the two machines returned the same sample count in every single test. Where a figure differed between them it was a time, or a count reached under a clock, never a count reached under a sample setting. **Windows is untested and stays untested** — no Windows machine has run it — so nothing here speaks for it.

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

## Things a person at a computer with Blender open could settle in a few minutes

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
   itself.

3. **Whether the Layer Samples control is drawn only when a view layer carries an override.** That
   needs eyes on a Blender window, which neither machine could give this round. Whether the setting
   keeps its value is a different question and was answered: it survives a save and a reopen.

---

2026-08-10 — Claude Opus 5 (Claude Code) — Settings, Blender 5.0

I used no helper agents for this work, so the declaration above covers the whole of it.

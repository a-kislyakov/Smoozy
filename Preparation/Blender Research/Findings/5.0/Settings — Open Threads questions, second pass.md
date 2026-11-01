# Settings — Open Threads questions, second pass (Blender 5.0)

**Sources consulted, and when.** Blender 5.0 manual (`Blender Sources/manual_5.0`, including one image from its `_images` folder), Blender 5.0 Python API documentation (`Blender Sources/api_5.0`), Blender 5.0 release notes (`Blender Sources/release_notes_5.0`), and Cycles' own files (`Blender Sources/cycles_5.0`). All were taken on 2026-07-30, and that is the date read for everything below.

I read only the question sheet and files inside Blender Sources/. I did not open any other part of this project.

Questions answered: 18 and 19 only.

---

## 18. What values will a render's sample count be set to, and what does a render do when it is given a value outside those?

### Short answer

There is no single sample count in Blender 5.0. There are several, each belonging to a different engine or to a view layer, and each with its own accepted range:

| Setting | Where it lives | Accepted values | Default |
| --- | --- | --- | --- |
| Cycles final-render samples | scene (`scene.cycles.samples`) | whole numbers from 1 to 16777216 | 4096 |
| Cycles viewport samples | scene (`scene.cycles.preview_samples`) | whole numbers from 0 to 16777216 (0 means unlimited) | 1024 |
| Cycles sample-subset length | scene (`scene.cycles.sample_subset_length`) | whole numbers from 1 to 16777216 | 2048 |
| Cycles sample-subset offset | scene (`scene.cycles.sample_offset`) | whole numbers from 0 to 16777216 | 0 |
| Cycles adaptive minimum samples | scene (`scene.cycles.adaptive_min_samples`) | whole numbers from 0 to 4096 (0 means automatic) | 0 |
| EEVEE final-render samples | scene (`scene.eevee.taa_render_samples`) | whole numbers from 1 upward, no stated upper bound | 64 |
| EEVEE viewport samples | scene (`scene.eevee.taa_samples`) | whole numbers from 0 upward, no stated upper bound (0 means unlimited) | 16 |
| Per-view-layer sample override | view layer (`view_layer.samples`) | whole numbers from 0 upward, no stated upper bound (0 means "use the scene setting") | 0 |

The number 16777216 is what `1 << 24` works out to. The Cycles files write the limit that way, not as a decimal number; converting it is arithmetic, so I flag it.

When a value outside the accepted range is handed in, Blender does not refuse it and does not report an error. It quietly replaces it with the nearest end of the range. This is stated for properties built the way Cycles' are, and it is stated separately for typing a number into a field in the interface.

### LABEL

Mixed, split by part:

- The accepted ranges for the eight Cycles settings above: **observed** (read directly out of Cycles' own files).
- The accepted ranges for the two EEVEE settings and the view-layer override: **documented** (stated in the Python API documentation).
- What happens to an out-of-range value for the Cycles settings: **inferred**. The Python API documentation says what `min` and `max` do to a property defined through `bpy.props`; Cycles' settings are defined that way, with those two arguments written in. Joining the two is a step, and the documentation nowhere says it of Cycles' `samples`. *(Label changed from documented by the citation check of 2026-08-30: the parenthesis this line used to carry was itself the step, and every other document in this round labels the same step inferred.)*
- What happens to an out-of-range value for the EEVEE settings and the view-layer override: **not in the sources**. Those are not built through `bpy.props`, so the wording quoted below does not cover them by its own terms, and the API pages for them give a range and say nothing about assignment. *(Changed by the citation check of 2026-08-30. This line previously read "inferred" and carried the clamping behaviour across from the Python case by analogy. An analogy is not a step from the quoted words to the claim — the words do not carry it by any route — so the carry-across has been cut. Seven of the eight documents in this round rule this same question "not in the sources", which is what these sources support. The hand tests of 2026-08-30 answer the behaviour by test; that is a separate thing from what the sources state.)*
- That there is no way to hand a sample count in at launch other than through Python: **documented** (the command-line argument list has no such option).

### The exact words

**Cycles final-render samples** — `Blender Sources/cycles_5.0/properties.py`, class `CyclesRenderSettings` (which begins at line 378), lines 479–484:

```
    samples: IntProperty(
        name="Samples",
        description="Number of samples to render for each pixel",
        min=1, max=(1 << 24),
        default=4096,
    )
```

**Cycles viewport samples** — same file, lines 485–492:

```
    preview_samples: IntProperty(
        name="Viewport Samples",
        description="Number of samples to render in the viewport, unlimited if 0",
        min=0,
        soft_min=1,
        max=(1 << 24),
        default=1024,
    )
```

Note the two different floors on this one: `min=0` and `soft_min=1`. The manual explains what the pair means (quoted below under "value limits").

**Cycles sample subset** — same file, lines 500–513:

```
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

**Cycles adaptive minimum samples** — same file, lines 580–586:

```
    adaptive_min_samples: IntProperty(
        name="Adaptive Min Samples",
        description="Minimum AA samples for adaptive sampling, to discover noisy features before stopping sampling. Zero for automatic setting based on noise threshold",
        min=0,
        max=4096,
        default=0,
    )
```

That these all hang off the scene is shown at the same file, lines 1102–1107:

```
    def register(cls):
        bpy.types.Scene.cycles = PointerProperty(
            name="Cycles Render Settings",
            description="Cycles render settings",
            type=cls,
        )
```

**EEVEE samples** — Python API documentation, `SceneEEVEE`, https://docs.blender.org/api/5.0/bpy.types.SceneEEVEE.html, read 2026-07-30:

> taa_render_samples
>
> Number of samples per pixel for rendering
>
> Type:
>
> int in [1, inf], default 64

> taa_samples
>
> Number of samples, unlimited if 0
>
> Type:
>
> int in [0, inf], default 16

**Per-view-layer sample override** — Python API documentation, `ViewLayer`, https://docs.blender.org/api/5.0/bpy.types.ViewLayer.html, read 2026-07-30:

> samples
>
> Override number of render samples for this view layer, 0 will use the scene setting
>
> Type:
>
> int in [0, inf], default 0

The manual describes the same override, and says the two engines treat it differently — manual, https://docs.blender.org/manual/en/5.0/render/layers/override.html, read 2026-07-30:

> Samples
>
> Allows the view layer to use a different number of samples than the main scene.
> This is controlled by the Layer Samples
> when using the Cycles renderer. When using EEVEE, the Samples override is always used.

The "Layer Samples" setting it refers to is a scene-level Cycles choice of three — manual, https://docs.blender.org/manual/en/5.0/render/cycles/render_settings/sampling.html, read 2026-07-30:

> Layer Samples
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
> Ignore the overrides.

The three names, with their own wording, are in Cycles' own files at `Blender Sources/cycles_5.0/properties.py`, lines 64–68:

```
enum_use_layer_samples = (
    ('USE', "Use", "Per render layer number of samples override scene samples"),
    ('BOUNDED', "Bounded", "Bound per render layer number of samples by global samples"),
    ('IGNORE', "Ignore", "Ignore per render layer number of samples"),
)
```

and its default is `'USE'` (same file, lines 550–555).

That EEVEE honours the override at all is new in this version — release notes, https://developer.blender.org/docs/release_notes/5.0/eevee/, read 2026-07-30:

> View Layer Overrides (Material, World, Sample) are now supported. Older files using this feature will render differently.

**What happens to an out-of-range value.** Python API documentation, `bpy.props`, the entry for `IntProperty`, https://docs.blender.org/api/5.0/bpy.props.html, read 2026-07-30:

> min (int) – Hard minimum, trying to assign a value below will silently assign this minimum instead.

> max (int) – Hard maximum, trying to assign a value above will silently assign this maximum instead.

> soft_min (int) – Soft minimum (>= min), user won’t be able to drag the widget below this value in the UI.

> soft_max (int) – Soft maximum (<= max), user won’t be able to drag the widget above this value in the UI.

Every Cycles sample-count setting quoted above is an `IntProperty` with `min` and `max` given. Applying this wording to them is the reader's own step — small, but a step, which is why the label above reads inferred. *(Narrowed by the citation check of 2026-08-30; this sentence previously said the wording applies to them "directly".)*

**Value limits in the interface.** Manual, https://docs.blender.org/manual/en/5.0/interface/controls/buttons/fields.html, read 2026-07-30:

> Value Limits
>
> Most numerical values are restricted by “soft limit” and “hard limit” value ranges.
> Changing values by dragging with the mouse is restricted to the “soft limit” value range.
> Input via keyboard will allow the use of wider value ranges, but never wider than the “hard limit”.

**No launch-time sample count.** The manual's list of command-line arguments has a "Render Options" section and a "Cycles Render Options" section. The Cycles section holds two entries and neither is a sample count — manual, https://docs.blender.org/manual/en/5.0/advanced/command_line/arguments.html, read 2026-07-30:

> Cycles Render Options
>
> Cycles add-on options must be specified following a double dash.
>
> --cycles-device <device>
>
> Set the device used for rendering.
> Valid options are: CPU CUDA OPTIX HIP ONEAPI METAL.

> --cycles-print-stats
>
> Log statistics about render memory and time usage.

### One extra thing the Cycles files show

Cycles' own file-version upgrade code multiplies a stored sample count by itself when reading a file that used a since-removed "square samples" option — `Blender Sources/cycles_5.0/version_update.py`, lines 242–251:

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

**observed** that this code exists and what it says. It is a route by which a sample count can be pushed past 16777216 during file loading (a stored 5000 would square to 25,000,000). What actually lands in the setting afterwards is **inferred** from the clamping rule above, and the step is: the multiplication result is assigned back to an `IntProperty` with `max=(1 << 24)`, and assignment above the maximum silently assigns the maximum. I did not run this.

### Where I looked

- Searched all of `Blender Sources/cycles_5.0` for `samples`. Read every hit in `properties.py` (26 hits), and the hits in `presets.py`, `version_update.py`, `engine.py` and `ui.py`.
- Searched all of `Blender Sources/api_5.0` for `taa_render_samples`, and for `samples (bpy` in the index files. Read `bpy.types.SceneEEVEE.txt` and `bpy.types.ViewLayer.txt` in the relevant places. Read the whole `IntProperty` and `FloatProperty` entries in `bpy.props.txt`.
- Searched all of `Blender Sources/manual_5.0` for `sample`, `Sample Overrides`, `soft limit`, `hard limit`, `clamped to`, `is clamped`, `will be clamped`, `out of range`, `outside the valid`, `invalid value`. Read in full: the Cycles Sampling page, the EEVEE Sampling page, the View Layer page, the View Layer Override page, the View Layer Filter page, and the "Value Limits" part of the number-fields page.
- Searched `Blender Sources/manual_5.0/advanced/command_line/` for `sample` — no hits at all. Read the whole Render Options and Cycles Render Options sections of the arguments page to be sure.
- Searched all of `Blender Sources/manual_5.0` and `Blender Sources/api_5.0` and `Blender Sources/cycles_5.0` for `cycles-samples` and `cycles_samples` — no hits anywhere.
- Searched `Blender Sources/release_notes_5.0` for `sample` across `cycles.md`, `eevee.md`, `rendering.md`, `core.md`, `python_api.md`. Three hits; the EEVEE one is quoted above, the Cycles one is unrelated, the core one is a sample of console output.
- Ruled out: `Blender Sources/cycles_5.0/source` — searched it, it holds only kernel and utility headers, no file that reads or bounds a sample count from settings.
- Ruled out on searching: the manual nowhere states a numeric ceiling or floor for any sample count. Every "clamped" hit in the manual is about some other feature (weights, F-curve handles, the Map Range node, crease angles, constraint limits, viewport frame rate, solidify material slots, dynamic paint depth). "out of range", "outside the valid" and "invalid value" have no hits anywhere in the manual.

### Not covered

- I did not find, and therefore cannot say, what an out-of-range value does when it arrives by any route other than assignment to the setting or typing into a field. In particular, a value coming from a driver, a keyframe, or a library override is not addressed by anything I read.
- The clamping wording says "silently". It does not say whether anything is written to the log at any log level. I found no statement either way.
- I found no upper bound stated for the EEVEE sample counts or the view-layer override; "inf" is what the documentation prints. What Blender actually does with, say, three billion is not covered by anything I read.
- The Cycles viewport samples setting has both a soft floor of 1 and a hard floor of 0. Which one a value of 0 lands on depends on how it is set; the manual explains the general rule but nothing states the outcome for this specific setting.
- Nothing I read says what the render does when the view-layer override, the scene count, and the Layer Samples mode disagree in some particular way beyond the three-line description of Use / Bounded / Ignore.
- The question says "a render's sample count". I answered it as covering every setting in Blender 5.0 that is a count of samples for a render, including the viewport ones and the subset ones, and said which is which. If it was meant to mean only the one final-render count for the active engine, the answer is the first row of the table for Cycles and the sixth for EEVEE.

### Outside the interface

Holds, with one difference and one gap.

The clamping rule quoted from `bpy.props` is about assigning to the setting. That is exactly what happens when Blender is driven from outside — the only route in, since the command-line argument list has no sample-count option, is Python handed in at launch (`--python-expr` or `--python`), and Python assignment is the assignment the rule describes. **inferred**, and the step is: the documentation describes assignment without saying where the assignment comes from, and I am reading that as covering assignment from a launch-time script.

The difference: the "Value Limits" wording about dragging and keyboard entry is about the interface only, by its own terms ("in the UI", "Changing values by dragging with the mouse"). There is no interface outside the window, so the soft limits have no effect there. **inferred** from the wording being about widgets.

The gap: nothing I read says whether a render started from outside reports anything when a sample count is clamped.

### Operating system

Nothing I read ties any of these ranges, defaults, or the clamping behaviour to an operating system. The Cycles file states one set of numbers with no platform condition, and the API documentation states one range per setting with no platform condition. So I have no reason from the sources to think it differs, and no statement that it does not. The sources do not say.

---


---

## Hand-test results — question 18 (2026-08-30)

**On what computers.** Two, and their results are kept apart.

- **The Creator's own MacBook Pro**, M1 Pro chip, macOS 26.6.2 (build 25G83), Apple's own graphics and no second card. Blender 5.0.1 at `/Applications/Blender 5.0.1.app/Contents/MacOS/Blender`, build hash `a3db93c5b259`.
- **A Linux machine rented for these tests**: Vast.ai instance 49253900, machine 27187, in Poland — a container rather than a whole machine (`systemd-detect-virt` answers `docker`, process 1 is `/usr/bin/dash`, no service manager), Ubuntu 22.04.4 LTS on kernel 6.8.0-48-generic, an AMD Ryzen 5 5600X with 5.76 cores allotted against 12 visible threads, a memory ceiling of about 21.2 GiB against the 31 GB the ordinary reading reports, a 45 GB overlay root, and **one NVIDIA GeForce RTX 3060 with 12288 MiB on driver 550.120**. It had **no display server as rented**; one was installed for the panel test and it draws in software (Xvfb, OpenGL renderer `llvmpipe`). Blender 5.0.1 at `/opt/blender/5.0/blender`, **build hash `a3db93c5b259` — the same build as the Mac's**, checked before any test was run.

Every run used a throwaway settings folder made beforehand and `--factory-startup`, so none of the Creator's own settings was read or touched. Every Cycles render was on the processor except where a card is named.


### What values the sample counts take

**Done.** With no file open, read every sample count's declared floor, ceiling, dragging bounds and starting value out of the running program.

**Came back**, identically on both machines:

```
E1 scene.cycles.samples hard_min=1 hard_max=16777216 soft_min=1 soft_max=16777216 default=4096 current=4096
E1 scene.cycles.preview_samples hard_min=0 hard_max=16777216 soft_min=1 soft_max=16777216 default=1024 current=1024
E1 scene.cycles.sample_offset hard_min=0 hard_max=16777216 soft_min=0 soft_max=16777216 default=0 current=0
E1 scene.cycles.sample_subset_length hard_min=1 hard_max=16777216 soft_min=1 soft_max=16777216 default=2048 current=2048
E1 scene.cycles.adaptive_min_samples hard_min=0 hard_max=4096 soft_min=0 soft_max=4096 default=0 current=0
E1 scene.cycles.preview_adaptive_min_samples hard_min=0 hard_max=4096 soft_min=0 soft_max=4096 default=0 current=0
E1 scene.cycles.guiding_training_samples hard_min=0 hard_max=2147483647 soft_min=1 soft_max=2147483647 default=128 current=128
E1 scene.cycles.preview_denoising_start_sample hard_min=0 hard_max=16777216 soft_min=0 soft_max=16777216 default=1 current=1
E1 scene.eevee.taa_render_samples hard_min=1 hard_max=2147483647 soft_min=1 soft_max=2147483647 default=64 current=64
E1 scene.eevee.taa_samples hard_min=0 hard_max=2147483647 soft_min=0 soft_max=2147483647 default=16 current=16
E1 view_layer.samples hard_min=0 hard_max=2147483647 soft_min=0 soft_max=10000 default=0 current=0
E1 scene.grease_pencil_settings.aa_samples hard_min=1 hard_max=2147483647 soft_min=1 soft_max=256 default=8 current=8
E1 scene.display.render_aa items=['OFF', 'FXAA', '5', '8', '11', '16', '32'] default='8' current='8'
```

**What that settles for this answer.**

- The upper end this document records as **"inf"** for the EEVEE counts, the view layer's override and the Grease Pencil count **is 2 147 483 647**.
- **`guiding_training_samples` does have a maximum** — 2 147 483 647, the whole-number type's own ceiling — where its declaration writes none.
- Two dragging bounds appear that no source states: **the view layer's override cannot be dragged past 10 000**, and the Grease Pencil count not past 256.
- Every Cycles floor and ceiling this document quotes from Cycles' own file is what the running program holds.
- **Every count named in only some versions' documents is present, with the same limits, in all four versions**, so no version's silence about any of them was a difference in Blender.
- One thing to read carefully: the Grease Pencil line ends `current=8` here and `current=1` in 4.5. **That is the value in each version's startup file, not a limit**; the declared starting value is 8 in all four.

### What a value outside the range does

**Done.** Assigned 0, −5, 16 777 217, 99 999 999, 2 147 483 647, 2 147 483 648 and 10¹² to each count in turn and read the value back, in one run that also printed two lines of its own and attempted a deliberately wrong assignment, so that "nothing was reported" could be told apart from "nothing was captured".

**Came back**, on the Creator's Mac:

```
E2 CONTROL-STDOUT this line proves standard output is captured
E2 CONTROL-STDERR this line proves standard error is captured
E2 CONTROL-RAISES AttributeError: 'CyclesRenderSettings' object has no attribute 'samples_no_such_setting'
E2 scene.cycles.samples = 0 -> stored 1 (no error)
E2 scene.cycles.samples = -5 -> stored 1 (no error)
E2 scene.cycles.samples = 16777217 -> stored 16777216 (no error)
E2 scene.cycles.samples = 99999999 -> stored 16777216 (no error)
E2 scene.cycles.samples = 2147483647 -> stored 16777216 (no error)
E2 scene.cycles.samples = 2147483648 -> REFUSED ValueError: bpy_struct: item.attr = val: CyclesRenderSettings.samples value not in 'int' range ((-2147483647 -1), 2147483647)
E2 scene.cycles.samples = 1000000000000 -> REFUSED ValueError: bpy_struct: item.attr = val: CyclesRenderSettings.samples value not in 'int' range ((-2147483647 -1), 2147483647)
E2 scene.cycles.preview_samples = -5 -> stored 0 (no error)
E2 scene.cycles.adaptive_min_samples = 99999999 -> stored 4096 (no error)
E2 scene.cycles.sample_subset_length = 0 -> stored 1 (no error)
```

and the counts this document records as **not in the sources** behave the same way:

```
E2 scene.eevee.taa_render_samples = 0 -> stored 1 (no error)
E2 scene.eevee.taa_render_samples = 99999999 -> stored 99999999 (no error)
E2 scene.eevee.taa_render_samples = 2147483648 -> REFUSED ValueError: bpy_struct: item.attr = val: SceneEEVEE.taa_render_samples value not in 'int' range ((-2147483647 -1), 2147483647)
E2 view_layer.samples = -5 -> stored 0 (no error)
E2 scene.grease_pencil_settings.aa_samples = 0 -> stored 1 (no error)
```

**The rented Linux machine gave the same values in every case**, and differed only in the wording of the refusal: it prints `value not in 'int' range ((-0x7fffffff - 1), 0x7fffffff)` where the Mac prints `((-2147483647 -1), 2147483647)`. **A program reading Blender's output for that refusal must match both shapes.**

**What that settles.** The silent correction this document infers from `bpy.props` is what happens — and it happens for the EEVEE, view-layer and Grease Pencil counts too, which this document could not settle. **Nothing at all is reported.** A number too large for a whole number is not corrected but refused, with a `ValueError`.

### What a value that is not a whole number does

**Done.** Assigned a decimal, text, text that looks like a number, nothing, and a true-or-false value to each count; and a choice outside the list to Workbench's anti-aliasing setting, with valid choices in the same run as a control.

**Came back**, the same on both machines:

```
E3 scene.cycles.samples = 4.7 -> REFUSED TypeError: bpy_struct: item.attr = val: CyclesRenderSettings.samples expected an int type, not float (still 4096)
E3 scene.cycles.samples = 'many' -> REFUSED TypeError: bpy_struct: item.attr = val: CyclesRenderSettings.samples expected an int type, not str (still 4096)
E3 scene.cycles.samples = '64' -> REFUSED TypeError: bpy_struct: item.attr = val: CyclesRenderSettings.samples expected an int type, not str (still 4096)
E3 scene.cycles.samples = None -> REFUSED TypeError: bpy_struct: item.attr = val: CyclesRenderSettings.samples expected an int type, not NoneType (still 4096)
E3 scene.cycles.samples = True -> stored 1 (no error, was 4096)
E3 scene.display.render_aa = '7' -> REFUSED TypeError: bpy_struct: item.attr = val: enum "7" not found in ('OFF', 'FXAA', '5', '8', '11', '16', '32') (still '8')
E3 scene.display.render_aa = 'OFF' -> stored 'OFF' (no error)
E3 scene.display.render_aa = 'fxaa' -> REFUSED TypeError: bpy_struct: item.attr = val: enum "fxaa" not found in ('OFF', 'FXAA', '5', '8', '11', '16', '32') (still '32')
```

The same refusals came back for the EEVEE, view-layer and Grease Pencil counts.

**What that settles.** A value that is not a whole number is **refused outright, never rounded and never truncated** — which this document records as unstated. A choice outside a fixed list is refused too, and the refusal names every real choice. Matching a choice is case-sensitive. A true-or-false value is the one exception: it is accepted and stores 1.

### What sample count actually reached the render

This is a different claim from the two above, and it is the one this document could not reach: what the *setting* stores is not what the *render* works to.

**Done.** Built the test scene, set the count out of range, and rendered one frame at 64 by 48 with adaptive sampling off, keeping everything Blender printed.

**Came back**, on the Creator's Mac in 5.0.1:

```
E6 asked=99999999 stored=16777216 (no error)
E6 engine=CYCLES setting=samples blender=5.0.1
E6 RENDER-STARTS
00:00.333  render           | Fra: 1 | Mem: 33M | Sample 0/16777216
00:00.336  render           | Fra: 1 | Remaining: 11:51:19.99 | Mem: 33M | Sample 1/16777216
00:12.898  render           | Fra: 1 | Remaining: 07:08:36.29 | Mem: 33M | Sample 8193/16777216
RUNCAP exit=-9 killed_at_25.0s=True
```

and the case at the other end, as a control:

```
E6 asked=0 stored=1 (no error)
E6 engine=CYCLES setting=samples blender=5.0.1
E6 RENDER-STARTS
00:00.499  render           | Fra: 1 | Mem: 2M | Sample 0/1
00:00.502  render           | Fra: 1 | Mem: 2M | Sample 1/1
00:00.573  render           | Saved: '.../e6-5.0/e6_0001.png'
RUNCAP exit=0 killed_at_25.0s=False
```

EEVEE, whose count is not corrected at 99 999 999, works to the number it was given: `Rendering 1 / 99999999 samples`. **The rented Linux machine gave the same counts in every case.**

**What that settles.** **The corrected number is the number the render works to.** A count above the ceiling does not fail, does not warn and does not stop — it settles in for a fourteen-hour render at 16 777 216 samples. The only place the corrected figure appears is the ordinary progress line.

**Outside the interface — brought up to date by the hand tests of 2026-08-30.** Every result above was got from a Blender started with no window, driven by a script handed in at launch. So the correction, the refusals and the count the render works to are all **observed** for exactly the case this project cares about. What is still untested outside the window is the soft-limit half of the number-field rule, which has no counterpart there.

**Operating system — brought up to date by the hand tests of 2026-08-30.** Every accepted value, every correction and every refusal came back the same on macOS and on Linux. **One thing does differ, and it is the wording**: the refusal of a number too large for a whole number reads `((-2147483647 -1), 2147483647)` on macOS and `((-0x7fffffff - 1), 0x7fffffff)` on Linux. Windows is untested; the tests are written out ready to run in `Audits/Blender Research/Settings/Hand tests — Open Threads questions.md`.

## 19. What settings does a scene or a view layer carry that govern denoising, and what does each of them do?

### Short answer

There are four separate groups. Three sit on the scene, one on the view layer.

**Group 1 — Cycles final-render denoising (on the scene).** Six settings.

**Group 2 — Cycles viewport denoising (on the scene).** Seven settings. Same shape as group 1, plus one for when to start.

**Group 3 — Cycles per-view-layer denoising (on the view layer).** Two settings.

**Group 4 — settings on the scene that govern denoising done in the compositor rather than by the render engine.** Three settings, and they belong to the scene's render settings rather than to any one engine.

Separately, EEVEE carries settings whose stated job is noise reduction: three switches inside its ray-tracing options, and one scene-level switch for temporal reprojection. Whether those count as "denoising" depends on how the word is read; the sources use the word "Denoising" as a heading for the ray-tracing three and use "Denoise" in the description of the reprojection one, so I have included them and flagged the reading.

### Group 1 — Cycles, final render, on the scene

All six live on `scene.cycles`. **observed**, from `Blender Sources/cycles_5.0/properties.py`, class `CyclesRenderSettings` (begins line 378), which registers itself onto `bpy.types.Scene.cycles` at lines 1102–1107 (quoted under question 18).

Lines 400–437:

```
    use_denoising: BoolProperty(
        name="Use Denoising",
        description="Denoise the rendered image",
        default=True,
        update=update_render_passes,
    )
    denoiser: EnumProperty(
        name="Denoiser",
        description="Denoise the image with the selected denoiser. "
        "For denoising the image after rendering",
        items=enum_denoiser,
        default=4,  # Use integer to avoid error in builds without OpenImageDenoise.
        update=update_render_passes,
    )
    denoising_prefilter: EnumProperty(
        name="Denoising Prefilter",
        description="Prefilter noisy guiding (albedo and normal) passes to improve denoising quality when using OpenImageDenoise",
        items=enum_denoising_prefilter,
        default='ACCURATE',
    )
    denoising_quality: EnumProperty(
        name="Denoising Quality",
        description="Overall denoising quality when using OpenImageDenoise",
        items=enum_denoising_quality,
        default='HIGH',
    )
    denoising_input_passes: EnumProperty(
        name="Denoising Input Passes",
        description="Passes used by the denoiser to distinguish noise from shader and geometry detail",
        items=enum_denoising_input_passes,
        default='RGB_ALBEDO_NORMAL',
    )
    denoising_use_gpu: BoolProperty(
        name="Denoise on GPU",
        description="Perform denoising on GPU devices configured in the system tab in the user preferences. This is significantly faster than on CPU, but requires additional GPU memory. When large scenes need more GPU memory, this option can be disabled",
        default=False,
    )
```

What each does, in the manual's own words — manual, https://docs.blender.org/manual/en/5.0/render/cycles/render_settings/sampling.html, read 2026-07-30:

> Denoising
>
> Denoising uses a specialized algorithm to get a less noisy image without requiring more samples.

> Denoise (Render)
>
> Whether to perform denoising for the final rendered image.

> Denoiser
>
> The algorithm to use.
>
> Automatic:
>
> Uses GPU accelerated denoising if supported, for best performance.
> Prefers OpenImageDenoise over OptiX.
>
> OpenImageDenoise:
>
> Uses Intel’s Open Image Denoise,
> an AI denoiser. Typically provides the highest quality.
>
> OptiX:
>
> Uses NVIDIA’s OptiX AI denoiser.
> Supports GPU acceleration on some older NVIDIA GPUs where OpenImageDenoise does not.
>
> Only available on NVIDIA GPUs when configured in the Cycles Render Devices user preferences.

> Passes
>
> Controls which Render Passes the denoiser should use as input.
> Generally, the more passes the denoiser has access to, the better the result.
> It is recommended to use at least Albedo as None can blur out details,
> especially at lower sample counts.
>
> None:
>
> Denoises the image using color data.
>
> Albedo:
>
> Denoises the image using color and albedo data.
>
> Albedo + Normal:
>
> Denoises the image using color, albedo, and normal pass data.

> Prefilter OpenImageDenoise
>
> Controls whether prefiltering is applied to the Passes before denoising.
> Visible only when using OpenImageDenoise.

(The manual then describes the three prefilter choices — None, Fast, Accurate — at length; Cycles' own file gives the same three, quoted below.)

> Quality OpenImageDenoise
>
> Overall denoising quality.
> Visible only when using OpenImageDenoise.
>
> High:
>
> Produces the highest quality at the cost of time.
>
> Balanced:
>
> Balanced between performance and quality.
>
> Fast:
>
> Sacrifices quality for speed (ideal for viewport rendering).

> Use GPU
>
> Perform denoising on the GPU.
> This is significantly faster than on CPU, but requires additional GPU memory.
> When large scenes need more GPU memory, this option can be disabled.

One thing the manual and the file disagree in emphasis about, and it matters: the manual's "Denoiser" entry lists **Automatic** as a choice. Cycles' own file builds the final-render denoiser list without an Automatic entry — `Blender Sources/cycles_5.0/properties.py`, lines 299–303:

```
def enum_denoiser(self, context):
    items = []
    items += enum_optix_denoiser(self, context)
    items += enum_openimagedenoise_denoiser(self, context)
    return items
```

The Automatic entry is added only in the *viewport* denoiser list, at lines 280–296. So **observed**: in Blender 5.0's Cycles files, the final-render Denoiser choice offers only OptiX and OpenImageDenoise, and Automatic appears in the viewport one. That the manual's Denoiser section lists Automatic without saying it is viewport-only is a discrepancy I am reporting, not resolving.

Also observed: neither entry in the final-render list is unconditional. Same file, lines 265–277:

```
def enum_openimagedenoise_denoiser(self, context):
    import _cycles
    if _cycles.with_openimagedenoise:
        return [('OPENIMAGEDENOISE', "OpenImageDenoise",
                 n_("Use Intel OpenImageDenoise AI denoiser"), 4)]
    return []


def enum_optix_denoiser(self, context):
    if not context or bool(context.preferences.addons[__package__].preferences.get_devices_for_type('OPTIX')):
        return [('OPTIX', "OptiX", n_(
            "Use the OptiX AI denoiser with GPU acceleration, only available on NVIDIA GPUs when configured in the system tab in the user preferences"), 2)]
    return []
```

The three choice lists in full, same file, lines 306–340:

```
enum_denoising_input_passes = (
    ('RGB', "None", "Don't use utility passes for denoising", 1),
    ('RGB_ALBEDO', "Albedo", "Use albedo pass for denoising", 2),
    ('RGB_ALBEDO_NORMAL', "Albedo and Normal", "Use albedo and normal passes for denoising", 3),
)

enum_denoising_prefilter = (
    ('NONE',
     "None",
     "No prefiltering, use when guiding passes are noise-free",
     1),
    ('FAST',
     "Fast",
     "Denoise color and guiding passes together. Improves quality when guiding passes are noisy using least amount of extra processing time",
     2),
    ('ACCURATE',
     "Accurate",
     "Prefilter noisy guiding passes before denoising color. Improves quality when guiding passes are noisy using extra processing time",
     3),
)

enum_denoising_quality = (
    ('HIGH',
     "High",
     "High quality",
     1),
    ('BALANCED',
     "Balanced",
     "Balanced between performance and quality",
     2),
    ('FAST',
     "Fast",
     "High performance",
     3),
)
```

### Group 2 — Cycles, viewport, on the scene

Seven settings, same file, lines 438–478. They mirror group 1 and add a start point:

```
    use_preview_denoising: BoolProperty(
        name="Use Viewport Denoising",
        description="Denoise the image in the 3D viewport",
        default=False,
    )
    preview_denoiser: EnumProperty(
        name="Viewport Denoiser",
        description="Denoise the image after each preview update with the selected denoiser",
        items=enum_preview_denoiser,
        default=0,
    )
    preview_denoising_prefilter: EnumProperty(
        name="Viewport Denoising Prefilter",
        description="Prefilter noisy guiding (albedo and normal) passes to improve denoising quality when using OpenImageDenoise",
        items=enum_denoising_prefilter,
        default='FAST',
    )
    preview_denoising_quality: EnumProperty(
        name="Viewport Denoising Quality",
        description="Overall denoising quality when using OpenImageDenoise",
        items=enum_denoising_quality,
        default='BALANCED',
    )
    preview_denoising_input_passes: EnumProperty(
        name="Viewport Denoising Input Passes",
        description="Passes used by the denoiser to distinguish noise from shader and geometry detail",
        items=enum_denoising_input_passes,
        default='RGB_ALBEDO',
    )
    preview_denoising_start_sample: IntProperty(
        name="Start Denoising",
        description="Sample to start denoising the preview at",
        min=0, max=(1 << 24),
        default=1,
    )
    preview_denoising_use_gpu: BoolProperty(
        name="Denoise Preview on GPU",
        description="Perform denoising on GPU devices configured in the system tab in the user preferences. This is significantly faster than on CPU, but requires additional GPU memory. When large scenes need more GPU memory, this option can be disabled",
        default=True,
    )
```

The viewport denoiser list, unlike the final-render one, has an Automatic entry — same file, lines 280–296:

```
def enum_preview_denoiser(self, context):
    optix_items = enum_optix_denoiser(self, context)
    oidn_items = enum_openimagedenoise_denoiser(self, context)

    if len(optix_items) or len(oidn_items):
        items = [
            ('AUTO',
             "Automatic",
             n_("Use GPU accelerated denoising if supported, for the best performance. "
                "Prefer OpenImageDenoise over OptiX"),
             0)]
    else:
        items = [('AUTO', "None", n_("Blender was compiled without a viewport denoiser"), 0)]

    items += optix_items
    items += oidn_items
    return items
```

Manual wording for the two of these it covers — same Sampling page, read 2026-07-30:

> Denoise (Viewport)
>
> Whether to perform denoising for the 3D Viewport in the Rendered shading mode.

> Start Sample
>
> Sample at which to start denoising in the 3D Viewport.

### Group 3 — Cycles, on the view layer

Two settings. **observed**, from `Blender Sources/cycles_5.0/properties.py`, class `CyclesRenderLayerSettings` (begins line 1479), lines 1532–1543:

```
    use_denoising: BoolProperty(
        name="Use Denoising",
        description="Denoise the rendered image",
        default=True,
        update=update_render_passes,
    )
    denoising_store_passes: BoolProperty(
        name="Store Denoising Passes",
        description="Store the denoising feature passes and the noisy image. The passes adapt to the denoiser selected for rendering",
        default=False,
        update=update_render_passes,
    )
```

That these hang off the view layer, not the scene — same file, lines 1546–1551:

```
    def register(cls):
        bpy.types.ViewLayer.cycles = PointerProperty(
            name="Cycles ViewLayer Settings",
            description="Cycles ViewLayer Settings",
            type=cls,
        )
```

Two further observations from Cycles' own files about how the view-layer switch relates to the scene one.

It is drawn greyed out when the scene switch is off — `Blender Sources/cycles_5.0/ui.py`, lines 990–991:

```
        sub.prop(view_layer.cycles, "use_denoising", text="Denoising")
        sub.active = scene.cycles.use_denoising
```

And the two are combined with "and" when deciding which extra passes exist — `Blender Sources/cycles_5.0/engine.py`, lines 242–250:

```
    # Denoising passes.
    if scene.cycles.use_denoising and crl.use_denoising:
        yield ("Noisy Image", "RGBA", 'COLOR')
        if crl.use_pass_shadow_catcher:
            yield ("Noisy Shadow Catcher", "RGB", 'COLOR')
    if crl.denoising_store_passes:
        yield ("Denoising Normal", "XYZ", 'VECTOR')
        yield ("Denoising Albedo", "RGB", 'COLOR')
        yield ("Denoising Depth", "Z", 'VALUE')
```

Reading that code as meaning a view layer is denoised only when both switches are on is **inferred**, and the step is: this code decides the pass list, not the denoising itself, and I am reading the presence of the "Noisy Image" pass under `scene.cycles.use_denoising and crl.use_denoising` as standing for denoising happening. What is straightforwardly **observed** is that the pass list is gated on both.

The manual documents the pass but not the switch — manual, https://docs.blender.org/manual/en/5.0/render/layers/passes.html, read 2026-07-30:

> Denoising Data Cycles
>
> Includes Denoising Albedo, Denoising Normal, and the combined image
> before denoising. Can be used with the Denoise Node
> as a replacement for automatic denoising.

The per-view-layer Denoising switch itself I could not find anywhere in the manual — see "Where I looked".

### Group 4 — scene settings that govern denoising done in the compositor

These are on the scene's render settings, not on any engine's, and the Cycles and EEVEE manual pages document the same three panel entries. Python API documentation, `RenderSettings`, https://docs.blender.org/api/5.0/bpy.types.RenderSettings.html, read 2026-07-30:

> compositor_denoise_device
>
> The device to use to process the denoise nodes in the compositor
>
> AUTO
> Auto – Use the same device used by the compositor to process the denoise node.
>
> CPU
> CPU – Use the CPU to process the denoise node.
>
> GPU
> GPU – Use the GPU to process the denoise node if available, otherwise fallback to CPU.
>
> Type:
>
> enum in ['AUTO', 'CPU', 'GPU'], default 'AUTO'

> compositor_denoise_final_quality
>
> The quality used by denoise nodes during the compositing of final renders if the nodes’ quality option is set to Follow Scene

(its type line, a few lines further down: `enum in ['HIGH', 'BALANCED', 'FAST'], default 'HIGH'`)

> compositor_denoise_preview_quality
>
> The quality used by denoise nodes during viewport and interactive compositing if the nodes’ quality option is set to Follow Scene

(its type line: `enum in ['HIGH', 'BALANCED', 'FAST'], default 'BALANCED'`)

The manual documents the same three under a "Denoise Nodes" heading, with identical wording on both the Cycles and the EEVEE performance pages — manual, https://docs.blender.org/manual/en/5.0/render/cycles/render_settings/performance.html and https://docs.blender.org/manual/en/5.0/render/eevee/render_settings/performance.html, read 2026-07-30:

> Denoise Nodes
>
> Denoising Device
>
> The device to use to process Denoise nodes in the compositor.

> Preview Quality
>
> The quality used by Denoise nodes during viewport
> and interactive compositing of a render if their quality is set to Follow Scene.

> Final Quality
>
> The quality used by Denoise nodes during the final
> render if their quality is set to Follow Scene.

And the node end of that link — manual, https://docs.blender.org/manual/en/5.0/compositing/types/filter/denoise.html, read 2026-07-30:

> Quality
>
> Follow Scene:
>
> Use the scene’s quality setting.

### EEVEE's noise-reduction settings

Three of them sit inside the scene's EEVEE ray-tracing options (`scene.eevee.ray_tracing_options`) — Python API documentation, `RaytraceEEVEE`, https://docs.blender.org/api/5.0/bpy.types.RaytraceEEVEE.html, read 2026-07-30:

> denoise_bilateral
>
> Blur the resolved radiance using a bilateral filter
>
> Type:
>
> boolean, default True

> denoise_spatial
>
> Reuse neighbor pixels’ rays
>
> Type:
>
> boolean, default True

> denoise_temporal
>
> Accumulate samples by reprojecting last tracing results
>
> Type:
>
> boolean, default True

> use_denoise
>
> Enable noise reduction techniques for raytraced effects
>
> Type:
>
> boolean, default True

That is four, counting the master switch. The manual gives them plainer descriptions — manual, https://docs.blender.org/manual/en/5.0/render/eevee/render_settings/raytracing.html, read 2026-07-30:

> Denoising
>
> Denoising can be enabled to reduce the amount of noise from the raw ray-traced output.
> This can help image stability but will also over-blur the final ray-traced output.
>
> Spatial Reuse
>
> Reuse the rays from neighbor pixels.
> Can introduce some light leaks across surfaces.
>
> Temporal Accumulation
>
> Accumulate samples by re-projecting the last ray tracing results.
> This removes Fireflies but also introduces color bias.
> Useful for viewport temporal stability or making renders converge faster.
>
> Bilateral Filter
>
> Blur the resolved ray-traced output using a bilateral filter.

One more sits directly on the scene's EEVEE settings — Python API documentation, `SceneEEVEE`, https://docs.blender.org/api/5.0/bpy.types.SceneEEVEE.html, read 2026-07-30:

> use_taa_reprojection
>
> Denoise image using temporal reprojection (can leave some ghosting)
>
> Type:
>
> boolean, default True

The manual calls the same thing Temporal Reprojection and describes it as a viewport setting — manual, https://docs.blender.org/manual/en/5.0/render/eevee/render_settings/sampling.html, read 2026-07-30, under the "Viewport" heading:

> Temporal Reprojection
>
> Reduces noise while moving the viewport or during animation playback. Can leave some ghosting.

### LABEL

Mixed, split by part:

- The existence, name, kind, default and choice lists of every Cycles setting in groups 1, 2 and 3: **observed**, from Cycles' own files.
- What each Cycles setting does, for the ones the manual covers: **documented**.
- The per-view-layer Denoising switch and the Store Denoising Passes switch: **observed** only. Their behaviour is **not in the sources** beyond the one-line descriptions inside Cycles' own file and the pass-list code quoted above — the manual does not carry them.
- The final-render Denoiser list containing only OptiX and OpenImageDenoise while the viewport list also contains Automatic: **observed**.
- Group 4: **documented**, in both the API documentation and the manual.
- The EEVEE settings: **documented**.

### Where I looked

- Searched the whole manual for `denois` (case-insensitive). Fourteen files matched. Read all fourteen at the matching places: the Cycles Sampling page, the Cycles Performance page, the Cycles GPU Rendering page, the Cycles Reducing Noise page, the EEVEE Performance page, the EEVEE Raytracing page, the View Layer Passes page, the Denoise Node page, and the table-of-contents and index files (`render/cycles/render_settings/index.txt`, `render/cycles/optimizations/index.txt`, `compositing/index.txt`, `compositing/types/filter/index.txt`, `genindex.txt`) plus the version history page. The last six carry only headings or historical one-liners and were ruled out on that basis.
- Read the whole EEVEE Sampling page and the whole Cycles Sampling page rather than just the matching lines.
- Read the whole View Layer Filter page (`render/layers/filter.html`) and the whole View Layer page (`render/layers/view_layer.html`) looking for the per-view-layer Denoising switch. It is in neither. The Filter page lists Include (Environment, Surfaces, Curves, Volume, Grease Pencil) and Use (Motion Blur only) — no Denoising — even though Cycles' own `ui.py` draws Denoising in that same "Use" column, next to Motion Blur, at lines 985–991.
- Opened the one image on the View Layer page, `_images/render_layers_view-layer_panel.png`. It shows the View Layer panel (Use for Rendering, Render Single Layer) and a Passes panel (Combined, Z, Mist, Normal, Ambient Occlusion, Subsurface Direct, Subsurface Color). No denoising control is visible in it, so it does not settle the question either way. **documented** that those are the controls the picture shows.
- Searched the whole API documentation for `denois`. Sixteen files matched. The substantive ones are `bpy.types.RaytraceEEVEE.txt`, `bpy.types.SceneEEVEE.txt`, `bpy.types.RenderSettings.txt`, `bpy.types.CompositorNodeDenoise.txt` and `bpy.ops.cycles.txt`; the rest are index files and node-listing pages.
- Confirmed a real absence in the API documentation: searched the index files for `use_denoising`, `denoising_quality`, `denoising_prefilter` and `denoiser` — no hits. The `genindex-D` file's only denoise entries are `denoise_animation() (in module bpy.ops.cycles)`, `denoise_bilateral`, `denoise_spatial` and `denoise_temporal`. So none of the Cycles denoising settings appear in the Python API documentation at all; the only place they are written down is Cycles' own files. (I did not find a statement explaining why, and I am not guessing.)
- Searched the whole of `Blender Sources/cycles_5.0` for `denois`. Read every hit in `properties.py` (all of them, quoted above where relevant), `ui.py` (30 hits, all read), `engine.py`, `presets.py` and `version_update.py`.
- Searched `Blender Sources/cycles_5.0/source` for `denois`. Sixteen files matched, all of them kernel or utility headers (`kernel/film/denoising_passes.h`, `kernel/film/volume_guiding_denoise.h`, and so on). None of them is a settings file, so none can name a scene or view-layer setting; ruled out on that basis.
- Searched `Blender Sources/release_notes_5.0` for `denois` across all twenty-three files. One hit, in `cycles.md`: "The OptiX denoiser result quality is improved by flipping the image vertically to have a top-left origin before processing." — https://developer.blender.org/docs/release_notes/5.0/cycles/, read 2026-07-30. It is about the denoiser's output, not about a setting, so it does not change the list.
- Also searched the release notes for `quality` in `compositor.md` in case the compositor denoise quality settings had changed. The hits are about the Glare node, not denoising.
- Terms searched across the manual and the API documentation, beyond the ones above: `denoiser`, `OpenImageDenoise`, `OptiX`, `prefilter`, `noise reduction`, `temporal reprojection`, `taa`, `Noisy Image`, `Denoising Data`.

### Not covered

- I could not settle what the per-view-layer Cycles Denoising switch actually does to the rendered image, because the manual does not carry it and Cycles' own file gives it only the one-line description "Denoise the rendered image". The pass-list code shows the two switches combined with "and"; what the renderer does with that is beyond what I read.
- Nothing I read says what happens when the scene's denoiser choice names a denoiser the machine cannot provide (OptiX with no NVIDIA card, for instance). The choice list is built to omit unavailable entries, but a file arriving with an unavailable choice already stored is not addressed.
- I have not settled whether the EEVEE ray-tracing denoise switches and the temporal reprojection switch are "denoising" in the sense the question means. I included them and said why. If the question means only the AI denoiser applied to a finished image, the answer is groups 1 to 4 and nothing else.
- The manual's Denoiser section lists Automatic as a final-render choice while Cycles' own file builds that list without it. I am reporting the discrepancy, not resolving it.
- Nothing I read states the order in which these apply — for example whether a view layer's denoising happens before or after anything the compositor's Denoise node does. The Denoise Node page says it "Can be used ... as a replacement for automatic denoising", which implies a choice between them, but "implies" is not "states".
- Whether any of these settings can be animated, driven, or overridden per view layer beyond the two that live on the view layer is not covered by anything I read.

### Outside the interface

Partly, and the split is clean but only partly documented.

Groups 1, 3 and 4's final-quality setting are about the final render, so nothing I read suggests they behave differently when Blender is driven from outside its window.

Group 2 is viewport-only by its own descriptions ("Denoise the image in the 3D viewport", "Sample at which to start denoising in the 3D Viewport"), as is `compositor_denoise_preview_quality` ("during viewport and interactive compositing") and EEVEE's Temporal Reprojection (the manual files it under "Viewport"). That these have no bearing on a render started from outside with no window is **inferred**, and the step is: the descriptions say viewport, and I am reading "no viewport" as "no effect". Nothing I read states it.

I found no statement anywhere in the manual, the API documentation or the release notes about denoising in background or command-line rendering. The one command-line caveat I found on any render-settings page is unrelated to denoising — manual, https://docs.blender.org/manual/en/5.0/render/layers/view_layer.html, read 2026-07-30, about Render Single Layer:

> This option is ignored when rendering from the command line.

So for denoising specifically: the sources do not say.

### Operating system

This one genuinely differs by operating system, and the manual says so — but about which denoiser can be accelerated, not about which settings exist.

Manual, https://docs.blender.org/manual/en/5.0/render/cycles/gpu_rendering.html, read 2026-07-30:

> OptiX is supported on Windows and Linux and requires a NVIDIA graphics cards with compute capability 5.0 and higher
> and a driver version of at least 535.

> GPU acceleration for OpenImageDenoise is available for compute capability 7.0 and higher, which includes
> all NVIDIA RTX cards.

> HIP is supported on Windows and Linux and requires a
> AMD graphics card with the RDNA1 architecture or newer. Both discrete GPUs and APUs are supported.

> GPU accelerated denoising is available on discrete Radeon RX 6000 and Radeon RX 7000 GPUs.

> oneAPI is a computation library that is supported on Windows and Linux and requires a
> Intel® Arc™ graphics card with the Xe HPG architecture.
> Hardware acceleration for ray-tracing and denoising is supported.

> Metal is supported on Apple computers with Apple Silicon.
> macOS 13.0 or newer is required to support all features.

> GPU accelerated denoising is available on Apple Silicon.

Reading those together: OptiX is not available on macOS, because OptiX is stated as Windows and Linux only. That is **documented** for the platform statement and **inferred** for the "therefore the OptiX denoiser choice cannot appear on macOS" step — the step being that the manual states OptiX itself is Windows and Linux only, and the Denoiser entry says OptiX is "Only available on NVIDIA GPUs when configured in the Cycles Render Devices user preferences".

The settings themselves — the names, the kinds, the defaults — are written once in Cycles' own file with no platform condition, so I have no reason from the sources to think the list of settings differs by operating system. The one condition Cycles' file does carry is a build condition, not a platform one: `if _cycles.with_openimagedenoise` (properties.py, line 267).

For the EEVEE denoising settings and the compositor denoise settings, the sources say nothing about operating systems at all.

---


---

## Hand-test results — question 19 (2026-08-30)

**On what computers.** Two, and their results are kept apart.

- **The Creator's own MacBook Pro**, M1 Pro chip, macOS 26.6.2 (build 25G83), Apple's own graphics and no second card. Blender 5.0.1 at `/Applications/Blender 5.0.1.app/Contents/MacOS/Blender`, build hash `a3db93c5b259`.
- **A Linux machine rented for these tests**: Vast.ai instance 49253900, machine 27187, in Poland — a container rather than a whole machine (`systemd-detect-virt` answers `docker`, process 1 is `/usr/bin/dash`, no service manager), Ubuntu 22.04.4 LTS on kernel 6.8.0-48-generic, an AMD Ryzen 5 5600X with 5.76 cores allotted against 12 visible threads, a memory ceiling of about 21.2 GiB against the 31 GB the ordinary reading reports, a 45 GB overlay root, and **one NVIDIA GeForce RTX 3060 with 12288 MiB on driver 550.120**. It had **no display server as rented**; one was installed for the panel test and it draws in software (Xvfb, OpenGL renderer `llvmpipe`). Blender 5.0.1 at `/opt/blender/5.0/blender`, **build hash `a3db93c5b259` — the same build as the Mac's**, checked before any test was run.

Every run used a throwaway settings folder made beforehand and `--factory-startup`, so none of the Creator's own settings was read or touched. Every Cycles render was on the processor except where a card is named.


### Every denoising setting a scene or a view layer carries

**Done.** With no file open, listed every setting on the scene, the scene's render settings, the scene's EEVEE settings and ray-tracing options, the view layer and the view layer's EEVEE settings whose name contains "denois", with its kind, its starting value and its choices.

**Came back**, identically on both machines. Thirteen on `scene.cycles`, three on `scene.render`, four on `scene.eevee.ray_tracing_options`, `use_taa_reprojection` on `scene.eevee`, and **two on `view_layer.cycles`**:

```
E8 view_layer.cycles denoising_store_passes type=BOOLEAN default=False current=False
E8 view_layer.cycles use_denoising type=BOOLEAN default=True current=True
```

**Every starting value this document records is right**, including the pair it makes a point of — `denoising_use_gpu` off for the render and `preview_denoising_use_gpu` on for the viewport.

**EEVEE's view layer carries nothing about denoising in any version** — the run answered `E8 view_layer.eevee NONE`. Its full list is `['ambient_occlusion_distance', 'use_pass_bloom', 'use_pass_transparent', 'use_pass_volume_direct']`, which is a difference between versions worth recording because the four documents in this round disagree about it: **4.5 has three entries and 5.0, 5.1 and 5.2 have four**, the extra one being `ambient_occlusion_distance`. None of them is a denoising setting.

**One difference between versions, tested identically in all four.** Blender 5.2 carries **two more** view-layer denoising settings that 4.5, 5.0 and 5.1 have not got — `denoising_pass_follow_reflections` and `denoising_pass_use_albedo_roughness_weighting`, both starting on. This version has only the two above.

### Which denoisers this machine offers, and what decides it

**Done.** On a factory startup, asked the final-render and viewport denoiser settings for a value that cannot exist, so that Blender's refusal names every real choice; then read what devices Cycles' preferences report; then chose a device type, switched its devices on, and asked again.

**Came back on the Creator's Mac:**

```
E9 denoiser choices <- bpy_struct: item.attr = val: enum "NO_SUCH_CHOICE_XYZ" not found in ('OPENIMAGEDENOISE')
E9 preview_denoiser choices <- bpy_struct: item.attr = val: enum "NO_SUCH_CHOICE_XYZ" not found in ('AUTO', 'OPENIMAGEDENOISE')
E9 compute_device_type='METAL'
E9 devices_for_type OPTIX   -> []
E9 devices_for_type METAL   -> [('Apple M1 Pro (GPU - 14 cores)', True), ('Apple M1 Pro', False)]
E9 set compute_device_type=OPTIX REFUSED TypeError: bpy_struct: item.attr = val: enum "OPTIX" not found in ('NONE', 'METAL')
```

**Came back on the rented Linux machine with an RTX 3060:**

```
E9 denoiser choices <- bpy_struct: item.attr = val: enum "NO_SUCH_CHOICE_XYZ" not found in ('OPTIX', 'OPENIMAGEDENOISE')
E9 preview_denoiser choices <- bpy_struct: item.attr = val: enum "NO_SUCH_CHOICE_XYZ" not found in ('AUTO', 'OPTIX', 'OPENIMAGEDENOISE')
E9 compute_device_type='NONE'
E9 devices_for_type OPTIX   -> [('NVIDIA GeForce RTX 3060', True), ('AMD Ryzen 5 5600X 6-Core Processor', False)]
E9 after compute_device_type=OPTIX (2 devices switched on) denoiser choices <- bpy_struct: item.attr = val: enum "NO_SUCH_CHOICE_XYZ" not found in ('OPTIX', 'OPENIMAGEDENOISE')
```

**What that settles.**

- **The list of denoisers is a fact about the machine, not about the version.** It is the same in all four versions on a given machine, and the two machines differ.
- **OptiX is offered on the machine with an NVIDIA card before any device type has been chosen in the preferences** — the device type reads `NONE` on that first line and OptiX is already there — and switching devices on changes nothing. So the manual's "only available on NVIDIA GPUs when configured in the Cycles Render Devices user preferences" does not describe what governs the list: **having the card is enough.**
- **`AUTO` is never a final-render choice and is always the first viewport choice**, on both machines. The manual reads as though "Automatic" were available to the final render; Cycles' own file is right and the manual is not.

### Whether the two denoising switches must both be on

**Done.** Two ways. First, rendered the test scene at 8 samples with adaptive sampling off in every combination of the scene's switch and the view layer's, and compared the pictures channel by channel, including one picture against itself as a control. Second, read the passes the view layer produces through a Render Layers node, in six combinations of the two switches and Store Denoising Passes.

**The picture came back**, in 5.0.1 on the Creator's Mac:

```
PIXELCASE both on   vs both off  : PIXELS max_difference 0.7333333492279053 channels_differing 6334 of 12288
PIXELCASE scene on, layer off vs both off : PIXELS max_difference 0.0 channels_differing 0 of 12288
PIXELCASE scene off, layer on vs both off : PIXELS max_difference 0.0 channels_differing 0 of 12288
PIXELCASE both on   vs scene on, layer off : PIXELS max_difference 0.7333333492279053 channels_differing 6334 of 12288
PIXELCASE both off  vs both off (control)  : PIXELS max_difference 0.0 channels_differing 0 of 12288
```

The rented Linux machine gave the same verdict, with slightly different figures because the two processors trace slightly different noise; **the three zeros and the control zero were identical on both**.

**The passes came back**, identically on both machines:

```
E7P scene=off layer=off store=off denoising-related=[]
E7P scene=on  layer=on  store=off denoising-related=['Noisy Image']
E7P scene=on  layer=off store=off denoising-related=[]
E7P scene=off layer=on  store=off denoising-related=[]
E7P scene=off layer=off store=on denoising-related=['Denoising Normal', 'Denoising Albedo', 'Denoising Depth']
E7P scene=on  layer=on  store=on denoising-related=['Noisy Image', 'Denoising Normal', 'Denoising Albedo', 'Denoising Depth']
```

**What that settles.** **Cycles denoises a view layer only when the scene's switch and the view layer's are both on** — observed, not inferred. Either switch alone leaves the picture identical, to the last channel, to both switches off. **The "Noisy Image" pass appears under exactly the same condition.** And **Store Denoising Passes is not gated by either switch**: its feature passes appear whenever it is on, whatever the two switches say.


*(Added 2026-08-30, and it is a repeat of the pass half above rather than a new question.
**Observed.** **What was done:** `e7passes.py`, unchanged — the same six combinations of the
scene's switch, the view layer's switch and Store Denoising Passes, read through a Render Layers
node with no file written. **To what:** Blender 5.0.1, build hash `a3db93c5b259`, run with
`-b --factory-startup --python e7passes.py` and a throwaway settings folder, **three times over**.
**On what computer:** the Creator's own Mac — a MacBook Pro with an M1 Pro chip, macOS 26.6.2 (build 25G83), Apple's own graphics and no NVIDIA card. It was run because the hand-test list's summary of this test said the
block came back the same in all four versions, while the four versions' own findings documents did
not agree with each other about the order of the feature passes.*

***A negative result would have looked like*** *a different order from one run to the next, or an
order different from the one this document already records — either would have said this block
could not be relied on.*

***What came back.*** *All three runs gave the same output, character for character:*

```
E7P scene=off layer=off store=off denoising-related=[]  total_sockets=31
E7P scene=on  layer=on  store=off denoising-related=['Noisy Image']  total_sockets=31
E7P scene=on  layer=off store=off denoising-related=[]  total_sockets=31
E7P scene=off layer=on  store=off denoising-related=[]  total_sockets=31
E7P scene=off layer=off store=on denoising-related=['Denoising Normal', 'Denoising Albedo', 'Denoising Depth']  total_sockets=31
E7P scene=on  layer=on  store=on denoising-related=['Noisy Image', 'Denoising Normal', 'Denoising Albedo', 'Denoising Depth']  total_sockets=31
E7P blender 5.0.1
```

***What this settles.*** *The order this document records is what the machine prints, three times
out of three. **Across the four versions the order is not the same**, which is what the question
was: 4.5 and 5.0 give `['Denoising Normal', 'Denoising Albedo', 'Denoising Depth']`, 5.1 gives
`['Denoising Albedo', 'Denoising Normal', 'Denoising Depth']`, and 5.2 gives five names,
`['Denoising Albedo', 'Denoising Specular Albedo', 'Denoising Normal', 'Denoising Roughness',
'Denoising Depth']`. The hand-test list's summary claiming all four were identical has been
corrected there; the four findings documents were right. **Nothing about which passes appear, or
under what condition, changes.***

***What this does not cover.*** *macOS alone. The rented Linux machine is destroyed and nothing was
rented, so the "identically on both machines" line above stands exactly as its own run recorded it
and was not re-checked. Every run here was Blender driven from outside its own window with `-b`.
The raw output, word for word, is at `Audits/Blender Research/Settings/Hand tests — Open Threads
questions — raw output/Mac 2026-08-30/`.)*

### Whether the view layer's Denoising checkbox is drawn dimmed

**Done.** On the rented machine, with a display installed for the purpose, opened Blender with a window in each version, set the Properties editor to its View Layer tab and its search box to "Denoising" so the Filter panel opens, and captured the display from outside Blender — once with the scene's Denoise on and once with it off. **The display draws in software (Xvfb, `llvmpipe`)**, which draws a dimmed checkbox the same way a card would, but is not a real workstation's screen.

**Came back.** Counted over the whole 1400 by 2400 display, the two captures differ by **482 pixels**, in a single region **72 pixels by 13, at 1187 across and 1365 down** — the "Denoising" checkbox and its label, and nothing else on the screen. With the scene's Denoise **on**, that checkbox is drawn in full brightness, white label and bright blue ticked box; with it **off**, it is drawn exactly like the `Motion Blur` checkbox beside it, grey label and muted blue box. **The tick stays in the box in both.**

**What that settles.** The dimming this document infers from one line of Cycles' interface file is what a person sees. **The switch is not cleared and not hidden — it is dimmed and stays ticked.**

### Whether Prefilter and Quality do anything under a denoiser that is not OpenImageDenoise

**Done.** On the rented machine only, because the Creator's Mac offers one denoiser. Rendered the same frame with the path tracing on the processor and the denoising on the card, under OptiX with Prefilter None and Quality Fast, then under OptiX with Prefilter Accurate and Quality High, then the same pair under OpenImageDenoise — and a repeat of one OptiX case as a control.

**Came back**, in 5.0.1:

```
E8BCASE optix_acc_high vs optix_acc_high_repeat (control) : PIXELS max_difference 0.0 channels_differing 0 of 27648
E8BCASE optix_none_fast vs optix_acc_high                : PIXELS max_difference 0.0 channels_differing 0 of 27648
E8BCASE oidn_none_fast  vs oidn_acc_high                 : PIXELS max_difference 0.38431375846266747 channels_differing 10739 of 27648
```

**What that settles.** Under OptiX those two settings change **not one channel of 27 648**; under OpenImageDenoise the same change moves about ten and a half thousand. The control shows two identical runs come back byte for byte the same, so a difference would have been seen. **So the two settings are not merely hidden under another denoiser — they have no effect at all.**

### Whether OptiX could be used at all on that machine, with the path tracing on the card too

**Done.** On the rented Linux machine only, on 2026-08-30, while the entry above was being set up. That entry deliberately keeps the path tracing on the processor and puts only the denoising on the card. Before it was settled that way, the same frame was rendered with the **path tracing on the card as well**, under OptiX, in all four versions on that one machine — Vast.ai instance 49253900, machine 27187, in Poland: a container rather than a whole machine, Ubuntu 22.04.4 LTS on kernel 6.8.0-48-generic, an AMD Ryzen 5 5600X with 5.76 cores allotted, and **one NVIDIA GeForce RTX 3060 with 12288 MiB on driver 550.120**. Blender was driven from a terminal with no window, with `--factory-startup` and a throwaway settings folder, and all four build hashes were checked against the Creator's Mac before anything was run. **This was met beside another test rather than asked as a question**, and it was run once.

**Came back.** **Blender 5.0.1 could use OptiX on that machine**, as could 4.5.12 LTS and 5.1.2. **Blender 5.2.1 LTS, on the same machine, the same card and the same driver, could not**, and wrote no image. These are the machine's own words in 5.2.1, and they are the only output this test captured:

```
New backend is missing implementation for PTX intrinsic optix.ptx.copysign.f32
00:00.711  cycles           | ERROR Failed to load OptiX kernel from '/opt/blender/5.2/5.2/scripts/addons_core/cycles/lib/kernel_optix.ptx.zst' (OPTIX_ERROR_INTERNAL_COMPILER_ERROR)
00:00.711  cycles           | ERROR Refer to the Cycles GPU rendering documentation for possible solutions:
                            | https://docs.blender.org/manual/en/latest/render/cycles/gpu_rendering.html
00:00.711  reports          | ERROR Failed to load OptiX kernel from '…kernel_optix.ptx.zst' (OPTIX_ERROR_INTERNAL_COMPILER_ERROR)
```

**Nothing was captured from the three versions that worked.** The record says they could and shows no output for them, so that half rests on the sentence and not on words of its own.

**What a different result would have looked like.** The other three versions on that same machine, on the same card and the same driver, on the same day, are the control. Had the card or the driver been the whole story, all four would have failed alike; three of the four loaded their OptiX kernel and rendered, which is what makes the fourth's failure a difference between Blenders rather than a broken machine. Had 5.2.1 rendered as well, there would have been no result here at all.

**What this does not cover, and it is almost everything.** **One card, one driver, one machine, one day, one run of four Blenders.** It is **not a statement about Blender 5.2.1 anywhere else**, and nothing in it says the same thing would happen on another card, another driver, or another machine — **every other card and every other driver is untested, in every version**. **No other result in this document depends on it**: the entry above keeps the path tracing on the processor and only the denoising on the card, which is why that one ran in all four versions here. It is written down because a render farm choosing a driver would want to know this can happen.

**Label: observed** (2026-08-30).

### Whether the scene's compositor denoise settings reach a render, under both engines

**Done.** Built a compositor of Render Layers → Denoise → output, switched Cycles' own denoising off so the node was the only thing denoising, and rendered the same frame with `compositor_denoise_final_quality` at Fast and at High, under Cycles and under EEVEE, with a repeat as a control.

**Came back**, in 5.0.1 on the Creator's Mac:

```
E9C node_quality='NO-QUALITY-ON-NODE'
E9CASE cy_high vs cy_high_repeat (control) : PIXELS max_difference 0.0 channels_differing 0 of 27648
E9CASE cy_fast vs cy_high                  : PIXELS max_difference 0.08627453446388245 channels_differing 9136 of 27648
E9CASE ee_fast vs ee_high                  : PIXELS max_difference 0.027450978755950928 channels_differing 3528 of 27648
```

The rented machine gave the same verdict.

*(Added by the citation check of 2026-08-30, which repeated this run on the same Mac and the same build. Its run came back:*

```
E9CASE cy_high vs cy_high_repeat (control) : PIXELS max_difference 0.0 channels_differing 0 of 27648
E9CASE cy_fast vs cy_high                  : PIXELS max_difference 0.08627453446388245 channels_differing 9135 of 27648
E9CASE ee_fast vs ee_high                  : PIXELS max_difference 0.027450978755950928 channels_differing 3528 of 27648
```

*Every figure agrees except one: `9135` against the `9136` above. Both runs are recorded and neither is preferred; the check was not in the room for the first. The verdict the passage rests on is untouched, since the maximum difference agrees exactly and the control was zero in both. The figure has been sent back to the hand tests to be run again.)*

**What that settles.** That setting changes a final render **under both engines** — engine-independent by test rather than by inference. **And one difference between versions, tested in all four.** The Denoise node here carries **no `quality` of its own at all**, where 4.5's does and starts at `FOLLOW_SCENE`. So from 5.0 on the node has no choice but to follow the scene, and the manual's condition "if the nodes' quality option is set to Follow Scene" describes something that exists only in 4.5. That matches the removal this version's change log records, and it is now observed as well.


*(Added 2026-08-30, and it is a repeat of the run above rather than a new question. **Observed.**
**What was done:** the same test, unchanged — the scene built by this version with `make_scene.py`,
a compositor of Render Layers → Denoise → output, Cycles' own denoising off, 96 by 72, 8 samples,
adaptive sampling off, then the five cases and the three comparisons of `compare_pixels.py`.
**To what:** Blender 5.0.1, build hash `a3db93c5b259`, started with `-b --factory-startup` and a throwaway
settings folder, so nothing of the machine's own preferences was loaded. **Run twelve times over in
one sitting**, each round rebuilding the scene from the script. **On what computer:** the Creator's own Mac — a MacBook Pro with an M1 Pro chip, macOS 26.6.2 (build 25G83), Apple's own graphics and no NVIDIA card. It was run
because two earlier runs of this test in 5.0 disagreed by one channel and nobody knew whether the
figure was stable at all.*

***A negative result would have looked like*** *the same count in every round, which would have said
the figure can be quoted as exact. The control — two renders at the same setting compared against
each other — is what says a difference would have been seen if there had been one.*

***What came back.*** *The twelve `cy_fast vs cy_high` lines, in order:*

```
round 1  E9CASE cy_fast vs cy_high : PIXELS max_difference 0.08627453446388245 channels_differing 9136 of 27648
round 2  E9CASE cy_fast vs cy_high : PIXELS max_difference 0.08627453446388245 channels_differing 9135 of 27648
round 3  E9CASE cy_fast vs cy_high : PIXELS max_difference 0.08627453446388245 channels_differing 9135 of 27648
round 4  E9CASE cy_fast vs cy_high : PIXELS max_difference 0.08627453446388245 channels_differing 9136 of 27648
round 5  E9CASE cy_fast vs cy_high : PIXELS max_difference 0.08627453446388245 channels_differing 9136 of 27648
round 6  E9CASE cy_fast vs cy_high : PIXELS max_difference 0.08627453446388245 channels_differing 9134 of 27648
round 7  E9CASE cy_fast vs cy_high : PIXELS max_difference 0.08627453446388245 channels_differing 9137 of 27648
round 8  E9CASE cy_fast vs cy_high : PIXELS max_difference 0.08627453446388245 channels_differing 9136 of 27648
round 9  E9CASE cy_fast vs cy_high : PIXELS max_difference 0.08627453446388245 channels_differing 9134 of 27648
round 10 E9CASE cy_fast vs cy_high : PIXELS max_difference 0.08627453446388245 channels_differing 9135 of 27648
round 11 E9CASE cy_fast vs cy_high : PIXELS max_difference 0.08627453446388245 channels_differing 9136 of 27648
round 12 E9CASE cy_fast vs cy_high : PIXELS max_difference 0.08627453446388245 channels_differing 9136 of 27648
```

*In every one of the twelve rounds the other two lines were identical:*

```
E9CASE cy_high vs cy_high_repeat : PIXELS max_difference 0.0 channels_differing 0 of 27648
E9CASE ee_fast vs ee_high : PIXELS max_difference 0.08627453446388245 channels_differing 3528 of 27648
```

*and the maximum difference on the Cycles line, `0.08627453446388245`, never moved.*

*Four different figures came back — 9134, 9135, 9136 and 9137. **The count is not reproducible in this version on this machine.** The figure recorded above, `9136`, came back in six rounds of the twelve, and the citation check's `9135` in three; two further values appeared that neither earlier run saw. **All the records stand and none is preferred.** Everything else in the block above reproduced exactly, twelve times over.*

*One further reading, taken in 5.0 and not repeated in the other three. The pictures were compared
across rounds, and then one single scene file was rendered again and again. Across rounds the fast
picture is identical every time and the high-quality picture is not:*

```
E13X cy_fast round1 vs round2 : PIXELS max_difference 0.0 channels_differing 0 of 27648
E13X cy_fast round1 vs round3 : PIXELS max_difference 0.0 channels_differing 0 of 27648
E13X cy_fast round1 vs round4 : PIXELS max_difference 0.0 channels_differing 0 of 27648
E13X cy_fast round1 vs round5 : PIXELS max_difference 0.0 channels_differing 0 of 27648
E13X cy_high round1 vs round2 : PIXELS max_difference 0.003921568393707275 channels_differing 1 of 27648
E13X cy_high round1 vs round3 : PIXELS max_difference 0.003921568393707275 channels_differing 1 of 27648
E13X cy_high round1 vs round4 : PIXELS max_difference 0.003921568393707275 channels_differing 2 of 27648
E13X cy_high round1 vs round5 : PIXELS max_difference 0.003921568393707275 channels_differing 2 of 27648
```

*and the five rounds' scene files are not byte-identical either:*

```
round1 scene.blend sha256 bbb06c84c261b86a0d1fac89eb89da7471b64b2d04f56915aeb144f350539c3e
round2 scene.blend sha256 bfb7c112a27e70b927b476f7313f85d4f8ca0eafa46e0545344dbc5d41738961
round3 scene.blend sha256 caae8affe3d940001179c6fc33ea558309e6873bd2abc12f1bec39be69d27da3
round4 scene.blend sha256 8a2363193e8022c753834caaf015faefd6d5380feb4ec21eadb0621b07f5626c
round5 scene.blend sha256 0680339cc8737ff33d0c565ed52d2b508993fd648db5da60a913ee3cbc66c365
```

*Kept to one scene file, six Cycles High renders and six Cycles Fast renders all came back
identical, and the comparison gave the same number six times:*

```
E13R hi1 vs hi2 : PIXELS max_difference 0.0 channels_differing 0 of 27648
E13R hi1 vs hi3 : PIXELS max_difference 0.0 channels_differing 0 of 27648
E13R hi1 vs hi4 : PIXELS max_difference 0.0 channels_differing 0 of 27648
E13R hi1 vs hi5 : PIXELS max_difference 0.0 channels_differing 0 of 27648
E13R hi1 vs hi6 : PIXELS max_difference 0.0 channels_differing 0 of 27648
E13R fa1 vs fa2 : PIXELS max_difference 0.0 channels_differing 0 of 27648
E13R fa1 vs fa3 : PIXELS max_difference 0.0 channels_differing 0 of 27648
E13R fa1 vs fa4 : PIXELS max_difference 0.0 channels_differing 0 of 27648
E13R fa1 vs fa5 : PIXELS max_difference 0.0 channels_differing 0 of 27648
E13R fa1 vs fa6 : PIXELS max_difference 0.0 channels_differing 0 of 27648
E13R fa1 vs hi1 : PIXELS max_difference 0.08627453446388245 channels_differing 9136 of 27648
E13R fa2 vs hi2 : PIXELS max_difference 0.08627453446388245 channels_differing 9136 of 27648
E13R fa3 vs hi3 : PIXELS max_difference 0.08627453446388245 channels_differing 9136 of 27648
E13R fa4 vs hi4 : PIXELS max_difference 0.08627453446388245 channels_differing 9136 of 27648
E13R fa5 vs hi5 : PIXELS max_difference 0.08627453446388245 channels_differing 9136 of 27648
E13R fa6 vs hi6 : PIXELS max_difference 0.08627453446388245 channels_differing 9136 of 27648
```

***What this does and does not touch.*** *The verdict this passage rests on is untouched: the
setting still changes a final render under both engines, and a change of some nine thousand
channels against a control of nought is not in doubt. **What should not be quoted as exact is the
last digit of the count**, in 4.5, 5.0 and 5.1. The same twelve rounds were run in each of the four
versions on the same machine on the same day.*

***What this does not cover.*** *macOS alone. The rented Linux machine that the first run of this
test also used is destroyed, and nothing was rented, so **nothing here says whether the count wobbles
on Linux** and the "the rented machine gave the same verdict" line above stands exactly as its own
run recorded it. Every run here was Blender driven from outside its own window with `-b`. Nothing
here is timed, so the other work running on this Mac that day does not bear on it. The raw output,
word for word, is at `Audits/Blender Research/Settings/Hand tests — Open Threads questions — raw
output/Mac 2026-08-30/`.)*

### What each denoising setting is called on screen

**Done.** Read the display name the running program holds for every denoising setting.

**Came back**, identically on both machines: `use_denoising` is **Use Denoising**, `denoiser` is **Denoiser**, `denoising_input_passes` is **Denoising Input Passes**, `denoising_prefilter` is **Denoising Prefilter**, `denoising_quality` is **Denoising Quality**, `denoising_use_gpu` is **Denoise on GPU**; the viewport set carries the same names with **Viewport** in front, except `preview_denoising_start_sample`, which is **Start Denoising**, and `preview_denoising_use_gpu`, which is **Denoise Preview on GPU**. On the view layer, `use_denoising` is **Use Denoising** and `denoising_store_passes` is **Store Denoising Passes**. On the scene's render settings the three are **Compositor Denoise Node Device**, **Compositor Final Denoise Quality** and **Compositor Preview Denoise Quality**. EEVEE's four ray-tracing settings are **Denoise**, **Spatial Reuse**, **Temporal Accumulation** and **Bilateral Filter**.

**One place where Blender and the manual do not agree.** Blender calls `use_taa_reprojection` **"Viewport Denoising"**; the manual calls it **"Temporal Reprojection"**. This document passes the manual's name on; both names are for the same setting.

### What a render does with a file that names a denoiser the machine has not got

**Done.** Made a file on the rented Linux machine with `scene.cycles.denoiser` set to `OPTIX`, carried it to the Creator's Mac, which offers no OptiX, and rendered it there — once as saved and once with denoising switched off, as a control.

**Came back**, in 5.0.1 on the Mac:

```
00:00.455  bpy.rna          | WARNING current value '2' matches no enum in 'CyclesRenderSettings', '', 'denoiser'
READDEN2 blender=5.0.1 mode=as-saved denoiser_now='' use_denoising=True
00:00.689  render           | Saved: '5.0-as-saved0001.png'
E12CASE 5.0 as-saved vs the same file with denoising switched off : PIXELS max_difference 0.7098039388656616 channels_differing 13746 of 27648
```

and the stored value cannot be assigned back to itself:

```
READDEN re-assigning the value it already holds: REFUSED TypeError: bpy_struct: item.attr = val: enum "" not found in ('OPENIMAGEDENOISE')
```

**What that settles.**

- **The render goes ahead.** Blender does not refuse and does not stop; it saves the picture.
- **The picture is still denoised** — it differs from the same file with denoising switched off by nearly fourteen thousand channels of 27 648. Blender falls back to a denoiser the machine has rather than doing nothing.
- **The setting reads back as an empty string.** A program reading the denoiser out of such a file gets nothing, not `OPTIX`.
- **A warning is printed, and only when the setting is read from a script.** Nothing about the mismatch appears in the render's own output, so a run that never reads the setting says nothing at all about it.

**Outside the interface — brought up to date by the hand tests of 2026-08-30.** Every result above except the dimmed checkbox was got from a Blender started with no window. The denoiser list, the two switches' effect on the picture and on the passes, the compositor denoise settings and the fallback for a denoiser the machine has not got are all **observed** with no window open. **The edge this document flags in `enum_optix_denoiser` was tested and did not behave as the reading suggested**: on a windowless run on a Mac with no NVIDIA card the OptiX entry was **not** offered, and on a windowless run on a machine with such a card it **was**, before any device was configured. So what governs the list is the card, not the absence of a context.

**Operating system — brought up to date by the hand tests of 2026-08-30.** The settings themselves — their names, kinds, starting values and choices — came back **identical on macOS and on Linux**, so nothing about which settings exist depends on the operating system. What differs is which denoisers are on offer: OpenImageDenoise alone on the Mac, OptiX and OpenImageDenoise on the Linux machine with an NVIDIA card. **That difference follows the card rather than the operating system as far as these two machines can show**, and no machine with an AMD or an Intel card was available. Windows is untested; the tests are written out ready to run in `Audits/Blender Research/Settings/Hand tests — Open Threads questions.md`.


---

## Things a person with Blender 5.0 open could settle by hand

*(Pruned on 2026-08-30. Everything this list held was run — see the hand-test results above — except the items below, which are genuinely still open. The tests, with their exact commands and their raw output, are at `Audits/Blender Research/Settings/Hand tests — Open Threads questions.md`.)*

1. **What denoisers a machine with an AMD or an Intel graphics card offers.** The two machines used were a Mac with Apple's own graphics and a Linux machine with an NVIDIA card. Nothing here says what a third kind of card offers, and the manual's claim that AMD denoising starts at a different card generation on Linux and on Windows is untested.
2. **What happens when a sample count is driven, or animated, out of its range.** Every test above set the value directly. A driver and a keyframe are different routes and neither was tried.
3. **What Blender does on loading a file whose stored sample count is above the ceiling.** The correction was observed at the moment of assignment; the file-loading path is a different one, and no file old enough to trigger the squared-samples conversion exists here.
4. **What a number field does when a value outside the range is dragged rather than typed.** The dragging bounds were read off the running program, but no test dragged a control.
5. **Everything on Windows.** Eleven entries are written out ready to run in the hand-test list named above, for the separate Windows batch. They are not unrunnable — this project has a route to a Windows computer, and the account allows one machine at a time.

---

2026-08-30 — Claude Opus 5 (Claude Code) — Settings, Blender 5.0
2026-08-30 — Claude Opus 5 (Claude Code) — hand-test results for the Open Threads questions (18 and 19) written in, the two standing lines on each answer brought up to date, and the researcher's own list pruned to what is still open
2026-08-30 — Claude Opus 5 (Claude Code) — hand-test entries 13 and 14 written in: the compositor denoise-quality comparison run twelve times over in one sitting, and the order of the denoising feature passes read three times over, both on the Creator's Mac and both recorded as observed

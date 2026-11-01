# Settings — Open Threads questions, second pass (Blender 4.5)

Sources consulted, all read on 2026-07-30 (the snapshot date given): the Blender 4.5 manual (`Blender Sources/manual_4.5`, including one image from its `_images` folder), the Blender 4.5 Python API documentation (`Blender Sources/api_4.5`), the Blender 4.5 release notes (`Blender Sources/release_notes_4.5`), and Cycles' own files (`Blender Sources/cycles_4.5`).

I read only the question sheet and files inside Blender Sources/. I did not open any other part of this project.

Questions answered: 18 and 19 only.

---

## 18. What values will a render's sample count be set to, and what does a render do when it is given a value outside those?

### Short answer

There is no single sample count. Each render engine carries its own, and a view layer can carry an override. The accepted values are:

| Setting | Where it lives | Accepted values | Default |
|---|---|---|---|
| Cycles Render (Max) Samples | scene | whole numbers 1 to 16777216 | 4096 |
| Cycles Viewport (Max) Samples | scene | whole numbers 0 to 16777216 | 1024 |
| Cycles Sample Subset Offset | scene | whole numbers 0 to 16777216 | 0 |
| Cycles Sample Subset Length | scene | whole numbers 1 to 16777216 | 2048 |
| Cycles Adaptive Min Samples (render) | scene | whole numbers 0 to 4096 | 0 |
| EEVEE Render Samples | scene | whole numbers 1 and up | 64 |
| EEVEE Viewport Samples | scene | whole numbers 0 and up | 16 |
| Workbench anti-aliasing | scene | a choice from a fixed list: Off, FXAA, 5, 8, 11, 16, 32 | 8 |
| Grease Pencil SSAA Samples | scene | whole numbers 1 and up | 8 |
| Per view layer sample override | view layer | whole numbers 0 and up | 0 |

Out-of-range behaviour splits three ways:

- **Typing a number into the field in Blender's window.** Documented: keyboard entry is allowed a wider range than dragging, "but never wider than the 'hard limit'".
- **Setting the Cycles sample count from a script.** Inferred: it is silently clamped to 1 or to 16777216. Step written out below.
- **Setting the EEVEE, Grease Pencil or view-layer sample count from a script.** Not in the sources. Nothing I could find states what happens, and nothing states that anything is reported.

There is no launch-time instruction that carries a sample count, so no launch-time instruction can hand one an out-of-range value. (See the record of looking.)

### LABEL: mixed — see each part

---

**Part A — the accepted values (Cycles). Label: observed.**

From Cycles' own files. `Blender Sources/cycles_4.5/properties.py`, inside `class CyclesRenderSettings(bpy.types.PropertyGroup):` which begins at line 376. That class is attached to the scene at lines 1102–1106 of the same file: `bpy.types.Scene.cycles = PointerProperty(... type=cls,)`.

Lines 482–487:

```
    samples: IntProperty(
        name="Samples",
        description="Number of samples to render for each pixel",
        min=1, max=(1 << 24),
        default=4096,
    )
```

Lines 488–495:

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

Lines 503–515:

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

Lines 583–589:

```
    adaptive_min_samples: IntProperty(
        name="Adaptive Min Samples",
        description="Minimum AA samples for adaptive sampling, to discover noisy features before stopping sampling. Zero for automatic setting based on noise threshold",
        min=0,
        max=4096,
        default=0,
    )
```

`1 << 24` is 16777216.

Date read: 2026-07-30.

---

**Part B — the accepted values (everything else). Label: documented.**

Per view layer, from the Python API documentation, `https://docs.blender.org/api/4.5/bpy.types.ViewLayer.html`:

> "samples
>
> Override number of render samples for this view layer, 0 will use the scene setting
>
> Type:
>
> int in [0, inf], default 0"

EEVEE, from `https://docs.blender.org/api/4.5/bpy.types.SceneEEVEE.html`:

> "taa_render_samples
>
> Number of samples per pixel for rendering
>
> Type:
>
> int in [1, inf], default 64"

> "taa_samples
>
> Number of samples, unlimited if 0
>
> Type:
>
> int in [0, inf], default 16"

Grease Pencil, from `https://docs.blender.org/api/4.5/bpy.types.SceneGpencil.html`:

> "aa_samples
>
> Number of supersampling anti-aliasing samples per pixel for final render
>
> Type:
>
> int in [1, inf], default 8"

The 4.5 release notes say this Grease Pencil sample count is a render setting, at `https://developer.blender.org/docs/release_notes/4.5/grease_pencil/`:

> "The SSAA sample count can be controlled with the `Grease Pencil` > `Render` > `SSAA Samples` render setting."

Workbench takes a choice from a fixed list rather than a number, from `https://docs.blender.org/api/4.5/bpy.types.SceneDisplay.html`:

> "enum in ['OFF', 'FXAA', '5', '8', '11', '16', '32'], default '8'"

and in the manual at `https://docs.blender.org/manual/en/4.5/render/workbench/sampling.html`:

> "The number of samples are predefined so it uses the best distribution of the samples.
>
> 5, 8, 11, 16, 32"

The manual states the meaning of zero for the Cycles viewport count, at `https://docs.blender.org/manual/en/4.5/render/cycles/render_settings/sampling.html`:

> "Setting this value to zero enables indefinite sampling."

Date read: 2026-07-30.

---

**Part C — a value outside the range, typed into the field inside Blender's window. Label: documented.**

Manual, `https://docs.blender.org/manual/en/4.5/interface/controls/buttons/fields.html`, under "Value Limits":

> "Most numerical values are restricted by "soft limit" and "hard limit" value ranges.
> Changing values by dragging with the mouse is restricted to the "soft limit" value range.
> Input via keyboard will allow the use of wider value ranges, but never wider than the "hard limit"."

Date read: 2026-07-30.

This page does not say what is shown or reported when a value beyond the hard limit is typed. It says only that the range is never exceeded.

---

**Part D — a value outside the range set from a script, for the Cycles sample count. Label: inferred.**

The Python API documentation, `https://docs.blender.org/api/4.5/bpy.props.html`, describing `bpy.props.IntProperty`:

> "min (int) – Hard minimum, trying to assign a value below will silently assign this minimum instead."

> "max (int) – Hard maximum, trying to assign a value above will silently assign this maximum instead."

**The step I took:** Cycles' `samples` is created with `IntProperty(... min=1, max=(1 << 24) ...)` — that is the observed quote in Part A, and `IntProperty` there is the one imported from `bpy.props` (`Blender Sources/cycles_4.5/properties.py` imports it at the top of the file). The API page above says what `min` and `max` do to an assignment on any property made this way. Putting the two together: assigning 0 or a negative number to the Cycles render sample count silently stores 1, and assigning anything above 16777216 silently stores 16777216. No source states this about the Cycles sample count itself; a reader has to join the two statements, so this is inferred, not documented.

Date read: 2026-07-30.

---

**Part E — a value outside the range set from a script, for the EEVEE, Grease Pencil or view-layer counts, and anything reported. Label: not in the sources.**

These settings are not created by Cycles' files, and I found nothing in the manual or the API documentation stating what happens when they are assigned a value beyond the range the API documentation prints for them, nor anything stating that a render reports, warns, refuses, or fails because of an out-of-range sample count. The `bpy.props` sentences quoted in Part D are on the page describing how to *define* new properties from Python; they do not, on their own words, describe these built-in settings. Applying them there would be a guess, so I do not.

### Not covered

- Whether a value beyond the hard limit typed into a field is trimmed silently or shown as something else; the manual says only that the range is never exceeded.
- What the sample count means for the run of a render once it is set — how many samples are actually taken, and what else stops the render. That is a different question and I take no view on it here. I did notice, and note only as the edge of this answer, two lines that turned up in searching: `https://docs.blender.org/manual/en/4.5/render/eevee/render_settings/depth_of_field.html` — "Enabling this option can change the scene's actual sample count." — and `https://docs.blender.org/manual/en/4.5/render/eevee/render_settings/motion_blur.html` — "When using multiple time steps, the render sample count is rounded up to the next multiple". I did not pursue either, because both are about what a render then does rather than about which values the setting accepts.
- Whether the ranges printed in the API documentation as `inf` have a real ceiling in the running program. The documentation prints no ceiling, and I did not find one stated anywhere.
- Sample counts belonging to individual features rather than to the render as a whole (shadow rays, volume steps, ambient-occlusion node samples, and so on). I did not chase these.
- What happens when a driver or an animated value pushes a sample count out of range.

### Outside the interface

The values themselves are properties of the scene, the view layer and the Cycles settings inside the file, so they are the same whether Blender is driven by a person or started by something else — nothing in the sources ties any of them to a window being open.

Two things differ. First, Part C — the "hard limit" trimming of typed input — is expressly about a number field in Blender's window, so it does not describe anything that happens when Blender is started from outside. Second, there is no launch-time instruction that sets a sample count: the manual's list of command-line arguments (`https://docs.blender.org/manual/en/4.5/advanced/command_line/arguments.html`) contains no argument for it, and the only two Cycles add-on arguments are `--cycles-device` and `--cycles-print-stats` (`Blender Sources/cycles_4.5/engine.py`, lines 13–20). So the only route in from outside that the sources show is a script, which puts you in Part D or Part E.

### Operating system

The sources state nothing operating-system-specific about any sample count or about out-of-range values. The ranges are written in files that are the same on every system. The 4.5 Cycles release notes do list operating-system-specific problems (Windows, Ubuntu), but none of them concern sample counts. So: the sources do not say this differs by operating system, and I found nothing that suggests it does.

### Record of looking

Files searched, all under `Blender Sources/`, 4.5 folders only:

- `cycles_4.5/properties.py` — searched for `samples`, `denois`, `class Cycles`. Read lines 260–340, 376–480, 475–620, 1090–1115, 1494–1548 in full.
- `cycles_4.5/ui.py` — searched for `denois`; read lines 196–246, 288–330, 960–1040.
- `cycles_4.5/engine.py`, `__init__.py`, `presets.py`, `version_update.py` — searched for `samples`, `denois`, `add_argument`, `argv`, `parser`; read `engine.py` lines 1–40 and 215–260.
- `cycles_4.5/source/` — listed; it contains only `kernel` and `util` (kernel and maths headers), no scene-synchronisation code, so nothing there states what a sample count does at render time.
- `manual_4.5` — searched the whole folder for: `sample count`, `Max Samples`, `number of samples`, `samples per pixel`, `Sample Override`, `Samples Override`, `Override Samples`, `clamp`, `out of range`, `valid range`, `allowed range`, `limit of the`, `invalid value`, `rejects`, `too high`, `too low`, `reports an error`, `error message`. Read in full: `render/cycles/render_settings/sampling.txt`, `render/eevee/render_settings/sampling.txt`, `render/workbench/sampling.txt`, `render/cycles/render_settings/grease_pencil.txt`, `interface/controls/buttons/fields.txt`, `advanced/command_line/render.txt`, and the Cycles section of `advanced/command_line/arguments.txt`.
- `api_4.5` — searched all files for `samples` as a property heading, and for `clamp`, `clamped`, `hard_min`, `hard_max`, `soft_min`, `soft limit`, `out of range`, `silently`, `TypeError`, `ValueError`. Read: `bpy.props.txt` (integer and float property sections), `bpy.types.IntProperty.txt`, `bpy.types.ViewLayer.txt`, `bpy.types.SceneEEVEE.txt`, `bpy.types.SceneGpencil.txt`, `bpy.types.SceneDisplay.txt`, and `change_log.txt`.
- `release_notes_4.5` — searched all nineteen files for `sample`. Read `cycles.md` and `rendering.md` in full; read the matching lines of `grease_pencil.md`.

Ruled out and why:

- Command-line arguments as a route for an out-of-range sample count: searched `advanced/command_line/` for `sample` and got no match at all, and read the Cycles options section, which lists only two arguments.
- An error or warning being reported: searched the manual for `reports an error` and `error message` (the hits are about graphics drivers, shadow buffers and start-up, none about property values) and the API documentation for `TypeError` and `ValueError` (the hits are about registering classes and about string types, none about numeric ranges).
- The compositor and the video sequencer: their `samples` hits (`CompositorNodeVecBlur`, directional blur, plane track deform, audio) are settings of individual nodes and strips, not a render's sample count, so I did not pursue them.

---


---

## Hand-test results — question 18 (2026-08-30)

**On what computers.** Two, and their results are kept apart.

- **The Creator's own MacBook Pro**, M1 Pro chip, macOS 26.6.2 (build 25G83), Apple's own graphics and no second card. Blender 4.5.12 LTS at `/Applications/Blender 4.5.12.app/Contents/MacOS/Blender`, build hash `84afd5f785f7`.
- **A Linux machine rented for these tests**: Vast.ai instance 49253900, machine 27187, in Poland — a container rather than a whole machine (`systemd-detect-virt` answers `docker`, process 1 is `/usr/bin/dash`, no service manager), Ubuntu 22.04.4 LTS on kernel 6.8.0-48-generic, an AMD Ryzen 5 5600X with 5.76 cores allotted against 12 visible threads, a memory ceiling of about 21.2 GiB against the 31 GB the ordinary reading reports, a 45 GB overlay root, and **one NVIDIA GeForce RTX 3060 with 12288 MiB on driver 550.120**. It had **no display server as rented**; one was installed for the panel test and it draws in software (Xvfb, OpenGL renderer `llvmpipe`). Blender 4.5.12 LTS at `/opt/blender/4.5/blender`, **build hash `84afd5f785f7` — the same build as the Mac's**, checked before any test was run.

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
E1 scene.grease_pencil_settings.aa_samples hard_min=1 hard_max=2147483647 soft_min=1 soft_max=256 default=8 current=1
E1 scene.display.render_aa items=['OFF', 'FXAA', '5', '8', '11', '16', '32'] default='8' current='8'
```

**What that settles for this answer.**

- The upper end this document records as **"inf"** for the EEVEE counts, the view layer's override and the Grease Pencil count **is 2 147 483 647**.
- **`guiding_training_samples` does have a maximum** — 2 147 483 647, the whole-number type's own ceiling — where its declaration writes none.
- Two dragging bounds appear that no source states: **the view layer's override cannot be dragged past 10 000**, and the Grease Pencil count not past 256.
- Every Cycles floor and ceiling this document quotes from Cycles' own file is what the running program holds.
- **Two settings this document does not name are present all the same** — `preview_adaptive_min_samples` and `preview_denoising_start_sample` — and the counts it does name are present in every other version too, so no version's silence about any of them was a difference in Blender.
- One thing to read carefully: the Grease Pencil line ends `current=1` in 4.5 and `current=8` in 5.0, 5.1 and 5.2. **That is the value in each version's startup file, not a limit**; the declared starting value is 8 in all four.

### What a value outside the range does

**Done.** Assigned 0, −5, 16 777 217, 99 999 999, 2 147 483 647, 2 147 483 648 and 10¹² to each count in turn and read the value back, in one run that also printed two lines of its own and attempted a deliberately wrong assignment, so that "nothing was reported" could be told apart from "nothing was captured".

**Came back**, on the Creator's Mac:

```
E2 CONTROL-STDOUT this line proves standard output is captured
E2 CONTROL-STDERR this line proves standard error is captured
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

**Came back**, on the Creator's Mac in 4.5.12 LTS:

```
E6 asked=99999999 stored=16777216 (no error)
E6 engine=CYCLES setting=samples blender=4.5.12 LTS
E6 RENDER-STARTS
Fra:1 Mem:8.86M (Peak 9.09M) | Time:00:00.00 | Mem:0.32M, Peak:0.32M | Scene, ViewLayer | Sample 0/16777216
Fra:1 Mem:8.99M (Peak 9.09M) | Time:00:00.00 | Remaining:14:13:27.99 | Mem:0.36M, Peak:0.36M | Scene, ViewLayer | Sample 1/16777216
Fra:1 Mem:8.99M (Peak 9.09M) | Time:00:15.07 | Remaining:08:34:13.27 | Mem:0.36M, Peak:0.36M | Scene, ViewLayer | Sample 8193/16777216
RUNCAP exit=-9 killed_at_25.0s=True
```

and the case at the other end, as a control:

```
E6 asked=0 stored=1 (no error)
E6 engine=CYCLES setting=samples blender=4.5.12 LTS
E6 RENDER-STARTS
Fra:1 Mem:8.86M (Peak 9.10M) | Time:00:00.00 | Mem:0.32M, Peak:0.32M | Scene, ViewLayer | Sample 0/1
Fra:1 Mem:9.04M (Peak 9.10M) | Time:00:00.01 | Mem:0.36M, Peak:0.36M | Scene, ViewLayer | Sample 1/1
Saved: '.../e6-4.5/e6_0001.png'
RUNCAP exit=0 killed_at_25.0s=False
```

EEVEE, whose count is not corrected at 99 999 999, works to the number it was given: `Rendering 1 / 99999999 samples`. **The rented Linux machine gave the same counts in every case.**

**What that settles.** **The corrected number is the number the render works to.** A count above the ceiling does not fail, does not warn and does not stop — it settles in for a fourteen-hour render at 16 777 216 samples. The only place the corrected figure appears is the ordinary progress line.

**Outside the interface — brought up to date by the hand tests of 2026-08-30.** Every result above was got from a Blender started with no window, driven by a script handed in at launch. So the correction, the refusals and the count the render works to are all **observed** for exactly the case this project cares about. What is still untested outside the window is the soft-limit half of the number-field rule, which has no counterpart there.

**Operating system — brought up to date by the hand tests of 2026-08-30.** Every accepted value, every correction and every refusal came back the same on macOS and on Linux. **One thing does differ, and it is the wording**: the refusal of a number too large for a whole number reads `((-2147483647 -1), 2147483647)` on macOS and `((-0x7fffffff - 1), 0x7fffffff)` on Linux. Windows is untested; the tests are written out ready to run in `Audits/Blender Research/Settings/Hand tests — Open Threads questions.md`.

## 19. What settings does a scene or a view layer carry that govern denoising, and what does each of them do?

### Ambiguity, flagged

"Denoising" could mean narrowly the render engine's own denoising of the rendered image, or broadly anything held by the scene or a view layer that governs any denoising. I answer the broad reading, because the question says "govern denoising" without narrowing it, and I mark which group each setting belongs to. Under the narrow reading, only groups 1 and 2 below would count.

### LABEL: mixed — group 1 observed, groups 2 to 4 documented

---

### Group 1 — Cycles, held by the scene

All quotes from `Blender Sources/cycles_4.5/properties.py`, inside `class CyclesRenderSettings` (begins line 376), which is attached to the scene at lines 1102–1106. **Label: observed.** Date read: 2026-07-30.

Lines 403–408 — the master switch for the final render:

```
    use_denoising: BoolProperty(
        name="Use Denoising",
        description="Denoise the rendered image",
        default=True,
        update=update_render_passes,
    )
```

Lines 409–416 — which denoiser:

```
    denoiser: EnumProperty(
        name="Denoiser",
        description="Denoise the image with the selected denoiser. "
        "For denoising the image after rendering",
        items=enum_denoiser,
        default=4,  # Use integer to avoid error in builds without OpenImageDenoise.
        update=update_render_passes,
    )
```

The choices it offers are built at lines 263–301, by two functions rather than written out as a fixed list. That they therefore depend on the build and on the machine is what those functions do when they run, which the file cannot carry, so that reading is **inferred**, not observed. *(Label added by the citation check of 2026-08-30.)*

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

Lines 417–422 — prefilter:

```
    denoising_prefilter: EnumProperty(
        name="Denoising Prefilter",
        description="Prefilter noisy guiding (albedo and normal) passes to improve denoising quality when using OpenImageDenoise",
        items=enum_denoising_prefilter,
        default='ACCURATE',
    )
```

with the choices at lines 310–323: `NONE` ("No prefiltering, use when guiding passes are noise-free"), `FAST` ("Denoise color and guiding passes together..."), `ACCURATE` ("Prefilter noisy guiding passes before denoising color...").

Lines 423–428 — quality:

```
    denoising_quality: EnumProperty(
        name="Denoising Quality",
        description="Overall denoising quality when using OpenImageDenoise",
        items=enum_denoising_quality,
        default='HIGH',
    )
```

with the choices at lines 325–338: `HIGH` ("High quality"), `BALANCED` ("Balanced between performance and quality"), `FAST` ("High performance").

Lines 429–434 — which passes feed the denoiser:

```
    denoising_input_passes: EnumProperty(
        name="Denoising Input Passes",
        description="Passes used by the denoiser to distinguish noise from shader and geometry detail",
        items=enum_denoising_input_passes,
        default='RGB_ALBEDO_NORMAL',
    )
```

with the choices at lines 304–308: `RGB` ("Don't use utility passes for denoising"), `RGB_ALBEDO` ("Use albedo pass for denoising"), `RGB_ALBEDO_NORMAL` ("Use albedo and normal passes for denoising").

Lines 435–440 — where the denoising runs:

```
    denoising_use_gpu: BoolProperty(
        name="Denoise on GPU",
        description="Perform denoising on GPU devices configured in the system tab in the user preferences. This is significantly faster than on CPU, but requires additional GPU memory. When large scenes need more GPU memory, this option can be disabled",
        default=False,
    )
```

The scene also carries a parallel set for the 3D viewport only, lines 441–480: `use_preview_denoising` (default `False`), `preview_denoiser` (default `AUTO`, described as "Denoise the image after each preview update with the selected denoiser"), `preview_denoising_prefilter` (default `FAST`), `preview_denoising_quality` (default `BALANCED`), `preview_denoising_input_passes` (default `RGB_ALBEDO`), `preview_denoising_start_sample` ("Sample to start denoising the preview at", `min=0, max=(1 << 24)`, default 1), and `preview_denoising_use_gpu` (default `True`).

The manual describes the same panel in words, at `https://docs.blender.org/manual/en/4.5/render/cycles/render_settings/sampling.html` (**label for this quote: documented**):

> "Denoise (Render)
>
> Whether to perform denoising for the final rendered image."

> "Passes
>
> Controls which Render Passes the denoiser should use as input.
> Generally, the more passes the denoiser has access to, the better the result.
> It is recommended to use at least Albedo as None can blur out details,
> especially at lower sample counts."

> "Use GPU
>
> Perform denoising on the GPU.
> This is significantly faster than on CPU, but requires additional GPU memory.
> When large scenes need more GPU memory, this option can be disabled."

---

### Group 2 — Cycles, held by a view layer

`Blender Sources/cycles_4.5/properties.py`, `class CyclesRenderLayerSettings` (begins line 1494), attached to the view layer at lines 1537–1541 (`bpy.types.ViewLayer.cycles = PointerProperty(...)`). **Label: observed.** Date read: 2026-07-30.

Lines 1522–1533:

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

The two switches work together, not independently. `Blender Sources/cycles_4.5/engine.py`, lines 235–239:

```
    # Denoising passes.
    if scene.cycles.use_denoising and crl.use_denoising:
        yield ("Noisy Image", "RGBA", 'COLOR')
        if crl.use_pass_shadow_catcher:
            yield ("Noisy Shadow Catcher", "RGB", 'COLOR')
```

(`crl` is the view layer's Cycles settings and `scene.cycles` the scene's.) The interface reflects the same relationship — `Blender Sources/cycles_4.5/ui.py`, lines 977–978:

```
        sub.prop(view_layer.cycles, "use_denoising", text="Denoising")
        sub.active = scene.cycles.use_denoising
```

The view layer's denoising-data switch appears in the Passes ‣ Data panel, `ui.py` line 1033:

```
        col.prop(cycles_view_layer, "denoising_store_passes", text="Denoising Data")
```

The manual describes what that produces, at `https://docs.blender.org/manual/en/4.5/render/layers/passes.html` (**label: documented**):

> "Denoising Data Cycles
>
> Includes Denoising Albedo, Denoising Normal, and the combined image
> before denoising. Can be used with the Denoise Node
> as a replacement for automatic denoising."

Supporting this, the Python API documentation for the Cycles animation-denoising operator says both levels are consulted, at `https://docs.blender.org/api/4.5/bpy.ops.cycles.html` (**label: documented**):

> "Denoise rendered animation sequence using current scene and view layer settings. Requires denoising data passes and output to OpenEXR multilayer files"

---

### Group 3 — EEVEE, held by the scene

**Label: documented.** Date read: 2026-07-30.

EEVEE's ray-tracing denoising sits on a settings block hanging off the scene's EEVEE settings (`SceneEEVEE.ray_tracing_options`, named in the References section of the page below). From `https://docs.blender.org/api/4.5/bpy.types.RaytraceEEVEE.html`:

> "use_denoise
>
> Enable noise reduction techniques for raytraced effects
>
> Type:
>
> boolean, default True"

> "denoise_spatial
>
> Reuse neighbor pixels' rays
>
> Type:
>
> boolean, default True"

> "denoise_temporal
>
> Accumulate samples by reprojecting last tracing results
>
> Type:
>
> boolean, default True"

> "denoise_bilateral
>
> Blur the resolved radiance using a bilateral filter
>
> Type:
>
> boolean, default True"

The manual explains the same four, at `https://docs.blender.org/manual/en/4.5/render/eevee/render_settings/raytracing.html`:

> "Denoising
>
> Denoising can be enabled to reduce the amount of noise from the raw ray-traced output.
> This can help image stability but will also over-blur the final ray-traced output."

> "Spatial Reuse
>
> Reuse the rays from neighbor pixels.
> Can introduce some light leaks across surfaces."

> "Temporal Accumulation
>
> Accumulate samples by re-projecting the last ray tracing results.
> This removes Fireflies but also introduces color bias.
> Useful for viewport temporal stability or making renders converge faster."

> "Bilateral Filter
>
> Blur the resolved ray-traced output using a bilateral filter."

One further EEVEE scene setting is described by the API documentation in denoising terms, though the manual files it under Viewport. From `https://docs.blender.org/api/4.5/bpy.types.SceneEEVEE.html`:

> "use_taa_reprojection
>
> Denoise image using temporal reprojection (can leave some ghosting)
>
> Type:
>
> boolean, default True"

and the manual, `https://docs.blender.org/manual/en/4.5/render/eevee/render_settings/sampling.html`, under the heading "Viewport":

> "Temporal Reprojection
>
> Reduces noise while moving the viewport or during animation playback. Can leave some ghosting."

---

### Group 4 — scene settings that govern the compositor's Denoise node

**Label: documented.** Date read: 2026-07-30.

These are on the scene's render settings, and they exist for every engine — the manual repeats the same panel on the Cycles page and the EEVEE page. From `https://docs.blender.org/api/4.5/bpy.types.RenderSettings.html`:

> "compositor_denoise_device
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
> enum in ['AUTO', 'CPU', 'GPU'], default 'AUTO'"

> "compositor_denoise_final_quality
>
> The quality used by denoise nodes during the compositing of final renders if the nodes' quality option is set to Follow Scene
>
> ...
>
> Type:
>
> enum in ['HIGH', 'BALANCED', 'FAST'], default 'HIGH'"

> "compositor_denoise_preview_quality
>
> The quality used by denoise nodes during viewport and interactive compositing if the nodes' quality option is set to Follow Scene
>
> ...
>
> Type:
>
> enum in ['HIGH', 'BALANCED', 'FAST'], default 'BALANCED'"

The manual gives the panel, at `https://docs.blender.org/manual/en/4.5/render/cycles/render_settings/performance.html` and identically at `https://docs.blender.org/manual/en/4.5/render/eevee/render_settings/performance.html`:

> "Denoise Nodes
>
> Denoising Device
>
> The device to use to process Denoise nodes in the compositor."

The link back from the node is documented too, at `https://docs.blender.org/api/4.5/bpy.types.CompositorNodeDenoise.html`:

> "FOLLOW_SCENE
> Follow Scene – Use the scene's denoising quality setting."

The 4.5 release notes record the compositor change, at `https://developer.blender.org/docs/release_notes/4.5/compositor/`:

> "The Denoise node now supports GPU devices."

and the 4.5 API change log lists `bpy.types.RenderSettings.compositor_denoise_device` under "Added" (`https://docs.blender.org/api/4.5/change_log.html`).

---

### What a view layer does **not** carry

The manual's picture of the View Layer panel — `_images/render_layers_view-layer_panel.png`, shown on `https://docs.blender.org/manual/en/4.5/render/layers/view_layer.html` and captioned "View Layer panel (shown here for the EEVEE render engine)" — shows two switches under "View Layer" (Use for Rendering, Render Single Layer) and a block of pass checkboxes. No denoising control is visible in it. **Label: documented** (it is part of the manual), and I take from it only what is visible: this panel, for EEVEE, shows no denoising control. The same manual page says the rest is engine-specific: "Additional options shown in this panel are different for each render engine."

So on the sources I have, the only per-view-layer denoising settings are the two Cycles ones in Group 2.

### Not covered

- What each setting does to the finished picture. The sources describe what the settings are for; they do not describe the result of any particular combination, and I did not run a render.
- Whether the denoiser choice list on a given machine contains OptiX or OpenImageDenoise. The Cycles files show the list is assembled from what the build supports and what devices are configured, so it is not fixed; I could not determine what any particular machine would offer.
- The Denoise node's own settings (prefilter, quality, its inputs). Those belong to a node, not to a scene or a view layer, so they are outside what was asked; I mention the node only where a scene setting reaches into it.
- Any denoising held elsewhere — on an object, a material, or the user preferences. I did not look for those; the question asked about the scene and the view layer.
- Whether the EEVEE ray-tracing denoising settings apply to a final render as well as the viewport. The manual's Raytracing page does not split viewport from render.

### Outside the interface

Every setting above is stored in the file, on the scene or the view layer, so nothing in the sources ties any of them to a window being open. The one place the sources do draw a line is the choice of denoiser: `enum_optix_denoiser` in `Blender Sources/cycles_4.5/properties.py` (lines 271–276) reads `context.preferences.addons[...].preferences.get_devices_for_type('OPTIX')` and returns nothing if there is no such device — and the function's first test is `if not context`, which offers OptiX when there is no context at all. What that means for a render started from outside I do not know, and the sources do not say, so I do not claim it.

There is no launch-time instruction that turns denoising on or off, sets the denoiser, or sets its quality: the manual's command-line argument list has no such argument, and Cycles' own two arguments are `--cycles-device` and `--cycles-print-stats` only (`Blender Sources/cycles_4.5/engine.py`, lines 13–20). The only route in from outside that the sources show is a script.

### Operating system

Partly, and one setting is expressly not the same everywhere. The OptiX denoiser is documented as NVIDIA-only, at `https://docs.blender.org/manual/en/4.5/render/cycles/render_settings/sampling.html`:

> "OptiX:
>
> Uses NVIDIA's OptiX AI denoiser.
> Supports GPU acceleration on some older NVIDIA GPUs where OpenImageDenoise does not.
>
> Only available on NVIDIA GPUs when configured in the Cycles Render Device user preferences."

That is a graphics-card condition rather than an operating-system one, but it does mean the denoiser choice is not the same on every machine. `enum_openimagedenoise_denoiser` in `properties.py` (lines 263–269) similarly returns nothing unless `_cycles.with_openimagedenoise` is set, which is a property of the build.

For the rest, the sources say nothing operating-system-specific. Note that Cycles' scene denoiser property carries the comment `# Use integer to avoid error in builds without OpenImageDenoise.` (line 414), which shows the writers expected builds where that denoiser is missing.

### Record of looking

Files searched, all under `Blender Sources/`, 4.5 folders only:

- `manual_4.5` — searched the whole folder for `denois` (case-insensitive), which returned exactly thirteen files: `compositing/index.txt`, `compositing/types/filter/denoise.txt`, `compositing/types/filter/index.txt`, `genindex.txt`, `getting_started/about/history.txt`, `render/cycles/gpu_rendering.txt`, `render/cycles/optimizations/index.txt`, `render/cycles/optimizations/reducing_noise.txt`, `render/cycles/render_settings/index.txt`, `render/cycles/render_settings/performance.txt`, `render/cycles/render_settings/sampling.txt`, `render/eevee/render_settings/performance.txt`, `render/eevee/render_settings/raytracing.txt`, `render/layers/passes.txt`. I read every one of those that could hold a scene or view-layer setting. Also searched for `noise reduction` and `Sample Override`. Read in full: `render/cycles/render_settings/sampling.txt`, `render/eevee/render_settings/raytracing.txt`, `render/eevee/render_settings/sampling.txt`, `render/layers/view_layer.txt`.
- `manual_4.5/_images` — listed every file whose name contains `denois` or `sampling`. The only ones are two before/after example photographs, one compositing example, one node picture and one unrelated texture panel; there is no screenshot of the Cycles Sampling or Denoise panel in the manual. I opened `render_layers_view-layer_panel.png`, which is the one panel screenshot bearing on what a view layer carries.
- `api_4.5` — searched all files for `denois` (case-insensitive). Outside the general index files the hits are in exactly six pages: `bpy.ops.cycles.txt`, `bpy.types.CompositorNode.txt` (a list of node names), `bpy.types.CompositorNodeDenoise.txt`, `bpy.types.NodeInternalSocketTemplate.txt` (a list), `bpy.types.RaytraceEEVEE.txt`, `bpy.types.RenderSettings.txt`, `bpy.types.SceneEEVEE.txt`, plus one line of `change_log.txt`. I read all of the relevant ones. Also searched `bpy.types.Scene.txt` and `bpy.types.ViewLayer.txt` directly for `denois` — no match in either, which is why the per-view-layer denoising in Cycles shows up only in Cycles' own files.
- `cycles_4.5` — searched `properties.py`, `ui.py`, `engine.py`, `__init__.py`, `presets.py`, `version_update.py` for `denois`. Read `properties.py` lines 260–340, 395–480, 1494–1548 and the two `register` methods; `ui.py` lines 246–360 and 960–1040; `engine.py` lines 1–40 and 215–260.
- `release_notes_4.5` — searched all nineteen files for `denois`; only two lines match, one in `compositor.md` and one in `bugfixes.md`. Read `cycles.md` and `rendering.md` in full to confirm neither adds a denoising setting in 4.5.
- `advanced/command_line/` — searched for `denois`; no match.

Ruled out and why:

- `getting_started/about/history.txt` and `compositing/index.txt` — mentions in passing, not settings.
- `render/cycles/gpu_rendering.txt` and `render/cycles/optimizations/reducing_noise.txt` — these point at the Sampling panel rather than describing settings of their own; `reducing_noise.txt` says only "To use this enable Denoising in the Render tab of the Properties."
- `compositing/types/filter/denoise.txt` and `bpy.types.CompositorNodeDenoise.txt` — the node's own settings, which belong to a node rather than to a scene or a view layer. I used them only for the "Follow Scene" link back to the scene.
- `bpy.types.SceneEEVEE.txt` other sample-related settings (shadow rays and steps, volumetric steps) — these reduce noise as a side effect but are not described as denoising, so I did not include them.

---


---

## Hand-test results — question 19 (2026-08-30)

**On what computers.** Two, and their results are kept apart.

- **The Creator's own MacBook Pro**, M1 Pro chip, macOS 26.6.2 (build 25G83), Apple's own graphics and no second card. Blender 4.5.12 LTS at `/Applications/Blender 4.5.12.app/Contents/MacOS/Blender`, build hash `84afd5f785f7`.
- **A Linux machine rented for these tests**: Vast.ai instance 49253900, machine 27187, in Poland — a container rather than a whole machine (`systemd-detect-virt` answers `docker`, process 1 is `/usr/bin/dash`, no service manager), Ubuntu 22.04.4 LTS on kernel 6.8.0-48-generic, an AMD Ryzen 5 5600X with 5.76 cores allotted against 12 visible threads, a memory ceiling of about 21.2 GiB against the 31 GB the ordinary reading reports, a 45 GB overlay root, and **one NVIDIA GeForce RTX 3060 with 12288 MiB on driver 550.120**. It had **no display server as rented**; one was installed for the panel test and it draws in software (Xvfb, OpenGL renderer `llvmpipe`). Blender 4.5.12 LTS at `/opt/blender/4.5/blender`, **build hash `84afd5f785f7` — the same build as the Mac's**, checked before any test was run.

Every run used a throwaway settings folder made beforehand and `--factory-startup`, so none of the Creator's own settings was read or touched. Every Cycles render was on the processor except where a card is named.


### Every denoising setting a scene or a view layer carries

**Done.** With no file open, listed every setting on the scene, the scene's render settings, the scene's EEVEE settings and ray-tracing options, the view layer and the view layer's EEVEE settings whose name contains "denois", with its kind, its starting value and its choices.

**Came back**, identically on both machines. Thirteen on `scene.cycles`, three on `scene.render`, four on `scene.eevee.ray_tracing_options`, `use_taa_reprojection` on `scene.eevee`, and **two on `view_layer.cycles`**:

```
E8 view_layer.cycles denoising_store_passes type=BOOLEAN default=False current=False
E8 view_layer.cycles use_denoising type=BOOLEAN default=True current=True
```

**Every starting value this document records is right**, including the pair it makes a point of — `denoising_use_gpu` off for the render and `preview_denoising_use_gpu` on for the viewport.

**EEVEE's view layer carries nothing about denoising in any version** — the run answered `E8 view_layer.eevee NONE`. Its full list is `['use_pass_bloom', 'use_pass_transparent', 'use_pass_volume_direct']`, which is a difference between versions worth recording because the four documents in this round disagree about it: **4.5 has three entries and 5.0, 5.1 and 5.2 have four**, the extra one being `ambient_occlusion_distance`. None of them is a denoising setting.

**One difference between versions, tested identically in all four.** Blender 5.2 carries **two more** view-layer denoising settings that 4.5, 5.0 and 5.1 have not got — `denoising_pass_follow_reflections` and `denoising_pass_use_albedo_roughness_weighting`, both starting on. This version has only the two above. So 5.2's document naming them and the others' silence is a real change in Blender, not a gap in the record.

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
- **OptiX is offered on the machine with an NVIDIA card before any device type has been chosen in the preferences** — the device type reads `NONE` on that first line and OptiX is already there — and switching devices on changes nothing. So the manual's "only available on NVIDIA GPUs when configured in the Cycles Render Device user preferences" does not describe what governs the list: **having the card is enough.** *(Quotation corrected by the citation check of 2026-08-30: 4.5's manual writes "Device", singular; 5.0, 5.1 and 5.2 write "Devices".)*
- **`AUTO` is never a final-render choice and is always the first viewport choice**, on both machines. The manual reads as though "Automatic" were available to the final render; Cycles' own file is right and the manual is not.

### Whether the two denoising switches must both be on

**Done.** Two ways. First, rendered the test scene at 8 samples with adaptive sampling off in every combination of the scene's switch and the view layer's, and compared the pictures channel by channel, including one picture against itself as a control. Second, read the passes the view layer produces through a Render Layers node, in six combinations of the two switches and Store Denoising Passes.

**The picture came back**, in 4.5.12 LTS on the Creator's Mac:

```
PIXELCASE both on   vs both off  : PIXELS max_difference 0.615686297416687 channels_differing 6379 of 12288
PIXELCASE scene on, layer off vs both off : PIXELS max_difference 0.0 channels_differing 0 of 12288
PIXELCASE scene off, layer on vs both off : PIXELS max_difference 0.0 channels_differing 0 of 12288
PIXELCASE both on   vs scene on, layer off : PIXELS max_difference 0.615686297416687 channels_differing 6379 of 12288
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
node with no file written. **To what:** Blender 4.5.12 LTS, build hash `84afd5f785f7`, run with
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
E7P blender 4.5.12 LTS
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

**Came back.** Counted over the whole 1400 by 2400 display, the two captures differ by **481 pixels**, in a single region **72 pixels by 13, at 1192 across and 1587 down** — the "Denoising" checkbox and its label, and nothing else on the screen. With the scene's Denoise **on**, that checkbox is drawn in full brightness, white label and bright blue ticked box; with it **off**, it is drawn exactly like the `Motion Blur` checkbox beside it, grey label and muted blue box. **The tick stays in the box in both.**

**What that settles.** The dimming this document infers from one line of Cycles' interface file is what a person sees. **The switch is not cleared and not hidden — it is dimmed and stays ticked.**

### Whether Prefilter and Quality do anything under a denoiser that is not OpenImageDenoise

**Done.** On the rented machine only, because the Creator's Mac offers one denoiser. Rendered the same frame with the path tracing on the processor and the denoising on the card, under OptiX with Prefilter None and Quality Fast, then under OptiX with Prefilter Accurate and Quality High, then the same pair under OpenImageDenoise — and a repeat of one OptiX case as a control.

**Came back**, in 4.5.12 LTS:

```
E8BCASE optix_acc_high vs optix_acc_high_repeat (control) : PIXELS max_difference 0.0 channels_differing 0 of 27648
E8BCASE optix_none_fast vs optix_acc_high                : PIXELS max_difference 0.0 channels_differing 0 of 27648
E8BCASE oidn_none_fast  vs oidn_acc_high                 : PIXELS max_difference 0.32549022138118744 channels_differing 10712 of 27648
```

**What that settles.** Under OptiX those two settings change **not one channel of 27 648**; under OpenImageDenoise the same change moves about ten and a half thousand. The control shows two identical runs come back byte for byte the same, so a difference would have been seen. **So the two settings are not merely hidden under another denoiser — they have no effect at all.**

### Whether OptiX could be used at all on that machine, with the path tracing on the card too

**Done.** On the rented Linux machine only, on 2026-08-30, while the entry above was being set up. That entry deliberately keeps the path tracing on the processor and puts only the denoising on the card. Before it was settled that way, the same frame was rendered with the **path tracing on the card as well**, under OptiX, in all four versions on that one machine — Vast.ai instance 49253900, machine 27187, in Poland: a container rather than a whole machine, Ubuntu 22.04.4 LTS on kernel 6.8.0-48-generic, an AMD Ryzen 5 5600X with 5.76 cores allotted, and **one NVIDIA GeForce RTX 3060 with 12288 MiB on driver 550.120**. Blender was driven from a terminal with no window, with `--factory-startup` and a throwaway settings folder, and all four build hashes were checked against the Creator's Mac before anything was run. **This was met beside another test rather than asked as a question**, and it was run once.

**Came back.** **Blender 4.5.12 LTS could use OptiX on that machine**, as could 5.0.1 and 5.1.2. **Blender 5.2.1 LTS, on the same machine, the same card and the same driver, could not**, and wrote no image. These are the machine's own words in 5.2.1, and they are the only output this test captured:

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

**Came back**, in 4.5.12 LTS on the Creator's Mac:

```
E9C node_quality='FOLLOW_SCENE'
E9CASE cy_high vs cy_high_repeat (control) : PIXELS max_difference 0.0 channels_differing 0 of 27648
E9CASE cy_fast vs cy_high                  : PIXELS max_difference 0.05098041892051697 channels_differing 9178 of 27648
E9CASE ee_fast vs ee_high                  : PIXELS max_difference 0.027450978755950928 channels_differing 3487 of 27648
```

The rented machine gave the same verdict.

**What that settles.** That setting changes a final render **under both engines** — engine-independent by test rather than by inference. **And one difference between versions, tested in all four.** In 4.5 the Denoise node carries a `quality` of its own and it starts at `FOLLOW_SCENE`; in 5.0, 5.1 and 5.2 the node has no `quality` at all, so from 5.0 on it has no choice but to follow the scene. The manual's condition "if the nodes' quality option is set to Follow Scene" therefore describes something that exists only in 4.5.


*(Added 2026-08-30, and it is a repeat of the run above rather than a new question. **Observed.**
**What was done:** the same test, unchanged — the scene built by this version with `make_scene.py`,
a compositor of Render Layers → Denoise → output, Cycles' own denoising off, 96 by 72, 8 samples,
adaptive sampling off, then the five cases and the three comparisons of `compare_pixels.py`.
**To what:** Blender 4.5.12 LTS, build hash `84afd5f785f7`, started with `-b --factory-startup` and a throwaway
settings folder, so nothing of the machine's own preferences was loaded. **Run twelve times over in
one sitting**, each round rebuilding the scene from the script. **On what computer:** the Creator's own Mac — a MacBook Pro with an M1 Pro chip, macOS 26.6.2 (build 25G83), Apple's own graphics and no NVIDIA card. It was run
because two earlier runs of this test in 5.0 disagreed by one channel and nobody knew whether the
figure was stable at all.*

***A negative result would have looked like*** *the same count in every round, which would have said
the figure can be quoted as exact. The control — two renders at the same setting compared against
each other — is what says a difference would have been seen if there had been one.*

***What came back.*** *The twelve `cy_fast vs cy_high` lines, in order:*

```
round 1  E9CASE cy_fast vs cy_high : PIXELS max_difference 0.05098041892051697 channels_differing 9177 of 27648
round 2  E9CASE cy_fast vs cy_high : PIXELS max_difference 0.05098041892051697 channels_differing 9177 of 27648
round 3  E9CASE cy_fast vs cy_high : PIXELS max_difference 0.05098041892051697 channels_differing 9177 of 27648
round 4  E9CASE cy_fast vs cy_high : PIXELS max_difference 0.05098041892051697 channels_differing 9177 of 27648
round 5  E9CASE cy_fast vs cy_high : PIXELS max_difference 0.05098041892051697 channels_differing 9177 of 27648
round 6  E9CASE cy_fast vs cy_high : PIXELS max_difference 0.05098041892051697 channels_differing 9176 of 27648
round 7  E9CASE cy_fast vs cy_high : PIXELS max_difference 0.05098041892051697 channels_differing 9176 of 27648
round 8  E9CASE cy_fast vs cy_high : PIXELS max_difference 0.05098041892051697 channels_differing 9178 of 27648
round 9  E9CASE cy_fast vs cy_high : PIXELS max_difference 0.05098041892051697 channels_differing 9177 of 27648
round 10 E9CASE cy_fast vs cy_high : PIXELS max_difference 0.05098041892051697 channels_differing 9178 of 27648
round 11 E9CASE cy_fast vs cy_high : PIXELS max_difference 0.05098041892051697 channels_differing 9177 of 27648
round 12 E9CASE cy_fast vs cy_high : PIXELS max_difference 0.05098041892051697 channels_differing 9178 of 27648
```

*In every one of the twelve rounds the other two lines were identical:*

```
E9CASE cy_high vs cy_high_repeat : PIXELS max_difference 0.0 channels_differing 0 of 27648
E9CASE ee_fast vs ee_high : PIXELS max_difference 0.05098041892051697 channels_differing 3487 of 27648
```

*and the maximum difference on the Cycles line, `0.05098041892051697`, never moved.*

*Three different figures came back — 9176, 9177 and 9178. **The count is not reproducible in this version on this machine.** The figure recorded above, `9178`, came back in three rounds of the twelve; nine gave 9177 or 9176. **Both records stand and neither is preferred** — the first run is the first run, and these twelve are these twelve. Everything else in the block above reproduced exactly, twelve times over.*

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

**Came back**, in 4.5.12 LTS on the Mac:

```
WARN (bpy.rna): source/blender/python/intern/bpy_rna.cc:1362 pyrna_enum_to_py: current value '2' matches no enum in 'CyclesRenderSettings', '', 'denoiser'
READDEN2 blender=4.5.12 LTS mode=as-saved denoiser_now='' use_denoising=True
Saved: '4.5-as-saved0001.png'
E12CASE 4.5 as-saved vs the same file with denoising switched off : PIXELS max_difference 0.7176471278071404 channels_differing 13899 of 27648
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

## Things a person with Blender 4.5 open could settle by hand

*(Pruned on 2026-08-30. Everything this list held was run — see the hand-test results above — except the items below, which are genuinely still open. The tests, with their exact commands and their raw output, are at `Audits/Blender Research/Settings/Hand tests — Open Threads questions.md`.)*

1. **What denoisers a machine with an AMD or an Intel graphics card offers.** The two machines used were a Mac with Apple's own graphics and a Linux machine with an NVIDIA card. Nothing here says what a third kind of card offers, and the manual's claim that AMD denoising starts at a different card generation on Linux and on Windows is untested.
2. **What happens when a sample count is driven, or animated, out of its range.** Every test above set the value directly. A driver and a keyframe are different routes and neither was tried.
3. **What Blender does on loading a file whose stored sample count is above the ceiling.** The correction was observed at the moment of assignment; the file-loading path is a different one, and no file old enough to trigger the squared-samples conversion exists here.
4. **What a number field does when a value outside the range is dragged rather than typed.** The dragging bounds were read off the running program, but no test dragged a control.
5. **Everything on Windows.** Eleven entries are written out ready to run in the hand-test list named above, for the separate Windows batch. They are not unrunnable — this project has a route to a Windows computer, and the account allows one machine at a time.

---

2026-08-30 — Claude Opus 5 (Claude Code) — Settings (Open Threads questions 18 and 19), Blender 4.5
2026-08-30 — Claude Opus 5 (Claude Code) — hand-test results for the Open Threads questions (18 and 19) written in, the two standing lines on each answer brought up to date, and the researcher's own list pruned to what is still open
2026-08-30 — Claude Opus 5 (Claude Code) — hand-test entries 13 and 14 written in: the compositor denoise-quality comparison run twelve times over in one sitting, and the order of the denoising feature passes read three times over, both on the Creator's Mac and both recorded as observed

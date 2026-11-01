# Settings — Open Threads questions (18 and 19), Blender 5.1

**Sources consulted, and on what date.** On 2026-07-30 (the snapshot date given to me, used as the date read throughout) I consulted: the Blender 5.1 manual (`Blender Sources/manual_5.1`), the Blender 5.1 Python API documentation (`Blender Sources/api_5.1`), the Blender 5.1 release notes (`Blender Sources/release_notes_5.1`), and Cycles' own files as shipped in `Blender Sources/cycles_5.1`.

I read only the question sheet and files inside Blender Sources/. I did not open any other part of this project.

I did not use the web, and did not need it.

---

## Question 18. What values will a render's sample count be set to, and what does a render do when it is given a value outside those?

**LABEL: mixed — see each part. The acceptable values for Cycles are observed (from Cycles' own files); for EEVEE, Grease Pencil and the view-layer override they are documented (API reference). What happens on an out-of-range value is documented for Python-defined properties and for typing into the interface; inferred for the Cycles sample count specifically; not in the sources for the render itself reporting anything.**

### Ambiguity flagged

"A render's sample count" does not name one setting. Blender 5.1 has several. I answered the reading I believe is meant — the settings that say how many samples a final render takes — and I list each one I found, rather than picking one silently. The other reading would be that only the scene-level Cycles "Max Samples" is meant; if so, the first entry below is the answer and the rest is context.

### The acceptable values, setting by setting

**Cycles, scene-level render sample count — whole numbers from 1 to 16777216, default 4096.**

Exact words, from Cycles' own files:

```
    samples: IntProperty(
        name="Samples",
        description="Number of samples to render for each pixel",
        min=1, max=(1 << 24),
        default=4096,
    )
```

- File: `Blender Sources/cycles_5.1/properties.py`, lines 479–483, inside `class CyclesRenderSettings(bpy.types.PropertyGroup)` (that class begins at line 378).
- Label for this: **observed** — I read the property's declaration in the file supplied, not a description of it.
- `1 << 24` is 16777216. That arithmetic is mine, not the file's.
- Date read: 2026-07-30.

The same class carries these neighbouring sample counts, all observed in `Blender Sources/cycles_5.1/properties.py`:

- `preview_samples` (viewport, lines 485–492): `min=0`, `soft_min=1`, `max=(1 << 24)`, `default=1024`, described as "Number of samples to render in the viewport, unlimited if 0".
- `sample_offset` (lines 501–506): `min=0, max=(1 << 24)`, `default=0`, described as "0-based index of sample to start rendering from".
- `sample_subset_length` (lines 508–513): `min=1, max=(1 << 24)`, `default=2048`, described as "The number of samples to render in this subset".
- `adaptive_min_samples` (lines 580–586): `min=0`, `max=4096`, `default=0`, described as "Minimum AA samples for adaptive sampling, to discover noisy features before stopping sampling. Zero for automatic setting based on noise threshold".
- `preview_adaptive_min_samples` (lines 602–608): `min=0`, `max=4096`, `default=0`, same description with ", for viewport renders" appended.
- `guiding_training_samples` (lines 707–716): `min=0`, `soft_min=1`, `default=128`, and **no `max` is given in the declaration**. Description: "The maximum number of samples used for training path guiding. … A value of 0 will continue training until the last sample".

**EEVEE, final-render sample count — from 1 upwards, default 64.**

Exact words: "Number of samples per pixel for rendering (in [1, inf], default 64)" — the entry for `taa_render_samples`.

- Source: Python API documentation, `https://docs.blender.org/api/5.1/bpy.types.SceneEEVEE.html`
- Label: **documented**. Date read: 2026-07-30.

The viewport counterpart on the same page: `taa_samples` — "Number of samples, unlimited if 0 (in [0, inf], default 16)".

The manual's EEVEE page states the same two settings in words but gives no numbers: "Samples / The number of samples to use in the final render." — `https://docs.blender.org/manual/en/5.1/render/eevee/render_settings/sampling.html`, read 2026-07-30.

**Per view layer, a sample-count override — from 0 upwards, default 0, where 0 means "don't override".**

Exact words: "Override number of render samples for this view layer, 0 will use the scene setting (in [0, inf], default 0)" — the `samples` entry.

- Source: Python API documentation, `https://docs.blender.org/api/5.1/bpy.types.ViewLayer.html`
- Label: **documented**. Date read: 2026-07-30.

The manual describes the same override without numbers: "Allows the view layer to use a different number of samples than the main scene. This is controlled by the Layer Samples when using the Cycles renderer. When using EEVEE, the Samples override is always used." — `https://docs.blender.org/manual/en/5.1/render/layers/override.html`, read 2026-07-30.

**Grease Pencil's own anti-aliasing sample count — from 1 upwards, default 8.**

Exact words: "Number of supersampling anti-aliasing samples per pixel for final render (in [1, inf], default 8)" — the `aa_samples` entry.

- Source: Python API documentation, `https://docs.blender.org/api/5.1/bpy.types.SceneGpencil.html`
- Label: **documented**. Date read: 2026-07-30.

**Workbench — not a number at all, but a fixed list of choices.**

Exact words: "Method of anti-aliasing when rendering final image (default '8')" followed by the choices, ending "Type: Literal[‘OFF’, ‘FXAA’, ‘5’, ‘8’, ‘11’, ‘16’, ‘32’]" — the `render_aa` entry.

- Source: Python API documentation, `https://docs.blender.org/api/5.1/bpy.types.SceneDisplay.html`
- Label: **documented**. Date read: 2026-07-30.

The manual says the same in words: "The number of samples are predefined so it uses the best distribution of the samples. / 5, 8, 11, 16, 32" — `https://docs.blender.org/manual/en/5.1/render/workbench/sampling.html`, read 2026-07-30.

### What happens when a value outside those is given

**For a setting declared with `bpy.props.IntProperty`, the documentation says a number below the minimum or above the maximum is silently replaced by the minimum or maximum.**

Exact words, describing the `min` and `max` arguments of `bpy.props.IntProperty`:

> "min (int) – Hard minimum, trying to assign a value below will silently assign this minimum instead."
>
> "max (int) – Hard maximum, trying to assign a value above will silently assign this maximum instead."

- Source: Python API documentation, `https://docs.blender.org/api/5.1/bpy.props.html`, lines 854 and 856 of the page text.
- Label for this statement on its own: **documented**. Date read: 2026-07-30.

Applying it to the Cycles sample count is one step further, so:

- Label: **inferred**. The step I took: Cycles' `samples` is declared as `IntProperty(… min=1, max=(1 << 24) …)` in `Blender Sources/cycles_5.1/properties.py` lines 479–483 (observed above), and the API documentation says that for an `IntProperty`, assigning below `min` silently assigns `min` and assigning above `max` silently assigns `max`. So setting the Cycles sample count to 0, or to a negative number, ends up as 1; setting it above 16777216 ends up as 16777216; and nothing is reported at the moment of assignment. What a render itself says about it is a separate matter and is not in the sources — see below. The documentation does not name Cycles, and Cycles' files do not describe assignment behaviour — I joined the two.
- I could not find the same statement for the settings that are *not* defined through `bpy.props` — `taa_render_samples`, `ViewLayer.samples`, `aa_samples`. For those the API page gives the range but says nothing about what an out-of-range assignment does. That part is **not in the sources**.

**When typed into Blender's own window, a value can go beyond the drag range but never beyond the hard range.**

Exact words:

> "Most numerical values are restricted by “soft limit” and “hard limit” value ranges. Changing values by dragging with the mouse is restricted to the “soft limit” value range. Input via keyboard will allow the use of wider value ranges, but never wider than the “hard limit”."

- Source: manual, `https://docs.blender.org/manual/en/5.1/interface/controls/buttons/fields.html`
- Label: **documented**. Date read: 2026-07-30.

**What the render reports about it — not in the sources.** I found no statement anywhere in the 5.1 manual, API documentation or release notes that a render prints a warning, writes a message, or fails when a sample count outside the range is given. The API wording is "silently", which is the closest the sources come, and it describes the assignment rather than the render.

**A value that is not a number at all — not in the sources** for sample counts. The API documentation shows one example of a wrong-type assignment raising `TypeError`, but it is about an object's name, not about any sample count: "TypeError: bpy_struct: item.attr= val: Object.name expected a string type, not str" (`https://docs.blender.org/api/5.1/info_gotchas_file_paths_and_encoding.html`, read 2026-07-30). I will not stretch that to sample counts. Likewise for Workbench's fixed list of choices, I found nothing saying what happens when a value outside the list is given.

### Release notes

I searched all twenty 5.1 release-note files for "samples" and "sampling". Nothing in them changes or states sample-count ranges. The only sampling-related lines are about Sculpt and Vertex Paint colour sampling and a compositor scaling bug, none of which bear on this question.

### Where I looked (question 18)

- Searched `Blender Sources/cycles_5.1/properties.py` for `samples`; read every match with its surrounding declaration. Also read `ui.py` around lines 240, 313 and 472 to confirm which of those settings are the ones shown as "Max Samples", "Samples" and "Min Samples".
- Searched the whole of `manual_5.1` for: `Max Samples` (no hits), `sample count` (hits listed above), `samples` within `render/cycles`, `render/eevee`, `render/workbench`, `render/layers`, and `advanced/command_line`. Read in full: `render/cycles/render_settings/sampling.txt`, `render/eevee/render_settings/sampling.txt`, `render/workbench/sampling.txt`, `render/layers/override.txt`.
- Searched the whole of `manual_5.1` for `soft limit` and `hard limit`; the only substantive hit is `interface/controls/buttons/fields.txt`, which I read.
- Searched the whole of `manual_5.1` for `clamp` and `out of range` outside the interface pages; ruled out `advanced/limits.txt` (it is about coordinate precision, not settings) and the node/curve hits (unrelated).
- Searched the whole of `api_5.1` for `taa_render_samples`, `taa_samples`, `samples per pixel`, `clamp`, `hard limit`, `soft limit`, `is clamped`, `are clamped`, `will be clamped`, `out of range`, `ValueError`, `TypeError`. Read `bpy.types.SceneEEVEE.txt`, `bpy.types.ViewLayer.txt`, `bpy.types.SceneDisplay.txt`, `bpy.types.SceneGpencil.txt`, and the `IntProperty` section of `bpy.props.txt`.
- Ruled out `bpy.types.SceneDisplay.matcap_ssao_samples` ("Number of samples (in [1, 500], default 16)"): its own text ties it to the matcap screen-space ambient occlusion effect in the viewport, not to a render's sample count.
- Ruled out the `ValueError` hits in `bpy.props.txt` (lines 412, 424, 489, …): every one of them is about a custom `get`/`set` callback the add-on author writes, not about ordinary assignment.
- Searched `release_notes_5.1` (all files) for `samples` and `sampling`.

### Not covered

- What actually happens at render time when a sample count is out of range for the settings that are not `bpy.props`-defined (EEVEE, the view-layer override, Grease Pencil).
- Whether anything is written to the console, the log, or the status bar when a value is clamped.
- Whether a value that is not a whole number, or not a number at all, is rejected or converted, for any sample count.
- What the upper bound written as "inf" in the API documentation is in practice. I take it as "no upper bound stated"; the sources do not give a number.
- Whether a sample count can be driven or animated to an out-of-range value, and what then happens.

### Outside the interface

The Cycles range itself (`min=1, max=(1 << 24)`) is a property declaration in Cycles' own file. It is part of the setting, not part of the window, so it does not depend on anyone clicking. The "silently assign the minimum/maximum instead" statement is from the Python property documentation, which is the path a render driven from outside Blender would use — so that part is specifically about the non-interface case. The one part that is interface-only is the soft-limit/hard-limit typing rule quoted from `interface/controls/buttons/fields.html`: dragging and keyboard entry only exist inside Blender's window, and the sources say nothing about a soft limit having any effect outside it. Beyond that, the sources do not say whether a render started from outside behaves differently.

### Operating system

The sources state no operating-system difference for any of this. The ranges are in a single shared file (`properties.py`) and the API documentation is not split by system. I found nothing in the 5.1 manual, API documentation or release notes that makes a sample-count range or an out-of-range behaviour depend on the operating system. The nearest thing to a system-dependent statement I found near this subject is about which graphics hardware supports what (`https://docs.blender.org/manual/en/5.1/render/cycles/gpu_rendering.html`), which is a different matter and does not mention sample counts.

---


---

## Hand-test results — question 18 (2026-08-30)

**On what computers.** Two, and their results are kept apart.

- **The Creator's own MacBook Pro**, M1 Pro chip, macOS 26.6.2 (build 25G83), Apple's own graphics and no second card. Blender 5.1.2 at `/Applications/Blender 5.1.2.app/Contents/MacOS/Blender`, build hash `ec6e62d40fa9`.
- **A Linux machine rented for these tests**: Vast.ai instance 49253900, machine 27187, in Poland — a container rather than a whole machine (`systemd-detect-virt` answers `docker`, process 1 is `/usr/bin/dash`, no service manager), Ubuntu 22.04.4 LTS on kernel 6.8.0-48-generic, an AMD Ryzen 5 5600X with 5.76 cores allotted against 12 visible threads, a memory ceiling of about 21.2 GiB against the 31 GB the ordinary reading reports, a 45 GB overlay root, and **one NVIDIA GeForce RTX 3060 with 12288 MiB on driver 550.120**. It had **no display server as rented**; one was installed for the panel test and it draws in software (Xvfb, OpenGL renderer `llvmpipe`). Blender 5.1.2 at `/opt/blender/5.1/blender`, **build hash `ec6e62d40fa9` — the same build as the Mac's**, checked before any test was run.

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

**Came back**, on the Creator's Mac in 5.1.2:

```
E6 asked=99999999 stored=16777216 (no error)
E6 engine=CYCLES setting=samples blender=5.1.2
E6 RENDER-STARTS
00:00.352  render           | Fra: 1 | Mem: 33M | Sample 0/16777216
00:00.354  render           | Fra: 1 | Remaining: 08:54:23.99 | Mem: 33M | Sample 1/16777216
RUNCAP exit=-9 killed_at_25.0s=True

(This run printed only two progress lines before the twenty-five-second limit stopped it; 5.0 and 5.2 reached the 8193rd sample in the same time. Nothing turns on that — the number after the slash is the answer, and it is the same.)
```

and the case at the other end, as a control:

```
E6 asked=0 stored=1 (no error)
E6 engine=CYCLES setting=samples blender=5.1.2
E6 RENDER-STARTS
00:00.569  render           | Fra: 1 | Mem: 2M | Sample 0/1
00:00.576  render           | Fra: 1 | Mem: 2M | Sample 1/1
00:00.650  render           | Saved: '.../e6-5.1/e6_0001.png'
RUNCAP exit=0 killed_at_25.0s=False
```

EEVEE, whose count is not corrected at 99 999 999, works to the number it was given: `Rendering 1 / 99999999 samples`. **The rented Linux machine gave the same counts in every case.**

**What that settles.** **The corrected number is the number the render works to.** A count above the ceiling does not fail, does not warn and does not stop — it settles in for a fourteen-hour render at 16 777 216 samples. The only place the corrected figure appears is the ordinary progress line.

**Outside the interface — brought up to date by the hand tests of 2026-08-30.** Every result above was got from a Blender started with no window, driven by a script handed in at launch. So the correction, the refusals and the count the render works to are all **observed** for exactly the case this project cares about. What is still untested outside the window is the soft-limit half of the number-field rule, which has no counterpart there.

**Operating system — brought up to date by the hand tests of 2026-08-30.** Every accepted value, every correction and every refusal came back the same on macOS and on Linux. **One thing does differ, and it is the wording**: the refusal of a number too large for a whole number reads `((-2147483647 -1), 2147483647)` on macOS and `((-0x7fffffff - 1), 0x7fffffff)` on Linux. Windows is untested; the tests are written out ready to run in `Audits/Blender Research/Settings/Hand tests — Open Threads questions.md`.

## Question 19. What settings does a scene or a view layer carry that govern denoising, and what does each of them do?

**LABEL: mixed — the Cycles settings, on the scene and on the view layer, are observed in Cycles' own files, and the scene-level final-render ones are documented in the manual as well; the per-view-layer "Denoising" toggle is observed only, and the claim that it is greyed out is inferred; the EEVEE settings and the three that steer the compositor's Denoise nodes are documented.**

I found five groups. I give each setting, where it sits, and what the source says it does.

### Group 1 — Cycles, on the scene: final-render denoising

The manual describes these under Render ‣ Sampling. Exact words, from `https://docs.blender.org/manual/en/5.1/render/cycles/render_settings/sampling.html` (manual, read 2026-07-30):

> "Denoising / Denoising uses a specialized algorithm to get a less noisy image without requiring more samples."
>
> "Denoise (Render) / Whether to perform denoising for the final rendered image."
>
> "Denoiser / The algorithm to use." — with three choices: "Automatic: Uses GPU accelerated denoising if supported, for best performance. Prefers OpenImageDenoise over OptiX."; "OpenImageDenoise: Uses Intel’s Open Image Denoise, an AI denoiser. Typically provides the highest quality."; "OptiX: Uses NVIDIA’s OptiX AI denoiser. Supports GPU acceleration on some older NVIDIA GPUs where OpenImageDenoise does not. / Only available on NVIDIA GPUs when configured in the Cycles Render Devices user preferences."
>
> "Passes / Controls which Render Passes the denoiser should use as input. Generally, the more passes the denoiser has access to, the better the result. It is recommended to use at least Albedo as None can blur out details, especially at lower sample counts." — with "None: Denoises the image using color data.", "Albedo: Denoises the image using color and albedo data.", "Albedo + Normal: Denoises the image using color, albedo, and normal pass data."
>
> "Prefilter OpenImageDenoise / Controls whether prefiltering is applied to the Passes before denoising. Visible only when using OpenImageDenoise." — with "None: Does not apply any prefiltering to the input passes. This option retains the most detail and is the fastest, but assumes the input passes are noise free which may require a high sample count. If the input passes aren’t noise free, then noise will remain in the image after denoising."; "Fast: Assumes the input passes are not noise free, yet does not apply prefiltering to the input passes. This option is faster than Accurate but produces a blurrier result."; "Accurate: Prefilters the input passes before denoising to reduce noise. This option usually produces more detailed results than Fast, but with increased processing time."
>
> "Quality OpenImageDenoise / Overall denoising quality. Visible only when using OpenImageDenoise." — with "High: Produces the highest quality at the cost of time."; "Balanced: Balanced between performance and quality."; "Fast: Sacrifices quality for speed (ideal for viewport rendering)."
>
> "Use GPU / Perform denoising on the GPU. This is significantly faster than on CPU, but requires additional GPU memory. When large scenes need more GPU memory, this option can be disabled."

Label for the above: **documented**.

The same settings, with their internal names and defaults, are **observed** in `Blender Sources/cycles_5.1/properties.py`, `class CyclesRenderSettings` (begins line 378), lines 400–437:

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

The choices those enumerations offer are declared just above, lines 306–338 of the same file (also **observed**):

```
enum_denoising_input_passes = (
    ('RGB', "None", "Don't use utility passes for denoising", 1),
    ('RGB_ALBEDO', "Albedo", "Use albedo pass for denoising", 2),
    ('RGB_ALBEDO_NORMAL', "Albedo and Normal", "Use albedo and normal passes for denoising", 3),
)
```

```
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

The list of denoisers is built at run time rather than fixed, lines 265–304 (**observed**):

```
def enum_openimagedenoise_denoiser(self, context):
    import _cycles
    if _cycles.with_openimagedenoise:
        return [('OPENIMAGEDENOISE', "OpenImageDenoise",
                 n_("Use Intel OpenImageDenoise AI denoiser"), 4)]
```

*(Quotation corrected by the citation check of 2026-08-30: the second line was given as `del context`, which appears nowhere in any of the four versions' Cycles files. `cycles_5.1/properties.py` line 266 reads `import _cycles`.)*

```
def enum_denoiser(self, context):
    items = []
    items += enum_optix_denoiser(self, context)
    items += enum_openimagedenoise_denoiser(self, context)
    return items
```

and the OptiX entry, lines 273–277:

```
def enum_optix_denoiser(self, context):
    ...
            "Use the OptiX AI denoiser with GPU acceleration, only available on NVIDIA GPUs when configured in the system tab in the user preferences"), 2)]
```

Date read for all of the above: 2026-07-30. File: `Blender Sources/cycles_5.1/properties.py`.

### Group 2 — Cycles, on the scene: viewport denoising

The manual gives one line: "Denoise (Viewport) / Whether to perform denoising for the 3D Viewport in the Rendered shading mode." and "Start Sample / Sample at which to start denoising in the 3D Viewport." (manual, same Cycles sampling page, read 2026-07-30) — **documented**.

Cycles' own file carries a full parallel set, **observed** in `Blender Sources/cycles_5.1/properties.py` lines 438–478: `use_preview_denoising` ("Denoise the image in the 3D viewport", default False), `preview_denoiser` ("Denoise the image after each preview update with the selected denoiser"), `preview_denoising_prefilter` (default `'FAST'`), `preview_denoising_quality` (default `'BALANCED'`), `preview_denoising_input_passes` (default `'RGB_ALBEDO'`), `preview_denoising_start_sample` ("Sample to start denoising the preview at", `min=0, max=(1 << 24)`, `default=1`), and `preview_denoising_use_gpu` ("Denoise Preview on GPU", default True).

The viewport denoiser list carries an entry the render one does not. That comparison is mine, drawn from the two builder functions; neither fragment below states it on its own. The two fragments are also quoted without the function heads they sit under, so which list each belongs to cannot be told from the quote alone. Lines 280–293 (**observed**, for the fragments themselves):

```
             n_("Use GPU accelerated denoising if supported, for the best performance. "
                "Prefer OpenImageDenoise over OptiX"),
```

```
        items = [('AUTO', "None", n_("Blender was compiled without a viewport denoiser"), 0)]
```

I include this group because these settings sit on the scene, which is what the question asks about. They govern the viewport rather than a final render.

### Group 3 — Cycles, on the view layer

Two settings, both **observed** in `Blender Sources/cycles_5.1/properties.py`, `class CyclesRenderLayerSettings` (begins line 1479), lines 1532–1542:

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

That class attaches itself to the view layer, lines 1546–1551 (**observed**):

```
        bpy.types.ViewLayer.cycles = PointerProperty(
            name="Cycles ViewLayer Settings",
            description="Cycles ViewLayer Settings",
            type=cls,
        )
```

The first of the two is drawn in the view layer's Filter panel. **Observed** in `Blender Sources/cycles_5.1/ui.py`, `class CYCLES_RENDER_PT_filter` (begins line 963, with `bl_context = "view_layer"`), lines 989–991:

```
        sub = col.row()
        sub.prop(view_layer.cycles, "use_denoising", text="Denoising")
        sub.active = scene.cycles.use_denoising
```

The third of those lines sets the row's `active` value from the scene-level setting. That the assignment is there is reading. That the toggle is therefore **greyed out on screen** whenever the scene-level setting is off is a step past reading, so it carries the label **inferred**, not observed. The step: the row holding this toggle is given `sub.active = scene.cycles.use_denoising`, and from that I concluded the toggle appears greyed out. Nothing quoted here says what that value does to what is drawn, and I did not run Blender to look. (Question 18 above treats the matching step the same way — Cycles' file states a range, the documentation states what an out-of-range assignment does, and joining the two to say what happens is labelled inferred there.)

The second is drawn among the view layer's passes, **observed** in the same file, line 1029:

```
        col.prop(cycles_view_layer, "denoising_store_passes", text="Denoising Data")
```

**Worth recording:** the manual's own page for that panel, `https://docs.blender.org/manual/en/5.1/render/layers/filter.html` (read 2026-07-30), lists only Environment, Surfaces, Curves, Volume, Grease Pencil and Motion Blur. It does **not** mention the per-view-layer Denoising toggle. So that setting is observed in Cycles' files and **not in the sources** as far as the manual goes; where I looked for it is recorded below.

The manual does describe what the stored passes are for, on `https://docs.blender.org/manual/en/5.1/render/layers/passes.html` (read 2026-07-30) — **documented**:

> "Denoising Data Cycles / Includes Denoising Albedo, Denoising Normal, and the combined image before denoising. Can be used with the Denoise Node as a replacement for automatic denoising."

Those two passes are declared in Cycles' own file, lines 232–233 (**observed**):

```
    ('DENOISING_ALBEDO', "Denoising Albedo", "Albedo pass used by denoiser"),
    ('DENOISING_NORMAL', "Denoising Normal", "Normal pass used by denoiser"),
```

### Group 4 — EEVEE, on the scene

EEVEE's denoising settings sit inside its ray-tracing options, which hang off the scene's EEVEE settings ("ray_tracing_options / EEVEE settings for tracing reflections (readonly)" — `https://docs.blender.org/api/5.1/bpy.types.SceneEEVEE.html`, read 2026-07-30). That quoted line names the property but not its type, so the link between it and the `RaytraceEEVEE` entries quoted next is not carried by the words I quoted here.

Exact words from `https://docs.blender.org/api/5.1/bpy.types.RaytraceEEVEE.html` (API documentation, read 2026-07-30) — **documented**:

> "denoise_bilateral / Blur the resolved radiance using a bilateral filter (default True)"
>
> "denoise_spatial / Reuse neighbor pixels’ rays (default True)"
>
> "denoise_temporal / Accumulate samples by reprojecting last tracing results (default True)"
>
> "use_denoise / Enable noise reduction techniques for raytraced effects (default True)"

The manual says the same in its own words, on `https://docs.blender.org/manual/en/5.1/render/eevee/render_settings/raytracing.html` (read 2026-07-30) — **documented**:

> "Denoising / Denoising can be enabled to reduce the amount of noise from the raw ray-traced output. This can help image stability but will also over-blur the final ray-traced output."
>
> "Spatial Reuse / Reuse the rays from neighbor pixels. Can introduce some light leaks across surfaces."
>
> "Temporal Accumulation / Accumulate samples by re-projecting the last ray tracing results. This removes Fireflies but also introduces color bias. Useful for viewport temporal stability or making renders converge faster."
>
> "Bilateral Filter / Blur the resolved ray-traced output using a bilateral filter."

One more EEVEE scene setting is described with the word denoise, though the manual files it under viewport sampling. Exact words: "Denoise image using temporal reprojection (can leave some ghosting) (default True)" — the `use_taa_reprojection` entry on `https://docs.blender.org/api/5.1/bpy.types.SceneEEVEE.html` (read 2026-07-30). The manual's EEVEE sampling page lists it under Viewport: "Temporal Reprojection / Reduces noise while moving the viewport or during animation playback. Can leave some ghosting." (`https://docs.blender.org/manual/en/5.1/render/eevee/render_settings/sampling.html`, read 2026-07-30). Both **documented**.

I found no denoising settings on `bpy.types.ViewLayerEEVEE` — its only entries are `ambient_occlusion_distance`, `use_pass_bloom`, `use_pass_transparent` and `use_pass_volume_direct` (`https://docs.blender.org/api/5.1/bpy.types.ViewLayerEEVEE.html`, read 2026-07-30). *(Corrected by the citation check of 2026-08-30: the fourth entry was missing from this list. The conclusion — none of them is about denoising — is unchanged.)*

### Group 5 — scene render settings that govern the compositor's Denoise nodes

These sit on the scene's render settings rather than on Cycles or EEVEE, and they govern denoising done by nodes rather than by the render engine. Exact words from `https://docs.blender.org/api/5.1/bpy.types.RenderSettings.html` (API documentation, read 2026-07-30) — **documented**:

> "compositor_denoise_device / The device to use to process the denoise nodes in the compositor (default 'AUTO')" — choices "AUTO / Auto – Use the same device used by the compositor to process the denoise node.", "CPU / CPU – Use the CPU to process the denoise node.", "GPU / GPU – Use the GPU to process the denoise node if available, otherwise fallback to CPU."
>
> "compositor_denoise_final_quality / The quality used by denoise nodes during the compositing of final renders if the nodes’ quality option is set to Follow Scene (default 'HIGH')"
>
> "compositor_denoise_preview_quality / The quality used by denoise nodes during viewport and interactive compositing if the nodes’ quality option is set to Follow Scene (default 'BALANCED')"

The manual presents the same three under Render ‣ Performance for both engines: "Denoise Nodes / Denoising Device / The device to use to process Denoise nodes in the compositor." and "Preview Quality / The quality used by Denoise nodes during viewport and interactive compositing of a render if their quality is set to Follow Scene." and "Final Quality / The quality used by Denoise nodes during the final render if their quality is set to Follow Scene." — `https://docs.blender.org/manual/en/5.1/render/cycles/render_settings/performance.html` and `https://docs.blender.org/manual/en/5.1/render/eevee/render_settings/performance.html`, both read 2026-07-30. **Documented.**

### Release notes

One 5.1 release note bears on denoising. Exact words:

> "Improved denoising quality with textured transparency and roughness. (blender/blender!154988)"

- Source: release notes, `https://developer.blender.org/docs/release_notes/5.1/cycles/`, under "Other features". Read 2026-07-30. **Documented.**

One further note names denoising, in the corrective-releases list. Exact words:

> "Denoise Animtion operator Broken in 5.0 and 5.1. (blender/blender#155982)"

- Source: release notes, `https://developer.blender.org/docs/release_notes/5.1/corrective_releases/`. Read 2026-07-30. **Documented.** (The spelling "Animtion" is the source's, not mine. This is about an operator — `bpy.ops.cycles.denoise_animation(*, input_filepath='', output_filepath='')`, listed on `https://docs.blender.org/api/5.1/bpy.ops.cycles.html`, read 2026-07-30 — not about a scene or view-layer setting, so I record it but do not count it as an answer to the question asked.)

No 5.1 release note adds, removes or renames any scene or view-layer denoising setting, as far as my search of all twenty release-note files for "denois" found.

### Where I looked (question 19)

- Searched all of `Blender Sources/cycles_5.1/properties.py` for `denois`; read every hit with its surrounding declaration, plus the enumeration blocks at lines 265–338 that supply the choices. Confirmed which class each property belongs to by listing the `class` lines and the `register` blocks that attach them to `bpy.types.Scene` (line 1103) and `bpy.types.ViewLayer` (line 1547).
- Searched `Blender Sources/cycles_5.1/ui.py` for `denois` to find which panel each setting is drawn in; read the panel classes at lines 246, 320, 840, 963 and 994.
- Searched all of `manual_5.1` for `denois`; the files with hits were `render/cycles/gpu_rendering.txt`, `render/cycles/render_settings/index.txt`, `render/cycles/render_settings/performance.txt`, `render/cycles/render_settings/sampling.txt`, `render/cycles/optimizations/reducing_noise.txt`, `render/eevee/render_settings/raytracing.txt`, `render/eevee/render_settings/performance.txt`, `render/layers/passes.txt`, `compositing/index.txt`, `compositing/types/filter/index.txt`, `compositing/types/filter/denoise.txt`, `getting_started/about/history.txt`, and the index. I read the Cycles sampling page in full, the EEVEE ray-tracing page in full, both performance pages' Denoise Nodes sections, the passes page's Denoising Data entry, and the reducing-noise page's Denoising section.
- Read `manual_5.1/render/layers/filter.txt` and `manual_5.1/render/layers/override.txt` in full, to check what the manual says the view layer carries.
- Ruled out `compositing/types/filter/denoise.txt` and `compositing/types/filter/index.txt`: they describe the Denoise node, which is a node in a node tree, not a setting the scene or a view layer carries. The three scene-level settings that steer that node (Group 5) I did include, because those do sit on the scene.
- Ruled out `getting_started/about/history.txt`: it is a version history, not a description of settings.
- Ruled out `render/cycles/gpu_rendering.txt` as an answer in itself: it says which hardware supports accelerated denoising, not what any setting does. I mention it only under the operating-system line below.
- Searched all of `api_5.1` for `denois`, and read the index entries in `genindex-D.txt` and `genindex-U.txt` to enumerate every documented attribute whose name contains "denoise": `denoise_bilateral`, `denoise_spatial`, `denoise_temporal` (all `RaytraceEEVEE`), `use_denoise` (`RaytraceEEVEE`), plus `denoise_animation()` in `bpy.ops.cycles`. Also read `bpy.types.RenderSettings.txt`, `bpy.types.SceneEEVEE.txt`, `bpy.types.RaytraceEEVEE.txt`, `bpy.types.ViewLayerEEVEE.txt`, `bpy.types.ViewLayer.txt`.
- Searched all of `api_5.1` and `manual_5.1` for `noise reduction` and `noise-reduction`; ruled out the geometry-nodes and keying hits as unrelated.
- Confirmed there is no `bpy.types.Cycles*` page in `api_5.1` (only `bpy.ops.cycles.txt` and the unrelated `bpy.types.FModifierCycles.txt`), which is why the Cycles settings above are cited from Cycles' own files rather than from the API documentation.
- Searched all of `release_notes_5.1` for `denois`.

### Not covered

- What each setting does to the finished picture. I report what the sources say the settings are for; I did not render anything and cannot say what any of them produces.
- Which of these settings a render actually honours, and in what order, when scene-level and view-layer-level values disagree. Cycles' interface file sets the view-layer toggle's row from the scene-level setting, from which greying out is inferred above; either way that is an interface behaviour, and I did not find a statement about what the render itself does.
- The per-view-layer Denoising toggle is not described anywhere in the 5.1 manual that I found. My account of it comes from the property's own description text and from where it is drawn, nothing more.
- Whether `denoiser` and `preview_denoiser` offer the same choices on every machine. Cycles' file builds those lists at run time and the entries depend on how Blender was built and what hardware is configured, so the list is not fixed. I did not determine what any particular machine offers.
- Denoising settings that might live somewhere other than the scene or a view layer — on an object, a material, or the world. I did not search for those, because the question asks about the scene and the view layer.
- The Denoise node's own settings. Out of scope by the same reasoning.

### Outside the interface

All of these are properties on the scene or the view layer, reachable and settable through Blender's Python interface with no window involved — Group 1 through Group 3 as `scene.cycles.…` and `view_layer.cycles.…` (the attachment points are quoted above from `properties.py`), Group 4 as the EEVEE settings on the scene, Group 5 on the scene's render settings. Nothing in the sources ties any of them to a person clicking. Two interface-only behaviours I did find, and they are behaviours of the window rather than of the settings: the view-layer Denoising toggle is inferred above to be greyed out when the scene-level one is off (`ui.py` line 991, `sub.active = scene.cycles.use_denoising`), and the Prefilter and Quality settings are described as "Visible only when using OpenImageDenoise" (manual, Cycles sampling page). Greying out and hiding do not exist outside the window; whether the underlying value still has an effect there, the sources do not say.

### Operating system

The settings themselves are declared in one shared file with no operating-system condition, and the API documentation is not split by system, so the settings exist everywhere. What is stated to differ by machine is which denoiser is available, and that is about hardware rather than operating system — with one exception where the manual names a platform. Exact words from `https://docs.blender.org/manual/en/5.1/render/cycles/gpu_rendering.html` (manual, read 2026-07-30):

> "GPU acceleration for OpenImageDenoise is available for compute capability 7.0 and higher"

> "GPU accelerated denoising is available on discrete Radeon RX 6000 and newer."

> "GPU accelerated denoising is available on all supported GPUs."

> "Hardware acceleration for ray-tracing and denoising is supported."

> "GPU accelerated ray-tracing and denoising is available on Apple Silicon."

Those five lines are quoted without whatever headings they sit under on that page, so which graphics hardware each one applies to cannot be told from the quote alone.

The OptiX choice is stated to be "only available on NVIDIA GPUs when configured in the system tab in the user preferences" (`Blender Sources/cycles_5.1/properties.py`, line 276) — again hardware, not operating system. Beyond the Apple Silicon line, the sources do not say that any denoising setting behaves differently from one operating system to another.

---


---

## Hand-test results — question 19 (2026-08-30)

**On what computers.** Two, and their results are kept apart.

- **The Creator's own MacBook Pro**, M1 Pro chip, macOS 26.6.2 (build 25G83), Apple's own graphics and no second card. Blender 5.1.2 at `/Applications/Blender 5.1.2.app/Contents/MacOS/Blender`, build hash `ec6e62d40fa9`.
- **A Linux machine rented for these tests**: Vast.ai instance 49253900, machine 27187, in Poland — a container rather than a whole machine (`systemd-detect-virt` answers `docker`, process 1 is `/usr/bin/dash`, no service manager), Ubuntu 22.04.4 LTS on kernel 6.8.0-48-generic, an AMD Ryzen 5 5600X with 5.76 cores allotted against 12 visible threads, a memory ceiling of about 21.2 GiB against the 31 GB the ordinary reading reports, a 45 GB overlay root, and **one NVIDIA GeForce RTX 3060 with 12288 MiB on driver 550.120**. It had **no display server as rented**; one was installed for the panel test and it draws in software (Xvfb, OpenGL renderer `llvmpipe`). Blender 5.1.2 at `/opt/blender/5.1/blender`, **build hash `ec6e62d40fa9` — the same build as the Mac's**, checked before any test was run.

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

**The picture came back**, in 5.1.2 on the Creator's Mac:

```
PIXELCASE both on   vs both off  : PIXELS max_difference 0.7058823704719543 channels_differing 6307 of 12288
PIXELCASE scene on, layer off vs both off : PIXELS max_difference 0.0 channels_differing 0 of 12288
PIXELCASE scene off, layer on vs both off : PIXELS max_difference 0.0 channels_differing 0 of 12288
PIXELCASE both on   vs scene on, layer off : PIXELS max_difference 0.7058823704719543 channels_differing 6307 of 12288
PIXELCASE both off  vs both off (control)  : PIXELS max_difference 0.0 channels_differing 0 of 12288
```

The rented Linux machine gave the same verdict, with slightly different figures because the two processors trace slightly different noise; **the three zeros and the control zero were identical on both**.

**The passes came back**, identically on both machines:

```
E7P scene=off layer=off store=off denoising-related=[]
E7P scene=on  layer=on  store=off denoising-related=['Noisy Image']
E7P scene=on  layer=off store=off denoising-related=[]
E7P scene=off layer=on  store=off denoising-related=[]
E7P scene=off layer=off store=on denoising-related=['Denoising Albedo', 'Denoising Normal', 'Denoising Depth']
E7P scene=on  layer=on  store=on denoising-related=['Noisy Image', 'Denoising Albedo', 'Denoising Normal', 'Denoising Depth']
```

**What that settles.** **Cycles denoises a view layer only when the scene's switch and the view layer's are both on** — observed, not inferred. Either switch alone leaves the picture identical, to the last channel, to both switches off. **The "Noisy Image" pass appears under exactly the same condition.** And **Store Denoising Passes is not gated by either switch**: its feature passes appear whenever it is on, whatever the two switches say.


*(Added 2026-08-30, and it is a repeat of the pass half above rather than a new question.
**Observed.** **What was done:** `e7passes.py`, unchanged — the same six combinations of the
scene's switch, the view layer's switch and Store Denoising Passes, read through a Render Layers
node with no file written. **To what:** Blender 5.1.2, build hash `ec6e62d40fa9`, run with
`-b --factory-startup --python e7passes.py` and a throwaway settings folder, **three times over**.
**On what computer:** the Creator's own Mac — a MacBook Pro with an M1 Pro chip, macOS 26.6.2 (build 25G83), Apple's own graphics and no NVIDIA card. It was run because the hand-test list's summary of this test said the
block came back the same in all four versions, while the four versions' own findings documents did
not agree with each other about the order of the feature passes.*

***A negative result would have looked like*** *a different order from one run to the next, or an
order different from the one this document already records — either would have said this block
could not be relied on.*

***What came back.*** *All three runs gave the same output, character for character:*

```
E7P scene=off layer=off store=off denoising-related=[]  total_sockets=2
E7P scene=on  layer=on  store=off denoising-related=['Noisy Image']  total_sockets=3
E7P scene=on  layer=off store=off denoising-related=[]  total_sockets=2
E7P scene=off layer=on  store=off denoising-related=[]  total_sockets=2
E7P scene=off layer=off store=on denoising-related=['Denoising Albedo', 'Denoising Normal', 'Denoising Depth']  total_sockets=5
E7P scene=on  layer=on  store=on denoising-related=['Noisy Image', 'Denoising Albedo', 'Denoising Normal', 'Denoising Depth']  total_sockets=6
E7P blender 5.1.2
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

**Came back**, in 5.1.2:

```
E8BCASE optix_acc_high vs optix_acc_high_repeat (control) : PIXELS max_difference 0.0 channels_differing 0 of 27648
E8BCASE optix_none_fast vs optix_acc_high                : PIXELS max_difference 0.0 channels_differing 0 of 27648
E8BCASE oidn_none_fast  vs oidn_acc_high                 : PIXELS max_difference 0.3137255012989044 channels_differing 10641 of 27648
```

**What that settles.** Under OptiX those two settings change **not one channel of 27 648**; under OpenImageDenoise the same change moves about ten and a half thousand. The control shows two identical runs come back byte for byte the same, so a difference would have been seen. **So the two settings are not merely hidden under another denoiser — they have no effect at all.**

### Whether OptiX could be used at all on that machine, with the path tracing on the card too

**Done.** On the rented Linux machine only, on 2026-08-30, while the entry above was being set up. That entry deliberately keeps the path tracing on the processor and puts only the denoising on the card. Before it was settled that way, the same frame was rendered with the **path tracing on the card as well**, under OptiX, in all four versions on that one machine — Vast.ai instance 49253900, machine 27187, in Poland: a container rather than a whole machine, Ubuntu 22.04.4 LTS on kernel 6.8.0-48-generic, an AMD Ryzen 5 5600X with 5.76 cores allotted, and **one NVIDIA GeForce RTX 3060 with 12288 MiB on driver 550.120**. Blender was driven from a terminal with no window, with `--factory-startup` and a throwaway settings folder, and all four build hashes were checked against the Creator's Mac before anything was run. **This was met beside another test rather than asked as a question**, and it was run once.

**Came back.** **Blender 5.1.2 could use OptiX on that machine**, as could 4.5.12 LTS and 5.0.1. **Blender 5.2.1 LTS, on the same machine, the same card and the same driver, could not**, and wrote no image. These are the machine's own words in 5.2.1, and they are the only output this test captured:

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

**Came back**, in 5.1.2 on the Creator's Mac:

```
E9C node_quality='NO-QUALITY-ON-NODE'
E9CASE cy_high vs cy_high_repeat (control) : PIXELS max_difference 0.0 channels_differing 0 of 27648
E9CASE cy_fast vs cy_high                  : PIXELS max_difference 0.08627453446388245 channels_differing 9136 of 27648
E9CASE ee_fast vs ee_high                  : PIXELS max_difference 0.027450978755950928 channels_differing 3528 of 27648
```

The rented machine gave the same verdict.

**What that settles.** That setting changes a final render **under both engines** — engine-independent by test rather than by inference. **And one difference between versions, tested in all four.** The Denoise node here carries **no `quality` of its own at all**, where 4.5's does and starts at `FOLLOW_SCENE`. So from 5.0 on the node has no choice but to follow the scene, and the manual's condition "if the nodes' quality option is set to Follow Scene" describes something that exists only in 4.5.


*(Added 2026-08-30, and it is a repeat of the run above rather than a new question. **Observed.**
**What was done:** the same test, unchanged — the scene built by this version with `make_scene.py`,
a compositor of Render Layers → Denoise → output, Cycles' own denoising off, 96 by 72, 8 samples,
adaptive sampling off, then the five cases and the three comparisons of `compare_pixels.py`.
**To what:** Blender 5.1.2, build hash `ec6e62d40fa9`, started with `-b --factory-startup` and a throwaway
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
round 2  E9CASE cy_fast vs cy_high : PIXELS max_difference 0.08627453446388245 channels_differing 9136 of 27648
round 3  E9CASE cy_fast vs cy_high : PIXELS max_difference 0.08627453446388245 channels_differing 9135 of 27648
round 4  E9CASE cy_fast vs cy_high : PIXELS max_difference 0.08627453446388245 channels_differing 9136 of 27648
round 5  E9CASE cy_fast vs cy_high : PIXELS max_difference 0.08627453446388245 channels_differing 9135 of 27648
round 6  E9CASE cy_fast vs cy_high : PIXELS max_difference 0.08627453446388245 channels_differing 9136 of 27648
round 7  E9CASE cy_fast vs cy_high : PIXELS max_difference 0.08627453446388245 channels_differing 9136 of 27648
round 8  E9CASE cy_fast vs cy_high : PIXELS max_difference 0.08627453446388245 channels_differing 9136 of 27648
round 9  E9CASE cy_fast vs cy_high : PIXELS max_difference 0.08627453446388245 channels_differing 9136 of 27648
round 10 E9CASE cy_fast vs cy_high : PIXELS max_difference 0.08627453446388245 channels_differing 9136 of 27648
round 11 E9CASE cy_fast vs cy_high : PIXELS max_difference 0.08627453446388245 channels_differing 9136 of 27648
round 12 E9CASE cy_fast vs cy_high : PIXELS max_difference 0.08627453446388245 channels_differing 9135 of 27648
```

*In every one of the twelve rounds the other two lines were identical:*

```
E9CASE cy_high vs cy_high_repeat : PIXELS max_difference 0.0 channels_differing 0 of 27648
E9CASE ee_fast vs ee_high : PIXELS max_difference 0.08627453446388245 channels_differing 3528 of 27648
```

*and the maximum difference on the Cycles line, `0.08627453446388245`, never moved.*

*Two different figures came back — 9135 and 9136. **The count is not reproducible in this version on this machine**, though it moved less than in 4.5 or 5.0. The figure recorded above, `9136`, came back in nine rounds of the twelve; three gave 9135. **Both stand and neither is preferred.** Everything else in the block above reproduced exactly, twelve times over.*

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

**Came back**, in 5.1.2 on the Mac:

```
00:00.462  bpy.rna          | WARNING current value '2' matches no enum in 'CyclesRenderSettings', '', 'denoiser'
READDEN2 blender=5.1.2 mode=as-saved denoiser_now='' use_denoising=True
00:00.628  render           | Saved: '5.1-as-saved0001.png'
E12CASE 5.1 as-saved vs the same file with denoising switched off : PIXELS max_difference 0.7372549176216125 channels_differing 13680 of 27648
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

## Things a person with Blender 5.1 open could settle by hand

*(Pruned on 2026-08-30. Everything this list held was run — see the hand-test results above — except the items below, which are genuinely still open. The tests, with their exact commands and their raw output, are at `Audits/Blender Research/Settings/Hand tests — Open Threads questions.md`.)*

1. **What denoisers a machine with an AMD or an Intel graphics card offers.** The two machines used were a Mac with Apple's own graphics and a Linux machine with an NVIDIA card. Nothing here says what a third kind of card offers, and the manual's claim that AMD denoising starts at a different card generation on Linux and on Windows is untested.
2. **What happens when a sample count is driven, or animated, out of its range.** Every test above set the value directly. A driver and a keyframe are different routes and neither was tried.
3. **What Blender does on loading a file whose stored sample count is above the ceiling.** The correction was observed at the moment of assignment; the file-loading path is a different one, and no file old enough to trigger the squared-samples conversion exists here.
4. **What a number field does when a value outside the range is dragged rather than typed.** The dragging bounds were read off the running program, but no test dragged a control.
5. **Everything on Windows.** Eleven entries are written out ready to run in the hand-test list named above, for the separate Windows batch. They are not unrunnable — this project has a route to a Windows computer, and the account allows one machine at a time.

---

2026-08-29 — Claude Opus 5 (Claude Code) — Settings, Blender 5.1
2026-08-30 — Claude Opus 5 (Claude Code) — hand-test results for the Open Threads questions (18 and 19) written in, the two standing lines on each answer brought up to date, and the researcher's own list pruned to what is still open
2026-08-30 — Claude Opus 5 (Claude Code) — hand-test entries 13 and 14 written in: the compositor denoise-quality comparison run twelve times over in one sitting, and the order of the denoising feature passes read three times over, both on the Creator's Mac and both recorded as observed

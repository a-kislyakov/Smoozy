# Settings — Open Threads questions (Blender 4.5)

Sources consulted, all on **2026-07-30** (the snapshot date given for every source), read on 2026-08-29:

- The Blender 4.5 manual, as plain text under `Blender Sources/manual_4.5`, plus one look through its `_images` folder.
- The Blender 4.5 Python API reference, as plain text under `Blender Sources/api_4.5`.
- The Blender 4.5 release notes, as plain files under `Blender Sources/release_notes_4.5`.
- Cycles' own files under `Blender Sources/cycles_4.5`.

I read only the question sheet and files inside Blender Sources/. I did not open any other part of this project.

What this document is. It is the first pass at questions 18 and 19 for Blender 4.5; no findings document for these questions existed before it. The research brief was built from the standard template with its second-pass lines filled in, so it asked for a wider search and for a written record of where the researcher looked, and every version's brief carried that same wording. That is why the record of looking below is fuller than a first pass usually carries. Every answer below was written cold from the sources.

---

## 18. What values will a render's sample count be set to, and what does a render do when it is given a value outside those?

**LABEL: observed** for the accepted ranges of the Cycles sample counts; **documented** for the EEVEE, view-layer, Grease Pencil and Workbench ones; **inferred** for what happens to a Cycles sample count given a value outside its range, and for reading the manual's "soft limit" and "hard limit" as the `soft_min` and `min`/`max` written in Cycles' file; **not in the sources** for what a render *reports* when that happens, and for what crossing the range does to the counts that are not Cycles'.

### The short answer

There is no single "sample count". Each render engine carries its own, and there are a few more sample counts beside the main one. Most are whole numbers with their own accepted range; Workbench's is not a number at all but a fixed list of choices. For Cycles' sample counts, and only those, the sources support the reading that a value outside the range is pulled back to the nearest end of the range with nothing said about it — that reading is inferred, and it is set out in full below. For the EEVEE, view-layer, Grease Pencil and Workbench counts the sources give the accepted values and do not say what crossing them does.

### What each sample count will accept

**Cycles, final render — "Render (Max) Samples"**

Cycles' own file `properties.py`, in the class `CyclesRenderSettings` (which begins at line 376), lines 482 to 487:

```
    samples: IntProperty(
        name="Samples",
        description="Number of samples to render for each pixel",
        min=1, max=(1 << 24),
        default=4096,
    )
```

So: a whole number from 1 to 16777216 (that is what `1 << 24` comes to), starting at 4096. Zero is not accepted.

**Cycles, viewport — "Viewport (Max) Samples"**

Same file, lines 488 to 495:

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

A whole number from 0 to 16777216, starting at 1024. Zero is accepted here and means unlimited. The manual says the same thing in words:

> "Setting this value to zero enables indefinite sampling."

(Blender manual, https://docs.blender.org/manual/en/4.5/render/cycles/render_settings/sampling.html, read 2026-07-30.)

Note the `soft_min=1` alongside `min=0`. The two are different limits; see "What happens outside the range" below.

**Cycles, the sample subset**

Same file, lines 497 to 515:

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

Offset: 0 to 16777216. Length: 1 to 16777216.

Nothing in either the file or the manual makes one of these depend on the other — there is no stated rule that offset plus length must not exceed the max samples. The manual only advises what to set:

> "While this overrides (Max) Samples in terms of the samples that will get rendered, it's still important to set (Max) Samples to the total number of samples that will be rendered across all subsets – otherwise, the subsets will have incompatible noise and combining them will give a worse result."

(Same manual page and date.)

**Cycles, adaptive sampling's minimum sample count**

Same file, lines 583 to 589:

```
    adaptive_min_samples: IntProperty(
        name="Adaptive Min Samples",
        description="Minimum AA samples for adaptive sampling, to discover noisy features before stopping sampling. Zero for automatic setting based on noise threshold",
        min=0,
        max=4096,
        default=0,
    )
```

0 to 4096 — a much narrower range than the main sample count. The viewport twin, `preview_adaptive_min_samples` (lines 605 to 611), has the same 0 to 4096 range.

**Cycles, path guiding training samples**

Same file, lines 710 to 719:

```
    guiding_training_samples: IntProperty(
        name="Training Samples",
        description="The maximum number of samples used for training path guiding. "
        "Higher samples lead to more accurate guiding, however may also unnecessarily slow "
        "down rendering once guiding is accurate enough. "
        "A value of 0 will continue training until the last sample",
        min=0,
        soft_min=1,
        default=128,
    )
```

Minimum 0. No maximum is given at all here, unlike the settings above.

**Per view layer, overriding the scene's count**

The API reference, under `ViewLayer`:

> "samples
>
> Override number of render samples for this view layer, 0 will use the scene setting
>
> Type:
>
> int in [0, inf], default 0"

(Blender Python API, https://docs.blender.org/api/4.5/bpy.types.ViewLayer.html, read 2026-07-30.)

"inf" is how this reference writes the upper end of this range. Whether that means no ceiling was set, or a ceiling the reference does not print, the page does not say. *(Narrowed by the citation check of 2026-08-30: the earlier wording claimed the notation meant no upper limit was set, which the page does not carry.)*

How that override is treated is itself a setting. Cycles' `properties.py`, lines 68 to 72:

```
enum_use_layer_samples = (
    ('USE', "Use", "Per render layer number of samples override scene samples"),
    ('BOUNDED', "Bounded", "Bound per render layer number of samples by global samples"),
    ('IGNORE', "Ignore", "Ignore per render layer number of samples"),
)
```

and the manual, same Sampling page:

> "Bounded:
>
> Allow the overrides as long as they don't exceed the scene-level sample count."

That is the one place in the sources where a sample count is described as being cut back to another value rather than to its own limit — and it happens only when Layer Samples is set to Bounded.

**EEVEE**

The API reference, under `SceneEEVEE`:

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

(Blender Python API, https://docs.blender.org/api/4.5/bpy.types.SceneEEVEE.html, read 2026-07-30.)

So EEVEE's final-render count accepts a whole number from 1 upward with no stated ceiling, and starts at 64; its viewport count accepts 0, meaning unlimited, and starts at 16. The manual agrees in words: "The number of samples to use in the final render." and, for the viewport, "When setting this to zero the viewport will be resampled continuously." (https://docs.blender.org/manual/en/4.5/render/eevee/render_settings/sampling.html, read 2026-07-30.)

**Grease Pencil's anti-aliasing sample count**

> "aa_samples
>
> Number of supersampling anti-aliasing samples per pixel for final render
>
> Type:
>
> int in [1, inf], default 8"

(Blender Python API, https://docs.blender.org/api/4.5/bpy.types.SceneGpencil.html, read 2026-07-30.)

The 4.5 release notes name this setting:

> "The SSAA sample count can be controlled with the `Grease Pencil` > `Render` > `SSAA Samples` render setting."

(Blender 4.5 release notes, https://developer.blender.org/docs/release_notes/4.5/grease_pencil/, read 2026-07-30.)

The quoted words name the setting. That it is *new* in 4.5, rather than merely mentioned in that version's notes, is a step the words do not take — inferred, not documented.

**Workbench**

Workbench's sample count is not a free number at all. It is a fixed list:

> "Type:
>
> enum in ['OFF', 'FXAA', '5', '8', '11', '16', '32'], default '8'"

(Blender Python API, `SceneDisplay.render_aa`, https://docs.blender.org/api/4.5/bpy.types.SceneDisplay.html, read 2026-07-30.) The listed choices are described as "5 Samples", "8 Samples", "11 Samples", "16 Samples", "32 Samples", plus no anti-aliasing and a single-pass method. Nothing between those numbers can be asked for.

### What happens when a value outside the range is given

Two different things, depending on how the value arrives.

**Typed into a field in Blender's own window.** The manual, on number fields:

> "Value Limits
>
> Most numerical values are restricted by "soft limit" and "hard limit" value ranges.
> Changing values by dragging with the mouse is restricted to the "soft limit" value range.
> Input via keyboard will allow the use of wider value ranges, but never wider than the "hard limit"."

(Blender manual, https://docs.blender.org/manual/en/4.5/interface/controls/buttons/fields.html, read 2026-07-30.)

Reading the manual's "soft limit" and "hard limit" as the `soft_min` and `min`/`max` written in Cycles' file is a step the manual does not take in so many words, so what follows is **inferred** rather than documented: for Cycles' viewport sample count, dragging would not take it below 1, while typing 0 would be accepted — and 0 is the value that means unlimited.

**Assigned as a value, rather than typed.** The API reference, describing how an integer setting is defined:

> "min (int) – Hard minimum, trying to assign a value below will silently assign this minimum instead."

> "max (int) – Hard maximum, trying to assign a value above will silently assign this maximum instead."

(Blender Python API, `bpy.props.IntProperty`, https://docs.blender.org/api/4.5/bpy.props.html, read 2026-07-30.)

**The inference, written out.** Cycles' sample counts are defined with exactly this `IntProperty`, with exactly these `min` and `max` arguments — that is what the quoted lines from `properties.py` above are. The API reference states what those arguments do. Putting the two together: assigning 0 to Cycles' final-render sample count silently gives 1, and assigning 100000000 silently gives 16777216. No error is raised and no message is produced — "silently" is the reference's own word. The step a reader has to supply is only that the sentence about `min` and `max` applies to the `min` and `max` actually written in the Cycles file. That step is small, but it is a step, so this is inferred rather than documented.

**What the render then does.** The sources describe no separate check at render time, no warning, and no refusal to start — that is silence, and where it was looked for is recorded below. That the setting therefore already holds a value inside its range when a render starts, so the out-of-range value never reaches the render, follows from the clamping inference above and covers only the Cycles counts that inference covers. It is **inferred**, not read: Cycles' own files say what a setting is and what it accepts, never what a render does with it.

**Not covered.**

- What is *reported*. Nothing in the sources gives a message, a warning, a console line or a return value for an out-of-range sample count. "silently" says there is none for the assignment case, but that is a statement about the assignment, not about the render.
- The clamping sentence I quoted is written for settings *defined* through `bpy.props`. Cycles' sample counts are defined that way (observed). EEVEE's, the view layer's, the Grease Pencil one and Workbench's are not defined in any file I was given — they are part of Blender itself, and the reference lists only their ranges, not what happens when the range is crossed. The sources do not state the rule for those. I did not find it stated anywhere else; see the record of looking below.
- What happens when a sample count is given something that is not a whole number at all — a decimal, a negative, text — is not stated in the sources. No search for this is recorded below, so the silence is not established and the question has been sent back to be researched again.
- Whether the *actual* number of samples rendered equals the setting is a different matter and is not settled here: the same Cycles page describes a Time Limit, adaptive sampling, and the sample subset, each of which can end sampling before the count is reached. This answer is about which values the setting will accept.
- Whether "inf" in the API reference means genuinely no ceiling, or a ceiling the reference does not print, is not stated. The reference's own introduction pages, where its way of writing a range would be explained, were not searched for this, so the question has been sent back to be researched again.

**Outside the interface.** The clamping-on-assignment sentence is about assigning a value, not about clicking, so it holds when Blender is driven from outside its own window. The Value Limits sentence about soft limits and keyboard input is explicitly about number fields in Blender's window and has no equivalent outside it — there is no "dragging" and no soft limit outside, so the wider hard range is the only one that applies. One further point: I searched the manual's command-line arguments page for "sample" and found no match at all. That is one word on one page, so it shows that page does not use the word — not that no launch-time instruction of any name reaches a sample count.

**Operating system.** The ranges above are written as plain numbers in Cycles' own file and in the API reference, with no condition on operating system anywhere near them. The sources say nothing that would make a sample count's accepted values differ between operating systems. That is silence, not a statement that they are the same everywhere.

### Where I looked, for question 18

Searched (all of `Blender Sources/manual_4.5`, all of `Blender Sources/api_4.5`, all of `Blender Sources/release_notes_4.5`, all of `Blender Sources/cycles_4.5`):

- `samples` across the Cycles files — found the definitions quoted above in `properties.py`; also checked `engine.py`, `operators.py`, `presets.py`, `version_update.py` and `__init__.py` for any handling of a sample count. `presets.py` only lists the setting names. `version_update.py` only assigns old-file defaults (lines 150 to 244). Neither adds a range check.
- `taa_render_samples`, `taa_samples` across the API reference — only `bpy.types.SceneEEVEE.txt` and the index files.
- `ssaa`, `aa_samples`, `antialias` across the API reference and the manual — found `SceneGpencil`, `SceneDisplay.render_aa`, and the three Grease Pencil render-settings manual pages.
- `samples` in the view-layer manual pages — found the Cycles override entry.
- `clamp`, `out of range`, `hard limit`, `soft limit`, `soft_min`, `silently` across the API reference — found `bpy.props`, `bpy.types.IntProperty` and `bpy.types.FloatProperty`. `bpy.types.IntProperty` gives only the names of the four limits ("hard_max", "hard_min", "soft_max", "soft_min", each described as "value used by buttons") and does not say what crossing them does.
- `TypeError`, `ValueError` across all the `info_*` pages of the API reference and `bpy.types.bpy_struct` — the only matches are about registering classes and about names, none about a number outside its range. Ruled out.
- `clamp`, `hard limit`, `soft limit`, `out of range` across the whole manual — the only relevant hit is the Value Limits paragraph quoted above. The others are about mesh weights, curve handles, firefly clamping and light intensity, all unrelated. Ruled out.
- `sample` in the command-line arguments manual page — no match.
- The release notes: `sample` and `denois` across all nineteen files. The only sample-count change in 4.5 is the Grease Pencil SSAA count quoted above.
- The manual's `_images` folder for a screenshot of a Cycles sampling panel that might show a slider's limits — searched for image names containing "sampling" or "denois". The only matches are `render_layers_denoising_example1.jpg`, `render_layers_denoising_example2.jpg`, `compositing_types_filter_denoise_example.jpg`, `node-types_CompositorNodeDenoise.webp` and `render_materials_legacy-textures_types_image-movie_image-sampling-panel.png`. None is a picture of the Render Sampling panel, so no picture settles a limit. The Cycles Sampling manual page carries no `[image:` line at all.

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

**LABEL: observed** for the Cycles settings and their defaults (read from Cycles' own files); **documented** for what most of them do (the manual and the API reference describe them) and for the engine-independent and EEVEE ones; **inferred** for the three readings marked as such below — how the scene's and the view layer's denoising switches combine, and what the OptiX list does when there is no context.

There are five groups. Three belong to Cycles — the scene's settings for the final render, the scene's settings for the viewport, and the view layer's — one belongs to the scene whatever the engine, and one belongs to EEVEE.

### Group 1 — Cycles, on the scene, for the final render

These sit on the scene at `cycles`, in the property group `CyclesRenderSettings`. All quotes in this group are from `Blender Sources/cycles_4.5/properties.py`, read 2026-07-30, at the line numbers given.

**Use Denoising** (lines 403 to 408)

```
    use_denoising: BoolProperty(
        name="Use Denoising",
        description="Denoise the rendered image",
        default=True,
        update=update_render_passes,
    )
```

On or off. On by default. The manual calls it "Denoise (Render)" and says: "Whether to perform denoising for the final rendered image."

**Denoiser** (lines 409 to 416)

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

Which denoiser to use. The choices are not a fixed list — they are worked out when the setting is drawn, by `enum_denoiser` (lines 297 to 301), which stitches together two other lists:

```
def enum_optix_denoiser(self, context):
    if not context or bool(context.preferences.addons[__package__].preferences.get_devices_for_type('OPTIX')):
        return [('OPTIX', "OptiX", n_(
            "Use the OptiX AI denoiser with GPU acceleration, only available on NVIDIA GPUs when configured in the system tab in the user preferences"), 2)]
    return []
```

(lines 271 to 275) and

```
def enum_openimagedenoise_denoiser(self, context):
    import _cycles
    if _cycles.with_openimagedenoise:
        return [('OPENIMAGEDENOISE', "OpenImageDenoise",
                 n_("Use Intel OpenImageDenoise AI denoiser"), 4)]
    return []
```

(lines 263 to 268). So for the final render there are at most two choices, OptiX and OpenImageDenoise. **Inferred**, not observed: that either can be absent — OptiX when no OptiX device is configured, OpenImageDenoise when the Blender build does not have it — asks a reader to supply what those two `if` tests do when they run, which the file itself cannot carry. What the quoted lines observe is only that each entry sits inside a condition. *(Label added by the citation check of 2026-08-30.)* The default is the number 4, which is OpenImageDenoise's number, and the comment in the file says why a number is used rather than a name.

The manual describes both in words:

> "OpenImageDenoise:
>
> Uses Intel's Open Image Denoise,
> an AI denoiser. Typically provides the highest quality."

> "OptiX:
>
> Uses NVIDIA's OptiX AI denoiser.
> Supports GPU acceleration on some older NVIDIA GPUs where OpenImageDenoise does not."

(Blender manual, https://docs.blender.org/manual/en/4.5/render/cycles/render_settings/sampling.html, read 2026-07-30.)

**Denoising Input Passes** (lines 429 to 434)

```
    denoising_input_passes: EnumProperty(
        name="Denoising Input Passes",
        description="Passes used by the denoiser to distinguish noise from shader and geometry detail",
        items=enum_denoising_input_passes,
        default='RGB_ALBEDO_NORMAL',
    )
```

with the choices at lines 304 to 308:

```
enum_denoising_input_passes = (
    ('RGB', "None", "Don't use utility passes for denoising", 1),
    ('RGB_ALBEDO', "Albedo", "Use albedo pass for denoising", 2),
    ('RGB_ALBEDO_NORMAL', "Albedo and Normal", "Use albedo and normal passes for denoising", 3),
)
```

Which extra information the denoiser gets to work from. Albedo and Normal by default. The manual adds advice: "Generally, the more passes the denoiser has access to, the better the result. It is recommended to use at least Albedo as None can blur out details, especially at lower sample counts."

**Denoising Prefilter** (lines 417 to 422)

```
    denoising_prefilter: EnumProperty(
        name="Denoising Prefilter",
        description="Prefilter noisy guiding (albedo and normal) passes to improve denoising quality when using OpenImageDenoise",
        items=enum_denoising_prefilter,
        default='ACCURATE',
    )
```

with the choices at lines 310 to 323:

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
```

Whether the extra passes are cleaned up before they are used. Accurate by default. The manual says it is **shown** only under one denoiser — "Visible only when using OpenImageDenoise." — and the setting's own description names OpenImageDenoise. *(Narrowed by the citation check of 2026-08-30: the earlier wording said it "applies to OpenImageDenoise only", which reaches past the quote. Being hidden and having no effect are two claims, and the manual's words carry only the first.)*

**Denoising Quality** (lines 423 to 428)

```
    denoising_quality: EnumProperty(
        name="Denoising Quality",
        description="Overall denoising quality when using OpenImageDenoise",
        items=enum_denoising_quality,
        default='HIGH',
    )
```

with the choices at lines 325 to 334: `HIGH` ("High quality"), `BALANCED` ("Balanced between performance and quality"), `FAST` ("High performance"). High by default. Again OpenImageDenoise only.

**Denoise on GPU** (lines 435 to 439)

```
    denoising_use_gpu: BoolProperty(
        name="Denoise on GPU",
        description="Perform denoising on GPU devices configured in the system tab in the user preferences. This is significantly faster than on CPU, but requires additional GPU memory. When large scenes need more GPU memory, this option can be disabled",
        default=False,
    )
```

Where the denoising work happens. **Off by default** for the final render. Note this is separate from where the *render* happens.

The manual's version: "Perform denoising on the GPU. This is significantly faster than on CPU, but requires additional GPU memory. When large scenes need more GPU memory, this option can be disabled."

### Group 2 — Cycles, on the scene, for the viewport

The same scene carries a second, complete set for the 3D viewport in Rendered shading mode. They are separate settings with their own values, not the same settings reused. From the same file:

| Setting | Lines | What it does | Default |
|---|---|---|---|
| `use_preview_denoising` | 441 to 445 | "Denoise the image in the 3D viewport" | **False** |
| `preview_denoiser` | 446 to 451 | "Denoise the image after each preview update with the selected denoiser" | 0, which is `AUTO` |
| `preview_denoising_prefilter` | 452 to 457 | same choices as above | `FAST` |
| `preview_denoising_quality` | 458 to 463 | same choices as above | `BALANCED` |
| `preview_denoising_input_passes` | 464 to 469 | same choices as above | `RGB_ALBEDO` |
| `preview_denoising_start_sample` | 470 to 475 | "Sample to start denoising the preview at", min 0, max 16777216 | 1 |
| `preview_denoising_use_gpu` | 476 to 480 | same wording as `denoising_use_gpu` | **True** |

Two differences worth naming, both observed in the file. The viewport denoiser has an extra choice the final-render one does not, `AUTO`, defined at lines 278 to 294:

```
        items = [
            ('AUTO',
             "Automatic",
             n_("Use GPU accelerated denoising if supported, for the best performance. "
                "Prefer OpenImageDenoise over OptiX"),
             0)]
    else:
        items = [('AUTO', "None", n_("Blender was compiled without a viewport denoiser"), 0)]
```

— and when neither denoiser is available at all, that one entry is renamed "None" with the description "Blender was compiled without a viewport denoiser". And the GPU switch is on by default for the viewport but off for the final render.

`preview_denoising_start_sample` has no equivalent in the final-render group; there is no setting to begin denoising partway through a final render.

### Group 3 — Cycles, on the view layer

These sit on the view layer at `cycles`, in the property group `CyclesRenderLayerSettings`, which begins at line 1494 of the same file. There are exactly two.

**Use Denoising** (lines 1522 to 1527)

```
    use_denoising: BoolProperty(
        name="Use Denoising",
        description="Denoise the rendered image",
        default=True,
        update=update_render_passes,
    )
```

Per view layer, on or off, on by default.

How it combines with the scene's switch is visible in Cycles' own code. `Blender Sources/cycles_4.5/engine.py`, line 236, deciding which passes exist:

```
    if scene.cycles.use_denoising and crl.use_denoising:
        yield ("Noisy Image", "RGBA", 'COLOR')
```

(`crl` is the view layer's Cycles settings.) And `Blender Sources/cycles_4.5/ui.py`, lines 976 and 977, drawing the checkbox in the View Layer properties:

```
        sub.prop(view_layer.cycles, "use_denoising", text="Denoising")
        sub.active = scene.cycles.use_denoising
```

**Inferred, and the step written out.** The quoted lines are what the files hold: one line puts the scene's switch and the view layer's switch together, and the other ties the checkbox's active state to the scene's switch. Reading those as *the pass is produced only when both switches are on* and *the checkbox is drawn dimmed when the scene's is off* asks a reader to supply what those lines do when they run, which Cycles' own files cannot carry. So both readings are **inferred**, and both are on the hand-test list.

**Store Denoising Passes** (lines 1528 to 1533)

```
    denoising_store_passes: BoolProperty(
        name="Store Denoising Passes",
        description="Store the denoising feature passes and the noisy image. The passes adapt to the denoiser selected for rendering",
        default=False,
        update=update_render_passes,
    )
```

Off by default. Its own description says it stores passes; nothing quoted says whether it also changes the rendered image. `engine.py` lines 240 to 243 name three:

```
    if crl.denoising_store_passes:
        yield ("Denoising Normal", "XYZ", 'VECTOR')
        yield ("Denoising Albedo", "RGB", 'COLOR')
        yield ("Denoising Depth", "Z", 'VALUE')
```

The manual describes it under its interface name, "Denoising Data":

> "Denoising Data Cycles
>
> Includes Denoising Albedo, Denoising Normal, and the combined image
> before denoising. Can be used with the Denoise Node
> as a replacement for automatic denoising."

(Blender manual, https://docs.blender.org/manual/en/4.5/render/layers/passes.html, read 2026-07-30.)

The Cycles Sampling manual page points the same way: "If you enable the Denoising Data render pass, you can alternatively denoise in the compositing step using the Denoise Node."

### Group 4 — on the scene, whatever the engine

Three settings sit on the scene's render settings and govern the Denoise node in the compositor rather than the engine's own denoising. All three from the API reference, `RenderSettings`, https://docs.blender.org/api/4.5/bpy.types.RenderSettings.html, read 2026-07-30:

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
> …
> enum in ['HIGH', 'BALANCED', 'FAST'], default 'HIGH'"

> "compositor_denoise_preview_quality
>
> The quality used by denoise nodes during viewport and interactive compositing if the nodes' quality option is set to Follow Scene
> …
> enum in ['HIGH', 'BALANCED', 'FAST'], default 'BALANCED'"

These three appear in both engines' Performance panels in the manual, worded identically — see https://docs.blender.org/manual/en/4.5/render/cycles/render_settings/performance.html and https://docs.blender.org/manual/en/4.5/render/eevee/render_settings/performance.html, both read 2026-07-30. The two **quality** settings apply only when a Denoise node's own quality option is set to Follow Scene; the reference says so in the words quoted above. The **device** setting carries no such condition in its quoted words — it says only that it is the device used to process the denoise nodes in the compositor. All three are described in terms of denoise nodes in the compositor. That all three therefore need such a node to have anything to act on is a step the words do not take, so it is **inferred**, not documented. *(Label added by the citation check of 2026-08-30.)*

4.5 changed one thing here. The release notes:

> "The Denoise node now supports GPU devices."

(Blender 4.5 release notes, https://developer.blender.org/docs/release_notes/4.5/compositor/, read 2026-07-30.)

And a fix landed in 4.5:

> "Denoiser does not handle highly illuminated scenes when run on GPU"

(Blender 4.5 release notes, https://developer.blender.org/docs/release_notes/4.5/bugfixes/, read 2026-07-30.)

### Group 5 — EEVEE, on the scene

EEVEE's denoising settings read differently from Cycles'. The quoted descriptions below speak of reprojecting, reusing neighbouring pixels' rays, and blurring the resolved result; none of them names a separate clean-up pass over a finished image. Calling them part of how EEVEE draws rather than a pass afterwards is a step the words do not take, so it is not claimed here.

On the scene's EEVEE settings directly:

> "use_taa_reprojection
>
> Denoise image using temporal reprojection (can leave some ghosting)
>
> Type:
>
> boolean, default True"

(Blender Python API, https://docs.blender.org/api/4.5/bpy.types.SceneEEVEE.html, read 2026-07-30.) The manual lists this under Viewport as "Temporal Reprojection": "Reduces noise while moving the viewport or during animation playback. Can leave some ghosting."

And four more inside the ray-tracing options, which the scene's EEVEE settings hold as `ray_tracing_options` ("EEVEE settings for tracing reflections"). From https://docs.blender.org/api/4.5/bpy.types.RaytraceEEVEE.html, read 2026-07-30:

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

The manual gives the same four under a "Denoising" heading on the Raytracing page:

> "Denoising can be enabled to reduce the amount of noise from the raw ray-traced output.
> This can help image stability but will also over-blur the final ray-traced output."

with "Spatial Reuse — Reuse the rays from neighbor pixels. Can introduce some light leaks across surfaces.", "Temporal Accumulation — Accumulate samples by re-projecting the last ray tracing results. This removes Fireflies but also introduces color bias. Useful for viewport temporal stability or making renders converge faster.", and "Bilateral Filter — Blur the resolved ray-traced output using a bilateral filter." (https://docs.blender.org/manual/en/4.5/render/eevee/render_settings/raytracing.html, read 2026-07-30.)

All four sit under the ray-tracing module, which the API reference shows is itself off by default: "use_raytracing — Enable the ray-tracing module — boolean, default False".

**Not covered.**

- **No EEVEE or Workbench denoising setting on a view layer.** I read the whole of `ViewLayerEEVEE` in the API reference; it carries only pass switches (`use_pass_bloom`, `use_pass_transparent`, `use_pass_volume_direct`). Nothing about denoising. *(Corrected by the citation check of 2026-08-30: the list previously named a fourth entry, `use_pass_volume_indirect`, which is not on 4.5's page. The conclusion is unchanged.)* Workbench has none either. So the view-layer half of this question is answered by Cycles alone.
- **What each setting actually does to the picture.** The sources describe what each setting is for, in one sentence each. They do not say what a render produces with one setting versus another, beyond the two before-and-after example pictures on the Reducing Noise manual page (`_images/render_layers_denoising_example1.jpg` and `_images/render_layers_denoising_example2.jpg`, captioned "Example render before denoising." and "Example render after denoising."). I looked at neither as evidence of any particular setting, because the captions do not say which settings were used.
- **The per-view-layer Denoising checkbox is not in the manual.** I searched the whole manual for "denois" and the view-layer pages return only the Denoising Data pass. So the description of the per-view-layer switch, and the rule that both it and the scene's must be on, comes from Cycles' own files rather than from documentation.
- **The exact ordering and interaction between groups** — for instance whether Cycles' own denoising runs before or after a compositor Denoise node, or what happens when both are on — is not stated in what was read. The manual only calls the node an "alternative". But the compositor's own Denoise node pages were set aside on scope rather than read, and they are where such a statement would most likely sit, so this silence is not established and the question has been sent back to be researched again.
- **Denoising is not the only thing that reduces noise.** Adaptive sampling, path guiding, the light tree and clamping all do too, and all sit on the scene. I have not listed them, because the question asks about denoising.

**Outside the interface.** Every setting here is a value stored on the scene or the view layer, reachable and settable without Blender's window being involved. Three cautions.

First, the whole viewport group (`use_preview_denoising` and its six companions) is described everywhere as applying to the 3D viewport in Rendered shading mode. A render started from outside Blender is a final render, so those seven have no bearing on it. `compositor_denoise_preview_quality` is likewise described as applying to "viewport and interactive compositing".

Second, the list of denoisers offered is worked out at the moment it is drawn, and depends on what the user's preferences say. That is a genuine difference outside the window, and the file shows an odd edge: `enum_optix_denoiser` begins `if not context or bool(...)`. What is read is that line and nothing more. Reading it as *when there is no context at all the OptiX entry is returned whatever devices are configured* asks a reader to supply how the line behaves when it runs, so that reading is **inferred**, not observed. What it would mean for a render driven from outside is a further step nobody has taken. It is on the hand-test list.

Third, `enum_openimagedenoise_denoiser` depends on `_cycles.with_openimagedenoise` — a property of the Blender build, not of the window. A build without it offers no OpenImageDenoise anywhere.

**Operating system.** The settings themselves are the same everywhere; nothing in their definitions is conditional on an operating system. What differs is which denoisers and which devices are actually available, and the manual is explicit about that:

> "GPU acceleration for OpenImageDenoise is available for compute capability 7.0 and higher, which includes all NVIDIA RTX cards."

> "HIP is supported on Windows and Linux… GPU accelerated denoising is available on discrete Radeon RX 6000 and Radeon RX 7000 GPUs."

> "oneAPI is a computation library that is supported on Windows and Linux… Hardware acceleration for ray-tracing and denoising is supported… GPU accelerated denoising is available on all supported GPUs."

> "Metal is supported on Apple computers with Apple Silicon. macOS 13.0 or newer is required to support all features… GPU accelerated denoising is available on Apple Silicon."

(Blender manual, https://docs.blender.org/manual/en/4.5/render/cycles/gpu_rendering.html, read 2026-07-30.)

So `denoising_use_gpu` and the OptiX choice can be set on any operating system, but what they can do differs. OptiX is described as "only available on NVIDIA GPUs when configured in the system tab in the user preferences". Which operating systems can carry such a card is not something the quoted words say, and nothing here supplies it.

### Where I looked, for question 19

- `denois` (case-insensitive) across the entire manual. Fourteen files matched: `genindex.txt`, `render/cycles/gpu_rendering.txt`, `render/cycles/render_settings/index.txt`, `render/cycles/render_settings/performance.txt`, `render/cycles/render_settings/sampling.txt`, `render/cycles/optimizations/index.txt`, `render/cycles/optimizations/reducing_noise.txt`, `render/eevee/render_settings/performance.txt`, `render/eevee/render_settings/raytracing.txt`, `render/layers/passes.txt`, `compositing/index.txt`, `compositing/types/filter/index.txt`, `compositing/types/filter/denoise.txt`, `getting_started/about/history.txt`. I read the render-related ones. I ruled out the compositing ones as being about the Denoise node itself rather than a setting a scene or view layer carries — except where the scene's three `compositor_denoise_*` settings feed it, which I have included. I ruled out `history.txt` and `genindex.txt` as not carrying settings.
- `denois` across the entire API reference. Matches in `bpy.ops.cycles.txt` (the `denoise_animation` operator — an operator, not a setting the scene carries, so ruled out of this answer), `bpy.types.CompositorNodeDenoise.txt` (the node's own settings, not the scene's), `bpy.types.RenderSettings.txt`, `bpy.types.SceneEEVEE.txt`, `bpy.types.RaytraceEEVEE.txt`, `bpy.types.NodeInternalSocketTemplate.txt` and `change_log.txt`. Nothing else.
- `denois` across all of Cycles' own files. All matches are in `properties.py`, `ui.py`, `engine.py`, `operators.py`, `presets.py` and `version_update.py`. I read every match. The `source/` folder holds only kernel headers and matched nothing.
- `denois` and `sample` across all nineteen release-note files for 4.5.
- Read `bpy.types.ViewLayer.txt` and `bpy.types.ViewLayerEEVEE.txt` in full to check for a view-layer denoising setting outside Cycles. `ViewLayerEEVEE` has none.
- Read `bpy.types.SceneEEVEE.txt` and `bpy.types.RaytraceEEVEE.txt` in full.
- Checked `bpy.types.Scene.txt` for what the scene carries — it points to `CyclesRenderSettings` for `cycles`, which has no page of its own in the API reference (there is no `bpy.types.CyclesRenderSettings.txt`), which is why Cycles' own file is the source for that whole group.
- Searched `_images` for a picture of a denoising panel. The only denoising pictures are the two before-and-after examples and two compositor-node pictures; there is no screenshot of the Sampling panel or the View Layer panel.

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

2026-08-29 — Claude Opus 5 (Claude Code) — Settings, open threads (questions 18 and 19), Blender 4.5
2026-08-30 — Claude Opus 5 (Claude Code) — hand-test results for the Open Threads questions (18 and 19) written in, the two standing lines on each answer brought up to date, and the researcher's own list pruned to what is still open
2026-08-30 — Claude Opus 5 (Claude Code) — hand-test entries 13 and 14 written in: the compositor denoise-quality comparison run twelve times over in one sitting, and the order of the denoising feature passes read three times over, both on the Creator's Mac and both recorded as observed

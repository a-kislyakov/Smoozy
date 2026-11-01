# Settings — Open Threads questions — Blender 5.0

**Sources consulted, and on what date.** All reading was done on 2026-08-29, from the snapshot taken on 2026-07-30 that sits in `Blender Sources/`. Specifically: the Blender 5.0 manual (`manual_5.0`, including its `_images` folder), the Blender 5.0 Python API documentation (`api_5.0`), the Blender 5.0 release notes (`release_notes_5.0`), and Cycles' own files (`cycles_5.0`). I also read the question sheet `Preparation/Blender Research/Questions/Settings.md`. I did not use the web.

I read only the question sheet and files inside `Blender Sources/`. I did not open any other part of this project.

*One thing to declare alongside that line, so it is exactly true.* This is a first pass — the only research pass there has been on questions 18 and 19 for this version. The brief it was written from called it a second pass and said an earlier findings document existed; neither was so. The researcher checked by listing the file names in `Preparation/Blender Research/Findings/5.0/` — names only, no file opened — and found no `Settings — Open Threads questions.md` there. That listing showed only topic titles ("Frames", "Images", "Reports"), said nothing about Blender, and nothing from it is used anywhere below. What the brief's second-pass wording actually asked for — search widely, and write down where you looked — is what the two records of looking below hold.

Answers cover questions **18** and **19** only, as instructed.

---

## 18. What values will a render's sample count be set to, and what does a render do when it is given a value outside those?

**The question was ambiguous, and I say so plainly.** "A render's sample count" could mean (a) the single number in a scene's render settings that says how many samples a final render takes, or (b) every setting in Blender that counts samples. I answer reading (a) — the sample count that governs a final render — and I give it for each render engine that has one, plus the per-view-layer override of it, because in Blender 5.0 there is no one engine-independent sample count. Reading (b) would additionally sweep in things like volumetric step counts, shadow ray counts and motion-blur samples; I have not covered those.

### 18a. Cycles — the scene's final-render sample count

**LABEL: observed** (from Cycles' own files).

The property is `samples` on Cycles' scene settings. It is defined as a whole-number property with a minimum of 1, a maximum of 16,777,216 (`1 << 24` is 16,777,216) and a default of 4096. What those two bounds then do to a number handed in is a step beyond the file, and 18b takes it.

Exact words:

```
    samples: IntProperty(
        name="Samples",
        description="Number of samples to render for each pixel",
        min=1, max=(1 << 24),
        default=4096,
    )
```

- File and place: `Blender Sources/cycles_5.0/properties.py`, lines 479–484, inside `class CyclesRenderSettings(bpy.types.PropertyGroup)` (that class begins at line 378).
- Date read: 2026-07-30 snapshot, read 2026-08-29.

That this property belongs to the scene is documented separately: the API documentation lists `cycles` on `Scene` as "Cycles render settings", of type `CyclesRenderSettings, (readonly)` — https://docs.blender.org/api/5.0/bpy.types.Scene.html (read 2026-08-29).

The manual gives the same setting a name and a meaning but no numbers: "**Render (Max) Samples** — The number of paths to trace per pixel in the final render. A higher number results in a cleaner image at the cost of a longer render time." — https://docs.blender.org/manual/en/5.0/render/cycles/render_settings/sampling.html (read 2026-08-29). Cycles' interface code labels the same property "Max Samples" when adaptive sampling is on and "Samples" when it is off (`Blender Sources/cycles_5.0/ui.py`, lines 311–316). *(Address corrected by the citation check of 2026-08-30: the lines given were 306–310, which are the noise-threshold rows; the labelling is at 311–316.)*

**Not covered:** this settles how the property is defined, not what a render engine does with the number after it has been stored.

### 18b. What happens when the Cycles sample count is given a value outside 1 … 16,777,216

There are two separate cases and they have different answers.

**Case 1 — assigned from a script.** **LABEL: inferred.**

The API documentation for `bpy.props.IntProperty` says of the two arguments used above:

> "min (int) – Hard minimum, trying to assign a value below will silently assign this minimum instead."
>
> "max (int) – Hard maximum, trying to assign a value above will silently assign this maximum instead."

- Source: Python API documentation, https://docs.blender.org/api/5.0/bpy.props.html (the `bpy.props.IntProperty` entry begins at line 788 of the snapshot file). Date read: 2026-08-29.

**The step I took, written out:** the documentation quoted above describes what `min` and `max` mean for any property *defined* with `bpy.props.IntProperty`. Cycles' `samples` is defined with `IntProperty(... min=1, max=(1 << 24) ...)`, as quoted in 18a. Putting the two together — the rule from the documentation, applied to the definition observed in Cycles' file — gives: assigning 0 or a negative number silently stores 1; assigning more than 16,777,216 silently stores 16,777,216. No error, no message. The documentation nowhere states this about Cycles' `samples` specifically, so this is inferred, not documented.

**Case 2 — typed into the field in Blender's own window.** **LABEL: inferred.**

> "Most numerical values are restricted by "soft limit" and "hard limit" value ranges. Changing values by dragging with the mouse is restricted to the "soft limit" value range. Input via keyboard will allow the use of wider value ranges, but never wider than the "hard limit"."

- Source: manual, https://docs.blender.org/manual/en/5.0/interface/controls/buttons/fields.html (lines 110–114 of the snapshot file). Date read: 2026-08-29.

**The step I took, written out:** the passage is about numerical fields in general — and it says "most" of them, not all — and it never mentions the sample count. Applying it to this field is the same kind of step as Case 1: the rule from the manual, applied to a field that shows a property defined with a hard minimum of 1 and a hard maximum of 16,777,216, gives that a typed number will not be stored outside those two. The manual nowhere says this of the sample count, so this is inferred, not documented. It is written as inferred here on purpose, because Case 1 above takes a step of exactly the same shape and carries the same label.

**Not covered:** the manual passage does not say what the field *displays* or *reports* when a too-large number is typed — only that the value will not go past the hard limit. Neither source says anything is written to the console or the info log. Whether a fractional value (say 100.5) is rounded, truncated or refused is not stated in either source. I found no statement anywhere in the sources about what a *render* does at render time — as opposed to what the *property* does at assignment time — with an out-of-range request.

### 18c. Cycles — the viewport sample count

**LABEL: observed.** Defined as a whole-number property with a minimum of 0, a soft minimum of 1, a maximum of 16,777,216 and a default of 1024. Its own description says it is unlimited if 0.

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

- File and place: `Blender Sources/cycles_5.0/properties.py`, lines 485–493.
- Date read: 2026-07-30 snapshot, read 2026-08-29.

**What the soft minimum does. LABEL: inferred.** The step: the manual's "Value Limits" passage quoted in 18b says dragging is held to the soft limits and typing to the hard ones; applied to a property defined with a soft minimum of 1 and a minimum of 0, that gives dragging held at 1 while 0 can still be typed. Neither source says this of this property.

The manual states the zero meaning independently: "**Viewport (Max) Samples** — The number of paths to trace per pixel in the 3D Viewport (when using the Rendered shading mode). Setting this value to zero enables indefinite sampling." — https://docs.blender.org/manual/en/5.0/render/cycles/render_settings/sampling.html (read 2026-08-29). **LABEL for that sentence: documented.**

**Not covered:** this is the viewport count, not the final-render count.

### 18d. Cycles — three more whole-number sample settings on the same scene settings

**LABEL: observed.** All three are on the same scene settings class.

The minimum sample count used by adaptive sampling is defined with a minimum of 0, a maximum of 4096 and a default of 0:

```
    adaptive_min_samples: IntProperty(
        name="Adaptive Min Samples",
        description="Minimum AA samples for adaptive sampling, to discover noisy features before stopping sampling. Zero for automatic setting based on noise threshold",
        min=0,
        max=4096,
        default=0,
    )
```

- `Blender Sources/cycles_5.0/properties.py`, lines 580–586.

The two sample subset settings are defined as:

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

- `Blender Sources/cycles_5.0/properties.py`, lines 500–512.
- Date read: 2026-07-30 snapshot, read 2026-08-29.

The manual documents the subset in prose, including that the length overrides the max samples for what actually gets rendered: "**Length** — The number of samples in the subset. While this overrides (Max) Samples in terms of the samples that will get rendered, it's still important to set (Max) Samples to the total number of samples that will be rendered across all subsets – otherwise, the subsets will have incompatible noise and combining them will give a worse result." — https://docs.blender.org/manual/en/5.0/render/cycles/render_settings/sampling.html (read 2026-08-29). **LABEL: documented.**

**Not covered:** the sources say nothing about what happens if the offset plus the length runs past the max samples, or past 16,777,216. Neither Cycles' interface code nor the manual shows a warning for that combination.

### 18e. EEVEE — the scene's sample counts

**LABEL: documented.**

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

- Source: Python API documentation, https://docs.blender.org/api/5.0/bpy.types.SceneEEVEE.html (lines 300–314 of the snapshot file). Date read: 2026-08-29.

The manual gives the same two in prose: "**Render / Samples** — The number of samples to use in the final render." and "**Viewport / Samples** — The number of samples to use in the 3D Viewport. When setting this to zero the viewport will be resampled continuously." — https://docs.blender.org/manual/en/5.0/render/eevee/render_settings/sampling.html (read 2026-08-29).

That these belong to the scene is documented: `Scene.eevee`, "EEVEE settings for the scene", type `SceneEEVEE, (readonly)` — https://docs.blender.org/api/5.0/bpy.types.Scene.html (read 2026-08-29).

**Not covered:** the API documentation writes the upper bound as "inf", which is not a number a computer can store; the real upper bound is not stated anywhere I found. And nothing in the sources says what happens when a value below 1 is assigned to `taa_render_samples` from a script — see the gap recorded below.

### 18f. Workbench — the scene's sample setting is a choice, not a number

**LABEL: documented.**

> "render_aa
>
> Method of anti-aliasing when rendering final image
>
> OFF No Anti-Aliasing – Scene will be rendering without any anti-aliasing.
> FXAA Single Pass Anti-Aliasing – Scene will be rendered using a single pass anti-aliasing method (FXAA).
> 5 5 Samples – Scene will be rendered using 5 anti-aliasing samples.
> 8 8 Samples – Scene will be rendered using 8 anti-aliasing samples.
> 11 11 Samples – Scene will be rendered using 11 anti-aliasing samples.
> 16 16 Samples – Scene will be rendered using 16 anti-aliasing samples.
> 32 32 Samples – Scene will be rendered using 32 anti-aliasing samples.
>
> Type:
>
> enum in ['OFF', 'FXAA', '5', '8', '11', '16', '32'], default '8'"

- Source: Python API documentation, https://docs.blender.org/api/5.0/bpy.types.SceneDisplay.html (lines 41–70 of the snapshot file). Date read: 2026-08-29.

The manual agrees the list is fixed: "The number of samples are predefined so it uses the best distribution of the samples. 5, 8, 11, 16, 32" — https://docs.blender.org/manual/en/5.0/render/workbench/sampling.html (read 2026-08-29). It also says the choice is stored per scene: "The setting for viewport rendering and final rendering is saved per scene."

**Not covered:** the sources do not say what happens when a value outside that list is assigned. That is recorded as a gap below.

### 18g. The per-view-layer override of the sample count

**LABEL: documented.**

> "samples
>
> Override number of render samples for this view layer, 0 will use the scene setting
>
> Type:
>
> int in [0, inf], default 0"

- Source: Python API documentation, https://docs.blender.org/api/5.0/bpy.types.ViewLayer.html (lines 147–153 of the snapshot file). Date read: 2026-08-29.

The manual says how the override is treated: "**Samples** — Allows the view layer to use a different number of samples than the main scene. This is controlled by the Layer Samples when using the Cycles renderer. When using EEVEE, the Samples override is always used." — https://docs.blender.org/manual/en/5.0/render/layers/override.html (read 2026-08-29).

The Cycles side of that control is defined with three choices. **LABEL: observed.**

```
enum_use_layer_samples = (
    ('USE', "Use", "Per render layer number of samples override scene samples"),
    ('BOUNDED', "Bounded", "Bound per render layer number of samples by global samples"),
    ('IGNORE', "Ignore", "Ignore per render layer number of samples"),
)
```

- `Blender Sources/cycles_5.0/properties.py`, lines 64–68; the property itself, `use_layer_samples`, is at lines 550–555 with `default='USE'`.
- Date read: 2026-07-30 snapshot, read 2026-08-29.

The manual states the same three in prose: "**Use:** Allow view layers to override the scene-level sample count. **Bounded:** Allow the overrides as long as they don't exceed the scene-level sample count. **Ignore:** Ignore the overrides." — https://docs.blender.org/manual/en/5.0/render/cycles/render_settings/sampling.html (read 2026-08-29).

That EEVEE honours this override is a change announced for 5.0: "View Layer Overrides (Material, World, Sample) are now supported. Older files using this feature will render differently." — release notes, https://developer.blender.org/docs/release_notes/5.0/eevee/ (read 2026-08-29). **LABEL: documented.**

**Not covered:** "Bounded" is described as bounding the layer count by the scene count, but neither source says what number is actually used when a layer asks for more — whether it is cut down to the scene count or the layer is refused. That is not stated.

### 18h. Where I looked, and what I ruled out

- Searched all 2,195 manual files and all 2,062 API files for `sample` (case-insensitive) and for `samples` specifically; then narrowed by folder.
- Manual pages opened in full: `render/cycles/render_settings/sampling.txt`, `render/eevee/render_settings/sampling.txt`, `render/workbench/sampling.txt`, `render/layers/override.txt`, `render/layers/view_layer.txt`, `render/layers/filter.txt`, `interface/controls/buttons/fields.txt`, `advanced/limits.txt`, `advanced/command_line/render.txt`.
- API pages opened: `bpy.props.txt`, `bpy.types.SceneEEVEE.txt`, `bpy.types.ViewLayer.txt`, `bpy.types.SceneDisplay.txt`, `bpy.types.Scene.txt`, `bpy.types.IntProperty.txt`, `bpy.types.bpy_struct.txt`, `change_log.txt`.
- Cycles files opened: `properties.py`, `ui.py`, `engine.py`, `version_update.py`, `operators.py`. I also listed `cycles_5.0/source`, which holds only `kernel` and `util` header files — the code that reads these settings during a render is not present in the folder I was given, so I could not check what a render does with them beyond what the property definitions say.
- Terms searched across manual and API for the out-of-range behaviour: `clamp`, `clamped to`, `will be clamped`, `out of range`, `outside the range`, `beyond the range`, `hard min`, `hard max`, `soft min`, `soft limit`, `hard limit`, `valid range`, `allowed range`, `minimum and maximum`, `silently`, `ValueError`, `TypeError`. The only statements that bear on it are the two quoted above (`bpy.props.IntProperty`, and the manual's "Value Limits" passage). Everything else was about unrelated things — weight painting, F-curve handles, the Map Range node, Freestyle crease angles, geometry-node index wrapping, dynamic paint depth — and I ruled each out on reading the line.
- I checked `advanced/limits.txt` ("Working Limits") in case sample counts were listed there. It covers only spatial precision and the maximum frame count; it says nothing about samples.
- I searched the whole manual and API for the literal `16777216` and found nothing, so the Cycles upper bound is not written anywhere in the documentation — only in Cycles' own file.
- I searched the API pages for Cycles' own property names (`Number of samples to render for each pixel`, `CyclesRenderSettings`) and confirmed the Cycles scene properties are **not** individually documented in the Python API reference. `bpy.types.Scene.txt` names `cycles` as a pointer to `CyclesRenderSettings`, and `change_log.txt` names the class, but no page lists its members. This is why 18a had to come from Cycles' own file.
- I checked the 5.0 API change log (`change_log.txt`, "4.5 to 5.0") for anything touching sample counts. Under `bpy.types.CyclesRenderSettings` the only entries are "Added: `bpy.types.CyclesRenderSettings.volume_biased`" and "Removed: `feature_set`". No sample property is listed as added, removed or renamed. Under `bpy.types.RenderSettings` the removals are all baking properties. Source: https://docs.blender.org/api/5.0/change_log.html (read 2026-08-29).
- I searched all 23 release-note files for `sample` and `denois`. Nothing in them changes any sample range.
- I looked for a picture. The manual's Cycles and EEVEE sampling pages contain no `[image: ...]` line at all, so there is no screenshot of a sampling panel to read numbers from. I opened the one view-layer picture that exists, `_images/render_layers_view-layer_panel.png` (referenced from https://docs.blender.org/manual/en/5.0/render/layers/view_layer.html); it shows only the "View Layer" panel with "Use for Rendering" and "Render Single Layer", and the top of a "Passes" panel. No samples override and no denoising control is visible in it.

**Gaps I am recording honestly, as "not in the sources":** for the built-in properties (EEVEE's two, the view layer's one, Workbench's choice), no source I found says what happens when a script assigns a value outside the stated range. The `bpy.props` statement covers properties *defined* through `bpy.props`, which is how Cycles' properties are made, but the built-in ones are not defined that way in anything I was given. I will not guess.

**Outside the interface.** The ranges themselves are properties of the settings, and settings are carried in the file, so nothing here depends on a person being present. But two parts of this answer do not carry outside Blender's window. The manual's "Value Limits" passage is explicitly about dragging and typing in a field, so it says nothing about a render driven from outside. And I searched the full command-line arguments page for `sample` and `denois` and found zero occurrences — https://docs.blender.org/manual/en/5.0/advanced/command_line/arguments.html (read 2026-08-29) — so the sources describe no launch instruction that hands in a sample count, and therefore describe no out-of-range handling for one. What the sources *do* leave available from outside is `--python-expr`, listed on that same page, which would set the property and so fall under the script case in 18b. Beyond that the sources are silent.

**Operating system.** Nothing in the property definitions, the API ranges, or the manual passages is conditioned on an operating system. The `min`/`max` values quoted from Cycles' file are plain constants with no platform test around them (the only platform test I saw in Cycles' files is an unrelated one about a macOS version and a shading feature, `Blender Sources/cycles_5.0/ui.py`, in `use_mnee`). The sources do not state that sample ranges differ by operating system, and I found nothing suggesting they do.

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

I take "a scene or a view layer" to mean settings stored on the scene or on a view layer, so I cover all four groups that exist in Blender 5.0: Cycles' denoising settings on the scene, Cycles' two on each view layer, the engine-independent compositor-denoise settings on the scene, and EEVEE's denoising settings on the scene. The Denoise node's own settings are not scene or view-layer settings, so I describe them only where a scene setting governs them.

### 19a. Cycles, on the scene — final-render denoising

**LABEL: observed** for the exact settings, names, kinds and defaults; **documented** for what several of them mean.

All of the following are on `class CyclesRenderSettings(bpy.types.PropertyGroup)` in `Blender Sources/cycles_5.0/properties.py` (class begins line 378), reached from a scene as `Scene.cycles`.

**`use_denoising` — the master switch for denoising the final image. On by default.**

```
    use_denoising: BoolProperty(
        name="Use Denoising",
        description="Denoise the rendered image",
        default=True,
        update=update_render_passes,
    )
```
Lines 400–405.

Manual: "**Denoise (Render)** — Whether to perform denoising for the final rendered image." — https://docs.blender.org/manual/en/5.0/render/cycles/render_settings/sampling.html (read 2026-08-29).

**`denoiser` — which denoising algorithm is used for the final image.**

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
Lines 406–413.

Its list of choices is not fixed in the file — it is built when asked:

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
Lines 265–277.

```
def enum_denoiser(self, context):
    items = []
    items += enum_optix_denoiser(self, context)
    items += enum_openimagedenoise_denoiser(self, context)
    return items
```
Lines 299–303.

**LABEL for the following sentence: inferred.** The choices offered for the final-render denoiser are OptiX and OpenImageDenoise, and each is offered only under a condition — OpenImageDenoise only if this build of Blender was made with it, OptiX only if there is no context to check or if OptiX devices are configured in the preferences. **The step I took:** the two functions above return their entries only inside those `if` tests and return an empty list otherwise, and `enum_denoiser` returns nothing but the concatenation of the two; from that I conclude the offered list can be shorter, and can in principle be empty, on a machine or a build that fails both tests. The file does not say so in words.

Manual, on the same setting: "**Denoiser** — The algorithm to use." followed by "**OpenImageDenoise:** Uses Intel's Open Image Denoise, an AI denoiser. Typically provides the highest quality." and "**OptiX:** Uses NVIDIA's OptiX AI denoiser. Supports GPU acceleration on some older NVIDIA GPUs where OpenImageDenoise does not. Only available on NVIDIA GPUs when configured in the Cycles Render Devices user preferences." — https://docs.blender.org/manual/en/5.0/render/cycles/render_settings/sampling.html (read 2026-08-29). **LABEL: documented.**

**`denoising_input_passes` — which extra picture data the denoiser is fed so it can tell noise from real detail.**

```
    denoising_input_passes: EnumProperty(
        name="Denoising Input Passes",
        description="Passes used by the denoiser to distinguish noise from shader and geometry detail",
        items=enum_denoising_input_passes,
        default='RGB_ALBEDO_NORMAL',
    )
```
Lines 426–431, with the three choices at lines 306–310:

```
enum_denoising_input_passes = (
    ('RGB', "None", "Don't use utility passes for denoising", 1),
    ('RGB_ALBEDO', "Albedo", "Use albedo pass for denoising", 2),
    ('RGB_ALBEDO_NORMAL', "Albedo and Normal", "Use albedo and normal passes for denoising", 3),
)
```

Manual: "**Passes** — Controls which Render Passes the denoiser should use as input. Generally, the more passes the denoiser has access to, the better the result. It is recommended to use at least Albedo as None can blur out details, especially at lower sample counts." — same manual page, read 2026-08-29. **LABEL: documented.**

**`denoising_prefilter` — whether, and how hard, those extra passes are cleaned up before they are used. Shown only when OpenImageDenoise is the chosen denoiser.**

```
    denoising_prefilter: EnumProperty(
        name="Denoising Prefilter",
        description="Prefilter noisy guiding (albedo and normal) passes to improve denoising quality when using OpenImageDenoise",
        items=enum_denoising_prefilter,
        default='ACCURATE',
    )
```
Lines 414–419, with the three choices at lines 312–325 (`NONE` "No prefiltering, use when guiding passes are noise-free"; `FAST` "Denoise color and guiding passes together. Improves quality when guiding passes are noisy using least amount of extra processing time"; `ACCURATE` "Prefilter noisy guiding passes before denoising color. Improves quality when guiding passes are noisy using extra processing time").

Manual: "**Prefilter OpenImageDenoise** — Controls whether prefiltering is applied to the Passes before denoising. Visible only when using OpenImageDenoise." — same manual page. **LABEL: documented.** Cycles' interface code matches that visibility rule: the prefilter and quality rows are drawn only `if cscene.denoiser == 'OPENIMAGEDENOISE'` (`Blender Sources/cycles_5.0/ui.py`, lines 347–349).

**`denoising_quality` — how much effort the denoiser spends. Shown only when OpenImageDenoise is the chosen denoiser.**

```
    denoising_quality: EnumProperty(
        name="Denoising Quality",
        description="Overall denoising quality when using OpenImageDenoise",
        items=enum_denoising_quality,
        default='HIGH',
    )
```
Lines 420–425, with the three choices at lines 327–338 (`HIGH` "High quality"; `BALANCED` "Balanced between performance and quality"; `FAST` "High performance").

Manual: "**Quality OpenImageDenoise** — Overall denoising quality. Visible only when using OpenImageDenoise." with "**High:** Produces the highest quality at the cost of time. **Balanced:** Balanced between performance and quality. **Fast:** Sacrifices quality for speed (ideal for viewport rendering)." — same manual page. **LABEL: documented.**

**`denoising_use_gpu` — whether the denoising work itself runs on graphics hardware. Off by default for the final render.**

```
    denoising_use_gpu: BoolProperty(
        name="Denoise on GPU",
        description="Perform denoising on GPU devices configured in the system tab in the user preferences. This is significantly faster than on CPU, but requires additional GPU memory. When large scenes need more GPU memory, this option can be disabled",
        default=False,
    )
```
Lines 432–437.

Manual: "**Use GPU** — Perform denoising on the GPU. This is significantly faster than on CPU, but requires additional GPU memory. When large scenes need more GPU memory, this option can be disabled." — same manual page. **LABEL: documented.**

- File and place for all of the above: `Blender Sources/cycles_5.0/properties.py`, at the line numbers given.
- Date read: 2026-07-30 snapshot, read 2026-08-29.

**Not covered:** the file settles that these settings exist, under these names, with these kinds and defaults, and the manual quoted alongside settles what each is for. Neither settles what a render actually does with any of them. In particular, "shown only when OpenImageDenoise is the chosen denoiser" is what the manual and the interface code say about the prefilter and quality settings; whether either still has an effect when another denoiser is chosen is not stated anywhere I found.

### 19b. Cycles, on the scene — viewport denoising

The same shape of settings exists a second time for the 3D Viewport, with different defaults. All observed from `Blender Sources/cycles_5.0/properties.py`. **LABEL: observed.**

| Setting | Line | What it does (its own description text) | Default |
|---|---|---|---|
| `use_preview_denoising` | 438–442 | "Denoise the image in the 3D viewport" | `False` |
| `preview_denoiser` | 443–448 | "Denoise the image after each preview update with the selected denoiser" | `0`, which is the value carried by `AUTO` in the function quoted below |
| `preview_denoising_prefilter` | 449–454 | "Prefilter noisy guiding (albedo and normal) passes to improve denoising quality when using OpenImageDenoise" | `'FAST'` |
| `preview_denoising_quality` | 455–460 | "Overall denoising quality when using OpenImageDenoise" | `'BALANCED'` |
| `preview_denoising_input_passes` | 461–466 | "Passes used by the denoiser to distinguish noise from shader and geometry detail" | `'RGB_ALBEDO'` |
| `preview_denoising_start_sample` | 467–472 | "Sample to start denoising the preview at"; `min=0, max=(1 << 24)` | `1` |
| `preview_denoising_use_gpu` | 473–477 | "Perform denoising on GPU devices configured in the system tab in the user preferences. This is significantly faster than on CPU, but requires additional GPU memory. When large scenes need more GPU memory, this option can be disabled" | `True` |

The viewport denoiser has one extra choice the final-render one does not — an automatic setting, and a stand-in when the build has no denoiser at all:

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
Lines 280–296.

Manual, on two of these: "**Denoise (Viewport)** — Whether to perform denoising for the 3D Viewport in the Rendered shading mode." and "**Start Sample** — Sample at which to start denoising in the 3D Viewport." and, for the automatic choice, "**Automatic:** Uses GPU accelerated denoising if supported, for best performance. Prefers OpenImageDenoise over OptiX." — https://docs.blender.org/manual/en/5.0/render/cycles/render_settings/sampling.html (read 2026-08-29). **LABEL: documented.**

- Date read: 2026-07-30 snapshot, read 2026-08-29.

**Not covered:** these govern the 3D Viewport, and the sources do not say they have any effect on a final render.

### 19c. Cycles, on each view layer — two settings

**LABEL: observed.**

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

- File and place: `Blender Sources/cycles_5.0/properties.py`, lines 1532–1543, inside `class CyclesRenderLayerSettings(bpy.types.PropertyGroup)` (class begins line 1479). That class attaches itself to view layers at lines 1546–1551: `bpy.types.ViewLayer.cycles = PointerProperty(name="Cycles ViewLayer Settings", ... type=cls,)`.
- Date read: 2026-07-30 snapshot, read 2026-08-29.

**How the layer switch relates to the scene switch. LABEL: inferred.** The scene switch gates the layer switch in two visible ways.

In the interface, the layer's checkbox is drawn greyed out when the scene's is off:

```
        sub = col.row()
        sub.prop(view_layer.cycles, "use_denoising", text="Denoising")
        sub.active = scene.cycles.use_denoising
```
`Blender Sources/cycles_5.0/ui.py`, lines 989–991, inside `class CYCLES_RENDER_PT_filter` whose `bl_context` is `"view_layer"` (lines 964–967). *(Address corrected by the citation check of 2026-08-30: the class begins at 964 and its `bl_context` line is 967, one past the range previously given.)*

And in the list of passes a render produces, the noisy-image pass appears only when both are on:

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
`Blender Sources/cycles_5.0/engine.py`, lines 242–250.

**The step I took:** these two pieces of code show the scene switch controlling whether the layer switch is drawn as available, and controlling whether the noisy-image pass is produced. From that I conclude the layer switch is subordinate to the scene switch — a layer cannot turn denoising on when the scene has it off. The files do not say this in words, and neither piece of code is the code that actually decides whether the picture gets denoised, so this is inferred.

Note also, from the last three lines quoted, that `denoising_store_passes` is *not* gated by either denoising switch: those three passes are produced whenever that one setting is on. **LABEL: inferred**, from the plain structure of the `if` statements above.

Manual, on what those stored passes are: "**Denoising Data** Cycles — Includes Denoising Albedo, Denoising Normal, and the combined image before denoising. Can be used with the Denoise Node as a replacement for automatic denoising." — https://docs.blender.org/manual/en/5.0/render/layers/passes.html (read 2026-08-29). **LABEL: documented.** The Cycles sampling page points the same way: "If you enable the Denoising Data render pass, you can alternatively denoise in the compositing step using the Denoise Node." — https://docs.blender.org/manual/en/5.0/render/cycles/render_settings/sampling.html (read 2026-08-29).

**A documented gap worth recording.** The manual page that describes the view-layer panel these two settings live in does not mention denoising at all. Its "Use" section lists only one entry: "**Motion Blur** Cycles, EEVEE — Render motion blur for current layer, if enabled in the Render Settings." — https://docs.blender.org/manual/en/5.0/render/layers/filter.html (read 2026-08-29). The per-view-layer Denoising checkbox exists in Cycles' own file and is drawn in that same panel, but it is **not in the manual**. The view-layer picture in the manual does not show it either (see the record of looking below).

**One documented statement that ties both levels together:** the operator that denoises an already-rendered sequence is described as "Denoise rendered animation sequence using current scene and view layer settings. Requires denoising data passes and output to OpenEXR multilayer files" — Python API documentation, https://docs.blender.org/api/5.0/bpy.ops.cycles.html (read 2026-08-29). **LABEL: documented.**

### 19d. On the scene, independent of engine — the compositor's Denoise nodes

These are on `Scene.render` (`RenderSettings`), so they are carried by the scene. **LABEL: documented** for the settings themselves, below. That they apply whichever engine renders is a step, not a quote: nothing in their own wording names an engine, and the manual carries the same three on the performance page of both engines. **LABEL for that step: inferred.**

> "compositor_denoise_device
>
> The device to use to process the denoise nodes in the compositor
>
> AUTO Auto – Use the same device used by the compositor to process the denoise node.
> CPU CPU – Use the CPU to process the denoise node.
> GPU GPU – Use the GPU to process the denoise node if available, otherwise fallback to CPU.
>
> Type:
>
> enum in ['AUTO', 'CPU', 'GPU'], default 'AUTO'"

> "compositor_denoise_final_quality
>
> The quality used by denoise nodes during the compositing of final renders if the nodes' quality option is set to Follow Scene
>
> ... Type: enum in ['HIGH', 'BALANCED', 'FAST'], default 'HIGH'"

> "compositor_denoise_preview_quality
>
> The quality used by denoise nodes during viewport and interactive compositing if the nodes' quality option is set to Follow Scene
>
> ... Type: enum in ['HIGH', 'BALANCED', 'FAST'], default 'BALANCED'"

- Source: Python API documentation, https://docs.blender.org/api/5.0/bpy.types.RenderSettings.html (lines 47–95 of the snapshot file). Date read: 2026-08-29.

The manual carries the same three under "Denoise Nodes", on the performance page of each engine, with fuller wording for the quality choices: "**High:** Produces the highest quality output at the cost of long processing times. **Balanced:** Balanced between performance and quality, typically processing in half the time as High, while retaining most of the quality. **Fast:** Produces an output quickly at a noticeable cost of quality." — https://docs.blender.org/manual/en/5.0/render/cycles/render_settings/performance.html and https://docs.blender.org/manual/en/5.0/render/eevee/render_settings/performance.html (both read 2026-08-29).

The node side of the arrangement is documented too: the Denoise node's Quality has a "**Follow Scene:** Use the scene's quality setting." choice — https://docs.blender.org/manual/en/5.0/compositing/types/filter/denoise.html (read 2026-08-29). **LABEL: documented.**

**Not covered:** these govern Denoise nodes in the compositor. They are not the same thing as Cycles' own denoising of the rendered image, and the sources do not say the two interact.

### 19e. On the scene — EEVEE's denoising settings

**LABEL: documented.**

On `Scene.eevee` directly:

> "use_taa_reprojection
>
> Denoise image using temporal reprojection (can leave some ghosting)
>
> Type:
>
> boolean, default True"

- Source: https://docs.blender.org/api/5.0/bpy.types.SceneEEVEE.html (lines 364–370 of the snapshot file). Date read: 2026-08-29.

The manual calls it "**Temporal Reprojection** — Reduces noise while moving the viewport or during animation playback. Can leave some ghosting." and places it under the Viewport heading — https://docs.blender.org/manual/en/5.0/render/eevee/render_settings/sampling.html (read 2026-08-29).

On `Scene.eevee.ray_tracing_options`, which is documented as "EEVEE settings for tracing reflections", type "RaytraceEEVEE, (readonly)" (same API page, lines 260–266), there are four more:

> "use_denoise — Enable noise reduction techniques for raytraced effects — boolean, default True"
>
> "denoise_spatial — Reuse neighbor pixels' rays — boolean, default True"
>
> "denoise_temporal — Accumulate samples by reprojecting last tracing results — boolean, default True"
>
> "denoise_bilateral — Blur the resolved radiance using a bilateral filter — boolean, default True"

- Source: https://docs.blender.org/api/5.0/bpy.types.RaytraceEEVEE.html (lines 9–31 and 80–86 of the snapshot file). Date read: 2026-08-29.

The manual describes the same four in prose: "**Denoising** — Denoising can be enabled to reduce the amount of noise from the raw ray-traced output. This can help image stability but will also over-blur the final ray-traced output." then "**Spatial Reuse** — Reuse the rays from neighbor pixels. Can introduce some light leaks across surfaces." / "**Temporal Accumulation** — Accumulate samples by re-projecting the last ray tracing results. This removes Fireflies but also introduces color bias. Useful for viewport temporal stability or making renders converge faster." / "**Bilateral Filter** — Blur the resolved ray-traced output using a bilateral filter." — https://docs.blender.org/manual/en/5.0/render/eevee/render_settings/raytracing.html (read 2026-08-29).

**Not covered:** these clean up EEVEE's ray-traced effects and its temporal sampling. They are not a post-render denoiser of the finished picture in the way Cycles' `use_denoising` is, and the sources do not describe them as one. The manual also notes the ray-tracing settings "are only visible if Screen-Trace is the active tracing Method" — that sentence is on the same page and applies to the Precision and Thickness settings above the Denoising heading; whether it extends to the Denoising group is not stated, and I have not assumed it does.

### 19f. What did not change in 5.0

**LABEL: documented.** The Python API change log for "4.5 to 5.0" lists, under `bpy.types.CyclesRenderSettings`, only "Added: `bpy.types.CyclesRenderSettings.volume_biased`" and "Removed: `feature_set`"; under `bpy.types.CyclesRenderLayerSettings`, only additions of volume passes (`use_pass_volume_majorant`, `use_pass_volume_scatter`, `use_pass_volume_transmit` and neighbours). No denoising property on either class is listed as added, removed or renamed. Source: https://docs.blender.org/api/5.0/change_log.html (read 2026-08-29).

The one denoising line in the 5.0 release notes concerns behaviour, not settings: "The OptiX denoiser result quality is improved by flipping the image vertically to have a top-left origin before processing." — https://developer.blender.org/docs/release_notes/5.0/cycles/ (read 2026-08-29). **LABEL: documented.**

The change log does record that the Denoise *node* lost settings between 4.5 and 5.0 — under `bpy.types.CompositorNodeDenoise`, "Removed: `prefilter`, `quality`, `use_hdr`" (https://docs.blender.org/api/5.0/change_log.html, read 2026-08-29). I record it only because those are the node counterparts of scene settings covered in 19d; the node is not a scene or view-layer setting and I take no view on it here.

### 19g. Where I looked, and what I ruled out

- Searched all 2,062 API files and all 2,195 manual files for `denois` (case-insensitive), which is the stem covering denoise, denoiser, denoising, denoised. Also searched for `noise`, `noise reduction`, and `noise threshold` to catch settings named around it rather than after it.
- The complete list of API files that mention it: `bpy.types.txt`, `bpy.types.SceneEEVEE.txt`, `bpy.types.NodeInternalSocketTemplate.txt`, `bpy.types.CompositorNodeDenoise.txt`, `bpy.types.CompositorNode.txt`, `bpy.types.RaytraceEEVEE.txt`, `bpy.ops.cycles.txt`, `bpy.types.RenderSettings.txt`, `change_log.txt`, and the alphabetical index pages. I opened every one of these except the index pages and the two generic node-listing pages, which carry only names already covered.
- The complete list of manual files that mention it: `render/cycles/gpu_rendering.txt`, `render/cycles/optimizations/reducing_noise.txt`, `render/cycles/optimizations/index.txt`, `render/cycles/render_settings/sampling.txt`, `render/cycles/render_settings/index.txt`, `render/cycles/render_settings/performance.txt`, `render/eevee/render_settings/performance.txt`, `render/eevee/render_settings/raytracing.txt`, `render/layers/passes.txt`, `compositing/types/filter/denoise.txt`, `compositing/index.txt`, `compositing/types/filter/index.txt`, `getting_started/about/history.txt`, and `genindex.txt`. I opened all the substantive ones. I ruled out `compositing/index.txt` and `compositing/types/filter/index.txt` (contents listings only), `getting_started/about/history.txt` (a historical note, not a setting), `render/cycles/optimizations/index.txt` and `render/cycles/render_settings/index.txt` (contents listings), and `genindex.txt` (an index).
- I read `render/cycles/optimizations/reducing_noise.txt` in the relevant part; it points at the setting rather than describing it — "To fix this there is a post-processing technique to cleanup the final bit of noise. To use this enable Denoising in the Render tab of the Properties." (https://docs.blender.org/manual/en/5.0/render/cycles/optimizations/reducing_noise.html, read 2026-08-29) — so it adds nothing beyond the sampling page.
- Cycles' own files: searched `properties.py`, `ui.py`, `engine.py`, `operators.py`, `version_update.py`, `presets.py`, `osl.py`, `camera.py`, `__init__.py` for `denois`. `properties.py` holds every setting; `ui.py` holds how they are laid out and gated; `engine.py` holds the pass list quoted above; `operators.py` holds the denoise-animation operator; `version_update.py` holds only old-file fix-ups. `cycles_5.0/source` contains only `kernel` and `util` header files — the code that performs denoising during a render is not present in the folder I was given.
- To be sure I had not missed a per-view-layer denoising setting outside Cycles, I searched `bpy.types.ViewLayer.txt` for `denois` and found none, and I read `render/layers/view_layer.txt` and `render/layers/filter.txt` in full. Neither documents one. There is no `bpy.types.CyclesRenderLayerSettings` page in the API documentation — the only Cycles pages there are `bpy.ops.cycles.txt` and the unrelated `bpy.types.FModifierCycles.txt` — which is why 19c had to come from Cycles' own file.
- I looked for pictures. Searching the manual's `_images` folder for `denois` returned four files, none of them a settings panel: `compositing_types_filter_denoise_example.jpg` and the two `render_layers_denoising_example*.jpg` are before-and-after renders, and `node-types_CompositorNodeDenoise.webp` is the Denoise node itself. The Cycles sampling page, which is where the denoising settings are described, carries no `[image: ...]` line at all. I opened `_images/render_layers_view-layer_panel.png` (from https://docs.blender.org/manual/en/5.0/render/layers/view_layer.html): it shows the "View Layer" panel — "Use for Rendering", "Render Single Layer" — and the top of the "Passes" panel with Combined, Z, Mist, Normal, Ambient Occlusion, Subsurface Direct, Subsurface Color. **What is visible in it: no denoising control.** I do not read anything further into that; a panel not shown in one screenshot is not a panel that does not exist.
- Searched all 23 release-note files for `denois`; one hit, quoted in 19f.

**Outside the interface.** Every setting listed above is stored on the scene or the view layer, which is to say in the file, so a render driven from outside Blender's window carries them as saved. Two things differ, and both are documented rather than guessed. First, the list of denoisers on offer is not stored in the file: the manual says OptiX is "Only available on NVIDIA GPUs when configured in the Cycles Render Devices user preferences" (https://docs.blender.org/manual/en/5.0/render/cycles/render_settings/sampling.html, read 2026-08-29), and Cycles' own `enum_optix_denoiser` tests `context.preferences.addons[__package__].preferences.get_devices_for_type('OPTIX')` (`Blender Sources/cycles_5.0/properties.py`, line 274). Preferences belong to the machine, not the file. **LABEL: inferred** for the consequence — a file that names OptiX may be opened on a machine where OptiX is not among the choices; **the step:** the choice list is computed from preferences at the moment it is asked for, and preferences travel with the machine rather than with the file. Second, I searched the whole command-line arguments page for `denois` and found zero occurrences (https://docs.blender.org/manual/en/5.0/advanced/command_line/arguments.html, read 2026-08-29), so the sources describe no launch instruction that turns denoising on or off. Beyond these two points the sources do not say whether denoising behaves differently in a render started from outside. I note in passing that the manual does flag one view-layer setting as differing — "This option is ignored when rendering from the command line", said of Render Single Layer (https://docs.blender.org/manual/en/5.0/render/layers/view_layer.html, read 2026-08-29) — which shows the manual does record such differences where they exist; no comparable note appears against any denoising setting.

**Operating system.** The settings themselves are the same everywhere — nothing in their definitions is conditioned on an operating system. What is not the same everywhere is which denoiser can run, and this is documented plainly. **LABEL: documented**, all from https://docs.blender.org/manual/en/5.0/render/cycles/gpu_rendering.html (read 2026-08-29):

- "OptiX is supported on Windows and Linux and requires a NVIDIA graphics cards with compute capability 5.0 and higher and a driver version of at least 535." The quoted words name Windows and Linux and do not name macOS. Reading that as *OptiX is not available on macOS* is a step the sentence does not take. **LABEL for that step: inferred.**
- "GPU acceleration for OpenImageDenoise is available for compute capability 7.0 and higher, which includes all NVIDIA RTX cards."
- For AMD: "HIP is supported on Windows and Linux" and "GPU accelerated denoising is available on discrete Radeon RX 6000 and Radeon RX 7000 GPUs."
- For Intel: "oneAPI is a computation library that is supported on Windows and Linux" and "Hardware acceleration for ray-tracing and denoising is supported" and "GPU accelerated denoising is available on all supported GPUs."
- For Apple: "Metal is supported on Apple computers with Apple Silicon. macOS 13.0 or newer is required to support all features." and "GPU accelerated denoising is available on Apple Silicon."

So `denoiser` set to OptiX, and `denoising_use_gpu` set on, are the two settings whose effect depends on the operating system and the hardware. **LABEL for that sentence: inferred**, drawn from the quoted lines above rather than stated in any of them. The sources do not say what a render does when a file asks for OptiX on a machine that has none.

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

*Every figure agrees except one: `9135` against the `9136` above. Both runs are recorded and neither is preferred; the check was not in the room for the first. The verdict the passage rests on — that the setting changes a final render — is untouched, since the maximum difference agrees exactly and the control was zero in both. The figure has been sent back to the hand tests to be run again.)*

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

2026-08-29 — Claude Opus 5 (Claude Code) — Settings, open threads (questions 18 and 19), Blender 5.0
2026-08-30 — Claude Opus 5 (Claude Code) — hand-test results for the Open Threads questions (18 and 19) written in, the two standing lines on each answer brought up to date, and the researcher's own list pruned to what is still open
2026-08-30 — Claude Opus 5 (Claude Code) — hand-test entries 13 and 14 written in: the compositor denoise-quality comparison run twelve times over in one sitting, and the order of the denoising feature passes read three times over, both on the Creator's Mac and both recorded as observed

# Settings — Open Threads questions, range and denoising follow-up (Blender 5.1)

**Sources consulted, and when.** Blender 5.1 manual, Blender 5.1 Python API documentation, Blender 5.1 release notes, and Cycles' own files for 5.1 — all as supplied in `Blender Sources/`, all read as of the snapshot date **2026-07-30**. The question sheet `Preparation/Blender Research/Questions/Settings.md` was read for the wording of questions 18 and 19.

**I read only the question sheet and files inside `Blender Sources/`. I did not open any other part of this project.** No helper agents were used, so this declaration covers the whole of the work below.

Answering points A, B and C only, in order.

---

## A. The notation the Python API documentation uses for a maximum

### The reading I answered

The question can be read two ways, because the API documentation contains two things that fit "a maximum for a whole-number setting written in a notation rather than as a plain number". I answer the first as the main reading and record the second underneath, because the first is the one that appears in the per-setting entries a reader would consult when looking up a sample count or a resolution percentage.

### The notation: `inf`

**LABEL: documented** (that the notation is used and where).
**LABEL: not in the sources** (for what it stands for — nothing in the three sources defines, explains or gives it a meaning).

Every numeric setting in the API documentation carries a parenthesised range and default. Most maxima are plain numbers. Some are the token `inf`.

Exact words, from the entry for the setting that matters most to the sample-count question:

> "taa_render_samples
>
> Number of samples per pixel for rendering (in [1, inf], default 64)
>
> Type:
>
> int"

Source: Python API documentation, `https://docs.blender.org/api/5.1/bpy.types.SceneEEVEE.html` — date read 2026-07-30.

And, on the same page, one line further down:

> "taa_samples
>
> Number of samples, unlimited if 0 (in [0, inf], default 16)
>
> Type:
>
> int"

Same page, same date.

For contrast, a whole-number setting whose maximum *is* a plain number:

> "resolution_percentage
>
> Percentage scale for render resolution (in [1, 32767], default 100)
>
> Type:
>
> int"

Source: Python API documentation, `https://docs.blender.org/api/5.1/bpy.types.RenderSettings.html` — date read 2026-07-30.

**How widely `inf` is used for whole-number settings.** I counted every entry in the API documentation whose stated type is `int` (or an array of `int`) and whose range uses something other than plain digits. The result: `inf` is the only such notation, and it appears in six shapes — `[0, inf]` (192 entries), `[-inf, inf]` (169), `[1, inf]` (11), `[-1, inf]` (9), `[2, inf]` (1) and `[4, inf]` (1). *(Citation check, 2026-08-30: the first two counts read 174 and 166. Counted against the supplied files they are 192 and 169; the other four reproduce exactly, and the six shapes are right and are the only six. Corrected by this check.)* No whole-number entry anywhere in the API documentation writes a maximum in scientific notation, in powers, or in any other form. Scientific notation such as `[-1.04857e+06, 1.04857e+06]` occurs only on settings whose type is `float`. This is a count over the supplied files, so it is **observed** rather than quoted.

**What `inf` stands for: the sources do not say.** I searched the whole of the API documentation for the token `inf` standing on its own outside a range bracket. There are no such occurrences at all — the token appears only inside `(in [ … ])`. The manual never uses the token; the release notes use it once, in a bug-fix line about EXR pixel brightness, not as a notation. The pages that would be the natural home for an explanation do not give one:

- `https://docs.blender.org/api/5.1/bpy.types.IntProperty.html` describes `hard_max` only as "Maximum value used by buttons", and states its own range as `[-inf, inf]` — that is, it uses the notation without explaining it.
- `https://docs.blender.org/api/5.1/info_api_reference.html` ("API Reference Usage") explains how to find a property in the reference but says nothing about how ranges are written.
- The manual's glossary contains no entry for it.

I therefore cannot report what the notation stands for without supplying it myself, and I have not.

### The second reading: `2**31 - 1`

**LABEL: documented.**

The API documentation also writes a whole-number maximum as `2**31 - 1`, in the signature of the function that creates an integer setting. Exact words:

> "bpy.props.IntProperty(*, name='', description='', translation_context='*', default=0, min=-2\*\*31, max=2\*\*31 - 1, soft_min=-2\*\*31, soft_max=2\*\*31 - 1, step=1, options={'ANIMATABLE'}, override=set(), tags=set(), subtype='NONE', update=None, get=None, set=None, get_transform=None, set_transform=None)"

The same form appears in the signature of `bpy.props.IntVectorProperty` on the same page.

Here the notation *is* given a meaning, in the parameter list immediately below it:

> "max (int) – Hard maximum, trying to assign a value above will silently assign this maximum instead."

and

> "min (int) – Hard minimum, trying to assign a value below will silently assign this minimum instead."

Source: Python API documentation, `https://docs.blender.org/api/5.1/bpy.props.html` — date read 2026-07-30.

That tells you what the value is *for* — the hard maximum a Python-defined whole-number setting gets when its creator does not choose one. It does not tell you the arithmetic value of `2**31 - 1`; the documentation leaves that to the reader. It also says nothing about `inf`.

### A related notation outside the documentation

**LABEL: observed.** Cycles' own files also write whole-number maxima in a notation rather than as plain digits, using a bit shift. This is not the Python API documentation and so is not an answer to the question, but it is the same phenomenon and worth having on the record:

> `        min=1, max=(1 << 24),`

That is the maximum of Cycles' render sample count. File: `Blender Sources/cycles_5.1/properties.py`, line 482, inside the `samples: IntProperty(` block that begins at line 479. Date read 2026-07-30. The same `(1 << 24)` appears as the maximum of `preview_samples` (line 490), `sample_offset` (line 503), `sample_subset_length` (line 510) and `preview_denoising_start_sample` (line 470). Nothing in Cycles' files explains the notation either.

### Where I looked, for A

- Counted and grouped every `(in [ … ], default … )` block in the whole of `Blender Sources/api_5.1`, split by the stated type, to establish which notations occur for whole-number settings and how often. *(Citation check, 2026-08-30: this read "all 1,899 files". The folder holds 1898 files at its top level plus a `bpy_types_enum_items` folder holding 206 more — 2104 files in all; 1,899 counts top-level entries rather than files. The count itself covered everything. Corrected by this check.)*
- Searched all of `api_5.1` for `inf` as a standalone word outside range brackets — nothing.
- Searched all of `api_5.1` for `2**31`, `2 ** 31`, `sys.maxsize`, `INT_MAX` — only the two `bpy.props` signatures.
- Searched all of `api_5.1` for "infinity" — three hits, all unrelated (sound loop count, defocus node, radial tiling node).
- Read in full: `bpy.types.IntProperty`, the `IntProperty` and `IntVectorProperty` sections of `bpy.props`, the opening of `info_api_reference`.
- Searched all of `api_5.1` for "soft limit", "hard limit", "soft_min", "soft_max" — hits only in `bpy.ops.wm`, `bpy.props`, `bpy.types.FloatProperty`, `bpy.types.IntProperty` and the generated indexes.
- Searched the whole manual for `inf]` and for `inf` in the glossary — no hits.
- Searched the whole of the release notes for `inf` — one hit, `bugfixes.md`, about EXR brightness, not the notation.
- Ruled out: the manual, because it never uses the notation at all; the release notes, for the same reason.

### Not covered

I have not settled what value `inf` resolves to for a whole-number setting, because the sources do not say. I have not settled whether `inf` marks a genuinely unbounded setting or a setting whose bound the documentation simply could not print. I have not checked notations for settings whose type is not a whole number beyond confirming that scientific notation is float-only.

### Outside the interface

This is a fact about how a documentation page is written, so nothing about it depends on whether Blender is being driven by a person or by something else. The sources say nothing that would make it differ.

### Operating system

Nothing in the API documentation ties the range notation to an operating system, and the pages are not published per operating system. The sources say nothing that would make it differ.

---

## B. What a render does with a value outside what a setting accepts, for a setting whose acceptable values are not in Cycles' own files

### The reading I answered

The question is ambiguous about which setting is meant. Read in the place it sits — question 18, on the sample count — the setting whose acceptable values are *not* in Cycles' own files is a sample count belonging to some other engine, EEVEE's being the case in point: Cycles' files do state their own sample range (`min=1, max=(1 << 24)`, quoted above), while EEVEE's sample count has no range anywhere except the API documentation's `in [1, inf]`. I answer that reading. The other reading is broader — any setting at all whose range comes from Blender itself rather than from the Cycles add-on, `resolution_percentage` among them — and my answer comes out the same for both, so the ambiguity does not change the outcome.

### Answer

**LABEL: not in the sources.**

Nothing in the manual, the API documentation, the release notes, or Cycles' own files states what a render does when a setting whose range is defined outside Cycles is given a value outside that range. There is no statement of clamping, no statement of an error, no statement of a message reported, and no statement of a render refusing to start.

### The near miss, and why it does not answer the question

The sources contain exactly one statement about assigning an out-of-range value to a whole-number setting:

> "max (int) – Hard maximum, trying to assign a value above will silently assign this maximum instead."

> "min (int) – Hard minimum, trying to assign a value below will silently assign this minimum instead."

Source: Python API documentation, `https://docs.blender.org/api/5.1/bpy.props.html` — date read 2026-07-30.

Two reasons this is not the answer to the question asked.

1. It describes the `min` and `max` **arguments of `bpy.props.IntProperty`** — that is, the behaviour of settings *created through* `bpy.props`. Cycles' settings are exactly that kind: `Blender Sources/cycles_5.1/properties.py` defines `samples`, `sample_offset`, `sample_subset_length` and the rest with `IntProperty(...)` (**observed**). So this statement covers the settings that *are* defined in Cycles' own files — the opposite of what the question asks about. Carrying it across to a setting Blender defines internally, such as EEVEE's sample count or the resolution percentage, is a step the sources do not take, and taking it would be inference, not documentation.
2. It is about *assigning* a value to a setting. What a *render* then does is a further step again. The sources do not join the two.

The manual's one statement about value limits is explicitly about typing into a field in the interface, not about a render:

> "Most numerical values are restricted by “soft limit” and “hard limit” value ranges.
> Changing values by dragging with the mouse is restricted to the “soft limit” value range.
> Input via keyboard will allow the use of wider value ranges, but never wider than the “hard limit”."

Source: manual, `https://docs.blender.org/manual/en/5.1/interface/controls/buttons/fields.html` — date read 2026-07-30. It says what the interface will let you enter. It says nothing about what happens if a value gets in by another route, and nothing about the render.

For completeness on the EEVEE side: the manual's EEVEE sampling page gives no range at all. Its whole description of the setting is:

> "Samples
>
> The number of samples to use in the final render."

Source: manual, `https://docs.blender.org/manual/en/5.1/render/eevee/render_settings/sampling.html` — date read 2026-07-30.

### Where I looked, for B

- Searched the manual, the API documentation and the release notes together for: "out of range", "outside the range", "outside of the range", "exceeds the maximum", "above the maximum", "below the minimum", "is clamped", "clamped to", "will be clamped", "capped", "rounded to", "coerced", "rejected", "refuses", "silently".
  - The only hits with any bearing were the `bpy.props` lines quoted above.
  - Every other clamping hit was a different feature entirely and was ruled out on that basis: the FFmpeg CRF value (`render/output/properties/output.html`), Freestyle crease angle modifiers, weight painting, the Map Range node, the Geometry Attribute constraint, geometry-node index inputs, dynamic paint intersection depth, sculpt brush strength.
- Searched the API documentation's `info_*` pages, `bpy.types.bpy_struct` and `bpy.props` for "ValueError", "TypeError", "RuntimeError", "assign a value", "assigning", "not writable" — hits were about operator registration, file-path encoding, and property callbacks, none about numeric range on a render setting.
- Read `bpy.types.SceneEEVEE` for both sample-count settings and `bpy.types.RenderSettings` for the resolution percentage — each gives a range and a default and nothing about out-of-range values.
- Read the full command-line pages: `https://docs.blender.org/manual/en/5.1/advanced/command_line/arguments.html` and `https://docs.blender.org/manual/en/5.1/advanced/command_line/render.html`. Neither mentions out-of-range values, and neither offers an argument that sets a sample count. The one argument that states a range, `-t, --threads`, gives it as "[1-1024], 0 to use the systems processor count" and says nothing about what happens outside it.
- Read the EEVEE sampling manual page in full.
- Searched Cycles' own files. The supplied copy contains the add-on's Python (`__init__.py`, `properties.py`, `ui.py`, `engine.py`, `operators.py`, `presets.py`, `osl.py`, `camera.py`, `version_update.py`), the kernel headers under `source/kernel`, the utility headers under `source/util`, and compiled OSL shaders under `shader`. It does not contain the part of Cycles that reads a scene's settings and starts a render. Whatever validation happens there is not in the files I was given. **This is a limit of the sources, not a finding of absence in Cycles itself.**
- Searched the release notes for 5.1 for any change to value ranges or validation — nothing.

### Not covered

This answer does not say what actually happens; it says the sources do not. It does not settle whether a value outside the range can even be stored on a setting defined by Blender rather than by Cycles, nor whether such a value would be clamped when written, when saved, when loaded, or when the render starts — the sources address none of those moments. It also does not cover engines beyond Cycles and EEVEE.

### Outside the interface

The one manual statement I found on value limits is explicitly about dragging and keyboard entry, which are interface actions; it therefore says nothing about a render driven from outside Blender. The `bpy.props` statement is about assignment in Python and does not mention the interface either way. So: the sources do not say. This gap is exactly where the question bites, and I cannot close it from what I was given.

### Operating system

Nothing found ties any of this to an operating system. The sources do not say.

---

## C. What a denoiser's prefiltering setting and its quality setting do when a different denoiser is in use

### Answer

**LABEL: documented** — for what the manual says, which is about *visibility*, not about what the settings do.
**LABEL: observed** — for what Cycles' own files show.
**LABEL: not in the sources** — for what the two settings actually do when a different denoiser is in use.

The manual describes both settings and, for each, states only that it is visible only when OpenImageDenoise is in use. Exact words:

> "Prefilter OpenImageDenoise
>
> Controls whether prefiltering is applied to the Passes before denoising.
> Visible only when using OpenImageDenoise."

> "Quality OpenImageDenoise
>
> Overall denoising quality.
> Visible only when using OpenImageDenoise."

Source: manual, `https://docs.blender.org/manual/en/5.1/render/cycles/render_settings/sampling.html` — date read 2026-07-30.

That is a statement about what you can see, not about what the render does. The manual never says the settings are ignored, disabled, unset, or reset when OptiX is the denoiser, and it never says they are used.

Cycles' own files carry the same restriction in their own descriptions of the two settings:

> `    denoising_prefilter: EnumProperty(`
> `        name="Denoising Prefilter",`
> `        description="Prefilter noisy guiding (albedo and normal) passes to improve denoising quality when using OpenImageDenoise",`

> `    denoising_quality: EnumProperty(`
> `        name="Denoising Quality",`
> `        description="Overall denoising quality when using OpenImageDenoise",`

File: `Blender Sources/cycles_5.1/properties.py`, lines 414–425. Date read 2026-07-30. The viewport pair carry word-for-word the same qualification: `preview_denoising_prefilter` at lines 449–454 and `preview_denoising_quality` at lines 455–460.

**Observed, from Cycles' files.** Two facts, and they pull in different directions.

First, the panel only draws these two controls when the denoiser is OpenImageDenoise:

> `        col.prop(cscene, "denoising_input_passes", text="Passes")`
> `        if cscene.denoiser == 'OPENIMAGEDENOISE':`
> `            col.prop(cscene, "denoising_prefilter", text="Prefilter")`
> `            col.prop(cscene, "denoising_quality", text="Quality")`

File: `Blender Sources/cycles_5.1/ui.py`, lines 346–349, inside `class CYCLES_RENDER_PT_sampling_render_denoise`'s `draw` method. Date read 2026-07-30.

The viewport panel does the same, but against the denoiser the viewport will actually end up using rather than the setting as stored:

> `        effective_preview_denoiser = get_effective_preview_denoiser(context, has_oidn_gpu)`
> `        if effective_preview_denoiser == 'OPENIMAGEDENOISE':`
> `            col.prop(cscene, "preview_denoising_prefilter", text="Prefilter")`
> `            col.prop(cscene, "preview_denoising_quality", text="Quality")`

File: `Blender Sources/cycles_5.1/ui.py`, lines 275–278. `get_effective_preview_denoiser` is defined at lines 173–186 and resolves the viewport's `AUTO` choice to `OPENIMAGEDENOISE` or `OPTIX` before the comparison is made.

Second, and this is the part that matters: the two settings themselves are defined unconditionally. They are plain `EnumProperty` definitions with fixed item lists (`enum_denoising_prefilter` at lines 312–325, `enum_denoising_quality` at lines 327–340) and fixed defaults (`'ACCURATE'` and `'HIGH'` for the render pair; `'FAST'` and `'BALANCED'` for the viewport pair). Nothing in their definitions depends on which denoiser is chosen. Contrast the `denoiser` setting itself, whose available choices *are* built by a function at run time (`enum_denoiser`, lines 299–303). Only their appearance in the panel is conditional.

**Label for what follows: inferred.** The prefilter and quality settings exist and hold a value whatever the denoiser is. The step: within these files nothing but the panel's `draw` method tests the denoiser, and neither declaration carries an update handler or a re-computed option list, so a setting whose row is not drawn still holds whatever it held. *(Citation check, 2026-08-30: this sentence sat under the "observed" heading. What the files show is the text of the declarations; that the settings hold a value while the program runs is behaviour and a step past that text. Relabelled inferred, with the step written out. Changed by this check.)*

**What the sources do not say.** Neither the manual, nor the API documentation, nor the release notes, nor Cycles' own files state what these two settings do — or fail to do — during a render when OptiX is the denoiser. The whole of what is on the record is: their descriptions qualify them with "when using OpenImageDenoise", and the panel hides them otherwise.

**The step I did not take, and why.** It is tempting to read "hidden in the panel" as "ignored by the render". That step is not in the sources. The code quoted above is a panel's `draw` method — it decides what a panel shows. The part of Cycles that reads these settings and hands them to a denoiser is not in the supplied files at all (see the file inventory under B). Anything about what the render does with them is therefore beyond what I can support. If forced to label an inference, the most I could say is: *inferred* — a setting whose description is qualified "when using OpenImageDenoise" is unlikely to be consulted by a different denoiser; the step being that a description of intent is not a statement of behaviour. I record that as inference and do not offer it as a finding.

### Where I looked, for C

- Searched the whole manual for "prefilter" — only two files contain it: `render/cycles/render_settings/sampling.txt` and `compositing/types/filter/denoise.txt`. Read the denoising section of the first in full (its lines 54–160).
- Searched the whole manual for "OptiX" — 20 lines across 10 files, read each. *(Citation check, 2026-08-30: this read "17 hits". Neither the line count nor the file count is 17. What the line is used for is unchanged and confirmed: none of those lines describes what prefilter or quality do under OptiX. Corrected by this check.)* The denoiser-related ones are on the sampling page (the Denoiser choices) and `render/cycles/gpu_rendering.html`. None describes what prefilter or quality do under OptiX.
- Searched the whole API documentation for `denoising_prefilter` and `denoising_quality` — **no hits at all**. Cycles' scene settings do not appear in the 5.1 Python API documentation; the only Cycles page there is `bpy.ops.cycles`. So the API documentation cannot answer this question, and I ruled it out on that basis.
- Searched the whole of the 5.1 release notes for "denois" — four hits: two bug-fix lines (`bugfixes.md` lines 308 and 511), one corrective-release line about a Denoise Animation operator (`corrective_releases.md` line 52), and one line in `cycles.md`: "Improved denoising quality with textured transparency and roughness." None addresses the interaction asked about. Searched the release notes for "optix" — no hits.
- Searched all of Cycles' own files for "prefilter" and read every hit: `properties.py` (the enum and the four property definitions), `presets.py` (lines 58 and 83, which list the settings among those a sampling preset stores), and `ui.py` (the two panel conditions). Nothing in `engine.py`, the kernel headers or the utility headers mentions it.
- Read `ui.py` lines 145–200 (`show_denoise_active`, `show_preview_denoise_active`, `get_effective_preview_denoiser`) and lines 246–356 (both Denoise panels) in full.
- Checked the manual's sampling page for a screenshot of the Denoise panel — there is none; the page contains no images at all. I also checked the manual's `_images` folder for any denoise-panel screenshot and found none. The four denoise pictures it holds are `compositing_types_filter_denoise_example.jpg` and `node-types_CompositorNodeDenoise.webp`, both belonging to the compositor's Denoise node rather than to the render settings, and `render_layers_denoising_example1.jpg` and `render_layers_denoising_example2.jpg`, which are before-and-after example renders. *(Citation check, 2026-08-30: this named two of the four and omitted the two example renders. The conclusion — no picture of the Denoise panel — is unchanged and confirmed. Corrected by this check.)* So no picture can settle what the panel offers under OptiX.
- Ruled out as a different thing: the compositor's Denoise node (`https://docs.blender.org/manual/en/5.1/compositing/types/filter/denoise.html`), which has its own Prefilter and Quality controls, including a Quality option described as "Use the scene’s quality setting." That node is not a scene or view-layer denoising setting, and the question is about the latter, so I did not treat it as an answer. It is noted here only so nobody mistakes it for one later.

### Not covered

This answer does not settle what a render actually does with the prefilter and quality settings when OptiX is the denoiser — whether it reads them and ignores them, never reads them, or does something else. It does not cover what happens when the viewport denoiser is left on `Automatic` and resolves to OptiX during an actual render. It does not cover the compositor's Denoise node, which carries its own separate Prefilter and Quality. It does not cover whether the stored values survive a save and reload while OptiX is selected — nothing in the sources addresses that, though the settings' unconditional definition in `properties.py` and their presence in `presets.py` are the only facts nearby.

### Outside the interface

The manual's statement is about visibility, which is an interface matter and says nothing about a render driven from outside Blender. The Cycles code that acts on the denoiser choice is a panel `draw` method, which is also an interface matter. So for the question that actually matters here — what a render started from outside Blender does with these two settings — the sources are silent, and neither the manual's "Visible only when…" nor the panel condition may be carried over to it.

### Operating system

One thing on the record does depend on the operating system, and it bears on which denoiser can be in use at all:

> "OptiX is supported on Windows and Linux and requires a NVIDIA graphics cards with compute capability 5.0 and higher
> and a driver version of at least 535."

Source: manual, `https://docs.blender.org/manual/en/5.1/render/cycles/gpu_rendering.html` — date read 2026-07-30.

Cycles' own files add that the OptiX choice is offered only where a suitable device is configured: `enum_optix_denoiser` in `Blender Sources/cycles_5.1/properties.py`, lines 273–277, returns the OptiX item when `get_devices_for_type('OPTIX')` finds something — and also when there is no context at all, the test being `if not context or bool(…)`. Its own description reads *(Citation check, 2026-08-30: this said the item is returned "only when `get_devices_for_type('OPTIX')` finds something", which is one condition short of what the file says. Corrected by this check.)* "only available on NVIDIA GPUs when configured in the system tab in the user preferences" (**observed**). Beyond that, the sources say nothing about the prefilter and quality settings differing by operating system.

---

## Things a person with Blender open could settle by hand

These are written for somebody who cannot see any of the reasoning above. Each says what to open, what to do, and what to look at.

1. **What `inf` means for a whole-number setting.** Open Blender 5.1. Open the Scripting workspace and use the Python console. Type `bpy.types.SceneEEVEE.bl_rna.properties['taa_render_samples'].hard_max` and press Return. Write down the number shown. Do the same for `.hard_min`, `.soft_max` and `.soft_min`. Then do the same four for `bpy.types.RenderSettings.bl_rna.properties['resolution_percentage']`, whose maximum the documentation prints as a plain 32767, so you have a case where the printed number and the reported number can be compared. This settles what number the documentation is standing in for when it prints `inf`.

2. **What happens when a sample count is set above its maximum.** In the Python console, with EEVEE selected as the render engine, type `bpy.context.scene.eevee.taa_render_samples = 999999999` and press Return. Note whether an error appears in red. Then type `bpy.context.scene.eevee.taa_render_samples` and press Return, and write down the number that comes back. Repeat with a value of `0` and with `-5`. Repeat the whole exercise for `bpy.context.scene.render.resolution_percentage` using `99999`, `0` and `-5`.

3. **The same, but for a Cycles setting, for comparison.** Switch the render engine to Cycles. In the Python console type `bpy.context.scene.cycles.samples = 999999999`, press Return, then type `bpy.context.scene.cycles.samples` and write down what comes back. The number to watch for is 16777216. Also try `0` and `-1` and record what comes back and whether an error is shown.

4. **Whether an out-of-range value survives to a render.** Do step 2 or 3, then actually render one frame (F12) and watch the status bar and the console window for any message about the sample count. Record whether the render starts, whether it completes, and whether anything is reported. Then repeat the same from outside Blender: save the file, and run Blender from a terminal in background mode rendering one frame, having set the value with a `--python-expr` argument beforehand. Compare what is printed to the terminal with what the interface showed.

5. **Whether the denoiser's Prefilter and Quality settings keep their values under OptiX.** On a machine with an NVIDIA card where OptiX is available: Render Properties → Sampling → Render → Denoise. Set Denoiser to OpenImageDenoise, set Prefilter to None and Quality to Fast. Now set Denoiser to OptiX. Confirm the Prefilter and Quality controls disappear from the panel. Then, in the Python console, type `bpy.context.scene.cycles.denoising_prefilter` and `bpy.context.scene.cycles.denoising_quality` and write down what comes back. Then switch Denoiser back to OpenImageDenoise and check whether the panel shows None and Fast again. This settles whether the values are merely hidden or actually changed.

6. **Whether those two settings can still be changed while OptiX is selected.** With Denoiser set to OptiX, type `bpy.context.scene.cycles.denoising_prefilter = 'NONE'` in the Python console and press Return. Note whether an error appears. Then read the value back. This settles whether the restriction is only in the panel or also in the setting itself.

7. **Whether they have any effect on an OptiX render.** With Denoiser set to OptiX, render the same frame twice at a low sample count — once with `denoising_prefilter` set to `'NONE'` and `denoising_quality` to `'FAST'`, once with `'ACCURATE'` and `'HIGH'` — setting them through the Python console each time since the panel will not show them. Save both images and compare them, and compare the render times reported. Identical images and identical times would indicate the settings are not consulted; differences would indicate they are.

8. **What a save and reload does.** With Denoiser set to OptiX and Prefilter and Quality set to non-default values through the Python console, save the file, quit Blender, reopen the file, and read both values back in the Python console. This settles whether the values survive being written to a file while a denoiser that does not show them is selected.

---

2026-08-30 — Claude Opus 5 (Claude Code) — Settings, Blender 5.1

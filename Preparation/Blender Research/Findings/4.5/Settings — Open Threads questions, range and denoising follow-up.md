# Settings — Open Threads questions, range and denoising follow-up (Blender 4.5)

**Sources consulted, and on what date.** Blender 4.5's manual, Blender 4.5's Python API documentation, Blender 4.5's release notes, and Cycles' own files as supplied — all from `Blender Sources/`, all taken on **2026-07-30**, which is the date read for every citation below. I also opened one picture from the manual's `_images` folder (named in point C).

**I read only the question sheet and files inside `Blender Sources/`. I did not open any other part of this project.**

One thing disclosed in full, so the line above can be trusted: to check that the folder I was told to save into existed, I ran a command that returned a *count* of the entries in `Preparation/Blender Research/Findings/4.5/` and printed only the number 91. No file name was displayed and no file there was opened. If that is judged to cross the line, the line above should be read with this exception attached.

This pass answers only points A, B and C, which sit inside questions 18 and 19 of the question sheet. It does not answer questions 18 and 19 in full.

---

## A. The notation the Python API documentation uses for a maximum

**LABEL: documented** (that the notation is used and what it is) / **not in the sources** (what it stands for).

### The notation

The notation is the bare word **`inf`**, written in place of a number inside the square-bracket range that the API documentation gives for a property's type.

Three examples, all of them whole-number settings, all of them sample counts:

> `taa_render_samples`
>
> `Number of samples per pixel for rendering`
>
> `Type:`
>
> `int in [1, inf], default 64`

Python API documentation, `bpy.types.SceneEEVEE`, lines 316–322.
Address: https://docs.blender.org/api/4.5/bpy.types.SceneEEVEE.html
Date read: 2026-07-30.

> `taa_samples`
>
> `Number of samples, unlimited if 0`
>
> `Type:`
>
> `int in [0, inf], default 16`

Same page, lines 324–330. Same address, same date.

> `aa_samples`
>
> `Number of supersampling anti-aliasing samples per pixel for final render`
>
> `Type:`
>
> `int in [1, inf], default 8`

Python API documentation, `bpy.types.SceneGpencil`, lines 9–15.
Address: https://docs.blender.org/api/4.5/bpy.types.SceneGpencil.html
Date read: 2026-07-30.

The same word is used at the bottom end of a range too, as `-inf`.

### How widely it is used, and that it is never explained

I counted every bracketed range in the 4.5 API documentation that contains the word. The commonest forms, with the number of times each appears across the whole of `api_4.5`:

| Range as written | Times it appears |
| --- | --- |
| `in [0, inf]` | 1828 |
| `in [-inf, inf]` | 1783 |
| `in [1, inf]` | 62 |
| `in [1e-06, inf]` | 38 |
| `in [-1, inf]` | 29 |

I then searched the whole of the 4.5 API documentation for the standalone word `inf` and stripped out every line where it sits inside a bracketed range. **Nothing was left — zero lines.** In other words, in Blender 4.5's Python API documentation the word `inf` appears only ever as part of a range, and is never defined, explained, expanded or given a meaning anywhere.

I also searched the 4.5 manual for the standalone word `inf`: **zero hits**, of any kind. And the 4.5 release notes: **zero hits**.

**So what it stands for is not in the sources.** The documentation never says.

### The nearest the sources come to giving it a meaning

Two things sit close, and I record both because they are the only material there is. Neither is a definition.

**First.** The documentation for the function scripts use to define a whole-number setting states its own default limits in a second notation, not as plain numbers:

> `bpy.props.IntProperty(*, name='', description='', translation_context='*', default=0, min=-2**31, max=2**31 - 1, soft_min=-2**31, soft_max=2**31 - 1, step=1, options={'ANIMATABLE'}, override=set(), tags=set(), subtype='NONE', update=None, get=None, set=None)`

Python API documentation, `bpy.props`, line 510 (`IntVectorProperty` at line 550 is the same).
Address: https://docs.blender.org/api/4.5/bpy.props.html
Date read: 2026-07-30.

If the question means this second notation rather than `inf`, then the answer is `2**31 - 1`, which the sources likewise never spell out in words. I read the question as asking about `inf`, because `inf` is what is written in place of a maximum on the settings pages themselves, whereas `2**31 - 1` appears only in the signature of the property-defining function. I flag the ambiguity rather than choosing silently.

**Second.** One operator in the documentation has a whole-number argument whose stated default is a specific large number while its stated range uses `inf`:

> `... export_animation=False, start_frame=-2147483648, end_frame=2147483647, ...`

and, further down the same page, the description of that same argument:

> `start_frame (int in [-inf, inf], (optional)) – Start Frame, The first frame to be exported`

Python API documentation, `bpy.ops.wm`, `obj_export` at line 1747 and line 1812.
Address: https://docs.blender.org/api/4.5/bpy.ops.wm.html
Date read: 2026-07-30.

**LABEL for what follows: inferred.** The step: a stated default must be a value the setting accepts; this argument's default is 2147483647 and its stated maximum is `inf`; therefore in a whole-number range `inf` stands for something no smaller than 2147483647. That is all the sources support. They do not say it equals 2147483647, they do not say it means "no limit", and they do not say it means "infinity". A reader has to supply every one of those, so none of them is documented.

I searched for, and did not find, any legend or key page that would settle it. Specifically: `info_api_reference.txt` (the page called "API Reference Usage", which explains how to read the reference — https://docs.blender.org/api/4.5/info_api_reference.html) contains no explanation of the range notation; `index.txt` (the API's own front page) contains none; `bpy.types.IntProperty` describes `hard_max`, `hard_min`, `soft_max`, `soft_min` only as "Maximum value used by buttons" and "Minimum value used by buttons" and gives no notation key; `bpy.types.Property` carries none of those four attributes at all, so it gives no key either. *(Citation check, 2026-08-30: this sentence attributed those descriptions to `bpy.types.Property` as well as to `bpy.types.IntProperty`. They are on `bpy.types.IntProperty` only. The conclusion — that neither page carries a key to the notation — is unchanged. Corrected by this check.)*

### Where I looked, and what I ruled out

- Searched the whole of `api_4.5` for the standalone word `inf`; tabulated every bracketed range containing it; confirmed no occurrence outside a range. *(Citation check, 2026-08-30: this line read "all 1851 files of `api_4.5`". The folder holds 1850 files at its top level plus a `bpy_types_enum_items` folder holding 199 more — 2049 files in all; 1851 counts top-level entries rather than files. The search itself covered everything, and the five counts in the table above all reproduce exactly. Corrected by this check.)*
- Searched all of `api_4.5`, `manual_4.5` and `release_notes_4.5` for: `infinity`, `Infinity`, `INFINITY`, `infinit`, `notation`, `unbounded`, `no upper limit`, `hard limit`, `hard-coded limit`, `2**31`, `2**63`, `maxsize`, `INT_MAX`, `2147483647`. The only hits of any relevance are the ones quoted above. `infinity` occurs twice in the API documentation and in neither case is it about a range notation (a sound loop count in `aud`, and the Defocus node's "128 (infinity) is perfect focus").
- Read in full: `info_api_reference.txt`, the head of `index.txt`, the head of `bpy.props.txt`, `bpy.types.Property.txt`, `bpy.types.IntProperty.txt`. Ruled out: none of them carries a key to the notation.
- Ruled out the manual and the release notes entirely for this point, by search: the word does not occur in either.

**Not covered.** This does not settle what number, if any, `inf` actually stands for, nor whether it means the same thing in a whole-number range as it does in a decimal one, nor whether it means the same thing everywhere it appears. It does not settle what happens if you assign a value larger than whatever `inf` stands for — see point B. It says nothing about Cycles' own sample count, whose maximum is written as a plain expression in Cycles' files rather than as `inf` (see B).

**Outside the interface.** The API documentation is documentation of the Python interface, which is the interface used when Blender is driven from outside its own window. The notation is a property of the documentation, not of a running Blender, so it reads the same either way. The sources do not say whether the underlying limit behaves differently when Blender is driven from outside; nothing I found addresses that.

**Operating system.** The sources say nothing about the notation, or the limit behind it, differing by operating system. I found no operating-system qualification anywhere near any of the quoted material.

---

## B. What a render does with a value outside what the setting accepts, where Cycles' files do not define that setting's range

**LABEL: not in the sources.**

### How I read the question

Cycles' own files *do* define the acceptable values of Cycles' sample count:

> `    samples: IntProperty(`
> `        name="Samples",`
> `        description="Number of samples to render for each pixel",`
> `        min=1, max=(1 << 24),`
> `        default=4096,`
> `    )`

Cycles' own files, `properties.py`, lines 482–487. **Label for that quote alone: observed.**

So the setting whose acceptable values are *not* defined in Cycles' files, in the neighbourhood of question 18, is the sample count of an engine that is not Cycles. In 4.5's API documentation those are `bpy.types.SceneEEVEE.taa_render_samples` (`int in [1, inf]`), `bpy.types.SceneEEVEE.taa_samples` (`int in [0, inf]`) and `bpy.types.SceneGpencil.aa_samples` (`int in [1, inf]`), all quoted in point A. I confirmed by search that no file in `cycles_4.5` mentions `taa_` at all — zero hits — so these ranges genuinely live outside Cycles' files.

The question is ambiguous about which setting is meant. The other reading is that it means any setting at all whose range the API documentation writes with `inf` rather than a plain number, Cycles' files being silent on it. My answer below is the same under either reading, because the sources are silent in the same way for both.

### The answer

**Nothing in the 4.5 manual, the 4.5 Python API documentation, the 4.5 release notes, or Cycles' supplied files states what a render does when one of these settings is given a value outside what it accepts.** No statement about an error, a warning, a message, a refusal, a substituted value, or a failed render.

The two closest statements in the sources are these, and neither is about a render.

**First — and this is the only statement anywhere in the sources about assigning an out-of-range value:**

> `min (int) – Hard minimum, trying to assign a value below will silently assign this minimum instead.`
>
> `max (int) – Hard maximum, trying to assign a value above will silently assign this maximum instead.`

Python API documentation, `bpy.props`, lines 522 and 524 (the `IntProperty` entry; the identical wording appears for `IntVectorProperty` at 564/566 and, with `float`, for `FloatProperty` at 430/432 and `FloatVectorProperty` at 476/478).
Address: https://docs.blender.org/api/4.5/bpy.props.html
Date read: 2026-07-30.

Read that carefully. It is on the page for the function a **script** uses to **define a new** whole-number setting, and it describes what happens for a setting so defined. `taa_render_samples` is not a setting defined that way — it is one of Blender's built-in settings, and the page that documents it says nothing of the kind. **To carry this sentence over to a built-in setting is a step the sources do not take**, so if anyone states it as fact about `taa_render_samples`, the label is inferred, not documented, and the step is: the wording appears on the general property-definition page, so it may describe how all whole-number settings behave; the sources do not say that it does. I am not making that claim here; I am recording that it is the only nearby wording and that it does not reach.

**Second**, the manual describes what limits do to a person typing into a field:

> `Most numerical values are restricted by “soft limit” and “hard limit” value ranges.`
> `Changing values by dragging with the mouse is restricted to the “soft limit” value range.`
> `Input via keyboard will allow the use of wider value ranges, but never wider than the “hard limit”.`

Manual, "Buttons ‣ Fields", the "Value Limits" section, lines 69–71.
Address: https://docs.blender.org/manual/en/4.5/interface/controls/buttons/fields.html
Date read: 2026-07-30.

This is explicitly about dragging with a mouse and typing at a keyboard. It is a statement about what the interface will let a person enter, not about what a render does with a value that arrived some other way.

### Where I looked, and what I ruled out

- The manual's Cycles Sampling page in full (https://docs.blender.org/manual/en/4.5/render/cycles/render_settings/sampling.html). It describes "Render (Max) Samples" as "The number of paths to trace per pixel in the final render" and gives **no** acceptable range and **no** out-of-range behaviour. Ruled out.
- The manual's EEVEE Sampling page in full (https://docs.blender.org/manual/en/4.5/render/eevee/render_settings/sampling.html). "Samples — The number of samples to use in the final render." No range, no out-of-range behaviour. Ruled out.
- `bpy.types.SceneEEVEE`, `bpy.types.SceneGpencil`, `bpy.types.SceneDisplay`, `bpy.types.RenderSettings` in the API documentation. They state ranges; none states what happens outside them. Ruled out.
- The manual's command-line arguments page (https://docs.blender.org/manual/en/4.5/advanced/command_line/arguments.html). I listed every option it documents. There is **no** command-line option for a sample count in 4.5 — the render-related options are `-b`, `-a`, `-S`, `-f`, `-s`, `-e`, `-j`, `-o`, `-E`, `-t`, `-F`, `-x`, `--cycles-device`, `--cycles-print-stats`. So there is no command-line option whose out-of-range handling could have been documented. Ruled out.
- Searched all of `api_4.5` and `manual_4.5` for: `silently`, `clamp`, `clamped`, `capped`, `is capped`, `out of range`, `outside the range`, `outside the limit`, `beyond the limit`, `exceed`, `exceeds the`, `hard limit`, `soft limit`, `hard maximum`, `hard minimum`, `hard_max`, `hard_min`, `invalid value`, `rejected`, `truncated to`, `will be ignored if`, `ValueError`, `value error`, `at least 1`, `minimum of 1`, `will not go below`, `falls back to`. Everything returned was either unrelated (node sockets, bevel overlap, cloth impulses, audio distance models, rigid-body springs) or is quoted above.
- Searched the whole of `release_notes_4.5` for `samples`, `denois`, `range` and `limit`. Nothing about out-of-range handling. Ruled out.
- Checked `info_gotchas_*` in the API documentation. The nearest thing is about names, not numbers: "Data names may not match the assigned values if they exceed the maximum length, are already used or an empty string" (`info_gotchas_internal_data_and_python_objects.txt`, line 44). Different subject. Ruled out.
- Searched all of `cycles_4.5` — the add-on's Python plus the kernel and utility headers — for any handling of an out-of-range sample count. Nothing. The files define ranges; they contain no code that reports or reacts to a value outside one.

**Not covered.** This does not settle what actually happens — only that the sources do not say. It does not settle whether a value outside the range is silently changed, refused, or passed through. It does not settle whether anything is printed when it happens. It says nothing about Cycles' own sample count, whose range *is* in Cycles' files and which the question excludes.

**Outside the interface.** This matters here, and the sources leave it open. The one manual statement I found (soft and hard limits) is explicitly about mouse dragging and keyboard typing, so it does not carry to a Blender driven from outside its own window at all. The one API statement I found (silently assign the limit instead) is about assigning in Python, which *is* how a setting would be given a value from outside — but it is on the page for script-defined settings, not for the built-in ones at issue. So: the sources do not say what happens outside the interface, and the only wording that plausibly bears on it does not cover the settings in question.

**Operating system.** The sources say nothing about this differing by operating system. Nothing I found carries an operating-system qualification.

---

## C. What the prefiltering setting and the quality setting do when a different denoiser is in use

**LABEL: documented** (that they are visible only under one denoiser) / **observed** (that the interface draws them only under one denoiser, and that they exist and keep their value regardless) / **not in the sources** (what the render itself does with them under a different denoiser).

### What the manual says

The manual gives these two settings names that carry the denoiser's name in the heading itself, and states plainly when each is shown:

> `Prefilter OpenImageDenoise`
>
> `Controls whether prefiltering is applied to the Passes before denoising.`
> `Visible only when using OpenImageDenoise.`

Manual, Cycles ‣ Render Settings ‣ Sampling, lines 111–114.
Address: https://docs.blender.org/manual/en/4.5/render/cycles/render_settings/sampling.html
Date read: 2026-07-30.

> `Quality OpenImageDenoise`
>
> `Overall denoising quality.`
> `Visible only when using OpenImageDenoise.`

Same page, lines 132–135. Same address, same date.

The same page lists the denoisers a person can choose between:

> `Automatic:`
>
> `Uses GPU accelerated denoising if supported, for best performance.`
> `Prefers OpenImageDenoise over OptiX.`
>
> `OpenImageDenoise:`
>
> `Uses Intel’s Open Image Denoise,`
> `an AI denoiser. Typically provides the highest quality.`
>
> `OptiX:`
>
> `Uses NVIDIA’s OptiX AI denoiser.`
> `Supports GPU acceleration on some older NVIDIA GPUs where OpenImageDenoise does not.`

Same page, lines 75–87. Same address, same date. *(Citation check, 2026-08-30: the citation read "lines 74–85"; the quoted block runs 75–87. The quotation also stopped one sentence short of the page's OptiX entry, and that sentence has been added. Nothing in the answer turns on it. Corrected by this check.)*

**So what the manual states is a fact about visibility: with OptiX in use, these two settings are not shown.** It does not state that they stop having an effect, and it does not state that they keep one. That distinction is the whole of this point, and the manual does not cross it.

### What Cycles' own files show

**Label: observed.** The two settings exist as ordinary settings with fixed lists of choices, and nothing in their definition ties them to a denoiser:

> `    denoising_prefilter: EnumProperty(`
> `        name="Denoising Prefilter",`
> `        description="Prefilter noisy guiding (albedo and normal) passes to improve denoising quality when using OpenImageDenoise",`
> `        items=enum_denoising_prefilter,`
> `        default='ACCURATE',`
> `    )`
> `    denoising_quality: EnumProperty(`
> `        name="Denoising Quality",`
> `        description="Overall denoising quality when using OpenImageDenoise",`
> `        items=enum_denoising_quality,`
> `        default='HIGH',`
> `    )`

Cycles' own files, `properties.py`, lines 417–428.

The viewport pair is defined the same way, with the same two descriptions and different defaults (`FAST` and `BALANCED`):

> `    preview_denoising_prefilter: EnumProperty(`
> `        name="Viewport Denoising Prefilter",`
> `        description="Prefilter noisy guiding (albedo and normal) passes to improve denoising quality when using OpenImageDenoise",`
> `        items=enum_denoising_prefilter,`
> `        default='FAST',`
> `    )`
> `    preview_denoising_quality: EnumProperty(`
> `        name="Viewport Denoising Quality",`
> `        description="Overall denoising quality when using OpenImageDenoise",`
> `        items=enum_denoising_quality,`
> `        default='BALANCED',`
> `    )`

Cycles' own files, `properties.py`, lines 452–463.

Note what those descriptions do and do not say. Each says what the setting does **"when using OpenImageDenoise"**. Neither says what it does otherwise. There is no condition on the setting itself, no restriction on when it may be changed, and no alternative list of choices for another denoiser.

The condition is in the panel-drawing code only:

> `        col.prop(cscene, "denoising_input_passes", text="Passes")`
> `        if cscene.denoiser == 'OPENIMAGEDENOISE':`
> `            col.prop(cscene, "denoising_prefilter", text="Prefilter")`
> `            col.prop(cscene, "denoising_quality", text="Quality")`

Cycles' own files, `ui.py`, lines 346–349, inside the final-render Denoise panel (`CYCLES_RENDER_PT_sampling_render_denoise`).

One detail worth having, also observed. The final-render panel tests the chosen denoiser exactly (`cscene.denoiser == 'OPENIMAGEDENOISE'`), so when the denoiser is left on Automatic the two rows are not drawn there. The viewport panel does something different — it works out which denoiser Automatic resolves to first:

> `        effective_preview_denoiser = get_effective_preview_denoiser(context, has_oidn_gpu)`
> `        if effective_preview_denoiser == 'OPENIMAGEDENOISE':`
> `            col.prop(cscene, "preview_denoising_prefilter", text="Prefilter")`
> `            col.prop(cscene, "preview_denoising_quality", text="Quality")`

Cycles' own files, `ui.py`, lines 275–278, with `get_effective_preview_denoiser` at lines 173–186.

Both settings are also recorded in Cycles' sampling presets, unconditionally — `"cycles.denoising_prefilter"` and `"cycles.denoising_quality"` at `presets.py` lines 58–59, and the viewport pair at lines 83–84 — with no test of which denoiser is chosen.

**Label: inferred**, and here is the step. The settings are defined with no restriction, are saved into presets with no restriction, and only the drawing of the panel is conditional. Therefore a setting whose panel row is hidden still exists and still holds whatever value it holds. What I cannot get from these files is whether a render *reads* that value when OptiX is the denoiser. A reader has to supply that, so it is not documented and not observed.

### What is not in the sources

**Nothing in any of the four sources states what a render does with the prefiltering setting or the quality setting when a denoiser other than OpenImageDenoise is in use.** Not that they are ignored, not that they still apply, not that they are reset, not that anything is reported.

And the part of Cycles that would settle it is not in the copy I was given. The supplied `cycles_4.5` folder holds the add-on's Python (`__init__.py`, `camera.py`, `engine.py`, `operators.py`, `osl.py`, `presets.py`, `properties.py`, `ui.py`, `version_update.py`), a `shader` folder, and `source/` containing only `kernel/` and `util/` — 262 files, of which 253 are headers, alongside two `.cpp` files, six `.cu` files and one `.metal` file. *(Citation check, 2026-08-30: this read "262 files, all headers". The 262 count is right and so is the absence it is used for — I checked the folder by looking and there is no denoiser implementation and no session or render-start code — but the files are not all headers. Corrected by this check.)* I searched all of it for `prefilter` and for `denoising_quality`: **every hit is in `properties.py`, `ui.py` or `presets.py`**, all quoted above. There is no denoiser implementation here, no code that hands settings to OpenImageDenoise or to OptiX. **I record that as a limit of the sources rather than guessing.**

### Where I looked, and what I ruled out

- Searched the whole 4.5 manual for `Prefilter`: exactly two pages contain it — the Cycles Sampling page (quoted above) and the compositor's Denoise node page. Searched for `OpenImageDenoise`: hits on the Cycles Sampling page, the Cycles GPU Rendering page, and one line of the version history page. Read the Cycles Sampling denoising section in full.
- Read the compositor Denoise node page in full (https://docs.blender.org/manual/en/4.5/compositing/types/filter/denoise.html). It is a different thing — a node, not the scene's denoiser — and the manual says of it: "It uses Open Image Denoise". Its Quality has an option the scene setting does not: "Follow Scene: Use the scene's quality setting." I record it as adjacent, not as an answer, because the node has no choice of denoiser and so cannot say what happens under a different one.
- Opened the picture `_images/node-types_CompositorNodeDenoise.webp`, named on that same Denoise node page. **Label for what I take from it: documented**, since it is part of the manual. It shows a node headed "Denoise" with a Prefilter menu reading "Accurate" and a Quality menu reading "Follow Scene", above Image, Normal, Albedo and a ticked HDR. It shows nothing about a different denoiser being in use.
- Looked for a picture of the Cycles Sampling ‣ Denoise panel. There is none: the Cycles Sampling page contains no `[image: ...]` marker at all, and the manual's `_images` folder holds no screenshot of that panel. The only denoising pictures in it are four: `node-types_CompositorNodeDenoise.webp` (the compositor node above), `compositing_types_filter_denoise_example.jpg`, and the two before/after example renders `render_layers_denoising_example1.jpg` and `render_layers_denoising_example2.jpg`. *(Citation check, 2026-08-30: this list named three of the four and omitted `compositing_types_filter_denoise_example.jpg`. The conclusion — no screenshot of the Denoise panel — is unchanged and confirmed. Corrected by this check.)*
- Read the Cycles "Reducing Noise" page's Denoising section. It says only to enable Denoising in the Render tab. Nothing about prefiltering or quality. Ruled out.
- Searched the whole of `release_notes_4.5` for `denois`, `prefilter` and `quality`. Three hits, none relevant: the compositor's Denoise node gaining GPU support, a bug fix about the denoiser on GPU, and Grease Pencil line quality. Ruled out.
- Searched the API documentation's change log for `denois`: one hit, `bpy.types.RenderSettings.compositor_denoise_device` under "Added". Not about these two settings. Ruled out.
- Cycles' settings do not appear in the Python API documentation at all (they are an add-on's), so there is no API page for `denoising_prefilter` or `denoising_quality` to check.

**Not covered.** This does not settle whether a render actually ignores these two settings when OptiX is the denoiser — only that the sources state they are not *shown*, and that the code which would decide the rest is not in the supplied files. It does not settle what happens when the denoiser is left on Automatic and Automatic resolves to OpenImageDenoise for a final render — the manual does not address it, and the panel code shows only that the rows are not drawn in that case. It does not cover the compositor's Denoise node, which is a separate thing.

**Outside the interface.** This matters a great deal here, and the sources fall short of it. Everything the sources state — the manual's "Visible only when using OpenImageDenoise", and the panel-drawing condition in `ui.py` — is about **what is drawn on screen**. A Blender driven from outside its own window draws no panel, so none of it applies. The settings themselves are ordinary settings and can be set from outside regardless of the chosen denoiser; whether a render then uses them, the sources do not say. **An answer that a render ignores them under OptiX cannot be supported from these sources.**

**Operating system.** The sources say nothing about these two settings differing by operating system. One neighbouring fact from the same manual page does depend on hardware rather than operating system: OptiX is "Only available on NVIDIA GPUs when configured in the Cycles Render Device user preferences" (Cycles Sampling page, line 88 area, same address and date). That constrains which denoiser can be chosen, not what these two settings do.

---

## Things a person with Blender 4.5 open could settle by hand in a few minutes

Each of these settles something the sources do not.

1. **What `inf` stands for as a maximum (point A).** Open Blender 4.5. In the Scripting workspace's Python Console, with the render engine set to EEVEE, type `bpy.context.scene.eevee.bl_rna.properties['taa_render_samples'].hard_max` and press Return. Write down the number shown. Do the same with `hard_min`, and with `soft_max` and `soft_min`. Repeat for `bpy.context.scene.grease_pencil_settings.bl_rna.properties['aa_samples']`. This turns `inf` into an actual number.

2. **Whether the number in step 1 is the same for a decimal setting.** In the same console, type `bpy.context.scene.eevee.bl_rna.properties['light_threshold'].hard_max` and write down what is shown. This tells you whether `inf` means one thing for whole numbers and another for decimals.

3. **What happens when a sample count is set too high (point B).** With EEVEE as the render engine, in the Python Console type `bpy.context.scene.eevee.taa_render_samples = 99999999999`, press Return, then on the next line type `bpy.context.scene.eevee.taa_render_samples` and press Return. Write down (a) whether anything red appeared, (b) what value is reported back. Then repeat with `= 0` and with `= -5`. Note for each whether the value was changed silently, refused with a message, or accepted.

4. **The same, from outside Blender (point B, the part that matters).** From a terminal, run Blender with no window: `blender -b yourfile.blend --python-expr "import bpy; bpy.context.scene.eevee.taa_render_samples = 99999999999; print('GOT', bpy.context.scene.eevee.taa_render_samples)" -f 1`. Write down the whole of what is printed — the `GOT` line, any error text, whether the frame rendered, and the exit code (`echo $?` on macOS or Linux, `echo %ERRORLEVEL%` on Windows). Repeat with `= 0` and with `= -5`. This is the one that the sources cannot answer at all.

5. **Whether the prefilter and quality settings survive switching denoiser (point C).** Set the render engine to Cycles. In Render Properties ‣ Sampling ‣ Render ‣ Denoise, set Denoiser to OpenImageDenoise, set Prefilter to `None` and Quality to `Fast`. Now change Denoiser to OptiX — the two rows should disappear. In the Python Console type `bpy.context.scene.cycles.denoising_prefilter` and `bpy.context.scene.cycles.denoising_quality` and write down what each reports. Then change Denoiser back to OpenImageDenoise and check whether the panel still shows `None` and `Fast`. This settles whether the values are kept or reset.

6. **Whether they can still be changed while OptiX is chosen (point C).** With Denoiser set to OptiX, in the Python Console type `bpy.context.scene.cycles.denoising_prefilter = 'ACCURATE'` and press Return. Write down whether it is accepted or refused, then read the value back.

7. **Whether the rows appear under Automatic (point C).** With Cycles as the engine and an NVIDIA card enabled in Preferences ‣ System ‣ Cycles Render Devices, set Denoiser to Automatic in Render Properties ‣ Sampling ‣ Render ‣ Denoise. Write down whether Prefilter and Quality rows are shown in that panel. Then look at Render Properties ‣ Sampling ‣ Viewport ‣ Denoise with its denoiser also on Automatic, and write down whether the rows are shown there. Note any difference between the two panels.

8. **Whether the prefilter setting changes the rendered image under OptiX (point C, the real question).** With Cycles, a noisy scene at a low sample count, Denoiser set to OptiX, and Denoising Data passes enabled: render once with `bpy.context.scene.cycles.denoising_prefilter = 'NONE'` set from the Python Console, save the image; render again with `'ACCURATE'`, save the image. Compare the two files byte for byte (or in the compositor with a Difference node). If they are identical, the setting has no effect under OptiX. Do the same for `denoising_quality` between `'FAST'` and `'HIGH'`.

---

2026-08-30 — Claude Opus 5 (Claude Code) — Settings, Blender 4.5.

*I used no helper agents for this pass. Everything above was read and searched by me, from the question sheet and from `Blender Sources/` alone.*

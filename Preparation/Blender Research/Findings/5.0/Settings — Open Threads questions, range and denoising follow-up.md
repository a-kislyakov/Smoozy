# Settings — Open Threads questions, range and denoising follow-up (Blender 5.0)

**Sources consulted, and on what date.** Blender 5.0 manual (`Blender Sources/manual_5.0`, including its `_images` folder), Blender 5.0 Python API documentation (`Blender Sources/api_5.0`), Blender 5.0 release notes (`Blender Sources/release_notes_5.0`), and Cycles' own files (`Blender Sources/cycles_5.0`). All read on **2026-07-30** (the snapshot date given to me).

I read only the question sheet and files inside Blender Sources/. I did not open any other part of this project.

No helper agents were used. Everything below I searched and read myself, so the declaration above covers the whole document.

These three points sit inside questions 18 and 19 of the Settings sheet. I answered only A, B and C.

---

## A. The notation the Python API documentation uses for a maximum

**Short answer.** The notation is the bare word `inf`. It appears in the "Type:" line of a property, inside square brackets, in place of a number — for example `int in [1, inf]`. **The API documentation never defines it, never explains it, and never gives it a meaning anywhere.** So I can tell you what the notation *is*, from many places where it is used, but I cannot tell you from the sources what it *stands for*.

**LABEL: documented** (that the notation is used, and what it looks like) **and not in the sources** (what it stands for).

### The notation, quoted where it is used

The clearest example is the whole-number sample count of the EEVEE engine:

> `taa_render_samples`
>
> Number of samples per pixel for rendering
>
> Type:
>
> `int in [1, inf], default 64`

and

> `taa_samples`
>
> Number of samples, unlimited if 0
>
> Type:
>
> `int in [0, inf], default 16`

- Source: Python API documentation, `https://docs.blender.org/api/5.0/bpy.types.SceneEEVEE.html`
- Date read: 2026-07-30

It is not a one-off. Counting every whole-number "Type:" line across the whole of the API documentation, these are the commonest forms, in order:

| form | how many times it occurs |
| --- | --- |
| `int in [0, inf]` | 1474 |
| `int in [-inf, inf]` | 535 |
| `int in [0, 100]` | 67 |
| `int in [-1048574, 1048574]` | 63 |
| `int in [1, 9]` | 56 |
| `int in [1, inf]` | 53 |

*(Citation check, 2026-08-30: this table gave four rows and called them "the four commonest forms". `int in [0, 100]` occurs 67 times, not 63 — 63 is the count of a different form, `int in [-1048574, 1048574]` — and `int in [1, inf]` is sixth, not fourth. The two `inf` counts, 1474 and 535, and the figure 53 all reproduce exactly. Corrected by this check; the two rows that were missing are added so the ordering can be read.)*

The word `inf` also stands in for the maximum (and the minimum) of decimal settings — `float in [-inf, inf]` occurs 593 times, `float in [0, inf]` 304 times. Every other maximum in the whole-number lines is written as a plain number (`100`, `32767`, `1048574`, `65535`, and so on): for a whole-number setting, `inf` is the only non-numeric maximum the documentation uses. *(Citation check, 2026-08-30: the last sentence read "`inf` is the only non-numeric maximum the documentation uses", without the restriction to whole-number settings. It does not hold across the whole documentation — decimal settings carry maxima in scientific notation, for example `float in [0, 1.04857e+06]`. It does hold for whole-number settings, which is what the table is about. Narrowed by this check.)*

- Source: Python API documentation, the whole of `https://docs.blender.org/api/5.0/` (counted across every page of it)
- Date read: 2026-07-30

### There is no place where it is defined

The question asks me to quote any place where the notation is defined, explained or given a meaning. There is none. I searched for one and found nothing:

- Searched every page of the API documentation for `inf` as a standalone word, excluding the "Type:" range lines themselves. **Zero results.** The word never appears outside those lines.
- Searched every page of the API documentation for `infinit` (which catches "infinite" and "infinity"). Every hit is unrelated prose about something else — an infinite cone in the sound module, "infinite recursion" warnings on property callbacks, an infinite slab in a material setting, "0 = infinite" on an animation modifier, a defocus node where "128 (infinity) is perfect focus". None of them is talking about the range notation.
- Searched the API documentation's own introductory and explanatory pages — `index`, `info_api_reference`, `info_overview`, `info_quickstart`, `info_best_practice`, `info_advanced`, `info_tips_and_tricks`, `change_log`, and all eight `info_gotchas_*` pages — for `inf`, for `int in [`, and for the words "notation", "convention", "range", "limit". Nothing describes how the "Type:" line is written. The nearest thing to a general statement about how properties are documented is one sentence saying they are arranged to match how the data is stored internally, which says nothing about ranges.
- Searched the whole manual for `inf` as a standalone word. **Zero results.** The manual does not use this notation at all.
- Searched the whole of the 5.0 release notes for `inf` as a standalone word. **Zero results.**
- Searched the alphabetical index pages of the API documentation for `inf`. **Zero results.**

**One adjacent thing I found, and what it is not.** The API documentation does write a whole-number maximum as an arithmetic expression in one other place, but it is a different kind of place — the signature of the function used to *create* a new setting, not the "Type:" line of an existing one:

> `bpy.props.IntProperty(*, name='', description='', translation_context='*', default=0, min=-2**31, max=2**31 - 1, soft_min=-2**31, soft_max=2**31 - 1, step=1, options={'ANIMATABLE'}, subtype='NONE', ...)`

- Source: Python API documentation, `https://docs.blender.org/api/5.0/bpy.props.html`
- Date read: 2026-07-30

I am recording this because it is the only other non-plain-number maximum in the API documentation, and because someone reading my answer may wonder whether it explains `inf`. **It does not, and I am not claiming it does.** Nothing in the sources connects `2**31 - 1` to `inf`. They are in different places, doing different jobs, and no page says that one is the other. To say "`inf` means the largest whole number Blender can hold" would be me supplying the step, not the source, so I am not saying it.

**Not covered.** What `inf` actually stands for. Whether it means a genuinely unbounded setting, or a setting whose upper end simply was not declared, or the largest value the storage can hold — the sources do not say, and I will not guess. Also not covered: whether the same notation appears in Blender's tooltips or anywhere in the running program, and whether a setting whose maximum shows as `inf` behaves any differently from one with a numbered maximum.

**Outside the interface.** This is a fact about how a document is written, not about how Blender behaves, so it holds wherever you read the document. The `inf` notation is what you see when you read the API documentation — whether Blender is being driven by a person clicking or by something else does not touch it. The sources do not say whether Blender itself, when driven from outside and asked what a setting's limits are, reports anything resembling `inf`; I found no page addressing that.

**Operating system.** The API documentation is a single set of pages with no per-operating-system variants, and none of the pages I read attaches an operating system to a range line. So as a fact about the document, it is the same everywhere. Whether the underlying limit differs by operating system is not stated anywhere I looked.

---

## B. What a render does when a setting is given a value outside what it accepts, for a setting whose acceptable values are not in Cycles' own files

**A note on the reading.** "A setting whose acceptable values are not defined in Cycles' own files" is not named, so I took it to mean a sample count that lives outside Cycles — chiefly EEVEE's, whose acceptable values are stated only in the Python API documentation (`taa_render_samples`, `int in [1, inf]`; `taa_samples`, `int in [0, inf]`), because Cycles' own sample count *is* declared in Cycles' files (`min=1, max=(1 << 24)`). The other reading is broader — any setting at all whose range comes from the API documentation rather than from Cycles. My answer is the same under both readings, and I searched for both.

**Short answer.** **No.** Nothing in the manual, the API documentation, the release notes or Cycles' files states what a *render* does when such a setting is handed a value outside what it accepts. I found two statements that come close, and neither one is that.

**LABEL: not in the sources.**

### The two near misses, quoted, and why neither answers the question

**First near miss — the API documentation, but about settings you create yourself.** The page describing how to define a new whole-number setting says exactly what happens on an out-of-range assignment:

> `min` (int) – Hard minimum, trying to assign a value below will silently assign this minimum instead.
>
> `max` (int) – Hard maximum, trying to assign a value above will silently assign this maximum instead.

- Source: Python API documentation, `https://docs.blender.org/api/5.0/bpy.props.html`
- Date read: 2026-07-30

The identical sentences appear four times on that page — for whole-number settings, lists of whole numbers, decimal settings and lists of decimals.

Why this does not answer the question: this page describes the `min` and `max` you pass in when *defining a new setting through this particular function*. It is a statement about that function's arguments. EEVEE's sample count is not defined that way — it is a built-in setting, and its page gives no such sentence. To carry "silently assign this maximum instead" across to a built-in setting, a reader has to supply the assumption that built-in settings enforce their limits the same way. The sources never say so. That step is mine, not the document's, so I am not taking it, and I am not labelling this documented for the question asked.

**Second near miss — the manual, but about typing into a field.** The manual describes limits as a thing the interface enforces:

> Value Limits
>
> Most numerical values are restricted by “soft limit” and “hard limit” value ranges.
> Changing values by dragging with the mouse is restricted to the “soft limit” value range.
> Input via keyboard will allow the use of wider value ranges, but never wider than the “hard limit”.

- Source: manual, `https://docs.blender.org/manual/en/5.0/interface/controls/buttons/fields.html`
- Date read: 2026-07-30

Why this does not answer the question: it is about a person editing a number field with a mouse or keyboard. It says entry is "never wider than the hard limit", but it does not say what happens to the value you attempted — whether it is cut down to the limit, refused, or something else — and it says nothing at all about a render, or about a value arriving from anywhere other than a person's hands on the interface. It is the interface half of the question and not the render half.

### Where I looked and found nothing

- Searched the whole manual for: `clamped to`, `is clamped`, `will be clamped`, `soft limit`, `hard limit`, `out of range`, `outside the range`, `outside the valid range`, `invalid value`, `exceeds the maximum`, `above the maximum`, `larger than the maximum`, `greater than the maximum`. Every "clamped" hit belongs to something unrelated — weight painting, graph editor handles, a Map Range node, Freestyle crease angles, a geometry constraint index, a transformation constraint, dynamic paint depth, solidify material slots, viewport frame rate. None is about a render being handed an unacceptable setting value.
- Searched the whole API documentation for: `silently assign`, `hard maximum`, `hard minimum`, `clamp`, `out of range`, `outside the range`, `exceeds`, `above the maximum`. Only the `bpy.props` sentences above are on point; the rest are unrelated (a mesh validation note about out-of-range indices, a `bl_math` clamp function, an operator flag about soft limits, angle thresholds).
- Read the pages that would carry the fact if it existed anywhere: the manual's EEVEE sampling page (`https://docs.blender.org/manual/en/5.0/render/eevee/render_settings/sampling.html`) — it describes the viewport and render sample counts in prose ("The number of samples to use in the final render") and states no range and no out-of-range behaviour at all; the manual's number-fields page, quoted above; the API page for the EEVEE scene settings, which gives the ranges and nothing else.
- Read the manual's command-line pages — `arguments`, `render`, `index`, `extension_arguments` (under `https://docs.blender.org/manual/en/5.0/advanced/command_line/`). Searched them for `sample`, `invalid`, `clamp`, `out of`. There is no sample-count argument there and no statement about what happens to a value a setting will not accept. The only "clamping" on those pages is about a frame count on an unrelated argument.
- Searched the whole of the 5.0 release notes for any statement about samples or limits changing. The Cycles release note has no `sample` hits at all; nothing in any release note addresses out-of-range values.
- Searched Cycles' own files for anything that would cut a value down at render time — `clamp` across all its Python files (only the light-path clamping settings, a different feature entirely), and `samples` in `engine.py`, `__init__.py` and `version_update.py` (only version-upgrade code that sets old files' sample counts, nothing about rejecting or trimming a value).
- **A limit of the sources, stated plainly.** The Cycles folder I was given holds the add-on's Python (`__init__.py`, `properties.py`, `ui.py`, `engine.py`, `operators.py`, `presets.py`, `osl.py`, `camera.py`, `version_update.py`) and, under `source/`, only `kernel/` and `util/` — 269 files, of which 260 are headers, alongside two `.cpp` files, six `.cu` files and one `.metal` file. *(Citation check, 2026-08-30: this read "269 files, all of them headers". The 269 count is right and so is the conclusion — I checked the folder by looking and there is no denoiser implementation and no session or render-start code — but the files are not all headers. Corrected by this check.)* The part of Cycles that starts a render and acts on these settings is not in what I was given. If the answer lives in code, it lives in a part I do not have, and I cannot see it.

**Not covered.** My answer settles only that the sources are silent. It does not settle what Blender actually does — a value may well be trimmed to the limit, or refused, or accepted whole; I have no basis in the sources to say which. It also does not settle whether the answer differs by route (a value set by script, by a launch instruction, or typed into a field), nor whether anything is reported to the person or to the console when it happens.

**Outside the interface.** The sources do not say. This matters here more than usual: the one statement I found about limits, the manual's Value Limits paragraph, is explicitly about dragging with the mouse and typing on the keyboard, so it speaks only for a person sitting at the interface. I found nothing at all describing what happens to an out-of-range value when Blender is started and driven by something other than a person clicking. That gap is the whole of the answer to this point.

**Operating system.** The sources do not say. Nothing I read attaches any of this to an operating system, but nothing states that it is the same on all of them either, so I am recording it as unstated rather than as uniform.

---

## C. What the prefiltering setting and the quality setting do when a different denoiser is in use

**Short answer.** The sources say one thing and one thing only: **the two settings are not shown when a denoiser other than OpenImageDenoise is in use.** The manual says this outright. Cycles' own files show the same thing and show the exact test that decides it. What the settings *do* — whether the stored values are consulted, ignored, or have any effect on the picture — when another denoiser runs is **not stated anywhere**, and the part of Cycles that would show it is not in the files I was given.

**LABEL: documented** (that they are hidden), **observed** (the test in Cycles' files that hides them, and that their declarations carry no condition on the denoiser), **inferred** (that the values are still stored while another denoiser is chosen), **and not in the sources** (what they do to a render under another denoiser). *(Citation check, 2026-08-30: "that the values are still stored" sat under observed and has been moved to inferred, to match the correction further down. Changed by this check.)*

### What the manual says

The manual describes both settings and, for each, adds the same sentence about visibility:

> Prefilter OpenImageDenoise
>
> Controls whether prefiltering is applied to the Passes before denoising.
> Visible only when using OpenImageDenoise.

and

> Quality OpenImageDenoise
>
> Overall denoising quality.
> Visible only when using OpenImageDenoise.

- Source: manual, `https://docs.blender.org/manual/en/5.0/render/cycles/render_settings/sampling.html`
- Date read: 2026-07-30

The same page names the alternatives, which is what "a different denoiser" means here:

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

- Source: same page and date as above.

Note the exact word the manual uses: **"Visible"**. It is a statement about what you can see on screen. It is not a statement about what the render does.

### What Cycles' own files show

**The settings' own descriptions carry the same qualification.** Both the final-render pair and the viewport pair say "when using OpenImageDenoise" inside their own description text:

```
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
```

- Source: Cycles' own files, `properties.py`, lines 414–425 (the final-render pair). The viewport pair, `preview_denoising_prefilter` and `preview_denoising_quality`, carries word-for-word the same two descriptions at lines 449–460, with defaults `'FAST'` and `'BALANCED'`.
- Date read: 2026-07-30
- **LABEL: observed.**

**The test that hides them is written out.** For the final render:

```
        col.prop(cscene, "denoising_input_passes", text="Passes")
        if cscene.denoiser == 'OPENIMAGEDENOISE':
            col.prop(cscene, "denoising_prefilter", text="Prefilter")
            col.prop(cscene, "denoising_quality", text="Quality")
```

- Source: Cycles' own files, `ui.py`, lines 346–349, inside the panel class `CYCLES_RENDER_PT_sampling_render_denoise`.
- Date read: 2026-07-30
- **LABEL: observed.**

For the viewport the same two settings are placed behind a slightly different test — not the chosen denoiser but the *effective* one, which resolves the "Automatic" choice first:

```
        effective_preview_denoiser = get_effective_preview_denoiser(context, has_oidn_gpu)
        if effective_preview_denoiser == 'OPENIMAGEDENOISE':
            col.prop(cscene, "preview_denoising_prefilter", text="Prefilter")
            col.prop(cscene, "preview_denoising_quality", text="Quality")
```

- Source: Cycles' own files, `ui.py`, lines 275–278, inside the panel class `CYCLES_RENDER_PT_sampling_viewport_denoise`. The helper it calls is defined at lines 173–185 of the same file and returns the chosen denoiser unless that choice is `"AUTO"`, in which case it returns `'OPENIMAGEDENOISE'` if an OpenImageDenoise-capable graphics device is present, `'OPTIX'` if an OptiX-capable one is, and `'OPENIMAGEDENOISE'` otherwise.
- Date read: 2026-07-30
- **LABEL: observed.**

**The values do not go away.** Both settings are declared as ordinary choice settings with a fixed list of options and a fixed default, and nothing in their declaration ties them to the chosen denoiser — no update handler, no re-computed list of options. The list of options is the same three (`NONE`, `FAST`, `ACCURATE`) and the same three (`HIGH`, `BALANCED`, `FAST`) whatever denoiser is chosen, and both settings appear in the list of settings a Cycles preset writes out (`presets.py`, lines 58–59 for the render pair, 83–84 for the viewport pair) without any condition on the denoiser.

- Source: Cycles' own files, `properties.py` lines 312–338 (the two option lists) and 414–460 (the four declarations); `presets.py` lines 58–59 and 83–84.
- Date read: 2026-07-30
- **LABEL: observed** — for what the files say: the two declarations carry no condition on the denoiser, no update handler and a fixed list of options, and `presets.py` lists them with no condition either.
- **LABEL: inferred** — that the values are stored and kept regardless of which denoiser is chosen. The step: within these files nothing but the panel's `draw` method tests the denoiser, and neither declaration has anything that would change or clear the value, so a setting whose row is not drawn still holds whatever it held. **What that means for a render under OptiX is a further step I am not taking** — see immediately below.

*(Citation check, 2026-08-30: "The values are stored and kept regardless of which denoiser is chosen" was labelled observed. The quoted lines show what the file says — no condition, no update handler, a fixed option list; that the values are kept while the program runs is behaviour and a step past the text. Split into the observed part and the inferred part, with the step written out. Changed by this check.)*

### What is not stated: whether they do anything

I could find no statement anywhere about whether an OptiX render, or an "Automatic" render that resolves to OptiX, reads these two values, ignores them, or is affected by them. Everything in the sources is about visibility. Where I looked:

- Searched the whole manual for `prefilter`. Exactly two pages contain it: the Cycles sampling page quoted above, and the compositor's Denoise node page. Read both in full. Neither says what happens to the setting under a different denoiser; the Cycles page says only "Visible only when using OpenImageDenoise", twice.
- Read the compositor Denoise node page in full (`https://docs.blender.org/manual/en/5.0/compositing/types/filter/denoise.html`). It has its own Prefilter and Quality options, and it states that this node "uses Open Image Denoise" — there is no second denoiser to switch to there, so the question does not arise for it. Its Quality does have a "Follow Scene" option described as "Use the scene’s quality setting", which is the only link between the two places that the sources draw. It says nothing about other denoisers.
- Searched the whole API documentation for `prefilter`. One hit only: the change log, which records that `prefilter`, `quality` and `use_hdr` were **Removed** from `bpy.types.CompositorNodeDenoise` between 4.5 and 5.0 (`https://docs.blender.org/api/5.0/change_log.html`). I confirmed on the 5.0 page for that node that those three no longer appear as settings (`https://docs.blender.org/api/5.0/bpy.types.CompositorNodeDenoise.html`). This is about the compositor node, not the scene's denoiser, and it does not touch the question — I record it because it is the only API-documentation appearance of the word. Cycles' own scene denoising settings do not appear in the API documentation at all; searching it for "Cycles" returns only an unrelated animation modifier and the Cycles operator page.
- Searched every page of the 5.0 release notes for `prefilter` (**zero results**) and for `denois`. The single hit is in the Cycles notes and is about something else: "The OptiX denoiser result quality is improved by flipping the image vertically to have a top-left origin before processing." (`https://developer.blender.org/docs/release_notes/5.0/cycles/`, read 2026-07-30.) It says nothing about the prefiltering or quality settings.
- Searched every file in the Cycles folder for `prefilter` and for `denoising_quality`. All hits are the ones I have quoted — the two option lists and four declarations in `properties.py`, the four panel lines in `ui.py`, the four preset lines in `presets.py`. There is nowhere else. In particular there is nothing that reads these values and hands them to a denoiser.
- **A limit of the sources, stated plainly.** As in point B, the Cycles files I was given contain the add-on's Python and, under `source/`, only kernel and utility headers. I searched `source/` for `denois` and the files that come back are kernel headers about denoising *passes* and related maths, not the code that runs a denoiser or reads these two settings. The part of Cycles that would settle what OptiX does with a stored prefilter or quality value is not in what I have. I am not guessing at it.

**Not covered.** My answer does not settle whether the stored prefilter and quality values have any effect on an OptiX render — only that the sources never say. It does not settle what happens on the "Automatic" setting once it resolves to OptiX. It does not settle what happens if a value is set while OpenImageDenoise is chosen and the denoiser is then switched. And it does not settle the reverse case — whether OptiX has settings of its own that are hidden when OpenImageDenoise is in use — which I did not investigate, because it was not asked.

**Outside the interface.** This one needs care, because the whole of the documented answer is about the interface. "Visible only when using OpenImageDenoise" is a statement about a panel on a screen, and the code that enforces it sits in the file that draws that panel. When Blender is driven from outside its own window there is no panel and nothing is drawn, so the only documented fact here — that the settings are hidden — has nothing to bite on. The settings themselves still exist and still hold their values, because they are declared independently of the panel. What a render started from outside actually does with those values under a different denoiser is exactly the thing the sources do not say. So: the visibility rule does not carry outside the interface, and nothing replaces it there.

**Operating system.** The sources do not say that the visibility rule differs by operating system, and I found nothing attaching it to one. Two nearby things in the sources do depend on the machine rather than the operating system as such: the OptiX denoiser is described in the manual as "Only available on NVIDIA GPUs when configured in the Cycles Render Devices user preferences", and Cycles' own files show the viewport panel resolving "Automatic" by asking which kinds of graphics device are present (`ui.py`, lines 173–194). That is a hardware condition, not an operating-system one, and I am not turning it into one. Beyond that, unstated.

---

## Things a person with Blender 5.0 open could settle by hand in a few minutes

These are written for someone who cannot see my reasoning. Each one is a thing the sources left open.

**1. What `inf` actually is, for a whole-number setting.** Open Blender, switch the render engine to EEVEE, then open the Scripting workspace and in the Python console type `bpy.context.scene.eevee.taa_render_samples = 10**12` and press Return, then type `bpy.context.scene.eevee.taa_render_samples` and press Return. Write down exactly what the second line prints, and write down whether the first line printed an error. Repeat with `10**20` and with `-5`. This settles both what the upper end really is and what happens above it. (Relates to A and B.)

**2. The same for a setting whose maximum is a plain number.** In the same console, with any engine, type `bpy.context.scene.render.resolution_percentage = 999999` then read it back. Note whether the stored value is the maximum, the value you gave, or unchanged, and whether anything was printed. This shows whether a numbered maximum and an `inf` maximum behave alike. (Relates to B.)

**3. Whether an out-of-range value survives to a render, and whether anything is said.** Start Blender from a terminal so you can see its output. Set an out-of-range sample count as in item 1, then render one frame and watch the terminal. Write down every line printed that mentions the setting, and write down the sample count the render actually used. Then do the same again with Blender started from the terminal in its no-window mode against a saved file, driving the same change through a script handed in at launch, and compare the two terminal outputs word for word. This is the exact gap in point B. (Relates to B.)

**4. Whether the prefilter and quality settings are truly only hidden, not cleared.** With Cycles as the engine, open Render Properties, expand Sampling, expand Render, expand Denoise. Set Denoiser to OpenImageDenoise, set Prefilter to `None` and Quality to `Fast`. Now set Denoiser to OptiX — the two fields should disappear. In the Python console read `bpy.context.scene.cycles.denoising_prefilter` and `bpy.context.scene.cycles.denoising_quality` and write down what they print. Then set Denoiser back to OpenImageDenoise and confirm on screen whether the fields come back showing `None` and `Fast`. This settles whether the values persist while hidden. (Relates to C.)

**5. Whether the prefilter setting changes an OptiX render at all.** On a machine with an NVIDIA graphics card configured in Preferences, set up a noisy scene at a low sample count with denoising on and Denoiser set to OptiX. Render once with `denoising_prefilter` set to `NONE` (set it through the Python console, since the field is hidden) and save the image. Without changing anything else, set it to `ACCURATE`, render again, and save. Compare the two images pixel for pixel. Identical images mean OptiX ignores the setting; different images mean it does not. Repeat the same pair for `denoising_quality` between `HIGH` and `FAST`. This is the exact gap in point C. (Relates to C.)

**6. Whether the viewport pair follows the same rule as the render pair.** Repeat item 4 in the Viewport section of the Denoise panel rather than the Render section, and additionally set the viewport Denoiser to Automatic on a machine with an NVIDIA card and note whether Prefilter and Quality are shown or hidden. Cycles' files show the viewport uses the resolved denoiser rather than the chosen one, so Automatic is the interesting case. (Relates to C.)

**7. Whether any of the above differs when Blender is driven from outside.** Repeat items 3 and 5 with Blender started in its no-window mode from a terminal, driving every setting change through a script handed in at launch rather than by clicking, and compare the resulting images and terminal output against the clicked run. Do this on each operating system you care about and note any difference. (Relates to all three points, and to the two closing lines on each of them.)

---

2026-08-30 — Claude Opus 5 (Claude Code) — Settings, Blender 5.0.

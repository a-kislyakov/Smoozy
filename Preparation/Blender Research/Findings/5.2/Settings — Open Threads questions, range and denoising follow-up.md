# Settings — Open Threads questions, range and denoising follow-up (Blender 5.2)

Sources consulted, all on **2026-07-30** (the snapshot date, used as the date read throughout):
the Blender 5.2 manual (`Blender Sources/manual_5.2`, including its `_images` folder), the Blender 5.2
Python API documentation (`Blender Sources/api_5.2`), the Blender 5.2 release notes
(`Blender Sources/release_notes_5.2`), and Cycles' own files (`Blender Sources/cycles_5.2`).

I read only the question sheet and files inside Blender Sources/. I did not open any other part of this project.

One disclosure, so the line above is exact: my first command listed the names of the folders at the top
level of this project, before I confined myself to the two permitted places. That listing showed only
names my brief had already given me (App, Audits, Preparation, README.md, CLAUDE.md and so on). I opened
none of them, and I did not list, search or read anything inside them.

Question sheet read: `Preparation/Blender Research/Questions/Settings.md` (questions 18 and 19 are the
context for the three points below).

Only points A, B and C are answered here. No other question on the sheet is addressed.

---

## A. The notation the Python API documentation uses for a maximum

**Answer.** The notation is **`inf`**. It appears inside the square-bracket range that the API
documentation prints after a setting's description, in the form `(in [minimum, maximum], default …)`.
For a whole-number setting the maximum is sometimes a plain number and sometimes the word `inf`.

**What it stands for: the sources never say.** Searching the whole of the API documentation, the whole
of the manual and the whole of the release notes, `inf` is never defined, expanded, or given a meaning
anywhere. It only ever appears inside those range brackets. So the honest answer to "what exactly does
it stand for" is that the three sources supplied do not state it.

**LABEL: documented** — for the fact that `inf` is the notation used, and for the exact places it
appears. **LABEL: not in the sources** — for what `inf` stands for.

### The exact words

A whole-number setting whose maximum is written as `inf`:

> `taa_render_samples`
>
> `Number of samples per pixel for rendering (in [1, inf], default 64)`
>
> `Type:`
>
> `int`

- Source: Python API documentation, https://docs.blender.org/api/5.2/bpy.types.SceneEEVEE.html
- Date read: 2026-07-30

Another one on the same page, showing `inf` used with a minimum of zero:

> `taa_samples`
>
> `Number of samples, unlimited if 0 (in [0, inf], default 16)`
>
> `Type:`
>
> `int`

- Source: Python API documentation, https://docs.blender.org/api/5.2/bpy.types.SceneEEVEE.html
- Date read: 2026-07-30

`inf` is also used for the *minimum* in the same brackets, for whole-number settings:

> `simulation_frame_end`
>
> `Frame at which simulations end (in [-inf, inf], default 250)`
>
> `Type:`
>
> `int`

- Source: Python API documentation, https://docs.blender.org/api/5.2/bpy.types.Scene.html
- Date read: 2026-07-30

And the contrast — the same documentation writing a plain number for other whole-number settings, which
is what makes `inf` a notation used "sometimes" rather than always:

> `frame_end`
>
> `Final frame of the playback/rendering range (in [0, 1048574], default 250)`

> `frame_step`
>
> `Number of frames to skip forward while rendering/playing back each frame (in [0, 1048574], default 1)`

- Source: Python API documentation, https://docs.blender.org/api/5.2/bpy.types.Scene.html
- Date read: 2026-07-30

> `resolution_percentage`
>
> `Percentage scale for render resolution (in [1, 32767], default 100)`
>
> `Type:`
>
> `int`

- Source: Python API documentation, https://docs.blender.org/api/5.2/bpy.types.RenderSettings.html
- Date read: 2026-07-30

### The nearest thing to a definition, and why it is not one

The closest the documentation comes is that the page describing an integer property's own attributes
itself uses `inf` for those attributes' ranges, without saying what it means:

> `hard_max`
>
> `Hard maximum, trying to assign a value above will silently assign this maximum instead (in [-inf, inf], default 0, readonly)`
>
> `Type:`
>
> `int`

- Source: Python API documentation, https://docs.blender.org/api/5.2/bpy.types.IntProperty.html
- Date read: 2026-07-30

That tells you `inf` is used even where the value being described is a whole number, but it does not
explain the notation.

### A second, different notation, in case that is what was meant

The question is ambiguous. A second notation is used for a whole-number maximum, in the signature of
the function that creates an integer setting. There the maximum is written as a power of two rather
than as a plain number:

> `bpy.props.IntProperty(*, name='', description='', translation_context='*', default=0, min=-2**31, max=2**31 - 1, soft_min=-2**31, soft_max=2**31 - 1, step=1, options={'ANIMATABLE'}, override=set(), tags=set(), subtype='NONE', update=None, get=None, set=None, get_transform=None, set_transform=None)`

- Source: Python API documentation, https://docs.blender.org/api/5.2/bpy.props.html
- Date read: 2026-07-30

The same `2**31 - 1` appears in the `IntVectorProperty` signature on that page. The documentation does
not explain that notation either. `**` is Python's own "to the power of", so `2**31 - 1` is two to the
thirty-first, minus one — but that meaning comes from the Python language, not from any of the three
sources here, so I record it as an unstated reading rather than a documented one.

**I take `inf` to be the notation asked about**, because it is the one that appears in the printed range
of an actual setting, and because it is not a plain number in any sense. `2**31 - 1` is a plain number
written in a compact form. I flag the other reading so nothing is hidden.

### Not covered

- What `inf` stands for. The sources do not say. (In ordinary use it is short for "infinity", but I did
  not read that anywhere in these files, so I am not claiming it as a finding.)
- Whether a whole-number setting shown as `[1, inf]` truly has no upper bound in the running program,
  or has one that the documentation simply does not print. Nothing in the three sources settles this,
  and the settings shown with `inf` (for example EEVEE's `taa_render_samples`) are not defined in
  Cycles' own files, so the supplied Cycles files cannot settle it either.
- Whether `inf` means the same thing for a whole-number setting as it does for a decimal one.
- Why some whole-number settings get a plain maximum and others get `inf`.

### Outside the interface

The API documentation describes the settings themselves, not a way of driving Blender. The same page
and the same range notation apply whether Blender is opened by a person or started and driven by
something else. The sources do not state any difference. This is the documentation's own printed
notation, so there is nothing here that could differ between the two.

### Operating system

The sources say nothing about the notation varying by operating system, and there is one API
documentation site for the version rather than one per system. Nothing in the manual, the API
documentation or the release notes ties the notation to a system.

### Where I looked

- Searched the whole of `api_5.2` for `inf]` — 1,239 files contain it, so the notation is pervasive.
- Listed every distinct bracketed form containing `inf` to confirm it only ever occurs inside
  `(in [ … ], default …)`.
- Searched the whole of `api_5.2` for every occurrence of `inf` as a standalone word **outside** those
  brackets: exactly one hit, `freestyle.utils` — "The result is None or in the range [-inf, inf]" —
  which is again a range, not a definition.
- Searched the whole of `api_5.2` and the whole of `manual_5.2` for `infinity`, `infinite`,
  `unbounded`, `no upper limit`, `no maximum`. Hits were about lighting, focal blur, sound loop counts
  and cloth sewing forces; none define the notation.
- Searched every `.txt` file of `manual_5.2` for `inf` as a standalone word: **no hits at all**. The
  manual never uses the notation.
- Searched `release_notes_5.2` for `inf` as a standalone word: one hit, in the bug-fix list, about a
  crash with "inf" vertex coordinates. Not a definition.
- Read `https://docs.blender.org/api/5.2/info_api_reference.html` (the page that explains how to use the
  reference) looking for a legend or convention section explaining the bracketed range. It explains ID
  data-blocks and how to find properties; it does not explain the range notation.
- Searched the API documentation's own narrative pages (`info_*`, `index`) for words like `notation`,
  `denotes`, `range is given/shown/written`. Nothing.
- Checked `https://docs.blender.org/manual/en/5.2/glossary/` for an entry. None.
- Read `https://docs.blender.org/manual/en/5.2/advanced/limits.html` in full — it gives working limits
  for space, time (1,048,574 frames per scene) and text field lengths. It does not mention the API
  notation or sample counts.
- Read `https://docs.blender.org/api/5.2/bpy.types.IntProperty.html` and
  `https://docs.blender.org/api/5.2/bpy.props.html` in the relevant parts.
- Ruled out `manual_5.2/_images`: no screenshot could show a documentation notation, and the searches
  above show the manual does not use it.

---

## B. What a render does when a setting whose acceptable values are not defined in Cycles' own files is given a value outside what it accepts

**The question is ambiguous and I say so plainly.** I read "a setting whose acceptable values are not
defined in Cycles' own files" as, in the context of question 18, a sample count that lives outside
Cycles — chiefly EEVEE's render sample count, `taa_render_samples`, whose range the API documentation
prints but whose actual limits appear nowhere in the Cycles files supplied. The other reading is "any
setting at all that Cycles does not define", which would include the resolution percentage. My answer
below covers both, because what I found is stated generally rather than per setting.

**Answer.** The sources do **not** say what a *render* does with such a value. What they do say is what
happens at the moment the value is *assigned*: it is silently replaced by the nearest limit. That is
documented, and it is documented generally for integer settings, not for any one setting.

**LABEL: documented** — that assigning a value above the hard maximum silently assigns the maximum
instead, and assigning below the hard minimum silently assigns the minimum instead.
**LABEL: not in the sources** — what the render itself does, or reports, when handed an out-of-range
value. Nothing in the manual, the API documentation or the release notes describes a render-time
behaviour, a message, a warning, or an error for an out-of-range setting value.

### The exact words

> `hard_max`
>
> `Hard maximum, trying to assign a value above will silently assign this maximum instead (in [-inf, inf], default 0, readonly)`
>
> `hard_min`
>
> `Hard minimum, trying to assign a value below will silently assign this minimum instead (in [-inf, inf], default 0, readonly)`

That page describes itself as:

> `RNA integer number property definition`

- Source: Python API documentation, https://docs.blender.org/api/5.2/bpy.types.IntProperty.html
- Date read: 2026-07-30

The same wording appears as the description of the arguments used to create an integer setting:

> `min (int) – Hard minimum, trying to assign a value below will silently assign this minimum instead.`
>
> `max (int) – Hard maximum, trying to assign a value above will silently assign this maximum instead.`

- Source: Python API documentation, https://docs.blender.org/api/5.2/bpy.props.html
- Date read: 2026-07-30

The same two sentences also appear on
https://docs.blender.org/api/5.2/bpy.types.FloatProperty.html for decimal settings, and in `bpy.props`
for the float and float-vector versions as well as the integer and integer-vector ones. The word
"silently" occurs twelve times in the whole of the API documentation, in three files and nowhere else:
eight times in `bpy.props` (four blocks — `FloatProperty`, `FloatVectorProperty`, `IntProperty`,
`IntVectorProperty`), twice on the `bpy.types.IntProperty` page and twice on the
`bpy.types.FloatProperty` page. *(Citation check, 2026-08-30: this passage read "twice more in `bpy.props`
for the float and float-vector versions. Those are the only four places…", which left the integer-vector
block unaccounted for and undercounted the places. The substance — that the word occurs nowhere outside
property-definition material — is confirmed. Corrected by this check.)*

### The step from that to a render — and it is a step

The quoted words are about **assignment**, not about rendering. To get from "assignment silently clamps"
to "the render uses the clamped value" a reader has to supply the idea that the render reads the stored
setting after the assignment has already changed it. That is an inference, so:

**LABEL: inferred** — after an out-of-range assignment the setting *holds* the limit, not the value that
was offered. Step taken: the documentation says the out-of-range value is replaced at the moment of
assignment, therefore the value the setting is left holding is the limit. The sources do not spell this
out and do not say whether anything is reported when it happens.

**What is not settled by that, and is not in the sources: whether a render reads the stored value.**
Nothing in the manual, the API documentation, the release notes or the supplied Cycles files says that
a render takes its sample count from the stored setting, and the part of Cycles that would show it is
not among the files supplied. So what a render started after such an assignment actually uses is open.

*(Citation check, 2026-08-30: the inferred claim read "a render started after an out-of-range assignment
would use the clamped value", and the step it wrote out said "what the render later reads is the limit".
The quoted words carry the assignment half — the setting is left holding the limit — but the step does
not derive that a render reads the stored value; it assumes it, and no source states it. Narrowed to
what the words carry, with the remainder recorded as a gap. Changed by this check.)*

### For comparison, what Cycles' own files do define

For Cycles' own sample count the acceptable values *are* in the supplied files, which is what makes the
contrast in the question meaningful:

> ```
>     samples: IntProperty(
>         name="Samples",
>         description="Number of samples to render for each pixel",
>         min=1, max=(1 << 24),
>         default=4096,
>     )
> ```

- Source: Cycles' own files, `Blender Sources/cycles_5.2/properties.py`, lines 485–490
- Date read: 2026-07-30
- **LABEL: observed** (this is the file's text; what a render does with it is not observed)

The comparable EEVEE setting has no counterpart anywhere in the supplied Cycles files — it is only in
the API documentation, printed as `(in [1, inf], default 64)`. So for that setting neither the manual
nor Cycles' files state a maximum at all.

### Not covered

- Whether anything at all is printed, logged or returned when a value is clamped. The sources say
  "silently", which tells you nothing is reported at assignment time; they say nothing about the render.
- What happens when an out-of-range value reaches a setting by a route other than a Python assignment —
  for example from a file saved by a different version, or from an argument handed in at launch. Not
  addressed anywhere I searched.
- What the actual limits of EEVEE's render sample count are. `inf` is printed; no number is given, and
  the supplied Cycles files do not contain that setting.
- Whether the clamping described for `bpy.props`-created settings is the same mechanism as for settings
  built into Blender. The `IntProperty` page describes itself as the "RNA integer number property
  definition" and uses the same words, which is why I treat the statement as general — but the sources
  do not state that generality outright.
- Whether a value inside the printed range but nonsensical for a render (for example a sample count of
  zero where zero has a special meaning) behaves differently. Question 18's territory, not settled here.

### Outside the interface

The clamping sentences are on Python API pages and describe assignment to a setting. They carry no
condition about Blender's window being open, and Python assignment is exactly what something driving
Blender from outside would do. So the documented part reads the same either way. The sources do not
state any difference, and they say nothing about a render started from outside reporting an out-of-range
value differently. I found no page describing an out-of-range setting value in the command-line
documentation at https://docs.blender.org/manual/en/5.2/advanced/command_line/arguments.html.

### Operating system

Nothing in the three sources ties this to an operating system. The clamping sentences carry no such
condition, and the command-line page's per-system material is about launching, not about setting values.
The sources do not say it differs anywhere.

### Where I looked

- Searched the whole of `api_5.2` for `silently` — twelve hits, all four distinct sentences quoted
  above, and nothing else.
- Searched the whole of `api_5.2` and the whole of `manual_5.2` for `clamp` / `clamped` / `clamping`.
  In the manual the hits are curve handles, weight painting, light clamping, brush colour ramps, a
  playback cache argument, and node sockets; the one general page,
  https://docs.blender.org/manual/en/5.2/advanced/limits.html, says only "While object positions, vertex
  locations are not clamped, larger values become increasingly imprecise." In the API the hits are
  property *names* (`clamp_surface_direct`, `impulse_clamp`, and so on) and two `imbuf` notes about
  compression quality being "clamped" — none of them about render settings.
- Searched both for `out of range`, `outside the range`, `invalid value`, `is ignored`,
  `will be ignored`. Nothing about a setting being given a value it does not accept.
- Searched the API's narrative pages (`info_gotchas_*`, `info_best_practice`, `info_quickstart`,
  `info_overview`) for `ValueError`, `TypeError`, `RuntimeError`. The hits are about wrong *types*
  (assigning a number where a string is expected), about operator context failures, and about class
  registration — not about out-of-range numbers.
- Read https://docs.blender.org/manual/en/5.2/advanced/command_line/arguments.html in the render-options
  and Cycles-options parts, and searched it for `error`, `exit code`, `fail`, `samples`, `percentage`.
  The Cycles options listed there are `--cycles-device` and `--cycles-print-stats`; there is no
  sample-count or percentage argument on that page and no statement about bad values.
- Read https://docs.blender.org/manual/en/5.2/advanced/command_line/render.html in full. It covers
  argument order, `-f`, `-a`, `-E`, `-s`, `-e`, `-t`, and says nothing about out-of-range values.
- Read the EEVEE sampling page,
  https://docs.blender.org/manual/en/5.2/render/eevee/render_settings/sampling.html — it describes the
  viewport and render sample counts in one line each and states no limits.
- Read the Cycles sampling page,
  https://docs.blender.org/manual/en/5.2/render/cycles/render_settings/sampling.html — it describes
  Render (Max) Samples and the time limit and states no limits either.
- Searched `release_notes_5.2` for relevant material; the only nearby hits are in the bug-fix list and
  concern other subjects.
- Searched all of `cycles_5.2` for the EEVEE sample setting names — absent, as expected, since it is not
  a Cycles setting.
- Ruled out `manual_5.2/_images`: what a render does with a rejected value is behaviour, and a
  screenshot of a panel cannot show it.

---

## C. What a denoiser's prefiltering setting and its quality setting do when a different denoiser is in use

**Answer.** The sources say what happens to those two settings **on screen** — they are not shown — and
they say nothing at all about what the settings **do** to the render when another denoiser is in use.

The manual states, for each of the two settings, that it is "Visible only when using OpenImageDenoise."
Cycles' own interface file matches that: both controls are drawn only inside a test for the denoiser
being OpenImageDenoise. Neither the manual, nor the API documentation, nor the release notes, nor the
Cycles files supplied say whether the stored values still have any effect when the denoiser is OptiX,
and the part of Cycles that would show it is not among the files supplied.

**LABEL: documented** — the two settings are visible only when OpenImageDenoise is the denoiser, and
their descriptions are scoped to OpenImageDenoise.
**LABEL: observed** — the two controls are drawn only under a test for OpenImageDenoise, and their
stored descriptions in Cycles' own files name OpenImageDenoise.
**LABEL: not in the sources** — what either setting does to the rendered result when a different
denoiser is in use.

### The exact words — the manual

> `Prefilter OpenImageDenoise`
>
> `Controls whether prefiltering is applied to the Passes before denoising.`
> `Visible only when using OpenImageDenoise.`

> `Quality OpenImageDenoise`
>
> `Overall denoising quality.`
> `Visible only when using OpenImageDenoise.`

The denoiser choices these are scoped against, from the same page:

> `Denoiser`
>
> `The algorithm to use.`
>
> `Automatic:`
>
> `Uses GPU accelerated denoising if supported, for best performance.`
> `Prefers OpenImageDenoise over OptiX.`
>
> `OpenImageDenoise:`
>
> `Uses Intel's Open Image Denoise,`
> `an AI denoiser. Typically provides the highest quality.`
>
> `OptiX:`
>
> `Uses NVIDIA's OptiX AI denoiser.`
> `Supports GPU acceleration on some older NVIDIA GPUs where OpenImageDenoise does not.`
>
> `Only available on NVIDIA GPUs when configured in the Cycles Render Devices user preferences.`

- Source: manual, https://docs.blender.org/manual/en/5.2/render/cycles/render_settings/sampling.html
- Date read: 2026-07-30

Note the manual names the two settings "Prefilter **OpenImageDenoise**" and "Quality
**OpenImageDenoise**" — the denoiser's name is part of the heading.

### The exact words — Cycles' own files

The two settings' stored descriptions both name OpenImageDenoise:

> ```
>     denoising_prefilter: EnumProperty(
>         name="Denoising Prefilter",
>         description="Prefilter noisy guiding (albedo and normal) passes to improve denoising quality when using OpenImageDenoise",
>         items=enum_denoising_prefilter,
>         default='ACCURATE',
>     )
>     denoising_quality: EnumProperty(
>         name="Denoising Quality",
>         description="Overall denoising quality when using OpenImageDenoise",
>         items=enum_denoising_quality,
>         default='HIGH',
>     )
> ```

- Source: Cycles' own files, `Blender Sources/cycles_5.2/properties.py`, lines 420–431
- Date read: 2026-07-30
- **LABEL: observed**

The viewport pair carries the same wording with different defaults:

> ```
>     preview_denoising_prefilter: EnumProperty(
>         name="Viewport Denoising Prefilter",
>         description="Prefilter noisy guiding (albedo and normal) passes to improve denoising quality when using OpenImageDenoise",
>         items=enum_denoising_prefilter,
>         default='FAST',
>     )
>     preview_denoising_quality: EnumProperty(
>         name="Viewport Denoising Quality",
>         description="Overall denoising quality when using OpenImageDenoise",
>         items=enum_denoising_quality,
>         default='BALANCED',
>     )
> ```

- Source: Cycles' own files, `Blender Sources/cycles_5.2/properties.py`, lines 455–466
- Date read: 2026-07-30
- **LABEL: observed**

And the panel that draws them puts both behind a test on the chosen denoiser:

> ```
>         col.prop(cscene, "denoising_input_passes", text="Passes")
>         if cscene.denoiser == 'OPENIMAGEDENOISE':
>             col.prop(cscene, "denoising_prefilter", text="Prefilter")
>             col.prop(cscene, "denoising_quality", text="Quality")
> ```

- Source: Cycles' own files, `Blender Sources/cycles_5.2/ui.py`, lines 346–349, inside
  `CYCLES_RENDER_PT_sampling_render_denoise`
- Date read: 2026-07-30
- **LABEL: observed**

The viewport panel does the same, against the denoiser that "Automatic" resolves to:

> ```
>         effective_preview_denoiser = get_effective_preview_denoiser(context, has_oidn_gpu)
>         if effective_preview_denoiser == 'OPENIMAGEDENOISE':
>             col.prop(cscene, "preview_denoising_prefilter", text="Prefilter")
>             col.prop(cscene, "preview_denoising_quality", text="Quality")
> ```

- Source: Cycles' own files, `Blender Sources/cycles_5.2/ui.py`, lines 275–278, inside
  `CYCLES_RENDER_PT_sampling_viewport_denoise`
- Date read: 2026-07-30
- **LABEL: observed**

Note the difference between the two panels: the final-render panel tests the denoiser setting itself
(`cscene.denoiser == 'OPENIMAGEDENOISE'`), while the viewport panel first resolves "Automatic" to a real
denoiser and tests that.

Both settings are listed among those a sampling preset stores, with no condition on the denoiser beside
them:

> ```
>         "cycles.denoiser",
>         "cycles.denoising_input_passes",
>         "cycles.denoising_prefilter",
>         "cycles.denoising_quality",
> ```

- Source: Cycles' own files, `Blender Sources/cycles_5.2/presets.py`, lines 56–59
- Date read: 2026-07-30
- **LABEL: observed** — for what the file lists.

**LABEL: inferred** — that both settings keep their values when a different denoiser is chosen. The step:
within these files nothing but the panel's `draw` method tests the denoiser, the two declarations carry
no update handler and no re-computed option list, and the preset list names them with no condition, so a
setting whose row is not drawn still holds whatever it held.

*(Citation check, 2026-08-30: the sentence introducing this quote read "Both settings keep their values
regardless — they are saved and restored as part of a sampling preset, with no condition on the
denoiser", labelled observed. The four quoted lines show two names sitting in a preset's list; that the
settings keep their values while the program runs is behaviour and a step past that text. Split into the
observed part and the inferred part, with the step written out. Changed by this check.)*

### The line I am not crossing

Hiding a control is not the same as neutralising a setting. The files show the control is not drawn; they
do not show that the value is ignored. The place where the stored value would either be handed to the
denoiser or dropped is in the part of Cycles that runs the denoiser, and that part is **not among the
files supplied** — the folder holds the add-on's Python and, under `source/`, `kernel/` and `util/`:
280 files, of which 267 are headers, alongside two `.cpp` files, ten `.cu` files and one `.metal` file. I
searched the whole of `cycles_5.2/source` for "denois": twenty files come back, among them
`kernel/types.h`, `kernel/data_template.h`, `kernel/features.h`, `kernel/film/light_passes.h`,
`kernel/film/read.h`, `kernel/integrator/path_state.h` and `util/math_fast.h`. Every hit is a pass name,
a flag or related maths; there are no denoiser settings and no prefilter or quality handling anywhere in
them. So this is a limit of the sources, not a finding. *(Citation check, 2026-08-30: this passage said
the folder holds "the kernel and the utility headers only" and that the "denois" hits are in two named
files. The absence it is used for is right and I confirmed it by looking at the folder — there is no
denoiser implementation and no session or render-start code, and `prefilter` and `denoising_quality`
occur nowhere outside `properties.py`, `ui.py` and `presets.py`. The file breakdown and the hit count
were not. Corrected by this check.)*

### Not covered

- Whether the stored prefilter or quality value has any effect on the rendered image when OptiX is the
  denoiser. Not stated anywhere, and not visible in the files supplied.
- What happens when the denoiser is "Automatic" and resolves to OptiX for a final render. The final-render
  panel tests the raw setting rather than a resolved one, and the manual's "Visible only when using
  OpenImageDenoise" does not say which of the two it means.
- Whether the settings are reset, preserved, or reinterpreted when the denoiser is changed. Cycles' files
  show no update handler on either setting; the manual is silent.
- The compositor's own Denoise node is a separate thing with its own Prefilter and Quality, and its
  Quality has a "Follow Scene: Use the scene's quality setting." choice
  (https://docs.blender.org/manual/en/5.2/compositing/types/filter/denoise.html). That node is described
  as using Open Image Denoise only, so it raises no "different denoiser" case. I mention it only so it is
  clear I did not confuse the two.
- Nothing here covers EEVEE. EEVEE's own denoising, if any, has no prefilter or quality setting in the
  pages I read.

### Outside the interface

This is the one place the answer bites. Everything the sources state is about **visibility** — whether a
control is drawn in a panel. When Blender is started and driven by something else, no panel is drawn at
all, so "Visible only when using OpenImageDenoise" says nothing about that case. The settings themselves
still exist and can still be read and written; Cycles' files define them unconditionally on the scene,
not conditionally on the denoiser. What they then do to a render driven from outside is exactly the thing
the sources do not state. The manual, the API documentation, the release notes and the supplied Cycles
files all leave it open.

### Operating system

Partly system-dependent, and the sources say so for the denoiser choice rather than for these two
settings. The manual states OptiX is "Only available on NVIDIA GPUs when configured in the Cycles Render
Devices user preferences", and Cycles' own files carry the same condition in the OptiX entry's
description: "Use the OptiX AI denoiser with GPU acceleration, only available on NVIDIA GPUs when
configured in the system tab in the user preferences" (`Blender Sources/cycles_5.2/properties.py`, lines
279–282). That is a hardware and configuration condition, not an operating-system one, and the sources
do not name any operating system. Cycles' files also show OpenImageDenoise itself is offered only if the
build includes it — the entry is produced under a test of `_cycles.with_openimagedenoise`
(`Blender Sources/cycles_5.2/properties.py`, lines 271–275) — and there is a fallback item reading
"Blender was compiled without a viewport denoiser" (same file, line 298). Which builds include it is not
stated in any of the sources.

### Where I looked

- Searched the whole of `manual_5.2` for `Prefilter` — exactly two pages contain it:
  `render/cycles/render_settings/sampling` and `compositing/types/filter/denoise`. Read both in full.
- Read https://docs.blender.org/manual/en/5.2/render/cycles/render_settings/sampling.html end to end,
  covering Denoising, Denoiser, Passes, Prefilter, Quality, Start Sample and Use GPU.
- Read https://docs.blender.org/manual/en/5.2/compositing/types/filter/denoise.html in full, to be sure
  the compositor node was not the thing being asked about.
- Searched `cycles_5.2/properties.py` for every occurrence of `denois` and read the whole denoising
  block, lines 271–340 (the enumerations) and 406–482 (the settings).
- Searched `cycles_5.2/ui.py` for every occurrence of `denois` and read both denoising panels and the
  helper functions `show_denoise_active`, `show_preview_denoise_active`,
  `get_effective_preview_denoiser`, `has_oidn_gpu_devices` and `has_optixdenoiser_gpu_devices`
  (lines 149–196, 246–288, 320–356).
- Searched every other Python file in `cycles_5.2` for `prefilter` and `quality`: the only hits outside
  `properties.py` and `ui.py` are the four preset entries in `presets.py`.
- Searched the whole of `cycles_5.2/source` (kernel and util) for `denois`. Only pass names, flags and
  film members. The denoiser implementation is not in the supplied files.
- Searched the whole of `api_5.2` for a Cycles settings page. There is none — the only Cycles entries are
  `bpy.ops.cycles` and an unrelated `FModifierCycles`. Read
  https://docs.blender.org/api/5.2/bpy.ops.cycles.html in full: three operators, none of which mentions
  prefilter or quality. So the API documentation cannot answer this question at all.
- Read https://docs.blender.org/api/5.2/bpy.types.CompositorNodeDenoise.html — it carries no prefilter or
  quality properties in 5.2, so it adds nothing.
- Searched `release_notes_5.2` for `denois`: four hits — an EEVEE reflection denoiser tweak, and three
  bug-fix list entries (OptiX OSL denoising albedo for rough metals, a Denoise Animation operator
  regression, and a compositor hang). None states what prefilter or quality do under another denoiser.
- Checked `manual_5.2/_images` for a screenshot of the Cycles sampling or denoising panel that might show
  the controls present or absent. The only denoise-related images are
  `compositing_types_filter_denoise_example.jpg`, `node-types_CompositorNodeDenoise.webp`,
  `render_layers_denoising_example1.jpg` and `render_layers_denoising_example2.jpg`. The Cycles sampling
  manual page contains no `[image: …]` marker at all, so there is no panel screenshot to read. Ruled out.

---

## Things a person with Blender 5.2 open could settle in a few minutes

1. **What `inf` means for a whole-number setting.** Open the Scripting workspace's Python console and
   type `bpy.types.SceneEEVEE.bl_rna.properties['taa_render_samples'].hard_max`, then press Return. Write
   down the number shown. Do the same for `hard_min`. Compare with the documented range `[1, inf]`. Repeat
   with `bpy.types.RenderSettings.bl_rna.properties['resolution_percentage'].hard_max`, which the
   documentation prints as the plain number 32767, to check the two agree.

2. **Whether an out-of-range value is silently clamped, and what a render then uses.** In the Python
   console set the EEVEE render sample count far above any plausible limit —
   `bpy.context.scene.eevee.taa_render_samples = 999999999` — press Return, then on the next line type
   `bpy.context.scene.eevee.taa_render_samples` and note what comes back. Watch whether any red error
   line appears. Then try a value below the minimum, `= 0`, and note the same two things. Repeat both
   with `bpy.context.scene.render.resolution_percentage`.

3. **The same, from outside Blender.** From a terminal run
   `blender -b yourfile.blend --python-expr "import bpy; bpy.context.scene.eevee.taa_render_samples = 999999999; print('STORED', bpy.context.scene.eevee.taa_render_samples)" -f 1`
   and read the line beginning STORED in the output, plus anything printed near it. That settles whether
   the clamping and the silence hold when Blender is driven from outside.

4. **Whether the Cycles sample count clamps at the number in Cycles' files.** With the render engine set
   to Cycles, in the Python console set `bpy.context.scene.cycles.samples = 99999999` and read the value
   back. Cycles' file says the maximum is `1 << 24`, which is 16777216; check whether that is the number
   returned.

5. **Whether Prefilter and Quality really disappear with OptiX.** On a machine with an NVIDIA card
   configured for OptiX in Preferences ▸ System ▸ Cycles Render Devices: set the render engine to Cycles,
   open Render Properties ▸ Sampling ▸ Denoise, set Denoiser to OpenImageDenoise and note that Prefilter
   and Quality are shown. Switch Denoiser to OptiX and note whether both vanish. Then switch Denoiser to
   Automatic and note which of the two states you get.

6. **Whether the hidden values survive and still matter.** With Denoiser set to OpenImageDenoise, set
   Quality to Fast and Prefilter to None. Switch Denoiser to OptiX. In the Python console read
   `bpy.context.scene.cycles.denoising_quality` and `bpy.context.scene.cycles.denoising_prefilter` to
   confirm the values are still there. Then render the same frame twice with OptiX — once with those two
   set to Fast/None, once with them set to High/Accurate — and compare the two images. If they are
   identical, the settings do nothing under OptiX; if they differ, they do. This is the one thing none of
   the documentation answers.

7. **The same comparison run from outside Blender**, in case the panel's hiding is the only thing that
   changes: `blender -b yourfile.blend -E CYCLES --python-expr "import bpy; c=bpy.context.scene.cycles; c.denoiser='OPTIX'; c.denoising_quality='FAST'; c.denoising_prefilter='NONE'" -o //optix_fast_ -f 1`
   and again with `'HIGH'` and `'ACCURATE'` and a different output name, then compare the two files.

---

2026-08-30 — Claude Opus 5 (Claude Code) — Settings, Blender 5.2.

No helper agents were used for this pass. Every file named above I opened myself.

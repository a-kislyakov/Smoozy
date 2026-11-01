# Settings — later questions (14 and 15), Blender 4.5

Sources consulted, all read on **2026-07-30** (the snapshot date given to me):

- Blender 4.5 Manual — `https://docs.blender.org/manual/en/4.5/`
- Blender 4.5 Python API reference — `https://docs.blender.org/api/4.5/`
- Blender 4.5 release notes — `https://developer.blender.org/docs/release_notes/4.5/`
- Cycles' own files, as supplied (searched; nothing on this topic — see the record of looking under question 14)
- One manual image: `_images/render_output_properties_format_panel.png`, the picture on the Format page

I read only the question sheet and files inside Blender Sources/. I did not open any other part of this project.

I started no helper agents. Everything below I read myself.

---

## 14. What values can a render's resolution percentage be set to?

**LABEL: documented.**

The setting is `resolution_percentage` on the render settings. The API reference gives its kind, its permitted span and its starting value in one place.

Exact words:

> resolution_percentage
>
> Percentage scale for render resolution
>
> Type:
>
> int in [1, 32767], default 100

- Source: Python API reference, `https://docs.blender.org/api/4.5/bpy.types.RenderSettings.html`
- Date read: 2026-07-30

So, as the API page prints it: whole numbers only, from 1 to 32767 inclusive, starting at 100. There is no zero, no negative number, and no fractional value such as 12.5 — the type is a whole number. One limit on that, set out under "Not covered" below and carried here so nobody reads the span without it: the page does not say whether 1 and 32767 are the outright limits or only the limits for dragging the control, and the sources do not settle which.

The manual's own description of the same control, on the Format page:

> Percentage
>
> Slider to reduce or increase the size of the rendered image relative to the Resolution values.
> This is useful for small test renders that have the same proportions as the final image.

- Source: Manual, `https://docs.blender.org/manual/en/4.5/render/output/properties/format.html`
- Date read: 2026-07-30

Note the words "or increase" — the manual says outright that the control can go above the plain resolution, not only below it. That matches a permitted top of 32767.

The API reference also shows how the value is turned into pixels, in its worked example of a render engine written in Python:

> scale = scene.render.resolution_percentage / 100.0
> self.size_x = int(scene.render.resolution_x * scale)
> self.size_y = int(scene.render.resolution_y * scale)

- Source: Python API reference, `https://docs.blender.org/api/4.5/bpy.types.RenderEngine.html`
- Date read: 2026-07-30

I quote that only for what the number means. It is example code for an engine somebody writes themselves; it is not a statement about what Blender's own engines do with the value.

**From the picture — documented.** *(Relabelled by the citation check, 2026-08-09 (Opus 5): this was labelled "observed" on the reasoning that a picture carries no words to quote. The Method rules otherwise — a picture in the manual is a document. What is visible in it is documented; anything a reader has to supply from it is inferred. The other three versions already labelled it this way. Nothing about what was seen has changed.)*

- The picture: `_images/render_output_properties_format_panel.png`, the Format panel picture shown on the manual page cited above, `https://docs.blender.org/manual/en/4.5/render/output/properties/format.html`.
- What is visible in it: a field labelled `%` reading `100%`, sitting under `Resolution X 1920 px` and `Y 1080 px`. The `%` field is drawn as a slider whose coloured bar fills the field completely at 100%.
- Date read: 2026-07-30.

That the bar is full at 100 is the whole of what is visible. Reading a dragging limit of 100 out of that would be **inferred**, and I have not relied on it below. The picture says nothing about which values the setting will take, so it carries no part of the answer above.

**Not covered.**

- The API page prints one span, `[1, 32767]`. It does not say whether that is the outright limit or the limit for dragging the slider. The manual (see question 15) says most numbers have two such spans, a "soft limit" and a "hard limit", and the API page does not label which one it is printing. So I can state the span; I cannot state from the sources which of the two it is.
- Nothing I found says how the pixel size is rounded at a given percentage for Blender's own engines, nor what happens if a percentage makes a dimension round down to zero.
- Nothing I found says whether any render engine narrows or ignores this range.
- Cycles' own supplied files say nothing about it: I searched all of them for `resolution_percentage` and for `resolution_x` and got no hits at all. It is not a Cycles setting.

**Observed — hand test, 2026-08-09.** This closes the gap named just above. The tests are written out in full, with their exact commands, in `Audits/Blender Research/Settings/Hand tests — later questions.md`, entries 1, 2 and 5.

*What was done.* Blender 4.5.12 was started from a terminal with no window, with factory settings and a throwaway settings folder, and a short script printed what Blender itself holds for the property `resolution_percentage` on the scene's render settings in the factory startup scene.

*What came back*, word for word:

```
PROP resolution_percentage type = 'INT'
PROP resolution_percentage default = 100
PROP resolution_percentage hard_min = 1
PROP resolution_percentage hard_max = 32767
PROP resolution_percentage soft_min = 1
PROP resolution_percentage soft_max = 100
PROP resolution_percentage step = 10
PROP resolution_percentage is_animatable = False
PROP resolution_percentage is_readonly = False
PROP resolution_percentage current value = 100
```

*On what kind of computer.* The rented Linux machine — Ubuntu 22.04.5 LTS on kernel 6.8, one NVIDIA GeForce RTX 5060 with 8 GB, twenty-three cores of an AMD EPYC 7K62, 49 GB of memory, no display server. The same script in Blender 4.5.12 on the Creator's Mac — a MacBook Pro with an M1 Pro chip running macOS 26.5.2 — printed the identical ten lines. The two machines agreed.

So the pair the API page prints, 1 and 32767, is the outright pair, and this setting carries a separate narrower pair, 1 and 100, for dragging.

**Observed — hand test, 2026-08-09, what values are actually taken.** In the same way, a script assigned a series of values and read each one back. On the rented Linux machine, word for word:

```
SET 1 -> 1
SET 100 -> 100
SET 32767 -> 32767
SET 0 -> 1
SET -50 -> 1
SET 32768 -> 32767
SET 40000 -> 32767
SET 2147483648 -> RAISED ValueError: bpy_struct: item.attr = val: RenderSettings.resolution_percentage value not in 'int' range ((-0x7fffffff - 1), 0x7fffffff)
SET 50.7 -> RAISED TypeError: bpy_struct: item.attr = val: RenderSettings.resolution_percentage expected an int type, not float
SET 0.4 -> RAISED TypeError: bpy_struct: item.attr = val: RenderSettings.resolution_percentage expected an int type, not float
SET '50' -> RAISED TypeError: bpy_struct: item.attr = val: RenderSettings.resolution_percentage expected an int type, not str
SET 'abc' -> RAISED TypeError: bpy_struct: item.attr = val: RenderSettings.resolution_percentage expected an int type, not str
```

The Creator's Mac printed the same lines in Blender 4.5.12, with one difference: it wrote the limits of a machine whole number in ordinary figures, `((-2147483647 -1), 2147483647)`, where the Linux machine wrote them in hexadecimal. So a fractional value is refused rather than cut down, which this answer left open.

**Observed — hand test, 2026-08-09, where the setting is read inside Blender.** Every Python file inside the installed Blender 4.5.12 was searched for `resolution_percentage`. Eighteen lines came back on the rented Linux machine and the same eighteen on the Creator's Mac: fourteen render preset files each setting it to 100, a manual-reference entry, the preview renderer, the preset list, and the line that draws it in the panel —

```
/opt/blender/4.5/4.5/scripts/startup/bl_ui/properties_output.py:119:        col.prop(rd, "resolution_percentage", text="%")
```

None of the eighteen is in Cycles' own files, which matches what is recorded under "Not covered" below.

**Outside the interface.** The span above is stated on the Python API page for the property, and that page carries no wording tying the span to a person clicking. Taking it as holding when Blender is driven from outside its own window is a step I am supplying, not something the page states — **inferred**, and the step is: the API page describes the property a script reads and writes, so what it says about the property's span is what a script meets. The manual's description of the same control as a "Slider" is a description of the interface control. Separately, I read the whole command-line arguments page and there is no argument that sets a resolution percentage — the render-related arguments listed are `-a, --render-anim`, `-f, --render-frame <frame>`, `-o, --render-output <path>` and `-F, --render-format <format>` (`https://docs.blender.org/manual/en/4.5/advanced/command_line/arguments.html`, read 2026-07-30). Beyond that, the sources do not say that the permitted values differ when Blender is driven from outside.

Beyond that, **observed — hand test, 2026-08-09**: every test recorded above was run from a terminal with no window at all, on both machines, and the numbers and the behaviour were the same as the API page prints. No step is needed for that; there was no window in any run.

**Operating system.** The sources do not say. Nothing on any page I read attaches an operating system to this property or its span, and I found no per-system note about it anywhere in the manual, the API reference or the 4.5 release notes.

**Observed — hand test, 2026-08-09.** The tests above were run in Blender 4.5.12 on two operating systems: Ubuntu 22.04.5 LTS on an x86 machine (the rented Linux machine) and macOS 26.5.2 on Apple silicon (the Creator's Mac). Every number was the same on both. The only difference was the wording of one message, quoted above, about the limits of a machine whole number. **Windows was not tested** — no Windows machine has run it — so the Windows part of this line stays untested rather than answered.

---

## 15. What does a render do when it is asked for a value outside what that setting accepts, and what does it report?

**LABEL: not in the sources**, for the question as asked about this setting. Two nearby things *are* documented, and I give them below with their own labels, but neither is a statement about a render.

**The question was ambiguous, and I answered one reading.** I read "that setting" as the resolution percentage from question 14, because the sheet's closing note says these two questions are about "that one setting". I read "asked for a value outside what that setting accepts" as: something sets it to a number outside 1–32767 — by typing it, by a script, or by the file already carrying such a value. The other reading, which I did not answer, is that the render is asked for a percentage that is *inside* the permitted span but produces an unusable picture size (for example a size that rounds to zero pixels). **That reading is unanswered, not answered with silence.** The record of looking below holds no search aimed at it — nothing on rounding, on a smallest picture size, or on a dimension reaching zero — so this document establishes nothing either way about it, and it is owed a fresh look.

I found no statement anywhere in the 4.5 manual, the 4.5 API reference or the 4.5 release notes about what a render does, or what it prints, when this setting is given a number outside its span. No error message, no warning, no refusal to start, no wording about the render stopping.

### What the sources do say, nearby

**(a) Typing a number into any numeric field — documented, but about the interface only.**

> Value Limits
>
> Most numerical values are restricted by "soft limit" and "hard limit" value ranges.
> Changing values by dragging with the mouse is restricted to the "soft limit" value range.
> Input via keyboard will allow the use of wider value ranges, but never wider than the "hard limit".

- Source: Manual, `https://docs.blender.org/manual/en/4.5/interface/controls/buttons/fields.html`
- Date read: 2026-07-30

This says the value cannot be pushed past the hard limit. It does not say what happens to the number you typed when you try — whether it is refused, cut back to the limit, or ignored — and it says nothing about a message.

The same two-limit idea is spelled out again for properties a person adds by hand:

> Min, Max
>
> The minimum/maximum value the custom property can take.
>
> ...
>
> Soft Limits
>
> Enables limits that the Property Value slider can be adjusted to
> without having to input the value numerically.

- Source: Manual, `https://docs.blender.org/manual/en/4.5/files/custom_properties.html`
- Date read: 2026-07-30

**(b) Setting a number out of range from a script — documented for script-made settings; applying it here is inferred.**

The API reference, describing how a whole-number setting is defined:

> min (int) – Hard minimum, trying to assign a value below will silently assign this minimum instead.
>
> max (int) – Hard maximum, trying to assign a value above will silently assign this maximum instead.

- Source: Python API reference, `https://docs.blender.org/api/4.5/bpy.props.html`
- Date read: 2026-07-30

The identical wording appears for fractional settings on the same page.

That is a clear statement of both halves of the question — the value is pulled back to the nearest limit, and it happens **silently**, meaning nothing is reported. The catch: this page describes settings *created by a script*. It is the reference for defining new settings, not for `resolution_percentage`, which is one of Blender's own. Carrying the statement across to `resolution_percentage` is a step the source does not take. **Inferred**, and the step is: the page describes the general behaviour of a whole-number setting that carries a minimum and a maximum, `resolution_percentage` is a whole-number setting shown with a minimum and a maximum, therefore setting it to 0 or to 40000 from a script would land on 1 or on 32767 with nothing said. A reader has to supply the assumption that Blender's own settings behave the same way as script-made ones. I could not confirm that assumption from anything I read, so I would not build on it.

**(c) What is reported.** The only word the sources offer is "silently", in (b). I found nothing that says a message, a warning, a log line or a failure of any kind results from an out-of-range number in a render setting. I checked how the API reference does describe errors — it shows them plainly where they exist, for instance:

> TypeError: bpy_struct: item.attr= val: Object.name expected a string type, not str

- Source: Python API reference, `https://docs.blender.org/api/4.5/info_gotchas_file_paths_and_encoding.html`
- Date read: 2026-07-30

So the reference does describe raised errors where there are any. It describes one for the wrong *kind* of value. It describes none for a number outside a span.

### Where I looked (the record for "not in the sources")

Searches, all across the whole of `manual_4.5` and `api_4.5` unless noted:

- `resolution_percentage` — five hits in the API reference, all listed under question 14 (the property itself, an index entry twice, an example in the render engine page, an example in the quick start). None mentions limits being exceeded.
- `Percentage` across the whole manual — the only render-output hit is the Format page quoted under question 14. Other hits are unrelated controls (shadow resolution, volume resolution, light probe fade, compositor nodes, sculpting detail, particles, grease pencil, preferences, custom property subtypes).
- `out of range`, `outside the range`, `values outside` across the whole manual — eleven hits, every one about something else: animation strips, drivers, a scale constraint, smooth modifiers, geometry-node index wrapping, mesh analysis. None about render settings.
- `out of range`, `outside the range`, `is clamped`, `clamped to` across the whole API reference — four hits: a maths helper, a drawing call, mesh index checks, and the soft-limits wording on an operator for editing custom properties. None about render settings.
- `clamp` across the manual — hits only on curve handle types and a page saying object positions are not clamped.
- `silently assign` across the API reference — eight hits, all of them the `bpy.props` wording quoted at (b).
- `hard limit`, `soft limit`, `soft_min` across the API reference — the `bpy.props` page, the operator for editing custom properties, and the pages describing a setting's own `hard_min` / `hard_max` / `soft_min` / `soft_max` readouts.
- `TypeError`, `ValueError`, `RuntimeError`, `raise` across the API reference's introductory and pitfalls pages — errors are documented for wrong kinds of value, for operators failing their checks, and for registration mistakes. Nothing for a number outside a span.
- `exit code`, `return code`, `exit status`, `reports an error`, `error is reported` across both the manual and the API reference — two hits, neither about renders or settings.
- The command-line arguments page, read in full for anything about reporting or about setting a resolution percentage — nothing on either.
- The whole of the 4.5 release notes, searched for `percentage` and `resolution` — the only hit touching this setting is a fixed crash listed on the bug-fixes page, quoted below, which is about a crash and not about out-of-range values:

  > Blender crashing when changing the frame if the render resolution percentage has just been changed following a render, and there's an image editor open that's set to render result.

  - Source: Release notes, `https://developer.blender.org/docs/release_notes/4.5/bugfixes/`
  - Date read: 2026-07-30

- All of Cycles' supplied files, searched for `resolution_percentage` and `resolution_x` — no hits.

**Not covered.**

- What a render actually does, as opposed to what the setting does when written. Even taking the inference at (b), it describes the moment of setting the value, not the render afterwards.
- Whether the file format itself can carry a percentage outside 1–32767 (for instance a file written by something other than Blender), and what a render does with it if so.
- Whether an animation curve or a driver can drive the value outside the span, and what happens then.
- What, if anything, appears in the text output of a render started from outside Blender.

**Observed — hand tests, 2026-08-09.** Four tests bear on this question. All are written out in full, with their exact commands, in `Audits/Blender Research/Settings/Hand tests — later questions.md`, entries 2, 3, 4 and 6.

**One — what happens to the value, and whether anything is said.** Blender 4.5.12 was started from a terminal with no window, with factory settings and a throwaway settings folder, and a script assigned a series of values to `resolution_percentage` on the scene's render settings and read each one back. On the rented Linux machine — Ubuntu 22.04.5 LTS on kernel 6.8, one NVIDIA GeForce RTX 5060 with 8 GB, twenty-three cores of an AMD EPYC 7K62, 49 GB of memory, no display server — the whole of what it printed:

```
SET 1 -> 1
SET 100 -> 100
SET 32767 -> 32767
SET 0 -> 1
SET -50 -> 1
SET 32768 -> 32767
SET 40000 -> 32767
SET 2147483648 -> RAISED ValueError: bpy_struct: item.attr = val: RenderSettings.resolution_percentage value not in 'int' range ((-0x7fffffff - 1), 0x7fffffff)
SET 50.7 -> RAISED TypeError: bpy_struct: item.attr = val: RenderSettings.resolution_percentage expected an int type, not float
SET 0.4 -> RAISED TypeError: bpy_struct: item.attr = val: RenderSettings.resolution_percentage expected an int type, not float
SET '50' -> RAISED TypeError: bpy_struct: item.attr = val: RenderSettings.resolution_percentage expected an int type, not str
SET 'abc' -> RAISED TypeError: bpy_struct: item.attr = val: RenderSettings.resolution_percentage expected an int type, not str
```

Nothing else was printed beside the first seven lines. The same script in Blender 4.5.12 on the Creator's Mac — a MacBook Pro with an M1 Pro chip running macOS 26.5.2 — printed the same lines, except that it wrote the limits of a machine whole number in ordinary figures, `((-2147483647 -1), 2147483647)`, instead of in hexadecimal. **The two machines agreed** on every stored value and on which values raised.

The last two lines are the control. They show this channel does carry a complaint when there is one, so "nothing was printed" for `0`, `-50`, `32768` and `40000` is a real result and not a channel that never speaks.

**Two — whether the caller is told.** In the same way, two runs per version per machine with the switch that turns a Python failure into a chosen exit status. The first raised an error on purpose and came back with that status; the second assigned 40000 and came back with 0. On the rented Linux machine in Blender 4.5.12, the control run printed:

```
CONTROL about to raise
Traceback (most recent call last):
  File "/root/ht-settings/control.py", line 2, in <module>
    raise ZeroDivisionError("deliberate control error")
ZeroDivisionError: deliberate control error
Error: script failed, file: '/root/ht-settings/control.py', exiting.
EXIT 42
```

and the run that assigned 40000 printed:

```
READBACK 32767
EXIT 0
```

The Creator's Mac gave the same pair, 42 and 0, in Blender 4.5.12. **The two machines agreed.** Because the control really did come back with 42, the 0 is a real result: a value corrected on the way in is not signalled to whatever started Blender.

**Three — what a render actually does.** Six runs per version per machine, each starting Blender with no window on the factory startup scene, setting the pixel dimensions and the percentage from a script, and rendering one frame with Cycles on the processor at one sample. The size of each written picture was read out of the file itself rather than worked out from the percentage. On the rented Linux machine in Blender 4.5.12:

```
asked 1920 1080 at 200    stored 200      wrote a picture 3840 by 2160
asked 1920 1080 at 33     stored 33       wrote a picture 633 by 356
asked 1920 1080 at 0      stored 1        wrote a picture 19 by 10
asked 1920 1080 at 40000  stored 32767    started rendering, did not finish
asked 4 4 at 1            stored 1        wrote nothing, and refused
asked 4 4 at 32767        stored 32767    wrote a picture 1310 by 1310
```

The refusal, word for word:

```
Error: Image too small
```

and no file appeared. The run at 40000 did not refuse and did not warn: it stored 32767 and began rendering a picture 32767 per cent of 1920 by 1080, in 53,284 tiles. It was still going when a 240-second limit killed it. Its last line before that:

```
Fra:1 Mem:315.03M (Peak 619.03M) | Time:03:59.02 | Remaining:04:27:39.94 | Mem:306.47M, Peak:306.47M | Scene, ViewLayer | Rendered 780/53284 Tiles, Sample 1/1
```

Searching that run's whole output for `Error`, `ERROR` and `Warning` returned nothing.

The Creator's Mac gave the same six results in Blender 4.5.12 — the same stored values, the same picture sizes, the same refusal, the same unfinished render — reaching a different number of tiles in the four minutes, which is a fact about the two machines. **The two machines agreed on every stored value, every picture size and every exit status.**

Case one of the six is the control: at 200 per cent the picture came out at 3840 by 2160 on both machines in all four versions, so the way the sizes were measured works and a plain percentage does scale the picture. The odd results in the other cases therefore mean something.

*Note added by the citation check, 2026-08-09 (Opus 5).* This check repeated all six runs on both machines in all four versions and got the same stored values, the same picture sizes and the same refusal. One step is missing from the description above: it does not say how the single frame was rendered. The output quoted here is what Blender's own `-f 1` argument produces after the script; calling the render operator from inside the script gives the same picture sizes and the same refusal but different printed output — in 5.0, 5.1 and 5.2 it raises a Python error instead of printing the `reports | ERROR Image too small` line, and the very long render prints no progress at all. The missing step has gone onto the hand-test list.

*Observed — hand tests, second batch, 2026-08-09 (Opus 5).* The missing step has now been run rather than described. Each of the six runs above was made twice over, in all four versions on both machines: once with Blender's own `-f 1` argument after the script, and once by calling the render operator from inside the script, `bpy.ops.render.render(write_still=True)`. **The output quoted above is what the `-f 1` route produces, and that is the route these six runs used.** Ninety-six runs in all, forty-eight on each machine.

The two routes agree on everything about the setting: in Blender 4.5.12, case for case, on the rented Linux machine (Ubuntu 22.04.5 LTS) and on the Creator's Mac (macOS 26.5.2), each route stored the same value and wrote a picture of the same size — 3840 by 2160, 633 by 356, 19 by 10, nothing at 4 by 4 at 1 per cent, and 1310 by 1310 — measured from the written files. Nothing above changes.

They disagree on two things. **The file is named differently:** the `-f 1` route wrote `out_0001.png`, the operator route wrote `out_.png` from the same output path, in every case that wrote anything, on both machines. **The refusal is reported differently:** the `-f 1` route printed only `Error: Image too small`, as quoted above; the operator route printed that same line *and* a Python traceback ending `RuntimeError: Error: Image too small`. In everything else 4.5's two routes printed the same Cycles progress lines — 53 to 60 lines under `-f 1` and 55 to 62 under the operator, the difference being two lines the test script itself printed. The long run at 40000 per cent printed progress under **both** routes in 4.5: on the rented Linux machine 264 lines under `-f 1` and 195 under the operator, on the Creator's Mac 298 and 260.

This agrees with the citation check on the point it raised, and corrects one edge of it: the check wrote that under the operator route "a very long render prints no progress at all", which holds in 5.0, 5.1 and 5.2 but **not** in 4.5, where the operator route prints progress just as the other route does. The control held — the 200 per cent case came out at 3840 by 2160 under both routes on both machines, so both routes were being made to do the same job. Neither standing line below is affected: both routes run outside the interface, and the two operating systems gave the same result run for run. The whole record, with the raw output, is entry 7 of `Audits/Blender Research/Settings/Hand tests — later questions.md`.

**Four — whether a saved file can hold a value outside the range: not run.** Because the value is corrected at the moment it is assigned, a file saved afterwards already holds a value inside the range, so that test could only ever report a value inside the range whatever a file can hold. It cannot come out the other way, and a test that cannot fail was not run.

**What still has no answer, and why.** What the `%` field does when an out-of-range number is typed into it, where the slider stops when dragged, and whether the Info editor or the status bar shows anything, were **not run this round**: they need eyes on a window, the rented Linux machine has no display server at all, and the Creator ran no tests. **Windows was not tested at all** — no Windows machine has run it. **Untried rather than untestable: a Windows machine can be rented, and every one this project has read reported a Server edition — see the equipment section of `Preparation/Blender Research/Method.md`.**

**Outside the interface.** The one documented statement I have, at (a), is explicitly about a person typing into a field and dragging a slider — it does not hold outside the interface, because there is no field and no dragging. The statement at (b) is about setting a value from a script, which is the outside-the-interface path, but it is about script-made settings, so it does not settle this one. For the question as asked, the sources do not say what happens outside the interface, and I will not guess.

**Observed — hand tests, 2026-08-09.** This line is now settled by observation rather than by a step. Every test above was run from a terminal with no window, on both machines: the value is corrected without a word, the exit status stays 0 against a control that proves the exit-code switch fires, and the render then proceeds at the corrected value. What has no counterpart outside the window — typing into the field, dragging the slider, the Info editor — was not run, for want of a machine with a screen.

**Operating system.** The sources do not say. I found no page anywhere in the manual, the API reference or the 4.5 release notes that attaches an operating system to how out-of-range values are handled or reported.

**Observed — hand tests, 2026-08-09.** The tests above were run in Blender 4.5.12 on two operating systems: Ubuntu 22.04.5 LTS on an x86 machine (the rented Linux machine) and macOS 26.5.2 on Apple silicon (the Creator's Mac). Every stored value, every picture size, every exit status and the refusal message all matched. The only difference was the wording of one message, quoted above, about the limits of a machine whole number. **Windows was not tested** — no Windows machine has run it — so the Windows part of this line stays untested rather than answered.

---

## Things a person with Blender 4.5 open could settle by hand in a few minutes

Pruned on 2026-08-09, after the hand tests were run. What they settled is written into the answers above, and the whole record — every command, every line the machines printed — is in `Audits/Blender Research/Settings/Hand tests — later questions.md`. Only genuinely open items are left here.

1. **What the `%` field does when a number outside the range is typed into it.** Not run this round: it needs eyes on a window. The rented Linux machine has no display server at all, and the Creator ran no tests. Note that the numbers behind the field are no longer open — the outright pair and the dragging pair were read off the property from a terminal, and are recorded under question 14.

2. **Where the slider stops when it is dragged, and what a person sees while dragging.** Not run this round, same reason. Again, only what the mouse can reach is open; the dragging pair itself is recorded under question 14 as 1 and 100.

3. **Whether the Info editor or the status bar shows anything when a value is corrected.** Not run this round, same reason. The other half of that question is settled above: on the channel a caller outside Blender can see, nothing at all was printed, and a control in the same run showed that channel does carry complaints.

4. **Whether a driver or an animated value can carry the setting outside its range.** Blender reports the property as not animatable — `is_animatable = False`, quoted under question 14 — but no driver was attached and no key was set, so this is not settled.

5. **Whether a file written by something other than Blender can hold a value outside the range.** Untouched. What was tested is a value assigned inside Blender, which is corrected at once.

**Not run, and why.** Whether a saved file can carry an out-of-range value across a save and a reopen. The value is corrected at the moment it is assigned, so a file saved afterwards already holds a value inside the range, and the test could only ever report a value inside the range. It cannot come out the other way, so it was not run.

**Not run.** Anything on Windows. No Windows machine has run it, so the Windows part of the operating-system line on both answers stays untested rather than answered.

---

2026-08-09 — Opus 5 (Claude Code) — Settings, Blender 4.5

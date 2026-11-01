# Settings — later questions (Blender 5.2)

**Sources consulted, and when.** Blender 5.2 manual, Blender 5.2 Python API reference, Blender 5.2 release notes, and Cycles' own files for 5.2 — all from the snapshot in `Blender Sources/` taken on 2026-07-30. I also opened one manual image, the Format panel screenshot shown on the manual's Format page. The work was done on 2026-08-09; every citation carries **2026-07-30** as its date read, because that is the day the copy those words came from was taken — and that includes quotations from Cycles' own files, which are part of the same snapshot. *(Levelled by the citation check, 2026-08-09 (Opus 5): the Cycles quotations here originally carried 2026-08-09, the day they were opened, which the other three versions do not do.)*

I read only the question sheet and files inside Blender Sources/. I did not open any other part of this project.

Questions answered here: 14 and 15 only.

---

## 14. What values can a render's resolution percentage be set to?

**Label: documented** that the setting holds a whole number and starts at 100. **Inferred** that the values it takes run from 1 to 32767.

Whole numbers from 1 to 32767. The starting value is 100.

The step, written out. The page gives those two numbers inside a notation it never explains anywhere I could find. Reading "(in [1, 32767])" as *the values this setting will take* is a step the reader has to supply. It is the same step question 15 relies on and labels inferred there, so it is labelled the same way here.

Exact words, Python API reference, page `https://docs.blender.org/api/5.2/bpy.types.RenderSettings.html`, read 2026-07-30:

> resolution_percentage
>
> Percentage scale for render resolution (in [1, 32767], default 100)
>
> Type:
>
> int

"int" is the page's own word for a whole number, and it is the type it gives for this setting.

The manual describes the same setting without giving numbers. Exact words, manual, page `https://docs.blender.org/manual/en/5.2/render/output/properties/format.html`, read 2026-07-30:

> Percentage
>
> Slider to reduce or increase the size of the rendered image relative to the Resolution values.
> This is useful for small test renders that have the same proportions as the final image.

So the manual says outright that the setting can increase the size as well as reduce it; it does not say by how much.

**From the picture.** The manual's Format panel screenshot — the image shown on the manual page cited just above, `https://docs.blender.org/manual/en/5.2/render/output/properties/format.html`, held in the copy as `_images/render_output_properties_format_panel.png`, read 2026-07-30 — shows a row labelled `%` holding `100%`, drawn as a filled bar rather than a plain number field. That is what is visible. Label for that: **documented**. Anything about how far that bar can be dragged is not visible in the picture and I have not claimed it. *(The caveat that once stood here — that a picture has no words to quote, so it is a description rather than the source speaking — was trimmed by the citation check, 2026-08-09 (Opus 5). The Method rules a picture in the manual to be a document like any other, and the other three versions carry no such hedge.)*

**Supporting, on what the number means.** The Python API reference's worked example for a custom render engine derives the picture size from it. Exact words, page `https://docs.blender.org/api/5.2/bpy.types.RenderEngine.html`, read 2026-07-30:

> scale = scene.render.resolution_percentage / 100.0
> self.size_x = int(scene.render.resolution_x * scale)
> self.size_y = int(scene.render.resolution_y * scale)

Cycles' own files contain the same arithmetic. Exact words, `Blender Sources/cycles_5.2/ui.py`, lines 399–401, read 2026-07-30:

>             render_scale = scene.render.resolution_percentage / 100.0
>             render_size_x = int(scene.render.resolution_x * render_scale)
>             render_size_y = int(scene.render.resolution_y * render_scale)

Label for those two: **documented** for the API page (it is documentation) and **observed** for the Cycles file. But note the step I am not taking: the API extract is example code for somebody writing their own engine, and the Cycles extract sits inside the code that draws a warning label in a panel, not inside the code that renders. Saying "this is how a Cycles render computes its picture size" would be **inferred**, and I am not asserting it.

**Ambiguity, flagged.** I read "a render's resolution percentage" as the Percentage setting in Render Properties → Format, the one the sheet's question 1 listed among render settings. There is another setting in 5.2 that release notes describe with the word percentage — exact words, release notes, page `https://developer.blender.org/docs/release_notes/5.2/cycles/`, read 2026-07-30: "In the Simplify panel, there is a new Texture Resolution percentage setting, in addition to the existing Texture Limit." That is a different setting from the one in the Format panel. Cycles' own files hold two settings whose names carry the words Texture Resolution, and they are decimal factors rather than whole-number percentages. Exact words, `Blender Sources/cycles_5.2/properties.py`, lines 1021–1035, read 2026-07-30:

>     texture_resolution: FloatProperty(
>         name="Viewport Texture Resolution",
>         default=1.0,
>         description="Scale factor for texture resolution used by viewport rendering",
>         min=0.00001, max=1.0,
>         subtype='FACTOR',
>     )
>
>     texture_resolution_render: FloatProperty(
>         name="Render Texture Resolution",
>         default=1.0,
>         description="Scale factor for texture resolution used by final rendering",
>         min=0.00001, max=1.0,
>         subtype='FACTOR',
>     )

Label: **observed** (Cycles' own files) for those lines, **documented** for the release-note sentence. **Inferred** that the setting the release note names is one of those two: the note calls it "Texture Resolution" in the Simplify panel, the file calls them "Viewport Texture Resolution" and "Render Texture Resolution", and nothing quoted here says they are the same thing — a reader has to join them by their names. On that inference, if the question meant that setting instead, the answer is 0.00001 to 1.0, starting at 1.0.

**Not covered.**
- The sources do not say whether the slider can be dragged over the whole 1–32767 range or only over a narrower band. The manual states that such a narrower band ("soft limit") generally exists for most numeric settings — quoted in full under question 15 — but it gives no numbers for this one, and the API page lists no narrower band for it.
- The sources do not say what happens to a value typed with a fraction (for example 12.5) — whether it is rounded, cut off, or refused.
- The sources do not say whether the resulting picture size is capped. Resolution X and Y are separately stated as "(in [4, 65536], default 1920)" and "(in [4, 65536], default 1080)" on the same API page, but nothing states what limits the two numbers combined.
- Nothing in the 5.2 release notes mentions this setting changing. I searched all of them for "resolution_percentage" and "percentage"; "resolution_percentage" returns nothing, and "percentage" returns three hits — the Cycles texture setting above, a Grease Pencil note about placing dots by a percentage of the stroke radius, and a bug-fix line about a node input percentage slider. *(Corrected by the citation check, 2026-08-09 (Opus 5): a fourth, "compositor", hit was listed here, and there is none — the compositor page's only near word is a "Resolution" output. The conclusion is unchanged.)*

**Observed — hand test, 2026-08-09.** This closes the gap named just above. The tests are written out in full, with their exact commands, in `Audits/Blender Research/Settings/Hand tests — later questions.md`, entries 1, 2 and 5.

*What was done.* Blender 5.2.0 was started from a terminal with no window, with factory settings and a throwaway settings folder, and a short script printed what Blender itself holds for the property `resolution_percentage` on the scene's render settings in the factory startup scene.

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

*On what kind of computer.* The rented Linux machine — Ubuntu 22.04.5 LTS on kernel 6.8, one NVIDIA GeForce RTX 5060 with 8 GB, twenty-three cores of an AMD EPYC 7K62, 49 GB of memory, no display server. The same script in Blender 5.2.0 on the Creator's Mac — a MacBook Pro with an M1 Pro chip running macOS 26.5.2 — printed the identical ten lines. The two machines agreed.

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

The Creator's Mac printed the same lines in Blender 5.2.0, with one difference: it wrote the limits of a machine whole number in ordinary figures, `((-2147483647 -1), 2147483647)`, where the Linux machine wrote them in hexadecimal. So a fractional value is refused rather than cut down, which this answer left open.

**Observed — hand test, 2026-08-09, where the setting is read inside Blender.** Every Python file inside the installed Blender 5.2.0 was searched for `resolution_percentage`. Nineteen lines came back on the rented Linux machine and the same nineteen on the Creator's Mac: fourteen render preset files each setting it to 100, a manual-reference entry, the preview renderer, the preset list, the line that draws it in the panel, and one line in Cycles' own files —

```
/opt/blender/5.2/5.2/scripts/startup/bl_ui/properties_output.py:119:        col.prop(rd, "resolution_percentage", text="%")
/opt/blender/5.2/5.2/scripts/addons_core/cycles/ui.py:399:            render_scale = scene.render.resolution_percentage / 100.0
```

Searching the same install for `texture_resolution` returned five lines, all in 5.2: a manual-reference entry, two panel lines in `cycles/ui.py` at 2362 and 2390, and the two property definitions in `cycles/properties.py` at 1021 and 1029. The same search in the installed 4.5.12, 5.0.1 and 5.1.2 returned nothing on either machine.

**Outside the interface.** The two numbers come from the Python API reference page for `RenderSettings`, which is the reference for the setting as a script sets it rather than for a panel. So they are stated for the case that matters when Blender is driven from outside its own window. Whether the same two numbers bound what a person can reach inside the window is a separate matter, and the sources point one way on it: the manual's Value Limits passage, quoted under question 15, says dragging is held to a narrower band. That band is an inside-the-window limit only, since there is no dragging outside it, and the sources give no numbers for it here. **Inferred**, and the step is that a limit described for dragging does not carry over to a value set by a script. I found no statement anywhere that the 1 and the 32767 themselves differ between the two ways of working.

**Observed — hand test, 2026-08-09.** Every test recorded above was run from a terminal with no window at all, on both machines, so this line is settled by observation rather than by a step: the limits and the values taken are the same when Blender is driven from outside its own window.

**Operating system.** The sources do not say. Nothing on either page read is qualified by operating system, and I found no per-system note for this setting. What was looked at is the pages quoted above; no separate search on operating-system words was recorded, so this line rests on those pages alone.

**Observed — hand test, 2026-08-09.** The tests above were run in Blender 5.2.0 on two operating systems: Ubuntu 22.04.5 LTS on an x86 machine (the rented Linux machine) and macOS 26.5.2 on Apple silicon (the Creator's Mac). Every number was the same on both. The only difference was the wording of one message, quoted above, about the limits of a machine whole number. **Windows was not tested** — no Windows machine has run it — so the Windows part of this line stays untested rather than answered.

---

## 15. What does a render do when it is asked for a value outside what that setting accepts, and what does it report?

**Label: inferred.** The value that gets stored silently becomes the nearest one the setting does accept, and nothing is reported.

Set it to 0 or below and the stored value becomes 1. Set it above 32767 and the stored value becomes 32767. No error, no message.

This answers the second half of the question and only part of the first. What the sources carry is what happens to the *value*. What a render then does with a value that was changed on the way in — whether it behaves any differently from a render given that same value directly — is not stated anywhere I found, and is listed under **Not covered** below.

The step I took, written out. The sources do not say this about the resolution percentage itself. They say it about whole-number settings in general, and they say the resolution percentage is one. Exact words, Python API reference, page `https://docs.blender.org/api/5.2/bpy.types.IntProperty.html`, read 2026-07-30:

> hard_max
>
> Hard maximum, trying to assign a value above will silently assign this maximum instead (in [-inf, inf], default 0, readonly)

> hard_min
>
> Hard minimum, trying to assign a value below will silently assign this minimum instead (in [-inf, inf], default 0, readonly)

The same wording appears where the API reference describes making a new whole-number setting. Exact words, page `https://docs.blender.org/api/5.2/bpy.props.html`, read 2026-07-30:

> min (int) – Hard minimum, trying to assign a value below will silently assign this minimum instead.
>
> max (int) – Hard maximum, trying to assign a value above will silently assign this maximum instead.

To get from those words to my answer, a reader has to supply two things: that the resolution percentage is a whole-number setting (the API page does say so — "Type: int"), and that the 1 and the 32767 written as "(in [1, 32767], default 100)" are that setting's hard minimum and hard maximum. The second is not stated anywhere I could find; the API pages never explain what the "(in [a, b])" notation means. That is why this is **inferred** and not documented. The word "silently" is the source's own, and it is the whole of what the sources offer on the "what does it report" half of the question: nothing is reported.

**In the interface, typing a value in.** Label: **documented** for the general rule below; **inferred** that it covers this particular setting, since the passage says "Most numerical values" and never names the resolution percentage. That is the same shape of step as the one taken above with whole-number settings, and it gets the same label. Exact words, manual, page `https://docs.blender.org/manual/en/5.2/interface/controls/buttons/fields.html`, read 2026-07-30:

> Value Limits
>
> Most numerical values are restricted by "soft limit" and "hard limit" value ranges.
> Changing values by dragging with the mouse is restricted to the "soft limit" value range.
> Input via keyboard will allow the use of wider value ranges, but never wider than the "hard limit".

So dragging cannot leave the narrower band, and typing cannot leave the wider one. This passage says nothing about any message being shown either.

**A different case, kept separate.** Label: **documented** that the page shows this error text for the case it describes. Handing the setting something that is not a number at all is not the same as handing it a number that is too big. The sources show that case only for a different setting, and it does raise an error rather than being silent. Exact words, Python API reference, page `https://docs.blender.org/api/5.2/info_gotchas_file_paths_and_encoding.html`, read 2026-07-30:

> TypeError: bpy_struct: item.attr= val: Object.name expected a string type, not str

I have not carried that across to the resolution percentage; the sources do not.

**Not covered.**
- The sources never mention the resolution percentage by name in connection with out-of-range values. Everything above is the general rule for whole-number settings.
- The sources do not say what a *render* does afterwards — only what the stored value becomes. Whether a render started with a clamped value differs in any way from one started with that value typed in is not stated.
- The sources do not say whether anything is written to the log, the console, or the status line when a value is clamped, beyond the word "silently".
- The sources do not say what happens when the value arrives another way — from a driver, from an animated value, or from a library override.
- The sources do not give the narrower "soft limit" band for this setting, so I cannot say which values are reachable by dragging as opposed to typing.

**Where I looked, for the parts I am calling silent.** I searched the whole 5.2 manual, the whole 5.2 API reference and all 5.2 release notes for: `resolution_percentage`, `resolution percentage`, `Resolution %`, `Percentage`, `Percentage scale`, `32767`, `hard limit`, `soft limit`, `hard_min`, `hard_max`, `soft_min`, `soft_max`, `Value Limits`, `clamp`, `will be clamped`, `outside the range`, `invalid value`, `out of range`, `TypeError`, `ValueError`, `RuntimeError`. I read the manual's Format page, its Number Fields page, its command-line arguments page, the API pages for `RenderSettings`, `IntProperty`, `FloatProperty`, `bpy.props`, the API's introductory and "gotchas" pages, and the Cycles release notes. I searched Cycles' own files for `resolution_percentage` — one hit, quoted under question 14 — and for `resolution`. Nothing in any of them addresses this setting's out-of-range behaviour directly.

**Observed — hand tests, 2026-08-09.** Four tests bear on this question. All are written out in full, with their exact commands, in `Audits/Blender Research/Settings/Hand tests — later questions.md`, entries 2, 3, 4 and 6.

**One — what happens to the value, and whether anything is said.** Blender 5.2.0 was started from a terminal with no window, with factory settings and a throwaway settings folder, and a script assigned a series of values to `resolution_percentage` on the scene's render settings and read each one back. On the rented Linux machine — Ubuntu 22.04.5 LTS on kernel 6.8, one NVIDIA GeForce RTX 5060 with 8 GB, twenty-three cores of an AMD EPYC 7K62, 49 GB of memory, no display server — the whole of what it printed:

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

Nothing else was printed beside the first seven lines. The same script in Blender 5.2.0 on the Creator's Mac — a MacBook Pro with an M1 Pro chip running macOS 26.5.2 — printed the same lines, except that it wrote the limits of a machine whole number in ordinary figures, `((-2147483647 -1), 2147483647)`, instead of in hexadecimal. **The two machines agreed** on every stored value and on which values raised.

The last two lines are the control. They show this channel does carry a complaint when there is one, so "nothing was printed" for `0`, `-50`, `32768` and `40000` is a real result and not a channel that never speaks.

**Two — whether the caller is told.** In the same way, two runs per version per machine with the switch that turns a Python failure into a chosen exit status. The first raised an error on purpose and came back with that status; the second assigned 40000 and came back with 0. On the rented Linux machine in Blender 5.2.0, the control run printed:

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

The Creator's Mac gave the same pair, 42 and 0, in Blender 5.2.0. **The two machines agreed.** Because the control really did come back with 42, the 0 is a real result: a value corrected on the way in is not signalled to whatever started Blender.

**Three — what a render actually does.** Six runs per version per machine, each starting Blender with no window on the factory startup scene, setting the pixel dimensions and the percentage from a script, and rendering one frame with Cycles on the processor at one sample. The size of each written picture was read out of the file itself rather than worked out from the percentage. On the rented Linux machine in Blender 5.2.0:

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
00:00.473  render           | Rendering single frame
00:00.473  reports          | ERROR Image too small
```

and no file appeared. The run at 40000 did not refuse and did not warn: it stored 32767 and began rendering a picture 32767 per cent of 1920 by 1080, in 53,284 tiles. It was still going when a 240-second limit killed it. Its last line before that:

```
03:59.293  render           | Fra: 1 | Remaining: 08:34:49.37 | Mem: 307M | Rendered 408/53284 Tiles, Sample 1/1
```

Searching that run's whole output for `Error`, `ERROR` and `Warning` returned nothing.

The Creator's Mac gave the same six results in Blender 5.2.0 — the same stored values, the same picture sizes, the same refusal, the same unfinished render — reaching a different number of tiles in the four minutes, which is a fact about the two machines. **The two machines agreed on every stored value, every picture size and every exit status.**

Case one of the six is the control: at 200 per cent the picture came out at 3840 by 2160 on both machines in all four versions, so the way the sizes were measured works and a plain percentage does scale the picture. The odd results in the other cases therefore mean something.

*Note added by the citation check, 2026-08-09 (Opus 5).* This check repeated all six runs on both machines in all four versions and got the same stored values, the same picture sizes and the same refusal. One step is missing from the description above: it does not say how the single frame was rendered. The output quoted here is what Blender's own `-f 1` argument produces after the script; calling the render operator from inside the script gives the same picture sizes and the same refusal but different printed output — in 5.0, 5.1 and 5.2 it raises a Python error instead of printing the `reports | ERROR Image too small` line, and the very long render prints no progress at all. The missing step has gone onto the hand-test list.

*Observed — hand tests, second batch, 2026-08-09 (Opus 5).* The missing step has now been run rather than described. Each of the six runs above was made twice over, in all four versions on both machines: once with Blender's own `-f 1` argument after the script, and once by calling the render operator from inside the script, `bpy.ops.render.render(write_still=True)`. **The output quoted above is what the `-f 1` route produces, and that is the route these six runs used.** Ninety-six runs in all, forty-eight on each machine.

The two routes agree on everything about the setting: in Blender 5.2.0, case for case, on the rented Linux machine (Ubuntu 22.04.5 LTS) and on the Creator's Mac (macOS 26.5.2), each route stored the same value and wrote a picture of the same size — 3840 by 2160, 633 by 356, 19 by 10, nothing at 4 by 4 at 1 per cent, and 1310 by 1310 — measured from the written files. Nothing above changes.

They disagree on three things. **The file is named differently:** the `-f 1` route wrote `out_0001.png`, the operator route wrote `out_.png` from the same output path, in every case that wrote anything. **The operator route prints no Cycles progress at all:** twelve lines in total for a render that finishes, whichever case it is, against 58 to 65 under the `-f 1` route — the same counts on both machines. Blender's own `Saved:` line is still there among those twelve, which is the control: the output was being captured, so the missing progress lines are an absence. **The refusal is reported differently:** neither of the two lines quoted above, `render | Rendering single frame` and `reports | ERROR Image too small`, appears anywhere under the operator route. What appears instead is `RuntimeError: Error: Image too small` inside a Python traceback, followed by a bare `Error: Image too small`. Both machines, same shape.

The long run at 40000 per cent shows the gap at its widest. Under `-f 1` it printed 195 lines on the rented Linux machine and 342 on the Creator's Mac in ninety seconds, all of them progress. Under the operator route it printed nothing whatever — a repeat with the script's own output flushed, cut off at sixty seconds, printed exactly seven lines and not one line of Blender's own. From the rented Linux machine:

```
CYCLES samples 1 device CPU
VERSION 5.2.0 LTS
ASKED 1920 1080 40000
STORED 1920 1080 32767
ROUTE B -- the frame is rendered by calling the render operator from inside this script
FILEPATH /root/b2-settings/run/B2/d/5.2/out_
EXIT 124
```

The Creator's Mac printed the same seven lines in Blender 5.2.0, including the same `STORED 1920 1080 32767`, differing only in the output path and in the way its runner reports a killed run.

The stored value is 32767 either way, so the answer above stands; only the printing differs. This agrees with the citation check's account. The control held — the 200 per cent case came out at 3840 by 2160 under both routes on both machines. Neither standing line below is affected: both routes run outside the interface, and the two operating systems gave the same result run for run. The whole record, with the raw output, is entry 7 of `Audits/Blender Research/Settings/Hand tests — later questions.md`.

**Four — whether a saved file can hold a value outside the range: not run.** Because the value is corrected at the moment it is assigned, a file saved afterwards already holds a value inside the range, so that test could only ever report a value inside the range whatever a file can hold. It cannot come out the other way, and a test that cannot fail was not run.

**What still has no answer, and why.** What the `%` field does when an out-of-range number is typed into it, where the slider stops when dragged, and whether the Info editor or the status bar shows anything, were **not run this round**: they need eyes on a window, the rented Linux machine has no display server at all, and the Creator ran no tests. **Windows was not tested at all** — no Windows machine has run it. **Untried rather than untestable: a Windows machine can be rented, and every one this project has read reported a Server edition — see the equipment section of `Preparation/Blender Research/Method.md`.**

**Outside the interface.** The silent-clamping wording is about assigning a value, which is what a script does; the manual's Value Limits passage is about dragging and typing in a field, which only exists inside the window. So the part that matters when Blender is driven from outside is the clamping, and by the same reading it applies there. One consequence, **inferred**: because nothing is raised, there is nothing for the command-line switch that turns Python errors into an exit code to catch. Exact words, manual, page `https://docs.blender.org/manual/en/5.2/advanced/command_line/arguments.html`, read 2026-07-30:

> --python-exit-code <code>
>
> Set the exit-code in [0..255] to exit if a Python exception is raised
> (only for scripts executed from the command line), zero disables.

The step: that switch acts on a raised exception; a clamp is described as silent, so no exception is raised, so the switch does not fire. A caller outside Blender therefore gets no signal that its number was changed.

**Observed — hand tests, 2026-08-09.** This line is now settled by observation rather than by a step. Every test above was run from a terminal with no window, on both machines: the value is corrected without a word, the exit status stays 0 against a control that proves the exit-code switch fires, and the render then proceeds at the corrected value. What has no counterpart outside the window — typing into the field, dragging the slider, the Info editor — was not run, for want of a machine with a screen.

**Operating system.** The sources do not say. Nothing on any of the pages read is qualified by operating system, and I found no per-system note about value limits.

**Observed — hand tests, 2026-08-09.** The tests above were run in Blender 5.2.0 on two operating systems: Ubuntu 22.04.5 LTS on an x86 machine (the rented Linux machine) and macOS 26.5.2 on Apple silicon (the Creator's Mac). Every stored value, every picture size, every exit status and the refusal message all matched. The only difference was the wording of one message, quoted above, about the limits of a machine whole number. **Windows was not tested** — no Windows machine has run it — so the Windows part of this line stays untested rather than answered. What was looked at is the pages listed under "Where I looked" above; no separate search on operating-system words was recorded, so this line rests on those pages alone.

---

## Things only a running Blender 5.2 can settle

Pruned on 2026-08-09, after the hand tests were run. What they settled is written into the answers above, and the whole record — every command, every line the machines printed — is in `Audits/Blender Research/Settings/Hand tests — later questions.md`. Only genuinely open items are left here.

1. **What the `%` field does when a number outside the range is typed into it.** Not run this round: it needs eyes on a window. The rented Linux machine has no display server at all, and the Creator ran no tests. Note that the numbers behind the field are no longer open — the outright pair and the dragging pair were read off the property from a terminal, and are recorded under question 14.

2. **Where the slider stops when it is dragged, and what a person sees while dragging.** Not run this round, same reason. Again, only what the mouse can reach is open; the dragging pair itself is recorded under question 14 as 1 and 100.

3. **Whether the Info editor or the status bar shows anything when a value is corrected.** Not run this round, same reason. The other half of that question is settled above: on the channel a caller outside Blender can see, nothing at all was printed, and a control in the same run showed that channel does carry complaints.

4. **Whether a driver or an animated value can carry the setting outside its range.** Blender reports the property as not animatable — `is_animatable = False`, quoted under question 14 — but no driver was attached and no key was set, so this is not settled.

5. **Whether a file written by something other than Blender can hold a value outside the range.** Untouched. What was tested is a value assigned inside Blender, which is corrected at once.

**Not run, and why.** Whether a saved file can carry an out-of-range value across a save and a reopen. The value is corrected at the moment it is assigned, so a file saved afterwards already holds a value inside the range, and the test could only ever report a value inside the range. It cannot come out the other way, so it was not run.

**Not run.** Anything on Windows. No Windows machine has run it, so the Windows part of the operating-system line on both answers stays untested rather than answered.

---

2026-08-09 — Opus 5 (Claude Code) — Settings, Blender 5.2

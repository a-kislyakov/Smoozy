# Settings — later questions, second pass (Blender 5.2)

Questions answered: 14 and 15 only.

**Sources consulted, and when.** Blender 5.2 manual, Blender 5.2 Python API documentation, Blender 5.2 release notes, and Cycles' own files for 5.2 — all as supplied in `Blender Sources/`, snapshot taken 2026-07-30. I read them on 2026-08-09. That snapshot date is the "date read" on every citation below, as instructed.

I read only the question sheet and files inside Blender Sources/. I did not open any other part of this project.

---

## 14. What values can a render's resolution percentage be set to?

**Label: documented** (with one inferred part, marked as such below.)

### The answer

The setting is `resolution_percentage`, on a scene's render settings. The Python API documentation states its accepted values outright:

> resolution_percentage
>
> Percentage scale for render resolution (in [1, 32767], default 100)
>
> Type:
>
> int

- Source: Blender 5.2 Python API documentation, https://docs.blender.org/api/5.2/bpy.types.RenderSettings.html
- Date read: 2026-07-30

So: **whole numbers from 1 to 32767, and the value it starts at is 100.** There is no 0 and no negative value in that range, and because the type is stated as `int`, there are no fractional percentages such as 50.5.

The manual describes the same setting in the Output Properties, under the Format panel, where it is labelled simply "Percentage":

> Percentage
>
> Slider to reduce or increase the size of the rendered image relative to the Resolution values.
> This is useful for small test renders that have the same proportions as the final image.

- Source: Blender 5.2 manual, https://docs.blender.org/manual/en/5.2/render/output/properties/format.html
- Date read: 2026-07-30

Note the manual says the slider can "reduce or increase" — so values above 100 are meant, not only values below it. That matches the upper end of 32767 in the API documentation.

The manual gives a worked example of what a value means, in its description of the things you can put into a render output path:

> resolution_x / resolution_y:
>
> The x and y resolution of the rendered image.
> This factors in the resolution scale as well, so if the scene resolution is
> 1000x600 and the scale is 50%, then resolution_x and resolution_y
> will be 500 and 300, respectively.

- Source: Blender 5.2 manual, https://docs.blender.org/manual/en/5.2/files/file_paths.html
- Date read: 2026-07-30

### The picture

The manual's Format panel page carries a screenshot, `[image: _images/render_output_properties_format_panel.png]`. I opened it. It shows a panel headed "Format" with fields "Resolution X" (1920 px), "Y" (1080 px), and below them a field labelled "%" reading **100%**, drawn as a slider whose coloured bar is filled all the way across. Below that are Aspect X/Y, Render Region, Crop to Render Region and Frame Rate.

- Label for what the picture shows: **documented** (it is part of the manual).
- Source: Blender 5.2 manual, https://docs.blender.org/manual/en/5.2/render/output/properties/format.html (image `_images/render_output_properties_format_panel.png`)
- Date read: 2026-07-30

### The inferred part: two different ranges

The manual says numeric fields in Blender generally have two ranges:

> Most numerical values are restricted by "soft limit" and "hard limit" value ranges.
> Changing values by dragging with the mouse is restricted to the "soft limit" value range.
> Input via keyboard will allow the use of wider value ranges, but never wider than the "hard limit".

- Source: Blender 5.2 manual, https://docs.blender.org/manual/en/5.2/interface/controls/buttons/fields.html
- Date read: 2026-07-30

**Inferred:** the range `[1, 32767]` quoted from the API documentation is the wider "hard limit" range, not the range you can reach by dragging the slider; the drag range ends at or near 100. The step I took: the screenshot shows the slider's bar completely filled at a value of 100. A slider's bar is filled in proportion to how far the value has travelled along the slider's own range. If the slider's own range ran to 32767, a value of 100 would show the bar essentially empty. It shows it full, so the slider's own range ends at about 100 — which is a narrower range than 1–32767, and therefore the API figure is the wider one. A reader has to accept that a filled bar means "at the top of the slider's range" to follow this, so it is inferred, not documented.

Nothing in the sources I searched states the soft limit's number outright.

**Not covered.** I did not find any statement of the soft (drag) limit as a number. I did not find any statement of what happens when a percentage produces a pixel size that rounds down to zero (for example 1% of a very small resolution) — the sources are silent on that. This answer is about the render's own resolution percentage, `scene.render.resolution_percentage`, and settles nothing about the other percentage-shaped settings I found while searching, which are separate: the Cycles Simplify "Texture Resolution" percentage, the EEVEE shadow map resolution percentage, the EEVEE film overscan percentage, the Video Sequencer preview resolution, and `preview_pixel_size` (documented on the same API page as `AUTO`/`1`/`2`/`4`/`8`, a viewport setting, not this one). It also settles nothing about whether the numbers differ in other Blender versions — I looked only at 5.2.

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

**Outside the interface.** The range and default come from the Python API documentation, which describes the data property itself rather than any window or panel, so the accepted values are a property of the setting and not of the interface. The part that is interface-only is the slider: dragging, and the narrower soft range it is limited to, only exist where there is a window to drag in. I searched the manual's command-line pages and found **no command-line argument that sets a resolution percentage** (see the record below), so from outside the window the setting is reached as a data property. The sources do not say anything further about a difference between the two situations.

**Observed — hand test, 2026-08-09.** Every test recorded above was run from a terminal with no window at all, on both machines, so this line is settled by observation rather than by a step: the limits and the values taken are the same when Blender is driven from outside its own window.

**Operating system.** The sources state nothing operating-system-specific about this setting. The API page gives one range and one default with no per-system qualification, and the manual's Format page makes no mention of any operating system. The only operating-system remark I found anywhere near this material is a command-line switch about high-resolution displays on MacBooks, which concerns the screen and not the render's resolution percentage.

**Observed — hand test, 2026-08-09.** The tests above were run in Blender 5.2.0 on two operating systems: Ubuntu 22.04.5 LTS on an x86 machine (the rented Linux machine) and macOS 26.5.2 on Apple silicon (the Creator's Mac). Every number was the same on both. The only difference was the wording of one message, quoted above, about the limits of a machine whole number. **Windows was not tested** — no Windows machine has run it — so the Windows part of this line stays untested rather than answered.

---

## 15. What does a render do when it is asked for a value outside what that setting accepts, and what does it report?

**The question is ambiguous, and I say so plainly.** "Outside what that setting accepts" can mean two different things, because the setting has two ranges (see question 14). I answer the reading I think is meant — a value outside the full accepted range, that is below 1 or above 32767 — and I also cover the second reading, a value merely outside the narrower drag range but still inside 1–32767, because the sources answer that one too.

**Label: inferred** for the main answer, built out of two documented statements. **Not in the sources** for the second half of the question — what it reports.

### Reading one: a value below 1 or above 32767

The Python API documentation states, for RNA integer properties, what an assignment outside the limits does:

> hard_max
>
> Hard maximum, trying to assign a value above will silently assign this maximum instead (in [-inf, inf], default 0, readonly)

> hard_min
>
> Hard minimum, trying to assign a value below will silently assign this minimum instead (in [-inf, inf], default 0, readonly)

- Source: Blender 5.2 Python API documentation, https://docs.blender.org/api/5.2/bpy.types.IntProperty.html (the page describes itself as "RNA integer number property definition")
- Date read: 2026-07-30

The same words appear where integer properties are created:

> min (int) – Hard minimum, trying to assign a value below will silently assign this minimum instead.
>
> max (int) – Hard maximum, trying to assign a value above will silently assign this maximum instead.

- Source: Blender 5.2 Python API documentation, https://docs.blender.org/api/5.2/bpy.props.html
- Date read: 2026-07-30

**Inferred:** asking for a resolution percentage below 1 gives you 1, and asking for one above 32767 gives you 32767; the render then proceeds at that adjusted value. The step I took: the API documentation states the resolution percentage's type is `int` with a range of `[1, 32767]` (quoted in full under question 14), and the pages above state what an out-of-limits assignment does to an integer property of that kind. Joining the two requires the reader to accept that the resolution percentage is one of the integer properties those pages describe. That step is small but it is a step, so the label is inferred and not documented.

**What it reports: not in the sources, and the wording points to nothing.** The only word the sources offer on reporting is "silently", in the quotes above. I searched for any message, warning or error connected to this and found none (see the record below). I take "silently" at face value only as far as it goes: the documentation uses that word, it does not additionally describe any message. I did not find a page anywhere in the manual, the API documentation or the 5.2 release notes that shows what is printed, if anything, when a render is started with an out-of-range resolution percentage.

Worth setting beside it: where the manual *does* want to describe clamping and say so, it does. For the video encoding quality setting it writes "CRF values outside the allowed range are clamped to the nearest allowed value" (Blender 5.2 manual, https://docs.blender.org/manual/en/5.2/render/output/properties/output.html, read 2026-07-30). No comparable sentence exists on the Format page for the resolution percentage.

### Reading two: a value outside the slider's drag range but inside 1–32767

**Documented**, for someone working in the window:

> Most numerical values are restricted by "soft limit" and "hard limit" value ranges.
> Changing values by dragging with the mouse is restricted to the "soft limit" value range.
> Input via keyboard will allow the use of wider value ranges, but never wider than the "hard limit".

- Source: Blender 5.2 manual, https://docs.blender.org/manual/en/5.2/interface/controls/buttons/fields.html
- Date read: 2026-07-30

So the value is accepted if it is typed, and simply cannot be reached by dragging. The manual describes no message in either case.

### What a render does with the value it ends up with

**Observed**, from Cycles' own files, that this panel's drawing code divides the percentage by 100 and multiplies each resolution figure by the result, cutting the answer down to a whole number. That whatever value survives the limits is therefore what scales the rendered picture is **inferred**, not observed, and I do not rest anything on it: the numbers computed in these three lines feed nothing but a warning label in a panel. *(Narrowed and relabelled by the citation check, 2026-08-09 (Opus 5), which found the original wording reaching past what the file carries — and contradicting this document's own caution three lines below.)*

> ```
> render_scale = scene.render.resolution_percentage / 100.0
> render_size_x = int(scene.render.resolution_x * render_scale)
> render_size_y = int(scene.render.resolution_y * render_scale)
> ```

- Source: Cycles' own files, `cycles_5.2/ui.py`, lines 399–401, inside the sampling panel's tile-size advice
- Date read: 2026-07-30

The same three lines appear in the manual's own worked example of writing a render engine:

> scale = scene.render.resolution_percentage / 100.0
> self.size_x = int(scene.render.resolution_x * scale)
> self.size_y = int(scene.render.resolution_y * scale)

- Source: Blender 5.2 Python API documentation, https://docs.blender.org/api/5.2/bpy.types.RenderEngine.html
- Date read: 2026-07-30

I am not taking these past what they say. They show the arithmetic a value goes through. They do not show what happens to an out-of-range value, and I make no claim that they do.

**Not covered.** I found nothing on any of these, and they remain open: what text, if any, is printed when the value is adjusted; whether the adjustment is recorded anywhere a caller could see it; what happens if the value handed over is not a whole number at all, or is larger than a machine integer can hold; what happens to a file saved with an out-of-range value already stored in it; and whether a driver or an animated value feeding this setting is treated the same way as a direct assignment. The 5.2 release notes say nothing about any of it.

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

**Outside the interface.** The clamping statements are on the Python API pages, which describe the property itself and are not tied to a window, so on my reading they hold when Blender is driven from outside. The manual's soft-limit and hard-limit paragraph is explicitly about dragging and keyboard entry in a field, and that half has no meaning outside the window — with no window there is no drag range, only the full range. What is genuinely unsettled is the reporting half of the question, which is exactly where being outside the interface matters most: I found nothing in the sources describing what a run started from outside prints when it is given a value it cannot use. Note also that I found no command-line argument that sets this value at all, so from outside it can only be reached as a data property.

**Observed — hand tests, 2026-08-09.** This line is now settled by observation rather than by a step. Every test above was run from a terminal with no window, on both machines: the value is corrected without a word, the exit status stays 0 against a control that proves the exit-code switch fires, and the render then proceeds at the corrected value. What has no counterpart outside the window — typing into the field, dragging the slider, the Info editor — was not run, for want of a machine with a screen.

**Operating system.** The sources state nothing operating-system-specific here. The clamping wording on the API pages and the value-limits paragraph in the manual are written without any per-system qualification, and I found no page that varies either by operating system.

**Observed — hand tests, 2026-08-09.** The tests above were run in Blender 5.2.0 on two operating systems: Ubuntu 22.04.5 LTS on an x86 machine (the rented Linux machine) and macOS 26.5.2 on Apple silicon (the Creator's Mac). Every stored value, every picture size, every exit status and the refusal message all matched. The only difference was the wording of one message, quoted above, about the limits of a machine whole number. **Windows was not tested** — no Windows machine has run it — so the Windows part of this line stays untested rather than answered.

---

## Record of where I looked

This applies to both questions. Everything below was searched across the whole of the 5.2 manual, the whole of the 5.2 Python API documentation, the whole of the 5.2 release notes and the Cycles 5.2 files, unless I say otherwise. All searches ignored letter case.

**Terms searched, and what came back:**

- `resolution_percentage` — six hits in total: the API's Render Settings page (the range and default), the API's quickstart page (an example of reading it in the console), the API's render-engine page (the scale arithmetic), Cycles' `ui.py` line 399, and two index pages. No hit anywhere in the manual's own text and none in the release notes.
- `percentage` across the manual — around sixty hits. I went through the render-related ones: the Format panel page (the one that matters), the Cycles Simplify page (volume resolution), the EEVEE sampling page (shadow map resolution), the EEVEE film page (overscan), the EEVEE light probe page, the camera page (passepartout), and the Freestyle 2D-transform page. All except the Format page are different settings. I also ruled out the Preferences page, whose "Percentage: Values are expressed as a percentage between 0 and 100" is about how factor values are *displayed*, not about this setting.
- `32767` — the API's Render Settings page plus about twenty unrelated properties that share the same upper bound. No page explains the number.
- `render scale`, `resolution scale`, `render_percentage`, `percentage scale`, `scale for render`, `downscale`, `Resolution %`, `% of the resolution`, `half resolution`, `50%`, `test render`, `preview resolution` — nothing new about the render's percentage beyond what is cited above. `resolution scale` did turn up the file-paths page example, which I have quoted.
- `soft limit`, `hard limit`, `soft min`, `soft max`, `hard min`, `hard max` — the manual's Value Limits paragraph, the custom-properties page, and the API's property pages (`IntProperty`, `FloatProperty`, `bpy.props`). All quoted or ruled out above.
- `silently` — fifteen hits: eight on `bpy.props`, two on `bpy.types.IntProperty`, two on `bpy.types.FloatProperty`, one on the manual's data-blocks page, and one each in the release notes' bug-fix and compositor pages. The ones that bear on this question are the hard-minimum and hard-maximum wording on the API's property pages; the rest are unrelated. This is the only word in the sources touching what is or is not reported. *(Count corrected from "nine hits" by the citation check, 2026-08-09 (Opus 5); the conclusion is unchanged.)*
- `clamp`, `will be clamped`, `value is clamped`, `is limited to`, `restricted to`, `out of range`, `outside the range`, `outside the allowed`, `invalid value`, `not accepted` — the render-related hits were the CRF paragraph on the Output page (quoted above, a different setting), three Freestyle crease-angle pages, and a scattering of modifier and sculpt pages. Nothing about the resolution percentage.
- `TypeError`, `ValueError`, `RuntimeError` in the API's information pages — hits are about file-path encoding, operator poll failures and add-on registration. None concern assigning a number outside a property's range.

**Pages I opened in full or in part, and why:**

- Manual, Output Properties → Format (the whole page, 63 lines) — the only page in the manual describing this setting. It ends at the Frame Rate section; there is no further text about the percentage.
- Manual, that page's screenshot `render_output_properties_format_panel.png` — to see what the field offers.
- Manual, Interface → Controls → Buttons → Fields — for number fields, sliders and the Value Limits paragraph.
- Manual, Files → File Paths — for the worked 50% example.
- Manual, Output Properties → Output — checked the encoding section, where the manual does spell out clamping for a different setting.
- Manual, Output Properties → index — to be sure there was no separate page for resolution or dimensions. The sections are Format, Frame Range, Time Stretching, Stereoscopy, Output, Color Management, Pixel Density, Encoding, Metadata, Post Processing.
- Manual, Advanced → Working Limits — read for any statement of a resolution or percentage ceiling. It covers space precision, the frame maximum, and text-field lengths. Nothing about resolution percentage.
- Manual, Advanced → Command Line → arguments, index, render — searched the whole `advanced` folder for `resolution` and `percent`. The single hit is a switch about high-resolution displays on MacBooks. **There is no command-line argument that sets a resolution percentage.**
- API, `bpy.types.RenderSettings` — the property, its range, its default, its type; also checked `preview_pixel_size` next to it, which is a separate viewport setting.
- API, `bpy.types.IntProperty` — read in full, for hard and soft limits.
- API, `bpy.props` — the `IntProperty` creation signature and its parameter descriptions.
- API, `bpy.types.RenderEngine` — the render-engine example showing the scale arithmetic.
- API, `bpy.ops.wm` — read the entries for `context_set_int`, `context_set_string` and `context_set_value`, since setting a property through an operator is a plausible route. None of them describes clamping or any message.
- API, `info_api_reference` — checked for an explanation of the `(in [x, y], default n)` notation used throughout the API pages. **It does not explain it.** That is why the hard-versus-soft point under question 14 is inferred rather than documented.
- Release notes 5.2 — searched every subject file for `resolution`, `percentage`, `clamp` and `limit`. The only percentage-related entry is a new Cycles Simplify "Texture Resolution percentage", which is a different setting; and a user-interface entry about a new right-click "Swap Dimensions" on Resolution X and Y. Nothing changes the render's resolution percentage, its range, or how out-of-range values are handled.
- Cycles' own files — searched every file for `resolution_percentage`, `resolution` and `percent`. The only use of the render's percentage is `ui.py` lines 399–401. The `source` folder holds only `kernel` and `util`; it contains no occurrence of the setting. Cycles does not define this property and does not appear to constrain it.

**What I did not do.** I did not go to the web, and did not need to. When this reading was done I did not run Blender, and nothing in the reading above is labelled from something I ran. *(Corrected by the citation check, 2026-08-09 (Opus 5). The sentence used to say flatly "I did not run Blender", which stopped being true once the hand tests of 2026-08-09 were written into this document. Those tests, and only those, are labelled from runs.)*

---

## Things a person with Blender 5.2 open could settle by hand in a few minutes

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

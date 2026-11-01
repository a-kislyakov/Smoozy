# Settings — later questions (Blender 5.1)

**Sources consulted, and when.** Blender's manual for 5.1, Blender's Python API reference for 5.1, the 5.1 release notes, and Cycles' own files for 5.1 — all as the copies held in `Blender Sources/`, taken on 2026-07-30. That snapshot date is the date read for every finding below. I also looked at one picture from the manual (named in the answer that uses it). I did not use the web.

I read only the question sheet and files inside Blender Sources/. I did not open any other part of this project.

Questions answered here: 14 and 15 only.

---

## 14. What values can a render's resolution percentage be set to?

**Answer.** The documentation gives the setting as a whole number in the range 1 to 32767, with a starting value of 100. Whether that pair is the limit a person or a script is actually held to is not settled by the sources — see *Not covered* below.

**Label: documented** for the kind, the range as the documentation prints it, and the starting value. One further step in this answer is marked **inferred** where it is taken.

The setting is `resolution_percentage` on a scene's render settings. The Python API reference states its kind, its range and its starting value on one line:

> resolution_percentage
>
> Percentage scale for render resolution (in [1, 32767], default 100)
>
> Type:
>
> int

- Source: Blender's Python API documentation, https://docs.blender.org/api/5.1/bpy.types.RenderSettings.html
- Date read: 2026-07-30

The manual describes the same control in the Format panel, and says plainly that it goes both ways — smaller and larger:

> Percentage
>
> Slider to reduce or increase the size of the rendered image relative to the Resolution values.
> This is useful for small test renders that have the same proportions as the final image.

- Source: Blender's manual, https://docs.blender.org/manual/en/5.1/render/output/properties/format.html
- Date read: 2026-07-30

The manual's picture of that panel shows the control as a field labelled `%`, reading `100%`, with its slider bar filled all the way across.

- Picture: `_images/render_output_properties_format_panel.png`, shown on https://docs.blender.org/manual/en/5.1/render/output/properties/format.html
- Date read: 2026-07-30
- Label for what the picture gives: **documented** (the words `%` and `100%` are visible in it, and the bar is visibly full). That the bar being full at 100 means dragging stops at 100 is **inferred**, and I have not relied on it.

Two more places show the value being divided by 100. Neither is a statement about the setting, and I set out below exactly what each one gives.

The API page for writing a render engine carries an example that divides the value by 100 and multiplies the pixel dimensions by the result:

> scale = scene.render.resolution_percentage / 100.0
> self.size_x = int(scene.render.resolution_x * scale)
> self.size_y = int(scene.render.resolution_y * scale)

- Source: Blender's Python API documentation, https://docs.blender.org/api/5.1/bpy.types.RenderEngine.html
- Date read: 2026-07-30
- What it gives: those lines are on the page, as an example for somebody writing their own render engine. They are an illustration of what such an engine may do, not a statement about what Blender's own render does. That is the same limit I apply to a different API page in question 15.

Cycles' own files carry one line that divides the value by 100:

> `render_scale = scene.render.resolution_percentage / 100.0`

- File: `cycles_5.1/ui.py`, line 399, inside the panel-drawing code that warns about render tiling
- Date read: 2026-07-30
- What it gives, **observed**: that this line is in that file, in that place. It does not carry what a render does with the value; that would be a step beyond reading the file.

That the value is therefore a plain scale factor, and not something with its own units, is **inferred**. The step is that the API page's own words call it a "Percentage scale for render resolution", and both places above divide it by 100 before use, so it behaves as a plain multiplier. A reader has to supply that step, so it is not documented.

**Not covered.**

- The range `[1, 32767]` is what the API page prints. That page does not say whether that pair is the *hard* limit or the *soft* limit. Blender's integer settings carry both — the API reference lists `hard_min`, `hard_max`, `soft_min` and `soft_max` as separate readable facts about any integer setting (https://docs.blender.org/api/5.1/bpy.types.IntProperty.html, read 2026-07-30) — and the two can differ. So I can say 1 and 32767 are the numbers the documentation gives for this setting, and nothing more. Both ways are open from the sources: if the printed pair is the hard limit, a person dragging the slider may be stopped somewhere earlier; if it is the soft limit, a number typed in may be allowed past 32767. Neither can be ruled out here. See the by-hand check at the end.
- Nothing in the sources says what happens to the resulting pixel width and height when a large percentage pushes them past the limits given for `resolution_x` and `resolution_y` (documented as `[4, 65536]` on the same API page). I did not chase that; it is a different setting.
- This answer is about the render's resolution percentage — the `%` field in the Format panel, `resolution_percentage`. It is not about other percentage-like or scale-like controls elsewhere in Blender.
- Nothing in the 5.1 release notes mentions this setting. I searched the whole of `release_notes_5.1` for "resolution" and "percentage"; the four hits are about multiresolution tangent data, a curves resolution attribute, monitor resolution scale, and a video-sequencer proxy display percentage. None is this setting.

**Observed — hand test, 2026-08-09.** This closes the gap named just above. The tests are written out in full, with their exact commands, in `Audits/Blender Research/Settings/Hand tests — later questions.md`, entries 1, 2 and 5.

*What was done.* Blender 5.1.2 was started from a terminal with no window, with factory settings and a throwaway settings folder, and a short script printed what Blender itself holds for the property `resolution_percentage` on the scene's render settings in the factory startup scene.

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

*On what kind of computer.* The rented Linux machine — Ubuntu 22.04.5 LTS on kernel 6.8, one NVIDIA GeForce RTX 5060 with 8 GB, twenty-three cores of an AMD EPYC 7K62, 49 GB of memory, no display server. The same script in Blender 5.1.2 on the Creator's Mac — a MacBook Pro with an M1 Pro chip running macOS 26.5.2 — printed the identical ten lines. The two machines agreed.

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

The Creator's Mac printed the same lines in Blender 5.1.2, with one difference: it wrote the limits of a machine whole number in ordinary figures, `((-2147483647 -1), 2147483647)`, where the Linux machine wrote them in hexadecimal. So a fractional value is refused rather than cut down, which this answer left open.

**Observed — hand test, 2026-08-09, where the setting is read inside Blender.** Every Python file inside the installed Blender 5.1.2 was searched for `resolution_percentage`. Nineteen lines came back on the rented Linux machine and the same nineteen on the Creator's Mac: fourteen render preset files each setting it to 100, a manual-reference entry, the preview renderer, the preset list, the line that draws it in the panel, and one line in Cycles' own files —

```
/opt/blender/5.1/5.1/scripts/startup/bl_ui/properties_output.py:119:        col.prop(rd, "resolution_percentage", text="%")
/opt/blender/5.1/5.1/scripts/addons_core/cycles/ui.py:399:            render_scale = scene.render.resolution_percentage / 100.0
```

**Outside the interface.** Holds. The range and starting value come from the Python API reference, which describes the setting itself rather than a window; the same page is what a script reads and writes, and scripts are how this setting is reached when Blender is driven from outside. This last step — that the documented range applies equally when no window is open — is **inferred** from the fact that the API page states the range as a property of the setting and never mentions the interface. Note also that the command-line arguments page for 5.1 lists no argument that sets a resolution percentage (I read the whole Render Options and Format Options sections at https://docs.blender.org/manual/en/5.1/advanced/command_line/arguments.html, and searched every source for "render-percentage" and "render_percentage" with no hits), so from outside, this setting is reached by editing the file or by running Python.

**Observed — hand test, 2026-08-09.** Every test recorded above was run from a terminal with no window at all, on both machines, so this line is settled by observation rather than by a step: the limits and the values taken are the same when Blender is driven from outside its own window.

**Operating system.** The sources say nothing that ties this setting, its range or its starting value to any operating system. The pages this answer rests on — the `bpy.types.RenderSettings` page, the Format panel page and the command-line arguments page, all named above — draw no such distinction. No separate search for operating-system wording is recorded for this question.

**Observed — hand test, 2026-08-09.** The tests above were run in Blender 5.1.2 on two operating systems: Ubuntu 22.04.5 LTS on an x86 machine (the rented Linux machine) and macOS 26.5.2 on Apple silicon (the Creator's Mac). Every number was the same on both. The only difference was the wording of one message, quoted above, about the limits of a machine whole number. **Windows was not tested** — no Windows machine has run it — so the Windows part of this line stays untested rather than answered.

---

## 15. What does a render do when it is asked for a value outside what that setting accepts, and what does it report?

I read this as: what happens when the resolution percentage is given a number outside its accepted range. That is the reading question 14 sets up, and the sheet's closing note says these two are about "that one setting". Another reading is possible — "any setting of a render, not just this one" — and where the sources say something general rather than specific to the percentage, I have said so.

**Answer.** For the percentage specifically, **not in the sources.** No source states what Blender does with an out-of-range resolution percentage, and none states that it reports anything at all.

Two general statements sit close to the question, and they do not carry the same weight. One says only that a number typed into a field never goes wider than the hard limit; it does not say what becomes of a number offered beyond it, and it says nothing at all about what is reported. The other says an out-of-range value assigned from Python is silently replaced by the limit. Neither is written about this setting, so applying either to the percentage is a step I have to take, and I label it accordingly.

**Label: not in the sources** (for the percentage itself), with two **inferred** readings below.

### What the sources do say

**1. For most number fields, a typed number never goes wider than the hard limit.** The manual, describing number fields in general:

> Value Limits
>
> Most numerical values are restricted by “soft limit” and “hard limit” value ranges.
> Changing values by dragging with the mouse is restricted to the “soft limit” value range.
> Input via keyboard will allow the use of wider value ranges, but never wider than the “hard limit”.

- Source: Blender's manual, https://docs.blender.org/manual/en/5.1/interface/controls/buttons/fields.html
- Date read: 2026-07-30

That passage says "Most numerical values", not "all", and never names the resolution percentage. Reading it as covering the `%` field is **inferred**: the step is that the `%` field is a numerical value field of the ordinary kind, which the manual's own picture of the Format panel supports but does not state. The passage also says nothing about what is reported — only that the value never goes wider than the hard limit.

**2. Where a setting is defined from Python with a hard minimum and maximum, an out-of-range assignment is silently replaced by the limit.** The Python API reference, describing how an integer setting is defined:

> min (int) – Hard minimum, trying to assign a value below will silently assign this minimum instead.
>
> max (int) – Hard maximum, trying to assign a value above will silently assign this maximum instead.

- Source: Blender's Python API documentation, https://docs.blender.org/api/5.1/bpy.props.html (the `bpy.props.IntProperty` entry; the same two sentences appear for `IntVectorProperty`, `FloatProperty` and `FloatVectorProperty`)
- Date read: 2026-07-30

The word "silently" is the only statement in any source I read about what is reported when a value is out of range: nothing is.

Reading this as covering the resolution percentage is **inferred**, and the step is a real one. Those sentences are on the page that describes how a *new* setting is created from Python by an add-on. The resolution percentage is not created that way — it is built into Blender. The step I take is that both are integer settings with a hard minimum and hard maximum of the same kind, so the described behaviour of a hard limit is the behaviour of this setting's hard limit. That is a reasonable step and it is still a step; a reader has to supply it, so this is not documented.

### What is not in the sources

- No source says anything about an out-of-range resolution percentage in particular. There is exactly one place in the whole of the 5.1 API documentation where `resolution_percentage` is documented — the `bpy.types.RenderSettings` page (the API index at https://docs.blender.org/api/5.1/genindex-R.html lists exactly one entry for it: "resolution_percentage (bpy.types.RenderSettings attribute)") — and it says only what question 14 quotes. The manual's Format page says only what question 14 quotes.
- No source says a render *fails*, *stops*, *warns*, or writes any message because of an out-of-range value in this or any comparable render setting.
- No source distinguishes what happens when the value is already out of range in a saved file from what happens when it is set just before rendering.

**Where I looked, to be able to say "not in the sources".** Across `manual_5.1`, `api_5.1`, `release_notes_5.1` and `cycles_5.1` I searched for: `resolution_percentage`, `resolution percentage`, `Percentage scale`, `Resolution %`, `render-percentage`, `render_percentage`, `clamp` and its forms, `clamped to`, `out of range`, `outside … range`, `out of bounds`, `invalid value`, `value is clamped`, `rejected`, `will be ignored`, `hard limit`, `soft limit`, `hard_min`, `hard_max`, `soft_min`, `soft_max`, `silently`, `ValueError`, `TypeError`, `RuntimeError`. Pages I read in full or in the relevant part and ruled out: the Format panel page (https://docs.blender.org/manual/en/5.1/render/output/properties/format.html) — describes the control, says nothing about limits; the number-fields page (https://docs.blender.org/manual/en/5.1/interface/controls/buttons/fields.html) — the general statement quoted above, nothing setting-specific; the command-line arguments page (https://docs.blender.org/manual/en/5.1/advanced/command_line/arguments.html) — no argument for this setting, and no general statement about bad argument values beyond logging levels; `bpy.types.RenderSettings` and `bpy.types.IntProperty` (https://docs.blender.org/api/5.1/bpy.types.RenderSettings.html, https://docs.blender.org/api/5.1/bpy.types.IntProperty.html) — the range and the existence of hard/soft limits, no behaviour; the API's own gotchas pages (https://docs.blender.org/api/5.1/info_gotcha.html and the `info_gotchas_*` pages it gathers) — they cover operators, file paths, meshes, threading and crashes, and none covers assigning an out-of-range number; the 5.1 release notes, whole folder — nothing on this setting. The only near-miss anywhere in the manual is a different setting entirely: "CRF values outside the allowed range are clamped to the nearest allowed value" on https://docs.blender.org/manual/en/5.1/render/output/properties/output.html, which is about video encoding quality, not the resolution percentage, and I have not carried it across.

**Not covered.**

- I cannot settle from the sources whether an out-of-range percentage is silently clamped, refused, or something else, for this setting in particular. Both general statements point at silent clamping; neither names it.
- I cannot settle what, if anything, appears in Blender's console or status output when this happens, beyond the single word "silently" in a passage about a different kind of setting.
- I cannot settle what happens when the value given is not a number at all (for instance text passed into a script). The sources show a `TypeError` for assigning the wrong kind of thing to a different setting (https://docs.blender.org/api/5.1/info_gotchas_file_paths_and_encoding.html, read 2026-07-30), but nothing about this one, and I have not carried that across either.
- Nothing here settles what a *render* does — that is, whether an out-of-range value that has been clamped changes the picture, the timing, or the exit of a render. The sources describe settings, not runs.

**Observed — hand tests, 2026-08-09.** Four tests bear on this question. All are written out in full, with their exact commands, in `Audits/Blender Research/Settings/Hand tests — later questions.md`, entries 2, 3, 4 and 6.

**One — what happens to the value, and whether anything is said.** Blender 5.1.2 was started from a terminal with no window, with factory settings and a throwaway settings folder, and a script assigned a series of values to `resolution_percentage` on the scene's render settings and read each one back. On the rented Linux machine — Ubuntu 22.04.5 LTS on kernel 6.8, one NVIDIA GeForce RTX 5060 with 8 GB, twenty-three cores of an AMD EPYC 7K62, 49 GB of memory, no display server — the whole of what it printed:

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

Nothing else was printed beside the first seven lines. The same script in Blender 5.1.2 on the Creator's Mac — a MacBook Pro with an M1 Pro chip running macOS 26.5.2 — printed the same lines, except that it wrote the limits of a machine whole number in ordinary figures, `((-2147483647 -1), 2147483647)`, instead of in hexadecimal. **The two machines agreed** on every stored value and on which values raised.

The last two lines are the control. They show this channel does carry a complaint when there is one, so "nothing was printed" for `0`, `-50`, `32768` and `40000` is a real result and not a channel that never speaks.

**Two — whether the caller is told.** In the same way, two runs per version per machine with the switch that turns a Python failure into a chosen exit status. The first raised an error on purpose and came back with that status; the second assigned 40000 and came back with 0. On the rented Linux machine in Blender 5.1.2, the control run printed:

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

The Creator's Mac gave the same pair, 42 and 0, in Blender 5.1.2. **The two machines agreed.** Because the control really did come back with 42, the 0 is a real result: a value corrected on the way in is not signalled to whatever started Blender.

**Three — what a render actually does.** Six runs per version per machine, each starting Blender with no window on the factory startup scene, setting the pixel dimensions and the percentage from a script, and rendering one frame with Cycles on the processor at one sample. The size of each written picture was read out of the file itself rather than worked out from the percentage. On the rented Linux machine in Blender 5.1.2:

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
00:00.351  render           | Rendering single frame
00:00.351  reports          | ERROR Image too small
```

and no file appeared. The run at 40000 did not refuse and did not warn: it stored 32767 and began rendering a picture 32767 per cent of 1920 by 1080, in 53,284 tiles. It was still going when a 240-second limit killed it. Its last line before that:

```
03:59.428  render           | Fra: 1 | Remaining: 08:47:08.49 | Mem: 308M | Rendered 399/53284 Tiles, Sample 1/1
```

Searching that run's whole output for `Error`, `ERROR` and `Warning` returned nothing.

The Creator's Mac gave the same six results in Blender 5.1.2 — the same stored values, the same picture sizes, the same refusal, the same unfinished render — reaching a different number of tiles in the four minutes, which is a fact about the two machines. **The two machines agreed on every stored value, every picture size and every exit status.**

Case one of the six is the control: at 200 per cent the picture came out at 3840 by 2160 on both machines in all four versions, so the way the sizes were measured works and a plain percentage does scale the picture. The odd results in the other cases therefore mean something.

*Note added by the citation check, 2026-08-09 (Opus 5).* This check repeated all six runs on both machines in all four versions and got the same stored values, the same picture sizes and the same refusal. One step is missing from the description above: it does not say how the single frame was rendered. The output quoted here is what Blender's own `-f 1` argument produces after the script; calling the render operator from inside the script gives the same picture sizes and the same refusal but different printed output — in 5.0, 5.1 and 5.2 it raises a Python error instead of printing the `reports | ERROR Image too small` line, and the very long render prints no progress at all. The missing step has gone onto the hand-test list.

*Observed — hand tests, second batch, 2026-08-09 (Opus 5).* The missing step has now been run rather than described. Each of the six runs above was made twice over, in all four versions on both machines: once with Blender's own `-f 1` argument after the script, and once by calling the render operator from inside the script, `bpy.ops.render.render(write_still=True)`. **The output quoted above is what the `-f 1` route produces, and that is the route these six runs used.** Ninety-six runs in all, forty-eight on each machine.

The two routes agree on everything about the setting: in Blender 5.1.2, case for case, on the rented Linux machine (Ubuntu 22.04.5 LTS) and on the Creator's Mac (macOS 26.5.2), each route stored the same value and wrote a picture of the same size — 3840 by 2160, 633 by 356, 19 by 10, nothing at 4 by 4 at 1 per cent, and 1310 by 1310 — measured from the written files. Nothing above changes.

They disagree on three things. **The file is named differently:** the `-f 1` route wrote `out_0001.png`, the operator route wrote `out_.png` from the same output path, in every case that wrote anything. **The operator route prints no Cycles progress at all:** twelve lines in total for a render that finishes, whichever case it is, against 57 to 64 under the `-f 1` route — the same counts on both machines. Blender's own `Saved:` line is still there among those twelve, which is the control: the output was being captured, so the missing progress lines are an absence. **The refusal is reported differently:** neither of the two lines quoted above, `render | Rendering single frame` and `reports | ERROR Image too small`, appears anywhere under the operator route. What appears instead is `RuntimeError: Error: Image too small` inside a Python traceback, followed by a bare `Error: Image too small`. Both machines, same shape.

The long run at 40000 per cent shows the gap at its widest. Under `-f 1` it printed 197 lines on the rented Linux machine and 355 on the Creator's Mac in ninety seconds, all of them progress. Under the operator route it printed nothing whatever — a repeat with the script's own output flushed, cut off at sixty seconds, printed exactly seven lines and not one line of Blender's own. From the rented Linux machine:

```
CYCLES samples 1 device CPU
VERSION 5.1.2
ASKED 1920 1080 40000
STORED 1920 1080 32767
ROUTE B -- the frame is rendered by calling the render operator from inside this script
FILEPATH /root/b2-settings/run/B2/d/5.1/out_
EXIT 124
```

The Creator's Mac printed the same seven lines in Blender 5.1.2, including the same `STORED 1920 1080 32767`, differing only in the output path and in the way its runner reports a killed run.

The stored value is 32767 either way, so the answer above stands; only the printing differs. This agrees with the citation check's account. The control held — the 200 per cent case came out at 3840 by 2160 under both routes on both machines. Neither standing line below is affected: both routes run outside the interface, and the two operating systems gave the same result run for run. The whole record, with the raw output, is entry 7 of `Audits/Blender Research/Settings/Hand tests — later questions.md`.

**Four — whether a saved file can hold a value outside the range: not run.** Because the value is corrected at the moment it is assigned, a file saved afterwards already holds a value inside the range, so that test could only ever report a value inside the range whatever a file can hold. It cannot come out the other way, and a test that cannot fail was not run.

**What still has no answer, and why.** What the `%` field does when an out-of-range number is typed into it, where the slider stops when dragged, and whether the Info editor or the status bar shows anything, were **not run this round**: they need eyes on a window, the rented Linux machine has no display server at all, and the Creator ran no tests. **Windows was not tested at all** — no Windows machine has run it. **Untried rather than untestable: a Windows machine can be rented, and every one this project has read reported a Server edition — see the equipment section of `Preparation/Blender Research/Method.md`.**

**Outside the interface.** Partly. The Python statement about hard limits ("silently assign this minimum instead") is about assigning a value, which is the same act whether or not a window is open, so if it applies at all it applies outside the interface too — that is **inferred**, on the step that the sources never make this behaviour conditional on a window. The manual's statement is explicitly about the interface — dragging with the mouse, input via keyboard — and by its own words has nothing to say about a render driven from outside. So the one source that speaks about reporting at all ("silently") is the one that survives outside the interface, and the one that describes limits in detail does not. This is the weak point of the answer, and it is why the honest verdict for the percentage itself is "not in the sources".

**Observed — hand tests, 2026-08-09.** This line is now settled by observation rather than by a step. Every test above was run from a terminal with no window, on both machines: the value is corrected without a word, the exit status stays 0 against a control that proves the exit-code switch fires, and the render then proceeds at the corrected value. What has no counterpart outside the window — typing into the field, dragging the slider, the Info editor — was not run, for want of a machine with a screen.

**Operating system.** The sources say nothing that ties any of this to an operating system, and draw no such distinction on any of the pages named above.

**Observed — hand tests, 2026-08-09.** The tests above were run in Blender 5.1.2 on two operating systems: Ubuntu 22.04.5 LTS on an x86 machine (the rented Linux machine) and macOS 26.5.2 on Apple silicon (the Creator's Mac). Every stored value, every picture size, every exit status and the refusal message all matched. The only difference was the wording of one message, quoted above, about the limits of a machine whole number. **Windows was not tested** — no Windows machine has run it — so the Windows part of this line stays untested rather than answered.

---

## Things a person with Blender open could settle in a few minutes

Pruned on 2026-08-09, after the hand tests were run. What they settled is written into the answers above, and the whole record — every command, every line the machines printed — is in `Audits/Blender Research/Settings/Hand tests — later questions.md`. Only genuinely open items are left here.

1. **What the `%` field does when a number outside the range is typed into it.** Not run this round: it needs eyes on a window. The rented Linux machine has no display server at all, and the Creator ran no tests. Note that the numbers behind the field are no longer open — the outright pair and the dragging pair were read off the property from a terminal, and are recorded under question 14.

2. **Where the slider stops when it is dragged, and what a person sees while dragging.** Not run this round, same reason. Again, only what the mouse can reach is open; the dragging pair itself is recorded under question 14 as 1 and 100.

3. **Whether the Info editor or the status bar shows anything when a value is corrected.** Not run this round, same reason. The other half of that question is settled above: on the channel a caller outside Blender can see, nothing at all was printed, and a control in the same run showed that channel does carry complaints.

4. **Whether a driver or an animated value can carry the setting outside its range.** Blender reports the property as not animatable — `is_animatable = False`, quoted under question 14 — but no driver was attached and no key was set, so this is not settled.

5. **Whether a file written by something other than Blender can hold a value outside the range.** Untouched. What was tested is a value assigned inside Blender, which is corrected at once.

**Not run, and why.** Whether a saved file can carry an out-of-range value across a save and a reopen. The value is corrected at the moment it is assigned, so a file saved afterwards already holds a value inside the range, and the test could only ever report a value inside the range. It cannot come out the other way, so it was not run.

**Not run.** Anything on Windows. No Windows machine has run it, so the Windows part of the operating-system line on both answers stays untested rather than answered.

---

2026-08-09 — Opus 5 (Claude Code) — Settings, Blender 5.1

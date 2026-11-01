# Settings — later questions (Blender 5.0)

Questions answered: 14 and 15 only.

**Sources consulted, and when.** Blender 5.0 manual, Blender 5.0 Python API reference, and Blender 5.0 release notes, all as the plain-text copies in `Blender Sources/`, taken on 2026-07-30 — that is the date read for every quotation below. Also one picture from the manual's own image folder, and Cycles' own files for 5.0. *(Audit note: no finding below draws on Cycles' own files, and nothing here records what was looked for in them or what was ruled out.)*

I read only the question sheet and files inside Blender Sources/. I did not open any other part of this project.

---

## 14. What values can a render's resolution percentage be set to?

**Label: documented.**

The setting is the render resolution percentage, `resolution_percentage`. The Python API reference states its kind, its accepted span and its default:

> resolution_percentage
>
> Percentage scale for render resolution
>
> Type:
>
> int in [1, 32767], default 100

Source: Python API reference, https://docs.blender.org/api/5.0/bpy.types.RenderSettings.html — read 2026-07-30.

So: whole numbers only, from 1 to 32767 inclusive, and 100 unless changed. Zero is not in the span, and neither is any negative number.

The default of 100 appears a second time, in a worked console example in the API reference's quick-start page:

> \>>> bpy.data.scenes[0].render.resolution_percentage
>
> 100

Source: Python API reference, https://docs.blender.org/api/5.0/info_quickstart.html — read 2026-07-30.

The manual describes the same setting but gives no numbers at all:

> Percentage
>
> Slider to reduce or increase the size of the rendered image relative to the Resolution values.
> This is useful for small test renders that have the same proportions as the final image.

Source: manual, https://docs.blender.org/manual/en/5.0/render/output/properties/format.html — read 2026-07-30.

Note that the manual's wording allows for increase as well as reduction, which matches an accepted span running well above 100.

### What the picture shows

The manual's Format page carries a picture of the panel: `[image: _images/render_output_properties_format_panel.png]`, captioned "Format panel." I opened it. Visible in it: a field labelled `%` reading `100%`, drawn as a slider whose coloured bar fills the field's whole width; above it `Resolution X` reading `1920 px` and `Y` reading `1080 px`. Label for that: **documented** (the picture is part of the manual, same page and date as above).

**One step beyond the picture — label: inferred.** The manual says of sliders that they "have a colored bar in the background to display values over a range, e.g. percentage values" (manual, https://docs.blender.org/manual/en/5.0/interface/controls/buttons/fields.html, read 2026-07-30). The bar is completely full at 100. Putting those two together, the range the slider draws appears to top out at 100 — that is, dragging is bounded at 100 even though the accepted span runs to 32767. The step a reader has to supply is that a full bar means the value is at the top of the drawn range. The sources nowhere state this setting's drag bounds outright.

That distinction matters because the manual states the two bounds are different things:

> Most numerical values are restricted by "soft limit" and "hard limit" value ranges.
> Changing values by dragging with the mouse is restricted to the "soft limit" value range.
> Input via keyboard will allow the use of wider value ranges, but never wider than the "hard limit".

Source: manual, https://docs.blender.org/manual/en/5.0/interface/controls/buttons/fields.html — read 2026-07-30.

**Which of the two the API reference prints — label: inferred.** This answer treats the span `[1, 32767]` as the outer, hard one, and reports it as such. Nothing quoted says which of the two it is. The step a reader has to supply is that the span printed beside a setting in the API reference is its hard span and not its drag span. If it turned out to be the drag span, this answer and the answer to question 15 both change.

### Not covered

- This settles what the setting will hold, not what a render makes of it. Nothing here says a value of 32767 produces a picture rather than failing.
- The soft bounds — the span the slider itself will drag through — are not stated in words anywhere I searched; the reading above comes from a picture. *(Audit note: no search terms were recorded for this, so there is no record of how hard it was looked for. Sent back for research.)*
- Nothing here says how the pixel size is worked out from the percentage, or how a fractional result is rounded.
- Whether an animated value or a driver can push the stored value outside `[1, 32767]` is not addressed by anything I found.
- The scene's own pixel dimensions have their own separate spans, quoted here only because they sit beside the percentage on the same page: `resolution_x` and `resolution_y` are each "int in [4, 65536]" (same API page and date). What the percentage and those two do together is not stated.

### What a hand test settled

**Observed — hand test, 2026-08-09.** This closes the gap named just above. The tests are written out in full, with their exact commands, in `Audits/Blender Research/Settings/Hand tests — later questions.md`, entries 1, 2 and 5.

*What was done.* Blender 5.0.1 was started from a terminal with no window, with factory settings and a throwaway settings folder, and a short script printed what Blender itself holds for the property `resolution_percentage` on the scene's render settings in the factory startup scene.

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

*On what kind of computer.* The rented Linux machine — Ubuntu 22.04.5 LTS on kernel 6.8, one NVIDIA GeForce RTX 5060 with 8 GB, twenty-three cores of an AMD EPYC 7K62, 49 GB of memory, no display server. The same script in Blender 5.0.1 on the Creator's Mac — a MacBook Pro with an M1 Pro chip running macOS 26.5.2 — printed the identical ten lines. The two machines agreed.

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

The Creator's Mac printed the same lines in Blender 5.0.1, with one difference: it wrote the limits of a machine whole number in ordinary figures, `((-2147483647 -1), 2147483647)`, where the Linux machine wrote them in hexadecimal. So a fractional value is refused rather than cut down, which this answer left open.

**Observed — hand test, 2026-08-09, where the setting is read inside Blender.** Every Python file inside the installed Blender 5.0.1 was searched for `resolution_percentage`. Nineteen lines came back on the rented Linux machine and the same nineteen on the Creator's Mac: fourteen render preset files each setting it to 100, a manual-reference entry, the preview renderer, the preset list, the line that draws it in the panel, and one line in Cycles' own files —

```
/opt/blender/5.0/5.0/scripts/startup/bl_ui/properties_output.py:119:        col.prop(rd, "resolution_percentage", text="%")
/opt/blender/5.0/5.0/scripts/addons_core/cycles/ui.py:399:            render_scale = scene.render.resolution_percentage / 100.0
```

### Outside the interface

**Label: inferred.** The span itself is not an interface fact. It is stated in the Python API reference, which is the face Blender presents to a script — the same face something outside Blender drives it through. So `[1, 32767]` holds when the value is set by a script rather than by a person. The step a reader has to supply is that a span printed in the API reference governs a value set from a script; nothing quoted says that in so many words.

What does **not** carry over is the slider. The manual's distinction between dragging and typing is about a person at a field; there is no dragging outside the interface, so only the hard span is in play there.

**On setting it at the moment a render is started: not in the sources.** I searched the manual's list of command line arguments and its command line rendering page for `percent`, `resolution` and `scale`; the only hit in either was a display option about "native pixel size, for high resolution displays (MacBook Retina)", which is not this setting. Pages searched: https://docs.blender.org/manual/en/5.0/advanced/command_line/arguments.html and https://docs.blender.org/manual/en/5.0/advanced/command_line/render.html, read 2026-07-30. So the sources record no command line argument that sets the resolution percentage. *(Audit note: the question sheet says questions 14 and 15 do not repeat question 1, which asked what can be settled at the moment a render is started. This paragraph answers a slice of question 1, and question 1 owns that fact. It is kept here as the record of a search, not as part of the answer to question 14.)*

**Observed — hand test, 2026-08-09.** Every test recorded above was run from a terminal with no window at all, on both machines, so this line is settled by observation rather than by a step: the limits and the values taken are the same when Blender is driven from outside its own window.

### Operating system

The sources do not say the span differs by operating system, and the API reference states one span with no qualification of any kind attached to it. I searched the release notes for 5.0 for "resolution" and found nothing about this setting. Whether it in fact differs anywhere is not something the sources address.

**Observed — hand test, 2026-08-09.** The tests above were run in Blender 5.0.1 on two operating systems: Ubuntu 22.04.5 LTS on an x86 machine (the rented Linux machine) and macOS 26.5.2 on Apple silicon (the Creator's Mac). Every number was the same on both. The only difference was the wording of one message, quoted above, about the limits of a machine whole number. **Windows was not tested** — no Windows machine has run it — so the Windows part of this line stays untested rather than answered.

---

## 15. What does a render do when it is asked for a value outside what that setting accepts, and what does it report?

Read as asked about the resolution percentage, following the sheet's own note that questions 14 and 15 are about that one setting.

**Label: inferred**, with the step written out below. The nearest documented statement is about integer settings in general, not about this one by name.

### What the source says

The API reference, describing what the hard bounds of an integer setting mean, states:

> min (int) – Hard minimum, trying to assign a value below will silently assign this minimum instead.
>
> max (int) – Hard maximum, trying to assign a value above will silently assign this maximum instead.

Source: Python API reference, https://docs.blender.org/api/5.0/bpy.props.html — read 2026-07-30. (The same pair of lines appears against each of the numeric kinds on that page; the ones quoted are the integer pair.)

### The step

That page is describing the bounds given when a new integer setting is defined. The resolution percentage is an integer setting whose stated bounds are `[1, 32767]` (question 14). To get from the quoted words to an answer about this setting, a reader must accept that the bounds printed for `resolution_percentage` mean the same thing as the bounds described on that page. Nothing I read says so outright. Hence **inferred**, not documented.

On that reading: assigning 0, or any negative number, silently stores 1. Assigning 40000 silently stores 32767. Nothing is refused and nothing fails.

### What it reports

The source's own word is "silently", twice, so nothing is reported **in what that page describes** — no error, no warning, no message there. Label: **inferred**, by the same step as above. It carries no further than that page: whether anything reaches the console is not addressed by it, and is recorded as not in the sources below.

I looked for any statement to the contrary and found none. Searches run across the whole 5.0 manual, the whole 5.0 API reference and the whole 5.0 release notes: "invalid value" (no hits anywhere), "out of range" (two hits — an API page about invalid mesh indices, and a release-notes line about a glTF exporter fault ending "IndexError: list index out of range" — neither about a numeric bound; *corrected by the citation check, 2026-08-09 (Opus 5), which found these two hits and not the "text encoding" originally written here; the count and the conclusion are unchanged*), "too large" (ten hits, all about unrelated things — menus that do not fit a screen, cloth collision distances, a defocus blur), "clamp" (many hits, all about node and modifier features that clamp their own output, none about a value being assigned to a setting). I also read the manual's Format page and its number fields page in full. None of them says anything is reported.

### The other way in — typing into the field

If a person types a number into the field rather than a script assigning one, the manual says plainly where it stops:

> Input via keyboard will allow the use of wider value ranges, but never wider than the "hard limit".

Source: manual, https://docs.blender.org/manual/en/5.0/interface/controls/buttons/fields.html — read 2026-07-30. Label for that sentence alone: **documented**. It says the typed value will not go past the hard bound. It does not say what, if anything, appears when you try.

### A different kind of wrong value

If the value handed over is not a number at all, that is a different failure and it is not silent. The API reference shows the shape of the complaint, for a text setting:

> TypeError: bpy_struct: item.attr= val: Object.name expected a string type, not str

Source: Python API reference, https://docs.blender.org/api/5.0/info_gotchas_file_paths_and_encoding.html — read 2026-07-30. Label: **documented** for that message existing in that form for that setting; **inferred**, and weakly, that an equivalent complaint names the resolution percentage when something unusable is assigned to it. The example is about a different setting entirely and I found no worked example for this one.

### Not covered

- This is about the value being stored, not about the render. What a render does when it starts with the percentage already sitting at 32767 — whether it produces the picture, runs out of memory, or stops — is **not in the sources**. Nothing I searched describes it. *(Audit note: this is the question's other reading — what a **render** does, as against what the setting stores — and it is left open. No search terms were recorded for it, so this "not in the sources" has no record of looking behind it. Hand test 6 below is aimed at it.)*
- Whether anything is printed to the console when a render runs with the value at either bound is not stated.
- Whether the clamping happens at the moment of assignment or at the moment of rendering is not stated.
- The whole answer rests on one general statement about how integer bounds behave. If that statement does not govern this setting, the answer does not hold.

### What a hand test settled

**Observed — hand tests, 2026-08-09.** Four tests bear on this question. All are written out in full, with their exact commands, in `Audits/Blender Research/Settings/Hand tests — later questions.md`, entries 2, 3, 4 and 6.

**One — what happens to the value, and whether anything is said.** Blender 5.0.1 was started from a terminal with no window, with factory settings and a throwaway settings folder, and a script assigned a series of values to `resolution_percentage` on the scene's render settings and read each one back. On the rented Linux machine — Ubuntu 22.04.5 LTS on kernel 6.8, one NVIDIA GeForce RTX 5060 with 8 GB, twenty-three cores of an AMD EPYC 7K62, 49 GB of memory, no display server — the whole of what it printed:

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

Nothing else was printed beside the first seven lines. The same script in Blender 5.0.1 on the Creator's Mac — a MacBook Pro with an M1 Pro chip running macOS 26.5.2 — printed the same lines, except that it wrote the limits of a machine whole number in ordinary figures, `((-2147483647 -1), 2147483647)`, instead of in hexadecimal. **The two machines agreed** on every stored value and on which values raised.

The last two lines are the control. They show this channel does carry a complaint when there is one, so "nothing was printed" for `0`, `-50`, `32768` and `40000` is a real result and not a channel that never speaks.

**Two — whether the caller is told.** In the same way, two runs per version per machine with the switch that turns a Python failure into a chosen exit status. The first raised an error on purpose and came back with that status; the second assigned 40000 and came back with 0. On the rented Linux machine in Blender 5.0.1, the control run printed:

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

The Creator's Mac gave the same pair, 42 and 0, in Blender 5.0.1. **The two machines agreed.** Because the control really did come back with 42, the 0 is a real result: a value corrected on the way in is not signalled to whatever started Blender.

**Three — what a render actually does.** Six runs per version per machine, each starting Blender with no window on the factory startup scene, setting the pixel dimensions and the percentage from a script, and rendering one frame with Cycles on the processor at one sample. The size of each written picture was read out of the file itself rather than worked out from the percentage. On the rented Linux machine in Blender 5.0.1:

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
00:00.276  render           | Rendering single frame
00:00.276  reports          | ERROR Image too small
```

and no file appeared. The run at 40000 did not refuse and did not warn: it stored 32767 and began rendering a picture 32767 per cent of 1920 by 1080, in 53,284 tiles. It was still going when a 240-second limit killed it. Its last line before that:

```
03:59.650  render           | Fra: 1 | Remaining: 04:04:43.90 | Mem: 308M | Rendered 853/53284 Tiles, Sample 1/1
```

Searching that run's whole output for `Error`, `ERROR` and `Warning` returned nothing.

The Creator's Mac gave the same six results in Blender 5.0.1 — the same stored values, the same picture sizes, the same refusal, the same unfinished render — reaching a different number of tiles in the four minutes, which is a fact about the two machines. **The two machines agreed on every stored value, every picture size and every exit status.**

Case one of the six is the control: at 200 per cent the picture came out at 3840 by 2160 on both machines in all four versions, so the way the sizes were measured works and a plain percentage does scale the picture. The odd results in the other cases therefore mean something.

*Note added by the citation check, 2026-08-09 (Opus 5).* This check repeated all six runs on both machines in all four versions and got the same stored values, the same picture sizes and the same refusal. One step is missing from the description above: it does not say how the single frame was rendered. The output quoted here is what Blender's own `-f 1` argument produces after the script; calling the render operator from inside the script gives the same picture sizes and the same refusal but different printed output — in 5.0, 5.1 and 5.2 it raises a Python error instead of printing the `reports | ERROR Image too small` line, and the very long render prints no progress at all. The missing step has gone onto the hand-test list.

*Observed — hand tests, second batch, 2026-08-09 (Opus 5).* The missing step has now been run rather than described. Each of the six runs above was made twice over, in all four versions on both machines: once with Blender's own `-f 1` argument after the script, and once by calling the render operator from inside the script, `bpy.ops.render.render(write_still=True)`. **The output quoted above is what the `-f 1` route produces, and that is the route these six runs used.** Ninety-six runs in all, forty-eight on each machine.

The two routes agree on everything about the setting: in Blender 5.0.1, case for case, on the rented Linux machine (Ubuntu 22.04.5 LTS) and on the Creator's Mac (macOS 26.5.2), each route stored the same value and wrote a picture of the same size — 3840 by 2160, 633 by 356, 19 by 10, nothing at 4 by 4 at 1 per cent, and 1310 by 1310 — measured from the written files. Nothing above changes.

They disagree on three things. **The file is named differently:** the `-f 1` route wrote `out_0001.png`, the operator route wrote `out_.png` from the same output path, in every case that wrote anything. **The operator route prints no Cycles progress at all:** twelve lines in total for a render that finishes, whichever case it is, against 57 to 64 under the `-f 1` route — the same counts on both machines. Blender's own `Saved:` line is still there among those twelve, which is the control: the output was being captured, so the missing progress lines are an absence. **The refusal is reported differently:** neither of the two lines quoted above, `00:00.276  render           | Rendering single frame` and `00:00.276  reports          | ERROR Image too small`, appears anywhere under the operator route. What appears instead is `RuntimeError: Error: Image too small` inside a Python traceback, followed by a bare `Error: Image too small`. Both machines, same shape.

The long run at 40000 per cent shows the gap at its widest. Under `-f 1` it printed 278 lines on the rented Linux machine and 304 on the Creator's Mac in ninety seconds, all of them progress. Under the operator route it printed nothing whatever — a repeat with the script's own output flushed, cut off at sixty seconds, printed exactly seven lines and not one line of Blender's own. From the rented Linux machine:

```
CYCLES samples 1 device CPU
VERSION 5.0.1
ASKED 1920 1080 40000
STORED 1920 1080 32767
ROUTE B -- the frame is rendered by calling the render operator from inside this script
FILEPATH /root/b2-settings/run/B2/d/5.0/out_
EXIT 124
```

The Creator's Mac printed the same seven lines in Blender 5.0.1, including the same `STORED 1920 1080 32767`, differing only in the output path and in the way its runner reports a killed run.

The stored value is 32767 either way, so the answer above stands; only the printing differs. This agrees with the citation check's account. The control held — the 200 per cent case came out at 3840 by 2160 under both routes on both machines. Neither standing line below is affected: both routes run outside the interface, and the two operating systems gave the same result run for run. The whole record, with the raw output, is entry 7 of `Audits/Blender Research/Settings/Hand tests — later questions.md`.

**Four — whether a saved file can hold a value outside the range: not run.** Because the value is corrected at the moment it is assigned, a file saved afterwards already holds a value inside the range, so that test could only ever report a value inside the range whatever a file can hold. It cannot come out the other way, and a test that cannot fail was not run.

**What still has no answer, and why.** What the `%` field does when an out-of-range number is typed into it, where the slider stops when dragged, and whether the Info editor or the status bar shows anything, were **not run this round**: they need eyes on a window, the rented Linux machine has no display server at all, and the Creator ran no tests. **Windows was not tested at all** — no Windows machine has run it. **Untried rather than untestable: a Windows machine can be rented, and every one this project has read reported a Server edition — see the equipment section of `Preparation/Blender Research/Method.md`.**

### Outside the interface

**Label: inferred**, on the same step as the main finding. The main finding is about assignment from a script, which is exactly the route something outside Blender uses, so it holds there. The manual's sentence about typing into a field does not — there is no field to type into when Blender is driven from outside, so only the silent clamping applies.

One difference worth naming: inside the interface a person can see the number snap back and notice. Outside it, on the strength of the word "silently", nothing marks that the value was changed. Whether anything at all is written to the console in that case is **not in the sources**.

**Observed — hand tests, 2026-08-09.** This line is now settled by observation rather than by a step. Every test above was run from a terminal with no window, on both machines: the value is corrected without a word, the exit status stays 0 against a control that proves the exit-code switch fires, and the render then proceeds at the corrected value. What has no counterpart outside the window — typing into the field, dragging the slider, the Info editor — was not run, for want of a machine with a screen.

### Operating system

The sources do not say. The statement about hard bounds carries no mention of any operating system.

**Observed — hand tests, 2026-08-09.** The tests above were run in Blender 5.0.1 on two operating systems: Ubuntu 22.04.5 LTS on an x86 machine (the rented Linux machine) and macOS 26.5.2 on Apple silicon (the Creator's Mac). Every stored value, every picture size, every exit status and the refusal message all matched. The only difference was the wording of one message, quoted above, about the limits of a machine whole number. **Windows was not tested** — no Windows machine has run it — so the Windows part of this line stays untested rather than answered. The searches recorded above — "invalid value", "out of range", "too large" and "clamp", run across the whole 5.0 manual, the whole 5.0 API reference and the whole 5.0 release notes — turned up nothing tying this setting to an operating system, and neither did the search of the 5.0 release notes for "resolution" recorded under question 14.

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

2026-08-09 — Opus 5 (Claude Code) — Settings, Blender 5.0

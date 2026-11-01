# Settings — later questions (second pass), Blender 5.0

**Sources consulted, and when.** Blender's 5.0 manual, Blender's 5.0 Python API documentation, the Blender 5.0 release notes, and the Cycles 5.0 files, all as supplied inside `Blender Sources/`. All four were taken on 2026-07-30, and that is my date read for everything quoted below. I also opened one picture from the manual (named in answer 14).

I read only the question sheet and files inside Blender Sources/. I did not open any other part of this project.

I answered questions 14 and 15 only. I used no helpers, so there is no helper work to vouch for.

---

## 14. What values can a render's resolution percentage be set to?

**Label: documented.**

**The short answer.** Whole numbers from 1 to 32767. The starting value is 100. There is no zero, no negative value, and the setting is a whole number, not a fraction. Values above 100 are allowed and make the picture bigger than the resolution figures say.

**The exact words that carry it.**

From the Python API documentation, on the page for a scene's render settings:

> resolution_percentage
>
> Percentage scale for render resolution
>
> Type:
>
> int in [1, 32767], default 100

Source: Python API documentation, https://docs.blender.org/api/5.0/bpy.types.RenderSettings.html — read 2026-07-30.

("int" is that documentation's word for a whole number. "[1, 32767]" is how it writes the range it accepts, ends included. "default 100" is the value the setting starts at.)

From the manual, on the Format panel of the Output properties:

> Percentage
>
> Slider to reduce or increase the size of the rendered image relative to the Resolution values.
> This is useful for small test renders that have the same proportions as the final image.

Source: manual, https://docs.blender.org/manual/en/5.0/render/output/properties/format.html — read 2026-07-30.

That is the word "increase", so going above 100 is described by the manual and not only implied by the numeric range.

The same page names the two resolution figures the percentage is applied to:

> Resolution X, Y
>
> The number of pixels horizontally and vertically in the image.

and the API documentation gives their own ranges on the same page as the percentage:

> resolution_x … Type: int in [4, 65536], default 1920
>
> resolution_y … Type: int in [4, 65536], default 1080

Source: as above, both pages — read 2026-07-30.

**The picture.** The manual's Format panel screenshot (`_images/render_output_properties_format_panel.png`, shown on https://docs.blender.org/manual/en/5.0/render/output/properties/format.html — read 2026-07-30) shows the setting sitting under "Resolution X" and "Y" as a row labelled only "%", reading "100%", drawn as a bar that is filled all the way across. What is visible is the label, the value, and that the bar is full at 100. **Documented**, from the picture.

**What the setting does with the number.** The API documentation's own worked example for a render engine shows the arithmetic:

> scale = scene.render.resolution_percentage / 100.0
> self.size_x = int(scene.render.resolution_x * scale)
> self.size_y = int(scene.render.resolution_y * scale)

Source: Python API documentation, https://docs.blender.org/api/5.0/bpy.types.RenderEngine.html — read 2026-07-30.

The same three lines appear in Cycles' own files. **Observed** — Cycles 5.0, file `ui.py`, lines 399 to 401, inside the drawing code for the path guiding panel, read 2026-07-30 *(date read added by the citation check, 2026-08-09 (Opus 5), to match the snapshot date carried by every other quotation here)*:

> render_scale = scene.render.resolution_percentage / 100.0
> render_size_x = int(scene.render.resolution_x * render_scale)
> render_size_y = int(scene.render.resolution_y * render_scale)

Both are read as: divide the percentage by 100, multiply each resolution figure by it, and cut off the fraction. In Cycles' file these lines only decide whether to show an advisory label, not what gets rendered.

**One thing I could not settle: the softer limit for dragging.** The manual says number settings have two ranges, a loose one and a hard one:

> Most numerical values are restricted by "soft limit" and "hard limit" value ranges.
> Changing values by dragging with the mouse is restricted to the "soft limit" value range.
> Input via keyboard will allow the use of wider value ranges, but never wider than the "hard limit".

Source: manual, https://docs.blender.org/manual/en/5.0/interface/controls/buttons/fields.html — read 2026-07-30.

None of my sources states the softer range for this particular setting. The screenshot's bar being completely full at 100 would fit a softer range that stops at 100, but that is me reading a conclusion into a picture, so: **inferred**, weakly, and the step is "the bar is full at 100, therefore 100 is the top of the drag range" — a reader who does not accept that step has no answer here. The published range of 1 to 32767 is the hard one, and it is the one that matters for setting the value from outside.

**Not covered.**
- The softer drag range, as just said.
- What happens if a fraction such as 50.5 is offered. The setting is a whole number, but nothing I found says whether such an input is rounded, cut off, or refused.
- Whether every point in the range is usable in practice. A percentage of 1 with the smallest allowed resolution of 4 pixels works out to 0.04 pixels, and nothing I found says what a render does with that. I did not test it.
- Whether a file saved by other software, or by an older Blender, can hold a value outside 1 to 32767, and what happens when such a file is opened.
- Whether this range has ever been different. The Python API change log for 4.5 to 5.0 lists changes to the scene render settings, and this setting is not among them, so nothing changed for it in 5.0 — but that says nothing about earlier versions.

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

**Outside the interface.** It holds. The range quoted above is from the Python documentation itself, which is the way the setting is reached when Blender is driven from outside its own window, so it is a statement about that route rather than about the panel. The manual's Quick Start page shows the setting being read that way and giving back its starting value:

> \>\>\> bpy.data.scenes[0].render.resolution_percentage
> 100

Source: Python API documentation, https://docs.blender.org/api/5.0/info_quickstart.html — read 2026-07-30.

Where it differs from inside: there is no command-line switch that sets this percentage when Blender is started. I read the whole command-line arguments page and searched it for "percent", "%", "resolution" and "scale"; the only hit for "resolution" is an unrelated switch about high-resolution displays. So from outside, the value comes from the file or from a script, not from a start-up switch. **Documented by absence**, with the search recorded below.

**Observed — hand test, 2026-08-09.** Every test recorded above was run from a terminal with no window at all, on both machines, so this line is settled by observation rather than by a step: the limits and the values taken are the same when Blender is driven from outside its own window.

**Operating system.** The sources do not say. Nothing on the pages above ties this setting or its range to any operating system, and the release notes say nothing about it. I did not find a statement either way, so I am not claiming one.

**Observed — hand test, 2026-08-09.** The tests above were run in Blender 5.0.1 on two operating systems: Ubuntu 22.04.5 LTS on an x86 machine (the rented Linux machine) and macOS 26.5.2 on Apple silicon (the Creator's Mac). Every number was the same on both. The only difference was the wording of one message, quoted above, about the limits of a machine whole number. **Windows was not tested** — no Windows machine has run it — so the Windows part of this line stays untested rather than answered.

**Where I looked, and what I ruled out.**
- Searched every file of the API documentation for `resolution_percentage`: five hits, all pointing at the same one setting on the scene's render settings (the render settings page itself, the index, the Quick Start page, the render engine example page, and the alphabetical index). Searched the API's full index for "percentage" and "percent": the only render-related hit in the whole index is this one setting. So there is exactly one resolution percentage in the API, and I have it.
- Searched the whole manual for "Percentage": thirteen hits. Only the Output Format page is about this setting. Ruled out the others by reading their lines: node groups, dynamic topology detail, a Grease Pencil build modifier, the interface preferences, an EEVEE film overscan figure, an EEVEE light probe falloff, a compositor scale node, a particle boids speed, custom properties, and a physics view percentage.
- Searched the manual's render and scene-layout sections for "100%", "200%", "50%" and "percentage": nothing giving a range for this setting.
- Read the manual's Working Limits page (https://docs.blender.org/manual/en/5.0/advanced/limits.html) end to end, expecting a stated ceiling. It covers distances, the frame count, and text field lengths only. No resolution limits. Ruled out.
- Read the manual's Format panel page and its screenshot.
- Read the manual's Input Fields page for how number settings behave.
- Searched the release notes for 5.0 for "resolution": hits are about EEVEE curve resolution, subdivision, Grease Pencil resolution, a Mac crash at high resolution, a sequencer preview crash, and sequencer output resolution. None about the percentage or its range. Ruled out.
- Searched the Python API change log for `resolution_percentage`: no hits. Read the render settings entry in it: only bake settings were removed.
- Searched all Cycles 5.0 files for "resolution_percentage" and "percentage": one hit, quoted above. Cycles does not define this setting or restate its range; the Cycles source folder supplied here holds only kernel and utility files.

---

## 15. What does a render do when it is asked for a value outside what that setting accepts, and what does it report?

I read this as being about the resolution percentage, following the sheet's own note that questions 14 and 15 are about that one setting. The other reading would be "any render setting", which my sources would not let me answer in general.

The answer splits by how the value is offered, and my sources are much stronger on one route than the other.

### Typed or dragged into the panel — **documented**

> Most numerical values are restricted by "soft limit" and "hard limit" value ranges.
> Changing values by dragging with the mouse is restricted to the "soft limit" value range.
> Input via keyboard will allow the use of wider value ranges, but never wider than the "hard limit".

Source: manual, https://docs.blender.org/manual/en/5.0/interface/controls/buttons/fields.html — read 2026-07-30.

So dragging cannot leave the loose range, and typing cannot leave the hard range, which for this setting is 1 to 32767. The manual does not say what is shown when you type something beyond it — no message, no colour, nothing is described. It only says the wider value will not be used.

The same page notes that these fields accept arithmetic, which is another way a number outside the range could arrive:

> You can enter mathematical expressions into any number field.
> For example, enter 3*2 or 10/5+4 instead of 6.

Source: as above — read 2026-07-30.

### Set from a script or from outside Blender — **inferred**

The strongest statement I found is in the documentation for how integer settings are defined:

> min (int) – Hard minimum, trying to assign a value below will silently assign this minimum instead.
>
> max (int) – Hard maximum, trying to assign a value above will silently assign this maximum instead.

Source: Python API documentation, https://docs.blender.org/api/5.0/bpy.props.html — read 2026-07-30, in the entry for `bpy.props.IntProperty`. The same pair of sentences appears four times on that page — once each for `FloatProperty`, `FloatVectorProperty`, `IntProperty` and `IntVectorProperty` — which is eight sentences in all. *(Corrected from "eight times" by the citation check, 2026-08-09 (Opus 5); nothing else here rested on the number.)*

**The step I am taking, written out.** Those sentences are on the page about settings that add-ons create for themselves, not about Blender's own built-in settings. To reach the resolution percentage I have to join two further facts. First, Blender's own integer settings carry the same pair of limits: the API page for the integer setting type lists `hard_min` and `hard_max` alongside `soft_min` and `soft_max`, described as "Minimum value used by buttons" and "Maximum value used by buttons" (https://docs.blender.org/api/5.0/bpy.types.IntProperty.html — read 2026-07-30). Second, the render settings page prints this setting as "int in [1, 32767]", which is that same pair of hard limits. If those two joins hold, then offering 0 stores 1, offering 40000 stores 32767, offering -5 stores 1, and nothing is raised, printed, or logged. A reader who does not accept the joins is left with no answer for this route, and I want that visible: this is **inferred**, not documented.

**What it reports on this route: nothing, by the same inference.** The word in the quoted sentence is "silently". Blender's Info editor is the place messages would appear — "The Info editor logs the executed operators as well as errors, warnings," (manual, https://docs.blender.org/manual/en/5.0/editors/info_editor.html — read 2026-07-30) — but nothing in my sources connects an out-of-range value to a message there or anywhere else.

### What is genuinely **not in the sources**

- What a render does, as opposed to what the setting stores. Whether a render started with an out-of-range request differs in any way from one started at the clamped value is nowhere stated.
- What is reported when a value of the wrong kind entirely is offered — a word, a fraction, an empty value. The only worked example of such a complaint in my sources is for a name, not a number: "TypeError: bpy_struct: item.attr= val: Object.name expected a string type, not str" (Python API documentation, https://docs.blender.org/api/5.0/info_gotchas_file_paths_and_encoding.html — read 2026-07-30). That is a different setting of a different kind, and I am not carrying it across; I record it only because it is the nearest thing to a reported error message that I found.
- Whether a file that already holds an out-of-range value — written by other software, or carried from an older version — is corrected on opening, and whether that is announced.

### Not covered

My answer does not settle what happens at the moment of rendering, only what happens to the stored value. It does not settle the loose drag range for this setting (see answer 14). It does not settle rounding of fractional input. And the script route rests on an inference I have spelled out rather than on a sentence about this setting.

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

**Outside the interface.** This is where the answer changes most, and it matters.

Inside, the panel is a guard: the manual says typed input is never wider than the hard limit, so an impossible value cannot be committed at all. Outside, there is no such guard described — the quoted behaviour is to take the value and quietly replace it with the nearest allowed one. Anything driving Blender from outside would therefore get a render at a different percentage from the one it asked for, with no complaint. **Inferred**, by the step above.

There is one documented way an outside caller learns that anything went wrong at all, and it only fires on a raised error:

> --python-exit-code <code>
>
> Set the exit-code in [0..255] to exit if a Python exception is raised
> (only for scripts executed from the command line), zero disables.

Source: manual, https://docs.blender.org/manual/en/5.0/advanced/command_line/arguments.html — read 2026-07-30.

Since a clamped value raises nothing, that switch would not fire on it. **Inferred**, and it depends on the clamping inference above.

Also documented on that same page, and relevant to what an outside caller can see at all: the logging switches `--log` and `--log-level`, whose levels run "fatal: Fatal errors only", "error: Errors only", "warning: Warnings", "info: Information about devices, files, configuration, operations", "debug", "trace" (same page, same date). Nothing states that a clamped value produces a line at any of those levels; I checked because a warning-level line would be the natural place for one.

**Observed — hand tests, 2026-08-09.** This line is now settled by observation rather than by a step. Every test above was run from a terminal with no window, on both machines: the value is corrected without a word, the exit status stays 0 against a control that proves the exit-code switch fires, and the render then proceeds at the corrected value. What has no counterpart outside the window — typing into the field, dragging the slider, the Info editor — was not run, for want of a machine with a screen.

**Operating system.** The sources do not say. Nothing on any of the pages quoted here distinguishes one operating system from another, and the release notes for 5.0 carry nothing on the subject.

**Observed — hand tests, 2026-08-09.** The tests above were run in Blender 5.0.1 on two operating systems: Ubuntu 22.04.5 LTS on an x86 machine (the rented Linux machine) and macOS 26.5.2 on Apple silicon (the Creator's Mac). Every stored value, every picture size, every exit status and the refusal message all matched. The only difference was the wording of one message, quoted above, about the limits of a machine whole number. **Windows was not tested** — no Windows machine has run it — so the Windows part of this line stays untested rather than answered.

**Where I looked, and what I ruled out.**
- Searched the manual, the API documentation and the release notes together for: "clamped", "out of range", "outside the range", "invalid value", "must be between", "rejected", "silently", "silently assign", "too large", "too small", "soft limit", "hard limit", "soft min", "soft max". The only useful hits are the two quoted above (the Input Fields page and the `bpy.props` page). Everything else was unrelated: auto-clamped animation handles, weight painting clamping, an index error in a glTF exporter bug fix, a Mesh page about bad indices, a maths helper describing its own edges, and a menu that is too large for the screen.
- Searched the API documentation for "ValueError", "TypeError" and "AttributeError". The value-error hits are all about a specific case that does not apply here — an add-on supplying its own read and write routines for a setting, where "The callback is responsible to ensure that value limits (min/max, length…) are respected. Otherwise a ValueError exception is raised" (https://docs.blender.org/api/5.0/bpy.props.html — read 2026-07-30). That is about a badly written add-on, not about an out-of-range value on a built-in setting, so I ruled it out.
- Read the API pages for the integer setting type and for the base structure type, looking for a stated behaviour on assignment. The base type's only documented error is about resolving a path to a setting, not about its value.
- Read the manual's Input Fields page in full.
- Read the manual's Info editor page, looking for what gets logged.
- Read the manual's Python Errors troubleshooting page in full: it is about mismatched libraries on Windows and about add-ons shipping compiled files. Nothing about setting values. Ruled out.
- Read the manual's Working Limits page in full: distances, frame count, text lengths. Ruled out.
- Read the command-line arguments page for the Python and logging switches, and searched it for "error", "exit code", "resolution", "percent" and "scale".
- Searched all Cycles 5.0 files for anything that checks or corrects this setting: the one occurrence found in answer 14 is the whole of it. Cycles does not validate it.
- Searched the 5.0 release notes for anything about value limits or clamping on this setting: nothing.

---

## Things a person with Blender 5.0 open could settle by hand in a few minutes

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

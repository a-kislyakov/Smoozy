# Settings — later questions (second pass), Blender 5.1

**Sources consulted, and the date.** Blender's 5.1 manual, Blender's 5.1 Python API documentation, the 5.1 release notes, and Cycles' own 5.1 files — all as the copies held in `Blender Sources/`, taken on 2026-07-30. That snapshot date is my date read for everything below.

I read only the question sheet and files inside Blender Sources/. I did not open any other part of this project.

One thing I must add so that line is not misleading: before I ran a single tool, the harness I run in had already pasted the text of this project's `CLAUDE.md` into my context automatically. I did not open it and I could not prevent it. It is a signpost only — it names a file called `Start Here.md` and says to read it. I did not read that file, and I did not read anything it points to, or anything else under `Preparation/`, `Audits/`, or `App/`. The only project file I opened was the question sheet named in my inputs, and the only other files I opened were inside `Blender Sources/`. I wrote no helper agents into this work, so there is nobody else's reading to vouch for.

Only questions 14 and 15 are answered here. The rest of the sheet was left alone.

---

## 14. What values can a render's resolution percentage be set to?

**Label: documented**, with one **inferred** part clearly marked below.

### The answer

The setting is called `resolution_percentage`. It holds whole numbers only, its stated range runs from 1 to 32767, and its default is 100.

Exact words, from the Python API documentation:

> resolution_percentage
>
> Percentage scale for render resolution (in [1, 32767], default 100)
>
> Type:
>
> int

Source: Python API documentation, https://docs.blender.org/api/5.1/bpy.types.RenderSettings.html — read 2026-07-30.

Two things follow straight from those words. "Type: int" means whole numbers: there is no half a percent. And the low end is 1, not 0, so the setting cannot be switched off by setting it to nothing.

The same documentation shows the default value being read back in a worked example:

> \>\>\> bpy.data.scenes[0].render.resolution_percentage
> 100

Source: Python API documentation, https://docs.blender.org/api/5.1/info_quickstart.html — read 2026-07-30.

The manual confirms that values above 100 are meaningful and not just tolerated — it says the control can *increase* the size, not only reduce it:

> Percentage
>
> Slider to reduce or increase the size of the rendered image relative to the Resolution values.
> This is useful for small test renders that have the same proportions as the final image.

Source: manual, https://docs.blender.org/manual/en/5.1/render/output/properties/format.html — read 2026-07-30.

### What the picture shows

The manual page above carries `[image: _images/render_output_properties_format_panel.png]`, captioned "Format panel." I opened it. It shows the Format panel with three stacked fields: "Resolution X" reading `1920 px`, "Y" reading `1080 px`, and beneath them a field labelled `%` reading `100%`. The `%` field is drawn as a slider — a coloured bar behind the value — and at 100 that bar is filled all the way across. **Documented** (it is part of the manual): the control is labelled `%`, it is a slider, and its shipped value reads 100%.

The manual describes that kind of control:

> Sliders, a second type of number field, have a colored bar in the background
> to display values over a range, e.g. percentage values.

Source: manual, https://docs.blender.org/manual/en/5.1/interface/controls/buttons/fields.html — read 2026-07-30.

**Inferred**, and here is the step: the bar in the picture is completely full at 100, and the manual says the bar shows the value's position across its range; so the range the bar spans ends at 100. The manual states separately that dragging is confined to the narrower "soft limit" range while typing may go wider:

> Most numerical values are restricted by "soft limit" and "hard limit" value ranges.
> Changing values by dragging with the mouse is restricted to the "soft limit" value range.
> Input via keyboard will allow the use of wider value ranges, but never wider than the "hard limit".

Source: manual, https://docs.blender.org/manual/en/5.1/interface/controls/buttons/fields.html — read 2026-07-30.

Putting those together: values above 100 are reachable, but not by dragging the slider — they have to be typed, or set from outside the interface. The reader has to supply the join between the picture and the soft-limit rule, so this is inferred, not documented.

### Is 1 to 32767 the hard range or the drag range?

The API documentation never says which kind of bound its `(in [...])` figures are. **Inferred**, and here is the step: the manual's own working-limits page states a hard ceiling for a different setting in plain words —

> The maximum number of frames for each scene is currently 1,048,574

Source: manual, https://docs.blender.org/manual/en/5.1/advanced/limits.html — read 2026-07-30.

— and the API documentation prints exactly that number as the top of the matching property's range:

> frame_end
>
> Final frame of the playback/rendering range (in [0, 1048574], default 250)

Source: Python API documentation, https://docs.blender.org/api/5.1/bpy.types.Scene.html — read 2026-07-30.

Where the manual names a hard working limit, the API's `(in [...])` figure equals it. So the `(in [1, 32767])` printed for the percentage is most likely its hard limit too, with the drag range being the narrower 1-to-100 the picture suggests. This is an inference from one matching pair, not a rule the documentation states.

One honest caution against that inference: at least one property in the same documentation prints a default that falls outside its own stated range — "Seed for the noise generation (in [1, 32767], default 0)" on https://docs.blender.org/api/5.1/bpy.types.LineStyleThicknessModifier_Noise.html. So the printed figures are not always a perfect account of what a property will accept.

### Where this setting lives

It sits on the scene's render settings — `bpy.types.RenderSettings` — which means it belongs to Blender itself, not to a render engine. Cycles' own files never define it; they read it in one place only, three consecutive lines computing a pixel size. *(Corrected from "in two places" by the citation check, 2026-08-09 (Opus 5): `resolution_percentage` occurs exactly once in the whole of `cycles_5.1`, which is what the record at the end of this document already said.)* **Observed**, from Cycles' files:

> `render_scale = scene.render.resolution_percentage / 100.0`
> `render_size_x = int(scene.render.resolution_x * render_scale)`
> `render_size_y = int(scene.render.resolution_y * render_scale)`

File: `cycles_5.1/ui.py`, lines 399–401 — read 2026-07-30.

The API documentation's own example render engine does the identical thing:

> `scale = scene.render.resolution_percentage / 100.0`
> `self.size_x = int(scene.render.resolution_x * scale)`
> `self.size_y = int(scene.render.resolution_y * scale)`

Source: Python API documentation, https://docs.blender.org/api/5.1/bpy.types.RenderEngine.html — read 2026-07-30.

Note the whole-number truncation in both: the pixel count is cut down to a whole number, not rounded.

### Release notes

Nothing in the 5.1 release notes changes this setting. I searched every release-note file for "percentage" and "resolution"; the four hits were about a Curves attribute, GPU kernels reloading across monitors with different scale, video-sequencer proxy display percentage, and a Multiresolution fix — none of them this setting. (Files searched: all of `release_notes_5.1`, whose pages carry addresses under https://developer.blender.org/docs/release_notes/5.1/ .)

### Not covered

- Whether 1 and 32767 are the hard bounds or the drag bounds is not stated outright anywhere; my answer on that is an inference from one matching pair.
- The sources never print the drag range as a number. I read it off a picture.
- Nothing here settles what happens when the percentage times the resolution truncates to zero pixels, or to a size a chosen output format cannot hold.
- Nothing here settles whether the percentage can be animated or driven, or whether it can differ between view layers.
- Nothing settles whether a file saved by another version can carry a value outside this range and what happens when it is opened.

### What a hand test settled

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

### Outside the interface

The values it accepts are a property of the data, not of the window, so the range holds however Blender is started. But two differences matter.

First, there is no command-line option for it. I read the full list of command-line arguments for 5.1 (https://docs.blender.org/manual/en/5.1/advanced/command_line/arguments.html, read 2026-07-30). The Render Options section offers `-b`, `-a`, `-S`, `-f`, `-s`, `-e`, `-j`, `-o`, `-E`, `-t`, plus Cycles' `--cycles-device` and `--cycles-print-stats`, plus Format Options `-F` and `-x`. None of them is a resolution percentage. So from outside, the setting can only be changed by running Python — `-P, --python <filepath>`, `--python-text <name>` or `--python-expr <expression>`, all listed on that same page — or by changing the file.

Second, the soft-limit business is purely a thing of the interface. Dragging a slider does not exist outside it, so the 1-to-100 drag range has no meaning there; only the wider stated range does.

**Observed — hand test, 2026-08-09.** Every test recorded above was run from a terminal with no window at all, on both machines, so this line is settled by observation rather than by a step: the limits and the values taken are the same when Blender is driven from outside its own window.

### Operating system

The sources say nothing that makes this setting differ by operating system. The command-line arguments page does carry operating-system-specific notes elsewhere on it (for example `--no-native-pixels`, described as being for high resolution displays on MacBook Retina), but nothing of the kind attaches to resolution percentage anywhere I searched.

**Observed — hand test, 2026-08-09.** The tests above were run in Blender 5.1.2 on two operating systems: Ubuntu 22.04.5 LTS on an x86 machine (the rented Linux machine) and macOS 26.5.2 on Apple silicon (the Creator's Mac). Every number was the same on both. The only difference was the wording of one message, quoted above, about the limits of a machine whole number. **Windows was not tested** — no Windows machine has run it — so the Windows part of this line stays untested rather than answered.

### Where I looked, for question 14

- Searched all of `api_5.1`, `manual_5.1`, `release_notes_5.1` and `cycles_5.1` for `resolution_percentage`. Six files hit: `bpy.types.RenderSettings`, `bpy.types.RenderEngine`, `info_quickstart`, two index files, and `cycles_5.1/ui.py`. I opened every one of them.
- Searched the whole manual for `Percentage`, `percent`, `resolution percentage`, `Resolution %`, `render percentage`, `render scale`, `scale of the render`. The only render-settings hit was the Format panel page. Other hits were node groups, dyntopo, Grease Pencil build, preferences, EEVEE film overscan, EEVEE light probes, the Scale compositor node, custom properties, physics, boids — all ruled out as other settings.
- Searched `api_5.1` for `Percentage scale` — exactly one hit, the property itself. Searched for `32767` across all four source sets to see whether the number is described anywhere as a limit in words; it appears as the top of many unrelated integer properties and is never explained.
- Read `bpy.types.RenderSettings` around the property to see its neighbours: `resolution_x` and `resolution_y` are both "(in [4, 65536])", `pixel_aspect_x` is "(in [1, 200])". Read the whole of the manual's Format page and its picture, the whole of `interface/controls/buttons/fields`, and the whole of `advanced/limits`.
- Read `advanced/command_line/arguments` and `advanced/command_line/render` in full to check for a percentage option. Ruled both out: no such option exists.
- Checked `cycles_5.1` for any definition of its own percentage — `grep -rn "resolution" cycles_5.1` returns only the two ui.py reads, a world sampling-map resolution, and unrelated kernel code. Ruled out: Cycles does not define this setting.

---

## 15. What does a render do when it is asked for a value outside what that setting accepts, and what does it report?

I read this as the sheet's closing note directs: about the resolution percentage from question 14, not about settings in general.

**Label: not in the sources** for this setting specifically. The nearest thing the sources do say is a general rule about integer settings, quoted below, and carrying it across to this setting is **inferred**.

### What the sources actually say

Blender's documentation of how integer settings are declared states plainly what happens on an out-of-range assignment, and that it happens without any message:

> min (int) – Hard minimum, trying to assign a value below will silently assign this minimum instead.
>
> max (int) – Hard maximum, trying to assign a value above will silently assign this maximum instead.

Source: Python API documentation, https://docs.blender.org/api/5.1/bpy.props.html — read 2026-07-30. (The same two sentences appear four times on that page: for `IntProperty`, `IntVectorProperty`, `FloatProperty` and `FloatVectorProperty`.)

**Inferred**, and here is the step, which a reader has to supply and which I cannot close from the sources: that page documents `bpy.props`, the tools a script uses to *define its own* settings. The resolution percentage is not defined that way — it is one of Blender's own built-in settings, and nothing in the sources says built-in settings obey the same rule. If they do, then setting the percentage to 0 or to -50 quietly stores 1, setting it to 50000 quietly stores 32767, and nothing at all is reported: no error, no warning, no message. If they do not, the sources do not say what happens instead.

From inside the interface there is a separate and firmer answer, because the interface will not let the value out of range in the first place:

> Input via keyboard will allow the use of wider value ranges, but never wider than the "hard limit".

Source: manual, https://docs.blender.org/manual/en/5.1/interface/controls/buttons/fields.html — read 2026-07-30. Typing 50000 into the field is therefore not a case of the render being "asked for" an out-of-range value; the field never accepts it. What is reported is nothing — the manual describes no message for this.

### What the sources do not say

I could not find any statement, anywhere in the 5.1 manual, the 5.1 Python API documentation, the 5.1 release notes or Cycles' 5.1 files, about what a *render* does when its resolution percentage is out of range, or what it prints when that happens. Not an error, not a warning, not a line in the terminal, not an exit code, not a refusal to start.

That silence is worth stating precisely, because the manual clearly *can* document this kind of behaviour when it wants to. It does so for a different render setting on the very same branch of the manual:

> CRF values outside the allowed range are clamped to the nearest allowed value.

Source: manual, https://docs.blender.org/manual/en/5.1/render/output/properties/output.html — read 2026-07-30. That sentence is about the video-encoding quality figure, not the resolution percentage, and I am quoting it only to show that the manual writes such sentences where it means to. No comparable sentence exists for the percentage.

### Not covered

- Whether Blender's own built-in settings clamp the way script-defined ones are documented to.
- Whether anything at all is written to the terminal, the Info editor, or the status bar when it happens.
- What a render does when the percentage times the resolution truncates to zero pixels — both the Cycles code and the API's example engine cut the result down with a whole-number truncation, and nothing says what a zero-width or zero-height render does.
- What happens to a file that already holds an out-of-range value — for instance one written by a different version, or by editing outside Blender.
- Whether the render still starts, still writes a file, and with what exit status.

### What a hand test settled

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

### Outside the interface

This is where the answer matters most and where the sources are thinnest.

There is no command-line option that sets the percentage (see question 14), so the only way to ask for a bad value from outside is to run Python — `-P, --python <filepath>`, `--python-text <name>` or `--python-expr <expression>`, all on https://docs.blender.org/manual/en/5.1/advanced/command_line/arguments.html, read 2026-07-30. The interface's protection quoted above does not apply there: there is no field refusing the keystrokes, so the assignment reaches the setting.

Whether it then clamps quietly or raises is exactly what the sources do not say. What they do say is what would happen *if* it raised: Blender can be told to exit with a chosen code when a script raises.

> --python-exit-code <code>
>
> Set the exit-code in [0..255] to exit if a Python exception is raised
> (only for scripts executed from the command line), zero disables.

Source: manual, https://docs.blender.org/manual/en/5.1/advanced/command_line/arguments.html — read 2026-07-30. That option is off unless asked for, so by default a raising script would not, by itself, change how Blender exits.

If the general clamping rule does hold, then from outside there is nothing to detect: the value is silently changed and the render proceeds at a size nobody asked for. That is an inference, and it is the single most important thing in these two answers that the sources do not confirm. It should be settled by hand — see the list below.

**Observed — hand tests, 2026-08-09.** This line is now settled by observation rather than by a step. Every test above was run from a terminal with no window, on both machines: the value is corrected without a word, the exit status stays 0 against a control that proves the exit-code switch fires, and the render then proceeds at the corrected value. What has no counterpart outside the window — typing into the field, dragging the slider, the Info editor — was not run, for want of a machine with a screen.

### Operating system

The sources say nothing that makes any of this differ by operating system. The `--python-exit-code` option and the command-line arguments generally are documented without operating-system qualification.

**Observed — hand tests, 2026-08-09.** The tests above were run in Blender 5.1.2 on two operating systems: Ubuntu 22.04.5 LTS on an x86 machine (the rented Linux machine) and macOS 26.5.2 on Apple silicon (the Creator's Mac). Every stored value, every picture size, every exit status and the refusal message all matched. The only difference was the wording of one message, quoted above, about the limits of a machine whole number. **Windows was not tested** — no Windows machine has run it — so the Windows part of this line stays untested rather than answered.

### Where I looked, for question 15

- Searched all of `manual_5.1` and `api_5.1` for `clamped`, `clamps`, `soft limit`, `hard limit`, `soft_min`, `soft_max`, `hard_min`, `hard_max`, `out of range`, `out-of-range`, `outside the`, `out of the range`, `exceeds`, `invalid value`, `not in range`, `silently`, `is limited to`, `limited to the range`, `rounded to`, `nearest allowed`, `nearest valid`, `reverts`, `reset to`, `refuses`, `rejected`, `ignores the value`, `no effect`, `too small`, `minimum resolution`, `zero pixels`, `0 pixels`, `cannot render`, `unable to render`, `render failed`, `aborted`, `ValueError`.
- The only hits that bear on the question at all are the four I quoted or named: the `bpy.props` min/max sentences, the "soft limit"/"hard limit" paragraph in the manual's number-fields page, the CRF clamping sentence on the Output page, and `--python-exit-code`. Everything else was other subjects — clamping of light samples, of weights, of curve handles, of Freestyle crease angles, of constraint ranges, of fluid intersection depth, of shader-compilation threads — all ruled out as different settings.
- Read in full: `api_5.1/bpy.props.txt` around all four number-property definitions; `api_5.1/bpy.types.IntProperty.txt` (which lists `hard_min`, `hard_max`, `soft_min`, `soft_max` but describes all four only as "value used by buttons", explaining nothing about assignment); `api_5.1/info_api_reference.txt`, which is the page about reading the API reference and never explains what its `(in [...])` figures mean; `api_5.1/info_gotcha.txt` and its section list, which covers crashes, threads, internal data, operators, mesh modes, armatures and file paths — none of them property ranges; `api_5.1/bpy.types.bpy_struct.txt`, searched for `range`, `clamp` and `limit`, which yielded only unrelated custom-property notes.
- Read `manual_5.1/advanced/limits.txt` in full, on the chance it listed a limit for this setting. It covers space precision, the frame maximum, and text-field lengths only. Ruled out.
- Read the whole of `advanced/command_line/arguments.txt` and `advanced/command_line/render.txt`, looking for any statement about bad values given on the command line. The only range printed for any option is `-t, --threads … [1-1024]` and `--python-exit-code … [0..255]`; neither page says what happens if you exceed them.
- Searched all of `release_notes_5.1` for `clamp`, `range`, `limit` and `property`. The clamping hits were a sequencer retiming option removal and an audio-bitrate ceiling being raised; the property hits were about brush enums, a new `bpy.app.cachedir`, and name-property overrides. All ruled out as other subjects.
- Searched `cycles_5.1` for any validation or clamping of this setting. Cycles only reads it (`ui.py` lines 399–401) and never checks it. Ruled out.

---

## Things a person with Blender 5.1 open could settle in a few minutes

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

# Settings — later questions, second pass (Blender 4.5)

Sources consulted, all read on **2026-07-30** (the snapshot date given to me): Blender's manual for 4.5, Blender's Python API documentation for 4.5, Blender's release notes for 4.5, and Cycles' own files for 4.5. I also opened one picture from the manual, named in the answer that uses it.

I read only the question sheet and files inside `Blender Sources/`. I did not open any other part of this project.

I did not use the web. I started no helper agents, so there is nobody else's reading to vouch for — everything below I read myself.

Questions answered: 14 and 15. The rest of the sheet was left alone.

---

## 14. What values can a render's resolution percentage be set to?

**Label: documented.**

**The answer.** Whole numbers from **1 to 32767**, and the setting starts at **100**. It is a whole-number setting, so it takes no fractions. Zero is not in the range, and neither is any negative number. Values above 100 are allowed and are meant to be used — the manual says the slider both reduces and increases the size.

**The exact words that carry it.**

> resolution_percentage
>
> Percentage scale for render resolution
>
> Type:
>
> int in [1, 32767], default 100

Source: Blender's Python API documentation, https://docs.blender.org/api/4.5/bpy.types.RenderSettings.html — read 2026-07-30.

> Percentage
>
> Slider to reduce or increase the size of the rendered image relative to the Resolution values.
> This is useful for small test renders that have the same proportions as the final image.

Source: Blender's manual, https://docs.blender.org/manual/en/4.5/render/output/properties/format.html — read 2026-07-30.

The manual's own example of what the number means:

> The x and y resolution of the rendered
> image. This factors in the resolution scale as well, so if the scene
> resolution is 1000x600 and the scale is 50%, then resolution_x and
> resolution_y will be 500 and 300, respectively.

Source: Blender's manual, https://docs.blender.org/manual/en/4.5/files/file_paths.html — read 2026-07-30.

**The picture.** The Format panel screenshot on the manual page above (`_images/render_output_properties_format_panel.png`) shows the setting as a field labelled `%`, sitting under `Resolution X` (`1920 px`) and `Y` (`1080 px`), reading `100%`. It is drawn as a slider — a coloured bar behind the number — and at 100% the bar fills the whole width of the field. **Documented** (it is part of the manual): the setting is shown as a slider and its shipped value is 100%. That the bar is full at 100% is what the picture shows; that the slider therefore *stops* at 100 for dragging is not shown, and I am not claiming it.

**Two ranges, and only one of them is in the sources.** The manual says number settings in Blender generally have two ranges:

> Most numerical values are restricted by "soft limit" and "hard limit" value ranges.
> Changing values by dragging with the mouse is restricted to the "soft limit" value range.
> Input via keyboard will allow the use of wider value ranges, but never wider than the "hard limit".

Source: Blender's manual, https://docs.blender.org/manual/en/4.5/interface/controls/buttons/fields.html — read 2026-07-30.

The API page gives **one** range for this setting, `[1, 32767]`. The sources nowhere state what this setting's *soft* range is, so I cannot say how far the slider can be dragged as opposed to how far a typed number can go. Blender does keep both numbers and does expose them — `bpy.types.IntProperty` lists `hard_max`, `hard_min`, `soft_max`, `soft_min`, each "Maximum value used by buttons" / "Minimum value used by buttons" (https://docs.blender.org/api/4.5/bpy.types.IntProperty.html, read 2026-07-30) — but the documentation does not print this setting's soft pair anywhere. This is on the hands-on list at the end.

**The setting does not belong to a render engine.** I searched every Python file shipped with Cycles for 4.5 (`__init__.py`, `camera.py`, `engine.py`, `operators.py`, `osl.py`, `presets.py`, `properties.py`, `ui.py`, `version_update.py`) for `resolution_percentage`, `resolution` and `percent`. `resolution_percentage` and `percent` do not appear at all; `resolution` appears only for unrelated Cycles settings (`sample_map_resolution`, `simplify_volumes` shown as "Volume Resolution", and a tiled-rendering description). **Observed** — from `Blender Sources/cycles_4.5/`, files as listed, read 2026-07-30: Cycles defines no resolution-percentage setting of its own.

**Not covered.**
- What the soft (drag) range is, as opposed to the range printed on the API page.
- What happens to a fractional number handed to it from a script — whether it is cut down to a whole number or refused. The sources say the type is `int` and nothing more.
- What pixel size comes out at the extremes. The manual's worked example (1000×600 at 50% → 500×300) divides evenly; nothing in the sources says how a number that does not divide evenly is rounded. The API documentation carries a worked example in the custom-render-engine sample that truncates — `scale = scene.render.resolution_percentage / 100.0`, `self.size_x = int(scene.render.resolution_x * scale)` (https://docs.blender.org/api/4.5/bpy.types.RenderEngine.html, read 2026-07-30) — but that is sample code for somebody writing their own engine, not a statement about what Blender's own engines do.
- Whether the value can be animated or driven, and whether that could carry it outside the range.

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

None of the eighteen is in Cycles' own files.

**Outside the interface.** The range and starting value quoted above come from the Python API documentation, which *is* the outside-the-interface surface: the setting is reached as `bpy.data.scenes[0].render.resolution_percentage`, and the API quick-start shows exactly that, returning `100` (https://docs.blender.org/api/4.5/info_quickstart.html, read 2026-07-30). There is **no command-line switch for it**: I read the whole command-line arguments page and the whole command-line rendering page and the only hit for "resolution" is `--no-native-pixels`, about high-resolution displays, and there is no hit for "percent" or "%" at all (https://docs.blender.org/manual/en/4.5/advanced/command_line/arguments.html and https://docs.blender.org/manual/en/4.5/advanced/command_line/render.html, read 2026-07-30). So from outside, it is set either by editing the file or by running Python (`-P`, `--python-text`, `--python-expr`, all on the arguments page). The sources state no difference in what values are accepted between a background run and a run with the window open. The one part of the answer that is interface-only is the drag-versus-typing distinction quoted above, which has no meaning with no window.

**Observed — hand test, 2026-08-09.** Every test recorded above was run from a terminal with no window at all, on both machines, so this line is settled by observation rather than by a step: the limits and the values taken are the same when Blender is driven from outside its own window.

**Operating system.** The sources do not say. None of the pages quoted above carries any operating-system qualification for this setting. (The command-line arguments page does mark items that are operating-system-specific where they exist, for example "(Windows only)" on `-con`, and no such mark appears anywhere near this setting — but that is my reading of the absence, not a statement in the sources.)

**Observed — hand test, 2026-08-09.** The tests above were run in Blender 4.5.12 on two operating systems: Ubuntu 22.04.5 LTS on an x86 machine (the rented Linux machine) and macOS 26.5.2 on Apple silicon (the Creator's Mac). Every number was the same on both. The only difference was the wording of one message, quoted above, about the limits of a machine whole number. **Windows was not tested** — no Windows machine has run it — so the Windows part of this line stays untested rather than answered.

**Where I looked.** Searched the whole 4.5 API folder for `resolution_percentage` (5 hits: the RenderSettings entry, the quick-start example, the custom-engine example, and two index entries). Searched the whole 4.5 manual for `Resolution %`, `resolution percentage`, `Resolution Percentage`, `percentage scale`, `Percentage Scale`, `% Scale`, `resolution scale`, `render scale`, `scale of the render`, and for `Percentage` on its own (14 hits across the manual, of which exactly one — `render/output/properties/format` — is this setting; the others are dyntopo detail, a Grease Pencil modifier, the interface preferences, an EEVEE light-probe fade, EEVEE overscan, hair nodes, custom-property subtypes, compositor Split and Scale nodes, physics view percentage, and boid speed). Opened the Format panel picture. Searched the whole 4.5 API folder for `soft_min`/`soft_max`/`hard limit` and read `bpy.types.IntProperty`. Searched the whole 4.5 release-notes folder for `resolution`, `percentage`, `scale`, `format panel`, `output panel`, `render size` — the only entries touching this setting are two fixed-bug lines, quoted under question 15. Searched all of Cycles' 4.5 Python files as described above.

---

## 15. What does a render do when it is asked for a value outside what that setting accepts, and what does it report?

**The question was ambiguous, and here is the reading I took.** The sheet's own closing note says questions 14 and 15 are about "that one setting", so I answer for the **resolution percentage**. The other reading is the broader one — any render setting given a value it does not accept. As it happens the two rules I found are written about number settings in general rather than about this one setting, so they answer the broad reading too; what is specific to the percentage is only the numbers 1 and 32767.

**Label: documented for the general rule; inferred for applying it to this setting.** The step is written out below.

**The answer, in short.** There are two ways to hand it a bad number, and the sources describe both.

*Typed into the field with the window open.* The manual says a typed value is allowed past the drag range but never past the hard range:

> Input via keyboard will allow the use of wider value ranges, but never wider than the "hard limit".

Source: Blender's manual, https://docs.blender.org/manual/en/4.5/interface/controls/buttons/fields.html — read 2026-07-30. **Documented** that the typed value cannot go past the hard limit. The manual does **not** say what you see when you try — whether the number snaps back, whether a message appears, or nothing at all.

*Set from a script.* The Python API states plainly what happens, and states that nothing is said about it:

> min (int) – Hard minimum, trying to assign a value below will silently assign this minimum instead.
>
> max (int) – Hard maximum, trying to assign a value above will silently assign this maximum instead.

Source: Blender's Python API documentation, https://docs.blender.org/api/4.5/bpy.props.html — read 2026-07-30. The same sentences appear four times on that page, once for each of the whole-number and decimal property kinds.

**So: the value is pinned to the nearest end of the range, and nothing is reported.** "Silently" is the documentation's own word. No error, no warning, no message.

**The step I took, and why this is inferred and not documented.** The sentences above are on the page that describes how *new* settings are defined from Python. The resolution percentage is not one of those — it is one of Blender's own built-in settings. The documentation never says outright that built-in settings follow the same rule. What I leaned on is that both are described in the same terms: the percentage is published as "int in [1, 32767]", a whole-number setting with a hard low end and a hard high end, exactly the pair whose behaviour the sentences above define. A reader has to accept that one paragraph applies to the other setting. That is a supplied step, so the label is **inferred**. If that step is wrong, the honest position is that the sources do not say what a script gets when it assigns 0 or 40000 to this setting.

**What would carry a complaint outward, if there were one.** For a run driven from outside, the documented channel for a Python failure is:

> --python-exit-code <code>
>
> Set the exit-code in [0..255] to exit if a Python exception is raised
> (only for scripts executed from the command line), zero disables.

Source: Blender's manual, https://docs.blender.org/manual/en/4.5/advanced/command_line/arguments.html — read 2026-07-30. Since the documented behaviour is a silent pin rather than a raised failure, this channel would not fire — **inferred**, on the same step as above. And because there is no command-line switch that sets the percentage at all (see question 14), there is no command-line parsing complaint to be had either.

**The only thing the sources say about this setting misbehaving** is a fixed-bug line, and it is about something else:

> Blender crashing when changing the frame if the render resolution percentage has just been changed following a render, and there's an image editor open that's set to render result.

Source: Blender's release notes, https://developer.blender.org/docs/release_notes/4.5/bugfixes/ — read 2026-07-30. That page opens "There are a total of 494 commits in 4.5 that fixed issues that existed in Blender 4.4 or older." A second line on the same page reads "VSE Some strips wrongly occlude strips below, when render resolution scale is small". Both are **documented** statements that these faults were fixed in 4.5. Neither concerns a value outside the accepted range, and I am not offering them as an answer to the question — they are here because they are the only places in the whole 4.5 release notes where this setting appears, and the record of looking should show that.

**Not covered.**
- What the interface actually shows when you type an out-of-range number — snapped value, refusal, or a message. The manual states the restriction and stops.
- Whether Blender's own built-in settings really follow the "silently assign" rule quoted from the property-definition page. That is the inference above; nothing states it for built-in settings.
- What happens with a value that is *inside* the range but produces an unusable picture — for example a very small percentage against a small resolution, where the multiplication rounds down towards no pixels at all. The sources describe no floor, no warning, and no behaviour here.
- Whether a fractional number from a script is cut down or refused.
- Whether a driver or an animated value that lands outside the range is pinned in the same way.
- Whether anything is written to the render log or the status line during a render started with a pinned value.

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

**Outside the interface.** This is the part that survives best. The "silently assign" rule is written about setting values from Python, which is precisely the outside-the-interface route, and the run-from-outside failure channel (`--python-exit-code`) is documented on the command-line page. The typed-value rule is the opposite: it describes a person typing into a field and has no counterpart in a run with no window. The sources state no difference between the two situations in what values are finally accepted — both end at the same hard range.

**Observed — hand tests, 2026-08-09.** This line is now settled by observation rather than by a step. Every test above was run from a terminal with no window, on both machines: the value is corrected without a word, the exit status stays 0 against a control that proves the exit-code switch fires, and the render then proceeds at the corrected value. What has no counterpart outside the window — typing into the field, dragging the slider, the Info editor — was not run, for want of a machine with a screen.

**Operating system.** The sources do not say. Neither the property-definition page, nor the input-fields page, nor the command-line arguments entry quoted here carries any operating-system qualification.

**Observed — hand tests, 2026-08-09.** The tests above were run in Blender 4.5.12 on two operating systems: Ubuntu 22.04.5 LTS on an x86 machine (the rented Linux machine) and macOS 26.5.2 on Apple silicon (the Creator's Mac). Every stored value, every picture size, every exit status and the refusal message all matched. The only difference was the wording of one message, quoted above, about the limits of a machine whole number. **Windows was not tested** — no Windows machine has run it — so the Windows part of this line stays untested rather than answered.

**Where I looked.** Searched the whole 4.5 manual for `out of range`, `invalid value`, `clamped to`, `outside the allowed`, `soft limit`, `hard limit`, `silently` — the only relevant hits are the input-fields page quoted above and the custom-properties page (which describes `Min, Max`, `Soft Limits` and `Soft Min, Max` for user-added properties: "Enables limits that the Property Value slider can be adjusted to without having to input the value numerically", https://docs.blender.org/manual/en/4.5/files/custom_properties.html, read 2026-07-30). All other `clamped` hits in the manual are unrelated features — weight painting, graph-editor handles, the Map Range node, the Clamp To constraint, the Transformation constraint, viewport playback, Solidify, dynamic paint. Searched the whole 4.5 API folder for `clamp`, `out of range`, `outside the range`, `ValueError`, `TypeError`, `RuntimeError`, `AttributeError`, `soft_min`, `soft_max`, `hard limit`, `silently`; read `bpy.props`, `bpy.types.IntProperty`, and the failure examples in the gotchas and overview pages (which cover wrong *types* and wrong operator context, for example "TypeError: bpy_struct: item.attr= val: Object.name expected a string type, not str", but never a value outside a range). Read the command-line arguments page in full looking for an argument that sets this setting and for how bad argument values are reported — searched it for `invalid`, `error`, `warn`, `ignored`, `out of range`; nothing addresses this. Searched the manual's render section for documented render-time error messages — the only ones are the Cycles GPU "Error Messages" section and EEVEE's "Shadow buffer full", neither related. Searched the whole 4.5 release-notes folder as described under question 14; the Python-API release-notes page for 4.5 has no mention of `limit`, `range` or `clamp`.

---

## Things a person sitting at a computer with Blender 4.5 open could settle in a few minutes

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

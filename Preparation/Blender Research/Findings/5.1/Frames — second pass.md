# Frames — Blender 5.1 — second pass (questions 7 and 10)

Sources consulted, all read on **2026-07-30** (the snapshot date given to me):

- The Blender 5.1 manual (`Blender Sources/manual_5.1`), including its picture files in `_images`.
- The Blender 5.1 Python API documentation (`Blender Sources/api_5.1`).
- The Blender 5.1 release notes (`Blender Sources/release_notes_5.1`).
- Cycles' own files as shipped with 5.1 (`Blender Sources/cycles_5.1`).

I read only the question sheet and files inside Blender Sources/. I did not open any other part of this project.

Only the 5.1 folders were opened. The other version folders that sit beside them were not opened. I did the work myself and started no helper agents, so the line above covers everything that was read.

---

## 7. Can Blender be given a set of frames that is not one plain run — single frames, or two separate stretches?

**Short answer: yes, but only from the command line, and only in the one form the manual describes. Blender's own Frame Range panel offers a single stretch with an optional fixed interval, and nothing else.**

**A reading I had to choose.** The question is slightly ambiguous. I answered the reading I think is meant: *can Blender be told to render a set of frames that is not a single unbroken start-to-end run — for example a handful of individual frames, or two separate stretches.* The other reading would be about what a frame range setting is allowed to contain as a value, which is question 8's ground, so I did not answer that here.

### What the interface offers

LABEL: **documented**

The Frame Range panel holds three settings and no more:

> "Frame Start, End
>
> Set the Start and End frames for Rendering Animations.
>
> Step
>
> Controls the number of frames to advance for each frame rendered, a value above 1 skips frames.
> This is also used when viewing rendered animations."

Source: manual — https://docs.blender.org/manual/en/5.1/render/output/properties/frame_range.html — read 2026-07-30.

The picture on that same page confirms what the panel contains. I opened `Blender Sources/manual_5.1/_images/render_output_properties_frame_range_panel.png` (referenced from the page above as `[image: _images/render_output_properties_frame_range_panel.png]`, captioned "Frame Range panel."). Visible in it: a header "Frame Range", three number fields labelled "Frame Start" (value 1), "End" (value 250) and "Step" (value 1), and a closed sub-panel labelled "Time Stretching". Nothing else is visible. Label for what the picture shows: **documented**.

The same three settings are what the Python API exposes on the scene, one of each — not a list:

> "frame_end
>
> Final frame of the playback/rendering range (in [0, 1048574], default 250)"

> "frame_start
>
> First frame of the playback/rendering range (in [0, 1048574], default 1)"

> "frame_step
>
> Number of frames to skip forward while rendering/playing back each frame (in [0, 1048574], default 1)"

Source: API documentation — https://docs.blender.org/api/5.1/bpy.types.Scene.html — read 2026-07-30.

The step setting gives a fixed interval — every second frame, every fifth frame — which is a set that is not one plain run, but it is not an arbitrary set and it cannot express two separate stretches.

That there is **no** way in the panel to name a second stretch or a scattered list is a conclusion I drew from the absence of any such setting after searching (see the record below). Label for that conclusion: **inferred** — the step taken is: the panel page lists exactly three settings and the picture shows exactly three fields, and no other page or property I could find offers more, therefore the panel offers no more.

### The alternative range that does *not* count

LABEL: **documented**

Blender has a second start/end pair, the Preview Range, and the manual states plainly that it is not a rendering control:

> "This range only affects the preview in the 3D Viewport; it doesn't affect rendering."

Source: manual — https://docs.blender.org/manual/en/5.1/animation/animation_editors.html — read 2026-07-30.

The API says the same in its own words:

> "use_preview_range
>
> Use an alternative start/end frame range for animation playback and view renders (default False)"

Source: API documentation — https://docs.blender.org/api/5.1/bpy.types.Scene.html — read 2026-07-30.

So the presence of a second range in Blender does not give you two stretches for a real render.

### The command line: yes

LABEL: **documented**

> "-f, --render-frame <frame>
>
> Render frame <frame> and save it.
>
> +<frame> start frame relative, -<frame> end frame relative.
>
> A comma separated list of frames can also be used (no spaces).
>
> A range of frames can be expressed using .. separator between the first and last frames (inclusive)."

Source: manual — https://docs.blender.org/manual/en/5.1/advanced/command_line/arguments.html — read 2026-07-30.

This is the one place in the sources where Blender is handed a set of frames rather than a single stretch. A comma separated list of single frames is stated outright. A stretch expressed with `..` is stated outright.

**Whether the two can be mixed in one `-f` — for example two separate stretches written as `1..10,50..60` — is not stated.** The manual gives the comma list and the `..` stretch as two separate sentences and never shows them combined, and there is no worked example of either in the sources. See the record of looking below. Label for the mixed form: **not in the sources**.

**Observed — hand test 9, 2026-08-05.** The mixed form works. Blender 5.1.2 was started from a terminal with no window, rendering its own startup scene (the default cube) in Workbench at 64 by 64 pixels, PNG output into an empty folder, with the argument `-f 1..3,10..12`. It printed:

```
00:00.315  render           | Rendering single frame
00:00.315  render           | Rendering frame 1
00:00.449  render           | Saved: '…/g_0001.png'
00:00.449  render           | Rendering single frame
00:00.449  render           | Rendering frame 2
00:00.455  render           | Saved: '…/g_0002.png'
00:00.455  render           | Rendering single frame
00:00.455  render           | Rendering frame 3
00:00.460  render           | Saved: '…/g_0003.png'
00:00.460  render           | Rendering single frame
00:00.460  render           | Rendering frame 10
00:00.465  render           | Saved: '…/g_0010.png'
00:00.465  render           | Rendering single frame
00:00.465  render           | Rendering frame 11
00:00.470  render           | Saved: '…/g_0011.png'
00:00.470  render           | Rendering single frame
00:00.470  render           | Rendering frame 12
00:00.475  render           | Saved: '…/g_0012.png'
```

All six files were written and no error was printed. So two separate stretches are expressible in one `-f`, and the reading this document could not settle is settled. Note also what Blender calls each item: `Rendering single frame`, never `Rendering animation`. Run on the Creator's MacBook Pro, M1 Pro chip, macOS 26.5.2.

**Observed — hand tests 10 and 11, 2026-08-05.** Three of the other things marked not covered below are settled by the same means, on the same machine and date.

*A list may repeat a frame and may be out of order.* `-f 2,2,3` rendered frame 2 twice, over the same file, and left two files. `-f 5,1,9` rendered 5, then 1, then 9 — the order written, not sorted.

*A malformed list is refused and nothing is rendered, but Blender still ends with the exit code 0.* Blender 5.1.2 printed exactly:

```
Error: incorrect comma use '-f / --render-frame 1,,3'.
Warning: negative range ignored '-f / --render-frame 3..1'.
Error: not a number '-f / --render-frame 1-3'.
```

one message per run, and wrote no file in any of the three.

**Observed — hand test 20, 2026-08-05.** `-f` with a list behaves the same with a window open. The same `-f 1..3,10..12` command was run without `-b`, so Blender 5.1.2 opened its window; it wrote the same six files with the same names. Same machine and date.

Two further things the same page states about `-f`, which bear on how a set is given:

- Frames may be written relative to the range ends: `+<frame>` is relative to the start frame, `-<frame>` relative to the end. The worked example page shows `-f -2` with the explanation "Render only the second last frame." (manual — https://docs.blender.org/manual/en/5.1/advanced/command_line/render.html — read 2026-07-30).
- The single-frame form is shown as `blender -b file.blend -f 10`, explained as "Render only the 10th frame." (same page).

### Giving more than one render instruction in a single command

LABEL: **inferred**

The manual states:

> "Arguments are executed in the order they are given!"

and

> "Always position -f or -a as the last arguments."

Source: manual — https://docs.blender.org/manual/en/5.1/advanced/command_line/render.html — read 2026-07-30.

The step I took: if arguments are executed in the order given, then a command that sets a start and end, renders, then sets a different start and end and renders again would produce two separate stretches. The sources never show this being done and never say it is allowed, so this is my reasoning, not theirs. It is inference, and a reader should treat it as untested.

**Observed — hand test 24, 2026-08-05.** It is no longer untested, and the inference holds. Blender 5.1.2 was started from a terminal with no window, rendering its own startup scene in Workbench at 64 by 64 pixels to PNG into an empty folder, with the arguments `-s 1 -e 2 -a -s 20 -e 21 -a`. It printed:

```
00:00.314  render           | Rendering animation (frames 1..2)
00:00.469  render           | Saved: '…/m_0001.png'
00:00.474  render           | Saved: '…/m_0002.png'
00:00.474  render           | Rendering animation (frames 20..21)
00:00.479  render           | Saved: '…/m_0020.png'
00:00.484  render           | Saved: '…/m_0021.png'
```

Four files were left: `m_0001.png`, `m_0002.png`, `m_0020.png`, `m_0021.png`. Two separate stretches, from one command, announced as two separate animation renders. Run on the Creator's MacBook Pro, M1 Pro chip, macOS 26.5.2.

### The one other non-contiguous set in the sources

LABEL: **documented**

There is a render that picks its own scattered set of frames, but it is a viewport preview render, not a full render:

> "Render Keyframes
>
> To render an animation, but only those frames that have a keyframe, use
> 3D Viewport ‣ View ‣ Render Playblast on Keyframes. This only renders those frames for which
> the selected objects have an animation key. The other frames are still written to the output, but will simply
> repeat the last-rendered frame."

Source: manual — https://docs.blender.org/manual/en/5.1/editors/3dview/viewport_render.html — read 2026-07-30.

The same page states the limit on it:

> "Viewport rendering only works for the Workbench and EEVEE render engines.
> It's not supported for Cycles."

Source: same page, same date.

The Python side of the same thing:

> "render_keyed_only (bool) – Render Keyframes Only, Render only those frames where selected objects have a key in their animation data. Only used when rendering animation (optional)"

on `bpy.ops.render.opengl(*, animation=False, render_keyed_only=False, sequencer=False, write_still=False, view_context=True)`.

Source: API documentation — https://docs.blender.org/api/5.1/bpy.ops.render.html — read 2026-07-30.

Note carefully what the manual says this leaves on disk: it does **not** leave gaps. Every frame in the range is written; the un-keyed ones repeat the previous rendered frame. So this is a scattered set of frames *rendered*, not a scattered set of frames *written*.

**Observed — hand test 26, 2026-08-05.** This route is not available to a Blender driven from outside its own window. Calling the operator with the arguments quoted above — `bpy.ops.render.opengl(animation=True, render_keyed_only=True)` — in a Blender 5.1.2 started from a terminal with no window gave:

```
Error: Cannot use OpenGL render in background mode (no opengl context)
RAISED: RuntimeError Error: Cannot use OpenGL render in background mode (no opengl context)
FILES: []
```

No file was written. The same command gave the same message in Blender 4.5.12, 5.0.1 and 5.2.0. Run on the Creator's MacBook Pro, M1 Pro chip, macOS 26.5.2.

**Observed — hand test 32, run by the Creator, 2026-08-05.** With a window it does run, and the manual's account of what it writes holds. In Blender 5.1.2 the Creator took the default cube scene, put a keyframe on the cube at frame 3 and another at frame 5 and none anywhere else, set the frame range to 1–6 and an image output path, and used the keyframes-only item in the 3D Viewport's View menu. Their own words: "Anyway, all versions create 6 files, which are identical."

Six files for a six-frame range, though only two frames carry a key — so this really is a scattered set of frames *rendered* and a full set of frames *written*, as the manual says. The word "identical" is the Creator's own; they did not say what the six files were named or how big they were, and that is not recorded.

The menu item this document quotes as "Render Playblast on Keyframes" is labelled exactly that in Blender 5.1.2. The View menu reads:

> Render Viewport Preview
> Render Playblast
> Render Playblast on Keyframes

with the tooltip "Take a snapshot of the active viewport." and, underneath it, "Python: bpy.ops.render.opengl()". Blender 4.5.12 labels the same three items "Viewport Render Image", "Viewport Render Animation" and "Viewport Render Keyframes"; 5.0.1 and 5.2.0 match 5.1.2.

Run on the Creator's Mac — a MacBook Pro, M1 Pro chip, macOS 26.5.2 — so this settles macOS and leaves Windows and Linux open.

### The full-render operator takes one stretch only

LABEL: **documented**

> "bpy.ops.render.render(*, animation=False, write_still=False, use_viewport=False, use_sequencer_scene=False, layer='', scene='', frame_start=0, frame_end=0)"

> "frame_start (int) – Start Frame, Frame to start rendering animation at. If not specified, the scene start frame will be assumed. This should only be specified if doing an animation render (in [-inf, inf], optional)"

> "frame_end (int) – End Frame, Frame to end rendering animation at. If not specified, the scene end frame will be assumed. This should only be specified if doing an animation render (in [-inf, inf], optional)"

Source: API documentation — https://docs.blender.org/api/5.1/bpy.ops.render.html — read 2026-07-30.

One start, one end, no list.

A release note worth recording alongside it, since it concerns exactly this operator's handling of the range in this version's fix list:

> " * API bpy.ops.render.render() uses wrong frame range [[ae97e275cff](https://projects.blender.org/blender/blender/commit/ae97e275cff1d3331eaa9a4aef10da742a1a506d)] - Backported to 5.0.1"

Source: release notes — https://developer.blender.org/docs/release_notes/5.1/bugfixes/ — read 2026-07-30. (Listed under "Render & Cycles" in the bugfix list; the page opens "There are a total of 573 commits in 5.1 that fixed issues that existed in Blender 5.0 or older.") *(Citation check, 2026-08-05: the section was named "Rendering & Cycles"; the heading reads "Render & Cycles". Corrected to the page.)*

### Not covered

- Whether a comma list and a `..` stretch can be combined in one `-f`, which is what "two separate stretches" would most naturally need.
- Whether the frames in a `-f` list may repeat, may be out of order, or may fall outside the scene's start/end.
- Whether there is any limit on how many frames a `-f` list may hold.
- Whether `-f` and `-a` may be given in the same command, and what happens if they are.
- Whether the step setting interacts with a `-f` list at all.
- Whether a scattered set can be given at all when Blender is driven through Python inside a running session rather than through the command line. The operator above takes one stretch; whether a script may simply call it repeatedly is not addressed by the sources.

**Observed — hand tests 9, 10, 27 and 24, 2026-08-05.** Four of those six are settled, all with Blender 5.1.2 started from a terminal with no window, rendering the default cube scene in Workbench at a small size to PNG. Same machine and date throughout.

*A comma list and a `..` stretch can be combined* — recorded in full above.

*A list may repeat a frame and may be out of order* — recorded in full above. Frames outside the scene's start and end were rendered without complaint: with the scene's own range left at 1 to 250, `-f 5,1,9` and `-f 1..3,10..12` all rendered.

*A list of five thousand frames is accepted*, rendering all five thousand with the exit code `0`. A list of two hundred thousand never reached Blender: the shell refused it first, with `argument list too long`, and returned `127`. So the limit is the operating system's argument length rather than anything in Blender.

*The step does not interact with a `-f` list.* Every item in a list is announced as `Rendering single frame`, so a list is not an animation run at all.

The two still open are whether `-f` and `-a` may be given in the same command — only `-a` twice was tested, in hand test 24, and that worked — and the Python-inside-a-session route.

### Observed on the rented Linux machine — Linux hand tests 7 to 10, 19, 23, 24, 25, 26 and 34 to 38, 2026-08-06

Everything this question turns on was run again on **a rented Linux machine — Ubuntu 22.04.5 LTS, NVIDIA GeForce RTX 4090, driver 580.105.08, twelve cores, run over SSH** — in all four versions, and it agrees with the Creator's Mac throughout.

- `-f 5,1,9` rendered 5, then 1, then 9, in the order written.
- `-f 1..3,10..12` was accepted with no error and wrote all six files. **The two forms can be mixed in one argument**, and the label this document could not settle from the manual is settled by the machine on both operating systems now.
- `-f 2,2,3` rendered frame 2 twice over the same file.
- The three malformed lists were refused with the same three messages, word for word, nothing rendered, exit code 0 every time.
- The same list with a window open wrote the same six files.
- `-s 1 -e 2 -a -s 20 -e 21 -a` wrote four files, two separate stretches.
- The keyframes-only playblast with no window was refused with the same message; **with** a window it returned `{'FINISHED'}` and wrote six files for a six-frame range with keys on only two frames, every file the same size. Compared byte for byte, no two of the six are the same file — where the Creator, looking at them, reported them identical. Both records stand.
- A list of five thousand frames was accepted; two hundred thousand never reached Blender, the shell refusing it first with `Argument list too long` and code `126`, where the Mac said `argument list too long` and `127`. **The limit is the operating system's, and the two operating systems refuse it differently.**
- The same list under Cycles gave the same order as under Workbench, on the processor and on the NVIDIA card alike.

Four things were run that neither machine had tried before. They are **Linux only** — the Mac was never asked — and they were the same in all four versions:

- **`-f` may be given twice in one command.** `-f 1..3 -f 10..12` rendered both stretches, six files, exit code 0.
- **`-f` and `-a` may be mixed in one command.** `-s 1 -e 2 -a -f 20` carried out both, three files.
- **Step has no effect on a `-f` list.** With the scene's step set to 3, `-f 1,2,3,4,5` wrote all five files.
- **Whitespace inside a list is accepted.** `-f "1, 2, 3"` rendered frames 1, 2 and 3, exit code 0, even though the manual says "no spaces". That says what this machine took, not that the manual is wrong about what is supported.

The full record is in `Audits/Blender Research/Frames/Hand tests — Linux.md`.

### Outside the interface

The only way the sources describe of naming a set of frames that is not one plain run is a **command line** argument — `-f` with a comma list. That is a facility of Blender started and controlled from outside its own window; the manual presents it in the "Rendering From The Command Line" workflow, and shows it with `-b` (background). So the answer here is stronger outside the interface than inside it: outside, you can hand Blender a list of frames; inside, the panel gives you one stretch and a step. I found no statement that `-f` behaves differently in background mode versus with a window open. Whether the comma list is honoured identically when Blender is launched with a window is not stated. **Observed**, from hand test 20 above: it is. The same list, run without `-b` so that Blender 5.1.2 opened its window, wrote the same six files with the same names.

**Observed on the two-card machine — 2026-08-06.** Run over SSH on a **second rented Linux machine — Ubuntu 22.04.5 LTS, two NVIDIA GeForce RTX 4090s of 24 GB each on driver 580.95.05, an AMD EPYC 7B13 with sixty-one cores, no display server at all** — using `/opt/blender/blender-5.1.2-linux-x64/blender`, `Blender 5.1.2 (hash ec6e62d40fa9 built 2026-05-19 02:18:37)`, with `BLENDER_USER_RESOURCES` pointed at a folder of its own. **Nothing else was running on the machine.** Label: **observed**. A third machine; not merged with the Mac's or the one-card rented machine's results.

**Two graphics cards change neither the order nor the picture.** A four-frame Cycles animation at 640 by 360, 128 samples, seed 0, animated seed off, rendered with `--render-anim` with both cards ticked and then with one:

```
00:04.805  render           | Saved: '/root/smoozy/two cards/work/ord_5.1/f_0001.png'
00:05.981  render           | Saved: '/root/smoozy/two cards/work/ord_5.1/f_0002.png'
00:07.213  render           | Saved: '/root/smoozy/two cards/work/ord_5.1/f_0003.png'
00:08.324  render           | Saved: '/root/smoozy/two cards/work/ord_5.1/f_0004.png'
```

Ascending, one at a time, both ways. The files are not byte-identical — but neither are two runs on the same cards, Blender having written the time into each PNG at the settings these runs used. Comparing the pixels instead, of 921,600 values: **17 differed between two cards and one, 10 between two runs on two cards, and 8 between two runs on one card**, with a largest difference of `0.003921568393707275` — one step in 255 — in every case. So the two-card comparison differed in about twice as many values as either control, every difference being the same single step in 255. Nothing here shows the number of cards changing the picture, and nothing here rules it out either.


*(Hand tests on the four-card machine, 2026-08-06: the same test was run again with **four** graphics cards, on a fourth machine — a third rented Linux machine, four NVIDIA GeForce RTX 4060 Ti of 16 GB each, driver 580.95.05, sixty-one cores, 197 GB of memory, no display server, over SSH. The order was ascending both ways there too; of 921,600 values, 17 differed between four cards and one card and 4 between two runs on four cards, the largest difference in both being one step in 255. That run also established why the files in this test do not match: at the stamp settings these runs used Blender writes the date and the render time into every PNG, and Cycles writes three timing chunks of its own that no stamp setting removes. The full record is in this version's `Frames.md` under question 10, and the list is `Audits/Blender Research/Frames/Hand tests — four cards.md`. Nothing from that machine is merged with anything from this one.)*

*(Four-card citation check, 2026-08-06: the sentence above read "why no two files ever match". Narrowed. Two Blender PNGs of one frame **can** match byte for byte — this check saw it happen with the stamp settings off and again at factory stamp settings, when every time written into the two files happened to round alike. See the note in this version's `Frames.md` under question 10.)*

*(Two-card citation check, 2026-08-06: three corrections. This block sits under question 7 but is a question 10 result — it is about frame order and output, not about naming a set of frames — and the full record is in this version's `Frames.md` under question 10; it is left where it was written rather than moved, and named here for what it answers. The stamp sentence was narrowed to the settings these runs used. And the block gave the three counts with no reading of them, where the reading written elsewhere — that two runs on the same cards differ by as much as a two-card run differs from a one-card run — is not true of 5.1's own numbers.)*

### Operating system

The sources do not say that any of this differs by operating system. The command line page marks arguments that are OS-specific where they are — for example "(Windows only)", "(Linux only)", "(Windows & Linux only)" appear against other arguments on the same page — and no such marking appears against `-f`, `-a`, `-s`, `-e` or `-j`. That the render arguments therefore work on every system is **inferred** from that pattern, not stated. One practical caveat the sources do state, which is a shell matter rather than a Blender one: "Arguments are case sensitive! -F and -f are not the same." (manual — https://docs.blender.org/manual/en/5.1/advanced/command_line/render.html — read 2026-07-30).

### Record of where I looked (question 7)

Whole-tree searches across `manual_5.1` and `api_5.1`, case-insensitive:

- `frame range` — 50+ files; opened the render-relevant ones: `render/output/properties/frame_range.txt`, `render/output/properties/index.txt`, `render/output/introduction.txt`, `render/output/index.txt`, `editors/timeline.txt`, `animation/animation_editors.txt`, `scene_layout/scene/properties.txt`.
- `list of frames` — one hit, the command line arguments page.
- `comma separated` / `comma-separated` — four hits; only the command line arguments page is about frames.
- `specific frames`, `individual frames`, `arbitrary frames`, `non-contiguous`, `selected frames`, `frame list`, `chunks` — no hits relevant to rendering a set of frames.
- `subset of` — the only render hit is the "Animation Preview" advice on `render/output/animation.txt`, which is about the Step setting, not about a list.
- `nth frame`, `N'th frame`, `every other`, `skips frames`, `step forward`, `frame jump` — hits are the Step setting, the command line `-j` argument, and unrelated sequencer/physics settings.
- `render-frame`, `frame-start`, `frame-end`, `frame-jump`, `render-anim` — only `advanced/command_line/arguments.txt` and `advanced/command_line/render.txt`.
- `frame_step`, `frame_start`, `frame_end`, `frame_preview`, `use_preview_range` in the API — `bpy.types.Scene.txt` is the only place these are scene-level render range properties.
- `render_keyed_only`, `Render Keyframes Only` — `bpy.ops.render.txt` and `editors/3dview/viewport_render.txt`.
- `write_still`, `frame_path` in the API — `bpy.ops.render.txt`, `bpy.types.RenderSettings.txt`.
- `batch render`, `render farm`, `network render` — four hits, none offering a frame-set control.

Pages opened in full: `render/output/properties/frame_range.txt`, `render/output/properties/output.txt`, `render/output/introduction.txt`, `render/output/animation.txt`, `render/introduction.txt`, `advanced/command_line/arguments.txt` (render options section), `advanced/command_line/render.txt`, `advanced/command_line/index.txt`, `editors/3dview/viewport_render.txt`, `interface/window_system/topbar.txt` (render menu section), plus the API pages `bpy.ops.render.txt`, `bpy.types.Scene.txt`, `bpy.types.RenderSettings.txt`, `bpy.app.handlers.txt`.

Pictures opened: `_images/render_output_properties_frame_range_panel.png`, `_images/render_output_properties_output_panel.png`.

Ruled out and why: the Preview Range, because the manual says outright it does not affect rendering. The Video Sequencer's strip ranges, because they are strip timing rather than a render frame set. The physics and geometry-node bake ranges, because they bake caches rather than render frames. The animation editors' start/end operators, because they set the same single scene range.

Release notes: searched all of `release_notes_5.1` for `frame`. The hits are the sequencer's property renames, a playhead subframe display change, a compositor node, and the bugfix list. Nothing announces a new way to specify a set of frames to render.

Cycles files: searched all of `cycles_5.1` for `frame`. The only frame-range code is in the denoise operator in `operators.py`, not a render frame-set control. Nothing in `cycles_5.1/source` (271 files) concerns frame sets — its `frame` hits are all coordinate frames, shutter timing and volume velocity.

---

## 10. In what order does a render work through the frames it was given — a plain run, a run that skips frames at a fixed interval, and a list of separate frames — and what does an interruption leave behind in each case?

**Short answer: for a plain run the sources say start to end. For a stepped run they say it advances forward by the step. For a list of separate frames the sources do not say what order is used. What an interruption leaves behind is the frames already written, and the sources say so for the image-sequence case.**

### Order — a plain run

LABEL: **documented**

> "-a, --render-anim
>
> Render frames from start to end (inclusive)."

Source: manual — https://docs.blender.org/manual/en/5.1/advanced/command_line/arguments.html — read 2026-07-30.

"from start to end" names which frames are rendered and the direction the range is described in, and "(inclusive)" settles that both ends are rendered. Reading it as the order the render works through them is one small step beyond the words, so that half is **inferred** rather than documented. It is still the nearest thing to an ordering statement in the whole of the sources. *(Citation check, 2026-08-05: this read "'from start to end' is the order, stated outright… the clearest ordering statement in the whole of the sources", under a documented label. Split, and the order moved to inferred.)*

The scene properties agree in their wording — "First frame of the playback/rendering range" and "Final frame of the playback/rendering range" (API documentation — https://docs.blender.org/api/5.1/bpy.types.Scene.html — read 2026-07-30).

### Order — a run that skips frames at a fixed interval

LABEL: **documented**

> "Step
>
> Controls the number of frames to advance for each frame rendered, a value above 1 skips frames.
> This is also used when viewing rendered animations."

Source: manual — https://docs.blender.org/manual/en/5.1/render/output/properties/frame_range.html — read 2026-07-30.

> "frame_step
>
> Number of frames to skip forward while rendering/playing back each frame (in [0, 1048574], default 1)"

Source: API documentation — https://docs.blender.org/api/5.1/bpy.types.Scene.html — read 2026-07-30.

And the command line equivalent:

> "-j, --frame-jump <frames>
>
> Set number of frames to step forward after each rendered frame."

Source: manual — https://docs.blender.org/manual/en/5.1/advanced/command_line/arguments.html — read 2026-07-30.

Three separate statements all say the same thing about direction: "advance", "skip forward", "step forward after each rendered frame". Taken with "Render frames from start to end", the order is the start frame first, then start plus step, and so on forward. That last joining-up is one small step beyond the quotes, so: the direction and the size of each move are **documented**; the claim that the sequence begins at the start frame and moves forward through the range is **inferred**, the step being that a run that goes "from start to end" and "advances" by the step after each frame must produce start, start+step, start+2×step, and so on.

What the sources do not say is what happens at the far end — whether the last frame rendered is the last one at or below the end frame, or whether the end frame is always rendered even if the stepping would jump past it.

**Observed — hand test 12, 2026-08-05.** The inference holds, and the far end is settled: the run stops at the last frame at or below the end, and never adds the end frame on. Blender 5.1.2 was started from a terminal with no window, rendering its own startup scene in Workbench at 64 by 64 pixels to PNG into an empty folder.

With the step given on the command line as `-s 1 -e 10 -j 3 -a`, it printed:

```
00:00.312  render           | Rendering animation (frames 1..10)
00:00.447  render           | Saved: '…/f_0001.png'
00:00.453  render           | Saved: '…/f_0004.png'
00:00.457  render           | Saved: '…/f_0007.png'
00:00.462  render           | Saved: '…/f_0010.png'
```

With the step given as the scene's own setting instead — start 3, end 12, step 4, where the stepping cannot land on the end frame — it printed:

```
00:00.318  render           | Rendering animation (frames 3..12)
00:00.458  render           | Saved: '…/f_0003.png'
00:00.463  render           | Saved: '…/f_0007.png'
00:00.468  render           | Saved: '…/f_0011.png'
```

Frame 12 was not rendered. The two routes to a step — the command-line `-j` and the scene setting — gave the same frames. Run on the Creator's MacBook Pro, M1 Pro chip, macOS 26.5.2.

### Order — a list of separate frames

LABEL: **not in the sources**

The only way to give a list is the command line `-f` argument with a comma separated list (quoted in full under question 7). Nothing in the manual, the API documentation, the release notes or Cycles' files says whether Blender renders those frames in the order written, in ascending numerical order, or in any other order. Nothing says what happens if the same frame appears twice, or if the list is written out of order.

**Observed — hand tests 8 and 10, 2026-08-05.** A list is rendered in the order written, and a repeat is rendered twice. Blender 5.1.2 was started from a terminal with no window, rendering its own startup scene in Workbench at 64 by 64 pixels to PNG into an empty folder. With `-f 5,1,9` it printed:

```
00:00.315  render           | Rendering single frame
00:00.315  render           | Rendering frame 5
00:00.448  render           | Saved: '…/f_0005.png'
00:00.448  render           | Rendering single frame
00:00.448  render           | Rendering frame 1
00:00.454  render           | Saved: '…/f_0001.png'
00:00.454  render           | Rendering single frame
00:00.454  render           | Rendering frame 9
00:00.459  render           | Saved: '…/f_0009.png'
```

Five, then one, then nine — the order written, not sorted. With `-f 2,2,3` it printed:

```
00:00.318  render           | Rendering frame 2
00:00.460  render           | Saved: '…/h_0002.png'
00:00.460  render           | Rendering frame 2
00:00.465  render           | Saved: '…/h_0002.png'
00:00.465  render           | Rendering frame 3
00:00.471  render           | Saved: '…/h_0003.png'
```

Frame 2 was rendered twice, over the same file, and two files were left. Duplicates are neither rejected nor collapsed. Run on the Creator's MacBook Pro, M1 Pro chip, macOS 26.5.2.

Record of where I looked for this, beyond the searches listed under question 7:

- Searched the whole manual for `render order`, `rendering order`, `order of render`, `order in which`, `in the order`, `order given`, `as given`. The only hits about ordering are `Arguments are executed in the order they are given!` on the command line render page — which is about the arguments themselves, not about the frames inside one argument — and unrelated hits about modifier stacks, Freestyle strokes, rotation orders, node blending and strip selection.
- Searched for `sequential`, `start to end`, `first to last`, `one after another`, `one by one`, `ascending`, `descending`, `backward`, `in reverse`. The only render hit is the `-a` line quoted above. The rest are Grease Pencil build modifiers, Freestyle, inverse kinematics, curve primitives, and one line about an Action constraint playing in reverse — none about the order of a render's frames.
- Read the whole of `advanced/command_line/arguments.txt` render options section and the whole of `advanced/command_line/render.txt`. The `-f` entry is four short lines and says nothing about order.
- Read the whole of `render/output/animation.txt`, `render/output/introduction.txt`, `render/output/properties/frame_range.txt`, `render/output/properties/output.txt` and `render/introduction.txt`. None mention frame order for a list.
- Searched the API documentation for `frames to render` and `frame range`; the render-relevant hits are `bpy.ops.render.txt`, `bpy.types.Scene.txt` and `bpy.types.RenderSettings.txt`, all read, none stating an order.
- Searched all of `release_notes_5.1` for `frame`. No note concerns the order frames are rendered in.
- Searched all of `cycles_5.1` for `frame`, including the 271 files under `source/`. There is no frame loop in Cycles at all (see the observed finding below), so Cycles' files cannot settle this.

### What Cycles' own files show about frame-by-frame working

LABEL: **observed**

Cycles' own files hold a per-frame render call and a per-frame finish call, and no loop that walks the frames. *(Citation check, 2026-08-05: this read "Cycles is handed one frame at a time; it does not contain the loop that walks the frames." The first clause is about what happens when a render runs, which a file cannot show — it belongs with the inference set out three paragraphs below, not with the observed label. Narrowed to what the lines say.)*

Exact lines, from `Blender Sources/cycles_5.1/__init__.py`, inside `class CyclesRender(bpy.types.RenderEngine)` (the block beginning at the comment `# final render`):

```
    def render(self, depsgraph):
        engine.render(self, depsgraph)

    def render_frame_finish(self):
        engine.render_frame_finish(self)
```

The only loop over frames anywhere in Cycles' own Python files is in the denoise operator, `Blender Sources/cycles_5.1/operators.py`, inside `class CYCLES_OT_denoise_animation`, in its `execute` method:

```
            for frame in range(scene.frame_start, scene.frame_end + 1):
```

Date read: 2026-07-30.

What is observed: those lines exist, with those names, in those files. Cycles exposes a per-frame entry point and a per-frame finish, and its own only frame loop — which belongs to denoising, not to rendering — runs ascending from the start frame to the end frame inclusive.

What is **inferred** from that, and the step taken: because Cycles offers only a single-frame render call and a single-frame finish call, and contains no code that decides which frame comes next during a render, the choice of frames and their order is made by Blender outside Cycles. Therefore the order a render works through its frames is the same whichever engine is used. That last sentence is inference and the sources do not state it.

**Observed — hand test 25, 2026-08-05.** The inference holds against all three engines. Blender 5.1.2 was started from a terminal with no window and given `-f 5,1,9`, once with the engine set to Workbench, once to Cycles at 1 sample, and once to EEVEE. All three produced the same order — 5, then 1, then 9:

```
00:01.052  render           | Saved: '…/e_0005.png'
00:01.379  render           | Saved: '…/e_0001.png'
00:01.707  render           | Saved: '…/e_0009.png'
```

(That is the EEVEE run; the Workbench and Cycles runs printed the same three frame numbers in the same order.) One thing the test printed in passing: in Blender 5.1.2 the engine is named `BLENDER_EEVEE`, while in Blender 4.5.12 the same test had to set it as `BLENDER_EEVEE_NEXT`. Run on the Creator's MacBook Pro, M1 Pro chip, macOS 26.5.2.

### What an interruption leaves behind — a plain run

LABEL: **documented**

> "Images can also be used for rendering animations which has a couple advantages.
> For example, when rendering animations to image file formats the render job can be canceled
> and resumed at the last rendered frame by changing the frame range."

Source: manual — https://docs.blender.org/manual/en/5.1/render/output/introduction.html — read 2026-07-30.

> "This allows you an easy recovery if there is a problem and you have to re-start the rendering,
> since the frames you have already rendered will still be in the output directory."

> "Just disable the Overwrite option to start rendering where you left off."

Source: manual — https://docs.blender.org/manual/en/5.1/render/output/animation.html — read 2026-07-30. (Both from the "Hints" section, which opens "Your computer accidentally turns off in the middle of rendering your movie!")

The same page lists, among the reasons to choose the frame-sequence approach:

> "May need to interrupt rendering to use the computer, and want to be able to resume rendering where you left off."

Source: same page, same date.

So: an interrupted plain run into an image sequence leaves the frames already rendered sitting in the output directory, and the phrase "resumed at the last rendered frame" tells you they are the frames from the start up to wherever it stopped.

### What an interruption leaves behind — a stepped run

LABEL: **documented**

The manual describes exactly this state of affairs and how to carry on from it:

> "Animation Preview
>
> It can be useful to render a subset of the animated sequence,
> since only part of an animation may have an error.
>
> Using an image format for output,
> you can use the Frame Step option to render every N'th frame.
> Then disable Overwrite and re-render with Frame Step set to 1."

Source: manual — https://docs.blender.org/manual/en/5.1/render/output/animation.html — read 2026-07-30.

The instruction to re-render afterwards with Step 1 and Overwrite disabled only makes sense if what a stepped run leaves on disk is the stepped frames alone, with the skipped ones absent. Calling that a statement about an *interrupted* stepped run is a step further: the manual is describing a stepped run that finished, not one that was cut short. Label for the interrupted case: **inferred** — the step being that a stepped run writes each frame as it goes just as a plain run does, so cutting it short leaves the stepped frames up to the point it stopped.

### What an interruption leaves behind — a list of separate frames

LABEL: **not in the sources**

Nothing states what an interrupted `-f 1,3,5` style command leaves behind. The searches recorded above cover it. What can be said is only what the general statements above say: frames written to an image sequence stay on disk. Applying that to a list is inference, and I am flagging it rather than dressing it up: the sources do not treat the list case at all.

### The empty file an interruption can leave

LABEL: **documented**

Blender can be told to create an empty file for a frame as it works:

> "Image Sequence – Overwrite Image Multi-Layer EXR
>
> Create empty placeholder frames while rendering."

Source: manual — https://docs.blender.org/manual/en/5.1/render/output/properties/output.html — read 2026-07-30. (The heading line is repeated verbatim in the source for both the Overwrite and the Placeholders entries; the preceding entry reads "Overwrite existing files when rendering.")

The API states it more precisely:

> "use_placeholder
>
> Create empty placeholder files while rendering frames (similar to Unix 'touch') (default False)"

and beside it:

> "use_overwrite
>
> Overwrite existing files while rendering (default True)"

Source: API documentation — https://docs.blender.org/api/5.1/bpy.types.RenderSettings.html — read 2026-07-30.

I opened the Output panel picture, `Blender Sources/manual_5.1/_images/render_output_properties_output_panel.png`, referenced from the manual page above and captioned "Output panel." Visible in it: a file path field reading `/tmp/`, and a row of checkboxes labelled "Overwrite" (ticked) and "Placeholders" (unticked), with "File Extensions" (ticked) and "Cache Result" (unticked) below them. This confirms the two settings sit side by side and confirms their defaults as the API states them. Label: **documented**.

That a placeholder file is therefore what an interruption leaves behind for the frame that was in progress is **inferred** — the step being that "while rendering frames" means the empty file exists before the frame's real content is written, so a render cut off mid-frame would leave that empty file. The sources do not say when the placeholder is created relative to the frame's work, nor whether it is removed if the render is cancelled.

**Observed — hand tests 3, 6 and 7, 2026-08-05.** The first half of that inference holds and the second half does not. Blender 5.1.2 was started from a terminal with no window, rendering the default cube scene in Cycles to PNG with Placeholders on and Overwrite off, and a script listed the output folder at each of Blender's own render events:

```
WATCH [before the render starts] folder holds: (nothing)
WATCH [render_pre, frame 1 about to render] folder holds: f_0001.png=0 bytes
WATCH [render_post, frame 1 finished rendering] folder holds: f_0001.png=34818 bytes
WATCH [render_pre, frame 2 about to render] folder holds: f_0001.png=34818 bytes, f_0002.png=0 bytes
```

So the placeholder is made immediately before the frame's own work begins, one at a time — that half is right.

But an interruption leaves no placeholder behind. The same version, rendering frames 1 to 5 with each frame taking about five seconds, was interrupted fifteen seconds in, during frame 3. The folder afterwards held only:

```
f_0001.png  92486 bytes
f_0002.png  92486 bytes
```

No file of any kind for frame 3 — not a placeholder, not a part-written file, nothing. The same was true when the process was terminated politely and when it was killed outright. Run on the Creator's MacBook Pro, M1 Pro chip, macOS 26.5.2.

**Observed — hand test 29, run by the Creator, 2026-08-05.** The same is true when the stop comes from inside Blender's own window. A prepared file was opened in Blender 5.1.2 — Cycles, 4000 samples, 400 by 400 pixels, PNG output into an empty folder, frames 1 to 10, about twenty-five seconds a frame. Render Animation was started, frames 1 and 2 were allowed to finish, and Esc was pressed while frame 3 was under way. The folder afterwards held exactly:

```
92388 bytes  frame_0001.png
92388 bytes  frame_0002.png
```

The Creator's own words: "You can probably look at the files yourself. Stopping isn't instant, there's a slight lag. All versions." Run on the Creator's Mac — a MacBook Pro, M1 Pro chip, macOS 26.5.2 — so this settles macOS and leaves Windows and Linux open. That test had Placeholders off, so what a cancel does to a placeholder specifically is covered by the terminal runs above rather than by this one.

The manual's own use of the setting points the same way:

> "Primitive Render Farm
>
> An easy way to get multiple machines to share the rendering workload is to:
>
> Set up a shared directory over a network file system.
>
> Disable Overwrite, enable Placeholders in the Render Output panel.
>
> Start as many machines as you wish rendering to that directory."

Source: manual — https://docs.blender.org/manual/en/5.1/render/output/properties/output.html — read 2026-07-30.

### Cancelling is a distinct event Blender reports

LABEL: **documented**

> "bpy.app.handlers.render_cancel
>
> on canceling a render job. Accepts one argument: the scene data-block being rendered"

> "bpy.app.handlers.render_complete
>
> on completion of render job. Accepts one argument: the scene data-block being rendered"

> "bpy.app.handlers.render_write
>
> on writing a render frame (directly after the frame is written). Accepts one argument: the scene data-block being rendered"

Source: API documentation — https://docs.blender.org/api/5.1/bpy.app.handlers.html — read 2026-07-30.

Cancelling and completing are separate, named events, and there is a separate event that fires "directly after the frame is written" — so frames are written one at a time as the render proceeds, not all at the end. That last clause is **inferred**, the step being that an event described as firing directly after a frame is written must fire once per frame during the run.

### How a render is stopped

LABEL: **documented**

> "As with a normal render, you can abort it with Esc."

Source: manual — https://docs.blender.org/manual/en/5.1/editors/3dview/viewport_render.html — read 2026-07-30. (The page is about viewport render; the phrase "as with a normal render" is the only place the sources state how a normal render is aborted.)

> "Running Task
>
> Shows the progress of the currently running task (such as rendering or baking).
> Hovering the mouse pointer over the progress bar will display a time estimate.
> The task can be aborted by clicking the cancel button ()."

Source: manual — https://docs.blender.org/manual/en/5.1/interface/window_system/status_bar.html — read 2026-07-30.

### Not covered

- The order used for a list of separate frames. Completely unaddressed.
- Whether a stepped run always renders the end frame, or stops at the last frame the stepping lands on at or below it.
- Whether the frame that was in progress when the render stopped leaves a partial file, an empty file, or no file at all. The placeholder setting is the closest the sources come, and it does not settle it.
- What an interruption leaves behind when the output is a video file rather than an image sequence. The sources' advice on this ("highly not recommended and not a standard practice" for the direct-to-movie approach) sits on the same animation page, but the question of what a half-written video file contains is question 5's ground and I have not answered it here.
- Whether a render that is cancelled finishes the frame it is on first. That is question 11's ground.
- Whether the order differs between engines. Cycles' files show it does not choose the order, but the sources never state the general rule.

### Observed on the rented Linux machine — Linux hand tests 1, 7, 9, 11, 5, 24, 30 and 33, 2026-08-06

Every part of this was run again on **a rented Linux machine — Ubuntu 22.04.5 LTS, NVIDIA GeForce RTX 4090, driver 580.105.08, twelve cores, run over SSH** — in all four versions, and it agrees with the Creator's Mac.

- A plain run gave 1, 2, 3, 4, 5 in that order.
- A stepped run started on the start frame and moved forward by the step, and did not reach back for an end frame it had overshot.
- A list of separate frames rendered in the order written, and a repeat was rendered twice.
- An interruption left whole frames only, and nothing for the frame in flight, whichever way the render was stopped.

Two things this document could only infer are now tested.

**The choice of frames does not depend on the engine or on the card.** This document reasoned from Cycles' own files that the choice of frames is made outside the engine. On the rented Linux machine the same list gave the same order under Workbench and under Cycles in all four versions, and — pointing Cycles at the NVIDIA GeForce RTX 4090 through OPTIX with the processor switched off — the same order again. An interruption on the card left nothing for the frame in flight, printed the same `Sent an internal break event` line and exited with code `1`.

**A render started the way the window starts it uses the same order.** With a window open on that machine's display, `bpy.ops.render.render('INVOKE_DEFAULT', animation=True)` — the call the Render menu's Render Animation item makes — returned `{'RUNNING_MODAL'}`, and the six files it wrote, sorted by the time each was written, came out `['o_0001.png', 'o_0002.png', 'o_0003.png', 'o_0004.png', 'o_0005.png', 'o_0006.png']`. Ascending, one at a time, the same as a render driven from a terminal. Identical in all four versions.

**Windows remains untested**, because no Windows machine has run it. **Untried rather than untestable: a Windows machine can be rented, and every one this project has read reported a Server edition — see the equipment section of `Preparation/Blender Research/Method.md`.**

### Outside the interface

The plainest ordering statement in the sources — "Render frames from start to end (inclusive)" — is the description of a **command line** argument, so it is a statement about Blender driven from outside its own window. Nothing states that the order differs when the same render is started from the interface. The two ways of stopping a render that the sources describe are both interface actions (the Esc key, the status bar cancel button); the sources describe no equivalent for a render started from the command line, so how such a render is stopped, and what it reports when it is, is not covered. One release note bears on what is visible from outside during a run: the 5.1 bugfix list contains

> " * Show currently rendered frame in background mode [[7122b6cc561](https://projects.blender.org/blender/blender/commit/7122b6cc56149ff9b97c171c3a5005535156884c)] - Backported to 5.0.1"

Source: release notes — https://developer.blender.org/docs/release_notes/5.1/bugfixes/ — read 2026-07-30. That is a documented statement that background mode reporting of the current frame was fixed in this line of releases; it is not a statement about the order itself.

### Operating system

The sources do not say that the order, or what an interruption leaves behind, differs by operating system. No OS marking appears against any of the quoted material. The only OS-specific note anywhere near this ground is unrelated to frame order: the command line page marks `--no-window-frame` as Linux only and several registration arguments as Windows and Linux only. The placeholder property's description compares itself to a Unix command — "similar to Unix 'touch'" — but that is a description of what the file is, not a claim that it works only there.

---

## Things a person with Blender 5.1 open could still settle by hand

All ten items that stood here have been run — the results are written into the answers above as observed findings, dated 2026-08-05. The last of them, confirming the Frame Range panel offers nothing more, was answered by the Creator on that date: the panel holds **Frame Start** `1`, **End** `250`, **Step** `0` (after they typed a 0 into it), and an open **Time Stretching** section with **Old** `100` and **New** `100`, and nothing else; and in their own words, "Right-clicking the panel header does nothing." That is recorded in full under question 8 of `Preparation/Blender Research/Findings/5.1/Frames.md`.

The equipment limits that stood here have shrunk. Every one of these tests was run again on a rented Linux machine — Ubuntu 22.04.5 LTS, NVIDIA GeForce RTX 4090, driver 580.105.08, twelve cores, run over SSH — on 2026-08-06, and agreed with the Mac; the frame order was also checked on that machine's NVIDIA card. **Windows stays open**, because no Windows machine has run it, and no card of a third make can be reached.

---

2026-08-04 — Opus 5 (Claude Code) — Frames, Blender 5.1

2026-08-05 — Opus 5 (Claude Code) — hand-test results written into questions 7 and 10, and the hand-test list above pruned

2026-08-05 — Opus 5 (Claude Code) — the Creator's own hand-test results written into questions 7 and 10, and the hand-test list above closed

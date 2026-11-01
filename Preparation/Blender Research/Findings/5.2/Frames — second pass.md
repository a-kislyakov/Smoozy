# Frames — Blender 5.2 — second pass (question 7 only)

**Sources consulted, and when.** Blender 5.2 manual (`Blender Sources/manual_5.2`, including its `_images` folder), Blender 5.2 Python API documentation (`Blender Sources/api_5.2`), Blender 5.2 release notes (`Blender Sources/release_notes_5.2`), and Cycles' own files (`Blender Sources/cycles_5.2`). All read on 2026-07-30 (the snapshot date given to me). Nothing else was consulted — no web, no other version's folder, no memory.

I read only the question sheet and files inside `Blender Sources/`. I did not open any other part of this project.

I worked alone. I started no helper agents, so there is nobody else's reading to vouch for.

---

## 7. Can Blender be given a set of frames that is not one plain run — single frames, or two separate stretches?

**Short answer: yes, but only through one channel — the command line. Everywhere else in the sources, a render is given a start, an end and a step, which is one plain run.**

### Note on the question

The question does not say *how* Blender is being given the frames. I answer it for every channel the sources describe: the command line, the settings stored in the file (and their Python equivalents), the operator a script would call, and the menu commands. If the question was meant only about the settings panel, the answer is the "B" part below and it is no.

---

### A. The command line accepts a list of frames, and accepts a stretch written as a range

**Label: documented** (for a list of single frames)

Exact words:

> `-f, --render-frame <frame>`
>
> Render frame `<frame>` and save it.
>
> `+<frame>` start frame relative, `-<frame>` end frame relative.
>
> A comma separated list of frames can also be used (no spaces).
>
> A range of frames can be expressed using .. separator between the first and last frames (inclusive).

Source: the manual, https://docs.blender.org/manual/en/5.2/advanced/command_line/arguments.html
Date read: 2026-07-30

So "single frames" is answered outright: a comma separated list, no spaces. And a stretch can be written with `..` instead of being spelled out frame by frame.

**Two separate stretches — label: inferred.** The source says a comma separated list of frames may be used, and says separately that a range may be expressed with `..`. It does not, in as many words, say the two forms may be mixed in one `-f` argument (something like `-f 1..10,50..60`). Getting "two separate stretches" out of the quoted words requires the reader to supply the step that a `..` range counts as an item that may sit in the comma separated list. That step is mine, not the manual's, so this is inferred and not documented. It is worth testing by hand — see the list at the end.

**Observed — hand test 9, 2026-08-05.** It was tested, and the step holds. Blender 5.2.0 was started from a terminal with no window, rendering its own startup scene (the default cube) in Workbench at 64 by 64 pixels, PNG output into an empty folder, with the argument `-f 1..3,10..12`. It printed:

```
00:00.456  render           | Rendering single frame
00:00.456  render           | Rendering frame 1
00:00.617  render           | Saved: '…/g_0001.png'
00:00.617  render           | Rendering single frame
00:00.617  render           | Rendering frame 2
00:00.622  render           | Saved: '…/g_0002.png'
00:00.622  render           | Rendering single frame
00:00.622  render           | Rendering frame 3
00:00.628  render           | Saved: '…/g_0003.png'
00:00.628  render           | Rendering single frame
00:00.628  render           | Rendering frame 10
00:00.632  render           | Saved: '…/g_0010.png'
00:00.632  render           | Rendering single frame
00:00.632  render           | Rendering frame 11
00:00.637  render           | Saved: '…/g_0011.png'
00:00.637  render           | Rendering single frame
00:00.638  render           | Rendering frame 12
00:00.642  render           | Saved: '…/g_0012.png'
```

All six files were written and no error was printed. Note also what Blender calls each item — `Rendering single frame`, never `Rendering animation`. Run on the Creator's MacBook Pro, M1 Pro chip, macOS 26.5.2.

The same page fixes the order the argument must come in, which matters when the list is long and the output path is set on the same line:

> Arguments are executed in the order they are given. eg:
>
> `blender --background test.blend --render-frame 1 --render-output "/tmp"`
>
> …will not render to /tmp because --render-frame 1 renders before the output path is set.

Source: the manual, https://docs.blender.org/manual/en/5.2/advanced/command_line/arguments.html
Date read: 2026-07-30

And the render page repeats the point plainly:

> Always position -f or -a as the last arguments.

Source: the manual, https://docs.blender.org/manual/en/5.2/advanced/command_line/render.html
Date read: 2026-07-30

The only worked examples of `-f` on that page are single frames — `-f 10` ("Render only the 10th frame") and `-f -2` ("Render only the second last frame"). No example on either page shows a list or a `..` range in use.

**Not covered by A.** Whether a `..` range may sit inside a comma separated list. Whether the list may repeat a frame, or be out of order, and what that does. Whether the frame step (`-j`, `--frame-jump`) applies to a list given with `-f`, or is ignored — the page describes `-j` as "Set number of frames to step forward after each rendered frame" and says nothing about `-f`. How long a list may be. (The same page mentions a platform argument-length limit, but says it about `--python-expr`, not about `-f`, so I cannot carry it across.) What Blender prints or returns if one frame in the list is invalid.

**Observed — hand tests 8, 10 and 11, 2026-08-05.** Four of those five are settled. All were run with Blender 5.2.0 started from a terminal with no window, rendering the default cube scene in Workbench at 64 by 64 pixels to PNG into an empty folder.

*A `..` range may sit inside a comma separated list* — see hand test 9 above.

*A list may be out of order, and Blender honours the order written.* `-f 5,1,9` printed:

```
00:00.401  render           | Rendering single frame
00:00.401  render           | Rendering frame 5
00:00.555  render           | Saved: '…/f_0005.png'
00:00.555  render           | Rendering single frame
00:00.555  render           | Rendering frame 1
00:00.560  render           | Saved: '…/f_0001.png'
00:00.560  render           | Rendering single frame
00:00.560  render           | Rendering frame 9
00:00.566  render           | Saved: '…/f_0009.png'
```

*A list may repeat a frame, and the frame is rendered twice.* `-f 2,2,3` printed `Rendering frame 2` twice and `Saved:` the same file twice, leaving two files in the folder.

*The frame step does not apply to a `-f` list.* Every item is announced as `Rendering single frame`, so a list is not an animation run at all and there is nothing for the step to step through.

*What Blender prints and returns for a bad list.* One message, no file, and an exit code of `0` every time:

```
Error: incorrect comma use '-f / --render-frame 1,,3'.
Warning: negative range ignored '-f / --render-frame 3..1'.
Error: not a number '-f / --render-frame 1-3'.
```

**Observed — hand test 27, 2026-08-05.** The fifth is settled too, and the limit is not Blender's. A list of five thousand frames was accepted: Blender 5.2.0 rendered all five thousand and exited with code `0`. A list of two hundred thousand never reached Blender at all — the shell refused it first:

```
argument list too long: /Applications/Blender.app/Contents/MacOS/Blender
```

and returned `127`. So the platform argument-length limit this document could not carry across from `--python-expr` is exactly what bounds a `-f` list. Run on the Creator's MacBook Pro, M1 Pro chip, macOS 26.5.2; the figure will differ on another operating system.

---

### B. The settings stored in the file are a start, an end and a step — one plain run

**Label: documented**

The panel that holds the frames for a render offers exactly three things:

> Frame Start, End
>
> Set the Start and End frames for Rendering Animations.
>
> Step
>
> Controls the number of frames to advance for each frame rendered, a value above 1 skips frames.
> This is also used when viewing rendered animations.

Source: the manual, https://docs.blender.org/manual/en/5.2/render/output/properties/frame_range.html
Date read: 2026-07-30

The picture on that same page shows the panel itself. It contains three number fields, labelled `Frame Start`, `End` and `Step`, above a closed sub-panel labelled `Time Stretching`. There is no field for a list, and no way visible to add a second range. (Image: `_images/render_output_properties_frame_range_panel.png`, on the page above. Label for what the picture shows: **documented**. What is visible in it is visible; I am not reading anything further into it.)

The Python side matches, one number each:

> `frame_start` — First frame of the playback/rendering range (in [0, 1048574], default 1)
>
> `frame_end` — Final frame of the playback/rendering range (in [0, 1048574], default 250)
>
> `frame_step` — Number of frames to skip forward while rendering/playing back each frame (in [0, 1048574], default 1)

Source: the Python API documentation, https://docs.blender.org/api/5.2/bpy.types.Scene.html
Date read: 2026-07-30

There is a second, alternative range stored in the file, but it is explicitly not for a final render:

> `use_preview_range` — Use an alternative start/end frame range for animation playback and view renders (default False)

Source: the Python API documentation, https://docs.blender.org/api/5.2/bpy.types.Scene.html
Date read: 2026-07-30

So a step above 1 gives a set that is not a plain unbroken run — every Nth frame — and that is documented. But it is still one stretch with one spacing. Nothing in the settings holds a second stretch or an arbitrary list.

**Not covered by B.** Whether anything outside this panel can stand in for a list of ranges. Whether `frame_step` of 0 is meaningful (the range permits it; the documentation does not say what it does).

**Observed — hand tests 24 and 13, 2026-08-05.** Both are settled, and the first turns out to be a real answer to the question this document is about.

*Something outside the panel can stand in for two stretches.* Two render instructions in one command produce two separate runs. Blender 5.2.0, given `-s 1 -e 2 -a -s 20 -e 21 -a`, printed:

```
00:00.403  render           | Rendering animation (frames 1..2)
00:00.560  render           | Saved: '…/m_0001.png'
00:00.565  render           | Saved: '…/m_0002.png'
00:00.565  render           | Rendering animation (frames 20..21)
00:00.571  render           | Saved: '…/m_0020.png'
00:00.576  render           | Saved: '…/m_0021.png'
```

Four files, two stretches, one command.

*A step of 0.* The property accepted it and read back as `0`. Blender rendered and wrote frame 1 once, then announced frame 1 four more times without rendering or writing anything, and ended on its own with exit code `0`, leaving one file:

```
00:00.560  render           | Saved: '…/f_0001.png'
00:00.560  render           | Rendering frame 1
00:00.560  render           | Rendering frame 1
00:00.560  render           | Rendering frame 1
00:00.560  render           | Rendering frame 1
frame_step actually reads back as: 0
```

It did not hang. Run on the Creator's MacBook Pro, M1 Pro chip, macOS 26.5.2.

---

### C. The operator a script calls takes one start and one end

**Label: documented**

> `bpy.ops.render.render(*, animation=False, write_still=False, use_viewport=False, use_sequencer_scene=False, layer='', scene='', frame_start=0, frame_end=0)`
>
> `frame_start` (int) – Start Frame, Frame to start rendering animation at. If not specified, the scene start frame will be assumed. This should only be specified if doing an animation render (in [-inf, inf], optional)
>
> `frame_end` (int) – End Frame, Frame to end rendering animation at. If not specified, the scene end frame will be assumed. This should only be specified if doing an animation render (in [-inf, inf], optional)

Source: the Python API documentation, https://docs.blender.org/api/5.2/bpy.ops.render.html
Date read: 2026-07-30

*(Citation check, 2026-08-05: three release-note quotations stood here — that the `render.render()` operator gained `frame_start` and `frame_end`, and two entries recording a wrong-frame-range fix in it. All three were cited to the **5.0** and **5.1** release notes, not 5.2's, and this document's own opening declares no other version's folder was consulted. The 5.2 release notes hold nothing on this operator; I searched them. The quotations have been cut. When these two arguments arrived, and what was fixed in them, is not in the 5.2 sources, and a fresh researcher should be asked for the 5.2 answer rather than have another version's supplied here.)*

One call, one stretch. There is no argument on this operator that takes a list. A script could of course call it more than once, or set the current frame itself — `Scene.frame_set(frame, *, subframe=0.0)`, "Set scene frame updating all objects and view layers immediately" (same API page as B) — but the sources nowhere describe that as a way of rendering a frame set, so treating it as one would be **inferred**, and I am flagging it rather than claiming it.

---

### D. There is one built-in non-contiguous frame set, and it is viewport-render only

**Label: documented**

> Render Keyframes
>
> To render an animation, but only those frames that have a keyframe, use
> 3D Viewport ‣ View ‣ Render Playblast on Keyframes. This only renders those frames for which
> the selected objects have an animation key. The other frames are still written to the output, but will simply
> repeat the last-rendered frame.
>
> For example, when a six-frame animation is rendered, and the selected objects have a key on frames 3 and 5, the
> following frames will be output: [the 1st frame is always rendered; the 1st frame is repeated because there is no key on this frame; the 3rd frame is rendered; the 3rd frame is repeated…; the 5th frame is rendered; the 5th frame is repeated…]

Source: the manual, https://docs.blender.org/manual/en/5.2/editors/3dview/viewport_render.html
Date read: 2026-07-30

(The bracketed part is the manual's own six-item list, run together to keep the quote short; every phrase in it is the manual's wording.)

The Python side of the same thing:

> `bpy.ops.render.opengl(*, animation=False, render_keyed_only=False, sequencer=False, write_still=False, view_context=True)`
>
> `render_keyed_only` (bool) – Render Keyframes Only, Render only those frames where selected objects have a key in their animation data. Only used when rendering animation (optional)

Source: the Python API documentation, https://docs.blender.org/api/5.2/bpy.ops.render.html
Date read: 2026-07-30

Two limits on this one, both documented on the manual page above:

> Viewport rendering only works for the Workbench and EEVEE render engines.

and, from the quote already given, a file is still written for every frame of the range — the skipped ones just hold a repeat. So this picks out a non-contiguous set of frames to *render*, but does not produce a non-contiguous set of *files*.

There is no equivalent flag on the full render operator: `bpy.ops.render.render` has no `render_keyed_only` argument (see its full signature in C).

**Not covered by D.** Which frames count as "selected objects" when nothing is selected. Whether this works at all when Blender is driven from outside its window — the operator name says viewport and the manual describes it as a 3D Viewport menu item, and neither says what happens without a viewport.

**Observed — hand test 26, 2026-08-05.** The second of those is settled: it does not work without a window, and Blender says so plainly. Calling the operator with the exact arguments quoted above, in a Blender 5.2.0 started from a terminal with no window, gave:

```
Error: Cannot use OpenGL render in background mode (no opengl context)
RAISED: RuntimeError Error: Cannot use OpenGL render in background mode (no opengl context)
FILES: []
```

No file was written. The same command gave the same message in Blender 4.5.12, 5.0.1 and 5.1.2. Run on the Creator's MacBook Pro, M1 Pro chip, macOS 26.5.2.

**Observed — hand test 32, run by the Creator, 2026-08-05.** With a window it does run, and the manual's own worked example holds: a file for every frame of the range. In Blender 5.2.0 the Creator took the default cube scene, put a keyframe on the cube at frame 3 and another at frame 5 and none anywhere else, set the frame range to 1–6 and an image output path, and used the keyframes-only item in the 3D Viewport's View menu. Their own words: "Anyway, all versions create 6 files, which are identical."

Six files for a six-frame range, with keys on only two of the frames. That is the point section D makes: this picks out a non-contiguous set of frames to *render*, and still writes a full set of *files*. The word "identical" is the Creator's own; they did not say what the six files were named or how big they were, and that is not recorded.

The menu item this section quotes as "Render Playblast on Keyframes" is labelled exactly that in Blender 5.2.0. The View menu reads:

> Render Viewport Preview
> Render Playblast
> Render Playblast on Keyframes

with the tooltip "Take a snapshot of the active viewport." and, underneath it, "Python: bpy.ops.render.opengl()" — which is the operator this section quotes. Blender 4.5.12 labels the same three items "Viewport Render Image", "Viewport Render Animation" and "Viewport Render Keyframes"; 5.0.1 and 5.1.2 match 5.2.0.

Run on the Creator's Mac — a MacBook Pro, M1 Pro chip, macOS 26.5.2 — so this settles macOS and leaves Windows and Linux open. Which frames count as "selected objects" when nothing is selected is still not recorded.

---

### E. What is not in the sources

**Label: not in the sources**

The sources do not describe any way, other than the command line `-f` argument, to hand Blender a set of frames that is not one start-to-end run (optionally with a fixed step). In particular: no setting in the file that holds a list of frames or a second range, no field for one in the Frame Range panel, and no argument for one on the render operator.

**Where I looked, and what I searched for.**

Searches run across the whole of `manual_5.2`, `api_5.2` and `release_notes_5.2` at once (case-insensitive where useful):

- `render-frame`, `render_frame`, `frame-start`, `frame_start`, `frame_end`, `frame_step`, `frame_preview`, `use_preview_range`, `frame_current`
- `comma separated`, `comma-separated` — several hits in the whole 5.2 manual and API. Only one is about frames (the `-f` line quoted in A). The others are the `--log` argument, a Rigify tag field, a geometry-nodes effector filter, the Cryptomatte node's object/material names, and a handful of other geometry-node and extensions-repository fields. None bears on rendering a frame set. *(Citation check, 2026-08-05: this said "five hits"; searching both spellings, case-insensitively, across `manual_5.2` and `api_5.2` returns thirteen lines across ten files. The count has been dropped and the list widened. The substantive point — that only the `-f` line is about frames — holds.)*
- `list of frames`, `frame list`, `frames list`, `set of frames`, `individual frames`, `specific frames`, `selected frames`, `arbitrary frames`, `subset of frames` — no hit describes a renderable frame set other than the `-f` line. (`selected frames` hits are about keyframe selection in `bpy.ops.wm`; `specific frames` is a cloth-simulation sentence.)
- `non-contiguous`, `noncontiguous`, `discontinuous` — every hit is about mesh normals, UVs, face sets or rotation interpolation. Nothing about frames.
- `separator between` — one hit in the whole 5.2 documentation, the `..` line quoted in A.
- `frames to render`, `render only`, `each frame rendered`, `frame numbers`
- `render queue`, `batch render`, `render farm`, `render job` — the only substantive hit is the "Primitive Render Farm" hint on the Output Properties page, which is about sharing work by disabling Overwrite and enabling Placeholders, not about handing out frame sets.
- `keyed_only`, `Keyframes Only`, `Keyed Only`, `playblast` — these are what turned up D.
- `Frame Range`, `preview range`, `Render Animation`, `viewport render`, `marker` together with `render`.

Pages and files opened in full or in part:

- `advanced/command_line/arguments.txt` — read the whole Render Options block, and listed every line in the file that begins with a dash, to see the complete argument list. The only arguments that touch frames are `-a/--render-anim`, `-f/--render-frame`, `-s/--frame-start`, `-e/--frame-end` and `-j/--frame-jump` (plus `-s`, `-e`, `-j`, `-f` again under the animation *player*, which plays rather than renders). There is no `--frame-list` or anything like it. Also read the Argument Parsing and Argument Order sections at the end.
- `advanced/command_line/render.txt` — the whole page.
- `render/output/properties/frame_range.txt` — the whole page, plus its picture.
- `render/output/animation.txt` — the whole page (Rendering Animations). It describes Frame Step as the way to render "every N'th frame" and says nothing about lists or second ranges.
- `render/output/properties/output.txt` — the Overwrite / Placeholders / Primitive Render Farm part.
- `interface/window_system/topbar.txt` — the Render menu. It offers Render Image ("Render the active scene at the current frame"), Render Animation ("Render the animation of the active scene"), and the two Sequencer equivalents. No menu item takes a frame set.
- `editors/3dview/viewport_render.txt` — the whole page.
- `editors/timeline.txt` — the Frame Range section. It says the range is set with "the Start/End inputs in the Timeline header".
- `advanced/scripting/security.txt` — the background-rendering example, in case a frame set was described there. It only shows `--render-anim`.
- `api_5.2/bpy.types.Scene.txt` — every frame-named property.
- `api_5.2/bpy.types.RenderSettings.txt` — every frame-named entry. It holds the output path, frame rate, time remapping, shutter, placeholders, metadata stamps and `frame_path()`. No list of frames.
- `api_5.2/bpy.ops.render.txt` — every operator on it, in full for `render` and `opengl`.
- `api_5.2/bpy.types.RenderEngine.txt` — `frame_set(frame, subframe)`, which is documented as "Evaluate scene at a different frame (for motion blur)", not as a way of choosing what to render.
- `release_notes_5.2/rendering.md` in full, and searched all of `release_notes_5.2` for `frame_start`, `frame_end`, `render_frame`, `frame range`.
- Cycles' own files: searched every Python file in `cycles_5.2` for `frame`. The only hits are `render_frame_finish` (a per-frame hook in `__init__.py` line 76 and `engine.py` line 95) and the standalone denoise operator in `operators.py`, which walks `range(scene.frame_start, scene.frame_end + 1)` (line 79) — one plain run, and it is denoising, not rendering. Nothing in Cycles' files defines or accepts a frame set; frames are the host application's business, not the engine's.

I ruled out the compositing, modelling, sculpting, physics and video-editing parts of the manual only after the whole-documentation searches above returned nothing from them on this subject.

---

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

The one channel that does give a non-plain frame set — `-f` with a list — is a command line argument, so it exists only outside the interface. It has no counterpart inside Blender's own window in these sources: the Frame Range panel has three fields (B), and the Render menu has no frame-set command (E). Going the other way, the mechanism in D is described as a 3D Viewport menu item and an operator named `opengl`, and the sources do not say whether it works with no window.

The sources are explicit that command-line rendering is meant for exactly this:

> One advantage of using the command line is that we do not need a graphical display
> (no need for X server on Linux for example)
> and consequently we can render via a remote shell (typically SSH).

Source: the manual, https://docs.blender.org/manual/en/5.2/advanced/command_line/render.html
Date read: 2026-07-30

What the sources do not say: whether `-f` with a list behaves any differently with a window open (Blender can be given `-f` without `-b`; the page's examples always pair them). That is untested here.

**Observed — hand test 20, 2026-08-05.** It is no longer untested, and it does not differ. The same `-f 1..3,10..12` command was run without `-b`, so Blender 5.2.0 opened its window. It printed the same lines and wrote the same six files with the same names:

```
00:00.848  render           | Rendering frame 1
00:00.945  render           | Saved: '…/w_0001.png'
00:00.946  render           | Rendering frame 2
00:00.952  render           | Saved: '…/w_0002.png'
00:00.952  render           | Rendering frame 3
00:00.957  render           | Saved: '…/w_0003.png'
00:00.958  render           | Rendering frame 10
00:00.962  render           | Saved: '…/w_0010.png'
00:00.962  render           | Rendering frame 11
00:00.967  render           | Saved: '…/w_0011.png'
00:00.967  render           | Rendering frame 12
00:00.972  render           | Saved: '…/w_0012.png'
```

Run on the Creator's MacBook Pro, M1 Pro chip, macOS 26.5.2.

### Operating system

The sources do not say the frame arguments differ by operating system, and the same page marks other arguments that do. `-con/--start-console` is "(Windows only)", `--no-window-frame` is "(Linux only)", `-r/--register` and `--register-allusers` (and their unregister twins) are "(Windows & Linux only)", and `--qos <level>` is "(Windows only)". No such marker appears on `-f`, `-s`, `-e`, `-j` or `-a`. *(Citation check, 2026-08-05: two of these were named wrongly — there is no `--register-extension` argument on the page, and no `-q/--quality-of-service`; `-q` is `--quiet`. Corrected to the names the page uses.)* Reading the absence of a marker as "it works everywhere" is a step I am taking, so treat that as **inferred**, not documented. (Source for the markers: https://docs.blender.org/manual/en/5.2/advanced/command_line/arguments.html, read 2026-07-30.)

One real, documented, OS-shaped edge sits next to this: the same page says of `--python-expr` that "you are limited only by the platform's maximum argument length". That is said about a different argument, so I cannot carry it over to `-f` — but a very long comma separated frame list is the same shape of thing, and the sources do not say what its limit is on any operating system.

---

## Things a person with Blender 5.2 open could still settle by hand

Everything that stood here has now been run except one item, and the results are written into the answer above as observed findings, dated 2026-08-05 for the Creator's Mac and 2026-08-06 for the rented Linux machine. **Whitespace inside a `-f` list** was tested on the rented Linux machine and accepted, which is recorded above. One item is left:

1. **Which frames count as "selected objects" when nothing is selected**, for the keyframes-only playblast. The playblast itself was run with the cube selected, on both machines, and is recorded above; the nothing-selected case was not.

The equipment limits that stood here have shrunk. Every test above was run again on a rented Linux machine — Ubuntu 22.04.5 LTS, NVIDIA GeForce RTX 4090, driver 580.105.08, twelve cores, run over SSH — on 2026-08-06. **Windows stays open**, because no Windows machine has run it, and no card of a third make can be reached. **Untried rather than untestable: a Windows machine can be rented, and every one this project has read reported a Server edition — see the equipment section of `Preparation/Blender Research/Method.md`.**

---

2026-08-04 — Opus 5 (Claude Code) — Frames, Blender 5.2

2026-08-05 — Opus 5 (Claude Code) — hand-test results written into question 7, and the hand-test list above pruned

2026-08-05 — Opus 5 (Claude Code) — the Creator's own hand-test result written into question 7, and the hand-test list above pruned again

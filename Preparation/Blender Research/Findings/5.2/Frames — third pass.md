# Frames — Blender 5.2 — third pass

**Sources consulted, and when.** The Blender 5.2 manual (plain-text copy in `Blender Sources/manual_5.2`, plus one picture from its `_images` folder), the Blender 5.2 Python API documentation (`Blender Sources/api_5.2`), the Blender 5.2 release notes (`Blender Sources/release_notes_5.2`), and Cycles' own files for 5.2 (`Blender Sources/cycles_5.2`). All four were taken on **2026-07-30**, and that is the date read for every citation below. I did the searching and reading on 2026-08-05.

**I read only the question sheet and files inside Blender Sources/. I did not open any other part of this project.**

I used no helper agents. Every search and every file named below was opened by me, so the declaration above covers the whole of this pass.

Only question 7 was assigned to me. It is the only one answered here.

---

## 7. Can Blender be given a set of frames that is not one plain run — single frames, or two separate stretches?

### How I read the question

I read it as: can the thing you hand to a render be something other than a single unbroken start-to-end sweep — a single frame on its own, several loose frames, or two separate stretches. That is the reading I answer.

It could also be read more narrowly as: does Blender hold such a set anywhere as a saved setting, the way it holds Start and End. That second reading has a shorter answer, and I give it too, because the two answers differ.

### Answer

**Yes, but only when Blender is started from the command line.** There it accepts a single frame, a comma-separated list of frames, and a range written with two dots. Inside Blender's own window there is no such thing: the render range is Start, End and Step, and nothing else. Nothing in the file saves a loose set of frames.

Broken into separate claims, because they do not all carry the same weight:

---

**7a. A single frame on its own — yes. Documented.**

The command-line option that does it:

> "-f, --render-frame <frame>"
> "Render frame <frame> and save it."

And a worked example on the command-line rendering page:

> "-f 10"
> "Render only the 10th frame."

There is also a relative form, counted from the ends of the scene's range:

> "+<frame> start frame relative, -<frame> end frame relative."

with the example:

> "-f -2"
> "Render only the second last frame."

Source: manual, Command Line Arguments — https://docs.blender.org/manual/en/5.2/advanced/command_line/arguments.html — and Rendering From The Command Line — https://docs.blender.org/manual/en/5.2/advanced/command_line/render.html. Date read: 2026-07-30.

---

**7b. Several loose single frames in one go — yes. Documented.**

> "A comma separated list of frames can also be used (no spaces)."

Source: manual, Command Line Arguments — https://docs.blender.org/manual/en/5.2/advanced/command_line/arguments.html. Date read: 2026-07-30.

That sentence sits directly under the `-f, --render-frame` entry, so the list is what `-f` accepts. The "(no spaces)" is the manual's own wording, not mine.

---

**7c. A stretch written as one argument — yes. Documented.**

> "A range of frames can be expressed using .. separator between the first and last frames (inclusive)."

Source: manual, Command Line Arguments — https://docs.blender.org/manual/en/5.2/advanced/command_line/arguments.html. Date read: 2026-07-30.

Note "inclusive": the last frame named is rendered.

---

**7d. Two separate stretches in a single command — inferred, not documented.**

The manual states the comma list (7b) and the two-dot range (7c) as two separate sentences under the same option. It never says the two forms may be combined, and it gives no example of a combined form. So a command asking for one stretch and then another stretch — two ranges separated by a comma — is not something the sources show.

The step a reader has to supply to get there: both sentences describe what the single `-f` option accepts, so a value made of ranges joined by commas would be accepted. That is my step, not the manual's, which is why this is inferred and not documented. **It is the one part of this answer I would not build anything on without checking it by hand.** See the hand-check list at the end.

There is a second route to two stretches that leans on a different documented sentence:

> "Note"
> "Arguments are executed in the order they are given!"

Source: manual, Rendering From The Command Line — https://docs.blender.org/manual/en/5.2/advanced/command_line/render.html. Date read: 2026-07-30.

The same page adds:

> "Always position -f or -a as the last arguments."

Step supplied by me: if arguments run in the order given, then giving `-f` twice in one command, or simply running Blender twice, renders both stretches. The manual neither states nor rules out a repeated `-f`. Running Blender twice needs no inference at all and is the safe way to get two stretches.

---

**7e. Inside Blender's window — no. Documented.**

The render range panel offers exactly three numbers:

> "Frame Start, End"
> "Set the Start and End frames for Rendering Animations."

> "Step"
> "Controls the number of frames to advance for each frame rendered, a value above 1 skips frames."

Source: manual, Frame Range — https://docs.blender.org/manual/en/5.2/render/output/properties/frame_range.html. Date read: 2026-07-30.

I also opened the picture on that page (`_images/render_output_properties_frame_range_panel.png`, shown on the Frame Range page above). It shows a panel titled "Frame Range" with three fields — Frame Start set to 1, End set to 250, Step set to 1 — and a collapsed "Time Stretching" section below them. There is no field for a list of frames and no way in the picture to add a second range. Label for what the picture shows: **documented**. What is visible in it is visible; I have not assumed anything beyond the four rows it shows.

The Timeline editor exposes the same two numbers and no more:

> "You can change this using the Start/End inputs in the Timeline header, or in the Output Properties."

Source: manual, Timeline — https://docs.blender.org/manual/en/5.2/editors/timeline.html. Date read: 2026-07-30.

The same three numbers are what the file stores. In the Python API, the scene carries:

> "frame_start" — "First frame of the playback/rendering range (in [0, 1048574], default 1)"
> "frame_end" — "Final frame of the playback/rendering range (in [0, 1048574], default 250)"
> "frame_step" — "Number of frames to skip forward while rendering/playing back each frame (in [0, 1048574], default 1)"

Source: Python API, Scene — https://docs.blender.org/api/5.2/bpy.types.Scene.html. Date read: 2026-07-30.

There is no fourth property holding a set or a list. I checked this by name across the whole API index, not just this page (see the record of looking, below).

---

**7f. A fixed-interval subset — yes, and it is the only non-plain set the interface offers. Documented.**

The Step number above skips frames. The manual recommends it exactly for rendering part of an animation:

> "It can be useful to render a subset of the animated sequence, since only part of an animation may have an error."
> "Using an image format for output, you can use the Frame Step option to render every N'th frame."

Source: manual, Rendering Animations — https://docs.blender.org/manual/en/5.2/render/output/animation.html. Date read: 2026-07-30.

The command line can set the same thing:

> "-j, --frame-jump <frames>"
> "Set number of frames to step forward after each rendered frame."

Source: manual, Command Line Arguments — https://docs.blender.org/manual/en/5.2/advanced/command_line/arguments.html. Date read: 2026-07-30.

A warning on reading that page: further down, under a different heading, "Animation Playback Options", the same letters `-f`, `-j`, `-s` and `-e` appear again with completely different meanings (frames per second, frame step, play from, play until). Those belong to Blender's built-in animation player, not to rendering. I have not mixed the two.

---

**7g. One genuinely non-contiguous set does exist in the interface — but it is a viewport render, not a full render. Documented.**

> "To render an animation, but only those frames that have a keyframe, use 3D Viewport ‣ View ‣ Render Playblast on Keyframes. This only renders those frames for which the selected objects have an animation key. The other frames are still written to the output, but will simply repeat the last-rendered frame."

Source: manual, Viewport Render — https://docs.blender.org/manual/en/5.2/editors/3dview/viewport_render.html. Date read: 2026-07-30.

Two limits, both from the same page:

> "Viewport rendering only works for the Workbench and EEVEE render engines. It's not supported for Cycles."

and, from the quote above, the frames that are skipped are still written out as copies of the previous one. So the set of frames *computed* is non-contiguous; the set of files *written* is still the plain run.

The same thing exists in the Python API as a switch on the viewport-render operator:

> "render_keyed_only (bool) – Render Keyframes Only, Render only those frames where selected objects have a key in their animation data. Only used when rendering animation (optional)"

Source: Python API, bpy.ops.render — https://docs.blender.org/api/5.2/bpy.ops.render.html. Date read: 2026-07-30.

Which frames these are is decided by the keys on the selected objects. You cannot type in the list yourself.

---

**7h. From Python, a render can be handed one stretch per call, chosen at the time of the call. Documented parameters; combining calls is inferred.**

The render operator takes a start and an end:

> "bpy.ops.render.render(*, animation=False, write_still=False, use_viewport=False, use_sequencer_scene=False, layer='', scene='', frame_start=0, frame_end=0)"

> "frame_start (int) – Start Frame, Frame to start rendering animation at. If not specified, the scene start frame will be assumed. This should only be specified if doing an animation render (in [-inf, inf], optional)"

> "frame_end (int) – End Frame, Frame to end rendering animation at. If not specified, the scene end frame will be assumed. This should only be specified if doing an animation render (in [-inf, inf], optional)"

Source: Python API, bpy.ops.render — https://docs.blender.org/api/5.2/bpy.ops.render.html. Date read: 2026-07-30.

That is one stretch per call. There is no list parameter. Step supplied by me: calling it twice with different start and end values renders two separate stretches; that composition is mine, not the documentation's, so this route is **inferred**.

For loose single frames there is an equivalent pair of documented pieces — moving the scene to a frame, then rendering the current frame and saving it:

> "frame_set(frame, *, subframe=0.0)" — "Set scene frame updating all objects and view layers immediately"

Source: Python API, Scene — https://docs.blender.org/api/5.2/bpy.types.Scene.html. Date read: 2026-07-30.

> "write_still (bool) – Write Image, Save the rendered image to the output path (used only when animation is disabled) (optional)"

Source: Python API, bpy.ops.render — https://docs.blender.org/api/5.2/bpy.ops.render.html. Date read: 2026-07-30.

Both pieces are documented. Stringing them into a loop over an arbitrary set of frames is my step, so again **inferred**.

---

### LABEL

**Mixed, and deliberately so.** 7a, 7b, 7c, 7e, 7f and 7g are **documented**. 7d and 7h are **inferred**, with the step written out in each. One small point in the record below is **observed** (read directly out of a Cycles file). Nothing here is "not in the sources": the sources do answer the question asked, for the command line and for the interface.

### Not covered

- Whether one `-f` argument may mix commas and two-dot ranges — the exact case "two separate stretches in one command". The sources state the two forms separately and never combine them. Unsettled.
- Whether `-f` may be given more than once in one command. Unsettled.
- What happens if a frame is named twice, or if the list is out of order (does it render 5 before 3, and does it render 3 twice?). Nothing in the sources speaks to order or duplicates in a `-f` list.
- Whether Step or `-j` has any effect on a `-f` list. The sources describe `-j` as stepping "after each rendered frame", which is written about a run, not a list. Unsettled.
- What a `-f` list does with a video-file output rather than separate images. Not addressed anywhere I looked.
- The largest list `-f` will take, and what happens if it is exceeded. Not stated.
- Whether `bpy.ops.render.render`'s `frame_start`/`frame_end` obey the scene's Step. Not stated on that page.
- Whether the alternative range some editors offer for playback affects a real render. The API says of it: "Use an alternative start/end frame range for animation playback and view renders (default False)" (Scene, `use_preview_range`, https://docs.blender.org/api/5.2/bpy.types.Scene.html, read 2026-07-30). "Playback and view renders" does not say final renders. I did not find a statement either way about final renders, so I leave it open rather than read the omission as a denial.
- Ordering, interruption and what survives when such a set is rendered — those are other questions on the sheet and I have not strayed into them.

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

This is the part of the answer that matters most here, and it cuts the other way from what you might expect: **the loose-frame-set capability exists only outside the interface.**

- Driven from outside — started from a command line — Blender takes a single frame, a comma-separated list of frames, or a two-dot range, all through `-f` (7a, 7b, 7c). This is documented only on the command-line pages; it has no counterpart in the window.
- Inside the window there is no list. There are three numbers — Start, End, Step (7e, 7f) — plus the keyframes-only viewport render (7g), which is not a full render.
- The `-f` option is documented under "Render Options" and the command-line rendering page shows it used together with background mode (`blender -b file.blend -f 10`), so it is plainly meant for unattended running.
- One caution the manual states outright for outside-the-interface use: "Arguments are executed in the order they are given!" and "Always position -f or -a as the last arguments." (Rendering From The Command Line, https://docs.blender.org/manual/en/5.2/advanced/command_line/render.html, read 2026-07-30). The same page warns: "Arguments are case sensitive! -F and -f are not the same." A frame list placed before the output path is set will not go where you intended — the manual gives that exact failure as a worked example.
- The keyframes-only viewport render (7g) is reachable as a Python operator, so in principle it could be driven from outside, but the sources do not say whether viewport rendering works with no window open. I will not guess.

### Operating system

The sources do not state any operating-system difference for any of this.

Worth noting how much weight that carries: the Command Line Arguments page *does* flag arguments that are limited to one system, explicitly and in the same style — for instance "/?" is marked "(Windows only)" and "-r, --register" is marked "(Windows & Linux only)". No such marking appears on `-f`, `-a`, `-s`, `-e` or `-j`. Reading that absence as "works everywhere" is a step I am supplying, so treat it as **inferred**, not documented. The three per-system launch pages (Linux, macOS, Windows) explain only how to start Blender from a terminal; I searched all three for any mention of frames, rendering or argument handling and found none.

---

## Where I looked

Recorded in full, because this pass exists to have it. Everything below was searched across the whole of `manual_5.2`, `api_5.2`, `release_notes_5.2` and `cycles_5.2` unless I say otherwise.

**Searches that found the answer**

- `render-frame` — one file in the entire set of sources: the manual's Command Line Arguments page. That page is where 7a, 7b and 7c come from.
- `comma separated` / `comma-separated` — eleven files across the sources; only the Command Line Arguments page was about frames. The others were about extension repositories, a rig type, a spreadsheet-style import node, simulation nodes, a text node and a mask node, plus one unrelated line in a Cycles kernel header. All ruled out on subject.
- Every occurrence of the word "frame" in the Command Line Arguments page, read one by one (28 hits). That is how I found the second, unrelated set of `-f`/`-j`/`-s`/`-e` options belonging to the animation player, and made sure not to confuse them with the render options.
- `frame_step` — ten files in the API, three of them alphabetical index pages. Only `bpy.types.Scene` is the render-range one; the others belong to motion paths, point caches, a curve modifier, tool settings and an exporter, and were ruled out on what they belong to.
- `Frame Range` (any capitalisation) in the manual — fifty-one files. I opened the render-relevant ones: Frame Range (output properties), Rendering Animations, Output properties index, Timeline, Metadata. The rest belong to grease pencil, the animation editors, physics caches, the sequencer, importers and add-ons, and were ruled out on subject.
- `Viewport Render` — seven hits across four files; the Viewport Render page carries 7g.
- `Playblast` — four hits in the manual, none in the API, none in the 5.2 release notes except an unrelated bug-fix line.

**Searches that came back empty, which is what makes the "no list in the interface" part of the answer stand up**

Each of these was run across the manual, the API documentation and the release notes together:

- "list of frames" — one hit only, the comma-list sentence already quoted.
- "frame list" — nothing.
- "frames to render" — nothing.
- "specific frames" — one hit, in a cloth-simulation page about editing results. Ruled out.
- "individual frames" — nothing.
- "non-contiguous" / "noncontiguous" — two hits, both about mesh normals and UVs. Ruled out.
- "arbitrary frames" — nothing.
- "subset of frames" — nothing.
- "multiple ranges" — nothing.
- "render queue" — nothing.
- "network render" — nothing.
- "chunk of frames" — nothing.
- "batch render" — one hit, in a scripting-security page, not about frame sets.
- "frame ranges" (plural) — ten hits, mostly the same boilerplate line about previewing segments in the animation editors, plus grease pencil and preferences. None about handing a render a set.
- "custom range" — hits in grease pencil modifiers, a volumes setting, the geometry-nodes bake page (which does have a custom start/end range, but for baking, not rendering), and a keyframe operator. None is a frame set for rendering.
- "Render Range" — nothing.
- "Set Range" — one hit, on Action, about an action's own playback range.
- "every N" / "Nth frame" / "every other frame" — hits belong to a curve modifier, brick patterns, sequencer strip display, physics baking and a compositor retime node. The only render-relevant one is the Frame Step advice already quoted at 7f.

**Places I checked by structure rather than by keyword**

- The complete list of render operators in the API (`bpy.ops.render`, https://docs.blender.org/api/5.2/bpy.ops.render.html): sixteen operators. I read the signature of every one. Only two take frames at all — `render` (a start and an end) and `opengl` (the keyframes-only switch). None takes a list. The rest are presets, cache operations, the animation player, a dimension swap and window handling.
- The complete list of properties beginning "frame" on `bpy.types.Scene`: frame_current, frame_current_final, frame_end, frame_float, frame_preview_end, frame_preview_start, frame_start, frame_step, frame_subframe. No list, no collection, no set.
- The alphabetical API index entry for every property named `frame_start` and `frame_step` across all of Blender — twenty-four types and five types respectively. Scene is the only one that is the render range. I did this specifically to be sure no other data block quietly holds a render frame set.
- Properties beginning "frame" on `bpy.types.RenderSettings`: only `frame_map_new`, `frame_map_old` (time stretching) and the `frame_path()` function that reports where a given frame would be written. Nothing that holds a set.
- The whole of the render output documentation folder, file by file: animation, animation player, audio, introduction, format, frame range, index, metadata, output, post-processing and the three stereoscopy pages. The frame-set material is all in Frame Range and Rendering Animations.
- The command-line folder in full: arguments, extension arguments, index, the rendering page, and the three per-system launch pages. The `-c, --command` sub-commands were checked in case a render sub-command existed; the only documented sub-commands manage extensions.
- The 5.2 release notes, all twenty-one files: searched for "frame range", "render-frame", "frame_start", "frame step" and "command line", then re-read every hit. Nothing about frame ranges or frame sets. The only "frame" mentions in the whole 5.2 notes are two new annotation-stroke functions in the Python API notes, and one unrelated bug-fix line each in the bug-fix list. So nothing in 5.2 changed this area, as far as the notes record.
- Cycles' own files: searched every Python file for "frame". Five hits, none of which is a setting. Cycles has no frame-range settings of its own — the frame range belongs to the scene, not the engine. Label: **observed**, from reading `Blender Sources/cycles_5.2/__init__.py`, `engine.py`, `operators.py`, `properties.py` and `ui.py` on 2026-07-30.
- One observed detail from those files, included because it is the only place in Cycles that expresses a set of frames at all: Cycles' "Denoise Animation" operator walks a plain unbroken run. In `Blender Sources/cycles_5.2/operators.py`, inside `class CYCLES_OT_denoise_animation` (the class begins at line 30), line 79 reads:

  > `            for frame in range(scene.frame_start, scene.frame_end + 1):`

  This is start to end with nothing skipped, and it does not consult Step. Label: **observed**, from that file, read 2026-07-30. It concerns denoising, not rendering, so it settles nothing about renders — I record it only because it is the one frame set expressed in the Cycles files I was given, and it is not a list.
- The Cycles source folder supplied here contains only `kernel` and `util`. There is no Blender-integration code in it, so there was nothing there that could describe how frames are handed to a render.

**One thing I did not do.** I did not go to the web, and did not need to.

---

## Things a person with Blender 5.2 open could settle in a few minutes

**Nine items stood here, and eight of them have since been run.** They were answered by the hand tests on the Creator's Mac on 2026-08-05 and again on a rented Linux machine on 2026-08-06, and the results are written into the answer above and into `Preparation/Blender Research/Findings/5.2/Frames.md`. Taking them in the order they stood:

1. **One `-f` does accept two stretches.** `-f 1..3,10..12` wrote all six files with no error, on both machines. That settles 7d, the one inferred claim in this answer that anything would be built on.
2. **A comma list of loose frames works as documented**, on both machines.
3. **A list renders in the order written**, not sorted, and a repeat is rendered twice — on both machines.
4. **`-f` can be given twice in one command**, and both stretches are rendered. Run on the rented Linux machine only.
5. **Step does not apply to a `-f` list.** With the scene's step set to 3, `-f 1,2,3,4,5` wrote all five files. Run on the rented Linux machine only.
6. **The Frame Range panel holds only Frame Start, End, Step and a Time Stretching section**, confirmed by the Creator on the Mac and photographed on the rented Linux machine.
7. **The alternative playback range does not affect a real render.** With the scene range 1 to 8 and an alternative playback range of 4 to 5 switched on, all eight frames were written. Run on the rented Linux machine only.
8. **The keyframes-only viewport render writes six files for a six-frame range** with keys on only two frames, on both machines. It cannot be reached at all with no window. On the Mac the Creator reported the six files identical; compared byte for byte on the rented Linux machine, no two of the six are the same file, while all six are the same size. Both records stand.
9. **A list of five thousand frames is accepted and renders.** Two hundred thousand never reaches Blender — the operating system's shell refuses the command first, differently on each of the two machines.

**One is left**, and it is item 8's other half rather than one of the nine: which frames the keyframes-only playblast picks when nothing is selected. That was not run on either machine.

The full record of the Linux runs is in `Audits/Blender Research/Frames/Hand tests — Linux.md`.

---

2026-08-05 — Opus 5 (Claude Code) — Frames, Blender 5.2

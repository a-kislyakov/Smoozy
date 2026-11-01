# Frames — Blender 5.0

**Sources consulted.** Blender 5.0 manual (`Blender Sources/manual_5.0`, including its `_images` folder), Blender 5.0 Python API documentation (`Blender Sources/api_5.0`), Blender 5.0 release notes (`Blender Sources/release_notes_5.0`), and Cycles' own files (`Blender Sources/cycles_5.0`). Each citation below gives its date read.

I read only the question sheet and files inside Blender Sources/. I did not open any other part of this project.

No helper agents were used. Everything below I read myself.

---

## 1. Can a render that was interrupted by the computer going down be picked up from where it stopped?

**Answer.** Yes, but only frame by frame, and only when the animation was written as separate image files. The manual addresses this case by name. It documents re-running the job with Overwrite off so that frames already on disk are skipped. It does not document what happens inside the frame that was under way.

**Label: documented (the recovery route); not in the sources (what happens inside the frame under way).**

The manual's Hints section for rendering animations opens with the exact situation:

> "Your computer accidentally turns off in the middle of rendering your movie!"

and continues:

> "Unless your animation renders in a few minutes, it is best to render the animation as separate image files. Instead of rendering directly to a compressed movie file, use a lossless format (e.g. PNG)."

> "This allows you an easy recovery if there is a problem and you have to re-start the rendering, since the frames you have already rendered will still be in the output directory."

> "Just disable the Overwrite option to start rendering where you left off."

- Source: manual — https://docs.blender.org/manual/en/5.0/render/output/animation.html
- Date read: 2026-07-30

The same page lists, among the reasons to choose the Frame Sequence approach:

> "May need to interrupt rendering to use the computer, and want to be able to resume rendering where you left off."

- Source: manual — https://docs.blender.org/manual/en/5.0/render/output/animation.html
- Date read: 2026-07-30

**Not covered.** The sources do not say what happens to the frame that was being written at the instant the machine went down — whether a partly written file is left behind, and whether disabling Overwrite would then skip that damaged file rather than redo it. They do not say anything about resuming inside a frame (part-finished samples or tiles). They do not cover video output for this case (see question 5). There is no mention anywhere of a crash-resume feature, a checkpoint, or a saved render state; I searched the manual and API for "resume", "checkpoint", "crash", "power", "partial", "incomplete", "corrupt", "truncated", "atomic" and "temporary file" and found nothing beyond the above.

**Observed — hand test 7, 2026-08-05.** A computer going down cannot be reproduced from a terminal, but a Blender killed outright, with no chance to tidy up, is the closest thing to it. Blender 5.0.1 was started from a terminal with no window, rendering the default cube scene in Cycles at 1000 samples and 400 by 400 pixels, PNG output into an empty folder, frames 1 to 5 at about five seconds a frame. Fifteen seconds in, while frame 3 was rendering, the process was sent a kill it could not catch. It ended 0.2 seconds later with exit code `-9`. The folder afterwards:

```
f_0001.png  92406 bytes
f_0002.png  92406 bytes
```

The two finished frames survived intact and nothing at all was left for the frame in flight — so the question the answer above marks not in the sources, what happens inside the frame under way, has one part settled: nothing of it reaches disk. Run on the Creator's MacBook Pro, M1 Pro chip, macOS 26.5.2.

**Observed — hand test 18, 2026-08-05.** The same kill, with the output set to a single video file, left a 48-byte file holding no frames at all. Recorded in full under question 5.

**Still unsettled, and why.** A real power-off was written as hand test 31 and marked optional. The Creator declined it — "I would rather not do Test 36", 2026-08-05, that being its number on the combined list they were working from. It is their own choice, not a limit of the equipment, and it has been moved to the "cannot be run" section of the hand-test list. So the question splits in two. The half where **the program dies** is answered above: an uncatchable kill leaves every finished frame whole and nothing at all for the frame in flight. The half where **the operating system had not yet flushed a file to disk** when the power went is not answered, and nothing short of a real power-off reaches it.

**Observed — Linux hand test 6, 2026-08-06.** The same kill, on a second machine, gave the same answer. This was run on a rented Linux machine — Ubuntu 22.04.5 LTS, NVIDIA GeForce RTX 4090, driver 580.105.08, twelve cores, run over SSH. Blender 5.0.1 was started there from a terminal with no window, against Blender's own startup scene, with its own settings folder and factory startup. Rendering the default cube scene in Cycles at 1000 samples and 400 by 400 pixels, PNG output into an empty folder, frames 1 to 5, it was sent a kill it could not catch fifteen seconds in. The exit code recorded was `-9`, and the finished frames were on disk whole — f_0001.png, f_0002.png and f_0003.png at 92409 bytes each — with nothing at all for the frame that was in flight. How many frames had finished by then differed from the Mac run, because the machine was shared with other work; no claim is made here about how fast anything ran.

**The half this cannot reach stays unreachable on both machines.** A real power-off was offered to the Creator and declined. It cannot be run on the rented Linux machine either: that machine is reached over the network, so cutting its power would leave nothing to read the result from.

**Outside the interface.** The mechanism is a scene setting, not a window action: `bpy.types.RenderSettings.use_overwrite` — "Overwrite existing files while rendering", boolean, default `True` (API — https://docs.blender.org/api/5.0/bpy.types.RenderSettings.html, read 2026-07-30). Unlike `use_single_layer`, its documentation carries no "interface only" caveat, so a driven Blender can set it. That much is documented. That the resume therefore behaves identically outside the interface is **inferred** — the step being that the setting is scene data and the command-line render (`-a, --render-anim`) renders the scene's frame range with the scene's settings. The command-line argument list contains no resume-related flag of any kind; I read the whole of it. **Observed**, from hand tests 2 and 7: the whole recovery works on a Blender started from a terminal with no window — Overwrite off skips the frames already on disk (question 4), and the finished frames survive an uncatchable kill.

**Operating system.** The sources do not say. Nothing in the passages above is marked as differing by platform. **Observed on two operating systems, and they agree** — macOS 26.5.2 on the Creator's MacBook Pro (hand test 7, 2026-08-05) and Ubuntu 22.04.5 LTS on the rented Linux machine (Linux hand test 6, 2026-08-06): an uncatchable kill leaves every finished frame whole and nothing for the frame in flight on both. **Windows remains untested**, because no Windows machine has run it. **Untried rather than untestable: a Windows machine can be rented, and every one this project has read reported a Server edition — see the equipment section of `Preparation/Blender Research/Method.md`.**

---

## 2. Can a render that was stopped on purpose be picked up later?

**Answer.** Yes, for animations written to image file formats. The manual states it directly, and gives two ways: change the frame range, or leave the range alone and turn Overwrite off.

**Label: documented.**

> "For example, when rendering animations to image file formats the render job can be canceled and resumed at the last rendered frame by changing the frame range. This is useful if the animation takes a long time to render and the computers resources are needed for something else."

- Source: manual — https://docs.blender.org/manual/en/5.0/render/output/introduction.html
- Date read: 2026-07-30

The second route is the one quoted in question 1: "Just disable the Overwrite option to start rendering where you left off." (manual — https://docs.blender.org/manual/en/5.0/render/output/animation.html, read 2026-07-30).

How a render is stopped on purpose is documented in two places:

> "As with a normal render, you can abort it with Esc."

- Source: manual — https://docs.blender.org/manual/en/5.0/editors/3dview/viewport_render.html
- Date read: 2026-07-30

> "Shows the progress of the currently running task (such as rendering or baking). Hovering the mouse pointer over the progress bar will display a time estimate. The task can be aborted by clicking the cancel button ()."

- Source: manual — https://docs.blender.org/manual/en/5.0/interface/window_system/status_bar.html
- Date read: 2026-07-30

**Not covered.** "Resumed at the last rendered frame" is the granularity the sources offer; nothing finer. The sources do not say what becomes of the frame that was in progress when the stop was made — whether it is discarded, or left on disk part-written. They do not describe a way to stop a render from outside Blender's window at all (no signal handling, no stop file, no command); see the "Outside the interface" line below. Video output is not covered here.

**Observed — hand tests 6 and 7, 2026-08-05.** A render started from a terminal with no window can be stopped, and the frame in progress is discarded rather than written. Blender 5.0.1 was started with no window, rendering the default cube scene in Cycles at 1000 samples and 400 by 400 pixels, PNG output into an empty folder, frames 1 to 5, at about five seconds a frame. Fifteen seconds in — during frame 3 — the process was sent an interrupt, which is exactly what pressing Ctrl-C in a terminal does. The last lines Blender printed were:

```
00:10.474  render           | Saved: '…/f_0002.png'
00:10.474  render           | Time: 00:05.03 (Saving: 00:00.00)
00:10.474  render           | Rendering frame 3
00:10.474  render           | Start rendering: Scene, ViewLayer
00:10.474  render           | Engine: Cycles

Sent an internal break event. Press ^C again to kill Blender

Blender quit
```

It ended 0.8 to 1.0 seconds after the interrupt (three separate runs: 0.8, 0.8, 0.8 seconds) with exit code `1`. The folder afterwards:

```
f_0001.png  92406 bytes
f_0002.png  92406 bytes
```

So "the last rendered frame" is unambiguous: only whole frames are on disk. A polite terminate signal instead ended it in 0.2 seconds with exit code `-15`, printing nothing further, and left the same two files. Run on the Creator's MacBook Pro, M1 Pro chip, macOS 26.5.2.

**Observed — Linux hand tests 5 and 6, 2026-08-06.** On the rented Linux machine, Blender 5.0.1 printed exactly what it printed on the Mac when sent an interrupt fifteen seconds into the same render:

```
Sent an internal break event. Press ^C again to kill Blender

Blender quit
```

and exited with code `1`, leaving only whole frames and nothing for the frame in flight. A polite terminate ended it with code `-15` and a kill with code `-9`, both leaving the same. **The render did not freeze** — worth recording because Blender 5.0's release notes carry a fixed bug called "Linux Freeze when force cancelling a CLI render", and nothing of that kind was seen here in any of the four versions. How long each stop took to bite is kept under question 11 as raw output and is not offered as a measurement: the machine was shared.

**Outside the interface.** Both documented ways of stopping — Esc and the status bar cancel button — need Blender's window. `-b, --background` is described as "Run in background (often used for UI-less rendering)" (manual — https://docs.blender.org/manual/en/5.0/advanced/command_line/arguments.html, read 2026-07-30), and there is no documented stop control for that mode. That a background render can only be stopped by ending the process is **not in the sources**; I searched the manual, API and release notes for "SIGTERM", "signal", "kill", "interrupt", "pause render" and "graceful" and found nothing on the subject. The *resuming* half is scene data and so is available from outside (see question 1). One related fact is recorded in the release notes, as a fixed bug: "Linux Freeze when force cancelling a CLI render" (release notes — https://developer.blender.org/docs/release_notes/5.0/bugfixes/, read 2026-07-30), which at least confirms that force-cancelling a command-line render is a thing that happens; the note does not describe the behaviour. **Observed**, from hand tests 6 and 7: such a render is stopped by an interrupt, a polite terminate, or a kill, and all three leave only whole frames behind.

**Operating system.** The sources do not say, except for that one bug note, which is specific to Linux and describes a fault that was fixed in 5.0, not current behaviour. **Observed on two operating systems, and they agree** — macOS 26.5.2 (hand tests 6 and 7, 2026-08-05) and Ubuntu 22.04.5 LTS on the rented Linux machine (Linux hand tests 5 and 6, 2026-08-06). An interrupt, a polite terminate and an uncatchable kill each stop the render on both, with the same three exit codes, and each leaves only whole frames. **The Linux runs bear directly on that bug note**, since they are exactly what it describes — force-cancelling a command-line render on Linux — and in twelve interrupts across the four versions nothing froze; every one printed `Sent an internal break event`, exited with code `1`, and left the finished frames whole. **Windows remains untested**, because no Windows machine has run it.

---

## 3. Do frames already written survive a render ending part-way, whether it was stopped on purpose or not?

**Answer.** For a computer going down, yes, for animations written as image sequences: the manual says frames already rendered remain in the output directory. For a deliberate stop, the sources say the animation can be resumed at the last rendered frame, but do not separately say that the earlier files remain there.

**Label: documented (computer going down); inferred (deliberate stop).**

> "While rendering stills will allow you to view and save the image from the render buffer when it is complete, animations are a series of images, or frames, and are automatically saved directly out to a drive after being rendered."

- Source: manual — https://docs.blender.org/manual/en/5.0/render/output/animation.html
- Date read: 2026-07-30

> "This allows you an easy recovery if there is a problem and you have to re-start the rendering, since the frames you have already rendered will still be in the output directory."

- Source: manual — https://docs.blender.org/manual/en/5.0/render/output/animation.html
- Date read: 2026-07-30

**Not covered.** This is about image sequences. For a single video file the manual says the opposite kind of thing (question 5). The sources do not distinguish a deliberate stop from a crash here — the sentence quoted sits under a Hints heading that covers the computer turning off, and the Frame Sequence guidelines cover deliberate interruption, but no source states the two cases separately. Nothing is said about the frame that was in progress. Nothing is said about whether an already-written frame could be damaged by the interruption (for instance if the operating system had not yet flushed it to disk).

**Observed — hand tests 1, 6 and 7, 2026-08-05.** Frames already written survive, and the deliberate stop and the crash give the same answer — so the inference above is now tested rather than reasoned. Blender 5.0.1 was started from a terminal with no window, rendering the default cube scene to PNG, and stopped part-way in three different ways: an interrupt (Ctrl-C), a polite terminate, and a kill it could not catch. In all three the folder afterwards held the finished frames, whole:

```
f_0001.png  92406 bytes
f_0002.png  92406 bytes
```

and nothing for the frame in flight. The whole run, undisturbed, prints one frame at a time:

```
00:00.395  render           | Rendering animation (frames 1..5)
00:00.395  render           | Rendering frame 1
00:00.396  render           | Start rendering: Scene, ViewLayer
00:00.396  render           | Engine: Workbench
00:00.545  render           | Saved: '…/f_0001.png'
00:00.545  render           | Time: 00:00.14 (Saving: 00:00.05)
00:00.545  render           | Rendering frame 2
…
00:00.563  render           | Saved: '…/f_0005.png'
```

Run on the Creator's MacBook Pro, M1 Pro chip, macOS 26.5.2.

**Observed — hand test 29, run by the Creator, 2026-08-05.** The same holds for a render started and cancelled inside Blender's own window, which is the case the inference above was reaching for. A prepared file was opened in Blender 5.0.1 — Cycles, 4000 samples, 400 by 400 pixels, PNG output into an empty folder, frames 1 to 10, about twenty-five seconds a frame. Render Animation was started, frames 1 and 2 were allowed to finish, and Esc was pressed while frame 3 was under way. Afterwards the output folder held exactly two files and nothing else:

```
92315 bytes  frame_0001.png
92315 bytes  frame_0002.png
```

The Creator ran the render and the cancel and left the folder for reading; the listing above is what was on disk afterwards. Their own words on the cancel: "You can probably look at the files yourself. Stopping isn't instant, there's a slight lag. All versions." Run on the Creator's Mac — a MacBook Pro, M1 Pro chip, macOS 26.5.2 — so this settles macOS and leaves Windows and Linux open.

**Observed — Linux hand tests 1, 5 and 6, 2026-08-06.** The same three ways of ending a render were tried on a rented Linux machine — Ubuntu 22.04.5 LTS, NVIDIA GeForce RTX 4090, driver 580.105.08, twelve cores, run over SSH and gave the same answer: every frame already written was there, whole, and nothing at all was left for the frame in flight. Undisturbed, the plain five-frame run there printed the same shape of output as on the Mac — `Rendering animation (frames 1..5)`, then `Rendering frame N`, `Start rendering: Scene, ViewLayer`, `Engine: Workbench` and `Saved:` for each frame, all timestamped and tagged `render` — exit code 0, five files of 3566 bytes each. One thing differs between the machines and is not about frames: the same scene gives a 3566-byte file on Linux where the Mac gave a different size. Nothing on this sheet asks about pixels; it is recorded only so nobody assumes a frame rendered on two machines is the same file.

**Observed — Linux hand test 32, 2026-08-06.** A render started the way the window's own Render Animation item starts it also leaves only whole frames. With a window open on the rented Linux machine's display, Blender 5.0.1 was given `bpy.ops.render.render('INVOKE_DEFAULT', animation=True)` — the call that menu item makes — which returned `{'RUNNING_MODAL'}`, and a separate **Blender Render** window opened. The render was ended part-way and the folder held whole frames of 92275 bytes and nothing for the frame in flight. Note what this does **not** show: whether the **Esc** key reached Blender could not be demonstrated on that machine, because its display is shared with other work and a key press could not be shown to land in the intended window. So this is a render that was ended, not a demonstrated Esc cancel; the Esc half rests on the Creator's macOS run.

**Outside the interface.** The sources do not say. Nothing in the quoted text limits it to the interface, and writing frames out during an animation render is described as what animation rendering does, not as a window feature; but the manual never addresses background rendering in this passage. **Observed**, from the hand tests above: it holds for a render started from a terminal with no window, and hand test 29 shows it holds for a render started and cancelled inside the window too — the same two whole frames, nothing for the third. So this answer no longer depends on how the render was started.

**Operating system.** The sources do not say. **Observed on two operating systems, and they agree** — macOS 26.5.2 (hand tests 1, 6, 7 and 29, 2026-08-05) and Ubuntu 22.04.5 LTS on the rented Linux machine (Linux hand tests 1, 5, 6 and 32, 2026-08-06). Frames already written survive on both, whichever way the render ends. **Windows remains untested**, because no Windows machine has run it.

---

## 4. Can Blender be asked to render only the frames that are not already on disk, and if so, how does it decide what counts as already there?

**Answer.** For image output, yes — by turning off Overwrite. What the sources say about the decision is only the phrase "existing files"; they never spell out the test.

**Label: documented (that the option exists); inferred (how the decision is made).**

The Output panel entry:

> "Overwrite existing files when rendering."

and the entry immediately after it:

> "Create empty placeholder frames while rendering."

- Source: manual — https://docs.blender.org/manual/en/5.0/render/output/properties/output.html
- Date read: 2026-07-30

(A note on those two: in the plain-text copy both entries carry the heading text "Image Sequence – Overwrite Image Multi-Layer EXR", which appears to be a conversion fault — the second one has lost its own name. The picture on the same page settles the names.)

The picture beside them, `manual_5.0/_images/render_output_properties_output_panel.png` (the manual's "Output panel." illustration on the page above), shows a row of two checkboxes: **Overwrite**, ticked, and **Placeholders**, unticked. **Label for that: documented** (it is part of the manual). Read 2026-07-30.

The API gives the same two as scene settings:

> "use_overwrite — Overwrite existing files while rendering — Type: boolean, default True"

> "use_placeholder — Create empty placeholder files while rendering frames (similar to Unix 'touch') — Type: boolean, default False"

- Source: Python API — https://docs.blender.org/api/5.0/bpy.types.RenderSettings.html
- Date read: 2026-07-30

The manual gives the intended use of the pair together:

> "Hint
>
> Primitive Render Farm
>
> An easy way to get multiple machines to share the rendering workload is to:
>
> Set up a shared directory over a network file system.
>
> Disable Overwrite, enable Placeholders in the Render Output panel.
>
> Start as many machines as you wish rendering to that directory."

- Source: manual — https://docs.blender.org/manual/en/5.0/render/output/properties/output.html
- Date read: 2026-07-30

And a second use, for filling in a stepped run:

> "Using an image format for output, you can use the Frame Step option to render every N'th frame. Then disable Overwrite and re-render with Frame Step set to 1."

- Source: manual — https://docs.blender.org/manual/en/5.0/render/output/animation.html
- Date read: 2026-07-30

**How it decides — the inference.** The sources say only "existing files". The file a given frame would occupy is a documented, computable thing:

> "frame_path(*, frame=-2147483648, preview=False, view='') — Return the absolute path to the filename to be written for a given frame"

- Source: Python API — https://docs.blender.org/api/5.0/bpy.types.RenderSettings.html
- Date read: 2026-07-30

Step I took: "Overwrite existing files while rendering" plus "the filename to be written for a given frame" gives — a frame counts as already there when a file exists at the path Blender would write it to. That is my step, not the source's words. The sources nowhere say the test is mere existence rather than, say, existence with non-zero size, and the Placeholders option makes the difference matter: a placeholder is by its own description an *empty* file, and the render-farm hint only works if an empty file counts as "there". So the test is existence alone. That last leg is **inferred** too, from the fact that the hint is presented as working.

There is one **observed** piece of adjacent evidence, from Cycles' own files. Cycles' animation denoise operator decides whether a rendered frame is present like this:

> ```
>             for frame in range(scene.frame_start, scene.frame_end + 1):
>                 scene.render.filepath = in_filepath
>                 filepath = scene.render.frame_path(frame=frame)
>                 in_filepaths.append(filepath)
>
>                 if not os.path.isfile(filepath):
>                     scene.render.filepath = original_filepath
>                     err_msg = tip_("Frame '%s' not found, animation must be complete") % filepath
> ```

- Source file: `Blender Sources/cycles_5.0/operators.py`, lines 79–87 (inside the animation branch of the denoise operator)
- Date read: 2026-07-30

This shows a plain existence check (`os.path.isfile`) against `frame_path(frame)`. **Important limit:** this is Cycles' *denoise* operator deciding whether a sequence is complete. It is not the code that implements Overwrite. It shows the shape of the idea, and that `frame_path` is the accepted way to name a frame's file; it does not prove what a render does.

**Not covered.** The sources do not say what the Overwrite check does with a file that exists but is damaged or half-written, nor whether it looks at file size, modification time, or content. They do not say whether the check happens once at the start of the job or immediately before each frame (which matters for several machines sharing a folder). They do not say what happens when Overwrite is off and Placeholders is off and two machines pick the same frame. They do not say whether Overwrite applies to a video output at all — the manual's headings suggest these two options belong to the Image and Multi-Layer EXR media types rather than Video, but that reading rests on how the plain-text conversion renders the panel headings and I mark it **inferred**. Nor do they say whether File Output nodes in the compositor honour Overwrite.

**Observed — hand test 2, 2026-08-05.** The test is bare existence, and an empty file counts. Blender 5.0.1 was started from a terminal with no window and rendered the default cube scene to PNG, frames 1 to 5, into an empty folder. Then `f_0004.png` was deleted outright and `f_0003.png` was emptied to zero bytes but left in place:

```
3639 f_0001.png
3639 f_0002.png
   0 f_0003.png
3639 f_0005.png
```

The same render was then run again with Overwrite off. Blender printed:

```
00:00.269  render           | Rendering frame 1
00:00.269  render           | Skipping existing frame "…/f_0001.png"
00:00.269  render           | Rendering frame 2
00:00.269  render           | Skipping existing frame "…/f_0002.png"
00:00.269  render           | Rendering frame 3
00:00.269  render           | Skipping existing frame "…/f_0003.png"
00:00.269  render           | Rendering frame 4
00:00.386  render           | Saved: '…/f_0004.png'
00:00.386  render           | Rendering frame 5
00:00.386  render           | Skipping existing frame "…/f_0005.png"
```

and the folder afterwards:

```
3639 f_0001.png
3639 f_0002.png
   0 f_0003.png
3639 f_0004.png
3639 f_0005.png
```

The zero-byte file was skipped and stayed zero bytes. Note that Blender announces `Rendering frame N` even for a frame it then skips. (Blender 4.5.12 running the same test prints no such announcement, and writes `skipping existing frame` with a small s.) Run on the Creator's MacBook Pro, M1 Pro chip, macOS 26.5.2.

**Observed — hand test 22, 2026-08-05.** The path a frame is checked against is the one `frame_path` returns, formed exactly as the manual describes. Blender 5.0.1, asked from a terminal with no window:

```
filepath '<dir>/f_'                     -> frame_path(7) = '<dir>/f_0007.png'
filepath '<dir>/f_##_x'                 -> frame_path(7) = '<dir>/f_07_x.png'
filepath '<dir>/plain'                  -> frame_path(7) = '<dir>/plain0007.png'
filepath '<dir>/six_######'             -> frame_path(7) = '<dir>/six_000007.png'
with File Extensions off        -> frame_path(7) = '<dir>/f_0007'
frame_path(12345) = '<dir>/big_12345.png'
```

Same machine and date.

**Observed — hand test 5, 2026-08-05.** The check happens once per frame, immediately before that frame is rendered — not once at the start of the job. Two copies of Blender 5.0.1 were started at the same moment from a terminal with no window, each with its own separate settings folder, both rendering frames 1 to 6 of the same slow scene into one shared folder with Overwrite off and Placeholders on. *(Citation check, 2026-08-05: what made the scene slow — its sample count, resolution and format — was not written down, so this run cannot be repeated exactly from what is here. It was run again with the scene stated, as hand test 33 below.)* The first said:

```
00:08.594  render           | Saved: 'f_0001.png'
00:16.742  render           | Saved: 'f_0002.png'
00:16.742  render           | Skipping existing frame "f_0003.png"
00:24.856  render           | Saved: 'f_0004.png'
00:24.856  render           | Skipping existing frame "f_0005.png"
00:32.964  render           | Saved: 'f_0006.png'
```

and the second said:

```
00:08.594  render           | Saved: 'f_0001.png'
00:16.736  render           | Saved: 'f_0002.png'
00:24.844  render           | Saved: 'f_0003.png'
00:24.844  render           | Skipping existing frame "f_0004.png"
00:32.935  render           | Saved: 'f_0005.png'
00:32.935  render           | Skipping existing frame "f_0006.png"
```

All six frames were present afterwards, each 92318 bytes. Two things came out of it. Frames were skipped that did not exist when either render started, so the check is made per frame as the render reaches it. And frames 1 and 2 were rendered by both copies, so claiming a frame with a placeholder is not proof against two renders reaching the same frame at the same instant. Run on the Creator's MacBook Pro, M1 Pro chip, macOS 26.5.2, with nothing else running.

**Observed — hand test 33, 2026-08-05.** The same test again, with the scene written down so anyone can repeat it. Two copies of Blender 5.0.1 were started at the same moment from a terminal with no window, each pointed at its own settings folder, made before it was used. Both rendered frames 1 to 6 into one empty shared folder on the local disk. No blend-file was involved: the scene was built by this text, handed to Blender on the command line, and it is the whole of the scene.

```
import bpy
s=bpy.context.scene
s.render.engine='CYCLES'
s.cycles.samples=800
s.cycles.use_adaptive_sampling=False
s.cycles.use_denoising=False
s.render.resolution_x=400
s.render.resolution_y=400
s.render.use_overwrite=False
s.render.use_placeholder=True
```

In words: Blender's own startup scene — cube, light, camera — rendered by Cycles at 800 samples with adaptive sampling and denoising both off, 400 by 400 pixels, Overwrite off, Placeholders on. The rest was given on the command line as `-o "<folder>/f_####" -F PNG -x 1 -s 1 -e 6 -a`, so PNG with file extensions on, frames 1 to 6. Measured on this machine, one frame takes 4.84 seconds with one Blender rendering alone and about 9.3 seconds with two rendering at once — Blender's own `Time:` line in both cases. The first copy printed:

```
00:00.438  render           | Rendering frame 1
00:09.812  render           | Saved: 'f_0001.png'
00:09.812  render           | Rendering frame 2
00:09.813  render           | Skipping existing frame "f_0002.png"
00:09.813  render           | Rendering frame 3
00:19.080  render           | Saved: 'f_0003.png'
00:19.081  render           | Rendering frame 4
00:19.081  render           | Skipping existing frame "f_0004.png"
00:19.081  render           | Rendering frame 5
00:28.420  render           | Saved: 'f_0005.png'
00:28.420  render           | Rendering frame 6
00:28.420  render           | Skipping existing frame "f_0006.png"
```

and the second printed:

```
00:00.438  render           | Rendering frame 1
00:09.757  render           | Saved: 'f_0001.png'
00:09.757  render           | Rendering frame 2
00:19.075  render           | Saved: 'f_0002.png'
00:19.075  render           | Rendering frame 3
00:19.075  render           | Skipping existing frame "f_0003.png"
00:19.075  render           | Rendering frame 4
00:28.418  render           | Saved: 'f_0004.png'
00:28.418  render           | Rendering frame 5
00:28.418  render           | Skipping existing frame "f_0005.png"
00:28.418  render           | Rendering frame 6
00:33.110  render           | Saved: 'f_0006.png'
```

Both exited 0. Blender printed the whole folder path inside each of those file names; only the path has been cut here, the words and the timestamps are as printed. All six frames were present afterwards, each 92318 bytes.

This agrees with hand test 5 on both of its conclusions. The shared folder was empty when the two started, so every skipped frame was one the other copy had written during the run — the check is made per frame as the render reaches it, and the timestamps show it: the second copy started frame 2 at 00:09.757, and at 00:09.813 the first copy was told frame 2 already existed. And frame 1 was rendered by both copies, so a placeholder does not stop two renders reaching the same frame at the same instant. The only difference from hand test 5 is which frames fell to which copy: there both copies rendered frames 1 and 2, here only frame 1. Run on the Creator's MacBook Pro, M1 Pro chip, macOS 26.5.2, with nothing else running, and to one local folder rather than a shared network one.

**Observed — hand test 19, 2026-08-05.** Overwrite has no effect on a video output — which settles the reading the answer above marks inferred. Blender 5.0.1 rendered a two-frame FFmpeg MPEG-4 H.264 file from a terminal with no window, then was asked to render exactly the same thing again with Overwrite off. It printed no skipping line, the second run took 10.7 seconds — the full rendering time — and the file's modification time moved from `01:25:14` to `01:25:24`. Same machine and date.

**Observed — Linux hand tests 2, 21 and 18, 2026-08-06.** Everything above about the decision rule came back identical on a rented Linux machine — Ubuntu 22.04.5 LTS, NVIDIA GeForce RTX 4090, driver 580.105.08, twelve cores, run over SSH. Blender 5.0.1 there rendered frames 1 to 5, then `f_0004.png` was deleted and `f_0003.png` emptied to zero bytes, and the render was run again with Overwrite off:

```
00:00.317  render           | Rendering frame 1
00:00.317  render           | Skipping existing frame "…/f_0001.png"
00:00.317  render           | Rendering frame 2
00:00.317  render           | Skipping existing frame "…/f_0002.png"
00:00.317  render           | Rendering frame 3
00:00.317  render           | Skipping existing frame "…/f_0003.png"
00:00.317  render           | Rendering frame 4
00:01.015  render           | Saved: '…/f_0004.png'
00:01.015  render           | Rendering frame 5
00:01.015  render           | Skipping existing frame "…/f_0005.png"
```

The zero-byte file was skipped and stayed zero bytes; the folder afterwards held five files, four of 3566 bytes and `f_0003.png` still at 0. Asked what path frame 7 would take, Linux gave the same six answers as the Mac for every output path tried. And rendering the same two-frame video twice, the second time with Overwrite off, printed no skipping line and moved the file's modification time — so Overwrite has no effect on a video output there either.

**Observed — Linux hand test 27, 2026-08-06, twelve runs, and it does not agree with the Mac.** Two copies of Blender 5.0.1 sharing one folder with Overwrite off and Placeholders on were run **three times in each of the four versions** on the rented Linux machine, using the scene written out in hand test 33 above, unchanged. In all twelve runs the six frames were shared out between the two copies with **no frame rendered twice**. The first run in this version printed, from the first copy:

```
00:09.460  render           | Saved: 'f_0001.png'
00:09.460  render           | Skipping existing frame "f_0002.png"
00:09.460  render           | Skipping existing frame "f_0003.png"
00:19.283  render           | Saved: 'f_0004.png'
00:19.283  render           | Skipping existing frame "f_0005.png"
00:28.145  render           | Saved: 'f_0006.png'
```

and from the second copy:

```
00:00.245  render           | Skipping existing frame "f_0001.png"
00:08.784  render           | Saved: 'f_0002.png'
00:18.688  render           | Saved: 'f_0003.png'
00:18.688  render           | Skipping existing frame "f_0004.png"
00:28.056  render           | Saved: 'f_0005.png'
00:28.056  render           | Skipping existing frame "f_0006.png"
```

Both exited 0, and all six files were 92318 bytes. The other two runs in this version shared the frames out differently again and again showed no frame rendered twice.

**The two machines disagree, and neither is chosen.** On the Mac, at least one frame was rendered by both copies in most runs. On the rented Linux machine, in twelve runs, none was. This is a race between two programs and which way it falls is not fixed. What the Mac showed — that a placeholder can fail to reserve a frame — is not undone by twelve runs here that happened not to collide, and nothing here says a shared folder is safe on Linux. Both records stand. What both machines agree on is the other conclusion: every skipped frame was one the other copy had written during the run, so the check is made per frame as the render reaches it.

**Outside the interface.** Both are scene settings on `RenderSettings` with no "interface only" caveat in their documentation, so they can be set from a script or a `.blend` prepared in advance. That is documented. That the skipping then behaves the same in a background command-line render is **inferred** — the command-line argument list has no Overwrite flag, so the only route is the scene setting or a Python script. **Observed**, from the hand tests above: every one set the option from a Python expression on the command line and drove Blender with no window, and the skipping worked.

**Operating system.** The sources do not say. The placeholder tooltip mentions Unix `touch` only as a description of the effect, not as a platform limit. The render-farm hint assumes a shared directory "over a network file system" but names no system. **Observed on two operating systems.** The decision rule, the path checked, the per-frame timing and the lack of effect on a video output are identical on macOS 26.5.2 (hand tests 2, 22 and 19, 2026-08-05) and on Ubuntu 22.04.5 LTS on the rented Linux machine (Linux hand tests 2, 21 and 18, 2026-08-06). **Two Blenders sharing a folder came out differently on the two machines**, as recorded just above. Both machines wrote to a local folder rather than a shared network one, so the render-farm hint's own case is untested on either. **Windows remains untested**, because no Windows machine has run it.

---

## 5. What does an animation written as a single video file leave behind when the render stops part-way?

**Answer.** The manual says the file may be left unplayable, and that there is no picking it up — the whole thing has to be rendered again from the start.

**Label: documented.**

> "It's not recommended to render directly to a video format in the first instance. If a problem occurs while rendering, the file might become unplayable and you will have to re-render all frames from the beginning."

The same passage offers the alternative and its benefit:

> "Restart the rendering from the place (the frame) where any problem occurred."

- Source: manual — https://docs.blender.org/manual/en/5.0/render/output/properties/output.html
- Date read: 2026-07-30

The animation page frames the choice the same way. It calls rendering straight to a movie file the "Direct Approach", says it "is highly not recommended and not a standard practice", and lists among the conditions for using it:

> "Stable power supply."

> "Computer not needed for other uses."

- Source: manual — https://docs.blender.org/manual/en/5.0/render/output/animation.html
- Date read: 2026-07-30

One documented thing about the size of what is left behind:

> "If your video is huge and exceeds 2GiB, enable Autosplit Output. This will automatically split the output into multiple files after the first file is 2GiB in size."

- Source: manual — https://docs.blender.org/manual/en/5.0/render/output/properties/output.html
- Date read: 2026-07-30

and in the API: `use_autosplit` — "Autosplit output at 2GB boundary", boolean, default `False` (Python API — https://docs.blender.org/api/5.0/bpy.types.FFmpegSettings.html, read 2026-07-30).

**Not covered.** "Might become unplayable" is as precise as the sources get. They do not say whether a file is left on disk at all, whether it holds the frames encoded so far, whether the container is finalised, whether some players could still read it, or whether audio is affected. "Might" leaves open that it is sometimes playable; the sources give no rule for which. Autosplit's behaviour on an interrupted render is not described — only its 2GiB rule. Nothing is said about what an interrupted video render leaves when Autosplit has already produced one or more complete earlier parts.

**Observed — hand test 18, 2026-08-05.** What is left depends entirely on how the render was stopped, and the two answers are opposite. Blender 5.0.1 was started from a terminal with no window, rendering the default cube scene in Cycles at 1000 samples and 400 by 400 pixels, output set to FFmpeg, container MPEG-4, codec H.264, Autosplit off, frames 1 to 20, at about six seconds a frame. The output folder was watched twenty times a second:

```
  0.00s  (nothing)
  0.56s  movie_0001-0020.mp4=48 bytes
 20.06s  SENT SIGINT
 24.25s  movie_0001-0020.mp4=2423 bytes
```

Blender exited with code `1` four seconds after the interrupt. Note the file name: the whole frame range was baked into it, `movie_0001-0020.mp4`, at the moment the render started — before Blender knew it would not finish. The file existed from the start at 48 bytes and did not grow while the render ran.

Asked to open the file it had left, Blender 5.0.1 said:

```
file: movie_0001-0020.mp4 2423 bytes
OPENED. frame_duration = 3  size = (400, 400)  fps = 24.0
```

So a politely stopped video render leaves a file that opens and holds the frames finished before the stop — three of them here.

The same render, killed outright instead — no chance to tidy up — left this:

```
movie_0001-0020.mp4  48 bytes
```

and opening it gave `OPENED. frame_duration = 0`. Everything rendered was lost. Run on the Creator's MacBook Pro, M1 Pro chip, macOS 26.5.2. What an ordinary media player makes of either file was settled by hand test 30, recorded below.

**Observed — hand test 21, 2026-08-05.** With Autosplit Output switched on and the same interrupt twenty seconds in, Blender 5.0.1 left one file, `movie__0000001-0020.mp4`, 2423 bytes, which opened with `frame_duration = 3` — the same result as with Autosplit off, but under a different name: an extra underscore and seven-digit frame numbers instead of four. The render never came near the 2 GiB boundary, so what Autosplit does *at* that boundary when a render is interrupted is still not established; the reason it could not be reached is written into the hand-test list.

**Observed — hand test 30, run by the Creator, 2026-08-05.** An ordinary media player treats the two files completely differently, and "might become unplayable" turns out to be exactly right for one of them. The two files Blender 5.0.1 left — one from a render asked to stop politely twenty seconds in, one from the same render killed outright — were opened in QuickTime Player on the Creator's Mac.

The politely stopped one opened. The player window showed the rendered cube, and both time counters read `00:00`. The Creator's own words: "Part-way plays about half a second I think."

The killed one was refused, with this message:

> The document "killed outright — Blender 5.0.mp4" could not be opened.
>
> The file isn't compatible with QuickTime Player.

The Creator's written answer covered the part-way files; what the killed files did is taken from the screenshots they supplied, one per version, all four showing the same message with only the file name changed. Whether VLC was also tried is not recorded.

Run on the Creator's Mac — a MacBook Pro, M1 Pro chip, macOS 26.5.2 — so this settles macOS and leaves Windows and Linux open. It also leaves open what any player other than QuickTime does with either file.

**Observed — Linux hand tests 17 and 20, 2026-08-06.** The same two stops on a rented Linux machine — Ubuntu 22.04.5 LTS, NVIDIA GeForce RTX 4090, driver 580.105.08, twelve cores, run over SSH gave the same two opposite answers. The folder was watched twenty times a second: the video file appeared at 48 bytes within half a second of the render starting, named `movie_0001-0020.mp4` with the whole frame range baked into it, and did not grow while the render ran. Sent an interrupt twenty seconds in, it grew once, to 2277 bytes, and Blender 5.0.1 exited with code `1`. Asked to open what it had left:

```
file: movie_0001-0020.mp4 2277 bytes
OPENED. frame_duration = 2  size = (400, 400)  fps = 24.0
```

The same render killed outright instead left the file at its opening 48 bytes, and opening that gave `frame_duration = 0  size = (256, 256)  fps = 0.0`. With Autosplit switched on and the same interrupt below the boundary, one file was left, `movie__0000001-0020.mp4`, 2277 bytes, which opened with `frame_duration = 2` — the same under-a-different-name result as on the Mac.

**Observed — Linux hand test 28, 2026-08-06. The 2 GiB boundary itself, which the Mac could not reach.** The Mac could not get there because Blender's startup scene compresses to almost nothing. On the rented Linux machine two things were changed and nothing else: the picture was made out of high-frequency noise, so the encoder has nothing to compress away — a Noise Texture at scale 4000 and detail 15, looked up in window coordinates, feeding a Background node in the world — and the output was set to a lossless intra-frame codec, MKV with FFV1 and Lossless Output on, at 3840 by 2160 pixels with one Cycles sample. That is about 16 MB a frame, so the boundary is a hundred and twenty-odd frames away rather than tens of thousands.

Rendering 170 frames with Autosplit on, Blender 5.0.1 left **two files**. The first was closed at **2,028,653,135 bytes** and a second was started at that moment; the render carried on into it and finished normally, exit code 0, with the second file at 730,318,650 bytes. Two things fall out of that. The size at which the first file is closed is **below 2 GiB** (2,147,483,648 bytes) and above two thousand million bytes. And the second file's name carries a counter in its leading digits — `big__0010001-0170.mkv` against the first's `big__0000001-0170.mkv` — rather than the frame it starts at.

Then the same thing again with the render interrupted eight seconds after the second file appeared:

```
---- FOLDER AFTERWARDS ----
big__0000001-0300.mkv  2028653135 bytes
big__0010001-0300.mkv  32464519 bytes
```

and Blender opened both — the first with `frame_duration = 125`, the second with `frame_duration = 2`. So **the file that Autosplit closed at the boundary is whole and readable, and only the file being written when the stop arrived is short.** An interrupted Autosplit render does not lose everything: it loses only what went into the file that was open.

**One thing the two machines say differently, and it is about words rather than frames.** The Mac record above says this version prints no `Append frame` lines while encoding a video. On the rented Linux machine it prints such a line, worded `Video append frame 1`, `Video append frame 2` and so on, where Blender 4.5 prints `Append frame 1`. Both records stand. What is on the terminal is the only thing at issue; the file left behind is the same.

**Outside the interface.** The sources do not say. Nothing in these passages distinguishes a render started from the window from one started by `-a` on the command line. **Observed**, from hand tests 18 and 21: both were run on a Blender driven from a terminal with no window.

**Operating system.** The sources do not say. **Observed on two operating systems, and they agree** — macOS 26.5.2 (hand tests 18, 21 and 30, 2026-08-05) and Ubuntu 22.04.5 LTS on the rented Linux machine (Linux hand tests 17, 20 and 28, 2026-08-06). FFmpeg output with an MPEG-4 container and H.264 was available and worked on both. What a media player makes of a part-written file is settled for QuickTime Player on macOS only: the rented Linux machine has no media player installed at all and installing one was not permitted, so only Blender's own reader has opened these files there. **The 2 GiB Autosplit boundary itself (Linux hand test 28) is Linux only** — the Mac could not reach it at all, as the entry above says, so nothing about the split, the size the first file is closed at, the counter in the second file's name, or what an interruption after the split leaves is settled for macOS. *(Linux citation check, 2026-08-06: added by this check — Linux hand test 28 was listed among the tests the two machines agreed on, which reads as though the Mac had reached the boundary too.)* **Windows remains untested**, because no Windows machine has run it.

---

## 6. Is a file existing proof that the frame finished writing?

**Answer.** Not necessarily. Blender documents empty placeholder files while it is rendering frames. Reading that as an existing file before its frame has finished is an inference; the sources do not describe the timing of that file.

**Label: documented (that empty placeholder files are created while frames are rendering); inferred (that their existence is not proof a frame finished); not in the sources (whether a *normal* frame file can be caught half-written).**

> "use_placeholder — Create empty placeholder files while rendering frames (similar to Unix 'touch')"

- Source: Python API — https://docs.blender.org/api/5.0/bpy.types.RenderSettings.html
- Date read: 2026-07-30

> "Create empty placeholder frames while rendering."

- Source: manual — https://docs.blender.org/manual/en/5.0/render/output/properties/output.html
- Date read: 2026-07-30

So with Placeholders on — which is what the manual's render-farm hint tells you to do — a file at a frame's path may hold nothing at all. Applying the placeholder wording to a particular frame path is **inferred**. Default is off (`boolean, default False`).

What the sources *do* offer as evidence that a frame finished:

> "bpy.app.handlers.render_write — on writing a render frame (directly after the frame is written)"

- Source: Python API — https://docs.blender.org/api/5.0/bpy.app.handlers.html
- Date read: 2026-07-30

and a log line, shown in the 5.0 release notes' example of the new logging output:

> "00:28.631  render           | Saved: '/tmp/0001.png'"

with the surrounding note:

> "Most of Blender now uses the same logging system, including operators reports and background render progress. The output format has changed, and log levels are easier to control."

- Source: release notes — https://developer.blender.org/docs/release_notes/5.0/core/
- Date read: 2026-07-30

The Python API release notes flag the consequence:

> "The format of logging output has changed, including background render progress. Add-ons or render farms parsing this output may need to be updated."

- Source: release notes — https://developer.blender.org/docs/release_notes/5.0/python_api/
- Date read: 2026-07-30

One more thing that can put an extra file beside a frame: for OpenEXR output,

> "Preview — When rendering animations (or single frames via command line), save a JPEG copy of the image, for a quick preview."

- Source: manual — https://docs.blender.org/manual/en/5.0/files/media/image_formats.html
- Date read: 2026-07-30

**Not covered.** The sources do not say whether a frame is written to a temporary name and renamed, or written in place; so whether a file can be observed part-written is not settled. They do not say whether a placeholder is later overwritten in place by the real frame, or replaced. They do not say how big a placeholder is beyond "empty". They do not give a documented way, from outside Blender, to tell a finished frame from a placeholder other than parsing the log — and the log format is stated to have changed in 5.0.

**Observed — hand test 3, 2026-08-05.** With Placeholders on, a zero-byte file for a frame exists before that frame has rendered, and the placeholders are made one at a time rather than all at the start. Blender 5.0.1 was started from a terminal with no window, rendering the default cube scene in Cycles at 200 samples and 200 by 200 pixels to PNG, frames 1 to 3, Placeholders on, Overwrite off, with a script listing the output folder at each of Blender's own render events:

```
WATCH [before the render starts] folder holds: (nothing)
WATCH [render_pre, frame 1 about to render] folder holds: f_0001.png=0 bytes
WATCH [render_post, frame 1 finished rendering] folder holds: f_0001.png=34735 bytes
WATCH [render_write, frame 1 written] folder holds: f_0001.png=34735 bytes
WATCH [render_pre, frame 2 about to render] folder holds: f_0001.png=34735 bytes, f_0002.png=0 bytes
WATCH [render_post, frame 2 finished rendering] folder holds: f_0001.png=34735 bytes, f_0002.png=34735 bytes
WATCH [render_write, frame 2 written] folder holds: f_0001.png=34735 bytes, f_0002.png=34735 bytes
WATCH [render_pre, frame 3 about to render] folder holds: f_0001.png=34735 bytes, f_0002.png=34735 bytes, f_0003.png=0 bytes
WATCH [render_post, frame 3 finished rendering] folder holds: f_0001.png=34735 bytes, f_0002.png=34735 bytes, f_0003.png=34735 bytes
WATCH [render_write, frame 3 written] folder holds: f_0001.png=34735 bytes, f_0002.png=34735 bytes, f_0003.png=34735 bytes
WATCH [render_complete] folder holds: f_0001.png=34735 bytes, f_0002.png=34735 bytes, f_0003.png=34735 bytes
```

That also answers the timing the answer above records as not in the sources: the placeholder is made before the frame's own work begins, and the finished frame replaces it in place at the same path.

**Observed — hand test 4, 2026-08-05.** With Placeholders off, no file exists for the frame in progress at the moment it starts:

```
WATCH [before the render starts] folder holds: (nothing)
WATCH [render_pre, frame 1 about to render] folder holds: (nothing)
WATCH [render_post, frame 1 finished rendering] folder holds: f_0001.png=34735 bytes
WATCH [render_write, frame 1 written] folder holds: f_0001.png=34735 bytes
WATCH [render_pre, frame 2 about to render] folder holds: f_0001.png=34735 bytes
```

But a file can still be caught part-written. The same test was run against a much larger file — Cycles at 1000 samples and 1000 by 1000 pixels, OpenEXR output, about 32 seconds a frame — with the folder watched twenty times a second. Blender 5.0.1 was caught mid-write:

```
  0.00s  (nothing)
 32.10s  f_0001.exr=5448 bytes
 32.16s  f_0001.exr=477472 bytes
 63.84s  f_0001.exr=477472 bytes, f_0002.exr=477472 bytes
```

The file appeared at 5448 bytes and reached its finished 477472 bytes 60 milliseconds later. So with Placeholders off a file can exist, hold a non-zero size, and still be incomplete. The write is fast, not atomic, and Blender does not write to a temporary name and rename. Taking this with the placeholder result: neither a file's existence nor a non-zero size proves the frame finished.

Run on the Creator's MacBook Pro, M1 Pro chip, macOS 26.5.2.

**Observed — hand tests 6, 7, 5 and 33, 2026-08-05.** Two more things bearing on this answer, recorded in full elsewhere in this document. A render stopped at any point leaves no file at all for the frame in flight, whether interrupted, terminated politely, or killed — so a placeholder is not left behind by a cancel either (questions 2 and 3). And two Blenders sharing a folder with Placeholders on both rendered frames 1 and 2 in the first run and frame 1 in the re-run, so a placeholder does not reliably reserve a frame (question 4).

**Observed — Linux hand tests 3 and 4, 2026-08-06.** Both halves came back the same on a rented Linux machine — Ubuntu 22.04.5 LTS, NVIDIA GeForce RTX 4090, driver 580.105.08, twelve cores, run over SSH. With Placeholders on, Blender 5.0.1 there gave the same sequence of listings: nothing in the folder, then `f_0001.png=0 bytes` as frame 1 begins, then the same name at 34699 bytes when it finishes, then `f_0002.png=0 bytes` as frame 2 begins. With Placeholders off, no file at all exists for the frame in progress.

The part-written half is worth setting out carefully, because the two machines caught different versions. Watching a folder twenty times a second through a slow 1000-by-1000 OpenEXR render, the Linux watcher recorded for this version:

```
  0.00s  (nothing)
 38.30s  f_0001.exr=0 bytes
 38.35s  f_0001.exr=477459 bytes
 80.95s  f_0001.exr=477459 bytes, f_0002.exr=477459 bytes
```

**and 5.0 was caught with the file at 0 bytes** fifty milliseconds before it reached its finished size. On the Mac it was 5.0 that was caught, at 5,448 bytes. Taking the two machines together: with Placeholders off, an ordinary frame file **can** be seen at a size that is not its finished size, on both operating systems, and which version a watcher happens to catch is luck rather than a difference between versions. Neither the existence of a file nor a non-zero size proves the frame finished — and a zero-byte file is not proof of a placeholder either.

**Observed — Linux hand test 27, 2026-08-06.** Twelve runs of two Blenders sharing a folder with Placeholders on, on the rented Linux machine, showed no frame rendered twice — unlike the Mac. That is recorded in full under question 4, and it does not change the answer here: what it settles is how reliably a placeholder reserves a frame, not whether a file's existence proves a frame finished.

**Outside the interface.** The `render_write` handler is a Python handler; that is documented. That it runs in background mode is **not stated**. The release notes explicitly say background render progress goes through the same logging. Whether the `Saved:` line appears identically in background mode is **inferred** from the sentence "including background render progress" — the example shown is not labelled as background. **Observed**, from hand tests 3 and 4: `render_write` does fire in background mode — every `WATCH [render_write …]` line above was printed by such a handler in a Blender with no window — and the `Saved:` line does appear there, in the form `00:00.545  render           | Saved: '…/f_0001.png'`.

**Operating system.** The sources do not say. The `touch` comparison in the placeholder tooltip is a description, not a limit. **Observed on two operating systems, and they agree** — macOS 26.5.2 (hand tests 3 and 4, 2026-08-05) and Ubuntu 22.04.5 LTS on the rented Linux machine (Linux hand tests 3 and 4, 2026-08-06). Placeholders makes empty files on both, and an ordinary frame file can be caught part-written on both. **Windows remains untested**, because no Windows machine has run it.

---

## 7. Can Blender be given a set of frames that is not one plain run — single frames, or two separate stretches?

**Answer.** From the command line, yes: single frames, comma-separated lists of frames, and ranges written with `..`. The documented Frame Range panel offers Start, End and Step; the sources do not document a list there.

**Label: documented (command line and the documented panel); not in the sources (whether another Properties-editor control offers a list).**

> "-f, --render-frame <frame>
>
> Render frame <frame> and save it.
>
> +<frame> start frame relative, -<frame> end frame relative.
>
> A comma separated list of frames can also be used (no spaces).
>
> A range of frames can be expressed using .. separator between the first and last frames (inclusive)."

- Source: manual — https://docs.blender.org/manual/en/5.0/advanced/command_line/arguments.html
- Date read: 2026-07-30

Against that, the panel:

> "Frame Start, End — Set the Start and End frames for Rendering Animations."
>
> "Step — Controls the number of frames to advance for each frame rendered, a value above 1 skips frames."

- Source: manual — https://docs.blender.org/manual/en/5.0/render/output/properties/frame_range.html
- Date read: 2026-07-30

The picture on that page, `manual_5.0/_images/render_output_properties_frame_range_panel.png` ("Frame Range panel."), shows exactly three fields — Frame Start `1`, End `250`, Step `1` — and a collapsed Time Stretching section. Nothing resembling a list. **Label: documented.** Read 2026-07-30.

From Python, one range at a time:

> "bpy.ops.render.render(*, animation=False, write_still=False, use_viewport=False, use_sequencer_scene=False, layer='', scene='', frame_start=0, frame_end=0)"
>
> "frame_start … Frame to start rendering animation at. If not specified, the scene start frame will be assumed. This should only be specified if doing an animation render"

- Source: Python API — https://docs.blender.org/api/5.0/bpy.ops.render.html
- Date read: 2026-07-30

These two arguments are new in this version:

> "The `render.render()` operator now supports `frame_start` and `frame_end` optional arguments."

- Source: release notes — https://developer.blender.org/docs/release_notes/5.0/python_api/
- Date read: 2026-07-30

And a correction to them is recorded:

> "API bpy.ops.render.render() uses wrong frame range. (blender/blender#150125)"

listed under Blender 5.0.1, "Released on December 16th 2025, Blender 5.0.1 features 132 bug fixes".

- Source: release notes — https://developer.blender.org/docs/release_notes/5.0/corrective_releases/
- Date read: 2026-07-30

**Ambiguity, flagged rather than resolved.** The question's "two separate stretches" reads most naturally as: can one render command be given two disjoint ranges, e.g. 1–10 and 50–60. The sources document a comma-separated list of frames, and separately document `..` for a range. They do **not** state that the two notations may be mixed in one `-f` argument (something like `-f 1..10,50..60`). I answer the reading that a set of arbitrary single frames is certainly expressible as a comma list; whether two stretches can be given in one argument is **not in the sources**. The other reading — two stretches meaning two successive render commands — is trivially yes, since each `-f` renders what it is given, but that is not a single set. Where I looked for this: the whole of the command line arguments page; searches of the manual and API for "comma", "separated", "range of frames", "frame list", and of the release notes for "render-frame", "render_frame", "frame-jump" and "command line".

**Not covered.** Nothing states in what order a comma list is rendered (see question 10). Nothing states whether duplicates in a list are rendered twice. Nothing states an upper limit on list length. The interface's Frame Range panel has no list at all, so nothing covers a non-contiguous set stored in the `.blend` file. The Preview Range, which might look like a second range, is explicitly not one for rendering: "This range only affects the preview in the 3D Viewport; it doesn't affect rendering." (manual — https://docs.blender.org/manual/en/5.0/animation/animation_editors.html, read 2026-07-30).

A related but different facility, for viewport renders only:

> "To render an animation, but only those frames that have a keyframe, use 3D Viewport ‣ View ‣ Render Playblast on Keyframes. This only renders those frames for which the selected objects have an animation key. The other frames are still written to the output, but will simply repeat the last-rendered frame."

- Source: manual — https://docs.blender.org/manual/en/5.0/editors/3dview/viewport_render.html
- Date read: 2026-07-30

This does not give a sparse *set* — every frame is still written.

**Observed — hand tests 8, 9, 10, 11 and 20, 2026-08-05.** The ambiguity flagged above is settled: the two notations *can* be mixed in one `-f`. All five tests were run with Blender 5.0.1 started from a terminal, rendering the default cube scene to PNG into an empty folder each time.

*A list mixing single frames and stretches, `-f 1..3,10..12`.* Accepted, no error:

```
00:00.271  render           | Rendering single frame
00:00.271  render           | Rendering frame 1
00:00.395  render           | Saved: '…/g_0001.png'
00:00.395  render           | Rendering single frame
00:00.395  render           | Rendering frame 2
00:00.401  render           | Saved: '…/g_0002.png'
00:00.401  render           | Rendering single frame
00:00.401  render           | Rendering frame 3
00:00.406  render           | Saved: '…/g_0003.png'
00:00.406  render           | Rendering single frame
00:00.406  render           | Rendering frame 10
00:00.411  render           | Saved: '…/g_0010.png'
00:00.411  render           | Rendering single frame
00:00.411  render           | Rendering frame 11
00:00.415  render           | Saved: '…/g_0011.png'
00:00.416  render           | Rendering single frame
00:00.416  render           | Rendering frame 12
00:00.421  render           | Saved: '…/g_0012.png'
```

All six files were written. So two separate stretches are expressible in one `-f`. Note also that Blender treats each frame in the list as its own single-frame render — it prints `Rendering single frame` before each one, never `Rendering animation`.

*A list of separate frames, `-f 5,1,9`.* Rendered in the order written — 5, then 1, then 9 — not sorted. Files `f_0001.png`, `f_0005.png`, `f_0009.png`.

*A repeated frame, `-f 2,2,3`.* Frame 2 was rendered twice, over the same file. Two files were left. Duplicates are neither rejected nor collapsed.

*Malformed lists.* Each was refused, nothing was rendered, and in every case Blender's exit code was `0`:

```
Error: incorrect comma use '-f / --render-frame 1,,3'.
Warning: negative range ignored '-f / --render-frame 3..1'.
Error: not a number '-f / --render-frame 1-3'.
```

*With Blender's window open.* The same `-f 1..3,10..12` command without `-b`, so Blender opened its window, wrote the same six files with the same names.

Run on the Creator's MacBook Pro, M1 Pro chip, macOS 26.5.2.

**Observed — hand tests 24, 26 and 27, 2026-08-05.** Three more things, two of which this answer names as unsettled. Same machine and date.

*An upper limit on list length.* A list of five thousand frames was accepted: `-f` with `1,2,…,5000` rendered all five thousand and Blender exited with code `0`. A list of two hundred thousand never reached Blender — the shell refused it first, with `argument list too long`, and returned `127`. So the limit is the operating system's argument length rather than anything in Blender.

*Two separate stretches from one command.* `-s 1 -e 2 -a -s 20 -e 21 -a` produced two separate animation runs and four files:

```
00:00.262  render           | Rendering animation (frames 1..2)
00:00.394  render           | Saved: '…/m_0001.png'
00:00.399  render           | Saved: '…/m_0002.png'
00:00.399  render           | Rendering animation (frames 20..21)
00:00.404  render           | Saved: '…/m_0020.png'
00:00.408  render           | Saved: '…/m_0021.png'
```

*The keyframes-only playblast quoted above cannot be reached with no window.* Calling `bpy.ops.render.opengl(animation=True, render_keyed_only=True)` in a Blender 5.0.1 started with no window gave:

```
Error: Cannot use OpenGL render in background mode (no opengl context)
RAISED: RuntimeError Error: Cannot use OpenGL render in background mode (no opengl context)
FILES: []
```

**Observed — hand test 32, run by the Creator, 2026-08-05.** With a window, that playblast does run, and the manual's own account of what it leaves on disk holds: it does not leave gaps. In Blender 5.0.1 the Creator took the default cube scene, put a keyframe on the cube at frame 3 and another at frame 5 and none anywhere else, set the frame range to 1–6 and an image output path, and used the keyframes-only item in the 3D Viewport's View menu. Their own words: "Anyway, all versions create 6 files, which are identical."

So six files for a six-frame range, even though only two frames carry a key. What the six files were named, and how big they were, is not recorded.

The menu in Blender 5.0.1 reads:

> Render Viewport Preview
> Render Playblast
> Render Playblast on Keyframes

with the tooltip "Take a snapshot of the active viewport." and, underneath it, "Python: bpy.ops.render.opengl()". (Blender 4.5.12 labels the same three items "Viewport Render Image", "Viewport Render Animation" and "Viewport Render Keyframes"; 5.1.2 and 5.2.0 match 5.0.1.)

Run on the Creator's Mac — a MacBook Pro, M1 Pro chip, macOS 26.5.2 — so this settles macOS and leaves Windows and Linux open.

**Observed — Linux hand tests 7 to 10, 19, 23, 24, 25 and 26, 2026-08-06.** Every one of the tests above was run again on a rented Linux machine — Ubuntu 22.04.5 LTS, NVIDIA GeForce RTX 4090, driver 580.105.08, twelve cores, run over SSH with Blender 5.0.1 and came back the same. `-f 5,1,9` gave 5, then 1, then 9. `-f 1..3,10..12` wrote all six files with no error. `-f 2,2,3` rendered frame 2 twice over the same file. The three malformed lists were refused with the same three messages, word for word, nothing rendered, exit code 0 every time. The same list with a window open wrote the same six files. `-s 1 -e 2 -a -s 20 -e 21 -a` wrote four files. The keyframes-only playblast with no window was refused with the same message, word for word. And the same list under Cycles gave the same order as under Workbench.

The one difference is in how the operating system refuses a list that is too long. Five thousand frames were accepted and all five thousand rendered, exit code 0, on both machines. Two hundred thousand never reached Blender on either. On the Mac the shell said `argument list too long` and returned `127`; on the rented Linux machine it said `Argument list too long` and returned `126`. **So the limit is the operating system's on both machines, and the two operating systems refuse it differently.**

**Observed — Linux hand test 34, 2026-08-06.** The keyframes-only playblast was run with a window, on the rented Linux machine's own display. Blender 5.0.1, given the startup scene with a keyframe on the cube's location at frame 3 and another at frame 5 and none anywhere else, the cube selected and active, and a frame range of 1 to 6, was asked for `bpy.ops.render.opengl(animation=True, render_keyed_only=True)` with the 3D Viewport as its context — the same operator the View menu's item runs. It returned `{'FINISHED'}` and wrote **six files for a six-frame range**, `p_0001.png` to `p_0006.png`, every one of them 37198 bytes. That agrees with the Creator's macOS result. Where it does not agree: the Creator reported the six files as identical, and compared byte for byte here **no two of the six are the same file** — all fifteen pairs came back different, in every version, while every file is the same size. Looked at, they are the same picture. Both records stand; the eye and a byte comparison are answering different questions.

**Observed — Linux hand tests 35 to 38, 2026-08-06.** Four things about `-f` that neither machine had tried before, run on the rented Linux machine only, so **the Mac is untested for all four**. In Blender 5.0.1, and identically in the other three versions:

- **`-f` may be given twice in one command.** `-f 1..3 -f 10..12` rendered both stretches — six files, exit code 0.
- **`-f` and `-a` may be mixed in one command.** `-s 1 -e 2 -a -f 20` carried out both — three files, frames 1, 2 and 20, exit code 0.
- **Step has no effect on a `-f` list.** With the scene's step set to 3, which read back as 3, `-f 1,2,3,4,5` wrote all five files.
- **Whitespace inside a list is accepted.** `-f "1, 2, 3"` rendered frames 1, 2 and 3 and wrote three files, exit code 0, even though the manual says "no spaces". That says what this machine took, not that the manual is wrong about what is supported.

**Outside the interface.** The list-and-range syntax is documented as a command-line argument. The sources do not document an equivalent in the Properties editor. **Observed**, from hand test 20: `-f` with a list behaves the same whether or not Blender opens a window. **Observed**, from hand test 26: the keyframes-only playblast is the other way round — it cannot run at all without a window.

**Operating system.** The sources do not say. **Observed on two operating systems, and they agree on everything the question asks** — macOS 26.5.2 (hand tests 8 to 11, 20, 24, 26, 27 and 32, 2026-08-05) and Ubuntu 22.04.5 LTS on the rented Linux machine (Linux hand tests 7 to 10, 19, 23, 24, 25, 26 and 34, 2026-08-06). What differs is not Blender's: how long a list the operating system will pass on, and what it says when it will not. The four further `-f` behaviours in Linux hand tests 35 to 38 are **Linux only**; the Mac was never asked. **Windows remains untested**, because no Windows machine has run it.

---

## 8. What does a frame range consist of?

**Answer.** The documented range has three numbers: a start frame, an end frame, and a step. The command-line animation command says its end is included; applying that to every use of the scene range is inferred.

**Label: documented (the three values and command-line animation); inferred (that the scene range is always end-inclusive).**

> "This panel defines how long an animation will last in terms of frames."
>
> "Frame Start, End — Set the Start and End frames for Rendering Animations."
>
> "Step — Controls the number of frames to advance for each frame rendered, a value above 1 skips frames. This is also used when viewing rendered animations."

- Source: manual — https://docs.blender.org/manual/en/5.0/render/output/properties/frame_range.html
- Date read: 2026-07-30

The picture on the same page, `manual_5.0/_images/render_output_properties_frame_range_panel.png`, shows the three fields with values 1, 250, 1. **Label: documented.** Read 2026-07-30.

Exact kinds, defaults and ranges, from the API:

> "frame_start — First frame of the playback/rendering range — Type: int in [0, 1048574], default 1"
>
> "frame_end — Final frame of the playback/rendering range — Type: int in [0, 1048574], default 250"
>
> "frame_step — Number of frames to skip forward while rendering/playing back each frame — Type: int in [0, 1048574], default 1"

- Source: Python API — https://docs.blender.org/api/5.0/bpy.types.Scene.html
- Date read: 2026-07-30

That the end is included:

> "-a, --render-anim — Render frames from start to end (inclusive)."

- Source: manual — https://docs.blender.org/manual/en/5.0/advanced/command_line/arguments.html
- Date read: 2026-07-30

The same three exist as command-line arguments: `-s, --frame-start <frame>`, `-e, --frame-end <frame>` ("supports +/- for relative frames too") and `-j, --frame-jump <frames>` — "Set number of frames to step forward after each rendered frame" (same page, same date).

Cycles' own files treat the range the same way when they walk it:

> `            for frame in range(scene.frame_start, scene.frame_end + 1):`

- Source file: `Blender Sources/cycles_5.0/operators.py`, line 79
- **Label: observed.** Read 2026-07-30. Note this is Cycles' denoise operator, not the renderer; it shows how Cycles' own code reads the range (`+ 1`, i.e. end inclusive), not what the render loop does.

Two things that sit in or beside the panel but are not part of the range: Time Stretching, of which the manual warns "Using Time Stretching will not influence the Start or End frames set above" (same manual page); and the Preview Range, which "only affects the preview in the 3D Viewport; it doesn't affect rendering" (manual — https://docs.blender.org/manual/en/5.0/animation/animation_editors.html, read 2026-07-30). The API for that: `use_preview_range` — "Use an alternative start/end frame range for animation playback and view renders", boolean, default `False`; with `frame_preview_start` and `frame_preview_end`, both "int in [-inf, inf], default 0" (Python API — https://docs.blender.org/api/5.0/bpy.types.Scene.html, read 2026-07-30).

**Not covered.** The sources do not say what a Step of 0 does, though 0 is inside the documented range `[0, 1048574]`. They do not say what happens when End is below Start. They do not say whether the last frame of a stepped run is rendered when the step overshoots the end. `frame_end` is documented as the "final frame of the playback/rendering range" but the inclusive wording comes only from the command-line page, which is about `-a`.

**Observed — hand tests 12, 13 and 14, 2026-08-05.** All three were run with Blender 5.0.1 started from a terminal with no window, rendering the default cube scene to PNG into an empty folder.

*A step that lands exactly on the end frame.* Start 1, end 10, step 3, given as `-j 3` on the command line, produced files for frames 1, 4, 7 and 10.

*A step that overshoots the end frame.* Start 3, end 12, step 4, set as scene settings, produced files for frames 3, 7 and 11 — and the run announced itself as `Rendering animation (frames 3..12)`. Frame 12, the end frame, was not rendered. So a stepped run walks forward from the start frame and stops at the last frame at or below the end; the end frame is not added on.

*A step of 0.* The property accepted it and read back as `0`. Blender rendered and wrote frame 1 once, then announced frame 1 four more times without rendering or writing anything, and ended on its own with exit code `0`:

```
00:00.268  render           | Rendering animation (frames 1..5)
00:00.268  render           | Rendering frame 1
00:00.268  render           | Start rendering: Scene, ViewLayer
00:00.268  render           | Engine: Workbench
00:00.400  render           | Saved: '…/f_0001.png'
00:00.400  render           | Time: 00:00.13 (Saving: 00:00.04)
00:00.400  render           | Rendering frame 1
00:00.400  render           | Rendering frame 1
00:00.400  render           | Rendering frame 1
00:00.400  render           | Rendering frame 1
frame_step actually reads back as: 0
```

One file was left, `f_0001.png`, 3639 bytes. It did not hang.

*An end below the start.* Start 5, end 2 announced itself as `Rendering animation (frames 5..2)`, rendered nothing at all, and exited with code `0`. The folder was empty afterwards.

Run on the Creator's MacBook Pro, M1 Pro chip, macOS 26.5.2.

**Observed — hand test 28, run by the Creator, 2026-08-05.** The panel accepts a step of 0 and shows it, with no complaint. In Blender 5.0.1 the Creator opened Output Properties, the Frame Range panel, typed `0` into the Step field and pressed Enter. Their own words: "Typing 0 in the Step field doesn't do anything — no error, warning or any visible behavior. Right-clicking the panel header does nothing. The panel itself is in the screenshot. I only took one because it's the same in all 4 versions."

The screenshot shows the panel holding, in order: **Frame Start** `1`, **End** `250`, **Step** `0`, and an open **Time Stretching** section with **Old** `100` and **New** `100`. Nothing else. So the Step field keeps the 0 rather than snapping to 1, the panel's whole contents are those five numbers, and the two Time Stretching values are the `frame_map_old` and `frame_map_new` pair under their interface names.

Read alongside the run above: the field takes 0, and a render made with it writes the first frame once.

Run on the Creator's Mac — a MacBook Pro, M1 Pro chip, macOS 26.5.2 — so this settles macOS and leaves Windows and Linux open.

**Observed — Linux hand tests 11, 12 and 13, 2026-08-06.** The same three runs on a rented Linux machine — Ubuntu 22.04.5 LTS, NVIDIA GeForce RTX 4090, driver 580.105.08, twelve cores, run over SSH gave the same frames in Blender 5.0.1. Start 1, end 10, step 3 gave files for 1, 4, 7 and 10. Start 3, end 12, step 4 gave 3, 7 and 11, and did not render frame 12. A step of 0 read back as `0`, wrote frame 1 once, left one file of 3566 bytes, and did not hang. An end below the start rendered nothing, printed `Rendering animation (frames 5..2)` and stopped, exit code 0.

One thing the Mac run did not record. The printed output announces each stepped frame once per step — `Rendering frame 4` three times over for a step of 3 — and the 3-to-12 run ends with a bare `Rendering frame 15`, past the end frame, for which nothing is rendered and nothing is written.

**Observed — Linux hand test 31, 2026-08-06.** The Frame Range panel was photographed on the rented Linux machine's own display with Blender's own `bpy.ops.screen.screenshot`. The panel holds **Frame Start** `1`, **End** `250`, **Step** `1`, and a collapsed **Time Stretching** section, and nothing else. With the step set to 0 the field then reads **`0`** — the panel keeps the 0 and does not snap back to 1. That agrees with what the Creator reported on the Mac.

Two limits on that run, said plainly. The step was set from Python rather than typed into the field, so what the field does with typed input rests on the Creator's macOS run alone. And the panel header was not right-clicked. Both need the pointer put on a control, and on that machine the display is shared with other work: a click aimed at a control was followed by asking Blender for that control's state, and the state had not changed, so no click could be shown to land. Nothing that could not be demonstrated has been written down.

**Observed — Linux hand test 31 again, 2026-08-06, opened by operator rather than by hand.** The panel was then opened as a popup by Blender's own operator, which needs no pointer at all. `bpy.ops.wm.call_panel(name="RENDER_PT_frame_range")` returned `{'INTERFACE'}`, and the picture it was photographed in shows the whole panel with its Time Stretching section open:

> Frame Start `1`
> End `250`
> Step `0`
> Time Stretching
> Old `100`
> New `100`

Nothing else. That is exactly what the Creator reported from the Mac, and it settles the panel's whole contents on this machine as well. The panel identifier was read out of `bpy.types` rather than guessed: both `RENDER_PT_frame_range` and `RENDER_PT_time_stretching` are there.

**Observed — Linux hand tests 37 and 39, 2026-08-06.** Two things about the range that neither machine had tried, run on the rented Linux machine only. **Step has no effect on a `-f` list**: with the scene's step set to 3, `-f 1,2,3,4,5` wrote all five files. And **the alternative playback range does not change what a real render writes**: with the scene range 1 to 8, an alternative playback range of 4 to 5 and that range switched on, a plain animation render announced `Rendering animation (frames 1..8)` and wrote all eight frames.

**Outside the interface.** The same three values are scene properties and the same three exist as command-line arguments, so the range is expressible both ways. Documented. The command-line forms additionally accept relative values (`+`/`-`), which the panel does not offer. **Observed**, from hand tests 12, 13 and 14: the step behaves identically whether it is given as `-j` on the command line or as the scene's own setting.

**Operating system.** The sources do not say. **Observed on two operating systems, and they agree** — macOS 26.5.2 (hand tests 12, 13, 14 and 28, 2026-08-05) and Ubuntu 22.04.5 LTS on the rented Linux machine (Linux hand tests 11, 12, 13 and 31, 2026-08-06). The same three stepped runs give the same frames on both, and the panel shows a step of 0 on both. **Linux hand tests 37 and 39 — Step against a `-f` list, and the alternative playback range — are Linux only**; the Mac was never asked. *(Linux citation check, 2026-08-06: added by this check.)* **Windows remains untested**, because no Windows machine has run it.

---

## 9. What does a frame write when several view layers are active on it?

**Answer.** All view layers that are enabled are rendered before the frame is finished. What lands on disk is one result for the frame, not one file per view layer — with the exception of the multi-layer EXR route, where layers and passes are kept inside a single file. I have to mark the second half of that as inferred: the sources describe the pieces but never state the whole in one place.

**Label: inferred.** The step is set out below.

What is documented outright:

> "render_frame_finish() — Perform finishing operations after all view layers in a frame were rendered"

- Source: Python API — https://docs.blender.org/api/5.0/bpy.types.RenderEngine.html
- Date read: 2026-07-30

> "use — Enable or disable rendering of this View Layer — Type: boolean, default True"

- Source: Python API — https://docs.blender.org/api/5.0/bpy.types.ViewLayer.html
- Date read: 2026-07-30

> "Use for Rendering — The active view layer will be used during rendering.
>
> Render Single Layer — Only render the active view layer.
>
> Note
>
> This option is ignored when rendering from the command line."

- Source: manual — https://docs.blender.org/manual/en/5.0/render/layers/view_layer.html
- Date read: 2026-07-30

> "use_single_layer — Only render the active layer. Only affects rendering from the interface, ignored for rendering from command line. — Type: boolean, default False"

- Source: Python API — https://docs.blender.org/api/5.0/bpy.types.RenderSettings.html
- Date read: 2026-07-30

What the frame's file is:

> "Media Type — The type of media to save.
>
> Image: Saves each input as a separate image file. Each input can use its own image format settings.
>
> Multi-Layer EXR: Saves all inputs together in a single multi-layer OpenEXR file.
>
> Video: Encodes each frame into a video container."

and, separately on the same panel:

> "Saving – Cache Result — Saves the rendered view layers and their passes to a multi-layer OpenEXR image."

- Source: manual — https://docs.blender.org/manual/en/5.0/render/output/properties/output.html
- Date read: 2026-07-30

And what the single result is, when compositing is on:

> "In the Compositor, only the first image input is used as the final render result. Any additional sockets will be ignored."

> "The Group Output node defines the final output of a node group. It determines which values are passed out of the node group to the parent node tree (or to the renderer, in the case of the Compositor). In the Compositor, this node defines the image result that is sent to the renderer or to image output after rendering."

- Source: manual — https://docs.blender.org/manual/en/5.0/compositing/types/output/group_output.html
- Date read: 2026-07-30

> "Pipeline – Compositing — Renders the output from the compositing node setup, and then applies the Composite node tree on all images, displaying the image inputted in the Group Output node."

- Source: manual — https://docs.blender.org/manual/en/5.0/render/output/properties/post_processing.html
- Date read: 2026-07-30

The release notes record the change that put the Group Output in that position:

> "The **Composite** node was removed and replaced by a Group Output node whose first input needs to be a Color socket, and other inputs are ignored."

- Source: release notes — https://developer.blender.org/docs/release_notes/5.0/compositor/
- Date read: 2026-07-30

**The step I took.** From "after all view layers in a frame were rendered" I take that a frame renders every enabled view layer before it is done. From "only the first image input is used as the final render result" I take that the frame nonetheless resolves to one image, no matter how many view layers fed into it, when compositing is on. From "Saves the rendered view layers and their passes to a multi-layer OpenEXR image" I take that the multi-layer EXR is the format that keeps view layers separate on disk. A reader has to supply the joins between those three sentences, so the label is inferred, not documented.

**Not covered.** This is the weakest answer on the sheet and I want to be plain about it. The sources never say, in one sentence, what a frame writes when a scene has several view layers and the output format is an ordinary image such as PNG. In particular: whether one file is written or several; whether a per-view-layer name is appended if several; and what happens when compositing is switched off in Post Processing so there is no Group Output to pick a single image. The Output panel's Media Type wording talks about "inputs" without saying, on that page, what an input is for the scene's own output (the identical wording appears for the File Output node, where inputs are node sockets). The picture of the Output panel in the manual (`manual_5.0/_images/render_output_properties_output_panel.png`) does not show a Media Type row at all, so it cannot settle it either. Where I looked: the whole of `render/output/`, `render/layers/`, `compositing/` (introduction, compositor_system, types/output, types/input/scene), `editors/image/introduction`, and searches across manual and API for "multi-layer", "multilayer", "view layer" and "Render Result".

**Observed — hand tests 15, 16 and 17, 2026-08-05.** This settles the weakest answer on the sheet, and the answer is simpler than the reasoning above. Blender 5.0.1 was started from a terminal with no window. A script moved the default cube into its own collection and added a second view layer, `Second`, that leaves that collection out — so the two view layers plainly show different things, one with the cube and one without. Both had `use` set to True. One frame was rendered four ways, and the written files were loaded back and compared pixel by pixel:

```
VIEW LAYER 'ViewLayer'  use=True
VIEW LAYER 'Second'  use=True
A (both layers)       == C (first layer alone) : True
A (both layers)       == D (second layer alone): False
B (single-layer flag) == A (both layers)       : True
C == D (the two layers look different at all)  : False
A_both_0001.png  26972 bytes
B_single_0001.png  26972 bytes
C_firstonly_0001.png  26972 bytes
D_secondonly_0001.png  22991 bytes
```

Three answers come out of that. **One image file is written per frame** with these two view layers both enabled — never one per layer, and no view-layer name enters the file name. *(Citation check, 2026-08-05: this read "whatever the number of view layers"; the test used exactly two, so it has been narrowed to two.)* **Its pixels are the first view layer's alone**: identical to the same frame rendered with only that layer enabled, and different from the same frame rendered with only the second layer enabled. And **Render Single Layer changes nothing** in a render driven from outside the window — the file written with it on is pixel-for-pixel the file written with it off. That is what the documented "ignored when rendering from the command line" note amounts to in practice, and it means the inference recorded above — that ignoring the switch yields *all* enabled view layers in the written file — does not hold for the file: whatever is rendered, one layer's image is what lands on disk.

**Observed — hand test 16, 2026-08-05.** The multi-layer OpenEXR route does keep both view layers, in one file. In Blender 5.0.1 the media-type setting is how it is reached: with `media_type` set to `'MULTI_LAYER_IMAGE'`, the file-format list narrows to `('OPEN_EXR_MULTILAYER')` alone. With `media_type` left at `'IMAGE'` the version's full file-format list is:

```
['JPEG', 'OPEN_EXR', 'PNG', 'WEBP', 'BMP', 'CINEON', 'DPX', 'IRIS', 'JPEG2000',
 'HDR', 'TARGA', 'TARGA_RAW', 'TIFF', 'OPEN_EXR_MULTILAYER', 'FFMPEG']
```

(Blender 4.5.12 has no `media_type` setting at all; there the multi-layer format is simply one entry in the one list.) The render wrote one file, `vl_0001.exr`, 17927 bytes, and the names found inside its header were:

```
ViewLayer.Combined.A, ViewLayer.Combined.B, ViewLayer.Combined.G, ViewLayer.Combined.R,
Second.Combined.A, Second.Combined.B, Second.Combined.G, Second.Combined.R
```

Both view layers, named, inside the one file.

Run on the Creator's MacBook Pro, M1 Pro chip, macOS 26.5.2.

**Observed — Linux hand tests 14, 15 and 16, 2026-08-06.** The same three runs on a rented Linux machine — Ubuntu 22.04.5 LTS, NVIDIA GeForce RTX 4090, driver 580.105.08, twelve cores, run over SSH gave the same four comparisons in Blender 5.0.1:

```
A (both layers)       == C (first layer alone) : True
A (both layers)       == D (second layer alone): False
B (single-layer flag) == A (both layers)       : True
C == D (the two layers look different at all)  : False
A_both_0001.png  26926 bytes
B_single_0001.png  26926 bytes
C_firstonly_0001.png  26926 bytes
D_secondonly_0001.png  22892 bytes
```

One image file per frame, holding the first view layer's image, with Render Single Layer making no difference. The multi-layer OpenEXR route kept both view layers in one file there too, with both names inside it.

**Observed — Linux hand test 29, 2026-08-06. The compositor switched on, which nobody had tested on either machine.** Blender 5.0.1 on the rented Linux machine, with the same two view layers both enabled, rendered one frame five ways: with the compositor off and both layers on; with the compositor off and only the first; with the compositor off and only the second; with the compositor on and its Render Layers node reading the first view layer; and with that node moved to the second. Getting the compositor on took a version-specific route: this version has no `Scene.node_tree`; the compositor is a node group hung on `Scene.compositing_node_group`, which starts out empty, so a tree was built by hand — a Render Layers node feeding a **Group Output** node. The five files compared:

```
C (first layer alone)   == D (second layer alone) : False
A (comp off, both on)   == C (first layer alone)  : True
E (comp on, node=first) == C (first layer alone)  : True
E (comp on, node=first) == A (comp off, both on)  : True
F (comp on, node=Second)== D (second layer alone) : True
F (comp on, node=Second)== C (first layer alone)  : False
A_compoff_both_0001.png  26926 bytes
C_compoff_firstonly_0001.png  26926 bytes
D_compoff_secondonly_0001.png  22892 bytes
E_compon_first_0001.png  26926 bytes
F_compon_second_0001.png  22892 bytes
```

Two things come out of it, and they were identical in all four versions. **Switching the compositor on still writes one image file for the frame** — no second file, and no view-layer name in the name. And **what lands in that file is whichever view layer the compositor's Render Layers node is set to**: with the node moved to the second view layer, the file is pixel-for-pixel the second layer's own image and different from the first's, even though both view layers are enabled and the first is the active one.

Two things were found on the way and belong here. Asking for `Scene.node_tree` in this version raises `AttributeError: 'Scene' object has no attribute 'node_tree'`, where Blender 4.5 has that tree and it holds a **Composite** node. And touching `Scene.use_nodes` prints a warning: "'Scene.use_nodes' is expected to be removed in Blender 6.0".

**Outside the interface.** Render Single Layer is documented as ignored from the command line: "This option is ignored when rendering from the command line" (manual) and "Only affects rendering from the interface, ignored for rendering from command line" (API). The conclusion that this gives all enabled view layers from the command line, and only the active one from the window, joins those passages with the view-layer setting, so it is **inferred**. Whether "from the command line" also covers `bpy.ops.render.render()` called by a script inside a running Blender is **not stated**. **Observed**, from hand tests 15 and 17: the tests called `bpy.ops.render.render()` from a script inside a Blender started with no window, and the switch made no difference there either. What a render started by clicking writes is still untested.

**Operating system.** The sources do not say. **Observed on two operating systems, and they agree** — macOS 26.5.2 (hand tests 15, 16 and 17, 2026-08-05) and Ubuntu 22.04.5 LTS on the rented Linux machine (Linux hand tests 14, 15 and 16, 2026-08-06). The compositor-on case in Linux hand test 29 is **Linux only**; the Mac was never asked. **Windows remains untested**, because no Windows machine has run it.

---

## 10. In what order does a render work through the frames it was given — a plain run, a run that skips frames at a fixed interval, and a list of separate frames — and what does an interruption leave behind in each case?

**Answer.** For a plain run and for a stepped run, the documented direction is forward: from the start frame towards the end frame, advancing by the step each time. For a list of separate frames, the order is not stated anywhere. The sources say already rendered image-sequence frames remain after a problem; applying that to a stepped run or to a deliberate stop is inferred. They do not say what a list render leaves behind when interrupted.

**Label: documented for the stepped direction, and for image-sequence frames after a problem; inferred for the plain run's ascending order, for the stepped-run and deliberate-stop extension; not in the sources for the list order, interrupted list output, and the fate of the frame in progress.** *(Citation check, 2026-08-05: the plain run's order was labelled documented. "Render frames from start to end (inclusive)" names which frames are rendered and the direction the range is described in; it does not state the order they are worked through, so a reader must supply that step. Moved to inferred. The stepped direction stays documented — "step forward after each rendered frame" and "skip forward while rendering" do describe the movement between one rendered frame and the next.)*

Plain run:

> "-a, --render-anim — Render frames from start to end (inclusive)."

- Source: manual — https://docs.blender.org/manual/en/5.0/advanced/command_line/arguments.html
- Date read: 2026-07-30

Stepped run:

> "Step — Controls the number of frames to advance for each frame rendered, a value above 1 skips frames."

- Source: manual — https://docs.blender.org/manual/en/5.0/render/output/properties/frame_range.html
- Date read: 2026-07-30

> "frame_step — Number of frames to skip forward while rendering/playing back each frame"

- Source: Python API — https://docs.blender.org/api/5.0/bpy.types.Scene.html
- Date read: 2026-07-30

> "-j, --frame-jump <frames> — Set number of frames to step forward after each rendered frame."

- Source: manual — https://docs.blender.org/manual/en/5.0/advanced/command_line/arguments.html
- Date read: 2026-07-30

"Start to end", "advance", "step forward", "skip forward" — all four say forward, and none of them offers any other order. For the stepped run that is documented: "step forward after each rendered frame" and "skip forward while rendering" describe the movement from one rendered frame to the next. For the plain run it is a step past the words: "from start to end (inclusive)" says which frames, not in what order. *(Citation check, 2026-08-05: this paragraph read "I treat that as documented rather than inferred because the direction is in the words themselves", covering both cases. Split, and the plain run's order moved to inferred.)*

The 5.0 logging example shows the single-frame case announcing itself before it renders:

> "00:00.244  render           | Rendering single frame (frame 1)"

- Source: release notes — https://developer.blender.org/docs/release_notes/5.0/core/
- Date read: 2026-07-30

**List of separate frames — not in the sources.** The `-f` list is documented to exist (question 7) but nothing says whether it renders in the order given, in ascending order, or otherwise. Where I looked: the full command line arguments page; `bpy.ops.render` in the API; `RenderSettings` and `Scene` in the API; the manual's `render/output/` folder in full; searches across the manual for "sequential", "ascending", "in order", "one frame at a time", and across the release notes for "render-frame", "render_frame", "frame-jump".

**What an interruption leaves behind.**

- Plain run: the frames rendered so far, in the output directory — "since the frames you have already rendered will still be in the output directory" (manual — https://docs.blender.org/manual/en/5.0/render/output/animation.html, read 2026-07-30). Picking it up again: "Just disable the Overwrite option to start rendering where you left off" (same page), or "resumed at the last rendered frame by changing the frame range" (manual — https://docs.blender.org/manual/en/5.0/render/output/introduction.html, read 2026-07-30).
- Stepped run: the manual describes the intended follow-up — "you can use the Frame Step option to render every N'th frame. Then disable Overwrite and re-render with Frame Step set to 1" (manual — https://docs.blender.org/manual/en/5.0/render/output/animation.html, read 2026-07-30). The conclusion that an interrupted stepped run leaves a sparse set of frames on disk joins that passage with the recovery passage, so it is **inferred**. The documented next move is to lower Step and turn Overwrite off.
- List of separate frames: **not in the sources.** No source discusses interrupting a `-f` list render.
- In all three: whether the frame that was mid-render leaves a partial file is **not in the sources**. With Placeholders on, the option documents empty placeholder frames while rendering; applying that to the frame under way is **inferred** (question 6).
- If Placeholders is on, the pattern left behind will also contain empty files for frames that were started; that follows from "Create empty placeholder files while rendering frames" and is **inferred** as to which frames exactly.

**Not covered.** Nothing above settles whether the order can be changed, whether several frames can be in flight at once, or whether the order differs between engines. Nothing says whether Persistent Data (`use_persistent_data` — "Keep render data around for faster re-renders and animation renders, at the cost of increased memory usage", API — https://docs.blender.org/api/5.0/bpy.types.RenderSettings.html, read 2026-07-30) changes what an interruption leaves.

**Observed — hand tests 1, 8, 10, 12, 6 and 7, 2026-08-05.** Run with Blender 5.0.1 started from a terminal with no window, rendering the default cube scene to PNG.

*A plain run.* Frames 1 to 5 were rendered in ascending order, one at a time, each announced before it and saved after it. Quoted in full under question 3.

*A stepped run.* Start 1, end 10, step 3 gave files for frames 1, 4, 7, 10. Start 3, end 12, step 4 gave 3, 7, 11, and did not render the end frame. Quoted in full under question 8.

*A list of separate frames.* `-f 5,1,9` rendered them in the order written — 5, then 1, then 9. `-f 2,2,3` rendered frame 2 twice. Quoted in full under question 7.

*What an interruption leaves, in each case.* The same in all of them: whole frames only, nothing for the frame in flight, and nothing left over. A plain run interrupted at frame 3 left frames 1 and 2 at their full size. A kill that could not be caught left the same. With Placeholders on, no placeholder was left behind either — the answer above records that as inferred, and it does not hold. Quoted in full under questions 2, 3 and 6. For a video output the answer is different and is recorded under question 5.

Run on the Creator's MacBook Pro, M1 Pro chip, macOS 26.5.2.

**Observed — Linux hand tests 1, 7, 9, 11, 5 and 24, 2026-08-06.** Every part of this came back the same on a rented Linux machine — Ubuntu 22.04.5 LTS, NVIDIA GeForce RTX 4090, driver 580.105.08, twelve cores, run over SSH with Blender 5.0.1. A plain run gave 1, 2, 3, 4, 5 in that order. A stepped run started on the start frame and moved forward by the step, and did not reach back for an end frame it had overshot. `-f 5,1,9` rendered in the order written, and `-f 2,2,3` rendered frame 2 twice. An interruption left whole frames only, and nothing at all for the frame in flight, whichever way the render was stopped. The same list under Cycles gave the same order as under Workbench — and where the Mac ran that engine test in 5.1 and 5.2 only, it was run in all four versions here.

**Observed — Linux hand test 30, 2026-08-06. The order on a graphics card of another make.** The Mac has Apple's own graphics and nothing else, so it could not speak for any other card. On the rented Linux machine Cycles was pointed at its NVIDIA GeForce RTX 4090 through OPTIX, with the processor switched off:

```
compute_device_type set to: 'OPTIX'
DEVICE name='NVIDIA GeForce RTX 4090' type='CUDA' use=True
DEVICE name='AMD Ryzen Threadripper PRO 3975WX 32-Cores' type='CPU' use=False
DEVICE name='NVIDIA GeForce RTX 4090' type='OPTIX' use=True
scene cycles device = 'GPU', samples = 1000
```

`-f 5,1,9` rendered in the order 5, then 1, then 9 — the same as on the processor — with `Rendering single frame` announced before each. A plain run interrupted part-way left nothing for the frame in flight, printed the same `Sent an internal break event` line and exited with code `1`. **So neither the order nor what an interruption leaves depends on which of the two makes of card is doing the work.** One line printed on the way is not about frames: this version prints `WARNING HIPEW initialization failed: Error opening HIP dynamic library` on this machine, which is Cycles looking for AMD's library and not finding it; Blender 4.5 does not.

A first attempt at this test passed the sample count after a `--` on the command line, which made Blender hand every following argument to Python instead of reading it; nothing was rendered and the folder was empty. The setting was passed through an environment variable instead and the run above is the second attempt. Both are recorded rather than only the one that worked.

**Observed — Linux hand test 33, 2026-08-06. The order a render started the way the window's own Render Animation item starts it.** *(Linux citation check, 2026-08-06: this heading read "The order a render started by clicking uses"; nothing was clicked — the render was started by calling the operator that menu item calls, which is what the entry itself says — so it has been narrowed to the entry's own wording.)* This was untested on both machines. On the rented Linux machine, with a window open on its display, Blender 5.0.1 was given `bpy.ops.render.render('INVOKE_DEFAULT', animation=True)` — the call the Render menu's Render Animation item makes — for a six-frame Workbench range. It returned `{'RUNNING_MODAL'}`, a separate **Blender Render** window opened, and afterwards the six files were sorted by the time each was written:

```
the order the files were written in: ['o_0001.png', 'o_0002.png', 'o_0003.png', 'o_0004.png', 'o_0005.png', 'o_0006.png']
```

Ascending, one at a time, the same as a render driven from a terminal. Identical in all four versions.

**Outside the interface.** The stepped-run behaviour has a command-line twin (`-j, --frame-jump`) as well as the scene setting, and `-a` renders start to end. Documented. Whether the order is identical either way is **not in the sources**, though nothing suggests otherwise. **Observed** for the outside case only: every hand test above drove Blender from a terminal with no window, and `-j` and the scene setting gave the same frames. **Observed**, from Linux hand test 33: a render started the way the window's own Render Animation item starts it wrote its six frames in ascending order, one at a time, the same as a render driven from a terminal. *(Linux citation check, 2026-08-06: this sentence read "The order a render started by clicking uses is still untested", which Linux hand test 33 directly above had already settled. Corrected by this check.)* **The two-card runs below say nothing about the inside case**: that machine has no display server at all, so every run there was driven from outside with no window, and no window test could be repeated with two cards. *(Two-card citation check, 2026-08-06: added by this check — the line did not mention the two-card runs at all.)*

**Observed on the two-card machine — 2026-08-06. Whether the frame order or the output changes when both graphics cards are ticked rather than one.** *(Two-card citation check, 2026-08-06: this read "when two graphics cards do the work". The record below shows the preferences that were saved, not that both cards were used by the render — no device list or card load was captured here — so the heading now says what was set rather than what happened.)*

Run over SSH on a **second rented Linux machine — Ubuntu 22.04.5 LTS, two NVIDIA GeForce RTX 4090s of 24 GB each on driver 580.95.05, an AMD EPYC 7B13 with sixty-one cores, 98 GB of memory, no display server at all** — using Blender at `/opt/blender/blender-5.0.1-linux-x64/blender`, `Blender 5.0.1 (hash a3db93c5b259 built 2025-12-16 01:30:59)`, with `BLENDER_USER_RESOURCES` pointed at `/root/smoozy/two cards/res-5.0`, a folder of its own that was empty at the start. **Nothing else was running on the machine.** Label: **observed**. This is a third machine; nothing here is merged with the Creator's Mac or with the one-card rented Linux machine. The full list is `Audits/Blender Research/Frames/Hand tests — two cards and no screen.md`.

**The set-up.** A four-frame Cycles animation at 640 by 360, 128 samples, adaptive sampling off, denoising off, seed 0 and animated seed off, rendered with `--render-anim` into an empty folder. The preferences were saved with the technology `OPTIX` and both cards ticked, then again with only the card at `0000:00:07` ticked.

**The order is ascending, one at a time, with two cards as with one:**

```
00:04.319  render           | Saved: '/root/smoozy/two cards/work/ord_5.0/f_0001.png'
00:05.207  render           | Saved: '/root/smoozy/two cards/work/ord_5.0/f_0002.png'
00:06.064  render           | Saved: '/root/smoozy/two cards/work/ord_5.0/f_0003.png'
00:06.942  render           | Saved: '/root/smoozy/two cards/work/ord_5.0/f_0004.png'
```

**The files are not byte-identical — and neither are two runs on the same cards.** Every frame differed under `cmp` in both directions. That rules out the comparison method rather than the answer. At the settings these runs used, Blender wrote the time into the PNG as a `tEXtDate` chunk, so two runs made at different times differ there whatever the cards did: *(Two-card citation check, 2026-08-06: this read "Two reasons, and the first…" and then gave only one, and said "Blender stamps the time into the PNG… so any two runs differ there". Narrowed to the runs recorded here and their settings; whether the stamp can be switched off is not settled anywhere in this topic and has been sent back.)*

```
2026/08/06 11:09:48
```

against

```
2026/08/06 11:09:58
```

So the pixels were compared instead, by loading both images into Blender and counting how many of the 921,600 values differ. Of those:

- two cards against one card: **11**
- two cards against two cards, two separate runs: **9**
- one card against one card, two separate runs: **12**

**and the largest difference in every one of those comparisons was `0.003921568393707275` — one step in 255.** Two runs on the same cards differ by as much as a two-card run differs from a one-card run, so nothing here shows the number of cards changing the picture.

**Observed on the four-card machine — 2026-08-06. Whether the frame order or the picture changes when four graphics cards are ticked rather than one; and what Blender stamps into a picture.**

Run over SSH on a **third rented Linux machine — Ubuntu 22.04.5 LTS, four NVIDIA GeForce RTX 4060 Ti of 16 GB each on driver 580.95.05, an AMD EPYC 7R43 with sixty-one cores, 197 GB of memory, no display server** — using Blender at `/opt/blender/blender-5.0.1-linux-x64/blender`, `Blender 5.0.1 (hash a3db93c5b259 built 2025-12-16 01:30:59)`, with `BLENDER_USER_RESOURCES` pointed at `/root/smoozy/four cards/res-5.0`, a folder of its own emptied and remade before use. **Nothing else was running on the machine.** Label: **observed**. This is a **fourth** machine; nothing here is merged with the Creator's Mac, the one-card rented Linux machine or the two-card rented Linux machine. The full list, with every command and every script written out, is `Audits/Blender Research/Frames/Hand tests — four cards.md`.

**Of the three ways this machine differs from the two-card one, only one bears on what is below.** Four cards rather than two is the point of the test. The smaller card bears on the seconds in the output quoted here and on nothing else, and no timing from this machine is set against a two-card timing. The larger system memory bears on nothing here.

**The set-up.** The same four-frame Cycles animation as the two-card run: 640 by 360, 128 samples, adaptive sampling off, denoising off, seed 0 and animated seed off, rendered with `--render-anim` into an empty folder. Four passes, each into its own folder — all four cards ticked, one card, all four again, one again — so that every comparison has a control. The script that saves which cards are ticked, and the script that counts differing pixel values, are both written out in full in the list.

**The order is ascending, one at a time, with four cards as with one:**

```
00:00.249  render           | Rendering frame 1
00:07.697  render           | Saved: 'ord_5.0_four_a/f_0001.png'
00:07.697  render           | Rendering frame 2
00:09.200  render           | Saved: 'ord_5.0_four_a/f_0002.png'
00:09.200  render           | Rendering frame 3
00:10.758  render           | Saved: 'ord_5.0_four_a/f_0003.png'
00:10.759  render           | Rendering frame 4
00:12.327  render           | Saved: 'ord_5.0_four_a/f_0004.png'
```

The one-card passes printed the same four names in the same order.

**No two files matched, in any direction** — four cards against one, four against four, one against one — and every difference fell in the first three hundred bytes, which is where the text is. **The reason is now established rather than assumed**, and it is the part of this that reaches past this topic.

**What Blender writes into a PNG, and what turns it off.** A factory-settings scene carries sixteen stamp settings, seven of them on, in all four versions:

```
STAMP PROPERTIES: 16
    use_stamp = False
    use_stamp_camera = True
    use_stamp_date = True
    use_stamp_filename = True
    use_stamp_frame = True
    use_stamp_frame_range = False
    use_stamp_hostname = False
    use_stamp_labels = True
    use_stamp_lens = False
    use_stamp_marker = False
    use_stamp_memory = False
    use_stamp_note = False
    use_stamp_render_time = True
    use_stamp_scene = True
    use_stamp_sequencer_strip = False
    use_stamp_time = True
```

and a file written at those settings carries **eleven** text chunks:

```
    tEXt b'File\x00/root/smoozy/four cards/work/mini.blend'
    tEXt b'Date\x002026/08/06 15:14:38'
    tEXt b'Time\x0000:00:00:01'
    tEXt b'Frame\x00001'
    tEXt b'Camera\x00Camera'
    tEXt b'Scene\x00Scene'
    tEXt b'RenderTime\x0000:00.00'
    tEXt b'cycles.ViewLayer.samples\x004'
    tEXt b'cycles.ViewLayer.total_time\x0000:00.00'
    tEXt b'cycles.ViewLayer.render_time\x0000:00.00'
    tEXt b'cycles.ViewLayer.synchronization_time\x0000:00.00'
```

**Seven of them go when the sixteen settings are turned off. Four do not** — Cycles writes its own, and no setting on the scene removes them. Three of those four are times, and that is what decides whether two files match.

**On the run recorded here, a render fast enough that all three times rounded to `00:00.00` gave two files that matched byte for byte, in all four versions.** On the same scene made slower — 256 by 256 at 400 samples, on the processor, every stamp setting off — **and on that run this version's two files still did not match**:

```
out/stamp/s_a_0001.png out/stamp/s_b_0001.png differ: byte 332, line 3
32c221707fef1c9b291b8b6c1f92cc08  out/stamp/s_a_0001.png
c9edfee35f97244160c90cbffe3fbdc5  out/stamp/s_b_0001.png
    tEXt b'cycles.ViewLayer.samples\x00400'
    tEXt b'cycles.ViewLayer.total_time\x0000:00.22'
    tEXt b'cycles.ViewLayer.render_time\x0000:00.21'
    tEXt b'cycles.ViewLayer.synchronization_time\x0000:00.00'
    tEXt b'cycles.ViewLayer.samples\x00400'
    tEXt b'cycles.ViewLayer.total_time\x0000:00.22'
    tEXt b'cycles.ViewLayer.render_time\x0000:00.21'
    tEXt b'cycles.ViewLayer.synchronization_time\x0000:00.01'
IDAT of out/stamp/s_a_0001.png length 38663 md5 b981f3b7433e0b782e41bdf3026e5424
IDAT of out/stamp/s_b_0001.png length 38663 md5 b981f3b7433e0b782e41bdf3026e5424
```

**And in every one of those cases the picture is identical and only the timing text differs**, which the picture fingerprints show outright.

**So both earlier statements are wrong as written.** "Two Blender PNGs of the same frame can never match byte for byte" is wrong — they did match, in this version's own run above, on the fast scene. And "with the stamp settings off, two renders are byte-identical" is too strong — with every stamp setting off, 5.0, 5.1 and 5.2 still did not match on the slower scene, because Cycles' own timing chunks are outside those settings. **Whether two files match is decided by how long the render took, not by Blender.** The comparison that answers "is this the same render" is a comparison of the picture data alone.

**Four-card citation check, 2026-08-06 — a repeat on the same machine came out differently, and both runs stand.** This check ran the same pairs again on the four-card machine, which is still alive. **The mechanism above held exactly**: the picture was identical every single time, and the only thing that ever differed between two files was the timing text. **But the version-by-version pattern did not hold, and it is not a fact about the version.** Five separate pairs of the fast scene per version, every stamp setting off, gave — 4.5: identical, differ, identical, differ, identical; 5.0: differ, differ, identical, differ, differ; 5.1: identical, differ, differ, differ, differ; 5.2: differ, identical, identical, differ, differ. Four separate pairs of the slower scene per version, every stamp setting off, gave — 4.5: differ, differ, differ, differ (where the run above matched); 5.0: differ four times; 5.1: differ, identical, differ, differ (where the run above did not match); 5.2: differ four times. And **two renders can match byte for byte at factory stamp settings too**, which the run above reported they never do: six pairs back to back per version gave one identical pair in 4.5, one in 5.0, none in 5.1 and one in 5.2, and a twelfth pair in 5.2 came out identical with every one of its eleven text chunks the same, `Date` and `RenderTime` included:

```
    tEXt b'Date\x002026/08/06 16:39:06'
    tEXt b'RenderTime\x0000:00.01'
    tEXt b'cycles.ViewLayer.total_time\x0000:00.00'
```

So the reading that survives both runs is this: whether two files match is decided by whether every time written into them happens to round alike, and nothing else — not the version, not the stamp settings, not the number of cards. Neither run is overwritten and neither is preferred; the question is sent back to the hand tests to be run many times over rather than twice. *(End of the four-card citation check's note.)*


**With that settled, the pictures of the four passes were compared**, value by value, with a control. Frame 1, of 921,600 values: **8 differed between four cards and one card, and 9 between two separate runs on four cards**, the largest difference in both being one step in 255. So the four-card comparison differed in fewer values than the control, every difference anywhere being that same smallest step. **Nothing here shows the number of cards changing the picture, and nothing here rules it out either.** *(Four-card citation check, 2026-08-06: this check ran the same four passes again on the same machine. The order was ascending both ways in all four versions, no two files matched in any of the twelve comparisons per version, and the largest pixel difference was again one step in 255 everywhere. The counts came out **16 between four cards and one card and 11 between two runs on four cards**, against **8 and 9** above. Both runs stand. What this shows is that the ratio between the two counts is noise from one run to the next, so the sentence before last — how the four-card figure compares with the control — is about this run alone and not about the version. The last sentence is the one that survives both runs.)*

**Driven from outside — this block.** Every run above was made from outside Blender's window, and on this machine there is no other way: with no display server there is nothing for a window to open on. No window test could be repeated with four cards.

**Operating system — this block.** Linux, on the machine named at the top of it. The stamp result was got on this Linux machine; a checker's separate run on the Creator's Mac in Blender 4.5 also found the file matching with the stamp settings off, and the two are not merged here. Windows is untested.


**Operating system.** The sources do not say. **Observed on two operating systems, and they agree** — macOS 26.5.2 (hand tests 1, 8, 10, 12, 6 and 7, 2026-08-05) and Ubuntu 22.04.5 LTS on the rented Linux machine (Linux hand tests 1, 7, 9, 11, 5 and 24, 2026-08-06). **Observed on both makes of graphics card the project has**, from Linux hand test 30: Apple's own on the Mac and an NVIDIA GeForce RTX 4090 here, with the same result. **The order a render started the way the window starts it (Linux hand test 33) is Linux only**; the Mac was never asked. *(Linux citation check, 2026-08-06: added by this check — the line named only the tests run on both machines.)* **Observed with two graphics cards in one machine, and it changes nothing** — see the block above: ascending order both ways, and the picture differs no more between two cards and one than between two runs on the same cards. **Windows remains untested**, because no Windows machine has run it, and no card of any third make can be reached. **Observed with four graphics cards in one machine as well, on a fourth machine, and it changes nothing** — the order is ascending both ways, and the picture differs no more between four cards and one than between two runs on four cards. That run also settled what Blender writes into a PNG and when two files can match, which is what makes the picture comparison the reliable one. *(Added 2026-08-06 by the hand tests on the four-card machine.)*

---

## 11. Can a render be told to stop once the frame it is working on is finished, rather than the moment it is asked?

**Answer.** Not in the sources. The documented controls say a render can be aborted or canceled, but do not say whether stopping waits for the current frame to finish.

**Label: not in the sources.**

What is documented:

> "As with a normal render, you can abort it with Esc."

- Source: manual — https://docs.blender.org/manual/en/5.0/editors/3dview/viewport_render.html
- Date read: 2026-07-30

> "The task can be aborted by clicking the cancel button ()."

- Source: manual — https://docs.blender.org/manual/en/5.0/interface/window_system/status_bar.html
- Date read: 2026-07-30

> "test_break() — Test if the render operation should been canceled, this is a fast call that should be used regularly for responsiveness"

- Source: Python API — https://docs.blender.org/api/5.0/bpy.types.RenderEngine.html
- Date read: 2026-07-30

> "bpy.app.handlers.render_cancel — on canceling a render job. Accepts one argument: the scene data-block being rendered"

- Source: Python API — https://docs.blender.org/api/5.0/bpy.app.handlers.html
- Date read: 2026-07-30

`bpy.ops.render.view_cancel()` — "Cancel show render view" — is about the render *window*, not the job (Python API — https://docs.blender.org/api/5.0/bpy.ops.render.html, read 2026-07-30).

The `test_break()` wording — a cancel check that "should be used regularly for responsiveness" — points the other way from a deferred stop: it exists so an engine can drop what it is doing mid-frame. Reading it as proof that a stop is always immediate would be a step past what it says, so I do not claim that; I note it as the closest the sources come.

**Where I looked.** The full command line arguments page (no stop, pause, or drain flag of any kind). `bpy.ops.render` in full. `bpy.types.RenderSettings`, `bpy.types.Scene`, `bpy.types.RenderEngine`, `bpy.app.handlers` in full. The manual's `render/output/` folder in full. The 5.0 release notes in full for rendering, core, pipeline & I/O, python API, compositor and corrective releases. Searches across manual, API and release notes for: "stop after", "finish the current", "current frame is finished", "pause render", "graceful", "abort", "cancel", "interrupt", "SIGTERM", "signal", "kill", "resume", "checkpoint". The only hits of any relevance are the four quoted above, plus a 5.0 bugfix entry, "Linux Freeze when force cancelling a CLI render" (release notes — https://developer.blender.org/docs/release_notes/5.0/bugfixes/, read 2026-07-30), which describes a fixed fault rather than a feature.

**Not covered.** Because there is no documented deferred stop, nothing is settled about what would happen to a frame in progress when a stop is asked — whether it is abandoned, whether a partial file is written, or whether Blender waits for the current write to finish before quitting.

**Observed — hand tests 6 and 7, 2026-08-05.** A stop does not wait for the current frame, and the frame in progress is thrown away. Blender 5.0.1 was started from a terminal with no window, rendering the default cube scene in Cycles at 1000 samples and 400 by 400 pixels to PNG, frames 1 to 5, at about five seconds a frame. Fifteen seconds in — during frame 3 — it was sent an interrupt, the same thing Ctrl-C does. It printed:

```
Sent an internal break event. Press ^C again to kill Blender

Blender quit
```

and ended 0.8 seconds later, over three separate runs (0.8, 0.8, 0.8), with exit code `1`. The folder afterwards held `f_0001.png` and `f_0002.png` at 92406 bytes each, and nothing for frame 3. Frame 3's work was discarded, not written and not half-written.

The delay is worth recording, because it differs between versions and was steady over three runs each. Blender 5.0.1 and 5.1.2 both ended within about a second. Blender 4.5.12 took 5.6 to 5.8 seconds and 5.2.0 took 5.0 seconds — about as long as the rest of the frame would have needed — so those two appear to finish the frame's work before quitting, and then throw it away. In none of the four was a file written for the interrupted frame.

A polite terminate signal instead ended it in 0.2 seconds with exit code `-15`, printing nothing further, leaving the same two files. A kill that could not be caught ended it in 0.2 seconds with exit code `-9`, leaving the same two files.

Run on the Creator's MacBook Pro, M1 Pro chip, macOS 26.5.2.

**Observed — hand test 23, 2026-08-05.** Lowering the end frame while the render runs does not stop it. Blender 5.0.1 rendered frames 1 to 8 with a handler attached to Blender's own "frame written" event; the moment frame 2 was written the handler set the end frame to 2:

```
HANDLER: frame 2 has just been written; setting frame_end to 2
HANDLER: frame_end now reads 2
```

All eight frames were rendered anyway — `f_0001.png` through `f_0008.png`, 53079 bytes each. The animation loop does not re-read the end frame as it goes. Same machine and date.

**Observed — hand test 29, run by the Creator, 2026-08-05.** Blender's own window offers nothing that stops a render at a frame boundary either. With an animation render running in Blender 5.0.1, the Creator opened the topbar's **Render** menu. It held six items and no more:

> Render Image — F12
> Render Animation — ⌘ F12
> Render Audio…
> View Render — F11
> View Animation — ⌘ F11
> Lock Interface (a checkbox)

There is no stop, pause, cancel or finish-this-frame item among them.

The status bar at the bottom showed `Rendering...` with a progress bar, a percentage, and a small cross. Hovering the cross gave the tooltip:

> Stop this job.

The bottom right of the window read `Duration: 00:00+10 (Frame 3/10)`, and the render information line across the top of the image editor read, at that moment:

> Frame:3 | Last:0… : 00:15.58 | Mem: 19M | Sample 769/4000

So the only stop the window offers is the one the manual describes, and it is offered as "Stop this job" with nothing about waiting for the frame.

On when it takes effect, the Creator's own words: "Stopping isn't instant, there's a slight lag. All versions." They did not time the lag. That is consistent with the runs from a terminal, which measured about 0.8 seconds in this version, though those were an interrupt sent to a Blender with no window rather than Esc pressed in one.

Afterwards the output folder held exactly `frame_0001.png` and `frame_0002.png`, 92315 bytes each, and nothing for the frame that was in flight.

Run on the Creator's Mac — a MacBook Pro, M1 Pro chip, macOS 26.5.2 — so this settles macOS and leaves Windows and Linux open.

**Observed — Linux hand tests 5, 6 and 22, 2026-08-06.** The same answer on a rented Linux machine — Ubuntu 22.04.5 LTS, NVIDIA GeForce RTX 4090, driver 580.105.08, twelve cores, run over SSH. An interrupt, a polite terminate and an uncatchable kill each stopped Blender 5.0.1 there, none of them waited for the current frame, and none left anything for it. The handler route failed the same way: with a handler on Blender's own "frame written" event setting the end frame to 2 the moment frame 2 was written, all eight frames were rendered anyway.

```
HANDLER: frame 2 has just been written; setting frame_end to 2
HANDLER: frame_end now reads 2
the render call has returned
---- FOLDER AFTERWARDS ----
f_0001.png  53080 bytes
f_0002.png  53080 bytes
f_0003.png  53080 bytes
f_0004.png  53080 bytes
f_0005.png  53080 bytes
f_0006.png  53080 bytes
f_0007.png  53080 bytes
f_0008.png  53080 bytes
```

**On how long the stop took, this machine cannot say.** Three interrupts were sent in this version and the process ended 1.8, 3.6 and 4.8 seconds later. Those numbers are raw output kept for the record and **are not a measurement**: the machine was shared with other work throughout, which can only stretch them. The Mac's steady pattern — about five and a half seconds in 4.5 and five in 5.2, against about one second in 5.0 and 5.1 — is neither confirmed nor contradicted here. Settling it needs the machine to itself, and that is written into `Audits/Blender Research/Frames/Hand tests — Linux.md`.

**Observed — Linux hand test 32, 2026-08-06.** Blender's own window offers nothing that stops a render at a frame boundary on this machine either. With an animation render running in Blender 5.0.1 on the rented Linux machine's display — started the way the menu item starts it — the topbar's **Render** menu was opened with Blender's own `wm.call_menu` and photographed with Blender's own screenshot command. The picture shows six items and no more: Render Image (F12), Render Animation (Ctrl F12), Render Audio…, View Render (F11), View Animation (Ctrl F11), and a Lock Interface checkbox. No stop, pause, cancel or finish-this-frame item. The status bar at the foot of the window shows the word **Rendering…**, then a progress bar with a percentage in it, then a small cross.

Two things were not settled there and are not claimed. The cross's tooltip was not read, because hovering needs the pointer. And **whether the Esc key reached Blender could not be demonstrated** — that machine's display is shared with other work, and in one version a further frame was written after Esc was sent, which says the render was still going. So the "Stop this job." tooltip and the Esc cancel rest on the Creator's macOS run alone.

**Outside the interface.** Esc and the status-bar cancel button are documented interface controls. That neither is available in background mode is **inferred** from that description. What replaces them for a background render is **not in the sources**. **Observed**, from hand tests 6, 7 and 23: a background render *can* be stopped, by an interrupt, a polite terminate, or a kill; none of the three waits for the current frame; and no way was found to make one wait. **Observed**, from hand test 29: inside the window there is no such control either — the Render menu offers no stop of any kind, and the only one there is is the status bar's cross, labelled "Stop this job." So the answer is the same on both sides of the window: nothing stops a render at a frame boundary, and the frame in flight is thrown away.

**Operating system.** The sources do not say, beyond the single Linux bug note above. **Observed on two operating systems, and they agree that there is no such control** — macOS 26.5.2 (hand tests 6, 7, 23 and 29, 2026-08-05) and Ubuntu 22.04.5 LTS on the rented Linux machine (Linux hand tests 5, 6, 22 and 32, 2026-08-06). Nothing stops a render at a frame boundary on either, the frame in flight is thrown away on both, and the Render menu holds the same six items on both. **How long a stop takes to bite is settled on macOS only**, for the reason given above. **Windows remains untested**, because no Windows machine has run it.

---

## Things a person with Blender 5.0 open could still settle by hand

The list that stood here has been run — the results are written into the answers above as observed findings, dated 2026-08-05 for the Creator's Mac and 2026-08-06 for the rented Linux machine. Three of the items that stood here have since been answered: **what several view layers write with the compositor switched on** (question 9), **Autosplit at the 2 GiB boundary** (question 5), and **Linux and a graphics card of another make**, which now run through every answer above. Three items are left, and none of them is a matter of somebody sitting down at Blender for a few minutes. **Observed 2026-08-06 on a third machine, and it added one more:** two graphics cards in one machine leave the frame order unchanged, and no change in the picture was shown — see question 10. *(Two-card citation check, 2026-08-06: this read "change neither the frame order nor the picture". The pixel counts do not carry that: they show no difference beyond a single step in 255, and in 4.5 and 5.1 rather more of those than two runs on the same cards showed.)*

1. **What an abrupt power-off leaves behind.** (Question 1.) A Blender killed outright is recorded under question 1 and leaves the finished frames whole, on both machines. What that cannot reach is a file the operating system had not yet flushed to disk when the power went. This was written out as a test and offered; the Creator declined it, which is their call. It cannot be run on the rented Linux machine either, because that machine is reached over the network and cutting its power would leave nothing to read the result from.

2. **What a media player does with a part-written video file, on anything but QuickTime on macOS.** (Question 5.) QuickTime Player is recorded under question 5. The rented Linux machine has no media player installed at all and installing one was not permitted, so only Blender's own reader has opened these files there. What Blender's own Video Sequencer makes of one is also still unrecorded.

3. **Anything on Windows.** No Windows machine has run it, so every observed finding above covers macOS and Linux and leaves Windows open.

Three more things need the rented machine to itself rather than a machine shared with other work, and they are written out in `Audits/Blender Research/Frames/Hand tests — Linux.md`: how long a stop takes to bite, the pointer half of the window tests, and a demonstrated Esc cancel.

---

2026-08-04 — Opus 5 (Claude Code) — Frames, Blender 5.0

2026-08-05 — Opus 5 (Claude Code) — hand-test results written into questions 1 to 11, and the hand-test list above pruned

2026-08-05 — Opus 5 (Claude Code) — the Creator's own hand-test results written into questions 1, 3, 5, 7, 8 and 11, and the hand-test list above pruned again

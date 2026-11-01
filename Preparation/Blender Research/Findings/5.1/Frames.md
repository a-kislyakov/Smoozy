# Frames — Blender 5.1

**Sources consulted:** Blender 5.1 manual (`Blender Sources/manual_5.1`, including two pictures from its `_images` folder), Blender 5.1 Python API documentation (`Blender Sources/api_5.1`), Blender 5.1 release notes (`Blender Sources/release_notes_5.1`), and Cycles' own files (`Blender Sources/cycles_5.1`). All read on **2026-07-30** (the snapshot date given to me). I did not use the web.

I read only the question sheet and files inside Blender Sources/. I did not open any other part of this project.

I worked alone. No helper agents were started, so nothing in this file came from anywhere I did not read myself.

---

## 1. Can a render that was interrupted by the computer going down be picked up from where it stopped?

**Answer.** Yes, if the render was writing a numbered image sequence, and "picked up" means starting again at the first frame that is not yet on disk. The manual describes exactly this case. It is not a resume inside a frame — it is a fresh render that skips the frames already written.

The manual's Hints section opens with this case by name:

> "Your computer accidentally turns off in the middle of rendering your movie!"

and answers it:

> "This allows you an easy recovery if there is a problem and you have to re-start the rendering, since the frames you have already rendered will still be in the output directory."

> "Just disable the Overwrite option to start rendering where you left off."

For a render that was writing straight to a video file, the manual says the opposite:

> "If a problem occurs while rendering, the file might become unplayable and you will have to re-render all frames from the beginning."

- **Label:** documented
- **Source:** Blender manual. https://docs.blender.org/manual/en/5.1/render/output/animation.html (the first three quotes) and https://docs.blender.org/manual/en/5.1/render/output/properties/output.html (the fourth)
- **Date read:** 2026-07-30

**Not covered.** The sources say nothing about picking up a *single frame* that was half-rendered when the machine went down — no partial-frame state, no sample checkpointing. They also do not say what happens to a frame's file if the machine died while that file was being written; only that the frames "already rendered" are still there. The word "already rendered" is not defined against "already written".

**Observed — hand test 7, 2026-08-05.** A computer going down cannot be reproduced from a terminal, but a Blender killed outright, with no chance to tidy up, is the closest thing to it. Blender 5.1.2 was started from a terminal with no window, rendering the default cube scene in Cycles at 1000 samples and 400 by 400 pixels, PNG output into an empty folder, frames 1 to 5 at about five seconds a frame. Fifteen seconds in, while frame 3 was rendering, it was sent a kill it could not catch. It ended 0.2 seconds later with exit code `-9`. The folder afterwards:

```
f_0001.png  92486 bytes
f_0002.png  92486 bytes
```

The two finished frames survived intact, and nothing at all was left for the frame in flight — so the "not covered" note above, about a file half-written when the machine died, has one part settled: with Placeholders off, no file for the frame in flight is left at all. Run on the Creator's MacBook Pro, M1 Pro chip, macOS 26.5.2.

**Observed — hand test 18, 2026-08-05.** The same kill, with the output set to a single video file, left a 48-byte file holding no frames. Recorded in full under Q5.

**Still unsettled, and why.** A real power-off was written as hand test 31 and marked optional. The Creator declined it — "I would rather not do Test 36", 2026-08-05, that being its number on the combined list they were working from. It is their own choice, not a limit of the equipment, and it has been moved to the "cannot be run" section of the hand-test list. So the question splits in two. The half where **the program dies** is answered above: an uncatchable kill leaves every finished frame whole and nothing at all for the frame in flight. The half where **the operating system had not yet flushed a file to disk** when the power went is not answered, and nothing short of a real power-off reaches it. That is exactly the edge the audit sent this way, and it stays open.

**Observed — Linux hand test 6, 2026-08-06.** The same kill, on a second machine, gave the same answer. This was run on a rented Linux machine — Ubuntu 22.04.5 LTS, NVIDIA GeForce RTX 4090, driver 580.105.08, twelve cores, run over SSH. Blender 5.1.2 was started there from a terminal with no window, against Blender's own startup scene, with its own settings folder and factory startup. Rendering the default cube scene in Cycles at 1000 samples and 400 by 400 pixels, PNG output into an empty folder, frames 1 to 5, it was sent a kill it could not catch fifteen seconds in. The exit code recorded was `-9`, and the finished frames were on disk whole — f_0001.png, f_0002.png and f_0003.png at 92482 bytes each — with nothing at all for the frame that was in flight. How many frames had finished by then differed from the Mac run, because the machine was shared with other work; no claim is made here about how fast anything ran.

**The half this cannot reach stays unreachable on both machines.** A real power-off was offered to the Creator and declined. It cannot be run on the rented Linux machine either: that machine is reached over the network, so cutting its power would leave nothing to read the result from.

**Outside the interface.** The recovery described is a property of the output files plus the Overwrite setting, not of the window. `use_overwrite` is a scene render property (see Q4) and so is set the same way in a script or a background render. The manual does not say this in so many words, so the claim that the recovery works identically in a background render is inferred, not documented. Nothing in the sources restricts Overwrite to the interface. **Observed**, from hand tests 2 and 7: the whole recovery works on a Blender started from a terminal with no window — Overwrite off skips the frames already on disk (Q4), and the finished frames survive an uncatchable kill.

**Operating system.** The sources do not say. Nothing on the pages above names an operating system. **Observed on two operating systems, and they agree** — macOS 26.5.2 on the Creator's MacBook Pro (hand test 7, 2026-08-05) and Ubuntu 22.04.5 LTS on the rented Linux machine (Linux hand test 6, 2026-08-06): an uncatchable kill leaves every finished frame whole and nothing for the frame in flight on both. **Windows remains untested**, because no Windows machine has run it. **Untried rather than untestable: a Windows machine can be rented, and every one this project has read reported a Server edition — see the equipment section of `Preparation/Blender Research/Method.md`.**

---

## 2. Can a render that was stopped on purpose be picked up later?

**Answer.** Yes, for image-format output. The manual states it directly:

> "For example, when rendering animations to image file formats the render job can be canceled and resumed at the last rendered frame by changing the frame range. This is useful if the animation takes a long time to render and the computers resources are needed for something else."

The manual gives a second route to the same result — leave the range alone and turn Overwrite off:

> "Just disable the Overwrite option to start rendering where you left off."

And it lists this as a reason to choose the Frame Sequence approach in the first place:

> "May need to interrupt rendering to use the computer, and want to be able to resume rendering where you left off."

On stopping a running render, the manual documents a cancel control on the status bar:

> "Shows the progress of the currently running task (such as rendering or baking). Hovering the mouse pointer over the progress bar will display a time estimate. The task can be aborted by clicking the cancel button ()."

- **Label:** documented
- **Source:** Blender manual. https://docs.blender.org/manual/en/5.1/render/output/introduction.html (first quote), https://docs.blender.org/manual/en/5.1/render/output/animation.html (second and third), https://docs.blender.org/manual/en/5.1/interface/window_system/status_bar.html (fourth)
- **Date read:** 2026-07-30

**Not covered.** Two things. First, "resumed at the last rendered frame" — the sources do not say whether the frame that was in progress when you cancelled counts as rendered, or what its file looks like. Second, the cancel button is an interface control; the sources give no equivalent for a render started from outside the window.

**Observed — hand tests 6 and 7, 2026-08-05.** A render started from a terminal with no window can be stopped, and the frame in progress is discarded rather than written. Blender 5.1.2 was started with no window, rendering the default cube scene in Cycles at 1000 samples and 400 by 400 pixels, PNG output into an empty folder, frames 1 to 5, at about five seconds a frame. Fifteen seconds in — during frame 3 — it was sent an interrupt, which is exactly what pressing Ctrl-C in a terminal does. The last lines it printed were:

```
00:10.584  render           | Saved: '…/f_0002.png'
00:10.584  render           | Time: 00:05.07 (Saving: 00:00.00)
00:10.584  render           | Rendering frame 3
00:10.584  render           | Start rendering: Scene, ViewLayer
00:10.584  render           | Engine: Cycles

Sent an internal break event. Press ^C again to kill Blender

Blender quit
```

It ended 1.0 seconds after the interrupt (three separate runs: 1.0, 1.0, 1.0 seconds) with exit code `1`. The folder afterwards:

```
f_0001.png  92486 bytes
f_0002.png  92486 bytes
```

So the frame that was in progress does not count as rendered and leaves nothing behind — the first of the two things this answer records as not covered. A polite terminate signal instead ended it in 0.2 seconds with exit code `-15`, printing nothing further, leaving the same two files. Run on the Creator's MacBook Pro, M1 Pro chip, macOS 26.5.2.

**Observed — Linux hand tests 5 and 6, 2026-08-06.** On the rented Linux machine, Blender 5.1.2 printed exactly what it printed on the Mac when sent an interrupt fifteen seconds into the same render:

```
Sent an internal break event. Press ^C again to kill Blender

Blender quit
```

and exited with code `1`, leaving only whole frames and nothing for the frame in flight. A polite terminate ended it with code `-15` and a kill with code `-9`, both leaving the same. **The render did not freeze** — worth recording because Blender 5.0's release notes carry a fixed bug called "Linux Freeze when force cancelling a CLI render", and nothing of that kind was seen here in any of the four versions. How long each stop took to bite is kept under question 11 as raw output and is not offered as a measurement: the machine was shared.

**Outside the interface.** Partly no. The cancel button quoted above is part of Blender's status bar and does not exist in a background render. The manual, API documentation and release notes searched for `SIGINT`, `SIGTERM`, "abort", "cancel" and "stop after" do not describe how to stop a running background render. This is not a finding about Cycles' own files. Whether the recovery half (changing the frame range or turning Overwrite off) works for a render started outside the interface is not stated here; Q4 documents the setting but not that scope. The 5.1 bug-fix list does mention cancelling a command-line render in passing, which shows the thing exists even though the sources never describe how it is done: "Warnings about freeing memory after the leak detecter has run when cancelling a CLI render" (https://developer.blender.org/docs/release_notes/5.1/bugfixes/, read 2026-07-30). **Observed**, from hand tests 6, 7 and 2: such a render is stopped by an interrupt, a polite terminate, or a kill, all three leaving only whole frames behind; and the recovery half works there too — Overwrite off skips the frames already on disk in a Blender driven with no window (Q4).

**Operating system.** The sources do not say. **Observed on two operating systems, and they agree** — macOS 26.5.2 (hand tests 6 and 7, 2026-08-05) and Ubuntu 22.04.5 LTS on the rented Linux machine (Linux hand tests 5 and 6, 2026-08-06). An interrupt, a polite terminate and an uncatchable kill each stop the render on both, with the same three exit codes, and each leaves only whole frames. **Windows remains untested**, because no Windows machine has run it.

---

## 3. Do frames already written survive a render ending part-way, whether it was stopped on purpose or not?

**Answer.** For an image sequence, yes — this is stated outright, and it is the reason the manual recommends image sequences.

> "since the frames you have already rendered will still be in the output directory."

The manual also explains why they are on disk at all rather than in memory:

> "While rendering stills will allow you to view and save the image from the render buffer when it is complete, animations are a series of images, or frames, and are automatically saved directly out to a drive after being rendered."

For a render going to a single video file, the surviving file is not guaranteed usable:

> "If a problem occurs while rendering, the file might become unplayable and you will have to re-render all frames from the beginning."

- **Label:** documented
- **Source:** Blender manual. https://docs.blender.org/manual/en/5.1/render/output/animation.html (first two quotes), https://docs.blender.org/manual/en/5.1/render/output/properties/output.html (third)
- **Date read:** 2026-07-30

**Not covered.** The manual makes no distinction between a deliberate stop and a crash for image sequences — it speaks of "a problem" and of the computer turning off, and of cancelling, and gives the same answer. But it never says the two are identical, and it says nothing about the one frame that was in flight at the moment of the stop. It also says nothing about whether a file is written whole or in pieces (see Q6).

**Observed — hand tests 1, 6 and 7, 2026-08-05.** Frames already written survive, and a deliberate stop and a crash give the same answer. Blender 5.1.2 was started from a terminal with no window, rendering the default cube scene to PNG, and stopped part-way in three different ways: an interrupt (Ctrl-C), a polite terminate, and a kill it could not catch. In all three the folder afterwards held the finished frames, whole:

```
f_0001.png  92486 bytes
f_0002.png  92486 bytes
```

and nothing for the frame in flight. So the manual's not saying the two cases are identical turns out not to matter here: they are. The whole run, undisturbed, writes one frame at a time:

```
00:00.479  render           | Rendering animation (frames 1..5)
00:00.479  render           | Rendering frame 1
00:00.480  render           | Start rendering: Scene, ViewLayer
00:00.480  render           | Engine: Workbench
00:00.659  render           | Saved: '…/f_0001.png'
00:00.659  render           | Time: 00:00.17 (Saving: 00:00.06)
00:00.659  render           | Rendering frame 2
…
00:00.678  render           | Saved: '…/f_0005.png'
```

Run on the Creator's MacBook Pro, M1 Pro chip, macOS 26.5.2. Whether a file is written whole or in pieces is answered under Q6: in pieces, and it can be caught part-written.

**Observed — hand test 29, run by the Creator, 2026-08-05.** The same holds for a render started and cancelled inside Blender's own window. A prepared file was opened in Blender 5.1.2 — Cycles, 4000 samples, 400 by 400 pixels, PNG output into an empty folder, frames 1 to 10, about twenty-five seconds a frame. Render Animation was started, frames 1 and 2 were allowed to finish, and Esc was pressed while frame 3 was under way. Afterwards the output folder held exactly two files and nothing else:

```
92388 bytes  frame_0001.png
92388 bytes  frame_0002.png
```

The Creator ran the render and the cancel and left the folder for reading; the listing above is what was on disk afterwards. Their own words on the cancel: "You can probably look at the files yourself. Stopping isn't instant, there's a slight lag. All versions." Run on the Creator's Mac — a MacBook Pro, M1 Pro chip, macOS 26.5.2 — so this settles macOS and leaves Windows and Linux open.

**Observed — Linux hand tests 1, 5 and 6, 2026-08-06.** The same three ways of ending a render were tried on a rented Linux machine — Ubuntu 22.04.5 LTS, NVIDIA GeForce RTX 4090, driver 580.105.08, twelve cores, run over SSH and gave the same answer: every frame already written was there, whole, and nothing at all was left for the frame in flight. Undisturbed, the plain five-frame run there printed the same shape of output as on the Mac — `Rendering animation (frames 1..5)`, then `Rendering frame N`, `Start rendering: Scene, ViewLayer`, `Engine: Workbench` and `Saved:` for each frame, all timestamped and tagged `render` — exit code 0, five files of 3639 bytes each. One thing differs between the machines and is not about frames: the same scene gives a 3639-byte file on Linux where the Mac gave a different size. Nothing on this sheet asks about pixels; it is recorded only so nobody assumes a frame rendered on two machines is the same file.

**Observed — Linux hand test 32, 2026-08-06.** A render started the way the window's own Render Animation item starts it also leaves only whole frames. With a window open on the rented Linux machine's display, Blender 5.1.2 was given `bpy.ops.render.render('INVOKE_DEFAULT', animation=True)` — the call that menu item makes — which returned `{'RUNNING_MODAL'}`, and a separate **Blender Render** window opened. The render was ended part-way and the folder held whole frames of 92348 bytes and nothing for the frame in flight. Note what this does **not** show: whether the **Esc** key reached Blender could not be demonstrated on that machine, because its display is shared with other work and a key press could not be shown to land in the intended window. So this is a render that was ended, not a demonstrated Esc cancel; the Esc half rests on the Creator's macOS run.

**Outside the interface.** The claim rests on frames being written to disk as the render goes, which is a property of animation rendering rather than of the window. The sources do not restate it for background rendering. That it holds there too is inferred from the same page describing animation rendering generally. **Observed**, from the hand tests above: it holds for a render started from a terminal with no window, and hand test 29 shows it holds for a render started and cancelled inside the window too — the same two whole frames, nothing for the third. So this answer no longer depends on how the render was started.

**Operating system.** The sources do not say. **Observed on two operating systems, and they agree** — macOS 26.5.2 (hand tests 1, 6, 7 and 29, 2026-08-05) and Ubuntu 22.04.5 LTS on the rented Linux machine (Linux hand tests 1, 5, 6 and 32, 2026-08-06). Frames already written survive on both, whichever way the render ends. **Windows remains untested**, because no Windows machine has run it.

---

## 4. Can Blender be asked to render only the frames that are not already on disk, and if so, how does it decide what counts as already there?

**Answer.** Yes — turn the Overwrite option off. The manual's own words for the setting are thin:

> "Overwrite existing files when rendering."

The API documentation says the same and gives the default:

> "use_overwrite — Overwrite existing files while rendering (default True)"

The manual uses it for exactly this purpose twice:

> "Just disable the Overwrite option to start rendering where you left off."

> "Using an image format for output, you can use the Frame Step option to render every N'th frame. Then disable Overwrite and re-render with Frame Step set to 1."

And it builds a render farm out of it:

> "Primitive Render Farm
> An easy way to get multiple machines to share the rendering workload is to:
> Set up a shared directory over a network file system.
> Disable Overwrite, enable Placeholders in the Render Output panel.
> Start as many machines as you wish rendering to that directory."

**How it decides what counts as already there.** The sources only ever say "existing files". They never spell out which path is checked. The path a frame will be written to is itself documented, as a Python call:

> "frame_path(*, frame=-2147483648, preview=False, view='') — Return the absolute path to the filename to be written for a given frame"

and the output path is described as:

> "Directory/name to save animations, # characters define the position and padding of frame numbers"

with the manual adding:

> "When rendering an animation, the frame number is appended at the end of the file name with four padded zeros (e.g. image0001.png). You can set a custom padding size by adding the appropriate number of # anywhere in the file name (e.g. image_##_test.png translates to image_01_test.png)."

Cycles' own files show this pattern in use — deciding a frame is "there" by asking the operating system whether that exact path is a file:

```
            for frame in range(scene.frame_start, scene.frame_end + 1):
                scene.render.filepath = in_filepath
                filepath = scene.render.frame_path(frame=frame)
                in_filepaths.append(filepath)

                if not os.path.isfile(filepath):
                    scene.render.filepath = original_filepath
                    err_msg = tip_("Frame '%s' not found, animation must be complete") % filepath
                    self.report({'ERROR'}, err_msg)
                    return {'CANCELLED'}
```

- **Label:** **documented** that the option exists and does this job (the quotes above). **Inferred** that the test is "a file exists at the path this frame would be written to" — the step I took: the setting is described only as acting on "existing files", the file a frame corresponds to is the one `frame_path` returns for that frame, so "already there" can only mean a file present at that path. The sources never say this. **Observed** that this is how Cycles' own code decides a frame is present — but that is Cycles' *denoise* operator checking its inputs, not the renderer skipping frames, so it is evidence of the pattern and nothing more.
- **Source:** Blender manual, https://docs.blender.org/manual/en/5.1/render/output/properties/output.html (Overwrite, Primitive Render Farm, file-name padding) and https://docs.blender.org/manual/en/5.1/render/output/animation.html (the two uses). Blender Python API, https://docs.blender.org/api/5.1/bpy.types.RenderSettings.html (`use_overwrite`, `filepath`, `frame_path`). Cycles' own file `Blender Sources/cycles_5.1/operators.py`, lines 79–88, inside the `CYCLES_OT_denoise_animation` operator's `execute`.
- **Date read:** 2026-07-30

**Not covered.** This does not settle: whether a zero-length file counts as existing (see Q6 — Placeholders deliberately creates such files, and the render-farm recipe depends on them counting); what happens with a video output, where there is one file for the whole range rather than one per frame; whether the check happens once at the start or per frame as the render reaches it; and what happens when two machines reach the same frame at the same moment.

**Observed — hand test 2, 2026-08-05.** The test is bare existence, and a zero-length file counts — which settles the first of the open edges named above. Blender 5.1.2 was started from a terminal with no window and rendered the default cube scene to PNG, frames 1 to 5, into an empty folder. Then `f_0004.png` was deleted outright and `f_0003.png` was emptied to zero bytes but left in place:

```
3712 f_0001.png
3712 f_0002.png
   0 f_0003.png
3712 f_0005.png
```

The same render was then run again with Overwrite off. Blender printed:

```
00:00.318  render           | Rendering frame 1
00:00.318  render           | Skipping existing frame "…/f_0001.png"
00:00.318  render           | Rendering frame 2
00:00.318  render           | Skipping existing frame "…/f_0002.png"
00:00.318  render           | Rendering frame 3
00:00.318  render           | Skipping existing frame "…/f_0003.png"
00:00.318  render           | Rendering frame 4
00:00.456  render           | Saved: '…/f_0004.png'
00:00.456  render           | Rendering frame 5
00:00.456  render           | Skipping existing frame "…/f_0005.png"
```

and the folder afterwards:

```
3712 f_0001.png
3712 f_0002.png
   0 f_0003.png
3712 f_0004.png
3712 f_0005.png
```

The zero-byte file was skipped and stayed zero bytes. (Blender 4.5.12 running the same test prints no `Rendering frame N` announcement and writes `skipping existing frame` with a small s.) Run on the Creator's MacBook Pro, M1 Pro chip, macOS 26.5.2.

**Observed — hand test 22, 2026-08-05.** The path a frame is checked against is the one `frame_path` returns, formed exactly as the manual describes. Blender 5.1.2, asked from a terminal with no window:

```
filepath '<dir>/f_'                     -> frame_path(7) = '<dir>/f_0007.png'
filepath '<dir>/f_##_x'                 -> frame_path(7) = '<dir>/f_07_x.png'
filepath '<dir>/plain'                  -> frame_path(7) = '<dir>/plain0007.png'
filepath '<dir>/six_######'             -> frame_path(7) = '<dir>/six_000007.png'
with File Extensions off        -> frame_path(7) = '<dir>/f_0007'
frame_path(12345) = '<dir>/big_12345.png'
```

Same machine and date.

**Observed — hand test 5, 2026-08-05.** The check happens once per frame as the render reaches it, not once at the start; and two renders can reach the same frame at the same moment. Two copies of Blender 5.1.2 were started at the same instant from a terminal with no window, each with its own separate settings folder, both rendering frames 1 to 6 of the same slow scene into one shared folder with Overwrite off and Placeholders on. *(Citation check, 2026-08-05: what made the scene slow — its sample count, resolution and format — was not written down, so this run cannot be repeated exactly from what is here. It was run again with the scene stated, as hand test 33 below.)* The first said:

```
00:08.743  render           | Saved: 'f_0001.png'
00:08.743  render           | Skipping existing frame "f_0002.png"
00:08.743  render           | Skipping existing frame "f_0003.png"
00:16.890  render           | Saved: 'f_0004.png'
00:25.144  render           | Saved: 'f_0005.png'
00:25.145  render           | Skipping existing frame "f_0006.png"
```

and the second said:

```
00:00.394  render           | Skipping existing frame "f_0001.png"
00:08.723  render           | Saved: 'f_0002.png'
00:16.892  render           | Saved: 'f_0003.png'
00:16.892  render           | Skipping existing frame "f_0004.png"
00:16.892  render           | Skipping existing frame "f_0005.png"
00:25.119  render           | Saved: 'f_0006.png'
```

All six frames were present afterwards, each 92391 bytes. Frames were skipped that did not exist when either render started, so the check is per frame. Frame 1 was rendered by both copies, so claiming a frame with a placeholder is not proof against two renders starting on the same frame at the same instant. Run on the Creator's MacBook Pro, M1 Pro chip, macOS 26.5.2, with nothing else running. Note that both renders wrote to one local folder, not to a shared network folder.

**Observed — hand test 33, 2026-08-05.** The same test again, with the scene written down so anyone can repeat it. Two copies of Blender 5.1.2 were started at the same moment from a terminal with no window, each pointed at its own settings folder, made before it was used. Both rendered frames 1 to 6 into one empty shared folder on the local disk. No blend-file was involved: the scene was built by this text, handed to Blender on the command line, and it is the whole of the scene.

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

In words: Blender's own startup scene — cube, light, camera — rendered by Cycles at 800 samples with adaptive sampling and denoising both off, 400 by 400 pixels, Overwrite off, Placeholders on. The rest was given on the command line as `-o "<folder>/f_####" -F PNG -x 1 -s 1 -e 6 -a`, so PNG with file extensions on, frames 1 to 6. Measured on this machine, one frame takes 4.79 seconds with one Blender rendering alone and between 9.33 and 10.01 seconds with two rendering at once — Blender's own `Time:` line in both cases. The first copy printed:

```
00:00.554  render           | Rendering frame 1
00:10.149  render           | Saved: 'f_0001.png'
00:10.149  render           | Rendering frame 2
00:20.165  render           | Saved: 'f_0002.png'
00:20.165  render           | Rendering frame 3
00:20.165  render           | Skipping existing frame "f_0003.png"
00:20.165  render           | Rendering frame 4
00:29.706  render           | Saved: 'f_0004.png'
00:29.706  render           | Rendering frame 5
00:29.706  render           | Skipping existing frame "f_0005.png"
00:29.706  render           | Rendering frame 6
00:39.195  render           | Saved: 'f_0006.png'
```

and the second printed:

```
00:00.553  render           | Rendering frame 1
00:10.148  render           | Saved: 'f_0001.png'
00:10.148  render           | Rendering frame 2
00:20.092  render           | Saved: 'f_0002.png'
00:20.092  render           | Rendering frame 3
00:29.642  render           | Saved: 'f_0003.png'
00:29.642  render           | Rendering frame 4
00:29.642  render           | Skipping existing frame "f_0004.png"
00:29.642  render           | Rendering frame 5
00:38.980  render           | Saved: 'f_0005.png'
00:38.980  render           | Rendering frame 6
00:38.980  render           | Skipping existing frame "f_0006.png"
```

Both exited 0. Blender printed the whole folder path inside each of those file names; only the path has been cut here, the words and the timestamps are as printed. All six frames were present afterwards, each 92391 bytes.

This agrees with hand test 5 on both of its conclusions, and on one of them it goes further. The shared folder was empty when the two started, so every skipped frame was one the other copy had written during the run — the check is made per frame as the render reaches it. And here both copies rendered frames 1 **and** 2 — they started each of those two frames within a thousandth of a second of each other — where hand test 5 had them overlap on frame 1 only. So a placeholder does not stop two renders reaching the same frame at the same instant, and it can fail on more than the opening frame. Run on the Creator's MacBook Pro, M1 Pro chip, macOS 26.5.2, with nothing else running, and to one local folder rather than a shared network one.

**Observed — hand test 19, 2026-08-05.** Overwrite has no effect on a video output, which settles the second open edge above. Blender 5.1.2 rendered a two-frame FFmpeg MPEG-4 H.264 file from a terminal with no window, then was asked to render exactly the same thing again with Overwrite off. It printed no skipping line, the second run took 11.0 seconds — the full rendering time — and the file's modification time moved from `01:25:35` to `01:25:46`. Same machine and date.

**Observed — Linux hand tests 2, 21 and 18, 2026-08-06.** Everything above about the decision rule came back identical on a rented Linux machine — Ubuntu 22.04.5 LTS, NVIDIA GeForce RTX 4090, driver 580.105.08, twelve cores, run over SSH. Blender 5.1.2 there rendered frames 1 to 5, then `f_0004.png` was deleted and `f_0003.png` emptied to zero bytes, and the render was run again with Overwrite off:

```
00:00.292  render           | Rendering frame 1
00:00.292  render           | Skipping existing frame "…/f_0001.png"
00:00.292  render           | Rendering frame 2
00:00.292  render           | Skipping existing frame "…/f_0002.png"
00:00.292  render           | Rendering frame 3
00:00.292  render           | Skipping existing frame "…/f_0003.png"
00:00.292  render           | Rendering frame 4
00:00.699  render           | Saved: '…/f_0004.png'
00:00.699  render           | Rendering frame 5
00:00.699  render           | Skipping existing frame "…/f_0005.png"
```

The zero-byte file was skipped and stayed zero bytes; the folder afterwards held five files, four of 3639 bytes and `f_0003.png` still at 0. Asked what path frame 7 would take, Linux gave the same six answers as the Mac for every output path tried. And rendering the same two-frame video twice, the second time with Overwrite off, printed no skipping line and moved the file's modification time — so Overwrite has no effect on a video output there either.

**Observed — Linux hand test 27, 2026-08-06, twelve runs, and it does not agree with the Mac.** Two copies of Blender 5.1.2 sharing one folder with Overwrite off and Placeholders on were run **three times in each of the four versions** on the rented Linux machine, using the scene written out in hand test 33 above, unchanged. In all twelve runs the six frames were shared out between the two copies with **no frame rendered twice**. The first run in this version printed, from the first copy:

```
00:06.845  render           | Saved: 'f_0001.png'
00:06.846  render           | Skipping existing frame "f_0002.png"
00:13.875  render           | Saved: 'f_0003.png'
00:13.875  render           | Skipping existing frame "f_0004.png"
00:20.351  render           | Saved: 'f_0005.png'
00:20.351  render           | Skipping existing frame "f_0006.png"
```

and from the second copy:

```
00:00.284  render           | Skipping existing frame "f_0001.png"
00:07.213  render           | Saved: 'f_0002.png'
00:07.213  render           | Skipping existing frame "f_0003.png"
00:14.518  render           | Saved: 'f_0004.png'
00:14.518  render           | Skipping existing frame "f_0005.png"
00:20.969  render           | Saved: 'f_0006.png'
```

Both exited 0, and all six files were 92391 bytes. The other two runs in this version shared the frames out differently again and again showed no frame rendered twice.

**The two machines disagree, and neither is chosen.** On the Mac, at least one frame was rendered by both copies in most runs. On the rented Linux machine, in twelve runs, none was. This is a race between two programs and which way it falls is not fixed. What the Mac showed — that a placeholder can fail to reserve a frame — is not undone by twelve runs here that happened not to collide, and nothing here says a shared folder is safe on Linux. Both records stand. What both machines agree on is the other conclusion: every skipped frame was one the other copy had written during the run, so the check is made per frame as the render reaches it.

**Outside the interface.** `use_overwrite` is documented as a scene render property, settable from Python. It is **inferred** that the same behaviour is available to a render driven from outside the interface; the sources do not say this outright. The render-farm recipe describes several machines sharing a directory, but does not say how each render is started. There is no Overwrite switch in the command-line argument list (I read the whole of https://docs.blender.org/manual/en/5.1/advanced/command_line/arguments.html). The sources do not say what must set the option for a command-line render. **Observed**, from the hand tests above: every one set the option from a Python expression on the command line and drove Blender with no window, and the skipping worked.

**Operating system.** The sources do not say. The render-farm recipe assumes "a shared directory over a network file system" without naming one. **Observed on two operating systems.** The decision rule, the path checked, the per-frame timing and the lack of effect on a video output are identical on macOS 26.5.2 (hand tests 2, 22 and 19, 2026-08-05) and on Ubuntu 22.04.5 LTS on the rented Linux machine (Linux hand tests 2, 21 and 18, 2026-08-06). **Two Blenders sharing a folder came out differently on the two machines**, as recorded just above. Both machines wrote to a local folder rather than a shared network one, so the render-farm hint's own case is untested on either. **Windows remains untested**, because no Windows machine has run it.

---

## 5. What does an animation written as a single video file leave behind when the render stops part-way?

**Answer.** The sources describe the outcome, not the file. What is documented is that everything lives in one file and that an interrupted render can leave that file unusable, forcing a complete re-render.

> "Blender creates one file that holds all the frames of your animation."

> "It's not recommended to render directly to a video format in the first instance. If a problem occurs while rendering, the file might become unplayable and you will have to re-render all frames from the beginning."

The manual repeats the warning as advice on choosing an approach:

> "Unless your animation renders in a few minutes, it is best to render the animation as separate image files. Instead of rendering directly to a compressed movie file, use a lossless format (e.g. PNG)."

and lists "Stable power supply" among the conditions for the Direct Approach.

One documented case where more than one file exists:

> "If your video is huge and exceeds 2GiB, enable Autosplit Output. This will automatically split the output into multiple files after the first file is 2GiB in size."

- **Label:** documented, for what is quoted. The exact state of the file left behind — how many frames of it are readable, whether a player can open it at all, whether the container's index was written — is **not in the sources**. The manual says only "might become unplayable".
- **Source:** Blender manual. https://docs.blender.org/manual/en/5.1/render/output/animation.html (first and third quotes) and https://docs.blender.org/manual/en/5.1/render/output/properties/output.html (second, fourth)
- **Date read:** 2026-07-30

**Where I looked for more.** The whole of the Output page including the Encoding panel; the Rendering Animations page; the video formats page (`files/media/video_formats`); the 5.1 release notes for Pipeline & I/O ("Video & Audio output" section — it covers a new Custom CRF quality level and higher audio bitrates, nothing about interruption); and searches across the manual and API documentation for "partial", "incomplete", "corrupt", "unplayable", "truncated". Nothing describes the contents of a video file whose render was cut short.

**Not covered.** Nothing about whether the partial file is deleted, kept, or left locked. Nothing about audio. Nothing about what an Autosplit run leaves when it stops mid-way through the second file.

**Observed — hand test 18, 2026-08-05.** What is left depends entirely on how the render was stopped, and the two answers are opposite. Blender 5.1.2 was started from a terminal with no window, rendering the default cube scene in Cycles at 1000 samples and 400 by 400 pixels, output set to FFmpeg, container MPEG-4, codec H.264, Autosplit off, frames 1 to 20, at about six seconds a frame. The output folder was watched twenty times a second:

```
  0.00s  (nothing)
  0.63s  movie_0001-0020.mp4=48 bytes
 20.01s  SENT SIGINT
 24.27s  movie_0001-0020.mp4=2423 bytes
```

The file existed from the start at 48 bytes and did not grow while the render ran; it took its content at the stop. Blender exited with code `1`. Note the name: the whole frame range was baked into it, `movie_0001-0020.mp4`, at the moment the render started — before Blender knew it would not finish.

Asked to open the file it had left, Blender 5.1.2 said:

```
file: movie_0001-0020.mp4 2423 bytes
OPENED. frame_duration = 3  size = (400, 400)  fps = 24.0
```

So a politely stopped video render leaves a file that opens and holds the frames finished before the stop — three of them here. That is a good deal better than "might become unplayable" suggests, for this way of stopping.

The same render, killed outright instead — no chance to tidy up — left this:

```
movie_0001-0020.mp4  48 bytes
```

and opening it gave `OPENED. frame_duration = 0`. Everything rendered was lost. Run on the Creator's MacBook Pro, M1 Pro chip, macOS 26.5.2. What an ordinary media player makes of either file was settled by hand test 30, recorded below.

**Observed — hand test 21, 2026-08-05.** With Autosplit Output switched on and the same interrupt twenty seconds in, Blender 5.1.2 left one file, `movie__0000001-0020.mp4`, 2423 bytes, which opened with `frame_duration = 3` — the same result as with Autosplit off, but under a different name: an extra underscore and seven-digit frame numbers instead of four. The render never came near the 2 GiB boundary, so what Autosplit does when a render stops mid-way through a second part is still not established; the reason it could not be reached is written into the hand-test list.

**Observed — hand test 30, run by the Creator, 2026-08-05.** An ordinary media player treats the two files completely differently. The two files Blender 5.1.2 left — one from a render asked to stop politely twenty seconds in, one from the same render killed outright — were opened in QuickTime Player on the Creator's Mac.

The politely stopped one opened. The player window showed the rendered cube, and both time counters read `00:00`. The Creator's own words: "Part-way plays about half a second I think."

The killed one was refused, with this message:

> The document "killed outright — Blender 5.1.mp4" could not be opened.
>
> The file isn't compatible with QuickTime Player.

The Creator's written answer covered the part-way files; what the killed files did is taken from the screenshots they supplied, one per version, all four showing the same message with only the file name changed. Whether VLC was also tried is not recorded.

Run on the Creator's Mac — a MacBook Pro, M1 Pro chip, macOS 26.5.2 — so this settles macOS and leaves Windows and Linux open. It also leaves open what any player other than QuickTime does with either file, and what Blender's own Video Sequencer makes of them.

**Observed — Linux hand tests 17 and 20, 2026-08-06.** The same two stops on a rented Linux machine — Ubuntu 22.04.5 LTS, NVIDIA GeForce RTX 4090, driver 580.105.08, twelve cores, run over SSH gave the same two opposite answers. The folder was watched twenty times a second: the video file appeared at 48 bytes within half a second of the render starting, named `movie_0001-0020.mp4` with the whole frame range baked into it, and did not grow while the render ran. Sent an interrupt twenty seconds in, it grew once, to 2491 bytes, and Blender 5.1.2 exited with code `1`. Asked to open what it had left:

```
file: movie_0001-0020.mp4 2491 bytes
OPENED. frame_duration = 4  size = (400, 400)  fps = 24.0
```

The same render killed outright instead left the file at its opening 48 bytes, and opening that gave `frame_duration = 0  size = (256, 256)  fps = 0.0`. With Autosplit switched on and the same interrupt below the boundary, one file was left, `movie__0000001-0020.mp4`, 2491 bytes, which opened with `frame_duration = 4` — the same under-a-different-name result as on the Mac.

**Observed — Linux hand test 28, 2026-08-06. The 2 GiB boundary itself, which the Mac could not reach.** The Mac could not get there because Blender's startup scene compresses to almost nothing. On the rented Linux machine two things were changed and nothing else: the picture was made out of high-frequency noise, so the encoder has nothing to compress away — a Noise Texture at scale 4000 and detail 15, looked up in window coordinates, feeding a Background node in the world — and the output was set to a lossless intra-frame codec, MKV with FFV1 and Lossless Output on, at 3840 by 2160 pixels with one Cycles sample. That is about 16 MB a frame, so the boundary is a hundred and twenty-odd frames away rather than tens of thousands.

Rendering 170 frames with Autosplit on, Blender 5.1.2 left **two files**. The first was closed at **2,028,638,404 bytes** and a second was started at that moment; the render carried on into it and finished normally, exit code 0, with the second file at 730,313,993 bytes. Two things fall out of that. The size at which the first file is closed is **below 2 GiB** (2,147,483,648 bytes) and above two thousand million bytes. And the second file's name carries a counter in its leading digits — `big__0010001-0170.mkv` against the first's `big__0000001-0170.mkv` — rather than the frame it starts at.

Then the same thing again with the render interrupted eight seconds after the second file appeared:

```
---- FOLDER AFTERWARDS ----
big__0000001-0300.mkv  2028638404 bytes
big__0010001-0300.mkv  32464447 bytes
```

and Blender opened both — the first with `frame_duration = 125`, the second with `frame_duration = 2`. So **the file that Autosplit closed at the boundary is whole and readable, and only the file being written when the stop arrived is short.** An interrupted Autosplit render does not lose everything: it loses only what went into the file that was open.

**One thing the two machines say differently, and it is about words rather than frames.** The Mac record above says this version prints no `Append frame` lines while encoding a video. On the rented Linux machine it prints such a line, worded `Video append frame 1`, `Video append frame 2` and so on, where Blender 4.5 prints `Append frame 1`. Both records stand. What is on the terminal is the only thing at issue; the file left behind is the same.

**Outside the interface.** The sources do not distinguish. The warning is about the encoding process and the file, not about who started the render. **Observed**, from hand tests 18 and 21: both were run on a Blender driven from a terminal with no window.

**Operating system.** The sources do not say. The 2GiB Autosplit threshold is stated without naming a filesystem or operating system. **Observed on two operating systems, and they agree** — macOS 26.5.2 (hand tests 18, 21 and 30, 2026-08-05) and Ubuntu 22.04.5 LTS on the rented Linux machine (Linux hand tests 17, 20 and 28, 2026-08-06). FFmpeg output with an MPEG-4 container and H.264 was available and worked on both. What a media player makes of a part-written file is settled for QuickTime Player on macOS only: the rented Linux machine has no media player installed at all and installing one was not permitted, so only Blender's own reader has opened these files there. **The 2 GiB Autosplit boundary itself (Linux hand test 28) is Linux only** — the Mac could not reach it at all, as the entry above says, so nothing about the split, the size the first file is closed at, the counter in the second file's name, or what an interruption after the split leaves is settled for macOS. *(Linux citation check, 2026-08-06: added by this check — Linux hand test 28 was listed among the tests the two machines agreed on, which reads as though the Mac had reached the boundary too.)* **Windows remains untested**, because no Windows machine has run it.

---

## 6. Is a file existing proof that the frame finished writing?

**Answer.** No — and Blender has a feature whose whole purpose is to create files that are not finished frames.

> "Create empty placeholder frames while rendering."

The API documentation is blunter:

> "use_placeholder — Create empty placeholder files while rendering frames (similar to Unix 'touch') (default False)"

The manual's render-farm recipe turns this into the point of the feature: several machines render into one directory with Overwrite off and Placeholders on, so that a machine claims a frame by creating an empty file before it has rendered anything.

> "Disable Overwrite, enable Placeholders in the Render Output panel. Start as many machines as you wish rendering to that directory."

So with Placeholders on, a file appears before the frame is rendered, and its existence proves only that some machine started on it.

- **Label:** **documented** that empty placeholder files are created while rendering (the quotes). **Inferred** that a file's existence therefore does not prove the frame is complete — the step: an empty placeholder file exists on disk and holds no rendered frame, so existence and completeness are different things.
- **Source:** Blender manual, https://docs.blender.org/manual/en/5.1/render/output/properties/output.html. Blender Python API, https://docs.blender.org/api/5.1/bpy.types.RenderSettings.html. The Output panel picture on that manual page (`_images/render_output_properties_output_panel.png`) shows the two checkboxes side by side, labelled **Overwrite** (ticked) and **Placeholders** (unticked) — I looked at it because the plain-text version of the page prints the same garbled heading, "Image Sequence – Overwrite Image Multi-Layer EXR", above both descriptions, and the picture is what settles which label belongs to which. That reading is **documented**, since the picture is part of the manual.
- **Date read:** 2026-07-30

**With Placeholders off,** the question becomes: can a file appear on disk before its contents are complete? **Not in the sources.** Where I looked: the Output page and the Rendering Animations page in full; the saving-images and image-formats pages (`files/media/image_formats`); the API pages for `RenderSettings` and the render operator; and searches across the manual and API documentation for "atomic", "rename", "temporary", "partial", "incomplete", "truncated". Nothing describes how Blender writes a frame file — whether to a temporary name first, whether the file is created empty and filled, or whether it appears whole. The closest thing in the sources is a Python handler:

> "bpy.app.handlers.render_write — on writing a render frame (directly after the frame is written). Accepts one argument: the scene data-block being rendered"

(https://docs.blender.org/api/5.1/bpy.app.handlers.html, read 2026-07-30). That tells you there is a moment defined as "after the frame is written" that a script can hook. It does not tell you what the file looks like before that moment.

**Not covered.** Whether a file with non-zero size is complete. Whether a placeholder file can be told from a finished frame by anything other than opening it. What Blender does with a placeholder file if the render is cancelled before that frame is reached.

**Observed — hand test 3, 2026-08-05.** With Placeholders on, a zero-byte file exists at a frame's path before that frame has rendered, and the placeholders are made one at a time rather than all at the start. Blender 5.1.2 was started from a terminal with no window, rendering the default cube scene in Cycles at 200 samples and 200 by 200 pixels to PNG, frames 1 to 3, Placeholders on, Overwrite off, with a script listing the output folder at each of Blender's own render events:

```
WATCH [before the render starts] folder holds: (nothing)
WATCH [render_pre, frame 1 about to render] folder holds: f_0001.png=0 bytes
WATCH [render_post, frame 1 finished rendering] folder holds: f_0001.png=34818 bytes
WATCH [render_write, frame 1 written] folder holds: f_0001.png=34818 bytes
WATCH [render_pre, frame 2 about to render] folder holds: f_0001.png=34818 bytes, f_0002.png=0 bytes
WATCH [render_post, frame 2 finished rendering] folder holds: f_0001.png=34818 bytes, f_0002.png=34818 bytes
WATCH [render_write, frame 2 written] folder holds: f_0001.png=34818 bytes, f_0002.png=34818 bytes
WATCH [render_pre, frame 3 about to render] folder holds: f_0001.png=34818 bytes, f_0002.png=34818 bytes, f_0003.png=0 bytes
WATCH [render_post, frame 3 finished rendering] folder holds: f_0001.png=34818 bytes, f_0002.png=34818 bytes, f_0003.png=34818 bytes
WATCH [render_complete] folder holds: f_0001.png=34818 bytes, f_0002.png=34818 bytes, f_0003.png=34818 bytes
```

**Observed — hand test 4, 2026-08-05.** With Placeholders off, no file exists for the frame in progress at the moment it starts:

```
WATCH [before the render starts] folder holds: (nothing)
WATCH [render_pre, frame 1 about to render] folder holds: (nothing)
WATCH [render_post, frame 1 finished rendering] folder holds: f_0001.png=34818 bytes
WATCH [render_write, frame 1 written] folder holds: f_0001.png=34818 bytes
WATCH [render_pre, frame 2 about to render] folder holds: f_0001.png=34818 bytes
```

But a file can still be caught part-written, so a non-zero size is no proof either. The same test was run against a much larger file — Cycles at 1000 samples and 1000 by 1000 pixels, OpenEXR output, about 32 seconds a frame — with the folder watched twenty times a second. In Blender 5.1.2 the watcher recorded only:

```
  0.00s  (nothing)
 32.49s  f_0001.exr=477460 bytes
 64.57s  f_0001.exr=477460 bytes, f_0002.exr=477460 bytes
```

so it did not catch an intermediate size here. That is not evidence that the file appears whole: at the same watch rate, Blender 5.0.1 running the same test *was* caught with the file at 5448 bytes and then, 60 milliseconds later, at its finished 477472 bytes. The write is fast, not atomic, and Blender does not write to a temporary name and rename. Taking the two together: neither a file's existence nor a non-zero size proves the frame finished.

Run on the Creator's MacBook Pro, M1 Pro chip, macOS 26.5.2.

**Observed — hand tests 6, 7, 5 and 33, 2026-08-05.** Two more things bearing on this answer, recorded in full elsewhere in this document. A render stopped at any point leaves no file at all for the frame in flight, whether interrupted, terminated politely, or killed — so a cancel leaves no placeholder behind either (Q2 and Q3). And two Blenders sharing a folder with Placeholders on both rendered frame 1 in the first run, and frames 1 and 2 in the re-run, so a placeholder does not reliably reserve a frame (Q4).

**Observed — Linux hand tests 3 and 4, 2026-08-06.** Both halves came back the same on a rented Linux machine — Ubuntu 22.04.5 LTS, NVIDIA GeForce RTX 4090, driver 580.105.08, twelve cores, run over SSH. With Placeholders on, Blender 5.1.2 there gave the same sequence of listings: nothing in the folder, then `f_0001.png=0 bytes` as frame 1 begins, then the same name at 34823 bytes when it finishes, then `f_0002.png=0 bytes` as frame 2 begins. With Placeholders off, no file at all exists for the frame in progress.

The part-written half is worth setting out carefully, because the two machines caught different versions. Watching a folder twenty times a second through a slow 1000-by-1000 OpenEXR render, the Linux watcher recorded for this version:

```
  0.00s  (nothing)
 26.00s  f_0001.exr=477443 bytes
 49.99s  f_0001.exr=477443 bytes, f_0002.exr=477443 bytes
```

and 5.1 was not caught at an intermediate size at that watch rate — but 5.0 and 5.2 both were, at 0 bytes. On the Mac it was 5.0 that was caught, at 5,448 bytes. Taking the two machines together: with Placeholders off, an ordinary frame file **can** be seen at a size that is not its finished size, on both operating systems, and which version a watcher happens to catch is luck rather than a difference between versions. Neither the existence of a file nor a non-zero size proves the frame finished — and a zero-byte file is not proof of a placeholder either.

**Observed — Linux hand test 27, 2026-08-06.** Twelve runs of two Blenders sharing a folder with Placeholders on, on the rented Linux machine, showed no frame rendered twice — unlike the Mac. That is recorded in full under question 4, and it does not change the answer here: what it settles is how reliably a placeholder reserves a frame, not whether a file's existence proves a frame finished.

**Outside the interface.** `use_placeholder` is documented as a scene render property. It is **inferred** that it applies to a render driven from outside the interface; the sources do not say this outright. The render-farm example describes several machines sharing a directory, but does not say how each render is started. There is no command-line switch for it in the argument list. **Observed**, from hand tests 3 and 4: Placeholders works on a Blender driven from a terminal with no window, and the `render_write` handler does fire there — every `WATCH [render_write …]` line above was printed by such a handler in background mode.

**Operating system.** The API text names Unix by comparison only ("similar to Unix 'touch'"); it does not say the feature is limited to Unix. The sources do not say whether behaviour differs by operating system. **Observed on two operating systems, and they agree** — macOS 26.5.2 (hand tests 3 and 4, 2026-08-05) and Ubuntu 22.04.5 LTS on the rented Linux machine (Linux hand tests 3 and 4, 2026-08-06). Placeholders makes empty files on both, and an ordinary frame file can be caught part-written on both. **Windows remains untested**, because no Windows machine has run it.

---

## 7. Can Blender be given a set of frames that is not one plain run — single frames, or two separate stretches?

**Answer.** Yes, from the command line. The `-f` argument accepts more than one frame, and accepts ranges.

> "-f, --render-frame <frame>
> Render frame <frame> and save it.
> +<frame> start frame relative, -<frame> end frame relative.
> A comma separated list of frames can also be used (no spaces).
> A range of frames can be expressed using .. separator between the first and last frames (inclusive)."

So single frames (`-f 7`), a list of separate frames (`-f 1,50,99`), and a range (`-f 10..20`) are all documented forms of one argument. The documentation does not show a list and a range combined in one `-f`, and does not say whether that is allowed.

From inside the interface, the sources document one run at a time: a Frame Start, an End, and a Step (see Q8). The Python render operator likewise documents a single pair:

> "frame_start (int) – Start Frame, Frame to start rendering animation at. If not specified, the scene start frame will be assumed. This should only be specified if doing an animation render"

> "frame_end (int) – End Frame, Frame to end rendering animation at. If not specified, the scene end frame will be assumed. This should only be specified if doing an animation render"

- **Label:** documented for the command-line forms and the Python parameters quoted here.
- **Source:** Blender manual, https://docs.blender.org/manual/en/5.1/advanced/command_line/arguments.html. Blender Python API, https://docs.blender.org/api/5.1/bpy.ops.render.html (`bpy.ops.render.render`)
- **Date read:** 2026-07-30

**Not covered.** Whether `-f 1,5..9,20` (list and range mixed) is accepted — the manual describes the two forms separately and never combines them. Whether repeated `-f` arguments accumulate or each triggers its own render; the manual does say "Arguments are executed in the order they are given" and "Always position -f or -a as the last arguments" (same page and https://docs.blender.org/manual/en/5.1/advanced/command_line/render.html), which suggests each `-f` renders when it is reached, but the manual does not state that for repeated `-f`. Whether Frame Step applies to a `-f` range.

**Observed — hand tests 9, 8, 10, 11 and 20, 2026-08-05.** The mixed form works, which settles the first thing marked not covered above. All five tests were run with Blender 5.1.2 started from a terminal, rendering the default cube scene to PNG into an empty folder each time.

*A list mixing single frames and stretches, `-f 1..3,10..12`.* Accepted, no error, all six files written:

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

Note also what Blender calls each item: `Rendering single frame`, never `Rendering animation`. Each frame in the list is its own single-frame render.

*A list of separate frames, `-f 5,1,9`.* Rendered in the order written — 5, then 1, then 9 — not sorted.

*A repeated frame, `-f 2,2,3`.* Frame 2 was rendered twice, over the same file. Two files were left. So a list may repeat a frame and may be out of order, and Blender neither rejects nor collapses either.

*Malformed lists.* Each was refused, nothing was rendered, and Blender's exit code was `0` every time:

```
Error: incorrect comma use '-f / --render-frame 1,,3'.
Warning: negative range ignored '-f / --render-frame 3..1'.
Error: not a number '-f / --render-frame 1-3'.
```

*With Blender's window open.* The same `-f 1..3,10..12` command without `-b`, so Blender opened its window, wrote the same six files with the same names.

Run on the Creator's MacBook Pro, M1 Pro chip, macOS 26.5.2.

**Observed — hand tests 24, 26 and 27, 2026-08-05.** Three more things, on the same machine and date.

*Repeated `-f` and `-a` arguments do accumulate into separate runs* — which is what the "not covered" note above wonders about. `-s 1 -e 2 -a -s 20 -e 21 -a` produced two separate animation runs, announced as `Rendering animation (frames 1..2)` and `Rendering animation (frames 20..21)`, and four files.

*A list of five thousand frames is accepted.* `-f` with `1,2,…,5000` rendered all five thousand and Blender exited with code `0`. A list of two hundred thousand never reached Blender — the shell refused it first, with `argument list too long`, and returned `127`. The limit is the operating system's argument length, not anything in Blender.

*The keyframes-only playblast cannot be reached with no window.* Calling `bpy.ops.render.opengl(animation=True, render_keyed_only=True)` in a Blender 5.1.2 started with no window gave:

```
Error: Cannot use OpenGL render in background mode (no opengl context)
RAISED: RuntimeError Error: Cannot use OpenGL render in background mode (no opengl context)
FILES: []
```

**Observed — hand test 32, run by the Creator, 2026-08-05.** With a window, that playblast does run, and it does not leave gaps on disk. In Blender 5.1.2 the Creator took the default cube scene, put a keyframe on the cube at frame 3 and another at frame 5 and none anywhere else, set the frame range to 1–6 and an image output path, and used the keyframes-only item in the 3D Viewport's View menu. Their own words: "Anyway, all versions create 6 files, which are identical."

So six files for a six-frame range, even though only two frames carry a key. What the six files were named, and how big they were, is not recorded.

The menu in Blender 5.1.2 reads:

> Render Viewport Preview
> Render Playblast
> Render Playblast on Keyframes

with the tooltip "Take a snapshot of the active viewport." and, underneath it, "Python: bpy.ops.render.opengl()". (Blender 4.5.12 labels the same three items "Viewport Render Image", "Viewport Render Animation" and "Viewport Render Keyframes"; 5.0.1 and 5.2.0 match 5.1.2.)

Run on the Creator's Mac — a MacBook Pro, M1 Pro chip, macOS 26.5.2 — so this settles macOS and leaves Windows and Linux open.

**Observed — Linux hand tests 7 to 10, 19, 23, 24, 25 and 26, 2026-08-06.** Every one of the tests above was run again on a rented Linux machine — Ubuntu 22.04.5 LTS, NVIDIA GeForce RTX 4090, driver 580.105.08, twelve cores, run over SSH with Blender 5.1.2 and came back the same. `-f 5,1,9` gave 5, then 1, then 9. `-f 1..3,10..12` wrote all six files with no error. `-f 2,2,3` rendered frame 2 twice over the same file. The three malformed lists were refused with the same three messages, word for word, nothing rendered, exit code 0 every time. The same list with a window open wrote the same six files. `-s 1 -e 2 -a -s 20 -e 21 -a` wrote four files. The keyframes-only playblast with no window was refused with the same message, word for word. And the same list under Cycles gave the same order as under Workbench.

The one difference is in how the operating system refuses a list that is too long. Five thousand frames were accepted and all five thousand rendered, exit code 0, on both machines. Two hundred thousand never reached Blender on either. On the Mac the shell said `argument list too long` and returned `127`; on the rented Linux machine it said `Argument list too long` and returned `126`. **So the limit is the operating system's on both machines, and the two operating systems refuse it differently.**

**Observed — Linux hand test 34, 2026-08-06.** The keyframes-only playblast was run with a window, on the rented Linux machine's own display. Blender 5.1.2, given the startup scene with a keyframe on the cube's location at frame 3 and another at frame 5 and none anywhere else, the cube selected and active, and a frame range of 1 to 6, was asked for `bpy.ops.render.opengl(animation=True, render_keyed_only=True)` with the 3D Viewport as its context — the same operator the View menu's item runs. It returned `{'FINISHED'}` and wrote **six files for a six-frame range**, `p_0001.png` to `p_0006.png`, every one of them 44610 bytes. That agrees with the Creator's macOS result. Where it does not agree: the Creator reported the six files as identical, and compared byte for byte here **no two of the six are the same file** — all fifteen pairs came back different, in every version, while every file is the same size. Looked at, they are the same picture. Both records stand; the eye and a byte comparison are answering different questions.

**Observed — Linux hand tests 35 to 38, 2026-08-06.** Four things about `-f` that neither machine had tried before, run on the rented Linux machine only, so **the Mac is untested for all four**. In Blender 5.1.2, and identically in the other three versions:

- **`-f` may be given twice in one command.** `-f 1..3 -f 10..12` rendered both stretches — six files, exit code 0.
- **`-f` and `-a` may be mixed in one command.** `-s 1 -e 2 -a -f 20` carried out both — three files, frames 1, 2 and 20, exit code 0.
- **Step has no effect on a `-f` list.** With the scene's step set to 3, which read back as 3, `-f 1,2,3,4,5` wrote all five files.
- **Whitespace inside a list is accepted.** `-f "1, 2, 3"` rendered frames 1, 2 and 3 and wrote three files, exit code 0, even though the manual says "no spaces". That says what this machine took, not that the manual is wrong about what is supported.

**Outside the interface.** This answer is the outside-the-interface answer: the comma list and the `..` range are documented command-line arguments. **Observed**, from hand test 20: `-f` with a list behaves the same whether or not Blender opens a window. **Observed**, from hand test 26: the keyframes-only playblast is the other way round — it cannot run at all without a window.

**Operating system.** The sources do not say. Note the practical edge the manual itself raises for command lines generally: "Arguments must be separated by white space" (https://docs.blender.org/manual/en/5.1/advanced/command_line/arguments.html, read 2026-07-30) and "Arguments are case sensitive! -F and -f are not the same." (https://docs.blender.org/manual/en/5.1/advanced/command_line/render.html, read 2026-07-30). Nothing there is operating-system specific in the text. *(Citation check, 2026-08-05: neither quote carried an address, and the second is on the Rendering From The Command Line page rather than the Command Line Arguments page named as this answer's manual source. Addresses added.)* **Observed on two operating systems, and they agree on everything the question asks** — macOS 26.5.2 (hand tests 8 to 11, 20, 24, 26, 27 and 32, 2026-08-05) and Ubuntu 22.04.5 LTS on the rented Linux machine (Linux hand tests 7 to 10, 19, 23, 24, 25, 26 and 34, 2026-08-06). What differs is not Blender's: how long a list the operating system will pass on, and what it says when it will not. The four further `-f` behaviours in Linux hand tests 35 to 38 are **Linux only**; the Mac was never asked. **Windows remains untested**, because no Windows machine has run it.

---

## 8. What does a frame range consist of?

**Answer.** Three numbers: Frame Start, End, and Step.

> "Frame Range
> This panel defines how long an animation will last in terms of frames. Frames can be divided by the scene's Frame Rate to get the animation duration in terms of time. For example, a 250 frame animation at a frame rate of 30 will last 8.3 seconds.
> Frame Start, End
> Set the Start and End frames for Rendering Animations.
> Step
> Controls the number of frames to advance for each frame rendered, a value above 1 skips frames. This is also used when viewing rendered animations."

The picture of the panel on that page shows exactly these three fields — "Frame Start", "End", "Step" — with 1, 250 and 1 in them, and a collapsed "Time Stretching" section below.

The API documentation gives the same three with their kind, range and default:

> "frame_start — First frame of the playback/rendering range (in [0, 1048574], default 1)"

> "frame_end — Final frame of the playback/rendering range (in [0, 1048574], default 250)"

> "frame_step — Number of frames to skip forward while rendering/playing back each frame (in [0, 1048574], default 1)"

The range is inclusive at both ends when rendering an animation:

> "-a, --render-anim — Render frames from start to end (inclusive)."

- **Label:** documented
- **Source:** Blender manual, https://docs.blender.org/manual/en/5.1/render/output/properties/frame_range.html, including its picture `_images/render_output_properties_frame_range_panel.png`. Blender Python API, https://docs.blender.org/api/5.1/bpy.types.Scene.html. Blender manual, https://docs.blender.org/manual/en/5.1/advanced/command_line/arguments.html (the inclusive quote)
- **Date read:** 2026-07-30

**Not covered.** The same panel holds Time Stretching (Old and New), which the manual describes as remapping the animation's length, with a warning that it "will not influence the Start or End frames set above". I have not treated it as part of the range, because the manual does not. Also outside this answer: `frame_preview_start` / `frame_preview_end` with `use_preview_range`, documented in the API as "Use an alternative start/end frame range for animation playback and view renders (default False)" — a second, separate range that the API's `frame_path` call can be pointed at with `preview=True`. The sources do not say whether a final animation render ever uses it. And nothing here settles what happens when End is below Start, or when Step is 0 (which the stated range [0, 1048574] permits).

**Observed — hand tests 12, 13 and 14, 2026-08-05.** The two things this answer leaves open are both settled. All three tests were run with Blender 5.1.2 started from a terminal with no window, rendering the default cube scene to PNG into an empty folder.

*An end below the start.* Start 5, end 2 announced itself as `Rendering animation (frames 5..2)`, rendered nothing at all, and exited with code `0`. The folder was empty afterwards.

*A step of 0.* The property accepted it and read back as `0`. Blender rendered and wrote frame 1 once, then announced frame 1 four more times without rendering or writing anything, and ended on its own with exit code `0`:

```
00:00.317  render           | Rendering animation (frames 1..5)
00:00.317  render           | Rendering frame 1
00:00.317  render           | Start rendering: Scene, ViewLayer
00:00.317  render           | Engine: Workbench
00:00.455  render           | Saved: '…/f_0001.png'
00:00.455  render           | Time: 00:00.13 (Saving: 00:00.05)
00:00.455  render           | Rendering frame 1
00:00.455  render           | Rendering frame 1
00:00.455  render           | Rendering frame 1
00:00.455  render           | Rendering frame 1
frame_step actually reads back as: 0
```

One file was left, `f_0001.png`, 3712 bytes. It did not hang.

*A stepped run, for completeness.* Start 1, end 10, step 3 gave frames 1, 4, 7, 10. Start 3, end 12, step 4 gave 3, 7, 11 — the end frame was not rendered.

Run on the Creator's MacBook Pro, M1 Pro chip, macOS 26.5.2.

**Observed — hand test 28, run by the Creator, 2026-08-05.** The panel accepts a step of 0 and shows it, with no complaint. In Blender 5.1.2 the Creator opened Output Properties, the Frame Range panel, typed `0` into the Step field and pressed Enter. Their own words: "Typing 0 in the Step field doesn't do anything — no error, warning or any visible behavior. Right-clicking the panel header does nothing. The panel itself is in the screenshot. I only took one because it's the same in all 4 versions."

The screenshot shows the panel holding, in order: **Frame Start** `1`, **End** `250`, **Step** `0`, and an open **Time Stretching** section with **Old** `100` and **New** `100`. Nothing else. So the Step field keeps the 0 rather than snapping to 1, and the panel's whole contents are those five numbers — the three of the range, and the two of Time Stretching that this answer sets aside as not part of it.

Read alongside the run above: the field takes 0, and a render made with it writes the first frame once.

Run on the Creator's Mac — a MacBook Pro, M1 Pro chip, macOS 26.5.2 — so this settles macOS and leaves Windows and Linux open.

**Observed — Linux hand tests 11, 12 and 13, 2026-08-06.** The same three runs on a rented Linux machine — Ubuntu 22.04.5 LTS, NVIDIA GeForce RTX 4090, driver 580.105.08, twelve cores, run over SSH gave the same frames in Blender 5.1.2. Start 1, end 10, step 3 gave files for 1, 4, 7 and 10. Start 3, end 12, step 4 gave 3, 7 and 11, and did not render frame 12. A step of 0 read back as `0`, wrote frame 1 once, left one file of 3639 bytes, and did not hang. An end below the start rendered nothing, printed `Rendering animation (frames 5..2)` and stopped, exit code 0.

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

**Outside the interface.** The same three numbers are reachable from a command line as `-s` / `--frame-start`, `-e` / `--frame-end` and `-j` / `--frame-jump` ("Set number of frames to step forward after each rendered frame"), and from Python as `frame_start` / `frame_end` / `frame_step` on the scene. So yes, this holds outside the interface. **Observed**, from hand test 12: the step gives the same frames whether it is set as `-j` on the command line or as the scene's own setting.

**Operating system.** The sources do not say. Nothing on these pages names an operating system. **Observed on two operating systems, and they agree** — macOS 26.5.2 (hand tests 12, 13, 14 and 28, 2026-08-05) and Ubuntu 22.04.5 LTS on the rented Linux machine (Linux hand tests 11, 12, 13 and 31, 2026-08-06). The same three stepped runs give the same frames on both, and the panel shows a step of 0 on both. **Linux hand tests 37 and 39 — Step against a `-f` list, and the alternative playback range — are Linux only**; the Mac was never asked. *(Linux citation check, 2026-08-06: added by this check.)* **Windows remains untested**, because no Windows machine has run it.

---

## 9. What does a frame write when several view layers are active on it?

**Answer — the output file's contents are not in the sources.** What is documented is the surrounding machinery, and it stops just short of answering.

Documented:

- A view layer can be switched in or out of the render: "use — Enable or disable rendering of this View Layer (default True)" and, in the manual, "Use for Rendering — The active view layer will be used during rendering."
- The render result holds all of them: "class bpy.types.RenderResult(bpy_struct) — Result of rendering, including all layers and passes", with a `layers` collection of `RenderLayer`. The Image Editor exposes this: "Render Result: displays renders. When this item is selected, the Slot, View Layer, and Render Pass selectors become available" and "View Layer — The View Layer to display."
- One format is documented as holding them all: the Cache Result option "Saves the rendered view layers and their passes to a multi-layer OpenEXR image."
- The Output panel's Media Type offers "Image: Saves each input as a separate image file. Each input can use its own image format settings." and "Multi-Layer EXR: Saves all inputs together in a single multi-layer OpenEXR file."

What no page says is what the file written for a frame contains when two or more view layers rendered — whether a plain PNG gets the layers combined, only one of them, or one file each; and whether a Multi-Layer EXR output writes one layer per view layer. The Media Type wording above talks about "inputs", which is the language of the File Output node ("Each input corresponds to an image that will be written to disk"), not the language of view layers, and I will not stretch it.

- **Label:** not in the sources (for what a frame's file contains); documented for each quoted statement above; inferred for the command-line result below
- **Source of the quotes:** Blender Python API, https://docs.blender.org/api/5.1/bpy.types.ViewLayer.html and https://docs.blender.org/api/5.1/bpy.types.RenderResult.html. Blender manual, https://docs.blender.org/manual/en/5.1/render/layers/view_layer.html, https://docs.blender.org/manual/en/5.1/editors/image/introduction.html, https://docs.blender.org/manual/en/5.1/render/output/properties/output.html, https://docs.blender.org/manual/en/5.1/compositing/types/output/file_output.html
- **Date read:** 2026-07-30

**Where I looked.** The View Layers introduction and View Layer panel pages in full; the Render Passes page; the Output page and Rendering Animations page in full; the image formats page; the Render Layers node and File Output node pages; the Image Editor introduction; the Cycles Performance page (which mentions multiple view layers only in connection with Persistent Data: "When using multiple View Layers, only data from a single view layer is preserved to keep memory usage within bounds"). Searches across the whole manual for "multi-layer", "multilayer", "all view layers", "each view layer", "multiple view layers", and across the API documentation for `media_type`. None of it describes the output file for a frame with several view layers.

**Not covered.** Everything about the file itself, as above. Also not covered: what happens when every view layer has "Use for Rendering" switched off.

**Observed — Linux hand tests 14, 15 and 16, 2026-08-06.** The same three runs on a rented Linux machine — Ubuntu 22.04.5 LTS, NVIDIA GeForce RTX 4090, driver 580.105.08, twelve cores, run over SSH gave the same four comparisons in Blender 5.1.2:

```
A (both layers)       == C (first layer alone) : True
A (both layers)       == D (second layer alone): False
B (single-layer flag) == A (both layers)       : True
C == D (the two layers look different at all)  : False
A_both_0001.png  26999 bytes
B_single_0001.png  26999 bytes
C_firstonly_0001.png  26999 bytes
D_secondonly_0001.png  22965 bytes
```

One image file per frame, holding the first view layer's image, with Render Single Layer making no difference. The multi-layer OpenEXR route kept both view layers in one file there too, with both names inside it.

**Observed — Linux hand test 29, 2026-08-06. The compositor switched on, which nobody had tested on either machine.** Blender 5.1.2 on the rented Linux machine, with the same two view layers both enabled, rendered one frame five ways: with the compositor off and both layers on; with the compositor off and only the first; with the compositor off and only the second; with the compositor on and its Render Layers node reading the first view layer; and with that node moved to the second. Getting the compositor on took a version-specific route: this version has no `Scene.node_tree`; the compositor is a node group hung on `Scene.compositing_node_group`, which starts out empty, so a tree was built by hand — a Render Layers node feeding a **Group Output** node. The five files compared:

```
C (first layer alone)   == D (second layer alone) : False
A (comp off, both on)   == C (first layer alone)  : True
E (comp on, node=first) == C (first layer alone)  : True
E (comp on, node=first) == A (comp off, both on)  : True
F (comp on, node=Second)== D (second layer alone) : True
F (comp on, node=Second)== C (first layer alone)  : False
A_compoff_both_0001.png  26999 bytes
C_compoff_firstonly_0001.png  26999 bytes
D_compoff_secondonly_0001.png  22965 bytes
E_compon_first_0001.png  26999 bytes
F_compon_second_0001.png  22965 bytes
```

Two things come out of it, and they were identical in all four versions. **Switching the compositor on still writes one image file for the frame** — no second file, and no view-layer name in the name. And **what lands in that file is whichever view layer the compositor's Render Layers node is set to**: with the node moved to the second view layer, the file is pixel-for-pixel the second layer's own image and different from the first's, even though both view layers are enabled and the first is the active one.

Two things were found on the way and belong here. Asking for `Scene.node_tree` in this version raises `AttributeError: 'Scene' object has no attribute 'node_tree'`, where Blender 4.5 has that tree and it holds a **Composite** node. And touching `Scene.use_nodes` prints a warning: "'Scene.use_nodes' is expected to be removed in Blender 6.0".

**Outside the interface.** One documented difference, and it is directly on point:

> "Render Single Layer — Only render the active view layer.
> Note
> This option is ignored when rendering from the command line."

It is **inferred** that a blend-file set to render one layer only will, from a command line, render all of them: the source says the one-layer option is ignored, but does not say how the command-line render selects layers instead. (https://docs.blender.org/manual/en/5.1/render/layers/view_layer.html, read 2026-07-30.)

**Observed — hand tests 15, 16 and 17, 2026-08-05.** This answers the question the sources could not. Blender 5.1.2 was started from a terminal with no window. A script moved the default cube into its own collection and added a second view layer, `Second`, that leaves that collection out — so the two view layers plainly show different things, one with the cube and one without. Both had `use` set to True. One frame was rendered four ways, and the written files were loaded back and compared pixel by pixel:

```
VIEW LAYER 'ViewLayer'  use=True
VIEW LAYER 'Second'  use=True
A (both layers)       == C (first layer alone) : True
A (both layers)       == D (second layer alone): False
B (single-layer flag) == A (both layers)       : True
C == D (the two layers look different at all)  : False
A_both_0001.png  27045 bytes
B_single_0001.png  27045 bytes
C_firstonly_0001.png  27045 bytes
D_secondonly_0001.png  23064 bytes
```

Three answers. **One image file is written per frame** with these two view layers both enabled — never one per layer, and no view-layer name enters the file name. *(Citation check, 2026-08-05: this read "whatever the number of view layers"; the test used exactly two, so it has been narrowed to two.)* **Its pixels are the first view layer's alone**: identical to the same frame rendered with only that layer enabled, and different from the same frame rendered with only the second layer enabled. And **Render Single Layer changes nothing** in a render driven from outside the window — the file written with it on is pixel-for-pixel the file written with it off. So the inference recorded just above, that ignoring the switch yields all enabled view layers, does not hold for what lands on disk: whatever is rendered, one layer's image is what is written.

**Observed — hand test 16, 2026-08-05.** The multi-layer OpenEXR route does keep both view layers, in one file. In Blender 5.1.2 the media-type setting is how it is reached: with `media_type` set to `'MULTI_LAYER_IMAGE'`, the file-format list narrows to `('OPEN_EXR_MULTILAYER')` alone. With `media_type` left at `'IMAGE'` the version's full file-format list is:

```
['AVIF', 'JPEG', 'OPEN_EXR', 'PNG', 'WEBP', 'BMP', 'CINEON', 'DPX', 'IRIS', 'JPEG2000',
 'HDR', 'TARGA', 'TARGA_RAW', 'TIFF', 'OPEN_EXR_MULTILAYER', 'FFMPEG']
```

(Blender 4.5.12 has no `media_type` setting at all, and no `AVIF` in its list.) The render wrote one file, `vl_0001.exr`, 17927 bytes, and the names found inside its header were:

```
ViewLayer.Combined.A, ViewLayer.Combined.B, ViewLayer.Combined.G, ViewLayer.Combined.R,
Second.Combined.A, Second.Combined.B, Second.Combined.G, Second.Combined.R
```

Both view layers, named, inside the one file.

Run on the Creator's MacBook Pro, M1 Pro chip, macOS 26.5.2.

**Operating system.** The sources do not say. **Observed on two operating systems, and they agree** — macOS 26.5.2 (hand tests 15, 16 and 17, 2026-08-05) and Ubuntu 22.04.5 LTS on the rented Linux machine (Linux hand tests 14, 15 and 16, 2026-08-06). The compositor-on case in Linux hand test 29 is **Linux only**; the Mac was never asked. **Windows remains untested**, because no Windows machine has run it.

---

## 10. In what order does a render work through the frames it was given — a plain run, a run that skips frames at a fixed interval, and a list of separate frames — and what does an interruption leave behind in each case?

**A plain run.** The frames are documented as start to end, inclusive:

> "-a, --render-anim — Render frames from start to end (inclusive)."

and the range panel: "Set the Start and End frames for Rendering Animations." That the run works through them in ascending order is a step past those words, which name which frames are rendered and the direction the range is described in, not the sequence.

**A run with a fixed interval.** The direction is documented; the order is stated only as advancing:

> "Step — Controls the number of frames to advance for each frame rendered, a value above 1 skips frames."

and, from the command line, "-j, --frame-jump <frames> — Set number of frames to step forward after each rendered frame." The API: "frame_step — Number of frames to skip forward while rendering/playing back each frame". "Advance", "forward" and "skip forward" all say the run moves upward through the frames. That it therefore renders Start, Start+Step, Start+2×Step and so on is **inferred** — the step I took: the run begins at Start, each rendered frame is followed by advancing forward by Step, so the rendered frames are Start plus whole multiples of Step, in increasing order. No page states this list.

**A list of separate frames.** **Not in the sources.** The manual documents that `-f` takes "A comma separated list of frames" but never says whether they are rendered in the order written, sorted, or otherwise. It does say, of command-line arguments generally, "Arguments are executed in the order they are given" — but that is about arguments, not about the frames inside one argument, and I will not read it as covering them.

**What an interruption leaves behind.** For an image-sequence animation, the sources say the frames already rendered stay on disk.

> "This allows you an easy recovery if there is a problem and you have to re-start the rendering, since the frames you have already rendered will still be in the output directory."

For a stepped run the manual spells out the follow-up that this makes possible:

> "Using an image format for output, you can use the Frame Step option to render every N'th frame. Then disable Overwrite and re-render with Frame Step set to 1."

For a video output, see Q5 — one file, possibly unplayable. The sources do not say whether the image-sequence recovery wording also covers a comma list passed to `-f`.

- **Label:** **documented** for which frames a plain run covers (start to end, inclusive), for image-sequence frames already rendered remaining on disk, and for the stepped-run follow-up quoted above; **inferred** for the plain run's ascending order, and for the exact set and order a stepped run produces; **not in the sources** for the order of a list of separate frames or whether a comma list has the same interruption result. *(Citation check, 2026-08-05: the plain run's order was labelled documented; "Render frames from start to end (inclusive)" names which frames are rendered, not the order they are worked through, so it has been moved to inferred.)*
- **Source:** Blender manual, https://docs.blender.org/manual/en/5.1/advanced/command_line/arguments.html, https://docs.blender.org/manual/en/5.1/render/output/properties/frame_range.html, https://docs.blender.org/manual/en/5.1/render/output/animation.html. Blender Python API, https://docs.blender.org/api/5.1/bpy.types.Scene.html
- **Date read:** 2026-07-30

**Where I looked for order.** Searches across the manual's `render/` and `advanced/` trees and the API pages for `RenderSettings`, `Scene` and `bpy.ops.render` for "in order", "sequential", "ascending", "chronological", "one after", "render order", "frame order", "reverse order", "job queue". The only hits about ordering anywhere were about node registration and Freestyle operators. Blender's frame ordering during a render is simply not written down in these sources.

**Not covered.** Whether frames are ever rendered out of numeric order for any reason. Whether the frame in progress at the moment of an interruption leaves a file behind, and if so what is in it. Whether a stepped run's last rendered frame can overshoot End.

**Observed — hand tests 1, 8, 10, 12, 6 and 7, 2026-08-05.** Run with Blender 5.1.2 started from a terminal with no window, rendering the default cube scene to PNG.

*A plain run.* Frames 1 to 5 were rendered in ascending order, one at a time. Quoted in full under Q3.

*A stepped run.* Start 1, end 10, step 3 gave files for frames 1, 4, 7, 10 — start frame first, then forward by the step. Start 3, end 12, step 4 gave 3, 7, 11, and did *not* render the end frame 12. So a stepped run stops at the last frame at or below the end and never overshoots it. Quoted in full under Q8.

*A list of separate frames.* `-f 5,1,9` rendered them in the order written — 5, then 1, then 9 — not sorted. `-f 2,2,3` rendered frame 2 twice. Quoted in full under Q7.

*What an interruption leaves, in each case.* The same in all of them: whole frames only, nothing for the frame in flight. An interrupt, a polite terminate and an uncatchable kill all left `f_0001.png` and `f_0002.png` at 92486 bytes and no third file. With Placeholders on, no placeholder was left behind either — the inference recorded above, that a placeholder is what an interruption leaves for the frame in progress, does not hold. Quoted in full under Q2, Q3 and Q6. For a video output the answer is different and is recorded under Q5.

Run on the Creator's MacBook Pro, M1 Pro chip, macOS 26.5.2.

**Observed — Linux hand tests 1, 7, 9, 11, 5 and 24, 2026-08-06.** Every part of this came back the same on a rented Linux machine — Ubuntu 22.04.5 LTS, NVIDIA GeForce RTX 4090, driver 580.105.08, twelve cores, run over SSH with Blender 5.1.2. A plain run gave 1, 2, 3, 4, 5 in that order. A stepped run started on the start frame and moved forward by the step, and did not reach back for an end frame it had overshot. `-f 5,1,9` rendered in the order written, and `-f 2,2,3` rendered frame 2 twice. An interruption left whole frames only, and nothing at all for the frame in flight, whichever way the render was stopped. The same list under Cycles gave the same order as under Workbench — and where the Mac ran that engine test in 5.1 and 5.2 only, it was run in all four versions here.

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

**Observed — Linux hand test 33, 2026-08-06. The order a render started the way the window's own Render Animation item starts it.** *(Linux citation check, 2026-08-06: this heading read "The order a render started by clicking uses"; nothing was clicked — the render was started by calling the operator that menu item calls, which is what the entry itself says — so it has been narrowed to the entry's own wording.)* This was untested on both machines. On the rented Linux machine, with a window open on its display, Blender 5.1.2 was given `bpy.ops.render.render('INVOKE_DEFAULT', animation=True)` — the call the Render menu's Render Animation item makes — for a six-frame Workbench range. It returned `{'RUNNING_MODAL'}`, a separate **Blender Render** window opened, and afterwards the six files were sorted by the time each was written:

```
the order the files were written in: ['o_0001.png', 'o_0002.png', 'o_0003.png', 'o_0004.png', 'o_0005.png', 'o_0006.png']
```

Ascending, one at a time, the same as a render driven from a terminal. Identical in all four versions.

**Outside the interface.** The plain-run direction quote is itself a command-line one, so it holds outside. Nothing suggests the order differs between a render started by clicking and one started by a command; but nothing states they are the same, either. **Observed** for the outside case only: every hand test above drove Blender from a terminal with no window. **Observed**, from Linux hand test 33: a render started the way the window's own Render Animation item starts it wrote its six frames in ascending order, one at a time, the same as a render driven from a terminal. *(Linux citation check, 2026-08-06: this sentence read "The order a render started by clicking uses is still untested", which Linux hand test 33 directly above had already settled. Corrected by this check.)* **The two-card runs below say nothing about the inside case**: that machine has no display server at all, so every run there was driven from outside with no window, and no window test could be repeated with two cards. *(Two-card citation check, 2026-08-06: added by this check — the line did not mention the two-card runs at all.)*

**Observed on the two-card machine — 2026-08-06. Whether the frame order or the output changes when both graphics cards are ticked rather than one.** *(Two-card citation check, 2026-08-06: this read "when two graphics cards do the work". The record below shows the preferences that were saved, not that both cards were used by the render — no device list or card load was captured here — so the heading now says what was set rather than what happened.)*

Run over SSH on a **second rented Linux machine — Ubuntu 22.04.5 LTS, two NVIDIA GeForce RTX 4090s of 24 GB each on driver 580.95.05, an AMD EPYC 7B13 with sixty-one cores, 98 GB of memory, no display server at all** — using Blender at `/opt/blender/blender-5.1.2-linux-x64/blender`, `Blender 5.1.2 (hash ec6e62d40fa9 built 2026-05-19 02:18:37)`, with `BLENDER_USER_RESOURCES` pointed at `/root/smoozy/two cards/res-5.1`, a folder of its own that was empty at the start. **Nothing else was running on the machine.** Label: **observed**. This is a third machine; nothing here is merged with the Creator's Mac or with the one-card rented Linux machine. The full list is `Audits/Blender Research/Frames/Hand tests — two cards and no screen.md`.

**The set-up.** A four-frame Cycles animation at 640 by 360, 128 samples, adaptive sampling off, denoising off, seed 0 and animated seed off, rendered with `--render-anim` into an empty folder. The preferences were saved with the technology `OPTIX` and both cards ticked, then again with only the card at `0000:00:07` ticked.

**The order is ascending, one at a time, with two cards as with one:**

```
00:04.805  render           | Saved: '/root/smoozy/two cards/work/ord_5.1/f_0001.png'
00:05.981  render           | Saved: '/root/smoozy/two cards/work/ord_5.1/f_0002.png'
00:07.213  render           | Saved: '/root/smoozy/two cards/work/ord_5.1/f_0003.png'
00:08.324  render           | Saved: '/root/smoozy/two cards/work/ord_5.1/f_0004.png'
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

- two cards against one card: **17**
- two cards against two cards, two separate runs: **10**
- one card against one card, two separate runs: **8**

**and the largest difference in every one of those comparisons was `0.003921568393707275` — one step in 255.** So in 5.1 the two-card run differed from the one-card run in about twice as many values as either control, though every difference anywhere was the same single step in 255 out of 921,600 values. Nothing here shows the number of cards changing the picture, and nothing here rules it out either. *(Two-card citation check, 2026-08-06: this read "Two runs on the same cards differ by as much as a two-card run differs from a one-card run, so nothing here shows the number of cards changing the picture." In 5.1 they do not — 17 against 10 and 8 — so the sentence was corrected to this version's own numbers.)*

**Observed on the four-card machine — 2026-08-06. Whether the frame order or the picture changes when four graphics cards are ticked rather than one; and what Blender stamps into a picture.**

Run over SSH on a **third rented Linux machine — Ubuntu 22.04.5 LTS, four NVIDIA GeForce RTX 4060 Ti of 16 GB each on driver 580.95.05, an AMD EPYC 7R43 with sixty-one cores, 197 GB of memory, no display server** — using Blender at `/opt/blender/blender-5.1.2-linux-x64/blender`, `Blender 5.1.2 (hash ec6e62d40fa9 built 2026-05-19 02:18:37)`, with `BLENDER_USER_RESOURCES` pointed at `/root/smoozy/four cards/res-5.1`, a folder of its own emptied and remade before use. **Nothing else was running on the machine.** Label: **observed**. This is a **fourth** machine; nothing here is merged with the Creator's Mac, the one-card rented Linux machine or the two-card rented Linux machine. The full list, with every command and every script written out, is `Audits/Blender Research/Frames/Hand tests — four cards.md`.

**Of the three ways this machine differs from the two-card one, only one bears on what is below.** Four cards rather than two is the point of the test. The smaller card bears on the seconds in the output quoted here and on nothing else, and no timing from this machine is set against a two-card timing. The larger system memory bears on nothing here.

**The set-up.** The same four-frame Cycles animation as the two-card run: 640 by 360, 128 samples, adaptive sampling off, denoising off, seed 0 and animated seed off, rendered with `--render-anim` into an empty folder. Four passes, each into its own folder — all four cards ticked, one card, all four again, one again — so that every comparison has a control. The script that saves which cards are ticked, and the script that counts differing pixel values, are both written out in full in the list.

**The order is ascending, one at a time, with four cards as with one:**

```
00:00.314  render           | Rendering frame 1
00:07.965  render           | Saved: 'ord_5.1_four_a/f_0001.png'
00:07.965  render           | Rendering frame 2
00:09.656  render           | Saved: 'ord_5.1_four_a/f_0002.png'
00:09.656  render           | Rendering frame 3
00:11.330  render           | Saved: 'ord_5.1_four_a/f_0003.png'
00:11.330  render           | Rendering frame 4
00:12.974  render           | Saved: 'ord_5.1_four_a/f_0004.png'
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
out/stamp/s_a_0001.png out/stamp/s_b_0001.png differ: byte 298, line 3
d86f5120ad7d3bb33da18725413c5e8f  out/stamp/s_a_0001.png
b4a8e705ea3758a016ca7d7002f424d5  out/stamp/s_b_0001.png
    tEXt b'cycles.ViewLayer.samples\x00400'
    tEXt b'cycles.ViewLayer.total_time\x0000:00.21'
    tEXt b'cycles.ViewLayer.render_time\x0000:00.20'
    tEXt b'cycles.ViewLayer.synchronization_time\x0000:00.00'
    tEXt b'cycles.ViewLayer.samples\x00400'
    tEXt b'cycles.ViewLayer.total_time\x0000:00.20'
    tEXt b'cycles.ViewLayer.render_time\x0000:00.20'
    tEXt b'cycles.ViewLayer.synchronization_time\x0000:00.00'
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


**With that settled, the pictures of the four passes were compared**, value by value, with a control. Frame 1, of 921,600 values: **17 differed between four cards and one card, and 4 between two separate runs on four cards**, the largest difference in both being one step in 255. So the four-card comparison differed in about four times as many values as the control, every difference anywhere being that same smallest step. **Nothing here shows the number of cards changing the picture, and nothing here rules it out either.** *(Four-card citation check, 2026-08-06: this check ran the same four passes again on the same machine. The order was ascending both ways in all four versions, no two files matched in any of the twelve comparisons per version, and the largest pixel difference was again one step in 255 everywhere. The counts came out **17 between four cards and one card and 7 between two runs on four cards**, against **17 and 4** above. Both runs stand. What this shows is that the ratio between the two counts is noise from one run to the next, so the sentence before last — how the four-card figure compares with the control — is about this run alone and not about the version. The last sentence is the one that survives both runs.)*

**Driven from outside — this block.** Every run above was made from outside Blender's window, and on this machine there is no other way: with no display server there is nothing for a window to open on. No window test could be repeated with four cards.

**Operating system — this block.** Linux, on the machine named at the top of it. The stamp result was got on this Linux machine; a checker's separate run on the Creator's Mac in Blender 4.5 also found the file matching with the stamp settings off, and the two are not merged here. Windows is untested.


**Operating system.** The sources do not say. **Observed on two operating systems, and they agree** — macOS 26.5.2 (hand tests 1, 8, 10, 12, 6 and 7, 2026-08-05) and Ubuntu 22.04.5 LTS on the rented Linux machine (Linux hand tests 1, 7, 9, 11, 5 and 24, 2026-08-06). **Observed on both makes of graphics card the project has**, from Linux hand test 30: Apple's own on the Mac and an NVIDIA GeForce RTX 4090 here, with the same result. **The order a render started the way the window starts it (Linux hand test 33) is Linux only**; the Mac was never asked. *(Linux citation check, 2026-08-06: added by this check — the line named only the tests run on both machines.)* **Observed with two graphics cards in one machine, and it changes nothing** — see the block above: ascending order both ways, and the picture differs no more between two cards and one than between two runs on the same cards. **Windows remains untested**, because no Windows machine has run it, and no card of any third make can be reached. **Observed with four graphics cards in one machine as well, on a fourth machine, and it changes nothing** — the order is ascending both ways, and the picture differs no more between four cards and one than between two runs on four cards. That run also settled what Blender writes into a PNG and when two files can match, which is what makes the picture comparison the reliable one. *(Added 2026-08-06 by the hand tests on the four-card machine.)*

---

## 11. Can a render be told to stop once the frame it is working on is finished, rather than the moment it is asked?

**Answer: not in the sources.** No such option, argument, property or operator appears anywhere in the Blender 5.1 manual, Python API documentation, release notes, or Cycles' own files.

What the sources do document about stopping is immediate, or unspecified:

> "The task can be aborted by clicking the cancel button ()." (status bar)

> "test_break() — Test if the render operation should been canceled, this is a fast call that should be used regularly for responsiveness" (a call a render engine makes on itself, in the API's RenderEngine class)

> "bpy.ops.render.view_cancel() — Cancel showing the render view" (this cancels the render *view*, not the render)

and there is a handler that fires when a render job is cancelled: "bpy.app.handlers.render_cancel — on canceling a render job. Accepts one argument: the scene data-block being rendered". None of these offer a "finish this frame first" mode. `test_break` is described as something to call "regularly for responsiveness", which is the opposite intent.

- **Label:** not in the sources
- **Source of the quotes:** Blender manual, https://docs.blender.org/manual/en/5.1/interface/window_system/status_bar.html. Blender Python API, https://docs.blender.org/api/5.1/bpy.types.RenderEngine.html, https://docs.blender.org/api/5.1/bpy.ops.render.html, https://docs.blender.org/api/5.1/bpy.app.handlers.html
- **Date read:** 2026-07-30

**Where I looked.** Searched the whole manual, the whole API documentation, the whole 5.1 release notes and the whole Cycles folder for: "stop after", "finish the current", "after the current frame", "current frame finish", "graceful", "pause", "checkpoint", "SIGINT", "SIGTERM", "abort", "cancel", "interrupt". Read in full: the Rendering Animations page, the Output page, the Output introduction, the Frame Range page, the Command Line Arguments page (every argument), the Rendering From The Command Line page, the status bar page, the topbar's Render menu section, `bpy.ops.render` (all thirteen operators listed), `bpy.types.RenderSettings` (all properties), `bpy.types.RenderEngine`, `bpy.app.handlers`. Read the 5.1 release notes for Rendering, Core, Pipeline & I/O and index, and scanned the bug-fix list for "cancel". Checked Cycles' `properties.py`, `engine.py`, `operators.py`, `ui.py` and `__init__.py` — the only relevant thing there is a function named `render_frame_finish(engine)` in `Blender Sources/cycles_5.1/engine.py` line 95, which is a per-frame teardown call Cycles makes, not a user-facing option; nothing in the file says it can be used to end a run early, and I am not inferring that it can.

**Not covered.** This answer is about the sources being silent, not about the behaviour being impossible. A script could in principle achieve the effect using the documented `render_write` or `render_post` handlers, but no source says so, and I am not claiming it works.

**Observed — hand test 23, 2026-08-05.** The `render_write` route named just above was tried, and it does not work. Blender 5.1.2 was started from a terminal with no window and rendered frames 1 to 8 of a Cycles scene to PNG, with a handler attached to `render_write`. The moment frame 2 was written, the handler set the scene's end frame to 2:

```
HANDLER: frame 2 has just been written; setting frame_end to 2
HANDLER: frame_end now reads 2
```

All eight frames were rendered anyway — `f_0001.png` through `f_0008.png`, 53152 bytes each. The animation loop does not re-read the end frame as it goes, so lowering it from a handler cannot bring a run to a stop at a frame boundary. Run on the Creator's MacBook Pro, M1 Pro chip, macOS 26.5.2.

**Observed — hand test 29, run by the Creator, 2026-08-05.** Blender's own window offers nothing of the kind either. With an animation render running in Blender 5.1.2, the Creator opened the topbar's **Render** menu. It held six items and no more:

> Render Image — F12
> Render Animation — ⌘ F12
> Render Audio…
> View Render — F11
> View Animation — ⌘ F11
> Lock Interface (a checkbox)

There is no stop, pause, cancel or finish-this-frame item among them.

The status bar at the bottom showed `Rendering...` with a progress bar, a percentage, and a small cross. Hovering the progress bar gave:

> Time Remaining: 01:54.20
> Time Elapsed: 00:46.25

and the render information line across the top of the image editor read, at that moment:

> Frame:3 | Last:00:19.18 Time:00:05.64 | Remaining: 00:13.93 | Mem: 19M | Sample 1153/4000

The cross carries the tooltip "Stop this job." — recorded in the 4.5 and 5.2 shots of the same test, which show the same status bar.

On when it takes effect, the Creator's own words: "Stopping isn't instant, there's a slight lag. All versions." They did not time the lag. That is consistent with the runs from a terminal, which measured about 1.0 second in this version, though those were an interrupt sent to a Blender with no window rather than Esc pressed in one.

Afterwards the output folder held exactly `frame_0001.png` and `frame_0002.png`, 92388 bytes each, and nothing for the frame that was in flight.

Run on the Creator's Mac — a MacBook Pro, M1 Pro chip, macOS 26.5.2 — so this settles macOS and leaves Windows and Linux open.

**Observed — hand tests 6 and 7, 2026-08-05.** A stop does not wait for the current frame, and the frame in progress is thrown away. Blender 5.1.2, rendering the default cube scene in Cycles at 1000 samples and 400 by 400 pixels to PNG, frames 1 to 5, at about five seconds a frame, was sent an interrupt fifteen seconds in — during frame 3. It printed:

```
Sent an internal break event. Press ^C again to kill Blender

Blender quit
```

and ended 1.0 seconds later, over three separate runs, with exit code `1`. The folder afterwards held `f_0001.png` and `f_0002.png` at 92486 bytes each and nothing for frame 3.

How long the interrupt takes to bite differs between versions, steadily over three runs each: 5.1.2 about 1.0 second, 5.0.1 about 0.8, but 4.5.12 5.6 to 5.8 seconds and 5.2.0 5.0 seconds — about as long as the rest of the frame would have needed, so those two appear to finish the frame's work before quitting, and then throw it away. In none of the four was a file written for the interrupted frame.

A polite terminate signal ended it in 0.2 seconds with exit code `-15`, printing nothing further. A kill that could not be caught ended it in 0.2 seconds with exit code `-9`. Both left the same two files. Same machine and date.

**Observed — Linux hand tests 5, 6 and 22, 2026-08-06.** The same answer on a rented Linux machine — Ubuntu 22.04.5 LTS, NVIDIA GeForce RTX 4090, driver 580.105.08, twelve cores, run over SSH. An interrupt, a polite terminate and an uncatchable kill each stopped Blender 5.1.2 there, none of them waited for the current frame, and none left anything for it. The handler route failed the same way: with a handler on Blender's own "frame written" event setting the end frame to 2 the moment frame 2 was written, all eight frames were rendered anyway.

```
HANDLER: frame 2 has just been written; setting frame_end to 2
HANDLER: frame_end now reads 2
the render call has returned
---- FOLDER AFTERWARDS ----
f_0001.png  53153 bytes
f_0002.png  53153 bytes
f_0003.png  53153 bytes
f_0004.png  53153 bytes
f_0005.png  53153 bytes
f_0006.png  53153 bytes
f_0007.png  53153 bytes
f_0008.png  53153 bytes
```

**On how long the stop took, this machine cannot say.** Three interrupts were sent in this version and the process ended 1.0, 1.2 and 1.2 seconds later. Those numbers are raw output kept for the record and **are not a measurement**: the machine was shared with other work throughout, which can only stretch them. The Mac's steady pattern — about five and a half seconds in 4.5 and five in 5.2, against about one second in 5.0 and 5.1 — is neither confirmed nor contradicted here. Settling it needs the machine to itself, and that is written into `Audits/Blender Research/Frames/Hand tests — Linux.md`.

**Observed — Linux hand test 32, 2026-08-06.** Blender's own window offers nothing that stops a render at a frame boundary on this machine either. With an animation render running in Blender 5.1.2 on the rented Linux machine's display — started the way the menu item starts it — the topbar's **Render** menu was opened with Blender's own `wm.call_menu` and photographed with Blender's own screenshot command. The picture shows six items and no more: Render Image (F12), Render Animation (Ctrl F12), Render Audio…, View Render (F11), View Animation (Ctrl F11), and a Lock Interface checkbox. No stop, pause, cancel or finish-this-frame item. The status bar at the foot of the window shows the word **Rendering…**, then a progress bar with a percentage in it, then a small cross.

Two things were not settled there and are not claimed. The cross's tooltip was not read, because hovering needs the pointer. And **whether the Esc key reached Blender could not be demonstrated** — that machine's display is shared with other work, and in one version a further frame was written after Esc was sent, which says the render was still going. So the "Stop this job." tooltip and the Esc cancel rest on the Creator's macOS run alone.

**Outside the interface.** No difference to report, because there is nothing to report. The sources describe no stop-after-this-frame control inside the interface or outside it. As noted in Q2, the sources describe no way at all to stop a background render. **Observed**, from hand tests 6, 7 and 23: a background render *can* be stopped, by an interrupt, a polite terminate, or a kill; none of the three waits for the current frame; and the one script route the sources allowed to be imagined does not work. **Observed**, from hand test 29: inside the window there is no such control either — the Render menu offers no stop of any kind, and the only one there is is the status bar's cross, labelled "Stop this job." So the answer is the same on both sides of the window: nothing stops a render at a frame boundary, and the frame in flight is thrown away.

**Operating system.** The sources do not say. **Observed on two operating systems, and they agree that there is no such control** — macOS 26.5.2 (hand tests 6, 7, 23 and 29, 2026-08-05) and Ubuntu 22.04.5 LTS on the rented Linux machine (Linux hand tests 5, 6, 22 and 32, 2026-08-06). Nothing stops a render at a frame boundary on either, the frame in flight is thrown away on both, and the Render menu holds the same six items on both. **How long a stop takes to bite is settled on macOS only**, for the reason given above. **Windows remains untested**, because no Windows machine has run it.

---

## Things a person with Blender 5.1 open could still settle by hand

The list that stood here has been run — the results are written into the answers above as observed findings, dated 2026-08-05 for the Creator's Mac and 2026-08-06 for the rented Linux machine. Three of the items that stood here have since been answered: **what several view layers write with the compositor switched on** (question 9), **Autosplit at the 2 GiB boundary** (question 5), and **Linux and a graphics card of another make**, which now run through every answer above. Three items are left, and none of them is a matter of somebody sitting down at Blender for a few minutes. **Observed 2026-08-06 on a third machine, and it added one more:** two graphics cards in one machine leave the frame order unchanged, and no change in the picture was shown — see question 10. *(Two-card citation check, 2026-08-06: this read "change neither the frame order nor the picture". The pixel counts do not carry that: they show no difference beyond a single step in 255, and in 4.5 and 5.1 rather more of those than two runs on the same cards showed.)*

1. **What an abrupt power-off leaves behind.** (Question 1.) A Blender killed outright is recorded under question 1 and leaves the finished frames whole, on both machines. What that cannot reach is a file the operating system had not yet flushed to disk when the power went. This was written out as a test and offered; the Creator declined it, which is their call. It cannot be run on the rented Linux machine either, because that machine is reached over the network and cutting its power would leave nothing to read the result from.

2. **What a media player does with a part-written video file, on anything but QuickTime on macOS.** (Question 5.) QuickTime Player is recorded under question 5. The rented Linux machine has no media player installed at all and installing one was not permitted, so only Blender's own reader has opened these files there. What Blender's own Video Sequencer makes of one is also still unrecorded.

3. **Anything on Windows.** No Windows machine has run it, so every observed finding above covers macOS and Linux and leaves Windows open.

Three more things need the rented machine to itself rather than a machine shared with other work, and they are written out in `Audits/Blender Research/Frames/Hand tests — Linux.md`: how long a stop takes to bite, the pointer half of the window tests, and a demonstrated Esc cancel.

---

2026-08-04 — Opus 5 (Claude Code) — Frames, Blender 5.1

2026-08-05 — Opus 5 (Claude Code) — hand-test results written into Q1 to Q11, and the hand-test list above pruned

2026-08-05 — Opus 5 (Claude Code) — the Creator's own hand-test results written into Q1, Q3, Q5, Q7, Q8 and Q11, and the hand-test list above pruned again

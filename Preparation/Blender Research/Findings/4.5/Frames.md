# Frames — Blender 4.5

**Sources consulted, and the date.** Blender 4.5 manual (`Blender Sources/manual_4.5`, including two pictures from its `_images` folder), Blender 4.5 Python API documentation (`Blender Sources/api_4.5`), Blender 4.5 release notes (`Blender Sources/release_notes_4.5`), and Cycles' own files (`Blender Sources/cycles_4.5`). All were read on 2026-07-30 (the snapshot date), and the work was done on 2026-08-04.

I read only the question sheet and files inside Blender Sources/. I did not open any other part of this project.

A note on Cycles' files: the copy supplied for 4.5 holds the add-on's Python files and the render kernel headers only. Nothing in it governs which frames a render works through, how frames are written, or how a render is stopped. I searched it (for "frame", "overwrite", "placeholder", "resume") and found only an animated-seed description and a denoise operator that walks a frame range. So no answer below is drawn from Cycles' files; where I say so, that is what I mean.

---

## 1. Can a render that was interrupted by the computer going down be picked up from where it stopped?

**Answer.** Yes, if the animation was being written as separate image files. The manual treats a computer turning off mid-render as this case and gives the recovery: keep the already-written frames, turn Overwrite off, and render again. The sources do not describe resuming a single frame that was part-way through when the machine went down.

**Label: documented** (for the frame-level recovery).

**Exact words.**

> "Your computer accidentally turns off in the middle of rendering your movie!
>
> Unless your animation renders in a few minutes, it is best to render the animation as separate image files. Instead of rendering directly to a compressed movie file, use a lossless format (e.g. PNG).
>
> This allows you an easy recovery if there is a problem and you have to re-start the rendering, since the frames you have already rendered will still be in the output directory.
>
> Just disable the Overwrite option to start rendering where you left off."

- Source: manual, *Rendering Animations*, "Hints".
- Address: https://docs.blender.org/manual/en/4.5/render/output/animation.html
- Date read: 2026-07-30

**Not covered.** Nothing about resuming inside a frame (partial samples, partial tiles). Nothing about what happens if the machine died in the middle of writing a file — see question 6. Nothing about a movie-file render surviving a power loss; the manual's advice is to avoid that case rather than recover from it (see question 5). I also checked the manual's *Recovering Data* page (https://docs.blender.org/manual/en/4.5/troubleshooting/recover.html) — it is about recovering the blend-file after a crash, and says nothing about renders.

**Observed — hand test 7, 2026-08-05.** A computer going down cannot be reproduced from a terminal, but a Blender that is killed outright, with no chance to tidy up, is the closest thing to it. Blender 4.5.12 was started from a terminal with no window, rendering the default cube scene in Cycles at 1000 samples and 400 by 400 pixels, PNG output into an empty folder, frames 1 to 5. Fifteen seconds in, while frame 3 was rendering, the process was sent a kill it could not catch. Blender ended 0.2 seconds later. The exit code recorded was `-9`. The folder afterwards:

```
f_0001.png  94352 bytes
f_0002.png  94352 bytes
```

So the two finished frames survived intact and nothing at all was left for the frame that was in flight. Run on the Creator's MacBook Pro, M1 Pro chip, macOS 26.5.2. See also the video case in question 5, where a kill of this kind loses everything rendered so far.

**Observed — hand test 18, 2026-08-05.** The same kill, with the output set to a single video file, left a 48-byte file holding no frames. Recorded in full under question 5.

*(Linux citation check, 2026-08-06: the paragraph beginning "Still unsettled, and why" stood here as well as below, the copy below being this one with the Linux sentence added. The stale copy has been removed by this check; nothing was lost.)*

**Observed — Linux hand test 6, 2026-08-06.** The same kill, on a second machine, gave the same answer. This was run on a rented Linux machine — Ubuntu 22.04.5 LTS, NVIDIA GeForce RTX 4090, driver 580.105.08, twelve cores, run over SSH. Blender 4.5.12 was started there from a terminal with no window, rendering the default cube scene in Cycles at 1000 samples and 400 by 400 pixels, PNG output into an empty folder, frames 1 to 5. Fifteen seconds in it was sent a kill it could not catch. The exit code recorded was `-9`. The folder afterwards:

```
f_0001.png  94391 bytes
```

Fewer frames had finished than on the Mac, because the machine was shared with other work — no claim is made here about how fast anything ran. What matters is the same on both machines: every finished frame was whole, and there was nothing at all for the frame that was in flight.

**Still unsettled, and why.** A real power-off was written as hand test 31 and marked optional. The Creator declined it — "I would rather not do Test 36", 2026-08-05, that being its number on the combined list they were working from. It is their own choice, not a limit of the equipment, and it has been moved to the "cannot be run" section of the hand-test list. So the question splits in two. The half where **the program dies** is answered above: an uncatchable kill leaves every finished frame whole and nothing at all for the frame in flight, on both machines. The half where **the operating system had not yet flushed a file to disk** when the power went is not answered, and nothing short of a real power-off reaches it. It stays unanswered on the rented Linux machine too, for a different reason: that machine is reached over the network, so cutting its power would leave nothing to read the result from.

**Outside the interface.** The Overwrite setting is a scene render setting (`bpy.types.RenderSettings.use_overwrite`), not a window command, so it is carried in the blend-file and applies to a render started from outside. The manual gives an explicit "ignored when rendering from the command line" note for other settings (see question 9) and gives no such note for Overwrite. That reasoning is a step past the words, so: **inferred**. The sources do not say so outright. **Observed**, from hand tests 2 and 7 above: the whole recovery works on a Blender started from a terminal with no window — Overwrite off skips the frames already on disk (question 4), and the finished frames survive an uncatchable kill. The same holds on the rented Linux machine.

**Operating system.** The sources do not say. Nothing on this page or the Output page names an operating system. **Observed on two operating systems, and they agree.** macOS 26.5.2 on the Creator's MacBook Pro (hand test 7, 2026-08-05) and Ubuntu 22.04.5 LTS on the rented Linux machine (Linux hand test 6, 2026-08-06) both left every finished frame whole and nothing for the frame in flight after an uncatchable kill. **Windows remains untested**, because no Windows machine has run it. Nothing here depends on the graphics card: both runs used the processor, and the same test run on the rented machine's NVIDIA card (Linux hand test 30) gave the same answer.

---

## 2. Can a render that was stopped on purpose be picked up later?

**Answer.** Yes, for animations rendered to image file formats. The manual states it plainly, and names the way to do it: change the frame range so it starts at the last rendered frame. Question 1 records a separate recovery instruction for a computer turning off.

**Label: documented.**

**Exact words.**

> "Images can also be used for rendering animations which has a couple advantages. For example, when rendering animations to image file formats the render job can be canceled and resumed at the last rendered frame by changing the frame range. This is useful if the animation takes a long time to render and the computers resources are needed for something else."

- Source: manual, *Render Output → Introduction*.
- Address: https://docs.blender.org/manual/en/4.5/render/output/introduction.html
- Date read: 2026-07-30

And, for the reason to prefer image files over a movie file:

> "May need to interrupt rendering to use the computer, and want to be able to resume rendering where you left off."

- Source: manual, *Rendering Animations*, in the list of reasons to choose the Frame Sequence approach.
- Address: https://docs.blender.org/manual/en/4.5/render/output/animation.html
- Date read: 2026-07-30

**Not covered.** The sources do not say whether the frame that was in progress when you cancelled has been written or not — so "the last rendered frame" is a phrase you have to interpret by looking at the files on disk. Nothing about resuming part-way through a frame. Nothing about resuming a movie-file render.

**Observed — hand tests 6 and 7, 2026-08-05.** A render started from a terminal with no window *can* be stopped, and the frame in progress is discarded rather than written. Blender 4.5.12 was started with no window, rendering the default cube scene in Cycles at 1000 samples and 400 by 400 pixels, PNG output into an empty folder, frames 1 to 5, each frame taking about seven seconds. Fifteen seconds in — during frame 3 — the process was sent an interrupt, which is exactly what pressing Ctrl-C in a terminal does. Blender printed:

```
Sent an internal break event. Press ^C again to kill Blender

Blender quit
```

It ended 5.6 to 5.8 seconds after the interrupt (three separate runs: 5.8, 5.6, 5.6 seconds). The exit code was `1`. The folder afterwards:

```
f_0001.png  94352 bytes
f_0002.png  94352 bytes
```

So "the last rendered frame" is unambiguous here: only whole frames are on disk, and the frame that was in flight left nothing. A polite terminate signal instead of an interrupt ended it in 0.2 seconds with the exit code `-15`, printing nothing further, and left the same two files. Run on the Creator's MacBook Pro, M1 Pro chip, macOS 26.5.2.

**Observed — Linux hand tests 5 and 6, 2026-08-06.** The same on the rented Linux machine, three interrupts and one of each other kind. Blender 4.5.12 rendering the same scene there, sent an interrupt fifteen seconds in, printed exactly what it printed on the Mac:

```
Sent an internal break event. Press ^C again to kill Blender

Blender quit
```

and exited with code `1`, leaving only whole frames and nothing for the frame in flight. A polite terminate ended it with code `-15` and a kill with code `-9`, both leaving the same. **The render did not freeze**, which is worth recording because Blender 5.0's release notes carry a fixed bug called "Linux Freeze when force cancelling a CLI render"; nothing of that kind was seen here in any of the four versions.

How long each stop took to bite is kept under question 11 as raw output, and is not offered as a measurement — the machine was shared.

**Outside the interface.** Both routes are scene settings — the frame range (`frame_start` / `frame_end`) and Overwrite — so they survive into a render driven from outside, and the command line has its own `-s` / `-e` for the range (question 8). But the act of *stopping* a render is described only in interface terms (Esc, the status bar's cancel button). How a render started from outside the interface is stopped, and what that does, is **not in the sources**; question 11 records the pages and searches for stopping. **Observed**, from hand tests 6 and 7 above: an interrupt or a terminate signal both stop such a render, and both leave only whole frames behind. The same three ways of stopping behaved the same way on the rented Linux machine.

**Operating system.** The cited *Render Output → Introduction* and *Rendering Animations* pages do not name an operating system. **Observed on two operating systems, and they agree** — macOS 26.5.2 (hand tests 6 and 7, 2026-08-05) and Ubuntu 22.04.5 LTS on the rented Linux machine (Linux hand tests 5 and 6, 2026-08-06). An interrupt, a polite terminate and an uncatchable kill each stop the render on both, with the same three exit codes, and each leaves only whole frames. **Windows remains untested**, because no Windows machine has run it. **Untried rather than untestable: a Windows machine can be rented, and every one this project has read reported a Server edition — see the equipment section of `Preparation/Blender Research/Method.md`.**

---

## 3. Do frames already written survive a render ending part-way, whether it was stopped on purpose or not?

**Answer.** For an animation written as separate image files: yes. The manual says the already-rendered frames are still in the output directory, and that is the basis of both recovery routes above. For an animation written as a single video file there are no separate frames to survive, and the manual says the file itself may be unusable — see question 5.

**Label: documented.**

**Exact words.**

> "since the frames you have already rendered will still be in the output directory."

and, on why frames exist on disk at all during an animation render:

> "animations are a series of images, or frames, and are automatically saved directly out to a drive after being rendered."

- Source: manual, *Rendering Animations* (both quotes).
- Address: https://docs.blender.org/manual/en/4.5/render/output/animation.html
- Date read: 2026-07-30

**Not covered.** The manual's sentence is written about the computer turning off; it does not separately restate the case of a deliberate stop, though question 2's quote covers that. It does not say anything about the *last* file written — whether it is complete (question 6). It does not cover the compositor's File Output node, which writes its own files; the sources say only that it "writes out an image, for each frame range specified, to the filename entered" (https://docs.blender.org/manual/en/4.5/compositing/types/output/file_output.html, read 2026-07-30), and say nothing about what an interruption leaves behind there.

**Observed — hand tests 1, 6 and 7, 2026-08-05.** Frames already written do survive, whichever way the render ends. Blender 4.5.12 was started from a terminal with no window, rendering the default cube scene to PNG, and stopped part-way in three different ways: an interrupt (Ctrl-C), a polite terminate, and a kill it could not catch. In all three the folder afterwards held the finished frames, whole:

```
f_0001.png  94352 bytes
f_0002.png  94352 bytes
```

and nothing for the frame that was in flight. Run on the Creator's MacBook Pro, M1 Pro chip, macOS 26.5.2. The whole run, undisturbed, was:

```
Saved: '…/f_0001.png'
Time: 00:00.15 (Saving: 00:00.05)

Saved: '…/f_0002.png'
Time: 00:00.00 (Saving: 00:00.00)

Saved: '…/f_0003.png'
Time: 00:00.00 (Saving: 00:00.00)

Saved: '…/f_0004.png'
Time: 00:00.00 (Saving: 00:00.00)

Saved: '…/f_0005.png'
Time: 00:00.00 (Saving: 00:00.00)
```

Note what 4.5 prints and does not: a `Saved:` line per frame and nothing announcing the frames in advance. (In 5.0, 5.1 and 5.2 the same run additionally prints `Rendering animation (frames 1..5)` and `Rendering frame N`.)

**Observed — hand test 29, run by the Creator, 2026-08-05.** The same holds for a render started and cancelled inside Blender's own window. A prepared file was opened in Blender 4.5.12 — Cycles, 4000 samples, 400 by 400 pixels, PNG output into an empty folder, frames 1 to 10, about twenty-five seconds a frame. Render Animation was started, frames 1 and 2 were allowed to finish, and Esc was pressed while frame 3 was under way. Afterwards the output folder held exactly two files and nothing else:

```
94524 bytes  frame_0001.png
94524 bytes  frame_0002.png
```

The Creator ran the render and the cancel and left the folder for reading; the listing above is what was on disk afterwards. Their own words on the cancel: "You can probably look at the files yourself. Stopping isn't instant, there's a slight lag. All versions." Run on the Creator's Mac — a MacBook Pro, M1 Pro chip, macOS 26.5.2 — so this settles macOS and leaves Windows and Linux open.

**Observed — Linux hand tests 1, 5 and 6, 2026-08-06.** The same three ways of ending a render were tried on the rented Linux machine and gave the same answer: the frames already written were all there, whole, and nothing at all was left for the frame in flight. Undisturbed, the whole run printed:

```
Blender 4.5.12 LTS (hash 84afd5f785f7 built 2026-07-21 01:31:22)
Saved: '/root/smoozy/Frames/work/t01/4.5/f_0001.png'
Time: 00:00.38 (Saving: 00:00.08)

Saved: '/root/smoozy/Frames/work/t01/4.5/f_0002.png'
Time: 00:00.00 (Saving: 00:00.00)

Saved: '/root/smoozy/Frames/work/t01/4.5/f_0003.png'
Time: 00:00.00 (Saving: 00:00.00)

Saved: '/root/smoozy/Frames/work/t01/4.5/f_0004.png'
Time: 00:00.00 (Saving: 00:00.00)

Saved: '/root/smoozy/Frames/work/t01/4.5/f_0005.png'
Time: 00:00.00 (Saving: 00:00.00)


Blender quit
```

exit code 0, five files of 3583 bytes each. What 4.5 prints and does not print is the same on both machines: a `Saved:` line per frame and nothing announcing the frames in advance. One thing differs between the machines and is not about frames: the same scene gives a 3583-byte file on Linux and a 3658-byte file on the Mac. Nothing on this sheet asks about pixels; it is recorded only so nobody assumes a frame rendered on two machines is the same file.

**Observed — Linux hand test 32, 2026-08-06.** A render started the way the window's own Render Animation item starts it also leaves only whole frames. On the rented Linux machine, with a window open on its display, Blender 4.5.12 was given `bpy.ops.render.render('INVOKE_DEFAULT', animation=True)` — the call that menu item makes — which returned `{'RUNNING_MODAL'}`, and a separate **Blender Render** window opened. The render was ended part-way and the folder held exactly one whole frame, `frame_0001.png` at 94487 bytes, and nothing for the frame in flight. Note what this does **not** show: whether the **Esc** key reached Blender could not be demonstrated on that machine, because its display is shared with other work and a key press could not be shown to land in the intended window. So this is a render that was ended, not a demonstrated Esc cancel; the Esc half rests on the Creator's macOS run above.

**Outside the interface.** The mechanism described is "written to disk as each frame finishes", which is not an interface behaviour. But the sources do not state it separately for renders driven from outside. **Not in the sources** for that specific case. The cited *Rendering Animations* and *Output Properties → Output* pages do not distinguish how the render starts. **Observed**, from the hand tests above: it holds for a render started from a terminal with no window, and hand test 29 shows it holds for a render started and cancelled inside the window too — the same two whole frames, nothing for the third. So this answer no longer depends on how the render was started.

**Operating system.** The cited *Rendering Animations* page does not name an operating system. **Observed on two operating systems, and they agree** — macOS 26.5.2 (hand tests 1, 6, 7 and 29, 2026-08-05) and Ubuntu 22.04.5 LTS on the rented Linux machine (Linux hand tests 1, 5, 6 and 32, 2026-08-06). Frames already written survive on both, whichever way the render ends. **Windows remains untested**, because no Windows machine has run it.

---

## 4. Can Blender be asked to render only the frames that are not already on disk, and if so, how does it decide what counts as already there?

**Answer.** Yes — by turning off Overwrite. The manual and API both describe the setting as governing whether existing files are overwritten, and the manual's two hints use it exactly this way: to carry on where you left off, and to let several machines share one output folder.

How it decides is the weaker half. The sources say "existing files", and nothing more precise. The file a given frame corresponds to is the render output path with the frame number substituted (four padded zeros by default, or as many as there are `#` characters), and the API exposes that computed path as `frame_path()`. Putting those together — Blender looks for the file that frame *would* be written to, and skips the frame if it is there — is a step past the words.

**Label: documented** that the option exists and is used to skip frames already rendered. **Inferred** for the mechanism. The step: the setting is described only as being about "existing files while rendering", and the documentation separately defines exactly one filename per frame (path plus frame number, via the `#` padding rules and `frame_path()`); to get "it checks whether that file exists", a reader has to join those two facts. The sources never say it.

**Exact words.**

> "Overwrite
>
> Overwrite existing files when rendering.
>
> Placeholders
>
> Create empty placeholder frames while rendering."

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

- Source: manual, *Output Properties → Output*, "Image Sequence".
- Address: https://docs.blender.org/manual/en/4.5/render/output/properties/output.html
- Date read: 2026-07-30

> "use_overwrite
>
> Overwrite existing files while rendering
>
> Type:
>
> boolean, default True"

> "use_placeholder
>
> Create empty placeholder files while rendering frames (similar to Unix 'touch')
>
> Type:
>
> boolean, default False"

- Source: Python API, `bpy.types.RenderSettings`.
- Address: https://docs.blender.org/api/4.5/bpy.types.RenderSettings.html
- Date read: 2026-07-30

On what a frame's filename is:

> "When rendering an animation, the frame number is appended at the end of the file name with four padded zeros (e.g. image0001.png). You can set a custom padding size by adding the appropriate number of # anywhere in the file name (e.g. image_##_test.png translates to image_01_test.png)."

- Source: manual, *Output Properties → Output*, "File Path".
- Address: https://docs.blender.org/manual/en/4.5/render/output/properties/output.html
- Date read: 2026-07-30

> "frame_path(*, frame=-2147483648, preview=False, view='')
>
> Return the absolute path to the filename to be written for a given frame"

- Source: Python API, `bpy.types.RenderSettings`.
- Address: https://docs.blender.org/api/4.5/bpy.types.RenderSettings.html
- Date read: 2026-07-30

One 4.5 release note bears on that path:

> "`scene.render.frame_path()` used to be infallible but can now fail if there are errors processing Path Templates in the render output path."

- Source: release notes, *Python API*, "Render Paths".
- Address: https://developer.blender.org/docs/release_notes/4.5/python_api/
- Date read: 2026-07-30

The picture on the Output page shows the two checkboxes side by side, with Overwrite ticked and Placeholders unticked (`_images/render_output_properties_output_panel.png`, the "Output panel." picture on https://docs.blender.org/manual/en/4.5/render/output/properties/output.html, read 2026-07-30). **Documented**, from the picture: the panel's defaults as shipped are Overwrite on, Placeholders off — which matches the API defaults quoted above.

**Not covered.** The sources never say whether Blender checks file size, modification time, or anything but existence. They do not say what happens with Overwrite off and Placeholders off when two machines start at once. They do not say whether the check happens once at the start or once per frame. They do not say whether the check accounts for the file extension being added by File Extensions.

**Observed — hand test 2, 2026-08-05.** The test is bare existence, and an empty file counts. Blender 4.5.12 was started from a terminal with no window and rendered the default cube scene to PNG, frames 1 to 5, into an empty folder. Then `f_0004.png` was deleted outright and `f_0003.png` was emptied to zero bytes but left in place:

```
3658 f_0001.png
3658 f_0002.png
   0 f_0003.png
3658 f_0005.png
```

The same render was then run again with Overwrite off. Blender printed:

```
skipping existing frame "…/f_0001.png"
skipping existing frame "…/f_0002.png"
skipping existing frame "…/f_0003.png"
Saved: '…/f_0004.png'
skipping existing frame "…/f_0005.png"
```

and the folder afterwards:

```
3658 f_0001.png
3658 f_0002.png
   0 f_0003.png
3658 f_0004.png
3658 f_0005.png
```

The zero-byte file was skipped and stayed zero bytes. Note the wording: 4.5 prints `skipping existing frame` with a small s. (5.0, 5.1 and 5.2 print `Skipping existing frame` with a capital S, and precede each line with `Rendering frame N`.) Run on the Creator's MacBook Pro, M1 Pro chip, macOS 26.5.2.

**Observed — hand test 22, 2026-08-05.** The path a frame is checked against is the one `frame_path` returns, and it is formed exactly as the manual describes. Blender 4.5.12, asked from a terminal with no window:

```
filepath '<dir>/f_'                     -> frame_path(7) = '<dir>/f_0007.png'
filepath '<dir>/f_##_x'                 -> frame_path(7) = '<dir>/f_07_x.png'
filepath '<dir>/plain'                  -> frame_path(7) = '<dir>/plain0007.png'
filepath '<dir>/six_######'             -> frame_path(7) = '<dir>/six_000007.png'
with File Extensions off        -> frame_path(7) = '<dir>/f_0007'
frame_path(12345) = '<dir>/big_12345.png'
```

The file extension is part of the path when File Extensions is on and absent when it is off, and a frame number wider than the padding is not truncated. Same machine and date.

**Observed — hand test 5, 2026-08-05.** The check happens once per frame, immediately before that frame is rendered — not once at the start. Two copies of Blender 4.5.12 were started at the same moment from a terminal with no window, each with its own separate settings folder, both rendering frames 1 to 6 of the same slow scene into one shared folder with Overwrite off and Placeholders on. *(Citation check, 2026-08-05: what made the scene slow — its sample count, resolution and format — was not written down, so this run cannot be repeated exactly from what is here. It was run again with the scene stated, as hand test 33 below.)* The first said:

```
Saved: 'f_0001.png'
Saved: 'f_0002.png'
skipping existing frame "f_0003.png"
Saved: 'f_0004.png'
Saved: 'f_0005.png'
skipping existing frame "f_0006.png"
```

and the second said:

```
Saved: 'f_0001.png'
Saved: 'f_0002.png'
Saved: 'f_0003.png'
skipping existing frame "f_0004.png"
skipping existing frame "f_0005.png"
Saved: 'f_0006.png'
```

All six frames were present afterwards, each 94475 bytes. Two things came out of it. Frames were skipped that did not exist when either render started, so the check is made per frame as the render reaches it. And frames 1 and 2 were rendered by both copies, so claiming a frame with a placeholder is not proof against two renders starting on the same frame at the same instant. Run on the Creator's MacBook Pro, M1 Pro chip, macOS 26.5.2, with nothing else running.

**Observed — hand test 33, 2026-08-05.** The same test again, with the scene written down so anyone can repeat it. Two copies of Blender 4.5.12 were started at the same moment from a terminal with no window, each pointed at its own settings folder, made before it was used. Both rendered frames 1 to 6 into one empty shared folder on the local disk. No blend-file was involved: the scene was built by this text, handed to Blender on the command line, and it is the whole of the scene.

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

In words: Blender's own startup scene — cube, light, camera — rendered by Cycles at 800 samples with adaptive sampling and denoising both off, 400 by 400 pixels, Overwrite off, Placeholders on. The rest was given on the command line as `-o "<folder>/f_####" -F PNG -x 1 -s 1 -e 6 -a`, so PNG with file extensions on, frames 1 to 6. Measured on this machine, one frame takes 6.31 seconds with one Blender rendering alone and about 12.4 seconds with two rendering at once — Blender's own `Time:` line in both cases. The first copy printed:

```
Saved: 'f_0001.png'
Time: 00:12.74 (Saving: 00:00.06)
skipping existing frame "f_0002.png"
Saved: 'f_0003.png'
Time: 00:12.37 (Saving: 00:00.01)
skipping existing frame "f_0004.png"
Saved: 'f_0005.png'
Time: 00:12.39 (Saving: 00:00.01)
skipping existing frame "f_0006.png"
```

and the second printed:

```
Saved: 'f_0001.png'
Time: 00:12.55 (Saving: 00:00.07)
Saved: 'f_0002.png'
Time: 00:12.29 (Saving: 00:00.01)
skipping existing frame "f_0003.png"
Saved: 'f_0004.png'
Time: 00:12.40 (Saving: 00:00.01)
skipping existing frame "f_0005.png"
Saved: 'f_0006.png'
Time: 00:06.29 (Saving: 00:00.00)
```

Both exited 0. Blender printed the whole folder path inside each of those file names; only the path has been cut here, the words are as printed. The shared folder afterwards, with the size of each file and the time it was written:

```
94475 f_0001.png  22:57:22
94475 f_0002.png  22:57:34
94475 f_0003.png  22:57:35
94475 f_0004.png  22:57:47
94475 f_0005.png  22:57:47
94475 f_0006.png  22:57:53
```

This agrees with hand test 5 on both of its conclusions. The shared folder was empty when the two started, so every skipped frame was one the other copy had written during the run — the check is made per frame as the render reaches it. And frame 1 was rendered by both copies, so a placeholder does not stop two renders starting on the same frame at the same instant. The only difference from hand test 5 is which frames fell to which copy: there both copies rendered frames 1 and 2, here only frame 1. Run on the Creator's MacBook Pro, M1 Pro chip, macOS 26.5.2, with nothing else running, and to one local folder rather than a shared network one.

**Observed — hand test 19, 2026-08-05.** Overwrite has no effect on a video output. Blender 4.5.12 rendered a two-frame FFmpeg MPEG-4 H.264 file from a terminal with no window, then was asked to render exactly the same thing again with Overwrite off. It printed no skipping line, the second run took 14.5 seconds — the full rendering time — and the file's modification time moved from `01:24:48` to `01:25:03`. Same machine and date.

**Observed — Linux hand tests 2, 21 and 18, 2026-08-06.** Everything above about the decision rule came back identical on the rented Linux machine. Blender 4.5.12 there rendered frames 1 to 5, then `f_0004.png` was deleted and `f_0003.png` emptied to zero bytes, and the render was run again with Overwrite off:

```
skipping existing frame "/root/smoozy/Frames/work/t02/4.5/f_0001.png"
skipping existing frame "/root/smoozy/Frames/work/t02/4.5/f_0002.png"
skipping existing frame "/root/smoozy/Frames/work/t02/4.5/f_0003.png"
Saved: '/root/smoozy/Frames/work/t02/4.5/f_0004.png'
skipping existing frame "/root/smoozy/Frames/work/t02/4.5/f_0005.png"
```

The zero-byte file was skipped and stayed zero bytes, and the wording is the same small-s `skipping existing frame`. Asked what path frame 7 would take, Linux gave the same six answers as the Mac, for every output path tried. And rendering the same two-frame video twice, the second time with Overwrite off, printed no skipping line and moved the file's modification time — so Overwrite has no effect on a video output there either.

**Observed — Linux hand test 27, 2026-08-06, twelve runs, and it does not agree with the Mac.** Two copies of Blender 4.5.12 sharing one folder with Overwrite off and Placeholders on were run **three times in each of the four versions** on the rented Linux machine, using the scene written out in hand test 33 above, unchanged. In all twelve runs the six frames were shared out between the two copies with **no frame rendered twice**. The first run in this version printed, from the first copy:

```
skipping existing frame "f_0001.png"
Saved: 'f_0002.png'
Saved: 'f_0003.png'
skipping existing frame "f_0004.png"
Saved: 'f_0005.png'
skipping existing frame "f_0006.png"
```

and from the second copy:

```
Saved: 'f_0001.png'
skipping existing frame "f_0002.png"
skipping existing frame "f_0003.png"
Saved: 'f_0004.png'
skipping existing frame "f_0005.png"
Saved: 'f_0006.png'
```

Both exited 0, and all six files were 94433 bytes. The other two runs in this version shared the frames out differently again and again showed no frame rendered twice.

**The two machines disagree, and neither is chosen.** On the Mac, at least one frame was rendered by both copies in most runs. On the rented Linux machine, in twelve runs, none was. This is a race between two programs and which way it falls is not fixed. What the Mac showed — that a placeholder can fail to reserve a frame — is not undone by twelve runs here that happened not to collide, and nothing here says a shared folder is safe on Linux. Both records stand. What both machines agree on is the other conclusion: every skipped frame was one the other copy had written during the run, so the check is made per frame as the render reaches it.

**Outside the interface.** Overwrite and Placeholders are scene render settings stored in the blend-file, so a render started from outside uses whatever the file holds; there is no command-line switch for either in the manual's argument list (I searched *Command Line Arguments* for "overwrite" and "placeholder" and found neither). Changing them from outside therefore means a Python expression or a pre-set blend-file. **Inferred** — the step is that the argument list is presented as complete ("a full list of arguments") and does not contain them. **Observed**, from the hand tests above: every one of them set the option from a Python expression on the command line and drove Blender with no window, and the skipping worked. The same holds on the rented Linux machine.

**Operating system.** The cited *Output Properties → Output* page and `bpy.types.RenderSettings` entry do not name an operating system. The API's phrase "similar to Unix 'touch'" is an analogy for what a placeholder file is, not a statement that placeholders are Unix-only. **Observed on two operating systems.** The decision rule, the path checked, the per-frame timing and the lack of effect on a video output are identical on macOS 26.5.2 (hand tests 2, 22, 19, 2026-08-05) and on Ubuntu 22.04.5 LTS on the rented Linux machine (Linux hand tests 2, 21, 18, 2026-08-06). **Two Blenders sharing a folder came out differently on the two machines**, as recorded just above. Both machines wrote to a local folder rather than a shared network one, so the render-farm hint's own case is untested on either. **Windows remains untested**, because no Windows machine has run it.

---

## 5. What does an animation written as a single video file leave behind when the render stops part-way?

**Answer.** The sources do not say what state the file is left in, but they do say what it is worth: the manual warns the file may be unplayable and that you would have to render every frame again from the beginning. There is no partial-frame salvage described. Separately, the sources say that Autosplit Output splits a long render into more than one file at a 2 GB boundary; they do not say what a part-way stop leaves when this setting is on.

**Label: documented** for the consequence (unplayable, re-render from the start). **Not in the sources** for the exact state of the file — whether it exists, has a size, or holds the frames done so far.

**Exact words.**

> "It's not recommended to render directly to a video format in the first instance. If a problem occurs while rendering, the file might become unplayable and you will have to re-render all frames from the beginning."

and the counterpart benefit of image sequences:

> "Restart the rendering from the place (the frame) where any problem occurred."

- Source: manual, *Output Properties → Output*, "Tips".
- Address: https://docs.blender.org/manual/en/4.5/render/output/properties/output.html
- Date read: 2026-07-30

> "Autosplit Output
>
> If your video is huge and exceeds 2GiB, enable Autosplit Output. This will automatically split the output into multiple files after the first file is 2GiB in size."

- Source: manual, *Output Properties → Output*, "Encoding".
- Address: https://docs.blender.org/manual/en/4.5/render/output/properties/output.html
- Date read: 2026-07-30

> "use_autosplit
>
> Autosplit output at 2GB boundary
>
> Type:
>
> boolean, default False"

- Source: Python API, `bpy.types.FFmpegSettings`.
- Address: https://docs.blender.org/api/4.5/bpy.types.FFmpegSettings.html
- Date read: 2026-07-30

**Not covered.** Whether the file is left on disk at all; whether it has a size greater than zero; whether some players can read it; whether the container is finalised or its index written. The manual's "might become unplayable" is hedged and gives no rule for when it does and does not. Where I looked for more: the *Rendering Animations* page (Direct Approach section), the whole *Output Properties → Output* page including Encoding, *Image Formats* (https://docs.blender.org/manual/en/4.5/files/media/image_formats.html), and a search of the whole manual for "unplayable", "corrupt", "incomplete" — the only render-related hit is the one quoted.

**Observed — hand test 18, 2026-08-05.** What is left depends entirely on how the render was stopped, and the two answers are opposite. Blender 4.5.12 was started from a terminal with no window, rendering the default cube scene in Cycles at 1000 samples and 400 by 400 pixels, output set to FFmpeg, container MPEG-4, codec H.264, Autosplit off, frames 1 to 20, at about seven seconds a frame. The output folder was watched twenty times a second.

The video file appeared almost at once and did not grow while the render ran:

```
  0.00s  (nothing)
  0.53s  movie_0001-0020.mp4=48 bytes
 20.01s  SENT SIGINT
 24.02s  movie_0001-0020.mp4=2279 bytes
```

Blender exited with code `1` four seconds after the interrupt. Note the file name: Blender put the whole frame range into it, `movie_0001-0020.mp4`, at the moment the render started — before it knew the render would not finish. Blender also printed, during the run, `Append frame 1` and `Append frame 2`. (5.0, 5.1 and 5.2 print no such lines.)

Asked to open the file it had left, Blender 4.5.12 said:

```
file: movie_0001-0020.mp4 2279 bytes
OPENED. frame_duration = 2  size = (400, 400)  fps = 24.0
```

So a politely stopped video render leaves a file that opens and holds the frames finished before the stop.

The same render, killed outright instead — no chance to tidy up — left this:

```
movie_0001-0020.mp4  48 bytes
```

and opening it gave `OPENED. frame_duration = 0`. Everything rendered was lost. Run on the Creator's MacBook Pro, M1 Pro chip, macOS 26.5.2. What an ordinary media player makes of either file was settled by hand test 30, recorded below.

**Observed — hand test 21, 2026-08-05.** With Autosplit Output switched on and the same interrupt twenty seconds in, Blender 4.5.12 left one file, `movie__0000001-0020.mp4`, 2279 bytes, which opened with `frame_duration = 2` — the same result as with Autosplit off, but under a different name: an extra underscore and seven-digit frame numbers instead of four. The render never came near the 2 GiB boundary, so what Autosplit does *at* that boundary is still not established; the reason it could not be reached is written into the hand-test list.

**Observed — hand test 30, run by the Creator, 2026-08-05.** An ordinary media player treats the two files completely differently, which matches what Blender itself found. The two files Blender 4.5.12 left — one from a render asked to stop politely twenty seconds in, one from the same render killed outright — were opened in QuickTime Player on the Creator's Mac.

The politely stopped one opened. The player window showed the rendered cube, and both time counters read `00:00`. The Creator's own words: "Part-way plays about half a second I think."

The killed one was refused, with this message:

> The document "killed outright — Blender 4.5.mp4" could not be opened.
>
> The file isn't compatible with QuickTime Player.

The Creator's written answer covered the part-way files; what the killed files did is taken from the screenshots they supplied, one per version, all four showing the same message with only the file name changed. Whether VLC was also tried is not recorded.

Run on the Creator's Mac — a MacBook Pro, M1 Pro chip, macOS 26.5.2 — so this settles macOS and leaves Windows and Linux open. It also leaves open what any player other than QuickTime does with either file.

**Observed — Linux hand tests 17 and 20, 2026-08-06.** The same two stops on the rented Linux machine gave the same two opposite answers. The folder was watched twenty times a second:

```
  0.00s  (nothing)
  0.35s  movie_0001-0020.mp4=48 bytes
 20.02s  SENT SIGINT
 20.22s  movie_0001-0020.mp4=2413 bytes
```

Exit code `1`. Blender 4.5.12, asked to open what it had left:

```
file: movie_0001-0020.mp4 2413 bytes
OPENED. frame_duration = 3  size = (400, 400)  fps = 24.0
```

The same render killed outright instead left the file at its opening 48 bytes, and opening that gave `frame_duration = 0  size = (256, 256)  fps = 0.0`. With Autosplit switched on and the same interrupt below the boundary, one file was left, `movie__0000001-0020.mp4`, 2413 bytes, which opened with `frame_duration = 3` — the same under-a-different-name result as on the Mac.

**Observed — Linux hand test 28, 2026-08-06. The 2 GiB boundary itself, which the Mac could not reach.** The Mac could not get there because Blender's startup scene compresses to almost nothing. On the rented Linux machine two things were changed and nothing else: the picture was made out of high-frequency noise, so the encoder has nothing to compress away — a Noise Texture at scale 4000 and detail 15, looked up in window coordinates, feeding a Background node in the world — and the output was set to a lossless intra-frame codec, MKV with FFV1 and Lossless Output on, at 3840 by 2160 pixels with one Cycles sample. That is about 16 MB a frame, so the boundary is a hundred and twenty-odd frames away.

Rendering 170 frames with Autosplit on, Blender 4.5.12 left **two files**:

```
big__0000001-0170.mkv  2032129255 bytes
big__0010001-0170.mkv  731570522 bytes
```

The watcher recorded the moment the first was closed and the second begun:

```
  367.23s  NAMES CHANGED: big__0000001-0170.mkv=2032129255 bytes, big__0010001-0170.mkv=0 bytes
```

Exit code 0. Two things fall out of that. The first file was closed at **2,032,129,255 bytes** — which is below 2 GiB (2,147,483,648 bytes) and above two thousand million bytes. And the second file's name carries a counter in its leading digits, `0010001` against the first's `0000001`, rather than the frame it starts at.

Then the same thing again with the render interrupted eight seconds after the second file appeared:

```
---- FOLDER AFTERWARDS ----
big__0000001-0300.mkv  2032129255 bytes
big__0010001-0300.mkv  16263351 bytes
```

and Blender opened both:

```
file: big__0000001-0300.mkv 2032129255 bytes
OPENED. frame_duration = 125  size = (3840, 2160)  fps = 24.0
file: big__0010001-0300.mkv 16263351 bytes
OPENED. frame_duration = 1  size = (3840, 2160)  fps = 24.0
```

So **the file that Autosplit closed at the boundary is whole and readable, and only the file being written when the stop arrived is short**. An interrupted Autosplit render does not lose everything: it loses only what went into the file that was open.

**Outside the interface.** The warning is written about rendering generally, not about the interface, and the render output format is a scene setting. The sources do not state the case separately for a render driven from outside. **Not in the sources**. The *Rendering Animations* and *Output Properties → Output* pages named above do not distinguish how the render starts. **Observed**, from hand tests 18 and 21 and Linux hand tests 17, 20 and 28: all of them were run on a Blender driven from a terminal with no window.

**Operating system.** The cited *Output Properties → Output* and *Rendering Animations* pages do not name an operating system for this behaviour. Note the manual lists FFMPEG among the formats that "can be compiled into Blender, not available on all systems" (*Command Line Arguments*, Format Options, https://docs.blender.org/manual/en/4.5/advanced/command_line/arguments.html, read 2026-07-30) — that is about availability of the format, not about the behaviour above. **Observed on two operating systems, and they agree.** macOS 26.5.2 (hand tests 18, 21 and 30, 2026-08-05) and Ubuntu 22.04.5 LTS on the rented Linux machine (Linux hand tests 17, 20 and 28, 2026-08-06) gave the same answers for what a part-way stop leaves; FFmpeg output with an MPEG-4 container and H.264 was available and worked on both, and MKV with FFV1 worked on Linux. **The 2 GiB Autosplit boundary itself (Linux hand test 28) is Linux only** — the Mac could not reach it at all, as the entry above says, so nothing about the split, the size the first file is closed at, the counter in the second file's name, or what an interruption after the split leaves is settled for macOS. *(Linux citation check, 2026-08-06: added by this check — Linux hand test 28 was listed among the tests the two machines agreed on, which reads as though the Mac had reached the boundary too.)* **Windows remains untested**, because no Windows machine has run it.

**One thing the two machines say differently, and it is about words rather than frames.** The Mac record above says Blender 5.0, 5.1 and 5.2 print no `Append frame` lines. On the rented Linux machine those three do print such a line, worded `Video append frame 1`, `Video append frame 2` and so on, while 4.5 prints `Append frame 1`. Both records stand. What is on the terminal is the only thing at issue; the file left behind is the same.

**What a media player makes of these files stays half open.** QuickTime Player on macOS was settled by hand test 30. The rented Linux machine has no media player installed at all, and installing one was not permitted, so Blender's own reader is the only thing that has opened these files there.

---

## 6. Is a file existing proof that the frame finished writing?

**Answer.** No. Blender has a setting whose entire purpose is to create empty files for frames that have not been rendered yet, and it is the setting the manual recommends when several machines share an output folder. So a file at a frame's path can be an empty placeholder rather than a finished frame. Beyond that, the sources say nothing about whether a frame file is written in a way that makes a half-written file impossible.

**Label: inferred.** The step: the documentation says placeholders are *empty* files created *while* rendering frames; from that, a file being present cannot by itself mean the frame finished. The sources never state the conclusion in those words.

**Exact words.**

> "Placeholders
>
> Create empty placeholder frames while rendering."

- Source: manual, *Output Properties → Output*.
- Address: https://docs.blender.org/manual/en/4.5/render/output/properties/output.html
- Date read: 2026-07-30

> "use_placeholder
>
> Create empty placeholder files while rendering frames (similar to Unix 'touch')"

- Source: Python API, `bpy.types.RenderSettings`.
- Address: https://docs.blender.org/api/4.5/bpy.types.RenderSettings.html
- Date read: 2026-07-30

For the other half — that frames are written after being rendered, not before:

> "animations are a series of images, or frames, and are automatically saved directly out to a drive after being rendered."

- Source: manual, *Rendering Animations*.
- Address: https://docs.blender.org/manual/en/4.5/render/output/animation.html
- Date read: 2026-07-30

**Not covered.** This says nothing about the case where Placeholders is off. With Placeholders off, whether a file can exist and be incomplete — because the render died mid-write, or because the format writes progressively — is **not in the sources**. Nothing says whether Blender writes to a temporary name and renames, and nothing says a frame file is complete the moment it appears. Where I looked: the whole *Output Properties → Output* page, *Rendering Animations*, *Image Formats*, `bpy.types.RenderSettings` in full, `bpy.app.handlers`, and a manual-wide search for "placeholder", "overwrite", "temporary", "atomic", "incomplete", "corrupt".

The nearest thing to a completion signal in the sources is a Python handler:

> "bpy.app.handlers.render_write
>
> on writing a render frame (directly after the frame is written)"

- Source: Python API, `bpy.app.handlers`.
- Address: https://docs.blender.org/api/4.5/bpy.app.handlers.html
- Date read: 2026-07-30

**Observed — hand test 3, 2026-08-05.** With Placeholders on, a zero-byte file for a frame exists before that frame has rendered, and the placeholders are made one at a time rather than all at the start. Blender 4.5.12 was started from a terminal with no window, rendering the default cube scene in Cycles at 200 samples and 200 by 200 pixels to PNG, frames 1 to 3, with Placeholders on and Overwrite off, and a script listed the output folder at each of Blender's own render events:

```
WATCH [before the render starts] folder holds: (nothing)
WATCH [render_pre, frame 1 about to render] folder holds: f_0001.png=0 bytes
WATCH [render_post, frame 1 finished rendering] folder holds: f_0001.png=34473 bytes
WATCH [render_write, frame 1 written] folder holds: f_0001.png=34473 bytes
WATCH [render_pre, frame 2 about to render] folder holds: f_0001.png=34473 bytes, f_0002.png=0 bytes
WATCH [render_post, frame 2 finished rendering] folder holds: f_0001.png=34473 bytes, f_0002.png=34473 bytes
WATCH [render_write, frame 2 written] folder holds: f_0001.png=34473 bytes, f_0002.png=34473 bytes
WATCH [render_pre, frame 3 about to render] folder holds: f_0001.png=34473 bytes, f_0002.png=34473 bytes, f_0003.png=0 bytes
WATCH [render_post, frame 3 finished rendering] folder holds: f_0001.png=34473 bytes, f_0002.png=34473 bytes, f_0003.png=34473 bytes
WATCH [render_write, frame 3 written] folder holds: f_0001.png=34473 bytes, f_0002.png=34473 bytes, f_0003.png=34473 bytes
WATCH [render_complete] folder holds: f_0001.png=34473 bytes, f_0002.png=34473 bytes, f_0003.png=34473 bytes
WATCH [after the render returns] folder holds: f_0001.png=34473 bytes, f_0002.png=34473 bytes, f_0003.png=34473 bytes
```

**Observed — hand test 4, 2026-08-05.** With Placeholders off, the same listing shows no file at all for the frame in progress:

```
WATCH [before the render starts] folder holds: (nothing)
WATCH [render_pre, frame 1 about to render] folder holds: (nothing)
WATCH [render_post, frame 1 finished rendering] folder holds: f_0001.png=34473 bytes
WATCH [render_write, frame 1 written] folder holds: f_0001.png=34473 bytes
WATCH [render_pre, frame 2 about to render] folder holds: f_0001.png=34473 bytes
WATCH [render_post, frame 2 finished rendering] folder holds: f_0001.png=34473 bytes, f_0002.png=34473 bytes
```

The same test was then run against a much larger file — Cycles at 1000 samples and 1000 by 1000 pixels, OpenEXR output, about 43 seconds a frame — with the folder watched twenty times a second rather than at Blender's own events. In Blender 4.5.12 the watcher recorded:

```
  0.00s  (nothing)
 42.83s  f_0001.exr=477575 bytes
```

so it never caught the file at an intermediate size. That is not proof that the file appears whole: at the same twenty-times-a-second watch rate, Blender 5.0.1 running the same test *was* caught with the file at 5448 bytes and then, 60 milliseconds later, at its finished 477472 bytes. The write is fast, not atomic. Taking the two together: a file at a frame's path can be an empty placeholder, and it can also be a partly written real file. Neither its existence nor a non-zero size proves the frame finished.

Run on the Creator's MacBook Pro, M1 Pro chip, macOS 26.5.2.

**Observed — hand tests 6, 7, 5 and 33, 2026-08-05.** Three more things bearing on this answer, recorded in full elsewhere in this document. A render stopped at any point leaves no file at all for the frame in flight, whether it was interrupted, terminated politely, or killed (questions 2 and 3). Two Blenders sharing a folder with Placeholders on both rendered frames 1 and 2 in the first run and frame 1 in the re-run, so a placeholder does not reliably reserve a frame (question 4). And a frame is skipped purely because a file exists at its path, even an empty one (question 4).

**Observed — Linux hand tests 3 and 4, 2026-08-06.** Both halves came back the same on the rented Linux machine. With Placeholders on, Blender 4.5.12 there gave the same sequence of listings: nothing in the folder, then `f_0001.png=0 bytes` as frame 1 begins, then the same name at 34498 bytes when it finishes, then `f_0002.png=0 bytes` as frame 2 begins, and so on. With Placeholders off, no file at all exists for the frame in progress.

The part-written half is worth setting out carefully, because the two machines caught different versions. Watching a folder twenty times a second through a slow 1000-by-1000 OpenEXR render, the Linux watcher recorded for Blender 4.5.12:

```
  0.00s  (nothing)
 38.80s  f_0001.exr=477469 bytes
 75.92s  f_0001.exr=477469 bytes, f_0002.exr=477469 bytes
```

so 4.5 was not caught mid-write there, just as it was not on the Mac. **But 5.0 and 5.2 were both caught on Linux with the file at 0 bytes** before it reached its finished size, where on the Mac it was 5.0 that was caught, at 5,448 bytes. Taking the two machines together: with Placeholders off, an ordinary frame file **can** be seen at a size that is not its finished size, on both operating systems, and which version a watcher happens to catch is luck rather than a difference between versions. Neither the existence of a file nor a non-zero size proves the frame finished — and a zero-byte file is not proof of a placeholder either.

**Observed — Linux hand test 27, 2026-08-06.** Twelve runs of two Blenders sharing a folder with Placeholders on, on the rented Linux machine, showed no frame rendered twice — unlike the Mac. That is recorded in full under question 4, and it does not change the answer here: what it settles is how reliably a placeholder reserves a frame, not whether a file's existence proves a frame finished.

**Outside the interface.** The sources do not state this specifically for a render driven from outside. They describe Placeholders on the Output page and in `bpy.types.RenderSettings`, and the cited render-farm hint does not say how each machine starts its render. The `render_write` entry does not say whether it works in background mode. **Observed**, from hand tests 3 and 4: Placeholders works on a Blender driven from a terminal with no window, and the `render_write` handler does fire there — every `WATCH [render_write …]` line above was printed by such a handler in background mode. The same handler fired the same way on the rented Linux machine.

**Operating system.** The cited *Output Properties → Output* and *Rendering Animations* pages, and the `bpy.types.RenderSettings` and `bpy.app.handlers` entries, do not name an operating system. The "similar to Unix 'touch'" wording is a description of what an empty file is, not a limit to Unix-like systems; taking it as an OS limit would be reading more into it than it says. **Observed on two operating systems, and they agree** — macOS 26.5.2 (hand tests 3 and 4, 2026-08-05) and Ubuntu 22.04.5 LTS on the rented Linux machine (Linux hand tests 3 and 4, 2026-08-06). Placeholders makes empty files on both; a file can be caught part-written on both. **Windows remains untested**, because no Windows machine has run it.

---

## 7. Can Blender be given a set of frames that is not one plain run — single frames, or two separate stretches?

**Answer.** Yes, from the command line. The `-f` / `--render-frame` argument takes a comma-separated list of frames, and a range can be written with `..` between its first and last frame. So a list of single frames is documented, and two separate stretches follow from combining the two sentences (for example `-f 1..10,50..60`).

Inside Blender's own window, the Frame Range panel offers only Start, End and Step — nothing on that panel expresses a list or two stretches.

**Label: documented** for a list of single frames and for a `..` range. **Inferred** for two separate stretches — the step is combining "a comma separated list of frames can also be used" with "a range of frames can be expressed using `..`"; the sources never show the two written together.

**Exact words.**

> "-f, --render-frame <frame>
>
> Render frame <frame> and save it.
>
> +<frame> start frame relative, -<frame> end frame relative.
>
> A comma separated list of frames can also be used (no spaces).
>
> A range of frames can be expressed using .. separator between the first and last frames (inclusive)."

- Source: manual, *Command Line Arguments*, "Render Options".
- Address: https://docs.blender.org/manual/en/4.5/advanced/command_line/arguments.html
- Date read: 2026-07-30

For the interface side, the Frame Range panel and its picture:

> "Frame Start, End
>
> Set the Start and End frames for Rendering Animations.
>
> Step
>
> Controls the number of frames to advance by for each frame in the timeline."

- Source: manual, *Output Properties → Frame Range*.
- Address: https://docs.blender.org/manual/en/4.5/render/output/properties/frame_range.html
- Date read: 2026-07-30

The picture on that page (`_images/render_output_properties_frame_range_panel.png`, captioned "Frame Range panel.") shows the panel holding exactly four things: Frame Start (1), End (250), Step (1), and a collapsed Time Stretching section. **Documented**, from the picture: no field on this panel takes a list.

The command line also has a way to render a *single* frame relative to the ends of the range, which the manual demonstrates:

> "blender -b file.blend -o /project/renders/frame_##### -F OPEN_EXR -f -2 … -f -2 Render only the second last frame."

- Source: manual, *Rendering From The Command Line*, "Single Image".
- Address: https://docs.blender.org/manual/en/4.5/advanced/command_line/render.html
- Date read: 2026-07-30

**Not covered.** Whether the interface has any other route to an arbitrary set (a script, an add-on, the Python console) — the manual's Frame Range panel does not, and I found no other. Python does have the pieces: `bpy.types.Scene.frame_set(frame, *, subframe=0.0)` ("Set scene frame updating all objects and view layers immediately", https://docs.blender.org/api/4.5/bpy.types.Scene.html) and `bpy.ops.render.render(*, animation=False, write_still=False, use_viewport=False, layer='', scene='')` ("Write Image, Save the rendered image to the output path (used only when animation is disabled)", https://docs.blender.org/api/4.5/bpy.ops.render.html), both read 2026-07-30 *(citation check, 2026-08-05: the operator's signature was written as `render(animation=False, write_still=True)`; the page gives the signature above, with `write_still` defaulting to False. Corrected to the page.)* — so a script can render any set of frames one at a time. That conclusion is **inferred**, not stated. Also not covered: whether a list may contain duplicates, whether it must be in ascending order, and what the ordering of `-f 5,1,9` is (see question 10). Note also the manual's warning that the Preview Range in the Timeline "only affects the preview in the 3D Viewport; it doesn't affect rendering" (https://docs.blender.org/manual/en/4.5/editors/timeline.html, read 2026-07-30) — so it is not a route to a second stretch.

**Observed — hand tests 8, 9, 10, 11 and 20, 2026-08-05.** All five were run with Blender 4.5.12 started from a terminal, rendering the default cube scene to PNG into an empty folder each time.

*A list of separate frames, `-f 5,1,9`.* Blender printed:

```
Saved: '…/f_0005.png'
Saved: '…/f_0001.png'
Saved: '…/f_0009.png'
```

and the folder held `f_0001.png`, `f_0005.png`, `f_0009.png`. The list is rendered in the order written, not sorted.

*A list mixing single frames and stretches, `-f 1..3,10..12`.* Accepted, no error:

```
Saved: '…/g_0001.png'
Saved: '…/g_0002.png'
Saved: '…/g_0003.png'
Saved: '…/g_0010.png'
Saved: '…/g_0011.png'
Saved: '…/g_0012.png'
```

All six files were written. So the two forms may be combined in one argument, and two separate stretches are expressible in a single `-f`.

*A repeated frame, `-f 2,2,3`.* Frame 2 was rendered twice, over the same file:

```
Saved: '…/h_0002.png'
Saved: '…/h_0002.png'
Saved: '…/h_0003.png'
```

Two files were left. Duplicates are neither rejected nor collapsed.

*Malformed lists.* Each of the three was refused, nothing was rendered, and in every case Blender's exit code was `0`:

```
Error: incorrect comma use '-f / --render-frame 1,,3'.
Warning: negative range ignored '-f / --render-frame 3..1'.
Error: not a number '-f / --render-frame 1-3'.
```

*With Blender's window open.* The same `-f 1..3,10..12` command without `-b`, so Blender opened its window, wrote the same six files with the same names. The list argument does not depend on background mode.

Run on the Creator's MacBook Pro, M1 Pro chip, macOS 26.5.2.

**Observed — hand tests 24, 26 and 27, 2026-08-05.** Three more routes and limits, on the same machine and date.

*Two separate stretches can also be had from two render instructions in one command.* Blender 4.5.12 given `-s 1 -e 2 -a -s 20 -e 21 -a` wrote four files — `m_0001.png`, `m_0002.png`, `m_0020.png`, `m_0021.png`.

*The keyframes-only playblast cannot be reached with no window.* Calling `bpy.ops.render.opengl(animation=True, render_keyed_only=True)` in a Blender 4.5.12 started with no window gave:

```
Error: Cannot use OpenGL render in background mode (no opengl context)
RAISED: RuntimeError Error: Cannot use OpenGL render in background mode (no opengl context)
FILES: []
```

No file was written.

*A list of five thousand frames is accepted.* `-f` with `1,2,…,5000` rendered all five thousand and exited with code `0`. A list of two hundred thousand never reached Blender at all — the shell refused it first, with `argument list too long`, and returned `127`. So the limit is the operating system's argument length, not anything in Blender.

**Observed — hand test 32, run by the Creator, 2026-08-05.** Blender does have one built-in way of rendering a scattered set of frames, and it does not leave gaps on disk. In Blender 4.5.12 the Creator took the default cube scene, put a keyframe on the cube at frame 3 and another at frame 5 and none anywhere else, set the frame range to 1–6 and an image output path, and used the keyframes-only viewport render from the 3D Viewport's View menu. Their own words: "Anyway, all versions create 6 files, which are identical."

So six files for a six-frame range, even though only two frames carry a key. What the six files were named, and how big they were, is not recorded.

The menu item is labelled differently here than in the later versions. In Blender 4.5.12 the View menu reads:

> Viewport Render Image
> Viewport Render Animation
> Viewport Render Keyframes

with the tooltip "Take a snapshot of the active viewport." and, underneath it, "Python: bpy.ops.render.opengl()". (In Blender 5.0.1, 5.1.2 and 5.2.0 the same three items read "Render Viewport Preview", "Render Playblast" and "Render Playblast on Keyframes", with the same tooltip.)

Run on the Creator's Mac — a MacBook Pro, M1 Pro chip, macOS 26.5.2 — so this settles macOS and leaves Windows and Linux open.

**Observed — Linux hand tests 7 to 10, 19, 23, 24, 25 and 26, 2026-08-06.** Every one of the tests above was run again on the rented Linux machine with Blender 4.5.12 and came back the same. `-f 5,1,9` gave 5, then 1, then 9. `-f 1..3,10..12` wrote all six files with no error. `-f 2,2,3` rendered frame 2 twice over the same file. The three malformed lists were refused with the same three messages, word for word, nothing rendered, exit code 0 every time. The same list with a window open wrote the same six files. `-s 1 -e 2 -a -s 20 -e 21 -a` wrote four files. The keyframes-only playblast with no window was refused with the same message. And the same list under Cycles gave the same order as under Workbench.

The one difference is in how the operating system refuses a list that is too long. Five thousand frames were accepted and all five thousand rendered, exit code 0, on both machines. Two hundred thousand never reached Blender on either. On the Mac the shell said `argument list too long` and returned `127`; on the rented Linux machine it said:

```
/root/smoozy/Frames/work/batch2.sh: line 156: /opt/blender/blender-4.5.12-linux-x64/blender: Argument list too long
```

and returned `126`. **So the limit is the operating system's on both machines, and the two operating systems refuse it differently.**

**Observed — Linux hand test 34, 2026-08-06.** The keyframes-only playblast was run with a window, on the rented Linux machine's own display. Blender 4.5.12, given the startup scene with a keyframe on the cube's location at frame 3 and another at frame 5 and none anywhere else, the cube selected and active, and a frame range of 1 to 6, was asked for `bpy.ops.render.opengl(animation=True, render_keyed_only=True)` with the 3D Viewport as its context — the same operator the View menu's item runs. It returned `{'FINISHED'}` and wrote:

```
p_0001.png=37619 bytes | p_0002.png=37619 bytes | p_0003.png=37619 bytes |
p_0004.png=37619 bytes | p_0005.png=37619 bytes | p_0006.png=37619 bytes
```

Six files for a six-frame range with keys on only two frames, all six exactly the same size. That agrees with the Creator's macOS result. Where it does not agree: the Creator reported the six files as identical, and compared byte for byte here **no two of the six are the same file** — all fifteen pairs came back different, in every version. Looked at, they are the same picture. Both records stand; the eye and a byte comparison are answering different questions.

**Observed — Linux hand tests 35 to 38, 2026-08-06.** Four things about `-f` that neither machine had tried before, all run on the rented Linux machine only, so **the Mac is untested for all four**. In Blender 4.5.12, and identically in the other three versions:

- **`-f` may be given twice in one command.** `-f 1..3 -f 10..12` rendered both stretches — six files, exit code 0.
- **`-f` and `-a` may be mixed in one command.** `-s 1 -e 2 -a -f 20` carried out both — three files, frames 1, 2 and 20, exit code 0.
- **Step has no effect on a `-f` list.** With the scene's step set to 3, which read back as 3, `-f 1,2,3,4,5` wrote all five files.
- **Whitespace inside a list is accepted.** `-f "1, 2, 3"` rendered frames 1, 2 and 3 and wrote three files, exit code 0, even though the manual says "no spaces". That says what this machine took, not that the manual is wrong about what is supported.

**Outside the interface.** The documented `-f` route is from outside the interface. The cited Frame Range panel has no equivalent list control. The sources do not document another interface route. **Observed**, from hand test 20: `-f` with a list behaves the same whether or not Blender opens a window. **Observed**, from hand test 26: the keyframes-only playblast is the other way round — it cannot run at all without a window. Both hold on the rented Linux machine as well.

**Operating system.** The sources do not say. **Observed on two operating systems, and they agree on everything the question asks** — macOS 26.5.2 (hand tests 8 to 11, 20, 24, 26, 27 and 32, 2026-08-05) and Ubuntu 22.04.5 LTS on the rented Linux machine (Linux hand tests 7 to 10, 19, 23, 24, 25, 26 and 34, 2026-08-06). The one thing that differs is not Blender's: how long a list the operating system will pass on, and what it says when it will not. The four new `-f` behaviours in Linux hand tests 35 to 38 are **Linux only**; the Mac was never asked. **Windows remains untested**, because no Windows machine has run it. The command-line launch pages differ per operating system only in how you reach the executable (https://docs.blender.org/manual/en/4.5/advanced/command_line/launch/windows.html, .../macos.html, .../linux.html, read 2026-07-30); the arguments themselves are documented once, with no per-system notes. The manual does warn that "Arguments are case sensitive! -F and -f are not the same" (*Rendering From The Command Line*), which matters most where a shell or file system is not case-sensitive — but the manual does not make that connection, so I do not either.

---

## 8. What does a frame range consist of?

**Answer.** Three numbers: Frame Start, End, and Step. Start and End are the first and last frames of the range, inclusive; Step is how far the render advances between one rendered frame and the next. The same panel also holds Time Stretching, which remaps the animation's length but explicitly does not change Start or End.

**Label: documented.**

**Exact words.**

> "This panel defines how long an animation will last in terms of frames. …
>
> Frame Start, End
>
> Set the Start and End frames for Rendering Animations.
>
> Step
>
> Controls the number of frames to advance by for each frame in the timeline."

> "Warning
>
> Using Time Stretching will not influence the Start or End frames set above, so make sure that your animation is not cut off or has extraneous still frames at the end."

- Source: manual, *Output Properties → Frame Range*.
- Address: https://docs.blender.org/manual/en/4.5/render/output/properties/frame_range.html
- Date read: 2026-07-30

That the range is inclusive of both ends comes from the command line documentation:

> "-a, --render-anim
>
> Render frames from start to end (inclusive)."

- Source: manual, *Command Line Arguments*.
- Address: https://docs.blender.org/manual/en/4.5/advanced/command_line/arguments.html
- Date read: 2026-07-30

The API gives the three as properties, with their kind, default and range:

> "frame_end
>
> Final frame of the playback/rendering range
>
> Type:
>
> int in [0, 1048574], default 250"

> "frame_start
>
> First frame of the playback/rendering range
>
> Type:
>
> int in [0, 1048574], default 1"

> "frame_step
>
> Number of frames to skip forward while rendering/playing back each frame
>
> Type:
>
> int in [0, 1048574], default 1"

- Source: Python API, `bpy.types.Scene`.
- Address: https://docs.blender.org/api/4.5/bpy.types.Scene.html
- Date read: 2026-07-30

Time Stretching's two numbers live on the render settings, not the scene:

> "frame_map_new
>
> How many frames the Map Old will last
>
> Type:
>
> int in [1, 900], default 100"

> "frame_map_old
>
> Old mapping value in frames
>
> Type:
>
> int in [1, 900], default 100"

- Source: Python API, `bpy.types.RenderSettings`.
- Address: https://docs.blender.org/api/4.5/bpy.types.RenderSettings.html
- Date read: 2026-07-30

The panel picture (`_images/render_output_properties_frame_range_panel.png`, captioned "Frame Range panel.", on the Frame Range page) shows Frame Start 1, End 250, Step 1, and a collapsed Time Stretching. **Documented**, from the picture.

**Not covered.** The API says `frame_step` accepts 0 as its lowest value; the sources do not say what a step of 0 does to a render. They do not say what happens when End is lower than Start. They do not say whether the range is per scene or per render job. They do not say how the range interacts with a `-f` list (question 7).

**Observed — hand tests 12, 13 and 14, 2026-08-05.** All three were run with Blender 4.5.12 started from a terminal with no window, rendering the default cube scene to PNG into an empty folder.

*A step that lands exactly on the end frame.* Start 1, end 10, step 3, given as `-j 3` on the command line, produced files for frames 1, 4, 7 and 10.

*A step that overshoots the end frame.* Start 3, end 12, step 4, set as scene settings, produced files for frames 3, 7 and 11. Frame 12 — the end frame — was not rendered. So a stepped run walks forward from the start frame and stops at the last frame at or below the end; it does not add the end frame on.

*A step of 0.* The property accepted it and read back as `0`. Blender then rendered and wrote frame 1 once and stopped, leaving one file:

```
frame_step actually reads back as: 0
Saved: '…/f_0001.png'
f_0001.png  3658 bytes
```

It did not hang. (In 5.0, 5.1 and 5.2 the same test prints `Rendering frame 1` five times — once per frame of the 1-to-5 range — but still writes only the one file.)

*An end below the start.* Start 5, end 2 rendered nothing at all and exited with code `0`. Blender 4.5.12 printed only its version banner and `Blender quit`; the folder was empty afterwards.

Run on the Creator's MacBook Pro, M1 Pro chip, macOS 26.5.2.

**Observed — hand test 28, run by the Creator, 2026-08-05.** The panel accepts a step of 0 and shows it, with no complaint. In Blender 4.5.12 the Creator opened Output Properties, the Frame Range panel, typed `0` into the Step field and pressed Enter. Their own words: "Typing 0 in the Step field doesn't do anything — no error, warning or any visible behavior. Right-clicking the panel header does nothing. The panel itself is in the screenshot. I only took one because it's the same in all 4 versions."

The screenshot shows the panel holding, in order: **Frame Start** `1`, **End** `250`, **Step** `0`, and an open **Time Stretching** section with **Old** `100` and **New** `100`. Nothing else. So the Step field keeps the 0 rather than snapping to 1, and the panel's whole contents are those five numbers — which matches what the manual's own picture of the panel shows, with the Time Stretching section opened up.

Read alongside the run above: the field takes 0, and a render made with it writes the first frame once.

Run on the Creator's Mac — a MacBook Pro, M1 Pro chip, macOS 26.5.2 — so this settles macOS and leaves Windows and Linux open.

**Observed — Linux hand tests 11, 12 and 13, 2026-08-06.** The same three runs on the rented Linux machine gave the same frames in Blender 4.5.12. Start 1, end 10, step 3 gave files for 1, 4, 7 and 10. Start 3, end 12, step 4 gave 3, 7 and 11, and did not render frame 12. A step of 0 read back as `0`, wrote frame 1 once, left one file of 3583 bytes, and did not hang. An end below the start rendered nothing and exited 0, with 4.5 printing only its version banner and `Blender quit`.

One thing the Mac run did not record. In 5.0, 5.1 and 5.2 the printed output announces each stepped frame once per step — `Rendering frame 4` three times over for a step of 3 — and in the 3-to-12 run it ends with a bare `Rendering frame 15`, past the end frame, for which nothing is rendered and nothing is written. Blender 4.5 announces nothing at any time, so it shows none of this.

**Observed — Linux hand test 31, 2026-08-06.** The Frame Range panel was photographed on the rented Linux machine's own display, with Blender's own `bpy.ops.screen.screenshot`. The panel holds **Frame Start** `1`, **End** `250`, **Step** `1`, and a collapsed **Time Stretching** section, and nothing else. With the step set to 0 the field then reads **`0`** — the panel keeps the 0 and does not snap back to 1. That agrees with what the Creator reported on the Mac.

Two limits on that run, said plainly. The step was set from Python rather than typed into the field, so what the field does with typed input rests on the Creator's macOS run alone. And the panel header was not right-clicked. Both need the pointer put on a control, and on that machine the display is shared with other work: a click aimed at a control was followed by asking Blender for that control's state, and the state had not changed, so no click could be shown to land. Nothing that could not be demonstrated has been written down.

**Observed — Linux hand test 31 again, 2026-08-06, opened by operator rather than by hand.** The panel was then opened as a popup by Blender's own operator, which needs no pointer at all. `bpy.ops.wm.call_panel(name="RENDER_PT_frame_range")` returned `{'INTERFACE'}`, and the picture it was photographed in shows the whole panel with its Time Stretching section open:

> Frame Start `1`
> End `250`
> Step `0`
> Time Stretching
> Old `100`
> New `100`

Nothing else. That is exactly what the Creator reported from the Mac, and it settles the panel's whole contents on this machine as well. The panel identifier was read out of `bpy.types` rather than guessed: both `RENDER_PT_frame_range` and `RENDER_PT_time_stretching` are there.

**Observed — Linux hand tests 37 and 39, 2026-08-06.** Two things about the range that neither machine had tried, run on the rented Linux machine only. **Step has no effect on a `-f` list**: with the scene's step set to 3, `-f 1,2,3,4,5` wrote all five files. And **the alternative playback range does not change what a real render writes**: with the scene range 1 to 8, an alternative playback range of 4 to 5 and that range switched on, a plain animation render wrote all eight frames.

**Outside the interface.** The same three have command-line equivalents — `-s, --frame-start <frame>`, `-e, --frame-end <frame>`, and `-j, --frame-jump <frames>` ("Set number of frames to step forward after each rendered frame") — so the range is fully expressible from outside. Source and address as for the `-a` quote above. Note the manual's ordering rule: "Arguments are executed in the order they are given! … Always position -f or -a as the last arguments" (*Rendering From The Command Line*, https://docs.blender.org/manual/en/4.5/advanced/command_line/render.html, read 2026-07-30), and the worked examples on the Command Line Arguments page showing that loading the blend-file after setting an option undoes it.

**Operating system.** The cited Frame Range page, command-line pages, and API entries do not name an operating system. **Observed on two operating systems, and they agree** — macOS 26.5.2 (hand tests 12, 13, 14 and 28, 2026-08-05) and Ubuntu 22.04.5 LTS on the rented Linux machine (Linux hand tests 11, 12, 13 and 31, 2026-08-06). The same three stepped runs give the same frames on both, and the panel shows a step of 0 on both. **Linux hand tests 37 and 39 — Step against a `-f` list, and the alternative playback range — are Linux only**; the Mac was never asked. *(Linux citation check, 2026-08-06: added by this check.)* **Windows remains untested**, because no Windows machine has run it.

---

## 9. What does a frame write when several view layers are active on it?

**Answer.** Only partly answerable from the sources.

What is documented: the Python render-engine API has a call defined as running "after all view layers in a frame were rendered". A view layer can be enabled or disabled for rendering, and there is a Render Single Layer switch that limits a render to the active layer but is ignored from the command line. The manual says an OpenEXR file "can store multiple layers and passes", and describes Cache Result as saving "the rendered view layers and their passes to a multi-layer OpenEXR image". *(Citation check, 2026-08-05: this read "a multi-layer OpenEXR file"; the manual says it of "An OpenEXR file", under the heading OpenEXR. Narrowed to the source.)*

What is **not in the sources**: what a single frame file actually contains when several view layers are on and the output format is an ordinary one (PNG, JPEG, plain OpenEXR). Whether Blender writes one file holding a combined result, or one file per layer, or only the active layer, is nowhere stated in the manual or the API documentation I searched.

**Label: documented** for the parts above. **Not in the sources** for what the written file contains with an ordinary format.

**Exact words.**

> "render_frame_finish()
>
> Perform finishing operations after all view layers in a frame were rendered"

- Source: Python API, `bpy.types.RenderEngine`.
- Address: https://docs.blender.org/api/4.5/bpy.types.RenderEngine.html
- Date read: 2026-07-30

> "Use for Rendering
>
> The active view layer will be used during rendering.
>
> Render Single Layer
>
> Only render the active view layer.
>
> Note
>
> This option is ignored when rendering from the command line."

- Source: manual, *Layers & Passes → View Layer*.
- Address: https://docs.blender.org/manual/en/4.5/render/layers/view_layer.html
- Date read: 2026-07-30

> "use
>
> Enable or disable rendering of this View Layer
>
> Type:
>
> boolean, default False"

- Source: Python API, `bpy.types.ViewLayer`.
- Address: https://docs.blender.org/api/4.5/bpy.types.ViewLayer.html
- Date read: 2026-07-30

> "use_single_layer
>
> Only render the active layer. Only affects rendering from the interface, ignored for rendering from command line.
>
> Type:
>
> boolean, default False"

- Source: Python API, `bpy.types.RenderSettings`.
- Address: https://docs.blender.org/api/4.5/bpy.types.RenderSettings.html
- Date read: 2026-07-30

> "An OpenEXR file can store multiple layers and passes. This means OpenEXR images can be loaded into a Compositor keeping render layers and passes intact."

- Source: manual, *Image Formats*, OpenEXR.
- Address: https://docs.blender.org/manual/en/4.5/files/media/image_formats.html
- Date read: 2026-07-30

> "Cache Result
>
> Saves the rendered view layers and their passes to a multi-layer OpenEXR image."

- Source: manual, *Output Properties → Output*, "Saving".
- Address: https://docs.blender.org/manual/en/4.5/render/output/properties/output.html
- Date read: 2026-07-30

And, on the one thing that decides the final image when the compositor is in play:

> "The Composite node defines the final output of the Compositor. It is the node where the image result is sent to the renderer or image output after rendering. … If multiple Composite nodes are present in the node tree, only the active one is used."

- Source: manual, *Compositor → Output → Composite Node*.
- Address: https://docs.blender.org/manual/en/4.5/compositing/types/output/composite.html
- Date read: 2026-07-30

**Not covered.** The big one, stated above: what a non-multi-layer frame file holds when several view layers are active. Also not covered: whether the number of view layers changes how many files a frame produces; whether view layers are rendered in a defined order within a frame; what happens if every view layer is switched off. Where I looked: *Layers & Passes* (Introduction, View Layer, Passes, Filter), *Render Output* in full (Introduction, all Output Properties pages, Rendering Animations), *Image Formats*, *View Layers* under Scenes & Objects, the Composite / File Output / Render Layers node pages, `bpy.types.RenderSettings`, `bpy.types.ViewLayer`, `bpy.types.RenderResult`, `bpy.types.RenderLayer`, `bpy.types.RenderEngine`, and manual-wide searches for "multi-layer", "multilayer", "all layers", "each view layer", "per view layer", "all view layers". The 4.5 release notes contain nothing on this (searched *Rendering*, *Core*, *Pipeline & I/O*, *Python API*). Cycles' own files contain nothing on it either.

The compositor's File Output node is the documented way to write more than one file from one frame — "This node writes out an image, for each frame range specified, to the filename entered, as part of a frameset sequence" and "since this node can be hooked in anywhere in the node tree, it can also save intermediate images automatically" (https://docs.blender.org/manual/en/4.5/compositing/types/output/file_output.html, read 2026-07-30) — but the sources do not tie it to view layers, so I am not claiming that connection.

**Observed — hand tests 15, 16 and 17, 2026-08-05.** Blender 4.5.12 was started from a terminal with no window. A script moved the default cube into its own collection and added a second view layer, `Second`, that leaves that collection out — so the two view layers plainly show different things, one with the cube and one without. Both view layers had `use` set to True. One frame was rendered four ways and the written files were loaded back and compared pixel by pixel:

```
VIEW LAYER 'ViewLayer'  use=True
VIEW LAYER 'Second'  use=True
A (both layers)       == C (first layer alone) : True
A (both layers)       == D (second layer alone): False
B (single-layer flag) == A (both layers)       : True
C == D (the two layers look different at all)  : False
A_both_0001.png  27222 bytes
B_single_0001.png  27222 bytes
C_firstonly_0001.png  27222 bytes
D_secondonly_0001.png  23387 bytes
```

Three answers come out of that. **One image file is written per frame** with these two view layers both enabled — never one per layer. *(Citation check, 2026-08-05: this read "whatever the number of view layers"; the test used exactly two, so it has been narrowed to two.)* **Its pixels are the first view layer's alone**: identical to the same frame rendered with only that layer enabled, and different from the same frame rendered with only the second layer enabled. And **Render Single Layer changes nothing** in a render driven from outside the window: the file written with it on is pixel-for-pixel the file written with it off, which is what the documented "ignored when rendering from the command line" note amounts to in practice.

**Observed — hand test 16, 2026-08-05.** The multi-layer OpenEXR route does keep both view layers, in one file. Blender 4.5.12 offers it as a file format outright — the full list this version accepts is:

```
['BMP', 'IRIS', 'PNG', 'JPEG', 'JPEG2000', 'TARGA', 'TARGA_RAW', 'CINEON', 'DPX',
 'OPEN_EXR_MULTILAYER', 'OPEN_EXR', 'HDR', 'TIFF', 'WEBP', 'FFMPEG']
```

(In 5.0, 5.1 and 5.2 that list no longer offers `OPEN_EXR_MULTILAYER` alongside the others in the same way — those versions reach it through a separate media-type setting, `media_type = 'MULTI_LAYER_IMAGE'`, after which the file-format list holds `OPEN_EXR_MULTILAYER` alone.) The render wrote one file, `vl_0001.exr`, 17437 bytes, and the names found inside its header were:

```
Second.Combined.A, Second.Combined.B, Second.Combined.G, Second.Combined.R,
ViewLayer.Combined.A, ViewLayer.Combined.B, ViewLayer.Combined.G, ViewLayer.Combined.R
```

Both view layers, named, inside the one file.

Run on the Creator's MacBook Pro, M1 Pro chip, macOS 26.5.2.

**Observed — Linux hand tests 14, 15 and 16, 2026-08-06.** The same three runs on the rented Linux machine gave the same four comparisons in Blender 4.5.12:

```
A (both layers)       == C (first layer alone) : True
A (both layers)       == D (second layer alone): False
B (single-layer flag) == A (both layers)       : True
C == D (the two layers look different at all)  : False
A_both_0001.png  27320 bytes
B_single_0001.png  27320 bytes
C_firstonly_0001.png  27320 bytes
D_secondonly_0001.png  23429 bytes
```

One image file per frame, holding the first view layer's image, with Render Single Layer making no difference. The multi-layer OpenEXR route kept both view layers in one file there too, with the same names inside it.

**Observed — Linux hand test 29, 2026-08-06. The compositor switched on, which nobody had tested on either machine.** Blender 4.5.12 on the rented Linux machine, with the same two view layers both enabled, rendered one frame five ways: with the compositor off and both layers on; with the compositor off and only the first; with the compositor off and only the second; with the compositor on and its Render Layers node reading the first view layer; and with that node moved to the second. Switching the compositor on made this tree:

```
nodes in the tree Blender made: [('Composite', 'CompositorNodeComposite'), ('Render Layers', 'CompositorNodeRLayers'), ('Reroute', 'NodeReroute'), ('Viewer', 'CompositorNodeViewer')]
links in the tree Blender made: [('Render Layers', 'Image', '->', 'Reroute', 'Input'), ('Reroute', 'Output', '->', 'Composite', 'Image'), ('Reroute', 'Output', '->', 'Viewer', 'Image')]
Render Layers nodes: [('Render Layers', 'ViewLayer')]
```

and the five files compared:

```
C (first layer alone)   == D (second layer alone) : False
A (comp off, both on)   == C (first layer alone)  : True
E (comp on, node=first) == C (first layer alone)  : True
E (comp on, node=first) == A (comp off, both on)  : True
F (comp on, node=Second)== D (second layer alone) : True
F (comp on, node=Second)== C (first layer alone)  : False
A_compoff_both_0001.png  27320 bytes
C_compoff_firstonly_0001.png  27320 bytes
D_compoff_secondonly_0001.png  23429 bytes
E_compon_first_0001.png  27320 bytes
F_compon_second_0001.png  23429 bytes
```

Two things come out of it, and they were identical in all four versions. **Switching the compositor on still writes one image file for the frame** — no second file, and no view-layer name in the name. And **what lands in that file is whichever view layer the compositor's Render Layers node is set to**: with the node moved to the second view layer, the file is pixel-for-pixel the second layer's own image and different from the first's, even though both view layers are enabled and the first is the active one.

A version difference was found on the way and belongs here. In Blender 4.5 the compositor lives on the scene: `Scene.node_tree`, holding a **Composite** node as above. In Blender 5.0, 5.1 and 5.2 there is **no `Scene.node_tree` at all** — asking for it raises `AttributeError: 'Scene' object has no attribute 'node_tree'` — and the compositor is a node group hung on `Scene.compositing_node_group`, which starts out empty; a tree had to be built by hand there, a Render Layers node feeding a **Group Output** node. Those three versions also print a warning when `Scene.use_nodes` is touched: "'Scene.use_nodes' is expected to be removed in Blender 6.0".

**Outside the interface.** Render Single Layer is explicitly ignored when rendering from the command line. **Inferred:** a blend-file set to render only the active layer will then render the layers enabled for rendering. The step joins the statement that the switch is ignored with the API entry that enables or disables a view layer. **Observed**, from hand tests 15 and 17: with Blender driven from a terminal with no window, the setting made no difference to the file written, and the file written held the first view layer's image. The same holds on the rented Linux machine, with the compositor off and with it on.

**Operating system.** The manual and API material listed above, including the View Layer, Image Formats, Output, and compositor pages, does not name an operating system for this behaviour. **Observed on two operating systems, and they agree** — macOS 26.5.2 (hand tests 15, 16 and 17, 2026-08-05) and Ubuntu 22.04.5 LTS on the rented Linux machine (Linux hand tests 14, 15 and 16, 2026-08-06). The compositor-on case in Linux hand test 29 is **Linux only**; the Mac was never asked. **Windows remains untested**, because no Windows machine has run it.

---

## 10. In what order does a render work through the frames it was given — a plain run, a run that skips frames at a fixed interval, and a list of separate frames — and what does an interruption leave behind in each case?

**Answer.**

*A plain run.* Start to end, inclusive. **Inferred:** it works through those frames in ascending order. The step is reading the range from start to end as a sequence, not merely as its bounds.

*A run that skips at a fixed interval.* The sources say it advances by Step after each rendered frame. **Inferred:** it starts at Start and continues as Start, Start+Step, Start+2×Step, and so on. The manual's Animation Preview advice says to render every N'th frame first, then return with Step 1 and Overwrite off to fill the gaps.

*A list of separate frames.* **Not in the sources.** Nothing states whether `-f 5,1,9` renders in the order given, in ascending order, or otherwise.

*What an interruption leaves behind.* The sources describe this by output format, not by frame ordering. With an image format, they say frames already rendered are on disk in the output directory, and describe recovery by moving the range or turning Overwrite off. With a single video file, the manual warns the file may be unplayable and all frames must be rendered again. The sources say nothing specific about an interruption during a stepped run or a listed run.

**Label: documented** for a plain run's inclusive bounds and for the stated step increment. **Inferred** for the plain run's ascending order and the stepped sequence. **Not in the sources** for the order of a list of separate frames, and for anything interruption-specific to stepped or listed runs.

**Exact words.**

> "-a, --render-anim
>
> Render frames from start to end (inclusive)."

> "-j, --frame-jump <frames>
>
> Set number of frames to step forward after each rendered frame."

- Source: manual, *Command Line Arguments*, "Render Options".
- Address: https://docs.blender.org/manual/en/4.5/advanced/command_line/arguments.html
- Date read: 2026-07-30

> "Step
>
> Controls the number of frames to advance by for each frame in the timeline."

- Source: manual, *Output Properties → Frame Range*.
- Address: https://docs.blender.org/manual/en/4.5/render/output/properties/frame_range.html
- Date read: 2026-07-30

> "frame_step
>
> Number of frames to skip forward while rendering/playing back each frame"

- Source: Python API, `bpy.types.Scene`.
- Address: https://docs.blender.org/api/4.5/bpy.types.Scene.html
- Date read: 2026-07-30

> "Animation Preview
>
> It can be useful to render a subset of the animated sequence, since only part of an animation may have an error.
>
> Using an image format for output, you can use the Frame Step option to render every N'th frame. Then disable Overwrite and re-render with Frame Step set to 1."

- Source: manual, *Rendering Animations*.
- Address: https://docs.blender.org/manual/en/4.5/render/output/animation.html
- Date read: 2026-07-30

For what the interruption leaves behind, the quotes are the ones already given in questions 3 and 5: "the frames you have already rendered will still be in the output directory" (https://docs.blender.org/manual/en/4.5/render/output/animation.html) and "the file might become unplayable and you will have to re-render all frames from the beginning" (https://docs.blender.org/manual/en/4.5/render/output/properties/output.html), both read 2026-07-30.

**Not covered.** Whether "start to end" guarantees strictly ascending order rather than merely bounding the set — the phrase is directional but it is a description of which frames, not a promise about sequence, so reading it as an order is a small step and I flag it. Whether the last frame of a stepped run is End when End is not reachable from Start by whole steps. Whether a `-f` list is sorted, de-duplicated, or rendered as typed. Whether frames can ever be rendered out of order (for instance by more than one thread) — nothing in the sources suggests it and nothing rules it out. Where I looked, beyond the pages quoted: *Rendering From The Command Line*, *Render Output → Introduction*, the whole *Output Properties* section, `bpy.ops.render`, `bpy.types.RenderSettings`, `bpy.types.Scene`, `bpy.types.RenderEngine`, `bpy.app.handlers`, and manual-wide searches for "order", "sequentially", "one by one", "ascending". Cycles' own files hold nothing on frame sequencing.

**Ambiguity flagged.** "In what order does a render work through the frames it was given" could be asking about the order frames are *started* or the order files are *written*. I answered the order they are worked through. The other reading — whether writing can lag or reorder relative to rendering — is not in the sources.

**Observed — hand tests 1, 8, 10, 12 and 6, 2026-08-05.** Run with Blender 4.5.12 started from a terminal with no window, rendering the default cube scene to PNG.

*A plain run.* Frames 1 to 5 were written in ascending order, one `Saved:` line each, in the order 1, 2, 3, 4, 5. Quoted in full under question 3.

*A stepped run.* Start 1, end 10, step 3 gave files for frames 1, 4, 7, 10 — the start frame first, then forward by the step. Start 3, end 12, step 4 gave 3, 7, 11, and did not render the end frame 12. Quoted in full under question 8.

*A list of separate frames.* `-f 5,1,9` rendered them in the order written — 5, then 1, then 9 — not sorted. `-f 2,2,3` rendered frame 2 twice. Quoted in full under question 7.

*What an interruption leaves, in each case.* The same in all of them: whole frames only. A plain run interrupted at frame 3 left frames 1 and 2 at their full size and nothing for frame 3; a kill that could not be caught left the same. Quoted in full under questions 2 and 3. For a video output the answer is different and is recorded under question 5.

Run on the Creator's MacBook Pro, M1 Pro chip, macOS 26.5.2.

**Observed — Linux hand tests 1, 7, 9, 11, 5 and 24, 2026-08-06.** Every part of this came back the same on the rented Linux machine with Blender 4.5.12. A plain run gave 1, 2, 3, 4, 5 in that order. A stepped run started on the start frame and moved forward by the step, and did not reach back for an end frame it had overshot. `-f 5,1,9` rendered in the order written, and `-f 2,2,3` rendered frame 2 twice. An interruption left whole frames only, and nothing at all for the frame in flight, whichever way the render was stopped. The same list under Cycles gave the same order as under Workbench — and where the Mac ran that engine test in 5.1 and 5.2 only, it was run in all four versions here.

**Observed — Linux hand test 30, 2026-08-06. The order on a graphics card of another make.** The Mac has Apple's own graphics and nothing else, so it could not speak for any other card. On the rented Linux machine Cycles was pointed at its NVIDIA GeForce RTX 4090 through OPTIX, with the processor switched off:

```
compute_device_type set to: 'OPTIX'
DEVICE name='NVIDIA GeForce RTX 4090' type='CUDA' use=True
DEVICE name='AMD Ryzen Threadripper PRO 3975WX 32-Cores' type='CPU' use=False
DEVICE name='NVIDIA GeForce RTX 4090' type='OPTIX' use=True
scene cycles device = 'GPU', samples = 1000
```

`-f 5,1,9` rendered in the order 5, then 1, then 9 — the same as on the processor. A plain run interrupted part-way left nothing for the frame in flight, printed the same `Sent an internal break event` line and exited with code `1`. **So neither the order nor what an interruption leaves depends on which of the two makes of card is doing the work.** (One line was printed on the way that is not about frames: Blender 5.0, 5.1 and 5.2 print `WARNING HIPEW initialization failed: Error opening HIP dynamic library` on this machine, which is Cycles looking for AMD's library and not finding it. Blender 4.5 does not.)

A first attempt at this test passed the sample count after a `--` on the command line, which made Blender hand every following argument to Python instead of reading it; nothing was rendered and the folder was empty. The setting was passed through an environment variable instead and the run above is the second attempt. Both are recorded rather than only the one that worked.

**Observed — Linux hand test 33, 2026-08-06. The order a render started the way the window's own Render Animation item starts it.** *(Linux citation check, 2026-08-06: this heading read "The order a render started by clicking uses"; nothing was clicked — the render was started by calling the operator that menu item calls, which is what the entry itself says — so it has been narrowed to the entry's own wording.)* This was untested on both machines. On the rented Linux machine, with a window open on its display, Blender 4.5.12 was given `bpy.ops.render.render('INVOKE_DEFAULT', animation=True)` — the call the Render menu's Render Animation item makes — for a six-frame Workbench range. It returned `{'RUNNING_MODAL'}`, a separate **Blender Render** window opened, and afterwards the six files were sorted by the time each was written:

```
the order the files were written in: ['o_0001.png', 'o_0002.png', 'o_0003.png', 'o_0004.png', 'o_0005.png', 'o_0006.png']
```

Ascending, one at a time, the same as a render driven from a terminal. Identical in all four versions.

**Outside the interface.** The clearest statements of order are command-line ones (`-a`, `-j`), so they hold there by definition. Whether the interface's Render Animation follows the same order is not stated separately, though Step is the same scene property in both. **Inferred** that they match, and the step is that both are described by the same property `frame_step` and its identical wording. **Observed**, from Linux hand test 33, that they do match: a render started the way the window's own Render Animation item starts it wrote its six frames in ascending order, one at a time. That inference is now tested. **The two-card runs below say nothing about the inside case**: that machine has no display server at all, so every run there was driven from outside with no window, and no window test could be repeated with two cards. *(Two-card citation check, 2026-08-06: added by this check — the line did not mention the two-card runs at all.)*

**Observed on the two-card machine — 2026-08-06. Whether the frame order or the output changes when both graphics cards are ticked rather than one.** *(Two-card citation check, 2026-08-06: this read "when two graphics cards do the work". The record below shows the preferences that were saved, not that both cards were used by the render — no device list or card load was captured here — so the heading now says what was set rather than what happened.)*

Run over SSH on a **second rented Linux machine — Ubuntu 22.04.5 LTS, two NVIDIA GeForce RTX 4090s of 24 GB each on driver 580.95.05, an AMD EPYC 7B13 with sixty-one cores, 98 GB of memory, no display server at all** — using Blender at `/opt/blender/blender-4.5.12-linux-x64/blender`, `Blender 4.5.12 LTS (hash 84afd5f785f7 built 2026-07-21 01:31:22)`, with `BLENDER_USER_RESOURCES` pointed at `/root/smoozy/two cards/res-4.5`, a folder of its own that was empty at the start. **Nothing else was running on the machine.** Label: **observed**. This is a third machine; nothing here is merged with the Creator's Mac or with the one-card rented Linux machine. The full list is `Audits/Blender Research/Frames/Hand tests — two cards and no screen.md`.

**The set-up.** A four-frame Cycles animation at 640 by 360, 128 samples, adaptive sampling off, denoising off, seed 0 and animated seed off, rendered with `--render-anim` into an empty folder. The preferences were saved with the technology `OPTIX` and both cards ticked, then again with only the card at `0000:00:07` ticked.

**The order is ascending, one at a time, with two cards as with one:**

```
Saved: '/root/smoozy/two cards/work/ord_4.5/f_0001.png'
Saved: '/root/smoozy/two cards/work/ord_4.5/f_0002.png'
Saved: '/root/smoozy/two cards/work/ord_4.5/f_0003.png'
Saved: '/root/smoozy/two cards/work/ord_4.5/f_0004.png'
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

- two cards against one card: **57**
- two cards against two cards, two separate runs: **14**
- one card against one card, two separate runs: **10**

**and the largest difference in every one of those comparisons was `0.003921568393707275` — one step in 255.** So in 4.5 the two-card run differed from the one-card run in about four times as many values as either control, though every difference anywhere was the same single step in 255 out of 921,600 values. Nothing here shows the number of cards changing the picture, and nothing here rules it out either. *(Two-card citation check, 2026-08-06: this read "Two runs on the same cards differ by as much as a two-card run differs from a one-card run, so nothing here shows the number of cards changing the picture." In 4.5 they do not — 57 against 14 and 10 — so the sentence was corrected to this version's own numbers. It is true as written in 5.0 and 5.2.)*

**Observed on the four-card machine — 2026-08-06. Whether the frame order or the picture changes when four graphics cards are ticked rather than one; and what Blender stamps into a picture.**

Run over SSH on a **third rented Linux machine — Ubuntu 22.04.5 LTS, four NVIDIA GeForce RTX 4060 Ti of 16 GB each on driver 580.95.05, an AMD EPYC 7R43 with sixty-one cores, 197 GB of memory, no display server** — using Blender at `/opt/blender/blender-4.5.12-linux-x64/blender`, `Blender 4.5.12 LTS (hash 84afd5f785f7 built 2026-07-21 01:31:22)`, with `BLENDER_USER_RESOURCES` pointed at `/root/smoozy/four cards/res-4.5`, a folder of its own emptied and remade before use. **Nothing else was running on the machine.** Label: **observed**. This is a **fourth** machine; nothing here is merged with the Creator's Mac, the one-card rented Linux machine or the two-card rented Linux machine. The full list, with every command and every script written out, is `Audits/Blender Research/Frames/Hand tests — four cards.md`.

**Of the three ways this machine differs from the two-card one, only one bears on what is below.** Four cards rather than two is the point of the test. The smaller card bears on the seconds in the output quoted here and on nothing else, and no timing from this machine is set against a two-card timing. The larger system memory bears on nothing here.

**The set-up.** The same four-frame Cycles animation as the two-card run: 640 by 360, 128 samples, adaptive sampling off, denoising off, seed 0 and animated seed off, rendered with `--render-anim` into an empty folder. Four passes, each into its own folder — all four cards ticked, one card, all four again, one again — so that every comparison has a control. The script that saves which cards are ticked, and the script that counts differing pixel values, are both written out in full in the list.

**The order is ascending, one at a time, with four cards as with one:**

```
Saved: 'ord_4.5_four_a/f_0001.png'
Saved: 'ord_4.5_four_a/f_0002.png'
Saved: 'ord_4.5_four_a/f_0003.png'
Saved: 'ord_4.5_four_a/f_0004.png'
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

**On the run recorded here, a render fast enough that all three times rounded to `00:00.00` gave two files that matched byte for byte, in all four versions.** On the same scene made slower — 256 by 256 at 400 samples, on the processor, every stamp setting off — **and on that run this version's two files matched**, byte for byte:

```
cmp: the two files are identical
56bcff7313d1d5c54b1d438cfb85f6bb  out/stamp/s_a_0001.png
56bcff7313d1d5c54b1d438cfb85f6bb  out/stamp/s_b_0001.png
    tEXt b'cycles.ViewLayer.samples\x00400'
    tEXt b'cycles.ViewLayer.total_time\x0000:00.23'
    tEXt b'cycles.ViewLayer.render_time\x0000:00.22'
    tEXt b'cycles.ViewLayer.synchronization_time\x0000:00.00'
    tEXt b'cycles.ViewLayer.samples\x00400'
    tEXt b'cycles.ViewLayer.total_time\x0000:00.23'
    tEXt b'cycles.ViewLayer.render_time\x0000:00.22'
    tEXt b'cycles.ViewLayer.synchronization_time\x0000:00.00'
IDAT of out/stamp/s_a_0001.png length 39594 md5 aeed9e818aa245c6915ee63e2f29e775
IDAT of out/stamp/s_b_0001.png length 39594 md5 aeed9e818aa245c6915ee63e2f29e775
```

**And in every one of those cases the picture is identical and only the timing text differs**, which the picture fingerprints show outright.

**So both earlier statements are wrong as written.** "Two Blender PNGs of the same frame can never match byte for byte" is wrong — they did match, twice, in this version's own runs above: on the fast scene and on the slower one. And "with the stamp settings off, two renders are byte-identical" is too strong — with every stamp setting off, 5.0, 5.1 and 5.2 still did not match on the slower scene, because Cycles' own timing chunks are outside those settings. **Whether two files match is decided by how long the render took, not by Blender.** The comparison that answers "is this the same render" is a comparison of the picture data alone.

**Four-card citation check, 2026-08-06 — a repeat on the same machine came out differently, and both runs stand.** This check ran the same pairs again on the four-card machine, which is still alive. **The mechanism above held exactly**: the picture was identical every single time, and the only thing that ever differed between two files was the timing text. **But the version-by-version pattern did not hold, and it is not a fact about the version.** Five separate pairs of the fast scene per version, every stamp setting off, gave — 4.5: identical, differ, identical, differ, identical; 5.0: differ, differ, identical, differ, differ; 5.1: identical, differ, differ, differ, differ; 5.2: differ, identical, identical, differ, differ. Four separate pairs of the slower scene per version, every stamp setting off, gave — 4.5: differ, differ, differ, differ (where the run above matched); 5.0: differ four times; 5.1: differ, identical, differ, differ (where the run above did not match); 5.2: differ four times. And **two renders can match byte for byte at factory stamp settings too**, which the run above reported they never do: six pairs back to back per version gave one identical pair in 4.5, one in 5.0, none in 5.1 and one in 5.2, and a twelfth pair in 5.2 came out identical with every one of its eleven text chunks the same, `Date` and `RenderTime` included:

```
    tEXt b'Date\x002026/08/06 16:39:06'
    tEXt b'RenderTime\x0000:00.01'
    tEXt b'cycles.ViewLayer.total_time\x0000:00.00'
```

So the reading that survives both runs is this: whether two files match is decided by whether every time written into them happens to round alike, and nothing else — not the version, not the stamp settings, not the number of cards. Neither run is overwritten and neither is preferred; the question is sent back to the hand tests to be run many times over rather than twice. *(End of the four-card citation check's note.)*


**With that settled, the pictures of the four passes were compared**, value by value, with a control. Frame 1, of 921,600 values: **38 differed between four cards and one card, and 9 between two separate runs on four cards**, the largest difference in both being one step in 255. So the four-card comparison differed in about four times as many values as the control, every difference anywhere being that same smallest step. **Nothing here shows the number of cards changing the picture, and nothing here rules it out either.** *(Four-card citation check, 2026-08-06: this check ran the same four passes again on the same machine. The order was ascending both ways in all four versions, no two files matched in any of the twelve comparisons per version, and the largest pixel difference was again one step in 255 everywhere. The counts came out **36 between four cards and one card and 13 between two runs on four cards**, against **38 and 9** above. Both runs stand. What this shows is that the ratio between the two counts is noise from one run to the next, so the sentence before last — how the four-card figure compares with the control — is about this run alone and not about the version. The last sentence is the one that survives both runs.)*

**Driven from outside — this block.** Every run above was made from outside Blender's window, and on this machine there is no other way: with no display server there is nothing for a window to open on. No window test could be repeated with four cards.

**Operating system — this block.** Linux, on the machine named at the top of it. The stamp result was got on this Linux machine; a checker's separate run on the Creator's Mac in Blender 4.5 also found the file matching with the stamp settings off, and the two are not merged here. Windows is untested.


**Operating system.** The cited command-line, Frame Range, and Rendering Animations pages, and the `bpy.types.Scene` entry, do not name an operating system for frame order. **Observed on two operating systems, and they agree** — macOS 26.5.2 (hand tests 1, 8, 10, 12, 6 and 7, 2026-08-05) and Ubuntu 22.04.5 LTS on the rented Linux machine (Linux hand tests 1, 7, 9, 11, 5 and 24, 2026-08-06). **Observed on both makes of graphics card the project has**, from Linux hand test 30: Apple's own on the Mac and an NVIDIA GeForce RTX 4090 here, with the same result. **The order a render started the way the window starts it (Linux hand test 33) is Linux only**; the Mac was never asked. *(Linux citation check, 2026-08-06: added by this check — the line named only the tests run on both machines.)* **Observed with two graphics cards in one machine, and it changes nothing** — see the block above: ascending order both ways, and the picture differs no more between two cards and one than between two runs on the same cards. **Windows remains untested**, because no Windows machine has run it, and no card of any third make can be reached. **Observed with four graphics cards in one machine as well, on a fourth machine, and it changes nothing** — the order is ascending both ways, and the picture differs no more between four cards and one than between two runs on four cards. That run also settled what Blender writes into a PNG and when two files can match, which is what makes the picture comparison the reliable one. *(Added 2026-08-06 by the hand tests on the four-card machine.)*

---

## 11. Can a render be told to stop once the frame it is working on is finished, rather than the moment it is asked?

**Answer.** **Not in the sources.** Nothing in the 4.5 manual, the Python API documentation, the release notes, or Cycles' own files describes a stop that waits for the current frame to finish. The sources name cancellation and abortion, but do not say whether either happens immediately or at a frame boundary.

**Label: not in the sources.**

**Where I looked.** Manual: *Render Output* in full (Introduction, all five Output Properties pages, Rendering Animations, Animation Player), *Rendering From The Command Line*, *Command Line Arguments* in full including the debug and GPU options, *Topbar → Render Menu*, *Status Bar*, *Viewport Render*, *Troubleshooting → Recovering Data*. Python API: `bpy.ops.render` in full, `bpy.types.RenderSettings` in full, `bpy.types.Scene`, `bpy.types.RenderEngine` in full, `bpy.app.handlers` in full. Release notes: all nineteen 4.5 files searched. Cycles: all Python files searched. Searches run across the manual and the API documentation: "cancel", "cancel rendering", "cancel the render", "stop the render", "stop after", "abort", "pause", "after the current frame", "finish the current frame", "current frame is finished", "graceful", "signal", "SIGINT", "Ctrl-C". The only hits are the ones quoted below, plus unrelated matters (fluid baking, the animation player, grease pencil onion skinning).

**Exact words** — everything the sources do say about stopping:

> "test_break()
>
> Test if the render operation should been canceled, this is a fast call that should be used regularly for responsiveness"

- Source: Python API, `bpy.types.RenderEngine`.
- Address: https://docs.blender.org/api/4.5/bpy.types.RenderEngine.html
- Date read: 2026-07-30

> "Running Task
>
> Shows the progress of the currently running task (such as rendering or baking). Hovering the mouse pointer over the progress bar will display a time estimate. The task can be aborted by clicking the cancel button ()."

- Source: manual, *Status Bar*.
- Address: https://docs.blender.org/manual/en/4.5/interface/window_system/status_bar.html
- Date read: 2026-07-30

> "As with a normal render, you can abort it with Esc."

- Source: manual, *Viewport Render* (said of a viewport render, comparing it to a normal render).
- Address: https://docs.blender.org/manual/en/4.5/editors/3dview/viewport_render.html
- Date read: 2026-07-30

> "bpy.app.handlers.render_cancel
>
> on canceling a render job. Accepts one argument: the scene datablock being rendered"

- Source: Python API, `bpy.app.handlers`.
- Address: https://docs.blender.org/api/4.5/bpy.app.handlers.html
- Date read: 2026-07-30

**Not covered.** The absence above is about a *built-in* stop-after-this-frame. It does not settle whether the same effect can be reached another way — for instance by a script that watches `render_write` and then cancels, or by lowering the end frame mid-render. The sources describe neither. It also does not settle what a cancel does to the frame in progress: whether the partly-rendered frame is discarded, written, or left half-written is nowhere stated.

**Observed — hand tests 6 and 7, 2026-08-05.** A stop does not wait for the current frame, and the frame in progress is thrown away. Blender 4.5.12 was started from a terminal with no window, rendering the default cube scene in Cycles at 1000 samples and 400 by 400 pixels to PNG, frames 1 to 5, at about seven seconds a frame. Fifteen seconds in — during frame 3 — it was sent an interrupt, the same thing Ctrl-C does. It printed:

```
Sent an internal break event. Press ^C again to kill Blender

Blender quit
```

and ended with exit code `1`. The folder afterwards held `f_0001.png` and `f_0002.png` at 94352 bytes each, and nothing for frame 3. Frame 3's work was discarded, not written and not half-written.

How long the interrupt took to bite is worth recording, because it differs between versions and the pattern was steady over three runs each. In Blender 4.5.12 the process ended 5.8, 5.6 and 5.6 seconds after the interrupt — which is about as long as the rest of frame 3 would have taken, so 4.5 appears to finish the work on the frame before quitting, and then throw it away. In 5.0.1 and 5.1.2 the same test ended in 0.8 to 1.0 seconds. In 5.2.0 it ended in 5.0 seconds. In none of the four was a file written for the interrupted frame.

A polite terminate signal instead ended it in 0.2 seconds with exit code `-15`, printing nothing further, leaving the same two files. A kill that could not be caught ended it in 0.2 seconds with exit code `-9`, leaving the same two files.

Run on the Creator's MacBook Pro, M1 Pro chip, macOS 26.5.2.

**Observed — hand test 23, 2026-08-05.** Lowering the end frame while the render runs does not stop it. This was the obvious route to a stop-after-this-frame, and it does not work. Blender 4.5.12 rendered frames 1 to 8 with a handler attached to Blender's own "frame written" event; the moment frame 2 was written, the handler set the end frame to 2:

```
HANDLER: frame 2 has just been written; setting frame_end to 2
HANDLER: frame_end now reads 2
```

All eight frames were rendered anyway:

```
f_0001.png … f_0008.png, 54405 bytes each
```

So the animation loop does not re-read the end frame as it goes. Same machine and date.

**Observed — hand test 29, run by the Creator, 2026-08-05.** Blender's own window offers nothing that stops a render at a frame boundary either. With an animation render running in Blender 4.5.12, the Creator opened the topbar's **Render** menu. It held six items and no more:

> Render Image — F12
> Render Animation — ⌘ F12
> Render Audio…
> View Render — F11
> View Animation — ⌘ F11
> Lock Interface (a checkbox)

There is no stop, pause, cancel or finish-this-frame item among them.

The status bar at the bottom showed a progress bar with a percentage beside a small cross. Hovering the cross gave the tooltip:

> Stop this job.

Hovering the progress bar itself gave:

> Time Remaining: 01:29.82
> Time Elapsed: 01:04.05

and the render information line across the top of the image editor read, at that moment:

> Frame:3 | Last:00:25.98 Time:00:10.94 | Remaining:00:15.34 | Mem:2.55M, Peak:2.55M | Scene, ViewLayer | Sample 1665/4000

So the only stop the window offers is the one the manual describes, and it is offered as "Stop this job" with nothing about waiting for the frame.

On when it takes effect, the Creator's own words: "Stopping isn't instant, there's a slight lag. All versions." They did not time the lag. That matches what the runs from a terminal measured — 5.6 to 5.8 seconds in this version, against under a second in 5.0 and 5.1 — though those were an interrupt sent to a Blender with no window rather than Esc pressed in one.

Afterwards the output folder held exactly `frame_0001.png` and `frame_0002.png`, 94524 bytes each, and nothing for the frame that was in flight.

Run on the Creator's Mac — a MacBook Pro, M1 Pro chip, macOS 26.5.2 — so this settles macOS and leaves Windows and Linux open.

**Outside the interface.** Both stopping methods the sources name (Esc, the status bar's cancel button) are interface actions, and there is no counterpart documented for a render driven from outside — no command-line argument, no operator, no Python call. So for a render started from outside the interface, how to stop it at all is **not in the sources**, let alone how to stop it at a frame boundary. This is the gap that matters most for the question as asked. **Observed**, from hand tests 6, 7 and 23: such a render *can* be stopped, by an interrupt, a polite terminate, or a kill; none of the three waits for the current frame; and no way was found to make one wait. **Observed**, from hand test 29: inside the window there is no such control either — the Render menu offers no stop of any kind, and the only one there is is the status bar's cross, labelled "Stop this job." So the answer is the same on both sides of the window: nothing stops a render at a frame boundary, and the frame in flight is thrown away.

**Observed — Linux hand tests 5, 6 and 22, 2026-08-06.** The same answer on the rented Linux machine. An interrupt, a polite terminate and an uncatchable kill each stopped Blender 4.5.12 there, none of them waited for the current frame, and none left anything for it. The handler route failed the same way: with a handler on Blender's own "frame written" event setting the end frame to 2 the moment frame 2 was written, all eight frames were rendered anyway.

```
HANDLER: frame 2 has just been written; setting frame_end to 2
HANDLER: frame_end now reads 2
the render call has returned
---- FOLDER AFTERWARDS ----
f_0001.png  54388 bytes
f_0002.png  54388 bytes
f_0003.png  54388 bytes
f_0004.png  54388 bytes
f_0005.png  54388 bytes
f_0006.png  54388 bytes
f_0007.png  54388 bytes
f_0008.png  54388 bytes
```

**On how long the stop took, this machine cannot say.** Three interrupts were sent in each version, and the process ended 2.8, 1.4 and 7.6 seconds later in 4.5; 1.8, 3.6 and 4.8 in 5.0; 1.0, 1.2 and 1.2 in 5.1; and 4.0, 4.2 and 3.8 in 5.2. Those numbers are raw output kept for the record and **are not a measurement**: the machine was shared with other work throughout, which can only stretch them. The Mac's steady pattern — about five and a half seconds in 4.5 and five in 5.2, against about one second in 5.0 and 5.1 — is neither confirmed nor contradicted here. Settling it needs the machine to itself, and that is written into `Audits/Blender Research/Frames/Hand tests — Linux.md`.

**Observed — Linux hand test 32, 2026-08-06.** Blender's own window offers nothing that stops a render at a frame boundary on this machine either. With an animation render running in Blender 4.5.12 on the rented Linux machine's display — started the way the menu item starts it — the topbar's **Render** menu was opened with Blender's own `wm.call_menu` and photographed with Blender's own screenshot command. The picture shows six items and no more: Render Image (F12), Render Animation (Ctrl F12), Render Audio…, View Render (F11), View Animation (Ctrl F11), and a Lock Interface checkbox. No stop, pause, cancel or finish-this-frame item. The status bar at the foot of the window shows the word **Render…**, then a progress bar with a percentage in it, then a small cross. (In 5.0, 5.1 and 5.2 the same bar reads **Rendering…**.)

Two things were not settled there and are not claimed. The cross's tooltip was not read, because hovering needs the pointer. And **whether the Esc key reached Blender could not be demonstrated** — that machine's display is shared with other work, and in one version a further frame was written after Esc was sent, which says the render was still going. So the "Stop this job." tooltip and the Esc cancel rest on the Creator's macOS run alone.

**Operating system.** The manual and API pages, release notes, and Cycles files listed in **Where I looked** do not name an operating system for this kind of stop. **Observed on two operating systems, and they agree that there is no such control** — macOS 26.5.2 (hand tests 6, 7, 23 and 29, 2026-08-05) and Ubuntu 22.04.5 LTS on the rented Linux machine (Linux hand tests 5, 6, 22 and 32, 2026-08-06). Nothing stops a render at a frame boundary on either, the frame in flight is thrown away on both, and the Render menu holds the same six items on both. **How long a stop takes to bite is settled on macOS only**, for the reason above. **Windows remains untested**, because no Windows machine has run it.

---

## Things a person with Blender 4.5 open could still settle by hand

The list that stood here has been run — the results are written into the answers above as observed findings, dated 2026-08-05 for the Creator's Mac and 2026-08-06 for the rented Linux machine. Two of the four items that were left have since been answered: **Autosplit at the 2 GiB boundary** was reached on the rented Linux machine and is recorded under question 5, and **Linux and a graphics card of another make** are now covered throughout. Three items are left, and none of them is a matter of somebody sitting down at Blender for a few minutes. **Observed 2026-08-06 on a third machine, and it added one more:** two graphics cards in one machine leave the frame order unchanged, and no change in the picture was shown — see question 10. *(Two-card citation check, 2026-08-06: this read "change neither the frame order nor the picture". The pixel counts do not carry that: they show no difference beyond a single step in 255, and in 4.5 and 5.1 rather more of those than two runs on the same cards showed.)*

1. **What an abrupt power-off leaves behind.** (Question 1.) A Blender killed outright is recorded under question 1 and leaves the finished frames whole, on both machines. What that cannot reach is a file the operating system had not yet flushed to disk when the power went. This was written out as a test and offered; the Creator declined it, which is their call. It cannot be run on the rented Linux machine either, because that machine is reached over the network and cutting its power would leave nothing to read the result from.

2. **What a media player does with a part-written video file, on anything but QuickTime on macOS.** (Question 5.) QuickTime Player is recorded under question 5. The rented Linux machine has no media player installed at all and installing one was not permitted, so only Blender's own reader has opened these files there.

3. **Anything on Windows.** No Windows machine has run it, so every observed finding above covers macOS and Linux and leaves Windows open.

Three more things need the rented machine to itself rather than a machine shared with other work, and they are written out in `Audits/Blender Research/Frames/Hand tests — Linux.md`: how long a stop takes to bite, the pointer half of the window tests, and a demonstrated Esc cancel.

---

2026-08-04 — Opus 5 (Claude Code) — Frames, Blender 4.5

2026-08-05 — Opus 5 (Claude Code) — hand-test results written into questions 1 to 11, and the hand-test list above pruned

2026-08-05 — Opus 5 (Claude Code) — the Creator's own hand-test results written into questions 1, 3, 5, 7, 8 and 11, and the hand-test list above pruned again

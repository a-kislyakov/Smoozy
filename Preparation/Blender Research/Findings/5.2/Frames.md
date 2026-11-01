# Frames — Blender 5.2

**Sources consulted, and when.** Blender 5.2 manual (plain-text copy, including two pictures from its `_images` folder), Blender 5.2 Python API documentation, Blender 5.2 release notes, and Cycles' own files as shipped with Blender 5.2 — all as supplied in `Blender Sources/`, all read on **2026-07-30** (the snapshot date I was told to use). I also opened `Blender Sources/About this folder.md` and `Blender Sources/requirements/requirements_2026-08-01.txt`; the requirements page never names 5.2, so I have not used it as evidence for this version.

I read only the question sheet and files inside Blender Sources/. I did not open any other part of this project.

I did not use the web. I did not use any Blender knowledge of my own. No helper agents were used — I did all of this work myself, so the declaration above covers everything that went into this file.

---

## 1. Can a render that was interrupted by the computer going down be picked up from where it stopped?

**Answer.** Yes, when the animation was being written as separate image files: the frames already on disk are still there, and re-running the render with Overwrite switched off continues from where it stopped. The manual states this as its own advice for exactly this event.

**Label: documented.**

**Exact words.**

> "Your computer accidentally turns off in the middle of rendering your movie!
>
> Unless your animation renders in a few minutes, it is best to render the animation as separate image files. Instead of rendering directly to a compressed movie file, use a lossless format (e.g. PNG).
>
> This allows you an easy recovery if there is a problem and you have to re-start the rendering, since the frames you have already rendered will still be in the output directory.
>
> Just disable the Overwrite option to start rendering where you left off."

- Source: the manual, "Rendering Animations", Hints section — https://docs.blender.org/manual/en/5.2/render/output/animation.html
- Date read: 2026-07-30

**Not covered.** The manual says the *already-finished* frames survive and that you restart from there. It does not say anything about the frame that was mid-render when the power went — see question 6. It says nothing about picking up part-way *inside* a frame (no partial-frame resume is described anywhere in the sources). It does not describe recovery for the video-file case, other than the warning quoted under question 5. It says nothing about how the render is re-launched, only that Overwrite is disabled before it is.

**Observed — hand test 7, 2026-08-05.** A computer going down cannot be reproduced from a terminal, but a Blender killed outright, with no chance to tidy up, is the closest thing to it. Blender 5.2.0 was started from a terminal with no window, rendering the default cube scene in Cycles at 1000 samples and 400 by 400 pixels, PNG output into an empty folder, frames 1 to 5 at about six and a half seconds a frame. Fifteen seconds in, while frame 3 was rendering, it was sent a kill it could not catch. It ended 0.2 seconds later with exit code `-9`. The folder afterwards:

```
f_0001.png  92440 bytes
f_0002.png  92440 bytes
```

The two finished frames survived intact, and nothing at all was left for the frame in flight — which settles part of what this answer records as not covered: with Placeholders off, no file for the frame in flight is left behind at all. Run on the Creator's MacBook Pro, M1 Pro chip, macOS 26.5.2.

**Observed — hand test 18, 2026-08-05.** The same kill, with the output set to a single video file, left a 48-byte file holding no frames — so for video output the recovery does not exist. Recorded in full under question 5.

**Still unsettled, and why.** A real power-off was written as hand test 31 and marked optional. The Creator declined it — "I would rather not do Test 36", 2026-08-05, that being its number on the combined list they were working from. It is their own choice, not a limit of the equipment, and it has been moved to the "cannot be run" section of the hand-test list. So the question splits in two. The half where **the program dies** is answered above: an uncatchable kill leaves every finished frame whole and nothing at all for the frame in flight. The half where **the operating system had not yet flushed a file to disk** when the power went is not answered, and nothing short of a real power-off reaches it.

**Observed — Linux hand test 6, 2026-08-06.** The same kill, on a second machine, gave the same answer. This was run on a rented Linux machine — Ubuntu 22.04.5 LTS, NVIDIA GeForce RTX 4090, driver 580.105.08, twelve cores, run over SSH. Blender 5.2.0 was started there from a terminal with no window, against Blender's own startup scene, with its own settings folder and factory startup. Rendering the default cube scene in Cycles at 1000 samples and 400 by 400 pixels, PNG output into an empty folder, frames 1 to 5, it was sent a kill it could not catch fifteen seconds in. The exit code recorded was `-9`, and the finished frames were on disk whole — f_0001.png, f_0002.png and f_0003.png at 92435 bytes each — with nothing at all for the frame that was in flight. How many frames had finished by then differed from the Mac run, because the machine was shared with other work; no claim is made here about how fast anything ran.

**The half this cannot reach stays unreachable on both machines.** A real power-off was offered to the Creator and declined. It cannot be run on the rented Linux machine either: that machine is reached over the network, so cutting its power would leave nothing to read the result from.

**Observed — the nearest thing to it that could be run, on the rented Linux machine, 2026-08-06.** The Creator restarted the machine from the provider's own console while a 500-frame render was running. That is not a power cut, but it is an abrupt stop from outside the machine, and the machine's own login record shows the previous session ending in a crash rather than a shutdown — so nothing was given a chance to tidy up.

*What was done, to what.* A 500-frame render at 720 by 720, 300 samples, on the graphics card, started detached so that closing the connection could not end it, writing PNG frames into an empty folder. A second process wrote the count of finished frames into a file every five seconds. The Creator then clicked restart in Vast.ai's interface. The machine was gone for under a minute.

*What came back.* Blender had announced eleven frames as saved before the restart. **All eleven were on disk afterwards, whole, each ending as a PNG must.** Nothing was part-written:

```
whole 11, zero-byte 6, part-written 0
```

*And six more files sat beside them at zero bytes* — `f_0012.png` through `f_0017.png` — which are the placeholders this version writes ahead of itself. So the folder held seventeen files where eleven were pictures. The render itself did not come back, and nothing on the machine restarted it.

*On what kind of computer.* The rented Linux machine, over SSH, with the restart given from the provider's web interface rather than on the machine. **This settles Blender 5.2 on Linux only** — it was not run in 4.5, 5.0 or 5.1, and not on the Creator's Mac. A true power cut is still untested everywhere, and the difference may matter: a restart still lets the machine write out what it is holding, and a power cut does not.

**Outside the interface.** The Overwrite setting is a scene render setting (`bpy.types.RenderSettings.use_overwrite`, "Overwrite existing files while rendering (default True)" — https://docs.blender.org/api/5.2/bpy.types.RenderSettings.html, read 2026-07-30), so it is stored in the blend-file and applies to a render started from the command line too; the manual's command-line page documents `-a, --render-anim` for rendering the animation with "all the settings saved in the blend-file" (https://docs.blender.org/manual/en/5.2/advanced/command_line/render.html, read 2026-07-30). Beyond that, the sources do not say whether the recovery advice differs outside the interface. **Observed**, from hand tests 2 and 7: the whole recovery works on a Blender started from a terminal with no window — Overwrite off skips the frames already on disk (question 4), and the finished frames survive an uncatchable kill.

**Operating system.** The sources do not say. Nothing on the pages above is marked as OS-specific. The manual states Blender "is available for download on Windows, macOS, and Linux" (https://docs.blender.org/manual/en/5.2/getting_started/installing/index.html, read 2026-07-30). **Observed on two operating systems, and they agree** — macOS 26.5.2 on the Creator's MacBook Pro (hand test 7, 2026-08-05) and Ubuntu 22.04.5 LTS on the rented Linux machine (Linux hand test 6, 2026-08-06): an uncatchable kill leaves every finished frame whole and nothing for the frame in flight on both. **Windows remains untested**, because no Windows machine has run it. **Untried rather than untestable: a Windows machine can be rented, and every one this project has read reported a Server edition — see the equipment section of `Preparation/Blender Research/Method.md`.**

---

## 2. Can a render that was stopped on purpose be picked up later?

**Answer.** Yes, for animations rendered to image file formats. The manual names this as an advantage of image output and gives two ways to carry on: change the frame range, or switch Overwrite off.

**Label: documented.**

**Exact words.**

> "Images can also be used for rendering animations which has a couple advantages. For example, when rendering animations to image file formats the render job can be canceled and resumed at the last rendered frame by changing the frame range. This is useful if the animation takes a long time to render and the computers resources are needed for something else."

- Source: the manual, "Render Output ‣ Introduction" — https://docs.blender.org/manual/en/5.2/render/output/introduction.html
- Date read: 2026-07-30

And, listing when to prefer separate image files:

> "May need to interrupt rendering to use the computer, and want to be able to resume rendering where you left off."

- Source: the manual, "Rendering Animations" — https://docs.blender.org/manual/en/5.2/render/output/animation.html
- Date read: 2026-07-30

The Overwrite route is quoted in full under question 1.

**Not covered.** "Resumed at the last rendered frame" is the only statement of where it resumes; the sources do not say what happens to the frame that was in progress when the cancel arrived, nor whether that frame is left on disk in any state. Nothing is said about resuming a render that was stopped part-way through a video file (question 5), nor about resuming a *still* render.

**Observed — hand tests 6 and 7, 2026-08-05.** A render running without a window can be stopped, and the frame in progress does not count as rendered. Blender 5.2.0 was started from a terminal with no window, rendering the default cube scene in Cycles at 1000 samples and 400 by 400 pixels, PNG output into an empty folder, frames 1 to 5, at about six and a half seconds a frame. Fifteen seconds in — during frame 3 — it was sent an interrupt, which is exactly what pressing Ctrl-C in a terminal does. The last lines it printed were:

```
00:13.334  render           | Saved: '…/f_0002.png'
00:13.334  render           | Time: 00:06.38 (Saving: 00:00.01)
00:13.334  render           | Rendering frame 3
00:13.334  render           | Start rendering: Scene, ViewLayer
00:13.334  render           | Engine: Cycles

Sent an internal break event. Press ^C again to kill Blender

Blender quit
```

It ended 5.0 seconds after the interrupt (three separate runs: 5.0, 5.0, 5.0 seconds) with exit code `1`. The folder afterwards:

```
f_0001.png  92440 bytes
f_0002.png  92440 bytes
```

So "the last rendered frame" is unambiguous: only whole frames are on disk, and the frame that was in flight left nothing in any state. A polite terminate signal instead ended it in 0.2 seconds with exit code `-15`, printing nothing further, and left the same two files. Run on the Creator's MacBook Pro, M1 Pro chip, macOS 26.5.2.

**Observed — Linux hand tests 5 and 6, 2026-08-06.** On the rented Linux machine, Blender 5.2.0 printed exactly what it printed on the Mac when sent an interrupt fifteen seconds into the same render:

```
Sent an internal break event. Press ^C again to kill Blender

Blender quit
```

and exited with code `1`, leaving only whole frames and nothing for the frame in flight. A polite terminate ended it with code `-15` and a kill with code `-9`, both leaving the same. **The render did not freeze** — worth recording because Blender 5.0's release notes carry a fixed bug called "Linux Freeze when force cancelling a CLI render", and nothing of that kind was seen here in any of the four versions. How long each stop took to bite is kept under question 11 as raw output and is not offered as a measurement: the machine was shared.

**Outside the interface.** Both routes are scene settings — the frame range (`Scene.frame_start` / `frame_end`) and `use_overwrite` — so both are available to a render launched from the command line, and the command line additionally lets the range be set on the spot with `-s, --frame-start <frame>` and `-e, --frame-end <frame>` (https://docs.blender.org/manual/en/5.2/advanced/command_line/arguments.html, read 2026-07-30). How the render is *stopped* does differ: inside the interface the manual describes a cancel button — "The task can be aborted by clicking the cancel button ()." (https://docs.blender.org/manual/en/5.2/interface/window_system/status_bar.html, read 2026-07-30) — and the sources describe no equivalent for a render running without a window. **Observed**, from hand tests 6 and 7: there is one all the same — an interrupt, a polite terminate, or a kill all stop such a render, and all three leave only whole frames behind.

**Operating system.** The sources do not say. None of the pages above marks anything as OS-specific. **Observed on two operating systems, and they agree** — macOS 26.5.2 (hand tests 6 and 7, 2026-08-05) and Ubuntu 22.04.5 LTS on the rented Linux machine (Linux hand tests 5 and 6, 2026-08-06). An interrupt, a polite terminate and an uncatchable kill each stop the render on both, with the same three exit codes, and each leaves only whole frames. **Windows remains untested**, because no Windows machine has run it.

---

## 3. Do frames already written survive a render ending part-way, whether it was stopped on purpose or not?

**Answer.** Yes, for an animation written as separate image files. The manual states it for the accidental case in so many words, and states the resume-from-last-rendered-frame behaviour for the deliberate case.

**Label: documented** (accidental case, and image output). **Inferred** for the deliberate case, see below.

**Exact words.**

> "This allows you an easy recovery if there is a problem and you have to re-start the rendering, since the frames you have already rendered will still be in the output directory."

- Source: the manual, "Rendering Animations", Hints — https://docs.blender.org/manual/en/5.2/render/output/animation.html
- Date read: 2026-07-30

Supporting, on how frames get to disk in the first place:

> "animations are a series of images, or frames, and are typically saved directly to disk as they are rendered."

- Source: the manual, "Rendering Animations" — https://docs.blender.org/manual/en/5.2/render/output/animation.html
- Date read: 2026-07-30

**The inferred step (deliberate stop).** The manual does not say "frames survive a deliberate cancel" in those words. It says the job "can be canceled and resumed at the last rendered frame" (Render Output ‣ Introduction, quoted under question 2). To resume at the last rendered frame you must be able to tell which frames are already done, and the only mechanism the sources give for that is the frames sitting in the output directory. So: the reader must supply the step "resuming at the last rendered frame implies the earlier frames are still on disk". That is inference, not documentation.

**Not covered.** This is about *finished* frames only. It says nothing about the frame that was in flight. It does not hold for the single-video-file case — see question 5. It does not cover output written by a Compositor File Output node rather than the Output panel.

**Observed — hand tests 1, 6 and 7, 2026-08-05.** The deliberate stop no longer needs the inferred step: it was tested, and it gives the same answer as the accidental case. Blender 5.2.0 was started from a terminal with no window, rendering the default cube scene to PNG, and stopped part-way in three different ways — an interrupt (Ctrl-C), a polite terminate, and a kill it could not catch. In all three the folder afterwards held the finished frames, whole:

```
f_0001.png  92440 bytes
f_0002.png  92440 bytes
```

and nothing for the frame in flight. The whole run, undisturbed, writes one frame at a time:

```
00:00.402  render           | Rendering animation (frames 1..5)
00:00.402  render           | Rendering frame 1
00:00.402  render           | Start rendering: Scene, ViewLayer
00:00.402  render           | Engine: Workbench
00:00.558  render           | Saved: '…/f_0001.png'
00:00.558  render           | Time: 00:00.15 (Saving: 00:00.07)
00:00.558  render           | Rendering frame 2
…
00:00.589  render           | Saved: '…/f_0005.png'
```

Run on the Creator's MacBook Pro, M1 Pro chip, macOS 26.5.2.

**Observed — hand test 29, run by the Creator, 2026-08-05.** The same holds for a render started and cancelled inside Blender's own window, which is the deliberate-stop case this answer marks inferred. A prepared file was opened in Blender 5.2.0 — Cycles, 4000 samples, 400 by 400 pixels, PNG output into an empty folder, frames 1 to 10, about twenty-five seconds a frame. Render Animation was started, frames 1 and 2 were allowed to finish, and Esc was pressed while frame 3 was under way. Afterwards the output folder held exactly two files and nothing else:

```
92465 bytes  frame_0001.png
92465 bytes  frame_0002.png
```

The Creator ran the render and the cancel and left the folder for reading; the listing above is what was on disk afterwards. Their own words on the cancel: "You can probably look at the files yourself. Stopping isn't instant, there's a slight lag. All versions." Run on the Creator's Mac — a MacBook Pro, M1 Pro chip, macOS 26.5.2 — so this settles macOS and leaves Windows and Linux open.

**Observed — Linux hand tests 1, 5 and 6, 2026-08-06.** The same three ways of ending a render were tried on a rented Linux machine — Ubuntu 22.04.5 LTS, NVIDIA GeForce RTX 4090, driver 580.105.08, twelve cores, run over SSH and gave the same answer: every frame already written was there, whole, and nothing at all was left for the frame in flight. Undisturbed, the plain five-frame run there printed the same shape of output as on the Mac — `Rendering animation (frames 1..5)`, then `Rendering frame N`, `Start rendering: Scene, ViewLayer`, `Engine: Workbench` and `Saved:` for each frame, all timestamped and tagged `render` — exit code 0, five files of 3639 bytes each. One thing differs between the machines and is not about frames: the same scene gives a 3639-byte file on Linux where the Mac gave a different size. Nothing on this sheet asks about pixels; it is recorded only so nobody assumes a frame rendered on two machines is the same file.

**Observed — Linux hand test 32, 2026-08-06.** A render started the way the window's own Render Animation item starts it also leaves only whole frames. With a window open on the rented Linux machine's display, Blender 5.2.0 was given `bpy.ops.render.render('INVOKE_DEFAULT', animation=True)` — the call that menu item makes — which returned `{'RUNNING_MODAL'}`, and a separate **Blender Render** window opened. The render was ended part-way and the folder held whole frames of 92434 bytes and nothing for the frame in flight. Note what this does **not** show: whether the **Esc** key reached Blender could not be demonstrated on that machine, because its display is shared with other work and a key press could not be shown to land in the intended window. So this is a render that was ended, not a demonstrated Esc cancel; the Esc half rests on the Creator's macOS run.

**Outside the interface.** The sources do not say this differs. Frames are written by the render job itself, and the command-line page documents animation rendering with the same settings; nothing on these pages distinguishes the two. **Observed**, from the hand tests above: it holds for a render started from a terminal with no window, and hand test 29 shows it holds for a render started and cancelled inside the window too — the same two whole frames, nothing for the third. So this answer no longer depends on how the render was started.

**Operating system.** The sources do not say. **Observed on two operating systems, and they agree** — macOS 26.5.2 (hand tests 1, 6, 7 and 29, 2026-08-05) and Ubuntu 22.04.5 LTS on the rented Linux machine (Linux hand tests 1, 5, 6 and 32, 2026-08-06). Frames already written survive on both, whichever way the render ends. **Windows remains untested**, because no Windows machine has run it.

---

## 4. Can Blender be asked to render only the frames that are not already on disk, and if so, how does it decide what counts as already there?

**Answer.** Yes — switch Overwrite off. The sources are explicit that the switch exists and what it is for; they are *not* explicit about the test it applies. The only test named anywhere is the plain existence of the output file for that frame.

**Label: documented** for the switch. **Inferred** for the decision rule.

**Exact words — the switch.**

> "Image Sequence – Overwrite Image Multi-Layer EXR
>
> Overwrite existing files when rendering."

- Source: the manual, "Output" properties panel — https://docs.blender.org/manual/en/5.2/render/output/properties/output.html
- Date read: 2026-07-30

> "use_overwrite
>
> Overwrite existing files while rendering (default True)"

- Source: the Python API documentation, `bpy.types.RenderSettings` — https://docs.blender.org/api/5.2/bpy.types.RenderSettings.html
- Date read: 2026-07-30

**Exact words — what it is used for.**

> "Just disable the Overwrite option to start rendering where you left off."

> "Using an image format for output, you can use the Frame Step option to render every N'th frame. Then disable Overwrite and re-render with Frame Step set to 1."

- Source: the manual, "Rendering Animations" — https://docs.blender.org/manual/en/5.2/render/output/animation.html
- Date read: 2026-07-30

**Exact words — the companion switch, and the intended multi-machine use.**

> "Image Sequence – Overwrite Image Multi-Layer EXR
>
> Create empty placeholder frames while rendering."
>
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

- Source: the manual, "Output" properties panel — https://docs.blender.org/manual/en/5.2/render/output/properties/output.html
- Date read: 2026-07-30

(Note on that page: the two headings quoted above are byte-for-byte identical — "Image Sequence – Overwrite Image Multi-Layer EXR" — although their bodies describe two different settings. The second one's body is the Placeholders description. I am reporting the page as it stands.)

**The inferred step (the decision rule).** Nothing in the sources spells out the test. Chaining what is written: the setting is named "Overwrite **existing files**"; the file a given frame would be written to is a computable thing, since the API offers `frame_path`, described as "Return the absolute path to the filename to be written for a given frame" (https://docs.blender.org/api/5.2/bpy.types.RenderSettings.html, read 2026-07-30); and the render-farm hint has several machines skipping each other's work using nothing but files in a shared directory. The reader has to supply the step "with Overwrite off, a frame is skipped when a file already exists at the path that frame would be written to". That step is mine, not the source's. It follows, but it is inference.

**Corroborating pattern from a file I was given (observed).** Cycles' own denoise-animation operator builds exactly this test — frame range, per-frame path, file-exists check:

```
            for frame in range(scene.frame_start, scene.frame_end + 1):
                scene.render.filepath = in_filepath
                filepath = scene.render.frame_path(frame=frame)
                in_filepaths.append(filepath)

                if not os.path.isfile(filepath):
                    scene.render.filepath = original_filepath
                    err_msg = tip_("Frame '%s' not found, animation must be complete") % filepath
```

- Source file: `Blender Sources/cycles_5.2/operators.py`, lines 79–86, inside `class CYCLES_OT_denoise_animation` (`execute`)
- Date read: 2026-07-30

This is the denoiser deciding what is on disk, **not** the renderer deciding what to skip. It shows the shape Blender's own add-on code uses for the question "is this frame already there"; it is not evidence about the Overwrite check itself.

**Not covered.** How the file names are formed is documented (see question 6 and the File Path description), but nothing states what the check does about a file of zero length, a file with the wrong extension, a file whose padding differs from the current setting, or a directory that does not exist. Nothing states whether the check happens once at the start of the run or per frame. Nothing states what Overwrite does when the output is a video: the manual places the setting in an "Image Sequence" sub-panel shown for the Image and Multi-Layer EXR media types (inferred from that page's heading convention, e.g. "File Format Image", "Color Image Multi-Layer EXR"), which suggests it does not apply to Video output — but the page never says so.

**Observed — hand test 2, 2026-08-05.** The test is bare existence, and a file of zero length counts — one of the things this answer records as unstated. Blender 5.2.0 was started from a terminal with no window and rendered the default cube scene to PNG, frames 1 to 5, into an empty folder. Then `f_0004.png` was deleted outright and `f_0003.png` was emptied to zero bytes but left in place:

```
3712 f_0001.png
3712 f_0002.png
   0 f_0003.png
3712 f_0005.png
```

The same render was then run again with Overwrite off. Blender printed:

```
00:00.399  render           | Rendering frame 1
00:00.399  render           | Skipping existing frame "…/f_0001.png"
00:00.399  render           | Rendering frame 2
00:00.399  render           | Skipping existing frame "…/f_0002.png"
00:00.399  render           | Rendering frame 3
00:00.399  render           | Skipping existing frame "…/f_0003.png"
00:00.399  render           | Rendering frame 4
00:00.553  render           | Saved: '…/f_0004.png'
00:00.553  render           | Rendering frame 5
00:00.554  render           | Skipping existing frame "…/f_0005.png"
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

**Observed — hand test 22, 2026-08-05.** The path a frame is checked against is the one `frame_path` returns, and the file extension is part of it when File Extensions is on. Blender 5.2.0, asked from a terminal with no window:

```
filepath '<dir>/f_'                     -> frame_path(7) = '<dir>/f_0007.png'
filepath '<dir>/f_##_x'                 -> frame_path(7) = '<dir>/f_07_x.png'
filepath '<dir>/plain'                  -> frame_path(7) = '<dir>/plain0007.png'
filepath '<dir>/six_######'             -> frame_path(7) = '<dir>/six_000007.png'
with File Extensions off        -> frame_path(7) = '<dir>/f_0007'
frame_path(12345) = '<dir>/big_12345.png'
```

A frame number wider than the padding is not truncated. Same machine and date.

**Observed — hand test 5, 2026-08-05.** The check happens once per frame as the render reaches it, not once at the start of the run. Two copies of Blender 5.2.0 were started at the same instant from a terminal with no window, each with its own separate settings folder, both rendering frames 1 to 6 of the same slow scene into one shared folder with Overwrite off and Placeholders on. *(Citation check, 2026-08-05: what made the scene slow — its sample count, resolution and format — was not written down, so this run cannot be repeated exactly from what is here. It was run again with the scene stated, as hand test 33 below, and the two runs do not agree on everything.)* The first said:

```
00:10.778  render           | Saved: 'f_0001.png'
00:10.778  render           | Skipping existing frame "f_0002.png"
00:10.778  render           | Skipping existing frame "f_0003.png"
00:21.099  render           | Saved: 'f_0004.png'
00:31.270  render           | Saved: 'f_0005.png'
00:31.270  render           | Skipping existing frame "f_0006.png"
```

and the second said:

```
00:00.486  render           | Skipping existing frame "f_0001.png"
00:10.763  render           | Saved: 'f_0002.png'
00:21.100  render           | Saved: 'f_0003.png'
00:21.100  render           | Skipping existing frame "f_0004.png"
00:21.100  render           | Skipping existing frame "f_0005.png"
00:31.273  render           | Saved: 'f_0006.png'
```

All six frames were present afterwards, each 92504 bytes. Frames were skipped that did not exist when either render started, so the check is per frame. Frame 1 was rendered by both copies, so claiming a frame with a placeholder is not proof against two renders reaching the same frame at the same instant. Run on the Creator's MacBook Pro, M1 Pro chip, macOS 26.5.2, with nothing else running, and to one local folder rather than a shared network one.

**Observed — hand test 33, 2026-08-05.** The same test again, with the scene written down so anyone can repeat it. Two copies of Blender 5.2.0 were started at the same moment from a terminal with no window, each pointed at its own settings folder, made before it was used. Both rendered frames 1 to 6 into one empty shared folder on the local disk. No blend-file was involved: the scene was built by this text, handed to Blender on the command line, and it is the whole of the scene.

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

In words: Blender's own startup scene — cube, light, camera — rendered by Cycles at 800 samples with adaptive sampling and denoising both off, 400 by 400 pixels, Overwrite off, Placeholders on. The rest was given on the command line as `-o "<folder>/f_####" -F PNG -x 1 -s 1 -e 6 -a`, so PNG with file extensions on, frames 1 to 6. Measured on this machine, one frame takes 6.01 seconds with one Blender rendering alone and about 11.8 seconds with two rendering at once — Blender's own `Time:` line in both cases. The first copy printed:

```
00:00.525  render           | Rendering frame 1
00:00.525  render           | Skipping existing frame "f_0001.png"
00:00.525  render           | Rendering frame 2
00:12.416  render           | Saved: 'f_0002.png'
00:12.416  render           | Rendering frame 3
00:12.416  render           | Skipping existing frame "f_0003.png"
00:12.416  render           | Rendering frame 4
00:24.192  render           | Saved: 'f_0004.png'
00:24.192  render           | Rendering frame 5
00:24.192  render           | Skipping existing frame "f_0005.png"
00:24.192  render           | Rendering frame 6
00:36.053  render           | Saved: 'f_0006.png'
```

and the second printed:

```
00:00.525  render           | Rendering frame 1
00:12.415  render           | Saved: 'f_0001.png'
00:12.415  render           | Rendering frame 2
00:12.415  render           | Skipping existing frame "f_0002.png"
00:12.415  render           | Rendering frame 3
00:24.190  render           | Saved: 'f_0003.png'
00:24.190  render           | Rendering frame 4
00:24.191  render           | Skipping existing frame "f_0004.png"
00:24.191  render           | Rendering frame 5
00:36.095  render           | Saved: 'f_0005.png'
00:36.095  render           | Rendering frame 6
00:36.095  render           | Skipping existing frame "f_0006.png"
```

Both exited 0. Blender printed the whole folder path inside each of those file names; only the path has been cut here, the words and the timestamps are as printed. All six frames were present afterwards, each 92504 bytes.

On the first conclusion this agrees with hand test 5. The shared folder was empty when the two started, so every skipped frame was one the other copy had written during the run — the check is made per frame as the render reaches it. **On the second, this run came out the other way.** No frame was rendered twice. Both copies announced frame 1 at 00:00.525, and at that same instant the first copy was told the file already existed and skipped it, while the second went on to render it — so in 5.2 a placeholder can catch the other render on the very first frame. Hand test 5 had both copies render frame 1. Its conclusion still stands, because one collision is enough to show a placeholder is no proof against two renders reaching a frame together; what does not follow, and what this run corrects, is any expectation that they always collide. Two runs of the same test, one each way: which of the two happens is not fixed. Run on the Creator's MacBook Pro, M1 Pro chip, macOS 26.5.2, with nothing else running, and to one local folder rather than a shared network one.

**Observed — hand test 19, 2026-08-05.** Overwrite has no effect on a video output, which settles the reading this answer marks inferred from the panel's headings. Blender 5.2.0 rendered a two-frame FFmpeg MPEG-4 H.264 file from a terminal with no window, then was asked to render exactly the same thing again with Overwrite off. It printed no skipping line, the second run took 13.7 seconds — the full rendering time — and the file's modification time moved from `01:26:00` to `01:26:14`. Same machine and date.

**Observed — Linux hand tests 2, 21 and 18, 2026-08-06.** Everything above about the decision rule came back identical on a rented Linux machine — Ubuntu 22.04.5 LTS, NVIDIA GeForce RTX 4090, driver 580.105.08, twelve cores, run over SSH. Blender 5.2.0 there rendered frames 1 to 5, then `f_0004.png` was deleted and `f_0003.png` emptied to zero bytes, and the render was run again with Overwrite off:

```
00:00.380  render           | Rendering frame 1
00:00.380  render           | Skipping existing frame "…/f_0001.png"
00:00.380  render           | Rendering frame 2
00:00.380  render           | Skipping existing frame "…/f_0002.png"
00:00.380  render           | Rendering frame 3
00:00.380  render           | Skipping existing frame "…/f_0003.png"
00:00.380  render           | Rendering frame 4
00:00.808  render           | Saved: '…/f_0004.png'
00:00.808  render           | Rendering frame 5
00:00.808  render           | Skipping existing frame "…/f_0005.png"
```

The zero-byte file was skipped and stayed zero bytes; the folder afterwards held five files, four of 3639 bytes and `f_0003.png` still at 0. Asked what path frame 7 would take, Linux gave the same six answers as the Mac for every output path tried. And rendering the same two-frame video twice, the second time with Overwrite off, printed no skipping line and moved the file's modification time — so Overwrite has no effect on a video output there either.

**Observed — Linux hand test 27, 2026-08-06, twelve runs, and it does not agree with the Mac.** Two copies of Blender 5.2.0 sharing one folder with Overwrite off and Placeholders on were run **three times in each of the four versions** on the rented Linux machine, using the scene written out in hand test 33 above, unchanged. In all twelve runs the six frames were shared out between the two copies with **no frame rendered twice**. The first run in this version printed, from the first copy:

```
00:00.393  render           | Skipping existing frame "f_0001.png"
00:09.511  render           | Saved: 'f_0002.png'
00:09.511  render           | Skipping existing frame "f_0003.png"
00:19.516  render           | Saved: 'f_0004.png'
00:19.516  render           | Skipping existing frame "f_0005.png"
00:29.458  render           | Saved: 'f_0006.png'
```

and from the second copy:

```
00:08.334  render           | Saved: 'f_0001.png'
00:08.334  render           | Skipping existing frame "f_0002.png"
00:18.866  render           | Saved: 'f_0003.png'
00:18.866  render           | Skipping existing frame "f_0004.png"
00:28.894  render           | Saved: 'f_0005.png'
00:28.894  render           | Skipping existing frame "f_0006.png"
```

Both exited 0, and all six files were 92490 bytes. The other two runs in this version shared the frames out differently again and again showed no frame rendered twice.

**The two machines disagree, and neither is chosen.** On the Mac, at least one frame was rendered by both copies in most runs. On the rented Linux machine, in twelve runs, none was. This is a race between two programs and which way it falls is not fixed. What the Mac showed — that a placeholder can fail to reserve a frame — is not undone by twelve runs here that happened not to collide, and nothing here says a shared folder is safe on Linux. Both records stand. What both machines agree on is the other conclusion: every skipped frame was one the other copy had written during the run, so the check is made per frame as the render reaches it.

**Outside the interface.** `use_overwrite` and `use_placeholder` are scene render settings saved in the blend-file, so they are in force for a command-line render as well; the command-line arguments page lists no flag to set either of them (https://docs.blender.org/manual/en/5.2/advanced/command_line/arguments.html, read 2026-07-30). Whether the skipping behaviour itself differs when Blender is driven from outside — the sources do not say. **Observed**, from the hand tests above: every one set the option from a Python expression on the command line and drove Blender with no window, and the skipping worked.

**Operating system.** The sources do not say. The render-farm hint assumes "a shared directory over a network file system" without naming an operating system. The API's phrasing of the Placeholders setting refers to a Unix idiom — "(similar to Unix 'touch')" — but that is a description of the effect, not a platform restriction. **Observed on two operating systems.** The decision rule, the path checked, the per-frame timing and the lack of effect on a video output are identical on macOS 26.5.2 (hand tests 2, 22 and 19, 2026-08-05) and on Ubuntu 22.04.5 LTS on the rented Linux machine (Linux hand tests 2, 21 and 18, 2026-08-06). **Two Blenders sharing a folder came out differently on the two machines**, as recorded just above. Both machines wrote to a local folder rather than a shared network one, so the render-farm hint's own case is untested on either. **Windows remains untested**, because no Windows machine has run it.

---

## 5. What does an animation written as a single video file leave behind when the render stops part-way?

**Answer.** The sources do not describe the state of the file. They warn about the consequence: the file may be unplayable and the whole animation has to be rendered again from the start.

**Label: documented** (for the warning). **Not in the sources** (for what the file actually contains).

**Exact words.**

> "It's not recommended to render directly to a video format in the first instance. If a problem occurs while rendering, the file might become unplayable and you will have to re-render all frames from the beginning. If you first render out a set of static images such as the default PNG format or the higher-quality OpenEXR (which can retain HDR pixel data), you can combine them as an Image Strip in the Video Sequencer. This way, you can easily:
>
> Restart the rendering from the place (the frame) where any problem occurred."

- Source: the manual, "Output" properties panel, Tips — https://docs.blender.org/manual/en/5.2/render/output/properties/output.html
- Date read: 2026-07-30

Also relevant to what is on disk, though not about interruption:

> "Autosplit Output
>
> If your video is huge and exceeds 2GiB, enable Autosplit Output. This will automatically split the output into multiple files after the first file is 2GiB in size."

- Source: same page, Encoding panel — https://docs.blender.org/manual/en/5.2/render/output/properties/output.html
- Date read: 2026-07-30

And on the Direct Approach:

> "The Direct Approach, which is highly not recommended and not a standard practice, is where you set your output format to an AVI or MOV format, and click Animation to render your scene directly out to a movie file. Blender creates one file that holds all the frames of your animation."

- Source: the manual, "Rendering Animations" — https://docs.blender.org/manual/en/5.2/render/output/animation.html
- Date read: 2026-07-30

**Where I looked, for the part that is not in the sources.** Across the whole 5.2 manual I searched for `unplayable`, `corrupt`, `incomplete`, `partially`, `partial file`, `truncated`, `overwrite`, `placeholder`, `resume`, `abort`, `cancel`, `stop rendering`. The only hit bearing on a half-written video is the "might become unplayable" sentence quoted above. The Encoding panel, the video formats page (`files/media/video_formats.txt`), the Rendering Animations page and the Render Output introduction all describe video output without saying what is left on disk after an interruption. The 5.2 release notes carry nothing on the subject: `rendering.md`'s Render Output section lists two items — "Saving render output can now be disabled." and "Light/shadow linking can be easily copied to the selection" — and the rest of the page is colour-management and shader-node items (https://developer.blender.org/docs/release_notes/5.2/rendering/) *(citation check, 2026-08-05: the second Render Output item was missing from this list; added from the page)*, and `pipeline_io.md`'s Movies section covers stereoscopic and panoramic metadata export only (https://developer.blender.org/docs/release_notes/5.2/pipeline_io/); read 2026-07-30. Cycles' own files contain nothing about video containers or frame writing — my searches there for `frame`, `overwrite`, `placeholder`, `resume`, `checkpoint` turned up only a motion-blur seed description and the denoise operator quoted under question 4.

**Not covered.** Whether a partly written video file exists at all, whether it has a length, whether it holds the frames rendered so far in any recoverable form, and whether the answer differs by container (AVI, MOV, MP4) or codec. Whether Autosplit changes any of this. Whether a video render can be resumed at all — the quoted Tip says the opposite ("re-render all frames from the beginning") but only as advice against the approach, not as a statement of a rule.

**Observed — hand test 18, 2026-08-05.** What is left depends entirely on how the render was stopped, and the two answers are opposite. Blender 5.2.0 was started from a terminal with no window, rendering the default cube scene in Cycles at 1000 samples and 400 by 400 pixels, output set to FFmpeg, container MPEG-4, codec H.264, Autosplit off, frames 1 to 20, at about six and a half seconds a frame. The output folder was watched twenty times a second:

```
  0.00s  (nothing)
  0.81s  movie_0001-0020.mp4=48 bytes
 20.01s  SENT SIGINT
 23.20s  movie_0001-0020.mp4=2291 bytes
```

So a file does exist, from almost the start, at 48 bytes; it does not grow while the render runs; and it takes its content at the moment of the stop. Blender exited with code `1`. Note the name: the whole frame range was baked into it, `movie_0001-0020.mp4`, at the moment the render started — before Blender knew it would not finish.

Asked to open the file it had left, Blender 5.2.0 said:

```
file: movie_0001-0020.mp4 2291 bytes
OPENED. frame_duration = 2  size = (400, 400)  fps = 24.0
```

So a politely stopped video render leaves a file that opens and holds the frames finished before the stop — two of them here. For this way of stopping, "might become unplayable" turns out not to happen at all.

The same render, killed outright instead — no chance to tidy up — left this:

```
movie_0001-0020.mp4  48 bytes
```

and opening it gave `OPENED. frame_duration = 0`. Everything rendered was lost. Run on the Creator's MacBook Pro, M1 Pro chip, macOS 26.5.2. What an ordinary media player makes of either file was settled by hand test 30, recorded below.

**Observed — hand test 21, 2026-08-05.** With Autosplit Output switched on and the same interrupt twenty seconds in, Blender 5.2.0 left one file, `movie__0000001-0020.mp4`, 2418 bytes, which opened with `frame_duration = 3` — the same result as with Autosplit off, but under a different name: an extra underscore and seven-digit frame numbers instead of four. The render never came near the 2 GiB boundary, so what Autosplit does *at* that boundary is still not established; the reason it could not be reached is written into the hand-test list.

**Observed — hand test 30, run by the Creator, 2026-08-05.** An ordinary media player treats the two files completely differently, which is as sharp an answer to "might become unplayable" as this question is going to get. The two files Blender 5.2.0 left — one from a render asked to stop politely twenty seconds in, one from the same render killed outright — were opened in QuickTime Player on the Creator's Mac.

The politely stopped one opened. The player window showed the rendered cube, and both time counters read `00:00`. The Creator's own words: "Part-way plays about half a second I think."

The killed one was refused, with this message:

> The document "killed outright — Blender 5.2.mp4" could not be opened.
>
> The file isn't compatible with QuickTime Player.

The Creator's written answer covered the part-way files; what the killed files did is taken from the screenshots they supplied, one per version, all four showing the same message with only the file name changed. Whether VLC was also tried is not recorded.

Run on the Creator's Mac — a MacBook Pro, M1 Pro chip, macOS 26.5.2 — so this settles macOS and leaves Windows and Linux open. It also leaves open what any player other than QuickTime does with either file, and what Blender's own Video Sequencer makes of them.

**Observed — Linux hand tests 17 and 20, 2026-08-06.** The same two stops on a rented Linux machine — Ubuntu 22.04.5 LTS, NVIDIA GeForce RTX 4090, driver 580.105.08, twelve cores, run over SSH gave the same two opposite answers. The folder was watched twenty times a second: the video file appeared at 48 bytes within half a second of the render starting, named `movie_0001-0020.mp4` with the whole frame range baked into it, and did not grow while the render ran. Sent an interrupt twenty seconds in, it grew once, to 2428 bytes, and Blender 5.2.0 exited with code `1`. Asked to open what it had left:

```
file: movie_0001-0020.mp4 2428 bytes
OPENED. frame_duration = 3  size = (400, 400)  fps = 24.0
```

The same render killed outright instead left the file at its opening 48 bytes, and opening that gave `frame_duration = 0  size = (256, 256)  fps = 0.0`. With Autosplit switched on and the same interrupt below the boundary, one file was left, `movie__0000001-0020.mp4`, 2491 bytes, which opened with `frame_duration = 4` — the same under-a-different-name result as on the Mac.

**Observed — Linux hand test 28, 2026-08-06. The 2 GiB boundary itself, which the Mac could not reach.** The Mac could not get there because Blender's startup scene compresses to almost nothing. On the rented Linux machine two things were changed and nothing else: the picture was made out of high-frequency noise, so the encoder has nothing to compress away — a Noise Texture at scale 4000 and detail 15, looked up in window coordinates, feeding a Background node in the world — and the output was set to a lossless intra-frame codec, MKV with FFV1 and Lossless Output on, at 3840 by 2160 pixels with one Cycles sample. That is about 16 MB a frame, so the boundary is a hundred and twenty-odd frames away rather than tens of thousands.

Rendering 170 frames with Autosplit on, Blender 5.2.0 left **two files**. The first was closed at **2,024,660,891 bytes** and a second was started at that moment; the render carried on into it and finished normally, exit code 0, with the second file at 728,929,360 bytes. Two things fall out of that. The size at which the first file is closed is **below 2 GiB** (2,147,483,648 bytes) and above two thousand million bytes. And the second file's name carries a counter in its leading digits — `big__0010001-0170.mkv` against the first's `big__0000001-0170.mkv` — rather than the frame it starts at.

Then the same thing again with the render interrupted eight seconds after the second file appeared:

```
---- FOLDER AFTERWARDS ----
big__0000001-0300.mkv  2024660891 bytes
big__0010001-0300.mkv  32476274 bytes
```

and Blender opened both — the first with `frame_duration = 125`, the second with `frame_duration = 2`. So **the file that Autosplit closed at the boundary is whole and readable, and only the file being written when the stop arrived is short.** An interrupted Autosplit render does not lose everything: it loses only what went into the file that was open.

**One thing the two machines say differently, and it is about words rather than frames.** The Mac record above says this version prints no `Append frame` lines while encoding a video. On the rented Linux machine it prints such a line, worded `Video append frame 1`, `Video append frame 2` and so on, where Blender 4.5 prints `Append frame 1`. Both records stand. What is on the terminal is the only thing at issue; the file left behind is the same.

**Outside the interface.** The sources do not say. The Tip does not distinguish a render started by clicking from one started with `-a` on the command line. **Observed**, from hand tests 18 and 21: both were run on a Blender driven from a terminal with no window.

**Operating system.** The sources do not say. Container and codec compatibility is discussed on this page ("Some containers and codecs are not compatible with each other"), but nothing there is tied to an operating system. **Observed on two operating systems, and they agree** — macOS 26.5.2 (hand tests 18, 21 and 30, 2026-08-05) and Ubuntu 22.04.5 LTS on the rented Linux machine (Linux hand tests 17, 20 and 28, 2026-08-06). FFmpeg output with an MPEG-4 container and H.264 was available and worked on both. What a media player makes of a part-written file is settled for QuickTime Player on macOS only: the rented Linux machine has no media player installed at all and installing one was not permitted, so only Blender's own reader has opened these files there. **The 2 GiB Autosplit boundary itself (Linux hand test 28) is Linux only** — the Mac could not reach it at all, as the entry above says, so nothing about the split, the size the first file is closed at, the counter in the second file's name, or what an interruption after the split leaves is settled for macOS. *(Linux citation check, 2026-08-06: added by this check — Linux hand test 28 was listed among the tests the two machines agreed on, which reads as though the Mac had reached the boundary too.)* **Windows remains untested**, because no Windows machine has run it.

---

## 6. Is a file existing proof that the frame finished writing?

**Answer.** Not necessarily. Blender has a setting whose purpose is to create empty placeholder files while frames are rendering; the conclusion that such a file can occupy a finished frame's output path is an inferred step, set out below.

**Labels: inferred** (for the answer). **Documented** (for the Placeholders setting).

**Exact words.**

> "use_placeholder
>
> Create empty placeholder files while rendering frames (similar to Unix 'touch') (default False)"

- Source: the Python API documentation, `bpy.types.RenderSettings` — https://docs.blender.org/api/5.2/bpy.types.RenderSettings.html
- Date read: 2026-07-30

> "Create empty placeholder frames while rendering."

- Source: the manual, "Output" properties panel — https://docs.blender.org/manual/en/5.2/render/output/properties/output.html
- Date read: 2026-07-30

**What I take from a picture (documented).** In the screenshot of the Output panel on that same manual page, **Placeholders** appears as a checkbox in the right-hand column beside **Overwrite**, with Overwrite ticked and Placeholders unticked; the panel also shows File Extensions (ticked), Cache Result (unticked), File Format PNG, Color BW/RGB/RGBA with RGBA active, Color Depth 8/16 with 8 active, and Compression 15%.

- Picture: `_images/render_output_properties_output_panel.png`, captioned "Output panel.", on https://docs.blender.org/manual/en/5.2/render/output/properties/output.html
- Date read: 2026-07-30
- This confirms the two settings are separate, independently switchable checkboxes and that the default shown is Overwrite on, Placeholders off. It shows nothing about behaviour.

**The inferred step.** From "empty placeholder files … created while rendering", a file at a frame's output path may be an empty placeholder for a frame that is still being rendered, so its existence does not establish that the frame was finished and written. The reader supplies the step "placeholder files sit at the same paths that finished frames would occupy" — the sources do not say that in so many words, though the render-farm hint (quoted under question 4) only makes sense if they do. Label that step **inferred**.

**Related, as a data point (observed).** Blender's own Cycles add-on treats file presence as proof of a complete frame — `if not os.path.isfile(filepath): … "Frame '%s' not found, animation must be complete"` (`Blender Sources/cycles_5.2/operators.py`, lines 84–86, read 2026-07-30). That is an assumption made by the denoise operator, not a guarantee from Blender.

**Not covered.** The sources say nothing about a frame that was interrupted mid-write with Placeholders switched *off* — whether a half-written file is left, whether Blender writes to a temporary name and renames, or whether the file is removed. Nothing is said about file size, timestamps, or any completeness marker inside the file. Nothing states when a placeholder is created relative to the frame starting, nor whether it is replaced or deleted if the frame never renders.

**Observed — hand test 3, 2026-08-05.** The inferred step holds, and the timing this answer records as unstated is settled: a zero-byte placeholder does sit at exactly the path the finished frame will occupy, it is made immediately before that frame's own work begins, and the finished frame replaces it in place. Blender 5.2.0 was started from a terminal with no window, rendering the default cube scene in Cycles at 200 samples and 200 by 200 pixels to PNG, frames 1 to 3, Placeholders on, Overwrite off, with a script listing the output folder at each of Blender's own render events:

```
WATCH [before the render starts] folder holds: (nothing)
WATCH [render_pre, frame 1 about to render] folder holds: f_0001.png=0 bytes
WATCH [render_post, frame 1 finished rendering] folder holds: f_0001.png=34777 bytes
WATCH [render_write, frame 1 written] folder holds: f_0001.png=34777 bytes
WATCH [render_pre, frame 2 about to render] folder holds: f_0001.png=34777 bytes, f_0002.png=0 bytes
WATCH [render_post, frame 2 finished rendering] folder holds: f_0001.png=34777 bytes, f_0002.png=34777 bytes
WATCH [render_write, frame 2 written] folder holds: f_0001.png=34777 bytes, f_0002.png=34777 bytes
WATCH [render_pre, frame 3 about to render] folder holds: f_0001.png=34777 bytes, f_0002.png=34777 bytes, f_0003.png=0 bytes
WATCH [render_post, frame 3 finished rendering] folder holds: f_0001.png=34777 bytes, f_0002.png=34777 bytes, f_0003.png=34777 bytes
WATCH [render_complete] folder holds: f_0001.png=34777 bytes, f_0002.png=34777 bytes, f_0003.png=34777 bytes
```

The placeholders are made one at a time, not all at the start.

**Observed — hand test 4, 2026-08-05.** With Placeholders off, no file exists for the frame in progress at the moment it starts:

```
WATCH [before the render starts] folder holds: (nothing)
WATCH [render_pre, frame 1 about to render] folder holds: (nothing)
WATCH [render_post, frame 1 finished rendering] folder holds: f_0001.png=34777 bytes
WATCH [render_write, frame 1 written] folder holds: f_0001.png=34777 bytes
WATCH [render_pre, frame 2 about to render] folder holds: f_0001.png=34777 bytes
```

But a non-zero size is no proof either. The same test was run against a much larger file — Cycles at 1000 samples and 1000 by 1000 pixels, OpenEXR output, about 41 seconds a frame — with the folder watched twenty times a second. In Blender 5.2.0 the watcher recorded only:

```
  0.00s  (nothing)
 41.90s  f_0001.exr=477791 bytes
 82.95s  f_0001.exr=477791 bytes, f_0002.exr=477791 bytes
```

so it did not catch an intermediate size here. That is not evidence that the file appears whole: at the same watch rate, Blender 5.0.1 running the same test *was* caught with the file at 5448 bytes and then, 60 milliseconds later, at its finished 477472 bytes. So Blender does not write to a temporary name and rename — the file grows in place, fast. Neither existence nor a non-zero size proves the frame finished.

Run on the Creator's MacBook Pro, M1 Pro chip, macOS 26.5.2.

**Observed — hand tests 6, 7, 5 and 33, 2026-08-05.** Two more things bearing on this answer, recorded in full elsewhere in this document. A render stopped at any point leaves no file at all for the frame in flight, whether interrupted, terminated politely, or killed — so what this answer records as unstated, whether a placeholder survives a render that never reaches its frame, is settled: nothing is left (questions 2 and 3). And two Blenders sharing a folder with Placeholders on both rendered frame 1 in the first run, while in the re-run the placeholder caught the other copy and no frame was rendered twice — so a placeholder does not reliably reserve a frame, and it does not reliably fail to either (question 4).

**Observed — Linux hand tests 3 and 4, 2026-08-06.** Both halves came back the same on a rented Linux machine — Ubuntu 22.04.5 LTS, NVIDIA GeForce RTX 4090, driver 580.105.08, twelve cores, run over SSH. With Placeholders on, Blender 5.2.0 there gave the same sequence of listings: nothing in the folder, then `f_0001.png=0 bytes` as frame 1 begins, then the same name at 34805 bytes when it finishes, then `f_0002.png=0 bytes` as frame 2 begins. With Placeholders off, no file at all exists for the frame in progress.

The part-written half is worth setting out carefully, because the two machines caught different versions. Watching a folder twenty times a second through a slow 1000-by-1000 OpenEXR render, the Linux watcher recorded for this version:

```
  0.00s  (nothing)
 28.37s  f_0001.exr=477850 bytes
 57.62s  f_0001.exr=477850 bytes, f_0002.exr=0 bytes
 57.67s  f_0001.exr=477850 bytes, f_0002.exr=477850 bytes
```

**and 5.2 was caught with the second file at 0 bytes** fifty milliseconds before it reached its finished size. On the Mac it was 5.0 that was caught, at 5,448 bytes. Taking the two machines together: with Placeholders off, an ordinary frame file **can** be seen at a size that is not its finished size, on both operating systems, and which version a watcher happens to catch is luck rather than a difference between versions. Neither the existence of a file nor a non-zero size proves the frame finished — and a zero-byte file is not proof of a placeholder either.

**Observed — Linux hand test 27, 2026-08-06.** Twelve runs of two Blenders sharing a folder with Placeholders on, on the rented Linux machine, showed no frame rendered twice — unlike the Mac. That is recorded in full under question 4, and it does not change the answer here: what it settles is how reliably a placeholder reserves a frame, not whether a file's existence proves a frame finished.

**Outside the interface.** `use_placeholder` is a scene setting saved in the blend-file, so it applies to a command-line render as well; there is no command-line flag for it on the arguments page. The sources say nothing about the behaviour differing. **Observed**, from hand tests 3 and 4: Placeholders works on a Blender driven from a terminal with no window, and the `render_write` handler fires there too — every `WATCH [render_write …]` line above was printed by such a handler in background mode.

**Operating system.** The sources do not say. The API text compares the effect to the Unix `touch` command but does not limit the feature to any platform. **Observed on two operating systems, and they agree** — macOS 26.5.2 (hand tests 3 and 4, 2026-08-05) and Ubuntu 22.04.5 LTS on the rented Linux machine (Linux hand tests 3 and 4, 2026-08-06). Placeholders makes empty files on both, and an ordinary frame file can be caught part-written on both. **Windows remains untested**, because no Windows machine has run it.

---

## 7. Can Blender be given a set of frames that is not one plain run — single frames, or two separate stretches?

**Answer.** Yes, from the command line: `-f` takes a comma-separated list of frames, and a stretch can be written with `..`, so a list can mix single frames and stretches. The cited Frame Range panel offers only Start, End and Step, and the cited render operator takes only a start and an end. Those sources do not establish whether another interface route offers a list.

**Label: documented.**

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

- Source: the manual, "Command Line Arguments", Render Options — https://docs.blender.org/manual/en/5.2/advanced/command_line/arguments.html
- Date read: 2026-07-30

The scene-side settings, for contrast:

> "Frame Start, End
>
> Set the Start and End frames for Rendering Animations.
>
> Step
>
> Controls the number of frames to advance for each frame rendered, a value above 1 skips frames. This is also used when viewing rendered animations."

- Source: the manual, "Frame Range" properties panel — https://docs.blender.org/manual/en/5.2/render/output/properties/frame_range.html
- Date read: 2026-07-30

And the render operator:

> "bpy.ops.render.render(*, animation=False, write_still=False, use_viewport=False, use_sequencer_scene=False, layer='', scene='', frame_start=0, frame_end=0)"
>
> "frame_start (int) – Start Frame, Frame to start rendering animation at. If not specified, the scene start frame will be assumed. This should only be specified if doing an animation render (in [-inf, inf], optional)"
>
> "frame_end (int) – End Frame, Frame to end rendering animation at. If not specified, the scene end frame will be assumed. This should only be specified if doing an animation render (in [-inf, inf], optional)"

- Source: the Python API documentation, `bpy.ops.render` — https://docs.blender.org/api/5.2/bpy.ops.render.html
- Date read: 2026-07-30

**Not covered.** The `-f` page does not give a worked example of a mixed list, does not say whether whitespace anywhere is tolerated (it says "no spaces"), does not say what happens on a duplicate or out-of-order entry, does not state an upper limit on the length of the list, and does not say whether `+`/`-` relative frames may appear inside a list or a `..` stretch. It does not say whether `-f` may be given more than once. It does not say what order the frames come out in — see question 10. It does not say whether Step applies to a `-f` list.

**Observed — Linux hand tests 7 to 10, 19, 23, 24, 25 and 26, 2026-08-06.** Every one of the tests above was run again on a rented Linux machine — Ubuntu 22.04.5 LTS, NVIDIA GeForce RTX 4090, driver 580.105.08, twelve cores, run over SSH with Blender 5.2.0 and came back the same. `-f 5,1,9` gave 5, then 1, then 9. `-f 1..3,10..12` wrote all six files with no error. `-f 2,2,3` rendered frame 2 twice over the same file. The three malformed lists were refused with the same three messages, word for word, nothing rendered, exit code 0 every time. The same list with a window open wrote the same six files. `-s 1 -e 2 -a -s 20 -e 21 -a` wrote four files. The keyframes-only playblast with no window was refused with the same message, word for word. And the same list under Cycles gave the same order as under Workbench.

The one difference is in how the operating system refuses a list that is too long. Five thousand frames were accepted and all five thousand rendered, exit code 0, on both machines. Two hundred thousand never reached Blender on either. On the Mac the shell said `argument list too long` and returned `127`; on the rented Linux machine it said `Argument list too long` and returned `126`. **So the limit is the operating system's on both machines, and the two operating systems refuse it differently.**

**Observed — Linux hand test 34, 2026-08-06.** The keyframes-only playblast was run with a window, on the rented Linux machine's own display. Blender 5.2.0, given the startup scene with a keyframe on the cube's location at frame 3 and another at frame 5 and none anywhere else, the cube selected and active, and a frame range of 1 to 6, was asked for `bpy.ops.render.opengl(animation=True, render_keyed_only=True)` with the 3D Viewport as its context — the same operator the View menu's item runs. It returned `{'FINISHED'}` and wrote **six files for a six-frame range**, `p_0001.png` to `p_0006.png`, every one of them 44653 bytes. That agrees with the Creator's macOS result. Where it does not agree: the Creator reported the six files as identical, and compared byte for byte here **no two of the six are the same file** — all fifteen pairs came back different, in every version, while every file is the same size. Looked at, they are the same picture. Both records stand; the eye and a byte comparison are answering different questions.

**Observed — Linux hand tests 35 to 38, 2026-08-06.** Four things about `-f` that neither machine had tried before, run on the rented Linux machine only, so **the Mac is untested for all four**. In Blender 5.2.0, and identically in the other three versions:

- **`-f` may be given twice in one command.** `-f 1..3 -f 10..12` rendered both stretches — six files, exit code 0.
- **`-f` and `-a` may be mixed in one command.** `-s 1 -e 2 -a -f 20` carried out both — three files, frames 1, 2 and 20, exit code 0.
- **Step has no effect on a `-f` list.** With the scene's step set to 3, which read back as 3, `-f 1,2,3,4,5` wrote all five files.
- **Whitespace inside a list is accepted.** `-f "1, 2, 3"` rendered frames 1, 2 and 3 and wrote three files, exit code 0, even though the manual says "no spaces". That says what this machine took, not that the manual is wrong about what is supported.

**Outside the interface.** The list-and-stretch form is a command-line argument. The manual's command-line render page also warns about ordering of arguments — "Always position -f or -a as the last arguments." (https://docs.blender.org/manual/en/5.2/advanced/command_line/render.html, read 2026-07-30). The Frame Range panel picture (`_images/render_output_properties_frame_range_panel.png`, captioned "Frame Range panel.", on the Frame Range page) shows exactly three fields — Frame Start 1, End 250, Step 1 — plus a collapsed Time Stretching section, and nothing resembling a frame list. **Documented**, from that picture, read 2026-07-30. It does not establish that no other interface route exists.

**Observed — hand tests 9, 8, 10, 11, 20 and 24, 2026-08-05.** The mixed form does work, so the answer above holds in practice whatever its label. All six tests were run with Blender 5.2.0 started from a terminal, rendering the default cube scene in Workbench at 64 by 64 pixels to PNG into an empty folder each time.

*A list mixing single frames and stretches, `-f 1..3,10..12`.* Accepted, no error, all six files written:

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

Note what Blender calls each item: `Rendering single frame`, never `Rendering animation`. Each frame in the list is its own single-frame render — which also answers whether Step applies to a `-f` list: the list is not an animation run at all.

*Order and duplicates.* `-f 5,1,9` rendered 5, then 1, then 9 — the order written, not sorted. `-f 2,2,3` rendered frame 2 twice, over the same file, and left two files. So an out-of-order or repeated entry is neither rejected nor collapsed.

*Malformed lists.* Each was refused, nothing was rendered, and in every case Blender's exit code was `0`:

```
Error: incorrect comma use '-f / --render-frame 1,,3'.
Warning: negative range ignored '-f / --render-frame 3..1'.
Error: not a number '-f / --render-frame 1-3'.
```

*`-f` given more than once, and `-a` given more than once.* The arguments `-s 1 -e 2 -a -s 20 -e 21 -a` in one command produced two separate animation runs and four files:

```
00:00.403  render           | Rendering animation (frames 1..2)
00:00.560  render           | Saved: '…/m_0001.png'
00:00.565  render           | Saved: '…/m_0002.png'
00:00.565  render           | Rendering animation (frames 20..21)
00:00.571  render           | Saved: '…/m_0020.png'
00:00.576  render           | Saved: '…/m_0021.png'
```

*With Blender's window open.* The same `-f 1..3,10..12` command without `-b`, so Blender opened its window, wrote the same six files with the same names.

*How long a list may be.* A list of five thousand frames was accepted — all five thousand rendered, exit code `0`. A list of two hundred thousand never reached Blender: the shell refused it first, with `argument list too long`, and returned `127`. So the limit on list length is the operating system's argument length rather than anything in Blender.

*Whether whitespace is tolerated.* Not tested. The manual's "no spaces" was taken at its word.

Run on the Creator's MacBook Pro, M1 Pro chip, macOS 26.5.2.

**Observed — hand test 26, 2026-08-05.** The keyframes-only playblast, which is the one built-in way Blender picks a scattered set of frames, cannot be reached with no window. Calling `bpy.ops.render.opengl(animation=True, render_keyed_only=True)` in a Blender 5.2.0 started from a terminal with no window gave:

```
Error: Cannot use OpenGL render in background mode (no opengl context)
RAISED: RuntimeError Error: Cannot use OpenGL render in background mode (no opengl context)
FILES: []
```

No file was written. Same machine and date.

**Observed — hand test 32, run by the Creator, 2026-08-05.** With a window it does run, and it does not leave gaps on disk. In Blender 5.2.0 the Creator took the default cube scene, put a keyframe on the cube at frame 3 and another at frame 5 and none anywhere else, set the frame range to 1–6 and an image output path, and used the keyframes-only item in the 3D Viewport's View menu. Their own words: "Anyway, all versions create 6 files, which are identical."

So six files for a six-frame range, even though only two frames carry a key: a scattered set of frames rendered, a full set of frames written. What the six files were named, and how big they were, is not recorded.

The menu in Blender 5.2.0 reads:

> Render Viewport Preview
> Render Playblast
> Render Playblast on Keyframes

with the tooltip "Take a snapshot of the active viewport." and, underneath it, "Python: bpy.ops.render.opengl()". (Blender 4.5.12 labels the same three items "Viewport Render Image", "Viewport Render Animation" and "Viewport Render Keyframes"; 5.0.1 and 5.1.2 match 5.2.0.)

Run on the Creator's Mac — a MacBook Pro, M1 Pro chip, macOS 26.5.2 — so this settles macOS and leaves Windows and Linux open.

**Operating system.** The sources do not say. The command-line arguments page marks some flags as platform-specific (`-con` "Windows only", `--no-window-frame` "Linux only") but marks nothing on `-f`, `-s`, `-e`, `-j` or `-a`. **Observed on two operating systems, and they agree on everything the question asks** — macOS 26.5.2 (hand tests 8 to 11, 20, 24, 26, 27 and 32, 2026-08-05) and Ubuntu 22.04.5 LTS on the rented Linux machine (Linux hand tests 7 to 10, 19, 23, 24, 25, 26 and 34, 2026-08-06). What differs is not Blender's: how long a list the operating system will pass on, and what it says when it will not. The four further `-f` behaviours in Linux hand tests 35 to 38 are **Linux only**; the Mac was never asked. **Windows remains untested**, because no Windows machine has run it.

---

## 8. What does a frame range consist of?

**Answer.** Three numbers: Frame Start, End, and Step. Start and End are inclusive; Step is how far the render advances after each frame it renders.

**Label: documented.**

**Exact words.**

> "This panel defines how long an animation will last in terms of frames. Frames can be divided by the scene's Frame Rate to get the animation duration in terms of time. For example, a 250 frame animation at a frame rate of 30 will last 8.3 seconds.
>
> Frame Start, End
>
> Set the Start and End frames for Rendering Animations.
>
> Step
>
> Controls the number of frames to advance for each frame rendered, a value above 1 skips frames. This is also used when viewing rendered animations."

- Source: the manual, "Frame Range" properties panel — https://docs.blender.org/manual/en/5.2/render/output/properties/frame_range.html
- Date read: 2026-07-30

On inclusivity:

> "-a, --render-anim
>
> Render frames from start to end (inclusive)."

- Source: the manual, "Command Line Arguments" — https://docs.blender.org/manual/en/5.2/advanced/command_line/arguments.html
- Date read: 2026-07-30

The kinds, limits and defaults:

> "frame_start
>
> First frame of the playback/rendering range (in [0, 1048574], default 1)"
>
> "frame_end
>
> Final frame of the playback/rendering range (in [0, 1048574], default 250)"
>
> "frame_step
>
> Number of frames to skip forward while rendering/playing back each frame (in [0, 1048574], default 1)"

- Source: the Python API documentation, `bpy.types.Scene` — https://docs.blender.org/api/5.2/bpy.types.Scene.html
- Date read: 2026-07-30

Each is typed `int`. The same page also documents `frame_current` — "Current frame, to update animation data from Python frame_set() instead (in [-1048574, 1048574], default 1)" — and the read-only `frame_current_final`, "Current frame with subframe and time remapping applied".

**Corroborating (observed).** Cycles' own code walks the scene range inclusively:

```
            for frame in range(scene.frame_start, scene.frame_end + 1):
```

- Source file: `Blender Sources/cycles_5.2/operators.py`, line 79
- Date read: 2026-07-30

Note the step this does *not* take: this is the denoise operator's own loop, so it shows how Cycles' add-on reads the range, not how the renderer walks it.

**What a picture shows (documented).** The Frame Range panel screenshot shows three fields stacked — "Frame Start 1", "End 250", "Step 1" — and a collapsed "Time Stretching" section below them.

- Picture: `_images/render_output_properties_frame_range_panel.png`, captioned "Frame Range panel.", on https://docs.blender.org/manual/en/5.2/render/output/properties/frame_range.html
- Date read: 2026-07-30

**Also on the same panel, and part of the range's effect.**

> "Time Stretching
>
> Use to remap the length of an animation; making it run slower or faster. The Old and New settings may either be used as absolute values or as a ratio: For example, setting Old to a value of 2 and New to 1 will run the animation twice as fast."
>
> "Warning
>
> Using Time Stretching will not influence the Start or End frames set above, so make sure that your animation is not cut off or has extraneous still frames at the end."

- Source: same page, read 2026-07-30

**Not covered.** The API gives Step a lower bound of 0, and the sources do not say what a Step of 0 does. Nothing says what happens if End is below Start. Nothing states whether the preview range (`frame_preview_start` / `frame_preview_end`, both documented on `bpy.types.Scene` as "Alternative end/start frame for UI playback") ever affects a render. The panel's numbers are only the *scene's* range; a command-line `-f` list is not a range at all (question 7).

**Observed — hand tests 13, 14 and 12, 2026-08-05.** The first two of those are settled. All three tests were run with Blender 5.2.0 started from a terminal with no window, rendering the default cube scene in Workbench at 64 by 64 pixels to PNG into an empty folder.

*A step of 0.* The property accepted it and read back as `0`. Blender rendered and wrote frame 1 once, then announced frame 1 four more times without rendering or writing anything, and ended on its own with exit code `0`:

```
00:00.400  render           | Rendering animation (frames 1..5)
00:00.400  render           | Rendering frame 1
00:00.400  render           | Start rendering: Scene, ViewLayer
00:00.400  render           | Engine: Workbench
00:00.560  render           | Saved: '…/f_0001.png'
00:00.560  render           | Time: 00:00.15 (Saving: 00:00.06)
00:00.560  render           | Rendering frame 1
00:00.560  render           | Rendering frame 1
00:00.560  render           | Rendering frame 1
00:00.560  render           | Rendering frame 1
frame_step actually reads back as: 0
```

One file was left, `f_0001.png`, 3712 bytes. It did not hang.

*An end below the start.* Start 5, end 2 announced itself as `Rendering animation (frames 5..2)`, rendered nothing at all, and exited with code `0`. The folder was empty afterwards.

*A stepped run, for completeness.* Start 1, end 10, step 3 gave frames 1, 4, 7, 10. Start 3, end 12, step 4 gave 3, 7, 11 — the end frame 12 was not rendered.

Run on the Creator's MacBook Pro, M1 Pro chip, macOS 26.5.2.

**Observed — hand test 28, run by the Creator, 2026-08-05.** The panel accepts a step of 0 and shows it, with no complaint. In Blender 5.2.0 the Creator opened Output Properties, the Frame Range panel, typed `0` into the Step field and pressed Enter. Their own words: "Typing 0 in the Step field doesn't do anything — no error, warning or any visible behavior. Right-clicking the panel header does nothing. The panel itself is in the screenshot. I only took one because it's the same in all 4 versions."

The screenshot shows the panel holding, in order: **Frame Start** `1`, **End** `250`, **Step** `0`, and an open **Time Stretching** section with **Old** `100` and **New** `100`. Nothing else. So the Step field keeps the 0 rather than snapping to 1, and the panel's whole contents are those five numbers — the same three fields and the same collapsed section the manual's own picture shows, with Time Stretching opened up.

Read alongside the run above: the field takes 0, and a render made with it writes the first frame once.

Run on the Creator's Mac — a MacBook Pro, M1 Pro chip, macOS 26.5.2 — so this settles macOS and leaves Windows and Linux open.

**Observed — Linux hand tests 11, 12 and 13, 2026-08-06.** The same three runs on a rented Linux machine — Ubuntu 22.04.5 LTS, NVIDIA GeForce RTX 4090, driver 580.105.08, twelve cores, run over SSH gave the same frames in Blender 5.2.0. Start 1, end 10, step 3 gave files for 1, 4, 7 and 10. Start 3, end 12, step 4 gave 3, 7 and 11, and did not render frame 12. A step of 0 read back as `0`, wrote frame 1 once, left one file of 3639 bytes, and did not hang. An end below the start rendered nothing, printed `Rendering animation (frames 5..2)` and stopped, exit code 0.

One thing the Mac run did not record. The printed output announces each stepped frame once per step — `Rendering frame 4` three times over for a step of 3 — and the 3-to-12 run ends with a bare `Rendering frame 15`, past the end frame, for which nothing is rendered and nothing is written.

**Observed — Linux hand test 31, 2026-08-06.** The Frame Range panel was photographed on the rented Linux machine's own display with Blender's own `bpy.ops.screen.screenshot`. The panel holds **Frame Start** `1`, **End** `250`, **Step** `1`, and a collapsed **Time Stretching** section, and nothing else. With the step set to 0 the field then reads **`0`** — the panel keeps the 0 and does not snap back to 1. That agrees with what the Creator reported on the Mac.

Two limits on that run, said plainly. The step was set from Python rather than typed into the field, so what the field does with typed input rests on the Creator's macOS run alone. And the panel header was not right-clicked. Both need the pointer put on a control, and on that machine the display is shared with other work: a click aimed at a control was followed by asking Blender for that control's state, and the state had not changed, so no click could be shown to land. Nothing that could not be demonstrated has been written down.

**Observed — Linux hand test 31 again, 2026-08-06, and this version crashed.** The panel was then opened as a popup by Blender's own operator, which needs no pointer at all. In Blender 4.5.12, 5.0.1 and 5.1.2 `bpy.ops.wm.call_panel(name="RENDER_PT_frame_range")` returned `{'INTERFACE'}` and the popup photographed cleanly, showing Frame Start `1`, End `250`, Step `0`, and a Time Stretching section holding Old `100` and New `100`, and nothing else. **In Blender 5.2.0 on the rented Linux machine the same call brought Blender down**, twice out of two attempts, printing only:

```
Writing: /tmp/start-5.2.crash.txt
timeout: the monitored command dumped core
```

The neighbouring panel was fine in the same version: `bpy.ops.wm.call_panel(name="RENDER_PT_time_stretching")` returned `{'INTERFACE'}` and photographed, showing Old `100` and New `100`. Both panel identifiers were read out of `bpy.types` rather than guessed, and both are there in this version. What the panel holds in 5.2 is therefore taken from the Properties editor's own picture, recorded above, and not from a popup. This crash was not seen on the Creator's Mac, where the panel was never opened this way.

**Observed — Linux hand tests 37 and 39, 2026-08-06.** Two things about the range that neither machine had tried, run on the rented Linux machine only. **Step has no effect on a `-f` list**: with the scene's step set to 3, `-f 1,2,3,4,5` wrote all five files. And **the alternative playback range does not change what a real render writes**: with the scene range 1 to 8, an alternative playback range of 4 to 5 and that range switched on, a plain animation render announced `Rendering animation (frames 1..8)` and wrote all eight frames.

**Outside the interface.** The same three values are settable from the command line: `-s, --frame-start <frame>`, `-e, --frame-end <frame>`, and `-j, --frame-jump <frames>` — "Set number of frames to step forward after each rendered frame." (https://docs.blender.org/manual/en/5.2/advanced/command_line/arguments.html, read 2026-07-30). Note the arguments page also lists `-s`, `-e`, `-j` and `-f` again under "Animation Playback Options", where they mean playback rather than render settings and apply when `-a <options> <file(s)>` is used as a player — the page says that playback mode is "ignored if -b is set". Beyond that the sources note no difference.

**Operating system.** The sources do not say. None of these settings or flags carries a platform note. **Observed on two operating systems, and they agree** — macOS 26.5.2 (hand tests 12, 13, 14 and 28, 2026-08-05) and Ubuntu 22.04.5 LTS on the rented Linux machine (Linux hand tests 11, 12, 13 and 31, 2026-08-06). The same three stepped runs give the same frames on both, and the panel shows a step of 0 on both. **Linux hand tests 37 and 39 — Step against a `-f` list, and the alternative playback range — are Linux only**; the Mac was never asked. *(Linux citation check, 2026-08-06: added by this check.)* **Windows remains untested**, because no Windows machine has run it.

---

## 9. What does a frame write when several view layers are active on it?

**Ambiguity, flagged.** "What does a frame write" can mean (a) what ends up in the file or files on disk for that frame, or (b) what the render result holds in memory. I answer (a), what is written to disk, because the sheet is about frames and files. Under reading (b) the answer would rest on the Image Editor's Render Result having a View Layer selector, quoted below.

**Answer.** The sources do not state what a single-image file format writes when several view layers are enabled. What they do state: each enabled view layer is rendered and is available separately afterwards, and the multi-layer OpenEXR route stores view layers and their passes together in one file.

**Labels: documented** (for enabled view layers, their separate availability, Multi-Layer EXR, and Render Single Layer). **Not in the sources** (for what a single-image file writes when several view layers are enabled).

**Exact words — every enabled view layer is rendered, and each is separately available.**

> "use
>
> Enable or disable rendering of this View Layer (default True)"

- Source: the Python API documentation, `bpy.types.ViewLayer` — https://docs.blender.org/api/5.2/bpy.types.ViewLayer.html
- Date read: 2026-07-30

> "When a scene is rendered, each View Layer is available in the Compositor using they Render Layers Node. This allows you to process each layer independently, combining or adjusting them to create the final output."

- Source: the manual, "View Layers ‣ Introduction" — https://docs.blender.org/manual/en/5.2/render/layers/introduction.html
- Date read: 2026-07-30

> "Render Result: displays renders. When this item is selected, the Slot, View Layer, and Render Pass selectors become available (see below)."
>
> "View Layer
>
> The View Layer to display."

- Source: the manual, "Image Editor ‣ Introduction" — https://docs.blender.org/manual/en/5.2/editors/image/introduction.html
- Date read: 2026-07-30

**Exact words — the one file format the sources tie to multiple view layers.**

> "Saving – Cache Result
>
> Saves the rendered view layers and their passes to a multi-layer OpenEXR image."

> "Multi-Layer EXR:
>
> Saves all inputs together in a single multi-layer OpenEXR file."

- Source: the manual, "Output" properties panel — https://docs.blender.org/manual/en/5.2/render/output/properties/output.html
- Date read: 2026-07-30

> "media_type
>
> The type of media to save (default 'IMAGE')
>
> Type: Literal['IMAGE', 'MULTI_LAYER_IMAGE', 'VIDEO']"

- Source: the Python API documentation, `bpy.types.ImageFormatSettings` — https://docs.blender.org/api/5.2/bpy.types.ImageFormatSettings.html
- Date read: 2026-07-30

**Exact words — the setting that reduces it to one layer.**

> "Render Single Layer
>
> Only render the active view layer.
>
> Note
>
> This option is ignored when rendering from the command line."

- Source: the manual, "View Layer" properties panel — https://docs.blender.org/manual/en/5.2/render/layers/view_layer.html
- Date read: 2026-07-30

> "use_single_layer
>
> Only render the active layer. Only affects rendering from the interface, ignored for rendering from command line. (default False)"

- Source: the Python API documentation, `bpy.types.RenderSettings` — https://docs.blender.org/api/5.2/bpy.types.RenderSettings.html
- Date read: 2026-07-30

**What is not in the sources, and where I looked.** No page states how many files a frame with several view layers produces under the Image media type, nor which view layer's pixels land in a single-image file, nor whether view-layer names appear in the file names. I searched the whole 5.2 manual for `multi-layer`, `multilayer`, and `view layer` cross-referenced with `saved`, `written`, `output`, `render result`, `combin`, `composit`; I read `render/layers/introduction.txt`, `render/layers/view_layer.txt`, `render/output/properties/output.txt`, `render/output/animation.txt`, `render/output/introduction.txt`, `files/media/image_formats.txt`, `compositing/types/input/scene/render_layers.txt` and `compositing/types/output/file_output.txt`. The File Output node page speaks of "inputs" and their names, not view layers ("Each input corresponds to an image that will be written to disk"), so it does not settle the Output-panel case. The Output panel's own wording is also about "inputs" ("Image: Saves each input as a separate image file"), and no page in the 5.2 manual defines what the *scene's* inputs are or ties them to view layers. The 5.2 release notes say only "Saving render output can now be disabled." under Render Output (https://developer.blender.org/docs/release_notes/5.2/rendering/) and nothing about view layers and file writing. Cycles' own files contain nothing on the subject — searching them for `view_layer` in a file-writing context turns up only pass registration and the denoise operator's use of `context.view_layer`. All read 2026-07-30.

**Not covered.** Which pixels a JPEG or PNG receives when two view layers are enabled. Whether the number of files per frame changes. Whether view-layer names enter the file name. What happens when compositing is enabled and the Compositor combines the layers before saving. What a video output does with several view layers.

**Observed — hand tests 15, 16 and 17, 2026-08-05.** This settles the question the sources could not answer. Blender 5.2.0 was started from a terminal with no window. A script moved the default cube into its own collection and added a second view layer, `Second`, that leaves that collection out — so the two view layers plainly show different things, one with the cube and one without. Both had `use` set to True. One frame was rendered four ways, and the written files were loaded back and compared pixel by pixel:

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

Three answers, all of them things this document records as not in the sources. **One image file is written per frame** with these two view layers both enabled — never one per layer, so the number of files does not change. *(Citation check, 2026-08-05: this read "whatever the number of view layers"; the test used exactly two, so it has been narrowed to two.)* **No view-layer name enters the file name**: the file is `A_both_0001.png`, named from the output path and the frame number alone. And **the pixels are the first view layer's alone** — identical to the same frame rendered with only that layer enabled, different from the same frame rendered with only the second layer enabled.

The fourth answer is about the "sharp difference" recorded below. **Render Single Layer changed nothing here.** The file written with it on is pixel-for-pixel the file written with it off. So while the switch is indeed ignored outside the window, the thing that lands on disk is one layer's image either way — the difference does not show up in the written file.

**Observed — hand test 16, 2026-08-05.** The multi-layer OpenEXR route does keep both view layers, in one file. In Blender 5.2.0, setting `media_type` to `'MULTI_LAYER_IMAGE'` narrows the file-format list to `('OPEN_EXR_MULTILAYER')` alone; with `media_type` left at `'IMAGE'` the version's full file-format list is:

```
['AVIF', 'JPEG', 'OPEN_EXR', 'PNG', 'WEBP', 'BMP', 'CINEON', 'DPX', 'IRIS', 'JPEG2000',
 'HDR', 'TARGA', 'TARGA_RAW', 'TIFF', 'OPEN_EXR_MULTILAYER', 'FFMPEG']
```

(Blender 4.5.12 has no `media_type` setting at all, and no `AVIF` in its list.) The render wrote one file, `vl_0001.exr`, 17954 bytes, and the names found inside its header were:

```
ViewLayer.Combined.A, ViewLayer.Combined.B, ViewLayer.Combined.G, ViewLayer.Combined.R,
Second.Combined.A, Second.Combined.B, Second.Combined.G, Second.Combined.R
```

Both view layers, named, inside the one file.

Run on the Creator's MacBook Pro, M1 Pro chip, macOS 26.5.2. What was *not* tested is the same render with the compositor switched on, which stays open.

**Observed — Linux hand tests 14, 15 and 16, 2026-08-06.** The same three runs on a rented Linux machine — Ubuntu 22.04.5 LTS, NVIDIA GeForce RTX 4090, driver 580.105.08, twelve cores, run over SSH gave the same four comparisons in Blender 5.2.0:

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

**Observed — Linux hand test 29, 2026-08-06. The compositor switched on, which nobody had tested on either machine.** Blender 5.2.0 on the rented Linux machine, with the same two view layers both enabled, rendered one frame five ways: with the compositor off and both layers on; with the compositor off and only the first; with the compositor off and only the second; with the compositor on and its Render Layers node reading the first view layer; and with that node moved to the second. Getting the compositor on took a version-specific route: this version has no `Scene.node_tree`; the compositor is a node group hung on `Scene.compositing_node_group`, which starts out empty, so a tree was built by hand — a Render Layers node feeding a **Group Output** node. The five files compared:

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

**Outside the interface.** This is one place the sources state a real difference, and it is a sharp one: Render Single Layer "is ignored when rendering from the command line" / "Only affects rendering from the interface". So a blend-file that renders one layer when you click will render every enabled layer when driven from the command line, unless the individual layers' `use` flags are off. **Observed**, from hand tests 15 and 17: whatever is rendered, the file written from outside the window holds the first view layer's image, and turning the switch on made no difference to it. What a render started by clicking writes is still untested.

**Operating system.** The sources do not say. Nothing on these pages is marked as platform-specific. **Observed on two operating systems, and they agree** — macOS 26.5.2 (hand tests 15, 16 and 17, 2026-08-05) and Ubuntu 22.04.5 LTS on the rented Linux machine (Linux hand tests 14, 15 and 16, 2026-08-06). The compositor-on case in Linux hand test 29 is **Linux only**; the Mac was never asked. **Windows remains untested**, because no Windows machine has run it.

---

## 10. In what order does a render work through the frames it was given — a plain run, a run that skips frames at a fixed interval, and a list of separate frames — and what does an interruption leave behind in each case?

**Answer.** For a plain run, the sources say the render goes from start to end inclusive. For a run with Step above 1, the sources say the render advances Step frames after each frame rendered, so a forward walk through Start, Start+Step, and so on is inferred from those words. For a list of separate frames given with `-f`, the sources do not state an order at all. On interruption, the sources say that already-rendered frames remain in the output directory when a separate-image animation is restarted after a problem; they do not tie that statement to each of these three run forms or describe the result in each case.

**Labels: documented** (plain run and the general separate-image recovery advice). **Inferred** (Step order). **Not in the sources** (list order and any per-case account of what an interruption leaves).

**Exact words — plain run.**

> "-a, --render-anim
>
> Render frames from start to end (inclusive)."

- Source: the manual, "Command Line Arguments" — https://docs.blender.org/manual/en/5.2/advanced/command_line/arguments.html
- Date read: 2026-07-30

**Exact words — Step.**

> "Step
>
> Controls the number of frames to advance for each frame rendered, a value above 1 skips frames."

- Source: the manual, "Frame Range" properties panel — https://docs.blender.org/manual/en/5.2/render/output/properties/frame_range.html
- Date read: 2026-07-30

> "-j, --frame-jump <frames>
>
> Set number of frames to step forward after each rendered frame."

- Source: the manual, "Command Line Arguments" — https://docs.blender.org/manual/en/5.2/advanced/command_line/arguments.html
- Date read: 2026-07-30

> "frame_step
>
> Number of frames to skip forward while rendering/playing back each frame (in [0, 1048574], default 1)"

- Source: the Python API documentation, `bpy.types.Scene` — https://docs.blender.org/api/5.2/bpy.types.Scene.html
- Date read: 2026-07-30

**The inferred step (Step run).** "Advance" and "step forward" and "from start to end" together give a forward walk beginning at Start and adding Step each time. The reader has to supply that the first frame rendered is Start itself, and that the walk stops at or before End. Neither is written anywhere. So: inferred.

**Exact words — interruption, general.**

> "since the frames you have already rendered will still be in the output directory."

- Source: the manual, "Rendering Animations", Hints — https://docs.blender.org/manual/en/5.2/render/output/animation.html
- Date read: 2026-07-30

And the documented consequence of a Step run, which is the closest the sources come to describing what a stepped run leaves behind:

> "It can be useful to render a subset of the animated sequence, since only part of an animation may have an error.
>
> Using an image format for output, you can use the Frame Step option to render every N'th frame. Then disable Overwrite and re-render with Frame Step set to 1."

- Source: same page, "Animation Preview" — read 2026-07-30

That passage gives a workflow based on rendering every Nth frame and then running again with Step set to 1 and Overwrite off. Reading it as a sparse set of files that the later run fills in around is inference; it does not describe an interruption.

**Where I looked, for the parts that are not in the sources.** I searched the whole 5.2 manual for `in order`, `sequentially`, `one at a time`, `ascending`, `chronological` combined with `render`/`frame`, and found nothing about frame ordering; the only hits were about Freestyle chaining and particle instancing. I searched the API documentation for `start to end` and found nothing on `bpy.ops.render` or `bpy.types.RenderSettings`. The `bpy.ops.render.render` entry is marked "Undocumented, consider contributing." (https://docs.blender.org/api/5.2/bpy.ops.render.html, read 2026-07-30). Cycles' own files hold a `render` entry point and a `render_frame_finish` entry point (`Blender Sources/cycles_5.2/engine.py`, lines 89 and 95) and no loop over frames anywhere except in the denoise operator. *(Citation check, 2026-08-05: this read "Cycles' own files contain no frame loop of any kind — its `render` and `render_frame_finish` entry points … are called per frame from outside Cycles, and Cycles' only frame loop is the denoise operator's." Two things were wrong. "Called per frame from outside Cycles" is about what happens when a render runs, which the file cannot show — that is an inference, not a reading. And "no frame loop of any kind" was contradicted by the same sentence's own mention of the denoise loop. Narrowed to what the files hold.)* The 5.2 release notes say nothing about frame ordering.

**Not covered.** Whether the list form of `-f` renders in the order written, in sorted order, or with duplicates collapsed. Whether Step applies to a `-f` list. Whether an interruption is guaranteed to fall on a frame boundary. What the frame in flight leaves on disk in any of the three cases (question 6). Whether the answers change when Overwrite is off and frames are being skipped.

**Observed — hand tests 1, 12, 8, 10, 25, 6 and 7, 2026-08-05.** Every part of this question that the sources left open is now settled, for a render driven from a terminal with no window. Run with Blender 5.2.0 on the default cube scene to PNG.

*A plain run.* Frames 1 to 5 in ascending order, one at a time, each announced before it and saved after it. Quoted in full under question 3.

*A stepped run.* Start 1, end 10, step 3 gave files for frames 1, 4, 7, 10 — so the first frame rendered *is* the start frame, which this answer records as something a reader has to supply. Start 3, end 12, step 4 gave 3, 7, 11, and did not render the end frame 12 — so the walk stops at or below the end and never overshoots it. Quoted in full under question 8.

*A list of separate frames.* `-f 5,1,9` rendered them in the order written — 5, then 1, then 9 — not sorted. `-f 2,2,3` rendered frame 2 twice, so duplicates are not collapsed. Each item in a list is announced as `Rendering single frame`, so Step does not apply to a `-f` list at all. Quoted in full under question 7.

*Whether the order differs by engine.* No. `-f 5,1,9` gave the same order with Workbench, with Cycles, and with EEVEE.

*What an interruption leaves, in each case.* The same in all of them: whole frames only. An interrupt, a polite terminate and an uncatchable kill all left `f_0001.png` and `f_0002.png` at 92440 bytes and no third file — so an interruption does fall on a frame boundary as far as what reaches disk is concerned, and the frame in flight leaves nothing. Quoted in full under questions 2 and 3. For a video output the answer is different and is recorded under question 5.

*With Overwrite off and frames being skipped.* Skipping does not change the order: the run still walks the range forward, announcing and skipping each frame it passes. Quoted in full under question 4.

Run on the Creator's MacBook Pro, M1 Pro chip, macOS 26.5.2.

**Observed — Linux hand tests 1, 7, 9, 11, 5 and 24, 2026-08-06.** Every part of this came back the same on a rented Linux machine — Ubuntu 22.04.5 LTS, NVIDIA GeForce RTX 4090, driver 580.105.08, twelve cores, run over SSH with Blender 5.2.0. A plain run gave 1, 2, 3, 4, 5 in that order. A stepped run started on the start frame and moved forward by the step, and did not reach back for an end frame it had overshot. `-f 5,1,9` rendered in the order written, and `-f 2,2,3` rendered frame 2 twice. An interruption left whole frames only, and nothing at all for the frame in flight, whichever way the render was stopped. The same list under Cycles gave the same order as under Workbench — and where the Mac ran that engine test in 5.1 and 5.2 only, it was run in all four versions here.

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

**Observed — Linux hand test 33, 2026-08-06. The order a render started the way the window's own Render Animation item starts it.** *(Linux citation check, 2026-08-06: this heading read "The order a render started by clicking uses"; nothing was clicked — the render was started by calling the operator that menu item calls, which is what the entry itself says — so it has been narrowed to the entry's own wording.)* This was untested on both machines. On the rented Linux machine, with a window open on its display, Blender 5.2.0 was given `bpy.ops.render.render('INVOKE_DEFAULT', animation=True)` — the call the Render menu's Render Animation item makes — for a six-frame Workbench range. It returned `{'RUNNING_MODAL'}`, a separate **Blender Render** window opened, and afterwards the six files were sorted by the time each was written:

```
the order the files were written in: ['o_0001.png', 'o_0002.png', 'o_0003.png', 'o_0004.png', 'o_0005.png', 'o_0006.png']
```

Ascending, one at a time, the same as a render driven from a terminal. Identical in all four versions.

**Outside the interface.** All three shapes are expressible from the command line (`-a`, `-j`, `-f` with a list). The one warning the sources give about driving it from outside is about argument order rather than frame order: "Arguments are executed in the order they are given!" and "Always position -f or -a as the last arguments." (https://docs.blender.org/manual/en/5.2/advanced/command_line/render.html, read 2026-07-30). Whether the frame order itself differs inside and outside the interface — the sources do not say. **Observed** for the outside case only: every hand test above drove Blender from a terminal with no window, and `-j` and the scene's own Step setting gave the same frames. **Observed**, from Linux hand test 33: a render started the way the window's own Render Animation item starts it wrote its six frames in ascending order, one at a time, the same as a render driven from a terminal. *(Linux citation check, 2026-08-06: this sentence read "The order a render started by clicking uses is still untested", which Linux hand test 33 directly above had already settled. Corrected by this check.)* **The two-card runs below say nothing about the inside case**: that machine has no display server at all, so every run there was driven from outside with no window, and no window test could be repeated with two cards. *(Two-card citation check, 2026-08-06: added by this check — the line did not mention the two-card runs at all.)*

**Observed on the two-card machine — 2026-08-06. Whether the frame order or the output changes when both graphics cards are ticked rather than one.** *(Two-card citation check, 2026-08-06: this read "when two graphics cards do the work". The record below shows the preferences that were saved, not that both cards were used by the render — no device list or card load was captured here — so the heading now says what was set rather than what happened.)*

Run over SSH on a **second rented Linux machine — Ubuntu 22.04.5 LTS, two NVIDIA GeForce RTX 4090s of 24 GB each on driver 580.95.05, an AMD EPYC 7B13 with sixty-one cores, 98 GB of memory, no display server at all** — using Blender at `/opt/blender/blender-5.2.0-linux-x64/blender`, `Blender 5.2.0 LTS (hash fbe6228777e7 built 2026-07-14 01:32:04)`, with `BLENDER_USER_RESOURCES` pointed at `/root/smoozy/two cards/res-5.2`, a folder of its own that was empty at the start. **Nothing else was running on the machine.** Label: **observed**. This is a third machine; nothing here is merged with the Creator's Mac or with the one-card rented Linux machine. The full list is `Audits/Blender Research/Frames/Hand tests — two cards and no screen.md`.

**The set-up.** A four-frame Cycles animation at 640 by 360, 128 samples, adaptive sampling off, denoising off, seed 0 and animated seed off, rendered with `--render-anim` into an empty folder. The preferences were saved with the technology `OPTIX` and both cards ticked, then again with only the card at `0000:00:07` ticked.

**The order is ascending, one at a time, with two cards as with one:**

```
00:04.690  render           | Saved: '/root/smoozy/two cards/work/ord_5.2/f_0001.png'
00:06.124  render           | Saved: '/root/smoozy/two cards/work/ord_5.2/f_0002.png'
00:07.456  render           | Saved: '/root/smoozy/two cards/work/ord_5.2/f_0003.png'
00:08.733  render           | Saved: '/root/smoozy/two cards/work/ord_5.2/f_0004.png'
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
- two cards against two cards, two separate runs: **14**
- one card against one card, two separate runs: **10**

**and the largest difference in every one of those comparisons was `0.003921568393707275` — one step in 255.** Two runs on the same cards differ by as much as a two-card run differs from a one-card run, so nothing here shows the number of cards changing the picture.

**Observed on the four-card machine — 2026-08-06. Whether the frame order or the picture changes when four graphics cards are ticked rather than one; and what Blender stamps into a picture.**

Run over SSH on a **third rented Linux machine — Ubuntu 22.04.5 LTS, four NVIDIA GeForce RTX 4060 Ti of 16 GB each on driver 580.95.05, an AMD EPYC 7R43 with sixty-one cores, 197 GB of memory, no display server** — using Blender at `/opt/blender/blender-5.2.0-linux-x64/blender`, `Blender 5.2.0 LTS (hash fbe6228777e7 built 2026-07-14 01:32:04)`, with `BLENDER_USER_RESOURCES` pointed at `/root/smoozy/four cards/res-5.2`, a folder of its own emptied and remade before use. **Nothing else was running on the machine.** Label: **observed**. This is a **fourth** machine; nothing here is merged with the Creator's Mac, the one-card rented Linux machine or the two-card rented Linux machine. The full list, with every command and every script written out, is `Audits/Blender Research/Frames/Hand tests — four cards.md`.

**Of the three ways this machine differs from the two-card one, only one bears on what is below.** Four cards rather than two is the point of the test. The smaller card bears on the seconds in the output quoted here and on nothing else, and no timing from this machine is set against a two-card timing. The larger system memory bears on nothing here.

**The set-up.** The same four-frame Cycles animation as the two-card run: 640 by 360, 128 samples, adaptive sampling off, denoising off, seed 0 and animated seed off, rendered with `--render-anim` into an empty folder. Four passes, each into its own folder — all four cards ticked, one card, all four again, one again — so that every comparison has a control. The script that saves which cards are ticked, and the script that counts differing pixel values, are both written out in full in the list.

**The order is ascending, one at a time, with four cards as with one:**

```
00:00.448  render           | Rendering frame 1
00:08.210  render           | Saved: 'ord_5.2_four_a/f_0001.png'
00:08.210  render           | Rendering frame 2
00:09.950  render           | Saved: 'ord_5.2_four_a/f_0002.png'
00:09.950  render           | Rendering frame 3
00:11.743  render           | Saved: 'ord_5.2_four_a/f_0003.png'
00:11.743  render           | Rendering frame 4
00:13.375  render           | Saved: 'ord_5.2_four_a/f_0004.png'
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
out/stamp/s_a_0001.png out/stamp/s_b_0001.png differ: byte 297, line 3
f09ea9524d0f998958c2888ee5247f1c  out/stamp/s_a_0001.png
d22205d497820dd529ce1d60dc8dab07  out/stamp/s_b_0001.png
    tEXt b'cycles.ViewLayer.samples\x00400'
    tEXt b'cycles.ViewLayer.total_time\x0000:00.19'
    tEXt b'cycles.ViewLayer.render_time\x0000:00.19'
    tEXt b'cycles.ViewLayer.synchronization_time\x0000:00.00'
    tEXt b'cycles.ViewLayer.samples\x00400'
    tEXt b'cycles.ViewLayer.total_time\x0000:00.22'
    tEXt b'cycles.ViewLayer.render_time\x0000:00.21'
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


**With that settled, the pictures of the four passes were compared**, value by value, with a control. Frame 1, of 921,600 values: **9 differed between four cards and one card, and 9 between two separate runs on four cards**, the largest difference in both being one step in 255. So the four-card comparison differed in the same number of values as the control, every difference anywhere being that same smallest step. **Nothing here shows the number of cards changing the picture, and nothing here rules it out either.** *(Four-card citation check, 2026-08-06: this check ran the same four passes again on the same machine. The order was ascending both ways in all four versions, no two files matched in any of the twelve comparisons per version, and the largest pixel difference was again one step in 255 everywhere. The counts came out **10 between four cards and one card and 10 between two runs on four cards**, against **9 and 9** above. Both runs stand. What this shows is that the ratio between the two counts is noise from one run to the next, so the sentence before last — how the four-card figure compares with the control — is about this run alone and not about the version. The last sentence is the one that survives both runs.)*

**Driven from outside — this block.** Every run above was made from outside Blender's window, and on this machine there is no other way: with no display server there is nothing for a window to open on. No window test could be repeated with four cards.

**Operating system — this block.** Linux, on the machine named at the top of it. The stamp result was got on this Linux machine; a checker's separate run on the Creator's Mac in Blender 4.5 also found the file matching with the stamp settings off, and the two are not merged here. Windows is untested.


**Operating system.** The sources do not say. **Observed on two operating systems, and they agree** — macOS 26.5.2 (hand tests 1, 8, 10, 12, 6 and 7, 2026-08-05) and Ubuntu 22.04.5 LTS on the rented Linux machine (Linux hand tests 1, 7, 9, 11, 5 and 24, 2026-08-06). **Observed on both makes of graphics card the project has**, from Linux hand test 30: Apple's own on the Mac and an NVIDIA GeForce RTX 4090 here, with the same result. **The order a render started the way the window starts it (Linux hand test 33) is Linux only**; the Mac was never asked. *(Linux citation check, 2026-08-06: added by this check — the line named only the tests run on both machines.)* **Observed with two graphics cards in one machine, and it changes nothing** — see the block above: ascending order both ways, and the picture differs no more between two cards and one than between two runs on the same cards. **Windows remains untested**, because no Windows machine has run it, and no card of any third make can be reached. **Observed with four graphics cards in one machine as well, on a fourth machine, and it changes nothing** — the order is ascending both ways, and the picture differs no more between four cards and one than between two runs on four cards. That run also settled what Blender writes into a PNG and when two files can match, which is what makes the picture comparison the reliable one. *(Added 2026-08-06 by the hand tests on the four-card machine.)*

---

## 11. Can a render be told to stop once the frame it is working on is finished, rather than the moment it is asked?

**Answer.** Not in the sources. Blender 5.2's documentation describes ways to request an abort, but does not state when the request takes effect or offer an option to defer it until the current frame ends.

**Labels: not in the sources** (a deferred-stop option). **Documented** (ways to request an abort).

**What the sources do describe — asking to abort.**

> "The task can be aborted by clicking the cancel button ()."

- Source: the manual, "Status Bar", Running Task — https://docs.blender.org/manual/en/5.2/interface/window_system/status_bar.html
- Date read: 2026-07-30

> "As with a normal render, you can abort it with Esc."

- Source: the manual, "Viewport Render" — https://docs.blender.org/manual/en/5.2/editors/3dview/viewport_render.html
- Date read: 2026-07-30

> "test_break()
>
> Test if the render operation should been canceled, this is a fast call that should be used regularly for responsiveness"

- Source: the Python API documentation, `bpy.types.RenderEngine` — https://docs.blender.org/api/5.2/bpy.types.RenderEngine.html
- Date read: 2026-07-30

That last one says a render engine should test regularly whether the render should be cancelled. It does not say when a cancellation takes effect or describe a deferred stop. The API also exposes the moments around a stop as handlers: `render_cancel` — "on canceling a render job. Accepts one argument: the scene data-block being rendered" — alongside `render_pre`, `render_post`, `render_complete` and `render_write` — "on writing a render frame (directly after the frame is written)" (https://docs.blender.org/api/5.2/bpy.app.handlers.html, read 2026-07-30). A handler is a place to be told what happened; the sources do not describe any of them as a way to ask for a stop.

**Where I looked.** Across the whole 5.2 manual I searched for `after current frame`, `finish the current frame`, `stop after`, `pause render`, `Pause`, `defer`, `quit after`, `abort`, `graceful`, `stop rendering`, `interrupt`, `cancel`. The only render-related hits were the immediate-abort sentences quoted above, plus playback pausing in the Animation Player and the Timeline (unrelated to rendering). Across the API documentation I searched for `stop_render`, `cancel_render`, `is_break`, `interrupt`; the only hits were `test_break` on `RenderEngine`, `bpy.ops.render.view_cancel` ("Cancel showing the render view" — about the render window, not the render), and an unrelated `Event` property. I read the full operator list on `bpy.ops.render` (https://docs.blender.org/api/5.2/bpy.ops.render.html): the operators there are `clear_texture_cache`, several preset-add operators, `generate_texture_cache`, `opengl`, `play_rendered_anim`, `preset_add`, `render`, `shutter_curve_preset`, `swap_dimensions`, `view_cancel`, `view_show` — no stop or pause operator among them. `bpy.types.RenderSettings` has no property of that kind either; its `use_*` list runs from `use_auto_generate_texture_cache` to `use_texture_cache` with nothing about stopping. The 5.2 release notes mention cancelling only for the Render Audio operator — "The 'Render Audio' operator now shows a realtime progress bar that can be cancelled (previously Blender would freeze until the operation finished)." (https://developer.blender.org/docs/release_notes/5.2/sequencer/) — which is not this. Cycles' own files contain no such setting: `properties.py`, `ui.py`, `operators.py`, `engine.py` and `__init__.py` searched for `stop`, `pause`, `cancel`, `resume`, `checkpoint`; the only render-loop entry points are `render(engine, depsgraph)` and `render_frame_finish(engine)` in `Blender Sources/cycles_5.2/engine.py` lines 89 and 95, neither of which takes or exposes a stop-after-frame option. All read 2026-07-30.

**Not covered.** This does not settle whether the abort in practice lands mid-frame or at a frame boundary — the sources describe the request, not where it takes effect. It does not settle whether a Python handler such as `render_write` could be used to arrange a stop after the current frame; the sources neither describe nor rule that out. It does not cover render engines other than the ones shipped.

**Observed — hand tests 6 and 7, 2026-08-05.** A stop does not wait for the current frame, and the frame in progress is thrown away. Blender 5.2.0 was started from a terminal with no window, rendering the default cube scene in Cycles at 1000 samples and 400 by 400 pixels to PNG, frames 1 to 5, at about six and a half seconds a frame. Fifteen seconds in — during frame 3 — it was sent an interrupt, the same thing Ctrl-C does. It printed:

```
Sent an internal break event. Press ^C again to kill Blender

Blender quit
```

and ended with exit code `1`. The folder afterwards held `f_0001.png` and `f_0002.png` at 92440 bytes each, and nothing for frame 3. So the abort lands mid-frame as far as the work is concerned, and at a frame boundary as far as the files are concerned: the part-rendered frame is discarded, not written and not half-written.

How long the interrupt took to bite is worth recording, because it differs between versions and was steady over three runs each. Blender 5.2.0 ended 5.0, 5.0 and 5.0 seconds after the interrupt — about as long as the rest of frame 3 would have taken, so 5.2 appears to finish the frame's work and then throw it away. Blender 4.5.12 behaved the same way, at 5.6 to 5.8 seconds. Blender 5.0.1 and 5.1.2 ended in 0.8 to 1.0 seconds. In none of the four was a file written for the interrupted frame.

A polite terminate signal ended it in 0.2 seconds with exit code `-15`, printing nothing further. A kill that could not be caught ended it in 0.2 seconds with exit code `-9`. Both left the same two files.

Run on the Creator's MacBook Pro, M1 Pro chip, macOS 26.5.2.

**Observed — hand test 23, 2026-08-05.** The `render_write` route named just above was tried, and it does not give a stop-after-this-frame. Blender 5.2.0 rendered frames 1 to 8 with a handler attached to Blender's own "frame written" event; the moment frame 2 was written, the handler set the scene's end frame to 2:

```
HANDLER: frame 2 has just been written; setting frame_end to 2
HANDLER: frame_end now reads 2
```

All eight frames were rendered anyway — `f_0001.png` through `f_0008.png`, 53282 bytes each. The animation loop does not re-read the end frame as it goes. Same machine and date. That does not rule out every possible script route, but it rules out the obvious one.

**Observed — hand test 29, run by the Creator, 2026-08-05.** Blender's own window offers no deferred stop either, and the two abort routes the sources name are the only ones there are. With an animation render running in Blender 5.2.0, the Creator opened the topbar's **Render** menu. It held six items and no more:

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

> Frame:3 | Last:00:27.29 Time:00:07.73 | Remaining: 00:19.10 | Mem: 3M | Sample 1153/4000

So the window's offer is "Stop this job", with nothing about waiting for the frame — the request, not the timing, exactly as this answer says the sources describe it.

On when the request takes effect, the Creator's own words: "Stopping isn't instant, there's a slight lag. All versions." They did not time the lag. That matches what the runs from a terminal measured — about 5.0 seconds in this version, against under a second in 5.0 and 5.1 — though those were an interrupt sent to a Blender with no window rather than Esc pressed in one.

Afterwards the output folder held exactly `frame_0001.png` and `frame_0002.png`, 92465 bytes each, and nothing for the frame that was in flight. So the abort lands mid-frame in the window too, and the part-rendered frame is discarded.

Run on the Creator's Mac — a MacBook Pro, M1 Pro chip, macOS 26.5.2 — so this settles macOS and leaves Windows and Linux open.

**Observed — Linux hand tests 5, 6 and 22, 2026-08-06.** The same answer on a rented Linux machine — Ubuntu 22.04.5 LTS, NVIDIA GeForce RTX 4090, driver 580.105.08, twelve cores, run over SSH. An interrupt, a polite terminate and an uncatchable kill each stopped Blender 5.2.0 there, none of them waited for the current frame, and none left anything for it. The handler route failed the same way: with a handler on Blender's own "frame written" event setting the end frame to 2 the moment frame 2 was written, all eight frames were rendered anyway.

```
HANDLER: frame 2 has just been written; setting frame_end to 2
HANDLER: frame_end now reads 2
the render call has returned
---- FOLDER AFTERWARDS ----
f_0001.png  53282 bytes
f_0002.png  53282 bytes
f_0003.png  53282 bytes
f_0004.png  53282 bytes
f_0005.png  53282 bytes
f_0006.png  53282 bytes
f_0007.png  53282 bytes
f_0008.png  53282 bytes
```

**On how long the stop took, this machine cannot say.** Three interrupts were sent in this version and the process ended 4.0, 4.2 and 3.8 seconds later. Those numbers are raw output kept for the record and **are not a measurement**: the machine was shared with other work throughout, which can only stretch them. The Mac's steady pattern — about five and a half seconds in 4.5 and five in 5.2, against about one second in 5.0 and 5.1 — is neither confirmed nor contradicted here. Settling it needs the machine to itself, and that is written into `Audits/Blender Research/Frames/Hand tests — Linux.md`.

**Observed — Linux hand test 32, 2026-08-06.** Blender's own window offers nothing that stops a render at a frame boundary on this machine either. With an animation render running in Blender 5.2.0 on the rented Linux machine's display — started the way the menu item starts it — the topbar's **Render** menu was opened with Blender's own `wm.call_menu` and photographed with Blender's own screenshot command. The picture shows six items and no more: Render Image (F12), Render Animation (Ctrl F12), Render Audio…, View Render (F11), View Animation (Ctrl F11), and a Lock Interface checkbox. No stop, pause, cancel or finish-this-frame item. The status bar at the foot of the window shows the word **Rendering…**, then a progress bar with a percentage in it, then a small cross.

Two things were not settled there and are not claimed. The cross's tooltip was not read, because hovering needs the pointer. And **whether the Esc key reached Blender could not be demonstrated** — that machine's display is shared with other work, and in one version a further frame was written after Esc was sent, which says the render was still going. So the "Stop this job." tooltip and the Esc cancel rest on the Creator's macOS run alone.

**Outside the interface.** The two abort routes the sources do describe — the status-bar cancel button and Esc — both need Blender's window. The sources describe no way at all to ask a background render (`-b, --background`, "Run in background (often used for UI-less rendering)") to stop, deferred or immediate. So the question's premise has no documented answer outside the interface either. **Observed**, from hand tests 6, 7 and 23: a background render *can* be stopped, by an interrupt, a polite terminate, or a kill; none of the three waits for the current frame; and the one script route the sources allowed to be imagined does not work. **Observed**, from hand test 29: inside the window there is no such control either — the Render menu offers no stop of any kind, and the only one there is is the status bar's cross, labelled "Stop this job." So the answer is the same on both sides of the window: nothing stops a render at a frame boundary, and the frame in flight is thrown away.

**Operating system.** The sources do not say. Nothing about cancelling or aborting a render is marked as platform-specific in the 5.2 manual. **Observed on two operating systems, and they agree that there is no such control** — macOS 26.5.2 (hand tests 6, 7, 23 and 29, 2026-08-05) and Ubuntu 22.04.5 LTS on the rented Linux machine (Linux hand tests 5, 6, 22 and 32, 2026-08-06). Nothing stops a render at a frame boundary on either, the frame in flight is thrown away on both, and the Render menu holds the same six items on both. **How long a stop takes to bite is settled on macOS only**, for the reason given above. **Windows remains untested**, because no Windows machine has run it.

---

## Things a person with Blender 5.2 open could still settle by hand

The list that stood here has been run — the results are written into the answers above as observed findings, dated 2026-08-05 for the Creator's Mac and 2026-08-06 for the rented Linux machine. Three of the items that stood here have since been answered: **what several view layers write with the compositor switched on** (question 9), **Autosplit at the 2 GiB boundary** (question 5), and **Linux and a graphics card of another make**, which now run through every answer above. Three items are left, and none of them is a matter of somebody sitting down at Blender for a few minutes. **Observed 2026-08-06 on a third machine, and it added one more:** two graphics cards in one machine leave the frame order unchanged, and no change in the picture was shown — see question 10. *(Two-card citation check, 2026-08-06: this read "change neither the frame order nor the picture". The pixel counts do not carry that: they show no difference beyond a single step in 255, and in 4.5 and 5.1 rather more of those than two runs on the same cards showed.)*

1. **What an abrupt power-off leaves behind.** (Question 1.) A Blender killed outright is recorded under question 1 and leaves the finished frames whole, on both machines. What that cannot reach is a file the operating system had not yet flushed to disk when the power went. This was written out as a test and offered; the Creator declined it, which is their call. It cannot be run on the rented Linux machine either, because that machine is reached over the network and cutting its power would leave nothing to read the result from.

2. **What a media player does with a part-written video file, on anything but QuickTime on macOS.** (Question 5.) QuickTime Player is recorded under question 5. The rented Linux machine has no media player installed at all and installing one was not permitted, so only Blender's own reader has opened these files there. What Blender's own Video Sequencer makes of one is also still unrecorded.

3. **Anything on Windows.** No Windows machine has run it, so every observed finding above covers macOS and Linux and leaves Windows open.

Three more things need the rented machine to itself rather than a machine shared with other work, and they are written out in `Audits/Blender Research/Frames/Hand tests — Linux.md`: how long a stop takes to bite, the pointer half of the window tests, and a demonstrated Esc cancel.

---

2026-08-04 — Opus 5 (Claude Code) — Frames, Blender 5.2

2026-08-05 — Opus 5 (Claude Code) — hand-test results written into questions 1 to 11, and the hand-test list above pruned

2026-08-05 — Opus 5 (Claude Code) — the Creator's own hand-test results written into questions 1, 3, 5, 7, 8 and 11, and the hand-test list above pruned again

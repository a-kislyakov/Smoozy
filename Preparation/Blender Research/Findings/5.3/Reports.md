# Reports — findings, Blender 5.3

- **Sources consulted:** Blender 5.3 manual (snapshot taken 2026-10-11), Blender 5.3 Python API documentation (snapshot 2026-10-11), Blender 5.3 release notes (snapshot 2026-10-11), and Cycles' own files from Blender 5.3 (taken 2026-10-16). All of them were read from `Blender Sources/` on 2026-10-16. Dates read below use the snapshot dates, as instructed.
- `I read only the question sheet and files inside Blender Sources/. I did not open any other part of this project.`

Two things to add, so the line above is complete. (1) Inside `Blender Sources/` I also read `About this folder.md`, which describes the folder layout. I did not follow its link out of the folder. I listed the `requirements/` folder but did not open the file in it, and nothing below uses it. (2) Before I started, the tool I run in placed some text into my context on its own: the project's `CLAUDE.md` signpost and a one-line-per-entry list of the user's saved notes. I did not open either one, I did not follow either one, and nothing below comes from them. To check that the output file did not already exist, I tested for the file's existence without reading anything in that folder.

Manual quotations are from *The Blender 5.3 Manual* by the Blender Documentation Team, licensed CC-BY-SA 4.0. Quotations are kept to the words that carry each claim.

How to read the labels: **documented** means the source says it outright. **observed** means it is read straight off a Cycles file. **inferred** means it follows from a source, and the step is written out. **not in the sources** means I searched and found nothing. The search record is given.

**Hand tests, 2026-10-17 — how every run marked with that date was made.** *(Added by the hand tests. Everything above and every researcher's finding below is left as written; what a running Blender did is added beside it.)* A block headed **Observed by hand test, 2026-10-17** is **observed** in the second sense of the word: something was run and what came back is quoted. Every one was run by Claude Opus 5.5 (Claude Code) on **the Creator's Mac — a MacBook Pro with an Apple M1 Pro chip, its own 14-core graphics (Metal) and no other graphics card, 16 GB of memory, macOS 26.6.2.** That is the only machine behind any of them: nothing here was run on Linux or on Windows, and nothing on an NVIDIA, AMD or Intel graphics card. The Blender was **5.3.0 Beta**, run from `/Applications/Blender 5.3b.app/Contents/MacOS/Blender`, and it reads itself out as `Blender 5.3.0 Beta (hash 5d8151335b6b built 2026-10-16 01:36:01)`.

*The same for every run.* Every run used a settings folder of its own, given with the environment variable `BLENDER_USER_RESOURCES`, and a temporary folder of its own, given with `TMPDIR`, both inside a throwaway work folder, written `<work folder>` below (`/private/tmp/claude-501/smoozy-hand-tests-5.3/Reports`). That settings folder had its preferences saved once, with only the start-up splash and the automatic saving of preferences turned off, so a window opens straight onto the scene. Unless a block says otherwise Blender was started with no window (`-b`), its two output streams went to two separate files in the same run (`> out.txt 2> err.txt`), and the exit number is the one the shell held straight afterwards (`echo $?`). Where a block says a window was used, Blender was started with its window from the terminal on the same Mac, driven by a script on timers, and photographed with Blender's own `bpy.ops.screen.screenshot`; where it says pointer moves or key presses were sent, they were sent with Blender's own `Window.event_simulate`, which needs Blender started with `--enable-event-simulate`, and no real mouse or keyboard touched the window. A file "made missing" was moved aside immediately before its run and back immediately after.

*The scenes*, every one built by this Blender 5.3 itself by a script:
- `simple.blend` — the factory scene (one cube, one camera, one light), Cycles on the processor, 16 samples, 160 by 120, frames 1 to 3, PNG output. `nocamera.blend` is the same with the camera deleted; `missing.blend` the same with the cube's colour read through an Image Texture node from `gone.png`; `partway.blend` the factory scene at 2000 samples, 320 by 240, adaptive sampling off, frames 1 to 6.
- `base.blend` — a cube, a UV sphere, an area light and a camera; Cycles on the processor; 16 samples; adaptive sampling off; time limit 0; 160 by 120; frames 1 to 3; PNG output. `samples.blend` is the same at 4096 samples and 480 by 360.
- `good.blend` — the factory scene, Cycles on the processor, 8 samples, 64 by 64, frames 1 to 3. `slow.blend` the same at 800 by 800 and a million samples with adaptive sampling off, so one frame runs for hours. `tex_missing.blend` is `good.blend` with the cube's colour read from `missing_tex.png`; `lib_missing.blend` is `good.blend` without the cube, showing a collection `LibColl` linked from `lib.blend`; `font_missing.blend` is `good.blend` plus a text object using `missing_font.ttf`; `seq_missing.blend` an empty 64 by 64 scene whose Video Sequencer holds an image strip reading `missing_seq.png`; `cache_missing.blend` is `good.blend` with the cube read through a Mesh Sequence Cache modifier from `cube.abc`; `volume_missing.blend` is `good.blend` plus a Volume object reading `volume.vdb`, a file kept from a small smoke bake.
- `scene14.blend` — the factory scene, Cycles on the processor, 900 samples, adaptive sampling and denoising off, 720 by 405 at 100 per cent, frames 1 to 5, PNG.
- `seq.blend` — a scene `Main` whose Video Sequencer holds a scene strip of a scene `Shot` (the base scene at 256 samples, 320 by 240, frames 1 to 10); every workspace's sequencer scene is `Main`.

In the commands below, `"$B53"` stands for the Blender program named above, and paths are shortened to `<work folder>` and `<scene folder>`; nothing else in a quoted line is changed. **What none of these runs covers:** Linux, Windows, any graphics card but the Mac's own, and any other build of 5.3 — this is a beta, and a later 5.3 build was not tried.

---

## 1. What can cause a render to fail, and what does Blender do in each case?

**The question is ambiguous.** "Fail" can mean (a) the render stops or reports an error instead of finishing, or (b) Blender itself crashes. I answer both. I also list two ways a render can end early without failing, because a reader could count them as failures.

**Findings**

1.1 — **documented** — *Not enough memory on the graphics card (Cycles on a GPU).* The manual names this as the most common reason a scene that renders on the CPU will not render on the GPU. For four of the GPU types, Blender first falls back to the computer's main memory. The manual names one error message for this case.
> "There may be multiple causes, but the most common one is that there is not enough memory on your graphics card."
> "With CUDA, OptiX, HIP and Metal devices, if the GPU memory is full Blender will automatically try to use system memory. This has a performance impact, but will usually still result in a faster render than using CPU rendering."
> "Error: Out of memory" / "This usually means there is not enough memory to store the scene for use by the GPU."
Manual — https://docs.blender.org/manual/en/5.3/render/cycles/gpu_rendering.html — read 2026-10-11.

1.2 — **documented** — *The GPU program cannot be built for the card.*
> "CUDA Error: Kernel compilation failed" / "This error may happen if you have a new NVIDIA graphics card that is not yet supported by the Blender version and CUDA toolkit you have installed. In this case Blender may try to dynamically build a kernel for your graphics card and fail."
Manual — https://docs.blender.org/manual/en/5.3/render/cycles/gpu_rendering.html — read 2026-10-11.

1.3 — **documented** — *Compiler version mismatch on Linux.* The section is headed "Unsupported GNU version".
> "On Linux, depending on your GCC version you might get this error."
Manual — same page — read 2026-10-11.

1.4 — **documented** — *Windows time limit on a GPU that also drives the display.*
> "The NVIDIA OpenGL driver lost connection with the display driver" / "If a GPU is used for both display and rendering, Windows has a limit on the time the GPU can do render computations. If you have a particularly heavy scene, Cycles can take up too much GPU time."
Manual — same page — read 2026-10-11.

1.5 — **documented** — *EEVEE using too much GPU memory, and EEVEE's shadow memory filling up.*
> "In practice, using too much GPU memory can make the GPU driver crash, freeze, or kill the application."
Manual — https://docs.blender.org/manual/en/5.3/render/eevee/limitations/limitations.html — read 2026-10-11.
> "The error message Shadow buffer full means that the system cannot allocate enough shadow memory."
Manual — https://docs.blender.org/manual/en/5.3/render/eevee/light_settings.html — read 2026-10-11. The page does not say whether the render stops when this happens.

1.6 — **documented** — *No way to save the result.* If automatic saving is switched off and nothing else is set up to write the frames, rendering reports an error.
> "If no valid output method is available, rendering operations will report errors."
> "If Save Output is disabled, the render will fail unless an alternative output method is provided."
Manual — https://docs.blender.org/manual/en/5.3/render/output/animation.html — read 2026-10-11.
> "For animation renders, if the Video Sequencer is active and no alternative output is available, an error is raised. Otherwise, if compositing is enabled, active File Output Node nodes are used to save the result. If no such nodes exist, an error is reported."
> "Both Render Playblast and Render Playblast on Keyframes will raise an error when render output is disabled in this panel."
Manual — https://docs.blender.org/manual/en/5.3/render/output/properties/output.html — read 2026-10-11.
The switch is the setting `save_output`: "Write frames to disk for animation renders (default True)". API — https://docs.blender.org/api/5.3/bpy.types.RenderSettings.html — read 2026-10-11.

1.7 — **documented** — *Video format choices that do not fit together.*
> "so if you are getting errors check that your container and codec are compatible."
> "some codecs do not work with arbitrary dimensions."
> "If a problem occurs while rendering, the file might become unplayable and you will have to re-render all frames from the beginning."
Manual — https://docs.blender.org/manual/en/5.3/render/output/properties/output.html — read 2026-10-11.

1.8 — **documented** — *Blender crashes.* The manual lists the common causes. On a crash Blender writes a text file. File names and places differ by operating system (see the Operating system line).
> "The most common causes of Blender crashes: Running out of memory. Issues with graphics hardware or drivers. Bugs in Blender."
> "When Blender crashes, it writes out a text file which contains information that may help identify the cause of the crash. Usually, this file is written in the Temporary Directory directory."
Manual — https://docs.blender.org/manual/en/5.3/troubleshooting/crash.html — read 2026-10-11.
Two script-caused crashes are named in the API documentation:
> "a crash can occur while rendering with Cycles, with Python drivers, while a background thread is used to download some file."
API — https://docs.blender.org/api/5.3/info_gotchas_threading.html — read 2026-10-11.
> "If the handler changes data that is accessed by the viewport, this can cause a crash of Blender."
API — https://docs.blender.org/api/5.3/bpy.app.handlers.html — read 2026-10-11. This is about a script changing data from a frame-change hook while rendering.

1.9 — **documented** — *The person cancels the render.* In the interface, the progress bar has a cancel button. A viewport render can be stopped with Esc, "as with a normal render". Scripts can be told when a render job is cancelled.
> "The task can be aborted by clicking the cancel button ()."
Manual — https://docs.blender.org/manual/en/5.3/interface/window_system/status_bar.html — read 2026-10-11.
> "As with a normal render, you can abort it with Esc."
Manual — https://docs.blender.org/manual/en/5.3/editors/3dview/viewport_render.html — read 2026-10-11.
> `render_cancel` — "on canceling a render job."
API — https://docs.blender.org/api/5.3/bpy.app.handlers.html — read 2026-10-11.

1.10 — **inferred** — *Ctrl-C in the terminal cancels a render in the normal program.* The API page on Blender built as a Python module says:
> "Blender’s typical signal handlers are not initialized, so there is no special handling for Control-C to cancel a render and a crash log is not written in the event of a crash."
API — https://docs.blender.org/api/5.3/info_advanced_blender_as_bpy.html — read 2026-10-11.
The step: the sentence calls these handlers "typical", and says the module version lacks them. So the normal program has them, and they give Control-C "special handling … to cancel a render". The sources never say this directly about the normal program.

1.11 — **documented** — *A render started from a Python script reports errors.* The script gets an exception carrying the messages.
> "However, if there are error reports, a RuntimeError will be raised after the operator finishes execution, including all error report messages, regardless of the return status (even if it was {'FINISHED'})."
API — https://docs.blender.org/api/5.3/bpy.ops.html — read 2026-10-11.

1.12 — **documented / observed** — *Blender refuses what it was given on the command line.*
- **documented:** an unknown argument makes Blender exit. "…will exit since -ba is an unknown argument." Manual — https://docs.blender.org/manual/en/5.3/advanced/command_line/arguments.html — read 2026-10-11.
- **observed:** if the Cycles device named after `--cycles-device` cannot be set, the Cycles add-on calls `sys.exit(1)`. Cycles file `engine.py`, lines 38–41:
  ```
      if args.cycles_device:
          import _cycles
          if not _cycles.set_device_override(args.cycles_device):
              sys.exit(1)
  ```
  Read 2026-10-16.
- **documented:** "since sys.exit() will close Blender immediately." API — https://docs.blender.org/api/5.3/info_gotchas_crashes.html — read 2026-10-11.

1.13 — **documented** — *The blend-file cannot be loaded.* There is a script hook for this. It is the only place in the sources that names the case.
> `load_post_fail` — "on failure to load a new blend file (after)."
API — https://docs.blender.org/api/5.3/bpy.app.handlers.html — read 2026-10-11. The sources do not say what Blender does after a failed load.

1.14 — **documented** — *Two ways a render ends early without failing.*
- Time limit: "Stops rendering if the time exceeds the limit, even if the desired sample count hasn’t been reached yet." Manual — https://docs.blender.org/manual/en/5.3/render/cycles/render_settings/sampling.html — read 2026-10-11. EEVEE has the same wording at https://docs.blender.org/manual/en/5.3/render/eevee/render_settings/sampling.html.
- Unsupported engine for viewport render: "Viewport rendering only works for the Workbench and EEVEE render engines. It’s not supported for Cycles." Manual — https://docs.blender.org/manual/en/5.3/editors/3dview/viewport_render.html — read 2026-10-11.

**Not covered.** The sources give no complete list of failure causes. For most causes above they do not say what happens to the frame being rendered: whether it is saved part-done, dropped, or skipped. They also do not say whether an animation stops or continues to the next frame after a failed frame. For GPU errors, they do not say where the message appears. The Cycles preferences file skips GPUs whose driver is too old (`properties.py` line 2123: "Devices that do not meet the driver requirement are not used;"). It does not say whether a render then fails or carries on with other devices.

**Where I looked.** I read the manual pages for command-line rendering, arguments, launching, GPU rendering, crashes, startup, the status bar, viewport render, render output, the Output panel, sampling (Cycles and EEVEE) and EEVEE limitations. I read the API pages for `bpy.app.handlers`, `bpy.ops`, `RenderEngine`, `RenderSettings`, and the info_gotchas and info_advanced pages. I read all 5.3 release notes. In Cycles' files I read `engine.py`, `__init__.py` and `operators.py`, and searched `properties.py` and `osl.py`. Across the manual I searched for "fail" near render, frame, output, sample, bake, GPU, Cycles, kernel and device. I also searched for "error" in the render pages, and for "cancel", "abort", "interrupt", "stop", "kill", "Ctrl-C", "SIGINT", "signal", "crash" and "out of memory".

**Observed by hand test, 2026-10-17 — what is refused before a render starts (hand-test entry 10).** On the Creator's Mac, Blender 5.3.0 Beta, set up as described at the top of this document, with no window. Each of these ended at **exit 1** with nothing written, and its complaint on standard output under the `reports` or `cycles` category: a blend-file that is not there (`ERROR Cannot read file "…": No such file or directory`); a plain text file named `.blend` (`ERROR File format is not supported in file "…"`); a blend-file cut short (`ERROR Failed to read blend file '…': Missing DNA block`); an unknown argument after a valid file (`ERROR Cannot read file "<work folder>/--no-such-flag": No such file or directory`, with `unknown argument, loading as file: --no-such-flag` on the error stream); a Cycles device name Cycles does not know (`ERROR NONSENSE is not a valid Cycles device.`); and a device name it knows that has no hardware here — `CUDA`, `OPTIX`, `HIP`, `ONEAPI` — (`ERROR Found no Cycles device of the specified type`). An engine name that does not exist ended at **exit 1** with `Error: engine not found 'NOSUCHENGINE'` on the **error stream**. The manual's example `-ba` did not end by itself within 90 seconds. Every command and every line printed is quoted under question 12.

**Observed by hand test, 2026-10-17 — failures during or after a render, with no window (hand-test entry 11).** On the Creator's Mac, Blender 5.3.0 Beta, set up as described at the top of this document. Each command's two streams went to separate files and the exit number was read straight after. Where a capture below omits lines, it is the run of stage lines (`Synchronizing …`, `Initializing`, `Waiting for render to start`, `Updating …`, `Loading …`) and nothing else; every other line is quoted.

*1. No camera in the scene* — `"$B53" -b <scene folder>/nocamera.blend -o "<work folder>/out/e11/nc/frame_" -F PNG -f 1`: **exit 0**, and no output folder was made, so nothing was written. The error stream was empty. Standard output, whole:
```
00:00.326  blend            | Read blend: "<scene folder>/nocamera.blend"
00:00.331  reports          | ERROR No camera found in scene "Scene"
Blender 5.3.0 Beta (hash 5d8151335b6b built 2026-10-16 01:36:01)

Blender quit
```
There is **no `Rendering single frame` or `Rendering frame 1` line**, and the wording is `No camera found in scene "Scene"`.

*2. An output folder that does not exist* — `-o "<work folder>/out/e11/no_such_folder_53/deeper/frame_"`, neither folder existing beforehand: **exit 0**; both folders were made and held `frame_0001.png`. Error stream empty. Standard output, stage lines omitted:
```
00:00.320  blend            | Read blend: "<scene folder>/simple.blend"
00:00.325  render           | Rendering single frame
00:00.325  render           | Rendering frame 1
00:00.325  render           | Start rendering: Scene, ViewLayer
00:00.325  render           | Engine: Cycles
Blender 5.3.0 Beta (hash 5d8151335b6b built 2026-10-16 01:36:01)
00:00.330  render           | Fra: 1 | Mem: 1M | Sample 0/16
00:00.331  render           | Fra: 1 | Remaining: 00:00.01 | Mem: 2M | Sample 1/16
00:00.419  render           | Fra: 1 | Mem: 2M | Sample 16/16
00:00.419  render           | Fra: 1 | Mem: 2M | Finished
00:00.481  render           | Saved: '<work folder>/out/e11/no_such_folder_53/deeper/frame_0001.png'
00:00.481  render           | Time: 00:00.15 (Saving: 00:00.06)

Blender quit
```

*3. An output path blocked by a plain file* (a one-byte file named `blocker` where the folder would go) — `-o "<work folder>/out/e11/blocker/frame_"`: **exit 1**, nothing written. Error stream empty. Standard output, stage lines omitted:
```
00:00.308  blend            | Read blend: "<scene folder>/simple.blend"
00:00.313  render           | Rendering single frame
00:00.313  render           | Rendering frame 1
00:00.313  render           | Start rendering: Scene, ViewLayer
00:00.313  render           | Engine: Cycles
Blender 5.3.0 Beta (hash 5d8151335b6b built 2026-10-16 01:36:01)
00:00.318  render           | Fra: 1 | Mem: 1M | Sample 0/16
00:00.319  render           | Fra: 1 | Remaining: 00:00.01 | Mem: 2M | Sample 1/16
00:00.406  render           | Fra: 1 | Mem: 2M | Sample 16/16
00:00.406  render           | Fra: 1 | Mem: 2M | Finished
00:00.440  image            | ERROR Couldn't create directory for file <work folder>/out/e11/blocker/frame_0001.png: File exists
00:00.440  reports          | ERROR Render error (File exists) cannot save: '<work folder>/out/e11/blocker/frame_0001.png'
00:00.440  render           | Time: 00:00.12

Blender quit
```
The render itself finished; the saving is what failed, reported under two categories, `image` and `reports`.

*4. A broken template in the output path* — `-o "<work folder>/out/e11/tmpl/{blend_name.png" -F PNG -x 1 -a`: **exit 1**; the folder was never made. Error stream empty. Standard output, whole:
```
00:00.319  blend            | Read blend: "<scene folder>/simple.blend"
00:00.323  render           | Rendering animation (frames 1..3)
00:00.323  render           | Rendering frame 1
00:00.323  reports          | ERROR Parse errors in path '<work folder>/out/e11/tmpl/{blend_name.png':
                            | - Invalid or incomplete template expression '{blend_name.png'.
Blender 5.3.0 Beta (hash 5d8151335b6b built 2026-10-16 01:36:01)

Blender quit
```

*5. An output path of `//`* — a copy of `good.blend` in `<work folder>/out/e11/`, run as `"$B53" -b <work folder>/out/e11/slash.blend -o // -f 1`: **exit 0**, and the picture was written beside the blend-file as `0001.png`. Error stream empty. Standard output, stage lines omitted:
```
00:00.326  blend            | Read blend: "<work folder>/out/e11/slash.blend"
00:00.330  render           | Rendering single frame
00:00.331  render           | Rendering frame 1
00:00.331  render           | Start rendering: Scene, ViewLayer
00:00.331  render           | Engine: Cycles
Blender 5.3.0 Beta (hash 5d8151335b6b built 2026-10-16 01:36:01)
00:00.336  render           | Fra: 1 | Mem: 1M | Sample 0/8
00:00.336  render           | Fra: 1 | Remaining: 00:00.00 | Mem: 1M | Sample 1/8
00:00.388  render           | Fra: 1 | Mem: 1M | Sample 8/8
00:00.388  render           | Fra: 1 | Mem: 1M | Finished
00:00.450  render           | Saved: '<work folder>/out/e11/0001.png'
00:00.450  render           | Time: 00:00.11 (Saving: 00:00.06)

Blender quit
```

*6. Saving the render output switched off, with nothing else arranged.* The exact command, with the Python expression placed after the file and before `-o`: `"$B53" -b <scene folder>/simple.blend --python-expr "import bpy; bpy.context.scene.render.save_output = False" -o "<work folder>/out/e11/so_a/frame_" -F PNG -x 1 -a`. **Exit 0**; the output folder was never made, so nothing was written. Error stream empty. Standard output, whole:
```
00:00.317  blend            | Read blend: "<scene folder>/simple.blend"
00:00.321  reports          | ERROR Render output disabled in Output properties and no active compositing File Output nodes
Blender 5.3.0 Beta (hash 5d8151335b6b built 2026-10-16 01:36:01)

Blender quit
```
**No progress line, no `Rendering` line and no `Time:` line was printed** — the render did not run. The same expression with `-f 1` in place of `-x 1 -a` gave the same: exit 0, no folder, and standard output holding only `Read blend:`, the same `ERROR Render output disabled in Output properties and no active compositing File Output nodes`, the banner, a blank line and `Blender quit`.

*7. The same, with the render called from Python and an exit code asked for* — `"$B53" -b <scene folder>/simple.blend -o "<work folder>/out/e11/so_i/frame_" -F PNG --python-exit-code 7 --python-expr "import bpy; bpy.context.scene.render.save_output = False; bpy.ops.render.render(animation=True)"`: **exit 7**; nothing written. Standard output, whole:
```
00:00.321  blend            | Read blend: "<scene folder>/simple.blend"
Error: Render output disabled in Output properties and no active compositing File Output nodes
Blender 5.3.0 Beta (hash 5d8151335b6b built 2026-10-16 01:36:01)

Blender quit
```
and the error stream, whole (its two blank lines are as printed):
```
Traceback (most recent call last):
  File "<string>", line 1, in <module>
RuntimeError: Error: Render output disabled in Output properties and no active compositing File Output nodes


Error: script failed, expr: 'import bpy; bpy.context.scene.render.save_output = False; bpy.ops.render.render(animation=True)', exiting.
```

*Not run: an output folder that cannot be written to.* The plan was to make a folder with `chmod 500` and render into it. The tool the runner works through refused the permission change, twice, so the folder could not be made read-only and the case was not run. This is the one case of this entry with no result.

**Observed by hand test, 2026-10-17 — an animation whose output folder stops being writable part-way (hand-test entry 12).** On the Creator's Mac, Blender 5.3.0 Beta, set up as described at the top of this document, on `partway.blend` (2000 samples, 320 by 240, frames 1 to 6, about seven seconds a frame). One command started the render in the background with `-o "<work folder>/out/e12/out_53/frame_" -F PNG -x 1 -a`, waited a fixed twelve seconds, listed the output folder — it held `frame_0001.png` — renamed the folder away to `out_53_moved`, and put a one-byte plain file named `out_53` in its place; then it waited for the render to end. The render ended at **exit 1**. The error stream was empty. Standard output held 104 lines; every line naming a frame, a save, an error or a time, with its line number, word for word:

```
3:00:00.330  render           | Rendering frame 1
51:00:07.099  render           | Saved: '<work folder>/out/e12/out_53/frame_0001.png'
52:00:07.099  render           | Time: 00:06.76 (Saving: 00:00.06)
53:00:07.099  render           | Rendering frame 2
100:00:13.800  image            | ERROR Couldn't create directory for file <work folder>/out/e12/out_53/frame_0002.png: File exists
101:00:13.800  reports          | ERROR Render error (File exists) cannot save: '<work folder>/out/e12/out_53/frame_0002.png'
102:00:13.800  render           | Time: 00:06.70
```

and the run's last lines were `Time: 00:06.70`, a blank line and `Blender quit`. **No `Rendering frame 3` line, or any line for frames 3 to 6, was printed** — the animation stopped at the frame it could not save. The renamed folder held one file, `frame_0001.png` (65,499 bytes); no part-written file was left anywhere.

**Observed by hand test, 2026-10-17 — which handlers fire when a render fails, what the reports list holds, what a script's own render call gets, and where a log file puts a failure (hand-test entry 13).** On the Creator's Mac, Blender 5.3.0 Beta, set up as described at the top of this document, with no window.

*Which handlers fire.* A start-up script attached a printing handler to each of `render_init`, `render_pre`, `render_post`, `render_write`, `render_complete` and `render_cancel` (and a silent one to `render_stats`), printed `PROBE INSTALLED`, and would have printed `HANDLER MISSING:` for any that did not exist; none was missing. Six runs, each `"$B53" -b <file> --python handlers_probe.py -o <path> -F PNG -f 1` (or `-x 1 -a`). Every line each printed from the probe, and every `ERROR` or `WARNING` line, in order:

```
— good render (simple.blend), exit 0
PROBE INSTALLED
HANDLER FIRED: render_init args: (bpy.data.scenes['Scene'], None)
HANDLER FIRED: render_pre args: (bpy.data.scenes['Scene'], None)
HANDLER FIRED: render_post args: (bpy.data.scenes['Scene'], None)
HANDLER FIRED: render_write args: (bpy.data.scenes['Scene'], None)
HANDLER FIRED: render_complete args: (bpy.data.scenes['Scene'], None)
— no camera (nocamera.blend), exit 0
PROBE INSTALLED
00:00.323  reports          | ERROR No camera found in scene "Scene"
— output path blocked by a plain file, exit 1
PROBE INSTALLED
HANDLER FIRED: render_init args: (bpy.data.scenes['Scene'], None)
HANDLER FIRED: render_pre args: (bpy.data.scenes['Scene'], None)
00:00.453  image            | ERROR Couldn't create directory for file <work folder>/out/e13/blocker/frame_0001.png: File exists
00:00.453  reports          | ERROR Render error (File exists) cannot save: '<work folder>/out/e13/blocker/frame_0001.png'
HANDLER FIRED: render_cancel args: (bpy.data.scenes['Scene'], None)
— broken template in the output path, -x 1 -a, exit 1
PROBE INSTALLED
HANDLER FIRED: render_init args: (bpy.data.scenes['Scene'], None)
00:00.325  reports          | ERROR Parse errors in path '<work folder>/out/e13/tmpl/{blend_name.png':
HANDLER FIRED: render_cancel args: (bpy.data.scenes['Scene'], None)
— missing image texture (missing.blend, gone.png moved aside), exit 0
PROBE INSTALLED
HANDLER FIRED: render_init args: (bpy.data.scenes['Scene'], None)
HANDLER FIRED: render_pre args: (bpy.data.scenes['Scene'], None)
00:00.334  cycles           | WARNING Image file <scene folder>/gone.png does not exist.
00:00.335  cycles           | ERROR Failed to load 1 image files
HANDLER FIRED: render_post args: (bpy.data.scenes['Scene'], None)
HANDLER FIRED: render_write args: (bpy.data.scenes['Scene'], None)
HANDLER FIRED: render_complete args: (bpy.data.scenes['Scene'], None)
— saving switched off by --python-expr, -x 1 -a, exit 0
PROBE INSTALLED
00:00.325  reports          | ERROR Render output disabled in Output properties and no active compositing File Output nodes
```

Every error stream was empty. So **on a scene with no camera not one handler fired** — not even `render_init` — and the same is true of saving switched off. A blocked save fired `render_init`, `render_pre` and `render_cancel`; a broken template fired `render_init` and `render_cancel` only; a missing texture, which succeeds, fired the full set a good render fires.

*The reports list after a failure with no window.* `"$B53" -b <scene folder>/nocamera.blend -o "<work folder>/out/e13/nc2_" -F PNG -f 1 --python reports.py`, the script printing `len(bpy.context.window_manager.reports)` and each entry. **Exit 0**, error stream empty, and standard output, whole:

```
00:00.317  blend            | Read blend: "<scene folder>/nocamera.blend"
00:00.321  reports          | ERROR No camera found in scene "Scene"
Blender 5.3.0 Beta (hash 5d8151335b6b built 2026-10-16 01:36:01)
REPORTS COUNT: 0

Blender quit
```

*A render called from a script.* A start-up script called `bpy.ops.render.render(write_still=True)` inside a `try` and printed what came back. On `simple.blend` it printed `RENDER RETURNED: {'FINISHED'}` and the run ended at exit 0. On `nocamera.blend` the run ended at **exit 1**, the error stream was empty, and standard output, whole:

```
00:00.322  blend            | Read blend: "<scene folder>/nocamera.blend"
Error: Cannot render, no camera
RENDER RAISED: RuntimeError Error: Cannot render, no camera
Blender 5.3.0 Beta (hash 5d8151335b6b built 2026-10-16 01:36:01)

Blender quit
```

So the script's call **raised a `RuntimeError` carrying `Error: Cannot render, no camera`**, and the run ended at 1 although the script caught it — while the same file rendered with `-f 1` says `No camera found in scene "Scene"` and ends at 0 (hand-test entry 11).

*A log file on a failure.* `"$B53" -b <scene folder>/nocamera.blend --log-file "<work folder>/logs/e13j.log" -o "<work folder>/out/e13/lf_" -F PNG -f 1`: **exit 0**, error stream empty; standard output held only the version banner, a blank line and `Blender quit`, and the file held, whole:

```
00:00.320  blend            | Read blend: "<scene folder>/nocamera.blend"
00:00.324  reports          | ERROR No camera found in scene "Scene"
```

**Observed by hand test, 2026-10-17 — a crash on purpose, and where the crash file goes on macOS (hand-test entry 14).** On the Creator's Mac, Blender 5.3.0 Beta, set up as described at the top of this document — so Blender's temporary folder for these runs was `<work folder>/tmp`, given with `TMPDIR`. Blender was made to crash with a start-up script whose whole content is `import ctypes`, `print("ABOUT TO CRASH")` and `ctypes.string_at(0)`.

*With a blend-file loaded* — `"$B53" -b <scene folder>/simple.blend --python crash.py`: **exit 139**. The error stream was empty. Standard output, whole:

```
00:00.318  blend            | Read blend: "<scene folder>/simple.blend"
Blender 5.3.0 Beta (hash 5d8151335b6b built 2026-10-16 01:36:01)
Writing: <work folder>/tmp/simple.crash.txt
```

The `print` that ran just before the crash never reached standard output. The file `<work folder>/tmp/simple.crash.txt` existed afterwards, 28 lines, its headings:

```
# Blender 5.3.0, Commit date: 2026-10-16 00:30, Hash 5d8151335b6b
# backtrace
# Python backtrace
```

*With no blend-file loaded* — `"$B53" -b --factory-startup --python crash.py`: **exit 139**, error stream empty, standard output, whole:

```
Blender 5.3.0 Beta (hash 5d8151335b6b built 2026-10-16 01:36:01)
Writing: <work folder>/tmp/blender.crash.txt
```

and `<work folder>/tmp/blender.crash.txt` was written. **So on macOS the crash file goes to Blender's temporary folder, named after the blend-file when one is loaded and `blender.crash.txt` when none is.** The system's own reports folder was listed (Blender's names only) before and after the two runs: one new file, `Blender-2026-10-17-045619.ips`, appeared in `~/Library/Logs/DiagnosticReports/`; which of the two runs made it, and whether the system showed a window about it, was not established.

**Observed by hand test, 2026-10-17 — Ctrl-C typed into a real terminal (hand-test entry 16).** On the Creator's Mac, Blender 5.3.0 Beta, set up as described at the top of this document, on `slow.blend` (800 by 800, a million samples, so one frame runs for hours). Each run was started inside a real terminal made with Python's `pty.fork()`, and the interrupt byte `0x03` — what a terminal receives when a person presses Ctrl-C — was written into that terminal from outside at a fixed time; the harness then read everything the terminal showed and how the process ended. A terminal carries both streams together, so the two are not told apart here. Below, everything the terminal showed is quoted except the run of stage lines; the lines beginning `[harness]` are the harness's own.

*One press, a single frame* — `"$B53" -b <scene folder>/slow.blend -o "<work folder>/out/e16/one_####" -f 1`:

```
[harness] wrote 0x03 at 12.037 s
Blender 5.3.0 Beta (hash 5d8151335b6b built 2026-10-16 01:36:01)
00:00.323  blend            | Read blend: "<scene folder>/slow.blend"
00:00.328  render           | Rendering single frame
00:00.328  render           | Rendering frame 1
00:00.328  render           | Start rendering: Scene, ViewLayer
00:00.328  render           | Engine: Cycles
00:00.333  render           | Fra: 1 | Mem: 1M | Sample 0/1000000
00:00.356  render           | Fra: 1 | Remaining: 06:22:12.76 | Mem: 35M | Sample 1/1000000
^C
Sent an internal break event. Press ^C again to kill Blender
00:25.755  render           | Fra: 1 | Remaining: 06:52:56.06 | Mem: 35M | Sample 1025/1000000
00:25.761  render           | Fra: 1 | Remaining: 06:53:02.43 | Mem: 35M | Cancelled

Blender quit
[harness] exited, status 1
```

*One press, an animation* — the same with `-a` in place of `-f 1`:

```
[harness] wrote 0x03 at 12.019 s
Blender 5.3.0 Beta (hash 5d8151335b6b built 2026-10-16 01:36:01)
00:00.321  blend            | Read blend: "<scene folder>/slow.blend"
00:00.326  render           | Rendering animation (frames 1..3)
00:00.326  render           | Rendering frame 1
00:00.326  render           | Start rendering: Scene, ViewLayer
00:00.326  render           | Engine: Cycles
00:00.331  render           | Fra: 1 | Mem: 1M | Sample 0/1000000
00:00.355  render           | Fra: 1 | Remaining: 06:35:24.43 | Mem: 35M | Sample 1/1000000
^C
Sent an internal break event. Press ^C again to kill Blender
00:25.778  render           | Fra: 1 | Remaining: 06:53:19.99 | Mem: 35M | Sample 1025/1000000
00:25.784  render           | Fra: 1 | Remaining: 06:53:26.28 | Mem: 35M | Cancelled

Blender quit
[harness] exited, status 1
```

*Two presses, three seconds apart, a single frame:*

```
[harness] wrote 0x03 at 12.060 s
[harness] wrote 0x03 at 15.069 s
Blender 5.3.0 Beta (hash 5d8151335b6b built 2026-10-16 01:36:01)
00:00.323  blend            | Read blend: "<scene folder>/slow.blend"
00:00.327  render           | Rendering single frame
00:00.327  render           | Rendering frame 1
00:00.327  render           | Start rendering: Scene, ViewLayer
00:00.327  render           | Engine: Cycles
00:00.332  render           | Fra: 1 | Mem: 1M | Sample 0/1000000
00:00.356  render           | Fra: 1 | Remaining: 06:32:23.39 | Mem: 35M | Sample 1/1000000
^C
Sent an internal break event. Press ^C again to kill Blender
^C
Blender killed
Error: Not freed memory blocks: 30168, total unfreed memory 41.431965 MB
[harness] exited, status 2
```

So **one press ended the run at status 1**, for a frame and for an animation alike, through a `| Cancelled` line, `Blender quit`, and no further frame; **two presses ended it at status 2** with `Blender killed`. The output folder was never made in any of the three runs, so **no picture of any kind was left**. One more thing the times show: after the single press, written at about 12 seconds, the render went on to `Sample 1025/1000000` and printed `Cancelled` at `00:25.76` in Blender's own clock — about thirteen seconds later.

**Observed by hand test, 2026-10-17 — a missing image texture does not stop a render (hand-test entry 21).** On the Creator's Mac, Blender 5.3.0 Beta, with no window: a cube whose material reads a picture that had been moved away rendered to the end, ended at **exit 0**, and painted the cube magenta (centre of the picture `(0.5922, 0.2078, 0.6039, 1.0)` against `(0.549, 0.549, 0.549, 1.0)` with the picture present). Cycles printed `WARNING Image file <scene folder>/missing_tex.png does not exist.` and `ERROR Failed to load 1 image files`; EEVEE printed `image.gpu        | ERROR Image file "<scene folder>/missing_tex.png" does not exist`. The whole record is under question 13. *(The same holds for the older `missing.blend` in hand-test entry 13: exit 0 and the picture written.)*

**Observed by hand test, 2026-10-17 — a simulated Esc during a render with the window open (hand-test entry 31).** On the Creator's Mac, Blender 5.3.0 Beta, started with its window and with `--enable-event-simulate`, on `scene14.blend` frames 1 and 2, the render started by `bpy.ops.render.render('INVOKE_DEFAULT', animation=True)`. At 21 seconds — after frame 1 was saved at `00:17.676` — the script sent an Esc press and release into the window with `Window.event_simulate`, the pointer over the 3D viewport. Its own lines afterwards, word for word:

```
[plan  21.00] key ESC pressed and released with the pointer at 1200,900
[plan  24.13] shot of window 0 (1512 x 862) saved as e31_afteresc_w0.png
[plan  24.14] shot of window 1 (390 x 262) saved as e31_afteresc_w1.png
[plan  30.00] render_stats fired 98 times, render_complete 0, render_cancel 0
[plan  30.00] FIRST=('Mem: 0M | Synchronizing object | Cube', None)
[plan  30.00] LAST=('Remaining: 00:03.79 | Mem: 5M | Sample 673/900', None)
[plan  31.00] files in out/e31: ['f_0001.png']
```

**`render_cancel` did not fire, and the statistics handler was still being handed sample counts afterwards** — its latest string at 30 seconds read `Sample 673/900` for frame 2. The picture three seconds after the key still showed `Rendering...` in the Status Bar. Blender was then closed by the script at 32 seconds, before frame 2 was saved, and ended at exit 0. So a simulated Esc sent this way did not stop the render. Whether a real Esc pressed by a person does is not settled by this: no real key reached the window, and the Creator ran nothing in this sitting.

**Observed by hand test, 2026-10-17 — the Output panel's header, and the output path field holding a broken template (hand-test entry 33).** On the Creator's Mac, Blender 5.3.0 Beta, set up as described at the top of this document, started with its window and with `--enable-event-simulate` on `base.blend`. A start-up script switched the Properties editor to the Output tab and set `bpy.context.scene.render.filepath = '//out_{blend_name.png'`. Three runs, each ending at exit 0: the first only photographed the tab, so the checkbox and the field could be found; the second moved the pointer, with `Window.event_simulate`, onto the checkbox (window pixel 2616 across, 468 up) and photographed after three seconds, then onto the path field (2764, 408) and photographed after three seconds; the third moved onto the path field alone (2764, 408, then 2770, 410) and photographed after five seconds.

*The header.* The panel headed `Output` **carries a checkbox in its header, and it was ticked.** With the pointer on it, the picture shows a tooltip, word for word:

```
Save Output
Write frames to disk for animation renders.
```

*The path field.* **The field holding `//out_{blend_name.png` is drawn with a red background.** In the second run's picture, three seconds after the pointer moved onto it, no tooltip was showing. In the third run's picture, five seconds after, a tooltip was showing, word for word:

```
Output Path
Directory/name to save animations, # characters define the position and padding of frame numbers.
Value: //out_{blend_name.png
Path template error(s):
  - Invalid or incomplete template expression '{blend_name.png'.
```

The last two lines are drawn in red. Below the field the same pictures show `Saving` with `File Extensions` ticked and `Cache Result` not ticked, then `Media Type` set to `Image` and `File Format` to `PNG (.png)`.

**Observed by hand test, 2026-10-17 — the setting that switches saving off exists, and is on (hand-test entry 38).** On the Creator's Mac, Blender 5.3.0 Beta, read from the running program with its window open on `base.blend`: `scene.render.save_output = True`. The same setting switched off is what hand-test entries 11, 13 and 34 use; the checkbox it draws in the Output panel's header is under hand-test entry 33.

**Observed by hand test, 2026-10-17 — how a failure is announced with Blender's window open (hand-test entry 34).** On the Creator's Mac, Blender 5.3.0 Beta, set up as described at the top of this document, started with its window from a terminal. In every run a start-up script first turned the Outliner into an Info editor with its filters for debug, information, operator, warning and error messages on. Key presses were sent with `Window.event_simulate`, which needs `--enable-event-simulate`, the pointer over the 3D viewport; F12 and Ctrl-F12 are the keys the running program binds to rendering a still and an animation (hand-test entry 38). Every run ended at exit 0 with an empty error stream. **One thing about every picture:** in hand-test entries 30 and 31 the pictures lagged the window's real state by several seconds, so a picture shows what the window had last drawn.

*1. No camera, F12 pressed* (`nocamera.blend`). Two seconds after the key, the picture shows the message **in three places at once**, each with a white-on-red cross: along the bottom of the window on a red field, `No camera found in scene "Scene"`; in a floating box in the middle of the screen headed `Report: Error`, `No camera found in scene "Scene"`; and as an entry in the Info editor, `No camera found in scene "Sc` / `ene"` (wrapped by the editor's width). The picture taken twenty-one seconds after the key showed the same three, unchanged. The script's own lines afterwards, word for word:

```
[plan  25.00] WindowManager.reports holds 1 entries
[plan  25.00] REPORT type=ERROR message='No camera found in scene "Scene"'
[plan  26.00] render_stats fired 0 times, render_complete 0, render_cancel 0
```

No render window opened.

*2. No camera, render called from the script* (`bpy.ops.render.render('INVOKE_DEFAULT', write_still=True)` from a timer). The terminal showed `Error: No camera found in scene "Scene"`, and the script's own lines read:

```
[plan   2.01] render invoke {'write_still': True} raised RuntimeError('Error: No camera found in scene "Scene"\n')
[plan   5.00] WindowManager.reports holds 0 entries
```

The picture two seconds later shows **no message anywhere** — the Status Bar, the Info editor and the middle of the screen all empty of it.

*3. A broken template in the output path, Ctrl-F12 pressed* (`simple.blend` with `render.filepath = '//out_{blend_name.png'`). The terminal received, word for word:

```
00:04.649  operator         | ERROR Parse errors in path '//out_{blend_name.png':
                            | - Invalid or incomplete template expression '{blend_name.png'.
```

The picture three seconds later shows the message along the bottom of the window — `Parse errors in path '//out_{blend_name.png':- Invalid or incomplete template expression '{blend_name.png'.` — and in the Info editor over several lines; no floating box was in that picture. A second, small window had opened (160 by 128 points), photographed as plain grey. The reports list held one entry, `REPORT type=ERROR message="Parse errors in path '//out_{blend_name.png':\n- Invalid or incomplete template expression '{blend_name.png'."`.

*4. Saving switched off, Ctrl-F12 pressed* (`simple.blend` with `render.save_output = False`, output to `<work folder>/out/e34/so/f_`). The picture three seconds later shows the message in **three places** — along the bottom of the window on a red field, in a floating box headed `Report: Error`, and in the Info editor — reading `Render output disabled in Output properties and no active compositing File Output nodes`. The reports list held that one entry, type `ERROR`; the output folder was never made.

**Outside the interface.** The GPU, crash, output-method and command-line causes are not described as interface-only. The cancel button and Esc are interface controls. Outside the interface, cancelling is covered only by the inferred Ctrl-C point in 1.10. The Python exception in 1.11 applies whenever a script calls the render, including scripts run from the command line. The sources do not say how a failed render looks when Blender runs in the background. **Settled in part by the hand tests of 2026-10-17.** With no window: most failures print an `ERROR` line on standard output and end at exit 1 (a missing, unreadable or cut-short blend-file, a refused device or engine, a blocked or broken output path, an animation that cannot save a frame), but **a scene with no camera now ends at exit 0 having rendered nothing, with no render handler firing**, and so do saving switched off with nothing else arranged, a picture of no pixels, and a missing texture (which renders magenta). A render called from a script raises a `RuntimeError` instead, and the run ends at 1. With the window, a failure started by a key is shown in the Status Bar, a floating report box and the Info editor; one started from a script is raised into the script and shown nowhere. A simulated Esc did not stop a render.

**Operating system.** CUDA, OptiX, HIP and oneAPI are listed for Windows and Linux. Metal is listed for macOS on Apple Silicon. The fallback to main memory is named only for CUDA, OptiX, HIP and Metal, so oneAPI is not covered. The display-driver time limit is a Windows case. The compiler mismatch is a Linux case. Crash files differ by system: on Windows "test.blend will create a file called test.crash.txt"; on Linux "a file named blender.crash.txt is written to the /tmp directory"; on macOS "the macOS Crash Reporter shows a window with backtrace after some time, or when Blender is opened again" (https://docs.blender.org/manual/en/5.3/troubleshooting/crash.html, read 2026-10-11). For Ctrl-C, the API names Ctrl-Break as the Windows key in a related case (see 12.4). The other causes are not tied to a system. **Hand tests, 2026-10-17:** everything observed under this question was run on **the Creator's Mac (Apple M1 Pro, macOS 26.6.2) in Blender 5.3.0 Beta**, so it settles macOS on Apple silicon and nothing else. **Linux and Windows were not run in 5.3** — this sitting had no Linux or Windows machine — so they are untried, not untestable (the equipment this project can reach is set out in `Preparation/Blender Research/Method.md`); on Windows an edition would have to be named before anything was claimed.

---

## 2. Where does Blender say that a render failed, and what does it say there?

**Findings**

2.1 — **documented / observed** — *The console window (terminal).* The manual describes it as the place for messages, including internal errors. When Blender exits unexpectedly, those messages may give the cause.
> "The Console Window, also called a Terminal, is an operating system text window that displays messages about Blender’s operations, status, and internal errors."
> "If Blender exits unexpectedly, the messages may indicate the cause or error."
Manual — https://docs.blender.org/manual/en/5.3/advanced/command_line/index.html — read 2026-10-11.
Python errors are printed there in full, even when the interface shows nothing:
> "The error traceback is printed in full to the terminal which won’t always generate a report message in Blender’s user interface (depending on how the script is executed)."
API — https://docs.blender.org/api/5.3/info_tips_and_tricks.html — read 2026-10-11.
Shader-script compile errors also go there: "Compilation errors will be shown in the system console." Manual — https://docs.blender.org/manual/en/5.3/render/cycles/osl/index.html — read 2026-10-11. **observed:** the matching Cycles message is `report({'ERROR'}, "OSL script compilation failed, see console for errors")` (`osl.py`, line 314, read 2026-10-16; that this appears in the interface is inferred from report()). This is shader compilation, not a render as such.

2.2 — **documented / inferred** — *The status bar and the Info editor.* Short messages appear in the middle of the status bar and then disappear. The Info editor keeps a log of errors and warnings.
> "Informational messages or warnings, such as after saving a file. They disappear after a short time. Click them to show the full message in the Info Editor."
Manual — https://docs.blender.org/manual/en/5.3/interface/window_system/status_bar.html — read 2026-10-11.
> "The Info editor logs the executed operators as well as errors, warnings, and informational messages."
Manual — https://docs.blender.org/manual/en/5.3/editors/info_editor.html — read 2026-10-11.
**inferred:** render errors, such as those in 1.6 ("rendering operations will report errors"), end up in this log. The step: the manual uses the word "report" for both the render errors and the messages the status bar and Info editor show. It never links the two directly.

2.3 — **documented / observed** — *Messages from the render engine.* A render engine can send reports, and can set one error message that is shown after the render finishes.
> `report(type, message)` — "Report info, warning or error messages"
> `error_set(message)` — "Set error message displaying after the render is finished"
API — https://docs.blender.org/api/5.3/bpy.types.RenderEngine.html — read 2026-10-11.
**observed:** Cycles uses `report` for one case: `self.report({'ERROR'}, "OSL support disabled in this build")` (`__init__.py`, lines 108 and 115, read 2026-10-16).
The report kinds are listed in the API. They include "ERROR: Error." and "ERROR_OUT_OF_MEMORY: Out of Memory." API — https://docs.blender.org/api/5.3/bpy_types_enum_items/wm_report_items.html — read 2026-10-11.

2.4 — **documented** — *A Python script that started the render.* It receives a `RuntimeError` holding all the error report messages (quote in 1.11).

2.5 — **documented** — *A crash file, after a crash.* The file contains "a log of tools used up until the crash as well as some other debug information." Names and places are given in the Operating system line of question 1. Manual — https://docs.blender.org/manual/en/5.3/troubleshooting/crash.html — read 2026-10-11.

2.6 — **documented** — *The words themselves.* The manual gives these message texts:
- "Error: Out of memory" (GPU memory)
- "CUDA Error: Kernel compilation failed"
- "The NVIDIA OpenGL driver lost connection with the display driver"
- "Unsupported GNU version" is the section heading. The compiler line it quotes is "#error -- unsupported GNU version! gcc x.x and up are not supported!"

Source: https://docs.blender.org/manual/en/5.3/render/cycles/gpu_rendering.html — read 2026-10-11.
- "Shadow buffer full" (EEVEE) — https://docs.blender.org/manual/en/5.3/render/eevee/light_settings.html — read 2026-10-11.

For the missing-output case (1.6), the manual says only that "an error is raised" or "an error is reported". It does not give the wording.

2.7 — **documented** — *Signals for scripts, not messages.* The `render_cancel` hook ("on canceling a render job") and the `load_post_fail` hook ("on failure to load a new blend file (after)") let a script learn of these events. API — https://docs.blender.org/api/5.3/bpy.app.handlers.html — read 2026-10-11.

**Not covered.** The sources do not say where each GPU message appears: the status bar, the render window, the terminal, or more than one of these. They do not say where an `error_set` message is displayed. They do not give the wording for the missing-output error, or for a failure that is not a crash. They do not say what, if anything, the terminal prints when a render fails without crashing.

**Where I looked.** I read the same pages as question 1, plus the Info editor and the status bar. In the API I read `RenderEngine`, `Report`, `Operator.report`, the report and job type lists, `bpy.ops`, and the info_tips_and_tricks page. In Cycles' files I searched all `.py` files for `report(`, `print(`, `stderr`, `stdout`, `error` and `warning`. In the manual I searched for "error", "Error:", "system console", "console window" and "terminal".

**Observed by hand test, 2026-10-17 — the words of a refusal, and the stream they are on (hand-test entry 10).** On the Creator's Mac, Blender 5.3.0 Beta, with no window. The refusals quoted in full under question 12 all appeared on **standard output**, as log lines under `reports` (`ERROR Cannot read file …`, `ERROR File format is not supported in file …`, `ERROR Failed to read blend file '…': Missing DNA block`, `ERROR Found no Cycles device of the specified type`) or under `cycles` (`ERROR NONSENSE is not a valid Cycles device.`). Three things went to the **error stream** instead: `unknown argument, loading as file: -ba`, `unknown argument, loading as file: --no-such-flag`, and `Error: engine not found 'NOSUCHENGINE'`.

**Observed by hand test, 2026-10-17 — where these failures are reported with no window, and in what words (hand-test entry 11).** On the Creator's Mac, Blender 5.3.0 Beta, with no window; every command and every line is quoted under question 1. All of the messages were on **standard output**, under a log category, and the error stream was empty in every run but one. The words:

```
00:00.331  reports          | ERROR No camera found in scene "Scene"
```
```
00:00.440  image            | ERROR Couldn't create directory for file <work folder>/out/e11/blocker/frame_0001.png: File exists
00:00.440  reports          | ERROR Render error (File exists) cannot save: '<work folder>/out/e11/blocker/frame_0001.png'
```
```
00:00.323  reports          | ERROR Parse errors in path '<work folder>/out/e11/tmpl/{blend_name.png':
                            | - Invalid or incomplete template expression '{blend_name.png'.
```
```
00:00.321  reports          | ERROR Render output disabled in Output properties and no active compositing File Output nodes
```

The no-camera message, with no window, reads `No camera found in scene "Scene"`. The one run that wrote to the error stream was the render called from Python with saving switched off: there the message reached standard output as a plain `Error: Render output disabled in Output properties and no active compositing File Output nodes`, with no time and no category, and the error stream carried the Python traceback ending `RuntimeError: Error: Render output disabled in Output properties and no active compositing File Output nodes` and then `Error: script failed, expr: '…', exiting.`

**Observed by hand test, 2026-10-17 — an animation whose output folder stops being writable part-way (hand-test entry 12).** On the Creator's Mac, Blender 5.3.0 Beta, set up as described at the top of this document, on `partway.blend` (2000 samples, 320 by 240, frames 1 to 6, about seven seconds a frame). One command started the render in the background with `-o "<work folder>/out/e12/out_53/frame_" -F PNG -x 1 -a`, waited a fixed twelve seconds, listed the output folder — it held `frame_0001.png` — renamed the folder away to `out_53_moved`, and put a one-byte plain file named `out_53` in its place; then it waited for the render to end. The render ended at **exit 1**. The error stream was empty. Standard output held 104 lines; every line naming a frame, a save, an error or a time, with its line number, word for word:

```
3:00:00.330  render           | Rendering frame 1
51:00:07.099  render           | Saved: '<work folder>/out/e12/out_53/frame_0001.png'
52:00:07.099  render           | Time: 00:06.76 (Saving: 00:00.06)
53:00:07.099  render           | Rendering frame 2
100:00:13.800  image            | ERROR Couldn't create directory for file <work folder>/out/e12/out_53/frame_0002.png: File exists
101:00:13.800  reports          | ERROR Render error (File exists) cannot save: '<work folder>/out/e12/out_53/frame_0002.png'
102:00:13.800  render           | Time: 00:06.70
```

and the run's last lines were `Time: 00:06.70`, a blank line and `Blender quit`. **No `Rendering frame 3` line, or any line for frames 3 to 6, was printed** — the animation stopped at the frame it could not save. The renamed folder held one file, `frame_0001.png` (65,499 bytes); no part-written file was left anywhere.

**Observed by hand test, 2026-10-17 — which handlers fire when a render fails, what the reports list holds, what a script's own render call gets, and where a log file puts a failure (hand-test entry 13).** On the Creator's Mac, Blender 5.3.0 Beta, set up as described at the top of this document, with no window.

*Which handlers fire.* A start-up script attached a printing handler to each of `render_init`, `render_pre`, `render_post`, `render_write`, `render_complete` and `render_cancel` (and a silent one to `render_stats`), printed `PROBE INSTALLED`, and would have printed `HANDLER MISSING:` for any that did not exist; none was missing. Six runs, each `"$B53" -b <file> --python handlers_probe.py -o <path> -F PNG -f 1` (or `-x 1 -a`). Every line each printed from the probe, and every `ERROR` or `WARNING` line, in order:

```
— good render (simple.blend), exit 0
PROBE INSTALLED
HANDLER FIRED: render_init args: (bpy.data.scenes['Scene'], None)
HANDLER FIRED: render_pre args: (bpy.data.scenes['Scene'], None)
HANDLER FIRED: render_post args: (bpy.data.scenes['Scene'], None)
HANDLER FIRED: render_write args: (bpy.data.scenes['Scene'], None)
HANDLER FIRED: render_complete args: (bpy.data.scenes['Scene'], None)
— no camera (nocamera.blend), exit 0
PROBE INSTALLED
00:00.323  reports          | ERROR No camera found in scene "Scene"
— output path blocked by a plain file, exit 1
PROBE INSTALLED
HANDLER FIRED: render_init args: (bpy.data.scenes['Scene'], None)
HANDLER FIRED: render_pre args: (bpy.data.scenes['Scene'], None)
00:00.453  image            | ERROR Couldn't create directory for file <work folder>/out/e13/blocker/frame_0001.png: File exists
00:00.453  reports          | ERROR Render error (File exists) cannot save: '<work folder>/out/e13/blocker/frame_0001.png'
HANDLER FIRED: render_cancel args: (bpy.data.scenes['Scene'], None)
— broken template in the output path, -x 1 -a, exit 1
PROBE INSTALLED
HANDLER FIRED: render_init args: (bpy.data.scenes['Scene'], None)
00:00.325  reports          | ERROR Parse errors in path '<work folder>/out/e13/tmpl/{blend_name.png':
HANDLER FIRED: render_cancel args: (bpy.data.scenes['Scene'], None)
— missing image texture (missing.blend, gone.png moved aside), exit 0
PROBE INSTALLED
HANDLER FIRED: render_init args: (bpy.data.scenes['Scene'], None)
HANDLER FIRED: render_pre args: (bpy.data.scenes['Scene'], None)
00:00.334  cycles           | WARNING Image file <scene folder>/gone.png does not exist.
00:00.335  cycles           | ERROR Failed to load 1 image files
HANDLER FIRED: render_post args: (bpy.data.scenes['Scene'], None)
HANDLER FIRED: render_write args: (bpy.data.scenes['Scene'], None)
HANDLER FIRED: render_complete args: (bpy.data.scenes['Scene'], None)
— saving switched off by --python-expr, -x 1 -a, exit 0
PROBE INSTALLED
00:00.325  reports          | ERROR Render output disabled in Output properties and no active compositing File Output nodes
```

Every error stream was empty. So **on a scene with no camera not one handler fired** — not even `render_init` — and the same is true of saving switched off. A blocked save fired `render_init`, `render_pre` and `render_cancel`; a broken template fired `render_init` and `render_cancel` only; a missing texture, which succeeds, fired the full set a good render fires.

*The reports list after a failure with no window.* `"$B53" -b <scene folder>/nocamera.blend -o "<work folder>/out/e13/nc2_" -F PNG -f 1 --python reports.py`, the script printing `len(bpy.context.window_manager.reports)` and each entry. **Exit 0**, error stream empty, and standard output, whole:

```
00:00.317  blend            | Read blend: "<scene folder>/nocamera.blend"
00:00.321  reports          | ERROR No camera found in scene "Scene"
Blender 5.3.0 Beta (hash 5d8151335b6b built 2026-10-16 01:36:01)
REPORTS COUNT: 0

Blender quit
```

*A render called from a script.* A start-up script called `bpy.ops.render.render(write_still=True)` inside a `try` and printed what came back. On `simple.blend` it printed `RENDER RETURNED: {'FINISHED'}` and the run ended at exit 0. On `nocamera.blend` the run ended at **exit 1**, the error stream was empty, and standard output, whole:

```
00:00.322  blend            | Read blend: "<scene folder>/nocamera.blend"
Error: Cannot render, no camera
RENDER RAISED: RuntimeError Error: Cannot render, no camera
Blender 5.3.0 Beta (hash 5d8151335b6b built 2026-10-16 01:36:01)

Blender quit
```

So the script's call **raised a `RuntimeError` carrying `Error: Cannot render, no camera`**, and the run ended at 1 although the script caught it — while the same file rendered with `-f 1` says `No camera found in scene "Scene"` and ends at 0 (hand-test entry 11).

*A log file on a failure.* `"$B53" -b <scene folder>/nocamera.blend --log-file "<work folder>/logs/e13j.log" -o "<work folder>/out/e13/lf_" -F PNG -f 1`: **exit 0**, error stream empty; standard output held only the version banner, a blank line and `Blender quit`, and the file held, whole:

```
00:00.320  blend            | Read blend: "<scene folder>/nocamera.blend"
00:00.324  reports          | ERROR No camera found in scene "Scene"
```

**Observed by hand test, 2026-10-17 — a crash on purpose, and where the crash file goes on macOS (hand-test entry 14).** On the Creator's Mac, Blender 5.3.0 Beta, set up as described at the top of this document — so Blender's temporary folder for these runs was `<work folder>/tmp`, given with `TMPDIR`. Blender was made to crash with a start-up script whose whole content is `import ctypes`, `print("ABOUT TO CRASH")` and `ctypes.string_at(0)`.

*With a blend-file loaded* — `"$B53" -b <scene folder>/simple.blend --python crash.py`: **exit 139**. The error stream was empty. Standard output, whole:

```
00:00.318  blend            | Read blend: "<scene folder>/simple.blend"
Blender 5.3.0 Beta (hash 5d8151335b6b built 2026-10-16 01:36:01)
Writing: <work folder>/tmp/simple.crash.txt
```

The `print` that ran just before the crash never reached standard output. The file `<work folder>/tmp/simple.crash.txt` existed afterwards, 28 lines, its headings:

```
# Blender 5.3.0, Commit date: 2026-10-16 00:30, Hash 5d8151335b6b
# backtrace
# Python backtrace
```

*With no blend-file loaded* — `"$B53" -b --factory-startup --python crash.py`: **exit 139**, error stream empty, standard output, whole:

```
Blender 5.3.0 Beta (hash 5d8151335b6b built 2026-10-16 01:36:01)
Writing: <work folder>/tmp/blender.crash.txt
```

and `<work folder>/tmp/blender.crash.txt` was written. **So on macOS the crash file goes to Blender's temporary folder, named after the blend-file when one is loaded and `blender.crash.txt` when none is.** The system's own reports folder was listed (Blender's names only) before and after the two runs: one new file, `Blender-2026-10-17-045619.ips`, appeared in `~/Library/Logs/DiagnosticReports/`; which of the two runs made it, and whether the system showed a window about it, was not established.

**Observed by hand test, 2026-10-17 — how a failure is announced with Blender's window open (hand-test entry 34).** On the Creator's Mac, Blender 5.3.0 Beta, set up as described at the top of this document, started with its window from a terminal. In every run a start-up script first turned the Outliner into an Info editor with its filters for debug, information, operator, warning and error messages on. Key presses were sent with `Window.event_simulate`, which needs `--enable-event-simulate`, the pointer over the 3D viewport; F12 and Ctrl-F12 are the keys the running program binds to rendering a still and an animation (hand-test entry 38). Every run ended at exit 0 with an empty error stream. **One thing about every picture:** in hand-test entries 30 and 31 the pictures lagged the window's real state by several seconds, so a picture shows what the window had last drawn.

*1. No camera, F12 pressed* (`nocamera.blend`). Two seconds after the key, the picture shows the message **in three places at once**, each with a white-on-red cross: along the bottom of the window on a red field, `No camera found in scene "Scene"`; in a floating box in the middle of the screen headed `Report: Error`, `No camera found in scene "Scene"`; and as an entry in the Info editor, `No camera found in scene "Sc` / `ene"` (wrapped by the editor's width). The picture taken twenty-one seconds after the key showed the same three, unchanged. The script's own lines afterwards, word for word:

```
[plan  25.00] WindowManager.reports holds 1 entries
[plan  25.00] REPORT type=ERROR message='No camera found in scene "Scene"'
[plan  26.00] render_stats fired 0 times, render_complete 0, render_cancel 0
```

No render window opened.

*2. No camera, render called from the script* (`bpy.ops.render.render('INVOKE_DEFAULT', write_still=True)` from a timer). The terminal showed `Error: No camera found in scene "Scene"`, and the script's own lines read:

```
[plan   2.01] render invoke {'write_still': True} raised RuntimeError('Error: No camera found in scene "Scene"\n')
[plan   5.00] WindowManager.reports holds 0 entries
```

The picture two seconds later shows **no message anywhere** — the Status Bar, the Info editor and the middle of the screen all empty of it.

*3. A broken template in the output path, Ctrl-F12 pressed* (`simple.blend` with `render.filepath = '//out_{blend_name.png'`). The terminal received, word for word:

```
00:04.649  operator         | ERROR Parse errors in path '//out_{blend_name.png':
                            | - Invalid or incomplete template expression '{blend_name.png'.
```

The picture three seconds later shows the message along the bottom of the window — `Parse errors in path '//out_{blend_name.png':- Invalid or incomplete template expression '{blend_name.png'.` — and in the Info editor over several lines; no floating box was in that picture. A second, small window had opened (160 by 128 points), photographed as plain grey. The reports list held one entry, `REPORT type=ERROR message="Parse errors in path '//out_{blend_name.png':\n- Invalid or incomplete template expression '{blend_name.png'."`.

*4. Saving switched off, Ctrl-F12 pressed* (`simple.blend` with `render.save_output = False`, output to `<work folder>/out/e34/so/f_`). The picture three seconds later shows the message in **three places** — along the bottom of the window on a red field, in a floating box headed `Report: Error`, and in the Info editor — reading `Render output disabled in Output properties and no active compositing File Output nodes`. The reports list held that one entry, type `ERROR`; the output folder was never made.

**Observed by hand test, 2026-10-17 — missing files with Blender's window open (hand-test entry 35).** On the Creator's Mac, Blender 5.3.0 Beta, set up as described at the top of this document, started with its window from a terminal. Each file was moved aside just before its run. A start-up script turned the Outliner into an Info editor with its filters on, called `bpy.ops.render.render('INVOKE_DEFAULT', write_still=True)` two seconds in, photographed every window six seconds later, read `WindowManager.reports`, and quit. Every run ended at exit 0 with an empty error stream and wrote its picture.

*A missing image texture* (`tex_missing.blend`). The terminal still received both Cycles lines, word for word:

```
00:02.614  cycles           | WARNING Image file <scene folder>/missing_tex.png does not exist.
00:02.645  cycles           | ERROR Failed to load 1 image files
```

The script read `WindowManager.reports holds 0 entries`. In the picture **the Info editor was empty and the Status Bar carried no message**; the small render window (160 by 128 points) was photographed as plain grey.

*A missing linked library* (`lib_missing.blend`). The terminal received the five lines as the file was opened, before the script ran — note the category `blend.readfile` on three of them, where the same file read with no window put all five under `reports` (hand-test entry 21):

```
00:00.555  blend.readfile   | Read library: '<scene folder>/lib.blend', '//lib.blend', parent '<direct>'
00:00.555  reports          | WARNING Unable to open '<scene folder>/lib.blend': No such file or directory
00:00.555  blend.readfile   | Cannot find lib '<scene folder>/lib.blend'
00:00.556  blend.readfile   | LIB: Collection: 'LibColl' missing from '<scene folder>/lib.blend', parent '<direct>'
00:00.560  reports          | WARNING 1 libraries and 1 linked data-blocks are missing (including 0 ObjectData), please check the Info and Outliner editors for details
```

and the reports list **held all five**:

```
[plan   9.00] REPORT type=INFO message="Read library: '<scene folder>/lib.blend', '//lib.blend', parent '<direct>'"
[plan   9.00] REPORT type=WARNING message="Unable to open '<scene folder>/lib.blend': No such file or directory"
[plan   9.00] REPORT type=INFO message="Cannot find lib '<scene folder>/lib.blend'"
[plan   9.00] REPORT type=INFO message="LIB: Collection: 'LibColl' missing from '<scene folder>/lib.blend', parent '<direct>'"
[plan   9.00] REPORT type=WARNING message='1 libraries and 1 linked data-blocks are missing (including 0 ObjectData), please check the Info and Outliner editors for details'
```

In the picture the Info editor shows the entries, the last with a warning triangle — `1 libraries and 1 linked data-blocks are missing (including 0 ObjectData), please check the Info and Outliner editors for details` — and the Status Bar carries the same warning shortened to `1 libraries and 1 linked d...utliner editors for details`.

*Missing sequencer footage* (`seq_missing.blend`), never before opened in a window. Nothing about it reached the terminal; the reports list held **0 entries**; the picture shows an empty Info editor and no message in the Status Bar.

**Observed by hand test, 2026-10-17 — what a render engine's own reporting calls produce, with no window and with one (hand-test entry 37).** On the Creator's Mac, Blender 5.3.0 Beta, set up as described at the top of this document, on `good.blend`. A start-up script registered a tiny render engine, `SMOOZY_PROBE`, and set the scene to use it. Its `render` method printed a line beside each call and called, once each: `update_stats("PROBE STATS TEXT", "PROBE INFO TEXT")`, `update_progress(0.5)`, `update_memory_stats(memory_used=12.0, memory_peak=34.0)`, `report({'WARNING'}, "PROBE REPORT MESSAGE")`; it then wrote a plain grey picture, and last called `error_set("PROBE ERROR_SET MESSAGE")`. **Every call returned without complaint, with no window and with one.**

*With no window* — `"$B53" -b <scene folder>/good.blend --python engine_probe.py -o "<work folder>/out/e37/bg_" -F PNG -f 1`: exit 0, picture written, error stream empty. Standard output, whole (lines beginning `PROBE` are the script's own):

```
00:00.323  blend            | Read blend: "<scene folder>/good.blend"
PROBE ENGINE REGISTERED, scene engine = SMOOZY_PROBE
00:00.329  render           | Rendering single frame
00:00.329  render           | Rendering frame 1
00:00.329  render           | Start rendering: Scene, ViewLayer
00:00.329  render           | Engine: Smoozy Probe
PROBE: calling update_stats
00:00.329  render           | Fra: 1 | PROBE INFO TEXT
Blender 5.3.0 Beta (hash 5d8151335b6b built 2026-10-16 01:36:01)
PROBE: calling update_progress(0.5)
PROBE: calling update_memory_stats(memory_used=12.0, memory_peak=34.0)
PROBE: calling report({'WARNING'}, 'PROBE REPORT MESSAGE')
00:00.329  reports          | WARNING PROBE REPORT MESSAGE
PROBE: calling error_set('PROBE ERROR_SET MESSAGE')
PROBE: render() returning
00:00.390  render           | Saved: '<work folder>/out/e37/bg_0001.png'
00:00.390  render           | Time: 00:00.06 (Saving: 00:00.06)

Blender quit
```

So `update_stats` put its **second** text on a `render` line (`Fra: 1 | PROBE INFO TEXT`) and its first nowhere; `update_progress` and `update_memory_stats` printed nothing; `report` printed `reports | WARNING PROBE REPORT MESSAGE`; and **the `error_set` text appeared nowhere in the output**, and the run still ended at 0.

*With the window* — started from a terminal with the window, the render called with `bpy.ops.render.render('INVOKE_DEFAULT', write_still=True)` two seconds in, the engine waiting five seconds before its `error_set`, pictures at 5 and 11 seconds, the reports list read at both. Exit 0, error stream empty. Standard output, whole:

```
00:00.542  blend            | Read blend: "<scene folder>/good.blend"
PROBE ENGINE REGISTERED, scene engine = SMOOZY_PROBE
PROBE: Outliner turned into an Info editor
PROBE: INVOKE returned {'RUNNING_MODAL'}
PROBE: calling update_stats
PROBE: calling update_progress(0.5)
PROBE: calling update_memory_stats(memory_used=12.0, memory_peak=34.0)
PROBE: calling report({'WARNING'}, 'PROBE REPORT MESSAGE')
PROBE: shot of window 0 saved as e37_probe_during_w0.png
PROBE: shot of window 1 saved as e37_probe_during_w1.png
PROBE: WindowManager.reports holds 1 entries
PROBE: REPORT type=WARNING message='PROBE REPORT MESSAGE'
PROBE: calling error_set('PROBE ERROR_SET MESSAGE')
PROBE: render() returning
00:07.602  render           | Saved: '<work folder>/out/e37/win_.png'
PROBE: shot of window 0 saved as e37_probe_after_w0.png
PROBE: shot of window 1 saved as e37_probe_after_w1.png
PROBE: WindowManager.reports holds 1 entries
PROBE: REPORT type=WARNING message='PROBE REPORT MESSAGE'
00:13.572  blend            | Saved session recovery to "<work folder>/tmp/quit.blend"

Blender quit
```

The picture during the render shows the Status Bar reading `PROBE REPORT MESSAGE` with a warning triangle, then `Rendering...` with the bar at **`50%`** — the `update_progress(0.5)` — and the Info editor listing `PROBE REPORT MESSAGE` with a warning triangle. The picture after the render shows the Status Bar's warning shortened to `PROBE REPORT MES...` and the same Info-editor entry. The render window's own status line read `Frame:1 | Time:00:05.00 | M` and ran past the window's edge, where it is cut off, so whether the `error_set` text was drawn further along that line is **not established**. It was not on the terminal, not in the reports list, and not in the Status Bar or the Info editor in either picture.

**Outside the interface.** The status bar and the Info editor are part of the interface. The terminal is the documented place when Blender is started from a command line. The API pages state that Python tracebacks go to the terminal. Crash files are written whether or not the interface is running, but not when Blender runs as a Python module (quote in 1.10). The sources do not say whether engine reports and `error_set` messages reach the terminal when Blender runs without its interface. **Settled in part by the hand tests of 2026-10-17.** With no window every render failure tried was reported on **standard output**, under `reports`, `image`, `cycles` or `object.modifier`; the error stream carried only complaints about the command line, Python tracebacks, `maketx`'s own `Error:` line and the Alembic library's lines. `WindowManager.reports` was empty after a failure with no window. With the window, a key-started failure was announced in up to three places and filled the reports list with one entry; a library missing at load filled it with five; a missing texture or sequencer picture put nothing in the window at all. A render engine's `error_set` text was not found anywhere looked at.

**Operating system.** On Windows the console starts as a separate window and is then hidden: "To display the console again, go to Window ‣ Toggle System Console." (https://docs.blender.org/manual/en/5.3/advanced/command_line/launch/windows.html, read 2026-10-11). On macOS, output reaches a terminal only when the program inside the app bundle is started directly (see 4.1). Crash file places differ by system, as in question 1. The rest is not tied to a system. **Hand tests, 2026-10-17:** everything observed under this question was run on **the Creator's Mac (Apple M1 Pro, macOS 26.6.2) in Blender 5.3.0 Beta**, so it settles macOS on Apple silicon and nothing else. **Linux and Windows were not run in 5.3** — this sitting had no Linux or Windows machine — so they are untried, not untestable (the equipment this project can reach is set out in `Preparation/Blender Research/Method.md`); on Windows an edition would have to be named before anything was claimed.

---

## 3. What does Blender report about how far through a piece of work it has got?

**Findings**

3.1 — **documented** — *The status bar progress bar.* The middle of the status bar shows how far the current task has got, for rendering and baking among others. Hovering over it shows a time estimate. It has a cancel button.
> "Shows the progress of the currently running task (such as rendering or baking). Hovering the mouse pointer over the progress bar will display a time estimate. The task can be aborted by clicking the cancel button ()."
Manual — https://docs.blender.org/manual/en/5.3/interface/window_system/status_bar.html — read 2026-10-11.
**documented (picture):** `_images/interface_window-system_status-bar_ui-center.png` on that page shows a task named "Sequence Render", a bar showing "94%", and an "X" button. That is all the picture shows.

3.2 — **documented** — *What a render engine passes to Blender.* An engine reports progress as a fraction from 0 to 1. It can also set status text and memory figures.
> `update_progress(progress)` — "Update progress percentage of render"; "progress (float) – Percentage of render that’s done (in [0, 1])"
> `update_stats(stats, info)` — "Update and signal to redraw render status text"
API — https://docs.blender.org/api/5.3/bpy.types.RenderEngine.html — read 2026-10-11.

3.3 — **documented** — *Render statistics that get printed.* A script hook receives the printed render statistics as a string. The string contains the current frame, the current sample, the render time and the saving time.
> `render_stats` — "on printing render statistics. Accepts one argument: the render progress as a string containing current frame, current sample, render time and saving time."
API — https://docs.blender.org/api/5.3/bpy.app.handlers.html — read 2026-10-11.

3.4 — **documented** — *Stage hooks for scripts.* These mark stages of a render job rather than a percentage: `render_init` ("on initialization of a render job"), `render_pre` ("on render (before)"), `render_post` ("on render (after)"), `render_write` ("on writing a render frame (directly after the frame is written)"), `render_complete` ("on completion of render job") and `render_cancel`. Same page, read 2026-10-11.

3.5 — **documented** — *Progress feedback for scripts.* Scripts have their own progress calls: `progress_begin(min, max)` "Start progress report", `progress_update(value)` "Update the progress feedback", and `progress_end()` "Terminate progress report". API — https://docs.blender.org/api/5.3/bpy.types.WindowManager.html — read 2026-10-11. This is a general facility for scripts, not a render report.

**Not covered.** The sources do not give the exact status text shown during a render. They do not say whether the progress bar counts one frame or a whole animation, or how it handles several view layers. They do not give the format or wording of the printed statistics in 3.3, or say where they are printed.

**Where I looked.** I searched the manual and the API for "progress", "percent", "status", "stats", "statistics", "remaining", "estimate" and "elapsed". I opened the three status bar pictures. In Cycles' files I searched for "progress", "stats", "printf", "stderr" and "remaining". The kernel and utility files under `source/` only print values when debugging. The release notes have nothing on progress.

**Observed by hand test, 2026-10-17 — what a render prints about its progress with no window (hand-test entry 1).** On the Creator's Mac, Blender 5.3.0 Beta, set up as described at the top of this document: `"$B53" -b <scene folder>/base.blend -o "<work folder>/out/e01/e01_" -F PNG -f 1`, Cycles on the processor, 16 samples. The whole of that run's output is quoted under question 8. The lines that carry progress, word for word:

```
00:00.327  render           | Fra: 1 | Mem: 0M | Synchronizing object | Cube
00:00.329  render           | Fra: 1 | Mem: 1M | Sample 0/16
00:00.330  render           | Fra: 1 | Remaining: 00:00.01 | Mem: 2M | Sample 1/16
00:00.450  render           | Fra: 1 | Mem: 2M | Sample 16/16
00:00.450  render           | Fra: 1 | Mem: 2M | Finished
00:00.511  render           | Time: 00:00.18 (Saving: 00:00.06)
```

So with no window the progress report is a printed line under the `render` category: an elapsed time, a frame number (`Fra: 1`), sometimes a `Remaining:` time, a memory figure (`Mem:`), then either a stage name — forty such stage lines run between the first line and `Sample 0/16` — or a sample count against the maximum. No percentage appears anywhere in the run.

**Observed by hand test, 2026-10-17 — a three-frame animation (hand-test entry 3).** On the Creator's Mac, Blender 5.3.0 Beta, set up as described at the top of this document: `"$B53" -b <scene folder>/base.blend -o "<work folder>/out/e03/e03_" -F PNG -x 1 -a > out.txt 2> err.txt`. It printed `exit 0`, wrote `e03_0001.png`, `e03_0002.png` and `e03_0003.png`, left the error stream at 0 bytes, and put **150 lines** on standard output. Every line naming the range, a frame, a `Remaining:` figure, a saved file or a time, with its line number in the output, word for word:

```
2:00:00.325  render           | Rendering animation (frames 1..3)
3:00:00.326  render           | Rendering frame 1
48:00:00.332  render           | Fra: 1 | Remaining: 00:00.01 | Mem: 2M | Sample 1/16
51:00:00.486  render           | Saved: '<work folder>/out/e03/e03_0001.png'
52:00:00.486  render           | Time: 00:00.16 (Saving: 00:00.06)
53:00:00.486  render           | Rendering frame 2
96:00:00.489  render           | Fra: 2 | Remaining: 00:00.01 | Mem: 2M | Sample 1/16
99:00:00.572  render           | Saved: '<work folder>/out/e03/e03_0002.png'
100:00:00.572  render           | Time: 00:00.08 (Saving: 00:00.00)
101:00:00.572  render           | Rendering frame 3
144:00:00.575  render           | Fra: 3 | Remaining: 00:00.01 | Mem: 2M | Sample 1/16
147:00:00.657  render           | Saved: '<work folder>/out/e03/e03_0003.png'
148:00:00.657  render           | Time: 00:00.08 (Saving: 00:00.00)
```

Each frame ran its own `Start rendering: Scene, ViewLayer`, `Engine: Cycles`, its stage lines and its own three sample lines, `Sample 0/16`, `Sample 1/16` and `Sample 16/16`, carrying its own frame number (`Fra: 1`, `Fra: 2`, `Fra: 3`). **No line anywhere gave a figure for the range as a whole**, and the `Remaining:` figure appeared once in each frame. The last three lines of the run were `Time: 00:00.08 (Saving: 00:00.00)`, a blank line and `Blender quit`.

**Observed by hand test, 2026-10-17 — what the `render_stats` handler is handed, with no window (hand-test entry 4).** On the Creator's Mac, Blender 5.3.0 Beta, set up as described at the top of this document. A start-up script attached one handler to `bpy.app.handlers.render_stats` that printed `RENDER_STATS:` and the `repr` of its arguments on every call, and one to `render_complete` that printed the count. Commands: `"$B53" -b <scene folder>/base.blend --python stats_probe.py -o "<work folder>/out/e04/e04a_" -F PNG -f 1` and the same with `-x 1 -a` in place of `-f 1`. Both printed `exit 0`; both error streams were 0 bytes. **It fired 45 times for one frame and 133 times for three**, and the run's own count line read `RENDER_STATS FIRED 45 TIMES`. The first string, the sampling strings and the last three of the one-frame run, word for word:

```
RENDER_STATS: ('Mem: 0M | Synchronizing object | Cube', None)
RENDER_STATS: ('Mem: 1M | Sample 0/16', None)
RENDER_STATS: ('Remaining: 00:00.01 | Mem: 2M | Sample 1/16', None)
RENDER_STATS: ('Mem: 2M | Sample 16/16', None)
RENDER_STATS: ('Mem: 2M | Finished', None)
RENDER_STATS: ('Time: 00:00.15 (Saving: 00:00.06)', None)
```

In the three-frame run the `Remaining:` string arrived once in each frame, word for word the same each time, each just after the printed line it matches:

```
00:00.329  render           | Fra: 1 | Remaining: 00:00.01 | Mem: 2M | Sample 1/16
RENDER_STATS: ('Remaining: 00:00.01 | Mem: 2M | Sample 1/16', None)
00:00.487  render           | Fra: 2 | Remaining: 00:00.01 | Mem: 2M | Sample 1/16
RENDER_STATS: ('Remaining: 00:00.01 | Mem: 2M | Sample 1/16', None)
00:00.576  render           | Fra: 3 | Remaining: 00:00.01 | Mem: 2M | Sample 1/16
RENDER_STATS: ('Remaining: 00:00.01 | Mem: 2M | Sample 1/16', None)
```

So the handler is handed two arguments, a string and `None`; the string is the message part of the printed line, with the separator ` | `; **none of the 45 strings carries a frame number** (the printed lines do, as `Fra: 1`); the current sample and its maximum arrive as `Sample <n>/<max>` while sampling runs; and the render time and saving time arrive together in the last string only, as `Time: … (Saving: …)`.

**Observed by hand test, 2026-10-17 — EEVEE and Workbench, and the engine's name (hand-test entry 6).** On the Creator's Mac, Blender 5.3.0 Beta, set up as described at the top of this document, on `base.blend` (whose own engine is Cycles at 16 samples).

*The names.* A start-up script set `bpy.context.scene.render.engine` to four names in turn and printed what happened. The whole of what it printed:

```
ENGINE ACCEPTED: BLENDER_EEVEE
ENGINE RAISED: BLENDER_EEVEE_NEXT TypeError bpy_struct: item.attr = val: enum "BLENDER_EEVEE_NEXT" not found in ('BLENDER_EEVEE', 'BLENDER_WORKBENCH', 'CYCLES')
ENGINE ACCEPTED: BLENDER_WORKBENCH
ENGINE ACCEPTED: CYCLES
```

*EEVEE.* `"$B53" -b <scene folder>/base.blend -E BLENDER_EEVEE -o "<work folder>/out/e06/eevee_" -F PNG -f 1` printed `exit 0`, wrote its picture, left the error stream at 0 bytes, and printed 14 lines. The whole of them:

```
00:00.321  blend            | Read blend: "<scene folder>/base.blend"
00:00.327  render           | Rendering single frame
00:00.327  render           | Rendering frame 1
00:00.329  render           | Start rendering: Scene, ViewLayer
00:00.329  render           | Engine: EEVEE
00:00.472  render           | Fra: 1 | Rendering 1 / 64 samples
Blender 5.3.0 Beta (hash 5d8151335b6b built 2026-10-16 01:36:01)
00:00.683  render           | Fra: 1 | Rendering 25 / 64 samples
00:00.838  render           | Fra: 1 | Rendering 50 / 64 samples
00:00.925  render           | Fra: 1 | Rendering 64 / 64 samples
00:01.017  render           | Saved: '<work folder>/out/e06/eevee_0001.png'
00:01.017  render           | Time: 00:00.68 (Saving: 00:00.07)

Blender quit
```

So EEVEE's progress line is shaped `Fra: 1 | Rendering <n> / <max> samples`, printed at 1, 25, 50 and 64; it carries **no memory figure, no `Remaining:` field and no stage lines**, and it counted to 64 although the scene asks Cycles for 16, printing nothing about the difference.

*Workbench.* `"$B53" -b <scene folder>/base.blend -E BLENDER_WORKBENCH -o "<work folder>/out/e06/wb_" -F PNG -f 1` printed `exit 0`, wrote its picture, left the error stream at 0 bytes, and printed 10 lines. The whole of them:

```
00:00.334  blend            | Read blend: "<scene folder>/base.blend"
00:00.339  render           | Rendering single frame
00:00.339  render           | Rendering frame 1
00:00.339  render           | Start rendering: Scene, ViewLayer
00:00.339  render           | Engine: Workbench
00:00.507  render           | Saved: '<work folder>/out/e06/wb_0001.png'
00:00.507  render           | Time: 00:00.16 (Saving: 00:00.06)
Blender 5.3.0 Beta (hash 5d8151335b6b built 2026-10-16 01:36:01)

Blender quit
```

**Workbench printed no progress line and no sample line at all.** Neither engine mentioned a tile.

**Observed by hand test, 2026-10-17 — with Blender's window open: a render started the way F12 starts it, photographed while it ran (hand-test entry 30).** On the Creator's Mac, Blender 5.3.0 Beta, set up as described at the top of this document, started with its window from a terminal on `scene14.blend` limited to frames 1 and 2 (`-s 1 -e 2`, Cycles on the processor, 900 samples, 720 by 405), with output to `<work folder>/out/e30/f_`. A start-up script ran a timed plan: two seconds in, it called `bpy.ops.render.render('INVOKE_DEFAULT', animation=True)` — the call the F12 family of keys is bound to — and then photographed every open window with `bpy.ops.screen.screenshot` at 6, 16, 30, 40 and 52 seconds, counted the `render_stats` calls at 75 and quit at 77. It ended at exit 0 with an empty error stream. Its standard output, whole (lines beginning `[plan …]` are the script's own, with its own seconds):

```
00:00.563  blend            | Read blend: "<scene folder>/scene14.blend"
[plan   0.00] plan installed: 2 invoke anim|6 shot|16 shot|30 shot|40 shot|52 shot|75 stats|76 files out/e30|77 quit
[plan   2.03] render invoke {'animation': True} returned {'RUNNING_MODAL'}
[plan   6.16] shot of window 0 (1512 x 862) saved as e30_006.0_w0.png
[plan   6.18] shot of window 1 (390 x 262) saved as e30_006.2_w1.png
00:15.803  render           | Saved: '<work folder>/out/e30/f_0001.png'
[plan  16.14] shot of window 0 (1512 x 862) saved as e30_016.0_w0.png
[plan  16.16] shot of window 1 (390 x 262) saved as e30_016.1_w1.png
00:29.037  render           | Saved: '<work folder>/out/e30/f_0002.png'
[plan  30.14] shot of window 0 (1512 x 862) saved as e30_030.0_w0.png
[plan  30.15] shot of window 1 (390 x 262) saved as e30_030.1_w1.png
[plan  40.14] shot of window 0 (1512 x 862) saved as e30_040.0_w0.png
[plan  40.16] shot of window 1 (390 x 262) saved as e30_040.1_w1.png
[plan  52.15] shot of window 0 (1512 x 862) saved as e30_052.0_w0.png
[plan  52.16] shot of window 1 (390 x 262) saved as e30_052.1_w1.png
[plan  75.00] render_stats fired 102 times, render_complete 1, render_cancel 0
[plan  75.00] FIRST=('Mem: 0M | Synchronizing object | Cube', None)
[plan  75.00] LAST=('Time: 00:13.23 (Saving: 00:00.01)', None)
[plan  76.00] files in out/e30: ['f_0001.png', 'f_0002.png']
[plan  77.01] calling quit_blender()
[plan  77.01] still here after quit_blender()
01:17.596  blend            | Saved session recovery to "<work folder>/tmp/quit.blend"

Blender quit
```

So **the operator returned `{'RUNNING_MODAL'}` at once and the render went on afterwards**; the only render lines that reached the terminal were the two `Saved:` lines; the handler fired 102 times over two frames; and a second window, 390 by 262 points, was open during and after the render.

*The Status Bar.* In the pictures of the main window there is **one bar**: a small camera icon, the word **`Rendering...`**, a blue bar with a **whole-number percentage** written in it, and an **X** at its right. No frame is named. The readings, picture by picture: `0%` at 6 s, `92%` at 16 s, `78%` at 30 s, `92%` at 40 s, and at 52 s no bar at all. **The pictures lag the render**: the render log says frame 1 was saved at 15.8 s and frame 2 at 29.0 s, yet the 30-second and 40-second pictures still show the bar; a picture shows what the window had last drawn, not necessarily its state at that second. What the pictures do show is the percentage falling from 92 to 78 between two pictures and rising again to 92.

*The render window's own status line*, word for word as drawn (the window was narrow, and in two pictures the line runs past its right edge, where it is cut off):

```
Frame:1 | Time:00:00.00 | Mem: 1M | Sample 0/900                                              (picture at 6 s)
Frame:2 | Last:00:13.19 Time:00:00.01 | Remaining: 00:11.98 | Mem: 5M                       (picture at 16 s, cut off at "5M")
Frame:2 | Last:00:13.19 Time:00:12.24 | Remaining: 00:00.98 | Mem: 5M                       (picture at 30 s, cut off at "5M")
Frame:2 | Time:00:13.21 | Mem:5M, Peak: 5M                                                     (picture at 52 s, after the render)
```

So the fields are the frame number; during frame 2 the previous frame's time as `Last:`; the elapsed `Time:`; a `Remaining:` estimate; a memory figure; and, once finished, `Mem:5M, Peak: 5M`.

**Observed by hand test, 2026-10-17 — a render engine's progress call (hand-test entry 37).** On the Creator's Mac, Blender 5.3.0 Beta: a tiny render engine registered from a script called `update_progress(0.5)`, `update_stats("PROBE STATS TEXT", "PROBE INFO TEXT")` and `update_memory_stats(memory_used=12.0, memory_peak=34.0)`. With no window all three returned; only `update_stats` printed anything — its second text, as `render | Fra: 1 | PROBE INFO TEXT`. With the window open, the Status Bar's bar read `50%` during the render. The whole record is under question 2.

**Outside the interface.** The status bar exists only in the interface. The printed statistics in 3.3 are "printed", which by its wording does not depend on the interface, but the sources do not say where they go. The script hooks and the engine calls are part of the Python API, which is also used without the interface. **Settled in part by the hand tests of 2026-10-17.** With no window the printed `render` lines carry the frame, a `Remaining:` figure, `Mem:` and the sample count or stage, and the `render_stats` handler is handed their message part; a render called from a script prints none of them. With the window open no progress line reaches the terminal, but the handler still fires, the Status Bar shows `Rendering...` with a whole-number percentage, and the render window shows its own status line.

**Operating system.** The sources do not say this differs by system. **Hand tests, 2026-10-17:** everything observed under this question was run on **the Creator's Mac (Apple M1 Pro, macOS 26.6.2) in Blender 5.3.0 Beta**, so it settles macOS on Apple silicon and nothing else. **Linux and Windows were not run in 5.3** — this sitting had no Linux or Windows machine — so they are untried, not untestable (the equipment this project can reach is set out in `Preparation/Blender Research/Method.md`); on Windows an edition would have to be named before anything was claimed.

---

## 4. What does Blender write out as it works, what is that stream called, and what does it contain?

**Findings**

4.1 — **documented / observed** — *What it is called and where it shows.* The manual calls it the Console Window, "also called a Terminal". On Windows the menu entry is "Toggle System Console". The output appears in the terminal Blender was started from.
> "The Console Window, also called a Terminal, is an operating system text window that displays messages about Blender’s operations, status, and internal errors."
> "When Blender is manually started from a terminal, Blender output is shown in the Console Window it is started from."
Manual — https://docs.blender.org/manual/en/5.3/advanced/command_line/index.html — read 2026-10-11.
> "In order to run Blender you will have to specify that path to the Blender executable inside this folder, to get all output printed to the terminal."
Manual — https://docs.blender.org/manual/en/5.3/advanced/command_line/launch/macos.html — read 2026-10-11.
The manual never uses the words "standard output" or "standard error". **observed:** one Cycles file names standard error for one kind of output. In `engine.py`, lines 13–15, the help text for `--cycles-print-stats` is "Print rendering statistics to stderr". Read 2026-10-16. The manual describes the same option as "Log statistics about render memory and time usage." (https://docs.blender.org/manual/en/5.3/advanced/command_line/arguments.html, read 2026-10-11).

4.2 — **documented** — *What it contains.* The sources name these:
- Messages about operations and status, and internal errors (4.1).
- Startup messages, with the manual's examples "found bundled python: {DIR}" and "Read prefs: {DIR}/userpref.blend". Manual — https://docs.blender.org/manual/en/5.3/troubleshooting/startup.html — read 2026-10-11.
- Output of Python `print()`, and full error tracebacks: "You can see the output of print() as your script runs". API — https://docs.blender.org/api/5.3/info_tips_and_tricks.html — read 2026-10-11.
- Render statistics: the current frame, current sample, render time and saving time (3.3).
- With `--cycles-print-stats`, statistics on render memory and time (4.1).
- Video encoding output: "When you view the System Console, you can see some of the output of the encoding process. You will see even more output if you execute Blender as blender -d." Manual — https://docs.blender.org/manual/en/5.3/render/output/properties/output.html — read 2026-10-11.
- Shader-script compile errors (2.1).
- Debug messages when the `--debug…` options are given, and every operator called when `--debug-wm` is given (arguments page; API info_tips_and_tricks page).

4.3 — **documented** — *The logging system.* Messages belong to named categories and have levels. They can be sent to a file.
> `--log <match>` — "Enable logging categories, taking a single comma separated argument." Its example is `--log "render,cycles": both render and cycles messages.`
> `--log-level <level>` — the levels are "fatal", "error", "warning", "info", "debug" and "trace".
> `--log-file <filepath>` — "Set a file to output the log to."
> `--log-show-memory` — "Show memory usage for each log message."
> `-q, --quiet` — "Suppress status printing (warnings & errors are still printed)."
Manual — https://docs.blender.org/manual/en/5.3/advanced/command_line/arguments.html — read 2026-10-11.

4.4 — **documented** — *Other written records.* After a crash, Blender writes a crash file (2.5). The Info editor keeps an in-interface log of operators, errors, warnings and informational messages (2.2).

**Not covered.** The sources do not say which messages go to standard output and which to standard error. The only exception is the Cycles statistics in 4.1. They do not give the line format of any render message, or a list of what a render prints. Whether `-q` hides the render statistics is not stated.

**Where I looked.** I searched the manual, the API and the release notes for "stdout", "stderr", "standard output", "standard error", "console", "terminal", "log", "logging", "--log", "print" and "statistics". I searched the glossary for "console", "terminal" and "log" and found no entries. I read the command-line index, arguments, launch pages, startup troubleshooting, and the API's info_tips_and_tricks, info_quickstart and Context logging section. In Cycles' files I searched every `.py` file for `stderr`, `stdout` and `print(`.

**Observed by hand test, 2026-10-17 — which stream carries the render's own lines (hand-test entry 1).** On the Creator's Mac, Blender 5.3.0 Beta, set up as described at the top of this document: `"$B53" -b <scene folder>/base.blend -o "<work folder>/out/e01/e01_" -F PNG -f 1 > out.txt 2> err.txt`. **Standard output held all 54 lines; the error stream was 0 bytes.** The whole run is quoted under question 8. Every line but the version banner has the shape `<elapsed time>  <category>  | <message>`, and the categories in this run were `blend` and `render`.

**Observed by hand test, 2026-10-17 — what quiet mode takes away (hand-test entry 2).** On the Creator's Mac, Blender 5.3.0 Beta, set up as described at the top of this document. The run of hand-test entry 1 was repeated with `-q` added: `"$B53" -b -q <scene folder>/base.blend -o "<work folder>/out/e02/e02_" -F PNG -f 1 > out.txt 2> err.txt`. It printed `exit 0`, wrote `e02_0001.png`, left the error stream at 0 bytes, and left **51 lines** on standard output against entry 1's 54. The two outputs were compared line by line with every digit stripped from both (so that times and paths could not show as differences). The whole of what that comparison printed:

```
1d0
< :.  blend            | Read blend: "/private/tmp/claude-/smoozy-hand-tests-./Reports/scenes/base.blend"
53,54d51
< 
< Blender quit
```

So `-q` removed exactly three lines — `blend | Read blend: "<scene folder>/base.blend"`, the blank line before the end, and `Blender quit` — and nothing else. The version banner survived (line 6 of the quiet run), and so did every render line, including all three sample lines and the `Saved:` and `Time:` lines:

```
00:00.330  render           | Fra: 1 | Mem: 1M | Sample 0/16
00:00.331  render           | Fra: 1 | Remaining: 00:00.01 | Mem: 2M | Sample 1/16
00:00.423  render           | Fra: 1 | Mem: 2M | Sample 16/16
00:00.485  render           | Saved: '<work folder>/out/e02/e02_0001.png'
00:00.485  render           | Time: 00:00.16 (Saving: 00:00.06)
```

**Observed by hand test, 2026-10-17 — where Cycles' own statistics land, and what they hold (hand-test entry 5).** On the Creator's Mac, Blender 5.3.0 Beta, set up as described at the top of this document. Command: `"$B53" -b <scene folder>/base.blend -o "<work folder>/out/e05/e05a_" -F PNG -f 1 -- --cycles-print-stats > out.txt 2> err.txt`. It printed `exit 0` and wrote its picture. **The error stream was 0 bytes; the statistics were on standard output**, which held 132 lines. So the help text quoted in 4.1 above, "Print rendering statistics to stderr", does not describe where they appeared in this run. Where the blocks sit, by line number in standard output:

```
15:Update statistics:
82:00:00.427  render           | Fra: 1 | Mem: 2M | Sample 16/16
83:00:00.427  render           | Fra: 1 | Mem: 2M | Finished
84:00:00.490  render           | Saved: '<work folder>/out/e05/e05a_0001.png'
85:00:00.490  render           | Time: 00:00.16 (Saving: 00:00.06)
86:Render statistics:
```

So the second block, `Render statistics:`, comes **after** `Saved:` and `Time:`. The block is printed plain — no time, no category, no vertical bar. Its opening and closing lines, word for word (the kernel lines between them are times and percentages, cut here for length; there are 36 of them):

```
Render statistics:
Mesh statistics:
  Geometry:
    Total memory: 16.41K (16,800)
        Sphere                           16.02K (16,405)
        Cube                             289B (289)
        Area                             56B (56)
        backgroundlight                  50B (50)
Image statistics:
  Total image memory: 0B (0)
Kernel statistics:
  Total render time               : Total 100.00% (0.14s), Self 17.65% (0.02s)
…
Shader statistics:
  Material                        : 0.02s (Relative cost: 1.15)
  default_surface                 : 0.01s (Relative cost: 0.81)
Object statistics:
  Cube                            : 0.02s (Relative cost: 1.13)
  Sphere                          : 0.01s (Relative cost: 0.83)
```

Its memory figures are labelled by what the memory holds — geometry, image — and carry a unit letter and an exact count in brackets. **No sample count appears in either block**: searching the whole output for "sample", in any case, found only the three ordinary sample lines.

*Without the bare `--`.* The same command with `--cycles-print-stats` placed straight after `-f 1`, with no `--` before it, printed `exit 1`. It rendered and saved the frame first — `Saved: '<work folder>/out/e05/e05b_0001.png'` and `Time: 00:00.16 (Saving: 00:00.06)` are in its output — and printed no statistics block. Then standard output ended:

```
00:00.488  reports          | ERROR Cannot read file "<work folder>/--cycles-print-stats": No such file or directory

Blender quit
```

and the error stream held one line:

```
unknown argument, loading as file: --cycles-print-stats
```

**Observed by hand test, 2026-10-17 — the built-in help and the list of logging categories (hand-test entry 7).** On the Creator's Mac, Blender 5.3.0 Beta, set up as described at the top of this document.

*The help.* `"$B53" --help > out.txt 2> err.txt` printed `exit 0`, 521 lines on standard output and 0 bytes on the error stream. Its whole Logging Options section, word for word (tabs as the program printed them):

```
Logging Options:
--log <match>
	Enable logging categories, taking a single comma separated argument.

	--log "*": log everything
	--log "event": logs every category starting with 'event'.
	--log "render,cycles": log both render and cycles messages.
	--log "*mesh*": log every category containing 'mesh' sub-string.
	--log "*,^operator": log everything except operators, with '^prefix' to exclude.

--log-level <level>
	Set the logging verbosity level.

	fatal: Fatal errors only
	error: Errors only
	warning: Warnings
	info: Information about devices, files, configuration, operations
	debug: Verbose messages for developers
	trace: Very verbose code execution tracing

--log-show-memory 
	Show memory usage for each log message.

--log-show-source 
	Show source file and function name in output.

--log-show-backtrace 
	Show a back trace for each log message (debug builds only).

--log-file <filepath>
	Set a file to output the log to.

--log-list-categories 
	List all available logging categories for '--log', and exit.
```

and the quiet entry:

```
-q or --quiet 
	Suppress status printing (warnings & errors are still printed).
```

Searching the whole help for `buffer`, `flush`, `unbuffer`, `stdout` and `stderr`, in any case, found one line, line 97: `		Read from disk (Do not buffer).` — which sits under the animation player's options and is about reading picture files.

*The categories.* `"$B53" --log-list-categories > out.txt 2> err.txt` printed `exit 0`, 0 bytes on the error stream, and **153 category names**, one per line, with no banner among them. `render`, `cycles`, `image`, `image.write`, `reports` and `blend` are all there. The whole list, here joined onto running lines with spaces in place of the line breaks:

```
addon anim anim.action anim.data anim.driver anim.evaluation anim.fcurve anim.fmodifier anim.keyingset 
anim.motion_paths anim.nla asset.catalog asset.index asset.library asset.library.essentials 
asset.remote_library asset.remote_listing assets.disk_file_hash_service bke.armature blend blend.doversion 
blend.link blend.partial_write blend.readfile blend.validate blend.writefile bpy.interface bpy.rna 
color_management compositor.gpu context cycles depsgraph eevee event functions.mf_registry geom.annotations 
geom.armature_deform geom.attribute geom.bmesh.convert geom.customdata geom.mesh geom.mesh.convert geom.vfont 
geom.volume geometry.compare ghost.ndof ghost.system ghost.xr gpu.framebuffer gpu.metal gpu.mipmap 
gpu.platform gpu.shader gpu.shader.deps gpu.texture hydra.render hydra.scene_index image image.buffer 
image.gpu image.jpeg image.openexr image.openjpeg image.read image.webp image.write io.alembic io.common 
io.drop_import_file io.fbx io.materialx io.obj io.otio io.ply io.spz io.stl io.usd lib.bpath lib.icons lib.id 
lib.id_management lib.idprop lib.library lib.link_append lib.main lib.main_namemap lib.override 
lib.override.proxy_conversion lib.override.resync lib.packedfile lib.query lib.remap mask mask.rasterize 
material msgbus.pub msgbus.sub node object object.collection object.constraint object.dynamicpaint 
object.edit object.layer object.modifier operator outliner.tools paint.vertex physics.fluid 
physics.pointcache physics.rigidbody physics.softbody pose.paste project render reports rna.access 
rna.access_compare_override rna.define rna.node rna.rna_compare_override sculpt sculpt.bmesh sculpt.detail 
seq.edit sound system system.path tool.gizmo translation ui ui.font ui.handler ui.icon undo undo.armature 
undo.curve undo.curves undo.font undo.greasepencil undo.image undo.lattice undo.mball undo.mesh undo.particle 
undo.pointcloud undo.sculpt video.ffmpeg video.proxy video.read video.write wm.xr
```

**Observed by hand test, 2026-10-17 — the logging switches on a render (hand-test entry 8).** On the Creator's Mac, Blender 5.3.0 Beta, set up as described at the top of this document, on `base.blend`.

*Everything at information level.* `"$B53" -b <scene folder>/base.blend --log "*" --log-level info -o "<work folder>/out/e08/e08a_" -F PNG -f 1 > out.txt 2> err.txt` printed `exit 0`, wrote its picture, left the error stream at 0 bytes and put 146 lines on standard output. Counting the second column of every line gives the categories that carried a line: `render` 51, `cycles` 51, `blend` 6, `color_management` 1, `translation` 1; the other lines are continuation lines with an empty category, the banner and `Blender quit`. Its first eight lines, word for word:

```
00:00.015  color_management | Using Blender config: "/Applications/Blender 5.3b.app/Contents/Resources/5.3/datafiles/colormanagement/config.ocio"
00:00.099  translation      | Locale en_US used for translation
00:00.103  blend            | Read prefs: "<work folder>/cfg53/config/userpref.blend"
00:00.313  blend            | Read blend: "<scene folder>/base.blend"
00:00.318  blend            | Blender file read in 0m0.00s
00:00.318  blend            |  * Loading libraries: 0m0.00s
00:00.318  blend            |  * Applying overrides: 0m0.00s
00:00.318  blend            |  * Resyncing overrides: 0m0.00s (0 root overrides), including recursive resyncs: 0m0.00s)
```

So the render's progress lines are part of the log, under `render`, and the extra information-level lines come under `cycles` — device, passes, timing and memory lines such as these, word for word:

```
00:00.322  cycles           | Device with id "METAL_Apple M1 Pro (GPU - 14 cores)" supports OpenImageDenoise.
                            |   Usage: 646,216 (631.07K)
                            |   Peak: 1,212,488 (1.16M)
00:00.327  cycles           | Rendered 1 samples in 0.00107908 seconds (0.00107908 seconds per sample), occupancy: 1
00:00.342  cycles           | Rendered 15 samples in 0.0153321 seconds (0.00102214 seconds per sample), occupancy: 1
                            | Path tracing on: Apple M1 Pro (CPU) (8 threads) [CPU]
                            | Denoising on: Apple M1 Pro (CPU) (8 threads) [CPU]
                            | Resolution: 160x120
                            | Rendered 16 samples in 0.091049 seconds
                            | Average time per sample: 0.005691 seconds
```

*A log file.* `"$B53" -b <scene folder>/base.blend --log-file "<work folder>/logs/e08b.log" -o "<work folder>/out/e08/e08b_" -F PNG -f 1 > out.txt 2> err.txt` printed `exit 0` and wrote its picture. **The whole log moved into the file**: standard output held only these three lines —

```
Blender 5.3.0 Beta (hash 5d8151335b6b built 2026-10-16 01:36:01)

Blender quit
```

— the error stream was 0 bytes, and the file held 51 lines, beginning and ending:

```
00:00.322  blend            | Read blend: "<scene folder>/base.blend"
00:00.327  render           | Rendering single frame
…
00:00.488  render           | Saved: '<work folder>/out/e08/e08b_0001.png'
00:00.488  render           | Time: 00:00.16 (Saving: 00:00.06)
```

*Cycles' debug switch.* `"$B53" -b <scene folder>/base.blend --debug-cycles -o "<work folder>/out/e08/e08c_" -F PNG -f 1 > out.txt 2> err.txt` printed `exit 0`, wrote its picture, left the error stream at 0 bytes and put 139 lines on standard output, the debug material among them as ordinary log lines under `cycles`. Its first `cycles` lines, word for word:

```
00:00.326  cycles           | Added device "Apple M1 Pro (GPU - 14 cores)" with id "METAL_Apple M1 Pro (GPU - 14 cores)".
00:00.326  cycles           | Device with id "METAL_Apple M1 Pro (GPU - 14 cores)" supports OpenImageDenoise.
00:00.327  cycles           | Using default CPU kernels.
00:00.328  cycles           | Using default CPU kernels.
```

**Observed by hand test, 2026-10-17 — the one command that writes its failure on both streams (hand-test entry 19).** On the Creator's Mac, Blender 5.3.0 Beta: `"$B53" --command maketx "<work folder>/out/e19/no_such_image.png"` ended at exit 1 with `00:00.319  cycles           | ERROR Source image not found: <work folder>/out/e19/no_such_image.png` on standard output and `Error: Source image not found` on the error stream; given a picture that is there it printed only the written file's path, on standard output, and ended at 0. The whole record is under question 12.

**Outside the interface.** This stream is the main channel when Blender is started from a command line. The Info editor and status bar messages are interface-only. **Settled in part by the hand tests of 2026-10-17.** With no window everything a render prints went to **standard output**, Cycles' statistics included; `-q` removes three lines; `--log-file` moves the whole log into the file; `--debug-cycles` writes to standard output. The error stream carried only complaints about arguments, Python tracebacks, `maketx`'s `Error:` line and an Alembic library's own lines.

**Operating system.** On Windows the console is a separate window that is hidden after startup, and there is a Windows-only option "-con, --start-console": "Start with the console window open (ignored if -b is set), (Windows only)." On macOS, output reaches the terminal only when the program inside the `.app` folder is run directly (4.1). On Linux, the manual shows starting from a terminal and the "Run in terminal" option of desktop menus. Pages: https://docs.blender.org/manual/en/5.3/advanced/command_line/launch/windows.html, .../launch/macos.html, .../launch/linux.html and .../advanced/command_line/arguments.html — read 2026-10-11. **Hand tests, 2026-10-17:** everything observed under this question was run on **the Creator's Mac (Apple M1 Pro, macOS 26.6.2) in Blender 5.3.0 Beta**, so it settles macOS on Apple silicon and nothing else. **Linux and Windows were not run in 5.3** — this sitting had no Linux or Windows machine — so they are untried, not untestable (the equipment this project can reach is set out in `Preparation/Blender Research/Method.md`); on Windows an edition would have to be named before anything was claimed.

---

## 5. Does Blender estimate the time left, for what unit of work, and how does it describe that estimate?

**Findings**

5.1 — **documented** — *An estimate exists in the interface.* Hovering over the status bar progress bar shows "a time estimate". The unit of work is "the currently running task (such as rendering or baking)".
> "Shows the progress of the currently running task (such as rendering or baking). Hovering the mouse pointer over the progress bar will display a time estimate."
Manual — https://docs.blender.org/manual/en/5.3/interface/window_system/status_bar.html — read 2026-10-11.
That is the whole description. The manual does not say whether the estimate is time left or total time.

5.2 — **documented** — *Other uses of "time" and "estimate" that are not time left.*
- The printed render statistics contain "render time and saving time" (API, `render_stats`). Whether these are elapsed times is not stated.
- The Render Time pass is a per-pixel picture of time already spent: "An estimate for how long in milliseconds each pixel took to render. Note, this pass is not supported on GPU rendering." Manual — https://docs.blender.org/manual/en/5.3/render/layers/passes.html — read 2026-10-11.
- The image metadata can include "the render time" (https://docs.blender.org/manual/en/5.3/render/output/properties/metadata.html, read 2026-10-11).
- The animation guide tells the person to estimate the total themselves: "You can render a “typical” frame at the desired resolution, and then multiply by the number of frames that will ultimately go into the movie, to arrive at a total render time." Manual — https://docs.blender.org/manual/en/5.3/render/output/animation.html — read 2026-10-11.

**Not covered.** The sources do not say whether the hover estimate covers one frame or a whole animation, how it is worked out, or its wording. They do not say whether Blender prints any time-left estimate to the terminal. **Not in the sources:** any printed estimate of time left.

**Where I looked.** I searched the manual, the API, the release notes and Cycles' files for "remaining", "time left", "time remaining", "ETA", "estimate", "estimated" and "elapsed". The only hits about render time are the ones above. I searched Cycles' kernel and utility files under `source/` too. The word "eta" there means a refraction ratio and has nothing to do with time.

**Observed by hand test, 2026-10-17 — an estimate is printed with no window (hand-test entry 1).** On the Creator's Mac, Blender 5.3.0 Beta, set up as described at the top of this document: `"$B53" -b <scene folder>/base.blend -o "<work folder>/out/e01/e01_" -F PNG -f 1`, Cycles on the processor, 16 samples. One line of the run carries a field labelled `Remaining:`, between the frame number and the memory figure:

```
00:00.330  render           | Fra: 1 | Remaining: 00:00.01 | Mem: 2M | Sample 1/16
```

It was absent from the `Sample 0/16` line, the `Sample 16/16` line and the `Finished` line. The figure is written in minutes, seconds and hundredths.

**Observed by hand test, 2026-10-17 — a three-frame animation (hand-test entry 3).** On the Creator's Mac, Blender 5.3.0 Beta, set up as described at the top of this document: `"$B53" -b <scene folder>/base.blend -o "<work folder>/out/e03/e03_" -F PNG -x 1 -a > out.txt 2> err.txt`. It printed `exit 0`, wrote `e03_0001.png`, `e03_0002.png` and `e03_0003.png`, left the error stream at 0 bytes, and put **150 lines** on standard output. Every line naming the range, a frame, a `Remaining:` figure, a saved file or a time, with its line number in the output, word for word:

```
2:00:00.325  render           | Rendering animation (frames 1..3)
3:00:00.326  render           | Rendering frame 1
48:00:00.332  render           | Fra: 1 | Remaining: 00:00.01 | Mem: 2M | Sample 1/16
51:00:00.486  render           | Saved: '<work folder>/out/e03/e03_0001.png'
52:00:00.486  render           | Time: 00:00.16 (Saving: 00:00.06)
53:00:00.486  render           | Rendering frame 2
96:00:00.489  render           | Fra: 2 | Remaining: 00:00.01 | Mem: 2M | Sample 1/16
99:00:00.572  render           | Saved: '<work folder>/out/e03/e03_0002.png'
100:00:00.572  render           | Time: 00:00.08 (Saving: 00:00.00)
101:00:00.572  render           | Rendering frame 3
144:00:00.575  render           | Fra: 3 | Remaining: 00:00.01 | Mem: 2M | Sample 1/16
147:00:00.657  render           | Saved: '<work folder>/out/e03/e03_0003.png'
148:00:00.657  render           | Time: 00:00.08 (Saving: 00:00.00)
```

Each frame ran its own `Start rendering: Scene, ViewLayer`, `Engine: Cycles`, its stage lines and its own three sample lines, `Sample 0/16`, `Sample 1/16` and `Sample 16/16`, carrying its own frame number (`Fra: 1`, `Fra: 2`, `Fra: 3`). **No line anywhere gave a figure for the range as a whole**, and the `Remaining:` figure appeared once in each frame. The last three lines of the run were `Time: 00:00.08 (Saving: 00:00.00)`, a blank line and `Blender quit`.

**Observed by hand test, 2026-10-17 — what the `render_stats` handler is handed, with no window (hand-test entry 4).** On the Creator's Mac, Blender 5.3.0 Beta, set up as described at the top of this document. A start-up script attached one handler to `bpy.app.handlers.render_stats` that printed `RENDER_STATS:` and the `repr` of its arguments on every call, and one to `render_complete` that printed the count. Commands: `"$B53" -b <scene folder>/base.blend --python stats_probe.py -o "<work folder>/out/e04/e04a_" -F PNG -f 1` and the same with `-x 1 -a` in place of `-f 1`. Both printed `exit 0`; both error streams were 0 bytes. **It fired 45 times for one frame and 133 times for three**, and the run's own count line read `RENDER_STATS FIRED 45 TIMES`. The first string, the sampling strings and the last three of the one-frame run, word for word:

```
RENDER_STATS: ('Mem: 0M | Synchronizing object | Cube', None)
RENDER_STATS: ('Mem: 1M | Sample 0/16', None)
RENDER_STATS: ('Remaining: 00:00.01 | Mem: 2M | Sample 1/16', None)
RENDER_STATS: ('Mem: 2M | Sample 16/16', None)
RENDER_STATS: ('Mem: 2M | Finished', None)
RENDER_STATS: ('Time: 00:00.15 (Saving: 00:00.06)', None)
```

In the three-frame run the `Remaining:` string arrived once in each frame, word for word the same each time, each just after the printed line it matches:

```
00:00.329  render           | Fra: 1 | Remaining: 00:00.01 | Mem: 2M | Sample 1/16
RENDER_STATS: ('Remaining: 00:00.01 | Mem: 2M | Sample 1/16', None)
00:00.487  render           | Fra: 2 | Remaining: 00:00.01 | Mem: 2M | Sample 1/16
RENDER_STATS: ('Remaining: 00:00.01 | Mem: 2M | Sample 1/16', None)
00:00.576  render           | Fra: 3 | Remaining: 00:00.01 | Mem: 2M | Sample 1/16
RENDER_STATS: ('Remaining: 00:00.01 | Mem: 2M | Sample 1/16', None)
```

So the handler is handed two arguments, a string and `None`; the string is the message part of the printed line, with the separator ` | `; **none of the 45 strings carries a frame number** (the printed lines do, as `Fra: 1`); the current sample and its maximum arrive as `Sample <n>/<max>` while sampling runs; and the render time and saving time arrive together in the last string only, as `Time: … (Saving: …)`.

**Observed by hand test, 2026-10-17 — EEVEE and Workbench, and the engine's name (hand-test entry 6).** On the Creator's Mac, Blender 5.3.0 Beta, set up as described at the top of this document, on `base.blend` (whose own engine is Cycles at 16 samples).

*The names.* A start-up script set `bpy.context.scene.render.engine` to four names in turn and printed what happened. The whole of what it printed:

```
ENGINE ACCEPTED: BLENDER_EEVEE
ENGINE RAISED: BLENDER_EEVEE_NEXT TypeError bpy_struct: item.attr = val: enum "BLENDER_EEVEE_NEXT" not found in ('BLENDER_EEVEE', 'BLENDER_WORKBENCH', 'CYCLES')
ENGINE ACCEPTED: BLENDER_WORKBENCH
ENGINE ACCEPTED: CYCLES
```

*EEVEE.* `"$B53" -b <scene folder>/base.blend -E BLENDER_EEVEE -o "<work folder>/out/e06/eevee_" -F PNG -f 1` printed `exit 0`, wrote its picture, left the error stream at 0 bytes, and printed 14 lines. The whole of them:

```
00:00.321  blend            | Read blend: "<scene folder>/base.blend"
00:00.327  render           | Rendering single frame
00:00.327  render           | Rendering frame 1
00:00.329  render           | Start rendering: Scene, ViewLayer
00:00.329  render           | Engine: EEVEE
00:00.472  render           | Fra: 1 | Rendering 1 / 64 samples
Blender 5.3.0 Beta (hash 5d8151335b6b built 2026-10-16 01:36:01)
00:00.683  render           | Fra: 1 | Rendering 25 / 64 samples
00:00.838  render           | Fra: 1 | Rendering 50 / 64 samples
00:00.925  render           | Fra: 1 | Rendering 64 / 64 samples
00:01.017  render           | Saved: '<work folder>/out/e06/eevee_0001.png'
00:01.017  render           | Time: 00:00.68 (Saving: 00:00.07)

Blender quit
```

So EEVEE's progress line is shaped `Fra: 1 | Rendering <n> / <max> samples`, printed at 1, 25, 50 and 64; it carries **no memory figure, no `Remaining:` field and no stage lines**, and it counted to 64 although the scene asks Cycles for 16, printing nothing about the difference.

*Workbench.* `"$B53" -b <scene folder>/base.blend -E BLENDER_WORKBENCH -o "<work folder>/out/e06/wb_" -F PNG -f 1` printed `exit 0`, wrote its picture, left the error stream at 0 bytes, and printed 10 lines. The whole of them:

```
00:00.334  blend            | Read blend: "<scene folder>/base.blend"
00:00.339  render           | Rendering single frame
00:00.339  render           | Rendering frame 1
00:00.339  render           | Start rendering: Scene, ViewLayer
00:00.339  render           | Engine: Workbench
00:00.507  render           | Saved: '<work folder>/out/e06/wb_0001.png'
00:00.507  render           | Time: 00:00.16 (Saving: 00:00.06)
Blender 5.3.0 Beta (hash 5d8151335b6b built 2026-10-16 01:36:01)

Blender quit
```

**Workbench printed no progress line and no sample line at all.** Neither engine mentioned a tile.

**Observed by hand test, 2026-10-17 — the estimate on a long frame (hand-test entry 26).** On the Creator's Mac, Blender 5.3.0 Beta: on a plain 4096-sample frame that took 38 seconds, the one `Remaining:` figure was printed on the `Sample 1/4096` line, at `00:00.339`, and read `00:37.71`. Under adaptive sampling it appeared on every sample line from 1 to 160 and on the `Finished` line, rising and falling — `00:38.25`, `00:38.48`, `00:28.83`, `00:23.01`, `00:19.13`, `00:16.36`, `00:14.28`, `00:12.66`, `00:17.76`, then `Finished` with `Remaining: 00:22.76` — although the frame finished at about one second. Every line is quoted under question 11.

**Observed by hand test, 2026-10-17 — the estimate with the window open (hand-test entry 30).** On the Creator's Mac, Blender 5.3.0 Beta, a two-frame render started from a script the way F12 starts it, with the window open. The render window's status line carried `Remaining:` in frame 2 — `Frame:2 | Last:00:13.19 Time:00:00.01 | Remaining: 00:11.98 | Mem: 5M` early in the frame and `Frame:2 | Last:00:13.19 Time:00:12.24 | Remaining: 00:00.98 | Mem: 5M` near its end — so the estimate there counts down within a frame and starts again at the next. The whole record is under question 3. What the tooltip on the Status Bar's progress bar says is under hand-test entry 31.

**Observed by hand test, 2026-10-17 — what the tooltip on the progress bar says (hand-test entry 31).** On the Creator's Mac, Blender 5.3.0 Beta, set up as described at the top of this document, started with its window and with `--enable-event-simulate` on `scene14.blend`, frames 1 and 2. A start-up script started the render the way F12 starts it (`bpy.ops.render.render('INVOKE_DEFAULT', animation=True)`) two seconds in; at nine seconds it moved the pointer, with Blender's own `Window.event_simulate`, onto the progress bar's centre — window pixel 1585 across, 23 up, read off hand-test entry 30's pictures — and at thirteen seconds photographed the window. The picture shows, above the Status Bar, a box of **two lines**, word for word:

```
Time Remaining: 00:13.50
Time Elapsed: 00:07.48.
```

The second line ends with a full stop. Beside it the bar read `Rendering...` and `35%`. The figures are minutes, seconds and hundredths.

**Outside the interface.** The only documented time estimate is in the status bar, which is interface-only. The sources say nothing of an estimate outside the interface. **Settled in part by the hand tests of 2026-10-17.** An estimate **is** printed with no window, as `Remaining:` on Cycles' `render` lines, starting again in each frame and never covering the whole range; EEVEE prints none. With the window, resting the pointer on the progress bar shows `Time Remaining:` and `Time Elapsed:`, and the render window's line carries `Remaining:`.

**Operating system.** Not stated in the sources. The Render Time pass is limited by device (CPU only), not by system. **Hand tests, 2026-10-17:** everything observed under this question was run on **the Creator's Mac (Apple M1 Pro, macOS 26.6.2) in Blender 5.3.0 Beta**, so it settles macOS on Apple silicon and nothing else. **Linux and Windows were not run in 5.3** — this sitting had no Linux or Windows machine — so they are untried, not untestable (the equipment this project can reach is set out in `Preparation/Blender Research/Method.md`); on Windows an edition would have to be named before anything was claimed.

---

## 6. What does Blender report about the memory it is using on a graphics card?

**Findings**

6.1 — **documented** — *A status bar setting for GPU memory.* The API lists a preference that shows GPU video memory use in the status bar. It is off by default.
> `show_statusbar_vram` — "Show GPU video memory usage (default False)"
API — https://docs.blender.org/api/5.3/bpy.types.PreferencesView.html — read 2026-10-11.
The manual's status bar page and Status Bar preferences list "System Memory", described as "an estimate of Blender’s RAM consumption". They do not list a video memory item (https://docs.blender.org/manual/en/5.3/interface/window_system/status_bar.html and https://docs.blender.org/manual/en/5.3/editors/preferences/interface.html, read 2026-10-11). The status bar pictures show "Mem: 29.7 MiB" and "Mem: 32.7 MB" alongside old version numbers (v2.81.16, v2.80.74). They show no GPU figure.

6.2 — **documented** — *Memory figures that do not say which memory.*
- Render engines send "Current memory usage in megabytes" and "Peak memory usage in megabytes" through `update_memory_stats`. API — https://docs.blender.org/api/5.3/bpy.types.RenderEngine.html — read 2026-10-11.
- Image metadata: "Memory — Includes the peak memory usage." Manual — https://docs.blender.org/manual/en/5.3/render/output/properties/metadata.html. The API says this is on by default: "Include the peak memory usage in image metadata (default True)" (https://docs.blender.org/api/5.3/bpy.types.RenderSettings.html). Both read 2026-10-11.
- `--cycles-print-stats`: "Log statistics about render memory and time usage." (arguments page).
None of these say whether the figure is graphics-card memory, main memory, or both.

6.3 — **documented** — *What happens when graphics memory runs short.* For CUDA, OptiX, HIP and Metal, Blender moves on to main memory. Otherwise the message is "Error: Out of memory" (1.1). For EEVEE: "There is no standard way of estimating if the resources will fit into the GPU memory and/or if the GPU will render them successfully." Manual — https://docs.blender.org/manual/en/5.3/render/eevee/limitations/limitations.html — read 2026-10-11.

**Not covered.** The sources do not say what the GPU memory status bar item shows: one card or several, used or free, or units. They do not say whether it updates during a render. They do not say whether any printed render statistic gives graphics-card memory. The 5.3 Cycles release note mentions "unified memory optimizations" for one device line, but not reporting. **Not in the sources:** any per-card memory figure printed during a render.

**Where I looked.** I searched the manual, the API, the release notes and Cycles' `.py` files for "vram", "video memory", "GPU memory", "graphics memory", "device memory", "memory used", "memory usage", "peak", "Mem:" and "out of memory". I read the status bar page, Interface and System preferences, GPU rendering, EEVEE limitations, metadata, and the `RenderEngine`, `PreferencesView` and `RenderSettings` API pages. I opened the three status bar pictures and the metadata panel picture.

**Observed by hand test, 2026-10-17 — where Cycles' own statistics land, and what they hold (hand-test entry 5).** On the Creator's Mac, Blender 5.3.0 Beta, set up as described at the top of this document. Command: `"$B53" -b <scene folder>/base.blend -o "<work folder>/out/e05/e05a_" -F PNG -f 1 -- --cycles-print-stats > out.txt 2> err.txt`. It printed `exit 0` and wrote its picture. **The error stream was 0 bytes; the statistics were on standard output**, which held 132 lines. So the help text quoted in 4.1 above, "Print rendering statistics to stderr", does not describe where they appeared in this run. Where the blocks sit, by line number in standard output:

```
15:Update statistics:
82:00:00.427  render           | Fra: 1 | Mem: 2M | Sample 16/16
83:00:00.427  render           | Fra: 1 | Mem: 2M | Finished
84:00:00.490  render           | Saved: '<work folder>/out/e05/e05a_0001.png'
85:00:00.490  render           | Time: 00:00.16 (Saving: 00:00.06)
86:Render statistics:
```

So the second block, `Render statistics:`, comes **after** `Saved:` and `Time:`. The block is printed plain — no time, no category, no vertical bar. Its opening and closing lines, word for word (the kernel lines between them are times and percentages, cut here for length; there are 36 of them):

```
Render statistics:
Mesh statistics:
  Geometry:
    Total memory: 16.41K (16,800)
        Sphere                           16.02K (16,405)
        Cube                             289B (289)
        Area                             56B (56)
        backgroundlight                  50B (50)
Image statistics:
  Total image memory: 0B (0)
Kernel statistics:
  Total render time               : Total 100.00% (0.14s), Self 17.65% (0.02s)
…
Shader statistics:
  Material                        : 0.02s (Relative cost: 1.15)
  default_surface                 : 0.01s (Relative cost: 0.81)
Object statistics:
  Cube                            : 0.02s (Relative cost: 1.13)
  Sphere                          : 0.01s (Relative cost: 0.83)
```

Its memory figures are labelled by what the memory holds — geometry, image — and carry a unit letter and an exact count in brackets. **No sample count appears in either block**: searching the whole output for "sample", in any case, found only the three ordinary sample lines.

*Without the bare `--`.* The same command with `--cycles-print-stats` placed straight after `-f 1`, with no `--` before it, printed `exit 1`. It rendered and saved the frame first — `Saved: '<work folder>/out/e05/e05b_0001.png'` and `Time: 00:00.16 (Saving: 00:00.06)` are in its output — and printed no statistics block. Then standard output ended:

```
00:00.488  reports          | ERROR Cannot read file "<work folder>/--cycles-print-stats": No such file or directory

Blender quit
```

and the error stream held one line:

```
unknown argument, loading as file: --cycles-print-stats
```

**Observed by hand test, 2026-10-17 — a render on the Mac's own graphics, and what is printed about memory (hand-test entry 9).** On the Creator's Mac, Blender 5.3.0 Beta, set up as described at the top of this document, on `simple.blend` (Cycles, 16 samples, 160 by 120). Every run below printed `exit 0`, wrote its picture and left the error stream at 0 bytes.

*The two status-bar preferences.* A start-up script read them and printed:

```
preferences.view.show_statusbar_vram => False
preferences.view.show_statusbar_memory => False
```

*A Metal render chosen through the preferences.* A start-up script set Cycles' compute device type to `METAL`, turned every device on, and set the scene to the GPU. It printed:

```
DEVICE: 'Apple M1 Pro' 'CPU' use = True
DEVICE: 'Apple M1 Pro (GPU - 14 cores)' 'METAL' use = True
SCENE DEVICE: GPU
```

The render's lines had the same shape as a processor render, with one memory field, `Mem:`, and no second:

```
00:00.591  render           | Fra: 1 | Remaining: 00:01.03 | Mem: 1M | Sample 1/16
00:00.593  render           | Fra: 1 | Remaining: 00:00.49 | Mem: 1M | Sample 2/16
00:00.597  render           | Fra: 1 | Remaining: 00:00.12 | Mem: 1M | Sample 6/16
00:00.684  render           | Fra: 1 | Mem: 2M | Finished
```

*The same file with the device given on the command line* — `"$B53" -b <scene folder>/simple.blend -o "<work folder>/out/e09/e09b_" -F PNG -f 1 -- --cycles-device METAL` — printed a far larger figure on the same field:

```
00:00.692  render           | Fra: 1 | Mem: 1394M | Finished
```

Both carry the label `Mem:` and the unit `M`; neither says what memory it counts.

*Cycles' statistics on the Metal render* (`-- --cycles-print-stats` added to the preferences run). The whole `Render statistics:` block:

```
Render statistics:
Mesh statistics:
  Geometry:
    Total memory: 386B (386)
        Cube                             289B (289)
        backgroundlight                  50B (50)
        Light                            47B (47)
Image statistics:
  Total image memory: 0B (0)
Profiling information not available (only works with CPU rendering)
```

*Cycles' debug switch on the Metal render* (`--debug-cycles` added to the preferences run). The one block that names memory, and the device it rendered on, word for word:

```
00:00.488  cycles           | System memory statistics after full device sync:
                            |   Usage: 20,900 (20.41K)
                            |   Peak: 1,134,480 (1.08M)
                            | Path tracing on: Apple M1 Pro (GPU - 14 cores) (Metal) (display) [METAL_Apple M1 Pro (GPU - 14 cores)]
                            |                    Hardware Ray-Tracing: Off
                            |                  Apple M1 Pro (CPU) (7 threads) [CPU]
```

No `Mapped host memory limit` or `Local memory reserved` line appeared on this machine. **Searching all four runs for "VRAM", "video memory", "GPU memory" and "device memory", in any case, matched nothing.** The one memory figure plainly labelled is **system** memory; nothing printed says how much of the graphics memory is in use — which on this Mac is the same pool of memory as the processor's.

**Observed by hand test, 2026-10-17 — the Status Bar's own menu, and the video-memory readout (hand-test entry 32).** On the Creator's Mac, Blender 5.3.0 Beta, set up as described at the top of this document, started with its window and with `--enable-event-simulate` on `samples.blend`. A start-up script ran a timed plan; its own lines, word for word:

```
[plan   2.00] right click at 2800,20
[plan   4.14] shot of window 0 (1512 x 862) saved as e32_menu_w0.png
[plan   5.00] key ESC pressed and released with the pointer at 2800,20
[plan   6.11] shot of window 0 (1512 x 862) saved as e32_rest_before_w0.png
[plan   7.00] BEFORE show_statusbar_vram = False  show_statusbar_memory = False
[plan   7.00] step vram '' failed: AttributeError('bpy_struct: attribute "show_statusbar_vram" from "PreferencesView" is read-only')
[plan   9.12] shot of window 0 (1512 x 862) saved as e32_rest_after_w0.png
[plan  10.00] DEVICE 'Apple M1 Pro' 'CPU' use = True
[plan  10.00] DEVICE 'Apple M1 Pro (GPU - 14 cores)' 'METAL' use = True
[plan  10.00] scene device = GPU
[plan  11.02] render invoke {'write_still': True} returned {'RUNNING_MODAL'}
[plan  14.12] shot of window 0 (1512 x 862) saved as e32_render_w0.png
```

The run ended at exit 0 with an empty error stream.

*The menu.* A simulated right-click on the right-hand end of the Status Bar opened a menu headed `Status Bar` holding **exactly five items**, each with a tick box, as photographed:

```
Scene Statistics       (not ticked)
Scene Duration         (not ticked)
System Memory          (not ticked)
Extensions Updates     (ticked)
Blender Version        (ticked)
```

**No item mentions video memory or GPU memory**, and nothing is cut off. (The picture taken after the simulated Esc still showed the menu; the one at 14 seconds no longer did.)

*The readout.* **On this Mac the preference `show_statusbar_vram` could not be switched on: setting it raised `AttributeError('bpy_struct: attribute "show_statusbar_vram" from "PreferencesView" is read-only')`.** The step stopped at that line, so `show_statusbar_memory` was not switched on either. The right-hand end of the Status Bar, at rest and three seconds into a render on the Mac's own graphics (Metal), read only the network icon and `5.3.0 b`; no memory figure of any kind was shown.

**Observed by hand test, 2026-10-17 — a render engine's progress call (hand-test entry 37).** On the Creator's Mac, Blender 5.3.0 Beta: a tiny render engine registered from a script called `update_progress(0.5)`, `update_stats("PROBE STATS TEXT", "PROBE INFO TEXT")` and `update_memory_stats(memory_used=12.0, memory_peak=34.0)`. With no window all three returned; only `update_stats` printed anything — its second text, as `render | Fra: 1 | PROBE INFO TEXT`. With the window open, the Status Bar's bar read `50%` during the render. The whole record is under question 2.

**Outside the interface.** The status bar item exists only in the interface. The engine memory figures and the metadata are not tied to the interface. The printed statistics come from a command-line option. The sources do not say any of these is graphics-card memory. **Settled in part by the hand tests of 2026-10-17.** With no window nothing printed is labelled as graphics-card memory, on the processor or on Metal; `--debug-cycles` labels only system memory. With the window, the Status Bar's menu offers no video-memory item, and on this Mac `show_statusbar_vram` is read-only and could not be switched on.

**Operating system.** Not stated for the reporting itself. The fallback to main memory is named for CUDA, OptiX, HIP and Metal, not oneAPI (1.1). That makes it Windows and Linux for CUDA, OptiX and HIP, and macOS for Metal. **Hand tests, 2026-10-17:** everything observed under this question was run on **the Creator's Mac (Apple M1 Pro, macOS 26.6.2) in Blender 5.3.0 Beta**, so it settles macOS on Apple silicon and nothing else. **Linux and Windows were not run in 5.3** — this sitting had no Linux or Windows machine — so they are untried, not untestable (the equipment this project can reach is set out in `Preparation/Blender Research/Method.md`); on Windows an edition would have to be named before anything was claimed.

---

## 7. What does a render report about its own progress, across the whole range of picture sizes a render will accept?

**Reading of the question.** I take "the whole range of picture sizes" to mean the limits Blender accepts for picture width and height. The question is then whether progress reporting changes anywhere in that range.

**Findings**

7.1 — **documented** — *The range of sizes accepted.*
> `resolution_x` — "Number of horizontal pixels in the rendered image (in [4, 65536], default 1920)"
> `resolution_y` — "Number of vertical pixels in the rendered image (in [4, 65536], default 1080)"
> `resolution_percentage` — "Percentage scale for render resolution (in [1, 32767], default 100)"
API — https://docs.blender.org/api/5.3/bpy.types.RenderSettings.html — read 2026-10-11.

7.2 — **documented** — *The progress reports themselves do not mention size.* These are the status bar bar and estimate (3.1), the engine's 0-to-1 fraction (3.2), and the printed statistics with frame, sample, render time and saving time (3.3). No source says any of them changes with picture size.

7.3 — **observed / documented** — *Cycles splits pictures into tiles.* Large pictures are rendered in square tiles of a set size, and tiles are kept on disk while rendering.
- **observed:** `properties.py`, lines 1128–1139: `use_auto_tile` has the description "Deprecated, tiling is always enabled". `tile_size` has `default=2048`, `min=8` and `max=8192`, with the description "Render high resolution images in tiles of this size, to reduce memory usage. Tiles are cached to disk while rendering to save memory". Read 2026-10-16.
- **documented:** "Render high resolution images in tiles of this size, to reduce memory usage of render passes. Tiles are cached to disk while rendering to save memory." Manual — https://docs.blender.org/manual/en/5.3/render/cycles/render_settings/performance.html — read 2026-10-11.

7.4 — **inferred** — *Small and large pictures are handled differently.* A picture no wider or taller than the tile size fits in one tile. A larger one is split into several. The step: tiles are a fixed size (2048 by default, at most 8192), and accepted pictures go up to 65536 on a side. So above the tile size there must be more than one tile. The sources do not say the progress report changes when this happens.

7.5 — **documented** — *Engines can mark tiles.* `RenderEngine` has `use_highlight_tiles` and `tile_highlight_set(x, y, width, height, highlight)` ("Set highlighted state of the given tile"). API — https://docs.blender.org/api/5.3/bpy.types.RenderEngine.html — read 2026-10-11. The sources do not say whether Cycles uses these, or whether it is a progress report.

**Not covered.** **Not in the sources:** whether any progress report names tiles, counts them, or behaves differently for very small or very large pictures. The same goes for whether the progress fraction moves smoothly or in steps per tile. Picture size limits for EEVEE and Workbench beyond the shared settings above are not covered.

**Where I looked.** I read the `RenderSettings` resolution entries, the manual's Format page (https://docs.blender.org/manual/en/5.3/render/output/properties/format.html), the Cycles Performance page, the Working Limits page (https://docs.blender.org/manual/en/5.3/advanced/limits.html), and `RenderEngine`. In Cycles' files I read the tile settings and searched for "tile", "progress" and "resolution". I searched the manual and API for "tile", "high resolution", "resolution" with "progress", and "large image". All 5.3 release notes were read.

**Observed by hand test, 2026-10-17 — EEVEE and Workbench, and the engine's name (hand-test entry 6).** On the Creator's Mac, Blender 5.3.0 Beta, set up as described at the top of this document, on `base.blend` (whose own engine is Cycles at 16 samples).

*The names.* A start-up script set `bpy.context.scene.render.engine` to four names in turn and printed what happened. The whole of what it printed:

```
ENGINE ACCEPTED: BLENDER_EEVEE
ENGINE RAISED: BLENDER_EEVEE_NEXT TypeError bpy_struct: item.attr = val: enum "BLENDER_EEVEE_NEXT" not found in ('BLENDER_EEVEE', 'BLENDER_WORKBENCH', 'CYCLES')
ENGINE ACCEPTED: BLENDER_WORKBENCH
ENGINE ACCEPTED: CYCLES
```

*EEVEE.* `"$B53" -b <scene folder>/base.blend -E BLENDER_EEVEE -o "<work folder>/out/e06/eevee_" -F PNG -f 1` printed `exit 0`, wrote its picture, left the error stream at 0 bytes, and printed 14 lines. The whole of them:

```
00:00.321  blend            | Read blend: "<scene folder>/base.blend"
00:00.327  render           | Rendering single frame
00:00.327  render           | Rendering frame 1
00:00.329  render           | Start rendering: Scene, ViewLayer
00:00.329  render           | Engine: EEVEE
00:00.472  render           | Fra: 1 | Rendering 1 / 64 samples
Blender 5.3.0 Beta (hash 5d8151335b6b built 2026-10-16 01:36:01)
00:00.683  render           | Fra: 1 | Rendering 25 / 64 samples
00:00.838  render           | Fra: 1 | Rendering 50 / 64 samples
00:00.925  render           | Fra: 1 | Rendering 64 / 64 samples
00:01.017  render           | Saved: '<work folder>/out/e06/eevee_0001.png'
00:01.017  render           | Time: 00:00.68 (Saving: 00:00.07)

Blender quit
```

So EEVEE's progress line is shaped `Fra: 1 | Rendering <n> / <max> samples`, printed at 1, 25, 50 and 64; it carries **no memory figure, no `Remaining:` field and no stage lines**, and it counted to 64 although the scene asks Cycles for 16, printing nothing about the difference.

*Workbench.* `"$B53" -b <scene folder>/base.blend -E BLENDER_WORKBENCH -o "<work folder>/out/e06/wb_" -F PNG -f 1` printed `exit 0`, wrote its picture, left the error stream at 0 bytes, and printed 10 lines. The whole of them:

```
00:00.334  blend            | Read blend: "<scene folder>/base.blend"
00:00.339  render           | Rendering single frame
00:00.339  render           | Rendering frame 1
00:00.339  render           | Start rendering: Scene, ViewLayer
00:00.339  render           | Engine: Workbench
00:00.507  render           | Saved: '<work folder>/out/e06/wb_0001.png'
00:00.507  render           | Time: 00:00.16 (Saving: 00:00.06)
Blender 5.3.0 Beta (hash 5d8151335b6b built 2026-10-16 01:36:01)

Blender quit
```

**Workbench printed no progress line and no sample line at all.** Neither engine mentioned a tile.

**Observed by hand test, 2026-10-17 — what a render prints about its progress at each picture size (hand-test entry 23).** On the Creator's Mac, Blender 5.3.0 Beta, set up as described at the top of this document, on `base.blend` (Cycles on the processor, 16 samples, tile size left at its default). The same frame was rendered with no window at eight sizes, changing only the two resolution numbers from a `--python-expr` — `"$B53" -b <scene folder>/base.blend --python-expr "import bpy; r = bpy.context.scene.render; r.resolution_x = <x>; r.resolution_y = <y>" -o "<work folder>/out/e23/s<x>x<y>_" -F PNG -f 1` — and the lines carrying `Sample ` and `Tiles` were counted. Every run ended at exit 0 with an empty error stream and wrote a picture of exactly the size asked for (read back from the PNG header: 4x4, 64x64, 160x120, 1920x1080, 2048x2048, 2049x2049, 4096x4096, 10000x10000).

| Size | Lines with a sample count | Of them, lines with a tile count |
| --- | --- | --- |
| 4 by 4 | 3 | 0 |
| 64 by 64 | 3 | 0 |
| 160 by 120 | 3 | 0 |
| 1920 by 1080 | 3 | 0 |
| 2048 by 2048 | 3 | 0 |
| 2049 by 2049 | 12 | 12 |
| 4096 by 4096 | 12 | 12 |
| 10000 by 10000 (denoising switched off for this run alone, to keep the Mac's memory clear) | 75 | 75 |

**2048 by 2048 printed no tile; 2049 by 2049 printed four tiles.** The sample lines at 2048 by 2048, whole:

```
00:00.350  render           | Fra: 1 | Mem: 1M | Sample 0/16
00:00.535  render           | Fra: 1 | Remaining: 00:02.77 | Mem: 225M | Sample 1/16
00:11.532  render           | Fra: 1 | Mem: 225M | Sample 16/16
```

and all twelve at 2049 by 2049, word for word:

```
00:00.404  render           | Fra: 1 | Mem: 1M | Rendered 0/4 Tiles, Sample 0/16
00:00.587  render           | Fra: 1 | Remaining: 00:02.74 | Mem: 241M | Rendered 0/4 Tiles, Sample 1/16
00:03.618  render           | Fra: 1 | Remaining: 00:00.00 | Mem: 241M | Rendered 0/4 Tiles, Sample 16/16
00:03.618  render           | Fra: 1 | Remaining: 00:00.00 | Mem: 241M | Rendered 1/4 Tiles, Sample 16/16
00:03.618  render           | Fra: 1 | Remaining: 00:00.00 | Mem: 1M | Rendered 1/4 Tiles, Sample 1/16
00:03.622  render           | Fra: 1 | Remaining: 00:00.00 | Mem: 1M | Rendered 1/4 Tiles, Sample 16/16
00:03.622  render           | Fra: 1 | Remaining: 00:00.00 | Mem: 1M | Rendered 2/4 Tiles, Sample 16/16
00:03.622  render           | Fra: 1 | Remaining: 00:00.00 | Mem: 1M | Rendered 2/4 Tiles, Sample 1/16
00:03.623  render           | Fra: 1 | Remaining: 00:00.00 | Mem: 1M | Rendered 2/4 Tiles, Sample 16/16
00:03.624  render           | Fra: 1 | Remaining: 00:00.00 | Mem: 1M | Rendered 3/4 Tiles, Sample 16/16
00:03.624  render           | Fra: 1 | Remaining: 00:00.00 | Mem: 1M | Rendered 3/4 Tiles, Sample 1/16
00:03.624  render           | Fra: 1 | Mem: 1M | Rendered 3/4 Tiles, Sample 16/16
```

So once the picture is larger than one tile, the line reads `Rendered <n>/<total> Tiles, Sample <s>/<max>`, the sample count starts again inside each tile, and **the last line of the frame is the last tile's `Sample 16/16`**. At 10000 by 10000 the line counted **25 tiles**; its first and last sample lines:

```
00:00.530  render           | Fra: 1 | Mem: 1M | Rendered 0/25 Tiles, Sample 0/16
00:00.602  render           | Fra: 1 | Remaining: 00:27.69 | Mem: 81M | Rendered 0/25 Tiles, Sample 1/16
…
01:13.061  render           | Fra: 1 | Mem: 63M | Rendered 24/25 Tiles, Sample 16/16
```

*With the tiling switch off.* The 4096 by 4096 run again with `bpy.context.scene.cycles.use_auto_tile = False` added to the expression: exit 0, and **the same twelve lines with the same four tiles** — the first and last:

```
00:00.409  render           | Fra: 1 | Mem: 1M | Rendered 0/4 Tiles, Sample 0/16
…
00:13.078  render           | Fra: 1 | Mem: 241M | Rendered 3/4 Tiles, Sample 16/16
```

**Observed by hand test, 2026-10-17 — what the size settings accept, and what the tile settings say of themselves (hand-test entry 24).** On the Creator's Mac, Blender 5.3.0 Beta, set up as described at the top of this document. A start-up script set each size setting to values below, at and above its range, read each straight back, and then printed each setting's name, description, default and range as the running program holds them: `"$B53" -b <scene folder>/base.blend --python clamp.py`. Exit 0, error stream empty, and nothing else printed. What it printed, whole:

```
SET resolution_x = 0 -> READ BACK 4
SET resolution_x = 3 -> READ BACK 4
SET resolution_x = 4 -> READ BACK 4
SET resolution_x = 65536 -> READ BACK 65536
SET resolution_x = 65537 -> READ BACK 65536
SET resolution_x = 70000 -> READ BACK 65536
SET resolution_y = 0 -> READ BACK 4
SET resolution_y = 3 -> READ BACK 4
SET resolution_y = 4 -> READ BACK 4
SET resolution_y = 65536 -> READ BACK 65536
SET resolution_y = 65537 -> READ BACK 65536
SET resolution_y = 70000 -> READ BACK 65536
SET resolution_percentage = 0 -> READ BACK 1
SET resolution_percentage = 1 -> READ BACK 1
SET resolution_percentage = 32767 -> READ BACK 32767
SET resolution_percentage = 32768 -> READ BACK 32767
SET resolution_percentage = 70000 -> READ BACK 32767
PROPERTY use_auto_tile: name='Auto Tile' description='Deprecated, tiling is always enabled' default=True
PROPERTY tile_size: name='Tile Size' description='Render high resolution images in tiles of this size, to reduce memory usage. Tiles are cached to disk while rendering to save memory' default=2048 range [8, 8192]
PROPERTY resolution_x: name='Resolution X' description='Number of horizontal pixels in the rendered image' default=1920 range [4, 65536]
PROPERTY resolution_y: name='Resolution Y' description='Number of vertical pixels in the rendered image' default=1080 range [4, 65536]
PROPERTY resolution_percentage: name='Resolution Scale' description='Percentage scale for render resolution' default=100 range [1, 32767]
```

So **out-of-range values are clamped silently and never refused**: 0, 3 and 4 become 4; 65536, 65537 and 70000 become 65536; for the percentage, 0 becomes 1 and 32768 and 70000 become 32767. And the installed Cycles add-on's own interface file, searched for the two phrases, holds both — `ui.py`, line 405, `            # Calculation matches TileManager::compute_render_tile_size and`, and line 420, `                layout.label(text="May work poorly with render tiling", icon='STATUS_INFO')`.

**Observed by hand test, 2026-10-17 — the percentage scale, and a picture of no pixels (hand-test entry 25).** On the Creator's Mac, Blender 5.3.0 Beta, set up as described at the top of this document, on `base.blend`, each size set from a `--python-expr` as in hand-test entry 23. At 1920 by 1080 the written picture, read back from its own header, was **1920x1080** at 100 per cent, **960x540** at 50 and **633x356** at 33 — 633 being the product cut down, where rounding would give 634. All three ended at exit 0.

At 4 by 4 with the percentage at 1 — `"$B53" -b <scene folder>/base.blend --python-expr "import bpy; r = bpy.context.scene.render; r.resolution_x = 4; r.resolution_y = 4; r.resolution_percentage = 1" -o "<work folder>/out/e25/tiny_" -F PNG -f 1` — **nothing was written, and the run still ended at exit 0.** Error stream empty. Standard output, whole:

```
00:00.320  blend            | Read blend: "<scene folder>/base.blend"
00:00.325  render           | Rendering single frame
00:00.325  reports          | ERROR Image too small
Blender 5.3.0 Beta (hash 5d8151335b6b built 2026-10-16 01:36:01)

Blender quit
```

**Observed by hand test, 2026-10-17 — the Status Bar during a render, with the window open (hand-test entry 30).** On the Creator's Mac, Blender 5.3.0 Beta, a 720 by 405 render started the way F12 starts it and photographed with Blender's own screenshot command: one bar, a camera icon, the word `Rendering...`, a whole-number percentage inside the bar (`0%`, `92%`, `78%`, `92%` in successive pictures) and an X; no frame named. The whole record, including how far the pictures lag the render, is under question 3. Larger sizes were not photographed in this sitting.

**Observed by hand test, 2026-10-17 — a sequencer render, and what the Status Bar calls it (hand-test entry 36).** On the Creator's Mac, Blender 5.3.0 Beta, set up as described at the top of this document, started with its window on `seq.blend` (a scene `Main` whose Video Sequencer holds a scene strip of `Shot`, frames 1 to 10). A start-up script called `bpy.ops.render.render('INVOKE_DEFAULT', animation=True, use_sequencer_scene=True)` — the call the running program binds to Ctrl-Alt-F12 (hand-test entry 38) — and photographed the window at 4, 7 and 10 seconds. It returned `{'RUNNING_MODAL'}`; ten `Saved:` lines reached the terminal between `00:03.942` and `00:15.363`, one per frame, and no progress line; the `render_stats` handler fired 458 times and `render_complete` 11 times; the run ended at exit 0 with an empty error stream. **The Status Bar's task name read `Rendering sequence...`** in all three pictures, with the bar reading `0%`, `0%` and `33%` — the pictures lag the render, as recorded under question 3, so which frame each percentage belongs to is not established.

**Observed by hand test, 2026-10-17 — a render engine's progress call (hand-test entry 37).** On the Creator's Mac, Blender 5.3.0 Beta: a tiny render engine registered from a script called `update_progress(0.5)`, `update_stats("PROBE STATS TEXT", "PROBE INFO TEXT")` and `update_memory_stats(memory_used=12.0, memory_peak=34.0)`. With no window all three returned; only `update_stats` printed anything — its second text, as `render | Fra: 1 | PROBE INFO TEXT`. With the window open, the Status Bar's bar read `50%` during the render. The whole record is under question 2.

**Outside the interface.** The tile settings and resolution limits are scene settings and apply however the render is started. The status bar part of progress is interface-only (3.1). **Settled in part by the hand tests of 2026-10-17.** With no window, a picture larger than one tile prints a tile count on every sample line (`Rendered <n>/<total> Tiles, Sample <s>/<max>`), whether or not `use_auto_tile` is on; sizes clamp silently; a picture of no pixels ends at 0 with `ERROR Image too small`. With the window, the Status Bar shows one bar whatever the size, labelled `Rendering...` (or `Rendering sequence...`).

**Operating system.** Not stated in the sources. **Hand tests, 2026-10-17:** everything observed under this question was run on **the Creator's Mac (Apple M1 Pro, macOS 26.6.2) in Blender 5.3.0 Beta**, so it settles macOS on Apple silicon and nothing else. **Linux and Windows were not run in 5.3** — this sitting had no Linux or Windows machine — so they are untried, not untestable (the equipment this project can reach is set out in `Preparation/Blender Research/Method.md`); on Windows an edition would have to be named before anything was claimed.

---

## 8. What does a render print over the whole of a run, from its first line to its last?

**Findings**

8.1 — **documented** — *Startup lines.* The manual gives two startup messages and what they mean:
> "found bundled python: {DIR}" — "This message indicates that Blender was able to find the Python library for the Python interpreter embedded within Blender."
> "Read prefs: {DIR}/userpref.blend" — "The preferences use this path."
Manual — https://docs.blender.org/manual/en/5.3/troubleshooting/startup.html — read 2026-10-11.

8.2 — **documented (pictures)** — *Example terminal output in the manual.* All three pictures show ordinary interface sessions, not renders, and older Blender versions:
- `_images/advanced_command-line_launch_windows_console.png` (on https://docs.blender.org/manual/en/5.3/advanced/command_line/launch/windows.html) shows, in order: `Read new prefs: C:\Users\Aaron\AppData\Roaming\Blender Foundation\Blender\2.77\config\userpref.blend`, `found bundled python: c:\blender\2.77\python`, `read blend: C:\tmp\Vincent.blend`, a blank line, and `Blender quit`. The paths show version 2.77.
- `_images/advanced_command-line_launch_macos_mac.png` shows `ndof: 3Dx driver not found` and `found bundled python: /Applications/blender-2.64/blender.app/Contents/MacOS/2.64/python` (version 2.64).
- `_images/advanced_command-line_launch_linux_console.png` shows only sound-system messages (JACK and OpenAL).

8.3 — **documented** — *During the render.* Render statistics are printed. They contain the current frame, the current sample, the render time and the saving time (3.3). With `--cycles-print-stats`, Cycles adds statistics on memory and time, which its own file says go to standard error (4.1). `-q` suppresses "status printing" but not warnings and errors (4.3).

**Not covered.** **Not in the sources:** the sequence of lines a 5.3 render run prints, from first to last. That covers the first line, the per-frame lines, any "saved" line, the closing summary, and the last line. 8.2 shows a session ending with "Blender quit", but only for an old interface session. The sources do not say whether a render run in 5.3 ends the same way.

**Where I looked.** I searched the manual, the API and the release notes for "Fra:", "Saved:", "Saving", "Time:", "Mem:", "Peak", "Sample", "Finished", "quit", "read blend", "found bundled python" and "Read prefs". I searched `_images` for pictures named console, terminal, status, result or progress, and opened the three launch pictures. I read the command-line render page (https://docs.blender.org/manual/en/5.3/advanced/command_line/render.html), which shows commands but no output.

**Observed by hand test, 2026-10-17 — the whole of a one-frame run, first line to last (hand-test entry 1).** On the Creator's Mac, Blender 5.3.0 Beta, set up as described at the top of this document. The scene was `base.blend`. The command was:

```
"$B53" -b <scene folder>/base.blend -o "<work folder>/out/e01/e01_" -F PNG -f 1 > out.txt 2> err.txt; echo "exit $?"
```

It printed `exit 0` and wrote `e01_0001.png`. The error stream was **0 bytes**. Standard output held **54 lines**, and this is the whole of it:

```
00:00.319  blend            | Read blend: "<scene folder>/base.blend"
00:00.323  render           | Rendering single frame
00:00.323  render           | Rendering frame 1
00:00.323  render           | Start rendering: Scene, ViewLayer
00:00.323  render           | Engine: Cycles
00:00.327  render           | Fra: 1 | Mem: 0M | Synchronizing object | Cube
Blender 5.3.0 Beta (hash 5d8151335b6b built 2026-10-16 01:36:01)
00:00.327  render           | Fra: 1 | Mem: 0M | Synchronizing object | Area
00:00.327  render           | Fra: 1 | Mem: 0M | Initializing
00:00.328  render           | Fra: 1 | Mem: 0M | Waiting for render to start
00:00.328  render           | Fra: 1 | Mem: 0M | Updating Scene
00:00.328  render           | Fra: 1 | Mem: 0M | Updating Shaders
00:00.328  render           | Fra: 1 | Mem: 0M | Loading render kernels (may take a few minutes the first time)
00:00.328  render           | Fra: 1 | Mem: 1M | Updating Procedurals
00:00.328  render           | Fra: 1 | Mem: 1M | Updating Background
00:00.328  render           | Fra: 1 | Mem: 1M | Updating Scene Attribute
00:00.328  render           | Fra: 1 | Mem: 1M | Updating Camera
00:00.328  render           | Fra: 1 | Mem: 1M | Updating Meshes Flags
00:00.328  render           | Fra: 1 | Mem: 1M | Updating Objects
00:00.328  render           | Fra: 1 | Mem: 1M | Updating Objects | Copying Transformations to device
00:00.328  render           | Fra: 1 | Mem: 1M | Updating Particle Systems
00:00.328  render           | Fra: 1 | Mem: 1M | Updating Particle Systems | Copying Particles to device
00:00.328  render           | Fra: 1 | Mem: 1M | Updating Meshes
00:00.328  render           | Fra: 1 | Mem: 1M | Updating Objects | Applying Static Transformations
00:00.328  render           | Fra: 1 | Mem: 1M | Updating Scene BVH | Building
00:00.328  render           | Fra: 1 | Mem: 1M | Updating Scene BVH | Building BVH
00:00.328  render           | Fra: 1 | Mem: 1M | Updating Scene BVH | Building BVH 0%
00:00.328  render           | Fra: 1 | Mem: 1M | Updating Scene BVH | Copying BVH to device
00:00.328  render           | Fra: 1 | Mem: 1M | Updating Mesh | Computing attributes
00:00.329  render           | Fra: 1 | Mem: 1M | Updating Mesh | Copying Attributes to device
00:00.329  render           | Fra: 1 | Mem: 1M | Updating Mesh | Computing normals
00:00.329  render           | Fra: 1 | Mem: 1M | Updating Mesh | Copying Mesh to device
00:00.329  render           | Fra: 1 | Mem: 1M | Updating Objects Flags
00:00.329  render           | Fra: 1 | Mem: 1M | Updating Primitive Offsets
00:00.329  render           | Fra: 1 | Mem: 1M | Updating Images
00:00.329  render           | Fra: 1 | Mem: 1M | Updating Volume
00:00.329  render           | Fra: 1 | Mem: 1M | Updating Camera Volume
00:00.329  render           | Fra: 1 | Mem: 1M | Updating Lookup Tables
00:00.329  render           | Fra: 1 | Mem: 1M | Updating Lights
00:00.329  render           | Fra: 1 | Mem: 1M | Updating Lights | Computing tree
00:00.329  render           | Fra: 1 | Mem: 1M | Updating Integrator
00:00.329  render           | Fra: 1 | Mem: 1M | Updating Film
00:00.329  render           | Fra: 1 | Mem: 1M | Updating Lookup Tables
00:00.329  render           | Fra: 1 | Mem: 1M | Updating Baking
00:00.329  render           | Fra: 1 | Mem: 1M | Updating Device | Writing constant memory
00:00.329  render           | Fra: 1 | Mem: 1M | Loading denoising kernels (may take a few minutes the first time)
00:00.329  render           | Fra: 1 | Mem: 1M | Sample 0/16
00:00.330  render           | Fra: 1 | Remaining: 00:00.01 | Mem: 2M | Sample 1/16
00:00.450  render           | Fra: 1 | Mem: 2M | Sample 16/16
00:00.450  render           | Fra: 1 | Mem: 2M | Finished
00:00.511  render           | Saved: '<work folder>/out/e01/e01_0001.png'
00:00.511  render           | Time: 00:00.18 (Saving: 00:00.06)

Blender quit
```

What is in it, read straight off the lines above and nothing more: the first line is `blend | Read blend:`; the version banner is the seventh line and the only one with no time and no category; every other line has an elapsed time, a category name, a vertical bar and then the message; **exactly three lines carry a sample count** — `Sample 0/16`, `Sample 1/16` and `Sample 16/16` — for a sixteen-sample frame; `Remaining:` appears on one line only; the run ends with `Saved:`, `Time:`, a blank line and `Blender quit`.

**Observed by hand test, 2026-10-17 — what quiet mode takes away (hand-test entry 2).** On the Creator's Mac, Blender 5.3.0 Beta, set up as described at the top of this document. The run of hand-test entry 1 was repeated with `-q` added: `"$B53" -b -q <scene folder>/base.blend -o "<work folder>/out/e02/e02_" -F PNG -f 1 > out.txt 2> err.txt`. It printed `exit 0`, wrote `e02_0001.png`, left the error stream at 0 bytes, and left **51 lines** on standard output against entry 1's 54. The two outputs were compared line by line with every digit stripped from both (so that times and paths could not show as differences). The whole of what that comparison printed:

```
1d0
< :.  blend            | Read blend: "/private/tmp/claude-/smoozy-hand-tests-./Reports/scenes/base.blend"
53,54d51
< 
< Blender quit
```

So `-q` removed exactly three lines — `blend | Read blend: "<scene folder>/base.blend"`, the blank line before the end, and `Blender quit` — and nothing else. The version banner survived (line 6 of the quiet run), and so did every render line, including all three sample lines and the `Saved:` and `Time:` lines:

```
00:00.330  render           | Fra: 1 | Mem: 1M | Sample 0/16
00:00.331  render           | Fra: 1 | Remaining: 00:00.01 | Mem: 2M | Sample 1/16
00:00.423  render           | Fra: 1 | Mem: 2M | Sample 16/16
00:00.485  render           | Saved: '<work folder>/out/e02/e02_0001.png'
00:00.485  render           | Time: 00:00.16 (Saving: 00:00.06)
```

**Observed by hand test, 2026-10-17 — a three-frame animation (hand-test entry 3).** On the Creator's Mac, Blender 5.3.0 Beta, set up as described at the top of this document: `"$B53" -b <scene folder>/base.blend -o "<work folder>/out/e03/e03_" -F PNG -x 1 -a > out.txt 2> err.txt`. It printed `exit 0`, wrote `e03_0001.png`, `e03_0002.png` and `e03_0003.png`, left the error stream at 0 bytes, and put **150 lines** on standard output. Every line naming the range, a frame, a `Remaining:` figure, a saved file or a time, with its line number in the output, word for word:

```
2:00:00.325  render           | Rendering animation (frames 1..3)
3:00:00.326  render           | Rendering frame 1
48:00:00.332  render           | Fra: 1 | Remaining: 00:00.01 | Mem: 2M | Sample 1/16
51:00:00.486  render           | Saved: '<work folder>/out/e03/e03_0001.png'
52:00:00.486  render           | Time: 00:00.16 (Saving: 00:00.06)
53:00:00.486  render           | Rendering frame 2
96:00:00.489  render           | Fra: 2 | Remaining: 00:00.01 | Mem: 2M | Sample 1/16
99:00:00.572  render           | Saved: '<work folder>/out/e03/e03_0002.png'
100:00:00.572  render           | Time: 00:00.08 (Saving: 00:00.00)
101:00:00.572  render           | Rendering frame 3
144:00:00.575  render           | Fra: 3 | Remaining: 00:00.01 | Mem: 2M | Sample 1/16
147:00:00.657  render           | Saved: '<work folder>/out/e03/e03_0003.png'
148:00:00.657  render           | Time: 00:00.08 (Saving: 00:00.00)
```

Each frame ran its own `Start rendering: Scene, ViewLayer`, `Engine: Cycles`, its stage lines and its own three sample lines, `Sample 0/16`, `Sample 1/16` and `Sample 16/16`, carrying its own frame number (`Fra: 1`, `Fra: 2`, `Fra: 3`). **No line anywhere gave a figure for the range as a whole**, and the `Remaining:` figure appeared once in each frame. The last three lines of the run were `Time: 00:00.08 (Saving: 00:00.00)`, a blank line and `Blender quit`.

**Observed by hand test, 2026-10-17 — what the `render_stats` handler is handed, with no window (hand-test entry 4).** On the Creator's Mac, Blender 5.3.0 Beta, set up as described at the top of this document. A start-up script attached one handler to `bpy.app.handlers.render_stats` that printed `RENDER_STATS:` and the `repr` of its arguments on every call, and one to `render_complete` that printed the count. Commands: `"$B53" -b <scene folder>/base.blend --python stats_probe.py -o "<work folder>/out/e04/e04a_" -F PNG -f 1` and the same with `-x 1 -a` in place of `-f 1`. Both printed `exit 0`; both error streams were 0 bytes. **It fired 45 times for one frame and 133 times for three**, and the run's own count line read `RENDER_STATS FIRED 45 TIMES`. The first string, the sampling strings and the last three of the one-frame run, word for word:

```
RENDER_STATS: ('Mem: 0M | Synchronizing object | Cube', None)
RENDER_STATS: ('Mem: 1M | Sample 0/16', None)
RENDER_STATS: ('Remaining: 00:00.01 | Mem: 2M | Sample 1/16', None)
RENDER_STATS: ('Mem: 2M | Sample 16/16', None)
RENDER_STATS: ('Mem: 2M | Finished', None)
RENDER_STATS: ('Time: 00:00.15 (Saving: 00:00.06)', None)
```

In the three-frame run the `Remaining:` string arrived once in each frame, word for word the same each time, each just after the printed line it matches:

```
00:00.329  render           | Fra: 1 | Remaining: 00:00.01 | Mem: 2M | Sample 1/16
RENDER_STATS: ('Remaining: 00:00.01 | Mem: 2M | Sample 1/16', None)
00:00.487  render           | Fra: 2 | Remaining: 00:00.01 | Mem: 2M | Sample 1/16
RENDER_STATS: ('Remaining: 00:00.01 | Mem: 2M | Sample 1/16', None)
00:00.576  render           | Fra: 3 | Remaining: 00:00.01 | Mem: 2M | Sample 1/16
RENDER_STATS: ('Remaining: 00:00.01 | Mem: 2M | Sample 1/16', None)
```

So the handler is handed two arguments, a string and `None`; the string is the message part of the printed line, with the separator ` | `; **none of the 45 strings carries a frame number** (the printed lines do, as `Fra: 1`); the current sample and its maximum arrive as `Sample <n>/<max>` while sampling runs; and the render time and saving time arrive together in the last string only, as `Time: … (Saving: …)`.

**Observed by hand test, 2026-10-17 — where Cycles' own statistics land, and what they hold (hand-test entry 5).** On the Creator's Mac, Blender 5.3.0 Beta, set up as described at the top of this document. Command: `"$B53" -b <scene folder>/base.blend -o "<work folder>/out/e05/e05a_" -F PNG -f 1 -- --cycles-print-stats > out.txt 2> err.txt`. It printed `exit 0` and wrote its picture. **The error stream was 0 bytes; the statistics were on standard output**, which held 132 lines. So the help text quoted in 4.1 above, "Print rendering statistics to stderr", does not describe where they appeared in this run. Where the blocks sit, by line number in standard output:

```
15:Update statistics:
82:00:00.427  render           | Fra: 1 | Mem: 2M | Sample 16/16
83:00:00.427  render           | Fra: 1 | Mem: 2M | Finished
84:00:00.490  render           | Saved: '<work folder>/out/e05/e05a_0001.png'
85:00:00.490  render           | Time: 00:00.16 (Saving: 00:00.06)
86:Render statistics:
```

So the second block, `Render statistics:`, comes **after** `Saved:` and `Time:`. The block is printed plain — no time, no category, no vertical bar. Its opening and closing lines, word for word (the kernel lines between them are times and percentages, cut here for length; there are 36 of them):

```
Render statistics:
Mesh statistics:
  Geometry:
    Total memory: 16.41K (16,800)
        Sphere                           16.02K (16,405)
        Cube                             289B (289)
        Area                             56B (56)
        backgroundlight                  50B (50)
Image statistics:
  Total image memory: 0B (0)
Kernel statistics:
  Total render time               : Total 100.00% (0.14s), Self 17.65% (0.02s)
…
Shader statistics:
  Material                        : 0.02s (Relative cost: 1.15)
  default_surface                 : 0.01s (Relative cost: 0.81)
Object statistics:
  Cube                            : 0.02s (Relative cost: 1.13)
  Sphere                          : 0.01s (Relative cost: 0.83)
```

Its memory figures are labelled by what the memory holds — geometry, image — and carry a unit letter and an exact count in brackets. **No sample count appears in either block**: searching the whole output for "sample", in any case, found only the three ordinary sample lines.

*Without the bare `--`.* The same command with `--cycles-print-stats` placed straight after `-f 1`, with no `--` before it, printed `exit 1`. It rendered and saved the frame first — `Saved: '<work folder>/out/e05/e05b_0001.png'` and `Time: 00:00.16 (Saving: 00:00.06)` are in its output — and printed no statistics block. Then standard output ended:

```
00:00.488  reports          | ERROR Cannot read file "<work folder>/--cycles-print-stats": No such file or directory

Blender quit
```

and the error stream held one line:

```
unknown argument, loading as file: --cycles-print-stats
```

**Observed by hand test, 2026-10-17 — EEVEE and Workbench, and the engine's name (hand-test entry 6).** On the Creator's Mac, Blender 5.3.0 Beta, set up as described at the top of this document, on `base.blend` (whose own engine is Cycles at 16 samples).

*The names.* A start-up script set `bpy.context.scene.render.engine` to four names in turn and printed what happened. The whole of what it printed:

```
ENGINE ACCEPTED: BLENDER_EEVEE
ENGINE RAISED: BLENDER_EEVEE_NEXT TypeError bpy_struct: item.attr = val: enum "BLENDER_EEVEE_NEXT" not found in ('BLENDER_EEVEE', 'BLENDER_WORKBENCH', 'CYCLES')
ENGINE ACCEPTED: BLENDER_WORKBENCH
ENGINE ACCEPTED: CYCLES
```

*EEVEE.* `"$B53" -b <scene folder>/base.blend -E BLENDER_EEVEE -o "<work folder>/out/e06/eevee_" -F PNG -f 1` printed `exit 0`, wrote its picture, left the error stream at 0 bytes, and printed 14 lines. The whole of them:

```
00:00.321  blend            | Read blend: "<scene folder>/base.blend"
00:00.327  render           | Rendering single frame
00:00.327  render           | Rendering frame 1
00:00.329  render           | Start rendering: Scene, ViewLayer
00:00.329  render           | Engine: EEVEE
00:00.472  render           | Fra: 1 | Rendering 1 / 64 samples
Blender 5.3.0 Beta (hash 5d8151335b6b built 2026-10-16 01:36:01)
00:00.683  render           | Fra: 1 | Rendering 25 / 64 samples
00:00.838  render           | Fra: 1 | Rendering 50 / 64 samples
00:00.925  render           | Fra: 1 | Rendering 64 / 64 samples
00:01.017  render           | Saved: '<work folder>/out/e06/eevee_0001.png'
00:01.017  render           | Time: 00:00.68 (Saving: 00:00.07)

Blender quit
```

So EEVEE's progress line is shaped `Fra: 1 | Rendering <n> / <max> samples`, printed at 1, 25, 50 and 64; it carries **no memory figure, no `Remaining:` field and no stage lines**, and it counted to 64 although the scene asks Cycles for 16, printing nothing about the difference.

*Workbench.* `"$B53" -b <scene folder>/base.blend -E BLENDER_WORKBENCH -o "<work folder>/out/e06/wb_" -F PNG -f 1` printed `exit 0`, wrote its picture, left the error stream at 0 bytes, and printed 10 lines. The whole of them:

```
00:00.334  blend            | Read blend: "<scene folder>/base.blend"
00:00.339  render           | Rendering single frame
00:00.339  render           | Rendering frame 1
00:00.339  render           | Start rendering: Scene, ViewLayer
00:00.339  render           | Engine: Workbench
00:00.507  render           | Saved: '<work folder>/out/e06/wb_0001.png'
00:00.507  render           | Time: 00:00.16 (Saving: 00:00.06)
Blender 5.3.0 Beta (hash 5d8151335b6b built 2026-10-16 01:36:01)

Blender quit
```

**Workbench printed no progress line and no sample line at all.** Neither engine mentioned a tile.

**Observed by hand test, 2026-10-17 — a log file takes the whole run off standard output (hand-test entry 8).** On the Creator's Mac, Blender 5.3.0 Beta, set up as described at the top of this document: `"$B53" -b <scene folder>/base.blend --log-file "<work folder>/logs/e08b.log" -o "<work folder>/out/e08/e08b_" -F PNG -f 1` printed `exit 0` and wrote its picture; standard output held only the version banner, a blank line and `Blender quit`, and the file held the other 51 lines, from `blend | Read blend:` to the closing `render | Time:`. At information level (`--log "*" --log-level info`) the same render printed 146 lines; the categories carrying them were `render`, `cycles`, `blend`, `color_management` and `translation`. Both are quoted more fully under question 4.

**Observed by hand test, 2026-10-17 — with the window open, a script's render prints only the saved file (hand-test entry 29).** On the Creator's Mac, Blender 5.3.0 Beta, started with its window from a terminal on `base.blend`, a render called from the start-up script with `bpy.ops.render.render(write_still=True)` put one render line on the terminal — `00:00.780  render           | Saved: '<work folder>/out/e29/direct1_.png'` — and no progress line, while a `render_stats` handler attached by the same script fired 45 or 46 times in each of five runs. At the end the run printed `Saved session recovery to "…/quit.blend"`, a blank line and `Blender quit`. The whole record is under question 9.

**Observed by hand test, 2026-10-17 — with the window open, a render started the way F12 starts it prints only its saved files (hand-test entry 30).** On the Creator's Mac, Blender 5.3.0 Beta, started with its window from a terminal: a two-frame render started by `bpy.ops.render.render('INVOKE_DEFAULT', animation=True)` put two render lines on the terminal, `00:15.803  render           | Saved: '<work folder>/out/e30/f_0001.png'` and `00:29.037  render           | Saved: '<work folder>/out/e30/f_0002.png'`, and no progress line. The run ended with `Saved session recovery to "<work folder>/tmp/quit.blend"`, a blank line and `Blender quit`. The whole record is under question 3.

**Outside the interface.** The startup lines and render statistics are terminal output, the channel used outside the interface. The pictures in 8.2 are of interface sessions started from a terminal. **Settled in part by the hand tests of 2026-10-17.** With no window a one-frame run prints 54 lines here, from `blend | Read blend:` to `Blender quit`, all on standard output. With the window, a render prints only its `Saved:` lines on the terminal, and the session ends with `Saved session recovery to …` and `Blender quit`.

**Operating system.** The three example pictures come from Windows, macOS and Linux, and show different lines. They are old sessions, so this does not show whether 5.3 prints differently on each system. Not stated otherwise. **Hand tests, 2026-10-17:** everything observed under this question was run on **the Creator's Mac (Apple M1 Pro, macOS 26.6.2) in Blender 5.3.0 Beta**, so it settles macOS on Apple silicon and nothing else. **Linux and Windows were not run in 5.3** — this sitting had no Linux or Windows machine — so they are untried, not untestable (the equipment this project can reach is set out in `Preparation/Blender Research/Method.md`); on Windows an edition would have to be named before anything was claimed.

---

## 9. What does a render report about its progress under each of the ways a render can be started?

**Findings**

9.1 — **documented** — *The ways to start a render that the sources name.*
- Interface menu or shortcut: "Render Image F12", "Render Animation Ctrl-F12", "Render Sequencer Image Alt-F12" and "Render Sequencer Animation Ctrl-Alt-F12". Manual — https://docs.blender.org/manual/en/5.3/interface/window_system/topbar.html — read 2026-10-11.
- Viewport render: "Render Viewport Preview", "Render Playblast" and "Render Playblast on Keyframes". Manual — https://docs.blender.org/manual/en/5.3/editors/3dview/viewport_render.html — read 2026-10-11.
- Command line without the interface: `-b` with `-f <frame>` or `-a`. Manual — https://docs.blender.org/manual/en/5.3/advanced/command_line/arguments.html and https://docs.blender.org/manual/en/5.3/advanced/command_line/render.html — read 2026-10-11.
- Python: `bpy.ops.render.render(...)` and `bpy.ops.render.opengl(...)`. API — https://docs.blender.org/api/5.3/bpy.ops.render.html — read 2026-10-11. The render operator's own description is "Undocumented, consider contributing."
- Blender built as a Python module (`import bpy`). API — https://docs.blender.org/api/5.3/info_advanced_blender_as_bpy.html — read 2026-10-11.

9.2 — **documented** — *Interface renders.* The status bar shows the running task's progress, with a time estimate on hover and a cancel button (3.1).

9.3 — **documented** — *Viewport render.* It can be stopped with Esc, "as with a normal render". The page says nothing about progress. **documented (release note):** "Viewport renders are automatically paused during final renders." (https://developer.blender.org/docs/release_notes/5.3/cycles/, read 2026-10-11). This is about the live viewport pausing, not about progress reports.

9.4 — **documented / inferred** — *Command line with `-b`.* There is no user interface: "Run in background (often used for UI-less rendering)." (arguments page). `bpy.app.background` is "True when blender is running without a user interface (started with -b)" (https://docs.blender.org/api/5.3/bpy.app.html, read 2026-10-11). Output goes to the terminal (4.1).
**inferred:** in this mode there is no status bar progress bar and no hover estimate. The step: both belong to the interface's status bar, and `-b` runs without the interface. **documented:** render statistics are printed (3.3), but no source says this differs by how the render was started.

9.5 — **inferred** — *A script calling the render inside the interface.* The API says the interface stops redrawing while a script runs:
> "While a script executes, Blender waits for it to finish and is effectively locked until it’s done; while in this state Blender won’t redraw or respond to user input."
API — https://docs.blender.org/api/5.3/info_gotchas_internal_data_and_python_objects.html — read 2026-10-11.
> "EXEC_DEFAULT is used by default, running only the execute() method"
API — https://docs.blender.org/api/5.3/bpy.ops.html — read 2026-10-11.
The step: a render called from a script in the default way runs inside that script. While the script runs, the interface does not redraw, so the progress bar would not visibly move until the script ends. The sources do not state this for the render operator. They also do not describe the `'INVOKE_DEFAULT'` way of calling it.

9.6 — **documented** — *Blender as a Python module.* "Blender’s typical signal handlers are not initialized, so there is no special handling for Control-C to cancel a render and a crash log is not written in the event of a crash." (API page in 9.1). Otherwise it "is equivalent to running a script in background-mode" (same page). Nothing is said about progress output.

**Not covered.** **Not in the sources:** the exact progress shown under each way of starting a render. That includes whether the printed statistics appear in every case, whether a command-line render prints a progress line per sample or per frame, and whether a script-started render reports through the status bar after control returns to the interface.

**Where I looked.** I read the topbar Render menu, keymap pages (https://docs.blender.org/manual/en/5.3/interface/keymap/blender_default.html), viewport render, command-line pages, `bpy.ops`, `bpy.ops.render`, `bpy.app`, the info_gotchas pages, info_tips_and_tricks, info_advanced_blender_as_bpy and all release notes. I searched for "F12", "Render Image", "Render Animation", "render.render", "INVOKE_DEFAULT", "background mode", "--background" and "progress".

**Observed by hand test, 2026-10-17 — what the `render_stats` handler is handed, with no window (hand-test entry 4).** On the Creator's Mac, Blender 5.3.0 Beta, set up as described at the top of this document. A start-up script attached one handler to `bpy.app.handlers.render_stats` that printed `RENDER_STATS:` and the `repr` of its arguments on every call, and one to `render_complete` that printed the count. Commands: `"$B53" -b <scene folder>/base.blend --python stats_probe.py -o "<work folder>/out/e04/e04a_" -F PNG -f 1` and the same with `-x 1 -a` in place of `-f 1`. Both printed `exit 0`; both error streams were 0 bytes. **It fired 45 times for one frame and 133 times for three**, and the run's own count line read `RENDER_STATS FIRED 45 TIMES`. The first string, the sampling strings and the last three of the one-frame run, word for word:

```
RENDER_STATS: ('Mem: 0M | Synchronizing object | Cube', None)
RENDER_STATS: ('Mem: 1M | Sample 0/16', None)
RENDER_STATS: ('Remaining: 00:00.01 | Mem: 2M | Sample 1/16', None)
RENDER_STATS: ('Mem: 2M | Sample 16/16', None)
RENDER_STATS: ('Mem: 2M | Finished', None)
RENDER_STATS: ('Time: 00:00.15 (Saving: 00:00.06)', None)
```

In the three-frame run the `Remaining:` string arrived once in each frame, word for word the same each time, each just after the printed line it matches:

```
00:00.329  render           | Fra: 1 | Remaining: 00:00.01 | Mem: 2M | Sample 1/16
RENDER_STATS: ('Remaining: 00:00.01 | Mem: 2M | Sample 1/16', None)
00:00.487  render           | Fra: 2 | Remaining: 00:00.01 | Mem: 2M | Sample 1/16
RENDER_STATS: ('Remaining: 00:00.01 | Mem: 2M | Sample 1/16', None)
00:00.576  render           | Fra: 3 | Remaining: 00:00.01 | Mem: 2M | Sample 1/16
RENDER_STATS: ('Remaining: 00:00.01 | Mem: 2M | Sample 1/16', None)
```

So the handler is handed two arguments, a string and `None`; the string is the message part of the printed line, with the separator ` | `; **none of the 45 strings carries a frame number** (the printed lines do, as `Fra: 1`); the current sample and its maximum arrive as `Sample <n>/<max>` while sampling runs; and the render time and saving time arrive together in the last string only, as `Time: … (Saving: …)`.

**Observed by hand test, 2026-10-17 — a render called from a script, with no window, hands back a result or an exception (hand-test entry 13).** On the Creator's Mac, Blender 5.3.0 Beta, set up as described at the top of this document. A start-up script called `bpy.ops.render.render(write_still=True)` inside a `try`: on a good file it printed `RENDER RETURNED: {'FINISHED'}` and the run ended at 0; on a file with no camera it printed `RENDER RAISED: RuntimeError Error: Cannot render, no camera` (with the line `Error: Cannot render, no camera` printed on standard output just before), and the run ended at 1 although the script had caught it. The whole of that run is quoted under question 1.

**Observed by hand test, 2026-10-17 — a render started from a script, with no window, prints no progress (hand-test entry 28).** On the Creator's Mac, Blender 5.3.0 Beta, set up as described at the top of this document, on `base.blend`. Two runs that call `bpy.ops.render.render(write_still=True)` from Python instead of using `-f`: once as an expression, `"$B53" -b <scene folder>/base.blend -o "<work folder>/out/e28/expr_" -F PNG --python-expr "import bpy; print('RETURNED', bpy.ops.render.render(write_still=True))"`, and once from a script file that does the same inside a `try`. Both ended at exit 0, wrote their picture and left the error stream empty. **Neither printed a single sample line.** The expression run's standard output, whole:

```
00:00.320  blend            | Read blend: "<scene folder>/base.blend"
00:00.482  render           | Saved: '<work folder>/out/e28/expr_.png'
RETURNED {'FINISHED'}
Blender 5.3.0 Beta (hash 5d8151335b6b built 2026-10-16 01:36:01)

Blender quit
```

and the script file's run printed the same shape, ending in `RENDER RETURNED: {'FINISHED'}`. The same frame started with `-f 1` prints three sample lines and the stage lines (hand-test entry 1).

**Observed by hand test, 2026-10-17 — with Blender's window open: a render from the start-up script with the statistics handler attached, and two ways of closing (hand-test entry 29).** On the Creator's Mac, Blender 5.3.0 Beta, set up as described at the top of this document, **started with its window** (no `-b`) from a terminal: `"$B53" <scene folder>/base.blend -o "<work folder>/out/e29/<name>_" -F PNG --python win_stats.py`, run under a 240-second cap that would have ended it with `SIGKILL` and said so. The script attached a counting handler to `render_stats` and one to `render_complete`, called `bpy.ops.render.render(write_still=True)`, printed the counts, and then closed Blender one of two ways: `direct` called `bpy.ops.wm.quit_blender()` straight from the script; `timer` registered a timer that called it three seconds later. Run twice the first way and three times the second.

**Every run closed by itself and ended at exit 0, with an empty error stream; no crash file appeared in Blender's temporary folder.** The cap's own lines:

```
[harness] exited by itself after 1.5 s, status 0     (direct, run 1)
[harness] exited by itself after 1.4 s, status 0     (direct, run 2)
[harness] exited by itself after 4.3 s, status 0     (timer, run 1)
[harness] exited by itself after 4.4 s, status 0     (timer, run 2)
[harness] exited by itself after 4.2 s, status 0     (timer, run 3)
```

The first `direct` run's standard output, whole (the lines with no time are the script's own):

```
00:00.608  blend            | Read blend: "<scene folder>/base.blend"
BACKGROUND=False
VERSION=5.3.0 Beta
00:00.780  render           | Saved: '<work folder>/out/e29/direct1_.png'
RENDER RETURNED {'FINISHED'}
HANDLER FIRED 45 TIMES
FIRST=('Mem: 0M | Synchronizing object | Cube', None)
LAST=('Time: 00:00.16 (Saving: 00:00.06)', None)
RENDER_COMPLETE FIRED 1 TIMES
QUITWAY=direct
STILL HERE AFTER quit_blender()
00:00.960  blend            | Saved session recovery to "<work folder>/tmp/quit.blend"

Blender quit
```

and the first `timer` run's, whole:

```
00:00.557  blend            | Read blend: "<scene folder>/base.blend"
BACKGROUND=False
VERSION=5.3.0 Beta
00:00.734  render           | Saved: '<work folder>/out/e29/timer1_.png'
RENDER RETURNED {'FINISHED'}
HANDLER FIRED 45 TIMES
FIRST=('Mem: 0M | Synchronizing object | Cube', None)
LAST=('Time: 00:00.16 (Saving: 00:00.06)', None)
RENDER_COMPLETE FIRED 1 TIMES
QUITWAY=timer
TIMER REGISTERED FOR 3s
TIMER FIRED, CALLING quit_blender()
STILL HERE AFTER quit_blender() IN TIMER
00:03.758  blend            | Saved session recovery to "<work folder>/tmp/quit.blend"

Blender quit
```

**The handler fired 45, 46, 45, 45 and 46 times** across the five runs, with the window open, handed the same strings as with no window; and **the only render line that reached the terminal was `Saved:`** — no sample line, no stage line. At quit time every run printed `Saved session recovery to "<work folder>/tmp/quit.blend"`, which is Blender's temporary folder for these runs.

**Observed by hand test, 2026-10-17 — with Blender's window open: a render started the way F12 starts it, photographed while it ran (hand-test entry 30).** On the Creator's Mac, Blender 5.3.0 Beta, set up as described at the top of this document, started with its window from a terminal on `scene14.blend` limited to frames 1 and 2 (`-s 1 -e 2`, Cycles on the processor, 900 samples, 720 by 405), with output to `<work folder>/out/e30/f_`. A start-up script ran a timed plan: two seconds in, it called `bpy.ops.render.render('INVOKE_DEFAULT', animation=True)` — the call the F12 family of keys is bound to — and then photographed every open window with `bpy.ops.screen.screenshot` at 6, 16, 30, 40 and 52 seconds, counted the `render_stats` calls at 75 and quit at 77. It ended at exit 0 with an empty error stream. Its standard output, whole (lines beginning `[plan …]` are the script's own, with its own seconds):

```
00:00.563  blend            | Read blend: "<scene folder>/scene14.blend"
[plan   0.00] plan installed: 2 invoke anim|6 shot|16 shot|30 shot|40 shot|52 shot|75 stats|76 files out/e30|77 quit
[plan   2.03] render invoke {'animation': True} returned {'RUNNING_MODAL'}
[plan   6.16] shot of window 0 (1512 x 862) saved as e30_006.0_w0.png
[plan   6.18] shot of window 1 (390 x 262) saved as e30_006.2_w1.png
00:15.803  render           | Saved: '<work folder>/out/e30/f_0001.png'
[plan  16.14] shot of window 0 (1512 x 862) saved as e30_016.0_w0.png
[plan  16.16] shot of window 1 (390 x 262) saved as e30_016.1_w1.png
00:29.037  render           | Saved: '<work folder>/out/e30/f_0002.png'
[plan  30.14] shot of window 0 (1512 x 862) saved as e30_030.0_w0.png
[plan  30.15] shot of window 1 (390 x 262) saved as e30_030.1_w1.png
[plan  40.14] shot of window 0 (1512 x 862) saved as e30_040.0_w0.png
[plan  40.16] shot of window 1 (390 x 262) saved as e30_040.1_w1.png
[plan  52.15] shot of window 0 (1512 x 862) saved as e30_052.0_w0.png
[plan  52.16] shot of window 1 (390 x 262) saved as e30_052.1_w1.png
[plan  75.00] render_stats fired 102 times, render_complete 1, render_cancel 0
[plan  75.00] FIRST=('Mem: 0M | Synchronizing object | Cube', None)
[plan  75.00] LAST=('Time: 00:13.23 (Saving: 00:00.01)', None)
[plan  76.00] files in out/e30: ['f_0001.png', 'f_0002.png']
[plan  77.01] calling quit_blender()
[plan  77.01] still here after quit_blender()
01:17.596  blend            | Saved session recovery to "<work folder>/tmp/quit.blend"

Blender quit
```

So **the operator returned `{'RUNNING_MODAL'}` at once and the render went on afterwards**; the only render lines that reached the terminal were the two `Saved:` lines; the handler fired 102 times over two frames; and a second window, 390 by 262 points, was open during and after the render.

*The Status Bar.* In the pictures of the main window there is **one bar**: a small camera icon, the word **`Rendering...`**, a blue bar with a **whole-number percentage** written in it, and an **X** at its right. No frame is named. The readings, picture by picture: `0%` at 6 s, `92%` at 16 s, `78%` at 30 s, `92%` at 40 s, and at 52 s no bar at all. **The pictures lag the render**: the render log says frame 1 was saved at 15.8 s and frame 2 at 29.0 s, yet the 30-second and 40-second pictures still show the bar; a picture shows what the window had last drawn, not necessarily its state at that second. What the pictures do show is the percentage falling from 92 to 78 between two pictures and rising again to 92.

*The render window's own status line*, word for word as drawn (the window was narrow, and in two pictures the line runs past its right edge, where it is cut off):

```
Frame:1 | Time:00:00.00 | Mem: 1M | Sample 0/900                                              (picture at 6 s)
Frame:2 | Last:00:13.19 Time:00:00.01 | Remaining: 00:11.98 | Mem: 5M                       (picture at 16 s, cut off at "5M")
Frame:2 | Last:00:13.19 Time:00:12.24 | Remaining: 00:00.98 | Mem: 5M                       (picture at 30 s, cut off at "5M")
Frame:2 | Time:00:13.21 | Mem:5M, Peak: 5M                                                     (picture at 52 s, after the render)
```

So the fields are the frame number; during frame 2 the previous frame's time as `Last:`; the elapsed `Time:`; a `Remaining:` estimate; a memory figure; and, once finished, `Mem:5M, Peak: 5M`.

**Observed by hand test, 2026-10-17 — the render operator, the keys bound to it, and the menus, read from the running program with its window open (hand-test entry 38).** On the Creator's Mac, Blender 5.3.0 Beta, set up as described at the top of this document, started with its window and with `--enable-event-simulate` on `base.blend`. A start-up script printed the render operator's own description and arguments, every key in the active key map bound to `render.render`, `render.view_cancel` or `render.view_show`, and the scene's `save_output`; then opened the Render menu and the 3D viewport's View menu and photographed each. Exit 0, error stream empty. What it printed, keys from the active key map only (the default key map printed the same lines again):

```
[read] render.render description: '(undocumented operator)'
[read]   argument animation (BOOLEAN): 'Render files from the animation range of this scene'
[read]   argument write_still (BOOLEAN): 'Save the rendered image to the output path (used only when animation is disabled)'
[read]   argument use_viewport (BOOLEAN): 'When inside a 3D viewport, use layers and camera of the viewport'
[read]   argument use_sequencer_scene (BOOLEAN): 'Render the sequencer scene instead of the active scene'
[read]   argument layer (STRING): 'Single render layer to re-render (used only when animation is disabled)'
[read]   argument scene (STRING): 'Scene to render, current scene if not specified'
[read]   argument frame_start (INT): 'Frame to start rendering animation at. If not specified, the scene start frame will be assumed. This should only be specified if doing an animation render'
[read]   argument frame_end (INT): 'Frame to end rendering animation at. If not specified, the scene end frame will be assumed. This should only be specified if doing an animation render'
[read]   key in Blender / Screen: F12 PRESS ctrl=0 shift=0 alt=0 oskey=0 -> render.render {'animation': False, 'use_viewport': True, 'use_sequencer_scene': False, 'write_still': False}
[read]   key in Blender / Screen: F12 PRESS ctrl=0 shift=0 alt=0 oskey=1 -> render.render {'animation': True, 'use_viewport': True, 'use_sequencer_scene': False, 'write_still': False}
[read]   key in Blender / Screen: F12 PRESS ctrl=1 shift=0 alt=0 oskey=0 -> render.render {'animation': True, 'use_viewport': True, 'use_sequencer_scene': False, 'write_still': False}
[read]   key in Blender / Screen: F12 PRESS ctrl=0 shift=0 alt=1 oskey=0 -> render.render {'animation': False, 'use_viewport': True, 'use_sequencer_scene': True, 'write_still': False}
[read]   key in Blender / Screen: F12 PRESS ctrl=0 shift=0 alt=1 oskey=1 -> render.render {'animation': True, 'use_viewport': True, 'use_sequencer_scene': True, 'write_still': False}
[read]   key in Blender / Screen: F12 PRESS ctrl=1 shift=0 alt=1 oskey=0 -> render.render {'animation': True, 'use_viewport': True, 'use_sequencer_scene': True, 'write_still': False}
[read]   key in Blender / Screen: ESC PRESS ctrl=0 shift=0 alt=0 oskey=0 -> render.view_cancel {}
[read]   key in Blender / Screen: F11 PRESS ctrl=0 shift=0 alt=0 oskey=0 -> render.view_show {}
[read] scene.render.save_output = True
```

So the operator's own description reads `(undocumented operator)`, while every one of its eight arguments carries a description; F12 alone renders a still, F12 with Ctrl or with Cmd renders the animation, and with Alt each of those renders the sequencer scene instead; and Esc is bound to `render.view_cancel`, which closes the render view, not to anything that stops a render.

*The Render menu*, as photographed, holds: `Render Image` (shortcut `F12`), `Render Animation` (shortcut shown as `⌘ F12`, partly covered by a tooltip reading `Render active scene.`), `Render Audio...`, `View Render` (`F11`), `View Animation` (`⌘ F11`) and `Lock Interface` with an unticked box. **No `Render Sequencer Image` or `Render Sequencer Animation` entry was drawn** in this file's menu.

*The 3D viewport's View menu* holds, among its entries, `Render Viewport Preview`, `Render Playblast` and `Render Playblast on Keyframes`.

**Observed by hand test, 2026-10-17 — a sequencer render, and what the Status Bar calls it (hand-test entry 36).** On the Creator's Mac, Blender 5.3.0 Beta, set up as described at the top of this document, started with its window on `seq.blend` (a scene `Main` whose Video Sequencer holds a scene strip of `Shot`, frames 1 to 10). A start-up script called `bpy.ops.render.render('INVOKE_DEFAULT', animation=True, use_sequencer_scene=True)` — the call the running program binds to Ctrl-Alt-F12 (hand-test entry 38) — and photographed the window at 4, 7 and 10 seconds. It returned `{'RUNNING_MODAL'}`; ten `Saved:` lines reached the terminal between `00:03.942` and `00:15.363`, one per frame, and no progress line; the `render_stats` handler fired 458 times and `render_complete` 11 times; the run ended at exit 0 with an empty error stream. **The Status Bar's task name read `Rendering sequence...`** in all three pictures, with the bar reading `0%`, `0%` and `33%` — the pictures lag the render, as recorded under question 3, so which frame each percentage belongs to is not established.

**Observed by hand test, 2026-10-17 — a render engine's progress call (hand-test entry 37).** On the Creator's Mac, Blender 5.3.0 Beta: a tiny render engine registered from a script called `update_progress(0.5)`, `update_stats("PROBE STATS TEXT", "PROBE INFO TEXT")` and `update_memory_stats(memory_used=12.0, memory_peak=34.0)`. With no window all three returned; only `update_stats` printed anything — its second text, as `render | Fra: 1 | PROBE INFO TEXT`. With the window open, the Status Bar's bar read `50%` during the render. The whole record is under question 2.

**Outside the interface.** This question is mostly about that difference. The documented progress display (status bar) is interface-only. Outside the interface, the only documented report is the printed render statistics and terminal messages. Their exact form is not documented. **Settled in part by the hand tests of 2026-10-17.** `-f` and `-a` print the full progress with no window; a render called from Python prints only `Saved:`, with or without the window; with the window, the Status Bar bar appears for a render started the way F12 starts it, and the `render_stats` handler fires either way.

**Operating system.** **documented (release note):** "The Blender application icon no longer shows up in the Dock when running in background mode via the command line." This is a macOS item (https://developer.blender.org/docs/release_notes/5.3/user_interface/, read 2026-10-11). It is not about progress. Nothing else is tied to a system. **Hand tests, 2026-10-17:** everything observed under this question was run on **the Creator's Mac (Apple M1 Pro, macOS 26.6.2) in Blender 5.3.0 Beta**, so it settles macOS on Apple silicon and nothing else. **Linux and Windows were not run in 5.3** — this sitting had no Linux or Windows machine — so they are untried, not untestable (the equipment this project can reach is set out in `Preparation/Blender Research/Method.md`); on Windows an edition would have to be named before anything was claimed.

---

## 10. What does a render do when the machine's own memory runs out while it is sampling?

**Label: not in the sources** for the specific case (main memory running out during sampling). The nearest documented statements follow.

10.1 — **documented** — *Running out of memory is a common cause of crashes, and a crash leaves a file.*
> "The most common causes of Blender crashes: Running out of memory. …"
Manual — https://docs.blender.org/manual/en/5.3/troubleshooting/crash.html — read 2026-10-11. The crash file is described in 1.8 and 2.5.

10.2 — **documented** — *Elsewhere the manual warns of hangs as well as crashes.*
> "This can cause Blender to hang or crash if not enough memory is available."
Manual — https://docs.blender.org/manual/en/5.3/modeling/modifiers/generate/subdivision_surface.html — read 2026-10-11. This is about high subdivision levels, not sampling.

10.3 — **documented** — *Settings described as lowering memory use during rendering.*
- Tile size: "When running out of memory, lowering to 1024 or 512 pixels helps reduce memory usage, typically at a small performance cost." (https://docs.blender.org/manual/en/5.3/render/cycles/render_settings/performance.html)
- Lock Interface: "Lock interface during rendering in favor of giving more memory to the renderer." (https://docs.blender.org/manual/en/5.3/interface/window_system/topbar.html)

Both read 2026-10-11.

10.4 — **documented** — *A report kind for out-of-memory exists.* "ERROR_OUT_OF_MEMORY: Out of Memory." (https://docs.blender.org/api/5.3/bpy_types_enum_items/wm_report_items.html, read 2026-10-11). The sources do not say when it is used.

10.5 — **inferred** — *The outcome inferred from general crash statements is a crash, possibly a hang.* The step: 10.1 names running out of memory as a common cause of crashes, and 10.2 names hangs as well. Neither page mentions sampling, rendering, or main memory as opposed to graphics memory. The general statement is the only one that could cover this case.

**Not covered.** Whether the render stops cleanly with a message, crashes, hangs, or is ended by the operating system. Whether any part of the frame is saved. Whether anything is printed first. Whether the `ERROR_OUT_OF_MEMORY` report is used here.

**Where I looked.** I searched the manual, the API, the release notes and Cycles' `.py` files for "out of memory", "OUT_OF_MEMORY", "run out of", "running out", "system memory", "RAM", "swap", "allocate", "insufficient memory", "not enough memory" and "memory full". I read crash troubleshooting, GPU rendering, the Cycles Performance page, the System preferences, EEVEE limitations and the subdivision modifier warning. The Cycles kernel files have no messages about this.

**Outside the interface.** The crash statements are not tied to the interface. Lock Interface is an interface setting. The sources do not say whether running without the interface changes anything here. **Not tested in 5.3.** Filling the machine's own memory on purpose was not done on the Creator's Mac, which is their working computer, and this sitting had no other machine.

**Operating system.** The crash file differs by system (1.8). The sources do not say whether running out of main memory itself behaves differently by system. **Hand tests, 2026-10-17:** nothing was run for this question in 5.3 — not on the Creator's Mac (Apple M1 Pro, macOS 26.6.2), whose memory is not to be filled on purpose, and **not on Linux or Windows** — this sitting had no Linux or Windows machine — so they are untried, not untestable (the equipment this project can reach is set out in `Preparation/Blender Research/Method.md`); on Windows an edition would have to be named before anything was claimed.

---

## 11. What, if anything, does a render print that shows whether a frame took the number of samples it was asked for?

**Findings**

11.1 — **documented** — *The printed statistics include the current sample.* "the render progress as a string containing current frame, current sample, render time and saving time." (`render_stats`, https://docs.blender.org/api/5.3/bpy.app.handlers.html, read 2026-10-11). The text does not say whether the requested number of samples is printed beside it.

11.2 — **documented / observed** — *A frame can finish with fewer samples than requested.* This is why the question matters.
- Time Limit: "Stops rendering if the time exceeds the limit, even if the desired sample count hasn’t been reached yet." (https://docs.blender.org/manual/en/5.3/render/cycles/render_settings/sampling.html, read 2026-10-11).
- Adaptive sampling: "cutting short the sampling process in areas that have become less noisy than the specified threshold value" (same page).
- **observed:** adaptive sampling is on by default. `properties.py` lines 620–624: `use_adaptive_sampling` has `default=True`. The requested count `samples` has `min=1, max=(1 << 24)` and `default=4096` (lines 533–538). The time limit `time_limit` has `default=0.0`, which "Zero disables the limit" (lines 568–575). Read 2026-10-16.

11.3 — **documented / observed** — *A picture, not a printed line, shows samples per pixel.* The Sample Count pass is an image layer, not printed text:
> "Number of samples calculated for each pixel, divided by the maximum number of samples. Used to analyze adaptive sampling."
Manual — https://docs.blender.org/manual/en/5.3/render/layers/passes.html — read 2026-10-11. **observed:** `properties.py` lines 1575–1580, `pass_debug_sample_count`, "Number of samples per pixel taken, divided by the maximum number of samples. To analyze adaptive sampling", default False (read 2026-10-16).

**Not covered.** **Not in the sources:** any printed line showing the requested sample count, the number reached, or a flag that a frame stopped short. Whether the "current sample" in 11.1 is a frame-wide count, a per-tile count, or something else is not stated. Whether the time limit or adaptive stopping prints anything is not stated.

**Where I looked.** I searched the manual, the API, the release notes and Cycles' files for "sample count", "samples", "current sample", "Sample ", "time limit", "adaptive", "reached", "stats" and "statistics". I read the Cycles sampling page, the passes page, `render_stats`, and the Cycles sampling and pass settings in `properties.py` and `ui.py`.

**Observed by hand test, 2026-10-17 — the sample lines of a frame that took all its samples (hand-test entry 1).** On the Creator's Mac, Blender 5.3.0 Beta, set up as described at the top of this document: `"$B53" -b <scene folder>/base.blend -o "<work folder>/out/e01/e01_" -F PNG -f 1`, Cycles on the processor, 16 samples, adaptive sampling off. Exactly three lines carried a sample count, and they print the maximum beside the count:

```
00:00.329  render           | Fra: 1 | Mem: 1M | Sample 0/16
00:00.330  render           | Fra: 1 | Remaining: 00:00.01 | Mem: 2M | Sample 1/16
00:00.450  render           | Fra: 1 | Mem: 2M | Sample 16/16
```

The count runs from nought to the maximum and was never printed once per sample.

**Observed by hand test, 2026-10-17 — what the `render_stats` handler is handed, with no window (hand-test entry 4).** On the Creator's Mac, Blender 5.3.0 Beta, set up as described at the top of this document. A start-up script attached one handler to `bpy.app.handlers.render_stats` that printed `RENDER_STATS:` and the `repr` of its arguments on every call, and one to `render_complete` that printed the count. Commands: `"$B53" -b <scene folder>/base.blend --python stats_probe.py -o "<work folder>/out/e04/e04a_" -F PNG -f 1` and the same with `-x 1 -a` in place of `-f 1`. Both printed `exit 0`; both error streams were 0 bytes. **It fired 45 times for one frame and 133 times for three**, and the run's own count line read `RENDER_STATS FIRED 45 TIMES`. The first string, the sampling strings and the last three of the one-frame run, word for word:

```
RENDER_STATS: ('Mem: 0M | Synchronizing object | Cube', None)
RENDER_STATS: ('Mem: 1M | Sample 0/16', None)
RENDER_STATS: ('Remaining: 00:00.01 | Mem: 2M | Sample 1/16', None)
RENDER_STATS: ('Mem: 2M | Sample 16/16', None)
RENDER_STATS: ('Mem: 2M | Finished', None)
RENDER_STATS: ('Time: 00:00.15 (Saving: 00:00.06)', None)
```

In the three-frame run the `Remaining:` string arrived once in each frame, word for word the same each time, each just after the printed line it matches:

```
00:00.329  render           | Fra: 1 | Remaining: 00:00.01 | Mem: 2M | Sample 1/16
RENDER_STATS: ('Remaining: 00:00.01 | Mem: 2M | Sample 1/16', None)
00:00.487  render           | Fra: 2 | Remaining: 00:00.01 | Mem: 2M | Sample 1/16
RENDER_STATS: ('Remaining: 00:00.01 | Mem: 2M | Sample 1/16', None)
00:00.576  render           | Fra: 3 | Remaining: 00:00.01 | Mem: 2M | Sample 1/16
RENDER_STATS: ('Remaining: 00:00.01 | Mem: 2M | Sample 1/16', None)
```

So the handler is handed two arguments, a string and `None`; the string is the message part of the printed line, with the separator ` | `; **none of the 45 strings carries a frame number** (the printed lines do, as `Fra: 1`); the current sample and its maximum arrive as `Sample <n>/<max>` while sampling runs; and the render time and saving time arrive together in the last string only, as `Time: … (Saving: …)`.

**Observed by hand test, 2026-10-17 — where Cycles' own statistics land, and what they hold (hand-test entry 5).** On the Creator's Mac, Blender 5.3.0 Beta, set up as described at the top of this document. Command: `"$B53" -b <scene folder>/base.blend -o "<work folder>/out/e05/e05a_" -F PNG -f 1 -- --cycles-print-stats > out.txt 2> err.txt`. It printed `exit 0` and wrote its picture. **The error stream was 0 bytes; the statistics were on standard output**, which held 132 lines. So the help text quoted in 4.1 above, "Print rendering statistics to stderr", does not describe where they appeared in this run. Where the blocks sit, by line number in standard output:

```
15:Update statistics:
82:00:00.427  render           | Fra: 1 | Mem: 2M | Sample 16/16
83:00:00.427  render           | Fra: 1 | Mem: 2M | Finished
84:00:00.490  render           | Saved: '<work folder>/out/e05/e05a_0001.png'
85:00:00.490  render           | Time: 00:00.16 (Saving: 00:00.06)
86:Render statistics:
```

So the second block, `Render statistics:`, comes **after** `Saved:` and `Time:`. The block is printed plain — no time, no category, no vertical bar. Its opening and closing lines, word for word (the kernel lines between them are times and percentages, cut here for length; there are 36 of them):

```
Render statistics:
Mesh statistics:
  Geometry:
    Total memory: 16.41K (16,800)
        Sphere                           16.02K (16,405)
        Cube                             289B (289)
        Area                             56B (56)
        backgroundlight                  50B (50)
Image statistics:
  Total image memory: 0B (0)
Kernel statistics:
  Total render time               : Total 100.00% (0.14s), Self 17.65% (0.02s)
…
Shader statistics:
  Material                        : 0.02s (Relative cost: 1.15)
  default_surface                 : 0.01s (Relative cost: 0.81)
Object statistics:
  Cube                            : 0.02s (Relative cost: 1.13)
  Sphere                          : 0.01s (Relative cost: 0.83)
```

Its memory figures are labelled by what the memory holds — geometry, image — and carry a unit letter and an exact count in brackets. **No sample count appears in either block**: searching the whole output for "sample", in any case, found only the three ordinary sample lines.

*Without the bare `--`.* The same command with `--cycles-print-stats` placed straight after `-f 1`, with no `--` before it, printed `exit 1`. It rendered and saved the frame first — `Saved: '<work folder>/out/e05/e05b_0001.png'` and `Time: 00:00.16 (Saving: 00:00.06)` are in its output — and printed no statistics block. Then standard output ended:

```
00:00.488  reports          | ERROR Cannot read file "<work folder>/--cycles-print-stats": No such file or directory

Blender quit
```

and the error stream held one line:

```
unknown argument, loading as file: --cycles-print-stats
```

**Observed by hand test, 2026-10-17 — EEVEE and Workbench, and the engine's name (hand-test entry 6).** On the Creator's Mac, Blender 5.3.0 Beta, set up as described at the top of this document, on `base.blend` (whose own engine is Cycles at 16 samples).

*The names.* A start-up script set `bpy.context.scene.render.engine` to four names in turn and printed what happened. The whole of what it printed:

```
ENGINE ACCEPTED: BLENDER_EEVEE
ENGINE RAISED: BLENDER_EEVEE_NEXT TypeError bpy_struct: item.attr = val: enum "BLENDER_EEVEE_NEXT" not found in ('BLENDER_EEVEE', 'BLENDER_WORKBENCH', 'CYCLES')
ENGINE ACCEPTED: BLENDER_WORKBENCH
ENGINE ACCEPTED: CYCLES
```

*EEVEE.* `"$B53" -b <scene folder>/base.blend -E BLENDER_EEVEE -o "<work folder>/out/e06/eevee_" -F PNG -f 1` printed `exit 0`, wrote its picture, left the error stream at 0 bytes, and printed 14 lines. The whole of them:

```
00:00.321  blend            | Read blend: "<scene folder>/base.blend"
00:00.327  render           | Rendering single frame
00:00.327  render           | Rendering frame 1
00:00.329  render           | Start rendering: Scene, ViewLayer
00:00.329  render           | Engine: EEVEE
00:00.472  render           | Fra: 1 | Rendering 1 / 64 samples
Blender 5.3.0 Beta (hash 5d8151335b6b built 2026-10-16 01:36:01)
00:00.683  render           | Fra: 1 | Rendering 25 / 64 samples
00:00.838  render           | Fra: 1 | Rendering 50 / 64 samples
00:00.925  render           | Fra: 1 | Rendering 64 / 64 samples
00:01.017  render           | Saved: '<work folder>/out/e06/eevee_0001.png'
00:01.017  render           | Time: 00:00.68 (Saving: 00:00.07)

Blender quit
```

So EEVEE's progress line is shaped `Fra: 1 | Rendering <n> / <max> samples`, printed at 1, 25, 50 and 64; it carries **no memory figure, no `Remaining:` field and no stage lines**, and it counted to 64 although the scene asks Cycles for 16, printing nothing about the difference.

*Workbench.* `"$B53" -b <scene folder>/base.blend -E BLENDER_WORKBENCH -o "<work folder>/out/e06/wb_" -F PNG -f 1` printed `exit 0`, wrote its picture, left the error stream at 0 bytes, and printed 10 lines. The whole of them:

```
00:00.334  blend            | Read blend: "<scene folder>/base.blend"
00:00.339  render           | Rendering single frame
00:00.339  render           | Rendering frame 1
00:00.339  render           | Start rendering: Scene, ViewLayer
00:00.339  render           | Engine: Workbench
00:00.507  render           | Saved: '<work folder>/out/e06/wb_0001.png'
00:00.507  render           | Time: 00:00.16 (Saving: 00:00.06)
Blender 5.3.0 Beta (hash 5d8151335b6b built 2026-10-16 01:36:01)

Blender quit
```

**Workbench printed no progress line and no sample line at all.** Neither engine mentioned a tile.

**Observed by hand test, 2026-10-17 — what is printed when a frame stops short of its samples (hand-test entry 26).** On the Creator's Mac, Blender 5.3.0 Beta, set up as described at the top of this document, on `samples.blend` (Cycles on the processor, 4096 samples, adaptive sampling off, 480 by 360). Three renders with no window, each `"$B53" -b <scene folder>/samples.blend [--python-expr …] -o "<work folder>/out/e26/<name>_" -F PNG -f 1`: plain; with `bpy.context.scene.cycles.time_limit = 5.0`; and with `use_adaptive_sampling = True` and `adaptive_threshold = 0.1`. **All three ended at exit 0, wrote their picture, and left the error stream empty.** Every sample line of each, and the lines straight after the last one, word for word:

*Plain:*
```
00:00.329  render           | Fra: 1 | Mem: 1M | Sample 0/4096
00:00.339  render           | Fra: 1 | Remaining: 00:37.71 | Mem: 10M | Sample 1/4096
00:38.628  render           | Fra: 1 | Mem: 10M | Sample 4096/4096
00:38.628  render           | Fra: 1 | Mem: 10M | Finished
```

*Time limit of five seconds:*
```
00:00.331  render           | Fra: 1 | Mem: 1M | Sample 0/4096
00:00.340  render           | Fra: 1 | Remaining: 00:04.99 | Mem: 10M | Sample 1/4096
00:05.092  render           | Fra: 1 | Remaining: 00:00.23 | Mem: 10M | Sample 513/4096
00:05.393  render           | Fra: 1 | Mem: 10M | Sample 545/4096
00:05.651  render           | Fra: 1 | Mem: 10M | Finished
```

*Adaptive sampling at 0.1:*
```
00:00.334  render           | Fra: 1 | Mem: 1M | Sample 0/4096
00:00.344  render           | Fra: 1 | Remaining: 00:38.25 | Mem: 14M | Sample 1/4096
00:00.791  render           | Fra: 1 | Remaining: 00:38.48 | Mem: 14M | Sample 48/4096
00:00.792  render           | Fra: 1 | Remaining: 00:28.83 | Mem: 14M | Sample 64/4096
00:00.793  render           | Fra: 1 | Remaining: 00:23.01 | Mem: 14M | Sample 80/4096
00:00.794  render           | Fra: 1 | Remaining: 00:19.13 | Mem: 14M | Sample 96/4096
00:00.794  render           | Fra: 1 | Remaining: 00:16.36 | Mem: 14M | Sample 112/4096
00:00.795  render           | Fra: 1 | Remaining: 00:14.28 | Mem: 14M | Sample 128/4096
00:00.796  render           | Fra: 1 | Remaining: 00:12.66 | Mem: 14M | Sample 144/4096
00:01.057  render           | Fra: 1 | Remaining: 00:17.76 | Mem: 14M | Sample 160/4096
00:01.260  render           | Fra: 1 | Remaining: 00:22.76 | Mem: 14M | Finished
```

So **the count reached is printed, and it is the last sample line of the frame**: `Sample 545/4096` under the time limit and `Sample 160/4096` under adaptive sampling, each followed straight by `Finished`. **Nothing else is printed** — searching both outputs for "time limit", "noise", "threshold" and "adaptive", in any case, matched nothing.

**Observed by hand test, 2026-10-17 — what a saved picture records about samples, and whether the Sample Count pass reaches a file (hand-test entry 27).** On the Creator's Mac, Blender 5.3.0 Beta, set up as described at the top of this document, on `samples.blend` with its samples set to 512 and adaptive sampling switched on at 0.1 from a `--python-expr`. Every file below was read back by a plain Python script that reads the file's own bytes, not by Blender.

*A PNG.* `… -o "<work folder>/out/e27/png_" -F PNG -f 1`: exit 0, error stream empty. Its sample lines climbed to `Sample 96/512` and stopped there. The PNG's text fields, whole:

```
PNG png_0001.png 480x360
   File (tEXt) = <scene folder>/samples.blend
   Date (tEXt) = 2026/10/17 05:09:52
   Time (tEXt) = 00:00:00:01
   Frame (tEXt) = 1
   Camera (tEXt) = Camera
   Scene (tEXt) = Scene
   RenderTime (tEXt) = 00:00.91
   cycles.ViewLayer.samples (tEXt) = 512
   cycles.ViewLayer.total_time (tEXt) = 00:00.91
   cycles.ViewLayer.render_time (tEXt) = 00:00.91
   cycles.ViewLayer.synchronization_time (tEXt) = 00:00.00
```

So **`cycles.ViewLayer.samples` holds 512 — the maximum asked for — and not the 96 reached.**

*An OpenEXR* (`-F OPEN_EXR`): exit 0. Its header fields:

```
EXR exr_0001.exr magic 762f3101 version-flags 02040000
   Camera (string) = Camera
   Date (string) = 2026/10/17 05:09:54
   File (string) = <scene folder>/samples.blend
   Frame (string) = 1
   RenderTime (string) = 00:00.92
   Scene (string) = Scene
   Software (string) = Blender 5.3.0 Beta
   Time (string) = 00:00:00:01
   channels (chlist) = A, B, G, R
   colorInteropID (string) = lin_rec709_scene
   compression (compression) 1 bytes
   cycles.ViewLayer.render_time (string) = 00:00.91
   cycles.ViewLayer.samples (string) = 512
   cycles.ViewLayer.synchronization_time (string) = 00:00.00
   cycles.ViewLayer.total_time (string) = 00:00.91
   dataWindow (box2i) 16 bytes
   displayWindow (box2i) 16 bytes
   lineOrder (lineOrder) 1 bytes
   name (string) = 
   pixelAspectRatio (float) 4 bytes
   screenWindowCenter (v2f) 8 bytes
   screenWindowWidth (float) 4 bytes
   type (string) = scanlineimage
   xDensity (float) 4 bytes
```

*A multi-layer OpenEXR with the Sample Count pass switched on.* The expression also set `sc.view_layers[0].cycles.pass_debug_sample_count = True`, `image_settings.media_type = 'MULTI_LAYER_IMAGE'` and `image_settings.file_format = 'OPEN_EXR_MULTILAYER'`, and printed `PASS SET True MULTI_LAYER_IMAGE OPEN_EXR_MULTILAYER`. Exit 0, error stream empty, `ml_0001.exr` written. Its channel list, read from the file:

```
channels (chlist) = ViewLayer.Combined.A, ViewLayer.Combined.B, ViewLayer.Combined.G, ViewLayer.Combined.R
```

**Only Combined: the Sample Count pass did not reach the file**, and nothing was printed about it.

*Setting only the format.* An expression that set `file_format = 'OPEN_EXR_MULTILAYER'` without first setting `media_type` raised on the error stream, whole:

```
Traceback (most recent call last):
  File "<string>", line 1, in <module>
TypeError: bpy_struct: item.attr = val: enum "OPEN_EXR_MULTILAYER" not found in ('AVIF', 'JPEG', 'OPEN_EXR', 'PNG', 'WEBP', 'BMP', 'CINEON', 'DPX', 'IRIS', 'JPEG2000', 'HDR', 'TARGA', 'TARGA_RAW', 'TIFF')
```

and the render went ahead, wrote `fmt_0001.png` — a PNG — and ended at exit 0.

**Observed by hand test, 2026-10-17 — with the window open, a script's render prints only the saved file (hand-test entry 29).** On the Creator's Mac, Blender 5.3.0 Beta, started with its window from a terminal on `base.blend`, a render called from the start-up script with `bpy.ops.render.render(write_still=True)` put one render line on the terminal — `00:00.780  render           | Saved: '<work folder>/out/e29/direct1_.png'` — and no progress line, while a `render_stats` handler attached by the same script fired 45 or 46 times in each of five runs. At the end the run printed `Saved session recovery to "…/quit.blend"`, a blank line and `Blender quit`. The whole record is under question 9.

**Outside the interface.** The printed statistics and the Sample Count pass do not depend on the interface. The sources do not say whether the statistics string is printed when running without the interface. **Settled in part by the hand tests of 2026-10-17.** With no window the count reached is printed, and in this version **it is the last sample line of the frame** (`Sample 545/4096` under a time limit, then `Finished`), with nothing naming the limit or the threshold. A saved picture records the maximum (`cycles.ViewLayer.samples`), not the count reached, and the Sample Count pass did not reach a multi-layer file. With the window no sample line reaches the terminal, but the handler receives them.

**Operating system.** Not stated in the sources. **Hand tests, 2026-10-17:** everything observed under this question was run on **the Creator's Mac (Apple M1 Pro, macOS 26.6.2) in Blender 5.3.0 Beta**, so it settles macOS on Apple silicon and nothing else. **Linux and Windows were not run in 5.3** — this sitting had no Linux or Windows machine — so they are untried, not untestable (the equipment this project can reach is set out in `Preparation/Blender Research/Method.md`); on Windows an edition would have to be named before anything was claimed.

---

## 12. What ending status does a run of Blender give back to whatever started it, for each of the ways a run can end?

**Overall: mostly not in the sources.** No source gives the status number for a normal finish, for an interruption, or for a refused input in an ordinary render run. What the sources do say follows.

12.1 — **documented** — *Python exceptions in a command-line script.* There is an option to set the exit status Blender uses when a Python error is raised in a script run from the command line.
> `--python-exit-code <code>` — "Set the exit-code in [0..255] to exit if a Python exception is raised (only for scripts executed from the command line), zero disables."
Manual — https://docs.blender.org/manual/en/5.3/advanced/command_line/arguments.html — read 2026-10-11.

12.2 — **inferred** — *A render that fails inside a command-line script can end with that status.* The steps:
1. A render called from Python that produces error reports raises a `RuntimeError` (1.11).
2. A render with no output method "will report errors" (1.6).
3. With `--python-exit-code N`, an uncaught Python error in a command-line script makes Blender exit with N (12.1).

This covers only renders started from a script, and only failures that make error reports.

12.3 — **documented / observed / inferred** — *Command runs started with `-c`.* These are not render runs. For commands started with `-c`, the command's function "returns an int" and "The return value should be 0 for success, 1 on failure (specific error codes from the os module can also be used)." (`register_cli_command`, https://docs.blender.org/api/5.3/bpy.utils.html, read 2026-10-11). **observed:** Cycles' own `maketx` command writes `Error: …` to standard error and returns 1 on failure. On success it prints the output path and returns 0 (`maketx.py` lines 82–87, read 2026-10-16). **inferred:** the returned number becomes the run's ending status. The step: the API calls these return values success and failure codes and mentions the operating system's error codes. It does not say outright that the starting program receives them.

12.4 — **documented / observed** — *Refusals and interruptions.*
- Unknown argument: Blender "will exit" (quote in 1.12). The status is not given.
- Invalid `--cycles-device` value: the Cycles add-on calls `sys.exit(1)` (**observed**, `engine.py` line 41), and `sys.exit()` "will close Blender immediately" (**documented**, https://docs.blender.org/api/5.3/info_gotchas_crashes.html). The sources do not say that the 1 is what the starting program receives.
- Exit hook: `exit_pre` receives `False` when Blender "is running in background mode, or is exiting due to failed command line arguments, etc." (https://docs.blender.org/api/5.3/bpy.app.handlers.html, read 2026-10-11). This is a value passed to a script, not the ending status.
- Interruption: "Ctrl-C in the terminal (Ctrl-Break on Windows) will quit the script early." (https://docs.blender.org/api/5.3/info_tips_and_tricks.html, read 2026-10-11). See also 1.10. The status after an interruption is not given.
- Internal errors: `--debug-exit-on-error` — "Immediately exit when internal errors are detected." (arguments page). The status is not given.

**Not covered.** **Not in the sources:** the status for (a) a normal finish, (b) an interruption by Ctrl-C or a closed terminal, and (c) a refusal because a blend-file, argument or setting cannot be used. A crash's status is also not given. Whether a failed frame in a `-b … -a` run changes the status is not stated.

**Where I looked.** I searched the manual, the API and the release notes for "exit", "exit code", "exit-code", "exit status", "exit_code", "return code", "returncode", "status code", "error code", "errorlevel", "non-zero", "sys.exit", "quit", "SIGINT", "SIGTERM", "signal", "Ctrl-C" and "Ctrl-Break". In Cycles' files I searched for `sys.exit` and `return 1`. I read the arguments page in full, `bpy.app.handlers`, `bpy.utils`, info_gotchas_crashes, info_tips_and_tricks and info_advanced_blender_as_bpy.

**Observed by hand test, 2026-10-17 — a render that finished normally (hand-test entry 1).** On the Creator's Mac, Blender 5.3.0 Beta, set up as described at the top of this document: `"$B53" -b <scene folder>/base.blend -o "<work folder>/out/e01/e01_" -F PNG -f 1; echo "exit $?"` printed `exit 0` and the picture was written. Three more ways of finishing are under hand-test entry 15 below.

**Observed by hand test, 2026-10-17 — runs refused before anything renders (hand-test entry 10).** On the Creator's Mac, Blender 5.3.0 Beta, set up as described at the top of this document. Each command's two streams went to separate files and the exit number was read straight after. Nothing was written to any output folder except by the `METAL` run.

*A blend-file that is not there* — `"$B53" -b "<scene folder>/there_is_no_such_file.blend" -o "<work folder>/out/e10/a_" -f 1`: **exit 1**. Error stream empty. Standard output, whole:
```
00:00.321  reports          | ERROR Cannot read file "<scene folder>/there_is_no_such_file.blend": No such file or directory
Blender 5.3.0 Beta (hash 5d8151335b6b built 2026-10-16 01:36:01)

Blender quit
```

*A file that is not a blend-file* (one line of plain text saved as `notblend.blend`): **exit 1**. Error stream empty. Standard output, whole:
```
00:00.318  reports          | ERROR File format is not supported in file "<scene folder>/notblend.blend"
Blender 5.3.0 Beta (hash 5d8151335b6b built 2026-10-16 01:36:01)

Blender quit
```

*A blend-file cut short* (`truncated.blend`, the first 50,000 bytes of `good.blend`): **exit 1**. Error stream empty. Standard output, whole:
```
00:00.321  blend            | Read blend: "<scene folder>/truncated.blend"
00:00.321  reports          | ERROR Failed to read blend file '<scene folder>/truncated.blend': Missing DNA block
Blender 5.3.0 Beta (hash 5d8151335b6b built 2026-10-16 01:36:01)

Blender quit
```

*The manual's own unknown-argument example*, `"$B53" -ba <scene folder>/good.blend` (run under a 90-second cap): **it did not end by itself.** The cap printed `[harness] still running after 90 s; sent SIGKILL`, so there is no ending status to report. Standard output, whole:
```
00:00.551  reports          | ERROR Cannot read file "<work folder>/-ba": No such file or directory
00:00.551  blend            | Read blend: "<scene folder>/good.blend"
```
and the error stream, whole:
```
unknown argument, loading as file: -ba
```

*An argument Blender does not know, after a valid one* — `"$B53" -b <scene folder>/good.blend --no-such-flag -o "<work folder>/out/e10/e_####" -f 1`: **exit 1**, nothing written. Standard output, whole:
```
00:00.335  blend            | Read blend: "<scene folder>/good.blend"
00:00.341  reports          | ERROR Cannot read file "<work folder>/--no-such-flag": No such file or directory
Blender 5.3.0 Beta (hash 5d8151335b6b built 2026-10-16 01:36:01)

Blender quit
```
and the error stream, whole:
```
unknown argument, loading as file: --no-such-flag
```

*An engine name that does not exist* — `"$B53" -b <scene folder>/good.blend -E NOSUCHENGINE -o "<work folder>/out/e10/f_####" -f 1`: **exit 1**, nothing written. Standard output, whole:
```
00:00.321  blend            | Read blend: "<scene folder>/good.blend"
Blender 5.3.0 Beta (hash 5d8151335b6b built 2026-10-16 01:36:01)
```
and the error stream, whole (its first line is empty):
```

Error: engine not found 'NOSUCHENGINE'
```

*A render device given to Cycles* — `"$B53" -b <scene folder>/good.blend -o "<work folder>/out/e10/dev_<name>_####" -f 1 -- --cycles-device <name>`, six names:

| Name given | Exit | Picture written |
| --- | --- | --- |
| `NONSENSE` | **1** | no |
| `METAL` | **0** | yes, `dev_METAL_0001.png`, 55 lines printed |
| `CUDA` | **1** | no |
| `OPTIX` | **1** | no |
| `HIP` | **1** | no |
| `ONEAPI` | **1** | no |

Every error stream was empty. For `NONSENSE`, standard output, whole:
```
00:00.313  cycles           | ERROR NONSENSE is not a valid Cycles device.
Blender 5.3.0 Beta (hash 5d8151335b6b built 2026-10-16 01:36:01)
Error: Not freed memory blocks: 1, total unfreed memory 0.000004 MB
```
For `CUDA` — and `OPTIX`, `HIP` and `ONEAPI` printed the same lines apart from their times — standard output, whole:
```
00:00.367  blend            | Read blend: "<scene folder>/good.blend"
00:00.372  render           | Rendering single frame
00:00.372  render           | Rendering frame 1
00:00.372  render           | Start rendering: Scene, ViewLayer
00:00.372  render           | Engine: Cycles
00:00.376  render           | Fra: 1 | Mem: 0M | Initializing
Blender 5.3.0 Beta (hash 5d8151335b6b built 2026-10-16 01:36:01)
00:00.376  reports          | ERROR Found no Cycles device of the specified type

Blender quit
```

**Observed by hand test, 2026-10-17 — the ending status of renders that fail during or after rendering (hand-test entry 11).** On the Creator's Mac, Blender 5.3.0 Beta, with no window; every command and line is quoted under question 1. The exit numbers:

| What was run | Exit | Was a picture written |
| --- | --- | --- |
| a scene with no camera, `-f 1` | **0** | no |
| an output folder that does not exist | 0 | yes — the folders were made |
| an output path blocked by a plain file | 1 | no |
| a broken template in the output path, `-a` | 1 | no |
| an output path of `//` | 0 | yes, beside the blend-file |
| saving switched off, nothing else arranged, `-a` | **0** | no |
| the same with `-f 1` | **0** | no |
| the same, the render called from Python, with `--python-exit-code 7` | **7** | no |

So in this version **two failures that write nothing end at 0**: a scene with no camera, and saving switched off with nothing else arranged. The exit-code option turned the second into 7 only when the render was called from Python.

**Observed by hand test, 2026-10-17 — an animation whose output folder stops being writable part-way (hand-test entry 12).** On the Creator's Mac, Blender 5.3.0 Beta, set up as described at the top of this document, on `partway.blend` (2000 samples, 320 by 240, frames 1 to 6, about seven seconds a frame). One command started the render in the background with `-o "<work folder>/out/e12/out_53/frame_" -F PNG -x 1 -a`, waited a fixed twelve seconds, listed the output folder — it held `frame_0001.png` — renamed the folder away to `out_53_moved`, and put a one-byte plain file named `out_53` in its place; then it waited for the render to end. The render ended at **exit 1**. The error stream was empty. Standard output held 104 lines; every line naming a frame, a save, an error or a time, with its line number, word for word:

```
3:00:00.330  render           | Rendering frame 1
51:00:07.099  render           | Saved: '<work folder>/out/e12/out_53/frame_0001.png'
52:00:07.099  render           | Time: 00:06.76 (Saving: 00:00.06)
53:00:07.099  render           | Rendering frame 2
100:00:13.800  image            | ERROR Couldn't create directory for file <work folder>/out/e12/out_53/frame_0002.png: File exists
101:00:13.800  reports          | ERROR Render error (File exists) cannot save: '<work folder>/out/e12/out_53/frame_0002.png'
102:00:13.800  render           | Time: 00:06.70
```

and the run's last lines were `Time: 00:06.70`, a blank line and `Blender quit`. **No `Rendering frame 3` line, or any line for frames 3 to 6, was printed** — the animation stopped at the frame it could not save. The renamed folder held one file, `frame_0001.png` (65,499 bytes); no part-written file was left anywhere.

**Observed by hand test, 2026-10-17 — which handlers fire when a render fails, what the reports list holds, what a script's own render call gets, and where a log file puts a failure (hand-test entry 13).** On the Creator's Mac, Blender 5.3.0 Beta, set up as described at the top of this document, with no window.

*Which handlers fire.* A start-up script attached a printing handler to each of `render_init`, `render_pre`, `render_post`, `render_write`, `render_complete` and `render_cancel` (and a silent one to `render_stats`), printed `PROBE INSTALLED`, and would have printed `HANDLER MISSING:` for any that did not exist; none was missing. Six runs, each `"$B53" -b <file> --python handlers_probe.py -o <path> -F PNG -f 1` (or `-x 1 -a`). Every line each printed from the probe, and every `ERROR` or `WARNING` line, in order:

```
— good render (simple.blend), exit 0
PROBE INSTALLED
HANDLER FIRED: render_init args: (bpy.data.scenes['Scene'], None)
HANDLER FIRED: render_pre args: (bpy.data.scenes['Scene'], None)
HANDLER FIRED: render_post args: (bpy.data.scenes['Scene'], None)
HANDLER FIRED: render_write args: (bpy.data.scenes['Scene'], None)
HANDLER FIRED: render_complete args: (bpy.data.scenes['Scene'], None)
— no camera (nocamera.blend), exit 0
PROBE INSTALLED
00:00.323  reports          | ERROR No camera found in scene "Scene"
— output path blocked by a plain file, exit 1
PROBE INSTALLED
HANDLER FIRED: render_init args: (bpy.data.scenes['Scene'], None)
HANDLER FIRED: render_pre args: (bpy.data.scenes['Scene'], None)
00:00.453  image            | ERROR Couldn't create directory for file <work folder>/out/e13/blocker/frame_0001.png: File exists
00:00.453  reports          | ERROR Render error (File exists) cannot save: '<work folder>/out/e13/blocker/frame_0001.png'
HANDLER FIRED: render_cancel args: (bpy.data.scenes['Scene'], None)
— broken template in the output path, -x 1 -a, exit 1
PROBE INSTALLED
HANDLER FIRED: render_init args: (bpy.data.scenes['Scene'], None)
00:00.325  reports          | ERROR Parse errors in path '<work folder>/out/e13/tmpl/{blend_name.png':
HANDLER FIRED: render_cancel args: (bpy.data.scenes['Scene'], None)
— missing image texture (missing.blend, gone.png moved aside), exit 0
PROBE INSTALLED
HANDLER FIRED: render_init args: (bpy.data.scenes['Scene'], None)
HANDLER FIRED: render_pre args: (bpy.data.scenes['Scene'], None)
00:00.334  cycles           | WARNING Image file <scene folder>/gone.png does not exist.
00:00.335  cycles           | ERROR Failed to load 1 image files
HANDLER FIRED: render_post args: (bpy.data.scenes['Scene'], None)
HANDLER FIRED: render_write args: (bpy.data.scenes['Scene'], None)
HANDLER FIRED: render_complete args: (bpy.data.scenes['Scene'], None)
— saving switched off by --python-expr, -x 1 -a, exit 0
PROBE INSTALLED
00:00.325  reports          | ERROR Render output disabled in Output properties and no active compositing File Output nodes
```

Every error stream was empty. So **on a scene with no camera not one handler fired** — not even `render_init` — and the same is true of saving switched off. A blocked save fired `render_init`, `render_pre` and `render_cancel`; a broken template fired `render_init` and `render_cancel` only; a missing texture, which succeeds, fired the full set a good render fires.

*The reports list after a failure with no window.* `"$B53" -b <scene folder>/nocamera.blend -o "<work folder>/out/e13/nc2_" -F PNG -f 1 --python reports.py`, the script printing `len(bpy.context.window_manager.reports)` and each entry. **Exit 0**, error stream empty, and standard output, whole:

```
00:00.317  blend            | Read blend: "<scene folder>/nocamera.blend"
00:00.321  reports          | ERROR No camera found in scene "Scene"
Blender 5.3.0 Beta (hash 5d8151335b6b built 2026-10-16 01:36:01)
REPORTS COUNT: 0

Blender quit
```

*A render called from a script.* A start-up script called `bpy.ops.render.render(write_still=True)` inside a `try` and printed what came back. On `simple.blend` it printed `RENDER RETURNED: {'FINISHED'}` and the run ended at exit 0. On `nocamera.blend` the run ended at **exit 1**, the error stream was empty, and standard output, whole:

```
00:00.322  blend            | Read blend: "<scene folder>/nocamera.blend"
Error: Cannot render, no camera
RENDER RAISED: RuntimeError Error: Cannot render, no camera
Blender 5.3.0 Beta (hash 5d8151335b6b built 2026-10-16 01:36:01)

Blender quit
```

So the script's call **raised a `RuntimeError` carrying `Error: Cannot render, no camera`**, and the run ended at 1 although the script caught it — while the same file rendered with `-f 1` says `No camera found in scene "Scene"` and ends at 0 (hand-test entry 11).

*A log file on a failure.* `"$B53" -b <scene folder>/nocamera.blend --log-file "<work folder>/logs/e13j.log" -o "<work folder>/out/e13/lf_" -F PNG -f 1`: **exit 0**, error stream empty; standard output held only the version banner, a blank line and `Blender quit`, and the file held, whole:

```
00:00.320  blend            | Read blend: "<scene folder>/nocamera.blend"
00:00.324  reports          | ERROR No camera found in scene "Scene"
```

**Observed by hand test, 2026-10-17 — three ways of finishing normally (hand-test entry 15).** On the Creator's Mac, Blender 5.3.0 Beta, set up as described at the top of this document, on `good.blend`:

```
"$B53" -b <scene folder>/good.blend -o "<work folder>/out/e15/f_####" -f 1
"$B53" -b <scene folder>/good.blend -o "<work folder>/out/e15/a_####" -a
"$B53" -b <scene folder>/good.blend
```

**All three ended at exit 0.** The single frame wrote `f_0001.png`; the animation wrote `a_0001.png`, `a_0002.png` and `a_0003.png`; the third rendered nothing and still ended at 0, printing, whole:

```
00:00.321  blend            | Read blend: "<scene folder>/good.blend"
Blender 5.3.0 Beta (hash 5d8151335b6b built 2026-10-16 01:36:01)

Blender quit
```

with its error stream empty, as were the other two.

**Observed by hand test, 2026-10-17 — Ctrl-C typed into a real terminal (hand-test entry 16).** On the Creator's Mac, Blender 5.3.0 Beta, set up as described at the top of this document, on `slow.blend` (800 by 800, a million samples, so one frame runs for hours). Each run was started inside a real terminal made with Python's `pty.fork()`, and the interrupt byte `0x03` — what a terminal receives when a person presses Ctrl-C — was written into that terminal from outside at a fixed time; the harness then read everything the terminal showed and how the process ended. A terminal carries both streams together, so the two are not told apart here. Below, everything the terminal showed is quoted except the run of stage lines; the lines beginning `[harness]` are the harness's own.

*One press, a single frame* — `"$B53" -b <scene folder>/slow.blend -o "<work folder>/out/e16/one_####" -f 1`:

```
[harness] wrote 0x03 at 12.037 s
Blender 5.3.0 Beta (hash 5d8151335b6b built 2026-10-16 01:36:01)
00:00.323  blend            | Read blend: "<scene folder>/slow.blend"
00:00.328  render           | Rendering single frame
00:00.328  render           | Rendering frame 1
00:00.328  render           | Start rendering: Scene, ViewLayer
00:00.328  render           | Engine: Cycles
00:00.333  render           | Fra: 1 | Mem: 1M | Sample 0/1000000
00:00.356  render           | Fra: 1 | Remaining: 06:22:12.76 | Mem: 35M | Sample 1/1000000
^C
Sent an internal break event. Press ^C again to kill Blender
00:25.755  render           | Fra: 1 | Remaining: 06:52:56.06 | Mem: 35M | Sample 1025/1000000
00:25.761  render           | Fra: 1 | Remaining: 06:53:02.43 | Mem: 35M | Cancelled

Blender quit
[harness] exited, status 1
```

*One press, an animation* — the same with `-a` in place of `-f 1`:

```
[harness] wrote 0x03 at 12.019 s
Blender 5.3.0 Beta (hash 5d8151335b6b built 2026-10-16 01:36:01)
00:00.321  blend            | Read blend: "<scene folder>/slow.blend"
00:00.326  render           | Rendering animation (frames 1..3)
00:00.326  render           | Rendering frame 1
00:00.326  render           | Start rendering: Scene, ViewLayer
00:00.326  render           | Engine: Cycles
00:00.331  render           | Fra: 1 | Mem: 1M | Sample 0/1000000
00:00.355  render           | Fra: 1 | Remaining: 06:35:24.43 | Mem: 35M | Sample 1/1000000
^C
Sent an internal break event. Press ^C again to kill Blender
00:25.778  render           | Fra: 1 | Remaining: 06:53:19.99 | Mem: 35M | Sample 1025/1000000
00:25.784  render           | Fra: 1 | Remaining: 06:53:26.28 | Mem: 35M | Cancelled

Blender quit
[harness] exited, status 1
```

*Two presses, three seconds apart, a single frame:*

```
[harness] wrote 0x03 at 12.060 s
[harness] wrote 0x03 at 15.069 s
Blender 5.3.0 Beta (hash 5d8151335b6b built 2026-10-16 01:36:01)
00:00.323  blend            | Read blend: "<scene folder>/slow.blend"
00:00.327  render           | Rendering single frame
00:00.327  render           | Rendering frame 1
00:00.327  render           | Start rendering: Scene, ViewLayer
00:00.327  render           | Engine: Cycles
00:00.332  render           | Fra: 1 | Mem: 1M | Sample 0/1000000
00:00.356  render           | Fra: 1 | Remaining: 06:32:23.39 | Mem: 35M | Sample 1/1000000
^C
Sent an internal break event. Press ^C again to kill Blender
^C
Blender killed
Error: Not freed memory blocks: 30168, total unfreed memory 41.431965 MB
[harness] exited, status 2
```

So **one press ended the run at status 1**, for a frame and for an animation alike, through a `| Cancelled` line, `Blender quit`, and no further frame; **two presses ended it at status 2** with `Blender killed`. The output folder was never made in any of the three runs, so **no picture of any kind was left**. One more thing the times show: after the single press, written at about 12 seconds, the render went on to `Sample 1025/1000000` and printed `Cancelled` at `00:25.76` in Blender's own clock — about thirteen seconds later.

**Observed by hand test, 2026-10-17 — a signal sent from another program (hand-test entry 17).** On the Creator's Mac, Blender 5.3.0 Beta, set up as described at the top of this document, on `slow.blend`. Each run was started from a separate Python program with its two streams sent to files — `"$B53" -b <scene folder>/slow.blend -o "<work folder>/out/e17/<signal>_####" -f 1` — and twelve seconds later that program sent the signal to Blender's process number and reported how it ended. The harness's own lines, word for word, then what each showed:

```
[harness] sent SIGINT to process 10490 at 12.0 s
[harness] exited, status 1
[harness] sent SIGTERM to process 10494 at 12.0 s
[harness] killed by signal 15 (a shell would report 143)
[harness] sent SIGKILL to process 10497 at 12.0 s
[harness] killed by signal 9 (a shell would report 137)
[harness] sent SIGHUP to process 10499 at 12.0 s
[harness] killed by signal 1 (a shell would report 129)
[harness] sent SIGINT to process 10501 at 12.0 s
[harness] exited, status 1
```

The last run was the same render with `-a` in place of `-f 1`. **Every error stream was empty, and the output folder was never made, so nothing was written in any of the five.** The two `SIGINT` runs ended their standard output the same way, the single frame:

```
Sent an internal break event. Press ^C again to kill Blender
00:25.789  render           | Fra: 1 | Remaining: 06:53:27.95 | Mem: 35M | Cancelled

Blender quit
```

and the animation with `00:25.938  render           | Fra: 1 | Remaining: 06:55:49.49 | Mem: 35M | Cancelled` in that place. The `SIGTERM`, `SIGKILL` and `SIGHUP` runs printed nothing after their last sample line, `Fra: 1 | Remaining: … | Mem: 35M | Sample 1/1000000`. So Blender turns `SIGINT` into the same orderly ending as a typed Ctrl-C, status 1; it does not catch `SIGTERM` or `SIGHUP` — `SIGHUP` being what a program receives when the terminal it was started from is closed.

**Observed by hand test, 2026-10-17 — a Python script that fails, with and without an exit code (hand-test entry 18).** On the Creator's Mac, Blender 5.3.0 Beta, set up as described at the top of this document, on `good.blend`. The script `boom.py` is one line, `raise Exception("boom")`.

| Command (after `"$B53" -b <scene folder>/good.blend`) | Exit | Picture written |
| --- | --- | --- |
| `--python boom.py -o "<work folder>/out/e18/a_####" -f 1` | **0** | yes |
| `--python-exit-code 42 --python boom.py -o … -f 1` | **42** | no |
| `--python boom.py --python-exit-code 42 -o … -f 1` | **0** | yes |
| `--python-expr "raise Exception('boom-expr')" -o … -f 1` | **0** | yes |
| `--python-exit-code 42 --python-expr "raise Exception('boom-expr')" -o … -f 1` | **42** | no |

So **without the option a failed script leaves the run at 0 and the render goes ahead**; the option works only when it comes before the script. The traceback lands on the **error stream**. With no option, the error stream held, whole:

```
Traceback (most recent call last):
  File "<work folder>/scripts/boom.py", line 1, in <module>
    raise Exception("boom")
Exception: boom
```

With the option first, the error stream held the same four lines and then:

```

Error: script failed, file: 'scripts/boom.py', exiting.
```

and standard output held only `Read blend: "<scene folder>/good.blend"`, the banner, a blank line and `Blender quit`. For the expression the last line read `Error: script failed, expr: 'raise Exception('boom-expr')', exiting.`

**Observed by hand test, 2026-10-17 — commands registered from Python, the command list, and `maketx` (hand-test entry 19).** On the Creator's Mac, Blender 5.3.0 Beta, set up as described at the top of this document.

*A registered command's own number.* A script registered two commands with `bpy.utils.register_cli_command`: `smoozyfail`, whose callback prints `smoozyfail callback ran` and returns `7`, and `smoozyok`, whose callback prints `smoozyok callback ran` and returns `0`. `"$B53" --python reg_cmd.py --command smoozyfail` printed its callback's line and ended at **exit 7**; `"$B53" --python reg_cmd.py --command smoozyok` printed its line and ended at **exit 0**. Both error streams were empty. So the number a registered command's callback returns becomes the run's ending status, and is not limited to 0 and 1.

*The command list.* `"$B53" --command help` ended at exit 0 and printed, whole:

```
Blender Command Listing:
	asset_listing
	extension
	maketx
```

*Cycles' `maketx` command* (its form, read from `"$B53" -c maketx --help`, is `Blender --command maketx [-h] [--colorspace COLORSPACE] [--alpha-type …] [--cache-dir CACHE_DIR] [filepath]`). Given a small PNG that is there — `"$B53" --command maketx "<work folder>/out/e19/tex_src.png"` — it ended at **exit 0**, wrote a `.tx` file into a new `blender_tx` folder beside the picture, left the error stream empty, and printed, whole:

```
<work folder>/out/e19/blender_tx/tex_src.png.0-B30E7CBFC8AAF789D08104D249114503.tx
```

Given a picture that is not there — `"$B53" --command maketx "<work folder>/out/e19/no_such_image.png"` — it ended at **exit 1** and **wrote on both streams at once**. Standard output, whole:

```
00:00.319  cycles           | ERROR Source image not found: <work folder>/out/e19/no_such_image.png
```

Error stream, whole:

```
Error: Source image not found
```

**Observed by hand test, 2026-10-17 — runs that render nothing and still end at 0 (hand-test entry 20).** On the Creator's Mac, Blender 5.3.0 Beta, set up as described at the top of this document, on `good.blend`. Each run had its own output folder that did not exist beforehand, so whether a folder appeared is a fact about that run.

| What followed `"$B53" -b <scene folder>/good.blend` | Exit | Picture written | Error stream |
| --- | --- | --- | --- |
| `-o "<work folder>/out/e20/ctl/f_####" -f 1` (the control) | 0 | yes | empty |
| `-o "…/abc/f_####" -f abc` | **0** | **no — no folder made** | a blank line, then `Error: not a number '-f / --render-frame abc'.` |
| `-o "…/empty/f_####" -f ""` | **0** | **no — no folder made** | a blank line, then `Error: incorrect comma use '-f / --render-frame '.` |
| `-o` with nothing after it | **0** | no | a blank line, then `Error: you must specify a path after '-o  / --render-output'.` |
| `--python-exit-code notanumber -o "…/nan/f_####" -f 1` | 0 | yes | a blank line, then `Error: not a number '--python-exit-code notanumber', expected number in [0..255].` |
| `-o "…/rev/f_####" -s 10 -e 5 -a` | **0** | **no — no folder made** | empty |

For the three bad-argument runs that rendered nothing, standard output held only `Read blend: "<scene folder>/good.blend"`, the banner, a blank line and `Blender quit`. The reversed range announced itself and rendered nothing; its standard output, whole:

```
00:00.323  blend            | Read blend: "<scene folder>/good.blend"
00:00.328  render           | Rendering animation (frames 10..5)
Blender 5.3.0 Beta (hash 5d8151335b6b built 2026-10-16 01:36:01)

Blender quit
```

So a frame argument that is not a number, an empty one, an option missing its value, and a start frame after the end frame **each end at 0 having rendered nothing**; where Blender complains, it complains on the **error stream**.

**Observed by hand test, 2026-10-17 — with Blender's window open: a render from the start-up script with the statistics handler attached, and two ways of closing (hand-test entry 29).** On the Creator's Mac, Blender 5.3.0 Beta, set up as described at the top of this document, **started with its window** (no `-b`) from a terminal: `"$B53" <scene folder>/base.blend -o "<work folder>/out/e29/<name>_" -F PNG --python win_stats.py`, run under a 240-second cap that would have ended it with `SIGKILL` and said so. The script attached a counting handler to `render_stats` and one to `render_complete`, called `bpy.ops.render.render(write_still=True)`, printed the counts, and then closed Blender one of two ways: `direct` called `bpy.ops.wm.quit_blender()` straight from the script; `timer` registered a timer that called it three seconds later. Run twice the first way and three times the second.

**Every run closed by itself and ended at exit 0, with an empty error stream; no crash file appeared in Blender's temporary folder.** The cap's own lines:

```
[harness] exited by itself after 1.5 s, status 0     (direct, run 1)
[harness] exited by itself after 1.4 s, status 0     (direct, run 2)
[harness] exited by itself after 4.3 s, status 0     (timer, run 1)
[harness] exited by itself after 4.4 s, status 0     (timer, run 2)
[harness] exited by itself after 4.2 s, status 0     (timer, run 3)
```

The first `direct` run's standard output, whole (the lines with no time are the script's own):

```
00:00.608  blend            | Read blend: "<scene folder>/base.blend"
BACKGROUND=False
VERSION=5.3.0 Beta
00:00.780  render           | Saved: '<work folder>/out/e29/direct1_.png'
RENDER RETURNED {'FINISHED'}
HANDLER FIRED 45 TIMES
FIRST=('Mem: 0M | Synchronizing object | Cube', None)
LAST=('Time: 00:00.16 (Saving: 00:00.06)', None)
RENDER_COMPLETE FIRED 1 TIMES
QUITWAY=direct
STILL HERE AFTER quit_blender()
00:00.960  blend            | Saved session recovery to "<work folder>/tmp/quit.blend"

Blender quit
```

and the first `timer` run's, whole:

```
00:00.557  blend            | Read blend: "<scene folder>/base.blend"
BACKGROUND=False
VERSION=5.3.0 Beta
00:00.734  render           | Saved: '<work folder>/out/e29/timer1_.png'
RENDER RETURNED {'FINISHED'}
HANDLER FIRED 45 TIMES
FIRST=('Mem: 0M | Synchronizing object | Cube', None)
LAST=('Time: 00:00.16 (Saving: 00:00.06)', None)
RENDER_COMPLETE FIRED 1 TIMES
QUITWAY=timer
TIMER REGISTERED FOR 3s
TIMER FIRED, CALLING quit_blender()
STILL HERE AFTER quit_blender() IN TIMER
00:03.758  blend            | Saved session recovery to "<work folder>/tmp/quit.blend"

Blender quit
```

**The handler fired 45, 46, 45, 45 and 46 times** across the five runs, with the window open, handed the same strings as with no window; and **the only render line that reached the terminal was `Saved:`** — no sample line, no stage line. At quit time every run printed `Saved session recovery to "<work folder>/tmp/quit.blend"`, which is Blender's temporary folder for these runs.

**Observed by hand test, 2026-10-17 — a simulated Esc during a render with the window open (hand-test entry 31).** On the Creator's Mac, Blender 5.3.0 Beta, started with its window and with `--enable-event-simulate`, on `scene14.blend` frames 1 and 2, the render started by `bpy.ops.render.render('INVOKE_DEFAULT', animation=True)`. At 21 seconds — after frame 1 was saved at `00:17.676` — the script sent an Esc press and release into the window with `Window.event_simulate`, the pointer over the 3D viewport. Its own lines afterwards, word for word:

```
[plan  21.00] key ESC pressed and released with the pointer at 1200,900
[plan  24.13] shot of window 0 (1512 x 862) saved as e31_afteresc_w0.png
[plan  24.14] shot of window 1 (390 x 262) saved as e31_afteresc_w1.png
[plan  30.00] render_stats fired 98 times, render_complete 0, render_cancel 0
[plan  30.00] FIRST=('Mem: 0M | Synchronizing object | Cube', None)
[plan  30.00] LAST=('Remaining: 00:03.79 | Mem: 5M | Sample 673/900', None)
[plan  31.00] files in out/e31: ['f_0001.png']
```

**`render_cancel` did not fire, and the statistics handler was still being handed sample counts afterwards** — its latest string at 30 seconds read `Sample 673/900` for frame 2. The picture three seconds after the key still showed `Rendering...` in the Status Bar. Blender was then closed by the script at 32 seconds, before frame 2 was saved, and ended at exit 0. So a simulated Esc sent this way did not stop the render. Whether a real Esc pressed by a person does is not settled by this: no real key reached the window, and the Creator ran nothing in this sitting.

**Observed by hand test, 2026-10-17 — with the window open, a failure goes back to the script that asked, not to the program's status (hand-test entry 34).** On the Creator's Mac, Blender 5.3.0 Beta, started with its window: a render of a file with no camera, called from a start-up script with `bpy.ops.render.render('INVOKE_DEFAULT', write_still=True)`, raised `RuntimeError('Error: No camera found in scene "Scene"\n')` into that script, and Blender went on running until the script closed it, ending at exit 0. Pressing F12 on the same file announced the failure in the Status Bar, a floating report box and the Info editor, and that run also ended at exit 0 when the script closed it. The whole record is under question 1.

**Outside the interface.** Everything in this answer is about runs started by another program or a command line. That is the case the question asks about. **Settled in part by the hand tests of 2026-10-17.** Every number under this question was got with no window from a terminal, except where a block says a window was open. Normal finishes end at 0; a typed Ctrl-C or `SIGINT` ends at 1, two presses at 2; `SIGTERM`, `SIGKILL` and `SIGHUP` end by signal; refusals end at 1; **runs that render nothing can still end at 0** — a scene with no camera, saving switched off, a bad frame argument, a reversed range, a picture of no pixels; a failed script leaves 0 unless `--python-exit-code` comes before it; a registered command's return value becomes the status. With the window open the program's status says nothing about a render; a script's call raises instead.

**Operating system.** The sources name Ctrl-Break as the Windows equivalent of Ctrl-C for stopping a script (12.4). Nothing else is tied to a system. Whether status numbers differ between systems is not stated. **Hand tests, 2026-10-17:** everything observed under this question was run on **the Creator's Mac (Apple M1 Pro, macOS 26.6.2) in Blender 5.3.0 Beta**, so it settles macOS on Apple silicon and nothing else. **Linux and Windows were not run in 5.3** — this sitting had no Linux or Windows machine — so they are untried, not untestable (the equipment this project can reach is set out in `Preparation/Blender Research/Method.md`); on Windows an edition would have to be named before anything was claimed.

---

## 13. When a file the scene points at is not there, what does a render put on each of its output streams?

**Label: not in the sources.** No source says what a render prints, or on which stream, when a file the scene uses is missing. Nearby documented facts follow. None of them is about output streams.

13.1 — **documented** — *Missing files can be listed on request, as a warning in the interface.* "Report Missing Files … After selecting this option, a warning message will appear in the Info editor’s header. If no warning is shown, there are no missing external files." (https://docs.blender.org/manual/en/5.3/interface/window_system/topbar.html, read 2026-10-11). The operator is `bpy.ops.file.report_missing_files()`, "Report all missing external files" (https://docs.blender.org/api/5.3/bpy.ops.file.html, read 2026-10-11).

13.2 — **documented** — *A missing linked blend-file is replaced by stand-ins at load time.* "If Blender cannot find a library while loading a blend-file, it will create placeholder data-blocks to replace missing linked ones." (https://docs.blender.org/manual/en/5.3/files/linked_libraries/link_append.html, read 2026-10-11). Such stand-ins can be detected with `ID.is_missing`: "This data-block is a place-holder for missing linked data …" (https://docs.blender.org/api/5.3/bpy.types.ID.html, read 2026-10-11).

13.3 — **documented** — *Video Sequencer media.* "Render missing images/movies with a solid magenta color. When disabled, missing content will render fully transparent." (https://docs.blender.org/manual/en/5.3/editors/video_sequencer/preview/sidebar.html, read 2026-10-11). The API setting is `SequenceEditor.show_missing_media`, "(default False)" (https://docs.blender.org/api/5.3/bpy.types.SequenceEditor.html). Image sequences with placeholders: "Placeholder frames are rendered in pink in the VSE preview." (https://docs.blender.org/manual/en/5.3/video_editing/edit/montage/strips/image.html). Both read 2026-10-11. These describe pictures, not printed output.

13.4 — **observed** — *One Cycles command splits its output by stream.* Cycles' `maketx` command (not a render) writes its error to standard error and its normal result to standard output (`maketx.py` lines 82–87, read 2026-10-16). This is the only place in the sources where error and normal output are seen going to separate streams. Nothing says renders follow the same pattern.

**Not covered.** What a render prints for a missing image texture, missing linked file, missing cache or volume file, or missing font. Which stream each message goes to. Whether the render continues, and what the missing item looks like in a final (not Sequencer) render.

**Where I looked.** I searched the manual and the API for "missing", "not found", "cannot be found", "could not be found", "failed to load", "unable to load", "unable to find", "file not found", "magenta", "pink", "placeholder", "stdout", "stderr", "standard output" and "standard error". I read the Image Texture-related and linked-library pages, the File menu, the Sequencer preview sidebar, image strips, `ID`, `bpy.ops.file` and `SequenceEditor`. In Cycles' files I searched for "missing", "not found", `stderr`, `stdout` and `print(`. The release notes only mention a Sequencer thumbnail icon for "missing/broken media".

**Observed by hand test, 2026-10-17 — a blend-file handed on the command line that is not there (hand-test entry 10).** On the Creator's Mac, Blender 5.3.0 Beta, set up as described at the top of this document: `"$B53" -b "<scene folder>/there_is_no_such_file.blend" -o "<work folder>/out/e10/a_" -f 1 > out.txt 2> err.txt` printed `exit 1`, wrote nothing, and left the error stream empty. Standard output, whole:

```
00:00.321  reports          | ERROR Cannot read file "<scene folder>/there_is_no_such_file.blend": No such file or directory
Blender 5.3.0 Beta (hash 5d8151335b6b built 2026-10-16 01:36:01)

Blender quit
```

**Observed by hand test, 2026-10-17 — complaints about the command line go to the other stream (hand-test entry 20).** On the Creator's Mac, Blender 5.3.0 Beta, with no window. A blend-file that cannot be read is complained about on **standard output** (hand-test entry 10, quoted above). A bad argument is complained about on the **error stream**, each after a blank line: `Error: not a number '-f / --render-frame abc'.`, `Error: incorrect comma use '-f / --render-frame '.`, `Error: you must specify a path after '-o  / --render-output'.` and `Error: not a number '--python-exit-code notanumber', expected number in [0..255].` — and every one of those runs ended at exit 0. The whole record is under question 12.

**Observed by hand test, 2026-10-17 — a missing file of each kind: what each stream carries, and what the picture holds (hand-test entry 21).** On the Creator's Mac, Blender 5.3.0 Beta, set up as described at the top of this document, with no window. Each command was `"$B53" -b <scene folder>/<file> -o "<work folder>/out/e21/<name>_####" -f 1 > out.txt 2> err.txt`, the named file moved aside just before and back just after. **Every one of the nine runs ended at exit 0, wrote its picture, and left the error stream at 0 bytes.** Every line each printed about the missing file, word for word:

*The control, nothing missing* (`good.blend`): 54 lines, no warning or error of any kind.

*A missing image texture, Cycles* (`tex_missing.blend`, `missing_tex.png` moved aside): 57 lines. The three lines that differ from the control's kind, in order, with the stage line between them:

```
00:00.328  cycles           | WARNING Image file <scene folder>/missing_tex.png does not exist.
00:00.328  render           | Fra: 1 | Mem: 1M | Updating Images | Loading missing_tex.png
00:00.328  cycles           | ERROR Failed to load 1 image files
```

*The same texture, EEVEE* (`-E BLENDER_EEVEE` added): 15 lines; one line about it, under a category of its own, naming the file:

```
00:00.448  image.gpu        | ERROR Image file "<scene folder>/missing_tex.png" does not exist
```

*The same texture, with `-q`*: 54 lines; both Cycles lines survived quiet mode:

```
00:00.333  cycles           | WARNING Image file <scene folder>/missing_tex.png does not exist.
00:00.333  cycles           | ERROR Failed to load 1 image files
```

*The same texture, with `--log-file "<work folder>/logs/e21_c5.log"`*: standard output held only the banner, a blank line and `Blender quit`; both lines were in the file (54 lines), at its lines 13 and 37:

```
00:00.331  cycles           | WARNING Image file <scene folder>/missing_tex.png does not exist.
00:00.332  cycles           | ERROR Failed to load 1 image files
```

*A missing linked library* (`lib_missing.blend`, `lib.blend` moved aside): 55 lines; five about it, all on standard output, under `reports`, printed before the render started:

```
00:00.325  reports          | Read library: '<scene folder>/lib.blend', '//lib.blend', parent '<direct>'
00:00.325  reports          | WARNING Unable to open '<scene folder>/lib.blend': No such file or directory
00:00.325  reports          | Cannot find lib '<scene folder>/lib.blend'
00:00.326  reports          | LIB: Collection: 'LibColl' missing from '<scene folder>/lib.blend', parent '<direct>'
00:00.326  reports          | WARNING 1 libraries and 1 linked data-blocks are missing (including 0 ObjectData), please check the Info and Outliner editors for details
```

*Missing sequencer footage* (`seq_missing.blend`, `missing_seq.png` moved aside): 9 lines and **nothing whatever about the missing picture**, on either stream. Standard output, whole:

```
00:00.322  blend            | Read blend: "<scene folder>/seq_missing.blend"
00:00.326  render           | Rendering single frame
00:00.326  render           | Rendering frame 1
00:00.326  render           | Executing sequencer
00:00.387  render           | Saved: '<work folder>/out/e21/c7_0001.png'
00:00.387  render           | Time: 00:00.06 (Saving: 00:00.06)
Blender 5.3.0 Beta (hash 5d8151335b6b built 2026-10-16 01:36:01)

Blender quit
```

The same again with `--python-expr` printing and then switching off the sequencer's `show_missing_media` before the render: it printed `SHOW_MISSING_MEDIA = True` — the file built by this sitting's script read back `True` — and again nothing about the missing picture.

*A missing font on a text object* (`font_missing.blend`, `missing_font.ttf` moved aside): 67 lines; twelve about it — the same three lines four times over — all on standard output. The first three:

```
00:00.326  reports          | ERROR Unable to pack file, source path '<scene folder>/missing_font.ttf' not found
00:00.326  reports          | ERROR Unable to pack file, source path '<scene folder>/missing_font.ttf' not found
00:00.326  geom.vfont       | WARNING Font file doesn't exist: //missing_font.ttf
```

*What the pictures hold.* Each picture was read back in Blender and the red, green, blue and alpha of its centre printed (the cube sits in the centre of `good.blend`'s view):

| The run | Centre of the picture |
| --- | --- |
| the control, nothing missing | `(0.549, 0.549, 0.549, 1.0)` — plain grey |
| missing image texture, Cycles | `(0.5922, 0.2078, 0.6039, 1.0)` — magenta |
| the same with `-q`, and with `--log-file` | `(0.5922, 0.2078, 0.6039, 1.0)` both |
| missing image texture, EEVEE | `(0.5882, 0.2039, 0.6, 1.0)` — magenta |
| missing linked library | `(0.2275, 0.2275, 0.2275, 1.0)` — the background; the linked thing is not there |
| missing font | `(0.5451, 0.5451, 0.5451, 1.0)` — the cube; the text is not there |
| missing sequencer footage, `show_missing_media` on | `(0.7529, 0.3176, 0.6824, 1.0)` across the whole frame |
| missing sequencer footage, switched off | `(0.0, 0.0, 0.0, 0.0)` — fully transparent |

These scenes are this sitting's own, so the greys and the lit magenta are this scene's; only the sequencer figures do not depend on lighting.

**Observed by hand test, 2026-10-17 — a missing Alembic cache and a missing volume file (hand-test entry 22).** On the Creator's Mac, Blender 5.3.0 Beta, set up as described at the top of this document, with no window. Two kinds no earlier version tried. Each was rendered once with its file present and once with it moved aside, `"$B53" -b <scene folder>/<file> -o "<work folder>/out/e22/<name>_####" -f 1 > out.txt 2> err.txt`, and the two outputs compared with every digit stripped. **All four runs ended at exit 0 and wrote their picture.**

*An Alembic cache* (`cache_missing.blend`: the cube read through a Mesh Sequence Cache modifier from `//cube.abc`). With the file present: 54 lines, error stream empty. With it moved aside: 56 lines on standard output, two of them about it, both under `object.modifier`, the first printed before the render was announced:

```
00:00.318  object.modifier  | WARNING Object: "Cube", Modifier: "MeshSequenceCache", Could not create cache reader for file //cube.abc
00:00.318  object.modifier  | WARNING Object: "Cube", Modifier: "MeshSequenceCache", Could not create cache reader for file //cube.abc
```

and **the error stream carried 274 bytes**, whole:

```
Could not open as Ogawa file from provided streams.
Unable to open <scene folder>/cube.abc
Could not open as Ogawa file from provided streams.
Unable to open <scene folder>/cube.abc
```

The picture's centre read `(0.549, 0.549, 0.549, 1.0)` both with the file and without it — the cube was still drawn.

*A volume file* (`volume_missing.blend`: a Volume object reading `volume.vdb`). With the file present: 62 lines, error stream empty, among them `Synchronizing object | Smoke`, `Updating Images | Loading density`, `Updating Volume | Evaluating density for Smoke` and `Updating Volume | Building octree for Smoke`. With it moved aside: 59 lines, error stream empty, and **nothing printed about the missing file at all** — the comparison showed only those three stage lines gone:

```
21d20
< :.  render           | Fra:  | Mem: M | Updating Images | Loading density
41,42d39
< :.  render           | Fra:  | Mem: M | Updating Volume | Evaluating density for Smoke
< :.  render           | Fra:  | Mem: M | Updating Volume | Building octree for Smoke
```

The picture's centre read `(0.5451, 0.5451, 0.5451, 1.0)` with the file and `(0.549, 0.549, 0.549, 1.0)` without it.

**Observed by hand test, 2026-10-17 — missing files with Blender's window open (hand-test entry 35).** On the Creator's Mac, Blender 5.3.0 Beta, set up as described at the top of this document, started with its window from a terminal. Each file was moved aside just before its run. A start-up script turned the Outliner into an Info editor with its filters on, called `bpy.ops.render.render('INVOKE_DEFAULT', write_still=True)` two seconds in, photographed every window six seconds later, read `WindowManager.reports`, and quit. Every run ended at exit 0 with an empty error stream and wrote its picture.

*A missing image texture* (`tex_missing.blend`). The terminal still received both Cycles lines, word for word:

```
00:02.614  cycles           | WARNING Image file <scene folder>/missing_tex.png does not exist.
00:02.645  cycles           | ERROR Failed to load 1 image files
```

The script read `WindowManager.reports holds 0 entries`. In the picture **the Info editor was empty and the Status Bar carried no message**; the small render window (160 by 128 points) was photographed as plain grey.

*A missing linked library* (`lib_missing.blend`). The terminal received the five lines as the file was opened, before the script ran — note the category `blend.readfile` on three of them, where the same file read with no window put all five under `reports` (hand-test entry 21):

```
00:00.555  blend.readfile   | Read library: '<scene folder>/lib.blend', '//lib.blend', parent '<direct>'
00:00.555  reports          | WARNING Unable to open '<scene folder>/lib.blend': No such file or directory
00:00.555  blend.readfile   | Cannot find lib '<scene folder>/lib.blend'
00:00.556  blend.readfile   | LIB: Collection: 'LibColl' missing from '<scene folder>/lib.blend', parent '<direct>'
00:00.560  reports          | WARNING 1 libraries and 1 linked data-blocks are missing (including 0 ObjectData), please check the Info and Outliner editors for details
```

and the reports list **held all five**:

```
[plan   9.00] REPORT type=INFO message="Read library: '<scene folder>/lib.blend', '//lib.blend', parent '<direct>'"
[plan   9.00] REPORT type=WARNING message="Unable to open '<scene folder>/lib.blend': No such file or directory"
[plan   9.00] REPORT type=INFO message="Cannot find lib '<scene folder>/lib.blend'"
[plan   9.00] REPORT type=INFO message="LIB: Collection: 'LibColl' missing from '<scene folder>/lib.blend', parent '<direct>'"
[plan   9.00] REPORT type=WARNING message='1 libraries and 1 linked data-blocks are missing (including 0 ObjectData), please check the Info and Outliner editors for details'
```

In the picture the Info editor shows the entries, the last with a warning triangle — `1 libraries and 1 linked data-blocks are missing (including 0 ObjectData), please check the Info and Outliner editors for details` — and the Status Bar carries the same warning shortened to `1 libraries and 1 linked d...utliner editors for details`.

*Missing sequencer footage* (`seq_missing.blend`), never before opened in a window. Nothing about it reached the terminal; the reports list held **0 entries**; the picture shows an empty Info editor and no message in the Status Bar.

**Outside the interface.** 13.1 is an interface report, and 13.3 is about pictures. The question is about streams, which matter outside the interface, and the sources are silent there. **Settled in part by the hand tests of 2026-10-17.** With no window, a missing texture, library, font or Alembic cache is reported on **standard output** (Alembic adds four lines of its own on the error stream), a missing sequencer picture and a missing volume file print nothing, and every one of those renders ends at 0; a missing blend-file named on the command line ends at 1. With the window, a missing library shows in the Info editor, the Status Bar and the reports list; a missing texture or sequencer picture shows nowhere in the window.

**Operating system.** Not stated in the sources. **Hand tests, 2026-10-17:** everything observed under this question was run on **the Creator's Mac (Apple M1 Pro, macOS 26.6.2) in Blender 5.3.0 Beta**, so it settles macOS on Apple silicon and nothing else. **Linux and Windows were not run in 5.3** — this sitting had no Linux or Windows machine — so they are untried, not untestable (the equipment this project can reach is set out in `Preparation/Blender Research/Method.md`); on Windows an edition would have to be named before anything was claimed.

---

## 14. When a render's printed output is sent somewhere other than a terminal, what decides when each line of it becomes available to whatever is reading?

**Label: not in the sources.**

The sources do not discuss holding back output, releasing it in chunks or line by line, or any difference between printing to a terminal and printing to a file or another program.

Nearby facts, neither of which answers the question:
- **documented:** `--log-file <filepath>` — "Set a file to output the log to." (https://docs.blender.org/manual/en/5.3/advanced/command_line/arguments.html, read 2026-10-11). The page does not say when lines are written.
- **documented:** a preference stores lines in memory for the console: "Console Scroll-back Lines — The number of lines, buffered in memory of the console window. Useful for debugging purposes and command-line rendering." (https://docs.blender.org/manual/en/5.3/editors/preferences/system.html, read 2026-10-11). The API entry is `scrollback`: "Maximum number of lines to store for the console buffer (in [32, 32768], default 256)" (https://docs.blender.org/api/5.3/bpy.types.PreferencesSystem.html, read 2026-10-11). This is about how many lines are kept for display, not about when lines reach an outside reader.

**Not covered.** Everything the question asks: whether lines are passed on as each is printed, in blocks, or when the program ends. Whether the two output streams behave differently. Whether there is a setting or environment variable to change it.

**Where I looked.** I searched the manual, the API and the release notes for "buffer", "buffered", "unbuffered", "line buffer", "flush", "pipe", "redirect", "isatty", "tty", "PYTHONUNBUFFERED", "stdout", "stderr", "standard output" and "standard error". I left out hits about frame, vertex, GPU, storage and bitstream buffers, and about "pipeline". The only console-related hits are the two above. I searched Cycles' `.py` files for `flush`, `stdout` and `stderr`. The only hit, in `maketx.py`, writes a line and does not touch timing. I read the environment variables list on the arguments page; none concerns output timing.

**Observed by hand test, 2026-10-17 — the build's own help names no switch for when lines are released (hand-test entry 7).** On the Creator's Mac, Blender 5.3.0 Beta, set up as described at the top of this document: `"$B53" --help` printed 521 lines and `exit 0`. Searching all of them for `buffer`, `flush`, `unbuffer`, `stdout` and `stderr`, in any case, found exactly one line — `		Read from disk (Do not buffer).`, at line 97, under the animation player's options, which is about reading picture files rather than writing output. The help's whole Logging Options section is quoted under question 4; `--log-file <filepath>` is described there only as "Set a file to output the log to."

**Observed by hand test, 2026-10-17 — how the timing of each line was measured, and the check that the measurement can tell the two answers apart (hand-test entry 39).** On the Creator's Mac, Blender 5.3.0 Beta, set up as described at the top of this document. Every run for this question used `scene14.blend` (Cycles on the processor, 900 samples, 720 by 405, about twelve seconds a frame on this Mac) and `--factory-startup`. A small program, `reader.py`, sat on the far end of the pipe: it reads raw bytes as they arrive and prints, for each arrival, the seconds since it started, how many bytes arrived, the running total, and the first 70 bytes. Lines that arrive as they are produced show as many small arrivals spread over the run; lines held back show as a few large arrivals far apart.

*Held back on purpose.* A program that deliberately holds everything in 4096-byte blocks was put between Blender and the reader: `"$B53" -b --factory-startup <scene folder>/scene14.blend -s 1 -e 3 -a 2>&1 | python3 holdbuf.py | python3 reader.py`. What the reader printed, whole:

```
  13.071  chunk   1    4096 bytes  total    4096  b'00:00.319  blend            | Read blend: "/private/tmp/claude-501/smo'
  25.377  chunk   2    4096 bytes  total    8192  b' | Cube\n00:12.660  render           | Fra: 2 | Mem: 0M | Synchronizing'
  37.719  chunk   3    2936 bytes  total   11128  b'65  render           | Fra: 3 | Mem: 1M | Updating Procedurals\n00:24.9'
  37.722  END  3 arrivals, 11128 bytes total
```

*Prompt on purpose.* A script printing `tick 1` to `tick 15`, one a second, run in the way the tests below show makes Python print promptly: `PYTHONUNBUFFERED=1 "$B53" -b --factory-startup --python-use-system-env --python ticker.py 2>&1 | python3 reader.py`. The first five and last three arrivals:

```
   0.559  chunk   1      65 bytes  total      65  b'Blender 5.3.0 Beta (hash 5d8151335b6b built 2026-10-16 01:36:01)\n'
   0.733  chunk   2       7 bytes  total      72  b'tick 1\n'
   1.743  chunk   3       7 bytes  total      79  b'tick 2\n'
   2.750  chunk   4       7 bytes  total      86  b'tick 3\n'
   3.761  chunk   5       7 bytes  total      93  b'tick 4\n'
…
  14.862  chunk  17       8 bytes  total     176  b'tick 15\n'
  15.937  chunk  18      14 bytes  total     190  b'\nBlender quit\n'
  15.943  END  18 arrivals, 190 bytes total
```

So the reader shows the two shapes apart on this machine: three arrivals of whole 4096-byte blocks twelve seconds apart against eighteen arrivals a second apart.

**Observed by hand test, 2026-10-17 — a render's own lines through a pipe (hand-test entry 40).** On the Creator's Mac, Blender 5.3.0 Beta, as in hand-test entry 39: `"$B53" -b --factory-startup <scene folder>/scene14.blend -s 1 -e 3 -a 2>&1 | python3 reader.py`. **68 arrivals over 37.8 seconds**, spread across the run. The first three, the arrivals that carried each frame's `Saved:` line, and the last nine, word for word:

```
   0.728  chunk   1     119 bytes  total     119  b'00:00.317  blend            | Read blend: "/private/tmp/claude-501/smo'
   0.733  chunk   2      64 bytes  total     183  b'00:00.321  render           | Rendering animation (frames 1..3)\n'
   0.733  chunk   3      48 bytes  total     231  b'00:00.321  render           | Rendering frame 1\n'
…
  13.132  chunk  26     179 bytes  total    3870  b"00:12.721  render           | Saved: '/private/tmp/claude-501/smoozy-h"
  13.132  chunk  27      48 bytes  total    3918  b'00:12.721  render           | Rendering frame 2\n'
  25.432  chunk  47     227 bytes  total    7540  b"00:25.021  render           | Saved: '/private/tmp/claude-501/smoozy-h"
…
  25.433  chunk  61     132 bytes  total   10439  b'00:25.022  render           | Fra: 3 | Mem: 1M | Updating Integrator\n0'
  25.433  chunk  62     228 bytes  total   10667  b'00:25.022  render           | Fra: 3 | Mem: 1M | Updating Lookup Table'
  25.433  chunk  63      62 bytes  total   10729  b'00:25.022  render           | Fra: 3 | Mem: 1M | Sample 0/900\n'
  25.446  chunk  64      84 bytes  total   10813  b'00:25.034  render           | Fra: 3 | Remaining: 00:11.00 | Mem: 5M |'
  37.764  chunk  65      64 bytes  total   10877  b'00:37.352  render           | Fra: 3 | Mem: 5M | Sample 900/900\n'
  37.764  chunk  66      58 bytes  total   10935  b'00:37.352  render           | Fra: 3 | Mem: 5M | Finished\n'
  37.778  chunk  67     179 bytes  total   11114  b"00:37.366  render           | Saved: '/private/tmp/claude-501/smoozy-h"
  37.812  chunk  68      14 bytes  total   11128  b'\nBlender quit\n'
  37.816  END  68 arrivals, 11128 bytes total
```

Each line reached the reader within about half a second of the time Blender stamped on it — `00:12.721` at 13.132 seconds, `00:25.021` at 25.432, `00:37.366` at 37.778 — in arrivals of one or a few lines.

**Observed by hand test, 2026-10-17 — the same render through a real terminal (hand-test entry 41).** On the Creator's Mac, Blender 5.3.0 Beta: the same render with a real terminal in place of the pipe, supplied by the system's own `script` command — `script -q /dev/null "$B53" -b --factory-startup <scene folder>/scene14.blend -s 1 -e 3 -a < /dev/null | python3 reader.py`. **57 arrivals over 37.7 seconds**, the same pattern as through the pipe; every line ends `\r\n` rather than `\n`, which is the terminal's doing; the version banner arrived second, at 0.423 seconds (the first arrival, `^D\x08\x08`, is `script`'s own). The first three and last four arrivals:

```
   0.000  chunk   1       4 bytes  total       4  b'^D\x08\x08'
   0.423  chunk   2      66 bytes  total      70  b'Blender 5.3.0 Beta (hash 5d8151335b6b built 2026-10-16 01:36:01)\r\n'
   0.744  chunk   3     120 bytes  total     190  b'00:00.322  blend            | Read blend: "/private/tmp/claude-501/smo'
…
  37.674  chunk  55     124 bytes  total   11084  b'00:37.252  render           | Fra: 3 | Mem: 5M | Sample 900/900\r\n00:37'
  37.688  chunk  56     181 bytes  total   11265  b"00:37.266  render           | Saved: '/private/tmp/claude-501/smoozy-h"
  37.720  chunk  57      16 bytes  total   11281  b'\r\nBlender quit\r\n'
  37.726  END  57 arrivals, 11281 bytes total
```

**Observed by hand test, 2026-10-17 — the same render into a plain file (hand-test entry 42).** On the Creator's Mac, Blender 5.3.0 Beta: the same three-frame render with both streams sent to a file, and a second program printing the file's size whenever it changed, looking every quarter second for a fixed 120 seconds and then waiting for the run to end — `python3 filesize.py out.txt 120 out.txt -- "$B53" -b --factory-startup <scene folder>/scene14.blend -s 1 -e 3 -a`. What it printed, whole:

```
   0.002  file is       0 bytes
   0.781  file is    3569 bytes
  13.183  file is    7191 bytes
  25.618  file is   10813 bytes
  38.062  file is   11128 bytes
 124.130  run finished, exit 0, file is   11128 bytes
```

(The 124 seconds is the fixed watch ending, not the render.) **The file grew during the run, in one step at the start and one as each frame finished.** Counting its line endings afterwards: `newlines: 149   carriage returns: 0   bytes: 11128` — the progress readout is whole separate lines, never one line rewritten.

**Observed by hand test, 2026-10-17 — a Python script's own printing, nine ways (hand-test entry 43).** On the Creator's Mac, Blender 5.3.0 Beta, as in hand-test entry 39. The script `ticker.py` prints `tick 1` to `tick 15` with `print("tick %d" % i)`, waiting a second after each; `ticker_flush.py` is the same with `flush=True`. Every command began `"$B53" -b --factory-startup` and ended `2>&1 | python3 reader.py` unless said otherwise. The reader shows only the first 70 bytes of an arrival, so an arrival of 111 bytes reads as ten ticks but holds all fifteen.

*1. Plain, through a pipe* (`--python ticker.py`) — all fifteen at the end, whole:
```
  15.811  chunk   1     111 bytes  total     111  b'tick 1\ntick 2\ntick 3\ntick 4\ntick 5\ntick 6\ntick 7\ntick 8\ntick 9\ntick 10'
  15.874  chunk   2      65 bytes  total     176  b'Blender 5.3.0 Beta (hash 5d8151335b6b built 2026-10-16 01:36:01)\n'
  15.879  chunk   3      14 bytes  total     190  b'\nBlender quit\n'
  15.882  END  3 arrivals, 190 bytes total
```

*2. Plain, into a file* (`python3 filesize.py out.txt 20 out.txt -- "$B53" -b --factory-startup --python ticker.py`) — the file stayed at 0 bytes and filled at the end, whole:
```
   0.002  file is       0 bytes
  15.997  file is     190 bytes
  20.900  run finished, exit 0, file is     190 bytes
```

*3. With `PYTHONUNBUFFERED=1` set in front* — unchanged: `15.844  chunk   1     111 bytes …`, then the banner and `Blender quit`; 3 arrivals.

*4. With `flush=True` on each print* (`--python ticker_flush.py`) — **one a second**; the first five and the last two arrivals:
```
   0.727  chunk   1       7 bytes  total       7  b'tick 1\n'
   1.731  chunk   2       7 bytes  total      14  b'tick 2\n'
   2.741  chunk   3       7 bytes  total      21  b'tick 3\n'
   3.751  chunk   4       7 bytes  total      28  b'tick 4\n'
   4.759  chunk   5       7 bytes  total      35  b'tick 5\n'
…
  15.903  chunk  17      14 bytes  total     190  b'\nBlender quit\n'
  15.906  END  17 arrivals, 190 bytes total
```

*5. Plain, through a real terminal* (`script -q /dev/null "$B53" -b --factory-startup --python ticker.py < /dev/null | python3 reader.py`) — **one a second**, the banner first; the first five arrivals:
```
   0.000  chunk   1       4 bytes  total       4  b'^D\x08\x08'
   0.413  chunk   2      66 bytes  total      70  b'Blender 5.3.0 Beta (hash 5d8151335b6b built 2026-10-16 01:36:01)\r\n'
   0.732  chunk   3       8 bytes  total      78  b'tick 1\r\n'
   1.742  chunk   4       8 bytes  total      86  b'tick 2\r\n'
   2.752  chunk   5       8 bytes  total      94  b'tick 3\r\n'
```

*6. With `--python-use-system-env` alone* — unchanged: all fifteen at 15.847 seconds; 3 arrivals.

*7. With both `PYTHONUNBUFFERED=1` and `--python-use-system-env`* — **one a second**, the banner first; the first five arrivals:
```
   0.553  chunk   1      65 bytes  total      65  b'Blender 5.3.0 Beta (hash 5d8151335b6b built 2026-10-16 01:36:01)\n'
   0.728  chunk   2       7 bytes  total      72  b'tick 1\n'
   1.736  chunk   3       7 bytes  total      79  b'tick 2\n'
   2.747  chunk   4       7 bytes  total      86  b'tick 3\n'
   3.748  chunk   5       7 bytes  total      93  b'tick 4\n'
```

*8. With `stdbuf -o 0` in front of the whole command*, and *9. with `stdbuf -o L`* — the ticks unchanged, all fifteen at the end, but the banner moved to the front. Whole, for `-o L`:
```
   0.417  chunk   1      65 bytes  total      65  b'Blender 5.3.0 Beta (hash 5d8151335b6b built 2026-10-16 01:36:01)\n'
  15.849  chunk   2     111 bytes  total     176  b'tick 1\ntick 2\ntick 3\ntick 4\ntick 5\ntick 6\ntick 7\ntick 8\ntick 9\ntick 10'
  15.913  chunk   3      14 bytes  total     190  b'\nBlender quit\n'
  15.918  END  3 arrivals, 190 bytes total
```
and for `-o 0` the same shape, banner at 0.412 seconds and the ticks at 15.840.

So **a script's own `print()` lines reach a pipe or a file only when the run ends, unless each is flushed, or the output is a real terminal, or `PYTHONUNBUFFERED=1` is given together with `--python-use-system-env`**; either of those two alone changes nothing, and `stdbuf` moves only Blender's own banner.

**Observed by hand test, 2026-10-17 — the error stream on its own (hand-test entry 44).** On the Creator's Mac, Blender 5.3.0 Beta, as in hand-test entry 39, one frame of `scene14.blend` with Cycles' statistics switched on.

*First run, and why it does not count.* Typed in the runner's terminal, whose shell is zsh 5.9: `"$B53" -b --factory-startup <scene folder>/scene14.blend -f 1 -- --cycles-print-stats 2>&1 1>/dev/null | python3 reader.py`. The reader received **28 arrivals, 7,084 bytes — the render's ordinary lines**, the first `00:00.337  blend            | Read blend: …` and the last `\nBlender quit\n`. That is the shell's doing, not Blender's: zsh, unlike bash, by default copies a stream to every place it is redirected, so `1>/dev/null` after `2>&1` sent standard output into the pipe as well. The run measured nothing about the error stream.

*Second run, the same command under bash* — `bash -c '"$B53" -b --factory-startup <scene folder>/scene14.blend -f 1 -- --cycles-print-stats 2>&1 1>/dev/null | python3 reader.py'`. What the reader printed, whole:

```
  13.297  END  0 arrivals, 0 bytes total
```

**Nothing at all arrived on the error stream.** Keeping the two streams apart in files instead — `"$B53" -b --factory-startup <scene folder>/scene14.blend -f 1 -- --cycles-print-stats > out.txt 2> err.txt` — ended at exit 0 with **standard output 7,088 bytes and the error stream 0 bytes**, the statistics on standard output under `Update statistics:` (line 15) and `Render statistics:` (line 85).

**Observed by hand test, 2026-10-17 — the file written by `--log-file` fills in as the run goes (hand-test entry 45).** On the Creator's Mac, Blender 5.3.0 Beta, as in hand-test entry 39: a two-frame render writing everything at information level to a log file, both streams thrown away, and a second program printing the log file's size whenever it changed, looking every quarter second for a fixed 80 seconds — `python3 filesize.py run.log 80 /dev/null -- "$B53" -b --factory-startup <scene folder>/scene14.blend --log "*" --log-level info --log-file "<work folder>/logs/e45.log" -s 1 -e 2 -a`. What it printed, whole:

```
   0.002  file is       0 bytes
   0.522  file is     146 bytes
   0.782  file is    6696 bytes
  13.157  file is   14441 bytes
  25.567  file is   16418 bytes
  82.795  run finished, exit 0, file is   16418 bytes
```

(The 82 seconds is the fixed watch ending.) **The log file grew during the run**, in steps at the start and as each frame finished.

**Observed by hand test, 2026-10-17 — both kinds of line in one run (hand-test entry 46).** On the Creator's Mac, Blender 5.3.0 Beta, as in hand-test entry 39: one run that prints a script line at the very start and then renders two frames, into one pipe — `"$B53" -b --factory-startup <scene folder>/scene14.blend --python-expr "print('SCRIPT LINE PRINTED AT START')" -s 1 -e 2 -a 2>&1 | python3 reader.py`. The arrivals carrying the first line, each `Saved:`, the script's line and the end, with their arrival numbers, word for word:

```
   0.725  chunk   1     119 bytes  total     119  b'00:00.319  blend            | Read blend: "/private/tmp/claude-501/smo'
  13.120  chunk  27     115 bytes  total    3806  b"00:12.714  render           | Saved: '/private/tmp/claude-501/smoozy-h"
  25.410  chunk  44     179 bytes  total    7492  b"00:25.004  render           | Saved: '/private/tmp/claude-501/smoozy-h"
  25.416  chunk  45      29 bytes  total    7521  b'SCRIPT LINE PRINTED AT START\n'
  25.447  chunk  46      14 bytes  total    7535  b'\nBlender quit\n'
  25.450  END  46 arrivals, 7535 bytes total
```

The render's own lines arrived throughout the run; **the script's line, printed within the first second, reached the reader at 25.416 seconds, after both frames had finished**, just before `Blender quit`.

**Observed by hand test, 2026-10-17 — nothing already printed is lost when a run is stopped part-way (hand-test entry 47).** On the Creator's Mac, Blender 5.3.0 Beta, as in hand-test entry 39: a three-frame render with both streams sent to a file, stopped from a separate program at 22 seconds — once with an interrupt (`SIGINT`) and once with an outright kill (`SIGKILL`), which cannot flush anything a program is still holding. The program read the file's size and last three lines a moment before the signal and again after the run ended — `python3 stopfile.py <signal> 22 out.txt -- "$B53" -b --factory-startup <scene folder>/scene14.blend -s 1 -e 3 -a`. What it printed, whole, for the interrupt:

```
BEFORE SIGINT: file is 7191 bytes, ending:
   00:12.656  render           | Fra: 2 | Mem: 1M | Updating Device | Writing constant memory
   00:12.656  render           | Fra: 2 | Mem: 1M | Sample 0/900
   00:12.667  render           | Fra: 2 | Remaining: 00:10.62 | Mem: 5M | Sample 1/900
AFTER  SIGINT: file is 7448 bytes, ending:
   00:24.928  render           | Fra: 2 | Mem: 5M | Cancelled
   
   Blender quit
[harness] ended with status 1
```

and for the kill:

```
BEFORE SIGKILL: file is 7191 bytes, ending:
   00:12.659  render           | Fra: 2 | Mem: 1M | Updating Device | Writing constant memory
   00:12.659  render           | Fra: 2 | Mem: 1M | Sample 0/900
   00:12.672  render           | Fra: 2 | Remaining: 00:11.20 | Mem: 5M | Sample 1/900
AFTER  SIGKILL: file is 7191 bytes, ending:
   00:12.659  render           | Fra: 2 | Mem: 1M | Updating Device | Writing constant memory
   00:12.659  render           | Fra: 2 | Mem: 1M | Sample 0/900
   00:12.672  render           | Fra: 2 | Remaining: 00:11.20 | Mem: 5M | Sample 1/900
[harness] ended with signal 9
```

**After the kill the file held the same 7,191 bytes it held before, ending on the last line the run had printed** — nothing was lost.

**Outside the interface.** The question is entirely about running outside the interface. The sources are silent. **Settled by the hand tests of 2026-10-17, for macOS.** A render's own lines reach a pipe, a real terminal and a plain file as they are produced; a script's own `print()` lines reach a pipe or a file only when the run ends unless flushed, sent to a real terminal, or run with `PYTHONUNBUFFERED=1` together with `--python-use-system-env`; the error stream carries nothing on an ordinary render; nothing printed is lost when a run is killed.

**Operating system.** Not stated in the sources. **Hand tests, 2026-10-17:** everything observed under this question was run on **the Creator's Mac (Apple M1 Pro, macOS 26.6.2) in Blender 5.3.0 Beta**, so it settles macOS on Apple silicon and nothing else. **Linux and Windows were not run in 5.3** — this sitting had no Linux or Windows machine — so they are untried, not untestable (the equipment this project can reach is set out in `Preparation/Blender Research/Method.md`); on Windows an edition would have to be named before anything was claimed.

---

## Things a person at a computer with Blender 5.3 could settle by hand

**Pruned 2026-10-17 by the hand tests.** All ten items that stood here were run on 2026-10-17 on the Creator's Mac (Apple M1 Pro, macOS 26.6.2) in Blender 5.3.0 Beta, together with every test an earlier version of this topic holds a result for, and the results are written into the answers above as blocks headed **Observed by hand test, 2026-10-17**. What is still open, and why:

1. **Main memory running out during sampling (question 10).** Not run: the Creator's Mac is their working computer and its memory is not to be filled on purpose, and this sitting had no other machine.
2. **Anything on Linux or on Windows (every question).** Not run: this sitting had no Linux or Windows machine. That leaves, among others, where the crash file goes on Linux and Windows, Windows' hidden console, `-con`, Ctrl-Break and what `%ERRORLEVEL%` reports, display-library lines on the error stream on a Linux machine with a display, and an unwritable folder for a `root` user.
3. **Anything needing an NVIDIA, AMD or Intel graphics card, or more than one card (questions 1, 2, 6).** Not run: the Mac has only its own graphics, which share one pool of memory with the processor. That leaves working CUDA, OptiX, HIP and oneAPI renders (only their refusal here is settled), `Error: Out of memory`, a card filled while system memory remains, the kernel-build and "Unsupported GNU version" failures, the Windows driver time-out, EEVEE's "Shadow buffer full", and what the `VRAM:` readout says with such a card.
4. **The video-memory readout on this Mac (question 6).** `show_statusbar_vram` is read-only on the Mac, so it could not be switched on.
5. **An output folder that cannot be written to, on the Mac (questions 1, 12).** Not run: the tool the hand tests ran through refused to change the folder's permissions.
6. **Whether a real Esc pressed by a person stops a render (questions 1, 12).** A simulated Esc did not; no real key was pressed, the Creator running nothing in this sitting.
7. **Where a render engine's `error_set` text is shown (question 2).** Not in the terminal, the reports list, the Status Bar or the Info editor; the render window's status line ran past the window's edge in the picture, so whether it is there is not settled.
8. **Whether the Status Bar percentage starts again at each frame (questions 3, 5).** The pictures of the window lagged its real state by several seconds, so the percentages photographed cannot be tied to frames.
9. **Other kinds of missing file (question 13):** a sound, a point cache, an external OSL script, an image the compositor reads, an environment texture. Not tried.
10. **A disk filling up during an animation (questions 1, 12).** Not run: it means filling the Creator's own disk, or mounting a disk image with a tool other than Blender, which this sitting's rules did not allow.
11. **A blend-file written by a newer Blender (question 1)** — there is none newer than 5.3 — and **Blender loaded as a Python module (questions 1, 9, 12)** — no such build is installed.
12. **What an interactive viewport render or the Rendered shading mode reports, and the sample figure with several view layers or Sample Subset (questions 9, 11).** Not tried.

---

2026-10-16 — Claude Opus 5.5 (Claude Code) — Reports, Blender 5.3

2026-10-17 — Claude Opus 5.5 (Claude Code) — hand tests for Reports, Blender 5.3: ran 47 tests on the Creator's Mac, wrote their results into questions 1 to 14 as observed findings, brought both standing lines up to date, and pruned the list above

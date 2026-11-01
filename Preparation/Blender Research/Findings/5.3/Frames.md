# Frames — findings, Blender 5.3

- Sources consulted: the Blender 5.3 manual, the Blender 5.3 Python API documentation and the Blender 5.3 release notes (all copies taken 2026-10-11), and Cycles' own files for 5.3 (copy taken 2026-10-16). Work done on 2026-10-16.
- I read only the question sheet and files inside Blender Sources/. I did not open any other part of this project.

Note on the line above, for honesty: when this session started, the tool I run in placed two things into my working context without my asking — the project's `CLAUDE.md` signpost text and a one-line-per-entry index of the user's saved notes. I did not open either file, did not follow the signpost, and opened nothing it points to. Inside `Blender Sources/` I also read the folder's own note, `About this folder.md`; it names a method file outside the folder, which I did not open. I did not run Blender. Nothing below is labelled "observed" from running anything; the "observed" labels refer to reading Cycles' own files.

*Added by the hand tests, 2026-10-17: the note above is the researcher's and describes the researcher's sitting. The paragraphs headed **Observed — hand test…**, and the standing lines marked **brought up to date by the hand tests**, were added afterwards, on 2026-10-16 and 2026-10-17, from Blender 5.3.0 Beta run on the Creator's Mac; those "observed" labels do come from running Blender. The tests, their exact commands and every departure from them are in `Audits/Blender Research/Frames/Hand tests — 5.3.md`.*

One quirk of the text copy, which matters to several answers: on the Output page, the heading above the Placeholders description reads the same as the Overwrite heading ("Image Sequence – Overwrite Image Multi-Layer EXR" appears twice). The description under the second one is the Placeholders text. The panel picture `_images/render_output_properties_output_panel.png` shows two separate checkboxes, "Overwrite" and "Placeholders". I treat the second entry as the Placeholders entry.

---

## 1. Can a render that was interrupted by the computer going down be picked up from where it stopped?

**Answer.** Yes, when the animation is written as separate image files, and (**inferred**) only to the level of whole frames. The manual names this exact case. The frames already rendered stay in the output folder, and you start the render again with Overwrite turned off. Blender then starts "where you left off". When the animation is written as one video file, the manual warns that if a problem occurs while rendering, the file "might become unplayable" and you will have to render every frame again from the start. No source describes a way to pick up a video render from where it stopped.

**Label:** documented (restarting an image-sequence render from the frame where a problem occurred by turning Overwrite off; the warning that a video file might become unplayable and require re-rendering all frames from the beginning). Inferred (that recovery is only to whole frames; setting Overwrite off through Python when driving from outside). Not in the sources (whether a video render can ever be picked up from where it stopped).

**Sources:**
- "Your computer accidentally turns off in the middle of rendering your movie!" … "the frames you have already rendered will still be in the output directory." … "Just disable the Overwrite option to start rendering where you left off." — Manual, Rendering Animations, Hints. https://docs.blender.org/manual/en/5.3/render/output/animation.html — read 2026-10-11.
- "If a problem occurs while rendering, the file might become unplayable and you will have to re-render all frames from the beginning." — Manual, Output properties, Encoding, Tips. https://docs.blender.org/manual/en/5.3/render/output/properties/output.html — read 2026-10-11.
- "Restart the rendering from the place (the frame) where any problem occurred." (given as an advantage of rendering images rather than a video) — Manual, Output properties, Encoding, Tips. https://docs.blender.org/manual/en/5.3/render/output/properties/output.html — read 2026-10-11.

**Not covered:**
- Blender does not resume by itself. The sources describe a person starting the render again.
- Nothing says a frame that was only partly rendered can be continued. The unit of recovery in every passage is the frame.
- Nothing says what is left at the path of the frame that was being rendered or written when the power went (see 6, 13 and 16). If something is left there, Overwrite-off might count it as done. The sources do not say.
- Nothing about data the operating system had not yet put on the disk when the power went.

**Where I looked:** searched the whole 5.3 manual, API documentation, release notes and the Cycles files for: resume, resum, interrupt, abort, cancel, crash, power, turns off, recover, restart, overwrite, placeholder, unplayable, corrupt, incomplete. The passages above were the only ones about renders. `troubleshooting/recover` covers blend-file autosave only, not render output.

**Observed — hand tests 11 and 9, run by the chat on 2026-10-16: the program killed outright, the nearest thing to the computer going down that this sitting could run.** Label: **observed**. On the Creator's Mac (Apple M1 Pro, its own 14-core graphics through Metal, 16 GB of memory, macOS 26.6.2), in Blender 5.3.0 Beta (hash `5d8151335b6b`, built 2026-10-16), started from a terminal with no window, with a settings folder of its own and `--factory-startup`; the exact commands are in `Audits/Blender Research/Frames/Hand tests — 5.3.md`. Blender's startup scene, Cycles on the processor, 1000 samples, 400 by 400, PNG, frames 1 to 5, about 6.5 seconds a frame. Sent a kill it cannot catch (`SIGKILL`) two seconds into frame 3, Blender was gone at once with exit status -9. The folder afterwards:

```
    f_0001.png  92828 bytes
    f_0002.png  92828 bytes
```

Both opened as 400 by 400 pictures in Blender's own image loader; nothing was at frame 3's name. With Placeholders on, a kill left a zero-byte file at the frame in hand's name, and a later render with Overwrite off skipped that frame and left it empty (hand test 9, under question 4); with Overwrite off, the later render did pick up from frame 3.

**Not run in 5.3, and why: a machine actually going down.** A kernel reset or a real power cut could only have been done to the Creator's own working Mac, the one machine this sitting had, and was not. So what is left when the whole machine goes down, as against the program alone, is not settled for 5.3.

**Outside the interface:** The method holds when Blender is run from outside, as far as the sources go: run the render again. But Overwrite has no command-line flag. The argument list (https://docs.blender.org/manual/en/5.3/advanced/command_line/arguments.html — read 2026-10-11) has no overwrite or placeholder option. Overwrite is a scene setting, `RenderSettings.use_overwrite`, "Overwrite existing files while rendering (default True)" (https://docs.blender.org/api/5.3/bpy.types.RenderSettings.html — read 2026-10-11). So it must already be off in the saved blend-file, or be switched off through Python before the render starts. The Python step is inferred from the API listing the property; no page shows it being done on a command line.

**Outside the interface — brought up to date by the hand tests of 2026-10-16:** the observed results above were all got from Blender started from a terminal with no window, with Overwrite and Placeholders set through Blender's own Python: the finished frames survived a kill of the program, and a later render with Overwrite off skipped them and carried on. **Brought up to date again on 2026-10-17:** inside Blender's window (hand tests 37 and 41, under question 2), a simulated click on the status bar's cancel cross ended the render and left the finished frames whole, while a simulated Escape did not stop it. A person's own key or click was not used in 5.3, so whether a person's Escape stops it is left open.

**Operating system:** The sources do not say.

**Operating system — brought up to date by the hand tests of 2026-10-16:** observed on one operating system, macOS 26.6.2 on the Creator's Mac (Apple M1 Pro). No Linux or Windows machine was used for 5.3, so nothing above is settled for either in this version.

---

## 2. Can a render that was stopped on purpose be picked up later?

**Answer.** Yes, when it writes image files. The manual directly documents changing the frame range so the render starts at the last frame rendered. **Inferred:** you can also turn Overwrite off and render the same range again (combining the manual's statement that a frame sequence lets you interrupt rendering to use the computer and "resume rendering where you left off" with the Overwrite hint in answer 1, and with the inference in answer 3 that finished frames remain on disk after a deliberate stop). The sources do not say whether a video file render can be continued after a deliberate stop.

**Label:** documented (resuming a cancelled image-sequence render by changing the frame range; Esc and the status-bar cancel button as ways to stop). Inferred (resuming a deliberately stopped image-sequence render by turning Overwrite off; Control-C cancelling a normal command-line render; setting Overwrite off through Python from outside). Not in the sources (whether a deliberately stopped video render can be continued).

**Sources:**
- "when rendering animations to image file formats the render job can be canceled and resumed at the last rendered frame by changing the frame range." — Manual, Output introduction. https://docs.blender.org/manual/en/5.3/render/output/introduction.html — read 2026-10-11.
- Listed under reasons to choose the Frame Sequence approach: "May need to interrupt rendering to use the computer, and want to be able to resume rendering where you left off." — Manual, Rendering Animations. https://docs.blender.org/manual/en/5.3/render/output/animation.html — read 2026-10-11.
- Ways to stop on purpose that the sources name: "As with a normal render, you can abort it with Esc." — Manual, Viewport Render. https://docs.blender.org/manual/en/5.3/editors/3dview/viewport_render.html — read 2026-10-11. And the status bar's Running Task: "The task can be aborted by clicking the cancel button" — Manual, Status Bar. https://docs.blender.org/manual/en/5.3/interface/window_system/status_bar.html — read 2026-10-11.
- "-s, --frame-start <frame> — Set start to frame <frame>, supports +/- for relative frames too." "-e, --frame-end <frame> — Set end to frame <frame>, supports +/- for relative frames too." — Manual, Command Line Arguments. https://docs.blender.org/manual/en/5.3/advanced/command_line/arguments.html — read 2026-10-11.
- `bpy.ops.render.render(*, animation=False, …, frame_start=0, frame_end=0)` — API, bpy.ops.render. https://docs.blender.org/api/5.3/bpy.ops.render.html — read 2026-10-11.

**Not covered:**
- Video: the Direct Approach page says "Blender creates one file that holds all the frames of your animation" (https://docs.blender.org/manual/en/5.3/render/output/animation.html — read 2026-10-11). No page describes adding frames to an existing video file. Whether a deliberately stopped video can be continued is not in the sources.
- Whether the frame being worked on when the stop came is discarded, kept part-done, or left as an empty file (see 16).

**Where I looked:** the searches listed under 1, plus: Esc, stop, quit, pause, render job, Ctrl-F12, Render Animation (topbar Render menu page, keymap page, status bar page, Image Editor pages).

**Observed — hand tests 11, 9 and 6, run by the chat on 2026-10-16.** Label: **observed**. On the Creator's Mac (Apple M1 Pro, its own 14-core graphics through Metal, 16 GB of memory, macOS 26.6.2), in Blender 5.3.0 Beta (hash `5d8151335b6b`, built 2026-10-16), started from a terminal with no window, with a settings folder of its own and `--factory-startup`; the exact commands are in `Audits/Blender Research/Frames/Hand tests — 5.3.md`.

- *Stopped on purpose, with no window* (hand test 11, written out under question 3). Blender's startup scene, Cycles, 1000 samples, 400 by 400, PNG, frames 1 to 5. An interrupt two seconds into frame 3 printed `Sent an internal break event. Press ^C again to kill Blender`, then `Fra: 3 | Mem: 3M | Sample 1000/1000`, `Finished`, `Cancelled` and `Blender quit`, and ended with exit status 1 about four seconds later; a terminate ended it at once with exit status -15, and a kill at once with -9. Each left `f_0001.png` and `f_0002.png`, 92828 bytes each, opening as 400 by 400 pictures, and nothing for frame 3.
- *Started again with Overwrite off* (hand test 9). The same scene with Overwrite off and Placeholders on, killed two seconds into frame 2, left `f_0001.png` at 92828 bytes and `f_0002.png` at 0 bytes. The same render started again printed `Skipping existing frame "…/f_0001.png"`, `Skipping existing frame "…/f_0002.png"`, then saved frames 3, 4 and 5 and ended with exit status 0. Frame 2 stayed an empty file that does not open. Hand test 6 (under question 4) shows Overwrite off skipping by name alone, whatever the file holds.
- *Picking up a video* is under questions 5 and 17 (hand test 16).

**Observed — hand tests 37, 41 and 42, run by the chat on 2026-10-17: stopping a render inside Blender's own window.** Label: **observed**. Same Mac and Blender, started from the terminal **with** its window, and with `--enable-event-simulate`, which makes Blender take keys and clicks only from its own event simulation. **Every key and click below is a simulated event sent from a Python script inside Blender, not a person's hand**, at positions in pixels read off Blender's own photographs of its window. The window opened at 1512 by 862 points and photographed at 3024 by 1724 pixels. The Quick Setup panel that factory settings open was first closed by a simulated Escape with the pointer over it. Blender's startup scene, Cycles on the processor, 1000 samples, 400 by 400, PNG, frames 1 to 6. Two seconds after the window opened, the script started an animation render with `bpy.ops.render.render('INVOKE_DEFAULT', animation=True)` from the 3D Viewport — called from the script, not by clicking the Render menu's Render Animation item — which returned `{'RUNNING_MODAL'}`; a second window of 230 by 260 points opened. Every tenth of a second the script printed whether `bpy.app.is_job_running('RENDER')` answered True, and how many files the output folder held, whenever either changed. The exact commands are in `Audits/Blender Research/Frames/Hand tests — 5.3.md`.

- *Left alone* (hand test 37 a). Six files, about 7.6 seconds apart, `o_0001.png` to `o_0006.png`, 92828 bytes each, last written in that order; `POLL   46.482 job running: False | files: 6`.
- *A simulated Escape, two seconds into frame 3* (hand test 37 b twice, and hand test 41 a). Sent into both windows — in the first run with the key carrying Blender's default position (0, 0), in the other two with the pointer moved to each window's middle first (`ESC into window 1512x862 points with the pointer at (1512, 862) pixels`, `ESC into window 230x260 points with the pointer at (230, 260) pixels`). **In none of the three did the render stop.** For example:

  ```
  ESC   18.003 Escape sent into 2 windows | files at that moment: 2
  00:25.110  render           | Saved: '…/e37/b2/out/o_0003.png'
  POLL   22.586 job running: True | files: 3 | 4.584 s after the Escape
  00:32.610  render           | Saved: '…/e37/b2/out/o_0004.png'
  00:40.301  render           | Saved: '…/e37/b2/out/o_0005.png'
  ```

  Every frame saved opened as a 400 by 400 picture. A photograph two seconds after the Escape shows the status bar still reading `Rendering...`. In hand test 41 a, run with Placeholders on and Overwrite off, all six frames were written, 92828 bytes each, and the job stopped reporting itself running only after the sixth.
- *A simulated click on the status bar's cross, two seconds into frame 3* (hand test 41 b; Placeholders on, Overwrite off). The status bar during the render showed an icon, `Rendering...`, a progress bar with a percentage, and a cross; the pointer was moved onto the cross at (1736, 22), rested there three seconds — no tooltip shows in the photograph taken then — and was clicked:

  ```
  CLICK   18.003 LEFTMOUSE press and release at (1736, 22) | files at that moment: 3
  POLL   18.348 job running: False | files: 2 | 0.344 s after the Escape
  ```

  (The script's wording reads "after the Escape"; the time is measured from the click.) Nothing more was saved. Afterwards the folder held `o_0001.png` and `o_0002.png`, 92828 bytes each, both opening as 400 by 400 pictures, and nothing at frame 3's name: with Placeholders on, the folder had held three files at the moment of the click and held two afterwards. A photograph taken after the click shows the status bar reading `Canceling...` with the progress bar and the cross greyed.
- *What the window offers on the status bar's progress bar* (hand test 41 c). With the pointer resting on the progress bar, a photograph shows a box reading `Time Remaining: 02:07:08.81` and `Time Elapsed: 00:07.63.`. A simulated right click there opened a menu headed `0%` with one item, `Header`, whose submenu lists `Show Header` (ticked), `Show Menus` (ticked) and `Flip to Bottom`. The render carried on.
- *Blender's own quit, called by the script while a render ran.* The scripts end Blender with `bpy.ops.wm.quit_blender()` 45 seconds after the render starts. In the two runs of hand test 37 b, with the Escape sent into both windows, that came during frame 6: Blender printed `Saved session recovery to "/var/folders/…/T/quit.blend"` and `Blender quit`, exit status 0, leaving five whole frames and nothing for frame 6.

What the same stops leave of a video is under question 17.

**A limit on the photographs.** In these runs Blender's own screenshot command sometimes showed the window as it had been one change earlier — found in hand test 43, where a photograph taken after a field was clicked was the same file as the one taken before the click, until the window was made to redraw twice before each photograph. The photographs above are read for what appeared, not for when; the times come from the script's own lines.

**Not run in 5.3, and why: a person's own hand.** On the Creator's word there was no Creator's half to these tests, so every key and click in Blender's window came from Blender's own event simulation. Whether a person's Escape stops a render in 5.3 is not settled.

**Outside the interface:** Changing the frame range works from outside. `-s` / `--frame-start` and `-e` / `--frame-end` set the range on the command line (https://docs.blender.org/manual/en/5.3/advanced/command_line/arguments.html — read 2026-10-11). `bpy.ops.render.render` takes `frame_start` and `frame_end` (https://docs.blender.org/api/5.3/bpy.ops.render.html — read 2026-10-11). Overwrite-off needs Python or the saved file (see 1). The documented ways to stop on purpose (Esc, the cancel button) are interface controls. For a render started from a command line, the only mention of stopping is indirect: the page on Blender as a Python module says that there "Blender’s typical signal handlers are not initialized, so there is no special handling for Control-C to cancel a render" (https://docs.blender.org/api/5.3/info_advanced_blender_as_bpy.html — read 2026-10-11). It is **inferred** from that sentence that the normal Blender program does handle Control-C to cancel a render. No page says so directly.

**Outside the interface — brought up to date by the hand tests of 2026-10-16:** observed from Blender started from a terminal with no window. There an interrupt (what Control-C sends) cancelled the render and left the finished frames, and a later render with Overwrite off, set through Blender's own Python, skipped them and carried on (hand tests 9 and 11). So the inferred Control-C step and the Python step both held there. **Brought up to date again on 2026-10-17:** inside Blender's window, with every key and click a simulated event (hand tests 37, 41 and 42, above), the status bar's cancel button ended the render and left whole frames, and a simulated Escape did not stop it. A person's own key or click was not used in 5.3, so whether a person's Escape stops it is left open.

**Operating system:** The sources do not say.

**Operating system — brought up to date by the hand tests of 2026-10-16:** observed on one operating system, macOS 26.6.2 on the Creator's Mac (Apple M1 Pro). No Linux or Windows machine was used for 5.3, so nothing above is settled for either in this version.

---

## 3. Do frames already written survive a render ending part-way, whether it was stopped on purpose or not?

**Answer.** For image-file output, yes. Frames are written to disk one at a time as they finish, not at the end. For the computer-going-down case the manual says outright that they stay in the output folder. For a deliberate stop, that they survive is inferred: the manual says such a render can be "resumed at the last rendered frame", which only works if the earlier frames are still there. For one video file, the frames are inside that one file, and the manual warns it "might become unplayable" (see 5 and 17).

**Label:** documented (for the computer going down with image files; that frames are saved to disk as they are rendered; and the warning that a video file might become unplayable). Inferred (that image frames already written survive a deliberate stop; and that Cycles keeps the `bl_use_image_save` default of saving while rendering). Observed (that `CyclesRender` in Cycles' `__init__.py` does not set `bl_use_image_save`). Not in the sources (whether frames inside a single video file survive in practice). The step for the deliberate stop: the manual says a cancelled image render can be "resumed at the last rendered frame by changing the frame range". Resuming from the last rendered frame means the frames before it are not rendered again, so they must still be on disk.

**Sources:**
- "animations are a series of images, or frames, and are typically saved directly to disk as they are rendered." — Manual, Rendering Animations. https://docs.blender.org/manual/en/5.3/render/output/animation.html — read 2026-10-11.
- "since the frames you have already rendered will still be in the output directory." — Manual, Rendering Animations, Hints, under "Your computer accidentally turns off…". https://docs.blender.org/manual/en/5.3/render/output/animation.html — read 2026-10-11.
- "the render job can be canceled and resumed at the last rendered frame by changing the frame range." — https://docs.blender.org/manual/en/5.3/render/output/introduction.html — read 2026-10-11.
- `render_write`: "on writing a render frame (directly after the frame is written)." — API, bpy.app.handlers. https://docs.blender.org/api/5.3/bpy.app.handlers.html — read 2026-10-11. (Shows writing happens frame by frame during the render.)
- `bl_use_image_save`: "Save images/movie to disk while rendering an animation." … "(default True)" — API, RenderEngine. https://docs.blender.org/api/5.3/bpy.types.RenderEngine.html — read 2026-10-11.
- Cycles' own file `__init__.py`, class `CyclesRender`, lines 44–51, sets `bl_idname = 'CYCLES'` and `bl_use_eevee_viewport`, `bl_use_preview`, `bl_use_exclude_layers`, `bl_use_spherical_stereo`, `bl_use_custom_freestyle`. It does not set `bl_use_image_save`. — **observed** in the file, copy taken 2026-10-16. That Cycles therefore keeps the default "save … while rendering" is **inferred** from that and the API default.

**Not covered:**
- What happens to files the operating system had not yet put on the disk when the power went.
- What is left for the frame in hand (see 13 and 16).
- Whether images written by a compositor File Output node behave the same way. The node page says it "writes one or more images to disk during rendering or compositing" (https://docs.blender.org/manual/en/5.3/compositing/types/output/file_output.html — read 2026-10-11), but no page discusses those files after a stop.

**Where I looked:** searches from 1 and 2, plus: saved directly, as they are rendered, write, render_write, bl_use_image_save, handlers.

**Observed — hand test 11, run by the chat on 2026-10-16.** Label: **observed**. On the Creator's Mac (Apple M1 Pro, its own 14-core graphics through Metal, 16 GB of memory, macOS 26.6.2), in Blender 5.3.0 Beta (hash `5d8151335b6b`, built 2026-10-16), started from a terminal with no window, with a settings folder of its own and `--factory-startup`. Blender's startup scene, Cycles on the processor, 1000 samples, adaptive sampling and denoising off, 400 by 400 pixels, PNG, frames 1 to 5. Uninterrupted, it wrote five frames of 92828 bytes, each taking about 6.5 seconds, frame 3 beginning 13.8 seconds after the start. The exact commands are in `Audits/Blender Research/Frames/Hand tests — 5.3.md`.

Each run below was stopped 16.0 seconds after the start — two seconds into frame 3 — by a signal sent from outside: an interrupt (`SIGINT`, what Control-C sends), a polite terminate (`SIGTERM`), or a kill that cannot be caught (`SIGKILL`). The folder was listed half a second before the stop and again after the process was gone. Overwrite on and Placeholders off, as Blender's defaults.

*Interrupt, three runs.* Half a second before the stop the folder held `f_0001.png` and `f_0002.png`, 92828 bytes each. Everything printed from the interrupt on, first run:

```
Sent an internal break event. Press ^C again to kill Blender
00:19.453  render           | Fra: 3 | Mem: 3M | Sample 1000/1000
00:19.453  render           | Fra: 3 | Mem: 3M | Finished
00:19.453  render           | Fra: 3 | Mem: 3M | Cancelled

Blender quit
```

Exit status 1, the process gone 3.92, 3.70 and 4.00 seconds after the interrupt in the three runs. Afterwards the folder held `f_0001.png` and `f_0002.png`, 92828 bytes each, and nothing for frame 3.

*Terminate.* Exit status -15, the process gone at once (the harness measured 0.00 seconds); nothing printed after the signal; the folder afterwards `f_0001.png` and `f_0002.png`, 92828 bytes, nothing for frame 3.

*Kill.* Exit status -9, gone at once (0.01 seconds); the same two files and nothing for frame 3.

Blender's own image loader opened every frame 1 and frame 2 left by these five runs: `OPEN f_0001.png | 92828 bytes | size: (400, 400) | channels: 4 | depth: 32 | has_data: True | pixel values read: 640000`, and the same for `f_0002.png`, in all five. Frame 3: `OPEN f_0003.png | missing`.

What each stop left for the frame it had not finished — with Placeholders on, and over an older frame 3 — is written out under question 16. A stop at the moment a frame's own file is being written is under question 13.

*Images written by a compositor File Output node* (hand tests 17 and 15, under question 12): after an interrupt, a terminate or a kill at frame 8's or frame 10's "frame written" event, every image the node had written was on disk at its full size, and the eighth matched the uninterrupted run's value for value. A terminate or a kill while the node was still writing an image left that one image part-written and not opening; an interrupt at that moment let it finish.

*Stopped inside Blender's window* (hand test 41 b, 2026-10-17, under question 2): a simulated click on the status bar's cross two seconds into frame 3 left `o_0001.png` and `o_0002.png` whole, both opening as 400 by 400 pictures.

**Not run in 5.3, and why: the machine going down.** A kernel reset or a real power cut could only have been done to the Creator's own working Mac, the one machine this sitting had, and was not; only the program was ever stopped. What the operating system had not yet put on the disk at such a moment is not settled for 5.3.

**Outside the interface:** The `render_write` handler and the RenderEngine setting are Python features. They exist with or without the interface. The manual's statements are not limited to the interface. The sources do not say whether this differs when Blender is run with `-b`.

**Outside the interface — brought up to date by the hand tests of 2026-10-16:** every observed result above was got with `-b`, from Blender started from a terminal with no window: the finished frames survived an interrupt, a terminate and a kill there. **Brought up to date again on 2026-10-17:** they also survived a render ended inside Blender's window by a simulated click on the status bar's cancel cross (hand test 41 b).

**Operating system:** The sources do not say.

**Operating system — brought up to date by the hand tests of 2026-10-16:** observed on one operating system, macOS 26.6.2 on the Creator's Mac (Apple M1 Pro). No Linux or Windows machine was used for 5.3, so nothing above is settled for either in this version.

---

## 4. Can Blender be asked to render only the frames that are not already on disk, and if so, how does it decide what counts as already there?

**Answer.** Yes. Turn off **Overwrite**. On how it decides, the sources say only "existing files". They do not say whether Blender checks only that a file is there, or also its size, completeness or contents. **Inferred:** the check is against the file name that frame would be written to. That name is built from the output path, the `#` frame-number padding and the file extension setting. **Inferred:** an empty placeholder file counts as already there (see the step below). Overwrite sits under "Image Sequence" and is offered for the Image and Multi-Layer EXR media types. Nothing documents it for Video.

**Label:** documented (that it can be done by turning Overwrite off, and that Overwrite sits under Image Sequence for the Image and Multi-Layer EXR media types). Inferred (that the check is against the frame's output file name, that an empty placeholder file counts as existing, and setting `use_overwrite` and `use_placeholder` from Python when driving from outside). Not in the sources (whether Blender checks anything beyond a file existing at that name, such as size, completeness or contents; when the check happens; or whether Overwrite has any effect on Video or File Output nodes).

**Sources:**
- "Overwrite existing files when rendering." — Manual, Output properties, under the heading "Image Sequence – Overwrite Image Multi-Layer EXR". https://docs.blender.org/manual/en/5.3/render/output/properties/output.html — read 2026-10-11.
- `use_overwrite`: "Overwrite existing files while rendering (default True)" — API, RenderSettings. https://docs.blender.org/api/5.3/bpy.types.RenderSettings.html — read 2026-10-11.
- "Just disable the Overwrite option to start rendering where you left off." — https://docs.blender.org/manual/en/5.3/render/output/animation.html — read 2026-10-11.
- `use_placeholder`: "Create empty placeholder files while rendering frames (similar to Unix ‘touch’) (default False)" — API, RenderSettings. https://docs.blender.org/api/5.3/bpy.types.RenderSettings.html — read 2026-10-11.
- "Disable Overwrite, enable Placeholders in the Render Output panel. Start as many machines as you wish rendering to that directory." — Manual, Output properties, Hint "Primitive Render Farm". https://docs.blender.org/manual/en/5.3/render/output/properties/output.html — read 2026-10-11.
- `frame_path(*, frame=…, preview=False, view='')`: "Return the absolute path to the filename to be written for a given frame" — API, RenderSettings. https://docs.blender.org/api/5.3/bpy.types.RenderSettings.html — read 2026-10-11.
- File naming: "the frame number is appended at the end of the file name with four padded zeros (e.g. image0001.png). You can set a custom padding size by adding the appropriate number of # anywhere in the file name" — Manual, Output properties. https://docs.blender.org/manual/en/5.3/render/output/properties/output.html — read 2026-10-11. File Extensions: "Adds the correct file extensions per file type to the output files." — https://docs.blender.org/manual/en/5.3/render/output/properties/output.html — read 2026-10-11.

**Inference steps:**
- File name: Overwrite talks about "existing files". The only per-frame file Blender names is the one `frame_path` returns. So the file that counts is the one at that name. Change the padding or extension setting, and a different name is checked.
- Placeholders: the render-farm hint has several machines share one folder with Overwrite off and Placeholders on. That only spreads the work if a machine skips a frame for which another machine has made an empty placeholder. So an empty placeholder must count as "existing".

**Not covered:**
- Whether a zero-length or broken file counts as done. Implied for placeholders by the hint above; nothing says it outright, or says anything about broken files.
- Whether the check happens when the frame is reached or once at the start.
- Whether Overwrite also governs images written by a compositor File Output node.
- Whether Overwrite has any effect on Video output.

**Where I looked:** searched all 5.3 sources for overwrite, use_overwrite, placeholder, use_placeholder, existing, already rendered, already exist, skip, frame_path, check_existing. Ruled out the Video Sequencer "Use Placeholders" option (video_editing/edit/montage/strips/image): it is about missing images in a sequencer strip, not render output. Ruled out `check_existing` hits: these are file-browser operator options.

**Observed — hand tests 6, 7, 9 and 10, run by the chat on 2026-10-16.** Label: **observed**. All on the Creator's Mac (Apple M1 Pro, its own 14-core graphics through Metal, 16 GB of memory, macOS 26.6.2), in Blender 5.3.0 Beta (hash `5d8151335b6b`, built 2026-10-16), started from a terminal with no window, with a settings folder of its own and `--factory-startup`. Overwrite and Placeholders were set through Blender's own Python before each render (`scene.render.use_overwrite`, `scene.render.use_placeholder`). The exact commands are in `Audits/Blender Research/Frames/Hand tests — 5.3.md`.

*What counts as already there (hand test 6).* Blender's startup scene, Workbench, 64 by 64 pixels, PNG, frames 1 to 7. Before the render the output folder held: at frame 1's name a complete picture of 4074 bytes; at frame 2's an empty file; at frame 3's twenty bytes of text; at frame 4's a complete picture of another size (32 by 32, 1805 bytes); at frame 5's name a **folder**; nothing for frame 6; at frame 7's a complete picture. Rendered with Overwrite off, Blender printed:

```
00:00.343  render           | Rendering animation (frames 1..7)
00:00.343  render           | Rendering frame 1
00:00.343  render           | Skipping existing frame "…/f_0001.png"
00:00.343  render           | Rendering frame 2
00:00.343  render           | Skipping existing frame "…/f_0002.png"
00:00.343  render           | Rendering frame 3
00:00.343  render           | Skipping existing frame "…/f_0003.png"
00:00.343  render           | Rendering frame 4
00:00.343  render           | Skipping existing frame "…/f_0004.png"
00:00.343  render           | Rendering frame 5
00:00.343  render           | Skipping existing frame "…/f_0005.png"
00:00.343  render           | Rendering frame 6
00:00.502  render           | Saved: '…/f_0006.png'
00:00.502  render           | Rendering frame 7
00:00.502  render           | Skipping existing frame "…/f_0007.png"
```

Every placed thing kept its size and its last-changed time to the nanosecond — for example `f_0002.png 0 bytes 1792179685.940601720 Regular File` before and after, and `f_0005.png 64 bytes 1792179685.945492699 Directory` before and after — and only `f_0006.png` was made, at 4074 bytes. The same folder state rendered with `-f 1..7` in place of `-s 1 -e 7 -a` came out the same: each item announced as `Rendering single frame`, frames 1 to 5 and 7 skipped with the same words, frame 6 saved, nothing placed touched. With all seven complete frames already there and Overwrite off, every frame was skipped and the last line Blender printed was `00:00.331  reports          | No frames rendered, skipped to not overwrite`. With Overwrite on instead, all seven were saved again and every last-changed time moved.

*When the check is made (hand test 7).* Cycles on the processor, 1000 samples, 400 by 400, PNG, frames 1 to 6, Overwrite off. Before the start: complete pictures at frames 2, 3 and 6 only. Three seconds in, while frame 1 was rendering, frame 6's file was deleted and a complete picture put at frame 5's name (the folder listed at 3.1 s held `f_0002.png`, `f_0003.png` and `f_0005.png`). Blender printed:

```
00:07.040  render           | Saved: '…/f_0001.png'
00:07.040  render           | Rendering frame 2
00:07.040  render           | Skipping existing frame "…/f_0002.png"
00:07.040  render           | Rendering frame 3
00:07.040  render           | Skipping existing frame "…/f_0003.png"
00:07.040  render           | Rendering frame 4
00:13.757  render           | Saved: '…/f_0004.png'
00:13.757  render           | Rendering frame 5
00:13.757  render           | Skipping existing frame "…/f_0005.png"
00:13.757  render           | Rendering frame 6
00:20.251  render           | Saved: '…/f_0006.png'
```

Frame 5, absent when the render started and present when it was reached, was skipped; frame 6, present when the render started and gone when it was reached, was rendered. A skipped frame took no time Blender's clock could show (`00:07.040` to `00:07.040`) where a rendered one took about 6.7 seconds.

*A placeholder left by a killed render (hand test 9).* The same Cycles scene, frames 1 to 5, Overwrite off, Placeholders on, killed outright two seconds after frame 2 began (the kill sent by Blender's own process from a timer started at its `render_pre` event). Just before the kill the folder held `f_0001.png=92828 bytes, f_0002.png=0 bytes`; afterwards the same. The same render started again and run to the end printed `Skipping existing frame "…/f_0002.png"` and saved frames 3, 4 and 5; frame 2 stayed at 0 bytes, and Blender's own image loader opened nothing from it: `OPEN f_0002.png | 0 bytes | size: (0, 0) | channels: 0 | depth: 0 | has_data: False | pixel values read: 0`.

*The name that is checked (hand test 10).* Asked what file frame 7 would be written to:

```
PATH filepath '<dir>/f_' -> frame_path(7) = '<dir>/f_0007.png'
PATH filepath '<dir>/f_##_x' -> frame_path(7) = '<dir>/f_07_x.png'
PATH filepath '<dir>/plain' -> frame_path(7) = '<dir>/plain0007.png'
PATH filepath '<dir>/six_######' -> frame_path(7) = '<dir>/six_000007.png'
PATH with File Extensions off -> frame_path(7) = '<dir>/f_0007'
PATH frame_path(12345) = '<dir>/big_12345.png'
```

The files hand test 6 skipped and wrote were at exactly these names (`f_0001.png` and so on from `f_`).

*A video output* (hand test 16 e, written out under question 5). A two-frame MPEG-4 video rendered again over itself with Overwrite off printed no skipping line, rendered both frames again, and the file's last-changed time moved. Overwrite and Placeholders could both be switched with the output set to Video (hand test 19, under question 12).

*A render started inside Blender's window* (hand test 44, 2026-10-17). Blender opened with its window and `--enable-event-simulate`, the Quick Setup panel closed by a simulated Escape, Workbench, 64 by 64, PNG, frames 1 to 6, Overwrite off, the render started from the 3D Viewport with `bpy.ops.render.render('INVOKE_DEFAULT', animation=True)`, as in hand test 37 (question 2). Before it, the output folder held finished frames 1, 2 and 4:

```
o_0001.png 4074 bytes 1792194235.936426130
o_0002.png 4074 bytes 1792194235.941532252
o_0004.png 4074 bytes 1792194235.950291579
```

Blender printed `Saved: '…/o_0003.png'`, `Saved: '…/o_0005.png'` and `Saved: '…/o_0006.png'` and nothing about skipping; afterwards frames 1, 2 and 4 had the same last-changed times to the nanosecond, and 3, 5 and 6 were new (`o_0003.png 4074 bytes 1792194239.086873455`, and so on).

**Outside the interface:** There is no command-line flag for Overwrite or Placeholders. The argument list's only "overwrite" is about argument order: "loading the blend-file overwrites the render output that was set" (https://docs.blender.org/manual/en/5.3/advanced/command_line/arguments.html — read 2026-10-11). Both are scene settings saved in the blend-file and exposed to Python as `use_overwrite` and `use_placeholder`. Setting them from a script passed with `--python` or `--python-expr` before `-a` is **inferred** from the API listing and the argument-order rules. No page shows it.

**Outside the interface — brought up to date by the hand tests of 2026-10-16:** every observed result above was got from Blender started from a terminal with no window, with Overwrite and Placeholders set from Blender's own Python before the render (`scene.render.use_overwrite`, `scene.render.use_placeholder`), and the skipping worked there, with `-a` and with `-f` alike. **Brought up to date again on 2026-10-17:** in a render started inside Blender's window with Overwrite off (hand test 44), frames already there were left untouched too, with no skipping line printed.

**Operating system:** The sources do not say. The Placeholders description compares itself to "Unix ‘touch’", but it is not described as limited to any system.

**Operating system — brought up to date by the hand tests of 2026-10-16:** observed on one operating system, macOS 26.6.2 on the Creator's Mac (Apple M1 Pro). No Linux or Windows machine was used for 5.3, so nothing above is settled for either in this version.

---

## 5. What does an animation written as a single video file leave behind when the render stops part-way?

**Answer.** The manual says only that if a problem occurs, the file "might become unplayable" and every frame must be rendered again from the start. It does not say whether the file is kept, removed or cut short, or how many frames it holds. With **Autosplit Output** on, Blender starts a new file each time a file passes 2 GiB. **Inferred:** a stopped render can then leave more than one file.

**Label:** documented (the "might become unplayable" statement, and Autosplit Output splitting at 2 GiB). Inferred (more than one file left behind when a render stops after an Autosplit). Not in the sources (whether the file is kept or removed, whether it plays, or how many frames it holds).

**Sources:**
- "If a problem occurs while rendering, the file might become unplayable and you will have to re-render all frames from the beginning." — Manual, Output properties, Encoding, Tips. https://docs.blender.org/manual/en/5.3/render/output/properties/output.html — read 2026-10-11.
- "Blender creates one file that holds all the frames of your animation." — Manual, Rendering Animations, Direct Approach. https://docs.blender.org/manual/en/5.3/render/output/animation.html — read 2026-10-11. The same page lists "Stable power supply." among the conditions for choosing that approach.
- Media Type "Video: Encodes each frame into a video container." — Manual, Output properties. https://docs.blender.org/manual/en/5.3/render/output/properties/output.html — read 2026-10-11.
- "Autosplit Output … This will automatically split the output into multiple files after the first file is 2GiB in size." — Manual, Output properties. https://docs.blender.org/manual/en/5.3/render/output/properties/output.html — read 2026-10-11. API `use_autosplit`: "Autosplit output at 2GB boundary (default False)" — https://docs.blender.org/api/5.3/bpy.types.FFmpegSettings.html — read 2026-10-11.

**Inference step (Autosplit):** each file is closed and a new one begun at 2 GiB. If a render stops after one or more splits, the earlier files already exist. Whether those earlier files are playable is not stated.

**Not covered:** whether the file is kept or deleted; whether it plays; whether it differs by container or codec; whether it differs by the way the render was stopped (see 17).

**Where I looked:** searched all 5.3 sources for unplayable, video, movie, FFmpeg, container, codec, autosplit, encode, mux, close, finalize, trailer, header, corrupt, truncat, partial. Read the whole of the Video & Audio Formats page (https://docs.blender.org/manual/en/5.3/files/media/video_formats.html — read 2026-10-11) and the FFmpegSettings API page. Neither says anything about stopped renders. The 5.3 release notes on the sequencer deal with playback and editing, not writing video.

**Observed — hand tests 16, 15, 17 and 20, run by the chat on 2026-10-16.** Label: **observed**. On the Creator's Mac (Apple M1 Pro, its own 14-core graphics through Metal, 16 GB of memory, macOS 26.6.2), in Blender 5.3.0 Beta (hash `5d8151335b6b`, built 2026-10-16), started from a terminal with no window, with a settings folder of its own and `--factory-startup`; the exact commands are in `Audits/Blender Research/Frames/Hand tests — 5.3.md`. Every video was read back by loading it into Blender as a movie clip, the only video reader on hand; that reader says `loaded ok` even for a file with nothing in it, so the size and the frame count it gives are what count.

*A video stopped part-way* (hand test 16). Blender's startup scene, Cycles on the processor, 1000 samples, 400 by 400, frames 1 to 20, one FFmpeg MPEG-4 H.264 video, about 6 seconds a frame, the folder listed every 50 milliseconds. The file appeared within a second of the start, already named for the whole range and at 48 bytes — `[    0.887] WATCH movie_0001-0020.mp4=48` — and stayed at 48 bytes while `Video append frame 1`, `2` and `3` were printed. A signal was sent 20 seconds in, during frame 4:

- *Interrupt.* `Sent an internal break event. Press ^C again to kill Blender`, then `Fra: 4 | Mem: 3M | Cancelled` and `Blender quit`, exit status 1, 5.30 seconds after the interrupt. The file grew once, at the end — `[   25.266] WATCH movie_0001-0020.mp4=2427` — and Blender's reader gave `READVID movie_0001-0020.mp4 | 2427 bytes | loaded ok | frame_duration: 3 | size: (400, 400) | fps: 24.0`.
- *Terminate.* Exit status -15, gone at once, nothing printed. Left: `movie_0001-0020.mp4  48 bytes`, read as `frame_duration: 0 | size: (256, 256) | fps: 0.0`.
- *Kill.* Exit status -9, gone at once. Left: 48 bytes, `frame_duration: 0`.
- *Interrupt with Autosplit Output on* (below the 2 GiB boundary). The file was named `movie__0000001-0020.mp4` — an extra underscore and seven-digit numbers. The interrupt came as frame 3 was ending; Blender printed `Sent an internal break event…`, `Fra: 3 | Mem: 3M | Cancelled` and `Blender quit` within 0.10 seconds, and left 2288 bytes, read as `frame_duration: 2`.

*Rendering the same range again over a finished video with Overwrite off* (hand test 16 e). Two frames, uninterrupted, left `movie_0001-0002.mp4 2288 bytes 1792182056.708823102`. The same render again with Overwrite off printed no skipping line, rendered both frames (`Video append frame 1`, `Video append frame 2`, `Time: 00:06.22`, `Time: 00:06.15`) and left `movie_0001-0002.mp4 2288 bytes 1792182070.260996728`, its last-changed time moved.

The video stopped while the last image of a separate image sequence was being written, and the video with every image written but not yet closed, are under questions 12 and 17.

*The 2 GiB boundary with Autosplit Output on* (hand test 20, run 2026-10-17). To make a video big enough to reach it, the world was made of fine noise (a Noise Texture at scale 4000 and detail 15, in window coordinates, as the world's colour), so the picture hardly compresses; Cycles on the processor, 1 sample, 3840 by 2160, Matroska with the FFV1 codec and lossless output, Autosplit Output on, frames 1 to 300, output `…/e20/big_`. A Python handler on Blender's own "frame written" event listed the folder each time it fired, and three frames after a second file appeared it sent Blender's own process an interrupt (`SIGINT`). Each frame took about 3.4 seconds and added about 16 MB. The listing at the frames around the boundary:

```
SPLIT render_write frame 124 | [('big__0000001-0300.mkv', 1992294400)]
07:07.744  render           | Video append frame 125
SPLIT render_write frame 125 | [('big__0000001-0300.mkv', 2024699916), ('big__0010001-0300.mkv', 0)]
07:11.283  render           | Video append frame 126
SPLIT render_write frame 126 | [('big__0000001-0300.mkv', 2024699916), ('big__0010001-0300.mkv', 0)]
07:14.742  render           | Video append frame 127
SPLIT render_write frame 127 | [('big__0000001-0300.mkv', 2024699916), ('big__0010001-0300.mkv', 16252928)]
07:18.152  render           | Video append frame 128
SPLIT render_write frame 128 | [('big__0000001-0300.mkv', 2024699916), ('big__0010001-0300.mkv', 32243712)]
SPLIT third frame after the second file appeared; sending SIGINT to own process
Sent an internal break event. Press ^C again to kill Blender
07:18.153  render           | Rendering frame 129
07:18.153  render           | Start rendering: Scene, ViewLayer
07:18.153  render           | Engine: Cycles
Blender quit
```

So the first file stopped growing at **2,024,699,916 bytes**, below 2 GiB (2,147,483,648 bytes), when frame 125 was written, and a second file was started under a name whose leading digits are a counter, `big__0010001-0300.mkv`, not the frame it starts at. No line was printed about the split. Exit status 1, 0.06 seconds after the interrupt. Left:

```
big__0000001-0300.mkv  2024699916 bytes
big__0010001-0300.mkv  48672835 bytes
```

Blender's own movie reader: `READVID big__0000001-0300.mkv | 2024699916 bytes | loaded ok | frame_duration: 125 | size: (3840, 2160) | fps: 24.0` and `READVID big__0010001-0300.mkv | 48672835 bytes | loaded ok | frame_duration: 3 | size: (3840, 2160) | fps: 24.0`.

**Outside the interface:** The sources do not say whether this differs from the command line. The warning is not limited to the interface.

**Outside the interface — brought up to date by the hand tests of 2026-10-16:** the results above were got from Blender started from a terminal with no window — the video file's behaviour, every way of stopping such a render, and Overwrite against a video. A video render stopped from inside the window (2026-10-17) is under question 17: after a simulated click on the status bar's cross two seconds into frame 3, the file was 2288 bytes and Blender's reader found 2 frames in it; after a kill, 48 bytes and none.

**Operating system:** The sources do not say.

**Operating system — brought up to date by the hand tests of 2026-10-16:** observed on one operating system, macOS 26.6.2 on the Creator's Mac (Apple M1 Pro). No Linux or Windows machine was used for 5.3, so nothing above is settled for either in this version. Only Blender's own movie reader was used to read the files. **Not run in 5.3, and why: another video player.** The rules the hand tests ran under allowed Blender and no other program to be run, so whether these files play in QuickTime Player or any other player is not settled for 5.3.

---

## 6. Is a file existing proof that the frame finished writing?

**Answer.** No, not in general. With **Placeholders** on, Blender creates an empty file for a frame while that frame is still rendering. So a file can exist for a frame that has not finished. With Placeholders off, the sources do not say whether Blender writes straight to the final name or writes elsewhere and then moves the file. So they do not settle whether a part-written file can sit at the final name. For video, the manual's warning that a problem can leave the file "unplayable" means an existing video file is not proof it finished. The one documented signal that a frame has been written is the `render_write` handler, called "directly after the frame is written".

**Label:** inferred (that a file existing is not proof when Placeholders is on, and for video); not in the sources (whether a part-written image file can exist at the final path when Placeholders is off); documented (that Placeholders creates empty files while rendering, that a video file might become unplayable on a problem, and that the `render_write` handler runs directly after a frame is written). The inference steps:
- Placeholders: the source says it creates "empty placeholder" files "while rendering". An empty file made while rendering exists before the frame is done. So existence is not proof.
- Video: the source says the file "might become unplayable" after a problem, which implies the file can exist on disk without having finished properly.

**Sources:**
- "Create empty placeholder frames while rendering." — Manual, Output properties (second "Image Sequence" entry; see the note at the top). https://docs.blender.org/manual/en/5.3/render/output/properties/output.html — read 2026-10-11.
- `use_placeholder`: "Create empty placeholder files while rendering frames (similar to Unix ‘touch’) (default False)" — https://docs.blender.org/api/5.3/bpy.types.RenderSettings.html — read 2026-10-11.
- `render_write`: "on writing a render frame (directly after the frame is written)." — https://docs.blender.org/api/5.3/bpy.app.handlers.html — read 2026-10-11.
- "the file might become unplayable" — Manual, Output properties, Encoding Tips. https://docs.blender.org/manual/en/5.3/render/output/properties/output.html — read 2026-10-11.

**Not covered:**
- Whether an image is written in place or through a temporary file.
- Whether Blender removes a placeholder if the frame fails or the render is stopped.
- Whether a file of non-zero size can be incomplete.

**Where I looked:** searched all 5.3 sources for placeholder, touch, empty, temporary, temp, .tmp, rename, atomic, partial, incomplete, truncat, corrupt, write, written, render_write. The "temporary files" hits are about the TEMP/TMPDIR variables, autosave and copy-paste buffers, not render output.

**Observed — hand tests 8, 13 and 14, run by the chat on 2026-10-16.** Label: **observed**. On the Creator's Mac (Apple M1 Pro, its own 14-core graphics through Metal, 16 GB of memory, macOS 26.6.2), in Blender 5.3.0 Beta (hash `5d8151335b6b`, built 2026-10-16), started from a terminal with no window, with a settings folder of its own and `--factory-startup`; the exact commands are in `Audits/Blender Research/Frames/Hand tests — 5.3.md`.

*When a placeholder is made* (hand test 8). Blender's startup scene, Cycles on the processor, 200 samples, 200 by 200, PNG, frames 1 to 3, Placeholders on and Overwrite off, with a Python handler listing the output folder at each of Blender's own render events:

```
WATCH [before the render starts] folder holds: (nothing)
WATCH [render_pre, frame 1] folder holds: f_0001.png=0 bytes
00:00.728  render           | Saved: '…/e08/a/f_0001.png'
WATCH [render_post, frame 1] folder holds: f_0001.png=25009 bytes
WATCH [render_write, frame 1] folder holds: f_0001.png=25009 bytes
WATCH [render_pre, frame 2] folder holds: f_0001.png=25009 bytes, f_0002.png=0 bytes
00:01.052  render           | Saved: '…/e08/a/f_0002.png'
WATCH [render_post, frame 2] folder holds: f_0001.png=25009 bytes, f_0002.png=25009 bytes
WATCH [render_write, frame 2] folder holds: f_0001.png=25009 bytes, f_0002.png=25009 bytes
WATCH [render_pre, frame 3] folder holds: f_0001.png=25009 bytes, f_0002.png=25009 bytes, f_0003.png=0 bytes
00:01.380  render           | Saved: '…/e08/a/f_0003.png'
WATCH [render_post, frame 3] folder holds: f_0001.png=25009 bytes, f_0002.png=25009 bytes, f_0003.png=25009 bytes
WATCH [render_write, frame 3] folder holds: f_0001.png=25009 bytes, f_0002.png=25009 bytes, f_0003.png=25009 bytes
WATCH [render_complete] folder holds: f_0001.png=25009 bytes, f_0002.png=25009 bytes, f_0003.png=25009 bytes
```

With Placeholders off the same listing at each `render_pre` held nothing for the frame about to render (`WATCH [render_pre, frame 1] folder holds: (nothing)`, `WATCH [render_pre, frame 2] folder holds: f_0001.png=25009 bytes`).

*How a frame's file is written* (hand test 13). Cycles, 1 sample, 4096 by 4096, OpenEXR 32-bit float uncompressed, frames 1 to 3, the folder listed every 2 milliseconds. The only names that ever appeared were the frames' own final names, `frame_0001.exr` to `frame_0003.exr` — no other name, hidden or temporary. Each appeared at a few megabytes and grew in place, frame 1 through 127 different sizes in about 0.3 seconds:

```
[    1.582] WATCH frame_0001.exr=1794898
[    1.585] WATCH frame_0001.exr=4113234
[    1.587] WATCH frame_0001.exr=6529874
…
[    1.896] WATCH frame_0001.exr=266974034
[    1.899] WATCH frame_0001.exr=268501842
```

With Placeholders on the same run showed `frame_0001.exr=0` at 0.817 seconds, before any pixel was written, and frame 2's zero-byte file only at 1.923 seconds, after frame 1 had reached its full size.

So on this machine a file at a frame's name was, at different moments, an empty placeholder, a file part-way through being written, and a finished frame. What a stop leaves when it lands while the file is being written — sizes, and whether Blender's own loader opens what is left — is under question 13 (hand test 14).

**Outside the interface:** Placeholders is a scene setting with no command-line flag (see 4). The `render_write` handler works from Python scripts, so it is available when Blender is driven from outside. The sources do not say whether any of this differs outside the interface.

**Outside the interface — brought up to date by the hand tests of 2026-10-16:** every observed result above was got from Blender started from a terminal with no window. Placeholders, set from Blender's own Python, made its zero-byte files there, and the `render_write`, `render_pre` and `render_post` handlers fired there.

**Operating system:** The sources do not say.

**Operating system — brought up to date by the hand tests of 2026-10-16:** observed on one operating system, macOS 26.6.2 on the Creator's Mac (Apple M1 Pro). No Linux or Windows machine was used for 5.3, so nothing above is settled for either in this version.

---

## 7. Can Blender be given a set of frames that is not one plain run — single frames, or two separate stretches?

**Answer.** Yes, from the command line. `-f` / `--render-frame` takes a comma-separated list of frames, and a range written with `..` between its first and last frame. **Inferred:** two separate stretches are written by putting two `..` ranges in one comma list, for example `1..10,20..30`. The page describes the list and the range separately and gives no example joining them. In the interface, the Frame Range panel offers only Start, End and Step: one run with a fixed step. The Python render operator takes only a start and an end frame.

**Label:** documented (list of single frames and one `..` range with `-f`; Frame Range panel and `bpy.ops.render.render` offering only Start, End and Step). Inferred (several `..` ranges, or ranges and single frames, in one comma list). Not in the sources (any way to give a list of separate frames or two stretches inside the interface).

**Sources:**
- "-f, --render-frame <frame> — Render frame <frame> and save it. +<frame> start frame relative, -<frame> end frame relative. A comma separated list of frames can also be used (no spaces). A range of frames can be expressed using .. separator between the first and last frames (inclusive)." — Manual, Command Line Arguments. https://docs.blender.org/manual/en/5.3/advanced/command_line/arguments.html — read 2026-10-11.
- "Arguments are executed in the order they are given" — Manual, Command Line Arguments. https://docs.blender.org/manual/en/5.3/advanced/command_line/arguments.html — read 2026-10-11.
- "Always position -f or -a as the last arguments." — Manual, Rendering From The Command Line. https://docs.blender.org/manual/en/5.3/advanced/command_line/render.html — read 2026-10-11.
- Frame Range panel: "Frame Start, End", "Step" — https://docs.blender.org/manual/en/5.3/render/output/properties/frame_range.html — read 2026-10-11. The panel picture `_images/render_output_properties_frame_range_panel.png` shows only Frame Start, End, Step and a closed Time Stretching sub-panel (documented).
- `bpy.ops.render.render(*, animation=False, …, frame_start=0, frame_end=0)` — https://docs.blender.org/api/5.3/bpy.ops.render.html — read 2026-10-11.

**Inference step:** the help says a comma list may hold frames, and that a range is written with `..`. Putting a range where a frame would go in the list is the natural reading, but the page does not show it.

**Not covered:** the order such a list is rendered in (see 10); what happens with repeated or overlapping frames; whether `-f` with a list honours Overwrite and Placeholders as `-a` does; how `-j` (frame step) affects a `..` range given to `-f`.

**Where I looked:** read the whole Command Line Arguments page and the Rendering From The Command Line page (https://docs.blender.org/manual/en/5.3/advanced/command_line/render.html — read 2026-10-11). Searched all 5.3 sources for render-frame, render-anim, frame-jump, "..", comma separated, frame list, list of frames, non-contiguous, frame_start, frame_end. Checked `bpy.ops.render.render` and `bpy.ops.render.opengl`.

**Observed — hand test 2, run by the chat on 2026-10-16.** Label: **observed**. On the Creator's Mac (Apple M1 Pro, its own 14-core graphics through Metal, 16 GB of memory, macOS 26.6.2), in Blender 5.3.0 Beta (hash `5d8151335b6b`, built 2026-10-16), started from a terminal with no window, with a settings folder of its own and `--factory-startup`. Blender's startup scene, Workbench, 64 by 64 pixels, PNG, a fresh empty folder for each run. The exact commands are in `Audits/Blender Research/Frames/Hand tests — 5.3.md`.

*Two stretches in one list, `-f 1..3,10..12`* — the form the answer above marks inferred. Exit status 0, six files, `f_0001` to `f_0003` and `f_0010` to `f_0012`:

```
00:00.353  render           | Rendering single frame
00:00.353  render           | Rendering frame 1
00:00.531  render           | Saved: '…/f_0001.png'
00:00.531  render           | Rendering single frame
00:00.531  render           | Rendering frame 2
00:00.537  render           | Saved: '…/f_0002.png'
00:00.537  render           | Rendering single frame
00:00.537  render           | Rendering frame 3
00:00.542  render           | Saved: '…/f_0003.png'
00:00.542  render           | Rendering single frame
00:00.542  render           | Rendering frame 10
00:00.547  render           | Saved: '…/f_0010.png'
00:00.547  render           | Rendering single frame
00:00.547  render           | Rendering frame 11
00:00.553  render           | Saved: '…/f_0011.png'
00:00.553  render           | Rendering single frame
00:00.553  render           | Rendering frame 12
00:00.558  render           | Saved: '…/f_0012.png'
```

*Order and repeats.* `-f 5,1,9` saved 5, then 1, then 9. `-f 5,1,3..4,2` saved 5, 1, 3, 4, 2, in that order. `-f 2,2,3` printed `Saved: '…/f_0002.png'` twice and then frame 3, and left two files.

*Malformed lists.* Each was refused, nothing was rendered, and the exit status was 0 every time:

```
Error: incorrect comma use '-f / --render-frame 1,,3'.
Warning: negative range ignored '-f / --render-frame 3..1'.
Error: not a number '-f / --render-frame 1-3'.
```

A list ending in a comma — made by accident, the first time the five-thousand-frame list below was built — was refused the same way, `Error: incorrect comma use '-f / --render-frame 1,2,3, … ,4999,5000,'.`, with exit status 0 and nothing rendered.

*More than one request in one command.* `-s 1 -e 2 -a -s 20 -e 21 -a` printed `Rendering animation (frames 1..2)` and then `Rendering animation (frames 20..21)` and wrote four files. `-f 1..3 -f 10..12` wrote six. `-s 1 -e 2 -a -f 20` wrote frames 1, 2 and then 20, the last announced as `Rendering single frame`.

*Spaces inside the list.* `-f "1, 2, 3"` rendered and wrote frames 1, 2 and 3, exit status 0.

*How long a list may be.* A list of five thousand frames, 23,892 characters, was accepted: 5000 `Saved:` lines, 5000 files, exit status 0, 28 seconds. A list of two hundred thousand frames never reached Blender; zsh answered `(eval):4: argument list too long: /Applications/Blender 5.3b.app/Contents/MacOS/Blender` and the next command printed `exit 127`.

**Observed — hand tests 39, 40 and 36, run by the chat on 2026-10-17: with Blender's window open.** Label: **observed**. Same Mac and Blender, started from the terminal without `-b`, so Blender opened its window on the Mac's screen. Where a menu was opened, it was opened by a click sent as Blender's own simulated event (Blender started with `--enable-event-simulate`), not by a person's hand, after the Quick Setup panel that factory settings open had been closed by a simulated Escape; the window was photographed with Blender's own screenshot command.

- *`-f` with the window open* (hand test 39). The command of hand test 2's first run, `-f 1..3,10..12`, without `-b`. Blender printed the same lines as with `-b` — `Rendering single frame`, `Rendering frame 1`, `Saved: '…/e39/g_0001.png'`, and so on — for frames 1, 2, 3, 10, 11 and 12 in that order, and wrote the same six files, `g_0001.png` to `g_0003.png` and `g_0010.png` to `g_0012.png`, 4076 bytes each. It then stayed open with its window up until it was sent a terminate 40 seconds in (exit status -15).
- *The keyframes-only playblast* (hand test 40). Blender's startup scene, the cube's location keyed at frame 3 (height 0) and at frame 5 (height 1) and nowhere else, the cube selected and active, frames 1 to 6, PNG. From a terminal with no window, `bpy.ops.render.opengl(animation=True, render_keyed_only=True)` gave:

  ```
  Error: Cannot use OpenGL render in background mode (no opengl context)
  RAISED: RuntimeError Error: Cannot use OpenGL render in background mode (no opengl context)
  FILES: []
  ```

  With the window open, the same call, made from the 3D Viewport two seconds after the window opened, returned `{'FINISHED'}`, printed its `Saved:` lines in the order `p_0001`, `p_0003`, `p_0002`, `p_0006`, `p_0004`, `p_0005`, and left six files:

  ```
  p_0001.png  1466786 bytes
  p_0002.png  1466786 bytes
  p_0003.png  1466786 bytes
  p_0004.png  1466786 bytes
  p_0005.png  1467544 bytes
  p_0006.png  1467544 bytes
  ```

  Compared value by value, frames 1, 2, 3 and 4 held the same picture, and frames 5 and 6 the same picture; 4 against 5: `differing: 111164 | largest difference: 0.5960785150527954`. No two files were the same byte for byte (`cmp` on the first two: `differ: char 663, line 5`; six different checksums).
- *The View menu* (hand test 36), opened by a click on "View" in the 3D Viewport's header. Its photograph shows, after `Play Animation`, the three items `Render Viewport Preview`, `Render Playblast` and `Render Playblast on Keyframes`. With the simulated pointer then moved onto `Render Playblast on Keyframes` (hand test 43), the item lit up, and **no tooltip appeared** in photographs taken two and seven seconds after the pointer came to rest there; the tooltip's words were therefore not read in 5.3. No person's pointer was used in this sitting (see question 2), so whether a person resting the pointer there sees one is not settled.

**Outside the interface:** This is a command-line feature, so it is available from outside. The manual adds: "Always position -f or -a as the last arguments." (https://docs.blender.org/manual/en/5.3/advanced/command_line/render.html — read 2026-10-11), and "Arguments are executed in the order they are given" (https://docs.blender.org/manual/en/5.3/advanced/command_line/arguments.html — read 2026-10-11). Inside the interface, the sources show no way to give such a list.

**Outside the interface — brought up to date by the hand tests of 2026-10-16:** every `-f` list above was given to Blender started from a terminal with no window (`-b`), and there it was accepted with ranges and single frames mixed, rendered in the order written, and refused when malformed. **Brought up to date again on 2026-10-17:** given without `-b`, so that the window opened, `-f 1..3,10..12` wrote the same six files (hand test 39); the keyframes-only playblast ran only with the window open, and refused to run without one (hand test 40).

**Operating system:** The sources do not say.

**Operating system — brought up to date by the hand tests of 2026-10-16:** observed on one operating system, macOS 26.6.2 on the Creator's Mac (Apple M1 Pro). No Linux or Windows machine was used for 5.3, so nothing above is settled for either in this version.

---

## 8. What does a frame range consist of?

*The question can be read two ways: which settings make up a frame range, or which frames it includes. I answer both.*

**Answer.** The settings are a **Start** frame, an **End** frame and a **Step**. Step is how many frames to advance after each one rendered; above 1 it skips frames. **Time Stretching** (Old and New) sits in the same panel. It remaps animation speed and does not change Start or End. On which frames are included: a whole-animation render goes from start to end, both included. **Inferred:** with Step above 1 the frames are Start, then Start plus Step, and so on. Whether End is rendered when the steps do not land on it is not stated. The timeline's Preview Range is a separate range that does not affect rendering.

**Label:** documented (the settings, the ranges, start to end inclusive, Preview Range not affecting rendering). Inferred (which frames a step above 1 picks). Not in the sources (whether End is rendered when Step does not land on it).

**Sources:**
- "Frame Start, End — Set the Start and End frames for Rendering Animations." "Step — Controls the number of frames to advance for each frame rendered, a value above 1 skips frames." "Using Time Stretching will not influence the Start or End frames set above" — Manual, Frame Range. https://docs.blender.org/manual/en/5.3/render/output/properties/frame_range.html — read 2026-10-11.
- "-a, --render-anim — Render frames from start to end (inclusive)." "-s … Set start to frame <frame>, supports +/- for relative frames too." "-e … Set end to frame <frame>, supports +/- for relative frames too." "-j, --frame-jump <frames> — Set number of frames to step forward after each rendered frame." — https://docs.blender.org/manual/en/5.3/advanced/command_line/arguments.html — read 2026-10-11.
- Scene `frame_start`: "First frame of the playback/rendering range (in [0, 1048574], default 1)". `frame_end`: "Final frame of the playback/rendering range (in [0, 1048574], default 250)". `frame_step`: "Number of frames to skip forward while rendering/playing back each frame (in [0, 1048574], default 1)". — https://docs.blender.org/api/5.3/bpy.types.Scene.html — read 2026-10-11.
- `bpy.ops.render.render`: `frame_start` / `frame_end`: "If not specified, the scene start frame will be assumed. This should only be specified if doing an animation render" — https://docs.blender.org/api/5.3/bpy.ops.render.html — read 2026-10-11.
- Preview Range: "This range only affects the preview in the 3D Viewport; it doesn’t affect rendering." — https://docs.blender.org/manual/en/5.3/animation/animation_editors.html — read 2026-10-11.

**Inference step (step above 1):** "advance for each frame rendered" plus "from start to end" gives Start, Start+Step, Start+2×Step … The sources do not say whether the last value past End is clamped to End or dropped.

**Not covered:** what a Step of 0 does (the API range allows 0); whether End is rendered when Step does not land on it; frame numbers below 0 (the API range starts at 0, while `-f` accepts "-<frame> end frame relative").

**Where I looked:** Frame Range page, Format page, Output properties index, Command Line Arguments, Scene and RenderSettings API pages, timeline and animation-editor pages. Searched for frame range, frame_step, frame step, frame-jump, preview range, inclusive.

**Observed — hand test 3, run by the chat on 2026-10-16.** Label: **observed**. On the Creator's Mac (Apple M1 Pro, its own 14-core graphics through Metal, 16 GB of memory, macOS 26.6.2), in Blender 5.3.0 Beta (hash `5d8151335b6b`, built 2026-10-16), started from a terminal with no window, with a settings folder of its own and `--factory-startup`. Blender's startup scene, Workbench, 64 by 64 pixels, PNG, a fresh empty folder for each run. The exact commands are in `Audits/Blender Research/Frames/Hand tests — 5.3.md`.

- *Start 1, end 10, `-j 3`:* files for frames 1, 4, 7 and 10. Each stepped frame was announced three times — `Rendering frame 4` three lines running, then `Saved: '…/f_0004.png'` — and nothing was printed past frame 10.
- *Start 3, end 12, `-j 4`:* files for 3, 7 and 11 only; frame 12 was not rendered. The run ended with a bare `00:00.546  render           | Rendering frame 15`, a frame past the end, for which nothing was rendered or written.
- *Start 1, end 10, `-j 4`:* files for 1, 5 and 9, then a bare `Rendering frame 13`.
- *The scene's own step set to 3 instead of `-j`* (start 1, end 10): the same four files, 1, 4, 7 and 10, and the same printing.
- *A step of 0* (start 1, end 5): the setting read back as `step=0`. Blender wrote frame 1 once, then announced frame 1 four more times without rendering or writing, and ended by itself with exit status 0:

  ```
  00:00.342  render           | Rendering animation (frames 1..5)
  00:00.343  render           | Rendering frame 1
  00:00.511  render           | Saved: '…/e03/e/f_0001.png'
  00:00.511  render           | Rendering frame 1
  00:00.511  render           | Rendering frame 1
  00:00.511  render           | Rendering frame 1
  00:00.511  render           | Rendering frame 1
  ```

  One file was left, `f_0001.png`.
- *An end below the start* (start 5, end 2): `00:00.347  render           | Rendering animation (frames 5..2)`, nothing else, exit status 0, an empty folder.
- *The alternative playback range:* frames 1 to 8 with `use_preview_range` on and the playback range 4 to 5 (`PREVIEW True 4 5` printed back). The render announced `Rendering animation (frames 1..8)` and wrote all eight frames.
- *Step against a `-f` list* (hand test 2): with the scene's step set to 3 (read back as `step=3`), `-f 1,2,3,4,5` wrote all five files; and `-j 4 -f 1..10` wrote all ten. Each item was announced as `Rendering single frame`.
- *The Frame Range panel with a step of 0, in Blender's window* (hand test 36, 2026-10-17). Blender opened with its window, Properties switched to the Output tab and the scene's step set to 0 from Blender's own Python (read back as `frame_step reads 0`). The photograph shows the Frame Range panel holding `Frame Start` `1`, `End` `250`, `Step` `0` and a closed `Time Stretching` section, and nothing else. The same panel opened as a popup with `bpy.ops.wm.call_panel(name='RENDER_PT_frame_range')` returned `{'INTERFACE'}`; the popup showed `Frame Start` `1`, `End` `250`, `Step` `0` and `Time Stretching`, and Blender carried on and later quit with exit status 0. Typing the 0 into the field is under hand test 43 below.
- *Typing 0 into the Step field, and a right click on the panel's header* (hand test 43, 2026-10-17). The same window, Properties on the Output tab, the scene's step left at 1; every key and click a simulated event, not a person's hand, at positions read off the photograph above. A click on the Step field turned it into a text box showing `1`; the key `0` was sent and the box showed `0`; after Return the field showed `0` with its arrows, and the scene's step read back `frame_step reads 0`. Nothing else changed in the panel. A right click on the `Frame Range` header opened a menu headed `Context Menu` with one item, `Area`, whose submenu begins `Horizontal Split`, `Vertical Split`, `Maximize Area`, `Focus Mode`, `Duplicate Area into New Window`.

  The first of five runs sent the key's release with the character attached, and Blender refused that half: `STEP FAILED: RuntimeError Error: Value: must be 'PRESS' when unicode is set`; the press had gone, and the step read `0` after Return there too. In the first three runs each photograph showed the window as it had been one change earlier; from the fourth the window was redrawn twice before each photograph, and the photographs above are from the fourth and fifth.

These runs bear on the two points the answer above marks inferred and not in the sources — which frames a step picks, and whether End is rendered when the steps miss it — and on what a Step of 0 does.

**Outside the interface:** `-s`, `-e`, `-j` and `-a` set and use the range from the command line. `bpy.ops.render.render` takes `frame_start` and `frame_end`. Its text says: "If not specified, the scene start frame will be assumed. This should only be specified if doing an animation render" (https://docs.blender.org/api/5.3/bpy.ops.render.html — read 2026-10-11).

**Outside the interface — brought up to date by the hand tests of 2026-10-16:** the stepped runs, the step of 0, the end below the start and the playback range above were all run from a terminal with no window, and `-j` and the scene's own step gave the same frames. **Brought up to date again on 2026-10-17:** in Blender's window the Frame Range panel showed a step of 0 and kept it, whether set from Python (hand test 36) or typed into the field with simulated keys (hand test 43).

**Operating system:** The sources do not say.

**Operating system — brought up to date by the hand tests of 2026-10-16:** observed on one operating system, macOS 26.6.2 on the Creator's Mac (Apple M1 Pro). No Linux or Windows machine was used for 5.3, so nothing above is settled for either in this version.

---

## 9. What does a frame write when several view layers are active on it?

*Ambiguity: "write" could mean which files appear on disk for that frame, or what is inside them. I answer both as far as the sources go.*

**Answer.**
- Every view layer with **Use for Rendering** on is rendered for the frame. From the command line all of them are rendered, because "Render Single Layer" is ignored there.
- A render engine's per-frame finishing step runs after all view layers of that frame are done. Cycles has this step (observed in its files).
- With the **Multi-Layer EXR** output type, the frame goes into one OpenEXR file that can hold several layers and passes. **Inferred:** each active view layer appears as a layer in it.
- With a single-image format such as PNG, and the compositor in use, the file holds the compositor's result: the first image input of the Group Output node.
- The sources do **not** say which view layer's image goes into a single-image file when the compositor is off. They also do not say which view layer feeds the compositor's default Group Input image when several are rendered.
- The main output writes one file per frame for Multi-Layer EXR (documented) and (**inferred**) one file per frame for the Image media type. The sources do not say it writes one file per view layer.

**Label:** mixed. Documented: rendering of every enabled view layer; Render Single Layer ignored from the command line; finishing after all view layers; Multi-Layer EXR saving in a single file and OpenEXR holding several layers; Group Output's first image being the render result. Observed: Cycles defines the finishing step. Inferred: all active view layers stored in the Multi-Layer EXR file; the Image media type writing one file per frame rather than one per view layer. Not in the sources: which view layer a single-image file holds without compositing; which view layer feeds the compositor's default Group Input image when several are rendered.

**Sources:**
- "Use for Rendering — The active view layer will be used during rendering." "Render Single Layer — Only render the active view layer. Note: This option is ignored when rendering from the command line." — Manual, View Layer. https://docs.blender.org/manual/en/5.3/render/layers/view_layer.html — read 2026-10-11. Panel picture `_images/render_layers_view-layer_panel.png` shows both checkboxes (documented).
- ViewLayer `use`: "Enable or disable rendering of this View Layer (default True)" — https://docs.blender.org/api/5.3/bpy.types.ViewLayer.html. RenderSettings `use_single_layer`: "Only render the active layer. Only affects rendering from the interface, ignored for rendering from command line. (default False)" — https://docs.blender.org/api/5.3/bpy.types.RenderSettings.html. Both read 2026-10-11.
- `frame_path(*, frame=…, preview=False, view='')`: "Return the absolute path to the filename to be written for a given frame" — https://docs.blender.org/api/5.3/bpy.types.RenderSettings.html — read 2026-10-11.
- `render_frame_finish()`: "Perform finishing operations after all view layers in a frame were rendered" — https://docs.blender.org/api/5.3/bpy.types.RenderEngine.html — read 2026-10-11.
- Cycles' own file `__init__.py`, lines 76–77: `def render_frame_finish(self):` / `engine.render_frame_finish(self)`. And `engine.py`, lines 95–100, `def render_frame_finish(engine):` … `_cycles.render_frame_finish(engine.session)`. — **observed**, copy taken 2026-10-16.
- Media Type "Multi-Layer EXR: Saves all inputs together in a single multi-layer OpenEXR file." — https://docs.blender.org/manual/en/5.3/render/output/properties/output.html — read 2026-10-11. (The wording, "inputs", is the same as on the compositor File Output node page. The Output page does not say what the "inputs" are for the main render output.)
- "An OpenEXR file can store multiple layers and passes. This means OpenEXR images can be loaded into a Compositor keeping render layers and passes intact." — https://docs.blender.org/manual/en/5.3/files/media/image_formats.html — read 2026-10-11.
- "In the Compositor, only the first image input is used as the final render result." — https://docs.blender.org/manual/en/5.3/compositing/types/output/group_output.html. "By default, the Image socket contains the rendered image, equivalent to the Combined render pass." and "By default, the Image output is used as the final composited image for rendering." — https://docs.blender.org/manual/en/5.3/compositing/introduction.html. Both read 2026-10-11.
- "Pipeline – Compositing — Renders the output from the compositing node setup, and then applies the Composite node tree on all images, displaying the image inputted in the Group Output node." — https://docs.blender.org/manual/en/5.3/render/output/properties/post_processing.html — read 2026-10-11.
- Release note: "The default compositor node tree now uses a simple **Group Input** connected to a **Group Output** with no **Render Layers** node." — https://developer.blender.org/docs/release_notes/5.3/compositor/ — read 2026-10-11.
- Render Layers node: "The Render Layers node renders a View Layer from a scene and outputs its image and any enabled render passes to the compositor." — https://docs.blender.org/manual/en/5.3/compositing/types/input/scene/render_layers.html — read 2026-10-11.
- Separate from the frame's output file: "Cache Result — Saves the rendered view layers and their passes to a multi-layer OpenEXR image." It is stored "in the Render Cache folder as specified in the File Paths Preferences." — https://docs.blender.org/manual/en/5.3/render/output/properties/output.html — read 2026-10-11.
- `OPEN_EXR` and `OPEN_EXR_MULTILAYER` listed under "Formats that can be compiled into Blender, not available on all systems" — https://docs.blender.org/manual/en/5.3/advanced/command_line/arguments.html — read 2026-10-11.

**Inference steps:**
- Multi-Layer EXR: OpenEXR is documented to keep "render layers and passes intact". The Multi-Layer EXR type saves "all inputs together" in one file. Read together, each rendered view layer is a layer in that one file. No page lists the layers such a file contains.
- One file per frame for Image: `frame_path` returns a single filename for a given frame and takes no view-layer parameter, and no source mentions per-view-layer filenames for the main output.

**Not covered:**
- Which view layer a single-image file (PNG, JPEG and so on) holds when compositing is off.
- Which view layer the compositor's default Group Input "Image" carries when several are rendered.
- Whether compositor File Output nodes write separate files per view layer (only if wired that way; not discussed).
- Whether a stop between two view layers of one frame leaves anything for that frame.

**Where I looked:** View Layer pages (introduction, view layer, passes, filter, override); Output, Post Processing and Format pages; compositing introduction, Group Input, Group Output, File Output and Render Layers pages; image formats (OpenEXR); Image Editor introduction (Render Result View Layer selector); 5.3 compositor and rendering release notes; RenderSettings, ViewLayer, RenderEngine, ImageFormatSettings, Image and ImageUser API pages; Cycles `__init__.py` and `engine.py`. Searched for view layer, all view layers, each view layer, multiple view layers, multi-layer, multilayer, MULTI_LAYER, first layer, final render result, composite.

**Observed — hand test 21, run by the chat on 2026-10-16.** Label: **observed**. On the Creator's Mac (Apple M1 Pro, its own 14-core graphics through Metal, 16 GB of memory, macOS 26.6.2), in Blender 5.3.0 Beta (hash `5d8151335b6b`, built 2026-10-16), started from a terminal with no window, with a settings folder of its own and `--factory-startup`; the exact commands are in `Audits/Blender Research/Frames/Hand tests — 5.3.md`. A script (`viewlayers.py`) moved Blender's startup cube into a collection of its own and added a second view layer, `Second`, that leaves that collection out — so `ViewLayer` shows the cube and `Second` does not. Cycles on the processor, 32 samples, 200 by 200, frame 1 only (`-f 1`), PNG unless said otherwise.

*Files written.* With both view layers on and the compositor off, frame 1 wrote **one** file, `both_0001.png`, 25552 bytes, named from the output path and the frame number alone — no view-layer name in it. The same held with Render Single Layer on (`single_0001.png`, 25552 bytes), with only the first layer on (`first_0001.png`, 25370 bytes), and with the compositor on (`comp_default_0001.png`, `comp_first_0001.png`, `comp_second_0001.png`): one file for the frame each time.

*Which picture is in the one file.* Compared value by value (`pixcmp.py`, which loads both pictures with Blender's own image loader and counts the values that differ):

```
PIX both_0001.png vs first_0001.png | values: 160000 | differing: 0 | largest difference: 0.0
PIX single_0001.png vs both_0001.png | values: 160000 | differing: 0 | largest difference: 0.0
PIX comp_default_0001.png vs first_0001.png | values: 160000 | differing: 0 | largest difference: 0.0
PIX comp_first_0001.png vs first_0001.png | values: 160000 | differing: 0 | largest difference: 0.0
PIX comp_second_0001.png vs first_0001.png | values: 160000 | differing: 13116 | largest difference: 0.43529412150382996
PIX comp_default_0001.png vs comp_second_0001.png | values: 160000 | differing: 13116 | largest difference: 0.43529412150382996
```

So with both view layers on, the one file held the same picture as a render of `ViewLayer` alone — with the compositor off, with Render Single Layer on, with the compositor on and the tree Blender 5.3 makes by default (below), and with a Render Layers node reading `ViewLayer`. With a Render Layers node reading `Second`, the file held a different picture. A render of `Second` alone, to compare against, could not be made: it crashed every time (below).

*Multi-Layer EXR, both layers on.* One file, `exr_0001.exr`, 28239 bytes. Its header, read with a short script, holds **two parts**, one per view layer:

```
EXR exr_0001.exr | part 0 | 4 channels: ViewLayer.Combined.A, ViewLayer.Combined.B, ViewLayer.Combined.G, ViewLayer.Combined.R
EXR exr_0001.exr | part 1 | 4 channels: Second.Combined.A, Second.Combined.B, Second.Combined.G, Second.Combined.R
```

*The compositor tree Blender 5.3 makes by default.* The operator that makes a new compositing node group (`bpy.ops.node.new_compositing_node_group()`) made a group holding only a Group Input joined to a Group Output — `COMPOSITOR TREE 'NodeTree' nodes: [('NodeGroupOutput', 'Group Output', None), ('NodeGroupInput', 'Group Input', None)] | links: [('Group Input', 'Image', 'Group Output', 'Image')]` — which was then put on the scene with `scene.compositing_node_group` (printing `DeprecationWarning: 'Scene.compositing_node_group' is expected to be removed in Blender 6.0`) and the compositor switched on.

*Only the second view layer on.* Run three times, on 2026-10-16 and twice more on 2026-10-17. Every time Blender wrote nothing and ended with exit status **-11**. The two later runs kept every line Blender printed, and both read, from the script's own line on:

```
VIEW LAYER 'ViewLayer' use=False | VIEW LAYER 'Second' use=True | single_layer=False | compositing=False | format=PNG
00:00.319  render           | Rendering single frame
Writing: /var/folders/n3/z3rfjt654rn1k1x6f_1gk4540000gn/T/blender.crash.txt
```

The crash report it wrote begins:

```
# Blender 5.3.0, Commit date: 2026-10-16 00:30, Hash 5d8151335b6b

# backtrace
…
5   Blender                             0x00000001057f2e80 _ZN7blender3deg30deg_update_eval_copy_datablockEPKNS0_9DepsgraphEPKNS0_6IDNodeE + 3356
…
9   Blender                             0x0000000105cd8d88 _ZN7blender13RE_RenderAnimEPNS_6RenderEPNS_4MainEPNS_5SceneEPNS_9ViewLayerEPNS_6ObjectEiii + 1572
10  Blender                             0x0000000104b0a418 _ZN7blenderL23arg_handle_render_frameEiPPKcPv + 968
```

*Stopped between the two view layers* (hand test 21, part c). The same scene at 8000 samples, both view layers on, frame 1. Uninterrupted, Blender printed `Start rendering: Scene, ViewLayer` at the start, `Fra: 1 | Mem: 2M | Sample 8000/8000` and `Start rendering: Scene, Second` about 12.1 seconds later, `Sample 8000/8000` again about 6.9 seconds after that, and only then `Saved: '…/e21c/full/vl_0001.png'` — one file, 25383 bytes. For the stop, a Python handler watched the progress text Blender hands its `render_stats` handler and, three seconds after that text first held `8000/8000` — the end of `ViewLayer` — sent a signal to Blender's own process, while `Second` was rendering. Four runs, each into an empty folder, listed just before the stop and after the process was gone:

- *PNG, interrupt* (`SIGINT`). Just before: `(nothing)`. After the signal, `Sent an internal break event. Press ^C again to kill Blender`, then about 3.9 seconds later `00:19.162  render           | Fra: 1 | Mem: 2M | Sample 8000/8000`, `Fra: 1 | Mem: 2M | Finished`, `Fra: 1 | Mem: 2M | Cancelled`, `Blender quit`; exit status 1. Afterwards: `<empty>`.
- *PNG, kill* (`SIGKILL`). Just before: `(nothing)`. Exit status -9. Afterwards: `<empty>`.
- *Multi-Layer EXR, interrupt.* The same lines as the PNG interrupt; exit status 1; afterwards `<empty>`.
- *Multi-Layer EXR, kill.* Exit status -9; afterwards `<empty>`.

**Outside the interface:** Differs in one documented way: Render Single Layer is ignored from the command line, so every view layer with Use for Rendering on is rendered there, even if the file was saved with Render Single Layer on. Nothing else is described as differing.

**Outside the interface — brought up to date by the hand tests of 2026-10-16 and 2026-10-17:** every observed result above was got from Blender started from a terminal with no window. There, Render Single Layer on made no difference to the file written (`single_0001.png` against `both_0001.png`, `differing: 0`), one file was written per frame, and the stop between view layers left nothing. What the window writes was not tested in 5.3.

**Operating system:** The sources do not say. The command-line argument list says `OPEN_EXR` and `OPEN_EXR_MULTILAYER` are among "Formats that can be compiled into Blender, not available on all systems" (https://docs.blender.org/manual/en/5.3/advanced/command_line/arguments.html — read 2026-10-11). It does not name the systems.

**Operating system — brought up to date by the hand tests of 2026-10-16:** observed on one operating system, macOS 26.6.2 on the Creator's Mac (Apple M1 Pro). No Linux or Windows machine was used for 5.3, so nothing above is settled for either in this version.

---

## 10. In what order does a render work through the frames it was given — a plain run, a run that skips frames at a fixed interval, and a list of separate frames — and what does an interruption leave behind in each case?

**Answer.**
- **Plain run:** from Start to End, End included, moving forward one frame after each frame rendered.
- **Run with a step:** also forward, advancing by Step after each frame rendered. **Inferred:** the frames are Start, Start+Step, Start+2×Step … The sources do not say whether End is rendered when the steps miss it.
- **List of separate frames** (`-f` with commas): the sources do not give the order.
- **What an interruption leaves:**
  - Plain run: for image files, the frames finished before the stop — documented when the computer goes down, **inferred** on a deliberate stop (see 3). For a single video file, not in the sources beyond the documented warning that the file "might become unplayable" (see 5 and 17).
  - Step: **inferred** for image files, the frames finished before the stop, from the stepped set. The manual's preview workflow keeps the stepped frames and fills in the rest later: render every Nth frame, then turn Overwrite off and render again with Step 1.
  - List: not in the sources.
  - In every case, what is left for the frame in hand is not in the sources (see 16).

**Label:** documented (plain run and step move forward from start to end; finished image frames in a plain run survive power loss). Inferred (the exact stepped frames; what an interruption leaves in a plain image run on a deliberate stop and in a stepped image run). Not in the sources (the order of a `-f` list; what an interruption leaves with a list, with a video file, or for the frame in hand).

**Sources:**
- "-a, --render-anim — Render frames from start to end (inclusive)." "-j, --frame-jump <frames> — Set number of frames to step forward after each rendered frame." "A comma separated list of frames can also be used (no spaces)." — https://docs.blender.org/manual/en/5.3/advanced/command_line/arguments.html — read 2026-10-11.
- "Step — Controls the number of frames to advance for each frame rendered, a value above 1 skips frames." — https://docs.blender.org/manual/en/5.3/render/output/properties/frame_range.html — read 2026-10-11.
- Scene `frame_step`: "Number of frames to skip forward while rendering/playing back each frame (in [0, 1048574], default 1)" — https://docs.blender.org/api/5.3/bpy.types.Scene.html — read 2026-10-11.
- "you can use the Frame Step option to render every N’th frame. Then disable Overwrite and re-render with Frame Step set to 1." — Manual, Rendering Animations, Animation Preview. https://docs.blender.org/manual/en/5.3/render/output/animation.html — read 2026-10-11.
- "the frames you have already rendered will still be in the output directory." — Manual, Rendering Animations, Hints. https://docs.blender.org/manual/en/5.3/render/output/animation.html — read 2026-10-11.

**Inference step (step, interruption):** "step forward after each rendered frame" means only the stepped frames are ever attempted. Frames are saved as they are rendered (animation.html). So a stop leaves the stepped frames finished so far. The Animation Preview workflow relies on those stepped frames staying on disk and being skipped later.

**Not covered:** the order of a `-f` comma list (as typed, sorted, or otherwise); whether repeated frames in a list are rendered twice; whether the order is the same through `bpy.ops.render.render`; the frame in hand.

**Where I looked:** Command Line Arguments page, Rendering From The Command Line page, Frame Range page, Rendering Animations page, `bpy.ops.render` and Scene API pages. Searched all 5.3 sources for order, in order, ascending, descending, reverse, backwards, sorted, comma separated, frame-jump, frame_step.

**Observed — hand tests 1, 2, 3, 4, 5 and 22, run by the chat on 2026-10-16.** Label: **observed**. All on the Creator's Mac (Apple M1 Pro, its own 14-core graphics through Metal, 16 GB of memory, macOS 26.6.2), in Blender 5.3.0 Beta (hash `5d8151335b6b`, built 2026-10-16), started from a terminal with no window, with a settings folder of its own and `--factory-startup`, on Blender's startup scene, PNG output into an empty folder. The exact commands are in `Audits/Blender Research/Frames/Hand tests — 5.3.md`.

*A plain run* (hand test 1; Workbench, 64 by 64, frames 1 to 5). Exit status 0, five files of 4074 bytes, each saved straight after its own frame:

```
00:00.595  render           | Rendering animation (frames 1..5)
00:00.595  render           | Rendering frame 1
00:00.596  render           | Start rendering: Scene, ViewLayer
00:00.596  render           | Engine: Workbench
00:01.819  render           | Saved: '…/e01/f_0001.png'
00:01.819  render           | Time: 00:01.22 (Saving: 00:00.07)
00:01.819  render           | Rendering frame 2
…
00:01.836  render           | Rendering frame 5
00:01.836  render           | Start rendering: Scene, ViewLayer
00:01.836  render           | Engine: Workbench
00:01.840  render           | Saved: '…/e01/f_0005.png'
00:01.840  render           | Time: 00:00.00 (Saving: 00:00.00)
```

The files' last-changed times rose from frame 1 to frame 5.

*A render started inside Blender's window* (hand test 37 a, 2026-10-17, written out under question 2): started with `bpy.ops.render.render('INVOKE_DEFAULT', animation=True)`, frames 1 to 6 were saved in order and the script listed them, by last-changed time, as `['o_0001.png', 'o_0002.png', 'o_0003.png', 'o_0004.png', 'o_0005.png', 'o_0006.png']`.

*A stepped run* (hand test 3, written out under question 8): start 1, end 10, step 3 gave 1, 4, 7, 10; start 3, end 12, step 4 gave 3, 7, 11 and not 12.

*A list of separate frames* (hand test 2, written out under question 7): saved in the order written — `-f 5,1,9` gave 5, 1, 9 and `-f 5,1,3..4,2` gave 5, 1, 3, 4, 2 — and `-f 2,2,3` rendered frame 2 twice. Each item was announced as `Rendering single frame`.

*The same list with other engines and on the graphics card* (hand test 4; 64 by 64). `-f 5,1,9` saved 5, then 1, then 9 with Cycles on the processor at 16 samples, with EEVEE, and with Cycles on the Mac's own graphics through Metal, for which Blender printed `DEVICE name='Apple M1 Pro (GPU - 14 cores)' type='METAL' use=True` and `name='Apple M1 Pro' type='CPU' use=False`. On the graphics card the first frame took 184 seconds before it was saved and the next two under a second each; nothing was printed about the wait.

*What an interruption leaves with a list and with a step* (hand test 5). Cycles on the processor, 1000 samples, 400 by 400, about 6.5 to 9 seconds a frame. The stop was sent by Blender's own process two seconds after the frame named began.

- `-f 5,1,3..4,2`, interrupted (`SIGINT`) two seconds into the third frame rendered, frame 3. Every line after the interrupt:

  ```
  Sent an internal break event. Press ^C again to kill Blender
  00:19.752  render           | Fra: 3 | Mem: 3M | Sample 1000/1000
  00:19.752  render           | Fra: 3 | Mem: 3M | Finished
  00:19.752  render           | Fra: 3 | Mem: 3M | Cancelled
  00:19.752  render           | Rendering single frame
  00:19.752  render           | Rendering frame 4
  00:19.752  render           | Start rendering: Scene, ViewLayer
  00:19.752  render           | Engine: Cycles
  00:19.753  render           | Fra: 4 | Mem: 0M | Initializing
  00:19.753  render           | Rendering single frame
  00:19.753  render           | Rendering frame 2
  00:19.753  render           | Start rendering: Scene, ViewLayer
  00:19.753  render           | Engine: Cycles
  00:19.754  render           | Fra: 2 | Mem: 0M | Initializing

  Blender quit
  ```

  Exit status 1. The folder held `f_0001.png` and `f_0005.png`, 92830 bytes each, both opening as 400 by 400 pictures, and nothing for frames 3, 4 or 2.
- The same list killed outright (`SIGKILL`) two seconds into frame 3: exit status -9, the same two files, whole, nothing for frame 3.
- `-s 1 -e 10 -j 3 -a`, interrupted two seconds into frame 7, the third frame of the run: `Sent an internal break event. Press ^C again to kill Blender`, then `Fra: 7 | Mem: 3M | Sample 1000/1000`, `Finished`, `Cancelled`, `Blender quit`, exit status 1. The folder held `f_0001.png` and `f_0004.png`, 92829 bytes each, opening as pictures, and nothing for frame 7.
- The same stepped run killed outright two seconds into frame 7: exit status -9, the same two files, nothing for frame 7.

*A plain run stopped part-way* is written out under questions 2, 3 and 16 (hand test 11).

*What Blender writes into a PNG, and when two renders of one frame are the same file* (hand test 22) — what the earlier versions' comparisons of renders turned on. Blender's startup scene at factory settings carries sixteen stamp settings:

```
STAMPS {'use_stamp_time': True, 'use_stamp_date': True, 'use_stamp_frame': True, 'use_stamp_frame_range': False, 'use_stamp_camera': True, 'use_stamp_lens': False, 'use_stamp_scene': True, 'use_stamp_note': False, 'use_stamp_marker': False, 'use_stamp_filename': True, 'use_stamp_sequencer_strip': False, 'use_stamp_render_time': True, 'use_stamp': False, 'use_stamp_labels': True, 'use_stamp_memory': False, 'use_stamp_hostname': False}
```

Frame 1 rendered twice by Cycles at 64 by 64 and 4 samples, at those settings, gave two PNGs of 3994 bytes with eleven text chunks each — `File`, `Date`, `Time`, `Frame`, `Camera`, `Scene`, `RenderTime` and four of Cycles' own (`cycles.ViewLayer.samples`, `…total_time`, `…render_time`, `…synchronization_time`). The two differed only in `Date` (`2026/10/16 23:23:35` against `23:23:36`); `cmp` reported `differ: char 635, line 5`; the picture data had the same fingerprint (`IDAT length 3010 md5 6eaecf0d0846ac61619b519e17035dcf` in both) and `pixcmp` found `differing: 0`. With all sixteen stamp settings off, the files carried the four Cycles chunks only, and the two fast renders were the same file byte for byte (`SAME FILE byte for byte`). At 256 by 256 and 400 samples with the settings off, the two files differed only in Cycles' own times (`total_time 00:01.00` against `00:00.98`) while the picture data was identical.

**Outside the interface:** `-a`, `-j` and the `-f` list are command-line features, so they work from outside. The interface offers the plain run and the stepped run (Frame Range panel), not a list.

**Outside the interface — brought up to date by the hand tests of 2026-10-16:** every observed result above was got from Blender started from a terminal with no window, except the one render started inside Blender's window on 2026-10-17 (hand test 37 a), which also wrote its frames in order.

**Operating system:** The sources do not say.

**Operating system — brought up to date by the hand tests of 2026-10-16:** observed on one operating system, macOS 26.6.2 on the Creator's Mac (Apple M1 Pro). No Linux or Windows machine was used for 5.3, so nothing above is settled for either in this version. Apple's own graphics, through Metal, gave the same list order as the processor; no graphics card of another make was used.

---

## 11. Can a render be told to stop once the frame it is working on is finished, rather than the moment it is asked?

**Answer.** Not in the sources. No setting, command-line argument, operator or Python feature in the 5.3 sources is described as stopping a render after the current frame. The stop controls the sources do name (Esc, the status-bar cancel button, and, indirectly, Control-C) are described only as cancelling or aborting the render.

**Label:** not in the sources (any way to stop after the current frame finishes). Documented (Esc, the status-bar cancel button, and the render handlers). Inferred (Control-C cancelling a normal render; see 2 and 15).

**What the sources do say nearby, quoted so nobody has to look again:**
- "As with a normal render, you can abort it with Esc." — https://docs.blender.org/manual/en/5.3/editors/3dview/viewport_render.html — read 2026-10-11.
- "The task can be aborted by clicking the cancel button" — https://docs.blender.org/manual/en/5.3/interface/window_system/status_bar.html — read 2026-10-11.
- Handlers `render_cancel` ("on canceling a render job"), `render_write` ("on writing a render frame (directly after the frame is written)"), `render_post` ("on render (after)"), `render_complete` ("on completion of render job") — https://docs.blender.org/api/5.3/bpy.app.handlers.html — read 2026-10-11. None is described as able to end the render job.

**Not covered:** everything about a "finish this frame, then stop" request.

**Where I looked:** searched all 5.3 manual, API, release-note and Cycles files for: stop after, after the current frame, finish the current, finish frame, frame_finish, graceful, when finished, once finished, cancel, abort, interrupt, Esc, Ctrl-C, Control-C, signal, SIGINT, SIGTERM, test_break, render_cancel, render_write, pause. Read the Command Line Arguments page in full (no such option). Read the RenderEngine API page (`test_break`, `end_result`, `render_frame_finish`; the last is described as finishing work after all view layers of a frame, not as a stop request). Read the Cycles Python files (`__init__.py`, `engine.py`, `operators.py`, `properties.py` around the sampling settings). The Cycles "Time Limit" setting ("Limit the render time (excluding synchronization time)", properties.py lines 568–576) limits the render time of each frame. It is not a request to stop the job, so I ruled it out.

**Observed — hand tests 11, 5 and 12, run by the chat on 2026-10-16.** Label: **observed**. On the Creator's Mac (Apple M1 Pro, its own 14-core graphics through Metal, 16 GB of memory, macOS 26.6.2), in Blender 5.3.0 Beta (hash `5d8151335b6b`, built 2026-10-16), started from a terminal with no window, with a settings folder of its own and `--factory-startup`; the exact commands are in `Audits/Blender Research/Frames/Hand tests — 5.3.md`.

- *What an interrupt does with the frame in hand* (hand tests 11 and 5, written out under questions 3, 10 and 16). Interrupted two seconds into a frame of about 6.5 seconds, Blender printed `Sent an internal break event. Press ^C again to kill Blender` at once, then, about four seconds later, `Fra: 3 | Mem: 3M | Sample 1000/1000`, `Fra: 3 | Mem: 3M | Finished`, `Fra: 3 | Mem: 3M | Cancelled` and `Blender quit`, exit status 1. No file was written for that frame, and with a `-f` list the frames after it were announced (`Rendering single frame`, `Rendering frame 4`, `… | Initializing`) but not rendered. A terminate or a kill ended the process at once and wrote nothing for the frame either.
- *Lowering the end frame from Blender's own "frame written" handler* (hand test 12). Workbench, 64 by 64, frames 1 to 8, a handler on `render_write` that sets the scene's end frame to 2 the moment frame 2 is written:

  ```
  00:00.350  render           | Rendering animation (frames 1..8)
  00:00.551  render           | Saved: '…/e12/f_0001.png'
  00:00.557  render           | Saved: '…/e12/f_0002.png'
  HANDLER: frame 2 has just been written; setting frame_end to 2
  HANDLER: frame_end now reads 2
  00:00.572  render           | Saved: '…/e12/f_0003.png'
  …
  00:00.596  render           | Saved: '…/e12/f_0008.png'
  ```

  All eight frames were written, exit status 0.
- *The command-line help.* `blender --factory-startup --help` printed 521 lines, exit status 0. The only lines holding "stop", "cancel", "after", "pause" or "finish" were:

  ```
  9:	and can be re-enabled by passing in '-setaudio Default' afterwards.
  32:	Set number of frames to step forward after each rendered frame.
  427:	Pass '--help' after the command to see its help text.
  ```
- *The Render menu in Blender's window* (hand test 36, 2026-10-17). Blender opened with its window, the Quick Setup panel closed by an Escape and the topbar's "Render" title clicked, both sent as Blender's own simulated events; no render was running. The photograph shows six items: `Render Image` (`F12`), `Render Animation` (`⌘ F12`), `Render Audio...`, `View Render` (`F11`), `View Animation` (`⌘ F11`) and a `Lock Interface` checkbox. None of them stops, pauses or finishes a render.
- *The status bar while a render runs in the window* (hand test 41, 2026-10-17, written out under question 2). It showed `Rendering...`, a progress bar and a cross. A simulated right click on the progress bar opened a menu headed `0%` holding only `Header` (`Show Header`, `Show Menus`, `Flip to Bottom`); a simulated click on the cross ended the render job within 0.344 seconds with nothing saved for the frame in hand; a simulated Escape did not stop the render at all. Nothing offered to stop after the current frame.

**Outside the interface:** The sources do not say. No command-line or Python means is documented.

**Outside the interface — brought up to date by the hand tests of 2026-10-16:** from Blender started from a terminal with no window, none was found: an interrupt cancelled the frame in hand rather than finishing it, lowering the end frame from a "frame written" handler did not end the render, and the command-line help offers no such option. **Brought up to date again on 2026-10-17:** inside Blender's window none was found either — not in the Render menu, not on the status bar's progress bar or its right-click menu (hand tests 36 and 41).

**Operating system:** The sources do not say.

**Operating system — brought up to date by the hand tests of 2026-10-16:** observed on one operating system, macOS 26.6.2 on the Creator's Mac (Apple M1 Pro). No Linux or Windows machine was used for 5.3, so nothing above is settled for either in this version.

---

## 12. When one render writes one video and separate image files over the same frame range, what does an interruption leave if all the images have been written but the video has not finished closing?

**Answer.** Not in the sources. The sources describe the parts separately. The compositor's File Output node writes images "during rendering". The main output can be a video. And a video "might become unplayable" if a problem occurs. They do not describe the case where every image is written but the video is not yet closed. **Inferred:** the separate images written by then are on disk, because the node writes them during the render. What the video holds at that moment is not in the sources.

**Label:** not in the sources (the video's state, and whether there is a distinct step of closing the video after the images are written). Inferred (the separate images being present on disk). Documented (that the File Output node writes images during rendering, the main output can be a video, and a video might become unplayable if a problem occurs).

**Sources:**
- "The File Output node writes one or more images to disk during rendering or compositing." Its Media Type offers "Image" and "Multi-Layer EXR" (no video). — https://docs.blender.org/manual/en/5.3/compositing/types/output/file_output.html — read 2026-10-11.
- Main output Media Type "Video: Encodes each frame into a video container." — https://docs.blender.org/manual/en/5.3/render/output/properties/output.html — read 2026-10-11.
- "If a problem occurs while rendering, the file might become unplayable" — Manual, Output properties, Encoding Tips. https://docs.blender.org/manual/en/5.3/render/output/properties/output.html — read 2026-10-11.

**Inference step (images present):** the node writes "during rendering", frame by frame, with "the frame number appended". So by the time the last frame's image is written, all of them have been written. Whether they survive the interruption follows the same reasoning as answer 3. No page says it for File Output node images specifically.

**Not covered:** whether the video is playable, how many frames it holds, whether it is removed. Also the exact order within a frame of the node's image write and the video's frame encode. The phrase "has not finished closing" names a step (closing the video file) that no source describes.

**Where I looked:** File Output node page, Output properties page (Media Type, Encoding), Video & Audio Formats page, compositing introduction, CompositorNodeOutputFile and FFmpegSettings API pages, 5.3 compositor release notes. Searched for File Output, file_output, video, movie, close, finalize, mux, container, unplayable, write.

**Observed — hand test 19, run by the chat on 2026-10-16: what one render can be set to write.** Label: **observed**. On the Creator's Mac (Apple M1 Pro, its own 14-core graphics through Metal, 16 GB of memory, macOS 26.6.2), in Blender 5.3.0 Beta (hash `5d8151335b6b`, built 2026-10-16), started from a terminal with no window, with `--factory-startup`; a Python script on Blender's startup scene (`video_settings.py`, written out in `Audits/Blender Research/Frames/Hand tests — 5.3.md`) printed:

```
MEDIA media_type present: True | is_enum_flag: False | items: ['IMAGE', 'MULTI_LAYER_IMAGE', 'VIDEO']
MEDIA file_format is_enum_flag: False
MEDIA IMAGE: file_format PNG | is_movie_format False
MEDIA set FFMPEG ok, file_format now: FFMPEG
MEDIA set PNG under VIDEO FAILED: TypeError bpy_struct: item.attr = val: enum "PNG" not found in ('FFMPEG')
MEDIA VIDEO: media_type VIDEO | file_format FFMPEG | is_movie_format True
NODE format media_type items: ['IMAGE', 'MULTI_LAYER_IMAGE', 'VIDEO'] | now: MULTI_LAYER_IMAGE
NODE set media_type VIDEO FAILED: TypeError bpy_struct: item.attr = val: enum "VIDEO" not found in ('IMAGE', 'MULTI_LAYER_IMAGE')
NODE set file_format FFMPEG FAILED: TypeError bpy_struct: item.attr = val: enum "FFMPEG" not found in ('OPEN_EXR_MULTILAYER')
SETTINGS IMAGE file_format: PNG | is_movie_format: False
SETTINGS IMAGE use_overwrite was True now False
SETTINGS IMAGE use_placeholder was False now True
SETTINGS VIDEO file_format: FFMPEG | is_movie_format: True
SETTINGS VIDEO use_overwrite was True now False
SETTINGS VIDEO use_placeholder was False now True
```

So the render's own output took one media type at a time, and with it set to Video the only file format it accepted was FFmpeg; a compositor File Output node did not accept Video or FFmpeg. The case the question describes — one video and separate images from one render — was therefore set up as the render's own output writing the video and a File Output node writing the images.

**Observed — hand test 15, run by the chat on 2026-10-16: a stop while the last frame's image is still being written.** Label: **observed**. Same Mac and Blender. The picture made of fine noise at 4096 by 4096 (Cycles, 1 sample), frames 1 and 2; the render's own output one MPEG-4 H.264 video in `vid/`; a File Output node writing 16-bit PNGs at compression 100% to `img/`, each taking about 12.6 seconds to write. Uninterrupted, every frame printed the node's save and then the video's:

```
00:18.630  render           | Saved: '…/vid_full/img/ImageImage0001.png'
00:18.743  render           | Video append frame 1
00:18.743  render           | Time: 00:18.39 (Saving: 00:00.11)
…
00:37.091  render           | Saved: '…/vid_full/img/ImageImage0002.png'
00:37.200  render           | Video append frame 2
```

and left two images of 105,359,152 bytes and `vid0001-0002.mp4` of 1,846,606 bytes, which Blender's own movie reader read as `frame_duration: 2 | size: (4096, 4096) | fps: 24.0`.

Then three runs each of an interrupt, a terminate and a kill, sent by Blender's own process 6.5 seconds after frame 2's `Finished`, while the node's frame-2 image was part-written (50,659,328 to 53,121,024 bytes). In every one of the nine, the folder listed just before the stop — after `Video append frame 1` had been printed — held the video at **48 bytes**.

- *Interrupt, three runs alike.* Blender printed `Sent an internal break event. Press ^C again to kill Blender`, went on writing, printed `Saved: '…/img/ImageImage0002.png'` about 6.3 seconds later and then `Blender quit` — **no `Video append frame 2` line** — exit status 1. Left: both images whole (105,359,152 bytes, opening as 4096 by 4096), and `vid0001-0002.mp4` at 994,360 bytes, read as `frame_duration: 1`.
- *Terminate, three runs alike.* Exit status -15, nothing printed after the signal. Left: the first image whole; the second at the size it had reached (51,822,592, 52,404,224 and 52,703,232 bytes), not opening (`size: (0, 0) … has_data: False`); the video at 48 bytes, read as `frame_duration: 0 | size: (256, 256) | fps: 0.0`.
- *Kill, three runs alike.* Exit status -9. Left: the first image whole; the second part-written (52,703,232, 52,502,528 and 53,121,024 bytes), not opening; the video at 48 bytes, `frame_duration: 0`.

**Observed — hand test 17, run by the chat on 2026-10-16: every image written and the video not yet closed.** Label: **observed**. Same Mac and Blender. Blender's startup scene, Cycles on the processor, 128 samples, 480 by 270, 24 frames a second; the render's own output one MPEG-4 H.264 video in `vid/`; a File Output node writing PNGs to `img/`. A Python handler on Blender's own "frame written" event (`render_write`) printed both folders each time it fired and, where asked, sent a signal to Blender's own process at the frame named. The exact commands are in `Audits/Blender Research/Frames/Hand tests — 5.3.md`.

*Uninterrupted, eight frames.* At every one of the eight "frame written" events the video stood at 48 bytes, the images growing by one each time — the last of them:

```
00:07.275  render           | Video append frame 8
SELFSIG render_write frame 8 | vid: [vid0001-0008.mp4=48] | img: [ImageImage0001.png=90152, … , ImageImage0008.png=90152]
SELFSIG render_complete fired | vid: [vid0001-0008.mp4=2861] | img: [ImageImage0001.png=90152, …]
```

Exit status 0; the video 2861 bytes, read as `frame_duration: 8 | size: (480, 270) | fps: 24.0`; eight images of 90152 bytes.

*The state the question describes, reached exactly:* the signal sent by the handler the moment frame 8's "frame written" event fired, all eight images on disk and the video at 48 bytes.

- *Kill.* `SELFSIG sending SIGKILL to own process 84552`, exit status -9. Left: eight images of 90152 bytes and `vid0001-0008.mp4  48 bytes`, read as `frame_duration: 0 | size: (256, 256) | fps: 0.0`. The eighth image compared with the uninterrupted run's eighth, value by value: `differing: 0 | largest difference: 0.0`.
- *Terminate.* Exit status -15. The same: eight whole images and a 48-byte video with no frames.
- *Interrupt.* `Sent an internal break event. Press ^C again to kill Blender`, then the cancel handler fired with the video already at its full size — `SELFSIG render_cancel fired at frame 1 | vid: [vid0001-0008.mp4=2861] | …` — and `Blender quit`, exit status 1. Left: eight whole images and the video at 2861 bytes, read as `frame_duration: 8`.

*Part-way, twenty frames, the signal at frame 10's "frame written" event.* An interrupt: the cancel handler fired with the video at 3164 bytes, a copy taken at that moment was identical byte for byte to the file left afterwards, ten images on disk, the video read as `frame_duration: 10`, exit status 1. A terminate (exit status -15) and a kill (-9): ten whole images and the video at 48 bytes, `frame_duration: 0`. `render_complete` did not fire in any stopped run; it fired in the uninterrupted ones.

*Every container, killed the moment the last frame was written* (hand test 18). The same set-up, 32 samples, four frames, the container changed and the codec left at H.264; one uninterrupted run and one killed at frame 4's "frame written" event per container. What Blender's reader gave:

| container | uninterrupted | killed at the last frame |
|---|---|---|
| MPEG4 | 2577 bytes, 4 frames | 48 bytes, 0 frames |
| MKV | 2344 bytes, 4 frames | 0 bytes, 0 frames |
| WEBM | 264 bytes, 0 frames; exit status 0 after 1.1 seconds, no kill sent | the same |
| AVI | 7564 bytes, 4 frames | 0 bytes, 0 frames |
| DV | no video file; exit status 0 after 1.1 seconds, no kill sent | the same |
| FLASH | 5911 bytes, 4 frames | 0 bytes, 0 frames |
| MPEG1 | 6144 bytes, 3 frames | 0 bytes, 0 frames |
| MPEG2 | 8192 bytes, 3 frames | 0 bytes, 0 frames |
| OGG | 6222 bytes, 4 frames | 0 bytes, 0 frames |
| QUICKTIME | 2511 bytes, 4 frames | 36 bytes, 0 frames |

WEBM and DV never reached a frame. Run again with every line kept, WEBM printed `00:00.333  reports          | ERROR Could not initialize streams, probably unsupported codec combination` and DV `00:00.331  reports          | ERROR Render width has to be 720 pixels for DV!` (the word `ERROR` printed in colour), each followed by `Blender quit` and exit status 0; WEBM left a 264-byte file, DV no file.

*Running the same range again over what the kill left* (hand test 17, Overwrite on, as Blender's default). Twenty frames, uninterrupted, over the folder holding ten images and the 48-byte video: the video became 4635 bytes, read as `frame_duration: 20`, and all twenty images were written, the first ten again — every one of them with a new last-changed time (`ImageImage0001.png 90153 bytes 1792182132.164676451` before, `1792182142.323423691` after).

**Outside the interface:** The sources do not say.

**Outside the interface — brought up to date by the hand tests of 2026-10-16:** every result above was got from Blender started from a terminal with no window; the File Output node wrote its images there, and the render's own video output behaved as written out above.

**Operating system:** The sources do not say.

**Operating system — brought up to date by the hand tests of 2026-10-16:** observed on one operating system, macOS 26.6.2 on the Creator's Mac (Apple M1 Pro). No Linux or Windows machine was used for 5.3, so nothing above is settled for either in this version.

---

## 13. When a render is stopped at the exact moment it is writing a frame's own image file, what is left where it writes?

**Answer.** Not in the sources. No page describes how an image file is written (in place, or to a temporary name then moved), or what a stop during the write leaves. **Inferred, Placeholders on only:** the frame's path held an empty placeholder before writing began (see 6). The sources do not say what replaces or remains of it after a stop mid-write.

**Label:** not in the sources (what a stop mid-write leaves, and how an image file is written). Inferred (that an empty placeholder sat at the path before writing began when Placeholders is on, from the documented Placeholders descriptions).

**Sources (for the inferred note):** "Create empty placeholder frames while rendering." — https://docs.blender.org/manual/en/5.3/render/output/properties/output.html. `use_placeholder`: "Create empty placeholder files while rendering frames (similar to Unix ‘touch’)" — https://docs.blender.org/api/5.3/bpy.types.RenderSettings.html. Both read 2026-10-11.

**Not covered:** everything about a stop mid-write: whether a part-written file, an empty file, the old file (when Overwrite is on and a file was already there), or nothing is left.

**Where I looked:** searched all 5.3 sources for write, written, writing, save, saving, temporary, temp, .tmp, rename, atomic, partial, half, incomplete, truncat, corrupt, placeholder, render_write. Read the Output properties page, Rendering Animations page, image formats page, and the RenderSettings, RenderEngine and bpy.app.handlers API pages. The only write-timing statement is the `render_write` handler being called "directly after the frame is written", which says nothing about a stop during the write.

**Observed — hand tests 13 and 14, run by the chat on 2026-10-16.** Label: **observed**. On the Creator's Mac (Apple M1 Pro, its own 14-core graphics through Metal, 16 GB of memory, macOS 26.6.2), in Blender 5.3.0 Beta (hash `5d8151335b6b`, built 2026-10-16), started from a terminal with no window, with a settings folder of its own and `--factory-startup`; the exact commands are in `Audits/Blender Research/Frames/Hand tests — 5.3.md`.

*How the file is written* (hand test 13, written out under question 6): straight to the frame's own final name, appearing at a few megabytes and growing in place; no other name ever appeared in the folder, listed every 2 milliseconds.

*A stop in the middle of the write* (hand test 14). To make the write long enough to stop part-way through, the picture was made of fine noise (a Noise Texture at scale 4000 and detail 15, in window coordinates, as the world's colour), Cycles on the processor, 1 sample, 8192 by 8192, one frame. Uninterrupted, the writes took: PNG 16-bit at compression 100%, 52 seconds (Blender's own line `Time: 01:13.44 (Saving: 00:52.94)`), finished file 414,531,599 bytes; JPEG at quality 100, about half a second, 59,641,010 bytes; TIFF 16-bit Deflate, about 6 seconds, 422,042,702 bytes; OpenEXR 32-bit ZIP, about 1.6 seconds, 799,674,444 bytes; Multi-Layer EXR 32-bit ZIP, about 1.8 seconds, 799,674,594 bytes. The stop was sent by Blender's own process a fixed time after the progress text it hands its `render_stats` handler said `Finished`, timed to land part-way through the write; the folder was listed just before the stop and every 20 milliseconds throughout.

- *PNG, interrupt* (`SIGINT`) with 170,430,464 bytes on disk. Blender printed `Sent an internal break event. Press ^C again to kill Blender`, went on writing, and thirty seconds later printed `01:13.254  render           | Saved: '…/png_int/f_0001.png'`; exit status 1. The file was the full 414,531,599 bytes and opened: `OPEN f_0001.png | 414531599 bytes | size: (8192, 8192) | channels: 4 | depth: 128 | has_data: True | pixel values read: 268435456`.
- *PNG, terminate* (`SIGTERM`) with 172,552,192 bytes on disk: no `Saved:` line, exit status -15, the file left at 172,552,192 bytes, and `OPEN f_0001.png | 172552192 bytes | size: (0, 0) | channels: 0 | depth: 0 | has_data: False | pixel values read: 0`.
- *PNG, kill* (`SIGKILL`) with 174,333,952 bytes on disk: no `Saved:` line, exit status -9, left at 174,333,952 bytes, not opening (`size: (0, 0) … has_data: False`).
- *JPEG, kill, eight runs.* In five the kill landed in the write: left at 17,940,480, 56,942,592, 47,960,064, 16,822,272 and 31,363,072 bytes, against 59,641,010 finished. **Every one of the five opened** in Blender's own loader as a full-size picture: for example `OPEN f_0001.jpg | 16822272 bytes | size: (8192, 8192) | channels: 4 | depth: 24 | has_data: True | pixel values read: 268435456`. Compared value by value with the finished picture, the 16,822,272-byte file differed in 142,424,522 of 268,435,456 values and the 56,942,592-byte one in 9,455,771. In the other three runs the write finished before the kill was sent: Blender printed `Saved:`, ended with exit status 0, and left the finished 59,641,010 bytes.
- *TIFF, kill, five runs*: left at 165,630,021, 232,964,427, 238,008,822, 211,089,302 and 239,689,628 bytes, against 422,042,702 finished. None opened; Blender printed, for example, `ERROR OpenImageIO read failed: TIFFReadRawStrip failed reading line y=0: unknown error` and `ERROR IMB_load_image_from_memory: unknown file-format (…/tiff_kill_1/f_0001.tif)`, and `OPEN f_0001.tif | 165630021 bytes | size: (0, 0) | channels: 0 | depth: 0 | has_data: False | pixel values read: 0`.
- *OpenEXR, kill*: left at 613,782,356 bytes of 799,674,444, not opening. Listed again sixty seconds after the process was gone, the folder held the same: `f_0001.exr  613782356 bytes`.
- *Multi-Layer EXR, kill*: left at 520,197,098 bytes of 799,674,594, not opening.
- *Over an older finished frame, Overwrite on* (PNG, kill). The folder held the finished 414,531,599-byte `f_0001.png` beforehand. The listing every 20 milliseconds showed it go to nothing as the new write began — `[   18.204] WATCH f_0001.png=0`, then `73728` and growing — and the kill left `f_0001.png  183136256 bytes`, not opening. The older finished picture was gone.
- *Placeholders on* (PNG, kill). `f_0001.png=0` appeared 0.74 seconds after the start; the write then grew the same file, and the kill left it at 182,013,952 bytes, not opening.

In none of the terminate or kill runs was a `Saved:` line printed for the frame.

*A compositor File Output node's own file, stopped in the middle* (hand test 15). The same noise picture at 4096 by 4096, frames 1 and 2, with a File Output node writing 16-bit PNGs at compression 100% to `img/` and the Output panel writing 16-bit PNGs at compression 100% to `out/`. Uninterrupted, each frame printed the node's save first and the Output panel's about 12.5 seconds later:

```
00:05.530  render           | Executing compositor
00:18.131  render           | Saved: '…/png_full/img/ImageImage0001.png'
00:30.731  render           | Saved: '…/png_full/out/frame_0001.png'
00:30.731  render           | Time: 00:30.40 (Saving: 00:12.60)
```

Killed 6.5 seconds after frame 2's `Finished`, while the node's frame-2 file was being written, the folder just before the kill held `out/frame_0001.png=105359345 bytes, img/ImageImage0001.png=105359152 bytes, img/ImageImage0002.png=52703232 bytes`; exit status -9. Frame 2 had **no Output-panel file at all**, and the node's frame-2 file was left at 52,703,232 bytes against 105,359,152 finished, not opening: `OPEN ImageImage0002.png | 52703232 bytes | size: (0, 0) | channels: 0 | depth: 0 | has_data: False | pixel values read: 0`.

*What the File Output node needs before it writes anything in 5.3* (hand test 15, part a). Blender's startup scene, Cycles, 16 samples, 256 by 256, one frame. The full set-up — a node group of kind `CompositorNodeTree` put on `scene.compositing_node_group`, a Render Layers node, a File Output node given an input with `file_output_items.new('RGBA', 'Image')`, a link from the Render Layers node's Image into that input, a Group Output fed the same image, `use_compositing` on — wrote `node/nodeImage0001.png` and `out/frame_0001.png`. Built again seven times with one piece left out:

| left out | did the node write a file |
|---|---|
| nothing | yes |
| the item made with `file_output_items.new` (the link going into the node's only socket, a virtual one named `''`) | no |
| the group's output socket and its Group Output node | yes |
| the link from the Render Layers node into the node's input | no |
| `use_compositing`, set to off | no |
| the Render Layers node, the node fed from a **group input** instead | **yes** |
| putting the group on `scene.compositing_node_group` at all | no |

Setting `scene.compositing_node_group` printed `DeprecationWarning: 'Scene.compositing_node_group' is expected to be removed in Blender 6.0` in every run that set it.

**Not run in 5.3, and why: the machine going down in the middle of a write.** Only the program was stopped; the Creator's own working Mac, the one machine this sitting had, was not reset or powered off. What a write the operating system had not yet put on the disk leaves is not settled for 5.3.

**Outside the interface:** The sources do not say.

**Outside the interface — brought up to date by the hand tests of 2026-10-16:** every observed result above was got from Blender started from a terminal with no window, stopped by signals sent from Blender's own process. A stop from inside the window at the moment of a write was not run.

**Operating system:** The sources do not say.

**Operating system — brought up to date by the hand tests of 2026-10-16:** observed on one operating system, macOS 26.6.2 on the Creator's Mac (Apple M1 Pro). No Linux or Windows machine was used for 5.3, so nothing above is settled for either in this version.

---

## 14. When two renders write into one folder and are started at the same instant, what does each of them produce, and what does the folder hold at the end?

**Answer.** Not in the sources for the case asked: two renders started at the same instant. The manual's "Primitive Render Farm" hint says several machines can share an animation by rendering into one shared folder with Overwrite off and Placeholders on. It does not say what happens when two of them reach the same frame at the same moment. **Inferred:** with those settings, each render skips frames that already have a file there, including an empty placeholder (see 4). So each produces a share of the frames and the folder ends with one file per frame. That inference rests on the hint and does not cover two renders checking the same frame at the same instant. The sources say nothing about two renders with the default settings (Overwrite on, Placeholders off).

**Label:** not in the sources (the same-instant case; the default-settings case). Inferred (how the documented render-farm setup divides the work). Documented (the Primitive Render Farm setup with Overwrite off and Placeholders on).

**Sources:**
- "Primitive Render Farm — An easy way to get multiple machines to share the rendering workload is to: Set up a shared directory over a network file system. Disable Overwrite, enable Placeholders in the Render Output panel. Start as many machines as you wish rendering to that directory." — https://docs.blender.org/manual/en/5.3/render/output/properties/output.html — read 2026-10-11.

**Inference step:** "share the rendering workload" can only happen if a machine leaves a frame alone once another has claimed it. Placeholders create an empty file "while rendering", and Overwrite off leaves "existing files" alone. So the placeholder is the claim.

**Not covered:** two renders checking the same frame at the same time; whether the check and the placeholder creation happen as one step; two renders on one computer rather than several; network file system behaviour; what happens with Overwrite on; what is left if one of them is stopped.

**Where I looked:** searched all 5.3 sources for render farm, shared directory, network file, multiple machines, several machines, same folder, at the same time, simultaneous, concurrent, lock, race, placeholder, overwrite. The Cycles "Sample Subset" passage on distributing "a single frame over multiple machines" (https://docs.blender.org/manual/en/5.3/render/cycles/render_settings/sampling.html — read 2026-10-11) is about splitting one frame's samples into separate files merged afterwards, not two renders writing one folder; ruled out. The Metadata page's mention of a render farm note field is about stamping text; ruled out.

**Observed — the parts of this question one Blender can reach, hand tests 7, 9 and 10, run by the chat on 2026-10-16.** Label: **observed**. On the Creator's Mac (Apple M1 Pro, its own 14-core graphics through Metal, 16 GB of memory, macOS 26.6.2), in Blender 5.3.0 Beta (hash `5d8151335b6b`, built 2026-10-16), started from a terminal with no window, with a settings folder of its own and `--factory-startup`; set-ups and output are written out under question 4.

- *A frame put in place, and a frame deleted, while the render ran* (hand test 7, written out under question 4): with Overwrite off, a frame put in place three seconds into the run was skipped when reached, and a frame deleted three seconds in was rendered when reached.
- *Placeholders* (hand tests 8 and 9, under questions 6 and 4): a zero-byte file appeared at each frame's name as that frame began, one frame at a time; one left by a render killed part-way was skipped by the next render and stayed at 0 bytes.
- *Which names in braces the output path can carry* (hand test 10). Setting the output path to `{name}_` for twenty-one names and asking `frame_path` for frame 1, Blender 5.3 filled in nine — `{blend_name}` gave `Unsaved_0001.png`, `{blend_dir}` gave `_0001.png`, `{blend_name_lib}` `Unsaved_0001.png`, `{blend_dir_lib}` `_0001.png`, `{fps}` `24.0_0001.png`, `{resolution_x}` `1920_0001.png`, `{resolution_y}` `1080_0001.png`, `{scene_name}` `Scene_0001.png`, `{camera_name}` `Camera_0001.png` — and refused the other twelve, `node_name`, `pid`, `process_id`, `hostname`, `machine`, `user`, `run`, `instance`, `uuid`, `random`, `time` and `date`, each with, for example:

  ```
  RuntimeError: Error: Parse errors in path '…/{pid}_':
  - Unknown variable referenced in template expression '{pid}'.
  ```

  Two separate runs printed the same 27 lines.

**Not run in 5.3, and why: two renders at once.** The question as asked — two renders writing into one folder, started at the same instant — needs two copies of Blender running together. The rules this sitting ran under allowed one Blender at a time on the Creator's Mac, which has 16 GB of memory, so no run of two renders was made in 5.3, on the shipped defaults or on the manual's recipe, into a local folder or a shared one. That is a limit of this sitting and not of the machine; it is on the list as not run, with the reason. Two computers sharing a folder over a network were not run either: there is one computer, and serving a folder from it over the network would mean changing its system settings.

**Outside the interface:** The hint is not tied to the interface. It relies on Overwrite and Placeholders, which have no command-line flags (see 4).

**Outside the interface — brought up to date by the hand tests of 2026-10-16:** the one-Blender parts observed above were got from Blender started from a terminal with no window, with the two settings set from Blender's own Python; they took effect there. Two renders at once were not run in 5.3.

**Operating system:** The sources do not say. The hint names "a network file system" without naming any system.

**Operating system — brought up to date by the hand tests of 2026-10-16:** observed on one operating system, macOS 26.6.2 on the Creator's Mac (Apple M1 Pro). No Linux or Windows machine was used for 5.3, so nothing above is settled for either in this version.

---

## 15. When a render working through a range of frames is interrupted, what does it do between the moment it is asked to stop and the moment it ends — how long does that take, what settles that, and what does it print in the meantime?

**Answer.** Not in the sources for what Blender does in that interval, how long it takes, or what it prints. The API gives one mechanism that bears on the timing. A render engine is expected to check regularly whether it should cancel. **Inferred:** how soon a render actually ends after a stop request depends on how often the engine checks. The sources give no figure for Cycles or any other engine. They name a `render_cancel` handler that runs "on canceling a render job", but do not say when within the shutdown it runs. They describe the render statistics Blender prints (current frame, current sample, render time, saving time) and a switch to suppress status printing. They say nothing about what is printed while a stop is under way.

**Label:** not in the sources (what Blender does in the stop interval, how long it takes, and what it prints). Inferred (that stop timing depends on how often the engine checks `test_break()`, and that Control-C cancels a normal command-line render). Documented (`test_break`, `end_result`, `render_cancel`, `render_stats`, `-q` / `--quiet`, `--log`, and the signal-handler note for Blender as a Python module). Observed (Cycles handing rendering to `_cycles.render` with no final-render cancel code in the supplied Python files).

**Sources:**
- `test_break()`: "Test if the render operation should been canceled, this is a fast call that should be used regularly for responsiveness" — https://docs.blender.org/api/5.3/bpy.types.RenderEngine.html — read 2026-10-11.
- `end_result(result, *, cancel=False, …)`: "cancel (bool) – Cancel, Don't mark tile as done, don't merge results unless forced". "do_merge_results (bool) – Merge Results, Merge results even if cancel=true" — https://docs.blender.org/api/5.3/bpy.types.RenderEngine.html — read 2026-10-11.
- `render_cancel`: "on canceling a render job." `render_stats`: "on printing render statistics. Accepts one argument: the render progress as a string containing current frame, current sample, render time and saving time." — https://docs.blender.org/api/5.3/bpy.app.handlers.html — read 2026-10-11.
- "-q, --quiet — Suppress status printing (warnings & errors are still printed)." and `--log "render,cycles"` among the logging examples — https://docs.blender.org/manual/en/5.3/advanced/command_line/arguments.html — read 2026-10-11.
- Cycles' own files: `engine.py` lines 89–92, `def render(engine, depsgraph):` … `_cycles.render(engine.session, depsgraph.as_pointer())`. The rendering is handed to the compiled `_cycles` module, which is not among the supplied files. A search of the supplied Cycles files for cancel, test_break, abort, interrupt and stop found no code about cancelling a final render. Only `operators.py` lines 88 and 97 matched: a `return {'CANCELLED'}` and the comment "# TODO: support cancel and progress reports." in a different operator. — **observed**, copy taken 2026-10-16.

**Inference step (timing):** the engine is told to "use regularly" a call that reports whether it "should been canceled". So it learns of the stop only at its next check, and what it does after that is up to the engine. The gap between checks is not given anywhere.

**Not covered:** the length of the interval; whether the frame in hand is finished, dropped or saved; whether the compositor or file writing runs after a stop; the exact text printed; whether the printing differs with `-b`.

**Where I looked:** RenderEngine API page in full, bpy.app.handlers API page, Command Line Arguments page (logging, `-q`, `--cycles-print-stats`, `--debug-cycles`), Rendering From The Command Line page, status bar page, Blender-as-a-module page, all Cycles Python files, and a search of the Cycles `source` folder. Searched all 5.3 sources for cancel, abort, interrupt, stop, test_break, render_cancel, render_stats, print, Fra:, Saved:, Ctrl-C, Control-C, signal.

**Observed — hand tests 23 to 35, run by the chat on 2026-10-16.** Label: **observed**. On the Creator's Mac (Apple M1 Pro, its own 14-core graphics through Metal, 16 GB of memory, macOS 26.6.2), in Blender 5.3.0 Beta (hash `5d8151335b6b`, built 2026-10-16), started from a terminal, with a settings folder of its own and `--factory-startup`; the exact commands are in `Audits/Blender Research/Frames/Hand tests — 5.3.md`.

*The scene.* Built by a script and saved once from 5.3 (`q15_5.3.blend`): Cycles on the processor, 600 samples, adaptive sampling and denoising off, 32 bounces, 320 by 180, threads fixed at 4, frames 1 to 20; a camera, a ground plane, a glass ball and a metal ball turning from frame 1 to frame 20, a dim world and one area light. Uninterrupted over frames 1 to 3 it printed `Time: 00:15.68`, `00:15.71` and `00:15.68`, frame 3 beginning 32.2 seconds after the start.

*How the stop was made.* Blender was given a real terminal (a pseudo-terminal) and the Control-C character was typed into it at a fixed time from the start — what a person pressing Control-C in a terminal does, not a signal sent from outside. "The gap" is the time from typing the Control-C to the process being gone, measured by the program that typed it. Every run rendered frames 1 to 5 to PNG unless said otherwise.

*Control-C eight seconds into the third frame* (hand test 23; typed at 40.2 seconds). Everything printed from the Control-C on:

```
^C
Sent an internal break event. Press ^C again to kill Blender
00:47.285  render           | Fra: 3 | Mem: 2M | Sample 600/600
00:47.285  render           | Fra: 3 | Mem: 2M | Finished
00:47.285  render           | Fra: 3 | Mem: 2M | Cancelled

Blender quit
```

The first two lines came at once; the rest together at the end of the gap, which was **7.57 seconds**, with nothing printed in between. Exit status 1. The folder held `f_0001.png` (51474 bytes) and `f_0002.png` (51569 bytes) and nothing for frame 3.

*Where in the frame the stop lands* (hand test 24). The same command, the Control-C typed at other moments:

- 1 second after the start, during set-up: gap **15.47 seconds**; the lines at the end were `Fra: 1 | Mem: 2M | Sample 600/600`, `Fra: 1 | Mem: 2M | Finished`, `Fra: 1 | Mem: 2M | Cancelled`, `Blender quit` — the whole of frame 1, and no file for it.
- 2 seconds into the third frame (34.2 s): gap **13.71 seconds**, ending `Fra: 3 | Mem: 2M | Sample 600/600`, `Finished`, `Cancelled`, `Blender quit`.
- about half way through the third frame (40.1 s): gap **8.45 seconds**, the same four lines.
- 2 seconds into the third frame at **2400 samples** (`--python-expr "import bpy; bpy.context.scene.cycles.samples = 2400"` added before `-s`). An uninterrupted run gave about 63 seconds a frame, frame 3 beginning 126.2 seconds after the start, so the Control-C was typed at 128.2 seconds. **That run started about five seconds later than usual** — Blender printed its version line 5.1 seconds after it was started, against about 0.4 seconds in every other run — so the Control-C landed in frame 2, about 2.5 seconds before its end. That is kept as what happened: after the Control-C, `Fra: 2 | Mem: 2M | Sample 2400/2400`, `Fra: 2 | Mem: 2M | Finished`, `Fra: 2 | Mem: 2M | Cancelled`, `Blender quit`; gap **2.50 seconds**; `f_0001.png` on disk and nothing for frame 2. It was run again with the interrupt sent by Blender's own process two seconds after its `render_pre` event fired for frame 3, rather than typed at a fixed time. Everything from there:

  ```
  STOPPER 126.230 s: folder just before the stop: f_0001.png=48050 bytes, f_0002.png=48364 bytes
  STOPPER sending SIGINT to own process 90751

  Sent an internal break event. Press ^C again to kill Blender
  02:31.069  render           | Fra: 3 | Remaining: 00:35.50 | Mem: 2M | Sample 1025/2400
  02:31.070  render           | Fra: 3 | Remaining: 00:35.50 | Mem: 2M | Cancelled

  Blender quit
  ```

  The last line before the stop was `Fra: 3 | Remaining: 01:03.15 | Mem: 2M | Sample 1/2400`. `Blender quit` came **24.51 seconds** after the interrupt; there was no `Finished` line; exit status 1; `f_0001.png` and `f_0002.png` on disk and nothing for frame 3.
- During **compositing**. A compositor was added to the scene by a script (`q15_blur.py`): the Render Layers image through eight Bokeh Blur nodes of size 150 in a chain to the Group Output. With the scene's compositor device left as factory settings set it — it read back as `GPU` — compositing took a fraction of a second, so the compositor was set to the processor (`compositor_device` `CPU`); an uninterrupted run of frame 1 then printed `Fra: 1 | Compositing` at 16.30 seconds and `Saved:` at 23.39 seconds, about 7.1 seconds of compositing. Frames 1 to 5, the Control-C typed at 19.8 seconds, 3.6 seconds into frame 1's compositing:

  ```
  00:15.790  render           | Fra: 1 | Mem: 2M | Finished
  00:15.790  render           | Executing compositor
  00:15.790  render           | Fra: 1 | Compositing
  ^C
  Sent an internal break event. Press ^C again to kill Blender

  Blender quit
  ```

  Gap **0.83 seconds**; no `Cancelled` line and no `Saved:` line; exit status 1; the folder empty.

*How many frames were still to come* (hand test 25). Typed at 40.2 seconds: with frames 1 to 5 the gap was **7.19 seconds**; with frames 1 to 100, **7.33 seconds**. In both, the lines after the Control-C named frame 3 only (`Fra: 3 | Mem: 2M | Sample 600/600`, `Finished`, `Cancelled`) and then `Blender quit`.

*A second Control-C* (hand test 26). Typed five seconds after the first (40.2 and 45.2 seconds):

```
^C
Sent an internal break event. Press ^C again to kill Blender
^C
Blender killed
Error: Not freed memory blocks: 30675, total unfreed memory 8.477425 MB
```

The process was gone 5.01 seconds after the first Control-C, at once after the second; exit status **2**; `f_0001.png` and `f_0002.png` on disk. Typed 0.3 seconds apart instead, the same three lines followed the second Control-C, the process was gone 0.31 seconds after the first, exit status 2, the same two files.

*No terminal at all* (hand test 27). The same command started detached, in a session of its own, with no terminal and its printing going into a file; one signal sent from outside at 40.2 seconds.

- *Terminate* (`kill`): gone 0.01 seconds after the signal, exit status -15. Nothing was added to the printing file after the signal; its last line is `00:31.639  render           | Fra: 3 | Remaining: 00:15.70 | Mem: 2M | Sample 1/600`. `f_0001.png` and `f_0002.png` on disk.
- *Kill* (`kill -9`): gone 0.01 seconds after, exit status -9. Nothing added; the last line `00:31.665  render           | Fra: 3 | Remaining: 00:15.61 | Mem: 2M | Sample 1/600`. The same two files.
- *Interrupt* (`kill -INT`): gone 7.68 seconds after, exit status 1. The file ends:

  ```
  00:31.807  render           | Fra: 3 | Remaining: 00:15.76 | Mem: 2M | Sample 1/600
  00:47.402  render           | Fra: 3 | Mem: 2M | Sample 600/600

  Sent an internal break event. Press ^C again to kill Blender
  00:47.402  render           | Fra: 3 | Mem: 2M | Finished
  00:47.402  render           | Fra: 3 | Mem: 2M | Cancelled

  Blender quit
  ```

  — in the file, the `Sent an internal break event` line stands after the `Sample 600/600` line.

*A stop landing between frames* (hand test 28). The interrupt was sent by Blender's own "frame written" handler the instant frame 2 was written (32.2 seconds in). Everything printed from there:

```
00:31.773  render           | Saved: '…/e28/f_0002.png'
SELFSIG sending SIGINT to own process 87938
Sent an internal break event. Press ^C again to kill Blender
SELFSIG returned from sending the signal
00:31.774  render           | Fra: 3 | Mem: 0M | Initializing
SELFSIG render_cancel fired at frame 1 | e28: [f_0001.png=51474, f_0002.png=51569]

Blender quit
```

`Blender quit` came 0.035 seconds after the interrupt; exit status 1; `f_0001.png` and `f_0002.png` on disk and nothing for frame 3.

*`-q`* (hand test 29; `-q` straight after `-b`, typed at 40.2 seconds). The progress lines were still printed while rendering. After the Control-C: `Sent an internal break event. Press ^C again to kill Blender`, then `Fra: 3 | Mem: 2M | Sample 600/600`, `Finished` and `Cancelled` — and **no `Blender quit` line**. Gap 8.84 seconds, exit status 1.

*The fullest logging* (hand test 30; `--log "*" --log-level debug` straight after `-b`, accepted; typed at 40.2 seconds). Between the Control-C and the end it added a block of `cycles` lines the ordinary run does not print:

```
00:48.136  cycles           | Rendered 599 samples in 15.7413 seconds (0.0262794 seconds per sample), occupancy: 1
00:48.136  cycles           | Measured path tracing occupancy: 1
00:48.136  cycles           | Write tile result.
00:48.136  cycles           | Write tile result via buffer write callback.
00:48.137  cycles           | Invoke buffer update callback.
00:48.137  cycles           | Average display update time: 3.12505e-06 seconds.
00:48.137  render           | Fra: 3 | Mem: 2M | Sample 600/600
00:48.137  cycles           | Rendering was canceled.
00:48.137  render           | Fra: 3 | Mem: 2M | Finished
00:48.137  render           | Fra: 3 | Mem: 2M | Cancelled
00:48.137  cycles           | Total render time: 15.7706
00:48.137  cycles           | Render time (without synchronization): 15.7685
```

then `Blender quit`. Gap 8.43 seconds, exit status 1.

*`--debug-memory`* (hand test 31; straight after `-b`). Blender printed `Switching to fully guarded memory allocator.` as it started, and ran more slowly (frame 2 began at 18.4 seconds instead of 16.0). Nothing else about memory, blocks or leaks was printed beyond the progress line `Updating Device | Writing constant memory`, which the ordinary run of hand test 23 prints too. After the Control-C, typed at 40.2 seconds: the same four lines as in hand test 23 (`Sent an internal break event…`, `Fra: 3 | Mem: 2M | Sample 600/600`, `Finished`, `Cancelled`) and `Blender quit`; gap 10.92 seconds, exit status 1.

*`--disable-abort-handler`* (hand test 31; straight after `-b`, typed at 40.2 seconds). The same lines after the Control-C as hand test 23, `Blender quit` included; gap 8.03 seconds, exit status 1.

*What the render handlers see* (hand test 32). A Python file attached a printing function, with a timestamp, to `render_init`, `render_pre`, `render_post`, `render_write`, `render_cancel`, `render_complete`, `render_stats` and `exit_pre`; all eight printed `attached`, so **`exit_pre` exists in 5.3**. After the Control-C, typed at 40.2 seconds, nothing fired until the end of the gap, and then, all within a thousandth of a second:

```
00:47.336  render           | Fra: 3 | Mem: 2M | Sample 600/600
HANDLER render_stats     fired at 46.993 s | string: 'Mem: 2M | Sample 600/600'
00:47.336  render           | Fra: 3 | Mem: 2M | Finished
HANDLER render_stats     fired at 46.994 s | string: 'Mem: 2M | Finished'
00:47.336  render           | Fra: 3 | Mem: 2M | Cancelled
HANDLER render_stats     fired at 46.994 s | string: 'Mem: 2M | Cancelled'
HANDLER render_cancel    fired at 46.994 s
HANDLER exit_pre         fired at 46.994 s
```

then `Blender quit`. `render_post`, `render_write` and `render_complete` did not fire for frame 3. Gap 7.62 seconds, exit status 1.

*Inside Blender's window* (hand tests 37, 38, 41 and 42, 2026-10-17, written out under questions 2 and 17). Every key and click was a simulated event, not a person's hand. A simulated Escape did not stop the render, in four runs. A simulated click on the status bar's cross, two seconds into frame 3, ended the render job — `bpy.app.is_job_running('RENDER')`, asked every tenth of a second, first answered False **0.344 seconds** after the click for an image sequence and **1.739 seconds** after it for a video — with no further `Saved:` or `Video append` line. The output kept left out Blender's progress lines (those beginning `Fra:`); of the other lines, none printed after the click was about the stop. A photograph taken after the click shows the status bar reading `Canceling...`.

*The graphics card against the processor* (hand test 33). The same command with `-- --cycles-device CPU` added at the very end, then with `-- --cycles-device METAL`, then with `-- --cycles-device METAL` and the samples raised to 20000 (`--python-expr "import bpy; bpy.context.scene.cycles.samples = 20000"` before `-s`); the Control-C typed at 40.2 seconds in all three.

- *On the processor, 600 samples:* frames 1 and 2 saved at 15.99 and 31.52 seconds by Blender's clock; after the Control-C, `Sent an internal break event. Press ^C again to kill Blender`, then together at the end `Fra: 3 | Mem: 2M | Sample 600/600`, `Fra: 3 | Mem: 2M | Finished`, `Fra: 3 | Mem: 2M | Cancelled` and `Blender quit`. Gap **7.50 seconds**, exit status 1, `f_0001.png` (51474 bytes) and `f_0002.png` (51569 bytes) on disk.
- *With `METAL`, 600 samples:* each frame took about 3.5 seconds, and all five were saved before the Control-C was due — the last `00:18.665  render           | Saved: '…/e33/gpu600/f_0005.png'` — exit status 0, five files. The program that types the Control-C recorded `ACTION CTRLC at 40.2 s skipped: the process had already ended`. So at 600 samples on this machine no stop was made on the card.
- *With `METAL`, 20000 samples:* frame 1 was still rendering at 40.2 seconds. The last line before the Control-C was `00:35.728  render           | Fra: 1 | Remaining: 02:03.48 | Mem: 1370M | Sample 4431/20000 (Using optimized kernels)`. After it:

  ```
  ^C
  Sent an internal break event. Press ^C again to kill Blender
  01:02.955  render           | Fra: 1 | Remaining: 01:23.92 | Mem: 1370M | Sample 8527/20000 (Using optimized kernels)
  01:02.955  render           | Fra: 1 | Remaining: 01:23.92 | Mem: 1370M | Cancelled

  Blender quit
  ```

  No `Finished` line. Gap **23.28 seconds**, exit status 1, nothing on disk.

*Other engines* (hand test 34). `blender --factory-startup -E help` printed:

```
Blender Engine Listing:
	BLENDER_EEVEE
	BLENDER_WORKBENCH
	CYCLES
```

Blender's startup scene at 1920 by 1080, frames 1 to 500, PNG, the Control-C typed 20 seconds in:

- *EEVEE:* the last line before the Control-C was `00:19.273  render           | Fra: 22 | Rendering 1 / 64 samples`. After it, `Sent an internal break event. Press ^C again to kill Blender`, then `Fra: 22 | Rendering 25 / 64 samples`, `Fra: 22 | Rendering 50 / 64 samples`, `Fra: 22 | Rendering 64 / 64 samples` and `Blender quit`. Gap **0.55 seconds**, exit status 1; `f_0001.png` to `f_0021.png` on disk and nothing for frame 22.
- *Workbench:* the Control-C came while frame 169 was being started (`Rendering frame 169`, `Start rendering: Scene, ViewLayer`, `Engine: Workbench`); after it, `Sent an internal break event. Press ^C again to kill Blender` and `Blender quit`. Gap **0.05 seconds**, exit status 1; `f_0001.png` to `f_0168.png` on disk and nothing for frame 169.

*One video file instead of separate images* (hand test 35). The same command with the output set to an FFmpeg MPEG-4 H.264 video (`--python-expr "import bpy; r=bpy.context.scene.render; r.image_settings.media_type='VIDEO'; r.image_settings.file_format='FFMPEG'; r.ffmpeg.format='MPEG4'; r.ffmpeg.codec='H264'"`, output `…/e35/movie_`). Before the stop it printed `00:15.880  render           | Video append frame 1` and `00:31.447  render           | Video append frame 2`. After the Control-C, typed at 40.2 seconds, the same lines as hand test 23 — `Sent an internal break event…`, `Fra: 3 | Mem: 2M | Sample 600/600`, `Finished`, `Cancelled`, `Blender quit` — and no line naming the video file or its closing. Gap **7.54 seconds**, exit status 1. Left: `movie_0001-0005.mp4  4954 bytes`, which Blender's own movie reader read as `frame_duration: 2 | size: (320, 180) | fps: 24.0`.

**Outside the interface:** The sources do not say how it differs. One related, documented point: when Blender is used as a Python module, "Blender’s typical signal handlers are not initialized, so there is no special handling for Control-C to cancel a render" (https://docs.blender.org/api/5.3/info_advanced_blender_as_bpy.html — read 2026-10-11). **Inferred** from that: in the normal program, Control-C is handled specially to cancel a render, while in the module it is not. How long that cancel takes is not stated.

**Outside the interface — brought up to date by the hand tests of 2026-10-16 and 2026-10-17:** every observed result above was got from Blender started from a terminal with no window (`-b`): with a real terminal and the Control-C typed into it (hand tests 23 to 26 and 29 to 35), with no terminal at all and a signal sent from outside (hand test 27), and with the interrupt sent by Blender's own process (hand tests 24 d and 28). There, Control-C did cancel the render, as the answer above infers. **Inside Blender's window** (2026-10-17), a simulated click on the status bar's cancel cross ended the render job within 0.344 seconds for images and 1.739 seconds for a video; a simulated Escape did not stop it. A person's own key or click was not used in 5.3, so whether a person's Escape stops it is left open.

**Operating system:** The sources do not say.

**Operating system — brought up to date by the hand tests of 2026-10-16 and 2026-10-17:** observed on one operating system, macOS 26.6.2 on the Creator's Mac (Apple M1 Pro), on its processor and on its own graphics through Metal. No Linux or Windows machine, and no graphics card of another make, was used for 5.3, so nothing above is settled for those in this version.

---

## 16. When a render working through an animation is stopped at no particular moment, what is in the place it writes to for the frame it had not finished?

**Answer.** Not in the sources. **Inferred, Placeholders on only:** an empty placeholder file was created for that frame when it started rendering. The sources do not say whether Blender removes it when the render is stopped. With Placeholders off, the sources do not say whether anything is there.

**Label:** not in the sources (what is left for an unfinished frame on a stop). Inferred (that an empty placeholder file was created when the frame started rendering with Placeholders on, from the documented Placeholders descriptions).

**Sources (for the inferred note):**
- "Create empty placeholder frames while rendering." — https://docs.blender.org/manual/en/5.3/render/output/properties/output.html — read 2026-10-11.
- `use_placeholder`: "Create empty placeholder files while rendering frames (similar to Unix ‘touch’) (default False)" — https://docs.blender.org/api/5.3/bpy.types.RenderSettings.html — read 2026-10-11.

**Inference step:** "while rendering frames" plus "empty" means the file is there, empty, while the frame is being worked on. Nothing describes a clean-up step.

**Not covered:** whether a placeholder is removed on a stop; whether anything is written for an unfinished frame without Placeholders; whether an older file at that name (Overwrite on) is kept or replaced; the video case (see 17).

**Where I looked:** the same searches as 6, 13 and 15, plus: remove, delete, clean, cleanup, leftover, unfinished, in progress.

**Observed — hand tests 11, 5 and 9, run by the chat on 2026-10-16.** Label: **observed**. On the Creator's Mac (Apple M1 Pro, its own 14-core graphics through Metal, 16 GB of memory, macOS 26.6.2), in Blender 5.3.0 Beta (hash `5d8151335b6b`, built 2026-10-16), started from a terminal with no window, with a settings folder of its own and `--factory-startup`. Blender's startup scene, Cycles on the processor, 1000 samples, 400 by 400, PNG, frames 1 to 5, about 6.5 to 10 seconds a frame. Each render was stopped about two seconds into a frame, while that frame was still sampling; the set-up and the lines printed are under question 3, and the exact commands are in `Audits/Blender Research/Frames/Hand tests — 5.3.md`. What was at the frame's own name afterwards:

| Overwrite | Placeholders | at the frame's name before the render | stop | at the frame's name just before the stop | afterwards |
|---|---|---|---|---|---|
| on | off | nothing | interrupt (three runs) | nothing | **nothing** |
| on | off | nothing | terminate | nothing | **nothing** |
| on | off | nothing | kill | nothing | **nothing** |
| off | on | nothing | interrupt | `f_0003.png  0 bytes` | **nothing** — the zero-byte file was gone |
| off | on | nothing | terminate | `f_0003.png  0 bytes` | **`f_0003.png  0 bytes`**, which Blender's loader did not open (`size: (0, 0) … has_data: False`) |
| off | on | nothing | kill | `f_0003.png=0 bytes` | **`f_0003.png  0 bytes`**, not opening |
| on | off | an older complete frame 3 | interrupt | the older file | **the older file**, same size and last-changed time, picture identical (`differing: 0`) |
| on | off | an older complete frame 3 | kill | the older file | **the older file**, unchanged |
| on | on | an older complete frame 3 | interrupt | the older file | **the older file**, unchanged |
| on | on | an older complete frame 3 | kill | the older file | **the older file**, unchanged |
| off | on | nothing | **in Blender's window**: a simulated click on the status bar's cross (hand test 41 b, under question 2) | three files in the folder, frames 1 and 2 saved | **nothing** at frame 3's name; two files left |
| on | off | nothing | **in Blender's window**: a simulated Escape (hand test 37 b, under question 2) | — | the render did not stop; frame 3 was finished and saved |

The last two rows were got on 2026-10-17 in Blender's own window, with simulated events rather than a person's hand, and are written out under question 2 (hand tests 37 b and 41 b).

Two notes on how these were got. The first five rows were stopped by the harness 16.0 seconds after the start; the Placeholders kill and the four older-file rows were stopped by Blender's own process two seconds after its `render_pre` event fired for frame 3, because on those runs the Mac was rendering more slowly and a stop at a fixed time from the start landed in frame 2. **The fixed-time runs that landed in frame 2 are kept too**: with Placeholders on, a kill in frame 2 left `f_0002.png  0 bytes`; with an older frame 2 in place and Overwrite on, an interrupt and a kill, with Placeholders off and on, each left that older frame 2 untouched — the same last-changed time as the copy put there, and `differing: 0` against the original.

The interrupted Placeholders run printed, from the interrupt on: `Sent an internal break event. Press ^C again to kill Blender`, `Fra: 3 | Mem: 3M | Sample 1000/1000`, `Fra: 3 | Mem: 3M | Finished`, `Fra: 3 | Mem: 3M | Cancelled`, `Blender quit`; exit status 1, 5.10 seconds after the interrupt.

The same held for a list and a stepped run (hand test 5, under question 10): an interrupt or a kill two seconds into the frame in hand left nothing at that frame's name. And a zero-byte file left by a kill is skipped by a later render with Overwrite off and stays at 0 bytes (hand test 9, under question 4).

**Outside the interface:** The sources do not say.

**Outside the interface — brought up to date by the hand tests of 2026-10-16:** every observed result above was got from Blender started from a terminal with no window, stopped by signals, except the last two rows of the table, got in Blender's window on 2026-10-17: there a simulated click on the status bar's cross left nothing at the frame in hand's name, with Placeholders on.

**Operating system:** The sources do not say.

**Operating system — brought up to date by the hand tests of 2026-10-16:** observed on one operating system, macOS 26.6.2 on the Creator's Mac (Apple M1 Pro). No Linux or Windows machine was used for 5.3, so nothing above is settled for either in this version.

---

## 17. When a render writing an animation as a single video file is stopped before the end, what does each of the ways a render can be stopped leave of that file?

**Answer.** Not in the sources for any single way of stopping. The ways the sources name are:
- **Esc** (documented)
- the **status-bar cancel button** (documented)
- **Control-C** at a command line (only indirectly; inferred, see 15)
- the **computer turning off** (documented as a case to guard against)

For video, the only statement covers all of them together: "If a problem occurs while rendering, the file might become unplayable". It does not separate the methods, does not separate a deliberate cancel from a crash or power loss, and says only "might".

**Label:** not in the sources, one way at a time. Documented: the general warning and the stop methods. Inferred: Control-C as a stop method.

**Sources:**
- "As with a normal render, you can abort it with Esc." — https://docs.blender.org/manual/en/5.3/editors/3dview/viewport_render.html — read 2026-10-11.
- "The task can be aborted by clicking the cancel button" — https://docs.blender.org/manual/en/5.3/interface/window_system/status_bar.html — read 2026-10-11. Its picture `_images/interface_window-system_status-bar_ui-center.png` shows a running "Sequence Render" at 94% with a cross button beside it (documented).
- "Blender’s typical signal handlers are not initialized, so there is no special handling for Control-C to cancel a render" (about Blender as a Python module) — https://docs.blender.org/api/5.3/info_advanced_blender_as_bpy.html — read 2026-10-11.
- "Your computer accidentally turns off in the middle of rendering your movie!" and, for the Direct Approach, "Stable power supply." — https://docs.blender.org/manual/en/5.3/render/output/animation.html — read 2026-10-11.
- "If a problem occurs while rendering, the file might become unplayable and you will have to re-render all frames from the beginning." — https://docs.blender.org/manual/en/5.3/render/output/properties/output.html — read 2026-10-11.

**Not covered:** for each method, whether the file is kept, playable, or holds the frames rendered so far; whether Autosplit files already closed are affected (see 5); what a forced end of the process (not Control-C) leaves.

**Where I looked:** the searches from 5 and 15, plus: Esc, cancel button, abort, kill, force quit, End Task, power, turns off, crash, signal. Read the Video & Audio Formats page and the FFmpegSettings API page in full; neither covers stopping.

**Observed — hand tests 16, 15 and 17, run by the chat on 2026-10-16, and 20, 38 and 42, run on 2026-10-17: the ways a render started from a terminal can be stopped, and, in the last four rows, some ways inside Blender's window.** Label: **observed**. On the Creator's Mac (Apple M1 Pro, its own 14-core graphics through Metal, 16 GB of memory, macOS 26.6.2), in Blender 5.3.0 Beta (hash `5d8151335b6b`, built 2026-10-16), started from a terminal with no window, with a settings folder of its own and `--factory-startup`; set-ups, commands and the lines printed are under questions 5 and 12 and in `Audits/Blender Research/Frames/Hand tests — 5.3.md`. Every video was MPEG-4 H.264, read back with Blender's own movie reader.

| how it was stopped | where | what the video file was afterwards |
|---|---|---|
| interrupt (`SIGINT`, what Control-C sends), during frame 4 of 20 | hand test 16 a | 2427 bytes, 3 frames — the frames appended before the stop |
| terminate (`SIGTERM`), during frame 4 of 20 | hand test 16 b | 48 bytes, no frames |
| kill (`SIGKILL`), during frame 4 of 20 | hand test 16 c | 48 bytes, no frames |
| interrupt with Autosplit on, as frame 3 of 20 was ending | hand test 16 d | `movie__0000001-0020.mp4`, 2288 bytes, 2 frames |
| interrupt while the last frame's separate image was being written (three runs) | hand test 15 | 994,360 bytes, 1 frame — the last frame was not appended |
| terminate, the same moment (three runs) | hand test 15 | 48 bytes, no frames |
| kill, the same moment (three runs) | hand test 15 | 48 bytes, no frames |
| interrupt the moment the last frame was written (8 of 8) | hand test 17 | 2861 bytes, 8 frames |
| terminate, the same moment | hand test 17 | 48 bytes, no frames |
| kill, the same moment | hand test 17 | 48 bytes, no frames |
| interrupt the moment frame 10 of 20 was written | hand test 17 | 3164 bytes, 10 frames |
| terminate and kill, the same moment | hand test 17 | 48 bytes, no frames |
| interrupt three frames after Autosplit Output started a second file (Matroska, FFV1) | hand test 20 | two files: the first 2,024,699,916 bytes, 125 frames; the second 48,672,835 bytes, 3 frames |
| **in Blender's window**: a simulated Escape, two seconds into frame 3 of 20 | hand test 38 | the render did not stop; see below |
| **in Blender's window**: Blender's own quit (`bpy.ops.wm.quit_blender()`), called by the script 45 seconds after the render started, during frame 7 | hand test 38 | 2583 bytes, 6 frames |
| **in Blender's window**: a simulated click on the status bar's cross, two seconds into frame 3 of 20 | hand test 42 a | 2288 bytes, 2 frames |
| **in Blender's window**: a kill (`SIGKILL`) sent from the terminal, during frame 3 of 20 | hand test 42 b | 48 bytes, no frames |

*Inside Blender's window* (hand tests 38 and 42, 2026-10-17). The set-up of hand test 37, under question 2 — Blender with its window, every key and click a simulated event, the render started with `bpy.ops.render.render('INVOKE_DEFAULT', animation=True)` — with the output set to one MPEG-4 H.264 video, `o_0001-0020.mp4`, frames 1 to 20, about 7.5 seconds a frame. Blender printed `Video append frame 1`, `2`, … as it went.

- *A simulated Escape* into both windows, with the pointer at each window's middle, 18 seconds after the render started: the render went on (`Video append frame 3` to `Video append frame 6` followed). The script's own quit, 45 seconds in, found `o_0001-0020.mp4  48 bytes`; after `Blender quit` (exit status 0) the file was 2583 bytes, read as `frame_duration: 6 | size: (400, 400) | fps: 24.0`.
- *A simulated click on the cross*, 18 seconds in: `CLICK   18.004 LEFTMOUSE press and release at (1736, 22)`, then `POLL   19.744 job running: False | files: 1 | 1.739 s after the Escape` (timed from the click). No more frames were appended. Left: 2288 bytes, `frame_duration: 2`.
- *A kill sent from the terminal* 22 seconds after Blender started (about 19 seconds into the render), standing in for Force Quit, which was not used — it is a macOS menu, reached by a person's hand or by a program driving the system, and the rules the hand tests ran under allowed Blender alone to be run: exit status -9, gone at once; left `o_0001-0020.mp4  48 bytes`, `frame_duration: 0 | size: (256, 256) | fps: 0.0`.

The file was named for the whole frame range. Wherever it was looked at while the render ran — from the first second in hand test 16, at every "frame written" event in hand test 17, just before the stop in hand test 15 — it was 48 bytes, frames already appended or not. Of the runs started from a terminal with no window and stopped by a signal, only those stopped by an interrupt left it larger, and it grew as the process ended; uninterrupted runs grew it at the end of the render. In Blender's window, the click on the cross and Blender's own quit left it larger; the kill did not.

**Outside the interface:** Esc and the cancel button are interface controls. Control-C is the only command-line stop the sources touch, and only indirectly. What any of them leaves of the video is not stated.

**Outside the interface — brought up to date by the hand tests of 2026-10-16 and 2026-10-17:** every observed result above was got from Blender started from a terminal with no window, except the last four rows of the table, got in Blender's window: there a simulated click on the status bar's cancel cross left the frames appended so far, Blender's own quit left them too, a kill left 48 bytes, and a simulated Escape did not stop the render. A person's own key or click was not used in 5.3, so whether a person's Escape stops it is left open.

**Operating system:** The sources do not say. They name no difference between systems in how a render is stopped or what a stopped video file holds.

**Operating system — brought up to date by the hand tests of 2026-10-16:** observed on one operating system, macOS 26.6.2 on the Creator's Mac (Apple M1 Pro), and read back with Blender's own movie reader only — the rules the hand tests ran under allowed no other program to be run, so whether these files play elsewhere is not settled. No Linux or Windows machine was used for 5.3, so nothing above is settled for either in this version.

---

## Things a person with Blender 5.3 open could settle in a few minutes

*Pruned 2026-10-17 by the hand tests for this version, `Audits/Blender Research/Frames/Hand tests — 5.3.md`. The eleven items that stood here were run on 2026-10-16 and 2026-10-17 on the Creator's Mac, from a terminal and in Blender's own window, and their results are written into the answers above as observed findings: item 1 as hand tests 6, 7 and 44, item 2 as 11, 37 and 41, item 3 as 11 and 23 to 35, item 4 as 2 and 5, item 5 as 3 and 5, item 6 as 16, 38 and 42, item 7 as 21, item 8 as 17 and 18, item 10 as 12, 36 and 41, and item 11 as 13 and 14. Item 9 could not be run. What is still open is below, with the reason.*

1. **Two renders into one folder (question 14)** — the researcher's item 9. The rules the hand tests ran under allowed one Blender at a time on the one Mac. Overwrite off and Placeholders on, frames 1–20: start `blender -b test.blend -a` twice at the same moment into one empty folder; compare the two printed frame lists, check whether any frame was rendered by both, and list the folder with sizes. Repeat with Overwrite on and Placeholders off.
2. **A person's own Escape, click and pointer in Blender's window (questions 2, 3, 7, 8, 11, 15, 16, 17).** Every key and click in the window in 5.3 was a simulated event sent from inside Blender. A simulated Escape did not stop a render; whether a person's Escape does is not shown. Also not shown: the tooltip of the View menu's `Render Playblast on Keyframes` and of the status bar's cross, which did not appear for the simulated pointer. With a render running from Render ▸ Render Animation, press Esc in the render window during frame 3 and list the output folder; rest the pointer on those two controls and read the tooltips.
3. **What a stopped video plays as in a program other than Blender (questions 5, 17)** — the player half of the researcher's item 6. The rules the hand tests ran under allowed Blender alone to be run, so every video was read back with Blender's own movie reader. Open each stopped `.mp4` in a video player and note whether it plays and how many frames it shows.
4. **Force Quit from the Apple menu, and End Task on Windows, on a video render (question 17).** A kill sent from the terminal stood in for Force Quit.
5. **The whole machine going down (questions 1, 3, 13).** Only the program was ever stopped; the Creator's own working Mac was not powered off.
6. **Every other operating system and graphics card.** Everything above is macOS 26.6.2 on an Apple M1 Pro, on its processor and on its own graphics through Metal.

---

2026-10-16 — Claude Opus 5.5 (Claude Code) — Frames, Blender 5.3

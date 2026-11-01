# Frames — Architecture questions, second pass (Blender 5.2)

**Sources consulted, and the date.** Blender 5.2 manual (`Blender Sources/manual_5.2`, including its `_images` folder), Blender 5.2 Python API documentation (`Blender Sources/api_5.2`), Blender 5.2 release notes (`Blender Sources/release_notes_5.2`), and Cycles' own files (`Blender Sources/cycles_5.2`). All four were taken on **2026-07-30**, and that is the date read for everything quoted below.

I read only the question sheet and files inside Blender Sources/. I did not open any other part of this project.

Answering question **15** only. No helper agents were used; every search and every file below was opened by me.

---

## 15. When a render working through a range of frames is interrupted, what does it do between the moment it is asked to stop and the moment it ends — how long does that take, what settles that, and what does it print in the meantime?

**LABEL: not in the sources.**

None of the four sources describes what a running frame-range render does after it has been asked to stop. Nothing states how long the gap between the request and the end is, nothing names what settles that length, and nothing says what is written to screen or terminal during that gap.

What the sources do carry is the machinery either side of that gap: the ways a stop can be asked for, the fact that the stop is *polled* rather than delivered, a named point that a render job passes through when it is cancelled, and what a render prints while it is running normally. Those are set out below, each with its own label, because they are adjacent to the question rather than answers to it.

### What the sources do say, and how far it goes

**(a) A stop can be asked for, in the interface, and the request is polled — not immediate.**

The strongest thing in any source is the description of the call a render engine is expected to make while it renders:

> "Test if the render operation should been canceled, this is a fast call that should be used regularly for responsiveness"

— Python API documentation, `RenderEngine.test_break()`, https://docs.blender.org/api/5.2/bpy.types.RenderEngine.html — read 2026-07-30.

*Label for the quoted words: documented.* The call exists, it is described as a test of whether the render "should been canceled" [sic], and engines are told to make it regularly so the render stays responsive.

*Label for reading a stop-to-end gap out of it: inferred.* The step a reader has to supply: if cancellation is something the engine must ask about, then the cancel request is a flag set somewhere and the render carries on until the engine next asks. That makes the length of the gap depend on how often the engine asks, and the documentation's phrase "should be used regularly for responsiveness" only implies that asking rarely makes it longer. The documentation does not say this, does not put any number on it, and says nothing at all about what the built-in engines (Cycles, EEVEE, Workbench) actually do — this page describes the interface an engine author implements.

**(b) A stop is asked for, in the interface, by a cancel button or by Esc.**

> "The task can be aborted by clicking the cancel button ()."

— Manual, Status Bar, https://docs.blender.org/manual/en/5.2/interface/window_system/status_bar.html — read 2026-07-30. The bracket in the source text is where an icon sat; the text file carries no icon name.

Two lines above it, on the same page:

> "Shows the progress of the currently running task (such as rendering or baking). Hovering the mouse pointer over the progress bar will display a time estimate."

— same page, same date. *Label: documented.* This is what is shown *while* the task runs. The page does not say what the status bar shows after the cancel button has been clicked, or whether the progress bar stays, freezes, or empties during the wait.

And:

> "As with a normal render, you can abort it with Esc."

— Manual, Viewport Render, https://docs.blender.org/manual/en/5.2/editors/3dview/viewport_render.html — read 2026-07-30. *Label: documented*, and note what it documents: it is a sentence about viewport render that states in passing that a normal render is aborted with Esc. It says nothing about what follows the key press.

The manual also states plainly that a render job can be cancelled:

> "the render job can be canceled and resumed at the last rendered frame by changing the frame range"

— Manual, Output Introduction, https://docs.blender.org/manual/en/5.2/render/output/introduction.html — read 2026-07-30. *Label: documented.* This is about what is left afterwards and how to carry on, not about the interval.

**(c) There is a named point a render job passes through when cancelled, and a named point for a normal finish.**

> "on canceling a render job. Accepts one argument: the scene data-block being rendered"

— Python API documentation, `bpy.app.handlers.render_cancel`, https://docs.blender.org/api/5.2/bpy.app.handlers.html — read 2026-07-30.

Beside it, `render_complete` is "on completion of render job", `render_post` is "on render (after)", and `render_write` is "on writing a render frame (directly after the frame is written)" — same page, same date. *Label: documented.* So a cancelled render reaches a distinct, named moment that a normal finish does not. The documentation does not say when in the stopping sequence that moment falls, what happens before it, or whether `render_post` / `render_write` also fire for the frame that was in hand.

There is also a documented per-frame finishing step:

> "Perform finishing operations after all view layers in a frame were rendered"

— Python API documentation, `RenderEngine.render_frame_finish()`, https://docs.blender.org/api/5.2/bpy.types.RenderEngine.html — read 2026-07-30. *Label: documented* that such a step exists and runs after a frame's view layers are rendered. Nothing says whether it runs when the frame was cut short. Cycles implements it — see (e).

**(d) What a render prints while it is running.**

The clearest statement of what a render puts out as it goes:

> "on printing render statistics. Accepts one argument: the render progress as a string containing current frame, current sample, render time and saving time."

— Python API documentation, `bpy.app.handlers.render_stats`, https://docs.blender.org/api/5.2/bpy.app.handlers.html — read 2026-07-30. *Label: documented.* Blender prints render statistics, and the string carries current frame, current sample, render time, saving time. This describes an ordinary running render. Nothing says whether these keep being printed after a cancel is asked for, or whether a final one is printed.

There is a switch that turns that printing off:

> "Suppress status printing (warnings & errors are still printed)."

— Manual, `-q, --quiet` under Misc Options, https://docs.blender.org/manual/en/5.2/advanced/command_line/arguments.html — read 2026-07-30. *Label: documented.*

And a switch that adds more:

> "Log statistics about render memory and time usage."

— Manual, `--cycles-print-stats` under Cycles Render Options, same page and date. *Label: documented.*

Where that output lands, in general terms:

> "The Console Window, also called a Terminal, is an operating system text window that displays messages about Blender’s operations, status, and internal errors."

— Manual, Using Blender From The Command Line, https://docs.blender.org/manual/en/5.2/advanced/command_line/index.html — read 2026-07-30. *Label: documented*, and general: it is not specific to rendering, and not specific to stopping.

**(e) From Cycles' own files (observed).**

Cycles' add-on files state where its statistics go:

> `                        help="Print rendering statistics to stderr",`

— `Blender Sources/cycles_5.2/engine.py`, line 14, inside `_configure_argument_parser()`, in the `parser.add_argument("--cycles-print-stats", ...)` call. Read 2026-07-30. **Label: observed.** Note this is more specific than the manual's wording for the same switch: the file names **stderr** as the destination.

Cycles also implements the per-frame finishing step named in (c):

> `def render_frame_finish(engine):`

— `Blender Sources/cycles_5.2/engine.py`, line 95; it calls `_cycles.render_frame_finish(engine.session)`. Read 2026-07-30. **Label: observed** that the function exists and forwards to the compiled `_cycles` module. What it does, and whether it runs on a cancelled frame, is not visible — the compiled module is not in this folder.

Cycles' Python files contain **no** cancellation handling at all. The only two lines in any `.py` file there matching "cancel" are an operator returning `{'CANCELLED'}` because an input frame was missing, and this comment:

> `        # TODO: support cancel and progress reports.`

— `Blender Sources/cycles_5.2/operators.py`, line 97, in the denoise operator. Read 2026-07-30. **Label: observed.** This is the *denoise* operator, not rendering, and I am not carrying it over to rendering.

**(f) What the release notes say.** Nothing about the stopping of a frame-range render. Two nearby entries exist and neither answers the question — both are recorded under "ruled out" below.

### Ambiguity in the question

"What does it print" could mean (i) text put out to a terminal or console, or (ii) what the interface displays — status bar, progress bar, reports. I searched for both and answered both; the sources are silent on both for the stopping interval. I read "interrupted" as a stop asked for deliberately while the render is running (Esc, the status-bar cancel button, or the equivalent from outside), not as a machine losing power, since a machine that goes down does nothing in between and prints nothing.

### Not covered

- Any actual duration, or any bound on it, for any engine, in the interface or outside it.
- Whether the frame in hand is finished, abandoned, or written partially — that is questions 3, 11 and 13 and I have not answered it here.
- Whether the built-in engines (Cycles, EEVEE, Workbench) call `test_break()` often or rarely, and whether the answer differs between CPU and GPU rendering.
- Whether output already committed for the frame in hand is flushed, closed, or dropped during the wait.
- Whether `render_stats` keeps firing after the stop is asked for, and whether `render_post` / `render_write` fire for the cut-short frame.
- Whether the status-bar progress bar keeps moving, freezes, or clears during the wait, and whether any report message is shown at the end.
- The exact text of any line Blender prints, in any circumstance — no source in this folder gives a sample of render output. The manual's three terminal screenshots, one on each of the Linux, macOS and Windows launch pages, show only start-up lines: audio-backend warnings on Linux, a driver notice and the bundled-Python path on macOS, and preference/blend-file reads followed by `Blender quit` on Windows. None shows a render. *(Corrected by the citation check, 2026-08-26: this used to say "The one terminal screenshot in the manual shows only audio-backend start-up warnings." There are three — `_images/advanced_command-line_launch_linux_console.png`, `..._macos_mac.png` and `..._windows_console.png` — and I opened all three. The claim they were offered in support of, that no source gives a sample of render output, still holds.)*
- What a stop does to a render that is between frames rather than inside one.

### Outside the interface

**How the stop is asked for outside the window is documented; what follows it is not in the sources.** Two of the documented ways of asking — the status-bar cancel button and Esc — are things a person does in Blender's own window. The third is not:

> "Blender’s typical signal handlers are not initialized, so there is no special handling for Control-C
> to cancel a render and a crash log is not written in the event of a crash."

— Python API documentation, "Signal Handlers" on the Blender as a Python Module page,
https://docs.blender.org/api/5.2/info_advanced_blender_as_bpy.html — read 2026-07-30.
*Label for the words: documented* — the Python-module build has no special handling for Control-C to
cancel a render. *Label for the turn: inferred* — that the ordinary `blender` program does have it;
the sentence calls these Blender's *typical* handlers and says this build lacks them, which only
reads as a difference if the program being contrasted with has them.

Beyond that the silence holds: for a render started and driven from outside
(`blender -b file.blend -a`), no source in this folder says what the render does after it is asked to
stop, how long that takes, or what it prints. The command-line arguments page lists no argument for
interrupting a running render; `--disable-abort-handler` sits among the debug options beside
`--disable-crash-handler` and is about Blender's handling of internal aborts, not about stopping a
render, so I ruled it out.

> *Changed by the citation check, 2026-08-26. This line used to be headed "**Not in the sources.**" and to end "No source mentions signals, Ctrl-C, or process termination in connection with rendering." A source does: `Blender Sources/api_5.2/info_advanced_blender_as_bpy.txt`, lines 80–81, quoted above. The record above lists `Ctrl-C`, `SIGINT` and `signal` among the terms searched across `api_5.2`, and a search on the last two reaches that file, but the page is not among those the record lists as opened. This version's first-pass document found and quoted the same passage.*

Two things in the sources are not tied to the interface and would hold either way, though neither answers the question: the polling call `RenderEngine.test_break()` is part of the engine's own render loop, and `bpy.app.handlers.render_cancel` is a Python handler. There is also a way to ask, from Python, whether the render job is still going —

> "Check whether a job of the given type is running."

— Python API documentation, `bpy.app.is_job_running(job_type)`, https://docs.blender.org/api/5.2/bpy.app.html — read 2026-07-30, with `RENDER` listed as a job type ("Regular rendering.") at https://docs.blender.org/api/5.2/bpy_types_enum_items/wm_job_type_items.html. *Label: documented* that the call exists. It is a way somebody could *measure* the gap; it does not tell us the gap.


**Observed, 2026-08-26 — this line has now been tested, and it no longer rests on the sources.** Every one
of the sixteen tests recorded in the observed section below was run on a render driven from outside
Blender's own window, in this version, on two machines. What they found: Control-C typed into the terminal
does ask a running render to stop, and Blender answers it in words —
`Sent an internal break event. Press ^C again to kill Blender`. The render then goes on for a measurable
stretch before it ends, prints a small, fixed set of lines at the end of that stretch, and exits with
status 1. A second Control-C ends it at once with `Blender killed` and a different exit status. A plain
termination request and an unignorable kill both end it in about two hundredths of a second with nothing
added to its printing at all. `-q` changes what is printed. See the observed section for the figures and
for the exact lines.

### Operating system

**Not in the sources.** Nothing in the manual, the API documentation, the release notes or Cycles' files ties cancelling a render, or the wait after it, to any operating system. The only OS-specific things I found anywhere near this are unrelated to stopping: `-con, --start-console` is marked "(Windows only)", and the pages on launching Blender from a terminal are written separately for Linux, macOS and Windows. I found no statement that the behaviour asked about differs, and no statement that it is the same.

---


**Observed, 2026-08-26 — this line has now been tested on two of the three operating systems.** The same
sixteen tests were run on macOS and on Linux in this version. They agree on the shape of what happens and
on nearly every line printed, and they disagree on how long it takes: the same test run at the same moment
of the same frame gave a gap of a different length on each machine, and the line the render printed last
before `Cancelled` differed too — a completed frame on the Mac against a part-finished one on the Linux
machine. The figures and the lines are in the observed section below.

*Corrected 2026-08-30.* **Windows is no longer wholly untested, and it is nowhere near answered.** This
sentence used to read that Windows was not tested and could not be, because this project had no Windows
computer. A Windows machine was rented on 2026-08-29, and **what booted reported itself as Microsoft
Windows Server 2025 Standard**, on five independent readings, from an image named for a Windows 11
desktop. The first of the sixteen tests was started on it in all four versions, and the
machine was destroyed from outside the session while that run was going, so **this version got no result
from it at all**. **The other fifteen were never begun.** **A result from that machine is a Windows
Server 2025 result and is never to be written as "Windows"**: that word claims a desktop edition, which
is what an artist's own computer runs and which nothing in this project has tested. What the machine did
and did not do is in *Observed — the Windows machine, run on 2026-08-29* below.

*Added 2026-08-29.* One more result now sits under this line and it covers **Linux only**: what a click on
the status bar's cancel button does, settled on a third machine and written up under "Observed — the
citation check's re-runs" below. There is still no way to drive a key press or a click into Blender's own
window on the Creator's Mac, because that needs a permission granted by changing a system security setting;
the refusal was checked again on 2026-08-29 and is quoted below. So for what happens **inside** Blender's
window, this answer covers Linux and leaves both macOS and Windows open.

## Record of where I looked

**Full-text searches across `manual_5.2`, `api_5.2` and `release_notes_5.2` at once** (case-insensitive), for: `cancel`, `canceled`, `cancelled`, `abort`, `interrupt`, `stop`, `stop the render`, `stop rendering`, `cancel the render`, `cancel render`, `Ctrl-C`, `Ctrl+C`, `SIGINT`, `signal`, `terminate`, `kill`, `exit code`, `Esc`, `escape`, `graceful`, `shut down`, `shutdown`, `clean up`, `cleanup`, `flush`, `stdout`, `console output`, `progress bar`, `render window`, `render job`, `job is running`, `is_job_running`, `responsiveness`, `immediately stop`, `stops immediately`, `current sample`, `finish the current`, `finishes the current`, `until the current`, `in progress`, `partially`, `mid-render`, `midway`, `part-way`, `partway`, `unresponsive`, `freeze`, `still rendering`, `keeps rendering`, `busy`, `not respond`, `background mode`, `Fra:`, `Saved:`, `Mem:`, `Remaining`, `print`, `log`, `status bar`, `terminal`, `console window`, `lock interface`, `test_break`, `update_stats`, `update_progress`, `render_cancel`, `render_complete`, `render_post`, `render_write`, `render_stats`, `render_init`, `render_frame_finish`.

**Manual pages opened in full and read:**

- `render/introduction.txt`, `render/output/introduction.txt`, `render/output/animation.txt` — the animation-render workflow pages. The animation page is the most likely home for this answer and it has none: it covers frame-sequence versus direct-to-video, the guideline "May need to interrupt rendering to use the computer, and want to be able to resume rendering where you left off", and the Overwrite trick for carrying on. All of that is about *afterwards*.
- `interface/window_system/status_bar.txt` — the only page describing the running-task display and the cancel button.
- `interface/window_system/topbar.txt` (Render Menu section) — lists Render Image, Render Animation, View Render, Lock Interface. No cancel entry, nothing about stopping.
- `interface/operators.txt` — Esc/RMB cancel modal operators. Generic operator behaviour, not the render job; ruled out.
- `editors/3dview/viewport_render.txt` — carries the one sentence in the manual that says a normal render is aborted with Esc.
- `advanced/command_line/arguments.txt` — read the Render Options, Cycles Render Options, Format Options, Debug Options and Misc Options sections. No argument for stopping a render.
- `advanced/command_line/render.txt` — rendering from the command line, single image and animation. Nothing about interrupting.
- `advanced/command_line/index.txt`, `advanced/command_line/launch/linux.txt` — what the console shows and where output goes.
- `render/cycles/gpu_rendering.txt` (FAQ "Why is Blender unresponsive during rendering?") and `compositing/limitations.txt` ("UI Freezes") — both about responsiveness *during* a render on some GPUs, neither about a stop request; ruled out as not answering, and I have deliberately not stretched them into an explanation of the wait.
- `render/cycles/render_settings/sampling.txt` (Time Limit: "Stops rendering if the time exceeds the limit…") — a render ending itself on a clock, not an interruption; ruled out.
- `interface/keymap/blender_default.txt` — the shortcut list around rendering names only F11 and F12; no cancel binding is documented there.

**Pictures opened:**

- `_images/interface_window-system_status-bar_ui-center.png` (from the Status Bar page). It shows a single status-bar strip: a film-camera icon, the words "Sequence Render", a filled progress bar reading "94%", and an "X" button at the right. *Label: documented* — the cancel control is an X beside a percentage progress bar, and the job is named in the bar. What is visible is visible; I am not reading anything about the stopping interval out of it.
- `_images/advanced_command-line_launch_linux_console.png` (from the Linux launch page). It shows a terminal after `blender` was run: jack/ALSA audio-backend connection warnings and nothing else. No render output at all. Ruled out.

**Python API pages opened in full:**

- `bpy.types.RenderEngine.txt` — the whole page, including the example engine at the top and every method. `test_break()`, `update_stats()`, `update_progress()`, `render_frame_finish()`, `report()`, `error_set()` are the relevant ones and are quoted or described above.
- `bpy.app.handlers.txt` — the whole handler list, including the note that during rendering "the frame_change_pre and frame_change_post handlers are called from one thread and the viewport updates from a different thread".
- `bpy.ops.render.txt` — every operator on the page. `render.view_cancel()` is "Cancel showing the render view", which is about closing the render view, not stopping a render; ruled out. `render.render()` itself is marked "Undocumented, consider contributing."
- `bpy.app.txt` (`is_job_running`), `bpy_types_enum_items/wm_job_type_items.txt`, `bpy_types_enum_items/operator_return_items.txt`, `bpy.types.Operator.txt` (`cancel(context)`, "Called when the operator is canceled"). The operator-level cancel is generic operator API and says nothing about a render job's stopping interval; ruled out.

**Release notes:** searched every file in `release_notes_5.2` for `cancel`, `abort`, `interrupt`, `escape`, `stop`, `progress`, `log`, `print`, `status bar`. Two hits are near the subject and neither answers it:

> "The \"Render Audio\" operator now shows a realtime progress bar that can be cancelled (previously Blender would freeze until the operation finished)."

— Release notes, Video Sequencer, https://developer.blender.org/docs/release_notes/5.2/sequencer/ — read 2026-07-30. This is the audio-mixdown operator, not a frame render; ruled out.

> " * Render Sequence Animation stopped showing total progress"

— Release notes, Fixes, https://developer.blender.org/docs/release_notes/5.2/bugfixes/ — read 2026-07-30. A one-line bug-fix entry about a progress display, with no description of behaviour; ruled out.

**Cycles' own files:** searched all of `cycles_5.2` for `cancel`, `test_break`, `interrupt`, `abort`. In the Python files there are exactly two hits, both quoted above and both unrelated to stopping a render. In `cycles_5.2/source` every `cancel` hit is the mathematical sense (terms cancelling in a formula) in kernel headers such as `source/util/math_base.h` and `source/kernel/closure/bsdf_microfacet.h`; none is about stopping. Read `cycles_5.2/engine.py` and `cycles_5.2/__init__.py` in full. What is here is the add-on's Python layer plus kernel headers; the session and render-loop code, which is where a stop would be handled, is inside the compiled `_cycles` module and is not in this folder. That is why Cycles' files cannot settle this question.

**Not consulted:** the `requirements/` folder, and every other version's folder (`*_4.5`, `*_5.0`, `*_5.1`), since the question is about 5.2. The web was not used and was not needed.

---

---

## Observed — the hand tests, run on 2026-08-26

Everything in this section is what two machines printed on **2026-08-26**, not what a source says. I ran
all of it. The full list of tests, the exact commands, and the whole of every run's output are in
`Audits/Blender Research/Frames/Hand tests — Architecture questions.md` and in the folder beside it,
`Audits/Blender Research/Frames/Hand tests — Architecture questions/`.

### How it was set up

**The two machines.** The Creator's Mac — a MacBook Pro, M1 Pro, macOS 26.5.2, Apple's own graphics, no second card. And the rented Linux machine in Norway — a Docker container holding half its host, Ubuntu 22.04.5 LTS, one NVIDIA GeForce RTX 3060 12 GB on driver 595.84, and a processor quota of about fifteen cores of an Intel Xeon E5-2697A v4 shared with another tenant, which misreports itself as the host's 32 threads and 125 GB. **They are never merged below**: where a test ran on both, there are two
results and a plain statement of whether they agreed.

**The build. The two machines did not run the same build.** The Mac ran Blender 5.2.1 LTS Release Candidate (hash efb201aa0e0e built 2026-08-21 01:35:37). The Linux machine ran Blender 5.2.1 LTS (hash 9e2066aef7ef built 2026-08-25 02:12:34). Four days and a different hash apart. For this version, a difference between the two machines below may be the build rather than the operating system, and there is no finished 5.2.1 for macOS on this project and no release candidate for Linux on it, so the two causes cannot be told apart here.

**The scene.** A script built the same scene in every Blender version and saved it once per version, from
that version, so that no version had to convert another's file and print conversion messages into the very
output being measured. The scene: Cycles on the processor, 600 samples, adaptive sampling off, denoising
off, 32 bounces, 320 by 180 at full size, threads fixed at 4, frame range 1 to 20; a camera, a ground
plane, a glass ball and a metal ball keyframed to rotate so that no two frames are the same picture, a dim
world and one area light. One frame took about 21 seconds on the Mac and about 37 seconds on the Linux
container. *(Corrected by the citation check, 2026-08-26: this used to read "about 27 seconds on the
Linux container". That is contradicted by this document's own Linux output — a `Sample 257/600`
line carrying a `Remaining:` figure of about twenty-one seconds puts the whole frame near
thirty-seven — and by a repeat on both machines on 2026-08-26, whose finished Linux frames read
between 36.5 and 37.6 seconds in every version. The Mac figure is right: the repeat's finished Mac
frames read between 18.6 and 20.5 seconds.)* The Linux container's processor is shared with another tenant, so its timings are not a clean
measure of anything but itself.

**`--factory-startup` was on every command.** The Creator's Mac has add-ons installed which print their own
lines at start-up; a test about what a render prints cannot have another program's printing in it.

**How Control-C was pressed.** A signal sent with `kill -INT` is not the same act as a person pressing
Control-C: Control-C is a character typed into a terminal, which the terminal turns into a signal for the
whole foreground group. So the runs used a small harness that gave Blender a real terminal (a
pseudo-terminal), typed the actual Control-C character into it at a chosen moment, and printed every line
Blender produced with the seconds elapsed since the process started, plus a marked line at the instant of
the Control-C and another at the instant the process was gone. **"The gap" below always means the seconds
between those two marked lines.**

**The command.** On the Mac:

> `"/Applications/Blender.app/Contents/MacOS/Blender" -b --factory-startup ~/q15/q15_5.2.blend -s 1 -e 5 -o <an empty folder>/f_#### -F PNG -a`

On the Linux machine the same, with `/opt/blender/5.2.1/blender` and `/root/q15/q15_5.2.blend`.

---

### 1. Control-C typed eight seconds into the third frame

**On the Creator's Mac.** The gap was **12.87 seconds**. Immediately after the Control-C, and before anything
else:

> `^C`  
> `Sent an internal break event. Press ^C again to kill Blender`

Then nothing at all for the whole of the gap, until these came out together at the end of it:

> `01:03.055  render           | Fra: 3 | Mem: 2M | Sample 600/600`  
> `01:03.056  render           | Fra: 3 | Mem: 2M | Finished`  
> `01:03.056  render           | Fra: 3 | Mem: 2M | Cancelled`  
> `Blender quit`

The exit status was `1`. The output folder held `f_0001.png` and `f_0002.png` and nothing else: the frame
in hand left no file.

**On the rented Linux machine.** The gap was **7.82 seconds**. The same two lines came first:

> `^C`  
> `Sent an internal break event. Press ^C again to kill Blender`

and at the end of the gap:

> `01:29.164  render           | Fra: 3 | Remaining: 00:21.01 | Mem: 2M | Sample 257/600`  
> `01:29.165  render           | Fra: 3 | Remaining: 00:21.01 | Mem: 2M | Sample 0/600`  
> `01:29.165  render           | Fra: 3 | Remaining: 00:21.01 | Mem: 2M | Cancelled`  
> `Blender quit`

The exit status was `1`, and again only the first two frames were on disk.

**Did the two machines agree?** On the shape of it, yes: the same two lines at the start, a three-line
ending, the same exit status, the same two files on disk. On the length, no — 12.87 seconds against 7.82. And on
one thing they differed plainly: on the Mac the last sample line read `Sample 600/600` and was followed by
a `Finished` line; on the Linux machine it read `Sample 257/600`, there was no `Finished` line, and the
`Cancelled` line still carried a `Remaining:` figure.

### 2. Where in the frame the stop lands

The same command, with the Control-C typed at four different moments. The fourth run had the sample count
raised to 2400 by adding `--python-expr "import bpy; bpy.context.scene.cycles.samples = 2400"`.

| when the Control-C was typed | gap, Mac | gap, Linux machine |
|---|---|---|
| one second after the process started, during set-up | 20.54 s | 30.38 s |
| two seconds into the third frame | 18.84 s | 13.65 s |
| about half way through the third frame | 9.94 s | 4.97 s |
| two seconds into the third frame, at 2400 samples | 36.61 s | 13.66 s |

In the one-second run on the Mac the render went on to print `Sample 600/600` and `Finished` for frame 1
before `Cancelled`, and the gap of 20.54 seconds is the whole of that frame. In the 2400-sample run on the Mac
the last sample line before `Cancelled` read `Sample 1025/2400` — the same figure in all four versions —
with a `Remaining:` figure of three quarters of a minute or so still on it.

### 3. How many frames were still to come

Two runs, identical but for the frame range.

| frame range | gap, Mac | gap, Linux machine |
|---|---|---|
| `-s 1 -e 5` | 14.52 s | 7.73 s |
| `-s 1 -e 100` | 14.30 s | 7.77 s |

No line printed after the Control-C, in any of the four runs, mentioned a frame later than the one in hand.

### 4. A second Control-C, five seconds after the first

The first Control-C's own line says what a second one does: `Press ^C again to kill Blender`.

**On the Mac.** The process was still alive when the second Control-C was typed. What it printed, in order:

> `Blender killed`  
> `Error: Not freed memory blocks: 30820, total unfreed memory 9.496281 MB`

and it was gone in the same instant. Exit status: **exited normally with code 2**. The first two frames were on disk.

**On the Linux machine.** What it printed, in order:

> `Blender killed`  
> `Error: Not freed memory blocks: 30787, total unfreed memory 9.527412 MB`

Exit status: **exited normally with code 2**.

No sentence about freeing memory after a leak detector appeared on either machine. In 5.0 on the rented Linux machine, the same test printed such a sentence sixteen times.

### 5. A render with no terminal at all, stopped by a signal

Started detached, in its own session, with no terminal attached and its printing going to a plain file,
then sent one signal from outside eight seconds into the third frame.

| signal | gap, Mac | gap, Linux machine |
|---|---|---|
| a plain termination request (`kill`) | 0.03 s | 0.04 s |
| an unignorable kill (`kill -9`) | 0.02 s | 0.02 s |
| an interrupt (`kill -INT`) | 13.91 s | 7.63 s |

After a plain termination request, and after an unignorable kill, nothing whatever was added to the
printing file: its last line is an ordinary progress line for the frame in hand. After an interrupt

> *Corrected by the citation check, 2026-08-26: this used to say the last line was "from the middle of
> the frame". It is a line for the frame in hand, but it was printed near that frame's beginning — in a
> repeat on the Creator's Mac in all four versions the last line of the file reads `Sample 1/600`, because
> Blender prints a sample line only at the boundaries of Cycles' sample batches and the signal arrived
> before the next one. Nothing else about this entry changed: nothing whatever is added to the file.*

the file ends exactly as entry 1's runs did — the sample line, the `Cancelled` line, `Blender quit`.

### 6. A Control-C landing between frames

The Control-C was typed the instant a `Saved:` line appeared. The gap was **0.05 seconds** on the Mac and
**0.08 seconds** on the Linux machine. Everything printed between the Control-C and the process being gone,
on the Mac:

> `^C`  
> `Sent an internal break event. Press ^C again to kill Blender`  
> `00:22.530  render           | Fra: 2 | Mem: 0M | Initializing`  
> `Blender quit`

### 7. `-q` added

Entry 1's command with `-q` added straight after `-b`, and the Control-C typed fifty seconds in. Gap:
**17.11 seconds** on the Mac, **2.40 seconds** on the Linux machine.

The three render lines were still printed on both machines — the sample line, `Finished` or `Sample 0/600`, and `Cancelled` — but `Blender quit` was not.

The line `Sent an internal break event. Press ^C again to kill Blender` was printed under `-q` in every
version on both machines.

### 8. The fullest logging

**What was typed.** Entry 1's command with `--log "*" --log-level debug` added straight after `-b`.
Both machines accepted it.

**Gap.** **14.14 seconds** on the Mac, **23.13 seconds** on the Linux machine.

**What it added between the Control-C and the end.** A block of `cycles`-category lines the ordinary run
does not print. From the Mac's run, in order:

> `01:06.488  cycles           | Rendered 599 samples in 22.0723 seconds (0.0368485 seconds per sample), occupancy: 1`  
> `01:06.488  cycles           | Measured path tracing occupancy: 1`  
> `01:06.488  cycles           | Write tile result.`  
> `01:06.488  cycles           | Write tile result via buffer write callback.`  
> `01:06.488  cycles           | Invoke buffer update callback.`  
> `01:06.488  cycles           | Average display update time: 3.41702e-06 seconds.`  
> `01:06.488  render           | Fra: 3 | Mem: 2M | Sample 600/600`  
> `01:06.488  cycles           | Rendering was canceled.`  
> `01:06.488  render           | Fra: 3 | Mem: 2M | Finished`  
> `01:06.488  render           | Fra: 3 | Mem: 2M | Cancelled`  
> `01:06.488  cycles           | Total render time: 22.1152`  
> `01:06.488  cycles           | Render time (without synchronization): 22.1113`

One line in that block appears in no other run of this test, in any version, on either machine: `cycles           | Rendering was canceled.` It appeared in this version's run on the Mac and not in this version's run on the Linux machine, where the render was stopped part-way through the frame rather than at the end of it.

The other half of that difference: 5.0 and 5.1, in this same place, print a `Full path tracing report:`
line followed by a block of two dozen or so lines — the device path-traced on, a render-scheduler summary,
a timing table — and no `Rendering was canceled.` line at all. This version's run printed the cancel line
and no report block.


### 9. `--debug-memory` added

Gap: **13.88 seconds** on the Mac, **7.77 seconds** on the Linux machine. Nothing about memory, blocks, leaks or
freeing appeared anywhere in either run — not before the Control-C and not after it. The lines printed
between the Control-C and the end were the same as entry 1's.

### 10. `--disable-abort-handler` added

Gap: **13.83 seconds** on the Mac, **7.64 seconds** on the Linux machine. The lines printed between the
Control-C and the end were the same as entry 1's, and so was the exit status.

### 11. What the handlers see

A Python file loaded with `--python` attached a printing function, with a timestamp, to `render_init`,
`render_pre`, `render_post`, `render_write`, `render_cancel`, `render_complete`, `render_stats` and
`exit_pre`, and reported any it could not find.

**Whether this version has `exit_pre`:** **it does.** The run printed `HANDLER exit_pre         attached`, and the handler fired — 4.5 and 5.0 print `DOES NOT EXIST IN THIS VERSION` for the same line.

**What fired after the Control-C.** These, in this order, all within a thousandth of a second of each other,
at the very end of the gap and not before it: `render_stats`, `render_stats`, `render_stats`, `render_cancel`, `exit_pre`. `render_post`, `render_write` and `render_complete` did
not fire at all for the frame in hand. The order was the same on both machines.

**The exact string `render_stats` was handed** on its last call — the one alongside the `Cancelled` line —
on the Mac:

> `Mem: 2M | Cancelled`

and on the Linux machine:

> `Remaining: 00:20.75 | Mem: 2M | Cancelled`

Gap: 13.88 seconds on the Mac, 7.64 seconds on the Linux machine.

### 12. The graphics card against the processor

`--cycles-device` does exist in this version. Its `--help` entry reads `--cycles-device <device>` with the
example `# blender -b file.blend -f 20 -- --cycles-device OPTIX`.

| run | gap, Mac (`METAL`) | gap, Linux machine (`OPTIX`) |
|---|---|---|
| on the processor, `-- --cycles-device CPU`, 600 samples | 13.95 s | 7.59 s |
| on the card, 600 samples | 0.57 s | the render finished all five frames before the stop could be asked for |
| on the card, 20000 samples | 26.54 s | 30.01 s |

On the Mac's 600-sample card run the last line before `Cancelled` was

> `00:19.563  render           | Fra: 4 | Mem: 1410M | Sample 600/600 (Using optimized kernels)`

On the Linux machine the 600-sample card run finished normally, with five `Saved:` lines and exit status
`0`; that is recorded as the result it is, and the run was then repeated with the sample count raised to
20000, which is the third row of the table.

### 13. A different engine

`blender --factory-startup -E help` in this version lists `BLENDER_EEVEE`, `BLENDER_WORKBENCH` and `CYCLES`. Both runs
used a frame range of 1 to 500 at 1920 by 1080, with the Control-C typed twenty seconds in.

| engine | gap, Mac | gap, Linux machine |
|---|---|---|
| `BLENDER_EEVEE` | 1.21 s | 65.04 s |
| `BLENDER_WORKBENCH` | 0.15 s | 0.16 s |

The rented Linux machine's viewport is drawn by its processor rather than by its card. Its EEVEE frames
took about ninety seconds each there against a fraction of a second on the Mac, so the two EEVEE figures
are not a comparison of anything and are not offered as one.

### 14. One video file instead of separate images

The output was set to an FFmpeg MPEG-4 H.264 video with
`--python-expr "import bpy; s=bpy.context.scene; s.render.image_settings.file_format='FFMPEG'; s.render.ffmpeg.format='MPEG4'; s.render.ffmpeg.codec='H264'"`.
Gap: **13.49 seconds** on the Mac and **7.66 seconds** on the Linux machine, against entry 1's 12.87 and 7.82 for the
same version on the same machines. The lines printed between the Control-C and the end were the same as
entry 1's, and no line named the video file or its closing.

### 15. Inside Blender's own window — Escape

Run on the rented Linux machine only, on its display server at `DISPLAY=:1`, 1920 by 1080. Blender was
opened with a window on the blend file, a timer started the animation render five seconds later, and an
Escape key press was sent with `xdotool key --clearmodifiers Escape` fifteen seconds into the third frame,
after the window had been brought to the front. The whole screen was photographed immediately before the
key press and once a second for forty seconds afterwards.

**What the terminal printed: nothing.** A render started inside Blender's own window prints only a `Saved:`
line and a `Time:` line per frame — no per-sample progress lines at all. After the Escape no line of any
kind appeared, and no `Saved:` line for the frame in hand ever came.

**What the photographs show.** Immediately before the key press, the status bar carried a task icon, the
words `Rendering...`, a progress bar reading `38%`, and an X at its right end; the render window's header read

> `Frame:3 | Last:00:36.63 Time:00:14.35 | Remaining: 00:22.60 |`



Comparing each photograph against the one before it, the screen settled into a picture that did not change
again for the rest of the forty seconds, and it settled **between 0.0 and 1.0 seconds after the key press**. In the settled picture the status bar is empty — no
task, no words, no progress bar, no X — the render window's header reads

> `Frame:3 | Time:00:16.31 | Mem:2M, Peak: 2M`

with no `Remaining:` figure on it, and the timeline's current frame has gone back to 1.

**The Mac's half of this test could not be run**, for the reason given at the end of this section.

### 16. How long the render job reports itself as running

The same set-up as entry 15, with a timer printing `bpy.app.is_job_running('RENDER')` every tenth of a
second. Measured against the moment of the Escape, the last answer of `True` came **0.70 seconds** after it
and the first answer of `False` came **0.81 seconds** after it. The call did not crash Blender in this
version on this machine.

---

### What was not run, and why

- **Windows, in every one of these tests.** Neither of the two machines above runs Windows, so every
  figure in this section is macOS or Linux, and any use made of these figures has to say so. *(Corrected
  2026-08-30. This used to add "this project has no Windows computer", which was true when it was written
  and is not now.)* A Windows machine was rented on 2026-08-29, and what it did and did not run is in
  *Observed — the Windows machine, run on 2026-08-29* below.
- **The Mac's half of entries 15 and 16 — the window tests on macOS.** Driving a key press into another
  program's window on macOS means sending synthetic events through the accessibility interface, which
  needs a permission this session does not have and which is granted by changing a system setting. The
  window half of this answer therefore covers Linux only.
- **The status bar's cancel button, as against Escape — this is no longer an item that could not be run.**
  Three attempts were made on 2026-08-26 on the rented machine's screen — at 1180,1070, then twice at
  1069,1068, the position read off the photographs, the third with the main window raised first — and on
  every one the render carried on and the next frame was saved. A fourth and fifth, the same day, failed in
  the same way. All of that stands as recorded.

  > *Settled on 2026-08-29 on a third machine, and written up under "Observed — the citation check's
  > re-runs" below. A sixth attempt at 1069,1068 failed again, and the reason is that Blender's own Quick
  > Setup panel, which `--factory-startup` opens every time and which no window list shows because it is
  > drawn inside the main window, takes the first click made anywhere. With one click spent on that panel
  > first, the same click at the same place stopped the render in all four versions.*
- **Any comparison needing a second machine of the same kind, or a different make or number of graphics
  cards.** There is one Mac and one rented container with one NVIDIA RTX 3060.

## Observed — the citation check's re-runs, run on 2026-08-29

Everything in this section is what a **third** machine printed on **2026-08-29**, not what a source says.
The citation check of 2026-08-26 sent five things on to a second batch of hand tests; what follows is the
part of that batch which could be run in this version. The full list, the exact commands, and the whole of
every run's output are in
`Audits/Blender Research/Frames/Hand tests — the citation check's re-runs.md` and in the folder beside it,
`Audits/Blender Research/Frames/Hand tests — the citation check's re-runs/`.

### The third machine

Rented fresh for this batch, and neither of the two machines above: the Norway container was destroyed. A
Docker container in the United Kingdom, Ubuntu 22.04.5 LTS, kernel 6.8.0-136-generic, x86_64, an Intel Xeon
E5-2690 v4 at 2.60 GHz with 12 cores, a memory ceiling of 60.2 GB, and one NVIDIA GeForce GTX 1080 8 GB on
driver 580.173.02. Unlike the Norway container it does not misreport its own size — `nproc` says 12 against
a processor quota of 11.52 cores. Its display is Xvfb on `DISPLAY=:1` at 1920 by 1080 with openbox as the
window manager, and it is **drawn by the processor**: `glxinfo -B` reports `llvmpipe (LLVM 15.0.7, 256
bits)` and `Accelerated: no`, so nothing about what a viewport draws, or how fast, is representative. It ran
Blender 5.2.1 LTS (hash `9e2066aef7ef` built 2026-08-25 02:12:34) — the same hash the rented Linux machine above ran. The Mac's 5.2 in the 2026-08-26 runs was the release candidate `efb201aa0e0e`; on 2026-08-29 the Creator's Mac answers with this finished build instead, which is written up in the re-runs list. The scene and the harness were the same ones the 2026-08-26 runs used, copied
across unchanged. One frame took about 36 seconds in a background render and about 34 seconds in a render
started inside Blender's own window.

### What a click on the status bar's cancel button does

Run on the third machine's display, in this version. Blender was opened with a window on the scene file, a
timer started the animation render five seconds later over frames 1 to 5, and fifteen seconds into the third
frame the screen was photographed, every window on it was listed, the main window was raised, the pointer
was moved to **x=1069, y=1068** — where the 2026-08-26 photographs put the cancel X — the screen was
photographed again with the pointer sitting there, and then one click was made. The screen was photographed
every second for forty seconds afterwards.

**Done that way it did not stop the render, which is the sixth time that has happened.** The click landed at
`[+0094.481]` and `f_0003.png` was written at `[+0113.714]`, 19.2 seconds later. The run was still rendering when the
forty seconds of photographs ran out.

**Nothing was over the button.** The window list taken seconds before the click holds two windows and no
more:

> `window 4194306  name='q15_5.2 [~/q15/q15_5.2.blend] - Blender 5.2.1 LTS'  geometry='WINDOW=4194306 X=0 Y=38 WIDTH=1920 HEIGHT=1061 SCREEN=0'`
> `window 4194352  name='Blender Render'  geometry='WINDOW=4194352 X=54 Y=812 WIDTH=351 HEIGHT=256 SCREEN=0'`

Neither covers 1069,1068, and the pointer arrived where it was sent —
`xdotool getmouselocation --shell says: X=1069 Y=1068`.

**The click did not reach the button, and this is how that was established.** The button did not react to
the pointer at all. Comparing the photograph taken with the pointer elsewhere against the one taken with the
pointer resting on the cancel X, a 30 by 24 box around that X is **identical — nought pixels different**. A
control that a pointer is sitting on and which does not change is a control the pointer is not reaching.

**What took the click was Blender's own Quick Setup panel.** That was read off the photographs of the 5.1
run, where the picture taken immediately before the click shows the panel in the middle of the screen and
the one a second after shows it gone, with the tooltip **`Stop this job.`** under the pointer. `--factory-startup`
opens that panel, and it is drawn inside the main window rather than as a window of its own, so no window
list can show it. **In this version the panel was not read off a photograph**, and what stands here on its
own evidence is narrower: the first click did not reach the button and a second one, made at the same place
a moment later, did.

**With one click spent on the panel first, the same click at the same place stopped the render.** The run
was repeated with `xdotool mousemove 1600 1068 click 1` on an empty part of the status bar at `[+0092.705]`,
the pointer moved to 1069,1068 at `[+0094.802]`, and the click made at `[+0095.379]`. **`f_0003.png` was never
written.** This time the button did react to the pointer: **399 pixels** changed in the same 30 by 24 box
when the pointer arrived, and the X lit up; no tooltip was showing in the photograph taken at that instant. The screen then settled between 0.0 and 1.0 seconds after the click, and in the settled
picture the status bar carries no progress strip and no X.

### The window half on macOS — checked again, and still out of reach

The refusal was checked rather than assumed, on the Creator's Mac, macOS 26.5.2. In the terminal:

> `osascript -e 'tell application "System Events" to key code 53'`

came back with

> `36:47: execution error: System Events got an error: osascript is not allowed to send keystrokes. (1002)`

and exit status 1 — word for word what the citation check recorded on 2026-08-26. Driving a key press into
another program's window on macOS needs the accessibility permission, and granting it means changing a
system security setting, which is not an agent's to change. Nothing was installed and no setting was
touched. The window half of this answer therefore covers Linux and leaves macOS open.

---

## Observed — the Windows machine, run on 2026-08-29

Everything in this section is what a **fourth** machine printed on **2026-08-29**, not what a source says.
It is the first Windows computer this question has ever been run on. The full account — the machine, the way in, the
harness, the traps in it, and the raw output of every run — is in
`Audits/Blender Research/Frames/Hand tests — Windows.md` and in the folder beside it,
`Audits/Blender Research/Frames/Hand tests — Windows/`.

### The fourth machine, and the edition it turned out to be

A rented machine in the Netherlands: four logical processors of an `Intel Xeon Processor (SapphireRapids)`
at 2594 MHz, 8 GB of memory, a 60 GB disk, running under QEMU on somebody else's hardware. **It has no
graphics card of any kind** — only a `Microsoft Basic Display Adapter` with `AdapterRAM 0` — so every
figure from it is a processor figure. It stood about fifty-three minutes and was destroyed at 21:51:42 the
same day.

**It is not desktop Windows, and that must travel with every figure below.** It was created from an image
named `windows_desktop_windows_11_64-bit`, and what booted reported itself as **Microsoft Windows Server
2025 Standard** on five independent readings, none of them ambiguous:

> `(Get-CimInstance Win32_OperatingSystem).ProductType` → `3`
> `(Get-CimInstance Win32_OperatingSystem).Caption` → `Microsoft Windows Server 2025 Standard`
> `…\CurrentVersion\EditionID` → `ServerStandard`
> `…\CurrentVersion\InstallationType` → `Server`
> `…\ProductOptions\ProductType` → `ServerNT`

Its version is 10.0.26100, `DisplayVersion 24H2`, build **26100.32995**, and `cmd /c ver` answers
`Microsoft Windows [Version 10.0.26100.32995]`. **So everything below is a Windows Server 2025 result and
none of it may be written as "Windows"** — that word claims a desktop edition, which is what an artist's
own computer runs and which this project has still never tested. Windows Server 2025 and Windows 11 24H2
carry the same build number and the same console host, which is why the machine was worth using for this
question at all; that is a reason to keep the result, not a licence to relabel it. How the edition was
established, and what it costs the account it was rented from, is in
`Audits/Blender Research/Frames/Hand tests — Windows.md` and in the equipment section of
`Preparation/Blender Research/Method.md`.

**The Blender.** Blender 5.2.1 LTS, the official Windows x64 build taken from Blender's own download
server and unpacked to `C:\blender\5.2.1`. Its hash as the running program reports it is `9e2066aef7ef`, built 2026-08-25 02:38:20 — the
**finished** 5.2.1 LTS, **the same hash the two Linux machines above ran**. The Windows list records the
Creator's Mac as carrying the release candidate, hash `efb201aa0e0e` built 2026-08-21, which is what the
Mac ran in the 2026-08-26 runs; the note above records the Mac answering with the finished build on
2026-08-29. So at this version a difference between this machine and the Mac's 2026-08-26 figures may be
the build rather than the operating system, exactly as it may be between the Mac and the Linux machine
there.

**The scene and the set-up were the ones the 2026-08-26 runs used** — the same script, run on this machine
by this version so that no version had to convert another's file, and `--factory-startup` on every command.
**One frame took about 30 seconds here**, against about 21 on the Mac. Building the scene in this
version printed three `DeprecationWarning: 'Material.use_nodes' is expected to be removed in Blender
6.0` lines.

### How the Control-C was delivered, and why it is not quite a key press

**Windows has no pseudo-terminal**, so the harness the Mac and Linux runs used could not be carried over.
The Windows counterpart of a Control-C typed at a terminal is a console control event raised on the console
the process is attached to. The harness gave itself a console, started Blender as a child of it with
Blender's printing coming back down an ordinary pipe, and at the chosen moment raised
`GenerateConsoleCtrlEvent(CTRL_C_EVENT, 0)`. **It is the same event a key press produces and Blender cannot
tell the two apart, but it is not the identical act**, and that is said here rather than buried.

**A caution on the timestamps, and not on the gap.** Blender's printing came down a pipe rather than a
terminal, and its own output buffering on a pipe is not what it is on a terminal, so the moment at which a
particular line *appeared* is less trustworthy here than on the Mac and the Linux machines. The gap from
the Control-C to the process being gone is measured by the harness itself and is not affected. The
pseudo-console harness that would settle it is written and unfinished.

### 1. Control-C part-way through a frame — started in this version, and killed with the machine

The run was one pass over all four versions, with the same harness and the same command shape: the 2026-08-26
list's Blender command line character-for-character, with Windows paths, this version's program under
`C:\blender\5.2.1\blender.exe`, this version's own scene file, and `--after "Fra: 3"` with the space, as on the other
machines.

**The machine was destroyed from outside the session while that pass was going**, and the file the harness
was writing lived on the machine's own disk and went with it. Which version the pass had reached is not
known. **This version produced no result, nothing may be inferred for it, and the reason is the machine's
destruction rather than anything Blender did.**

### What was not run on this machine, and why

- **Entries 2 to 16, in every version.** The session was ended for budget and the machine was destroyed
  from outside it before any of them was begun. None was attempted and none failed, and **no result may be
  inferred for any of them.** A driver for entries 1 to 14 was written and is kept beside the list, with
  the Windows substitution each entry needs, but nothing was ever run through it.
- **Entry 1 in three of the four versions.** The four-version pass was under way when the machine went, and
  it took the file it was writing with it.
- **Anything needing desktop Windows.** This machine is Windows Server 2025 Standard, so the desktop
  edition stays untested here as it is everywhere else in this project.
- **Anything needing a graphics card.** There is none, so entry 12's card half could not have been run on
  it even with more time.
- **Blender's own window — entries 13, 15 and 16.** They need working graphics, and this machine has a
  basic display adapter with no hardware graphics. Whether Blender would open a window on it at all was
  never tested; that is an open question, not a result.

---

## Things a hand test could still settle

The list that stood here has been run — on 2026-08-26 on both of this project's machines, and on
2026-08-29 on a third one for what the citation check sent back. Its results are in the observed
sections above. **What the status bar's cancel button does has been taken off this list**, because the
2026-08-29 run settled it. What is left open, and why:

1. **Almost everything on Windows, and every last thing on desktop Windows.** A Windows machine was
   rented on 2026-08-29 and destroyed the same day. **It reported itself as Windows Server 2025 Standard**,
   so it says nothing about the desktop edition, which is untested in every version and every test.
   Entry 1's four-version pass was killed by the machine's destruction before this version produced
   anything. **Entries 2 to 16 were never begun, in any version.** Another machine is needed for them,
   and three things about the harness are already known — anyone running the rest will meet all three:

   - **A Ctrl+C key record written into the console's own input buffer does not raise the signal on that
     build.** That is what the console receives when a person presses the keys, and it is what the older
     literature recommends. A whole four-version pass was run that way and every run rendered to the end
     untouched, with the harness's own marker sitting in the middle of a render nothing had touched. Raise
     `GenerateConsoleCtrlEvent(CTRL_C_EVENT, 0)` on the console Blender is attached to instead.
   - **A process reached over SSH inherits "Control-C is disabled", and hands it on to everything it
     starts.** The event is raised, the call succeeds, the child is on the same console, and nothing
     happens. Clear the inherited state with `SetConsoleCtrlHandler(None, False)` **before** the child is
     created, or the child inherits it in turn. Any Windows work on this project that drives a program over
     SSH and expects it to answer a Control-C has to do this.
   - **The harness that would give trustworthy line timings is written and unfinished.** Windows does have
     a real terminal object, and writing the Control-C character into it is the exact counterpart of what
     the Mac and Linux runs did. That harness got as far as running Blender under one, but its output never
     reached the timing code, so it pressed nothing and no result rests on it. Until it works, the moment a
     particular printed line *appeared* in a Windows run is less trustworthy than the Mac's and Linux's.
     The gap from the Control-C to the process being gone is measured by the harness itself and is not
     affected.

   All three, with the raw output and the diagnosis behind each, are in
   `Audits/Blender Research/Frames/Hand tests — Windows.md`.
2. **The window half on macOS.** Escape and the status bar's cancel button were driven and photographed on
   the rented Linux machine's screen. On the Creator's Mac they were not, because driving a key press into
   another program's window there needs a permission granted by changing a system setting. Checked again on
   2026-08-29 rather than assumed, and it is still refused.
3. **Anything needing a second machine of the same kind, or a different make or number of graphics cards.**
   There is one Mac and one rented container with one NVIDIA RTX 3060.

---

2026-08-26 — Claude Opus 5 (Claude Code subagent) — Frames, Blender 5.2.

2026-08-26 — Claude Opus 5 (Claude Code) — hand tests for Frames, run on both machines and written back into this document.

2026-08-29 — Claude Opus 5 (Claude Code) — the citation check's re-runs written into this document: the cancel-button result on a third machine, and the macOS refusal checked again.

2026-08-30 — Claude Opus 5 (Claude Code) — the Windows machine written into this document: the machine and the edition it reported, entry 1 started in this version and killed with the machine, what was not run, and the three harness traps.

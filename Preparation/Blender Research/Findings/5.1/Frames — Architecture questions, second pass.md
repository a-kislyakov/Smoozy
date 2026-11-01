# Frames — Architecture questions, second pass (Blender 5.1)

**Sources consulted, and on what date.**
Blender 5.1's manual (`Blender Sources/manual_5.1`, including its `_images` folder), Blender 5.1's Python API documentation (`Blender Sources/api_5.1`), Blender 5.1's release notes (`Blender Sources/release_notes_5.1`), and Cycles' own files (`Blender Sources/cycles_5.1`). All four were taken on **2026-07-30**, and that is the date read for everything quoted below.

I read only the question sheet and files inside Blender Sources/. I did not open any other part of this project.

One disclosure, so the line above is exact. To confirm my own file had been written to the right place, I listed the contents of the folder I was told to save into. That showed me the file names in it and nothing else. I opened none of them, and I opened no other file, folder or listing anywhere in this project.

Questions answered here: **15 only.**

---

## 15. When a render working through a range of frames is interrupted, what does it do between the moment it is asked to stop and the moment it ends — how long does that take, what settles that, and what does it print in the meantime?

**LABEL: not in the sources.**

The four sources say that a render can be asked to stop, and they name three ways of asking. They do not describe the interval the question asks about. Nothing in the manual, the API documentation, the release notes or Cycles' own files says what a render does after it has been asked to stop and before it ends, how long that lasts, what makes it longer or shorter, or what it writes to the console in the meantime.

Below are the pieces that do exist. Each is labelled on its own. None of them answers the question; together they mark its edges.

### What the sources do say

**(a) There are three documented ways to ask a render to stop, and one of them works from outside the window.** — *documented*

> "The task can be aborted by clicking the cancel button ()."
Source: the manual, https://docs.blender.org/manual/en/5.1/interface/window_system/status_bar.html — read 2026-07-30.

> "As with a normal render, you can abort it with Esc."
Source: the manual, https://docs.blender.org/manual/en/5.1/editors/3dview/viewport_render.html — read 2026-07-30. (This sentence is on the Viewport Render page and says a viewport render is aborted the way "a normal render" is.)

> "Blender’s typical signal handlers are not initialized, so there is no special handling for Control-C to cancel a render and a crash log is not written in the event of a crash."
Source: the Python API documentation, https://docs.blender.org/api/5.1/info_advanced_blender_as_bpy.html — read 2026-07-30. (This page is about Blender built as a Python module. It states what is missing there, and in doing so states that Blender's ordinary signal handlers give Control-C special handling, and that the special handling is to cancel a render.)

**(b) What the interface shows while a render is running.** — *documented*
The manual's Status Bar page carries a picture of the middle of the status bar: `[image: _images/interface_window-system_status-bar_ui-center.png]`. I opened the picture. It shows a camera icon, the words "Sequence Render", a progress bar filled to "94%", and an "X" button at the right. The page's words for it:

> "Shows the progress of the currently running task (such as rendering or baking). Hovering the mouse pointer over the progress bar will display a time estimate."
Source: the manual, https://docs.blender.org/manual/en/5.1/interface/window_system/status_bar.html — read 2026-07-30.

This is what is on screen *while the render runs*. The page does not say what the status bar shows after the X is clicked and before the job ends.

**(c) Something does run on cancelling, but the sources only name it.** — *documented*

> "bpy.app.handlers.render_cancel
> on canceling a render job. Accepts one argument: the scene data-block being rendered"
Source: the Python API documentation, https://docs.blender.org/api/5.1/bpy.app.handlers.html — read 2026-07-30.

So Blender has a defined moment "on canceling a render job" at which anything hooked to it is run. The page does not say when in the interval that moment falls, what else happens around it, or how long any of it takes. For comparison, the same page lists `render_complete` ("on completion of render job"), `render_post`, `render_write` and `render_init`; it does not describe an ordering or a timing for any of them.

**(d) A stop is a request that the render engine has to notice, not an instant halt.** — *documented that the check exists; **inferred** that this is what settles the delay*

> "test_break()
> Test if the render operation should been canceled, this is a fast call that should be used regularly for responsiveness"
Source: the Python API documentation, https://docs.blender.org/api/5.1/bpy.types.RenderEngine.html — read 2026-07-30.

Documented: a render engine finds out that it should stop by calling `test_break()`, and the documentation tells engine authors to call it often "for responsiveness".

Inferred, and the step is this: if an engine only learns of the stop by asking, then it cannot react before it next asks, so the gap between the request and the engine reacting is set by how often that engine calls `test_break()`. The sources do not say how often Blender's own engines call it, nor how long the gap is in practice. This page is written for engines supplied as add-ons; it does not state that Cycles, EEVEE or Workbench work this way.

Also on that page, in the example engine:

> "# When the render engine instance is destroy, this is called. Clean up any
> # render engine data here, for example stopping running render threads."
Source: the Python API documentation, https://docs.blender.org/api/5.1/bpy.types.RenderEngine.html — read 2026-07-30.
That is a comment telling engine authors where to stop their own threads. It says nothing about Blender's own engines or about timing.

**(e) What a render prints at all — and it is not tied to the stop.** — *documented*

> "bpy.app.handlers.render_stats
> on printing render statistics. Accepts one argument: the render stats (render/saving time plus in background mode frame/used [peak] memory)."
Source: the Python API documentation, https://docs.blender.org/api/5.1/bpy.app.handlers.html — read 2026-07-30.

This is the only statement I found in the four sources about a render printing anything. It says render statistics are printed, and it says what those statistics contain: render and saving time, and — only when Blender is run in background mode — the frame and the used and peak memory. It does not say that anything is printed between a stop request and the end, and it does not give the wording of a single line.

Printing can also be turned down or up from the command line:

> "-q, --quiet
> Suppress status printing (warnings & errors are still printed)."
> "--log <match>
> Enable logging categories, taking a single comma separated argument."
> "--log "render,cycles": both render and cycles messages."
Source: the manual, https://docs.blender.org/manual/en/5.1/advanced/command_line/arguments.html — read 2026-07-30.

So what appears on the console during any render depends on the arguments Blender was started with. The sources do not say what the "render" or "cycles" log categories emit, nor whether either says anything at cancel time.

**(f) One release note touches the cancelling interval directly, and it is about printing.** — *documented*
Under the heading "Render & Cycles: 52" in Blender 5.1's list of fixed issues:

> "Warnings about freeing memory after the leak detecter has run when cancelling a CLI render"
Source: the release notes, https://developer.blender.org/docs/release_notes/5.1/bugfixes/ — read 2026-07-30.

Documented: this was one of the issues fixed in 5.1.

Inferred, and the step is this: for such warnings to have appeared, cancelling a command-line render must involve memory being freed and a leak detector being run, and before the fix that ordering produced warnings printed to the console. Reading the note as a fix, those warnings no longer appear in 5.1. The note gives no wording, no timing, and no account of what else happens in that stretch. This is the closest the four sources come to describing what happens between the stop and the end, and it describes a bug that was removed rather than the behaviour that remains.

**(g) What I ruled out and why.** Cycles' Time Limit reads:

> "Stops rendering if the time exceeds the limit, even if the desired sample count hasn’t been reached yet. Setting this value to zero disables the limit."
Source: the manual, https://docs.blender.org/manual/en/5.1/render/cycles/render_settings/sampling.html — read 2026-07-30.

That is a render stopping *itself* at a limit it was given, not a render being interrupted, so it is not an answer to this question. I note it because it is the nearest match to the words "stops rendering" anywhere in the 5.1 manual.

Likewise the manual's answer to "Why is Blender unresponsive during rendering?" (https://docs.blender.org/manual/en/5.1/render/cycles/gpu_rendering.html) is about the interface being sluggish *while* a render runs, which bears on how easily a stop can be asked for, not on what happens after it is asked. I ruled it out for that reason.

### Not covered

This answer does not settle any of the following, because the sources do not:

- How long the interval lasts, in seconds or in any other unit, for any engine.
- Whether the render finishes the sample, the tile, the view layer or the frame it is holding, or drops it.
- Whether denoising, compositing, sequencer post-processing or file saving still run for the frame in hand after the stop is asked.
- Whether anything already computed is flushed to disk during that interval.
- The exact text printed on the console during the interval, if any, in either background mode or the interface — including whether a "cancelled" line of any kind is printed.
- Whether a second interruption (a second Control-C, a second Esc) shortens the interval or forces an immediate exit.
- Whether the process's exit status differs from a completed render.
- Whether Cycles, EEVEE and Workbench differ from each other here. Cycles' own files in `Blender Sources/cycles_5.1` contain nothing on the subject at all (see the record below), so I cannot even separate Cycles from the rest.
- Whether the interval differs when the render is writing a video file rather than separate images.

### Outside the interface

Partly answered, and only at the edges.

Documented: Control-C is given "special handling" whose purpose is "to cancel a render", and that handling comes from Blender's signal handlers, which are present in Blender the application and absent in Blender built as a Python module (https://docs.blender.org/api/5.1/info_advanced_blender_as_bpy.html, read 2026-07-30). So a render driven from a command line can be asked to stop without anybody clicking anything, and a script that has imported `bpy` as a module cannot ask that way.

Documented: printing differs between the two settings. The render statistics carry "in background mode frame/used [peak] memory" (https://docs.blender.org/api/5.1/bpy.app.handlers.html, read 2026-07-30) — that is, extra items appear when Blender is run in background mode. And `-q, --quiet` suppresses status printing while leaving warnings and errors (https://docs.blender.org/manual/en/5.1/advanced/command_line/arguments.html, read 2026-07-30).

Documented: the release-note fix quoted at (f) is specifically about "cancelling a CLI render" — a command-line render, driven from outside the window.

Not said by the sources: what the interval consists of, how long it lasts, or what it prints, in either setting. The status bar with its progress bar and cancel button exists only inside the window; the sources do not describe any equivalent readout outside it.


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

The sources do not say. Nothing I found about stopping a render names an operating system, and nothing says the behaviour differs between them.

Two documented details are OS-specific and touch only where printed output can be *seen*, not what it is:

> "When Blender is started on a Microsoft Windows operating system, the Console Window (called the Command Prompt) is first created as a separate window on the desktop. The main Blender window will also appear and the Console Window will then be toggled off. To display the console again, go to Window ‣ Toggle System Console."
Source: the manual, https://docs.blender.org/manual/en/5.1/advanced/command_line/launch/windows.html — read 2026-07-30.

> "-con, --start-console
> Start with the console window open (ignored if -b is set), (Windows only)."
Source: the manual, https://docs.blender.org/manual/en/5.1/advanced/command_line/arguments.html — read 2026-07-30.

So on Windows, anything a render prints while stopping goes to a window that is hidden by default unless Blender was started with `-con` or the console is toggled on. The sources do not say this changes what is printed.


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

### Where I looked

**Whole-corpus searches** across `manual_5.1`, `api_5.1` and `release_notes_5.1` (all files, case-insensitive) for: `cancel`, `cancelled`, `canceled`, `abort`, `aborted`, `interrupt`, `Ctrl-C`, `Ctrl+C`, `SIGINT`, `signal`, `stop rendering`, `stops rendering`, `stopping`, `stopped`, `terminate`, `halt`, `kill`, `quit`, `exit code`, `Blender quit`, `render job`, `background job`, `Esc`, `escape`, `during rendering`, `while rendering`, `finish the current`, `will finish`, `responsive`, `unresponsive`, `freeze`, `clean up`, `cleanup`, `shut down`, `shutdown`, `flush`, `graceful`, `wind down`, `immediately`, `right away`, `at once`, `as soon as`, `progress`, `Remaining`, `statistics`, `console`, `print`, `Fra:`, `Time:`, `Saved:`, `Render Window`.

`SIGINT`, `Blender quit`, `Fra:`, `Saved:`, `stop rendering`, `finish the current`, `will finish`, `graceful`, `wind down`, `shutdown` and `right away` return nothing anywhere in the 5.1 sources.

**Pages I opened and read.**
- Manual: Rendering Animations (`render/output/animation.html`); Output Introduction (`render/output/introduction.html`); Render Introduction (`render/introduction.html`); Rendering From The Command Line (`advanced/command_line/render.html`); Command Line Arguments (`advanced/command_line/arguments.html`), all of the Logging, Debug, Format and Cycles sections; Command Line Launching — Windows (`advanced/command_line/launch/windows.html`); Status Bar (`interface/window_system/status_bar.html`); Topbar (`interface/window_system/topbar.html`, the Render menu and Lock Interface); Viewport Render (`editors/3dview/viewport_render.html`); Cycles GPU Rendering, the whole FAQ and Error Messages part (`render/cycles/gpu_rendering.html`); Cycles Sampling (`render/cycles/render_settings/sampling.html`); Default Keymap (`interface/keymap/blender_default.html`, the function-key list).
- Manual pictures: `_images/interface_window-system_status-bar_ui-center.png`, opened and described at (b) above. I checked the `_images` folder for any picture of a render window or a render-in-progress readout under names containing `render-window` or `render_window`; there are none in 5.1.
- API: `bpy.app.handlers` (whole page); `bpy.types.RenderEngine` (whole page, including the example engine, `test_break`, `end_result`, `update_stats`, `update_progress`, `render_frame_finish`); `info_advanced_blender_as_bpy` (whole page, including the Signal Handlers section); `info_gotchas_operators` (the operator-return discussion); `bpy.ops.render` (searched for cancel — the only hit is `view_cancel`, "Cancel showing the render view", which is about closing the render display, not stopping a render, so I ruled it out).
- Release notes: `index`, `rendering`, `cycles`, `core`, `pipeline_io` read in full; `bugfixes` searched in full and its "Render & Cycles: 52" section read; `corrective_releases`, `user_interface`, `sculpt`, `python_api` searched. The only entry touching the subject is the one quoted at (f).

**Cycles' own files.** I searched all of `Blender Sources/cycles_5.1` for `cancel`, `abort`, `interrupt` and `test_break`. Outside the licence texts, every hit is either the word "cancel" used in its mathematical sense inside kernel headers (for example `source/util/math_base.h`, `source/kernel/closure/bsdf_microfacet.h`, `source/kernel/light/area.h`) or two lines in `operators.py`: line 88, `return {'CANCELLED'}`, and line 97, `# TODO: support cancel and progress reports.`. Both sit inside `CYCLES_OT_denoise_animation`, the operator that denoises an already-rendered image sequence — line 88 is the operator giving up because an expected frame file is missing, and line 97 is a note that the denoiser cannot be cancelled. Neither concerns a render being interrupted, so I ruled both out. The Cycles files supplied here are the add-on's Python files plus kernel and utility headers; they contain no session, progress or interrupt handling of any kind, so they cannot answer this question.

**Ambiguity.** The question does not say how the render is interrupted, and the three documented ways (the status bar's cancel button, Esc, Control-C from outside the window) could behave differently from one another. I have answered it as covering all three, and reported separately for each where the sources say anything at all. The narrower reading — only Control-C on a command-line render — would give the same answer, with (f) as the single relevant fragment.

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

**The build.** Both machines ran the same build — Blender 5.1.2 (hash ec6e62d40fa9 built 2026-05-19 01:30:33) on the Mac, Blender 5.1.2 (hash ec6e62d40fa9 built 2026-05-19 02:18:37) on the Linux machine, the same hash — so a difference between the machines below is not a difference of build.

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

> `"/Applications/Blender 5.1.2.app/Contents/MacOS/Blender" -b --factory-startup ~/q15/q15_5.1.blend -s 1 -e 5 -o <an empty folder>/f_#### -F PNG -a`

On the Linux machine the same, with `/opt/blender/5.1.2/blender` and `/root/q15/q15_5.1.blend`.

---

### 1. Control-C typed eight seconds into the third frame

**On the Creator's Mac.** The gap was **12.60 seconds**. Immediately after the Control-C, and before anything
else:

> `^C`  
> `Sent an internal break event. Press ^C again to kill Blender`

Then nothing at all for the whole of the gap, until these came out together at the end of it:

> `01:01.788  render           | Fra: 3 | Mem: 6M | Sample 600/600`  
> `01:01.788  render           | Fra: 3 | Mem: 6M | Finished`  
> `01:01.788  render           | Fra: 3 | Mem: 6M | Cancelled`  
> `Blender quit`

The exit status was `1`. The output folder held `f_0001.png` and `f_0002.png` and nothing else: the frame
in hand left no file.

**On the rented Linux machine.** The gap was **6.60 seconds**. The same two lines came first:

> `^C`  
> `Sent an internal break event. Press ^C again to kill Blender`

and at the end of the gap:

> `01:21.756  render           | Fra: 3 | Remaining: 00:19.36 | Mem: 6M | Sample 257/600`  
> `01:21.756  render           | Fra: 3 | Remaining: 00:19.37 | Mem: 6M | Sample 0/600`  
> `01:21.756  render           | Fra: 3 | Remaining: 00:19.37 | Mem: 6M | Cancelled`  
> `Blender quit`

The exit status was `1`, and again only the first two frames were on disk.

**Did the two machines agree?** On the shape of it, yes: the same two lines at the start, a three-line
ending, the same exit status, the same two files on disk. On the length, no — 12.60 seconds against 6.60. And on
one thing they differed plainly: on the Mac the last sample line read `Sample 600/600` and was followed by
a `Finished` line; on the Linux machine it read `Sample 257/600`, there was no `Finished` line, and the
`Cancelled` line still carried a `Remaining:` figure.

### 2. Where in the frame the stop lands

The same command, with the Control-C typed at four different moments. The fourth run had the sample count
raised to 2400 by adding `--python-expr "import bpy; bpy.context.scene.cycles.samples = 2400"`.

| when the Control-C was typed | gap, Mac | gap, Linux machine |
|---|---|---|
| one second after the process started, during set-up | 19.94 s | 28.73 s |
| two seconds into the third frame | 18.54 s | 12.55 s |
| about half way through the third frame | 7.35 s | 18.15 s |
| two seconds into the third frame, at 2400 samples | 31.24 s | 12.62 s |

In the one-second run on the Mac the render went on to print `Sample 600/600` and `Finished` for frame 1
before `Cancelled`, and the gap of 19.94 seconds is the whole of that frame. In the 2400-sample run on the Mac
the last sample line before `Cancelled` read `Sample 1025/2400` — the same figure in all four versions —
with a `Remaining:` figure of three quarters of a minute or so still on it.

### 3. How many frames were still to come

Two runs, identical but for the frame range.

| frame range | gap, Mac | gap, Linux machine |
|---|---|---|
| `-s 1 -e 5` | 10.97 s | 6.88 s |
| `-s 1 -e 100` | 11.10 s | 6.79 s |

No line printed after the Control-C, in any of the four runs, mentioned a frame later than the one in hand.

### 4. A second Control-C, five seconds after the first

The first Control-C's own line says what a second one does: `Press ^C again to kill Blender`.

**On the Mac.** The process was still alive when the second Control-C was typed. What it printed, in order:

> `Blender killed`  
> `Error: Not freed memory blocks: 30344, total unfreed memory 13.381527 MB`

and it was gone in the same instant. Exit status: **exited normally with code 2**. The first two frames were on disk.

**On the Linux machine.** What it printed, in order:

> `Blender killed`  
> `Error: Not freed memory blocks: 30011, total unfreed memory 13.395760 MB`

Exit status: **exited normally with code 2**.

No sentence about freeing memory after a leak detector appeared on either machine. In 5.0 on the rented Linux machine, the same test printed such a sentence sixteen times.

### 5. A render with no terminal at all, stopped by a signal

Started detached, in its own session, with no terminal attached and its printing going to a plain file,
then sent one signal from outside eight seconds into the third frame.

| signal | gap, Mac | gap, Linux machine |
|---|---|---|
| a plain termination request (`kill`) | 0.03 s | 0.04 s |
| an unignorable kill (`kill -9`) | 0.03 s | 0.04 s |
| an interrupt (`kill -INT`) | 12.13 s | 7.05 s |

After a plain termination request, and after an unignorable kill, nothing whatever was added to the
printing file: its last line is an ordinary progress line for the frame in hand. After an interrupt

> *Corrected by the citation check, 2026-08-26: this used to say the last line was "from the middle of
> the frame". It is a line for the frame in hand, but it was printed near that frame's beginning — in a
> repeat on the Creator's Mac in all four versions the last line of the file reads `Sample 1/600`, because
> Blender prints a sample line only at the boundaries of Cycles' sample batches and the signal arrived
> before the next one. Nothing else about this entry changed: nothing whatever is added to the file.*

the file ends exactly as entry 1's runs did — the sample line, the `Cancelled` line, `Blender quit`.

### 6. A Control-C landing between frames

The Control-C was typed the instant a `Saved:` line appeared. The gap was **0.04 seconds** on the Mac and
**0.08 seconds** on the Linux machine. Everything printed between the Control-C and the process being gone,
on the Mac:

> `^C`  
> `Sent an internal break event. Press ^C again to kill Blender`  
> `00:19.251  render           | Start rendering: Scene, ViewLayer`  
> `00:19.251  render           | Engine: Cycles`  
> `00:19.252  render           | Fra: 2 | Mem: 0M | Initializing`  
> `Blender quit`

### 7. `-q` added

Entry 1's command with `-q` added straight after `-b`, and the Control-C typed fifty seconds in. Gap:
**9.38 seconds** on the Mac, **13.07 seconds** on the Linux machine.

The three render lines were still printed on both machines — the sample line, `Finished` or `Sample 0/600`, and `Cancelled` — but `Blender quit` was not.

The line `Sent an internal break event. Press ^C again to kill Blender` was printed under `-q` in every
version on both machines.

### 8. The fullest logging

**What was typed.** Entry 1's command with `--log "*" --log-level debug` added straight after `-b`.
Both machines accepted it.

**Gap.** **11.10 seconds** on the Mac, **6.60 seconds** on the Linux machine.

**What it added between the Control-C and the end.** A block of `cycles`-category lines the ordinary run
does not print. From the Mac's run, in order:

> `00:57.337  cycles           | Rendered 599 samples in 19.0246 seconds (0.0317607 seconds per sample), occupancy: 1`  
> `00:57.337  cycles           | Measured path tracing occupancy: 1`  
> `00:57.337  cycles           | Write tile result.`  
> `00:57.337  cycles           | Write tile result via buffer write callback.`  
> `00:57.337  cycles           | Invoke buffer update callback.`  
> `00:57.337  cycles           | Average display update time: 5.96046e-06 seconds.`  
> `00:57.337  render           | Fra: 3 | Mem: 6M | Sample 600/600`  
> `00:57.337  cycles           | Rendering in main loop is done in 19.0588 seconds.`  
> `00:57.337  cycles           | Full path tracing report:`

then a block of twenty-five further lines under that heading — the device it path-traced on
(`Path tracing on: Apple M1 Pro (CPU) (4 threads) [CPU]` and `Hardware Ray-Tracing: Off`), a
"Render Scheduler Summary" giving `Mode: Headless` and `Resolution: 320x180`, whether adaptive sampling
and the denoiser were used, a "Time (in seconds)" table, and
`Rendered 600 samples in 19.058755 seconds` — and then:

> `00:57.337  render           | Fra: 3 | Mem: 6M | Finished`  
> `00:57.337  render           | Fra: 3 | Mem: 6M | Cancelled`  
> `00:57.337  cycles           | Total render time: 19.0632`  
> `00:57.337  cycles           | Render time (without synchronization): 19.0589`

No line reading `Rendering was canceled.` appeared anywhere in this version's run, on either machine. In
5.2 on the Mac, the same test printed exactly that line, in this same place, and printed no
"Full path tracing report" block at all.



### 9. `--debug-memory` added

Gap: **11.10 seconds** on the Mac, **6.55 seconds** on the Linux machine. Nothing about memory, blocks, leaks or
freeing appeared anywhere in either run — not before the Control-C and not after it. The lines printed
between the Control-C and the end were the same as entry 1's.

### 10. `--disable-abort-handler` added

Gap: **11.05 seconds** on the Mac, **6.48 seconds** on the Linux machine. The lines printed between the
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

> `Mem: 6M | Cancelled`

and on the Linux machine:

> `Remaining: 00:19.29 | Mem: 6M | Cancelled`

Gap: 11.09 seconds on the Mac, 6.56 seconds on the Linux machine.

### 12. The graphics card against the processor

`--cycles-device` does exist in this version. Its `--help` entry reads `--cycles-device <device>` with the
example `# blender -b file.blend -f 20 -- --cycles-device OPTIX`.

| run | gap, Mac (`METAL`) | gap, Linux machine (`OPTIX`) |
|---|---|---|
| on the processor, `-- --cycles-device CPU`, 600 samples | 11.16 s | 6.72 s |
| on the card, 600 samples | 1.27 s | the render finished all five frames before the stop could be asked for |
| on the card, 20000 samples | 26.38 s | 27.12 s |

On the Mac's 600-sample card run the last line before `Cancelled` was

> `01:38.155  render           | Fra: 5 | Remaining: 00:01.01 | Mem: 1390M | Sample 420/600 (Using optimized kernels)`

On the Linux machine the 600-sample card run finished normally, with five `Saved:` lines and exit status
`0`; that is recorded as the result it is, and the run was then repeated with the sample count raised to
20000, which is the third row of the table.

### 13. A different engine

`blender --factory-startup -E help` in this version lists `BLENDER_EEVEE`, `BLENDER_WORKBENCH` and `CYCLES`. Both runs
used a frame range of 1 to 500 at 1920 by 1080, with the Control-C typed twenty seconds in.

| engine | gap, Mac | gap, Linux machine |
|---|---|---|
| `BLENDER_EEVEE` | 1.12 s | 68.04 s |
| `BLENDER_WORKBENCH` | 0.06 s | 0.20 s |

The rented Linux machine's viewport is drawn by its processor rather than by its card. Its EEVEE frames
took about ninety seconds each there against a fraction of a second on the Mac, so the two EEVEE figures
are not a comparison of anything and are not offered as one.

### 14. One video file instead of separate images

The output was set to an FFmpeg MPEG-4 H.264 video with
`--python-expr "import bpy; s=bpy.context.scene; s.render.image_settings.file_format='FFMPEG'; s.render.ffmpeg.format='MPEG4'; s.render.ffmpeg.codec='H264'"`.
Gap: **11.06 seconds** on the Mac and **6.60 seconds** on the Linux machine, against entry 1's 12.60 and 6.60 for the
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

> `Frame:3 | Last:00:34.15 Time:00:13.41 | Remaining: 00:21.11 |`

This version was the only one of the four in which the progress figure moved after the key press: it read `38%` in the photograph taken immediately before, and `44%` in the one taken at the instant of the press and again one second later, where it stood still.

> *Disputed by the citation check, 2026-08-26, and left standing rather than replaced.* A repeat of this
> test on the same machine that day found no such movement: the photograph taken immediately before the
> key press and the one taken at the instant of the press were byte-for-byte the same file, so nothing on
> the screen changed between them. The same repeat also put this version's settling between 0.0 and 1.0
> seconds after the key press rather than between 1.0 and 2.1. Both runs are recorded; neither is picked
> over the other. This is on the citation check's list to be run again.
>
> *Run again on 2026-08-29 by a third machine, and both counts came back the citation check's way: the two
> photographs were the same file, at `42%` rather than `38%`, and the screen settled between 0.0 and 1.0
> seconds. The full run is under "Observed — the citation check's re-runs" below. Two runs to one; the
> record above is kept and is the weaker of the two.*

Comparing each photograph against the one before it, the screen settled into a picture that did not change
again for the rest of the forty seconds, and it settled **between 1.0 and 2.1 seconds after the key press**. In the settled picture the status bar is empty — no
task, no words, no progress bar, no X — the render window's header reads

> `Frame:3 | Time:00:17.06 | Mem:6M, Peak: 6M`

with no `Remaining:` figure on it, and the timeline's current frame has gone back to 1.

**The Mac's half of this test could not be run**, for the reason given at the end of this section.

### 16. How long the render job reports itself as running

The same set-up as entry 15, with a timer printing `bpy.app.is_job_running('RENDER')` every tenth of a
second. Measured against the moment of the Escape, the last answer of `True` came **1.26 seconds** after it
and the first answer of `False` came **1.44 seconds** after it. The call did not crash Blender in this
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
Blender 5.1.2 (hash `ec6e62d40fa9` built 2026-05-19 02:18:37) — the same hash both machines above ran, so nothing below is a difference of build. The scene and the harness were the same ones the 2026-08-26 runs used, copied
across unchanged. One frame took about 36 seconds in a background render and about 34 seconds in a render
started inside Blender's own window.

### Escape inside Blender's own window, run again

Entry 15 above is recorded two ways on two counts, and the citation check sent both to a fresh run. The same
test was run again on the third machine's display — Blender opened with a window on the scene file, the
animation render started by a timer, and `xdotool key --clearmodifiers Escape` sent fifteen seconds into the
third frame after raising the window, with the screen photographed immediately before, at the instant of the
press, and once a second for forty seconds.

**Nothing was printed after the Escape**, as before.

**The first count — whether the progress figure moved. It did not.** The photograph taken immediately before
the key press and the one taken at the instant of the press are **the same file**:

> `c89338fa6d8de173a31bc670bc0f610b  01.png`
> `c89338fa6d8de173a31bc670bc0f610b  02.png`

Both show the status bar carrying a task icon, the words `Rendering...`, a progress bar reading **`42%`**,
and an X at its right end, and the render window's header reading
`Frame:3 | Last:00:33.28 Time:00:14.37 | Remaining: 00:19.17 |`.

**The second count — when the screen settled. Between 0.0 and 1.0 seconds after the key press.** The
photograph at 0.0 seconds is the last one that differs; every one of the thirty-nine from 1.0 seconds
onwards is the same file. In the settled picture the status bar is empty — no task, no words, no progress
bar, no X — the render window's header reads `Frame:3 | Time:00:16.19 | Mem:6M, Peak: 6M` with no
`Remaining:` figure, and the timeline's current frame has gone back to 1.

**Both counts go the citation check's way, and both records stand.** Entry 15 above records the figure
moving from `38%` to `44%` and the screen settling between 1.0 and 2.1 seconds. The citation check's repeat
and this third run each found no movement and a settling between 0.0 and 1.0 seconds. That is two runs
against one, on two different machines, and the no-movement half rests on file fingerprints rather than on
reading a figure off a picture, which is the stronger kind of evidence. The earlier record is kept where it
stands and is not deleted.

### What a click on the status bar's cancel button does

Run on the third machine's display, in this version. Blender was opened with a window on the scene file, a
timer started the animation render five seconds later over frames 1 to 5, and fifteen seconds into the third
frame the screen was photographed, every window on it was listed, the main window was raised, the pointer
was moved to **x=1069, y=1068** — where the 2026-08-26 photographs put the cancel X — the screen was
photographed again with the pointer sitting there, and then one click was made. The screen was photographed
every second for forty seconds afterwards.

**Done that way it did not stop the render, which is the sixth time that has happened.** The click landed at
`[+0089.015]` and `f_0003.png` was written at `[+0105.851]`, 16.8 seconds later. The run was still rendering when the
forty seconds of photographs ran out.

**Nothing was over the button.** The window list taken seconds before the click holds two windows and no
more:

> `window 4194306  name='q15_5.1 [~/q15/q15_5.1.blend] - Blender 5.1.2'  geometry='WINDOW=4194306 X=0 Y=38 WIDTH=1920 HEIGHT=1061 SCREEN=0'`
> `window 4194352  name='Blender Render'  geometry='WINDOW=4194352 X=54 Y=812 WIDTH=351 HEIGHT=256 SCREEN=0'`

Neither covers 1069,1068, and the pointer arrived where it was sent —
`xdotool getmouselocation --shell says: X=1069 Y=1068`.

**The click did not reach the button, and this is how that was established.** The button did not react to
the pointer at all. Comparing the photograph taken with the pointer elsewhere against the one taken with the
pointer resting on the cancel X, a 30 by 24 box around that X is **identical — nought pixels different**. A
control that a pointer is sitting on and which does not change is a control the pointer is not reaching.

**What is over it is not a window but Blender's own Quick Setup panel.** `--factory-startup` opened that
panel, and it is drawn inside the main window rather than as a window of its own, so no window list can show
it. It takes the first click made anywhere on the screen. The photograph taken immediately before the click
shows the panel sitting in the middle of the screen; the one taken a second after the click shows it gone,
with the tooltip **`Stop this job.`** showing under the pointer — so the click was spent closing the panel,
and the pointer had been on the button all along.

**With one click spent on the panel first, the same click at the same place stopped the render.** The run
was repeated with `xdotool mousemove 1600 1068 click 1` on an empty part of the status bar at `[+0088.512]`,
the pointer moved to 1069,1068 at `[+0090.614]`, and the click made at `[+0091.189]`. **`f_0003.png` was never
written.** This time the button did react to the pointer: **399 pixels** changed in the same 30 by 24 box
when the pointer arrived, and the X lit up and the tooltip `Stop this job.` was showing beside it. The screen then settled between 1.0 and 2.0 seconds after the click, and in the settled
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

**The Blender.** Blender 5.1.2, the official Windows x64 build taken from Blender's own download server
and unpacked to `C:\blender\5.1.2`. Its hash as the running program reports it is `ec6e62d40fa9`, built
2026-05-19 01:37:34 — **the same hash the Creator's Mac and the Linux machines above ran**, so nothing
below is a difference of build.

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
`C:\blender\5.1.2\blender.exe`, this version's own scene file, and `--after "Fra: 3"` with the space, as on the other
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

2026-08-26 — Claude Opus 5 (Claude Code subagent) — Frames, Blender 5.1.

2026-08-26 — Claude Opus 5 (Claude Code) — hand tests for Frames, run on both machines and written back into this document.

2026-08-29 — Claude Opus 5 (Claude Code) — the citation check's re-runs written into this document: the third machine's repeat of the Escape test, the cancel-button result, and the macOS refusal checked again.

2026-08-30 — Claude Opus 5 (Claude Code) — the Windows machine written into this document: the machine and the edition it reported, entry 1 started in this version and killed with the machine, what was not run, and the three harness traps.

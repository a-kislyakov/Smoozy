# Frames — Blender 5.0 — question 15

**Sources consulted, and when.** Blender's manual for 5.0, Blender's Python API documentation for 5.0, the 5.0 release notes, and Cycles' own files as shipped with 5.0 — all as the copies held in `Blender Sources/`, taken on 2026-07-30. That snapshot date is the date read for everything below. One picture from the manual was opened and is named where it is used.

I read only the question sheet and files inside Blender Sources/. I did not open any other part of this project.

I did not start any helper agents. Everything below is my own reading.

---

## 15. When a render working through a range of frames is interrupted, what does it do between the moment it is asked to stop and the moment it ends — how long does that take, what settles that, and what does it print in the meantime?

**How I read the question.** I took "interrupted" to mean a stop asked for from outside the render while it is running — a person pressing something, or another program asking — because the question itself speaks of "the moment it is asked to stop". It could also be read as the machine going down, but a machine going down has no "moment it is asked". I answered the first reading. The searches recorded below were all aimed at a stop asked for; none of them was aimed at a machine going down, so nothing here settles the second reading either way.

**LABEL: not in the sources** for the question as asked — the sources nowhere describe the stretch between a stop being asked for and a render ending, put any length on it, say what settles the length, or say what is written out during it.

Several things around the edges *are* documented, and I set them out below, because together they are as close as the sources come. Each carries its own label.

---

### What the sources do say

**1. There is such a thing as asking a render to stop, and it is treated as a thing separate from the render finishing.** — documented

> "For example, when rendering animations to image file formats the render job can be canceled
> and resumed at the last rendered frame by changing the frame range."

Manual, Render Output ‣ Introduction — https://docs.blender.org/manual/en/5.0/render/output/introduction.html — read 2026-07-30.

The Python API keeps a separate hook for it, distinct from the one for finishing:

> "bpy.app.handlers.render_cancel
>
> on canceling a render job. Accepts one argument: the scene data-block being rendered"

> "bpy.app.handlers.render_complete
>
> on completion of render job. Accepts one argument: the scene data-block being rendered"

API documentation, Application Handlers — https://docs.blender.org/api/5.0/bpy.app.handlers.html — read 2026-07-30.

What is *not* there: neither the manual nor the API says when `render_cancel` fires relative to the request, nor what happens between the two.

**2. Inside Blender's own window, a running task can be stopped by a cancel button, and one page says a render can be stopped with Esc.** — documented

> "The task can be aborted by clicking the cancel button ()."

Manual, Interface ‣ Window System ‣ Status Bar — https://docs.blender.org/manual/en/5.0/interface/window_system/status_bar.html — read 2026-07-30. The same page says of the same strip of the window: "Shows the progress of the currently running task (such as rendering or baking). Hovering the mouse pointer over the progress bar will display a time estimate."

The picture on that page (https://docs.blender.org/manual/en/5.0/interface/window_system/status_bar.html — read 2026-07-30), `_images/interface_window-system_status-bar_ui-center.png`, shows what that strip holds while a job runs: a small icon, the words "Sequence Render", a bar filled most of the way across with "94%" written in it, and an X button at the right end. That is what is visible; it is a picture of a job running, not of one being stopped, and it shows nothing about what happens after the X is clicked.

> "As with a normal render, you can abort it with Esc."

Manual, Editors ‣ 3D Viewport ‣ Viewport Render — https://docs.blender.org/manual/en/5.0/editors/3dview/viewport_render.html — read 2026-07-30. This is the only place the searches below turned up that says Esc stops a normal render, and it says it in passing while describing something else. The word `Esc` was not itself searched, so this is a silence met along the way rather than one swept for.

**3. The one mechanism the sources name that would settle how long a stop takes.** — documented that the call exists and what it is for; **inferred** for what that means about the delay

> "test_break()
>
> Test if the render operation should been canceled, this is a fast call that should be used regularly for responsiveness"

API documentation, `bpy.types.RenderEngine` — https://docs.blender.org/api/5.0/bpy.types.RenderEngine.html — read 2026-07-30.

The step I am taking, written out: the wording says a render engine has to *ask* whether it should stop, and is told to ask often so that stopping feels prompt. From that it follows that a stop is not instant — it waits until the engine next asks. There is a second assumption inside that step, and it should be named: the page quoted sets out the calls a render engine written in Python is given, and no source says that Blender's own engines go through this one. So the step also assumes they do. The source does not say how often any engine asks, does not give any number, and does not say what the engine does after it learns the answer is yes. So this names the thing that settles the delay in kind, and settles nothing about its size.

The same page shows that handing back a part-finished result has its own separate path:

> "end_result(result, *, cancel=False, highlight=False, do_merge_results=False)"
>
> "cancel (boolean, (optional)) – Cancel, Don't mark tile as done, don't merge results unless forced"
>
> "do_merge_results (boolean, (optional)) – Merge Results, Merge results even if cancel=true"

Same address, same date. This is a choice offered to an engine, not a description of what Blender's own engines choose. What Cycles or EEVEE actually do with it is not in the sources.

**4. What a render prints while it runs.** — documented, but for a running render, not for one being stopped

The 5.0 release notes describe the printing as reworked, and give a sample:

> "Most of Blender now uses the same logging system,
> including operators reports and background render progress.
> The output format has changed, and log levels are easier to control."

and the sample lines for a render:

> "00:00.244  render           | Rendering single frame (frame 1)
> 00:00.245  render           | Start rendering: Scene, ViewLayer
> 00:00.245  render           | Engine: Cycles
> 00:26.799  render           | Mem: 2983M | Sample 1/16
> 00:26.799  render           | Mem: 2983M | Sample 16/16
> 00:28.631  render           | Saved: '/tmp/0001.png'
> 00:28.631  render           | Time: 00:28.38 (Saving: 00:00.15)"

Release notes, Core — https://developer.blender.org/docs/release_notes/5.0/core/ — read 2026-07-30.

The same section names the switches that control it: "`--log-level info` enables all info level log messages, with a moderate amount of output. This may be useful to enable in production for troubleshooting issues." and records that "`--debug-cycles`" became "`--log cycles`", "`--debug-ffmpeg`" became "`--log video`", and "`--verbose`" became "`--log-level [info|debug|trace]`".

The manual lists switches for the same printing, including one that turns it off:

> "-q, --quiet
>
> Suppress status printing (warnings & errors are still printed)."

and, for choosing what is printed:

> "--log \"render,cycles\": both render and cycles messages."

Manual, Command Line Arguments — https://docs.blender.org/manual/en/5.0/advanced/command_line/arguments.html — read 2026-07-30. The same page also lists `--log-file <filepath>`: "Set a file to output the log to."

There is a Python hook tied to this printing:

> "bpy.app.handlers.render_stats
>
> on printing render statistics. Accepts one argument: the render stats (render/saving time plus in background mode frame/used [peak] memory)."

API documentation, Application Handlers — https://docs.blender.org/api/5.0/bpy.app.handlers.html — read 2026-07-30.

None of this describes a stop. The sample run above is a run that finished. Nothing the searches below reached shows a line printed when a stop is asked for, a line printed while one is being carried out, or a line printed at the end of a stopped render. That part is **not in the sources** — with one limit on how far that silence has been established: the searches below were aimed at the words for stopping, and not at the words for printing, so this is a silence met while looking for something else rather than one swept for.

**5. One documented statement that bears on how a stop is asked for from outside the window.** — documented for what it says; **inferred** for what it implies about ordinary Blender

> "Blender's typical signal handlers are not initialized, so there is no special handling for Control-C
> to cancel a render and a crash log is not written in the event of a crash."

API documentation, Blender as a Python Module, under the heading "Signal Handlers" — https://docs.blender.org/api/5.0/info_advanced_blender_as_bpy.html — read 2026-07-30.

What it says outright: when Blender is used as a Python module, Control-C has no special handling for cancelling a render. The step I am taking, written out: it says the module lacks Blender's "typical" signal handlers, and names cancelling a render by Control-C as one of the things those typical handlers do — so ordinary Blender does have special handling for Control-C to cancel a render. That is an inference from a sentence written to describe the opposite case, and it is the only mention of Control-C anywhere in the 5.0 sources. It says nothing at all about what that handling then does, or how long it takes.

**6. One change listed in 5.0 that touches this exact stretch.** — documented for the line itself; **inferred** for what the word "Freeze" says about the stretch

Under "Platforms & Builds", among the issues fixed in 5.0:

> " * Linux Freeze when force cancelling a CLI render"

Release notes, Bugfixes — https://developer.blender.org/docs/release_notes/5.0/bugfixes/ — read 2026-07-30. The line carries a link to the commit and, unlike many neighbouring lines, no "Backported to" note.

What the line says outright: something called force cancelling a command-line render exists, something called a freeze happened with it on Linux, and the line sits among the issues listed as fixed in 5.0. The step I am taking, written out: reading "Freeze" as meaning the stretch between the request and the end could fail to end at all. The line does not say that — it gives the one word and no description of what was frozen or for how long. What it does not settle either way: what "force cancelling" means, how it is done, what the behaviour is now, or how long it takes. The sources define none of that anywhere.

---

### Not covered

This answer does not settle:

- how long the stretch between the request and the end is, in any unit, for any engine;
- whether the frame in hand is finished, thrown away, or written part-done — the sources say nothing either way here (that is the subject of other questions on the sheet, and I did not find it answered for this one);
- whether anything at all is written to the log or the console between the request and the end, or what the last line of a stopped render looks like;
- what the process leaves behind as an exit code, or whether a stopped run is distinguishable from a finished one by anything a caller outside Blender can see;
- what a video file being written is doing during that stretch;
- what Blender's own two engines, Cycles and EEVEE, do with the `cancel` choice on `end_result`;
- what "force cancelling" is, as against ordinary cancelling.

---

### Outside the interface

Weakly, and mostly not.

Everything the sources say about *asking* a render to stop is tied to Blender's own window: a cancel button in the status bar, and Esc. The sources do not say whether either can be reached when Blender is driven from outside its own window, and they do not say what stands in their place. The one thing that reaches outside is the Control-C sentence in item 5 above, and it is written backwards — it describes a case where that handling is absent, and only implies that ordinary Blender has it. The 5.0 bugfix line in item 6 confirms that command-line renders do get cancelled and that something called "force cancelling" exists for them, but describes neither.

Two differences are documented rather than guessed. First, when Blender is used as a Python module rather than as a program, Control-C has no special handling for cancelling a render at all — quoted in item 5. Second, the printing described in item 4 is explicitly about "background render progress", and one of the sample fields, per the `render_stats` hook, is only present "in background mode".

Beyond that, the sources do not say whether the stretch between request and end behaves the same when Blender is driven from outside its own window. One limit on that silence: the searches below were never run on the word `background`, which is the word Blender's own quotes in item 4 use for a render run without its window. So this silence too was met while looking for the words for stopping, rather than swept for directly.


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

The sources say nothing that would make this hold or not hold generally, with one exception: the 5.0 bugfix line in item 6 names Linux specifically — "Linux Freeze when force cancelling a CLI render". So one thing about this stretch was named as a Linux problem and listed as fixed in 5.0. The line does not say whether the same thing happened on other operating systems, so it does not establish that Linux differed from them — only that Linux is the one named. Whether anything differs by operating system after that fix is not stated.

One other operating-system difference touches only where the printing goes, not what a stop does: on Windows the console is hidden by default and has to be brought back — "The main Blender window will also appear and the Console Window will then be toggled off. To display the console again, go to Window ‣ Toggle System Console." (Manual, Launching from the Command Line ‣ Windows — https://docs.blender.org/manual/en/5.0/advanced/command_line/launch/windows.html — read 2026-07-30.)

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

### Where I looked

Everything below was searched across the whole of `manual_5.0`, `api_5.0`, `release_notes_5.0` and `cycles_5.0` at once unless said otherwise. Searches were case-insensitive.

**Words searched, and what came back**

- `cancel` — 38 files in the manual, 22 in the API documentation, 5 lines in the release notes, and in Cycles' own files only arithmetic ("cancellation error", "cancels out") plus one comment. Every render-related hit is quoted above.
- `abort` — 12 hits across manual and API. Only three bear on rendering: the status bar cancel button, "As with a normal render, you can abort it with Esc" on the viewport render page, and `--disable-abort-handler`. Ruled out: `--disable-abort-handler` is listed under Debug Options beside `--disable-crash-handler`, its whole entry reads "Disable the abort handler.", and nothing in the sources connects it to stopping a render.
- `interrupt` — 4 files. The only render one is Rendering Animations, and it is about a person's intent, not the mechanism: "May need to interrupt rendering to use the computer, and want to be able to resume rendering where you left off." (https://docs.blender.org/manual/en/5.0/render/output/animation.html)
- `Control-C`, `control c`, `Ctrl-C`, `Ctrl+C`, `SIGINT`, `SIGTERM`, `signal` — one relevant hit in total, the Signal Handlers passage quoted in item 5. Every `Ctrl-C` in the manual is copy-and-paste in an editor. This is the strongest part of the silence: there is no page anywhere in the 5.0 manual or API documentation about signals, about stopping a running Blender from outside, or about what a stop does.
- `graceful`, `terminate`, `force quit`, `kill`, `shut down`, `shutdown`, `wind down`, `immediately stop`, `stop immediately` — nothing about stopping a render. `terminate` is entirely about light paths and Freestyle chains.
- `freeze`, `hang`, `deadlock`, `quit`, `exit` across the release notes — surfaced the Linux CLI-render line in item 6, and nothing else on this subject.
- `responsiv` — surfaced `test_break` (item 3) and the Cycles GPU FAQ. Ruled out: "Why is Blender unresponsive during rendering?" (https://docs.blender.org/manual/en/5.0/render/cycles/gpu_rendering.html) is about a graphics card being unable to draw the window while it renders. It is about the window not redrawing, not about how quickly a render notices a stop, and it says nothing about stopping.
- `render job`, `rendering job` — four hits, all quoted above.
- Phrases: `stop rendering`, `stops rendering`, `stop the render`, `finish the current`, `current frame is finished`, `render is stopped`, `stopped render`, `partially rendered`, `incomplete` — nothing. The single `stops rendering` hit is the Cycles Time Limit setting. Ruled out: "Stops rendering if the time exceeds the limit, even if the desired sample count hasn't been reached yet." (https://docs.blender.org/manual/en/5.0/render/cycles/render_settings/sampling.html) is a limit the render imposes on itself, not a stop asked for from outside, and the question is about the latter.
- `remaining`, `time estimate`, `progress bar` across the manual's render, editors and interface sections — one hit, the status bar time estimate quoted above.
- `Fra:` — nothing. The old-style per-frame progress line is not shown anywhere in the 5.0 manual or API documentation; the sample in the 5.0 release notes (item 4) is the only sample of render output in any of the sources.

**Pages opened in full and read**

- Manual: Render ‣ Introduction; Render ‣ index; Render Output ‣ Introduction; Rendering Animations; Interface ‣ Status Bar; Interface ‣ Operators; Editors ‣ 3D Viewport ‣ Viewport Render; Topbar (the whole Render Menu); Command Line ‣ index; Command Line ‣ Arguments (Render Options, Logging Options and Debug Options in full); Rendering From The Command Line; Cycles ‣ GPU Rendering (the FAQ); Cycles ‣ Sampling (the top, for Time Limit); Troubleshooting ‣ Startup (for what the console prints); Launching from the Command Line ‣ Windows.
- API documentation: `bpy.app.handlers` (all render hooks); `bpy.types.RenderEngine` (the whole page, including the worked example and every method); `bpy.ops.render` (every operator); `bpy.types.Operator` (the `cancel` method); `bpy.types.WindowManager` (its `cancel` hits); Blender as a Python Module; Gotchas ‣ Operators.
- Release notes: `core.md` (Logging in full), `bugfixes.md` (the Platforms & Builds section and every `cancel` hit in the file), and a sweep of `rendering.md`, `cycles.md`, `eevee.md`, `python_api.md` and `compositor.md` for the words above.
- One picture: `_images/interface_window-system_status-bar_ui-center.png`, described in item 2.

**Ruled out and why**

- `bpy.ops.render.view_cancel()` — "Cancel show render view" (https://docs.blender.org/api/5.0/bpy.ops.render.html). This is about hiding the render window, not about stopping a render. The neighbouring `bpy.ops.render.view_show()` reads "Toggle show render view", which confirms the pair is about the window.
- `bpy.ops.render.render(...)` — its own description is "Undocumented, consider contributing." It lists what a render can be asked to do and nothing about stopping one.
- Cycles' own files — the copy in `cycles_5.0` holds the add-on's Python (`__init__.py`, `engine.py`, `operators.py`, `properties.py`, `ui.py`, `camera.py`, `osl.py`, `presets.py`, `version_update.py`), the OSL shaders, and `source/` which is the render kernel and maths headers only (269 files, all `kernel/` and `util/`). There is no session or job code in it. Searching all of it for `cancel`, `abort`, `interrupt` and `progress` returned only arithmetic comments and one unrelated note in `cycles_5.0/operators.py`, line 97: `# TODO: support cancel and progress reports.` — which sits immediately above the call to `_cycles.denoise` in the standalone denoise operator, and is about denoising files already on disk, not about rendering. So Cycles' files, which for other questions are the strongest source here, are silent on this one.
- The general rule about Esc — "Esc, RMB / Cancels a modal operator." (https://docs.blender.org/manual/en/5.0/interface/operators.html). I did not lean on this. The page is about modal operators such as moving an object, and nothing in the manual states that a render is one of them; the viewport render page's passing remark is better evidence and is quoted above.

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

**The build.** Both machines ran the same build — Blender 5.0.1 (hash a3db93c5b259 built 2025-12-16 01:51:40) on the Mac, Blender 5.0.1 (hash a3db93c5b259 built 2025-12-16 01:30:59) on the Linux machine, the same hash — so a difference between the machines below is not a difference of build.

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

> `"/Applications/Blender 5.0.1.app/Contents/MacOS/Blender" -b --factory-startup ~/q15/q15_5.0.blend -s 1 -e 5 -o <an empty folder>/f_#### -F PNG -a`

On the Linux machine the same, with `/opt/blender/5.0.1/blender` and `/root/q15/q15_5.0.blend`.

---

### 1. Control-C typed eight seconds into the third frame

**On the Creator's Mac.** The gap was **12.00 seconds**. Immediately after the Control-C, and before anything
else:

> `^C`  
> `Sent an internal break event. Press ^C again to kill Blender`

Then nothing at all for the whole of the gap, until these came out together at the end of it:

> `01:01.014  render           | Fra: 3 | Mem: 6M | Sample 600/600`  
> `01:01.014  render           | Fra: 3 | Mem: 6M | Finished`  
> `01:01.014  render           | Fra: 3 | Mem: 6M | Cancelled`  
> `Blender quit`

The exit status was `1`. The output folder held `f_0001.png` and `f_0002.png` and nothing else: the frame
in hand left no file.

**On the rented Linux machine.** The gap was **8.81 seconds**. The same two lines came first:

> `^C`  
> `Sent an internal break event. Press ^C again to kill Blender`

and at the end of the gap:

> `01:35.736  render           | Fra: 3 | Remaining: 00:22.30 | Mem: 6M | Sample 257/600`  
> `01:35.737  render           | Fra: 3 | Remaining: 00:22.30 | Mem: 6M | Sample 0/600`  
> `01:35.737  render           | Fra: 3 | Remaining: 00:22.30 | Mem: 6M | Cancelled`  
> `Blender quit`

The exit status was `1`, and again only the first two frames were on disk.

**Did the two machines agree?** On the shape of it, yes: the same two lines at the start, a three-line
ending, the same exit status, the same two files on disk. On the length, no — 12.00 seconds against 8.81. And on
one thing they differed plainly: on the Mac the last sample line read `Sample 600/600` and was followed by
a `Finished` line; on the Linux machine it read `Sample 257/600`, there was no `Finished` line, and the
`Cancelled` line still carried a `Remaining:` figure.

### 2. Where in the frame the stop lands

The same command, with the Control-C typed at four different moments. The fourth run had the sample count
raised to 2400 by adding `--python-expr "import bpy; bpy.context.scene.cycles.samples = 2400"`.

| when the Control-C was typed | gap, Mac | gap, Linux machine |
|---|---|---|
| one second after the process started, during set-up | 19.67 s | 14.42 s |
| two seconds into the third frame | 17.85 s | 13.10 s |
| about half way through the third frame | 8.93 s | 4.04 s |
| two seconds into the third frame, at 2400 samples | 32.05 s | 13.38 s |

In the one-second run on the Mac the render went on to print `Sample 600/600` and `Finished` for frame 1
before `Cancelled`, and the gap of 19.67 seconds is the whole of that frame. In the 2400-sample run on the Mac
the last sample line before `Cancelled` read `Sample 1025/2400` — the same figure in all four versions —
with a `Remaining:` figure of three quarters of a minute or so still on it.

### 3. How many frames were still to come

Two runs, identical but for the frame range.

| frame range | gap, Mac | gap, Linux machine |
|---|---|---|
| `-s 1 -e 5` | 11.91 s | 7.09 s |
| `-s 1 -e 100` | 11.89 s | 7.10 s |

No line printed after the Control-C, in any of the four runs, mentioned a frame later than the one in hand.

### 4. A second Control-C, five seconds after the first

The first Control-C's own line says what a second one does: `Press ^C again to kill Blender`.

**On the Mac.** The process was still alive when the second Control-C was typed. What it printed, in order:

> `Blender killed`  
> `Error: Not freed memory blocks: 34740, total unfreed memory 14.596897 MB`  
> `Writing: /var/folders/n3/z3rfjt654rn1k1x6f_1gk4540000gn/T/q15_5.0.crash.txt`

and it was gone in the same instant. Exit status: **killed by signal 11**. The first two frames were on disk.

**On the Linux machine.** What it printed, in order:

> `Blender killed`  
> `Error: Not freed memory blocks: 34572, total unfreed memory 14.631191 MB`  
> `Freeing memory after the leak detector has run. This can happen when using static variables in C++ that are defined outside of functions. To fix this error, use the 'construct on first use' idiom.`

Exit status: **exited normally with code 2**.

That last sentence was printed sixteen times over, one straight after another. Of the eight runs of this test — four versions on two machines — it appeared in this one alone. It did not appear in this same version on the Mac, and it did not appear in 4.5, 5.1 or 5.2 on either machine.

### 5. A render with no terminal at all, stopped by a signal

Started detached, in its own session, with no terminal attached and its printing going to a plain file,
then sent one signal from outside eight seconds into the third frame.

| signal | gap, Mac | gap, Linux machine |
|---|---|---|
| a plain termination request (`kill`) | 0.03 s | 0.02 s |
| an unignorable kill (`kill -9`) | 0.03 s | 0.02 s |
| an interrupt (`kill -INT`) | 11.07 s | 6.80 s |

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
**0.07 seconds** on the Linux machine. Everything printed between the Control-C and the process being gone,
on the Mac:

> `^C`  
> `Sent an internal break event. Press ^C again to kill Blender`  
> `00:20.563  render           | Fra: 2 | Mem: 0M | Initializing`  
> `Blender quit`

### 7. `-q` added

Entry 1's command with `-q` added straight after `-b`, and the Control-C typed fifty seconds in. Gap:
**10.75 seconds** on the Mac, **0.91 seconds** on the Linux machine.

The three render lines were still printed on both machines — the sample line, `Finished` or `Sample 0/600`, and `Cancelled` — but `Blender quit` was not.

The line `Sent an internal break event. Press ^C again to kill Blender` was printed under `-q` in every
version on both machines.

### 8. The fullest logging

**What was typed.** Entry 1's command with `--log "*" --log-level debug` added straight after `-b`.
Both machines accepted it.

**Gap.** **10.43 seconds** on the Mac, **6.97 seconds** on the Linux machine.

**What it added between the Control-C and the end.** A block of `cycles`-category lines the ordinary run
does not print. From the Mac's run, in order:

> `00:56.668  cycles           | Rendered 599 samples in 18.3624 seconds (0.0306551 seconds per sample), occupancy: 1`  
> `00:56.668  cycles           | Measured path tracing occupancy: 1`  
> `00:56.668  cycles           | Write tile result.`  
> `00:56.668  cycles           | Write tile result via buffer write callback.`  
> `00:56.669  cycles           | Invoke buffer update callback.`  
> `00:56.669  cycles           | Average display update time: 2.86102e-06 seconds.`  
> `00:56.669  render           | Fra: 3 | Mem: 6M | Sample 600/600`  
> `00:56.669  cycles           | Rendering in main loop is done in 18.3946 seconds.`  
> `00:56.669  cycles           | Full path tracing report:`

then a block of twenty-four further lines under that heading — the device it path-traced on
(`Path tracing on: Apple M1 Pro (CPU) (4 threads) [CPU]`), a "Render Scheduler Summary" giving
`Mode: Headless` and `Resolution: 320x180`, whether adaptive sampling and the denoiser were used, a
"Time (in seconds)" table, and `Rendered 600 samples in 18.394562 seconds` — and then:

> `00:56.669  render           | Fra: 3 | Mem: 6M | Finished`  
> `00:56.669  render           | Fra: 3 | Mem: 6M | Cancelled`  
> `00:56.669  cycles           | Total render time: 18.3987`  
> `00:56.669  cycles           | Render time (without synchronization): 18.3948`

No line reading `Rendering was canceled.` appeared anywhere in this version's run, on either machine. In
5.2 on the Mac, the same test printed exactly that line, in this same place, and printed no
"Full path tracing report" block at all.



### 9. `--debug-memory` added

Gap: **10.36 seconds** on the Mac, **6.97 seconds** on the Linux machine. Nothing about memory, blocks, leaks or
freeing appeared anywhere in either run — not before the Control-C and not after it. The lines printed
between the Control-C and the end were the same as entry 1's.

### 10. `--disable-abort-handler` added

Gap: **11.91 seconds** on the Mac, **7.15 seconds** on the Linux machine. The lines printed between the
Control-C and the end were the same as entry 1's, and so was the exit status.

### 11. What the handlers see

A Python file loaded with `--python` attached a printing function, with a timestamp, to `render_init`,
`render_pre`, `render_post`, `render_write`, `render_cancel`, `render_complete`, `render_stats` and
`exit_pre`, and reported any it could not find.

**Whether this version has `exit_pre`:** **it does not.** The run printed `HANDLER exit_pre         DOES NOT EXIST IN THIS VERSION`.

**What fired after the Control-C.** These, in this order, all within a thousandth of a second of each other,
at the very end of the gap and not before it: `render_stats`, `render_stats`, `render_stats`, `render_cancel`. `render_post`, `render_write` and `render_complete` did
not fire at all for the frame in hand. The order was the same on both machines.

**The exact string `render_stats` was handed** on its last call — the one alongside the `Cancelled` line —
on the Mac:

> `Mem: 6M | Cancelled`

and on the Linux machine:

> `Remaining: 00:21.05 | Mem: 6M | Cancelled`

Gap: 11.17 seconds on the Mac, 7.85 seconds on the Linux machine.

### 12. The graphics card against the processor

`--cycles-device` does exist in this version. Its `--help` entry reads `--cycles-device <device>` with the
example `# blender -b file.blend -f 20 -- --cycles-device OPTIX`.

| run | gap, Mac (`METAL`) | gap, Linux machine (`OPTIX`) |
|---|---|---|
| on the processor, `-- --cycles-device CPU`, 600 samples | 11.52 s | 8.62 s |
| on the card, 600 samples | 0.71 s | the render finished all five frames before the stop could be asked for |
| on the card, 20000 samples | 33.73 s | 28.63 s |

On the Mac's 600-sample card run the last line before `Cancelled` was

> `02:17.450  render           | Fra: 4 | Remaining: 00:01.11 | Mem: 1390M | Sample 458/600 (Using optimized kernels)`

On the Linux machine the 600-sample card run finished normally, with five `Saved:` lines and exit status
`0`; that is recorded as the result it is, and the run was then repeated with the sample count raised to
20000, which is the third row of the table.

### 13. A different engine

`blender --factory-startup -E help` in this version lists `BLENDER_EEVEE`, `BLENDER_WORKBENCH` and `CYCLES`. Both runs
used a frame range of 1 to 500 at 1920 by 1080, with the Control-C typed twenty seconds in.

| engine | gap, Mac | gap, Linux machine |
|---|---|---|
| `BLENDER_EEVEE` | 2.35 s | 66.37 s |
| `BLENDER_WORKBENCH` | 0.14 s | 0.16 s |

The rented Linux machine's viewport is drawn by its processor rather than by its card. Its EEVEE frames
took about ninety seconds each there against a fraction of a second on the Mac, so the two EEVEE figures
are not a comparison of anything and are not offered as one.

### 14. One video file instead of separate images

The output was set to an FFmpeg MPEG-4 H.264 video with
`--python-expr "import bpy; s=bpy.context.scene; s.render.image_settings.file_format='FFMPEG'; s.render.ffmpeg.format='MPEG4'; s.render.ffmpeg.codec='H264'"`.
Gap: **11.53 seconds** on the Mac and **22.00 seconds** on the Linux machine, against entry 1's 12.00 and 8.81 for the
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

> `Frame:3 | Last:00:35.77 Time:00:13.92 | Remaining: 00:21.91 |`



Comparing each photograph against the one before it, the screen settled into a picture that did not change
again for the rest of the forty seconds, and it settled **between 0.0 and 1.0 seconds after the key press**. In the settled picture the status bar is empty — no
task, no words, no progress bar, no X — the render window's header reads

> `Frame:3 | Time:00:15.82 | Mem:6M, Peak: 6M`

with no `Remaining:` figure on it, and the timeline's current frame has gone back to 1.

**The Mac's half of this test could not be run**, for the reason given at the end of this section.

### 16. How long the render job reports itself as running

The same set-up as entry 15, with a timer printing `bpy.app.is_job_running('RENDER')` every tenth of a
second. Measured against the moment of the Escape, the last answer of `True` came **-0.03 seconds** after it
and the first answer of `False` came **0.15 seconds** after it. The call did not crash Blender in this
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
Blender 5.0.1 (hash `a3db93c5b259` built 2025-12-16 01:30:59) — the same hash both machines above ran, so nothing below is a difference of build. The scene and the harness were the same ones the 2026-08-26 runs used, copied
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
`[+0092.769]` and `f_0003.png` was written at `[+0110.791]`, 18.0 seconds later. The run was still rendering when the
forty seconds of photographs ran out.

**Nothing was over the button.** The window list taken seconds before the click holds two windows and no
more:

> `window 4194306  name='q15_5.0 [~/q15/q15_5.0.blend] - Blender 5.0.1'  geometry='WINDOW=4194306 X=0 Y=38 WIDTH=1920 HEIGHT=1061 SCREEN=0'`
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
was repeated with `xdotool mousemove 1600 1068 click 1` on an empty part of the status bar at `[+0092.722]`,
the pointer moved to 1069,1068 at `[+0094.839]`, and the click made at `[+0095.420]`. **`f_0003.png` was never
written.** This time the button did react to the pointer: **665 pixels** changed in the same 30 by 24 box
when the pointer arrived, and the X lit up and the tooltip `Stop this job.` was showing beside it. The screen then settled between 0.0 and 1.0 seconds after the click, and in the settled
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

**The Blender.** Blender 5.0.1, the official Windows x64 build taken from Blender's own download server
and unpacked to `C:\blender\5.0.1`. Its hash as the running program reports it is `a3db93c5b259`, built
2025-12-16 01:32:30 — **the same hash the Creator's Mac and the Linux machines above ran**, so nothing
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
`C:\blender\5.0.1\blender.exe`, this version's own scene file, and `--after "Fra: 3"` with the space, as on the other
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

2026-08-25 — Claude Opus 5 (Claude Code subagent) — Frames, Blender 5.0.

2026-08-26 — Claude Opus 5 (Claude Code) — hand tests for Frames, run on both machines and written back into this document.

2026-08-29 — Claude Opus 5 (Claude Code) — the citation check's re-runs written into this document: the cancel-button result on a third machine, and the macOS refusal checked again.

2026-08-30 — Claude Opus 5 (Claude Code) — the Windows machine written into this document: the machine and the edition it reported, entry 1 started in this version and killed with the machine, what was not run, and the three harness traps.

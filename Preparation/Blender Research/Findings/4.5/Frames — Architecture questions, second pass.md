# Frames — Architecture questions, second pass (Blender 4.5)

**Sources consulted, and when.** Blender 4.5 manual (`Blender Sources/manual_4.5`, including its `_images` folder), Blender 4.5 Python API documentation (`Blender Sources/api_4.5`), Blender 4.5 release notes (`Blender Sources/release_notes_4.5`), and Cycles' own files as shipped with Blender 4.5 (`Blender Sources/cycles_4.5`). All four were taken on **2026-07-30**, and that is the date read for everything quoted below.

I read only the question sheet and files inside Blender Sources/. I did not open any other part of this project.

This pass answers question **15** only.

---

## 15. When a render working through a range of frames is interrupted, what does it do between the moment it is asked to stop and the moment it ends — how long does that take, what settles that, and what does it print in the meantime?

**LABEL: not in the sources** — for the question as asked. None of the four sources describes what a running render does after it is asked to stop, how long that interval lasts, what decides its length, or what it writes to the console or the interface while it is happening. Several nearby things *are* documented, and I set them out below with their own labels, because together they are the closest the sources come and they show where the silence begins.

**A note on one word in the question.** "Prints" is ambiguous. It can mean text sent to the console/terminal, or the status text and progress shown in Blender's own window. I searched for both, and both come back empty for the interval in question. Where the sources say anything at all about either, it is recorded below.

### What the sources do say, piece by piece

**(a) A stop is a request the render has to notice, not a switch that halts it.** This is the strongest thing I found.

- LABEL: **documented** (for a render engine written in Python, which is what this page describes)
- Exact words: `test_break()` … "Test if the render operation should been canceled, this is a fast call that should be used regularly for responsiveness"
- Source: Python API documentation, https://docs.blender.org/api/4.5/bpy.types.RenderEngine.html
- Date read: 2026-07-30

The page also documents what an engine may do with the work it had in hand when it stops:

- Exact words: `end_result(result, *, cancel=False, highlight=False, do_merge_results=False)` … "cancel (boolean, (optional)) – Cancel, Don’t mark tile as done, don’t merge results unless forced" and "do_merge_results (boolean, (optional)) – Merge Results, Merge results even if cancel=true"
- Source: Python API documentation, https://docs.blender.org/api/4.5/bpy.types.RenderEngine.html
- Date read: 2026-07-30

And it documents, in the worked example on the same page, that an engine's own teardown is where running render threads are stopped:

- Exact words: "# When the render engine instance is destroy, this is called. Clean up any render engine data here, for example stopping running render threads."
- Source: Python API documentation, https://docs.blender.org/api/4.5/bpy.types.RenderEngine.html
- Date read: 2026-07-30

What follows from these, and what does not: the phrase "used regularly for responsiveness" says plainly that the gap between asking and stopping is set by how often the running code checks. That the same is true of Cycles, EEVEE or Workbench is **inferred**, and the step is this — those engines are engines in the same sense as the documented one, so they too must notice a break rather than be halted; the sources do not say it, and they give no check interval for any of them. No page anywhere gives a number of seconds, a sample, a tile, or any other unit for how much work stands between the request and the end.

**(b) There is a distinct finishing step at the end of a frame.**

- LABEL: **documented**
- Exact words: `render_frame_finish()` — "Perform finishing operations after all view layers in a frame were rendered"
- Source: Python API documentation, https://docs.blender.org/api/4.5/bpy.types.RenderEngine.html
- Date read: 2026-07-30

Cycles has that step, and a session teardown, wired up in its own files:

- LABEL: **observed** (read from the files supplied, not run)
- Exact words: in `Blender Sources/cycles_4.5/__init__.py`, class `CyclesRender`: "    def render_frame_finish(self):" / "        engine.render_frame_finish(self)" and "    def __del__(self):" / "        engine.free(self)"; and in `Blender Sources/cycles_4.5/engine.py`: "def render_frame_finish(engine):" … "    _cycles.render_frame_finish(engine.session)" and "def free(engine):" … "            _cycles.free(engine.session)"
- Date read: 2026-07-30

Whether either step runs after a stop, and how long either takes, is not in these files — what they call into (`_cycles`) is compiled and is not part of what I was given.

**(c) Something is run at the end of a cancelled render, and it is a different something from a finished one.**

- LABEL: **documented**
- Exact words: "bpy.app.handlers.render_cancel" — "on canceling a render job. Accepts one argument: the scene datablock being rendered"; alongside "bpy.app.handlers.render_complete" — "on completion of render job. Accepts one argument: the scene datablock being rendered"
- Source: Python API documentation, https://docs.blender.org/api/4.5/bpy.app.handlers.html
- Date read: 2026-07-30

The page does not say when in the interval this fires, what else runs before or after it, or whether the frame in hand has been dealt with by then.

**(d) The one documented handle on "is it still going".**

- LABEL: **documented**
- Exact words: "static bpy.app.is_job_running(job_type)" — "Check whether a job of the given type is running." with "job_type (str) – job type in Wm Job Type Items"; and in Wm Job Type Items, "RENDER:" — "Regular rendering."
- Source: Python API documentation, https://docs.blender.org/api/4.5/bpy.app.html and https://docs.blender.org/api/4.5/bpy_types_enum_items/wm_job_type_items.html
- Date read: 2026-07-30

That a render still winding down would report as running is **inferred** and the sources do not confirm it; the step is that the job is what was asked to stop, and nothing says the job is marked finished before its wind-down is done.

**(e) What the interface shows — before the stop, not during it.**

- LABEL: **documented**
- Exact words: "Running Task" — "Shows the progress of the currently running task (such as rendering or baking). Hovering the mouse pointer over the progress bar will display a time estimate. The task can be aborted by clicking the cancel button ()."
- Source: manual, https://docs.blender.org/manual/en/4.5/interface/window_system/status_bar.html
- Date read: 2026-07-30

I opened the picture that page names, `_images/interface_window-system_status-bar_ui-center.png`. LABEL: **documented** (it is part of the manual). What is visible: a strip showing a small camera icon, the words "Sequence Render", a progress bar reading "94%", and an X button at its right end. That is all it shows. It does not show a stopping or cancelling state, and I take nothing from it about what appears after the X is pressed.

The API side of the status text is documented, but again only as a thing an engine may do at any time, with nothing said about stopping:

- Exact words: `update_stats(stats, info)` — "Update and signal to redraw render status text"; `update_progress(progress)` — "Update progress percentage of render"
- Source: Python API documentation, https://docs.blender.org/api/4.5/bpy.types.RenderEngine.html
- Date read: 2026-07-30

**(f) What is printed, in general.** Two documented statements, neither of which touches the stopping interval.

- LABEL: **documented**
- Exact words: "bpy.app.handlers.render_stats" — "on printing render statistics. Accepts one argument: the render stats (render/saving time plus in background mode frame/used [peak] memory)."
- Source: Python API documentation, https://docs.blender.org/api/4.5/bpy.app.handlers.html
- Date read: 2026-07-30

- LABEL: **documented**
- Exact words: "The Console Window, also called a Terminal, is an operating system text window that displays messages about Blender’s operations, status, and internal errors." and "When Blender is manually started from a terminal, Blender output is shown in the Console Window it is started from."
- Source: manual, https://docs.blender.org/manual/en/4.5/advanced/command_line/index.html
- Date read: 2026-07-30

Cycles ships one switch of its own about printing, and it says nothing about stopping either:

- LABEL: **observed** (read from the file supplied)
- Exact words: in `Blender Sources/cycles_4.5/engine.py`, inside `_configure_argument_parser()`, the three lines: `parser.add_argument("--cycles-print-stats",` / `help="Print rendering statistics to stderr",` / `action='store_true')` (the second and third lines are indented to line up under the first argument)
- Date read: 2026-07-30

Nowhere in the manual, the API documentation or the release notes did I find a sample of Blender's render console output — not one line of it, in any state, running or stopping.

**(g) Aborting is documented as possible; its aftermath is not.**

- LABEL: **documented**
- Exact words: "As with a normal render, you can abort it with Esc."
- Source: manual, https://docs.blender.org/manual/en/4.5/editors/3dview/viewport_render.html
- Date read: 2026-07-30

- LABEL: **documented**
- Exact words: "For example, when rendering animations to image file formats the render job can be canceled and resumed at the last rendered frame by changing the frame range."
- Source: manual, https://docs.blender.org/manual/en/4.5/render/output/introduction.html
- Date read: 2026-07-30

Both say a render can be stopped. Neither says anything about the interval between the asking and the ending.

### Not covered

My answer does not settle: how long the wind-down lasts, in any unit, for any engine; whether the frame in hand is abandoned mid-sample, mid-tile, or at some other boundary; whether the per-frame finishing step, compositing, or file writing runs after a stop; whether output files are closed in an orderly way during it; the order in which the cancel handler, the finishing step and the engine teardown happen; and every word of what is printed to console or status bar while it happens. It also does not settle whether the *release notes* changed any of this in 4.5 — I searched them and found nothing on the subject, which is not the same as their saying nothing changed. And (a) is documented for a render engine written in Python; that Cycles and EEVEE behave the same way is my inference, not the manual's statement.

### Outside the interface

The sources do not say what a render does between being asked to stop and ending when Blender is driven from outside its own window. What they do give is how the asking happens there, and that comes from an unexpected page — the one on using Blender as a Python module, which describes what is *missing* there:

- LABEL: **documented** (for the Python-module build)
- Exact words: "Signal Handlers" — "Blender’s typical signal handlers are not initialized, so there is no special handling for Control-C to cancel a render and a crash log is not written in the event of a crash."
- Source: Python API documentation, https://docs.blender.org/api/4.5/info_advanced_blender_as_bpy.html
- Date read: 2026-07-30

LABEL for the reading of it: **inferred**. The step is this — the page says these are Blender's *typical* signal handlers and that this build does not initialize them, so the ordinary Blender program does initialize them, and in the ordinary program Control-C is handled as a request to cancel a render rather than being left to the operating system. That, in turn, is a request in the sense of (a) above, so a wind-down interval exists for a command-line render too. How long it is, and what is printed during it, the sources do not say. This is also the only mention of Control-C in a render sense anywhere in the three documentation sets — I searched all three for "Control-C", "Ctrl-C", "SIGINT", "SIGTERM", "SIGKILL", "signal", "kill" and "terminate"; every other hit is copy-and-paste keys or unrelated wording, and SIGINT, SIGTERM and SIGKILL do not appear at all.

One difference from inside the window is documented plainly: the cancel button in (e) is part of the Status Bar at the bottom of the Blender window, so it is not available at all when there is no window. The sources do not name any other difference, and in particular say nothing about whether the handlers in (c), or the finishing step in (b), behave differently in background mode.


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

The sources do not say. Nothing I found about stopping a render is marked as applying to one operating system and not another, and nothing describes an operating-system difference in the interval. Two documented points touch the neighbouring matter of whether you can *see* what is printed, and both are Windows-only:

- Exact words: "-con, --start-console" — "Start with the console window open (ignored if -b is set), (Windows only)."
- Source: manual, https://docs.blender.org/manual/en/4.5/advanced/command_line/arguments.html
- Date read: 2026-07-30

- Exact words: "When Blender is started on a Microsoft Windows operating system, the Console Window (called the Command Prompt) is first created as a separate window on the desktop. The main Blender window will also appear and the Console Window will then be toggled off. To display the console again, go to Window ‣ Toggle System Console."
- Source: manual, https://docs.blender.org/manual/en/4.5/advanced/command_line/launch/windows.html
- Date read: 2026-07-30

That is about visibility of output, not about the stopping interval, and I claim nothing further from it.


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
desktop. One of the sixteen tests — the first, a Control-C part-way through a frame — was
run on it in this version and answered. **The other fifteen were never begun**, and the machine was
destroyed from outside the session with a run still going. **A result from that machine is a Windows
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

**How I searched.** All searches were run across whole folders at once — `manual_4.5`, `api_4.5`, `release_notes_4.5` and `cycles_4.5` — not file by file, and case-insensitively.

**Terms searched, in all four sources:** cancel, cancelled, canceled, canceling, abort, aborted, interrupt, stop, stopped, stopping, halt, break, test_break, Esc, escape, Ctrl-C, Control-C, SIGINT, SIGTERM, SIGKILL, signal, kill, terminate, quit, exit, shutdown, "shut down", "clean up", cleanup, teardown, graceful, flush, pending, "wind down", responsive/responsiveness, progress, job, jobs, remaining, elapsed, "time estimate", "render time", console, print, statistics, stats, "Fra:", "Mem:", "Peak:", "Saved:", "Append frame", "Blender quit", partially, "in the middle of".

**Pages I opened and read in full or in the relevant part, and what each ruled out:**

- https://docs.blender.org/manual/en/4.5/render/output/animation.html — the whole rendering-animations workflow page, including its "Hints" on a computer turning off mid-render. It is about what survives a stop and how to resume, never about the moments after asking to stop.
- https://docs.blender.org/manual/en/4.5/render/output/introduction.html — the one sentence about a render job being cancelled and resumed. Nothing further.
- https://docs.blender.org/manual/en/4.5/render/introduction.html — general overview of the three engines; nothing on stopping.
- https://docs.blender.org/manual/en/4.5/interface/window_system/status_bar.html and the image `_images/interface_window-system_status-bar_ui-center.png` — the cancel button exists; no aftermath described.
- https://docs.blender.org/manual/en/4.5/interface/window_system/topbar.html — Render menu (Render Image F12, Render Animation Ctrl-F12, Lock Interface). Nothing about stopping a render.
- https://docs.blender.org/manual/en/4.5/editors/3dview/viewport_render.html — "abort it with Esc"; nothing after that.
- https://docs.blender.org/manual/en/4.5/advanced/command_line/index.html, .../render.html, .../arguments.html, .../launch/windows.html — the whole command-line section, including every argument. There is no argument, exit code, or signal behaviour documented for stopping a render. `--disable-crash-handler` and `--disable-abort-handler` exist but are about crashes and internal errors, not a user stopping a render, so I ruled them out.
- https://docs.blender.org/manual/en/4.5/render/cycles/render_settings/sampling.html — read for Time Limit ("Stops rendering if the time exceeds the limit, even if the desired sample count hasn’t been reached yet"). Ruled out: that is Cycles stopping itself at a limit, not an interruption, and the page says nothing about how the stop is carried out.
- https://docs.blender.org/manual/en/4.5/render/cycles/render_settings/performance.html — threads, tiling, tile size. Nothing about stopping or about how often work units complete in a way the page ties to stopping.
- https://docs.blender.org/manual/en/4.5/render/cycles/gpu_rendering.html — read the whole FAQ, including "Why is Blender unresponsive during rendering?". Ruled out: it explains why the interface may not redraw during a render, and says nothing about a stop request or how quickly one is acted on.
- https://docs.blender.org/manual/en/4.5/render/output/properties/output.html — read for the Encoding panel's console tip ("When you view the System Console, you can see some of the output of the encoding process"). Ruled out: about encoding output generally, not about stopping.
- https://docs.blender.org/manual/en/4.5/interface/keymap/introduction.html — Esc listed only as the general "Cancel" key.
- https://docs.blender.org/api/4.5/bpy.types.RenderEngine.html — read the entire page, both worked examples, all `bl_use_*` flags and every method. This is where (a), (b) and (e) come from, and it is the only place any of it is stated.
- https://docs.blender.org/api/4.5/bpy.app.handlers.html — read the whole page, all handlers and the three notes at the top.
- https://docs.blender.org/api/4.5/bpy.app.html — `is_job_running`.
- https://docs.blender.org/api/4.5/bpy_types_enum_items/wm_job_type_items.html — the five job types.
- https://docs.blender.org/api/4.5/bpy.ops.render.html — every render operator. `bpy.ops.render.render` has no argument bearing on stopping, and the only cancel-named operator on the page is `bpy.ops.render.view_cancel()` — "Cancel show render view" — which is about the render window, not the render, so I ruled it out.
- https://docs.blender.org/api/4.5/bpy.types.Operator.html — `cancel(context)`, "Called when the operator is canceled". Ruled out: this is the general operator interface, and says nothing about renders or about timing.
- https://docs.blender.org/api/4.5/info_advanced_blender_as_bpy.html — the "Signal Handlers" section quoted above.
- https://docs.blender.org/api/4.5/info_gotchas_operators.html — searched for render; no hits.
- Release notes for 4.5, all nineteen files, searched for cancel, interrupt, abort, stop, responsive. I read https://developer.blender.org/docs/release_notes/4.5/rendering/, https://developer.blender.org/docs/release_notes/4.5/cycles/ and https://developer.blender.org/docs/release_notes/4.5/python_api/ in full, and searched https://developer.blender.org/docs/release_notes/4.5/bugfixes/ and .../core/. Nothing on the subject. The only near hits in the bug-fix list are an unrelated Escape-key fix in animation editing and an unrelated unresponsiveness fix with metaballs; both ruled out as nothing to do with rendering.
- Cycles' own files, all of them, searched for cancel, interrupt, abort, test_break and stop. I read `Blender Sources/cycles_4.5/__init__.py` and `Blender Sources/cycles_4.5/engine.py` in full. The only cancel-shaped hits in the whole folder are `return {'CANCELLED'}` and the comment "# TODO: support cancel and progress reports." in `operators.py`, both inside the standalone denoising operator and nothing to do with stopping a render; the rest are the word "cancel" used mathematically in kernel headers. I ruled all of these out. **The reason the Cycles files cannot answer this question is that what is here is the add-on's Python, its shader files, and the kernel and utility headers under `source/` — the session and progress code that would carry a cancel is compiled into `_cycles` and is not in the folder.**

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

**The build.** Both machines ran the same build — Blender 4.5.12 LTS (hash 84afd5f785f7 built 2026-07-21 01:41:35) on the Mac, Blender 4.5.12 LTS (hash 84afd5f785f7 built 2026-07-21 01:31:22) on the Linux machine, the same hash — so a difference between the machines below is not a difference of build.

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

> `"/Applications/Blender 4.5.12.app/Contents/MacOS/Blender" -b --factory-startup ~/q15/q15_4.5.blend -s 1 -e 5 -o <an empty folder>/f_#### -F PNG -a`

On the Linux machine the same, with `/opt/blender/4.5.12/blender` and `/root/q15/q15_4.5.blend`.

---

### 1. Control-C typed eight seconds into the third frame

**On the Creator's Mac.** The gap was **12.32 seconds**. Immediately after the Control-C, and before anything
else:

> `^C`  
> `Sent an internal break event. Press ^C again to kill Blender`

Then nothing at all for the whole of the gap, until these came out together at the end of it:

> `Fra:3 Mem:12.17M (Peak 13.05M) | Time:00:20.32 | Mem:1.92M, Peak:1.92M | Scene, ViewLayer | Sample 600/600`  
> `Fra:3 Mem:12.17M (Peak 13.05M) | Time:00:20.32 | Mem:1.92M, Peak:1.92M | Scene, ViewLayer | Finished`  
> `Fra:3 Mem:12.17M (Peak 13.05M) | Time:00:20.32 | Mem:1.92M, Peak:1.92M | Scene, ViewLayer | Cancelled`  
> `Blender quit`

The exit status was `1`. The output folder held `f_0001.png` and `f_0002.png` and nothing else: the frame
in hand left no file.

**On the rented Linux machine.** The gap was **8.09 seconds**. The same two lines came first:

> `^C`  
> `Sent an internal break event. Press ^C again to kill Blender`

and at the end of the gap:

> `Fra:3 Mem:11.34M (Peak 13.10M) | Time:00:16.00 | Remaining:00:21.34 | Mem:1.92M, Peak:1.92M | Scene, ViewLayer | Sample 257/600`  
> `Fra:3 Mem:12.22M (Peak 13.10M) | Time:00:16.00 | Remaining:00:21.34 | Mem:1.92M, Peak:1.92M | Scene, ViewLayer | Sample 0/600`  
> `Fra:3 Mem:12.22M (Peak 13.10M) | Time:00:16.00 | Remaining:00:21.34 | Mem:1.92M, Peak:1.92M | Scene, ViewLayer | Cancelled`  
> `Blender quit`

The exit status was `1`, and again only the first two frames were on disk.

**Did the two machines agree?** On the shape of it, yes: the same two lines at the start, a three-line
ending, the same exit status, the same two files on disk. On the length, no — 12.32 seconds against 8.09. And on
one thing they differed plainly: on the Mac the last sample line read `Sample 600/600` and was followed by
a `Finished` line; on the Linux machine it read `Sample 257/600`, there was no `Finished` line, and the
`Cancelled` line still carried a `Remaining:` figure.

### 2. Where in the frame the stop lands

The same command, with the Control-C typed at four different moments. The fourth run had the sample count
raised to 2400 by adding `--python-expr "import bpy; bpy.context.scene.cycles.samples = 2400"`.

| when the Control-C was typed | gap, Mac | gap, Linux machine |
|---|---|---|
| one second after the process started, during set-up | 20.48 s | 31.80 s |
| two seconds into the third frame | 18.90 s | 14.15 s |
| about half way through the third frame | 9.82 s | 5.16 s |
| two seconds into the third frame, at 2400 samples | 33.81 s | 14.05 s |

In the one-second run on the Mac the render went on to print `Sample 600/600` and `Finished` for frame 1
before `Cancelled`, and the gap of 20.48 seconds is the whole of that frame. In the 2400-sample run on the Mac
the last sample line before `Cancelled` read `Sample 1025/2400` — the same figure in all four versions —
with a `Remaining:` figure of three quarters of a minute or so still on it.

### 3. How many frames were still to come

Two runs, identical but for the frame range.

| frame range | gap, Mac | gap, Linux machine |
|---|---|---|
| `-s 1 -e 5` | 12.79 s | 8.16 s |
| `-s 1 -e 100` | 12.86 s | 8.19 s |

No line printed after the Control-C, in any of the four runs, mentioned a frame later than the one in hand.

### 4. A second Control-C, five seconds after the first

The first Control-C's own line says what a second one does: `Press ^C again to kill Blender`.

**On the Mac.** The process was still alive when the second Control-C was typed. What it printed, in order:

> `Blender killed`  
> `libc++abi: terminating`

and it was gone in the same instant. Exit status: **killed by signal 6**. The first two frames were on disk.

**On the Linux machine.** What it printed, in order:

> `Blender killed`  
> `Fra:3 Mem:9.95M (Peak 13.10M) | Time:00:16.14 | Remaining:00:21.53 | Mem:1.92M, Peak:1.92M | Scene, ViewLayer | Sample 257/600`

Exit status: **killed by signal 6**.

The second of those Linux lines came 3.1 seconds after the second Control-C, and the process was gone in the same instant. No line about not-freed memory blocks, and no line about freeing memory after a leak detector, appeared on either machine.

### 5. A render with no terminal at all, stopped by a signal

Started detached, in its own session, with no terminal attached and its printing going to a plain file,
then sent one signal from outside eight seconds into the third frame.

| signal | gap, Mac | gap, Linux machine |
|---|---|---|
| a plain termination request (`kill`) | 0.02 s | 0.02 s |
| an unignorable kill (`kill -9`) | 0.03 s | 0.02 s |
| an interrupt (`kill -INT`) | 12.60 s | 8.17 s |

After a plain termination request, and after an unignorable kill, nothing whatever was added to the
printing file: its last line is an ordinary progress line for the frame in hand. After an interrupt

> *Corrected by the citation check, 2026-08-26: this used to say the last line was "from the middle of
> the frame". It is a line for the frame in hand, but it was printed near that frame's beginning — in a
> repeat on the Creator's Mac in all four versions the last line of the file reads `Sample 1/600`, because
> Blender prints a sample line only at the boundaries of Cycles' sample batches and the signal arrived
> before the next one. Nothing else about this entry changed: nothing whatever is added to the file.*

the file ends exactly as entry 1's runs did — the sample line, the `Cancelled` line, `Blender quit`.

### 6. A Control-C landing between frames

The Control-C was typed the instant a `Saved:` line appeared. The gap was **0.03 seconds** on the Mac and
**0.08 seconds** on the Linux machine. Everything printed between the Control-C and the process being gone,
on the Mac:

> `^C`  
> `Sent an internal break event. Press ^C again to kill Blender`  
> `Fra:2 Mem:9.83M (Peak 13.05M) | Time:00:00.00 | Mem:0.00M, Peak:0.00M | Scene | Initializing`  
> `Blender quit`

### 7. `-q` added

Entry 1's command with `-q` added straight after `-b`, and the Control-C typed fifty seconds in. Gap:
**13.73 seconds** on the Mac, **3.61 seconds** on the Linux machine.

Nothing at all was printed between the Control-C and the process being gone, on either machine — no sample line, no `Finished`, no `Cancelled`, no `Blender quit`.

The line `Sent an internal break event. Press ^C again to kill Blender` was printed under `-q` in every
version on both machines.

### 8. The fullest logging

**What was typed first, and what came back.** Entry 1's command with `--log "*" --log-level debug`
added straight after `-b`. On both machines it came back with

> `Error: not a number '--log-level debug'.`

and the run then ended in under two seconds without rendering the animation at all.

> *Disputed by the citation check, 2026-08-26, and left standing rather than replaced.* A repeat of
> this exact command on both of this project's machines came back differently: the error line was
> printed, and then the run **carried on and rendered**. On the Creator's Mac it ran for 60.5 seconds
> in all, the Control-C landed eight seconds into the third frame as in entry 1, the gap was 12.24
> seconds, the ending was the ordinary `Sample 600/600` / `Finished` / `Cancelled` / `Blender quit`,
> and the exit status was 1. On the rented Linux machine it ran for 91.1 seconds, the gap was 8.18
> seconds, and the ending was the ordinary `Sample 257/600` / `Sample 0/600` / `Cancelled` /
> `Blender quit`, exit status 1. Both runs are recorded; neither is picked over the other. This test
> is on the citation check's list to be run again by somebody who was not in either room.
>
> *Run again on 2026-08-29 by a third machine, and it came back the same way as the two repeats: the error
> line, then an ordinary render, a Control-C eight seconds into the third frame, a gap of 23.06 seconds and
> the ending `Sample 513/600` / `Sample 0/600` / `Cancelled` / `Blender quit`, exit status 1. The full run
> is under "Observed — the citation check's re-runs" below. Three runs to one; the record above is kept and
> is the weaker of the two.*

Reading this version's
own `--help` afterwards gave, for that switch: "Set the logging verbosity level (higher for more details)
defaults to 1, use -1 to log all levels." So in this version the switch takes a number, not a word. (In
5.0, 5.1 and 5.2 the same `--help` entry offers `fatal`, `error`, `warning`, `info`, `debug` and `trace`,
and `--log-level debug` is accepted.)

**The repaired run.** With `--log "*" --log-level -1` instead, the gap was **12.24 seconds** on the Mac and
**8.19 seconds** on the Linux machine. The extra logging added lines of its own all through the run, in the
shape

> `INFO (wm.files): source/blender/windowmanager/intern/wm_files.cc:962 file_read_reports_finalize: Blender file read in 0m0.00s`

and between the Control-C and the process being gone it added three lines of its own, after the
`Cancelled` line and before `Blender quit`, on both machines:

> `INFO (bpy.rna): source/blender/python/intern/bpy_rna.cc:8172 pyrna_srna_ExternalType: SRNA sub-classed: 'Panel'`  
> `INFO (bke.sound): source/blender/blenkernel/intern/sound.cc:482 delayed_close_thread_run: System exit requested`  
> `INFO (bke.sound): source/blender/blenkernel/intern/sound.cc:510 delayed_close_thread_run: Delayed device close thread finished`

(The word `INFO` is printed in colour in the terminal; the colour codes are stripped here.)


### 9. `--debug-memory` added

Gap: **13.60 seconds** on the Mac, **8.21 seconds** on the Linux machine. Nothing about memory, blocks, leaks or
freeing appeared anywhere in either run — not before the Control-C and not after it. The lines printed
between the Control-C and the end were the same as entry 1's.

### 10. `--disable-abort-handler` added

Gap: **13.48 seconds** on the Mac, **8.25 seconds** on the Linux machine. The lines printed between the
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

> `Fra:3 Mem:12.17M (Peak 13.05M) | Time:00:21.36 | Mem:1.92M, Peak:1.92M | Scene, ViewLayer | Cancelled`

and on the Linux machine:

> `Fra:3 Mem:12.22M (Peak 13.10M) | Time:00:16.06 | Remaining:00:21.41 | Mem:1.92M, Peak:1.92M | Scene, ViewLayer | Cancelled`

Gap: 13.37 seconds on the Mac, 8.12 seconds on the Linux machine.

### 12. The graphics card against the processor

`--cycles-device` does exist in this version. Its `--help` entry reads `--cycles-device <device>` with the
example `# blender -b file.blend -f 20 -- --cycles-device OPTIX`.

| run | gap, Mac (`METAL`) | gap, Linux machine (`OPTIX`) |
|---|---|---|
| on the processor, `-- --cycles-device CPU`, 600 samples | 13.44 s | 8.31 s |
| on the card, 600 samples | 0.19 s | the render finished all five frames before the stop could be asked for |
| on the card, 20000 samples | 38.71 s | 24.46 s |

On the Mac's 600-sample card run the last line before `Cancelled` was

> `Fra:4 Mem:10.25M (Peak 26.25M) | Time:00:03.30 | Remaining:00:01.42 | Mem:1361.75M, Peak:1361.75M | Scene, ViewLayer | Sample 414/600 (Using optimized kernels)`

On the Linux machine the 600-sample card run finished normally, with five `Saved:` lines and exit status
`0`; that is recorded as the result it is, and the run was then repeated with the sample count raised to
20000, which is the third row of the table.

### 13. A different engine

`blender --factory-startup -E help` in this version lists `BLENDER_EEVEE_NEXT`, `BLENDER_WORKBENCH` and `CYCLES`. Both runs
used a frame range of 1 to 500 at 1920 by 1080, with the Control-C typed twenty seconds in.

| engine | gap, Mac | gap, Linux machine |
|---|---|---|
| `BLENDER_EEVEE_NEXT` | 0.54 s | 72.84 s |
| `BLENDER_WORKBENCH` | 0.04 s | 0.21 s |

The rented Linux machine's viewport is drawn by its processor rather than by its card. Its EEVEE frames
took about ninety seconds each there against a fraction of a second on the Mac, so the two EEVEE figures
are not a comparison of anything and are not offered as one.

### 14. One video file instead of separate images

The output was set to an FFmpeg MPEG-4 H.264 video with
`--python-expr "import bpy; s=bpy.context.scene; s.render.image_settings.file_format='FFMPEG'; s.render.ffmpeg.format='MPEG4'; s.render.ffmpeg.codec='H264'"`.
Gap: **13.48 seconds** on the Mac and **8.19 seconds** on the Linux machine, against entry 1's 12.32 and 8.09 for the
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
words `Render...`, a progress bar reading `38%`, and an X at its right end; the render window's header read

> `Frame:3 | Last:00:37.85 Time:00:14.88 | Remaining:00:23.43 |`

At one second after the key press the status bar was unchanged — the same `Render...`, the same `38%`, the same X.

Comparing each photograph against the one before it, the screen settled into a picture that did not change
again for the rest of the forty seconds, and it settled **between 1.0 and 2.1 seconds after the key press**. In the settled picture the status bar is empty — no
task, no words, no progress bar, no X — the render window's header reads

> `Frame:3 | Time:00:16.92 | Mem:1.92M, Peak: 1.92M`

with no `Remaining:` figure on it, and the timeline's current frame has gone back to 1.

**The Mac's half of this test could not be run**, for the reason given at the end of this section.

### 16. How long the render job reports itself as running

The same set-up as entry 15, with a timer printing `bpy.app.is_job_running('RENDER')` every tenth of a
second. Measured against the moment of the Escape, the last answer of `True` came **0.79 seconds** after it
and the first answer of `False` came **0.90 seconds** after it. The call did not crash Blender in this
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
Blender 4.5.12 LTS (hash `84afd5f785f7` built 2026-07-21 01:31:22) — the same hash both machines above ran, so nothing below is a difference of build. The scene and the harness were the same ones the 2026-08-26 runs used, copied
across unchanged. One frame took about 36 seconds in a background render and about 34 seconds in a render
started inside Blender's own window.

### The fullest logging in this version, run by a third pair of hands

Entry 8 of the 2026-08-26 list is recorded two ways, and the citation check sent it to a machine that was in
neither room. The same command was run again on the third machine:

> `python3 /root/q15/ctrlc.py --label "item 1 — 4.5 — third machine" --after "Fra:3" --delay 8 -- /opt/blender/4.5.12/blender -b --factory-startup /root/q15/q15_4.5.blend --log "*" --log-level debug -s 1 -e 5 -o /root/q15/out/i1_4.5/f_#### -F PNG -a`

**The error line was printed and the run then rendered.** In its own words:

> `[+0000.143] Error: not a number '--log-level debug'.`
> `[+0000.143] Blender 4.5.12 LTS (hash 84afd5f785f7 built 2026-07-21 01:31:22)`
> `[+0000.412] Read blend: "/root/q15/q15_4.5.blend"`

Frame 1 was saved at `[+0036.701]` and frame 2 at `[+0072.890]`. The Control-C landed eight seconds into the
third frame and the gap was **23.06 seconds**:

> `[+0080.931] ***** CONTROL-C TYPED (number 1) at wall clock 21:00:44 *****`
> `[+0080.931] ^C`
> `[+0080.931] Sent an internal break event. Press ^C again to kill Blender`
> `[+0103.910] Fra:3 Mem:11.33M (Peak 13.09M) | Time:00:31.01 | Remaining:00:05.25 | Mem:1.92M, Peak:1.92M | Scene, ViewLayer | Sample 513/600`
> `[+0103.912] Fra:3 Mem:12.21M (Peak 13.09M) | Time:00:31.02 | Remaining:00:05.25 | Mem:1.92M, Peak:1.92M | Scene, ViewLayer | Sample 0/600`
> `[+0103.912] Fra:3 Mem:12.21M (Peak 13.09M) | Time:00:31.02 | Remaining:00:05.25 | Mem:1.92M, Peak:1.92M | Scene, ViewLayer | Cancelled`
> `[+0103.968] Blender quit`
> `[+0103.989] ***** PROCESS GONE at wall clock 21:01:08 *****`

Exit status 1. `f_0001.png` and `f_0002.png` were on disk afterwards.

**The two records are not equally supported, and this says so plainly.** Entry 8 above records the run
ending in under two seconds without rendering. Three runs on three machines — the two of 2026-08-26 and this
one — now record the error line followed by an ordinary render. The earlier record is kept where it stands
and is not deleted; it is the weaker of the two, being one run against three. It is also the odd one out
against a sister case already observed in this version:
`Preparation/Blender Research/Findings/4.5/What actually ran.md` records that `--log-level info` "put
`Error: not a number '--log-level info'.` on the error stream and then **rendered anyway, exiting 0**".
What no run explains is how the first one came to end early, and that is not claimed here.

### What a click on the status bar's cancel button does

Run on the third machine's display, in this version. Blender was opened with a window on the scene file, a
timer started the animation render five seconds later over frames 1 to 5, and fifteen seconds into the third
frame the screen was photographed, every window on it was listed, the main window was raised, the pointer
was moved to **x=1069, y=1068** — where the 2026-08-26 photographs put the cancel X — the screen was
photographed again with the pointer sitting there, and then one click was made. The screen was photographed
every second for forty seconds afterwards.

**Done that way it did not stop the render, which is the sixth time that has happened.** The click landed at
`[+0096.946]` and `f_0003.png` was written at `[+0117.786]`, 20.8 seconds later. The run was still rendering when the
forty seconds of photographs ran out.

**Nothing was over the button.** The window list taken seconds before the click holds two windows and no
more:

> `window 4194306  name='q15_4.5 [/root/q15/q15_4.5.blend] - Blender 4.5.12 LTS'  geometry='WINDOW=4194306 X=0 Y=38 WIDTH=1920 HEIGHT=1061 SCREEN=0'`
> `window 4194352  name='Blender Render'  geometry='WINDOW=4194352 X=787 Y=453 WIDTH=351 HEIGHT=256 SCREEN=0'`

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
was repeated with `xdotool mousemove 1600 1068 click 1` on an empty part of the status bar at `[+0094.537]`,
the pointer moved to 1069,1068 at `[+0096.643]`, and the click made at `[+0097.215]`. **`f_0003.png` was never
written.** This time the button did react to the pointer: **636 pixels** changed in the same 30 by 24 box
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

**The Blender.** Blender 4.5.12 LTS, the official Windows x64 build taken from Blender's own download
server and unpacked to `C:\blender\4.5.12`. Its hash as the running program reports it is `84afd5f785f7`,
built 2026-07-21 01:33:53 — **the same hash the Creator's Mac and the Linux machines above ran**, so
nothing below is a difference of build.

**The scene and the set-up were the ones the 2026-08-26 runs used** — the same script, run on this machine
by this version so that no version had to convert another's file, and `--factory-startup` on every command.
**One frame took about 30 seconds here**, against about 21 on the Mac. Building the scene in this
version printed no warning.

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

### 1. Control-C part-way through a frame

One run per version was started; **only this version's was measured with a Control-C that reached
Blender.** The command:

> `C:\pyemb\python.exe C:\q15\ctrlc_win.py --method event --label "e1 / 4.5 / windows" --after "Fra:3" --delay 8 --fallback-wait 300 -- C:\blender\4.5.12\blender.exe -b --factory-startup C:\q15\q15_4.5.blend -s 1 -e 5 -o C:\q15\out\e1_4.5_win\f_#### -F PNG -a`

Only the harness was changed from the list as written; the Blender command line is character-for-character
the one the 2026-08-26 list gives, with Windows paths.

**The lines below were captured through the session's terminal rather than from the file on the machine**,
because the machine was destroyed before that file could be fetched, **and only lines matching a filter
were captured** — so the render's own progress lines from before the Control-C are not in the archive.
That is a real limit of this record and it is not hidden.

**Delivered as a typed key event** — a real Ctrl+C key record written into the console's own input buffer,
which is what the console receives when a person presses the keys — **the run was untouched**:

> `[+0076.463] ***** CONTROL-C TYPED (number 1) at wall clock 23:43:39 *****`
> `[+0170.959] Blender quit`
> `[+0170.981] ***** PROCESS GONE at wall clock 23:45:13 *****`
> `== exit status: exit code 0 (unsigned 0, hex 0x00000000)`

All five frames rendered, nothing was printed in answer, there is no gap to measure, and the exit code is
0 — the code of a render that finished. **That is a fact about how the Control-C was delivered, not about
Blender.**

**Delivered as the console control event**, Blender answered:

> `[+0077.140] ***** CTRL_C_EVENT RAISED (number 1) at wall clock 23:46:31 *****`
> `[+0098.339] Sent an internal break event. Press ^C again to kill Blender`
> `[+0098.339] Fra:3 Mem:12.22M (Peak 13.09M) | Time:00:29.20 | Remaining:00:04.95 | Mem:1.92M, Peak:1.92M | Scene, ViewLayer | Cancelled`
> `[+0098.453] Blender quit`
> `[+0098.475] ***** PROCESS GONE at wall clock 23:46:52 *****`
> `== exit status: exit code 1 (unsigned 1, hex 0x00000001)`

So, for **Blender 4.5.12 on Windows Server 2025**, and for that alone:

- **The sentence is the same.** `Sent an internal break event. Press ^C again to kill Blender` — word for
  word what the Mac and the Linux machines print.
- **The gap was 21.335 seconds**, from the event to the process being gone. The frame in hand read
  `Time:00:29.20 | Remaining:00:04.95` at the moment it was cancelled, so the stop was asked for roughly
  two thirds of the way through a 30-second frame and the wind-down ran to the end of that frame's
  sampling. That is consistent with what the Mac and Linux runs found, but **one run in one version is not
  a finding about what settles the gap**, and entry 2, which is the test that would settle it, was not run.
- **What is printed** is the break sentence, then one `Fra:3 … | Cancelled` line, then `Blender quit`. **No
  memory or leak lines** — but `--debug-memory` was not on, and entry 9, which is the test for that, was
  not run.
- **The exit status is 1**, the same as the Mac's and the Linux machines'.

**Never merged with the Mac's or the Linux machines' figures.** This is a four-core shared virtual machine
where one frame takes about 30 seconds against the Mac's 21.

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
   Entry 1 was run on it in this version and its result is in the observed section above. **Entries 2 to
   16 were never begun, in any version.** Another machine is needed for them, and three things about the
   harness are already known — anyone running the rest will meet all three:

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

2026-08-26 — Claude Opus 5 (Claude Code subagent) — Frames, Blender 4.5.

2026-08-26 — Claude Opus 5 (Claude Code) — hand tests for Frames, run on both machines and written back into this document.

2026-08-29 — Claude Opus 5 (Claude Code) — the citation check's re-runs written into this document: the third machine's run of the 4.5 logging switch, the cancel-button result, and the macOS refusal checked again.

2026-08-30 — Claude Opus 5 (Claude Code) — the Windows machine's result written into this document: the machine and the edition it reported, entry 1 in this version, what was not run, and the three harness traps.

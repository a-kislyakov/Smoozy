# Frames — Architecture questions, second pass (Blender 5.0)

**Sources consulted, and when.** Blender 5.0 manual, Blender 5.0 Python API documentation, Blender 5.0 release notes, and the Cycles 5.0 add-on files — all as supplied in `Blender Sources/`, snapshot taken 2026-07-30. That snapshot date is my date read for everything cited below. I did the searching and writing on 2026-08-26.

I read only the question sheet and files inside Blender Sources/. I did not open any other part of this project.

**Scope.** I was given question 15 only. I read the whole sheet, and I answer 15 alone.

---

## 15. When a render working through a range of frames is interrupted, what does it do between the moment it is asked to stop and the moment it ends — how long does that take, what settles that, and what does it print in the meantime?

**Reading of the question, and the ambiguity in it.** "Interrupted" can mean two things. I answer the reading I think is meant: something *asks* the render to stop — a person clicks the cancel button, presses a key, or a command-line render is sent Control-C — and the render then takes some amount of time before it actually ends. The other reading would be an abrupt kill (power loss, the process being destroyed outright), which has no "in between" at all and would make the question empty. I say plainly that I chose the first reading.

**LABEL: not in the sources.**

For the question as asked — what the render is doing in that interval, how long the interval is, what governs its length, and what it prints during it — the 5.0 manual, the 5.0 Python API documentation, the 5.0 release notes and the Cycles 5.0 add-on files say nothing. Not one of the four sources describes the state a render passes through between a stop request and its end. Nothing in them names a duration, a bound on a duration, or any setting or condition that decides one. Nothing in them shows or names a message printed on the way out.

That is the answer. What follows is everything the sources *do* say that touches a part of the question, each labelled on its own, and then the record of where I looked.

---

### What the sources do say, piece by piece

**(a) There is a documented mechanism by which a running render learns it has been asked to stop, and the documentation states that it is meant to be checked often.**

LABEL: **documented** (that the check exists and why it is meant to be frequent).

> `test_break()`
>
> Test if the render operation should been canceled, this is a fast call that should be used regularly for responsiveness

Source: Python API documentation, https://docs.blender.org/api/5.0/bpy.types.RenderEngine.html (in file `Blender Sources/api_5.0/bpy.types.RenderEngine.txt`, lines 498–500). Date read: 2026-07-30. The wording, including "should been canceled", is quoted exactly as it stands.

LABEL for the next step: **inferred.** The step I took: the documentation says a render engine finds out it should stop by making this call, and that calling it "regularly" is what gives "responsiveness". From that it follows that a stop request is not acted on at the instant it arrives but at the engine's next check, and that the gap is as long as the engine leaves between checks. The sources do not say how often Cycles or EEVEE actually call it, so this gives no duration — only the shape of what settles one. A reader has to supply that reasoning, so this is inferred, not documented.

**(b) A cancelled render job ends down a different path than a finished one.**

LABEL: **documented.**

> `bpy.app.handlers.render_cancel`
>
> on canceling a render job. Accepts one argument: the scene data-block being rendered

> `bpy.app.handlers.render_complete`
>
> on completion of render job. Accepts one argument: the scene data-block being rendered

Source: Python API documentation, https://docs.blender.org/api/5.0/bpy.app.handlers.html (file `Blender Sources/api_5.0/bpy.app.handlers.txt`, lines 146–152). Date read: 2026-07-30.

What this settles is only that a cancel has its own ending, separate from a completion. The documentation does not say when in the interval that handler runs — at the request, or at the end, or somewhere between — nor what happens either side of it.

**(c) A render engine has a documented way of closing out a piece of work as cancelled rather than as done.**

LABEL: **documented.**

> `end_result(result, *, cancel=False, highlight=False, do_merge_results=False)`
>
> All pixels in the render result have been set and are final
>
> `cancel` (boolean, (optional)) – Cancel, Don't mark tile as done, don't merge results unless forced

Source: Python API documentation, https://docs.blender.org/api/5.0/bpy.types.RenderEngine.html (file `Blender Sources/api_5.0/bpy.types.RenderEngine.txt`, lines 458–470). Date read: 2026-07-30.

This tells you an engine can hand back partial work marked as not done. It does not say that Blender's own engines do this when a render is stopped, nor what Blender then does with it.

**(d) A frame has a documented finishing step of its own.**

LABEL: **documented.**

> `render_frame_finish()`
>
> Perform finishing operations after all view layers in a frame were rendered

Source: Python API documentation, https://docs.blender.org/api/5.0/bpy.types.RenderEngine.html (file `Blender Sources/api_5.0/bpy.types.RenderEngine.txt`, lines 374–376). Date read: 2026-07-30.

The sources do not say whether this step runs, is skipped, or is cut short when the render is asked to stop part-way through a frame.

**(e) What a render prints while it runs is documented exactly, for 5.0, and no cancel line appears in it.**

LABEL: **documented** (for the printed form of a running render).

The 5.0 release notes state that background render progress now goes through Blender's logging system, that the output format changed, and give a worked example:

> Most of Blender now uses the same logging system,
> including operators reports and background render progress.
> The output format has changed, and log levels are easier to control.

and, as the example output:

> ```
> 00:00.244  render           | Rendering single frame (frame 1)
> 00:00.245  render           | Start rendering: Scene, ViewLayer
> 00:00.245  render           | Engine: Cycles
> 00:26.799  render           | Mem: 2983M | Sample 1/16
> 00:26.799  render           | Mem: 2983M | Sample 16/16
> 00:28.631  render           | Saved: '/tmp/0001.png'
> 00:28.631  render           | Time: 00:28.38 (Saving: 00:00.15)
> ```

Source: release notes, https://developer.blender.org/docs/release_notes/5.0/core/ (file `Blender Sources/release_notes_5.0/core.md`, lines 66–98). Date read: 2026-07-30.

Note what this is and is not. It is a documented sample of a render that ran to the end and saved, printed under the `render` log category. It shows the shape of every line a render prints — a timestamp, a category, then the text. It does **not** show an interrupted render, and no line in any of the four sources shows what is printed after a stop is asked for.

Two related documented facts about printing:

> `--log "render,cycles"`: both render and cycles messages.

and

> `-q, --quiet`
>
> Suppress status printing (warnings & errors are still printed).

Source for both: manual, https://docs.blender.org/manual/en/5.0/advanced/command_line/arguments.html (file `Blender Sources/manual_5.0/advanced/command_line/arguments.txt`, lines 244 and 425–427). Date read: 2026-07-30.

> *Changed by the citation check, 2026-08-26: the first line number read 237, which is inside the `--log <match>` entry; the quoted example sits at line 244 of the same file. Same page, corrected line.*

And one from the Cycles add-on's own files:

LABEL: **observed** (I am reading the file that ships with Cycles, not documentation).

> ```
>     parser.add_argument("--cycles-print-stats",
>                         help="Print rendering statistics to stderr",
>                         action='store_true')
> ```

File: `Blender Sources/cycles_5.0/engine.py`, lines 13–15, inside `_configure_argument_parser()`. Date read: 2026-07-30. The manual describes the same flag as "Log statistics about render memory and time usage" (https://docs.blender.org/manual/en/5.0/advanced/command_line/arguments.html, file line 95). Neither says anything about statistics being printed when a render is stopped early.

There is also a handler tied to this printing:

> `bpy.app.handlers.render_stats`
>
> on printing render statistics. Accepts one argument: the render stats (render/saving time plus in background mode frame/used [peak] memory).

Source: Python API documentation, https://docs.blender.org/api/5.0/bpy.app.handlers.html (file `Blender Sources/api_5.0/bpy.app.handlers.txt`, lines 166–168). Date read: 2026-07-30.

**(f) Inside Blender's own window, a running render is shown as a job with a cancel button.**

LABEL: **documented.**

> Running Task
>
> Shows the progress of the currently running task (such as rendering or baking).
> Hovering the mouse pointer over the progress bar will display a time estimate.
> The task can be aborted by clicking the cancel button ().

Source: manual, https://docs.blender.org/manual/en/5.0/interface/window_system/status_bar.html (file `Blender Sources/manual_5.0/interface/window_system/status_bar.txt`, lines 30–34). Date read: 2026-07-30. (The empty brackets are how the text file renders an inline icon.)

I also opened the picture the manual places here, `_images/interface_window-system_status-bar_ui-center.png` (in `Blender Sources/manual_5.0/_images/`), part of the same manual page. LABEL: **documented**, from the picture. What is visible in it: a small strip showing a film-camera icon, the label "Sequence Render", a filled progress bar reading "94%", and a square button with an X at the right end. That is all it shows. It is a picture of a job in progress; there is no picture in the manual of a job that has been asked to stop, so the picture says nothing about the interval this question asks about.

Related, and it is what lets you watch for the end of the interval rather than guess at it:

> `static bpy.app.is_job_running(job_type)`
>
> Check whether a job of the given type is running.

Source: Python API documentation, https://docs.blender.org/api/5.0/bpy.app.html (file `Blender Sources/api_5.0/bpy.app.txt`, lines 496–506). Date read: 2026-07-30. The job types include:

> `RENDER`:
>
> Regular rendering.

Source: Python API documentation, https://docs.blender.org/api/5.0/bpy_types_enum_items/wm_job_type_items.html (file `Blender Sources/api_5.0/bpy_types_enum_items/wm_job_type_items.txt`). Date read: 2026-07-30.

One caution the sources themselves record about that call: the 5.0 corrective-release notes list, among the fixes in Blender 5.0.1, "Python API: Call of 'bpy.app.is_job_running' function in background crashes Blender 5.0. (blender/blender#150276)" — source: release notes, https://developer.blender.org/docs/release_notes/5.0/corrective_releases/ (file `Blender Sources/release_notes_5.0/corrective_releases.md`, line 89), date read 2026-07-30.

**(g) The stop request itself, from outside the window.**

LABEL: **inferred.**

The API documentation's page on using Blender as a Python module says, of that mode:

> Blender's typical signal handlers are not initialized, so there is no special handling for Control-C
> to cancel a render and a crash log is not written in the event of a crash.

Source: Python API documentation, https://docs.blender.org/api/5.0/info_advanced_blender_as_bpy.html (file `Blender Sources/api_5.0/info_advanced_blender_as_bpy.txt`, lines 79–81). Date read: 2026-07-30.

The step I took: this page says that in one particular mode the typical signal handlers are absent and *therefore* Control-C has no special handling to cancel a render. For that sentence to be worth writing, ordinary Blender must have those handlers and Control-C must have that special handling. The reader has to supply that inversion, so the label is inferred, not documented. Even granting it, the sentence says only that Control-C can ask a render to stop. It says nothing about what happens next, how long it takes, or what is printed.

The manual also lists, without explanation, a switch that turns one of these handlers off:

> `--disable-abort-handler`
>
> Disable the abort handler.

Source: manual, https://docs.blender.org/manual/en/5.0/advanced/command_line/arguments.html (file `Blender Sources/manual_5.0/advanced/command_line/arguments.txt`, lines 417–419). Date read: 2026-07-30. The manual does not say what the abort handler does, when it runs, or what it prints.

**(h) One release note bears directly on how long the interval can be, and it is about a failure rather than normal behaviour.**

LABEL: **documented.**

Among the bugs listed as fixed in 5.0, under "Platforms & Builds":

> Linux Freeze when force cancelling a CLI render

Source: release notes, https://developer.blender.org/docs/release_notes/5.0/bugfixes/ (file `Blender Sources/release_notes_5.0/bugfixes.md`, line 305). The same page opens: "There are a total of 588 commits in 5.0 that fixed issues that existed in Blender 4.5 or older." Date read: 2026-07-30.

What this establishes as documented fact: force-cancelling a render started from the command line is a real thing, it could freeze on Linux, and that was fixed in 5.0. What it does not establish: anything about the normal interval, its length, or what is printed. It also does not describe the fixed behaviour — only that the freeze is gone.

Also listed, among the 132 fixes in Blender 5.0.1: "Blender gets stuck during rendering. (blender/blender#150773)" — source: release notes, https://developer.blender.org/docs/release_notes/5.0/corrective_releases/ (file `Blender Sources/release_notes_5.0/corrective_releases.md`, line 19), date read 2026-07-30. The note gives no detail, and does not say the sticking had anything to do with stopping a render. I record it because it is the only other place in the sources where a render and an unbounded wait appear together.

---

### Not covered

This answer does not settle, because no source I was given states it:

- Whether the frame in hand is abandoned where it stands, finished to the end, or finished only to some checkpoint.
- Whether the steps that normally follow the sampling of a frame — denoising, compositing, colour conversion, writing the file, closing a video file — still run once a stop has been asked for.
- Any number at all for the interval: no typical value, no upper bound, no worst case.
- What settles the interval in Blender's own engines. Point (a) gives the shape of the answer (the wait is until the engine's next check) but no source says how often Cycles, EEVEE or Workbench check, or whether a long single operation — a sample, a tile, a shader compile, a video flush — can hold the check off.
- Whether the interval differs between a render that is stopped between frames and one stopped in the middle of a frame.
- Any message printed on the way out. The one documented sample of render printing (point e) is of a render that ran to the end.
- Whether the progress display in the status bar changes appearance while the stop is pending.
- Whether the answer differs between engines, or between rendering to separate images and rendering to a single video file.
- Whether `render_cancel` runs at the request or at the end.

The label for the question as a whole is **not in the sources**; every item above is part of why.

---

### Outside the interface

Does this hold when Blender is driven from outside its own window — started and controlled by something other than a person clicking?

The silence holds in both places, and it is if anything more complete outside the window. Points (a) to (e) are not tied to the interface: the engine's stop check, the cancel handler, the frame-finishing step and the log lines all apply to a render however it was started. Point (f) — the progress bar and its cancel button — is inside the window only; the sources describe no equivalent readout outside it.

Three differences the sources do state, rather than leave silent:

- The request arrives differently. Inside the window there is a documented cancel button on the status bar (point f). Outside it, the only stop route the sources touch is Control-C acting through Blender's signal handlers, and that only by the inversion in point (g), which is inferred.
- What is printed is documented specifically for the outside case. The 5.0 release notes' logging section says the change covers "background render progress" and gives the example lines (point e); `-q, --quiet` suppresses status printing; `--log render` and `--log cycles` select what is shown; `--log-file <filepath>` sends it to a file (manual, https://docs.blender.org/manual/en/5.0/advanced/command_line/arguments.html, file lines 231–270). None of that names a message tied to stopping.
- The one recorded fault in this area is specific to the outside case: "Linux Freeze when force cancelling a CLI render" (point h).

One trap worth naming: `bpy.app.is_job_running`, the obvious way for a controlling program to watch for the render actually ending, is documented (point f) but the corrective-release notes record it crashing Blender 5.0 in background mode, fixed in 5.0.1.


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

Does this hold on every operating system Blender runs on?

The silence itself is the same on every operating system — none of the four sources describes the interval on any of them, and nothing in them is written as applying to one system and not another.

Where the sources do draw an operating-system line, near this question:

- Linux. "Linux Freeze when force cancelling a CLI render", fixed in 5.0 (point h). This is the one place the sources say that stopping a render behaved differently on one operating system: before this fix, on Linux, force-cancelling a command-line render could freeze rather than end.
- Windows. Two adjacent, documented differences, neither about cancelling. The console that would carry any printed output is toggled differently: "When Blender is started on a Microsoft Windows operating system, the Console Window … To display the console again, go to Window ‣ Toggle System Console." — manual, https://docs.blender.org/manual/en/5.0/advanced/command_line/launch/windows.html (file `Blender Sources/manual_5.0/advanced/command_line/launch/windows.txt`, lines 15–18), date read 2026-07-30. And the manual records a Windows-only limit on how long a GPU may be held: "If a GPU is used for both display and rendering, Windows has a limit on the time the GPU can do render computations." — manual, https://docs.blender.org/manual/en/5.0/render/cycles/gpu_rendering.html (file `Blender Sources/manual_5.0/render/cycles/gpu_rendering.txt`, lines 168–169), date read 2026-07-30. *Line numbers corrected by the citation check, 2026-08-26: they read 169–171; the quoted sentence sits at lines 168–169 of that file.* I report that only because it is the one documented cap on the length of an uninterruptible stretch of rendering anywhere in the sources; the manual does not connect it to cancelling, and I do not.
- macOS. Nothing in the sources bears on this question for macOS. The one macOS mention nearby is incidental — the example log output in the 5.0 release notes happens to have been taken on a Mac ("Create Cocoa system", "Apple M3 Max").

Beyond that: the sources do not say.

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

### Record of where I looked

Everything below was run against `Blender Sources/manual_5.0` (5,197 files), `Blender Sources/api_5.0` (2,062 files), `Blender Sources/release_notes_5.0` (23 files) and `Blender Sources/cycles_5.0` (403 files). I searched the whole of each before opening anything, and I searched case-insensitively throughout.

**Terms swept across all four sources.** For the stop itself: *cancel, cancelled, canceled, cancelling, canceling, abort, interrupt, stop, halt, terminate, kill, quit, escape, Esc, break, test_break, signal, signal handler, SIGINT, Ctrl-C, Ctrl+C, Control-C, force quit*. For the interval and its length: *immediately, instantly, at once, delay, latency, responsive, responsiveness, wait, waiting, pending, how long, finish the current, will finish, finishes, after the current, graceful, shutdown, shut down, wind down, clean up, cleanup, unwind, discard, flush, finalize, part-way, partway, partially, midway, mid-render, stuck, freeze, hang, busy.* For what is printed: *print, console, system console, console window, stdout, stderr, log, logging, status, report, message, statistics, render stats, render_stats, progress, progress bar, Fra:, Mem:, Saved:, Time:.* For the surrounding machinery: *render_cancel, render_complete, render_init, render_pre, render_post, render_write, end_result, update_stats, update_progress, render_frame_finish, job, is_job_running, WM job.*

**Searches that returned nothing at all, anywhere in the four sources:** *SIGINT; force quit; graceful; finish the current; will finish; part-way; partway; midway; mid-render;* and the literal render-progress fragments *Fra:*, *Mem:*, *Saved:* and *Time:* as they appear in a console (*Mem:*, *Saved:* and *Time:* appear only inside the release-notes example block quoted at point (e), and *Fra:* appears nowhere in the 5.0 sources at all, not even in that block — which is why that block is the whole of what the sources show of render printing).

> *Changed by the citation check, 2026-08-26: this sentence used to say all four fragments "appear only inside the release-notes example block". Three do; `Fra:` is absent from the whole 5.0 snapshot, including that block — the block's sample is of a single-frame render and carries no frame field at all.*

**Pages I opened in full and what each ruled out.**

- Manual, https://docs.blender.org/manual/en/5.0/render/output/animation.html — the only manual page whose text contains "interrupt". Its use of the word is about the decision to stop ("May need to interrupt rendering to use the computer, and want to be able to resume rendering where you left off") and about recovery afterwards, not about the mechanics of stopping. Ruled out.
- Manual, https://docs.blender.org/manual/en/5.0/render/output/introduction.html — contains the only manual sentence in which a render is cancelled ("the render job can be canceled and resumed at the last rendered frame by changing the frame range"). It is about resuming, not about the interval. Ruled out for this question.
- Manual, https://docs.blender.org/manual/en/5.0/interface/window_system/status_bar.html, and its picture `_images/interface_window-system_status-bar_ui-center.png` — the only place the manual describes stopping a render at all. Yielded point (f) and nothing about what follows the click.
- Manual, https://docs.blender.org/manual/en/5.0/advanced/command_line/arguments.html — read the whole of the Logging Options, Debug Options and Cycles Render Options sections. Yielded points (e) and (g). The only occurrences of "abort" are `--disable-abort-handler` and `--disable-crash-handler`, with no explanation attached.
- Manual, https://docs.blender.org/manual/en/5.0/advanced/command_line/render.html — the page on rendering from the command line. It covers starting a render and the order of arguments. It says nothing about stopping one. Ruled out.
- Manual, https://docs.blender.org/manual/en/5.0/render/introduction.html — engine overview only. Ruled out.
- Manual, https://docs.blender.org/manual/en/5.0/interface/window_system/topbar.html — the Render menu, including Render Animation and Lock Interface. No stop entry, no mention of cancelling. Ruled out.
- Manual, https://docs.blender.org/manual/en/5.0/interface/keymap/blender_default.html and .../industry_compatible.html — checked for a documented key that stops a render. F11 and F12 are listed for showing the render window and rendering; no cancel key is listed for either keymap. Ruled out.
- Manual, https://docs.blender.org/manual/en/5.0/interface/operators.html — checked whether a render might be documented as a modal operator, which would make "Esc, RMB — Cancels a modal operator" apply. The page describes modal operators as ones that "require some sort of interactive input"; a render is described elsewhere as a job, not a modal operator, and no source connects the two. Ruled out as not established.
- Manual, https://docs.blender.org/manual/en/5.0/editors/info_editor.html — the editor that "logs the executed operators as well as errors, warnings, and informational messages". No render or cancel message is described. Ruled out.
- Manual, https://docs.blender.org/manual/en/5.0/render/cycles/gpu_rendering.html — read the whole Frequently Asked Questions and Error Messages sections, on the chance that unresponsiveness during rendering was described there. It is, but only as the interface being unable to redraw while the GPU renders; nothing about stopping. Yielded only the Windows GPU time limit noted above.
- Manual, https://docs.blender.org/manual/en/5.0/render/output/properties/output.html — read for what the encoder prints ("When you view the System Console, you can see some of the output of the encoding process. You will see even more output if you execute Blender as blender -d."). Nothing about stopping. Ruled out.
- Manual, https://docs.blender.org/manual/en/5.0/editors/preferences/interface.html — the Render In preference (Keep User Interface / Maximize Area / Image Editor / New Window). Describes where the render is shown, not how it ends. Ruled out.
- API, https://docs.blender.org/api/5.0/bpy.types.RenderEngine.html — read in full, including the worked example engine and the whole member list. Yielded points (a), (c), (d). Nothing else in it touches cancellation.
- API, https://docs.blender.org/api/5.0/bpy.app.handlers.html — read in full. Yielded points (b) and (e). Every handler with "cancel" in its name is listed there; the three are `composite_cancel`, `object_bake_cancel` and `render_cancel`, and none carries more than the one-line description quoted.
- API, https://docs.blender.org/api/5.0/bpy.ops.render.html — read the whole operator list. `bpy.ops.render.render()` is marked "Undocumented, consider contributing." The only cancel operator is `bpy.ops.render.view_cancel()`, "Cancel show render view", which closes the render window rather than the render. Ruled out, and worth flagging as an easy misreading.
- API, https://docs.blender.org/api/5.0/info_advanced_blender_as_bpy.html — yielded point (g).
- API, https://docs.blender.org/api/5.0/bpy.app.html and https://docs.blender.org/api/5.0/bpy_types_enum_items/wm_job_type_items.html — yielded the job-running check and the `RENDER` job type in point (f).
- API, https://docs.blender.org/api/5.0/info_gotchas_threading.html — opened because it was the only API page matching "waiting". It is about Python threads crashing Blender. Ruled out.
- API, https://docs.blender.org/api/5.0/bpy.types.HydraRenderEngine.html — matched the `test_break` search, but only as an inherited-member list with no text of its own. Ruled out.
- Release notes: I read https://developer.blender.org/docs/release_notes/5.0/rendering/ and https://developer.blender.org/docs/release_notes/5.0/core/ in full, and swept all 23 release-note files for the term list above. Yielded point (e) from `core`. `rendering` covers shader nodes, baking and pass renaming, with nothing on stopping a render — ruled out. `cycles`, `eevee`, `pipeline_io`, `python_api`, `user_interface`, `sequencer`, `compositor` produced no match on any stop-related term other than the ones reported.
- Release notes: `bugfixes` and `corrective_releases` swept for *cancel, abort, interrupt, stop, stuck, freeze, hang, render*. Yielded point (h), the `is_job_running` crash, and "Blender gets stuck during rendering". Everything else matching was unrelated (grease pencil primitives, fluid simulation, animation playback).
- Cycles' own files: I listed all 403 and searched every one. The Python files that ship with the add-on are `__init__.py`, `camera.py`, `engine.py`, `operators.py`, `osl.py`, `presets.py`, `properties.py`, `ui.py`, `version_update.py`; the rest are OSL shaders (`.oso`), shader headers and kernel headers. Searching the Python files for *cancel, abort, interrupt, test_break, progress* returns exactly two lines, both in `operators.py`: `return {'CANCELLED'}` (line 88, the denoise operator giving up when a frame file is missing) and the comment `# TODO: support cancel and progress reports.` (line 97, about the standalone denoise operator, not about rendering). Neither concerns stopping a render. What is **not** in this folder is the part of Cycles that would answer the question — the session and progress code that decides when a stop request is acted on. I looked for it under every plausible name and it is not present; the folder holds the add-on's Python, its OSL shaders, and the render kernel headers only. I record that as a genuine gap rather than treat the absence as evidence.

**What I did not do.** I did not go to the web, and I did not need to. I did not open any document in the folder I saved into, or anything else in this project.

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

2026-08-26 — Claude Opus 5 (Claude Code subagent) — Frames, Blender 5.0.

2026-08-26 — Claude Opus 5 (Claude Code) — hand tests for Frames, run on both machines and written back into this document.

2026-08-29 — Claude Opus 5 (Claude Code) — the citation check's re-runs written into this document: the cancel-button result on a third machine, and the macOS refusal checked again.

2026-08-30 — Claude Opus 5 (Claude Code) — the Windows machine written into this document: the machine and the edition it reported, entry 1 started in this version and killed with the machine, what was not run, and the three harness traps.

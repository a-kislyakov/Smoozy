# Reports — cross-sweep questions, second pass (Blender 5.2)

**Sources consulted, and the date.**
Blender 5.2 manual (`Blender Sources/manual_5.2`, including its `_images` folder), Blender 5.2 Python API documentation (`Blender Sources/api_5.2`), Blender 5.2 release notes (`Blender Sources/release_notes_5.2`), and the Cycles add-on's own files (`Blender Sources/cycles_5.2`). All four were taken on **2026-07-30**, and that is the date read for everything quoted below.

**I read only the question sheet and files inside Blender Sources/. I did not open any other part of this project.**

One disclosure, so the line above is exact. When I checked that my own file had saved under the right name, the folder listing showed me the names of the other documents sitting beside it. I opened none of them, and I read nothing but their file names. Nothing in this answer comes from them.

Question answered: **9 only.**

---

## 9. What does a render report about its progress under each of the ways a render can be started?

### The question was ambiguous — say so first

"The ways a render can be started" can be read two ways.

- **Reading A (the one I answer):** the different routes by which a render is set going — a menu item or hotkey inside Blender's window, a command line, a Python call, Blender loaded as a Python module.
- **Reading B (the one I do not answer):** the different *kinds* of render that can be started — a still, an animation, a viewport preview, a sequencer render.

I answer Reading A, because the phrase "started" points at how the run begins rather than at what it produces. Where the sources say something that splits along Reading B as well, I have included it under the route it belongs to, so the second reading is partly covered by accident rather than by design.

### Headline label

**LABEL: not in the sources** for the question as asked. No page in the 5.2 manual, the 5.2 API documentation, the 5.2 release notes, or the Cycles add-on files sets out what a render reports about its progress route by route. Nothing anywhere states what a render prints, or whether it prints at all, when it is started from a command line or from a script.

What the sources *do* carry is a set of separate pieces, each of which touches one route. I set them out below, each with its own label, because together they are the closest thing to an answer that exists and the gaps between them are the real finding.

---

### Piece 1 — Started from inside Blender's window: the Status Bar shows a progress bar

**LABEL: documented.**

The manual describes a progress display in the Status Bar, and names rendering as one of the tasks it covers.

Exact words:

> "The middle of the Status Bar displays information about in-progress operations."

> "Running Task
>
> Shows the progress of the currently running task (such as rendering or baking).
> Hovering the mouse pointer over the progress bar will display a time estimate.
> The task can be aborted by clicking the cancel button ()."

Source: the manual, https://docs.blender.org/manual/en/5.2/interface/window_system/status_bar.html
Date read: 2026-07-30

So, for a render started from inside Blender's window, three things are reported: a progress bar, a time estimate that appears when the mouse pointer rests on that bar, and a cancel control.

**Piece 1b — the picture beside those words.**

**LABEL: documented** (a picture in the manual is part of the manual).

The page carries `[image: _images/interface_window-system_status-bar_ui-center.png]`. I opened that file. It shows a single strip taken from the Status Bar containing, left to right: a small camera icon; the words **Sequence Render**; a horizontal bar filled most of the way with the figure **94%** written inside it; and an **X** button at the right end.

What is visible is visible: the running task is named in words, its progress is given as a filled bar carrying a whole-number percentage, and the cancel control sits beside it. What that picture does *not* show is any frame number, sample count, or elapsed time — but a single screenshot of one job at one moment cannot establish that those are never shown, so I claim nothing about their absence from this picture.

Source: the manual, https://docs.blender.org/manual/en/5.2/interface/window_system/status_bar.html (the image `_images/interface_window-system_status-bar_ui-center.png` beside it)
Date read: 2026-07-30

**Piece 1c — the routes that lead here.**

**LABEL: documented.** The manual lists the render commands in the top bar's Render menu:

> "Render Image F12
>
> Render the active scene at the current frame.
>
> Render Animation Ctrl-F12
>
> Render the animation of the active scene.
>
> ...
>
> Render Sequencer Image Alt-F12
>
> If a Sequencer Scene exists that differs from the active scene, render that
> scene instead.
>
> Render Sequencer Animation Ctrl-Alt-F12
>
> Render the animation of the Sequencer Scene."

Source: the manual, https://docs.blender.org/manual/en/5.2/interface/window_system/topbar.html
Date read: 2026-07-30

The manual does **not** say that these four differ from one another in what they report about progress. It says nothing on the subject either way.

---

### Piece 2 — Started from inside Blender's window: what happens to the screen is a user preference

**LABEL: documented.**

Where the picture of the render appears while it is being made is a setting, not a fixed behaviour. This matters for this question because one of the four choices makes the render invisible while it runs.

Exact words:

> "Render In
>
> When rendering, the user interface can do any of:
>
> Keep User Interface:
>
> The user interface does not change and the render is computed in the background.
>
> Maximize Area:
>
> A new Image editor is opened as a temporary window in full screen mode.
>
> Image Editor:
>
> The area that is the largest on screen is replaced by a temporary Image editor.
>
> New Window:
>
> A new Image editor is opened as a regularly sized temporary window."

Source: the manual, https://docs.blender.org/manual/en/5.2/editors/preferences/interface.html
Date read: 2026-07-30

Under "Keep User Interface" nothing on screen changes, so the Status Bar progress bar of Piece 1 is the only progress report left in view. That last sentence is my step, not the manual's, so treat it as **inferred**: the manual says the interface does not change, and separately says the Status Bar shows the progress of the running task; putting the two together is the reader's work, not the source's.

The manual also documents the command that shows the render picture on demand:

> "View Render F11
>
> Show the Render window. (Press again to switch back to the main Blender window.)"

Source: the manual, https://docs.blender.org/manual/en/5.2/interface/window_system/topbar.html
Date read: 2026-07-30

---

### Piece 3 — Started from inside Blender's window: viewport render

**LABEL: documented, and thin.**

The viewport render commands are a separate way to start a render:

> "Render a Still Image
>
> To render a still image, use 3D Viewport - View - Render Viewport Preview.
>
> Render an Animation
>
> To render an animation, use 3D Viewport - View - Render Playblast."

and

> "As with a normal render, you can abort it with Esc."

Source: the manual, https://docs.blender.org/manual/en/5.2/editors/3dview/viewport_render.html
Date read: 2026-07-30

The only progress-related thing the page says is that it can be aborted with Esc, "as with a normal render". It reports nothing about what these routes display while running.

---

### Piece 4 — Started from a command line: the sources describe the arguments, not the output

**LABEL: not in the sources** for the output itself; **documented** for the arguments that switch that output on and off.

The manual has a page devoted to rendering from a command line. It sets out the arguments and worked examples, and says why you would do it:

> "In some situations we want to increase the render speed,
> access Blender remotely to render something or build scripts that use the command line."

> "-b
>
> Render in the background (without UI)."

Source: the manual, https://docs.blender.org/manual/en/5.2/advanced/command_line/render.html
Date read: 2026-07-30

That page says **nothing** about what appears in the terminal while the render runs. Neither does the arguments page, nor the page on the console window. The nearest the manual comes to a general statement is:

> "The Console Window, also called a Terminal, is an operating system text window that displays
> messages about Blender's operations, status, and internal errors."

> "When Blender is manually started from a terminal,
> Blender output is shown in the Console Window it is started from."

Source: the manual, https://docs.blender.org/manual/en/5.2/advanced/command_line/index.html
Date read: 2026-07-30

That is a statement about the console, not about a render. It names "status" as one of the things shown, but does not say what a render's status text contains, or in what shape, or how often.

**The arguments that govern it.** Several arguments in the manual bear directly on progress output. All are **documented**:

> "-q, --quiet
>
> Suppress status printing (warnings & errors are still printed)."

> "--log <match>
>
> Enable logging categories, taking a single comma separated argument.
>
> --log "*": everything
> --log "event": every category starting with event.
> --log "render,cycles": both render and cycles messages."

> "--log-level <level>
>
> Set the logging verbosity level.
>
> fatal: Fatal errors only
> error: Errors only
> warning: Warnings
> info: Information about devices, files, configuration, operations
> debug: Verbose messages for developers
> trace: Very verbose code execution tracing"

> "--log-file <filepath>
>
> Set a file to output the log to."

> "--debug-jobs
>
> Enable time profiling for background jobs."

> "--cycles-print-stats
>
> Log statistics about render memory and time usage."

Source: the manual, https://docs.blender.org/manual/en/5.2/advanced/command_line/arguments.html
Date read: 2026-07-30

`-q` is the strongest of these for this question. Its existence, and its wording, tell us there is a class of output called "status printing" that a render run from a command line produces by default and that can be switched off, leaving warnings and errors behind. What that status printing consists of is not stated anywhere I could find.

---

### Piece 5 — The only place the sources say what render progress text contains

**LABEL: documented.**

One line in the Python API documentation describes the content of Blender's render statistics. It is attached to a handler, not to any particular way of starting a render.

Exact words:

> "bpy.app.handlers.render_stats
>
> on printing render statistics. Accepts one argument: the render progress as a string containing current frame, current sample, render time and saving time."

Source: the API documentation, https://docs.blender.org/api/5.2/bpy.app.handlers.html
Date read: 2026-07-30

So the render progress, where it is printed, is a single string carrying four things: current frame, current sample, render time, saving time. The phrase "on printing render statistics" ties the handler to an act of printing.

Two limits on this, both important. First, the page does not say *where* that string is printed, nor under which ways of starting a render the printing happens. Second, it does not give the string's shape — no separators, no field names, no example. Anyone hoping to read frame and sample numbers out of a line of text gets the list of ingredients and nothing else.

The neighbouring handlers are the other thing available at every route, and they are events rather than text:

> "bpy.app.handlers.render_init
>
> on initialization of a render job. Accepts one argument: the scene data-block being rendered"

> "bpy.app.handlers.render_pre
>
> on render (before). Accepts one argument: the scene data-block being rendered"

> "bpy.app.handlers.render_post
>
> on render (after). Accepts one argument: the scene data-block being rendered"

> "bpy.app.handlers.render_write
>
> on writing a render frame (directly after the frame is written). Accepts one argument: the scene data-block being rendered"

> "bpy.app.handlers.render_complete
>
> on completion of render job. Accepts one argument: the scene data-block being rendered"

> "bpy.app.handlers.render_cancel
>
> on canceling a render job. Accepts one argument: the scene data-block being rendered"

Source: the API documentation, https://docs.blender.org/api/5.2/bpy.app.handlers.html
Date read: 2026-07-30

The documentation does not say whether these fire under every way of starting a render.

---

### Piece 6 — What a render engine is given to report progress with

**LABEL: documented.**

The API documentation sets out the calls a render engine uses to report progress. These are the machinery behind whatever is displayed, whichever route started the render.

Exact words:

> "update_stats(stats, info)
>
> Update and signal to redraw render status text
>
> Parameters:
>
> stats (str) - Stats, (never None)
>
> info (str) - Info, (never None)"

> "update_progress(progress)
>
> Update progress percentage of render
>
> Parameters:
>
> progress (float) - Percentage of render that's done (in [0, 1])"

> "update_memory_stats(*, memory_used=0.0, memory_peak=0.0)
>
> Update memory usage statistics
>
> Parameters:
>
> memory_used (float) - Current memory usage in megabytes (in [0, inf], optional)
>
> memory_peak (float) - Peak memory usage in megabytes (in [0, inf], optional)"

Source: the API documentation, https://docs.blender.org/api/5.2/bpy.types.RenderEngine.html
Date read: 2026-07-30

Note the wording of `update_stats`: "signal to redraw render status text". Redrawing is a screen act. The documentation does not say what this call does when there is no screen. That silence is exactly the gap this question is aiming at.

The same page carries a comment inside its worked example that names two of the ways a render is started:

> "# This is the method called by Blender for both final renders (F12) and
> # small preview for materials, world and lights."

Source: the API documentation, https://docs.blender.org/api/5.2/bpy.types.RenderEngine.html
Date read: 2026-07-30

---

### Piece 7 — Started with Blender loaded as a Python module: one route loses the logging arguments

**LABEL: documented.**

When Blender is used as a Python module rather than run as a program, the arguments of Piece 4 are not available:

> "For the most part using Blender as a Python module is equivalent to running a script in background-mode
> (passing the command-line arguments --background or -b),
> however there are some differences to be aware of."

> "Command Line Arguments Unsupported
>
> Functionality controlled by command line arguments (shown by calling blender --help) isn't accessible."

> "Typically this isn't such a limitation although there are some command line arguments that don't have
> equivalents in Blender's Python API (--threads and --log for example)."

Source: the API documentation, https://docs.blender.org/api/5.2/info_advanced_blender_as_bpy.html
Date read: 2026-07-30

`--log` is named as having no equivalent. So on this route the logging categories that could carry render messages cannot be turned on at all.

The same page states a second difference, about stopping rather than reporting, which I include because it belongs to the same route and a reader of this answer will want it:

> "Signal Handlers
>
> Blender's typical signal handlers are not initialized, so there is no special handling for Control-C
> to cancel a render and a crash log is not written in the event of a crash."

Source: the API documentation, https://docs.blender.org/api/5.2/info_advanced_blender_as_bpy.html
Date read: 2026-07-30

---

### Piece 8 — Cycles is told which route started it

**LABEL: observed** (read out of the Cycles add-on's own files).

The Cycles add-on passes Blender's background flag into Cycles when it starts up.

Exact line, `Blender Sources/cycles_5.2/engine.py`, in `def init()`, line 52:

    _cycles.init(path, user_path, bpy.app.background)

And the flag it passes is documented as:

> "bpy.app.background
>
> Boolean, True when blender is running without a user interface (started with -b)"

Source: the API documentation, https://docs.blender.org/api/5.2/bpy.app.html
Date read: 2026-07-30

What this establishes is narrow and I will not stretch it: Cycles is *given* the fact of whether there is a user interface, at the moment it starts. Whether Cycles then reports its progress differently because of that is a step beyond the file, and the file does not take it. Any such claim would be **inferred**, and I am not making it, because I have no line of Cycles that acts on the flag — the Cycles files supplied here are the Python add-on plus the kernel and utility sources, and they do not contain the session or progress code.

**Piece 8b — the one printing switch Cycles exposes.**

**LABEL: observed.**

Exact lines, `Blender Sources/cycles_5.2/engine.py`, in `_configure_argument_parser()`, lines 13-15:

    parser.add_argument("--cycles-print-stats",
                        help="Print rendering statistics to stderr",
                        action='store_true')

and, in `_parse_command_line()`, lines 34-36:

    if args.cycles_print_stats:
        import _cycles
        _cycles.enable_print_stats()

Note that the add-on's own help text says the destination is **stderr**, while the manual's description of the same argument (quoted in Piece 4) says only "Log statistics about render memory and time usage" and does not name a destination.

Also observed, lines 27-29 of the same file, in `_parse_command_line()`:

    argv = sys.argv
    if "--" not in argv:
        return

So the switch is only looked for among the arguments that follow a bare `--`.

> **Corrected by the citation check (2026-08-11).** The line reference read 26-29 and the quotation carried a blank line between `argv = sys.argv` and `if "--" not in argv:`. In this version's file those three statements are at lines 27, 28 and 29 with no blank line between them; line 26 is the blank line that precedes them. The claim built on the quote is unaffected. That matches the manual's instruction that "Cycles add-on options must be specified following a double dash" (https://docs.blender.org/manual/en/5.2/advanced/command_line/arguments.html, read 2026-07-30). This is a route-dependent fact: the argument is a command-line thing, and there is no equivalent setting shown in the add-on's user-interface file.

File and place: `Blender Sources/cycles_5.2/engine.py`, lines 13-15, 27-29, 34-36 and 52.
Date read: 2026-07-30

---

### Piece 9 — The release notes name progress faults fixed in 5.2, and one of them is about route

**LABEL: documented**, but only as to the fact that an issue with that title was fixed. The notes give the bug report's title and a commit link, and say nothing about the behaviour before or after.

Under the heading "## Core: 18":

> " * Blender 5.1.1: Is Cycles render sample progress still accessible from Python without using -f? [[c065cc46602](https://projects.blender.org/blender/blender/commit/c065cc46602ad3d2dbe13814e1ef27cc8f8b5142)]"

Under the heading "## VFX & Video: 51":

> " * Render Sequence Animation stopped showing total progress [[efcc50166e8](https://projects.blender.org/blender/blender/commit/efcc50166e8fe14000a4c7e8ff7977a39d9889a2)]"

And under "## Render & Cycles: 50", which concerns something printed at the end of a run rather than during it:

> " * Blender generates fake "Full path tracing report" on exit with wrong numbers [[f521002b3f3](https://projects.blender.org/blender/blender/commit/f521002b3f30862d8093a77c7db41db2a13a775d)]"

Source: the release notes, https://developer.blender.org/docs/release_notes/5.2/bugfixes/
Date read: 2026-07-30

The first of these is the closest any source comes to the question. It is a question in a bug title, about whether Cycles sample progress reaches Python when a render is started other than with `-f`. It was fixed in 5.2. What the fixed behaviour is, the notes do not say, and I will not guess.

The introduction to the same page reads:

> "There are a total of 560 commits in 5.2 that fixed issues that existed in Blender 5.1 or older."

Source: the release notes, https://developer.blender.org/docs/release_notes/5.2/bugfixes/
Date read: 2026-07-30

One more entry, in the sequencer notes, concerns a progress bar for a different kind of render (audio), which I mention only so that its absence from my answer is deliberate rather than an oversight:

> "- The "Render Audio" operator now shows a realtime progress bar that can be cancelled (previously Blender would freeze until the operation finished). (blender/blender!155623)"

Source: the release notes, https://developer.blender.org/docs/release_notes/5.2/sequencer/
Date read: 2026-07-30

---

### Observed — what Blender 5.2 reports under each route (2026-08-11)

Run in Blender 5.2.0 on **the Creator's Mac (a MacBook Pro with an Apple M1 Pro chip, 16 GB of memory, macOS 26.5.2, Apple's own graphics and no second card)** and on **a rented Linux machine (Ubuntu 22.04.5 LTS on kernel 6.8, two NVIDIA RTX 4000 Ada Generation cards of 20 GB each on driver 580.105.08, eleven cores, 49 GB of memory)**, each with a settings folder of its own so nothing touched the Creator's own Blender settings. The scene was built by Blender 5.2 itself: a cube, a sphere, an area light and a camera; Cycles on the processor; 16 samples; adaptive sampling off; 160 by 120; output PNG. **LABEL: observed.** These take the eight gaps listed below in order.

**1. What a render prints, line by line, from a command line.** With no window and `-f 1` it prints fifty-five lines, of which exactly three carry a sample count:

```
00:00.658  render           | Fra: 1 | Mem: 1M | Sample 0/16
00:00.662  render           | Fra: 1 | Remaining: 00:00.06 | Mem: 2M | Sample 1/16
00:00.844  render           | Fra: 1 | Mem: 2M | Sample 16/16
```

The first line of the run is `00:00.633  blend            | Read blend: "…"` and the last is `Blender quit`. **With a window open, started from a terminal, no progress line reaches the terminal at all** — the capture holds the announcements and then `Saved:`. Both machines.

**2. Whether the Status Bar progress bar appears for a Python-started render.** It does. Every windowed run was started by `bpy.ops.render.render('INVOKE_DEFAULT', write_still=True)` — the operator call the F12 key is bound to — and the bar appeared each time, photographed with Blender's own screenshot command: one bar, a camera icon, the words **`Rendering...`**, a **whole-number percentage** inside the bar, and an **X** at the right end. Both machines: `Rendering...` beside `1%` and `48%` on the Mac, beside `26%` on the Linux machine.

**3. Whether the `render_stats` string is printed under every route.** In this version, yes — and that is not true of the three older versions. A handler attached to `bpy.app.handlers.render_stats` was called **forty-five times** for one frame with no window and **forty-four times** with a window. In Blender 4.5, 5.0 and 5.1 the same script with a window open produced **one** call, carrying only the closing line. Both machines.

**4. The shape of the string.** Two arguments are handed over, a string and `None`. The separator is ` | `. The first, a middle and the last of the forty-five, word for word:

```
STATS ('Mem: 0M | Synchronizing object | Cube', None)
STATS ('Remaining: 00:00.04 | Mem: 2M | Sample 1/16', None)
STATS ('Time: 00:00.22 (Saving: 00:00.06)', None)
```

**No frame number appears in any of the forty-five strings**, on either machine, although the API page describes the string as containing the current frame. The current sample appears only while a sampling stage is running, as `Sample <n>/<max>`. The render time and saving time arrive together in the last string only.

**5. Whether the four Render-menu commands differ.** A sequencer render drives the same status bar as an ordinary one, but with a different label: **`Rendering sequence...`**, where an ordinary render reads `Rendering...`. Both were caught mid-render on both machines. The label in the manual's one picture, `Sequence Render`, is neither of them. With no window a sequencer render prints `Rendering frame 1` and the ordinary three sample lines. A single frame and an animation are labelled the same and neither names the frame; an animation's percentage restarts at each frame rather than climbing across the range.

**6. Whether `update_stats` and `update_progress` do anything with no window.** They return. A tiny render engine registered from a script called `update_stats`, `update_progress` and `update_memory_stats`; all three returned without complaint, with a window and without, on both machines, and **none produced any output of its own** — only the plain lines printed beside each call appeared. So they are not window-bound, and they are not a printing channel either.

**7. What `-q` removes.** Three lines, and not the render: the `blend | Read blend:` line, the blank line before the end, and `Blender quit`. Every render line survives, including all three sample lines and the `Saved:` and `Time:` lines, and so does the version banner. The picture is still written and the run exits 0.

**8. Which logging categories exist.** `--log-list-categories` prints **149** names on the Creator's Mac and **183** on the rented Linux machine — so the list is longer on Linux — and **both `render` and `cycles` are in it** on both machines. `--log-level info` takes a plain run from fifty-five lines to 140 on the Mac; `--log "*"` gives 141.

**One thing happened on Linux and not on the Mac.** The windowed run with a `render_stats` handler attached ended in a **segmentation fault**, exit 139, after the handler had received all forty-four strings and after `render_complete` had fired. It wrote `/tmp/base_5.2.crash.txt`. It was repeated twice more and did the same both times. A windowed 5.2 render on that machine with **no** handler attached exited 0, and 4.5, 5.0 and 5.1 with the handler exited 0. On the Creator's Mac all four versions exited 0.

**A second pair of hands ran the same test on 2026-08-12 and did not see any of that. Both runs stand; neither is chosen.** The paragraph above is left exactly as its author wrote it. What follows is a separate run by somebody who was not in the room for it. **LABEL: observed.**

The repeat was asked for because the paragraph above does not say how its windowed Blender was made to end, and the exit status of a windowed run belongs to whoever ended it. Run on **the Creator's Mac (a MacBook Pro with an Apple M1 Pro chip, 16 GB of memory, macOS 26.5.2, Apple's own graphics and no second card)** and on **a rented Linux machine (Ubuntu 22.04.5 LTS on kernel 6.8, two NVIDIA RTX 4000 Ada Generation cards of 20 GB each on driver 580.105.08, an AMD Ryzen Threadripper PRO 3945WX with eleven cores, 49 GB of memory), whose window was opened on its own Xorg display**. Each version had a settings folder of its own, made before Blender was pointed at it, and its own copy of the scene. `/tmp` was emptied of `*.crash.txt` before every run.

**How the run was closed.** `bpy.ops.wm.quit_blender()` called straight from the start-up script did **not** close a windowed Blender on the rented Linux machine: the script printed `STILL HERE AFTER quit_blender()` and Blender was still going when a 120-second cap ended it, and again when a 240-second cap ended it — the status both times was 124, which belongs to `timeout` and not to Blender, and no crash file appeared. On the Creator's Mac the same call did close the run by itself, exiting 0, 24.7 seconds after the render in one run and 6.3 seconds after it in another. What closed the run on both machines every time was a timer: `bpy.app.timers.register` calling `bpy.ops.wm.quit_blender()` three seconds after the render.

**Closed by the timer, Blender 5.2 exited 0 on the rented Linux machine, three runs out of three** — no crash file in `/tmp`, nothing said about a segmentation fault, standard error empty. The whole of standard output from the first of the three, long paths shortened:

```
00:00.903  blend            | Read blend: "…/scenes/base_5.2.blend"
BACKGROUND=False
VERSION=5.2.0 LTS
00:01.302  render           | Saved: '…/out/e21_5.2_timer_.png'
HANDLER FIRED 45 TIMES
FIRST=('Mem: 0M | Synchronizing object | Cube', None)
LAST=('Time: 00:00.39 (Saving: 00:00.13)', None)
RENDER_COMPLETE FIRED 1 TIMES
QUITWAY=timer
TIMER REGISTERED FOR 3s
TIMER FIRED, CALLING quit_blender()
STILL HERE AFTER quit_blender() IN TIMER
00:04.314  reports          | ERROR Cannot open file /tmp/quit.blend@ for writing: Permission denied

Blender quit
```

The lines with no timestamp are the test script's own printing. **On the Creator's Mac, Blender 5.2 also exited 0, three runs out of three**, with no crash file and an empty standard error; the handler fired **46** times in the first run and **45** in each of the other two. At quit time the Mac printed `Saved session recovery to "/var/folders/…/T/quit.blend"` where the Linux machine printed the permission error above.

**So the two runs disagree on three things.** The earlier run: exit 139, a crash file, forty-four handler firings, on Linux only. This run: exit 0 on both machines, no crash file anywhere, forty-five firings on Linux and forty-six then forty-five then forty-five on the Mac. Blender 4.5, 5.0 and 5.1, run the same way on the same day, each exited 0 on both machines with the handler firing once.

**What the operator hands back.** Called plainly it returns `{'FINISHED'}` and blocks until the render is finished; started the way the F12 key starts it, `{'RUNNING_MODAL'}`, with the render going on afterwards. A picture was written in every case, this version's `save_output` setting standing at `True`.

**A render started from a script prints nothing.** With no window, `--python` calling `bpy.ops.render.render(write_still=True)` produced **no** sample line at all — only `Saved:`. Blender 4.5 prints the full progress on that route; 5.0, 5.1 and 5.2 do not. Run twice, once as an expression and once as a script file, on both machines.

### What the sources do not settle (the core of the answer)

**LABEL: not in the sources**, for the sources' own part. Each of these was searched for; the record of searching is below. **All eight have since been settled by running Blender**, in the observed section just above; what follows is the record of what the reading found, not a list of what is still unknown.

1. What a render prints, line by line, when started from a command line — in background or with a window. No page shows a sample of it. Neither of the two terminal screenshots in the manual shows a render running.
2. Whether the Status Bar progress bar and its time estimate appear when a render is started from Python inside a running Blender window, as opposed to from a menu or hotkey.
3. Whether the `render_stats` string is printed under every route, or only some.
4. The shape of the `render_stats` string — the separators, the labels, an example line.
5. Whether the four Render-menu commands (still, animation, sequencer still, sequencer animation) differ in what they report.
6. Whether `update_stats` and `update_progress` do anything at all when there is no window to redraw.
7. Whether `-q` removes render progress specifically, or some wider class of messages of which render progress is one part.
8. What `--log` category, if any, carries render progress. `--log "render,cycles"` is given as an example of the argument's syntax, not as a statement that those categories carry progress. The manual lists `--log-list-categories` — "List all available logging categories for --log, and exit" — which means the real list is obtainable only by running Blender, not by reading.

### Not covered

This answer does not settle what any route actually prints or displays, because no source states it. It does not cover render engines other than Cycles, EEVEE and Workbench, nor third-party engines, whose progress reporting is their own affair through the `RenderEngine` calls of Piece 6. It does not cover baking, which the Status Bar page names alongside rendering but which is a different job. It does not cover the compositor or the sequencer as separate reporters. It does not cover what happens when a render fails or is cancelled, beyond naming the handlers and controls that exist. And it treats "a render" as a final render throughout; the viewport preview shading modes, which draw continuously into the 3D Viewport, are not covered at all — the manual's one nearby line, "For the 3D Viewport, one sample is rendered at a time. When there are no changes to the scene or viewport the next sample will be rendered" (https://docs.blender.org/manual/en/5.2/render/workbench/sampling.html, read 2026-07-30), describes how sampling proceeds, not what is reported about it.

### Outside the interface

This is where the answer is weakest, and the weakness is the sources', not mine.

Only two of the pieces above are known to hold when Blender is driven from outside its own window.

- **Piece 4** holds by definition: `-q`, `--log`, `--log-level`, `--log-file`, `--debug-jobs` and `--cycles-print-stats` are command-line arguments and exist only on that route. `--cycles-print-stats` is **documented** as available on that route and **observed** in `engine.py` to require a preceding bare `--`.
- **Piece 8** holds: `bpy.app.background` is documented as "True when blender is running without a user interface (started with -b)", and `engine.py` is observed to pass that value to Cycles.

Piece 1 (the Status Bar progress bar and its time estimate) plainly does not hold outside the window — there is no Status Bar. Piece 2 (the Render In preference) does not hold either, for the same reason. Neither statement appears in the sources; both are **inferred** from the fact that the things described are parts of a window that is not there. That is a small step, but it is a step, so it is labelled.

For the pieces that matter most — Piece 5 (`render_stats`) and Piece 6 (`update_stats`, `update_progress`) — **the sources do not say** whether they behave the same outside the interface. `update_stats` is documented as "signal to redraw render status text", which implies a screen, but implying is not saying, and I will not turn it into a claim.

Piece 7 records the one documented difference of route: with Blender loaded as a Python module, `--log` has no equivalent.

**Observed (2026-08-11), and the two pieces that mattered most are now settled.** Piece 5, the `render_stats` handler, **fires in both cases in this version** — forty-five strings with no window, forty-four with one — where in Blender 4.5, 5.0 and 5.1 it fires once with a window. Piece 6, `update_stats` and `update_progress`, **work with no window**: both returned without complaint from a tiny render engine registered from a script, with a window and without, on both machines, and neither produced any output of its own, so `update_stats`'s "signal to redraw" is not a bar to using it with no screen. Piece 1 is borne out: with no window there is no status bar, and what stands in for it is a printed stream of three sample lines carrying a frame number, a `Remaining:` estimate and a memory figure. **A route the pieces above did not separate turns out to matter**: a render started from a script prints nothing but the file it saved, with a window or without, while `-f` and `-a` print the full progress.

**Added 2026-08-12, by a second pair of hands.** A windowed run started from a terminal, with the handler attached, ended cleanly on **both** machines — exit 0, no crash file, three runs each — when it was closed by a timer calling `bpy.ops.wm.quit_blender()`, and the handler fired forty-five times on the Linux machine and forty-six then forty-five then forty-five on the Mac. How such a run is closed is itself part of the answer: the same call made straight from the start-up script did not close a windowed Blender on the Linux machine within four minutes, and took between six and twenty-five seconds to close it on the Mac.

### Operating system

Nothing in Pieces 1 to 9 is stated to differ by operating system. Everything quoted above is written without an operating-system qualifier, except where noted below. The sources do, however, document three ways the operating system changes whether you can *see* what is printed.

- **Windows.** The console is hidden by default:

  > "When Blender is started on a Microsoft Windows operating system, the Console Window
  > (called the Command Prompt) is first created as a separate window on the desktop.
  > The main Blender window will also appear and the Console Window will then be toggled off.
  > To display the console again, go to Window - Toggle System Console."

  Source: the manual, https://docs.blender.org/manual/en/5.2/advanced/command_line/launch/windows.html, read 2026-07-30.

  And an argument exists to change that, Windows only:

  > "-con, --start-console
  >
  > Start with the console window open (ignored if -b is set), (Windows only)."

  Source: the manual, https://docs.blender.org/manual/en/5.2/advanced/command_line/arguments.html, read 2026-07-30.

- **macOS.** You must run the executable inside the application bundle to get the output at all:

  > "In order to run Blender you will have to specify that path to the Blender executable inside this folder,
  > to get all output printed to the terminal."

  Source: the manual, https://docs.blender.org/manual/en/5.2/advanced/command_line/launch/macos.html, read 2026-07-30.

- **Linux.** Running Blender from a terminal is described plainly, with a screenshot of the resulting output. I opened `_images/advanced_command-line_launch_linux_console.png`: it shows a terminal in which `blender` has been run and seven lines of audio-system messages have appeared. No render is running in it, so it settles nothing about render progress. (I also opened `_images/advanced_command-line_launch_windows_console.png`, which shows three message lines — the preferences file, the bundled Python and the blend-file being read, the first of them wrapping onto a second row — then "Blender quit". Again, no render.)

  > **Corrected by the citation check (2026-08-11).** The Linux picture's message count read "eight"; opening the picture gives seven: two "Cannot connect to server…" lines, one "jack server is not running…", two `JackShmReadWritePtr` lines and two `AL lib: (WW)` lines. The Windows picture's count read "five lines"; there are three message lines, of which the first wraps, so five rows of text appear below the typed command. Neither correction touches the point being made, that no render is running in either.

  Source: the manual, https://docs.blender.org/manual/en/5.2/advanced/command_line/launch/linux.html and .../windows.html, read 2026-07-30.

One operating-system limit on rendering itself, which bounds this question at the edges:

> "Headless Rendering
>
> Headless rendering is not supported on headless Windows systems."

Source: the manual, https://docs.blender.org/manual/en/5.2/render/eevee/limitations/limitations.html, read 2026-07-30. This is on the EEVEE limitations page, so it is about EEVEE.

**Observed (2026-08-11).** Settled for macOS and for Linux, and they agreed on every point in the observed section above: the same fifty-five printed lines, the same three sample lines, the same handler counts with and without a window, the same `Rendering...` and `Rendering sequence...` labels and whole-number percentages, the same three lines removed by `-q`, and the same behaviour from a render engine's own progress calls. **Two differences between the machines.** The windowed run with a `render_stats` handler attached ended in a segmentation fault on the Linux machine, three times out of three, and exited 0 on the Mac. And the list of logging categories is 149 entries on the Mac and 183 on the Linux machine. **The first of those two was not found again on 2026-08-12**: a second pair of hands ran the same test three times on each machine and got exit 0 and no crash file every time, on both. Both runs stand, side by side, and neither is chosen. **A different machine difference did show up in that later run**: `bpy.ops.wm.quit_blender()` called from a start-up script closed a windowed Blender on the Mac, after six seconds in one run and twenty-five in another, and did not close it on the Linux machine within four minutes; and at quit time the Mac printed `Saved session recovery to "/var/folders/…/T/quit.blend"` where the Linux machine printed `ERROR Cannot open file /tmp/quit.blend@ for writing: Permission denied`. **Windows is untested** — no Windows machine has run it — so the EEVEE line about headless Windows systems stands unchecked.

---

## Record of where I looked

This is not a footnote; it is part of the answer. Everything below was done against the 5.2 copies only.

**Searches run across the whole of `manual_5.2`, `api_5.2`, `release_notes_5.2` and `cycles_5.2` at once** (case-insensitive unless noted; the API's `genindex*` files were excluded from the hits because they are just an index):

`progress`, `progress bar`, `rendering progress`, `render progress`, `render statistics`, `current sample`, `sample count`, `Sample ` (with trailing space), `Fra:`, `Mem:`, `Time:`, `Elapsed`, `Remaining`, `Peak`, `Saved:`, `Append frame`, `Path Tracing Sample`, `Synchronizing`, `Loading render kernels`, `Rendering...`, `Waiting for render`, `stdout`, `stderr`, `terminal`, `Console Window`, `headless`, `batch render`, `render farm`, `estimate`, `time estimate`, `percentage of render`, `status printing`, `Suppress status`, `background`, `command line`, `status bar`, `console`, `log`, `while rendering`, `during rendering`, `during the render`, `render job`, `Render Window`, `render window`, `Display Mode`, `Keep User Interface`, `Render Image`, `Render Result`, `Viewport Render`, `update_progress`, `update_stats`, `F12`.

I searched the plain words (`progress`, `percent`, `estimate`, `remaining`) and the trade words (`sample`, `stderr`, `headless`, `render farm`, `batch`) and the words that would surround the thing if it existed (`Fra:`, `Mem:`, `Saved:`, `Peak`, `Elapsed`) — the last group on the reasoning that if the documentation ever quoted a line of render output verbatim, it would contain one of those fragments. Not one of them appears anywhere in the 5.2 manual, API or release notes. That is the backbone of the "not in the sources" answer.

**Pages opened in full and read:**

- Manual: `advanced/command_line/index.txt`, `advanced/command_line/render.txt`, `advanced/command_line/arguments.txt` (all 691 lines), `advanced/command_line/launch/linux.txt`, `.../macos.txt`, `.../windows.txt`, `interface/window_system/status_bar.txt`, `interface/window_system/topbar.txt` (Render menu and Window menu sections), `editors/preferences/interface.txt` (Temporary Editors section), `editors/3dview/viewport_render.txt`, `editors/info_editor.txt`, `editors/image/introduction.txt`, `editors/image/navigating.txt`, `editors/video_sequencer/sequencer/navigating.txt`, `render/index.txt`, `render/introduction.txt`, `render/output/introduction.txt`, `render/output/animation.txt`, `render/output/properties/output.txt`, `render/cycles/gpu_rendering.txt` (its "Frequently Asked Questions" section), `render/workbench/sampling.txt`, `render/eevee/limitations/limitations.txt`.
- API: `bpy.app.handlers.txt` (whole), `bpy.types.RenderEngine.txt` (whole), `bpy.ops.render.txt` (whole), `bpy.app.txt` (the `background` entry), `bpy.types.WindowManager.txt` (the `progress_begin` / `progress_update` / `progress_end` entries), `info_advanced_blender_as_bpy.txt`, `info_tips_and_tricks.txt` ("Use Blender without its User Interface"), `info_gotchas_operators.txt`.
- Release notes: the subject files `rendering.md`, `cycles.md`, `core.md`, `python_api.md`, `sequencer.md`, and `bugfixes.md` (its section headings, and the Core, Python API, Render & Cycles, and VFX & Video sections in full).
- Cycles files: `engine.py` (whole, 277 lines), `__init__.py` (its method list), and greps for `background`, `progress`, `stat`, `print` across `ui.py`, `properties.py`, `operators.py`, `maketx.py` and the `source/` tree.

**Pictures opened:** `_images/interface_window-system_status-bar_ui-center.png`, `_images/interface_window-system_status-bar_ui.png`, `_images/advanced_command-line_launch_linux_console.png`, `_images/advanced_command-line_launch_windows_console.png`. I also listed the whole `_images` folder (3,139 files) and searched its file names for `progress` and for `status-bar`; the four above are the only ones whose names suggest they might show a progress display or a console. None of the terminal screenshots shows a render in progress.

**Ruled out, and why:**

- `render/output/properties/format.txt`, `frame_range.txt`, `metadata.txt`, `post_processing.txt` — these describe what gets written and in what format, not what gets reported while working. Checked by search for the terms above; no hits.
- The Cycles `source/kernel` and `source/util` trees — hits for `current sample` there are code comments inside the path-tracing kernel about a sample's position and number, not about reporting anything to a user. The Cycles files supplied contain no session, progress or printing code, so there was nothing to read on this question beyond `engine.py`.
- `editors/info_editor.txt` — the Info editor logs operators, errors, warnings and informational messages. It says nothing about progress during a render, so it does not answer this question.
- The sequencer release note about the "Render Audio" progress bar — a different operator and a different kind of render. Quoted above only so its exclusion is visible.
- `bpy.types.WindowManager.progress_begin/update/end` — this is a progress-reporting facility for scripts to use, not something a render reports through. It is not part of the answer, but I checked it in case it was documented as the render's own channel. It is not.
- `bpy.ops.render.render()` in the API documentation — its description is the single line "Undocumented, consider contributing." It says nothing about progress. (Source: https://docs.blender.org/api/5.2/bpy.ops.render.html, read 2026-07-30.) I note this because it is the main Python route to starting a render, and the documentation for it is empty.

**I did not use the web, and did not need to.**

---

## Things a person with Blender open could settle in a few minutes

Twelve of the thirteen items that stood here have been run, on both machines, and their results are written into the answer above. Item 13, the comparison across operating systems, was run on the two the project has. What is left open is below.

1. **Windows (item 13).** No Windows machine has run it, so this answer covers macOS and Linux and leaves Windows open — including whether `-con` changes what is visible for a non-background run. **Untried rather than untestable: a Windows machine can be rented, and every one this project has read reported a Server edition — see the equipment section of `Preparation/Blender Research/Method.md`.**

2. **The time estimate that appears when the pointer rests on the progress bar.** It needs a hover, and neither machine has any way of moving the pointer without a hand on it, so it was dropped rather than run.

3. **Which key is bound to which render command (items 6 and 7).** With no window Blender's key configuration holds no items, so the bindings could not be read; the windowed runs above were started by the operator call the F12 key is bound to rather than by pressing a key.

4. **What an interactive viewport render or the Rendered shading mode reports.** Not tested.

5. **What a render started from Blender loaded as a Python module reports.** Neither machine has such a build.

2026-08-11 — Claude Opus 5 (Claude Code) — Reports, Blender 5.2

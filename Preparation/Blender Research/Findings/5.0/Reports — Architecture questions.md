# Reports — Blender 5.0 — questions 12 and 13

Sources consulted, all inside `Blender Sources/`, all read on **2026-07-30** (the snapshot date):
Blender's manual for 5.0 (`manual_5.0`, including two pictures from its `_images` folder), Blender's Python API documentation for 5.0 (`api_5.0`), the 5.0 release notes (`release_notes_5.0`), Cycles' own files as shipped with 5.0 (`cycles_5.0`), and the undated requirements page in `Blender Sources/requirements`. Nothing else.

I read only the question sheet and files inside Blender Sources/. I did not open any other part of this project.

I answered questions 12 and 13 only. I read the whole sheet first, so that I knew what the neighbouring questions were asking and did not answer them by accident. I did not use any helper agents; every search and every reading below is my own, so the declaration above covers the whole of this document.

---

## 12. What ending status does a run of Blender give back to whatever started it, for each of the ways a run can end?

**Short answer: mostly not in the sources.** The sources confirm that a run of Blender has an ending status and that a caller can set it for one narrow case. They never say what that status is for a run that finishes normally, for a run that is interrupted, or for a run refused because a file it was handed cannot be used. One Cycles file shows a status of 1 being asked for when Cycles is handed a device name it cannot use.

I have split the answer by the three endings the question names.

---

### 12a. Finishing normally

**LABEL: not in the sources.**

Nothing in the manual, the API documentation, the release notes or Cycles' files states what status a run hands back when it completes. The word "exit" appears in the manual exactly twice in a status sense, both inside the description of one command-line option (quoted under 12c below), and neither says what a normal finish produces.

The closest thing to a statement is a picture. The manual's Windows launching page carries a screenshot of a finished run:

- Picture: `_images/advanced_command-line_launch_windows_console.png`, shown on the manual page **https://docs.blender.org/manual/en/5.0/advanced/command_line/launch/windows.html** ("Blender's Console Window on Microsoft Windows."), read 2026-07-30.
- What is visible in it: a Command Prompt window in which `blender` was run, four lines of Blender output, then a blank line, then the line `Blender quit`, then a blank line, then the shell prompt `c:\blender>` again. **LABEL for this: documented** (it is part of the manual) — but what it documents is only that Blender prints `Blender quit` and that the shell prompt appears again after it. No status is shown, because the screenshot does not query one.

**Not covered.** Whether the status for a normal finish is zero. Whether a normal finish that produced warnings differs from one that did not. Whether finishing an animation with some frames unwritten counts as a normal finish.

**Outside the interface.** The whole question only has meaning outside the interface — a status is something a starting program receives — and the sources are silent for that case as much as any other. The picture above is of a run started from a terminal, which is the outside-the-interface case, and it still shows no status.

**Operating system.** The sources say nothing that would differ by operating system here, because they say nothing at all. The one picture is from Microsoft Windows; there is a matching picture from Linux (`_images/advanced_command-line_launch_linux_console.png`, on **https://docs.blender.org/manual/en/5.0/advanced/command_line/launch/linux.html**) which shows a run still in progress and likewise no status.

---

### 12b. Being interrupted

**LABEL: not in the sources.**

Nothing anywhere in the four sources connects an interruption to an ending status. The manual describes interruption only as something that happens inside the window: "As with a normal render, you can abort it with Esc." (manual, **https://docs.blender.org/manual/en/5.0/editors/3dview/viewport_render.html**, read 2026-07-30), and the status bar "task can be aborted by clicking the cancel button" (manual, **https://docs.blender.org/manual/en/5.0/interface/window_system/status_bar.html**, read 2026-07-30). Neither is about a status handed back to a starting program.

The only mention of interruption from outside the window is in the API documentation, and it is about a Python script rather than a run of Blender:

> "If the script runs for too long or you accidentally enter an infinite loop,
> Ctrl-C in the terminal (Ctrl-Break on Windows) will quit the script early."

Source: Python API documentation, **https://docs.blender.org/api/5.0/info_tips_and_tricks.html**, read 2026-07-30.

That sentence says the script stops, not that Blender stops, and says nothing about a status.

**Not covered.** What status an interrupted run gives back. Whether an interruption partway through an animation differs from one during a single frame. Whether the two Windows key combinations named above behave alike. Whether an interruption that Blender catches differs from one it does not.

**Outside the interface.** The one sentence above is explicitly about the terminal, so it is the outside-the-interface case, and it still does not name a status. The `Esc` and cancel-button routes are inside the interface only and cannot answer the question.

**Operating system.** The sources give one operating-system difference in how an interruption is sent — "Ctrl-C in the terminal (Ctrl-Break on Windows)" — but since no status is stated for either, nothing follows about statuses differing.

---

### 12c. Being refused because something it was handed cannot be used

**LABEL for 12c as a whole: not in the sources**, with one narrow exception. Nothing anywhere states the status a run gives back when the blend-file, the output path, the frame range, the scene name or the engine name it was handed cannot be used. The one case the sources touch at all is a rendering device name Cycles cannot set, and that is the third item below.

Here the sources do say something, in four separate places, none of which is the full answer.

**First — the one place the manual names an ending status at all. LABEL: documented.**

> "--python-exit-code <code>
>
> Set the exit-code in [0..255] to exit if a Python exception is raised
> (only for scripts executed from the command line), zero disables."

Source: manual, **https://docs.blender.org/manual/en/5.0/advanced/command_line/arguments.html**, read 2026-07-30.

What this settles: a run of Blender can end with a numbered status, at least for the one ending this option covers; that number can be set to anything from 0 to 255; and the caller chooses it, for a Python exception raised by a script the caller passed in. That *every* run of Blender hands back a status is a step past these words, and I am not taking it here.

"zero disables" says what setting the option to zero does. It does not say what applies when the option is not given at all. Reading it as *off unless the caller turns it on* is **inferred**, the step being that zero is what the option holds when nobody sets it.

What it does not settle: what the status is when the caller does not use this option; what the status is for any ending other than a Python exception; and whether "something it was handed cannot be used" (a missing blend-file, an unusable output path, an unknown engine name) counts as a Python exception at all — the sources never say it does, and I am not assuming it.

**Second — a status named in the Python API, for custom commands only. LABEL: documented, for what it actually covers.**

> "execute (callable) – Callback, taking a single list of strings and returns an int.
> The arguments are built from all command-line arguments following the command id.
> The return value should be 0 for success, 1 on failure (specific error codes from the os module can also be used)."

Source: Python API documentation, `bpy.utils.register_cli_command`, **https://docs.blender.org/api/5.0/bpy.utils.html**, read 2026-07-30.

This is the value a Python callback hands back when it implements a custom sub-command. The quoted words do not name the argument that reaches such a command; they say only that the arguments come from whatever follows the command id. The sentence tells us the convention that applies to the returned number: zero means success, one means failure, and codes from Python's `os` module may be used instead.

Note the limits carefully. This is a callback's return value, not stated anywhere to be the process status a starting program receives. Believing it becomes that status is a step past the words — **inferred**, and the step is: the two conventions it names (0 for success and 1 for failure; "specific error codes from the `os` module", which are process exit codes) are the conventions of a process ending status, so the number is very likely passed outward. The sources do not say so. It also has nothing to do with an ordinary render run, which does not go through a custom command.

**Third — Cycles' own file, for one kind of unusable thing. LABEL: observed.**

The Cycles add-on parses its own command-line arguments. When it is handed a rendering device it cannot set, it asks Python to end the process with the number 1:

```
    if args.cycles_device:
        import _cycles
        if not _cycles.set_device_override(args.cycles_device):
            sys.exit(1)
```

Source: Cycles' own files, file `cycles_5.0/engine.py`, lines 38–41, inside the function `_parse_command_line()`. Read 2026-07-30.

The surrounding lines show what device names are accepted:

```
    parser.add_argument("--cycles-device",
                        help="Set the device to use for Cycles, overriding user preferences and the scene setting."
                             "Valid options are 'CPU', 'CUDA', 'OPTIX', 'HIP', 'ONEAPI', or 'METAL'."
                             "Additionally, you can append '+CPU' to any GPU type for hybrid rendering.",
                        default=None)
```

Source: `cycles_5.0/engine.py`, lines 16–20. Read 2026-07-30.

What the file shows, and nothing more: the code asks for the number 1 when the device override cannot be set. That the whole run therefore ends and the starting program receives 1 is **inferred** — the step is that `sys.exit(1)` in Python raises a request to end with status 1, and the API documentation states that this ends Blender:

> "Some Python modules will call sys.exit() themselves when an error occurs,
> while not common behavior this is something to watch out for because it may seem
> as if Blender is crashing since sys.exit() will close Blender immediately."

Source: Python API documentation, **https://docs.blender.org/api/5.0/info_gotchas_crashes.html**, read 2026-07-30.

Even with that, the last link — that "closes Blender immediately" means the starting program is handed the number 1 rather than something else — is not stated anywhere I read. So: strong, but inferred.

**Fourth — one release note, which mentions a refusal-like case but names no status. LABEL: documented, but it does not answer the question.**

> "Warning when starting with non-blend file argument."

Source: release notes, **https://developer.blender.org/docs/release_notes/5.0/user_interface/**, read 2026-07-30.

It says a warning appears when Blender is started with a file that is not a blend-file. It calls it a warning, not a refusal, and says nothing about a status.

**Not covered for 12c.** What status Blender gives back when the blend-file it was told to open is missing, unreadable or from a version it cannot open. What status it gives back when the output path, the frame range, the scene name or the engine name is unusable. Whether Cycles' `sys.exit(1)` is reached at all in a run where the device is set in the file rather than on the command line. Whether the number in `--python-exit-code` also applies to non-Python failures (nothing suggests it does).

**Outside the interface.** All four findings are about a run started from a command line, which is the outside-the-interface case. The `--python-exit-code` option is explicitly limited to it — "only for scripts executed from the command line". Cycles' argument parsing runs only when `--` appears in the arguments (`cycles_5.0/engine.py`, lines 27–29: `argv = sys.argv` / `if "--" not in argv:` / `return`) — **LABEL: observed**, the same as the other readings of that file. That this makes it a command-line-only thing is **inferred**, the step being that those arguments are the ones a run is started with. None of this has any meaning inside the interface, where nothing is waiting for a status.

> **Brought up to date on 2026-08-26 by hand test.** The status a run hands back was measured on two machines in this version, and all of it was measured from outside Blender's own window, which is the only place a status exists. See "Answered by hand test — question 12" at the end of this question's material.


**Operating system.** The sources state no operating-system difference for any of the four. The Cycles file is one file used on every system, with no per-system branch around the `sys.exit(1)`; that it therefore behaves the same everywhere is **inferred**, the step being that the same Python line runs on every system, and it is weakened by the fact that the device names in question (`CUDA`, `OPTIX`, `HIP`, `ONEAPI`, `METAL`) are not all available on every system, so which names fail differs even if what happens after a failure does not.

> **Brought up to date on 2026-08-26 by hand test.** macOS and Linux were both measured in this version and agree almost everywhere; the places they do not are named in "Answered by hand test — question 12" below. Windows remains untested: no Windows machine has run it. **Untried rather than untestable: a Windows machine can be rented, and every one this project has read reported a Server edition — see the equipment section of `Preparation/Blender Research/Method.md`.**


### Where I looked, for question 12

Searched the whole of `manual_5.0`, `api_5.0` and `release_notes_5.0` at once, case-insensitively, for: `exit code`, `exit status`, `exit value`, `exit-code`, `exitcode`, `return code`, `status code`, `errorlevel`, `EXIT_SUCCESS`, `EXIT_FAILURE`, `exits with`, `non-zero`, `nonzero`, `return value`, `subprocess`, `Popen`, `os.system`, `shell script`, `batch file`, `batch script`, `$?`, `2>`, `>&`, `SIGINT`, `SIGTERM`, `signal`, `interrupt`, `Ctrl-C`, `Ctrl+C`, `quit`, `abort`. Searched `cycles_5.0` for `sys.exit`, `exit(`, `returncode`, `return code`.

Ruled out and why:
- **`--debug-exit-on-error`** ("Immediately exit when internal errors are detected", same manual page) — says when Blender exits, not with what status.
- **Operator return values** (`{'FINISHED'}`, `{'CANCELLED'}`) in `bpy.types.Operator` and `bpy.ops` — these are what one operator hands back to Blender inside a running session; the question explicitly asks for the status a program that started Blender receives, so these are the wrong thing. Ruled out deliberately, not overlooked.
- **`bpy.ops.wm.quit_blender()`** (**https://docs.blender.org/api/5.0/bpy.ops.wm.html**) — documented as "Quit Blender", with no arguments and no status mentioned.
- **`bpy.app`** — read for anything status-like; it has `use_userpref_skip_save_on_exit` and `background`, neither of which is an ending status.
- **The extension sub-commands page** (**https://docs.blender.org/manual/en/5.0/advanced/command_line/extension_arguments.html**) — searched for `exit`, `error`, `fail`, `status`; the only hits are thirteen copies of the boilerplate "show this help message and exit".
- **The release notes as a whole** — searched for the terms above; no hits about statuses. The `core.md` page's command-line section is about `--gpu-vsync` and about replacing `--debug-cycles`, `--debug-ffmpeg` and `--verbose` with logging flags; nothing about statuses.
- **`cycles_5.0/source` and `cycles_5.0/shader`** (269 and 120-odd files) — these are the rendering kernel's headers and shader definitions. Searched them for the terms above; the only `exit`-shaped hits are unrelated maths and debug-print helpers. They contain no program start-up or shut-down code, so they cannot carry an ending status.

---


## Answered by hand test — question 12, 2026-08-26

**LABEL: observed.** Everything in this section is what two real machines gave back when the question was put to them directly. None of it comes from documentation, and none of it was known before the runs.

### What was done, and on what

Two machines, and their results are never merged.

- **The Creator's Mac** — a MacBook Pro with an Apple M1 Pro processor, macOS 26.5.2, Apple's own graphics and no second graphics card. Blender 5.0.1, build hash `a3db93c5b259`, built 2025-12-16 01:51:40, run from `/Applications/Blender 5.0.1.app/Contents/MacOS/Blender`.
- **The rented Linux machine, in Norway** — a Docker container holding half its host, running Ubuntu 22.04.5 LTS on kernel `6.8.0-136-generic`, with one NVIDIA GeForce RTX 3060 of 12,288 MiB on driver 595.84, about fifteen cores of an Intel Xeon E5-2697A v4, and a display server of its own on `DISPLAY=:1`. Its viewport is drawn by its processor, not by the card, and it is used as `root`. Blender 5.0.1, build hash `a3db93c5b259`, built 2025-12-16 01:30:59, run from `/opt/blender/5.0.1/blender`.

Both machines carry the same build hash for this version. **This was read from each machine rather than assumed.** The brief for this round warned that the two machines might carry different builds of Blender 5.2, so the hash was read off every version on both machines before anything was run; on 2026-08-26 the two machines reported the same hash for all four versions, 5.2 included.

### How every run was made

Every run used a settings folder of its own — `export BLENDER_USER_RESOURCES=<work folder>/cfg` — so nothing touched the Creator's own Blender settings, and no add-on was loaded. Every scene, script, output and scratch file was written outside this project.

**Every blend-file was built in this very version**, by one script run four times, so that this version was never asked to read a file another version wrote. The scenes are a cube with a camera and a light rendering in Cycles on the processor at eight samples and 64 by 64 (`good.blend`); the same at 800 by 800 and a million samples with adaptive sampling off, so one frame runs for hours (`slow.blend`); a cube and a light with no camera (`nocamera.blend`); a cube whose material has an Image Texture node pointing at `missing_tex.png` (`tex_missing.blend`); a collection linked from `lib.blend` (`lib_missing.blend`); a Video Sequencer image strip pointing at `missing_seq.png` (`seq_missing.blend`); and a text object using `missing_font.ttf` (`font_missing.blend`). A file is made missing by moving it aside immediately before the run and moving it back immediately after.

**Both streams were always sent to two different files in the same run** — `> out.txt 2> err.txt` — and read separately, so an empty stream is recorded as an empty stream rather than assumed. Paths in the quotations below are shortened to `<work folder>` and `<scene folder>`; nothing else in any quoted line is changed.

The number reported below is the number the shell holds immediately after the run — read with `echo $?` in an ordinary run, and read from the process's wait status where a signal was involved, in which case both the signal and the number a shell would report are given.

### Finishing normally

Three commands, on both machines:

```
blender -b good.blend -o <work folder>/f_#### -f 1
blender -b good.blend -o <work folder>/a_#### -a
blender -b good.blend
```

**All three gave 0**, on both machines. The single frame wrote its picture; the animation wrote its three; the third rendered nothing at all and still gave 0.

### Being interrupted

**An interrupt typed at a terminal.** The run was started inside a real terminal made with `pty.fork()` and the interrupt character, byte `0x03`, was written into that terminal twelve seconds later — which is exactly what the terminal receives when a person presses Ctrl-C. The scene was `slow.blend`, which runs for hours on one frame.

**One press gave 1**, on both machines, for a single frame and for an animation alike, and **no picture file of any kind was left behind**. What the terminal showed:

```
^C
Sent an internal break event. Press ^C again to kill Blender

Blender quit
```

and just before that, among the progress lines, a stage line ending `| Cancelled`.

**Two presses, three seconds apart.** **Here the two machines part company.** On the Creator's Mac, 5.0 ended on signal 11, `SIGSEGV`, which a shell reports as **139** — three runs out of three, plus the first run, four in all. On the rented Linux machine the same thing gave exit code **2**, three runs out of three plus the first, four in all.

On the Mac the terminal showed:

```
^C
Sent an internal break event. Press ^C again to kill Blender
^C
Blender killed
Error: Not freed memory blocks: 33632, total unfreed memory 49.298939 MB
Writing: /var/folders/n3/z3rfjt654rn1k1x6f_1gk4540000gn/T/slow.crash.txt
```

That last line names a crash file, which is consistent with the signal. On Linux the same run stops after `Blender killed` and the unfreed-memory line, with no crash file named and exit code 2. **The disagreement is recorded, not explained.**

**A signal sent from another program.** The run was started with its two streams sent to files, and twelve seconds later a signal was sent to its process number from a separate program. On both machines, in this version:

| Signal sent | What came back |
| --- | --- |
| `SIGINT` (`kill -INT`), single frame | exit code **1** — the same as a typed Ctrl-C |
| `SIGINT`, during an animation | exit code **1** |
| `SIGTERM` (`kill -TERM`) | killed by signal 15; a shell reports **143** |
| `SIGKILL` (`kill -9`) | killed by signal 9; a shell reports **137** |

No picture file was written in any of them. So Blender catches `SIGINT` and turns it into the same orderly ending a person gets by typing Ctrl-C, and does not catch `SIGTERM`.

### Being refused something it was handed

**A blend-file that is not there.** Status **1** on both machines, and the complaint on standard output with standard error empty. **A file that exists but is not a blend-file** — a line of ordinary text renamed — also gave **1**, saying `File format is not supported in file "<path>"`. Both are quoted in full under question 13 below, where the stream matters.

**The manual's own unknown-argument example, `blender -ba good.blend`.** **Blender 5.0 does not end.** The run was still going after ninety seconds, on both machines, twice each, and the test harness had to kill it with `SIGKILL`. There is no ending status to report.

Standard output, two lines:

```
00:00.464  reports          | ERROR Cannot read file "<work folder>/-ba": No such file or directory
00:00.464  blend            | Read blend: "<scene folder>/good.blend"
```

Standard error, one line:

```
unknown argument, loading as file: -ba
```

Blender treats `-ba` as a file to open, fails to open it, opens the blend-file that followed instead, and carries on running. Blender 4.5 given the same command prints a second line on standard error — `Error: argument has no '.blend' file extension, not using as new file, exiting!` — and stops with status 1. **That line is gone in 5.0, and so is the stopping.**

**An argument Blender has never heard of, after a valid one** — `blender -b good.blend --no-such-flag -o <work folder>/f_#### -f 1`. Status **1** on both machines, nothing written. Blender says `unknown argument, loading as file: --no-such-flag` on standard error, then fails to read a file of that name, and stops because the file it was told to open cannot be read.

**An engine name that does not exist**, `blender -b good.blend -E NOSUCHENGINE -o <work folder>/x_#### -f 1`. **Status 1**, immediately, on both machines, four runs each. Standard output held two lines and standard error one:

```

Error: engine not found 'NOSUCHENGINE'
```

Blender 4.5 given the same command does not do this: on the Mac it ends on `SIGABRT` and on the rented Linux machine it never ends at all.

**A render device.** Five runs, each as `blender -b good.blend -o <work folder>/x_#### -f 1 -- --cycles-device <name>`:

| Name given | The Creator's Mac | The rented Linux machine |
| --- | --- | --- |
| `NOTADEVICE` | **1** | **1** |
| `OPTIX` | **1** | **0**, and it rendered |
| `CUDA` | **1** | **0**, and it rendered |
| `METAL` | **0**, and it rendered | **1** |
| `HIP` | **1** | **1** |

So a name that is not on Cycles' list and a name that is on it but has no hardware behind it **both give 1**, and which names have hardware behind them is what separates the two machines. In 5.0 the refusal of a name that is not on Cycles' list lands on **standard output**, and standard error is empty:

```
00:00.244  cycles           | ERROR NOTADEVICE is not a valid Cycles device.
Blender 5.0.1 (hash a3db93c5b259 built 2025-12-16 01:51:40)
Error: Not freed memory blocks: 1, total unfreed memory 0.000004 MB
```

The refusal of a name that is on the list but has no hardware behind it also lands on standard output:

```
00:00.279  reports          | ERROR Found no Cycles device of the specified type
```

In Blender 4.5 the first of those two is on standard error instead.

**A Python script that fails.** The script is one line, `raise Exception("boom")`.

- `blender -b good.blend --python boom.py -o <work folder>/x_#### -f 1` gave **0** on both machines — and **the render went ahead and the picture was written** even though the script had failed. The traceback landed on **standard error**.
- The same with `--python-exit-code 42` added **before the `--python` argument** gave **42** on both machines, and the render did not happen. Standard error carried the traceback and then one more line:

```
Error: script failed, file: '<path>/boom.py', exiting.
```

> **Corrected on 2026-08-26 by the citation check.** This entry read "the same with `--python-exit-code 42` added", without saying where in the command the option goes. Re-running it showed that the position decides the answer: put before `--python` (or before `--python-expr`) the run gives 42 and writes nothing, exactly as recorded; put after it, the script has already run and failed by the time the option is read, and the run gives **0** and writes its picture, on both machines and in all four versions. The command has been made explicit. Nothing else in the result changed.

- The same two runs with `--python-expr "raise Exception('boom-expr')"` in place of the script file gave the same two numbers, **0** and **42**.

So the option does what the manual says it does, and **without it a failed script leaves the run at 0**.

**A command registered from Python.** A script registering two commands through `bpy.utils.register_cli_command`, one whose callback returns `7` and one whose callback returns `0`:

```
blender --python reg_cmd.py --command smoozyfail
blender --python reg_cmd.py --command smoozyok
```

The first gave **7** and the second gave **0**, on both machines. **So the number a registered command's callback returns does become the run's ending status, and it is not limited to 0 and 1.** This is the step every reading of this question in the sources refused to take, and it holds.

**Cycles' own `maketx` command.** **There is no such command in 5.0.** Both runs gave **1**, with nothing on standard output and one line on standard error:

```
Unrecognized command: "maketx"
```

`blender --command help` in 5.0 lists one command:

```
Blender Command Listing:
	extension
```

**A scene with no camera**, `blender -b nocamera.blend -o <work folder>/nc_#### -f 1`. Status **1** on both machines, nothing written:

```
00:00.268  blend            | Read blend: "<scene folder>/nocamera.blend"
00:00.272  render           | Rendering single frame
00:00.272  render           | Rendering frame 1
00:00.272  reports          | ERROR Cannot render, no camera
Blender 5.0.1 (hash a3db93c5b259 built 2025-12-16 01:51:40)

Blender quit
```

**An output folder that cannot be written to** — made with `mkdir ro` and `chmod 500 ro`. On the Creator's Mac, status **1** in 5.0, with nothing written, and **both lines on standard output while standard error stayed empty**:

```
00:00.316  image.write      | ERROR OpenImageIO write failed: Could not open file "<work folder>/ro/p_0001.png"
00:00.316  reports          | ERROR Render error (Undefined error: 0) cannot save: '<work folder>/ro/p_0001.png'
00:00.316  render           | Time: 00:00.04
```

Blender 4.5 given the same run says `(Permission denied)` where 5.0 says `(Undefined error: 0)`, and puts a bare `<path>: Permission denied` on standard error, which 5.0 does not.

**On the rented Linux machine the same run gave 0 and wrote the picture**, in this version and in all four. That machine is used as `root`, and `root` is not stopped by the folder's permissions. **That is a difference between the machines, and a difference in how they are used rather than in Blender.**

**An output path of `//`**, `blender -b good.blend -o // -f 1`. Status **0** on both machines. It is not a refusal at all: the picture is written into the folder the blend-file sits in, as `0001.png`.

### The trap — runs that render nothing and still end at 0

Each of these was run with its own output folder, emptied immediately beforehand, so that "nothing was written" is a fact about that folder. On both machines, in this version:

| What was run | Status | Was a picture written |
| --- | --- | --- |
| `blender -b good.blend -o <d>/f_#### -f 1` (the control) | 0 | yes |
| `blender -b good.blend -o <d>/f_#### -f abc` | **0** | **no — the folder was empty** |
| `blender -b good.blend -o` | **0** | **no — the folder was empty** |
| `blender -b good.blend --python-exit-code notanumber -o <d>/f_#### -f 1` | 0 | yes |
| `blender -b good.blend --no-such-flag -o <d>/f_#### -f 1` | 1 | no |
| `blender -b good.blend -o <d>/f_#### -s 10 -e 5 -a` | **0** | **no — the folder was empty** |
| `blender -b tex_missing.blend -o <d>/f_#### -f 1`, texture moved away | **0** | yes, with a magenta hole in it |
| saving the render output switched off | see below | see below |
| `blender -b nocamera.blend -o <d>/f_#### -f 1` | 1 | no |

**So the suspicion is confirmed, and it is not one case but several.** A frame argument that is not a number, an option whose value is missing, and an animation whose start frame is after its end frame each **end at 0 having rendered nothing whatever**. In each of the first two, Blender does say something — `Error: not a number '-f / --render-frame abc'.` and `Error: you must specify a path after '-o  / --render-output'.` — but it says it on **standard error** and the status stays 0.

In 5.0 there is no such setting. The Python expression came back with `NO save_output IN THIS BUILD: 'RenderSettings' object has no attribute 'save_output'`, the render went ahead, and the picture was written. Status 0.

**The reversed frame range.** In 5.0 the run does announce the range it was given, on standard output:

```
00:00.258  blend            | Read blend: "<scene folder>/good.blend"
00:00.262  render           | Rendering animation (frames 10..5)
Blender 5.0.1 (hash a3db93c5b259 built 2025-12-16 01:51:40)

Blender quit
```

It says it is rendering frames 10 to 5, renders none of them, writes nothing, and ends at 0.

### The two standing lines, brought up to date

**Outside the interface.** Settled by measurement, for this version, on both machines. Every number above was got from a run started and watched by another program, with no window, which is the only setting in which an ending status exists at all. The window runs described under question 13 show the other side of the same coin: a render invoked from inside a window hands its outcome back to the Python that called it — `RuntimeError('Error: No camera found in scene "Scene"\n')` — and the *program's* status has nothing to do with it, because the program is still running.

**Operating system.** Settled for macOS and for Linux, and only for those two. Every result above was got on both, and **they agree everywhere except in two places**, each named where it occurs: a second interrupt typed at a terminal, where the Mac ends on `SIGSEGV` and writes a crash file while the Linux machine gives exit code 2; and an unwritable output folder, where the Linux machine is used as `root` and is therefore not stopped by the folder's permissions. Everything else — the clean finishes, the single interrupt, the signals, the refused files and arguments, the devices, the Python failures, the registered commands and the whole of the trap — came back the same on both. **Windows is untested and stays untested**: no Windows machine has run it, so what `echo %ERRORLEVEL%` or `$LASTEXITCODE` would report, and whether Ctrl-Break behaves as Ctrl-C does, are not settled by anything here.

### What these runs do not settle

- Anything on Microsoft Windows.
- What a working HIP or oneAPI device gives back. Both were refused for want of an AMD or an Intel card, so the *refusal* is settled and the success is not.
- What Blender loaded as a Python module hands back. Neither machine has such a build.
- Whether an unwritable output folder gives 1 on Linux, which needs an account that is not `root`.
- Why Blender 4.5 behaves as it does when handed an engine name that does not exist. What happened is recorded; nothing here explains it.

## 13. When a file the scene points at is not there, what does a render put on each of its output streams?

**LABEL: not in the sources.**

Nothing in the manual, the Python API documentation, the 5.0 release notes or Cycles' own files says what a render writes to its ordinary output stream and what it writes to its error stream when a file the scene points at is missing. The sources do not make that split for a missing file, and — with a single exception noted below — they do not name those two streams anywhere at all.

**The search that shows the silence.** I searched the whole of `manual_5.0`, `api_5.0` and `release_notes_5.0`, case-insensitively, for `stderr`, `stdout`, `standard output`, `standard error`, `output stream`, `error stream`, `redirect`, `pipe`, `piped`, `2>`, `>&`. Across all three sources there is exactly one hit, and it is a false one: the manual's Color Balance node page uses the phrase "Standard output image." about a picture (**https://docs.blender.org/manual/en/5.0/compositing/types/color/adjust/color_balance.html**). `stdout` and `stderr` do not appear in the API documentation at all, not even as `sys.stdout` or `sys.stderr`.

**The one exception, and it is not about missing files. LABEL: observed.** In all four sources, one line names a stream. It is in Cycles' own argument parser, and it concerns statistics, not files:

```
    parser.add_argument("--cycles-print-stats",
                        help="Print rendering statistics to stderr",
                        action='store_true')
```

Source: Cycles' own files, file `cycles_5.0/engine.py`, lines 13–15, inside `_configure_argument_parser()`. Read 2026-07-30.

I include it only because it is the sole evidence in my sources that Blender's own documentation and code distinguish the two streams at all. It says nothing about a missing file. Nothing else in `cycles_5.0` names a stream except a comment in `source/util/defines.h` about a Metal environment variable, which concerns shader debug messages on Apple hardware and not file loading.

**What the sources do say about a missing file — none of it stream-level.** I list these so it is clear what was checked and rejected as an answer, not because any of them answers the question.

- The manual's Report Missing Files command puts its result in the window, not on a stream:
  > "This option is useful to check if there are links to unpacked files that no longer exist.
  > After selecting this option, a warning message will appear in the Info editor's header.
  > If no warning is shown, there are no missing external files."

  Source: manual, **https://docs.blender.org/manual/en/5.0/interface/window_system/topbar.html**, read 2026-07-30.

- A missing linked blend-file produces stand-in data rather than a message:
  > "If Blender cannot find a library while loading a blend-file,
  > it will create placeholder data-blocks to replace missing linked ones."

  Source: manual, **https://docs.blender.org/manual/en/5.0/files/linked_libraries/link_append.html**, read 2026-07-30. The same fact appears in the API documentation as the property `ID.is_missing`: "This data-block is a place-holder for missing linked data (i.e. it is [an override of] a linked data that could not be found anymore)" (**https://docs.blender.org/api/5.0/bpy.types.ID.html**, read 2026-07-30).

- Missing footage in the video sequencer changes the picture, not any stream:
  > "Render missing images/movies with a solid magenta color.
  > When disabled, missing content will render fully transparent."

  Source: manual, **https://docs.blender.org/manual/en/5.0/editors/video_sequencer/preview/sidebar.html**, read 2026-07-30. This is the "Show Missing Media" setting in the sequencer's preview sidebar.

- The Python API says errors reach the terminal, without saying which stream, and without reference to missing files:
  > "The error traceback is printed in full to the terminal which won't always generate an report message in
  > Blender's user interface (depending on how the script is executed)."

  Source: Python API documentation, **https://docs.blender.org/api/5.0/info_tips_and_tricks.html**, read 2026-07-30.

**What 5.0's release notes say about where messages go.** The nearest the sources come to describing the shape of a render's output is the 5.0 logging change, and it names a file, never a stream:

> "Most of Blender now uses the same logging system,
> including operators reports and background render progress.
> The output format has changed, and log levels are easier to control."

Source: release notes, **https://developer.blender.org/docs/release_notes/5.0/core/**, read 2026-07-30. That page also gives an example of the new format, in which every line is a time, a category and a message — for example `00:00.088  blend            | Read prefs: "..."` and `00:28.631  render           | Saved: '/tmp/0001.png'`. None of the example lines is a missing-file line, and the page does not say which stream any line goes to.

The only statement anywhere about steering that output is a command-line option, and it steers it to a file:

> "--log-file <filepath>
>
> Set a file to output the log to."

Source: manual, **https://docs.blender.org/manual/en/5.0/advanced/command_line/arguments.html**, read 2026-07-30.

**Pictures checked. LABEL for both readings: documented** — a picture in the manual is part of the manual, the same as the picture read under 12a. Two manual screenshots show Blender's console output, and I opened both in case they settled which stream carries what. They do not. That a screenshot of a terminal cannot show which stream a line arrived on is **inferred**, the step being that both streams land in the same window; nothing in the sources says so.

- `_images/advanced_command-line_launch_windows_console.png`, on **https://docs.blender.org/manual/en/5.0/advanced/command_line/launch/windows.html**. Visible in it: four Blender start-up lines (reading preferences, finding bundled Python, reading a blend-file), then `Blender quit`. No missing-file message.
- `_images/advanced_command-line_launch_linux_console.png`, on **https://docs.blender.org/manual/en/5.0/advanced/command_line/launch/linux.html**. Visible in it: seven lines of sound-system complaints, including one reading `Cannot connect to server socket err = No such file or directory`. That line is about a sound server socket, not about a file the scene points at, so it does not answer the question. That it comes from the sound systems JACK and OpenAL rather than from Blender is **inferred** — a picture shows the lines, not what produced them. The picture cannot show which stream it came on either.

**Not covered.** Which stream carries a missing-file message, since the sources never say. Whether the answer differs by the kind of missing file — an image texture, a linked blend-file, a sound, a cache, a font, a volume — since none of these is described stream-wise either. Whether a missing file stops the render or lets it continue with a stand-in (the sources answer this only for linked blend-files and for sequencer footage, quoted above, and only in terms of what appears in the scene). What the message text actually is. Whether the 5.0 logging change moved these messages between streams; the release note says the format changed but not where the output goes.

**Outside the interface.** The question is only meaningful outside the interface, since streams are what a starting program reads, and the sources are silent for exactly that case. What they do describe for a missing file — the warning in the Info editor's header, the red path field, the magenta sequencer frame — are all things visible only inside the window, and a run started from outside with no window shows none of them. That the outside-the-interface case therefore has to rely on printed output alone is **inferred**, the step being that a run in background mode has no Info editor to put a warning in; the sources do not say what it prints instead.

> **Brought up to date on 2026-08-26 by hand test.** Both streams were captured separately, in this version, on two machines, for five kinds of missing file, and the same files were then opened with a real window. See "Answered by hand test — question 13" at the end of this question's material.


**Operating system.** The sources state no operating-system difference in stream behaviour, because they state no stream behaviour. They do state one difference in where the output is seen: on Microsoft Windows the console is a separate window that Blender hides, and "To display the console again, go to Window ‣ Toggle System Console" (manual, **https://docs.blender.org/manual/en/5.0/advanced/command_line/launch/windows.html**, read 2026-07-30), whereas on Linux and macOS the manual describes output simply going to the terminal Blender was started from (**https://docs.blender.org/manual/en/5.0/advanced/command_line/launch/linux.html** and **https://docs.blender.org/manual/en/5.0/advanced/command_line/launch/macos.html**, both read 2026-07-30). That is about visibility, not about which stream carries what.

> **Brought up to date on 2026-08-26 by hand test.** macOS and Linux were both measured in this version and agreed word for word on every missing-file run. Windows remains untested: no Windows machine has run it.


### Where I looked, for question 13

Whole-source searches across `manual_5.0`, `api_5.0` and `release_notes_5.0`, case-insensitively, for: `stderr`, `stdout`, `standard output`, `standard error`, `output stream`, `error stream`, `redirect`, `pipe`, `piped`, `2>`, `>&`, `terminal`, `console window`, `missing file`, `file not found`, `cannot find`, `could not find`, `unable to open`, `unable to find`, `unable to load`, `not found`, `cannot be loaded`, `cannot be found`, `cannot be opened`, `fails to load`, `failed to load`, `does not exist`, `no longer exist`, `magenta`, `pink`, `fail`, `failed`, `failure`. Searched `cycles_5.0` (all Python files, all 269 kernel and utility files, all shader files) for `stderr`, `stdout`, `missing`, `printf`, `fprintf`, `LOG(`.

Pages opened in full and ruled out:
- **https://docs.blender.org/manual/en/5.0/advanced/command_line/index.html** — describes the Console Window as the place messages appear ("an operating system text window that displays messages about Blender's operations, status, and internal errors") but treats it as one window, never as two streams.
- **https://docs.blender.org/manual/en/5.0/advanced/command_line/render.html** — the whole page is about which arguments to pass and in what order; it says nothing about output at all.
- **https://docs.blender.org/manual/en/5.0/advanced/command_line/arguments.html** — read in full, all 662 lines. The only output-destination statement is `--log-file`, quoted above.
- **The whole `troubleshooting` section of the manual** (its index lists Startup, 3D Viewport, Graphics Hardware, Crashes, Network, Python Errors, Recovering Data, Reporting a Bug) — the Python Errors page is about incompatible compiled libraries and duplicate DLLs, and refers to output going to "Command Line Window" as a single place. None of these pages concerns a missing scene file, and none names a stream.
- **https://docs.blender.org/manual/en/5.0/files/file_paths.html** — contains "animation will fail with an error message indicating the path errors", but that is about mistakes in an output path template, not about a file the scene points at being absent, and it names no stream.
- **https://docs.blender.org/manual/en/5.0/files/blend/compatibility.html** — about opening a file saved by a newer Blender; a different failure and no stream named.
- **https://docs.blender.org/manual/en/5.0/files/blend/open_save.html** — its "Items Not Found" is about tidying the recent-files list, not about rendering.
- **https://docs.blender.org/api/5.0/bpy.ops.file.html** — `file.report_missing_files()` is documented in one line, "Report all missing external files", with no statement of where the report goes; `file.find_missing_files()` likewise says only "Try to find missing external files".
- **https://docs.blender.org/api/5.0/bpy.types.Image.html** — searched for `missing`, `not found`, `exist`; nothing about what happens when the image file is gone.

---


## Answered by hand test — question 13, 2026-08-26

**LABEL: observed.** Everything in this section is what two real machines put on their two streams. None of it comes from documentation.

### What was done, and on what

Two machines, and their results are never merged.

- **The Creator's Mac** — a MacBook Pro with an Apple M1 Pro processor, macOS 26.5.2, Apple's own graphics and no second graphics card. Blender 5.0.1, build hash `a3db93c5b259`, built 2025-12-16 01:51:40, run from `/Applications/Blender 5.0.1.app/Contents/MacOS/Blender`.
- **The rented Linux machine, in Norway** — a Docker container holding half its host, running Ubuntu 22.04.5 LTS on kernel `6.8.0-136-generic`, with one NVIDIA GeForce RTX 3060 of 12,288 MiB on driver 595.84, about fifteen cores of an Intel Xeon E5-2697A v4, and a display server of its own on `DISPLAY=:1`. Its viewport is drawn by its processor, not by the card, and it is used as `root`. Blender 5.0.1, build hash `a3db93c5b259`, built 2025-12-16 01:30:59, run from `/opt/blender/5.0.1/blender`.

Both machines carry the same build hash for this version. **This was read from each machine rather than assumed.** The brief for this round warned that the two machines might carry different builds of Blender 5.2, so the hash was read off every version on both machines before anything was run; on 2026-08-26 the two machines reported the same hash for all four versions, 5.2 included.

### How every run was made

Every run used a settings folder of its own — `export BLENDER_USER_RESOURCES=<work folder>/cfg` — so nothing touched the Creator's own Blender settings, and no add-on was loaded. Every scene, script, output and scratch file was written outside this project.

**Every blend-file was built in this very version**, by one script run four times, so that this version was never asked to read a file another version wrote. The scenes are a cube with a camera and a light rendering in Cycles on the processor at eight samples and 64 by 64 (`good.blend`); the same at 800 by 800 and a million samples with adaptive sampling off, so one frame runs for hours (`slow.blend`); a cube and a light with no camera (`nocamera.blend`); a cube whose material has an Image Texture node pointing at `missing_tex.png` (`tex_missing.blend`); a collection linked from `lib.blend` (`lib_missing.blend`); a Video Sequencer image strip pointing at `missing_seq.png` (`seq_missing.blend`); and a text object using `missing_font.ttf` (`font_missing.blend`). A file is made missing by moving it aside immediately before the run and moving it back immediately after.

**Both streams were always sent to two different files in the same run** — `> out.txt 2> err.txt` — and read separately, so an empty stream is recorded as an empty stream rather than assumed. Paths in the quotations below are shortened to `<work folder>` and `<scene folder>`; nothing else in any quoted line is changed.

**A control was run first**, on a scene with nothing missing, in the same shape on the same machine. Without it an empty error stream in the runs below would carry no information at all.

### The control — nothing missing

```
blender -b good.blend -o <work folder>/c1_#### -f 1 > out.txt 2> err.txt
```

**The whole run landed on standard output and standard error was empty**, on both machines. Blender 5.0.1 wrote 52 lines: the version banner, `Read blend:`, the announcement lines `Rendering single frame`, `Rendering frame 1`, `Start rendering: Scene, ViewLayer` and `Engine: Cycles`, the progress lines, `Saved:` and `Time:` and `Blender quit`. Status 0.

### A missing image texture, rendered in Cycles

```
blender -b tex_missing.blend -o <work folder>/c2_#### -f 1 > out.txt 2> err.txt
```

**Blender 5.0 does complain, on standard output, and standard error is empty.** The line is:

```
00:00.268  cycles           | ERROR Image file '<scene folder>/missing_tex.png' does not exist.
```

Note the single quotation marks around the path. Blender 5.1 prints the same sentence **without** them, and Blender 5.2 prints it as a `WARNING` and adds a second line. Blender 4.5 prints nothing at all.

The line sits among the ordinary progress lines, a little after `Loading render kernels`, and the run carries on. Standard output held 54 lines in all; the control run, on a scene with nothing missing, held 52. The two extra lines are the complaint itself and one ordinary stage line, `Fra: 1 | Mem: 1M | Updating Images | Loading missing_tex.png`. The run ended at status **0** and wrote its picture.

Both machines gave this, word for word apart from the paths.

### The same missing texture, rendered in EEVEE

```
blender -b tex_missing.blend -E BLENDER_EEVEE -o <work folder>/c3_#### -f 1 > out.txt 2> err.txt
```

**In EEVEE the complaint is on standard output too, and standard error is empty.** The line is:

```
00:00.639  gpu.texture      | ERROR Failed to create GPU texture from Blender image
```

It does not name the file. Standard output held fifteen lines in all, ending `Saved:` and `Time:`; the status was 0 and the picture was written.

Blender 4.5 puts its EEVEE complaint — `GPUTexture: Blender Texture Not Loaded!` — on **standard error** instead. **So the stream changed between 4.5 and 5.0 for this message.** Both machines agreed on both versions.

### The same missing texture, with quiet mode

```
blender -b tex_missing.blend -q -o <work folder>/c4_#### -f 1 > out.txt 2> err.txt
```

**Quiet mode does not remove the missing-file line in 5.0.** With `-q` added, standard output still held it, and standard error was still empty:

```
00:00.272  cycles           | ERROR Image file '<scene folder>/missing_tex.png' does not exist.
```

What `-q` removed was the ordinary progress lines. Blender 4.5 given `-q` prints nothing at all beyond its version banner.

### The same missing texture, with a log file

```
blender -b tex_missing.blend --log-file <work folder>/c5.log -o <work folder>/c5_#### -f 1 > out.txt 2> err.txt
```

**The log file holds everything, and the streams are left almost empty.** With `--log-file <work folder>/c5.log` added, the missing-file line was in the file:

```
00:00.266  cycles           | ERROR Image file '<scene folder>/missing_tex.png' does not exist.
```

along with the whole rest of the log. Standard output was left with the version banner and `Blender quit`, and standard error was empty. Blender 4.5 given the same option writes an **empty** file and leaves both streams untouched.

### A missing linked library

```
blender -b lib_missing.blend -o <work folder>/c6_#### -f 1 > out.txt 2> err.txt
```

**All five lines are on standard output, and standard error is empty.** Status 0, picture written.

```
00:00.281  reports          | Read library: '<scene folder>/lib.blend', '//lib.blend', parent '<direct>'
00:00.281  reports          | WARNING Unable to open '<scene folder>/lib.blend': No such file or directory
00:00.281  reports          | Cannot find lib '<scene folder>/lib.blend'
00:00.282  reports          | LIB: Collection: 'LibColl' missing from '<scene folder>/lib.blend', parent '<direct>'
00:00.287  reports          | WARNING 1 libraries and 1 linked data-blocks are missing (including 0 ObjectData), please check the Info and Outliner editors for details
```

The same five messages appear in Blender 4.5, written as `Info:` and `Warning:` instead of under a category with `WARNING`.

### Missing sequencer footage

```
blender -b seq_missing.blend -o <work folder>/c7_#### -f 1 > out.txt 2> err.txt
```

**Nothing whatever, on either stream.** Standard output held only the ordinary run and standard error was empty. Status 0, and the frame was written.

The run was made again with the sequencer's own setting turned off first —

```
blender -b seq_missing.blend --python-expr "import bpy; bpy.context.scene.sequence_editor.show_missing_media=False" -o <work folder>/c11_#### -f 1 > out.txt 2> err.txt
```

— and it was **silent on both streams that way too**. So the sequencer's missing-media behaviour is a matter of what appears in the picture and nothing is printed about it either way.

**One thing read back rather than assumed.** The setting `show_missing_media` in the file this round built read back as **`True`**, in all four versions on both machines, checked with `print('SHOW_MISSING_MEDIA =', se.show_missing_media)`. The API documentation gives its default as `False`. The file was built by `bpy.ops.wm.read_factory_settings(use_empty=True)` followed by `scene.sequence_editor_create()`. What was read is recorded; nothing is concluded from it.

### A missing font on a text object

```
blender -b font_missing.blend -o <work folder>/c8_#### -f 1 > out.txt 2> err.txt
```

**Twelve lines, all on standard output, standard error empty.** The same three lines four times over:

```
00:00.304  reports          | ERROR Unable to pack file, source path '<scene folder>/missing_font.ttf' not found
00:00.304  reports          | ERROR Unable to pack file, source path '<scene folder>/missing_font.ttf' not found
00:00.304  geom.vfont       | WARNING Font file doesn't exist: //missing_font.ttf
```

Status 0, picture written. Blender 4.5 prints the same twelve, with the third carrying a source-file path and line number that 5.0 drops.

### A missing blend-file handed on the command line

```
blender -b <work folder>/not-here-at-all.blend -o <work folder>/c9_#### -f 1 > out.txt 2> err.txt
```

```
00:00.273  reports          | ERROR Cannot read file "<work folder>/not-here-at-all.blend": No such file or directory
Blender 5.0.1 (hash a3db93c5b259 built 2025-12-16 01:51:40)

Blender quit
```

Standard error was empty. Status **1**, no picture.

**This is worth setting beside the argument errors**, which go the other way. `-f abc`, `-f ""` and `-o` with no value each put their complaint on **standard error** — `Error: not a number '-f / --render-frame abc'.`, `Error: incorrect comma use '-f / --render-frame '.` and `Error: you must specify a path after '-o  / --render-output'.` — while a file that cannot be read is complained about on **standard output**. So Blender does use both streams, and which one it uses does not follow from how serious the trouble is.

### The one stream the sources name

```
blender -b good.blend -o <work folder>/c10_#### -f 1 -- --cycles-print-stats > out.txt 2> err.txt
```

**The statistics land on standard output, not on standard error, and standard error is empty.** Cycles' own help text for that option, which every reading of this question quotes, says the statistics go to `stderr`. **They do not.**

### What the picture holds where the missing file should have been

Every picture written above was read back with a script that prints the red, green, blue and alpha of three places in it — the centre, a quarter of the way across, and one pixel in from the corner. The two machines agreed to four decimal places.

> **Note added on 2026-08-26 by the citation check, which changed nothing in the table.** The table was re-run on both machines from a scene rebuilt to this document's own description — "a cube with a camera and a light". The two sequencer rows came back identical to the numbers below on both machines, and the two machines agreed with each other to four decimal places in every row, which is what the sentence above claims. The other rows could not be re-derived, because the description does not fix where the camera and the light sit or how bright the light is, and those decide the numbers. The numbers below are left exactly as first measured; what a re-run cannot settle is recorded here rather than overwritten.

| The run | The centre of the picture |
| --- | --- |
| the control, nothing missing | `(0.6902, 0.6902, 0.6902, 1.0)` — plain grey |
| missing image texture, Cycles | `(0.7373, 0.3882, 0.7412, 1.0)` — a lit magenta |
| missing image texture, EEVEE | `(0.7373, 0.3843, 0.7412, 1.0)` — the same magenta |
| missing linked library | `(0.0, 0.0, 0.0, 1.0)` — the linked thing simply is not there |
| missing font | `(0.0, 0.0, 0.0, 1.0)` |
| missing sequencer footage, setting on | `(0.7529, 0.3176, 0.6824, 1.0)` across the whole frame |
| missing sequencer footage, setting off | `(0.0, 0.0, 0.0, 0.0)` — fully transparent |

**Blender 5.0.1 paints a missing image texture magenta**.

### Blender's own channels, with a real window

The same files were opened with Blender's window rather than in background, on both machines. A start-up script turned the Outliner into an Info editor, invoked the render with `bpy.ops.render.render('INVOKE_DEFAULT')`, photographed the window and read `bpy.data.window_managers[0].reports`, then quit from a timer. On the rented Linux machine the whole screen was photographed with its own `shot.sh`; on the Creator's Mac, Blender's own `bpy.ops.screen.screenshot(filepath=...)` was used instead, so that nothing but Blender's own window was captured.

**For a missing image texture, Blender's own channels say nothing in 5.0.** The Info editor was empty, no report box appeared along the bottom of the window, and the Blender Render window showed a magenta cube. The build has no `WindowManager.reports`: the script's check printed `this build has no WindowManager.reports`.

**For a missing linked library the channels do speak in 5.0, and this is where they start.** The Info editor listed the five messages with a warning triangle beside the last of them, and a report box appeared along the bottom of the window reading:

```
1 libraries and 1 linked d…utliner editors for details
```

(the box shortens the middle of the sentence with an ellipsis of its own). Blender 4.5, given exactly the same file on the same two machines, leaves the Info editor empty and shows no box, while printing all five lines in the terminal.

For a scene with no camera, the render operator raised the error back into the script that called it rather than reporting it anywhere:

```
=== RENDER INVOKE RAISED: RuntimeError('Error: No camera found in scene "Scene"\n')
```

No Blender Render window opened and the Info editor stayed empty.

**One thing to know when reading the pictures.** Every run used a fresh settings folder, so Blender's Quick Setup panel is sitting over the middle of the 3D viewport in all of them. It never covered the Info editor, the status bar or the Blender Render window, and it did not stop the render.

### The two standing lines, brought up to date

**Outside the interface.** Settled by measurement, both ways round, for this version on both machines. Driven from outside with no window, the answers are the ones set out above: a line on standard output for a missing image texture, five lines on standard output for a missing library, twelve for a missing font, one for a missing blend-file, and nothing at all for missing sequencer footage — **and standard error empty in every one of them except 4.5 rendering in EEVEE**. Driven from inside its own window, the same files give nothing in the Info editor for the texture and five entries for the library. **So the two settings do not agree, and which way they differ depends on the kind of file.**

**Operating system.** Settled for macOS and for Linux, and only for those two. Every run above was made on both, and **the two machines agreed word for word in every one**, the file paths aside — the same messages, on the same streams, with the same empty streams, and the same colours in the pictures to four decimal places. **Windows is untested and stays untested**: no Windows machine has run it, so whether its hidden console changes what a starting program can capture is not settled by anything here.

### What these runs do not settle

- Anything on Microsoft Windows.
- Whether a display server attached to the run changes what reaches standard error. These runs had no `DISPLAY` set, and an earlier round of this topic recorded display-library complaints arriving on standard error when one was attached.
- Kinds of missing file not tried here: a sound, a volume file, a point cache, an Alembic or USD cache, an external OSL script, an image the compositor points at.
- Why `show_missing_media` reads back as `True` where the documentation gives `False`.

## What a running Blender still has to settle

**This list has been pruned.** Every item the researcher wrote here was carried out on 2026-08-26, in this version, on the Creator's Mac and on a rented Linux machine, and the results are written into this document under "Answered by hand test — question 12" and "Answered by hand test — question 13" above. The full list, with the exact commands, is at `Audits/Blender Research/Reports/Hand tests — Architecture questions.md`. Only the items below are still open.

**Still open, and reachable from a terminal.** Each of these is the same shape as the runs already made — build the scene, move the file aside, run the render with the two streams sent to separate files, then read both files and the status:

1. **A missing sound.** Add a sound to the scene or a sound strip to the Video Sequencer, save, move the sound file away, then run `blender -b scene.blend -o /tmp/out_#### -f 1 > out.txt 2> err.txt` and read both files and `echo $?`.
2. **A missing volume file.** Put an OpenVDB file on a Volume object, save, move the `.vdb` away, and run the same command.
3. **A missing cache.** Put an Alembic or USD file on a Mesh Sequence Cache modifier, save, move the file away, and run the same command. Do the same for a point cache.
4. **A missing external OSL script.** Put an external `.osl` file on a Script node with Open Shading Language switched on, save, move the file away, and run the same command. Cycles' own files carry two messages for this case — `OSL query failed to open %s` and `OSL script compilation failed, see console for errors` — and neither has been seen from a run.
5. **A missing image the compositor points at.** Put an Image node in the compositing tree pointing at a file, switch compositing on, save, delete the file, and run the same command.
6. **A missing image on an Environment Texture node in the world.** Same shape again.
7. **Missing sequencer footage with Blender's window open.** The missing texture and the missing library were both opened with a real window on both machines; the sequencer file was not. Open `seq_missing.blend` with its picture moved away, render from the window, and read the Info editor and the status bar.

**Still open, and out of reach of both machines.** These are in the "Cannot be run" section of `Audits/Blender Research/Reports/Hand tests — Architecture questions.md`, with the reason:

8. **Anything on Microsoft Windows** — no Windows machine has run it. This leaves unsettled what `echo %ERRORLEVEL%` and `$LASTEXITCODE` report for every run above, whether Ctrl-Break behaves as Ctrl-C does, and whether the hidden console changes what a starting program can capture.
9. **The File ▸ External Data ▸ Report Missing Files menu item** — the one destination the documentation actually names for a missing-file warning. Reading its exact wording needs a menu picked with a pointer, and neither machine can be sent a pointer click into Blender's window.
10. **A render stopped by pressing Esc inside the window** — same reason: neither machine can be sent a key press into Blender's own window.
11. **An unwritable output folder on Linux** — the rented machine is used as `root`, which is not stopped by a folder's permissions, so that run gave 0 there. Settling it needs an account that is not `root`.
12. **A working HIP or oneAPI render device** — neither machine has an AMD or an Intel card. What the *refusal* looks like is settled; what a working run gives back is not.
13. **Blender loaded as a Python module** rather than run as a program — neither machine has such a build.
14. **Whether a display server attached to a windowless run changes what reaches standard error.** The runs above had no `DISPLAY` set. An earlier round of this topic recorded display-library complaints arriving on standard error when one was attached, and that condition was not repeated here.


---

2026-08-25 — Claude Opus 5 (Claude Code subagent) — Reports, Blender 5.0.

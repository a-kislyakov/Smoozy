# Reports — Architecture questions, second pass (Blender 5.1)

**Sources consulted, and on what date.** I read Blender 5.1's manual (`Blender Sources/manual_5.1`, published at https://docs.blender.org/manual/en/5.1/), Blender 5.1's Python API documentation (`Blender Sources/api_5.1`, published at https://docs.blender.org/api/5.1/), Blender 5.1's release notes (`Blender Sources/release_notes_5.1`, published at https://developer.blender.org/docs/release_notes/5.1/), and Cycles' own files as shipped with Blender 5.1 (`Blender Sources/cycles_5.1`). All four were taken on 2026-07-30, and that is the date read for everything quoted below. I also read `Blender Sources/About this folder.md` and `Blender Sources/requirements/requirements_2026-08-01.txt`, both inside `Blender Sources/`; nothing from either is used in an answer.

I read only the question sheet and files inside Blender Sources/. I did not open any other part of this project.

For completeness, one thing short of opening anything: after writing this file I listed its own folder to confirm my file name was saved exactly as asked, and the names of the other documents there appeared in that listing. I opened none of them and read nothing but their names, and no answer above is touched by it.

Questions answered here: **12 and 13** only.

---

## 12. What ending status does a run of Blender give back to whatever started it, for each of the ways a run can end — finishing normally, being interrupted, and being refused because something it was handed cannot be used?

**LABEL: not in the sources**, for all three endings, with two partial exceptions that are set out below and labelled separately.

### The short answer

For none of the three endings do the sources state the number Blender hands back to the program that started it. There is no page in the manual, no page in the API documentation and no line in the 5.1 release notes that says "Blender returns 0 when it finishes", or gives any value for an interrupted run, or gives any value for a run refused because of a bad file or a bad argument.

Two things in the sources touch the edge of the question and are worth having:

**(a) Blender can be *told* to exit with a chosen number when a Python script raises an exception. LABEL: documented.**

> `--python-exit-code <code>`
>
> Set the exit-code in [0..255] to exit if a Python exception is raised
> (only for scripts executed from the command line), zero disables.

Source: the manual, Command Line Arguments, https://docs.blender.org/manual/en/5.1/advanced/command_line/arguments.html — read 2026-07-30.

Two things this does say. The number is chosen by whoever starts Blender, from 0 to 255, and it is not set unless asked for — "zero disables" is how the option describes its own off state. And it fires only on a Python exception, in a script run from the command line; it says nothing about a render failing, a file being missing, or a run being interrupted.

**(b) A command registered from Python is documented to return 0 for success and 1 for failure. LABEL: documented (for the callback's return value), not for the process's ending status.**

> The return value should be 0 for success, 1 on failure (specific error codes from the os module can also be used).

Source: the API documentation, `bpy.utils.register_cli_command(id, execute)`, https://docs.blender.org/api/5.1/bpy.utils.html — read 2026-07-30.

The same page says what such a command is: "Register a command, accessible via the (-c / --command) command-line argument." The convention 0-for-success, 1-for-failure is therefore stated in the sources — but stated as the number a Python callback should hand back to Blender, not as the number Blender hands back to the operating system. The page never joins those two up. Reading it as an exit status would require the reader to supply that step, so it is not a documented answer to the question asked.

### Ending one: finishing normally

**Not in the sources.** Nothing states the status of a run that renders what it was asked to render and stops. The manual's page on rendering from the command line (https://docs.blender.org/manual/en/5.1/advanced/command_line/render.html) walks through `blender -b file.blend -f 10` and `blender -b file.blend -a` and never mentions a returned status. Neither does the Command Line Arguments page, beyond `--python-exit-code`.

### Ending two: being interrupted

**Not in the sources.** No status is given. What the sources do say about interruption, none of which reaches a number:

- The API documentation, on running Blender as a Python module rather than as the program: "Blender's typical signal handlers are not initialized, so there is no special handling for Control-C to cancel a render and a crash log is not written in the event of a crash." Source: https://docs.blender.org/api/5.1/info_advanced_blender_as_bpy.html — read 2026-07-30. Read carefully this is a statement about the *module*: it says the module lacks handling that Blender itself is being contrasted against. That Blender the program therefore does have Control-C handling that cancels a render is a step the reader has to take, so it is **inferred**, not documented — and even taken, it gives no ending status.
- The API documentation, on using a terminal: "If the script runs for too long or you accidentally enter an infinite loop, Ctrl-C in the terminal (Ctrl-Break on Windows) will quit the script early." Source: https://docs.blender.org/api/5.1/info_tips_and_tricks.html — read 2026-07-30. This is about ending a *script*, not about the status of the run.
- The manual, on why to render an animation as separate images: "when rendering animations to image file formats the render job can be canceled and resumed at the last rendered frame by changing the frame range." Source: https://docs.blender.org/manual/en/5.1/render/output/introduction.html — read 2026-07-30. Cancelling is described; nothing is said about what the run hands back.

There is no mention of `SIGINT`, `SIGTERM`, or signals by name anywhere in the 5.1 manual, API documentation or release notes — see the record of looking below.

### Ending three: being refused because something it was handed cannot be used

**Not in the sources for the general case**, and **observed for one specific case**.

The sources state plainly that Blender *does* stop when it is handed something it cannot use, but never with what status:

- The manual, on argument parsing: "blender -ba test.blend" followed by "…will exit since -ba is an unknown argument." Source: https://docs.blender.org/manual/en/5.1/advanced/command_line/arguments.html — read 2026-07-30. Blender exits; the status is not given.
- The API documentation, describing the `exit_pre` handler's one argument: "False indicates that Blender is running in background mode, or is exiting due to failed command line arguments, etc." Source: https://docs.blender.org/api/5.1/bpy.app.handlers.html — read 2026-07-30. This confirms "exiting due to failed command line arguments" is a real ending Blender distinguishes internally, and confirms that Python can be given a chance to run just before it. It gives no status. (The `exit_pre` handler is itself new in 5.1: the release notes say "A new handler `bpy.app.handler.exit_pre` has been added, which allows registering Python code to run when Blender exits." Source: https://developer.blender.org/docs/release_notes/5.1/python_api/ — read 2026-07-30.)
- The manual lists a debug option: "`--debug-exit-on-error`" / "Immediately exit when internal errors are detected." Source: https://docs.blender.org/manual/en/5.1/advanced/command_line/arguments.html — read 2026-07-30. Again, exit is named, status is not.

**The one specific case with a number. LABEL: observed** (from Cycles' own files, not from documentation).

If Blender is started with `-- --cycles-device <device>` and Cycles cannot set that device, Cycles' own start-up code calls `sys.exit(1)`. The exact lines:

```
def _parse_command_line():
    import sys

    argv = sys.argv
    if "--" not in argv:
        return

    parser = _configure_argument_parser()
    args, _ = parser.parse_known_args(argv[argv.index("--") + 1:])

    if args.cycles_print_stats:
        import _cycles
        _cycles.enable_print_stats()

    if args.cycles_device:
        import _cycles
        if not _cycles.set_device_override(args.cycles_device):
            sys.exit(1)
```

File and place: `Blender Sources/cycles_5.1/engine.py`, function `_parse_command_line()`, lines 24–41; the `sys.exit(1)` is line 41. Read 2026-07-30.

That `_parse_command_line()` runs when Cycles starts up is also in the file — `init()` at lines 44–53 ends with `_parse_command_line()`.

What is observed is that the file contains that call, on that condition. That the run therefore ends and hands back the number 1 to whatever started Blender is one step beyond the file: it assumes `sys.exit(1)` inside Blender's embedded Python terminates the whole program with that status rather than being caught somewhere. Nothing in the four sources says whether Blender catches it. So **the ending status of 1 for this case is inferred, not observed**, and the step is exactly that assumption.

The manual documents the option that leads here: "`--cycles-device <device>`" / "Set the device used for rendering. Valid options are: CPU CUDA OPTIX HIP ONEAPI METAL." Source: https://docs.blender.org/manual/en/5.1/advanced/command_line/arguments.html — read 2026-07-30. It does not say what happens when the device cannot be set.

### Not covered

- No status is settled for any of the three endings by documentation. The one number found (1) is in Cycles' start-up code, for one narrow refusal — a `--cycles-device` value that cannot be applied — and even there the step from "the file calls `sys.exit(1)`" to "the process ends with status 1" is unverified.
- Nothing here distinguishes a run that rendered but produced a broken or empty image from one that produced a good one; the sources give no status for either.
- Nothing here covers what a *crash* hands back, which is a fourth ending the question did not ask about.
- Nothing here covers what happens when two failures coincide, or what `--python-exit-code` does when a render also fails.
- I did not run Blender. Everything above is from reading.

### Outside the interface

The whole question is about running Blender from outside its own window, and the fragments found are all from the parts of the documentation about exactly that. `--python-exit-code` is documented as applying "only for scripts executed from the command line". `register_cli_command` is documented as backing the `-c` / `--command` command-line argument. `--cycles-device` is a command-line argument. So what little there is, is about the outside-the-interface case already.

Where it differs inside: an ending status only exists for a run that something else started, so the question does not arise for a person clicking in Blender's window. The sources say nothing about this either way. One related difference is documented, for the Python-module way of driving Blender rather than the program: "Blender's typical signal handlers are not initialized, so there is no special handling for Control-C to cancel a render" (https://docs.blender.org/api/5.1/info_advanced_blender_as_bpy.html, read 2026-07-30).


> **Brought up to date on 2026-08-26 by hand test.** The status a run hands back was measured on two machines in this version, and all of it was measured from outside Blender's own window, which is the only place a status exists. See "Answered by hand test — question 12" at the end of this question's material.

### Operating system

The sources do not say whether any ending status differs by operating system, because they do not give the statuses at all. Three related things are marked by operating system on the Command Line Arguments page (https://docs.blender.org/manual/en/5.1/advanced/command_line/arguments.html, read 2026-07-30): "`/?`" / "Print this help text and exit (Windows only)."; "`-r, --register`" / "Register blend-file extension for current user, then exit (Windows & Linux only)."; and "`--qos <level>`" / "Set the Quality of Service (QoS) mode for hybrid CPU architectures (Windows only)." The `--python-exit-code` range of "[0..255]" is stated without any operating-system qualification. One interruption difference is documented: "Ctrl-C in the terminal (Ctrl-Break on Windows) will quit the script early" (https://docs.blender.org/api/5.1/info_tips_and_tricks.html, read 2026-07-30) — the key differs on Windows, but nothing is said about the status.


> **Brought up to date on 2026-08-26 by hand test.** macOS and Linux were both measured in this version and agree almost everywhere; the places they do not are named in "Answered by hand test — question 12" below. Windows remains untested: no Windows machine has run it.

### Record of where I looked — question 12

Searched, case-insensitively, across the whole of `manual_5.1`, `api_5.1`, `release_notes_5.1` and `cycles_5.1` at once:

- `exit code`, `exit status`, `exit-code`, `return code`, `returncode`, `error code`, `errorlevel`, `ERRORLEVEL`, `exit value`, `status code`, `non-zero`, `nonzero`. Only four hits mattered: `exit-code` in the manual's Command Line Arguments page (quoted above); `error code` in `bpy.utils` (quoted above); `status code` in `api_5.1/info_gotchas_threading.txt`, which is an HTTP response's status code printed inside a worked example about Python threads, not a process ending; and the `non-zero`/`nonzero` hits, every one of which is about numeric values in geometry, physics or shading settings (bone lengths, fluid settings, curve radii) and none about endings.
- `Ctrl-C`, `Ctrl+C`, `Control-C`, `Ctrl-Break`, `SIGINT`, `SIGTERM`, `signal handler`. `SIGINT` and `SIGTERM` appear nowhere in any of the four 5.1 sources. Every `Ctrl-C` hit in the manual is the copy shortcut, in interface, keymap, node, curve, text and mesh pages. The two that were not copy are the two quoted above.
- `sys.exit`, `os._exit`, `exit(`, `quit_blender`, `wm.quit`, `EXIT_FAILURE`, `EXIT_SUCCESS`, `abort()`, `exit_code`. In the documentation: `api_5.1/info_gotchas_crashes.txt` has a section warning that Python modules calling `sys.exit()` will "close Blender immediately" — a warning about scripts, not a statement of Blender's own ending status; `bpy.ops.wm.quit_blender()` exists but its page says nothing about a status. In Cycles' files: the only `sys.exit` is the one quoted above; `EXIT_FAILURE`, `EXIT_SUCCESS`, `abort()` and `exit_code` appear nowhere.
- `render farm`, `renderfarm`, `batch`, `automation`, `shell script`, `subprocess`, `0 for success`. These were to catch a passage aimed at whoever scripts Blender. The three `render farm` hits are about mixed hardware, placeholders and metadata notes. The `batch` and `automation` hits led to the manual's Command Line index page and its scripting security page, both of which I read in full; neither mentions a status. `0 for success` found only the `bpy.utils` line quoted above.

Pages read in full rather than searched: the manual's Command Line Arguments page (all 691 lines), Using Blender From The Command Line index, Rendering From The Command Line, Extensions Command Line Arguments, and Scripting & Security; the API's `bpy.utils.register_cli_command` entry with both of its worked examples, the `bpy.app.handlers.exit_pre` entry, `info_advanced_blender_as_bpy`, `info_tips_and_tricks`, and the `sys.exit` section of `info_gotchas_crashes`; Cycles' `engine.py` and `__init__.py`.

Release notes: I searched all 21 files of `release_notes_5.1` for `exit`, `return code` and `status`. The only ending-related hit is the `exit_pre` handler note quoted above. `user_interface.md`'s many "status" hits are all about the status bar in Blender's own window. `bugfixes.md` has one "Crash at exit" entry, which is a crash fix, not a statement of status.

Ruled out and why: the compositor, EEVEE, Cycles-settings, modelling, animation and sculpting parts of the manual, on the grounds that an ending status is a property of the program as a whole and the manual keeps that material in `advanced/command_line/` — which I read entirely. The `bpy.ops.*` pages, on the grounds that operators return one of Blender's own outcome words ("`FINISHED`", "`CANCELLED`" and so on, listed at `api_5.1/bpy_types_enum_items/operator_return_items.txt`) and not a process status; the question explicitly asks for the status the starting program receives, not anything internal.

---


## Answered by hand test — question 12, 2026-08-26

**LABEL: observed.** Everything in this section is what two real machines gave back when the question was put to them directly. None of it comes from documentation, and none of it was known before the runs.

### What was done, and on what

Two machines, and their results are never merged.

- **The Creator's Mac** — a MacBook Pro with an Apple M1 Pro processor, macOS 26.5.2, Apple's own graphics and no second graphics card. Blender 5.1.2, build hash `ec6e62d40fa9`, built 2026-05-19 01:30:33, run from `/Applications/Blender 5.1.2.app/Contents/MacOS/Blender`.
- **The rented Linux machine, in Norway** — a Docker container holding half its host, running Ubuntu 22.04.5 LTS on kernel `6.8.0-136-generic`, with one NVIDIA GeForce RTX 3060 of 12,288 MiB on driver 595.84, about fifteen cores of an Intel Xeon E5-2697A v4, and a display server of its own on `DISPLAY=:1`. Its viewport is drawn by its processor, not by the card, and it is used as `root`. Blender 5.1.2, build hash `ec6e62d40fa9`, built 2026-05-19 02:18:37, run from `/opt/blender/5.1.2/blender`.

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

**Two presses, three seconds apart.** 5.1 ended with exit code **2** — on both machines, four runs each. The terminal showed:

```
^C
Sent an internal break event. Press ^C again to kill Blender
^C
Blender killed
Error: Not freed memory blocks: 29125, total unfreed memory 48.342266 MB
```

Blender 4.5 given the same two presses ends on `SIGABRT` instead, and Blender 5.0 ends on `SIGSEGV` on the Mac while giving 2 on Linux.

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

**The manual's own unknown-argument example, `blender -ba good.blend`.** **Blender 5.1 does not end.** The run was still going after ninety seconds, on both machines, twice each, and the test harness had to kill it with `SIGKILL`. There is no ending status to report.

Standard output, two lines:

```
00:00.544  reports          | ERROR Cannot read file "<work folder>/-ba": No such file or directory
00:00.544  blend            | Read blend: "<scene folder>/good.blend"
```

Standard error, one line:

```
unknown argument, loading as file: -ba
```

Blender treats `-ba` as a file to open, fails to open it, opens the blend-file that followed instead, and carries on running. Blender 4.5 given the same command prints a second line on standard error — `Error: argument has no '.blend' file extension, not using as new file, exiting!` — and stops with status 1. **That line is gone in 5.1, and so is the stopping.**

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

So a name that is not on Cycles' list and a name that is on it but has no hardware behind it **both give 1**, and which names have hardware behind them is what separates the two machines. In 5.1 the refusal of a name that is not on Cycles' list lands on **standard output**, and standard error is empty:

```
00:00.277  cycles           | ERROR NOTADEVICE is not a valid Cycles device.
Blender 5.1.2 (hash ec6e62d40fa9 built 2026-05-19 01:30:33)
Error: Not freed memory blocks: 1, total unfreed memory 0.000004 MB
```

The refusal of a name that is on the list but has no hardware behind it also lands on standard output:

```
00:00.332  reports          | ERROR Found no Cycles device of the specified type
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

**Cycles' own `maketx` command.** **There is no such command in 5.1.** Both runs gave **1**, with nothing on standard output and one line on standard error:

```
Unrecognized command: "maketx"
```

`blender --command help` in 5.1 lists one command:

```
Blender Command Listing:
	extension
```

**A scene with no camera**, `blender -b nocamera.blend -o <work folder>/nc_#### -f 1`. Status **1** on both machines, nothing written:

```
00:00.313  blend            | Read blend: "<scene folder>/nocamera.blend"
00:00.317  render           | Rendering single frame
00:00.317  render           | Rendering frame 1
00:00.317  reports          | ERROR Cannot render, no camera
Blender 5.1.2 (hash ec6e62d40fa9 built 2026-05-19 01:30:33)

Blender quit
```

**An output folder that cannot be written to** — made with `mkdir ro` and `chmod 500 ro`. On the Creator's Mac, status **1** in 5.1, with nothing written, and **both lines on standard output while standard error stayed empty**:

```
00:00.362  image.write      | ERROR OpenImageIO write failed: Could not open file "<work folder>/ro/p_0001.png"
00:00.362  reports          | ERROR Render error (Undefined error: 0) cannot save: '<work folder>/ro/p_0001.png'
00:00.362  render           | Time: 00:00.04
```

Blender 4.5 given the same run says `(Permission denied)` where 5.1 says `(Undefined error: 0)`, and puts a bare `<path>: Permission denied` on standard error, which 5.1 does not.

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

In 5.1 there is no such setting. The Python expression came back with `NO save_output IN THIS BUILD: 'RenderSettings' object has no attribute 'save_output'`, the render went ahead, and the picture was written. Status 0.

**The reversed frame range.** In 5.1 the run does announce the range it was given, on standard output:

```
00:00.313  blend            | Read blend: "<scene folder>/good.blend"
00:00.317  render           | Rendering animation (frames 10..5)
Blender 5.1.2 (hash ec6e62d40fa9 built 2026-05-19 01:30:33)

Blender quit
```

It says it is rendering frames 10 to 5, renders none of them, writes nothing, and ends at 0.

### The two standing lines, brought up to date

**Outside the interface.** Settled by measurement, for this version, on both machines. Every number above was got from a run started and watched by another program, with no window, which is the only setting in which an ending status exists at all. The window runs described under question 13 show the other side of the same coin: a render invoked from inside a window hands its outcome back to the Python that called it — `RuntimeError('Error: No camera found in scene "Scene"\n')` — and the *program's* status has nothing to do with it, because the program is still running.

**Operating system.** Settled for macOS and for Linux, and only for those two. Every result above was got on both, and **they agree in every one except a single case**: an unwritable output folder, where the Linux machine is used as `root` and is therefore not stopped by the folder's permissions — a difference in how the machine is used rather than in Blender. Everything else — the clean finishes, the interrupts, the signals, the refused files and arguments, the devices, the Python failures, the registered commands and the whole of the trap — came back the same on both. **Windows is untested and stays untested**: no Windows machine has run it. **It is untried rather than untestable** — this project has a route to a Windows computer, described in the equipment section of `Preparation/Blender Research/Method.md`, and every machine that route has produced booted a **Server** edition, so a Windows answer would have to name its edition, so what `echo %ERRORLEVEL%` or `$LASTEXITCODE` would report, and whether Ctrl-Break behaves as Ctrl-C does, are not settled by anything here.

### What these runs do not settle

- Anything on Microsoft Windows.
- What a working HIP or oneAPI device gives back. Both were refused for want of an AMD or an Intel card, so the *refusal* is settled and the success is not.
- What Blender loaded as a Python module hands back. Neither machine has such a build.
- Whether an unwritable output folder gives 1 on Linux, which needs an account that is not `root`.
- Why Blender 4.5 behaves as it does when handed an engine name that does not exist. What happened is recorded; nothing here explains it.

## 13. When a file the scene points at is not there, what does a render put on each of its output streams?

**The question was ambiguous and I say so plainly.** "Output streams" has two readings. The one I take as meant, because the question sheet's own closing note says question 13 "asks which stream carries what", is the two text streams a started program writes to — standard output and standard error. The other reading is the render's own outputs: the picture and the file written for it. I answer the first reading fully and the second reading as well, because the second is where nearly all of the material turned out to be.

### Reading one: the two text streams

**LABEL: not in the sources.**

Nothing in Blender 5.1's manual, Python API documentation or release notes names standard output or standard error anywhere, for any condition — not for a missing file, and not for anything else. The words `stdout`, `stderr`, `standard output` and `standard error` do not appear in `manual_5.1` or `api_5.1` at all, with one irrelevant exception: the compositor's Color Balance page has the phrase "Standard output image.", which is the name of a node's picture output socket, not a text stream (https://docs.blender.org/manual/en/5.1/compositing/types/color/adjust/color_balance.html, read 2026-07-30). The word `stream` in the render part of the manual appears only about video and audio bitstreams in the Output Properties page.

So there is no documented answer to which stream carries what, when a file the scene points at is missing.

What the documentation does instead is describe one undifferentiated destination and call it the console:

> The Console Window, also called a Terminal, is an operating system text window that displays messages about Blender's operations, status, and internal errors.
>
> When Blender is manually started from a terminal, Blender output is shown in the Console Window it is started from.

Source: the manual, Using Blender From The Command Line, https://docs.blender.org/manual/en/5.1/advanced/command_line/index.html — read 2026-07-30. "Blender output" is a single thing here. The page never divides it in two.

The same page collapses the distinction again: "If Blender exits unexpectedly, the messages may indicate the cause or error." Errors and ordinary messages are both just "the messages".

**One exception, and it is the only place any stream is named in any of the four sources. LABEL: observed** (Cycles' own files, not documentation).

```
    parser.add_argument("--cycles-print-stats",
                        help="Print rendering statistics to stderr",
                        action='store_true')
```

File and place: `Blender Sources/cycles_5.1/engine.py`, function `_configure_argument_parser()`, lines 13–15. Read 2026-07-30.

That is Cycles' own help text for the `--cycles-print-stats` argument, and it says the standard error stream. It settles that Blender does write render output to standard error at least sometimes. It has nothing to do with a missing file, and it is the only stream-naming sentence in the whole of the manual, the API documentation, the release notes and Cycles' files for 5.1.

Worth noting for anyone comparing: the manual documents the same argument without the stream. "`--cycles-print-stats`" / "Log statistics about render memory and time usage." Source: https://docs.blender.org/manual/en/5.1/advanced/command_line/arguments.html — read 2026-07-30. The detail exists in Cycles' file and is dropped in the manual.

### Reading two: what the render puts in the picture

Here there is real material, from three places.

**(a) A missing image texture is rendered magenta by Cycles. LABEL: observed for what the file says; inferred for the colour on screen.**

Two lines, from two of Cycles' files.

```
/* Color to use when images are not found. */
#define IMAGE_MISSING_RGBA make_float4(1, 0, 1, 1)
```

File and place: `Blender Sources/cycles_5.1/source/util/types_image.h`, lines 12–13. Read 2026-07-30.

```
ccl_device float4
svm_image_texture(KernelGlobals kg, const int id, const float x, float y, const uint flags)
{
  if (id == -1) {
    return IMAGE_MISSING_RGBA;
  }
```

File and place: `Blender Sources/cycles_5.1/source/kernel/svm/image.h`, lines 21–26. Read 2026-07-30.

The same value is returned from one other place, the three-dimensional image lookup used for volume grids: `return IMAGE_MISSING_RGBA;` at `Blender Sources/cycles_5.1/source/kernel/util/image_3d.h`, line 258 — the last line of the function, reached when no supported image data type matched. Read 2026-07-30. Those three are every occurrence of `IMAGE_MISSING_RGBA` in Cycles' 5.1 files.

What is **observed** is that the files define a colour named "image missing", comment it "Color to use when images are not found", and return it from the image-texture lookup when the image identifier is `-1`.

What is **inferred**, and the step is this: `make_float4(1, 0, 1, 1)` is four numbers, and reading them as red, green, blue and alpha — which the name RGBA in `IMAGE_MISSING_RGBA` invites — gives full red, no green, full blue, fully opaque, which is magenta. That reading of the four numbers is a step the file does not spell out. A second step is that an image the scene points at which is not on disk is what produces the `-1`; the comment says images "not found" but the code path that sets `-1` is not in the files supplied here. So "Cycles paints a missing image texture solid magenta in the rendered picture" is inferred, not observed.

**(b) A missing image or movie in the video sequencer renders fully transparent by default, or magenta if asked. LABEL: documented.**

> Show Missing Media
>
> Render missing images/movies with a solid magenta color.
> When disabled, missing content will render fully transparent.

Source: the manual, Video Sequencer preview Sidebar, https://docs.blender.org/manual/en/5.1/editors/video_sequencer/preview/sidebar.html — read 2026-07-30.

The same page adds, as a tip: "Strips with missing content will be displayed as red in the timeline."

The API documentation gives the setting and, importantly, its default:

> show_missing_media
>
> Render missing images/movies with a solid magenta color (default False)

Source: the API documentation, `bpy.types.SequenceEditor`, https://docs.blender.org/api/5.1/bpy.types.SequenceEditor.html — read 2026-07-30.

Two things follow directly from the quoted words. The setting is off unless turned on, so the default behaviour for a missing sequencer file is fully transparent, not magenta. And the setting lives on `SequenceEditor`, which is scene data, rather than on a screen or space — so it is saved with the file and is not a property of the window you happen to be looking at.

**(c) A missing linked blend-file is replaced by placeholders rather than refused. LABEL: documented.**

> If Blender cannot find a library while loading a blend-file,
> it will create placeholder data-blocks to replace missing linked ones.
> That way, references to the missing data are not lost, and by relocating the missing library,
> the lost data can be automatically restored.

Source: the manual, Linking and Appending, under the heading "Broken Libraries", https://docs.blender.org/manual/en/5.1/files/linked_libraries/link_append.html — read 2026-07-30.

The Python API exposes the resulting state as a flag on every data-block:

> is_missing
>
> This data-block is a place-holder for missing linked data (i.e. it is [an override of] a linked data that could not be found anymore) (default False, readonly)

Source: the API documentation, `bpy.types.ID`, https://docs.blender.org/api/5.1/bpy.types.ID.html — read 2026-07-30.

So the load carries on with placeholders, and the fact is queryable from Python. Neither passage says anything is printed anywhere.

### Where a message is documented to go, when it is documented at all

Only one destination is ever named for a missing-file message, and it is Blender's own window rather than a stream:

> Report Missing Files
>
> This option is useful to check if there are links to unpacked files that no longer exist.
> After selecting this option, a warning message will appear in the Info editor's header.
> If no warning is shown, there are no missing external files.

Source: the manual, Topbar, File menu, External Data, https://docs.blender.org/manual/en/5.1/interface/window_system/topbar.html — read 2026-07-30.

That is a menu item a person clicks, not something a render does. The Info editor it names "logs the executed operators as well as errors, warnings, and informational messages" (https://docs.blender.org/manual/en/5.1/editors/info_editor.html, read 2026-07-30) — again a place in the window, not a stream.

The manual also uses two phrases in scare quotes when explaining why to keep a project's files together: deleting a file "will result in a 'file not found' error", and forgetting one gives a "'file is missing' error". Source: https://docs.blender.org/manual/en/5.1/video_editing/setup/directory_structure.html — read 2026-07-30. These are the manual describing an error loosely in passing. They are not presented as the wording Blender prints, and no destination is given for either.

### Not covered

- Which of the two text streams carries anything, under this condition or any other, apart from the single Cycles statistics line. That is unanswered by the sources, not answered negatively — I found silence, not a statement that nothing is printed.
- Whether a render prints anything at all when a file is missing. The sources neither show a message nor say there is none.
- The magenta finding covers image textures in Cycles and images and movies in the sequencer. It does not cover missing sounds, missing fonts, missing point caches, missing Alembic or USD files, missing OpenColorIO configurations, or missing volume files as such — I found nothing on any of those, and the one volume-shaped hit (`image_3d.h`) is about an unmatched data type, not about a file being absent.
- Nothing here says whether the render still finishes, still writes its output file, or is abandoned, when a file is missing. Question 12's material does not settle it either.
- EEVEE is not covered at all. `IMAGE_MISSING_RGBA` is Cycles', and Cycles' files are the only render engine's files supplied.
- I did not run Blender. Nothing above was checked against a running render.

### Outside the interface

Poorly, and this is the weak point of the whole answer.

Everything documented about *where* a missing-file message goes points at Blender's own window — the Info editor's header, red strips in the sequencer timeline, a warning after clicking a menu item. None of those exist when Blender is started and driven by another program. The sources do not say what replaces them.

Two of the findings do hold outside the interface, by their own terms. `IMAGE_MISSING_RGBA` is in the render kernel, which is what does the rendering however the render was started. And `show_missing_media` is a property of `SequenceEditor`, which is scene data saved in the blend-file, so it applies to a render started from outside just as to one started by clicking — that much is documented by where the API places the property. That the magenta or transparency therefore reaches the written-out picture file, and not only the preview, is a step past what either source says, so it is inferred.

The manual is explicit that output appears in the terminal when Blender is "manually started from a terminal" (https://docs.blender.org/manual/en/5.1/advanced/command_line/index.html, read 2026-07-30). It says nothing about a Blender started by another program rather than by a person at a terminal.


> **Brought up to date on 2026-08-26 by hand test.** Both streams were captured separately, in this version, on two machines, for five kinds of missing file, and the same files were then opened with a real window. See "Answered by hand test — question 13" at the end of this question's material.

### Operating system

The sources say nothing about this differing by operating system for missing files. No missing-file passage I found carries an operating-system qualifier, where the manual's Command Line Arguments page marks other things "(Windows only)" and "(Windows & Linux only)" freely — so the absence of such a mark here is at least consistent, though it is not a statement. `IMAGE_MISSING_RGBA` sits in Cycles' shared kernel files rather than in any per-platform file, which is where I looked; whether every device backend reaches that path is not something the supplied files settle.


> **Brought up to date on 2026-08-26 by hand test.** macOS and Linux were both measured in this version and agreed word for word on every missing-file run. Windows remains untested: no Windows machine has run it.

### Record of where I looked — question 13

Searched, case-insensitively, across the whole of `manual_5.1`, `api_5.1`, `release_notes_5.1` and `cycles_5.1` at once unless noted:

- Stream words: `stderr`, `stdout`, `standard error`, `standard output`, `standard out`, `sys.stdout`, `sys.stderr`, `sys.std`, `output stream`, `stream`, `pipe`, `redirect`, `2>&1`, `1>`, `file descriptor`. Total hits across manual and API: the Color Balance node's "Standard output image.", the Output Properties page's video and audio bitstream settings, one `sys.stdin` mention on the Command Line Arguments page (`-d, --debug` "Keeps Python's sys.stdin rather than setting it to None"), and pipeline-the-workflow-word hits in colour management and EEVEE ray tracing. In Cycles' files: the single `--cycles-print-stats` help string quoted above, and nothing else. The only `2>&1` anywhere is in a manual-building instruction on the manual's own translation contribution page.
- Missing-file words: `missing file`, `Missing Files`, `file not found`, `not found`, `cannot find`, `could not find`, `unable to open`, `could not open`, `cannot open`, `broken path`, `missing path`, `Find Missing`, `cannot be found`, `does not exist`, `no longer exist`, `unavailable`, `load the image`, `cannot load`, `failed to load`, `fails to load`, `Can't find`, `Cannot read`, `read error`. `unable to open`, `could not open`, `cannot open`, `broken path`, `missing path`, `failed to load`, `fails to load` and `Can't find` return nothing at all in any of the four sources. The rest led to the passages quoted above plus a long tail of unrelated uses (a vertex group that does not exist, a directory created if it does not exist, a modifier setting unavailable after binding, and so on).
- Placeholder and colour words, in case the behaviour was described without the word "missing": `magenta`, `pink`, `placeholder`, `broken`, `Broken`. `pink` appears nowhere in the 5.1 manual's render, files or troubleshooting sections. `magenta` in the manual is the Color Management blackbody shift, anaglyph glasses, and the sequencer's Show Missing Media line quoted above. `placeholder` gave the Output Properties "Create empty placeholder frames while rendering" setting — which is about frames not yet rendered, not files that are absent, and so ruled out — and the Broken Libraries passage quoted above.
- Message-destination words: `printed to the console`, `to the console`, `in the console`, `console output`, `terminal`, `warning message`, `error message`, `log message`, `logged`, `prints`, `reports an error`, `Error:`, `Warning:`. This was to catch a page that attributes a message to a stream. None does. The nearest anything comes is the manual's Cycles GPU Rendering page (https://docs.blender.org/manual/en/5.1/render/cycles/gpu_rendering.html, read 2026-07-30), which is written as a list of literal message texts — "Error: Out of memory", "CUDA Error: Kernel compilation failed", "The NVIDIA OpenGL driver lost connection with the display driver" — and never once says where they appear. Several troubleshooting pages say "Try running Blender from the command line, to see if any helpful error messages are printed", again with no stream.
- Pictures: I listed `manual_5.1/_images` filtered for `missing`, `notfound` and `broken`. There is no such image, so there is no screenshot of a missing-file state to open. I did not open any image for this question, because none was named in the passages found.

Pages read in full rather than searched: the manual's Video Sequencer preview Sidebar section around Show Missing Media, Linking and Appending (Broken Libraries and Relocate), Topbar External Data, Info Editor, Packed Data, File Paths, Directory Structure, Render Output Introduction, and Using Blender From The Command Line. In the API: `bpy.types.ID.is_missing`, `bpy.types.SequenceEditor.show_missing_media`, `bpy.types.BlendFileColorspace`, `bpy.ops.file.find_missing_files` and `bpy.ops.file.report_missing_files`. In Cycles: `engine.py`, `osl.py`, `operators.py`, `source/util/types_image.h`, `source/kernel/svm/image.h`, and `source/kernel/util/image_3d.h`.

Release notes: all 21 files of `release_notes_5.1` searched for `missing`, `not found` and `report`. Every hit is unrelated — a glTF/FBX export fix about a linked material, a multires cache path fix, documentation-coverage fixes, and one note that the Python `bpy` package was missing dependencies. Nothing about what a render prints when a file is absent.

Two adjacent things found and deliberately **ruled out as not answering the question asked**, recorded here so the absence is not mistaken for not looking:

- `Blender Sources/cycles_5.1/operators.py`, lines 84–88, contains `err_msg = tip_("Frame '%s' not found, animation must be complete") % filepath` followed by `self.report({'ERROR'}, err_msg)`. That is a missing file producing an exact message. But it belongs to `CYCLES_OT_denoise_animation`, the Denoise Animation operator, which reads already-rendered frames from disk. It is not a render, and it does not name a stream.
- `Blender Sources/cycles_5.1/osl.py` contains several missing-or-unreadable-file messages, including `report({'ERROR'}, rpt_("OSL query failed to open %s") % oso_path)` at line 312 and `report({'ERROR'}, "OSL script compilation failed, see console for errors")` at line 314. These run from `update_script_node`, which the API documents as "Compile shader script node" on `bpy.types.RenderEngine` (https://docs.blender.org/api/5.1/bpy.types.RenderEngine.html, read 2026-07-30) — a compile step, not the render, and again no stream is named. The phrase "see console for errors" points at the console as one undivided place, consistent with everything else above.

Ruled out and why: the modelling, animation, sculpting, physics and geometry-node parts of the manual, on the grounds that what a render prints is a property of rendering and the manual keeps that in `render/`, `advanced/command_line/` and `files/`, all of which I swept. The `bpy.ops.*` pages beyond the two file operators, on the grounds that they describe what an operator does, not what it prints.

---


## Answered by hand test — question 13, 2026-08-26

**LABEL: observed.** Everything in this section is what two real machines put on their two streams. None of it comes from documentation.

### What was done, and on what

Two machines, and their results are never merged.

- **The Creator's Mac** — a MacBook Pro with an Apple M1 Pro processor, macOS 26.5.2, Apple's own graphics and no second graphics card. Blender 5.1.2, build hash `ec6e62d40fa9`, built 2026-05-19 01:30:33, run from `/Applications/Blender 5.1.2.app/Contents/MacOS/Blender`.
- **The rented Linux machine, in Norway** — a Docker container holding half its host, running Ubuntu 22.04.5 LTS on kernel `6.8.0-136-generic`, with one NVIDIA GeForce RTX 3060 of 12,288 MiB on driver 595.84, about fifteen cores of an Intel Xeon E5-2697A v4, and a display server of its own on `DISPLAY=:1`. Its viewport is drawn by its processor, not by the card, and it is used as `root`. Blender 5.1.2, build hash `ec6e62d40fa9`, built 2026-05-19 02:18:37, run from `/opt/blender/5.1.2/blender`.

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

**The whole run landed on standard output and standard error was empty**, on both machines. Blender 5.1.2 wrote 51 lines on the Creator's Mac and 52 on the rented Linux machine: the version banner, `Read blend:`, the announcement lines `Rendering single frame`, `Rendering frame 1`, `Start rendering: Scene, ViewLayer` and `Engine: Cycles`, the progress lines, `Saved:` and `Time:` and `Blender quit`. Status 0.

### A missing image texture, rendered in Cycles

```
blender -b tex_missing.blend -o <work folder>/c2_#### -f 1 > out.txt 2> err.txt
```

**Blender 5.1 does complain, on standard output, and standard error is empty.** The line is:

```
00:00.323  cycles           | ERROR Image file <scene folder>/missing_tex.png does not exist.
```

**There are no quotation marks round the path.** Blender 5.0 prints the same sentence with single quotation marks round it; Blender 5.2 prints it as a `WARNING` and adds a second line; Blender 4.5 prints nothing at all.

The line sits among the ordinary progress lines, a little after `Loading render kernels`, and the run carries on. Standard output held 54 lines in all; the control run, on a scene with nothing missing, held 51 on the Creator's Mac and 52 on the rented Linux machine. Two of the extra lines are the complaint itself and one ordinary stage line, `Fra: 1 | Mem: 1M | Updating Images | Loading missing_tex.png`; the third is a `Synchronizing object | Cube` line that the Mac's control run happened not to print. The run ended at status **0** and wrote its picture.

Both machines gave this, word for word apart from the paths.

### The same missing texture, rendered in EEVEE

```
blender -b tex_missing.blend -E BLENDER_EEVEE -o <work folder>/c3_#### -f 1 > out.txt 2> err.txt
```

**In EEVEE the complaint is on standard output too, and standard error is empty.** The line is:

```
00:02.421  gpu.texture      | ERROR Failed to create GPU texture from Blender image
```

It does not name the file. Standard output held fifteen lines in all, ending `Saved:` and `Time:`; the status was 0 and the picture was written.

Blender 4.5 puts its EEVEE complaint — `GPUTexture: Blender Texture Not Loaded!` — on **standard error** instead. **So the stream changed between 4.5 and 5.0, and 5.1 keeps the newer behaviour.**

### The same missing texture, with quiet mode

```
blender -b tex_missing.blend -q -o <work folder>/c4_#### -f 1 > out.txt 2> err.txt
```

**Quiet mode does not remove the missing-file line in 5.1.** With `-q` added, standard output still held it, and standard error was still empty:

```
00:00.322  cycles           | ERROR Image file <scene folder>/missing_tex.png does not exist.
```

What `-q` removed was the ordinary progress lines. Blender 4.5 given `-q` prints nothing at all beyond its version banner.

### The same missing texture, with a log file

```
blender -b tex_missing.blend --log-file <work folder>/c5.log -o <work folder>/c5_#### -f 1 > out.txt 2> err.txt
```

**The log file holds everything, and the streams are left almost empty.** With `--log-file <work folder>/c5.log` added, the missing-file line was in the file:

```
00:00.318  cycles           | ERROR Image file <scene folder>/missing_tex.png does not exist.
```

along with the whole rest of the log. Standard output was left with the version banner and `Blender quit`, and standard error was empty. Blender 4.5 given the same option writes an **empty** file and leaves both streams untouched.

### A missing linked library

```
blender -b lib_missing.blend -o <work folder>/c6_#### -f 1 > out.txt 2> err.txt
```

**All five lines are on standard output, and standard error is empty.** Status 0, picture written.

```
00:00.356  reports          | Read library: '<scene folder>/lib.blend', '//lib.blend', parent '<direct>'
00:00.356  reports          | WARNING Unable to open '<scene folder>/lib.blend': No such file or directory
00:00.356  reports          | Cannot find lib '<scene folder>/lib.blend'
00:00.357  reports          | LIB: Collection: 'LibColl' missing from '<scene folder>/lib.blend', parent '<direct>'
00:00.358  reports          | WARNING 1 libraries and 1 linked data-blocks are missing (including 0 ObjectData), please check the Info and Outliner editors for details
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
00:00.346  reports          | ERROR Unable to pack file, source path '<scene folder>/missing_font.ttf' not found
00:00.346  reports          | ERROR Unable to pack file, source path '<scene folder>/missing_font.ttf' not found
00:00.346  geom.vfont       | WARNING Font file doesn't exist: //missing_font.ttf
```

Status 0, picture written. Blender 4.5 prints the same twelve, with the third carrying a source-file path and line number that 5.1 drops.

### A missing blend-file handed on the command line

```
blender -b <work folder>/not-here-at-all.blend -o <work folder>/c9_#### -f 1 > out.txt 2> err.txt
```

```
00:00.342  reports          | ERROR Cannot read file "<work folder>/not-here-at-all.blend": No such file or directory
Blender 5.1.2 (hash ec6e62d40fa9 built 2026-05-19 01:30:33)

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

**Blender 5.1.2 paints a missing image texture magenta**.

### Blender's own channels, with a real window

The same files were opened with Blender's window rather than in background, on both machines. A start-up script turned the Outliner into an Info editor, invoked the render with `bpy.ops.render.render('INVOKE_DEFAULT')`, photographed the window and read `bpy.data.window_managers[0].reports`, then quit from a timer. On the rented Linux machine the whole screen was photographed with its own `shot.sh`; on the Creator's Mac, Blender's own `bpy.ops.screen.screenshot(filepath=...)` was used instead, so that nothing but Blender's own window was captured.

**For a missing image texture, Blender's own channels say nothing in 5.1.** The Info editor was empty, no report box appeared along the bottom of the window, and the Blender Render window showed a magenta cube. The build has no `WindowManager.reports`: the script's check printed `this build has no WindowManager.reports`.

**For a missing linked library the channels do speak.** The Info editor listed the five messages with a warning triangle beside the last of them. Blender 4.5, given exactly the same file on the same two machines, leaves the Info editor empty while printing all five lines in the terminal.

For a scene with no camera, the render operator raised the error back into the script that called it rather than reporting it anywhere:

```
=== RENDER INVOKE RAISED: RuntimeError('Error: No camera found in scene "Scene"\n')
```

No Blender Render window opened and the Info editor stayed empty.

**One thing to know when reading the pictures.** Every run used a fresh settings folder, so Blender's Quick Setup panel is sitting over the middle of the 3D viewport in all of them. It never covered the Info editor, the status bar or the Blender Render window, and it did not stop the render.

### The two standing lines, brought up to date

**Outside the interface.** Settled by measurement, both ways round, for this version on both machines. Driven from outside with no window, the answers are the ones set out above: a line on standard output for a missing image texture, five lines on standard output for a missing library, twelve for a missing font, one for a missing blend-file, and nothing at all for missing sequencer footage — **and standard error empty in every one of them except 4.5 rendering in EEVEE**. Driven from inside its own window, the same files give nothing in the Info editor for the texture and five entries for the library. **So the two settings do not agree, and which way they differ depends on the kind of file.**

**Operating system.** Settled for macOS and for Linux, and only for those two. Every run above was made on both, and **the two machines agreed word for word in every one**, the file paths aside — the same messages, on the same streams, with the same empty streams, and the same colours in the pictures to four decimal places. **Windows is untested and stays untested**: no Windows machine has run it. **It is untried rather than untestable** — this project has a route to a Windows computer, described in the equipment section of `Preparation/Blender Research/Method.md`, and every machine that route has produced booted a **Server** edition, so a Windows answer would have to name its edition, so whether its hidden console changes what a starting program can capture is not settled by anything here.

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

8. **Anything on Microsoft Windows** — no Windows machine has run it. **It is untried rather than untestable** — this project has a route to a Windows computer, described in the equipment section of `Preparation/Blender Research/Method.md`, and every machine that route has produced booted a **Server** edition, so a Windows answer would have to name its edition. This leaves unsettled what `echo %ERRORLEVEL%` and `$LASTEXITCODE` report for every run above, whether Ctrl-Break behaves as Ctrl-C does, and whether the hidden console changes what a starting program can capture.
9. **The File ▸ External Data ▸ Report Missing Files menu item** — the one destination the documentation actually names for a missing-file warning. Reading its exact wording needs a menu picked with a pointer, and neither machine can be sent a pointer click into Blender's window.
10. **A render stopped by pressing Esc inside the window** — same reason: neither machine can be sent a key press into Blender's own window.
11. **An unwritable output folder on Linux** — the rented machine is used as `root`, which is not stopped by a folder's permissions, so that run gave 0 there. Settling it needs an account that is not `root`.
12. **A working HIP or oneAPI render device** — neither machine has an AMD or an Intel card. What the *refusal* looks like is settled; what a working run gives back is not.
13. **Blender loaded as a Python module** rather than run as a program — neither machine has such a build.
14. **Whether a display server attached to a windowless run changes what reaches standard error.** The runs above had no `DISPLAY` set. An earlier round of this topic recorded display-library complaints arriving on standard error when one was attached, and that condition was not repeated here.


---

2026-08-26 — Claude Opus 5 (Claude Code subagent) — Reports, Blender 5.1.

# Reports — Architecture questions, second pass (Blender 5.0)

**Sources consulted, and the date.** Blender's 5.0 manual (`https://docs.blender.org/manual/en/5.0/`), Blender's 5.0 Python API documentation (`https://docs.blender.org/api/5.0/`), the 5.0 release notes (`https://developer.blender.org/docs/release_notes/5.0/`), and Cycles' own files as shipped with Blender 5.0. All four were taken on 2026-07-30, and that is the date read for everything quoted below. Manual pictures were opened from the `_images` folder beside the manual text.

I read only the question sheet and files inside Blender Sources/. I did not open any other part of this project.

Questions answered here: **12** and **13** only. I read the whole sheet first, as instructed, and answered nothing else.

No helper agents were used. Every search and every file opened below was done by me.

---

## 12. What ending status does a run of Blender give back to whatever started it, for each of the ways a run can end — finishing normally, being interrupted, and being refused because something it was handed cannot be used?

**Label: not in the sources**, for the question as asked. One narrow piece of it is **documented**, and one case of the third kind is **observed** in Cycles' own files. I take those apart below so the pieces are not confused with each other.

### The short answer

Across the whole 5.0 manual, the whole 5.0 Python API documentation, and the whole 5.0 release notes, the phrase "exit code" appears exactly once, and "exit status", "return code" and "errorlevel" appear nowhere at all. There is no page, anywhere in the three documentation sources, that states what number a run of Blender hands back on finishing normally, on being interrupted, or on being refused a bad input. That is the answer to the question as it was asked.

### The one piece that is documented

The single mention is a command-line option that lets you *set* the status for one specific case — a Python exception in a script run from the command line.

Exact words:

> `--python-exit-code <code>`
>
> Set the exit-code in [0..255] to exit if a Python exception is raised
> (only for scripts executed from the command line), zero disables.

Source: the manual, `https://docs.blender.org/manual/en/5.0/advanced/command_line/arguments.html`. Date read: 2026-07-30.

What this settles: for one named cause of ending — a Python exception raised by a script given on the command line — the status handed back is whatever number you put in this option, and it must be in the range 0 to 255. What it plainly does not settle: what the status is when you do not pass this option (the manual does not give the option's default value), and what the status is for any of the three endings the question actually names.

Two further things the manual says about ending, neither of which gives a number:

> `--debug-exit-on-error`
>
> Immediately exit when internal errors are detected.

and, under "Argument Parsing":

> `blender -ba test.blend`
>
> …will exit since `-ba` is an unknown argument.

Both from the same manual page and the same date. The second is a case of "refused because something it was handed cannot be used" — a bad argument — and the manual says Blender exits, but says nothing about the status it exits with. Several other options are also documented as exiting (`-h`/`--help`, `-v`/`--version`, `-r`/`--register`, `--unregister`, `--log-list-categories`), and none of them names a status either.

### The one case that is observed, in Cycles' own files

Cycles' add-on parses its own command-line options after the `--` separator. When the device override it is handed is rejected, it ends the whole Python process.

Exact lines, from `Blender Sources/cycles_5.0/engine.py`, lines 38 to 41, inside the function `_parse_command_line()`:

```
    if args.cycles_device:
        import _cycles
        if not _cycles.set_device_override(args.cycles_device):
            sys.exit(1)
```

Label: **observed** — this is what the file says, read on 2026-07-30.

This is a "refused because something it was handed cannot be used" ending: the thing handed in is the value of `--cycles-device`, and the number `1` is written in the file.

Two steps beyond that line, which I am labelling separately so nobody mistakes them for what the file says:

- **Inferred**: that `set_device_override` returns false when the device name is not valid or not available on the machine. The step: the manual documents `--cycles-device` as taking one of a fixed list of values ("Valid options are: CPU CUDA OPTIX HIP ONEAPI METAL", manual, same command-line arguments page), and the Cycles file passes the given string straight to `set_device_override` and exits when it returns false. The function itself is not in the files I was given — `Blender Sources/cycles_5.0/source` contains only `kernel` and `util`, no device or scene code — so I cannot read what it actually checks.
- **Inferred**: that the operating system therefore sees a status of 1 from the whole Blender run. The step: the API documentation says of `sys.exit()` that "`sys.exit()` will close Blender immediately" (`https://docs.blender.org/api/5.0/info_gotchas_crashes.html`, date read 2026-07-30), so the call does end the Blender process rather than just the add-on; the number 1 is the argument written in the Cycles file. Nothing in any of the four sources states what status the process then reports to its caller. I did not run Blender to check this.

### Where I looked, and what I ruled out

Searches run over the whole of `manual_5.0`, `api_5.0`, `release_notes_5.0` and `cycles_5.0`, case-insensitive:

- `exit code`, `exit status`, `exit-code`, `exitcode`, `return code`, `errorlevel`, `error level`, `exit value`, `status code` — one hit in the whole manual (`--python-exit-code`, quoted above), nothing in the API documentation, nothing in the release notes, nothing in Cycles' files.
- `sys.exit`, `EXIT_SUCCESS`, `EXIT_FAILURE`, `%ERRORLEVEL%`, `$?` — nothing in the manual; in the API documentation only the "sys.exit" section of `info_gotchas_crashes` (quoted above); in Cycles' files only `engine.py` line 41 (quoted above) and the unrelated `engine.exit()` / `_cycles.exit()` shutdown pair at lines 56–58.
- `exit` as a whole word across the API documentation — 15 files, all checked. The only ones that were not the word appearing inside an unrelated sentence were `bpy.ops.wm.save_mainfile`'s `exit` parameter ("Exit, Exit Blender after saving"), `Preferences` "Save preferences on exit when modified", and `Gizmo.exit`. None concerns a status handed back.
- `Ctrl-C`, `Ctrl+C`, `SIGINT`, `SIGTERM`, `KeyboardInterrupt`, `signal` — every hit in the manual is copy-and-paste or an unrelated word. The only relevant hit anywhere is in the API documentation: "Ctrl-C in the terminal (Ctrl-Break on Windows) will quit the script early" (`https://docs.blender.org/api/5.0/info_tips_and_tricks.html`, date read 2026-07-30). That is about ending a running Python script, not about what a Blender run reports afterwards, so it does not answer the "interrupted" part.
- `quit`, `quits`, `quitting` across the manual and API documentation — all hits are the Quit menu item, the `quit.blend` recovery file, or unrelated. No status.
- `abort`, `cancel`, `interrupt` in the manual's `render` and `advanced` trees — only `--disable-abort-handler` ("Disable the abort handler.") and the note that a render job "can be canceled and resumed at the last rendered frame by changing the frame range" (`https://docs.blender.org/manual/en/5.0/render/output/introduction.html`). Neither names a status.
- `subprocess`, `render farm`, `renderfarm`, `batch process`, `shell script`, `automation` — the only hit that touches on a program driving Blender is a release note quoted under question 13 below, about log output format, not about status.

Pages I opened in full and ruled out: the manual's whole command-line section — `advanced/command_line/index`, `advanced/command_line/arguments`, `advanced/command_line/render`, `advanced/command_line/extension_arguments`, and all four `advanced/command_line/launch` pages (index, linux, macos, windows); the manual's `troubleshooting/startup` (its "Common Startup Messages" section lists console messages, none of them a status) and `troubleshooting/crash` and `troubleshooting/python`; the API documentation's `info_gotchas_crashes` and `info_tips_and_tricks`; the release notes' `core.md` and `python_api.md`; and the whole of `cycles_5.0/engine.py` and `cycles_5.0/__init__.py`.

Two pictures opened, both ruled out: `advanced_command-line_launch_windows_console.png` shows a Command Prompt window in which Blender has been started and has printed "Blender quit", with the prompt returning underneath — no status is shown; `advanced_command-line_launch_linux_console.png` shows a terminal with audio-backend warnings only.

One thing I deliberately set aside: the API documentation has a list of "Operator Return Items" with values `RUNNING_MODAL`, `CANCELLED`, `FINISHED`, `PASS_THROUGH`, `INTERFACE` (`https://docs.blender.org/api/5.0/bpy_types_enum_items/operator_return_items.html`). Those describe what one operator hands back inside Blender, not what a run of Blender hands back to the program that started it, which is what the question asks for. I ruled them out on that ground.

**Not covered.** I cannot say what status a normally finished render gives back, what status an interrupted run gives back, or what status any refusal other than the Cycles device case gives back. I cannot say what the default value of `--python-exit-code` is. I cannot say whether a render that fails part-way (out of memory, unwritable output path, missing file) changes the status at all. I did not run Blender, so nothing here is a measurement.

**Outside the interface.** This question is only about running Blender from outside its own window, so everything above already concerns that case. The one documented item, `--python-exit-code`, is limited by its own words to "scripts executed from the command line". The Cycles `sys.exit(1)` sits in `_parse_command_line()`, which the same file only reaches through `init()`, and the option it reads is a command-line one, so it too is an outside-the-window path. What happens inside the interface is not stated, and a status handed back to a caller has no meaning there in any case. The sources do not compare the two.

> **Brought up to date on 2026-08-26 by hand test.** The status a run hands back was measured on two machines in this version, and all of it was measured from outside Blender's own window, which is the only place a status exists. See "Answered by hand test — question 12" at the end of this question's material.


**Operating system.** The sources do not say. The manual gives one range, "[0..255]", for `--python-exit-code` with no operating system attached, and the Cycles line is plain Python with no operating system branch around it. Nothing in the four sources says whether the status a caller receives differs on Windows, macOS or Linux, and I will not guess. I note only that the manual marks other options as platform-limited where they are (for example `-con, --start-console` is "(Windows only)") and marks nothing platform-limited here.

> **Brought up to date on 2026-08-26 by hand test.** macOS and Linux were both measured in this version and agree almost everywhere; the places they do not are named in "Answered by hand test — question 12" below. Windows remains untested: no Windows machine has run it. **Untried rather than untestable: a Windows machine can be rented, and every one this project has read reported a Server edition — see the equipment section of `Preparation/Blender Research/Method.md`.**


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

**Label: not in the sources.**

**A note on the reading.** "Output streams" is ambiguous. I answered the reading I believe is meant: the two text streams a program that starts Blender receives from it — standard output and standard error. The other reading would be the render's own picture outputs (the rendered image, its passes, the files written by output nodes), and I say plainly what I did not chase: I did not treat the question as being about pixels.

### The short answer

None of the four sources ever names a stream. Searching the whole 5.0 manual, the whole 5.0 API documentation and the whole 5.0 release notes for `stdout`, `stderr`, `standard output`, `standard error`, `sys.stdout`, `sys.stderr`, `output stream`, `error stream`, `redirect` and `2>&1` produced exactly one hit in the manual, and it is not about this: `https://docs.blender.org/manual/en/5.0/compositing/types/color/adjust/color_balance.html` uses the words "Standard output image." about a node socket. The API documentation and the release notes have no hit at all.

So the sources do not say which stream carries what when a file the scene points at is missing, for the plain reason that they do not distinguish the streams anywhere.

### The only place a stream is named at all, in any of the four sources

Cycles' own files name `stderr` once, and it has nothing to do with missing files. From `Blender Sources/cycles_5.0/engine.py`, lines 13 to 15, inside `_configure_argument_parser()`:

```
    parser.add_argument("--cycles-print-stats",
                        help="Print rendering statistics to stderr",
                        action='store_true')
```

Label: **observed**, read 2026-07-30. What it settles: Cycles' own help text for `--cycles-print-stats` says those statistics go to standard error. What it does not settle: anything about missing files, and anything about where any other line goes.

There is a second mention of the word in `Blender Sources/cycles_5.0/source/util/defines.h`, lines 126 to 127 — "environment variables `MTL_LOG_LEVEL` should be set to `MTLLogLevelDebug`, and `MTL_LOG_TO_STDERR` should be set to `1`" — which is about Apple Metal shader debug logging, not about Blender's own output. I ruled it out.

The same manual page that documents `--cycles-print-stats` describes it only as "Log statistics about render memory and time usage" and does not mention a stream (`https://docs.blender.org/manual/en/5.0/advanced/command_line/arguments.html`, date read 2026-07-30).

### What the sources do say about a missing file, none of it about streams

These are the closest things I found, and I list them so the silence can be checked rather than taken on trust.

For a missing linked blend-file, the manual describes what Blender does, but at load time and to the data, not to any stream:

> Broken Libraries
>
> If Blender cannot find a library while loading a blend-file,
> it will create placeholder data-blocks to replace missing linked ones.
> That way, references to the missing data are not lost, and by relocating the missing library,
> the lost data can be automatically restored.

Source: the manual, `https://docs.blender.org/manual/en/5.0/files/linked_libraries/link_append.html`. Date read: 2026-07-30. Label: **documented**, for what it says — placeholder data-blocks — and it says nothing about output.

The matching Python property confirms the placeholder but again names no stream:

> `is_missing`
>
> This data-block is a place-holder for missing linked data (i.e. it is [an override of] a linked data that could not be found anymore)

Source: the API documentation, `https://docs.blender.org/api/5.0/bpy.types.ID.html`. Date read: 2026-07-30.

For missing external files generally, the manual describes a check the user runs by hand, and puts its result inside Blender's own window:

> Report Missing Files
>
> This option is useful to check if there are links to unpacked files that no longer exist.
> After selecting this option, a warning message will appear in the Info editor's header.
> If no warning is shown, there are no missing external files.

Source: the manual, `https://docs.blender.org/manual/en/5.0/interface/window_system/topbar.html`. Date read: 2026-07-30. This is a menu item and the Info editor, so it is inside the interface, and it is not something a render does on its own. The matching operator, `bpy.ops.file.report_missing_files()`, is documented with the single line "Report all missing external files" and no mention of output (`https://docs.blender.org/api/5.0/bpy.ops.file.html`, date read 2026-07-30).

Cycles' own files do contain a missing-file path for one kind of file a scene can point at — an external OSL shader script. From `Blender Sources/cycles_5.0/osl.py`, lines 308 to 312, inside `update_script_node()`:

```
        else:
            ok = False
            report({'ERROR'}, rpt_("OSL query failed to open %s") % oso_path)
    else:
        report({'ERROR'}, "OSL script compilation failed, see console for errors")
```

Label: **observed**, read 2026-07-30. Two limits on it, both important. First, these go through Blender's report system, not to a named stream — the file says "see console for errors" without saying which console stream. Second, `update_script_node` is documented as "Compile shader script node" (`https://docs.blender.org/api/5.0/bpy.types.RenderEngine.html`, date read 2026-07-30), and nothing in the sources says a render triggers it, so I am not claiming this is what a render does.

### What the sources say about where render output goes at all

This is the whole of it, and it never reaches a stream.

The 5.0 release notes say the logging system now carries render progress and operator reports:

> Most of Blender now uses the same logging system,
> including operators reports and background render progress.
> The output format has changed, and log levels are easier to control.

Source: the release notes, `https://developer.blender.org/docs/release_notes/5.0/core/`. Date read: 2026-07-30. Label: **documented**.

And the Python API release notes warn who is affected:

> The format of logging output has changed,
> including background render progress.
> Add-ons or render farms parsing this output may need to be updated.

Source: the release notes, `https://developer.blender.org/docs/release_notes/5.0/python_api/`. Date read: 2026-07-30.

The manual gives one option that moves the log somewhere else entirely, and one that splits what is printed by kind rather than by stream:

> `--log-file <filepath>`
>
> Set a file to output the log to.

> `-q, --quiet`
>
> Suppress status printing (warnings & errors are still printed).

Both from the manual, `https://docs.blender.org/manual/en/5.0/advanced/command_line/arguments.html`. Date read: 2026-07-30. The `--quiet` line is the closest the documentation comes to separating one kind of message from another — status on one side, warnings and errors on the other — but it describes what is suppressed, not where either kind is written.

The manual's own description of the console treats it as one thing, not two:

> The Console Window, also called a Terminal, is an operating system text window that displays
> messages about Blender's operations, status, and internal errors.

Source: the manual, `https://docs.blender.org/manual/en/5.0/advanced/command_line/index.html`. Date read: 2026-07-30.

### Where I looked, and what I ruled out

Searches run over the whole of `manual_5.0`, `api_5.0`, `release_notes_5.0` and `cycles_5.0`, case-insensitive:

- `stdout`, `stderr`, `standard output`, `standard error`, `sys.stdout`, `sys.stderr`, `output stream`, `error stream`, `redirect`, `2>&1`, `pipe` — results given above. One irrelevant manual hit, zero in the API documentation, zero in the release notes, two in Cycles' files (both quoted above, one irrelevant).
- `missing file`, `file not found`, `not found`, `missing`, `does not exist`, `doesn't exist`, `no longer exist`, `nonexistent`, `non-existent` — swept across the manual and the API documentation. Every hit that concerned an external file the scene points at is quoted above. The rest were unrelated uses of the word ("missing light", "missing caustics", "missing data layers", "The RNA type or default when not found", and so on).
- `unable to open`, `cannot open`, `cannot read`, `cannot be found`, `could not open`, `could not find`, `failed to open`, `failed to load` — seven hits in the whole manual, all listed and all unrelated except the Broken Libraries one quoted above.
- `Error:` and `Warning:` as literal strings in the manual — three and four hits respectively, all unrelated (Cycles GPU out-of-memory and kernel-compilation messages, a motion-tracking average error, and prose warnings in import/export pages).
- `magenta`, `pink` — the only relevant hit is the Video Sequencer's "Show Missing Media", "Render missing images/movies with a solid magenta color. When disabled, missing content will render fully transparent." (`https://docs.blender.org/manual/en/5.0/editors/video_sequencer/preview/sidebar.html`). That describes pixels in the picture, not a stream, so I ruled it out for the reading I took; it would belong to the other reading of the question, and I flag it here for that reason.
- `renders as black`, `renders transparent`, `treated as black` — nothing about missing files.
- `subprocess`, `render farm`, `automation`, `batch process` — the only relevant hit is the release note quoted above.

Pages and files opened in full and ruled out: the manual's `editors/info_editor` (it says the Info editor "logs the executed operators as well as errors, warnings, and informational messages", which is inside the window and names no stream); `troubleshooting/startup`, `troubleshooting/crash`, `troubleshooting/python`; `files/blend/open_save`; `files/linked_libraries/link_append`; `interface/window_system/topbar`; the whole command-line section including all four launch pages; the API documentation's `bpy.types.Image` (its `has_data`, "True if the image data is loaded into memory", is the only property touching on an unloaded file, and it names no output), `bpy.types.ID`, `bpy.ops.file`, `bpy.types.RenderEngine`; the release notes' `core.md`, `python_api.md`, `rendering.md`, `cycles.md`, `pipeline_io.md`, `bugfixes.md`; and, in Cycles' files, `engine.py`, `__init__.py`, `osl.py` and `operators.py`.

One structural point about Cycles' files, since it limits what could ever be found there: `Blender Sources/cycles_5.0/source` contains only `kernel` and `util`. There is no scene, image, or device code in it, so no image-loading or file-opening message can be read from these files at all. The four Python files at the top of `cycles_5.0` are the whole of what could carry such a message, and I read all four.

Two pictures opened and ruled out for this question: `advanced_command-line_launch_linux_console.png` (a terminal showing only JACK audio-backend warnings) and `advanced_command-line_launch_windows_console.png` (a Command Prompt showing preferences and blend-file reads, then "Blender quit"). Neither shows a missing file, and a screenshot cannot show which stream a line came from in any case.

**Not covered.** I cannot say which stream carries a missing-file message, because the sources never separate the streams. I cannot say what text such a message contains, because no source quotes one. I cannot say whether the answer differs between kinds of missing file — image texture, linked library, sound, movie strip, volume file, cache, font — because only the linked-library case is described at all, and that description is about data-blocks rather than output. I cannot say whether the render still produces a picture, or with what in place of the missing content, except for the Video Sequencer case quoted above. I did not run Blender.

**Outside the interface.** The question already concerns streams, which only exist outside Blender's window, and the sources are silent there. The one thing that clearly differs between inside and outside is documented: the "Report Missing Files" result "will appear in the Info editor's header", which is a thing only a person looking at the window can see. The release note that background render progress now goes through the shared logging system is the only statement tying render reporting to the outside case, and it concerns progress, not missing files. Nothing else in the sources distinguishes the two.

> **Brought up to date on 2026-08-26 by hand test.** Both streams were captured separately, in this version, on two machines, for five kinds of missing file, and the same files were then opened with a real window. See "Answered by hand test — question 13" at the end of this question's material.


**Operating system.** The sources do not say. Nothing in any missing-file passage, and nothing in the logging passages, is marked as differing by operating system. I note only that the manual's Windows launch page says the Console Window is created and then "toggled off", so that on Windows a person who has not opened it will not see any of this — but that is about a window being visible, not about what is written to a stream, and the manual does not say the written output differs.

> **Brought up to date on 2026-08-26 by hand test.** macOS and Linux were both measured in this version and agreed word for word on every missing-file run. Windows remains untested: no Windows machine has run it.


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

2026-08-26 — Claude Opus 5 (Claude Code subagent) — Reports, Blender 5.0.

# Reports — Blender 5.1 — questions 12 and 13

**Sources consulted, and on what date.** Blender's 5.1 manual (`Blender Sources/manual_5.1`, including two pictures from its `_images` folder), Blender's 5.1 Python API documentation (`Blender Sources/api_5.1`), Blender's 5.1 release notes (`Blender Sources/release_notes_5.1`), and Cycles' own files as shipped with Blender 5.1 (`Blender Sources/cycles_5.1`). All four were taken on 2026-07-30, and that is the date read for every finding below.

I read only the question sheet and files inside Blender Sources/. I did not open any other part of this project.

*One disclosure so that line can be trusted at face value.* Before I did anything, the tool that runs me pasted the text of this project's `CLAUDE.md` into my context automatically. I did not open that file, I did not follow the pointer in it, and I opened nothing it names. Its text is a signpost only and makes no claim about how Blender behaves, so nothing below can have come from it. I did the work with no helper agents, so this declaration covers only me. One further small thing, recorded for completeness: while checking that this file had saved under the right name, the folder listing showed me the names of the other findings files sitting beside it. I opened none of them, and a file name states nothing about how Blender behaves.

---

## Question 12

**What ending status does a run of Blender give back to whatever started it, for each of the ways a run can end — finishing normally, being interrupted, and being refused because something it was handed cannot be used? The status wanted is the one a program that started Blender receives, not anything Blender prints about itself.**

### Short answer

**LABEL: not in the sources** — for the question as asked.

None of the four sources states the ending status a run of Blender hands back to the program that started it. Not for a normal finish, not for an interruption, not for a refused input. The sources never name a status value for any of those three endings.

Three narrower, related things *are* stated, and each is given below with its own label. None of them answers the question for a plain render run.

### The three things the sources do say

#### (a) A status Blender can be *told* to use, for one specific failure

**LABEL: documented.**

The manual lists a command-line argument whose entire purpose is to set an ending status:

> `--python-exit-code <code>`
>
> Set the exit-code in [0..255] to exit if a Python exception is raised
> (only for scripts executed from the command line), zero disables.

- Source: Blender's manual, https://docs.blender.org/manual/en/5.1/advanced/command_line/arguments.html
- Date read: 2026-07-30

Read plainly, this says four things. There is a thing called an exit-code. It can be any whole number from 0 to 255. Blender will exit with the one you name if a Python script you started from the command line raises an exception. And naming zero disables that.

**One step a reader is likely to take here, and why it is not taken. LABEL: inferred.** "Zero disables" says what happens when zero is the value. It does not say what the value is when the argument is left out. So it does not follow from these words that the mechanism is off by default, and nothing below rests on it.

What the manual does *not* say, anywhere: what Blender exits with when this mechanism is not in play, or when no Python is involved at all.

#### (b) A number that a registered command is told to return

**LABEL: documented** (for the quote) / **inferred** (for what it means for the ending status).

The API documentation for `bpy.utils.register_cli_command` — the function that adds a command usable with the `-c` / `--command` argument — describes the callback it takes:

> `execute` (Callable[[list[str]], int]) – Callback, taking a single list of strings and returns an int.
> The arguments are built from all command-line arguments following the command id.
> The return value should be 0 for success, 1 on failure (specific error codes from the os module can also be used).

- Source: Blender's Python API documentation, https://docs.blender.org/api/5.1/bpy.utils.html
- Date read: 2026-07-30

The example script on that same page confirms the shape in practice — its command handler ends `return 0` on both paths, once after printing help and once after printing system information.

The manual's entry for the matching argument reads:

> `-c, --command <command>`
>
> Run a command which consumes all remaining arguments.
> Use `-c help` to list all other commands.
> Pass `--help` after the command to see its help text.
>
> This implies `--background` mode.

- Source: Blender's manual, https://docs.blender.org/manual/en/5.1/advanced/command_line/arguments.html
- Date read: 2026-07-30

**The inferred part, with the step written out.** The quoted words say a callback returns a number, and that 0 means success and 1 means failure. They do *not* say that this number becomes the status the starting program receives. To get there a reader has to supply the step "a number described as an error code, returned by the handler of a command that forces background mode, is passed out of the process as its ending status." That step is mine, not the source's. I flag it as a plausible reading, nothing stronger, and I would not build on it. Note too that this only ever applies to runs driven through `-c` / `--command`, not to a run that simply renders.

#### (c) A status Cycles' own file asks for when it is handed a device it cannot use

**LABEL: observed** (for the lines) / **inferred** (for what the starting program receives).

Cycles' own files include the add-on's command-line parsing. The argument and its handling read, exactly:

```
    parser.add_argument("--cycles-device",
                        help="Set the device to use for Cycles, overriding user preferences and the scene setting."
                             "Valid options are 'CPU', 'CUDA', 'OPTIX', 'HIP', 'ONEAPI', or 'METAL'."
                             "Additionally, you can append '+CPU' to any GPU type for hybrid rendering.",
                        default=None)
```

and

```
    if args.cycles_device:
        import _cycles
        if not _cycles.set_device_override(args.cycles_device):
            sys.exit(1)
```

- Source: Cycles' own files, file `engine.py`, at the top level of the Cycles add-on folder. The first block is in the function `_configure_argument_parser()`, lines 16–20. The second block is the closing four lines of the function `_parse_command_line()`, lines 38–41. This is the only `sys.exit` anywhere in Cycles' Python files.
- Date read: 2026-07-30

This is a genuine instance of the third ending in the question — a run refused because something it was handed cannot be used. The device name given to `--cycles-device` is checked; if the check comes back false, the code calls `sys.exit(1)`.

**The inferred part, with the step written out.** The file says `sys.exit(1)`. It does not say what the program that started Blender receives. To claim the caller sees 1, a reader has to supply the step "a `sys.exit(1)` raised inside the Cycles add-on while it is starting up, in the Python interpreter Blender has embedded, travels all the way out and becomes the whole run's ending status." Nothing in the files I was given shows that journey — not the embedding, not whether anything upstream catches it. So the honest statement is: *the file asks for 1*; whether 1 is what arrives is not shown here.

Also worth stating plainly: this covers exactly one kind of unusable input — a bad Cycles device name. A missing blend-file, an unwritable output path, an unknown render engine, a corrupt scene: the sources say nothing about the status for any of those.

### Two more places that touch on ending, but give no status

**LABEL: documented**, in both cases, for the quoted words alone.

The manual says Blender ends when handed an argument it does not know:

> `blender -ba test.blend`
>
> …will exit since `-ba` is an unknown argument.

and it lists a debug argument that ends a run early:

> `--debug-exit-on-error`
>
> Immediately exit when internal errors are detected.

- Source: Blender's manual, https://docs.blender.org/manual/en/5.1/advanced/command_line/arguments.html
- Date read: 2026-07-30

Both describe a run ending. Neither names the status it ends with.

### Interruption: nothing at all

**LABEL: not in the sources.**

I could find no statement, anywhere in the four sources, about a run of Blender being interrupted from outside — no mention of a signal, of Ctrl-C stopping a background render, or of what such a run hands back. The manual does mention stopping work from inside the interface (pressing Esc to abort a viewport render or a fluid simulation, clicking a cancel button in the status bar), but those are things a person does at a window, and none of them is described as ending the run or as producing any status.

### NOT COVERED

- The answer settles nothing about the status of a normal, successful render run. That is the most ordinary ending of all, and the sources are silent on it.
- It settles nothing about interruption.
- It settles almost nothing about refusal: one narrow case (an unusable Cycles device name) is shown to ask for 1, and even that is one inference away from the caller's view.
- It does not settle whether the numbers in (b) and (c) actually reach the starting program.
- It does not settle what happens when two of these could apply at once — say, a Python exception under `--python-exit-code` inside a run that would also have failed for another reason.
- It says nothing about a run that crashes, since the sources give no status for that either.

### Outside the interface

This question only has meaning outside the interface, and that is where the sources are thinnest.

The one mechanism that is documented outright, `--python-exit-code`, is explicitly fenced to outside-the-interface use: "only for scripts executed from the command line". The `-c` / `--command` route in (b) is fenced the same way — the manual says it "implies `--background` mode", so it never applies to a person clicking. The Cycles case in (c) is parsed from arguments given after `--` on a command line, which the manual's own example shows in a background render: `blender -b file.blend -f 20 -- --cycles-device OPTIX`.

So all three findings are outside-the-interface findings already. What is missing is missing everywhere: the sources never state what a run hands back, whether it was started by a person or by another program. Nothing in them draws a difference between the two for this.


> **Brought up to date on 2026-08-26 by hand test.** The status a run hands back was measured on two machines in this version, and all of it was measured from outside Blender's own window, which is the only place a status exists. See "Answered by hand test — question 12" at the end of this question's material.

### Operating system

Not stated. The manual gives the range as "[0..255]" with no operating-system qualifier on it.

That silence is worth a little weight, because the same page does mark operating-system-specific arguments where they exist — `/?` is marked "Windows only", `-r, --register` is marked "Windows & Linux only", `--qos` is marked "Windows only", `--debug-ghost` is marked "Linux only". `--python-exit-code` carries no such mark. But "the manual did not mark it" is not the same as "the manual said it holds everywhere", and I will not turn one into the other. Same for the two numbers in (b) and (c): no operating system is mentioned in connection with either.


> **Brought up to date on 2026-08-26 by hand test.** macOS and Linux were both measured in this version and agree almost everywhere; the places they do not are named in "Answered by hand test — question 12" below. Windows remains untested: no Windows machine has run it. **Untried rather than untestable: a Windows machine can be rented, and every one this project has read reported a Server edition — see the equipment section of `Preparation/Blender Research/Method.md`.**

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

**Operating system.** Settled for macOS and for Linux, and only for those two. Every result above was got on both, and **they agree in every one except a single case**: an unwritable output folder, where the Linux machine is used as `root` and is therefore not stopped by the folder's permissions — a difference in how the machine is used rather than in Blender. Everything else — the clean finishes, the interrupts, the signals, the refused files and arguments, the devices, the Python failures, the registered commands and the whole of the trap — came back the same on both. **Windows is untested and stays untested**: no Windows machine has run it, so what `echo %ERRORLEVEL%` or `$LASTEXITCODE` would report, and whether Ctrl-Break behaves as Ctrl-C does, are not settled by anything here.

### What these runs do not settle

- Anything on Microsoft Windows.
- What a working HIP or oneAPI device gives back. Both were refused for want of an AMD or an Intel card, so the *refusal* is settled and the success is not.
- What Blender loaded as a Python module hands back. Neither machine has such a build.
- Whether an unwritable output folder gives 1 on Linux, which needs an account that is not `root`.
- Why Blender 4.5 behaves as it does when handed an engine name that does not exist. What happened is recorded; nothing here explains it.


## Question 13

**When a file the scene points at is not there, what does a render put on each of its output streams?**

### A note on the wording first

The question can be read two ways, and I say so rather than choosing silently.

- **The reading I answer:** an external file that the loaded scene refers to is missing from disk — an image or movie used by a texture or a strip, a sound, a cache, a linked blend-file — and a render is then run. What text lands on which of the run's output streams.
- **The other reading:** the blend-file itself, handed to Blender on the command line, is not there.

I answer the first. As it happens the outcome is the same either way, because the sources never separate a run's output into streams at all, so neither reading can be answered from them.

### Short answer

**LABEL: not in the sources.**

The sources do not describe Blender's output as more than one stream, except in a single place that has nothing to do with missing files. They never state what a render prints when a file the scene points at is missing, and so they certainly never state which stream it goes on.

### What the sources say about the output, and why it does not answer the question

**LABEL: documented**, for each quote below.

The manual treats a run's text output as one undivided thing:

> The Console Window, also called a Terminal, is an operating system text window that displays messages about Blender's operations, status, and internal errors.

and, on the same page:

> When Blender is manually started from a terminal, Blender output is shown in the Console Window it is started from.

- Source: Blender's manual, https://docs.blender.org/manual/en/5.1/advanced/command_line/index.html
- Date read: 2026-07-30

"Blender output", singular, going to one window. Operations, status and internal errors are named as kinds of message, not as separate destinations.

The manual's troubleshooting page says the same in different words:

> The Blender Console Window can display many different types of status and error messages. Some messages simply inform the user what Blender is doing, but have no real impact on Blender's ability to function. Other messages can indicate serious errors that will most likely prevent Blender carrying out a particular task and may even make Blender non-responsive or shut down completely. The Blender Console Window messages can also originate internally from within the Blender code or from external sources such as Python scripts.

- Source: Blender's manual, https://docs.blender.org/manual/en/5.1/troubleshooting/startup.html
- Date read: 2026-07-30

Again: kinds of message, one window.

The closest the manual comes to dividing the output is an argument that sorts messages by *category*, not by destination:

> `-q, --quiet`
>
> Suppress status printing (warnings & errors are still printed).

- Source: Blender's manual, https://docs.blender.org/manual/en/5.1/advanced/command_line/arguments.html
- Date read: 2026-07-30

This tells you Blender distinguishes "status" from "warnings & errors" and can be told to drop the first. It does not say the two go to different streams. A single stream with one category filtered out would satisfy every word of it.

### The one place a stream is named at all

**LABEL: observed** (for the lines) / **inferred** (for what a run does with them).

Across all four sources, exactly one names an output stream. It is in Cycles' own files, in the help text of a Cycles command-line argument:

```
    parser.add_argument("--cycles-print-stats",
                        help="Print rendering statistics to stderr",
                        action='store_true')
```

- Source: Cycles' own files, file `engine.py`, at the top level of the Cycles add-on folder, in the function `_configure_argument_parser()`, lines 13–15.
- Date read: 2026-07-30

What the lines establish, and where the step is. **Observed:** an argument named `--cycles-print-stats` exists in Cycles' own files, and the help text written next to it names a stream called stderr and names rendering statistics. That is the whole of what reading the file gives.

**Inferred, and I do not rely on it:** that anything actually arrives on stderr when a render is run with this argument. A help text says what an argument is for; it is not a record of what a render did. To get from one to the other a reader has to supply the step "the help text describes the behaviour accurately." That step is mine, not the file's.

Either way, this concerns rendering statistics and not missing files.

It is worth noting that the manual's own entry for this same argument names no stream:

> `--cycles-print-stats`
>
> Log statistics about render memory and time usage.

- Source: Blender's manual, https://docs.blender.org/manual/en/5.1/advanced/command_line/arguments.html
- Date read: 2026-07-30

So even where Cycles' own help text names a stream, the manual's entry for the same argument does not.

### What the sources say about missing files, none of it about streams

**LABEL: documented**, for each quote.

A missing linked blend-file, on load:

> If Blender cannot find a library while loading a blend-file, it will create placeholder data-blocks to replace missing linked ones. That way, references to the missing data are not lost, and by relocating the missing library, the lost data can be automatically restored.

- Source: Blender's manual, https://docs.blender.org/manual/en/5.1/files/linked_libraries/link_append.html
- Date read: 2026-07-30

The Python API confirms those placeholders are visible to a script:

> `is_missing`
>
> This data-block is a place-holder for missing linked data (i.e. it is [an override of] a linked data that could not be found anymore) (default False, readonly)

- Source: Blender's Python API documentation, https://docs.blender.org/api/5.1/bpy.types.ID.html
- Date read: 2026-07-30

Both describe what Blender *builds* when a file is missing. Neither says a single word is printed.

Checking for missing files by hand is documented as putting its answer in the interface, and no stream is named:

> Report Missing Files
>
> This option is useful to check if there are links to unpacked files that no longer exist. After selecting this option, a warning message will appear in the Info editor's header. If no warning is shown, there are no missing external files.

- Source: Blender's manual, https://docs.blender.org/manual/en/5.1/interface/window_system/topbar.html
- Date read: 2026-07-30

The Info editor's header is a region of Blender's own window. The manual gives no other destination for that warning.

### NOT COVERED

- The answer settles nothing about what text, if any, a render prints when a referenced file is missing. It only establishes that the sources do not say.
- It settles nothing about which stream carries which kind of message in general, missing files or otherwise. The one stream ever named in the sources is named for an unrelated purpose.
- It does not settle whether a missing file makes a render fail, succeed with a hole in it, or succeed unchanged. The linked-library quote shows placeholders being built on *load*; it says nothing about a subsequent render.
- It does not distinguish among kinds of missing file — image, movie, sound, cache, linked blend-file — because no source draws that distinction in terms of output.
- It does not settle the second reading of the question (a missing blend-file handed to the command line), for the same reason.

### Outside the interface

Streams only exist outside the interface, so this question is an outside-the-interface question throughout, and the sources are silent throughout.

Two things they do say bear on the difference between the two ways of running. First, that output only reaches a terminal at all if Blender was started from one: "When Blender is manually started from a terminal, Blender output is shown in the Console Window it is started from." Second, the manual's note that on a Microsoft Windows machine started normally the console is created and then hidden:

> When Blender is started on a Microsoft Windows operating system, the Console Window (called the Command Prompt) is first created as a separate window on the desktop. The main Blender window will also appear and the Console Window will then be toggled off. To display the console again, go to Window ‣ Toggle System Console.

- Source: Blender's manual, https://docs.blender.org/manual/en/5.1/advanced/command_line/launch/windows.html
- Date read: 2026-07-30

Both are about where output is *shown*. Neither divides it into streams, and neither mentions missing files. The one destination the sources give for a missing-file warning is the Info editor's header, which is inside Blender's own window. The sources do not say what becomes of that warning in a run driven from outside.


> **Brought up to date on 2026-08-26 by hand test.** Both streams were captured separately, in this version, on two machines, for five kinds of missing file, and the same files were then opened with a real window. See "Answered by hand test — question 13" at the end of this question's material.

### Operating system

Not stated, and here the silence is total — the sources do not describe the behaviour on any operating system, so there is nothing to hold or fail to hold elsewhere.

The one adjacent fact worth recording is the Windows console note quoted just above, which is explicitly marked as Microsoft Windows behaviour and has no stated equivalent for macOS or Linux. The manual's macOS page instead says you must run the executable inside the application bundle "to get all output printed to the terminal", and its Linux page simply shows Blender run from a terminal. All three are about where output is seen, not about streams.

---


> **Brought up to date on 2026-08-26 by hand test.** macOS and Linux were both measured in this version and agreed word for word on every missing-file run. Windows remains untested: no Windows machine has run it.

## Record of where I looked

Everything below was done with whole-corpus text searches over `Blender Sources/manual_5.1`, `Blender Sources/api_5.1`, `Blender Sources/release_notes_5.1` and `Blender Sources/cycles_5.1`, before opening any individual page.

### Terms searched, across all four sources

For question 12: `exit code`, `exit status`, `return code`, `exit-code`, `errorlevel`, `exit`, `non-zero`, `nonzero`, `zero disables`, `returns zero`, `0 for success`, `on failure`, `abort`, `terminate`, `kill`, `crash`, `signal`, `SIGINT`, `SIGTERM`, `interrupt`, `cancel the render`, `stop the render`, `errno`, `parent process`, `calling process`, `waitpid`, `posix`, `quit_blender`, `unknown argument`, `invalid argument`, `unrecognized`, `bad argument`, `render farm`, `batch`, `shell script`, `automation`, `subprocess`, `spawn`.

For question 13: `stderr`, `stdout`, `standard error`, `standard output`, `error stream`, `output stream`, `error output`, `console output`, `terminal output`, `text output`, `redirect`, `2>`, `missing file`, `missing`, `file not found`, `cannot read`, `could not open`, `unable to open`, `failed to load`, `unable to load`, `load failed`, `failed to open`, `error loading`, `cannot find`, `can't find`, `could not find`, `does not exist`, `doesn't exist`, `magenta`, `pink`, `is_missing`, `report`, `console`, `print`.

### What each search turned up

- `exit code`, `exit status`, `return code`, `errorlevel` — one file in the whole manual matched, and it matched on `--python-exit-code` alone. Nothing at all in the API documentation, the release notes, or Cycles' files.
- `stderr` / `stdout` — nothing in the manual, the API documentation or the release notes. Two matches in Cycles' files: the `--cycles-print-stats` help text quoted above, and a comment in `source/util/defines.h` line 133 about a Metal logging environment variable, which is a note to a developer building Cycles and says nothing about what a render prints.
- `standard output` — one match in the whole manual, on the compositor's Color Balance node page, describing that node's output socket. Nothing to do with streams.
- `sys.exit` — one match in all of Cycles' Python files, the `--cycles-device` case quoted above.
- `signal`, `SIGINT`, `SIGTERM`, `interrupt`, `errno`, `parent process`, `calling process`, `waitpid`, `posix` — no relevant match anywhere. The matches for `signal` were about display signals and about probabilistic path termination; `interrupt` matched once, in a passage about a person wanting to stop rendering to use their computer, which names no status and no message.
- `missing` across the manual — 40-odd matches, every one read. They are about missing caustics, missing shadow updates, missing data layers, missing NLA strips, missing attachment data, and so on. The four that concern a missing *file* are the four quoted or discussed above (Report Missing Files, Find Missing Files, broken libraries, Show Missing Media). None mentions a stream.

### Pages opened in full and ruled out

- The manual's whole command-line section: the arguments page, the render-from-the-command-line page, the section index, and all three launching pages (Linux, macOS, Windows). This is the only part of the manual that discusses running Blender from outside, and it is where any statement about ending status or streams would have to live. It does not contain one.
- The manual's extension command-line arguments page. Every `exit` in it is argparse's stock "show this help message and exit". No status values.
- The manual's troubleshooting section: the startup page (read in full — its "Common Startup Messages" list has exactly two entries, `found bundled python:` and `Read prefs:`, neither about missing scene files or streams), the Python errors page (read in full — about mismatched precompiled libraries, not about streams), and the crash page.
- The manual's scripting security page, which covers batch rendering from the command line and mentions no status.
- The manual's Info editor page, read in full. It describes the editor that logs "errors, warnings, and informational messages" inside Blender's window; it never connects that log to a stream.
- The manual's video sequencer preview sidebar page. It carries the "Show Missing Media" setting quoted in the record above — "Render missing images/movies with a solid magenta color. When disabled, missing content will render fully transparent." I did **not** use this as an answer to question 13. It sits in the View panel of the Preview region's sidebar, so on its face it is a preview-display setting, and in any case it describes coloured pixels, not text on a stream.
- The API page for `bpy.utils`, read in full around `register_cli_command`, including the two worked example scripts.
- The API page for `bpy.types.Operator`, checked for any statement about where `report()` messages go. The `report()` entry itself gives only a type and a message and names no destination. The page's worked example does name one, in a comment: "Rather than printing, use the report function, / this way the message appears in the header." That is a place in Blender's own window, not a stream, so it does not bear on the question.

  > **Corrected on 2026-08-26 by the citation check.** This entry said the page "says only 'Rather than printing, use the report function' — no destination". The next line of the same comment, `api_5.1/bpy.types.Operator.txt` line 134, reads "this way the message appears in the header", which is a destination. The entry now says what the page says.
- The API page for `bpy.ops.file`, checked for `find_missing_files` and `report_missing_files`. Both are one-line descriptions ("Try to find missing external files", "Report all missing external files") with no statement of output.
- All 21 release-note files for 5.1, searched as a set. The index, the Core notes and the Rendering notes were opened. The only command-line change noted for 5.1 anywhere is in Core: the Python user-site-directory is no longer loaded by default, with `--python-use-user-env` to restore it, plus a new `--no-window-frame` argument on Linux. Nothing about ending status, nothing about streams, nothing about missing files.
- Cycles' own files. The Python side is nine files; `engine.py` is the only one with any command-line handling and the only one with any `exit`. The `source` folder holds only the render kernel and its maths and type headers — there is no file in it that loads an image from disk, parses a command line, or ends a run, so no message about a missing file and no status can be found there.

### Pictures opened

- `_images/advanced_command-line_launch_linux_console.png`, the screenshot on https://docs.blender.org/manual/en/5.1/advanced/command_line/launch/linux.html, captioned "Starting Blender from a Linux terminal window." **LABEL: documented**, for what is visible: the terminal shows `% blender` followed by seven lines of text, all of them from the sound system — "Cannot connect to server socket err = No such file or directory", "jack server is not running or cannot be started", two `JackShmReadWritePtr` lines, and two lines beginning `AL lib: (WW)`. Everything appears in one window in one colour. Nothing in the picture shows a stream, a status, or a missing scene file, and nothing in it says which stream any of those lines came from. Ruled out for both questions.

  > **Corrected on 2026-08-26 by the citation check.** The count read "eight lines". The picture was opened again and holds seven, listed above. Nothing else in the description changed.
- `_images/advanced_command-line_launch_windows_console.png`, the screenshot on https://docs.blender.org/manual/en/5.1/advanced/command_line/launch/windows.html, captioned "Blender's Console Window on Microsoft Windows." **LABEL: documented**, for what is visible: a Command Prompt showing `c:\blender>blender`, then lines reading `Read new prefs:`, `found bundled python:` and `read blend:`, then a blank line, then `Blender quit`, then the prompt again. It shows a run ending and the words printed at the end. It does not show the status handed back. Ruled out for question 12, and it shows no missing-file case, so ruled out for question 13.

### What I did not do

I did not use the web, and did not need to. I ran nothing: the folder I was given holds documentation and Cycles' source files, not a copy of Blender that could be started, so there was nothing to exercise and no observation of behaviour to report. Every "observed" label above refers to lines read in Cycles' own files, as the instructions define that word.

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

2026-08-25 — Claude Opus 5 (Claude Code subagent) — Reports, Blender 5.1.

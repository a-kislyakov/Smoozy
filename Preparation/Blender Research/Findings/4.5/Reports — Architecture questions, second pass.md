# Reports — Architecture questions, second pass (Blender 4.5)

**Questions answered here: 12 and 13 only.** The rest of the sheet was read, so that I knew what these two are not asking, but is not answered here.

**Sources consulted, and on what date.** All four sources for 4.5, all read on **2026-07-30** (the snapshot date I was told to use as my date read):

- Blender 4.5 manual — `Blender Sources/manual_4.5`, published at https://docs.blender.org/manual/en/4.5/ (including two pictures from its `_images` folder, named in the record below)
- Blender 4.5 Python API documentation — `Blender Sources/api_4.5`, published at https://docs.blender.org/api/4.5/
- Blender 4.5 release notes — `Blender Sources/release_notes_4.5`, published at https://developer.blender.org/docs/release_notes/4.5/
- Cycles' own files — `Blender Sources/cycles_4.5`

I read only the question sheet and files inside Blender Sources/. I did not open any other part of this project.

I did not use the web. I started no helper agents, so the declaration above covers all of the work reported here.

One thing to be straight about, because the declaration should be worth something. While checking that I had saved this file under exactly the name I was given, I ran a listing of the folder I was saving into, and the names of two neighbouring documents scrolled past. I opened neither, and I read nothing of their contents — only two file names. I mention it because the instructions told me not to list that folder at all, and a declaration that hides a small slip is not worth reading.

---

## 12. What ending status does a run of Blender give back to whatever started it, for each of the ways a run can end — finishing normally, being interrupted, and being refused because something it was handed cannot be used?

**LABEL: not in the sources**, for the question as a whole. Two of the three endings are not covered at all. Parts of the third have partial coverage, and those parts are labelled separately below.

### The short answer

Nowhere in the 4.5 manual, the 4.5 Python API documentation, the 4.5 release notes, or Cycles' own files is there a statement of what number a finished run of Blender hands back to the program that started it. There is no table of ending statuses, no "returns zero on success", and no mention of the words that such a thing is normally called (see the record of where I looked, below). What the sources do have is four narrow places where a status is set or implied, and none of them is the general answer.

### (a) Finishing normally

**LABEL: not in the sources.**

Nothing states what a run that completes hands back.

The closest the manual comes is a picture, not a sentence. The page on launching Blender from the command line on Windows carries a screenshot of a finished run:

> `[image: _images/advanced_command-line_launch_windows_console.png]`
> — Blender 4.5 manual, https://docs.blender.org/manual/en/4.5/advanced/command_line/launch/windows.html, read 2026-07-30

I opened that picture (`Blender Sources/manual_4.5/_images/advanced_command-line_launch_windows_console.png`). It shows a Command Prompt window in which Blender was started, printed four lines, printed `Blender quit`, and returned to the `c:\blender>` prompt. **What is visible is that Blender prints the words `Blender quit` on a normal end and the prompt comes back. Nothing about an ending status is visible** — the screenshot does not show the status being checked or displayed. Label for what I take from the picture: **documented**, and it settles only that the words `Blender quit` appear.

### (b) Being interrupted

**LABEL: not in the sources.**

Nothing states what a run hands back when it is stopped part-way. I found no mention anywhere in the four sources of interruption signals, of stopping a run from outside it, or of what the status is afterwards. The only "stopping" the sources describe is a person clicking or pressing a key inside the interface:

> "The task can be aborted by clicking the cancel button ()."
> — Blender 4.5 manual, https://docs.blender.org/manual/en/4.5/interface/window_system/status_bar.html, read 2026-07-30

> "As with a normal render, you can abort it with Esc."
> — Blender 4.5 manual, https://docs.blender.org/manual/en/4.5/editors/3dview/viewport_render.html, read 2026-07-30

Neither says anything about a status handed back, and both describe a person at the interface, which the question's own framing puts to one side.

There are two command-line switches that name handlers for abnormal ends, but they only turn the handlers off; they say nothing about a status:

> "--disable-crash-handler
>
> Disable the crash handler.
>
> --disable-abort-handler
>
> Disable the abort handler."
> — Blender 4.5 manual, https://docs.blender.org/manual/en/4.5/advanced/command_line/arguments.html, read 2026-07-30

And one that ends a run early on an internal fault, again with no status stated:

> "--debug-exit-on-error
>
> Immediately exit when internal errors are detected."
> — Blender 4.5 manual, https://docs.blender.org/manual/en/4.5/advanced/command_line/arguments.html, read 2026-07-30

### (c) Being refused because something it was handed cannot be used

This is the only ending with any coverage, and the coverage is in pieces. No piece is a general rule.

**(c1) A Python script raising an error — a status you choose yourself. LABEL: documented.**

> "--python-exit-code <code>
>
> Set the exit-code in [0..255] to exit if a Python exception is raised
> (only for scripts executed from the command line), zero disables."
> — Blender 4.5 manual, https://docs.blender.org/manual/en/4.5/advanced/command_line/arguments.html, read 2026-07-30

This is the only place in any of the four sources that uses the words "exit-code" at all. Read plainly, it says: Blender does not, by itself, pick a status when a script it was handed fails; you tell it which one to use, it must be between 0 and 255, it applies only to scripts given on the command line, and setting it to zero turns the behaviour off. That "zero disables" is the strongest thing the sources say about the default: the feature is off unless you switch it on.

**(c2) A rendering device name Blender cannot use. LABEL: observed (the file), then inferred (the consequence).**

Cycles reads its own command-line options after `--`. If it is handed a device it cannot honour, it stops the whole program:

```
    if args.cycles_device:
        import _cycles
        if not _cycles.set_device_override(args.cycles_device):
            sys.exit(1)
```
— `Blender Sources/cycles_4.5/engine.py`, lines 37–41, inside the function `_parse_command_line()`. Read 2026-07-30.

The option this belongs to is documented in the manual:

> "--cycles-device <device>
>
> Set the device used for rendering.
> Valid options are: CPU CUDA OPTIX HIP ONEAPI METAL.
>
> Append +CPU to a GPU device to render on both CPU and GPU."
> — Blender 4.5 manual, https://docs.blender.org/manual/en/4.5/advanced/command_line/arguments.html, read 2026-07-30

**Observed:** the file contains the line `sys.exit(1)` on the branch taken when `_cycles.set_device_override(...)` does not succeed.

**Inferred, with the step written out:** that the run therefore hands back the number 1. The step a reader has to supply is that Python's `sys.exit(1)` ends the program with the status 1. The sources given to me do not state that; I am supplying it. What the sources *do* state is that such a call ends Blender:

> "Some Python modules will call sys.exit() themselves when an error occurs,
> while not common behavior this is something to watch out for because it may seem
> as if Blender is crashing since sys.exit() will close Blender immediately."
> — Blender 4.5 Python API documentation, https://docs.blender.org/api/4.5/info_gotchas_crashes.html, read 2026-07-30

So "it ends the run" is documented; "the number handed back is 1" is inferred.

**(c3) A custom sub-command. LABEL: documented for what the callback returns; not in the sources for whether that becomes the run's status.**

Blender's `-c` / `--command` argument runs a command registered from Python, and the documentation states the convention for that command's own return value:

> "execute (callable) – Callback, taking a single list of strings and returns an int.
> The arguments are built from all command-line arguments following the command id.
> The return value should be 0 for success, 1 on failure (specific error codes from the os module can also be used)."
> — Blender 4.5 Python API documentation, https://docs.blender.org/api/4.5/bpy.utils.html (entry `bpy.utils.register_cli_command`), read 2026-07-30

This is the only place in the four sources that pairs "0 for success" with "1 on failure". Note carefully what it is and is not: it describes what the *Python function* must give back to Blender. It does not say that Blender then passes that number on to the program that started it. I could not find any sentence that closes that gap, so I am not claiming it.

**(c4) An argument Blender does not recognise. LABEL: documented that the run ends; not in the sources for the status.**

> "Arguments must be separated by white space, eg:
>
> blender -ba test.blend
>
> …will exit since -ba is an unknown argument."
> — Blender 4.5 manual, https://docs.blender.org/manual/en/4.5/advanced/command_line/arguments.html, read 2026-07-30

Blender ends the run. What it hands back is not stated.

### Was the question ambiguous?

Mildly, in one place: "being refused because something it was handed cannot be used" could mean a bad command-line argument, an unreadable blend-file, an unusable render device, or a script that fails. I read it as covering all of them and reported every one I could find anything on. The reading I did **not** take is that it means an operator inside Blender returning `CANCELLED`, because the question rules that out itself — it asks for "the one a program that started Blender receives, not anything Blender prints about itself", and an operator's return value never leaves Blender.

### Not covered

- No source states the status for a normal finish, so the common assumption that it is zero is not supported here — it is simply absent.
- No source states the status after any kind of interruption from outside.
- No source states the status after an unreadable or missing blend-file, an unknown argument, or an unusable output path.
- (c2) rests on one line of an add-on file plus a step I supplied about what `sys.exit(1)` does; it applies only to the `--cycles-device` option, not to render failures generally.
- (c3) settles what a registered sub-command must return to Blender, not what Blender returns onwards.
- (c1) settles a status you set yourself for one cause (a Python exception from a command-line script). It says nothing about any other cause.
- Nothing here settles whether the status differs between the ways a render is started (`-f`, `-a`, a script calling a render).

### Outside the interface

The whole question only has meaning outside the interface — a status handed back is something only a starting program sees. Everything cited above is command-line material: `--python-exit-code` is described as applying "only for scripts executed from the command line"; `--cycles-device` is a command-line option parsed after `--`; `-c` / `--command` is a command-line argument and the manual says it "implies --background mode". So what little the sources say is about the outside-the-interface case already. Where it differs from inside the interface: the sources do not say, because they never describe an ending status for a run driven by a person clicking, and a person clicking has no program waiting for one.


> **Brought up to date on 2026-08-26 by hand test.** The status a run hands back was measured on two machines in this version, and all of it was measured from outside Blender's own window, which is the only place a status exists. See "Answered by hand test — question 12" at the end of this question's material.

### Operating system

The sources do not say. None of the passages above is marked as belonging to one operating system. For contrast, the same manual page does mark other arguments as platform-specific — for example `/?` is marked "(Windows only)" and `--register` is marked "(Windows & Linux only)" — and `--python-exit-code`, `--cycles-device`, `-c` / `--command`, `--disable-crash-handler` and `--disable-abort-handler` carry no such mark. That the arguments exist everywhere is therefore reasonable to read from the page; that the *statuses* are the same everywhere is not stated anywhere, and I am not claiming it. The one number I found, the `1` in Cycles' `engine.py`, is in a plain Python file with no platform condition around it.


> **Brought up to date on 2026-08-26 by hand test.** macOS and Linux were both measured in this version and agree almost everywhere; the places they do not are named in "Answered by hand test — question 12" below. Windows remains untested: no Windows machine has run it. **Untried rather than untestable: a Windows machine can be rented, and every one this project has read reported a Server edition — see the equipment section of `Preparation/Blender Research/Method.md`.**

### Record of where I looked, for question 12

Searched, case-insensitively, across all four 4.5 sources at once (`manual_4.5`, `api_4.5`, `release_notes_4.5`, `cycles_4.5`):

- `exit code`, `exit status`, `return code`, `returncode`, `exit value`, `status code`, `errorlevel`, `exit-code` — one hit only, `--python-exit-code` in the manual's command line arguments page. `ERRORLEVEL`: no hits.
- `exit` as a whole word (and `exits`, `exited`, `exiting`) — every hit reviewed. Most are "exit Edit Mode" and similar; the useful ones are all on the command line arguments page and are quoted above.
- `sigint`, `sigterm`, `sigkill`, `signal`, `interrupt`, `abort`, `ctrl-c`, `ctrl+c`, `terminate`, `termination`, `kill` — nothing about a status. `signal` hits are about video signals and light path termination; `abort` hits are about cancelling an operation in the interface.
- `non-zero` / `nonzero`, `returns 0`, `returns 1`, `return value` — the only relevant hit is the `bpy.utils.register_cli_command` line quoted above.
- `shell script`, `batch file`, `wrapper script`, `render farm`, `subprocess`, `process id` — no hits bearing on a status. (`render farm` finds only the "Primitive Render Farm" hint about placeholder frames, and a metadata note.)
- `sys.exit`, `SystemExit`, `os._exit` — three hits total: the API's crash gotchas page, and the one line in `cycles_4.5/engine.py`.
- `quit_blender`, `quit` — only the operator `bpy.ops.wm.quit_blender()`, which is documented with no return information.

Pages read in full or in the relevant part, and what I ruled out:

- https://docs.blender.org/manual/en/4.5/advanced/command_line/arguments.html — read end to end (all 628 lines of the text file). This is the page that would carry an ending-status table if one existed; it does not have one.
- https://docs.blender.org/manual/en/4.5/advanced/command_line/index.html — read in full. Describes the Console Window and says "If Blender exits unexpectedly, the messages may indicate the cause or error", but gives no status.
- https://docs.blender.org/manual/en/4.5/advanced/command_line/render.html — read in full. Gives example commands for single frames and animations. No statuses.
- https://docs.blender.org/manual/en/4.5/advanced/command_line/launch/windows.html, .../linux.html, .../macos.html — read; they cover how to start Blender, not how it ends. I opened both console screenshots (`advanced_command-line_launch_windows_console.png`, `advanced_command-line_launch_linux_console.png`); neither shows a status being read.
- https://docs.blender.org/manual/en/4.5/advanced/command_line/extension_arguments.html — read; it is the help text for the extension sub-commands, and every entry ends "show this help message and exit" with no status.
- https://docs.blender.org/manual/en/4.5/advanced/deploying_blender.html — read in full, on the chance that a page written for studios would mention automation statuses. It does not.
- https://docs.blender.org/manual/en/4.5/troubleshooting/startup.html and the rest of the troubleshooting section — read; about messages, not statuses.
- https://docs.blender.org/api/4.5/bpy.utils.html — read the `register_cli_command` / `unregister_cli_command` entries and both worked examples in full.
- https://docs.blender.org/api/4.5/info_gotchas_crashes.html — read the `sys.exit` section in full.
- https://docs.blender.org/api/4.5/bpy.app.html — checked for anything about how a run ends; it has `background` and `use_userpref_skip_save_on_exit`, neither of which bears on a status.
- https://docs.blender.org/api/4.5/info_tips_and_tricks.html — read the "Use Blender without it's User Interface" section, which is the API's own advice on driving Blender from a command line. It suggests checking results by writing images, saving files or printing text; it never mentions a status.
- Release notes for 4.5: searched all nineteen files for `exit`, `command line`, `argument`, `background mode`, `quit`, `abort`, `cancel`, `crash`. Nothing about ending statuses. The only command-line news is a new `--profile-gpu` argument (https://developer.blender.org/docs/release_notes/4.5/eevee/).
- Cycles' own files: the 4.5 folder holds the add-on's Python (`__init__.py`, `engine.py`, `operators.py`, `osl.py`, `properties.py`, `ui.py`, `presets.py`, `camera.py`, `version_update.py`), the OSL shader sources, and the kernel headers. There is no standalone Cycles program and no session or error-handling source in it, so there is nowhere in these files for a general table of statuses to live. The single `sys.exit` is the one quoted.

---


## Answered by hand test — question 12, 2026-08-26

**LABEL: observed.** Everything in this section is what two real machines gave back when the question was put to them directly. None of it comes from documentation, and none of it was known before the runs.

### What was done, and on what

Two machines, and their results are never merged.

- **The Creator's Mac** — a MacBook Pro with an Apple M1 Pro processor, macOS 26.5.2, Apple's own graphics and no second graphics card. Blender 4.5.12 LTS, build hash `84afd5f785f7`, built 2026-07-21 01:41:35, run from `/Applications/Blender 4.5.12.app/Contents/MacOS/Blender`.
- **The rented Linux machine, in Norway** — a Docker container holding half its host, running Ubuntu 22.04.5 LTS on kernel `6.8.0-136-generic`, with one NVIDIA GeForce RTX 3060 of 12,288 MiB on driver 595.84, about fifteen cores of an Intel Xeon E5-2697A v4, and a display server of its own on `DISPLAY=:1`. Its viewport is drawn by its processor, not by the card, and it is used as `root`. Blender 4.5.12 LTS, build hash `84afd5f785f7`, built 2026-07-21 01:31:22, run from `/opt/blender/4.5.12/blender`.

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

**Two presses, three seconds apart.** 4.5 ended on signal 6, `SIGABRT`, which a shell reports as **134** — on both machines, four runs each. The terminal showed:

```
^C
Sent an internal break event. Press ^C again to kill Blender
^C
Blender killed
libc++abi: terminating
```

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

**The manual's own unknown-argument example, `blender -ba good.blend`.** **Blender 4.5 does end.** Status **1** on both machines, twice each. Standard output:

```
Blender 4.5.12 LTS (hash 84afd5f785f7 built 2026-07-21 01:41:35)
Error: Cannot read file "<work folder>/-ba": No such file or directory

Blender quit
```

Standard error, two lines:

```

unknown argument, loading as file: -ba
Error: argument has no '.blend' file extension, not using as new file, exiting! <work folder>/-ba
```

The second of those lines is the one that ends the run, and it is the line the newer versions no longer print.

**An argument Blender has never heard of, after a valid one** — `blender -b good.blend --no-such-flag -o <work folder>/f_#### -f 1`. Status **1** on both machines, nothing written. Blender says `unknown argument, loading as file: --no-such-flag` on standard error, then fails to read a file of that name, and stops because the file it was told to open cannot be read.

**An engine name that does not exist**, `blender -b good.blend -E NOSUCHENGINE -o <work folder>/x_#### -f 1`. **In 4.5 the two machines part company, and neither gives a plain number.**

On the Creator's Mac the run ended on signal 6, `SIGABRT`, which a shell reports as **134**. Three runs out of three, and a fourth with five minutes allowed. Standard output held two lines, standard error three:

```
Error: engine not found 'NOSUCHENGINE'
libc++abi: terminating
```

On the rented Linux machine the same command printed the same `Error: engine not found 'NOSUCHENGINE'` on standard error and then **never ended at all** — still running after ninety seconds on three runs out of three, and still running after three hundred seconds on a fourth. Each time the test harness had to kill it with `SIGKILL`.

The two machines do not agree, and neither agrees with Blender 5.0, 5.1 and 5.2, which give a plain 1.

**A render device.** Five runs, each as `blender -b good.blend -o <work folder>/x_#### -f 1 -- --cycles-device <name>`:

| Name given | The Creator's Mac | The rented Linux machine |
| --- | --- | --- |
| `NOTADEVICE` | **1** | **1** |
| `OPTIX` | **1** | **0**, and it rendered |
| `CUDA` | **1** | **0**, and it rendered |
| `METAL` | **0**, and it rendered | **1** |
| `HIP` | **1** | **1** |

So a name that is not on Cycles' list and a name that is on it but has no hardware behind it **both give 1**, and which names have hardware behind them is what separates the two machines. In 4.5 the refusal of a name that is not on Cycles' list lands on **standard error**:

```

Error: NOTADEVICE is not a valid Cycles device.
```

and the refusal of a name that is on the list but has no hardware behind it lands on **standard output**:

```
Blender 4.5.12 LTS (hash 84afd5f785f7 built 2026-07-21 01:41:35)
Read blend: "<scene folder>/good.blend"
Fra:1 Mem:8.18M (Peak 9.10M) | Time:00:00.00 | Mem:0.00M, Peak:0.00M | Scene | Initializing
Error: Found no Cycles device of the specified type

Blender quit
```

So in 4.5 the two kinds of refusal give the same number and arrive on different streams.

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

**Cycles' own `maketx` command.** **There is no such command in 4.5.** Both runs gave **1**, with nothing on standard output and one line on standard error:

```
Unrecognized command: "maketx"
```

`blender --command help` in 4.5 lists one command:

```
Blender Command Listing:
	extension
```

**A scene with no camera**, `blender -b nocamera.blend -o <work folder>/nc_#### -f 1`. Status **1** on both machines, nothing written:

```
Blender 4.5.12 LTS (hash 84afd5f785f7 built 2026-07-21 01:41:35)
Read blend: "<scene folder>/nocamera.blend"
Error: Cannot render, no camera

Blender quit
```

**An output folder that cannot be written to** — made with `mkdir ro` and `chmod 500 ro`. On the Creator's Mac, status **1** in 4.5, with nothing written. The complaint is split across both streams — this is the only run in the whole round where 4.5 writes to standard error about a file. Standard output carried:

```
Error: Render error (Permission denied) cannot save: '<work folder>/ro/p_0001.png'
Time: 00:00.04
```

and standard error carried one line:

```
<work folder>/ro/p_0001.png: Permission denied
```

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

In 4.5 there is no such setting. The Python expression came back with `NO save_output IN THIS BUILD: 'RenderSettings' object has no attribute 'save_output'`, the render went ahead, and the picture was written. Status 0.

**The reversed frame range.** In 4.5 the run announces nothing about the range at all. Standard output:

```
Blender 4.5.12 LTS (hash 84afd5f785f7 built 2026-07-21 01:41:35)
Read blend: "<scene folder>/good.blend"

Blender quit
```

### The two standing lines, brought up to date

**Outside the interface.** Settled by measurement, for this version, on both machines. Every number above was got from a run started and watched by another program, with no window, which is the only setting in which an ending status exists at all. The window runs described under question 13 show the other side of the same coin: a render invoked from inside a window hands its outcome back to the Python that called it — `RuntimeError('Error: No camera found in scene "Scene"\n')` — and the *program's* status has nothing to do with it, because the program is still running.

**Operating system.** Settled for macOS and for Linux, and only for those two. Every result above was got on both, and **they agree everywhere except in two places**, each named where it occurs: an engine name that does not exist, where the Mac ends on `SIGABRT` and the rented Linux machine never ends at all; and an unwritable output folder, where the Linux machine is used as `root` and is therefore not stopped by the folder's permissions. Everything else — the clean finishes, the interrupts, the signals, the refused files and arguments, the devices, the Python failures, the registered commands and the whole of the trap — came back the same on both. **Windows is untested and stays untested**: no Windows machine has run it, so what `echo %ERRORLEVEL%` or `$LASTEXITCODE` would report, and whether Ctrl-Break behaves as Ctrl-C does, are not settled by anything here.

### What these runs do not settle

- Anything on Microsoft Windows.
- What a working HIP or oneAPI device gives back. Both were refused for want of an AMD or an Intel card, so the *refusal* is settled and the success is not.
- What Blender loaded as a Python module hands back. Neither machine has such a build.
- Whether an unwritable output folder gives 1 on Linux, which needs an account that is not `root`.
- Why Blender 4.5 behaves as it does when handed an engine name that does not exist, and why the two machines part company there. What happened is recorded; nothing here explains it.

## 13. When a file the scene points at is not there, what does a render put on each of its output streams?

**LABEL: not in the sources**, for the question as asked — that is, for which of a run's output streams carries what.

### Was the question ambiguous?

Yes, and it matters here. "Output streams" has two readings:

- **The reading I took:** the streams a started program reads back from a run — the ordinary output stream and the error stream. This is the reading the sheet itself points to, since it says question 13 "asks which stream carries what, under one named condition", and since question 4 has already taken "what Blender writes out as it works, what is that stream called".
- **The other reading:** the picture a render produces — what appears in the image where the missing file should have been. I found one documented fact under that reading and report it at the end, marked clearly, rather than silently choosing.

### The answer under the reading I took

**The four sources for 4.5 almost never name a run's output streams at all.** Searching all four for `stderr`, `stdout`, `standard error`, `standard output`, `std::cerr` and `std::cout` returns exactly two hits in the whole set, and only one of them is about a stream:

> `                        help="Print rendering statistics to stderr",`
> — `Blender Sources/cycles_4.5/engine.py`, line 14, inside `_configure_argument_parser()`. Read 2026-07-30.

**LABEL for that line: observed.** It is the help text of the `--cycles-print-stats` option. It tells us that Blender's sources do distinguish the two streams somewhere, and that render statistics are deliberately put on the error stream. It says nothing at all about missing files. (The other hit is the phrase "Standard output image" on a compositor node page, which is about an image socket, not a stream.)

So: there is no statement anywhere in the 4.5 manual, the 4.5 API documentation, the 4.5 release notes or Cycles' files that says which stream a missing-file message goes to, or what a render puts on the other stream while that is happening. That is the answer.

### What the sources do say about missing files, none of it stream-specific

**LABEL: documented**, for each quotation; **not in the sources** for the stream each lands on.

**Blender has a command to look for missing files, and it answers inside the interface, not on a stream.**

> "Report Missing Files
>
> This option is useful to check if there are links to unpacked files that no longer exist.
> After selecting this option, a warning message will appear in the Info editor's header.
> If no warning is shown, there are no missing external files."
> — Blender 4.5 manual, https://docs.blender.org/manual/en/4.5/interface/window_system/topbar.html, read 2026-07-30

That names the destination — the Info editor's header — which is a place in Blender's window, not a stream. The matching Python entry is equally silent about streams:

> "bpy.ops.file.report_missing_files()
>
> Report all missing external files"
> — Blender 4.5 Python API documentation, https://docs.blender.org/api/4.5/bpy.ops.file.html, read 2026-07-30

**A missing linked blend-file is handled by substitution, with no message described.**

> "Broken Libraries
>
> If Blender cannot find a library while loading a blend-file,
> it will create placeholder data-blocks to replace missing linked ones.
> That way, references to the missing data are not lost, and by relocating the missing library,
> the lost data can be automatically restored."
> — Blender 4.5 manual, https://docs.blender.org/manual/en/4.5/files/linked_libraries/link_append.html, read 2026-07-30

This is the clearest statement in the sources of what Blender *does* when a file the scene points at is not there — it substitutes placeholders and carries on. It does not say that anything is printed, on any stream.

**Where messages generally go, when the sources name a place at all, it is "the console" or the Info editor — never a named stream.**

> "The Console Window, also called a Terminal, is an operating system text window that displays
> messages about Blender's operations, status, and internal errors."
> — Blender 4.5 manual, https://docs.blender.org/manual/en/4.5/advanced/command_line/index.html, read 2026-07-30

> "The Info editor logs the executed operators as well as errors, warnings,
> and informational messages."
> — Blender 4.5 manual, https://docs.blender.org/manual/en/4.5/editors/info_editor.html, read 2026-07-30

> "Compilation errors will be shown in the system console."
> — Blender 4.5 manual, https://docs.blender.org/manual/en/4.5/render/cycles/osl/index.html, read 2026-07-30

A console or terminal shows both streams mixed together, so none of these three sentences separates them. I checked this against the manual's own screenshot of a Linux terminal running Blender (`Blender Sources/manual_4.5/_images/advanced_command-line_launch_linux_console.png`, on https://docs.blender.org/manual/en/4.5/advanced/command_line/launch/linux.html): after the `% blender` line it shows seven lines, and every one of them is a sound-system failure message — three from the JACK server, two `JackShmReadWritePtr` lines, and two beginning `AL lib: (WW)`. Nothing marks which stream any line came from. **Documented, and it settles only that a program Blender depends on can write into the same window; none of Blender's own output is visible in the picture, so the picture does not show the two kinds together.**

> **Corrected on 2026-08-26 by the citation check.** This paragraph said the picture showed "sound-system failure messages and Blender's own output interleaved in one window", and settled "that they appear together in one window". The picture was opened again: it holds seven lines and all seven are from the sound system, with none of Blender's own output among them. The description and what it settles are narrowed to what is actually visible.

**The nearest the manual comes to separating kinds of output is one switch, and it separates kinds, not streams.**

> "-q, --quiet
>
> Suppress status printing (warnings & errors are still printed)."
> — Blender 4.5 manual, https://docs.blender.org/manual/en/4.5/advanced/command_line/arguments.html, read 2026-07-30

**LABEL: documented.** Blender's output has at least two classes — "status printing" on the one hand, "warnings & errors" on the other — and one can be silenced while the other cannot. Note what this does *not* say: it does not say the two classes go to different streams. Anyone reading a stream separation into this sentence would be supplying it themselves.

There is also a switch that diverts logging to a file, which is a third destination again:

> "--log-file <filepath>
>
> Set a file to output the log to."
> — Blender 4.5 manual, https://docs.blender.org/manual/en/4.5/advanced/command_line/arguments.html, read 2026-07-30

**What Cycles' own files show for one kind of missing file.** The only place in Cycles' 4.5 files where a file the scene points at can be absent and something is said about it is the OSL script node. The messages there are sent through Blender's operator reporting, not to a stream:

```
            report({'ERROR'}, tip_("OSL query failed to open %s") % oso_path)
    else:
        report({'ERROR'}, "OSL script compilation failed, see console for errors")
```
— `Blender Sources/cycles_4.5/osl.py`, lines 282 and 284, inside `update_script_node()`. Read 2026-07-30.

**LABEL: observed.** The exact words of two messages, and that they are raised at the `ERROR` level through `report`. The file's own second message says the detail is "in the console" — which stream that console text is on is not stated in any file I was given. There is also a nearby line that prints a Python traceback:

```
            import traceback
            traceback.print_exc()
```
— `Blender Sources/cycles_4.5/osl.py`, lines 232–233, inside `update_internal_script()`. Read 2026-07-30.

> **Corrected on 2026-08-26 by the citation check.** The address read "lines 227–228". The two quoted lines are in that file and in that function, but at lines 232–233; lines 227–228 hold `bytecode = pathlib.Path(oso_path).read_text()` and `md5 = hashlib.md5(usedforsecurity=False)`. Only the line numbers changed.

**LABEL: observed** for the presence of that call. I am deliberately not saying which stream it lands on: that would rest on knowledge of Python's own behaviour, which is outside the sources I was given.

Note also that this OSL path runs when the script node is updated, not necessarily during a render, so it is a weak fit for "what a render puts out".

### Under the other reading of "output streams" — the picture

**LABEL: documented**, and it covers only the video sequencer.

> "Show Missing Media
>
> Render missing images/movies with a solid magenta color.
> When disabled, missing content will render fully transparent."
> — Blender 4.5 manual, https://docs.blender.org/manual/en/4.5/editors/video_sequencer/preview/sidebar.html, read 2026-07-30

The matching Python entry gives its default:

> "show_missing_media
>
> Render missing images/movies with a solid magenta color
>
> Type:
>
> boolean, default False"
> — Blender 4.5 Python API documentation, https://docs.blender.org/api/4.5/bpy.types.SequenceEditor.html, read 2026-07-30

So, for video sequencer strips whose media is absent: off by default, meaning the missing content comes out fully transparent; switched on, it comes out solid magenta. The same manual page adds, for the interface rather than the picture: "Strips with missing content will be displayed as red in the timeline."

I found no equivalent statement anywhere for a missing image texture in a material, a missing font, a missing sound, a missing cache, or a missing linked blend-file — only the placeholder-data-block sentence quoted earlier.

### Not covered

- Which stream — the ordinary one or the error one — carries a missing-file message. Not settled, in any of the four sources.
- What the *other* stream contains at the same time. Not settled.
- Whether a missing file changes what the render writes to its image file, or whether the frame is written at all. Not settled.
- Whether the answer differs by the kind of missing file (image texture, linked blend-file, font, sound, cache, OSL script). The sources describe the linked-blend-file case and the sequencer-media case and are silent on the others.
- Whether a missing file stops the render or lets it continue. Only the linked-library sentence bears on this, and only for linked blend-files.
- The exact wording of any missing-file message, apart from the two OSL ones quoted from Cycles' own file.
- Whether `--quiet` would suppress a missing-file message. It depends on whether such a message counts as "status printing" or as a "warning", and the sources do not classify it.

### Outside the interface

Partly, and the difference is real. The one destination the manual actually names for the missing-file check — "a warning message will appear in the Info editor's header" — is a place inside Blender's window, so it does not exist at all when Blender is driven from outside. The manual does say that in general "When Blender is manually started from a terminal, Blender output is shown in the Console Window it is started from" (https://docs.blender.org/manual/en/4.5/advanced/command_line/index.html, read 2026-07-30), so output does reach the starting program. But nothing in the sources says what a missing file produces there, nor whether the Info-editor warning has any counterpart outside the interface. This is the sharpest gap in the answer, and it is precisely the part the question is asking about.


> **Brought up to date on 2026-08-26 by hand test.** Both streams were captured separately, in this version, on two machines, for five kinds of missing file, and the same files were then opened with a real window. See "Answered by hand test — question 13" at the end of this question's material.

### Operating system

The sources do not say. None of the passages above is marked as belonging to one operating system, and the missing-file behaviour is never discussed per platform. The only platform-flavoured remark I found nearby is that on Windows the console is hidden by default and is reopened with Window ‣ Toggle System Console (https://docs.blender.org/manual/en/4.5/advanced/command_line/launch/windows.html, read 2026-07-30) — which affects whether a person can see the output, not what is written.


> **Brought up to date on 2026-08-26 by hand test.** macOS and Linux were both measured in this version and agreed word for word on every missing-file run. Windows remains untested: no Windows machine has run it.

### Record of where I looked, for question 13

Searched, case-insensitively, across all four 4.5 sources at once:

- `stderr`, `stdout`, `standard error`, `standard output`, `std::cerr`, `std::cout` — two hits in the entire set, both named above.
- `stream` — in the manual it returns only video bit-rate and multiplexing text; nothing about a program's output streams.
- `redirect`, `pipe`, `2>&1`, `tee` — nothing bearing on the question.
- `sys.stdin`, `sys.stdout`, `sys.stderr` — a single hit, the `--debug` note "Keeps Python’s sys.stdin rather than setting it to None", which is about input, not output.
- `missing file`, `missing files`, `file not found`, `not found`, `cannot find`, `can't find`, `unable to open`, `unable to find`, `unable to load`, `missing image`, `missing texture`, `missing library`, `missing linked` — every hit reviewed; the useful ones are quoted above. The rest are boilerplate ("The RNA type or default when not found" appears on hundreds of API pages) or unrelated (a 404 page, an attribute-node fallback).
- `does not exist`, `doesn't exist`, `no longer exist`, `broken link`, `broken path`, `invalid path`, `unable to write`, `cannot be found/opened/read/written` — reviewed; only the topbar, recent-files and linked-libraries passages are relevant, and all three are quoted or named above.
- `magenta`, `pink`, `placeholder` — this is how I found the sequencer's Show Missing Media option and the render-output "Placeholders" option (the latter is about pre-creating empty frames for a shared render directory, not about missing input, so I ruled it out).
- `console window`, `terminal window`, `system console`, `toggle system console` — every hit reviewed; the relevant ones are quoted above.
- In Cycles' Python files only (`__init__.py`, `engine.py`, `operators.py`, `osl.py`, `properties.py`, `ui.py`, `presets.py`, `camera.py`, `version_update.py`): `self.report`, `report(`, `print(`, `error`, `warning`, `missing`, `not found`. This turned up the OSL messages quoted above and one denoising message, "Frame '%s' not found, animation must be complete" (`operators.py`, line 91), which belongs to the standalone denoise operator rather than to a render, so I ruled it out of this answer.

Pages read in full or in the relevant part, and what I ruled out:

- https://docs.blender.org/manual/en/4.5/interface/window_system/topbar.html — read the whole External Data section (Pack/Unpack Resources, Make Paths Relative/Absolute, Report Missing Files, Find Missing Files).
- https://docs.blender.org/manual/en/4.5/files/linked_libraries/link_append.html — read the Relocate and Broken Libraries sections in full.
- https://docs.blender.org/manual/en/4.5/files/blend/open_save.html — read the "Items Not Found" entry; it is about pruning the recent-files list, so ruled out.
- https://docs.blender.org/manual/en/4.5/editors/info_editor.html — read in full.
- https://docs.blender.org/manual/en/4.5/interface/window_system/status_bar.html — read the Running Task and Report Message entries.
- https://docs.blender.org/manual/en/4.5/advanced/command_line/arguments.html — read end to end, specifically for `--quiet`, `--log`, `--log-level`, `--log-file`, `--verbose` and the debug switches.
- https://docs.blender.org/manual/en/4.5/advanced/command_line/index.html — read in full.
- https://docs.blender.org/manual/en/4.5/render/cycles/osl/index.html — read the Usage section in full.
- https://docs.blender.org/manual/en/4.5/render/output/properties/output.html — read the Image Sequence and Encoding sections, which contain the only manual sentence about seeing more output with `blender -d`; nothing about missing input files.
- https://docs.blender.org/manual/en/4.5/editors/video_sequencer/preview/sidebar.html — read the View panel entries around Show Missing Media.
- https://docs.blender.org/manual/en/4.5/troubleshooting/startup.html — read in full; it lists common console messages at startup (`found bundled python`, `Read prefs`) and none concern missing scene files.
- https://docs.blender.org/api/4.5/bpy.ops.file.html — read the `find_missing_files` and `report_missing_files` entries in full.
- https://docs.blender.org/api/4.5/bpy.types.SequenceEditor.html — read the `show_missing_media` entry.
- https://docs.blender.org/api/4.5/info_gotchas_file_paths_and_encoding.html — read; it is about text encoding of paths, not about absent files. Ruled out.
- Release notes for 4.5: searched all nineteen files for `missing`, `stderr`, `stdout`, `log`, `error`. The only near-miss is a fixed bug titled "Internal mode in IES texture node flagged in missing file report" (https://developer.blender.org/docs/release_notes/4.5/bugfixes/), which confirms the missing-file report exists but says nothing about streams.
- Cycles' own files: as noted for question 12, this folder has no renderer session or output-handling source, only the add-on Python, the OSL shaders and the kernel headers, so there is no place in it that would carry a general statement about streams. The two mentions I found are the `--cycles-print-stats` help text and the OSL messages, both quoted.

---


## Answered by hand test — question 13, 2026-08-26

**LABEL: observed.** Everything in this section is what two real machines put on their two streams. None of it comes from documentation.

### What was done, and on what

Two machines, and their results are never merged.

- **The Creator's Mac** — a MacBook Pro with an Apple M1 Pro processor, macOS 26.5.2, Apple's own graphics and no second graphics card. Blender 4.5.12 LTS, build hash `84afd5f785f7`, built 2026-07-21 01:41:35, run from `/Applications/Blender 4.5.12.app/Contents/MacOS/Blender`.
- **The rented Linux machine, in Norway** — a Docker container holding half its host, running Ubuntu 22.04.5 LTS on kernel `6.8.0-136-generic`, with one NVIDIA GeForce RTX 3060 of 12,288 MiB on driver 595.84, about fifteen cores of an Intel Xeon E5-2697A v4, and a display server of its own on `DISPLAY=:1`. Its viewport is drawn by its processor, not by the card, and it is used as `root`. Blender 4.5.12 LTS, build hash `84afd5f785f7`, built 2026-07-21 01:31:22, run from `/opt/blender/4.5.12/blender`.

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

**The whole run landed on standard output and standard error was empty**, on both machines. Blender 4.5.12 LTS wrote 48 lines: the version banner, `Read blend:`, the progress lines, `Saved:` and `Time:` and `Blender quit`. Status 0.

### A missing image texture, rendered in Cycles

```
blender -b tex_missing.blend -o <work folder>/c2_#### -f 1 > out.txt 2> err.txt
```

**Blender 4.5 says nothing at all about a missing image texture, on either stream.** Standard error was empty. Standard output held the ordinary run and no complaint of any kind.

The control run — the same command on a scene with nothing missing — printed 48 lines on standard output and nothing on standard error. The missing-texture run printed **49**, and the one extra line is not a complaint. It is an ordinary progress line naming the stage:

```
Fra:1 Mem:8.25M (Peak 9.10M) | Time:00:00.00 | Mem:0.00M, Peak:0.00M | Scene, ViewLayer | Updating Images | Loading missing_tex.png
```

Everything else about the two runs is the same: `Read blend:`, the progress lines, `Sample 8/8`, `Finished`, `Saved: '<work folder>/c2_0001.png'`, `Time: 00:00.05 (Saving: 00:00.04)`, `Blender quit`. The run ended at status **0** and wrote its picture.

Both machines gave this, word for word apart from the paths. **This is a silence that was looked for and found, not a silence assumed**: the control was run first, in the same shape, on the same machine, so that an empty error stream would mean something.

### The same missing texture, rendered in EEVEE

```
blender -b tex_missing.blend -E BLENDER_EEVEE_NEXT -o <work folder>/c3_#### -f 1 > out.txt 2> err.txt
```

**In EEVEE, Blender 4.5 does say something, and it is the one line in this whole round that 4.5 puts on standard error.** Standard error held exactly one line:

```
GPUTexture: Blender Texture Not Loaded!
```

Standard output held eleven lines and none of them mentions the image:

```
Blender 4.5.12 LTS (hash 84afd5f785f7 built 2026-07-21 01:41:35)
Read blend: "<scene folder>/tex_missing.blend"
Fra:1 Mem:40.38M (Peak 40.38M) | Time:00:00.79 | Rendering 1 / 64 samples
Fra:1 Mem:40.46M (Peak 40.46M) | Time:00:00.88 | Rendering 25 / 64 samples
Fra:1 Mem:40.46M (Peak 40.46M) | Time:00:00.96 | Rendering 50 / 64 samples
Fra:1 Mem:40.46M (Peak 40.46M) | Time:00:00.99 | Rendering 64 / 64 samples
Saved: '<work folder>/c3_0001.png'
Time: 00:01.04 (Saving: 00:00.05)


Blender quit
```

The status was 0 and the picture was written. Both machines gave the same line on the same stream.

### The same missing texture, with quiet mode

```
blender -b tex_missing.blend -q -o <work folder>/c4_#### -f 1 > out.txt 2> err.txt
```

**Quiet mode silences the whole run in 4.5.** With `-q` added, standard output held one line and standard error was empty:

```
Blender 4.5.12 LTS (hash 84afd5f785f7 built 2026-07-21 01:41:35)
```

The render still happened and the picture was still written.

### The same missing texture, with a log file

```
blender -b tex_missing.blend --log-file <work folder>/c5.log -o <work folder>/c5_#### -f 1 > out.txt 2> err.txt
```

**The log file is empty in 4.5.** With `--log-file <work folder>/c5.log` added, the file was created and held nothing, and the two streams were exactly as they were without it — 49 lines on standard output, nothing on standard error.

### A missing linked library

```
blender -b lib_missing.blend -o <work folder>/c6_#### -f 1 > out.txt 2> err.txt
```

**Every one of the five lines is on standard output, and standard error is empty.** Status 0, picture written.

```
Blender 4.5.12 LTS (hash 84afd5f785f7 built 2026-07-21 01:41:35)
Read blend: "<scene folder>/lib_missing.blend"
Info: Read library: '<scene folder>/lib.blend', '//lib.blend', parent '<direct>'
Warning: Unable to open '<scene folder>/lib.blend': No such file or directory
Info: Cannot find lib '<scene folder>/lib.blend'
Info: LIB: Collection: 'LibColl' missing from '<scene folder>/lib.blend', parent '<direct>'
Warning: 1 libraries and 1 linked data-blocks are missing (including 0 ObjectData), please check the Info and Outliner editors for details
```

then the ordinary progress lines, then `Saved:` and `Time:` and `Blender quit`.

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
Error: Unable to pack file, source path '<scene folder>/missing_font.ttf' not found
Error: Unable to pack file, source path '<scene folder>/missing_font.ttf' not found
WARN (bke.vfont): source/blender/blenkernel/intern/vfont.cc:225 BKE_vfont_data_ensure: Font file doesn't exist: //missing_font.ttf
```

Status 0, picture written.

### A missing blend-file handed on the command line

```
blender -b <work folder>/not-here-at-all.blend -o <work folder>/c9_#### -f 1 > out.txt 2> err.txt
```

```
Blender 4.5.12 LTS (hash 84afd5f785f7 built 2026-07-21 01:41:35)
Error: Cannot read file "<work folder>/not-here-at-all.blend": No such file or directory

Blender quit
```

Standard error was empty. Status **1**, no picture.

**This is worth setting beside the argument errors**, which go the other way. `-f abc`, `-f ""` and `-o` with no value each put their complaint on **standard error** — `Error: not a number '-f / --render-frame abc'.`, `Error: incorrect comma use '-f / --render-frame '.` and `Error: you must specify a path after '-o  / --render-output'.` — while a file that cannot be read is complained about on **standard output**. So Blender does use both streams, and which one it uses does not follow from how serious the trouble is.

### The one stream the sources name

```
blender -b good.blend -o <work folder>/c10_#### -f 1 -- --cycles-print-stats > out.txt 2> err.txt
```

**The statistics land on standard output, not on standard error, and standard error is empty.** 4.5 wrote 123 lines on standard output, beginning:

```
Update statistics:
Scene:
    Total time: 0.000051s
        device_update                            0.000051s
```

and ending with `Shader statistics:`, `Object statistics:`, `Saved:` and `Time:`. Cycles' own help text for that option, which every reading of this question quotes, says the statistics go to `stderr`. **They do not.**

### What the picture holds where the missing file should have been

Every picture written above was read back with a script that prints the red, green, blue and alpha of three places in it — the centre, a quarter of the way across, and one pixel in from the corner. The two machines agreed to four decimal places.

> **Note added on 2026-08-26 by the citation check, which changed nothing in the table.** The table was re-run on both machines from a scene rebuilt to this document's own description — "a cube with a camera and a light". The sequencer row with the setting off came back identical to the number below on both machines, `(0.0, 0.0, 0.0, 0.0)`; the sequencer row with the setting on came back `(0.749, 0.3176, 0.6824, 1.0)` on both my machines against the `(0.7529, 0.3176, 0.6824, 1.0)` below — a difference of one step in 255, in red only, and both runs are on the record rather than merged. The two machines agreed with each other to four decimal places in every row, which is what the sentence above claims. The other rows could not be re-derived, because the description does not fix where the camera and the light sit or how bright the light is, and those decide the numbers. The numbers below are left exactly as first measured; what a re-run cannot settle is recorded here rather than overwritten.

| The run | The centre of the picture |
| --- | --- |
| the control, nothing missing | `(0.6902, 0.6902, 0.6902, 1.0)` — plain grey |
| missing image texture, Cycles | `(0.7373, 0.3882, 0.7412, 1.0)` — a lit magenta |
| missing image texture, EEVEE | `(0.7373, 0.3843, 0.7412, 1.0)` — the same magenta |
| missing linked library | `(0.0, 0.0, 0.0, 1.0)` — the linked thing simply is not there |
| missing font | `(0.0, 0.0, 0.0, 1.0)` |
| missing sequencer footage, setting on | `(0.7529, 0.3176, 0.6824, 1.0)` across the whole frame |
| missing sequencer footage, setting off | `(0.0, 0.0, 0.0, 0.0)` — fully transparent |

**Blender 4.5.12 LTS paints a missing image texture magenta**, and it does so while printing nothing at all about it.

### Blender's own channels, with a real window

The same files were opened with Blender's window rather than in background, on both machines. A start-up script turned the Outliner into an Info editor, invoked the render with `bpy.ops.render.render('INVOKE_DEFAULT')`, photographed the window and read `bpy.data.window_managers[0].reports`, then quit from a timer. On the rented Linux machine the whole screen was photographed with its own `shot.sh`; on the Creator's Mac, Blender's own `bpy.ops.screen.screenshot(filepath=...)` was used instead, so that nothing but Blender's own window was captured.

**In 4.5 Blender's own channels say nothing about either missing file — not about the texture, and not about the library.**

The Info editor was empty in every run, on both machines. No report box appeared along the bottom of the window. The build has no `WindowManager.reports` at all: the script's check printed `this build has no WindowManager.reports` every time.

For the missing texture, the Blender Render window showed a magenta cube. For the missing library, the Blender Render window showed a black square, while the terminal carried all five of the lines quoted above — **so in 4.5 the terminal is told and the window is not**.

For a scene with no camera, the render operator raised the error back into the script that called it rather than reporting it anywhere:

```
=== RENDER INVOKE RAISED: RuntimeError('Error: No camera found in scene "Scene"\n')
```

No Blender Render window opened and the Info editor stayed empty.

One thing to know when reading this. In the first run on the Creator's Mac the picture showed the Outliner where the Info editor should have been, even though the script had already changed it — the window had not redrawn. The run was made again with every area tagged for redraw first, and the second picture shows the Info editor, empty. Both runs are on the record.

**One thing to know when reading the pictures.** Every run used a fresh settings folder, so Blender's Quick Setup panel is sitting over the middle of the 3D viewport in all of them. It never covered the Info editor, the status bar or the Blender Render window, and it did not stop the render.

### The two standing lines, brought up to date

**Outside the interface.** Settled by measurement, both ways round, for this version on both machines. Driven from outside with no window, the answers are the ones set out above: nothing at all for a missing image texture, five lines on standard output for a missing library, twelve for a missing font, one for a missing blend-file, and nothing at all for missing sequencer footage — **and standard error empty in every one of them except 4.5 rendering in EEVEE**. Driven from inside its own window, the same files give nothing in the Info editor at all — not for the texture and not for the library. **So the two settings do not agree, and which way they differ depends on the kind of file.**

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

2026-08-26 — Claude Opus 5 (Claude Code subagent) — Reports, Blender 4.5.

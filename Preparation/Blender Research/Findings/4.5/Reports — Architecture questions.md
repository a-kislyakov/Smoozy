# Reports — Architecture questions (Blender 4.5)

Questions answered: **12 and 13** only.

**Sources consulted, and the date.** Blender's 4.5 manual (`Blender Sources/manual_4.5`, including its `_images` folder), Blender's 4.5 Python API documentation (`Blender Sources/api_4.5`), the 4.5 release notes (`Blender Sources/release_notes_4.5`), and Cycles' own files as shipped with 4.5 (`Blender Sources/cycles_4.5`). All four were taken on **2026-07-30**, and that is the date read for every finding below.

I read only the question sheet and files inside Blender Sources/. I did not open any other part of this project.

(For completeness: among the files inside `Blender Sources/` I also read `About this folder.md`, which is that folder's own note. It describes what the folder holds. I did not follow the link it carries to anything outside the folder. I used no helper agents; every finding below is my own reading.)

---

## 12. What ending status does a run of Blender give back to whatever started it, for each of the ways a run can end — finishing normally, being interrupted, and being refused because something it was handed cannot be used?

**LABEL: not in the sources** — for the three endings the question names. Nothing in the 4.5 manual, the 4.5 API documentation, the 4.5 release notes, or Cycles' own 4.5 files states the number a run of Blender hands back when a render finishes, when a run is interrupted, or when a run is refused because a file, an argument or a device it was handed cannot be used.

Four narrower things *are* in the sources, and they are the whole of what the sources say about ending status. None of them is the answer to the question, but together they show where the silence sits.

### 12a. The only exit code the manual ever names is one you set yourself, and only for Python errors

**Label: documented.**

Exact words:

> `--python-exit-code <code>`
>
> Set the exit-code in [0..255] to exit if a Python exception is raised
> (only for scripts executed from the command line), zero disables.

Source: the manual, https://docs.blender.org/manual/en/4.5/advanced/command_line/arguments.html — read 2026-07-30.

What this settles: an exit code exists, the code set by this option lives in the range 0 to 255, and there is a way to make a Python exception produce one. What it does not settle: the code for anything other than a Python exception, and what the value is when the option is not given. The words "zero disables" tell you what passing zero does; they do not state the default.

### 12b. A command registered for `-c` / `--command` returns a number, and the convention for that number is written down

**Label: documented.**

Exact words:

> `execute` (callable) – Callback, taking a single list of strings and returns an int.
> The arguments are built from all command-line arguments following the command id.
> The return value should be 0 for success, 1 on failure (specific error codes from the os module can also be used).

Source: the API documentation, https://docs.blender.org/api/4.5/bpy.utils.html (entry `bpy.utils.register_cli_command`) — read 2026-07-30.

The same page introduces that mechanism with these words:

> Register a command, accessible via the (-c / --command) command-line argument.

What this settles: a command registered this way is a callback that returns a whole number, and the documentation advises that the number be 0 for success and 1 for failure. What it does not settle: whether that number becomes the run's ending status at all, whether it reaches the calling program unchanged, and anything at all about a run that renders instead of running a command. The wording is "should be", which is advice to whoever writes the command, not a statement about Blender.

**A step the quoted words do not take.** Reading the returned number as the status the run hands back is a reader's step, not something either quotation says. Anything resting on it is **inferred**, not documented.

### 12c. An argument Blender does not recognise makes it exit — the status is not given

**Label: documented.**

Exact words:

> `blender -ba test.blend`
>
> …will exit since -ba is an unknown argument.

Source: the manual, https://docs.blender.org/manual/en/4.5/advanced/command_line/arguments.html (section "Argument Parsing") — read 2026-07-30.

This is the closest the manual comes to the third ending in the question — a run refused because something it was handed cannot be used. It says the run exits. It does not say with what status.

Three neighbouring options on the same page also say a run exits, and are equally silent on the status:

> `--debug-exit-on-error`
>
> Immediately exit when internal errors are detected.

> `-h, --help`
>
> Print this help text and exit.

> `-v, --version`
>
> Print Blender version and exit.

### 12d. Cycles' own file calls for the process to end with 1 when a named render device cannot be set

**Label: observed** (read out of a file supplied in this folder), with an **inferred** step marked below.

Exact lines, from `Blender Sources/cycles_4.5/engine.py`, inside the function `_parse_command_line()`, lines 38–41:

```
    if args.cycles_device:
        import _cycles
        if not _cycles.set_device_override(args.cycles_device):
            sys.exit(1)
```

The option those lines act on is declared a few lines above, at lines 16–20 of the same file:

```
    parser.add_argument("--cycles-device",
                        help="Set the device to use for Cycles, overriding user preferences and the scene setting."
                             "Valid options are 'CPU', 'CUDA', 'OPTIX', 'HIP', 'ONEAPI', or 'METAL'."
                             "Additionally, you can append '+CPU' to any GPU type for hybrid rendering.",
                        default=None)
```

File and place: `Blender Sources/cycles_4.5/engine.py`, lines 16–20 and 38–41. Read 2026-07-30. These files have no web address.

What is observed: the file says that if setting the named device fails, `sys.exit(1)` is called.

**The inferred step, written out:** to get from "the file calls `sys.exit(1)`" to "the program that started Blender receives 1" a reader has to supply that Blender's built-in Python is not intercepting that call and that nothing later changes the code. The API documentation supports the first half of that, but only in general terms:

> Some Python modules will call sys.exit() themselves when an error occurs,
> while not common behavior this is something to watch out for because it may seem
> as if Blender is crashing since sys.exit() will close Blender immediately.

Source: the API documentation, https://docs.blender.org/api/4.5/info_gotchas_crashes.html (section "sys.exit") — read 2026-07-30. That says Blender closes immediately; it does not say with what status. So "a bad `--cycles-device` value makes Blender return 1" is **inferred**, not observed.

### Where I looked, and what I ruled out

Searched, over all four 4.5 source folders at once (`manual_4.5`, `api_4.5`, `release_notes_4.5`, `cycles_4.5`), case-insensitively:

- `exit code`, `exit status`, `exit-code`, `return code`, `returncode` — two hits, both the `--python-exit-code` entry above.
- `exit` on its own across the manual — 35 hits. All are either an add-on's `--help` line, the command-line entries quoted above, or unrelated ("exit Edit Mode", "exit Fly navigation", "exit velocity", "saved on exit").
- `errorlevel`, `status code`, `error code`, `exit value`, `quit with`, `0 for success`, `on success`, `on failure`, `failure code` — the only relevant hit in the whole set is the `register_cli_command` line quoted in 12b.
- `sys.exit`, `os._exit`, `wm.quit_blender` — three hits: the gotchas page quoted above, `cycles_4.5/engine.py` quoted above, and `bpy.ops.wm.quit_blender()`, whose entire documentation is the two words "Quit Blender" (https://docs.blender.org/api/4.5/bpy.ops.wm.html).
- `ctrl-c`, `ctrl+c`, `sigint`, `sigterm`, `signal`, `kill` — no hit anywhere describes stopping a running Blender from outside. Every `Ctrl-C` hit is the copy shortcut. The one `kill` hit is about a graphics driver killing the application (https://docs.blender.org/manual/en/4.5/render/eevee/limitations/limitations.html). This is why the "interrupted" third of the question has no answer at all: the sources never describe an interrupted run, let alone its status.
- `abort` — hits are Esc-cancelling a tool or a simulation inside the interface, plus `--disable-abort-handler`, whose documentation is the single line "Disable the abort handler." None connects to a status handed back.

Pages opened in full and ruled out:

- https://docs.blender.org/manual/en/4.5/advanced/command_line/arguments.html — the full argument list. Every section read (Render, Cycles Render, Format, Animation Playback, Window, Python, Network, Logging, Debug, GPU, Misc, Sub-Commands, Argument Parsing, Argument Order, Environment Variables). Exit status appears only as 12a.
- https://docs.blender.org/manual/en/4.5/advanced/command_line/index.html — describes the Console Window and the use cases for running Blender from a terminal. Nothing on status.
- https://docs.blender.org/manual/en/4.5/advanced/command_line/render.html — the worked examples of rendering from the command line, single image and animation. Nothing on status.
- https://docs.blender.org/manual/en/4.5/advanced/command_line/launch/linux.html, `.../macos.html`, `.../windows.html` — how to start Blender on each system. Nothing on status. I also opened the two screenshots these pages carry, `_images/advanced_command-line_launch_linux_console.png` and `_images/advanced_command-line_launch_windows_console.png`, in case a shell prompt in them showed a code. The Linux one shows only audio-server warnings at startup. The Windows one shows a session ending with the line `Blender quit` and the prompt returning; no status is visible. (**documented**, from the manual's own pictures — but they settle nothing here.)
- https://docs.blender.org/manual/en/4.5/troubleshooting/startup.html and the rest of the `troubleshooting` section — about messages on screen, not about status.
- https://docs.blender.org/api/4.5/bpy.utils.html — read the whole `register_cli_command` entry and both of its worked examples. Both examples end their callback with `return 0`. That is example code, not a statement about Blender, so I have not treated it as a finding.
- https://docs.blender.org/api/4.5/bpy.app.handlers.html — read the handler list, because a handler for a cancelled render might have carried a statement about how the run ends. `render_cancel` is documented as "on canceling a render job. Accepts one argument: the scene datablock being rendered", and `load_post_fail` as "on failure to load a new blend file (after)". Neither says anything about ending status. Ruled out.
- All 19 release-note files for 4.5, searched for `exit`, `command line`, `command-line`, `error code`. Two hits, neither relevant (a Grease Pencil render-time note, and a new `--profile-gpu` argument in the EEVEE notes).
- Cycles' own files: `engine.py`, `operators.py`, `properties.py`, `osl.py`, `__init__.py`, `version_update.py` read or searched for exit-related calls. `engine.py` is the only one that ends the process. The rest of `cycles_4.5` is `source/` (kernel and utility headers that run on the render device) and compiled `.oso` shader files; neither kind can end the process.

**Not covered.** This answer does not settle the number handed back by: a render that completes; a render stopped by Ctrl-C, by a stop signal, or by Esc in the interface; a `.blend` that is absent, unreadable, or not a `.blend` at all; an engine name that does not exist; a frame range with nothing in it; or a scene whose files are missing. It does not settle what `--python-exit-code` defaults to when it is not passed. It does not settle whether a code returned by a `--command` callback survives to the calling program unchanged. And on 12d it settles only what the file says, not what a run does.

**Outside the interface.** Everything above is *only* about running Blender from outside its own window — an ending status has no meaning inside it. Both documented mechanisms say so in their own words: `--python-exit-code` applies "only for scripts executed from the command line", and `register_cli_command` registers something "accessible via the (-c / --command) command-line argument". The Cycles finding in 12d likewise sits behind a command-line argument. Where the sources are silent, they are silent for the driven-from-outside case too, which is the case the question is about.

> **Brought up to date on 2026-08-26 by hand test.** The status a run hands back was measured on two machines in this version, and all of it was measured from outside Blender's own window, which is the only place a status exists. See "Answered by hand test — question 12" at the end of this question's material.


**Operating system.** The sources do not say. The range "[0..255]" in 12a is given with no operating-system qualification. The same manual page does mark options that are limited to one system, in so many words — "(Windows only)" on `/?` and `-con`, "(Windows & Linux only)" on `--register` and `--unregister` — and it puts no such mark on `--python-exit-code`. Reading that absence as "it works everywhere" is a step the manual does not take, so it is **inferred**, and weakly. Nothing in any source states how an ending status behaves differently on Windows, macOS or Linux.

> **Brought up to date on 2026-08-26 by hand test.** macOS and Linux were both measured in this version and agree almost everywhere; the places they do not are named in "Answered by hand test — question 12" below. Windows remains untested: no Windows machine has run it. **Untried rather than untestable: a Windows machine can be rented, and every one this project has read reported a Server edition — see the equipment section of `Preparation/Blender Research/Method.md`.**


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

**LABEL: not in the sources.**

**A note on the question, which is ambiguous.** "Output streams" can be read two ways. The reading I have answered is the operating-system one: standard output and standard error, the two separate streams a program that started Blender can capture apart from one another. The other reading is Blender's own reporting channels — the terminal text, the Info editor, the status bar. I have said below what the sources hold for that second reading too, because it is short, but the first reading is the one I have taken as meant.

**On the first reading — standard output and standard error — the sources are silent, twice over.** Apart from one line in Cycles' own files, quoted below, they never distinguish the two streams at all, and nowhere do they state what a render prints when a file the scene points at is missing. Either gap alone would be enough; both are present.

The 4.5 manual, API documentation and release notes never use the words `stdout`, `stderr`, `standard error`, or `standard output` in any sense connected to Blender's output. A search for those four terms across all three returns exactly one hit, and it is a colour-correction node's output socket being called "Standard output image".

Exactly one place in everything I was given names a stream at all, and it is not about missing files. From `Blender Sources/cycles_4.5/engine.py`, lines 13–15:

```
    parser.add_argument("--cycles-print-stats",
                        help="Print rendering statistics to stderr",
                        action='store_true')
```

**Label for that line: observed** — and what is observed is narrow: the file declares an option, and the help text it declares it with names `stderr`. File and place: `Blender Sources/cycles_4.5/engine.py`, lines 13–15. Read 2026-07-30. It concerns render statistics, not missing files.

**A step beyond the file, marked.** Reading that help text as a statement about where a running render actually puts its statistics is a reader's step, so that reading is **inferred**, not observed. A file can show that an option exists and what its help text says; it cannot show what a render does with it. The same limit was marked on 12d and applies here for the same reason.

It is worth recording only because the manual's entry for the very same option names no stream:

> `--cycles-print-stats`
>
> Log statistics about render memory and time usage.

Source: the manual, https://docs.blender.org/manual/en/4.5/advanced/command_line/arguments.html — read 2026-07-30.

**On the second reading — Blender's own channels — here is everything the sources hold.** None of it is about a render, and none of it separates one channel from another under the condition the question names.

The manual treats terminal output as one undivided thing:

> The Console Window, also called a Terminal, is an operating system text window that displays
> messages about Blender's operations, status, and internal errors.
>
> When Blender is manually started from a terminal,
> Blender output is shown in the Console Window it is started from.

Source: the manual, https://docs.blender.org/manual/en/4.5/advanced/command_line/index.html — read 2026-07-30. **Label: documented.** "Blender output" is singular and undivided; nothing here splits it.

The one documented Blender feature aimed squarely at missing files is a manual action, not a render, and it reports into the interface:

> Report Missing Files
>
> This option is useful to check if there are links to unpacked files that no longer exist.
> After selecting this option, a warning message will appear in the Info editor's header.
> If no warning is shown, there are no missing external files.

Source: the manual, https://docs.blender.org/manual/en/4.5/interface/window_system/topbar.html (File ‣ External Data) — read 2026-07-30. **Label: documented.** It says a warning appears in the Info editor's header when a person picks that menu item. It says nothing about a render, and nothing about a stream.

The Info editor itself is described as a mixed log:

> The Info editor logs the executed operators as well as errors, warnings,
> and informational messages.

Source: the manual, https://docs.blender.org/manual/en/4.5/editors/info_editor.html — read 2026-07-30. **Label: documented.**

The API documentation lists the levels a message can carry — `DEBUG`, `INFO`, `OPERATOR`, `PROPERTY`, `WARNING`, `ERROR`, `ERROR_INVALID_INPUT`, `ERROR_INVALID_CONTEXT`, `ERROR_OUT_OF_MEMORY` (https://docs.blender.org/api/4.5/bpy_types_enum_items/wm_report_items.html, read 2026-07-30) — but never says which level lands on which stream, or whether any of them reaches a stream at all. **Label: documented**, for the list of levels itself.

### Where I looked, and what I ruled out

Searched, over all four 4.5 source folders at once, case-insensitively:

- `stderr`, `stdout`, `standard error`, `standard output` — across manual, API and release notes: one hit, the colour-balance node's output socket. Across `cycles_4.5`: one hit, the `--cycles-print-stats` help text quoted above.
- `stream` and `streams`, restricted to the command-line, troubleshooting and API `info_*` pages where such a word would have to appear — no hits.
- `redirect`, `2>&1`, `pipe`, `file descriptor` — no relevant hits. The `2>&1` hit is in the manual's own build instructions for translators, not about Blender's behaviour.
- `missing file`, `missing files`, `find missing`, `report missing` — four hits: the two topbar entries, the API entry for `bpy.ops.file.find_missing_files` ("Try to find missing external files", https://docs.blender.org/api/4.5/bpy.ops.file.html), and one line in the 4.5 bug-fix notes about an IES texture node being flagged in the missing-file report. None mentions a stream or a render.
- `no longer exist`, `does not exist`, `cannot be found`, `not be found`, `file is missing`, `broken link`, `unable to open`, `could not open`, `cannot open`, `failed to load`, `failed to open`, `failed to read` — 25 hits, all read. The relevant ones: a video-editing page that mentions a "'file not found' error" and a "'file is missing' error" only as reasons to keep a tidy project folder (https://docs.blender.org/manual/en/4.5/video_editing/setup/directory_structure.html); a linked-libraries page about fixing a broken link by relocating it (https://docs.blender.org/manual/en/4.5/files/linked_libraries/link_append.html); and the API property `bpy.types.ID.is_missing`, "This data-block is a place-holder for missing linked data (i.e. it is [an override of] a linked data that could not be found anymore)" (https://docs.blender.org/api/4.5/bpy.types.ID.html). All three concern the state of the data, not what gets printed.
- `printed to the console`, `to the console`, `console window`, `terminal window` — 24 hits, all read. Every one either tells a person to open a terminal so they can see messages, or is the undivided-output wording quoted above.
- `^Warning:`, `^Error:` and quoted `"Warning:"` / `"Error:"` — searched for message text quoted verbatim in the documentation, since a quoted message would show its own wording. The only render-related hit anywhere is "Error: Out of memory" on the GPU rendering page, which is a different condition from the one this question names.
- `not found`, `cannot open`, `unable to`, `failed to`, `fprintf`, `printf(` across `cycles_4.5` — the printing hits are all debug helpers inside device-side kernel headers (`printf("%s: %d %d %d\n", ...)` in `source/util/types_int3.h` and its neighbours) and an OSL `printf` shim. None of them concerns a missing file.

Ruled out and why:

- `Blender Sources/cycles_4.5/source/` — 380-odd files, all headers for code that runs on the render device (`kernel/`, `util/`) plus compiled `.oso` shaders. This part of Cycles has no access to the process's streams and contains no message a person would ever see. It cannot answer the question, so I did not read it page by page; I searched all of it for the terms above instead.
- `Blender Sources/cycles_4.5/operators.py` — I read it in full because a search hit looked promising: at line 91 it builds the message `"Frame '%s' not found, animation must be complete"` and passes it to `self.report({'ERROR'}, err_msg)`. That is the Denoise Animation operator checking for already-rendered frames on disk before it starts, not a render meeting a missing file, and `report` names no stream. Ruled out.
- The compositor's File Output and Image node pages, and the image-related API pages, were reached by the search terms above and hold nothing on missing-file messages.

**Two things I found while searching that bear on missing files but not on streams**, recorded here so the search is honest rather than as findings, since the question asks about streams and not about picture content:

- `Blender Sources/cycles_4.5/source/util/texture.h`, lines 11–17, carries the comment `/* Color to use when textures are not found. */` above an enum `TEX_IMAGE_MISSING_R = 1, TEX_IMAGE_MISSING_G = 0, TEX_IMAGE_MISSING_B = 1, TEX_IMAGE_MISSING_A = 1`.
- The manual's Video Sequencer preview overlay has a "Show Missing Media" setting: "Render missing images/movies with a solid magenta color. When disabled, missing content will render fully transparent." (https://docs.blender.org/manual/en/4.5/editors/video_sequencer/preview/sidebar.html)

**Not covered.** This answer does not settle what a render actually prints when a texture, a linked `.blend`, a movie or image-sequence strip, a font, a cache file or a volume file is missing — only that the sources do not say. It does not settle whether such a message exists at all. It does not settle whether the render still produces a picture, or with what in it. And it does not settle whether Blender's warnings and errors reach standard error or standard output, in a render or anywhere else; the single stream named in any of these sources is named for render statistics only.

**Outside the interface.** The question is about streams, which only exist when Blender is driven from outside its own window, so this is the case I searched. The sources are silent for it. For the inside case the sources do say a little — a warning in the Info editor's header, but only for the manual "Report Missing Files" action, and a magenta or transparent picture for missing sequencer media. Neither carries over to a run driven from outside, because neither is a stream and the manual never links the two. Where the difference lies is exactly what the sources do not say.

> **Brought up to date on 2026-08-26 by hand test.** Both streams were captured separately, in this version, on two machines, for five kinds of missing file, and the same files were then opened with a real window. See "Answered by hand test — question 13" at the end of this question's material.


**Operating system.** The sources do not say. Nothing in the manual, the API documentation, the release notes or Cycles' files ties any missing-file message, or the choice of stream, to Windows, macOS or Linux. The one stream named — `stderr`, in `cycles_4.5/engine.py` — is named in a Python file with no operating-system condition around it, but that is a statement about that one option, not about missing files.

> **Brought up to date on 2026-08-26 by hand test.** macOS and Linux were both measured in this version and agreed word for word on every missing-file run. Windows remains untested: no Windows machine has run it.


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

2026-08-25 — Claude Opus 5 (Claude Code subagent) — Reports, Blender 4.5.

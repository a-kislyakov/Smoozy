# Reports — Architecture questions, second pass (Blender 5.2)

**Sources consulted, and when.** Blender's 5.2 manual (`Blender Sources/manual_5.2`, including its `_images` pictures), Blender's 5.2 Python API documentation (`Blender Sources/api_5.2`), the 5.2 release notes (`Blender Sources/release_notes_5.2`), and Cycles' own files (`Blender Sources/cycles_5.2`). All four were taken on **2026-07-30**, and that is the date read for every finding below. I also read `Blender Sources/About this folder.md` and `Blender Sources/requirements/requirements_2026-08-01.txt`, both of which sit inside `Blender Sources/`.

I read only the question sheet and files inside Blender Sources/. I did not open any other part of this project.

Two things to declare, so the line above is not doing quiet work.

First, before writing this file I ran a check that the folder it had to be saved into exists. I suppressed that check's output, so I saw nothing from it but the words "dir exists" that I printed myself.

Second, and this one I got wrong. After the answers below were finished and saved, I ran a check that this file had been written under the right name, and that check listed the whole folder. So I saw the names of the other documents sitting beside this one — about thirty-four of them. I was told not to list that folder and I should not have. I opened none of them, and I read no word of any of their contents; what I saw was file names only, each naming a topic and a pass, none carrying any statement about how Blender behaves. It also happened after every answer below was already written and saved to disk, so it could not have shaped any of them. I am recording it rather than letting it pass, because the value of this pass rests on that being verifiable.

I did no work through helper agents. Everything below I read myself.

Questions answered: **12 and 13** only.

---

## 12. What ending status does a run of Blender give back to whatever started it, for each of the ways a run can end?

**LABEL: not in the sources.**

None of the three documents says what number a run of Blender 5.2 hands back to the program that started it. Not for a run that finishes normally, not for a run that is interrupted, and not for a run refused because something it was handed cannot be used. The words for that number — exit code, exit status, return code, status code, error level — appear in the 5.2 manual exactly once, in the description of one command-line argument, and appear nowhere at all in the 5.2 API documentation or the 5.2 release notes.

Below is everything the sources do say that touches the subject, and what each one leaves open.

### (a) One exit status can be set, for one cause only — documented

Exact words:

> `--python-exit-code <code>`
>
> Set the exit-code in [0..255] to exit if a Python exception is raised
> (only for scripts executed from the command line), zero disables.

Source: the manual, Command Line Arguments, Python Options.
Address: https://docs.blender.org/manual/en/5.2/advanced/command_line/arguments.html
Date read: 2026-07-30

This is the only sentence in any of the three documents that speaks of an exit code. It tells you three things: that a run of Blender has an exit code, that the allowed range is 0 to 255, and that you may choose the one used when a Python script given on the command line raises an exception. It does not say what code the run gives back when this argument is not passed, and it does not cover any of the three endings the question asks about.

### (b) A sub-command's own answer is expected to be 0 or 1 — documented

Blender 5.2 can be given a named command with `-c` / `--command`. The API documentation describes how such a command is registered, and what its callback should return.

Exact words:

> `execute` (Callable[[list[str]], int]) – Callback, taking a single list of strings and returns an int.
> The arguments are built from all command-line arguments following the command id.
> The return value should be 0 for success, 1 on failure (specific error codes from the `os` module can also be used).

Source: the API documentation, `bpy.utils.register_cli_command`.
Address: https://docs.blender.org/api/5.2/bpy.utils.html
Date read: 2026-07-30

This is a convention for what a command hands back to Blender. The page does not say that this number becomes the status the run hands back to the program that started Blender. That link is not made anywhere I could find.

### (c) Cycles asks for status 1 when handed a device it cannot use — observed, with an inferred step

This is the closest thing in anything I was given to the question's third ending — a run refused because something it was handed cannot be used.

Exact lines, from `Blender Sources/cycles_5.2/engine.py`, inside `_parse_command_line()`, lines 38–41:

```
    if args.cycles_device:
        import _cycles
        if not _cycles.set_device_override(args.cycles_device):
            sys.exit(1)
```

Source: Cycles' own files, `cycles_5.2/engine.py`, lines 38–41.
Date read: 2026-07-30

**Observed:** the file contains those four lines. `--cycles-device` is the argument the manual documents for choosing the render device (Command Line Arguments, Cycles Render Options: "Set the device used for rendering. Valid options are: CPU CUDA OPTIX HIP ONEAPI METAL."). When the value given cannot be set, this code runs `sys.exit(1)`.

**Inferred, and here is the step:** reading `sys.exit(1)` as "end the run with status 1" is a step beyond what the file says. It rests on what `sys.exit` means in Python generally, and none of my four sources states that. It also rests on Blender letting that request through to the program that started it, rather than catching it while starting an add-on — and none of my sources says which happens. I have marked the line itself observed and stopped there deliberately. I will not supply the rest from memory.

### (d) An unknown argument ends the run — documented, but the status is not given

Exact words:

> Arguments must be separated by white space, eg:
>
> `blender -ba test.blend`
>
> …will exit since `-ba` is an unknown argument.

Source: the manual, Command Line Arguments, Argument Parsing.
Address: https://docs.blender.org/manual/en/5.2/advanced/command_line/arguments.html
Date read: 2026-07-30

This is a plain statement that being handed something unusable ends the run. It does not say what status the run gives back.

The same page has a second case of the run ending on an internal problem, again without a status:

> `--debug-exit-on-error`
>
> Immediately exit when internal errors are detected.

Same page, same address, same date.

### (e) Interruption — the sources say a render can be cancelled, and nothing about the status

The only place in any of my sources that connects an interruption from outside to a render is a page about Blender used as a Python module.

Exact words:

> Signal Handlers
>
> Blender's typical signal handlers are not initialized, so there is no special handling for Control-C
> to cancel a render and a crash log is not written in the event of a crash.

Source: the API documentation, Blender as a Python Module.
Address: https://docs.blender.org/api/5.2/info_advanced_blender_as_bpy.html
Date read: 2026-07-30

**Inferred, and here is the step:** the sentence is about the Python module. Saying that Blender's "typical" signal handlers are *not* initialized there, and that Control-C therefore has no special handling, only carries meaning if the ordinary Blender program does have them and Control-C there does cancel a render. That is the reader supplying a step, so the label is inferred, not documented. Even taking the step, it tells you only that a render is cancelled. It says nothing about what the interrupted run then hands back.

One more, weaker, mention of interrupting from a terminal:

> If the script runs for too long or you accidentally enter an infinite loop,
> Ctrl-C in the terminal (Ctrl-Break on Windows) will quit the script early.

Source: the API documentation, Tips and Tricks, "Use the Terminal".
Address: https://docs.blender.org/api/5.2/info_tips_and_tricks.html
Date read: 2026-07-30

That is about ending a script, not a render, and again gives no status.

### (f) One supplied command answers 0 or 1 — observed

`Blender Sources/cycles_5.2/maketx.py` is the file behind `blender --command maketx`, a command the 5.2 release notes document ("To generate Cycles native tx files from the command line, the following commands as provided: `blender scene.blend --command maketx`" — release notes, Cycles, https://developer.blender.org/docs/release_notes/5.2/cycles/, read 2026-07-30).

Exact lines, `cycles_5.2/maketx.py`, lines 83–87:

```
        sys.stderr.write("Error: {:s}\n".format(str(ex)))
        return 1

    print(out_filepath)
    return 0
```

Source: Cycles' own files, `cycles_5.2/maketx.py`, lines 83–87.
Date read: 2026-07-30

**Observed:** on an error the file returns 1, on success it returns 0. Together with (b), that is the documented convention being followed. Whether either number reaches the program that started Blender is the same unanswered link as in (b).

The same file has a third path, lines 94, 97 and 100, which calls `parser.error("No image file path given and no blend file loaded.")` and two similar calls. What `parser.error` does to the run's status is not stated in the file, and I am not filling it in from memory.

### Not covered

- The three endings the question names are not answered. I found no number for a normal finish, none for an interruption, and none for a refusal, in any of my four sources.
- (a) covers only a Python exception from a command-line script, and only when the argument is passed.
- (b) and (f) concern what a `--command` sub-command hands to Blender, not what Blender hands to its caller.
- (c) shows a request for status 1 in Cycles' own file; whether that becomes the run's status is not established.
- Nothing here distinguishes a render that failed from a render that succeeded, in status terms.
- I found nothing about what a run gives back after a crash, or after being killed by something other than Control-C.

### Where I looked, and what I ruled out

Searched the whole of `manual_5.2`, `api_5.2` and `release_notes_5.2` for: `exit code`, `exit status`, `return code`, `status code`, `errorlevel`, `exit value`, `exit-code`, `python-exit-code`, `python_exit_code`, `non-zero` / `nonzero`, `error code`, `0 for success`, `on success`, `on failure`, `$?`, `exit` / `exits` / `exiting` / `exited`, `quit`, `abort`, `terminate`, `interrupt`, `SIGINT`, `SIGTERM`, `SIGKILL`, `signal handler`, `Ctrl-C`, `Control-C`, `Ctrl-Break`, `status` (with status-bar hits set aside), `parent process`, `subprocess`, `calling program`, `caller`, `shell script`, `render farm`, `batch render`, `batch process`.

Pages opened in full or in large part, and why they were ruled out:

- Command Line Arguments (https://docs.blender.org/manual/en/5.2/advanced/command_line/arguments.html) — the one page that mentions an exit code at all; read end to end. Everything it says is in (a) and (d) above.
- Using Blender From The Command Line (https://docs.blender.org/manual/en/5.2/advanced/command_line/index.html) — describes the console and its uses; no statuses.
- Launching from the Command Line, all three pages (https://docs.blender.org/manual/en/5.2/advanced/command_line/launch/linux.html, .../macos.html, .../windows.html) — how to start Blender from a terminal; no statuses.
- Rendering From The Command Line (https://docs.blender.org/manual/en/5.2/advanced/command_line/render.html) — read in full. Argument order and worked examples only.
- Extensions Command Line Arguments (https://docs.blender.org/manual/en/5.2/advanced/command_line/extension_arguments.html) — a list of sub-commands and their help lines; no statuses.
- Security / "Command Line" section (https://docs.blender.org/manual/en/5.2/advanced/scripting/security.html) — batch rendering with `--enable-autoexec`; no statuses.
- Troubleshooting: Startup, Crashes, Python Errors (https://docs.blender.org/manual/en/5.2/troubleshooting/startup.html, .../crash.html, .../python.html) — read; they describe messages and crash log files, never a status handed back.
- Rendering an Animation (https://docs.blender.org/manual/en/5.2/render/output/animation.html) — mentions needing to interrupt rendering, purely as workflow advice.
- `bpy.utils` (https://docs.blender.org/api/5.2/bpy.utils.html) — read the whole `register_cli_command` entry and both worked examples; (b) above.
- Blender as a Python Module (https://docs.blender.org/api/5.2/info_advanced_blender_as_bpy.html) — read in full; (e) above.
- Tips and Tricks (https://docs.blender.org/api/5.2/info_tips_and_tricks.html) — read the terminal section; (e) above.
- `bpy.ops.wm.quit_blender()` (https://docs.blender.org/api/5.2/bpy.ops.wm.html) — says only "Quit Blender" and "Returns: Result of the operator call." That is an operator result, not a run's status, so it does not answer the question.
- `bpy.types.Operator` (https://docs.blender.org/api/5.2/bpy.types.Operator.html) — operator results such as `CANCELLED` and `FINISHED` are within Blender, not statuses handed to a caller. Ruled out for that reason.
- All 21 release-note files for 5.2 — searched as above and read `core.md`, `pipeline_io.md` and `cycles.md` in full. `bugfixes.md` was searched; its only near hits are a fixed crash on exit with more than four CLI commands and a fake render report printed on exit, neither of which gives a status.
- Cycles' own files: searched every file for `exit`, `sys.exit`, `returncode`, `subprocess`, `stderr`, `stdout`. Hits are only the ones quoted in (c) and (f), plus `engine.py`'s `exit()` function which calls `_cycles.exit()` and says nothing about a status.
- Pictures: I opened the three terminal screenshots the manual points at — `_images/advanced_command-line_launch_linux_console.png`, `_images/advanced_command-line_launch_macos_mac.png`, `_images/advanced_command-line_launch_windows_console.png`. The Windows one shows a run ending with the printed line `Blender quit`. **Documented, from the picture:** that text appears in the console when Blender ends. It is a printed line, not a status, and the question rules printed text out, so it does not answer here. The other two show startup messages only. No picture in the manual shows a status being checked.

### Outside the interface

The whole of this answer is about a run driven from outside — the argument in (a), the unknown-argument case in (d), the `--command` convention in (b) and (f), and the `--cycles-device` line in (c) all belong to Blender started and controlled by something other than a person clicking. So there is no inside/outside split to report: the sources are silent on the status in exactly the setting the question cares about. The one place that draws a difference between settings is (e), and that difference is between the ordinary Blender program and Blender imported as a Python module, not between clicking and not clicking. There, the module has no Control-C handling and writes no crash log.


> **Brought up to date on 2026-08-26 by hand test.** The status a run hands back was measured on two machines in this version, and all of it was measured from outside Blender's own window, which is the only place a status exists. See "Answered by hand test — question 12" at the end of this question's material.

### Operating system

The sources give no operating-system difference for any of this, because they give no statuses to differ. Two nearby points are stated per system. First, `--python-exit-code` carries no platform note, unlike neighbouring arguments on the same page which are explicitly marked "(Windows only)" or "(Windows & Linux only)", so the argument itself reads as available everywhere. Second, the interrupt key differs: "Ctrl-C in the terminal (Ctrl-Break on Windows)" (API, Tips and Tricks, address above, read 2026-07-30). Beyond those, the sources do not say.


> **Brought up to date on 2026-08-26 by hand test.** macOS and Linux were both measured in this version and agree almost everywhere; the places they do not are named in "Answered by hand test — question 12" below. Windows remains untested: no Windows machine has run it. **Untried rather than untestable: a Windows machine can be rented, and every one this project has read reported a Server edition — see the equipment section of `Preparation/Blender Research/Method.md`.**

---


## Answered by hand test — question 12, 2026-08-26

**LABEL: observed.** Everything in this section is what two real machines gave back when the question was put to them directly. None of it comes from documentation, and none of it was known before the runs.

### What was done, and on what

Two machines, and their results are never merged.

- **The Creator's Mac** — a MacBook Pro with an Apple M1 Pro processor, macOS 26.5.2, Apple's own graphics and no second graphics card. Blender 5.2.1 LTS, build hash `9e2066aef7ef`, built 2026-08-25 01:35:57, run from `/Applications/Blender.app/Contents/MacOS/Blender`.
- **The rented Linux machine, in Norway** — a Docker container holding half its host, running Ubuntu 22.04.5 LTS on kernel `6.8.0-136-generic`, with one NVIDIA GeForce RTX 3060 of 12,288 MiB on driver 595.84, about fifteen cores of an Intel Xeon E5-2697A v4, and a display server of its own on `DISPLAY=:1`. Its viewport is drawn by its processor, not by the card, and it is used as `root`. Blender 5.2.1 LTS, build hash `9e2066aef7ef`, built 2026-08-25 02:12:34, run from `/opt/blender/5.2.1/blender`.

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

**Two presses, three seconds apart.** 5.2 ended with exit code **2** — on both machines, four runs each. The terminal showed:

```
^C
Sent an internal break event. Press ^C again to kill Blender
^C
Blender killed
Error: Not freed memory blocks: 29876, total unfreed memory 16.509563 MB
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

**The manual's own unknown-argument example, `blender -ba good.blend`.** **Blender 5.2 does not end.** The run was still going after ninety seconds, on both machines, twice each, and the test harness had to kill it with `SIGKILL`. There is no ending status to report.

Standard output, two lines:

```
00:00.620  reports          | ERROR Cannot read file "<work folder>/-ba": No such file or directory
00:00.620  blend            | Read blend: "<scene folder>/good.blend"
```

Standard error, one line:

```
unknown argument, loading as file: -ba
```

Blender treats `-ba` as a file to open, fails to open it, opens the blend-file that followed instead, and carries on running. Blender 4.5 given the same command prints a second line on standard error — `Error: argument has no '.blend' file extension, not using as new file, exiting!` — and stops with status 1. **That line is gone in 5.2, and so is the stopping.**

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

So a name that is not on Cycles' list and a name that is on it but has no hardware behind it **both give 1**, and which names have hardware behind them is what separates the two machines. In 5.2 the refusal of a name that is not on Cycles' list lands on **standard output**, and standard error is empty:

```
00:00.383  cycles           | ERROR NOTADEVICE is not a valid Cycles device.
Blender 5.2.1 LTS (hash 9e2066aef7ef built 2026-08-25 01:35:57)
Error: Not freed memory blocks: 1, total unfreed memory 0.000004 MB
```

The refusal of a name that is on the list but has no hardware behind it also lands on standard output:

```
00:00.402  reports          | ERROR Found no Cycles device of the specified type
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

**Cycles' own `maketx` command.** **This command exists only in 5.2, of the four versions.** Blender 4.5, 5.0 and 5.1 answer both runs with `Unrecognized command: "maketx"` on standard error and status 1.

In 5.2, a picture that is there gave status **0**, and the only thing printed was the path of the file it wrote, on standard output:

```
/Users/artist/smoozy-handtests/reports-arch/scenes/5.2/blender_tx/tex_src.png.0-B30E7CBFC8AAF789D08104D249114503.tx
```

(The same run on the rented Linux machine produced the same name with the same hash in it.)

A picture that is not there gave status **1**, and — uniquely in this whole round — **announced itself on both streams at once**. Standard output:

```
00:00.397  cycles           | ERROR Source image not found: <work folder>/no_such_image.png
```

Standard error:

```
Error: Source image not found
```

So a command's own `return 1` does reach the calling program, and its failure text is written twice, once through Blender's log and once straight to standard error.

`blender --command help` in 5.2 lists three commands where the older versions list one:

```
Blender Command Listing:
	asset_listing
	extension
	maketx
```

**A scene with no camera**, `blender -b nocamera.blend -o <work folder>/nc_#### -f 1`. Status **1** on both machines, nothing written:

```
00:00.396  blend            | Read blend: "<scene folder>/nocamera.blend"
00:00.400  render           | Rendering single frame
00:00.400  render           | Rendering frame 1
00:00.400  reports          | ERROR Cannot render, no camera
Blender 5.2.1 LTS (hash 9e2066aef7ef built 2026-08-25 01:35:57)

Blender quit
```

**An output folder that cannot be written to** — made with `mkdir ro` and `chmod 500 ro`. On the Creator's Mac, status **1** in 5.2, with nothing written, and **both lines on standard output while standard error stayed empty**:

```
00:00.445  image.write      | ERROR OpenImageIO write failed: Could not open file "<work folder>/ro/p_0001.png"
00:00.445  reports          | ERROR Render error (Undefined error: 0) cannot save: '<work folder>/ro/p_0001.png'
00:00.445  render           | Time: 00:00.04
```

Blender 4.5 given the same run says `(Permission denied)` where 5.2 says `(Undefined error: 0)`, and puts a bare `<path>: Permission denied` on standard error, which 5.2 does not.

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

**In 5.2 this setting exists, and it is the sharpest form of the trap.** The Python expression printed `SET save_output = False`, the run went on to render, printed its ordinary progress and its `Time:` line, ended at **status 0** — and **the output folder was empty**.

> **A disagreement recorded on 2026-08-26 by the citation check; nothing above was changed.** This row was re-run on both machines. The half the trap turns on came back the same: **status 0 and an empty output folder**. The rest did not. In the re-run, on both machines, 5.2 printed no progress lines and no `Time:` line at all; it printed one line —
>
> ```
> 00:00.391  reports          | ERROR Render output disabled in Output properties and no active compositing File Output nodes
> ```
>
> — and then quit, which reads as refusing the render outright rather than rendering and declining to save. The exact command this row used is not written down here, and a different placement of the Python expression could account for it. The original result stands as written; the re-run is recorded beside it, and the row is on the list to be run again.

Blender 4.5, 5.0 and 5.1 have no such setting: there the same command reported `NO save_output IN THIS BUILD: 'RenderSettings' object has no attribute 'save_output'` and wrote the picture.

**The reversed frame range.** In 5.2 the run does announce the range it was given, on standard output:

```
00:00.397  blend            | Read blend: "<scene folder>/good.blend"
00:00.402  render           | Rendering animation (frames 10..5)
Blender 5.2.1 LTS (hash 9e2066aef7ef built 2026-08-25 01:35:57)

Blender quit
```

It says it is rendering frames 10 to 5, renders none of them, writes nothing, and ends at 0.

### The two standing lines, brought up to date

**Outside the interface.** Settled by measurement, for this version, on both machines. Every number above was got from a run started and watched by another program, with no window, which is the only setting in which an ending status exists at all. The window runs described under question 13 show the other side of the same coin: a render invoked from inside a window hands its outcome back to the Python that called it — `RuntimeError('Error: No camera found in scene "Scene"\n')` — and the *program's* status has nothing to do with it, because the program is still running.

**Operating system.** Settled for macOS and for Linux, and only for those two. Every result above was got on both, and **they agree in every one except a single case**: an unwritable output folder, where the Linux machine is used as `root` and is therefore not stopped by the folder's permissions — a difference in how the machine is used rather than in Blender. Everything else — the clean finishes, the interrupts, the signals, the refused files and arguments, the devices, the Python failures, the registered commands and the whole of the trap — came back the same on both, and this round the two machines carry **the same 5.2 build**, hash `9e2066aef7ef`, so none of that sameness can be an accident of two different builds agreeing. **Windows is untested and stays untested**: no Windows machine has run it, so what `echo %ERRORLEVEL%` or `$LASTEXITCODE` would report, and whether Ctrl-Break behaves as Ctrl-C does, are not settled by anything here.

### What these runs do not settle

- Anything on Microsoft Windows.
- What a working HIP or oneAPI device gives back. Both were refused for want of an AMD or an Intel card, so the *refusal* is settled and the success is not.
- What Blender loaded as a Python module hands back. Neither machine has such a build.
- Whether an unwritable output folder gives 1 on Linux, which needs an account that is not `root`.
- Why Blender 4.5 behaves as it does when handed an engine name that does not exist. What happened is recorded; nothing here explains it.


## 13. When a file the scene points at is not there, what does a render put on each of its output streams?

**LABEL: not in the sources.**

Blender's 5.2 documentation never names an output stream. The words `stderr`, `stdout`, `standard output` and `standard error` do not appear anywhere in the 5.2 API documentation, anywhere in the 5.2 release notes, and nowhere in the 5.2 manual except one unrelated compositor socket description ("Standard output image." on the Color Balance node page). So there is no source that could tell you which stream carries what, under this condition or any other.

Where the manual does describe where messages go, it treats the console as one undivided thing:

> The Console Window, also called a Terminal, is an operating system text window that displays
> messages about Blender's operations, status, and internal errors.
>
> When Blender is manually started from a terminal,
> Blender output is shown in the Console Window it is started from.

Source: the manual, Using Blender From The Command Line.
Address: https://docs.blender.org/manual/en/5.2/advanced/command_line/index.html
Date read: 2026-07-30

And:

> The Blender Console Window can display many different types of status and error messages. Some messages simply
> inform the user what Blender is doing, but have no real impact on Blender's ability to function.

Source: the manual, Troubleshooting: Startup, "Common Startup Messages".
Address: https://docs.blender.org/manual/en/5.2/troubleshooting/startup.html
Date read: 2026-07-30

One stream is named for its own separate destination, but it is a chosen file rather than a standard stream, and nothing ties it to missing files:

> `--log-file <filepath>`
>
> Set a file to output the log to.

Source: the manual, Command Line Arguments, Logging Options.
Address: https://docs.blender.org/manual/en/5.2/advanced/command_line/arguments.html
Date read: 2026-07-30

### The manual grants that a missing-file error exists, without saying what is printed or where

> It will lower the probability that you accidentally delete a file
> (which will result in a 'file not found' error) or that you forget
> to include a necessary file when transferring the project ('file is missing' error).

Source: the manual, Video Editing: Directory Structure.
Address: https://docs.blender.org/manual/en/5.2/video_editing/setup/directory_structure.html
Date read: 2026-07-30

This is the closest the manual comes. It names two error wordings in passing, as advice about organising a project. It does not say a render prints them, does not give the full text, and does not say where they go.

### The closest things I found, none of which answer the stream question

I list these because a silence found while looking for something else is not a silence, and I want the boundary visible.

**One place in anything I was given does put a missing-file error on a named stream — and it is not a render.** Cycles' `maketx.py`, the file behind `blender --command maketx <image>`:

```
        sys.stderr.write("Error: {:s}\n".format(str(ex)))
        return 1

    print(out_filepath)
    return 0
```

Source: Cycles' own files, `cycles_5.2/maketx.py`, lines 83–87. **Observed.** Date read: 2026-07-30.

That is texture-cache generation for one image file, not a render of a scene, so it does not answer question 13. It does show that the error text goes to the error stream while the ordinary result goes through `print`, in that one command.

**Cycles picks a colour for a missing image, which goes into the picture and not onto a stream.**

> `/* Color to use when images are not found. */`
> `#define IMAGE_MISSING_RGBA make_float4(1, 0, 1, 1)`

Source: Cycles' own files, `cycles_5.2/source/util/types_image.h`, lines 17–18. **Observed.** Date read: 2026-07-30.

The file says the colour is used when images are not found, and gives its four numbers. Reading those four numbers as any particular colour on screen, or claiming a render actually shows it, would be a step beyond the file, so I have not taken it.

**The Video Sequencer has an option about missing media, and it too is about the picture.**

> Show Missing Media
>
> Render missing images/movies with a solid magenta color.
> When disabled, missing content will render fully transparent.

Source: the manual, Video Sequencer, Preview Sidebar, View tab ‣ View Settings.
Address: https://docs.blender.org/manual/en/5.2/editors/video_sequencer/preview/sidebar.html
Date read: 2026-07-30. **Documented.**

This sits among the preview's view settings, alongside proxy resolution and overexposure highlighting, so on the page's own arrangement it is about the preview. Nothing on the page mentions a stream.

**Missing files can be reported inside the interface, into a panel rather than a stream.**

> Report Missing Files
>
> This option is useful to check if there are links to unpacked files that no longer exist.
> After selecting this option, a warning message will appear in the Info editor's header.
> If no warning is shown, there are no missing external files.

Source: the manual, Topbar, File menu ‣ External Data.
Address: https://docs.blender.org/manual/en/5.2/interface/window_system/topbar.html
Date read: 2026-07-30. **Documented.**

This is a menu item a person clicks, not something a render does, and the message goes to an editor header.

**A missing linked blend-file is handled at load time, and the manual says only what happens to the data.**

> If Blender cannot find a library while loading a blend-file,
> it will create placeholder data-blocks to replace missing linked ones.

Source: the manual, Linking and Appending, "Broken Libraries".
Address: https://docs.blender.org/manual/en/5.2/files/linked_libraries/link_append.html
Date read: 2026-07-30. **Documented.**

Again, no stream and no printed text.

**Cycles reports an unopenable shader file, but from a node update rather than a render.** `cycles_5.2/osl.py` contains, at lines 312 and 364, `report({'ERROR'}, rpt_("OSL query failed to open %s") % oso_path)`, and at line 314 `report({'ERROR'}, "OSL script compilation failed, see console for errors")`. **Observed.** These sit in `update_script_node` and its camera-shader twin, which are the shader-script update path, not the render path. The phrase "see console for errors" points at the console without dividing it.

### Not covered

- Which stream carries what, under this condition, is not settled. My sources cannot settle it.
- The question's condition covers any file a scene points at. I found no source describing what a render prints for a missing image texture, a missing volume file, a missing font, a missing sound, a missing cache, or a missing linked blend-file — the manual describes the last only in terms of placeholder data, not printed text.
- I have not established whether a missing file makes a render fail at all, or whether it carries on. That is a different question and I did not answer it.
- The `maketx.py` finding is a command, not a render, and covers one image file given on the command line.
- The `IMAGE_MISSING_RGBA` finding tells you a colour exists in Cycles' own file. It does not tell you what a render actually produces.

### Where I looked, and what I ruled out

Searched the whole of `manual_5.2`, `api_5.2`, `release_notes_5.2` and `cycles_5.2` for the stream words: `stderr`, `stdout`, `standard error`, `standard output`, `output stream`, `error stream`, `file descriptor`, `2>&1`, `redirect`, `pipe`, `tee`. In the manual and API the only hits are the unrelated Color Balance socket line, and one build instruction in the manual's own contributor pages. In Cycles' files the only hits are `maketx.py`'s `sys.stderr.write` and `engine.py`'s help text for `--cycles-print-stats` ("Print rendering statistics to stderr", `cycles_5.2/engine.py`, line 14 — noted only to show the search reached it; it is about statistics, not missing files).

> **Corrected on 2026-08-26 by the citation check.** The line number read 13; the help string is on line 14, and line 13 is `parser.add_argument("--cycles-print-stats",`. This document's own question-12 section already cited line 14 correctly.

Searched the same four sources for the missing-file words: `missing file`, `find missing`, `report missing`, `missing librar*`, `missing image`, `missing textur*`, `file not found`, `not found`, `could not find`, `could not open`, `could not load`, `cannot find`, `cannot open`, `cannot load`, `can't load`, `unable to open`, `unable to load`, `does not exist`, `no such file`, `invalid path`, `file is missing`, `broken link`, `unresolved`, `dangling`, `placeholder`, `magenta`, `pink`.

Also searched for the reporting words, to catch a page that described the printing without using the stream words: `printed to`, `to the console`, `in the console`, `console window`, `to the terminal`, `in the terminal`, `Error:`, `Warning:`, `log-level`, `--log`, `logging categor*`, `log file`.

Pages opened in full or in large part, and why they were ruled out:

- Using Blender From The Command Line (address above) — quoted; treats the console as one thing.
- Command Line Arguments (address above) — read end to end. The Logging Options section is the only part that describes where text goes, and it offers a file, not a stream. `-q, --quiet` says "Suppress status printing (warnings & errors are still printed)", which divides text by kind but not by stream.
- Troubleshooting: Startup (address above) — quoted; describes console messages generally and lists two startup messages, neither about a missing scene file.
- Troubleshooting: Crashes (https://docs.blender.org/manual/en/5.2/troubleshooting/crash.html) — about crash log files, not streams or missing files.
- Video Editing: Directory Structure (address above) — quoted; the nearest wording in the manual.
- Video Sequencer Preview Sidebar (address above) — quoted; picture, not stream.
- Topbar, External Data (address above) — quoted; interface, not stream.
- Linking and Appending (address above) — quoted; data, not stream.
- Rendering an Animation (https://docs.blender.org/manual/en/5.2/render/output/animation.html) and Output Properties (https://docs.blender.org/manual/en/5.2/render/output/properties/output.html) — read the Overwrite/Placeholders and "Primitive Render Farm" material; about files written, not about text printed.
- Cycles GPU Rendering, "Error Messages" (https://docs.blender.org/manual/en/5.2/render/cycles/gpu_rendering.html) — the one manual page that prints actual Cycles error text, including "Error: Out of memory" and "CUDA Error: Kernel compilation failed". Read in full and ruled out: every message there is about graphics hardware, drivers or memory, none is about a missing file, and the page never says which stream any of them uses.
- Info Editor (https://docs.blender.org/manual/en/5.2/editors/info_editor.html) — read; it logs errors and warnings into an editor, and says nothing about streams.
- `bpy.types.Operator.report` (https://docs.blender.org/api/5.2/bpy.types.Operator.html) — the report mechanism is described only by its type and message; no destination is named.
- Blender as a Python Module (address above) — read in full while working on question 12; nothing about streams or missing files.
- All 21 release-note files for 5.2 — searched for every term above; the only hits are the `maketx` command in `cycles.md` and, in `python_api.md`, a note that `bpy.data.file_path_foreach` gained "Three new options … so render farms and other scripts can find all relevant image files for rendering". That is about finding files ahead of time, not about what a render prints when one is absent.
- Cycles' own files — searched every file. The relevant hits are the three quoted above (`maketx.py`, `types_image.h`, `osl.py`). `operators.py` line 86 reports "Frame '%s' not found, animation must be complete" for a missing frame, but that is the denoise operator working over already-rendered frames, not a render, and no stream is named.
- Pictures: I opened the three terminal screenshots (`_images/advanced_command-line_launch_linux_console.png`, `_images/advanced_command-line_launch_macos_mac.png`, `_images/advanced_command-line_launch_windows_console.png`). All three show startup text in a single terminal window, where the two streams would look identical anyway, and none shows a render or a missing scene file. I also considered `_images/troubleshooting_python_traceback.png` and ruled it out: it shows a Python traceback in an add-on context, not a render with a missing file.

### Outside the interface

The question is about output streams, which only exist when the run is driven from outside. So the answer holds in that setting by construction: the sources are silent there. What the sources do describe about missing files — the Report Missing Files menu item, the Find Missing Files browser, the Video Sequencer's preview option, the red strips in the timeline — is all inside the interface and cannot be reached by a run started and controlled by something else. That leaves a run driven from outside with no documented account of what it prints at all under this condition.


> **Brought up to date on 2026-08-26 by hand test.** Both streams were captured separately, in this version, on two machines, for five kinds of missing file, and the same files were then opened with a real window. See "Answered by hand test — question 13" at the end of this question's material.

### Operating system

The sources do not say. Nothing in what I found about missing files carries an operating-system note. One nearby thing is stated per system: on Windows a run started from an icon opens its own Console Window that is then hidden, and the manual says "To display the console again, go to Window ‣ Toggle System Console" (the manual, Launching from the Command Line: Windows, https://docs.blender.org/manual/en/5.2/advanced/command_line/launch/windows.html, read 2026-07-30). That affects whether a person sees the text, not which stream carries it.


> **Brought up to date on 2026-08-26 by hand test.** macOS and Linux were both measured in this version and agreed word for word on every missing-file run. Windows remains untested: no Windows machine has run it.

---


## Answered by hand test — question 13, 2026-08-26

**LABEL: observed.** Everything in this section is what two real machines put on their two streams. None of it comes from documentation.

### What was done, and on what

Two machines, and their results are never merged.

- **The Creator's Mac** — a MacBook Pro with an Apple M1 Pro processor, macOS 26.5.2, Apple's own graphics and no second graphics card. Blender 5.2.1 LTS, build hash `9e2066aef7ef`, built 2026-08-25 01:35:57, run from `/Applications/Blender.app/Contents/MacOS/Blender`.
- **The rented Linux machine, in Norway** — a Docker container holding half its host, running Ubuntu 22.04.5 LTS on kernel `6.8.0-136-generic`, with one NVIDIA GeForce RTX 3060 of 12,288 MiB on driver 595.84, about fifteen cores of an Intel Xeon E5-2697A v4, and a display server of its own on `DISPLAY=:1`. Its viewport is drawn by its processor, not by the card, and it is used as `root`. Blender 5.2.1 LTS, build hash `9e2066aef7ef`, built 2026-08-25 02:12:34, run from `/opt/blender/5.2.1/blender`.

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

**The whole run landed on standard output and standard error was empty**, on both machines. Blender 5.2.1 LTS wrote 53 lines: the version banner, `Read blend:`, the announcement lines `Rendering single frame`, `Rendering frame 1`, `Start rendering: Scene, ViewLayer` and `Engine: Cycles`, the progress lines, `Saved:` and `Time:` and `Blender quit`. Status 0.

### A missing image texture, rendered in Cycles

```
blender -b tex_missing.blend -o <work folder>/c2_#### -f 1 > out.txt 2> err.txt
```

**Blender 5.2 says two things, both on standard output, and standard error is empty.** They are:

```
00:00.409  cycles           | WARNING Image file <scene folder>/missing_tex.png does not exist.
00:00.409  cycles           | ERROR Failed to load 1 image files
```

**Two things changed at 5.2.** The first line, which 5.0 and 5.1 print as `ERROR`, is now a `WARNING`. And the second line is new — a count of the files that could not be loaded, which no earlier version prints. Blender 5.0 prints the first sentence with single quotation marks round the path, Blender 5.1 without them, and Blender 4.5 prints nothing at all.

The two lines sit among the ordinary progress lines, the second immediately after `Updating Images | Loading missing_tex.png`, and the run carries on. Standard output held 56 lines in all; the control run, on a scene with nothing missing, held 53. The run ended at status **0** and wrote its picture.

Both machines gave this, word for word apart from the paths — and this round the two machines carry **the same 5.2 build**, hash `9e2066aef7ef`, so nothing here can be a difference between a release candidate and a finished build.

### The same missing texture, rendered in EEVEE

```
blender -b tex_missing.blend -E BLENDER_EEVEE -o <work folder>/c3_#### -f 1 > out.txt 2> err.txt
```

**In EEVEE the complaint is on standard output too, and standard error is empty.** The line is:

```
00:02.159  gpu.texture      | ERROR Failed to create GPU texture from Blender image
```

It does not name the file, and it is the same line 5.0 and 5.1 print. Standard output held fifteen lines in all, ending `Saved:` and `Time:`; the status was 0 and the picture was written.

Blender 4.5 puts its EEVEE complaint — `GPUTexture: Blender Texture Not Loaded!` — on **standard error** instead.

### The same missing texture, with quiet mode

```
blender -b tex_missing.blend -q -o <work folder>/c4_#### -f 1 > out.txt 2> err.txt
```

**Quiet mode does not remove either line in 5.2.** With `-q` added, standard output still held both, and standard error was still empty:

```
00:00.404  cycles           | WARNING Image file <scene folder>/missing_tex.png does not exist.
00:00.405  cycles           | ERROR Failed to load 1 image files
```

What `-q` removed was the ordinary progress lines. Blender 4.5 given `-q` prints nothing at all beyond its version banner.

### The same missing texture, with a log file

```
blender -b tex_missing.blend --log-file <work folder>/c5.log -o <work folder>/c5_#### -f 1 > out.txt 2> err.txt
```

**The log file holds everything, and the streams are left almost empty.** With `--log-file <work folder>/c5.log` added, the warning was in the file:

```
00:00.402  cycles           | WARNING Image file <scene folder>/missing_tex.png does not exist.
```

along with the whole rest of the log. Standard output was left with three lines — the version banner, a blank line and `Blender quit` — and standard error was empty. Blender 4.5 given the same option writes an **empty** file and leaves both streams untouched.

### A missing linked library

```
blender -b lib_missing.blend -o <work folder>/c6_#### -f 1 > out.txt 2> err.txt
```

**All five lines are on standard output, and standard error is empty.** Status 0, picture written.

```
00:00.395  reports          | Read library: '<scene folder>/lib.blend', '//lib.blend', parent '<direct>'
00:00.395  reports          | WARNING Unable to open '<scene folder>/lib.blend': No such file or directory
00:00.395  reports          | Cannot find lib '<scene folder>/lib.blend'
00:00.395  reports          | LIB: Collection: 'LibColl' missing from '<scene folder>/lib.blend', parent '<direct>'
00:00.396  reports          | WARNING 1 libraries and 1 linked data-blocks are missing (including 0 ObjectData), please check the Info and Outliner editors for details
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
00:00.399  reports          | ERROR Unable to pack file, source path '<scene folder>/missing_font.ttf' not found
00:00.399  reports          | ERROR Unable to pack file, source path '<scene folder>/missing_font.ttf' not found
00:00.399  geom.vfont       | WARNING Font file doesn't exist: //missing_font.ttf
```

Status 0, picture written. Blender 4.5 prints the same twelve, with the third carrying a source-file path and line number that 5.2 drops.

### A missing blend-file handed on the command line

```
blender -b <work folder>/not-here-at-all.blend -o <work folder>/c9_#### -f 1 > out.txt 2> err.txt
```

```
00:00.392  reports          | ERROR Cannot read file "<work folder>/not-here-at-all.blend": No such file or directory
Blender 5.2.1 LTS (hash 9e2066aef7ef built 2026-08-25 01:35:57)

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

**Blender 5.2.1 LTS paints a missing image texture magenta**.

### Blender's own channels, with a real window

The same files were opened with Blender's window rather than in background, on both machines. A start-up script turned the Outliner into an Info editor, invoked the render with `bpy.ops.render.render('INVOKE_DEFAULT')`, photographed the window and read `bpy.data.window_managers[0].reports`, then quit from a timer. On the rented Linux machine the whole screen was photographed with its own `shot.sh`; on the Creator's Mac, Blender's own `bpy.ops.screen.screenshot(filepath=...)` was used instead, so that nothing but Blender's own window was captured.

**For a missing image texture, Blender's own channels say nothing in 5.2 — and here that can be shown outright rather than read off a picture.** This build has `WindowManager.reports`, and the script read it after the render:

```
=== WindowManager.reports holds 0 entries
```

The Info editor was empty, no report box appeared along the bottom of the window, and the Blender Render window showed a magenta cube. Both machines gave the same.

**For a missing linked library the channels do speak.** `WindowManager.reports` held five entries, quoted here exactly as the script printed them:

```
=== WindowManager.reports holds 5 entries
=== REPORT type=INFO message="Read library: '<scene folder>/lib.blend', '//lib.blend', parent '<direct>'"
=== REPORT type=WARNING message="Unable to open '<scene folder>/lib.blend': No such file or directory"
=== REPORT type=INFO message="Cannot find lib '<scene folder>/lib.blend'"
=== REPORT type=INFO message="LIB: Collection: 'LibColl' missing from '<scene folder>/lib.blend', parent '<direct>'"
=== REPORT type=WARNING message='1 libraries and 1 linked data-blocks are missing (including 0 ObjectData), please check the Info and Outliner editors for details'
```

The Info editor listed the same five with a warning triangle. Blender 4.5, given exactly the same file on the same two machines, leaves the Info editor empty while printing all five lines in the terminal.

For a scene with no camera, the render operator raised the error back into the script that called it rather than reporting it anywhere, and `WindowManager.reports` held **0** entries:

```
=== RENDER INVOKE RAISED: RuntimeError('Error: No camera found in scene "Scene"\n')
=== WindowManager.reports holds 0 entries
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

2026-08-26 — Claude Opus 5 (Claude Code subagent) — Reports, Blender 5.2.

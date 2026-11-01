# Reports — Blender 5.2 — questions 12 and 13

**Sources consulted, and the date.** Blender 5.2 manual (`Blender Sources/manual_5.2`, including its `_images` folder), Blender 5.2 Python API documentation (`Blender Sources/api_5.2`), Blender 5.2 release notes (`Blender Sources/release_notes_5.2`), and Cycles' own files (`Blender Sources/cycles_5.2`). All were taken on 2026-07-30, and that is the date read for everything below.

I read only the question sheet and files inside Blender Sources/. I did not open any other part of this project.

Two notes on scope, so nothing here is read for more than it is:

- Only questions 12 and 13 are answered. I read the whole sheet first so I would not answer a neighbouring question by mistake.
- I worked alone. No helper agents were started, so there is nobody else's reading to vouch for.

---

## 12. What ending status does a run of Blender give back to whatever started it, for each of the ways a run can end — finishing normally, being interrupted, and being refused because something it was handed cannot be used?

**LABEL: not in the sources.**

For all three endings named in the question, the sources never state the number Blender hands back to the program that started it. There is no page in the manual, no page in the Python API documentation, and no release note for 5.2 that says what a normal finish returns, what an interrupted run returns, or what a run that was refused something returns. The plain words for this idea — exit code, exit status, return code, errorlevel, non-zero — appear in the whole of the 5.2 manual exactly twice, both in the same sentence about one narrow opt-in feature, quoted below. They appear nowhere in the API documentation and nowhere in the release notes.

I read the question this way: the "ending status" is the number the operating system hands to a parent program when Blender's process finishes — what a script that launched Blender would test. The question's own last sentence says exactly that, so I do not think there is a second reading. I did rule out the two things nearest to it that the sheet excludes: an operator's return value (`FINISHED`, `CANCELLED`, and so on, listed at https://docs.blender.org/api/5.2/bpy_types_enum_items/operator_return_items.html) is a value passed inside Blender between Python and Blender, not to the program that started Blender; and the messages in the Info editor and the console are things Blender prints about itself, which the question explicitly sets aside.

### What the sources do say, near this but not it

These four are the whole of what I found. None of them answers the question as asked, and I am listing them so the gap is visible rather than papering over it.

**(a) One opt-in exit code, for one narrow case — documented.**

> "--python-exit-code <code>"
>
> "Set the exit-code in [0..255] to exit if a Python exception is raised
> (only for scripts executed from the command line), zero disables."

Source: the manual, https://docs.blender.org/manual/en/5.2/advanced/command_line/arguments.html — Python Options. Date read: 2026-07-30.

This is a code the caller chooses and passes in, and it fires only when a Python script run from the command line raises an exception. It says nothing about a normal finish, an interruption, or a refused input. The page does not state what the setting is when the argument is not given.

**(b) A return value for a command, described as 0 or 1 — documented, but for the command's callback, not for the process.**

> "execute (Callable[[list[str]], int]) – Callback, taking a single list of strings and returns an int."
>
> "The return value should be 0 for success, 1 on failure (specific error codes from the os module can also be used)."

Source: the Python API documentation, https://docs.blender.org/api/5.2/bpy.utils.html — `bpy.utils.register_cli_command`. Date read: 2026-07-30.

This is the number a command registered for `-c` / `--command` gives back inside Blender. The page does not say that this number becomes the status the process hands to whatever started Blender. Saying it does would be a step the reader has to supply, and I am not taking it.

The `-c` / `--command` argument itself is described in the manual, and that description carries no status either:

> "-c, --command <command>"
>
> "Run a command which consumes all remaining arguments.
> Use -c help to list all other commands.
> Pass --help after the command to see its help text."
>
> "This implies --background mode."

Source: the manual, https://docs.blender.org/manual/en/5.2/advanced/command_line/arguments.html — Misc Options. Date read: 2026-07-30.

**(c) Cycles' own file calls Python's exit with 1 when a device override is not accepted — observed.**

In Cycles' own files, `engine.py`, lines 38 to 41, inside `_parse_command_line`:

```
    if args.cycles_device:
        import _cycles
        if not _cycles.set_device_override(args.cycles_device):
            sys.exit(1)
```

Source: `Blender Sources/cycles_5.2/engine.py`, lines 38–41. Date read: 2026-07-30.

This is the closest thing in anything I was given to "refused because something it was handed cannot be used". It is a device name given after `--` on the command line, not a file, and what the file shows is a call to Python's `sys.exit(1)` inside Blender's embedded Python when `set_device_override` gives back something other than true. Two steps are missing and I take neither. What makes that function give back something other than true is not in these lines — it sits in `_cycles`, which is not one of the files I was given — so I cannot say a device that cannot be used is what fires this path. And what the whole program then hands back is a further step beyond what the file says.

Earlier in the same file, at line 14, its option list names a separate error stream — see question 13:

> `help="Print rendering statistics to stderr",`

Source: `Blender Sources/cycles_5.2/engine.py`, line 14. Date read: 2026-07-30.

**(d) A Cycles command that gives back 1 after catching an error — observed.**

In Cycles' own files, `maketx.py`, lines 82 to 87, inside `maketx_file`:

```
    except RuntimeError as ex:
        sys.stderr.write("Error: {:s}\n".format(str(ex)))
        return 1

    print(out_filepath)
    return 0
```

Source: `Blender Sources/cycles_5.2/maketx.py`, lines 82–87. Date read: 2026-07-30.

This is the `--command maketx` sub-command, not a render. What the quoted lines show is that `maketx_file` gives back 1 when a `RuntimeError` has been caught, and 0 when it reaches its last line. The lines that raise that error are not in the quote, so what brings it about — an image file that cannot be used, or something else — is a step past what I can read here. Whether the number this one function gives back is the number Blender's command machinery receives is a second step, and whether Blender's process then finishes with 1 is a third — the last of those is exactly the thing the `register_cli_command` page in (b) does not say.

### Things that mention exiting but give no status at all

Recorded because they are where a status would most naturally have been written, and is not:

- "--debug-exit-on-error" — "Immediately exit when internal errors are detected." (manual, https://docs.blender.org/manual/en/5.2/advanced/command_line/arguments.html, Debug Options)
- "blender -ba test.blend" … "…will exit since -ba is an unknown argument." (same page, Argument Parsing)
- "-h, --help" / "-v, --version" / "--log-list-categories" / the four `--register` and `--unregister` arguments — each described as printing something "and exit", with no status. (same page)
- On being interrupted, the API documentation says this, about running Blender as a Python module rather than as the program: "Blender's typical signal handlers are not initialized, so there is no special handling for Control-C to cancel a render and a crash log is not written in the event of a crash." (https://docs.blender.org/api/5.2/info_advanced_blender_as_bpy.html, Signal Handlers). It tells you the module lacks the handling; it does not say what the program does with Control-C, and it gives no status.
- The manual's "Primitive Render Farm" hint (https://docs.blender.org/manual/en/5.2/render/output/properties/output.html) describes sharing work across machines by using placeholder frames and a shared folder. It never mentions checking what a run returned.

### Where I looked

Searched across the whole of `manual_5.2`, `api_5.2`, `release_notes_5.2` and `cycles_5.2` for, in this order: `exit code`, `exit status`, `return code`, `exit-code`, `errorlevel`, `nonzero`, `non-zero`, `sys.exit`, `returncode`, `subprocess`, `SIGINT`, `SIGTERM`, `Ctrl-C`, `Ctrl+C`, `signal`, `abort`, `status code`, `failure code`, `EXIT_FAILURE`, `EXIT_SUCCESS`, `exit(1)`, `exit(0)`, `os.EX_`, `waitpid`, `process exit`, `exited with`, `$?`, `ERRORLEVEL`, `shell script`, `batch script`, `render farm`, and the bare word `exit` across the manual and the API documentation with the many interface senses (exit Edit Mode, exit a node group, and so on) filtered out.

Pages opened in full or in part: the manual's whole Command Line section — https://docs.blender.org/manual/en/5.2/advanced/command_line/index.html, .../arguments.html, .../render.html, .../extension_arguments.html; https://docs.blender.org/manual/en/5.2/troubleshooting/startup.html; https://docs.blender.org/manual/en/5.2/render/output/properties/output.html; https://docs.blender.org/api/5.2/bpy.utils.html; https://docs.blender.org/api/5.2/info_advanced_blender_as_bpy.html; https://docs.blender.org/api/5.2/info_gotchas_crashes.html; https://docs.blender.org/api/5.2/bpy.app.html; https://docs.blender.org/api/5.2/bpy.types.Operator.html; and all twenty-one release note files for 5.2 searched together. In Cycles' own files I read `engine.py`, `maketx.py`, `operators.py` and the command registration in `__init__.py` in full.

Ruled out and why: `bpy.app` (https://docs.blender.org/api/5.2/bpy.app.html) has no exit-status member — its only nearby entry is a debug value "Integer value which can be set to non-zero values for testing purposes". The extension sub-commands page carries only argparse's stock "show this help message and exit" thirteen times, with no statuses. The release notes for 5.2 mention exiting three times, all in the bug-fix list and none about status.

**Not covered.** This does not settle what number Blender actually returns in any of the three cases — only that the 5.2 documentation does not say. It also does not settle whether `--python-exit-code` or a `--command` return value reaches the parent program, or what happens when several of these could apply at once.

**Outside the interface.** Everything above is already about running Blender from outside its own window: `--python-exit-code` is stated to apply "only for scripts executed from the command line", and `-c` / `--command` "implies --background mode". Nothing in the sources describes a different status inside the interface, because nothing in the sources describes a status at all. There is one documented difference for the neighbouring case of interruption, quoted in full above: driving Blender as a Python module rather than as the program leaves its usual signal handlers uninitialised, so Control-C has no special handling there.

> **Brought up to date on 2026-08-26 by hand test.** The status a run hands back was measured on two machines in this version, and all of it was measured from outside Blender's own window, which is the only place a status exists. See "Answered by hand test — question 12" at the end of this question's material.


**Operating system.** The sources do not say. None of the exit-related text is marked as differing by operating system. Four nearby arguments are marked as limited — `/?` is "(Windows only)", and the four register/unregister arguments are "(Windows & Linux only)" — but those are about the arguments existing, not about statuses. The Windows console is described as behaving differently from the Linux and macOS terminals (https://docs.blender.org/manual/en/5.2/advanced/command_line/launch/windows.html), again about display, not status.

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

The 5.2 manual, the 5.2 Python API documentation and the 5.2 release notes never separate one output stream from another. The words `stdout`, `stderr`, `standard output` in the stream sense, `standard error`, `output stream`, and `file descriptor` do not occur anywhere in any of the three. What the manual has instead is one undivided thing it calls the Console Window:

> "The Console Window, also called a Terminal, is an operating system text window that displays
> messages about Blender's operations, status, and internal errors."
>
> "When Blender is manually started from a terminal,
> Blender output is shown in the Console Window it is started from."

Source: the manual, https://docs.blender.org/manual/en/5.2/advanced/command_line/index.html. Date read: 2026-07-30.

> "The Blender Console Window can display many different types of status and error messages. Some messages simply
> inform the user what Blender is doing, but have no real impact on Blender's ability to function. Other messages can
> indicate serious errors that will most likely prevent Blender carrying out a particular task and may even make Blender
> non-responsive or shut down completely. The Blender Console Window messages can also originate internally from
> within the Blender code or from external sources such as Python scripts."

Source: the manual, https://docs.blender.org/manual/en/5.2/troubleshooting/startup.html — Common Startup Messages. Date read: 2026-07-30.

So the sources describe messages going to one place, and never say which of two streams any message goes to — for a missing file or for anything else.

**The question was ambiguous and I answered one reading.** "Output streams" can mean the two text streams a program writes to, standard output and standard error; or it can mean the picture streams a render produces. I answered the first, because the sheet's own closing note says question 13 "asks which stream carries what", which fits text streams and not pictures. I did not search for the second reading, so I have no record of looking for it and nothing below is an answer to it. It is left as a hole.

### What the sources do say about a file that is not there

None of this is about streams. It is here so the gap is visible and so nothing below is mistaken for an answer.

**Missing linked blend-file — documented.**

> "If Blender cannot find a library while loading a blend-file,
> it will create placeholder data-blocks to replace missing linked ones.
> That way, references to the missing data are not lost, and by relocating the missing library,
> the lost data can be automatically restored."

Source: the manual, https://docs.blender.org/manual/en/5.2/files/linked_libraries/link_append.html — Broken Libraries. Date read: 2026-07-30.

This says what Blender builds in memory. It does not say anything is printed.

**Checking for missing files by hand — documented, and it points away from the console.**

> "Report Missing Files"
>
> "This option is useful to check if there are links to unpacked files that no longer exist.
> After selecting this option, a warning message will appear in the Info editor's header.
> If no warning is shown, there are no missing external files."

Source: the manual, https://docs.blender.org/manual/en/5.2/interface/window_system/topbar.html — File menu, External Data. Date read: 2026-07-30.

The named destination here is the Info editor's header, which is inside Blender's window, not a stream. The manual does not say where this message goes when there is no window.

**Missing footage in the video sequencer — documented, and it is about pixels, not streams.**

> "Show Missing Media"
>
> "Render missing images/movies with a solid magenta color.
> When disabled, missing content will render fully transparent."

Source: the manual, https://docs.blender.org/manual/en/5.2/editors/video_sequencer/preview/sidebar.html. Date read: 2026-07-30.

**The one place in anything I was given where a stream split is visible — observed, and it is not a render.**

Cycles' own files contain one command that handles a file it cannot use, and it does split its two kinds of message. In `maketx.py`, lines 82 to 87, inside `maketx_file`:

```
    except RuntimeError as ex:
        sys.stderr.write("Error: {:s}\n".format(str(ex)))
        return 1

    print(out_filepath)
    return 0
```

Source: `Blender Sources/cycles_5.2/maketx.py`, lines 82–87. Date read: 2026-07-30.

What the file says, and no more: the failure text, prefixed `Error: ` and ending in a newline, is written to the error stream by name; the success text, the output file path, is written with `print`. This is the `--command maketx` sub-command described in the 5.2 release notes ("To generate Cycles native tx files from the command line, the following commands as provided", https://developer.blender.org/docs/release_notes/5.2/cycles/), and the file it is handed is one named on the command line. It is not a render, and the file is not one the scene points at. I am not carrying this across to renders.

One other place in Cycles' own files names the error stream, and it is help text rather than a write:

> `help="Print rendering statistics to stderr",`

Source: `Blender Sources/cycles_5.2/engine.py`, line 14 — the help text for `--cycles-print-stats`. Date read: 2026-07-30.

The manual's own description of that same argument does not name a stream: "--cycles-print-stats" / "Log statistics about render memory and time usage." (https://docs.blender.org/manual/en/5.2/advanced/command_line/arguments.html, Cycles Render Options). So on this one point Cycles' own file is more specific than the manual, and the stream it names is the error one — in the description of an option, for statistics, not for missing files. What a run does with that option is a step past the help text, and I am not taking it.

**A picture I opened.** `Blender Sources/manual_5.2/_images/advanced_command-line_launch_linux_console.png`, shown on https://docs.blender.org/manual/en/5.2/advanced/command_line/launch/linux.html. It is a screenshot of a terminal just after `blender` was run. Visible in it are seven lines of start-up messages about a sound server that could not be reached.

> **Corrected on 2026-08-26 by the citation check.** The count read "eight lines". The picture was opened again and holds seven. Nothing else in the description changed.
 Documented, from the picture: Blender's console output can contain error text from things it depends on, mixed in with everything else. The picture cannot show which stream any line came from, and it shows nothing about missing scene files, so I take nothing further from it.

### Where I looked

Searched the whole of `manual_5.2`, `api_5.2`, `release_notes_5.2` and `cycles_5.2` for: `stdout`, `stderr`, `standard output`, `standard error`, `std::cout`, `std::cerr`, `output stream`, `file descriptor`, `redirect`, `pipe`, `2>&1`, `sys.stdout`, `sys.stderr`, `console`. Then, for the condition itself: `missing file`, `missing files`, `find missing`, `report missing`, `missing librar`, `missing image`, `missing texture`, `missing data`, `broken link`, `placeholder`, `cannot open`, `can't open`, `unable to open`, `unable to load`, `failed to load`, `could not`, `does not exist`, `no longer exist`, `cannot be found`, `can not be found`, `file is missing`, `not found`.

Pages opened: https://docs.blender.org/manual/en/5.2/advanced/command_line/index.html and the pages under it (arguments, render, extension_arguments, and the Linux/macOS/Windows launch pages); https://docs.blender.org/manual/en/5.2/troubleshooting/startup.html; https://docs.blender.org/manual/en/5.2/interface/window_system/topbar.html; https://docs.blender.org/manual/en/5.2/files/linked_libraries/link_append.html; https://docs.blender.org/manual/en/5.2/render/output/properties/output.html; https://docs.blender.org/manual/en/5.2/editors/video_sequencer/preview/sidebar.html; https://docs.blender.org/api/5.2/bpy.ops.file.html (`find_missing_files`); https://docs.blender.org/api/5.2/bpy.types.Operator.html (the `report` method); https://docs.blender.org/api/5.2/bpy.types.ID.html (`is_missing`); and all twenty-one 5.2 release note files searched together. In Cycles' own files I read `engine.py`, `maketx.py`, `operators.py` and `__init__.py`.

Ruled out and why. `bpy.ops.file.find_missing_files` (https://docs.blender.org/api/5.2/bpy.ops.file.html) is described only as "Try to find missing external files", with a list of its file-browser arguments and no mention of output. `bpy.types.ID.is_missing` (https://docs.blender.org/api/5.2/bpy.types.ID.html) describes the placeholder data-block — "This data-block is a place-holder for missing linked data (i.e. it is [an override of] a linked data that could not be found anymore)" — and again says nothing about printing. `Operator.report` (https://docs.blender.org/api/5.2/bpy.types.Operator.html) documents only its two arguments, with no destination named. Cycles' `operators.py` does report a missing file — "Frame '%s' not found, animation must be complete" at lines 84 to 87 — but that is the Denoise Animation operator checking rendered frames it was asked to denoise, not a render reading a file the scene points at, and it hands the text to `self.report`, which names no stream. The 5.2 release notes mention missing files only in the bug-fix list, in entries about crashes, with no description of output.

**Not covered.** This does not settle what a render actually writes, or where, when a file the scene points at is missing — only that the 5.2 documentation does not divide its output into streams anywhere, and so cannot answer the question as asked. It also does not settle whether the split seen in Cycles' `maketx` command reflects what a render does; the two are different code paths and I found nothing joining them. And it does not touch the second reading of the question — what a render's picture outputs hold when a file the scene points at is missing — which I did not search for at all.

**Outside the interface.** Not settled by the sources. The console text quoted above is described for a Blender started from a terminal, which is the outside-the-interface case, but no stream is named there either. The one documented difference I did find runs the other way: the Report Missing Files message is stated to appear "in the Info editor's header", which exists only when Blender has a window; the sources do not say what happens to that message when Blender is run with `-b`.

> **Brought up to date on 2026-08-26 by hand test.** Both streams were captured separately, in this version, on two machines, for five kinds of missing file, and the same files were then opened with a real window. See "Answered by hand test — question 13" at the end of this question's material.


**Operating system.** The sources do not say. Nothing in the missing-file text or the console text is marked as differing by operating system. The manual does describe the Windows console as behaving differently from the Linux and macOS terminals — it opens alongside the main window and is then toggled off, and "To display the console again, go to Window ‣ Toggle System Console" (https://docs.blender.org/manual/en/5.2/advanced/command_line/launch/windows.html) — but that is about where the text is shown, not about which stream carries it.

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

2026-08-25 — Claude Opus 5 (Claude Code subagent) — Reports, Blender 5.2.

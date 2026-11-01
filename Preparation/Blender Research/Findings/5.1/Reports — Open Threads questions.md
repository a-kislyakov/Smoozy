# Reports — Open Threads questions (Blender 5.1)

**Sources consulted, and the date:** Blender 5.1 manual (`manual_5.1`), Blender 5.1 Python API documentation (`api_5.1`), Blender 5.1 release notes (`release_notes_5.1`), and the Cycles add-on files supplied for 5.1 (`cycles_5.1`) — all read on 2026-07-30 (the snapshot date given).

I read only the question sheet and files inside Blender Sources/. I did not open any other part of this project.

Question answered: 14 only.

---

## 14. When a render's printed output is sent somewhere other than a terminal, what decides when each line of it becomes available to whatever is reading?

**Answer: not in the sources.**

**LABEL: not in the sources**

Nothing in the 5.1 manual, the 5.1 Python API documentation, the 5.1 release notes, or the Cycles files supplied says anything about *when* a printed line reaches a reader that is not a terminal. The sources talk about *where* output goes and *what* is printed. They never discuss the timing of its arrival, and they never distinguish a terminal from a file or from another program reading the output.

The closest the sources come, and why none of it answers the question:

- The manual's command-line pages describe output only as something shown in a terminal window. The opening page says: "The Console Window, also called a Terminal, is an operating system text window that displays messages about Blender's operations, status, and internal errors." and "When Blender is manually started from a terminal, Blender output is shown in the Console Window it is started from." Both sentences describe the terminal case only. Neither says what happens when the output goes elsewhere, and neither mentions timing.
  Source: manual, https://docs.blender.org/manual/en/5.1/advanced/command_line/index.html — read 2026-07-30.

- There is a command-line option for sending log output to a file. The whole of what the manual says about it is: "--log-file <filepath>" / "Set a file to output the log to." That names a destination that is not a terminal, which is the nearest thing in the sources to the situation the question describes — but it says nothing about when a line written that way becomes readable.
  Source: manual, https://docs.blender.org/manual/en/5.1/advanced/command_line/arguments.html — read 2026-07-30.

- There is an option that changes how much is printed, not when: "-q, --quiet" / "Suppress status printing (warnings & errors are still printed)."
  Source: manual, https://docs.blender.org/manual/en/5.1/advanced/command_line/arguments.html — read 2026-07-30.

- The Cycles files name one stream by name. In the add-on's own argument parser: `parser.add_argument("--cycles-print-stats",` / `help="Print rendering statistics to stderr",`. What these words show is that the option exists under that name and that its own help text says the statistics are printed to the error stream. What a render actually does when that option is used is a step beyond what reading a file can show, so nothing about that is claimed here. Either way, nothing here is about buffering or timing.
  Source: Cycles files supplied, file `cycles_5.1/engine.py`, lines 13–14, inside the function `_configure_argument_parser()` — read 2026-07-30. (This one is **observed** — it is a line in a file I was given, not documentation.)

- The manual's page on video output mentions the console as somewhere output appears: "When you view the System Console, you can see some of the output of the encoding process. You will see even more output if you execute Blender as blender -d." Again, a terminal, and no timing.
  Source: manual, https://docs.blender.org/manual/en/5.1/render/output/properties/output.html — read 2026-07-30.

None of these state, or let anyone derive, what decides when a printed line becomes available to a non-terminal reader. I could give a mechanism from general knowledge of how programs write output, but that would not be from these sources, so I am not writing it.

**Where I looked, and what I searched for**

Searched across the whole of `manual_5.1`, `api_5.1`, `release_notes_5.1` and `cycles_5.1` at once (case-insensitive, whole-tree text search), for each of these terms:

- The plain and technical words for the mechanism: `buffer`, `buffering`, `unbuffered`, `line buffer`, `line-buffer`, `block buffer`, `flush`, `setvbuf`, `fflush`, `isatty`, `PYTHONUNBUFFERED`. Every one of these returned either nothing at all, or only unrelated matches — `buffer` hits are frame buffers, Z-buffers, double buffering, render buffers, and the macOS `nano` instruction "Enter Y to save the modified buffer"; `flush` hits are selection flushing in `bmesh` and animation-editor flushing. There is not one hit anywhere about output buffering.
- The words for the streams: `stdout`, `stderr`. Zero hits in the manual. Zero hits in the API documentation. Zero in the release notes. One hit in the Cycles files — the `--cycles-print-stats` help text quoted above.
- The words for sending output elsewhere: `redirect`, `piping`, `pipe the`, `capture the output`, `output to a file`, `log to a file`, `log file`, `log-file`, `2>&1`, `tee`. Only two of these hit anything relevant: `--log-file` in the arguments page (quoted above), and a `2>&1` that appears in the manual's own translation-build instructions, which is about building the manual, not about Blender's output.
- The words for the situation: `terminal`, `console`, `tty`, `not a terminal`, `interactive`. The `terminal` and `console` hits are the command-line pages, the Python-console editor, and scattered mentions of seeing messages in a console. `tty` matched only unrelated words containing those letters. Nothing about a non-terminal destination.
- The words for timing: `real time`, `as it happens`, `immediately`, `line by line`, `chunk`, `appears in the console`, `printed to the console`, `printed to the terminal`, `written to the console`. Nothing relevant. The only near-hit is the macOS launch page's "to get all output printed to the terminal", which is about which executable you run, not about timing.
- Also searched for the shape of a render's progress line (`Fra:`) and for `carriage`, in case the sources described a line that is rewritten in place — which would bear on what a non-terminal reader sees. Zero hits for both.

Pages I opened in full and ruled out:

- https://docs.blender.org/manual/en/5.1/advanced/command_line/index.html — describes the console window and the use cases for the command line. No timing, no non-terminal destination.
- https://docs.blender.org/manual/en/5.1/advanced/command_line/arguments.html — the complete argument list, including the Logging Options (`--log`, `--log-level`, `--log-show-memory`, `--log-show-source`, `--log-show-backtrace`, `--log-file`, `--log-list-categories`), the Debug Options, the Misc Options, and the full list of environment variables Blender reads. No argument and no environment variable in 5.1 concerns output buffering or flushing. The environment-variable list covers only resource directories, the 3D-mouse socket, the colour-management config, the splash image, and the temporary directory.
- https://docs.blender.org/manual/en/5.1/advanced/command_line/render.html — rendering from the command line, single image and animation. Argument ordering, nothing about output timing.
- https://docs.blender.org/manual/en/5.1/advanced/command_line/launch/linux.html, .../macos.html, .../windows.html — all three read in full. All three describe starting Blender from a terminal and seeing output there. None mentions any other destination.
- https://docs.blender.org/manual/en/5.1/troubleshooting/crash.html — crash logs and the debug-log batch scripts. Nothing about output timing.
- https://docs.blender.org/api/5.1/info_tips_and_tricks.html — including its "Use the Terminal" and "Use Blender without its User Interface" sections, which are the API documentation's advice on running Blender headless and checking its output. Its suggestions for checking output are to render an image, save a blend-file, or "print them or write them to a file". No mention of when printed text arrives.
- https://docs.blender.org/api/5.1/info_advanced_blender_as_bpy.html — read for its notes on background-mode equivalence. Nothing on output timing.
- The API gotcha pages (`info_gotcha`, `info_gotchas_crashes`, `info_gotchas_threading`, `info_gotchas_file_paths_and_encoding`) — searched for the stream and buffering words; only ordinary `print()` examples.
- The 5.1 release notes, all files — searched for `log`, `logging`, `output`, `stream`, `console`, `terminal`, `buffer`. The hits are about compositor output nodes, video and audio output, framebuffers, and bug-fix titles. No note in 5.1 changes anything about how printed output is written or flushed.
- The Cycles files supplied (`cycles_5.1`). These are the add-on's Python files plus kernel and utility headers, shaders, and licences. I checked what is there: nine Python files and, under `source`, only `.h`, `.oso`, `.cu`, `.metal` and two `.cpp` kernel-entry files. There is no session, progress-reporting or console-printing code in what was supplied. Searching all of it for `printf`, `std::cout`, `fprintf`, `stdout`, `stderr` returns only maths and type headers using `printf`-style formatting macros, plus the one `engine.py` line quoted above. So the Cycles files cannot settle this question either.

I did not treat any manual image as evidence for this question. A picture shows what a window looked like at one moment, and when a printed line reaches a reader is not something a picture can show. I make no claim about what the images on those pages do or do not depict, not having opened them.

---

### Answered by hand test, 2026-08-30

**LABEL: observed.** Run by Claude Opus 5 (Claude Code) on 2026-08-30, in Blender 5.1.2, on two machines. The paragraphs above still stand: the sources say nothing. What follows is what a running Blender did. The full list, with every command written out and the raw output of every run, is `Audits/Blender Research/Reports/Hand tests — Open Threads questions.md`.

**The two machines, and the answers are never merged.**

- **The Creator's own Mac** — a MacBook Pro with an Apple M1 Pro chip, eight processor cores, 16 GB of memory, macOS 26.6.2, Apple's own graphics and no second card. Blender 5.1.2 at `/Applications/Blender 5.1.2.app/Contents/MacOS/Blender`, build hash `ec6e62d40fa9`.
- **A rented Linux machine** — Vast.ai instance `49254257` in the United Kingdom, a container rather than a whole machine, Ubuntu 22.04.4 LTS on kernel `6.8.0-124-generic`, an Intel Core i7-11800H with sixteen visible threads against a quota of 15.36 cores, a memory ceiling of about 15.8 GB, one NVIDIA GeForce RTX 3070 Laptop GPU on driver 535.309.01 that no test used, 32 GB of disk, and no display server. Blender 5.1.2 at `/opt/blender/blender-5.1.2-linux-x64/blender`, taken from Blender's own download server as the Linux x64 build, build hash `ec6e62d40fa9` — the same build as the Mac's. Rented and destroyed the same day.

**What was set up.** One file, `scene.blend`: the factory startup scene — one cube, one lamp, one camera — set to Cycles on the processor, 900 samples, adaptive sampling and denoising off, 720 by 405 at 100 per cent, frames 1 to 5, saving PNG files to `/tmp/r14out/f_`. It was built with Blender 4.5.12 so that every version opens it without converting anything. One frame takes about fifteen seconds on either machine. Every run carried `--factory-startup`, both to leave the Creator's own settings alone and because the Mac has third-party add-ons whose messages would otherwise have been read as Blender's own.

**How arrival was measured.** A reader that looks only after a run has ended cannot tell a line held back from a line printed promptly, so a small program (`reader.py`) was put on the far end of the pipe. It reads raw bytes as they arrive and prints, for each arrival, the seconds since it started, how many bytes arrived, and the first seventy bytes of them. Promptly-arriving output shows as many small arrivals timed to the run's own events; held-back output shows as few arrivals of one fixed size at long intervals. **That the instrument can show the second pattern was proved on both machines**, by putting a program between Blender and the reader that deliberately held everything in 4096-byte blocks. That calibration was run with Blender 4.5.12, since it measures the reader rather than Blender. On the Mac it gave four arrivals, at 0.559, 24.487, 46.278 and 75.931 seconds, of 4096, 4096, 4096 and 2709 bytes; on the rented Linux machine, four arrivals at 0.256, 18.103, 36.024 and 54.115 seconds, of 4096, 4096, 4096 and 2599 bytes.

**1. A render's own printed lines reach a pipe as they are produced.** Command, on both machines: `<blender> -b --factory-startup scene.blend -s 1 -e 3 -a 2>&1 | python3 reader.py`. On the Mac, 63 arrivals over 75.746 seconds. The last eight, word for word:

```
  52.377  chunk  57      62 bytes  total   10449  b'00:52.010  render           | Fra: 3 | Mem: 1M | Sample 0/900\n'
  52.418  chunk  58      84 bytes  total   10533  b'00:52.051  render           | Fra: 3 | Remaining: 00:36.64 | Mem: 5M |'
  66.248  chunk  59      86 bytes  total   10619  b'01:05.881  render           | Fra: 3 | Remaining: 00:10.46 | Mem: 5M |'
  75.663  chunk  60     122 bytes  total   10741  b'01:15.296  render           | Fra: 3 | Mem: 5M | Sample 900/900\n01:15.'
  75.681  chunk  61      62 bytes  total   10803  b"01:15.314  render           | Saved: '/tmp/r14out/f_0003.png'\n"
  75.681  chunk  62      64 bytes  total   10867  b'01:15.314  render           | Time: 00:23.30 (Saving: 00:00.01)\n'
  75.740  chunk  63      14 bytes  total   10881  b'\nBlender quit\n'
  75.746  END  63 arrivals, 10881 bytes total
```

The 56 arrivals before these are cut for length; they are the same shape, arriving in bursts as each frame finishes. On the rented Linux machine, 49 arrivals over 52.560 seconds, ending:

```
  52.485  chunk  46      64 bytes  total   10487  b'00:52.420  render           | Fra: 3 | Mem: 5M | Sample 900/900\n'
  52.486  chunk  47      58 bytes  total   10545  b'00:52.420  render           | Fra: 3 | Mem: 5M | Finished\n'
  52.510  chunk  48     126 bytes  total   10671  b"00:52.445  render           | Saved: '/tmp/r14out/f_0003.png'\n00:52.44"
  52.549  chunk  49      14 bytes  total   10685  b'\nBlender quit\n'
  52.560  END  49 arrivals, 10685 bytes total
```

**The two machines agreed.**

**2. A pipe is no different from a terminal for those lines.** The same render was run again with a real terminal in place of the pipe, supplied by the system's own `script` command. On the Mac, 46 arrivals ending at 65.971 seconds; on the rented Linux machine, 86 ending at 51.409. The arrival pattern is the same as in test 1 on each machine. The only difference in what arrived is that every line ends `\r\n` instead of `\n`, which is the terminal's doing. The last four arrivals on the Mac:

```
  65.879  chunk  44     124 bytes  total   10801  b'01:05.648  render           | Fra: 3 | Mem: 5M | Sample 900/900\r\n01:05'
  65.907  chunk  45     128 bytes  total   10929  b"01:05.676  render           | Saved: '/tmp/r14out/f_0003.png'\r\n01:05.6"
  65.961  chunk  46      16 bytes  total   10945  b'\r\nBlender quit\r\n'
  65.971  END  46 arrivals, 10945 bytes total
```

**3. The same lines reach a plain file as they are produced.** Command: `<blender> -b --factory-startup scene.blend -s 1 -e 3 -a > out.txt 2>&1`, with the file's size read every quarter second from a second process while the render was still going. On the Mac:

```
   0.000  file is       0 bytes
   1.024  file is    3543 bytes
  31.583  file is    7038 bytes
  64.418  file is   10449 bytes
  64.671  file is   10533 bytes
  88.021  file is   10619 bytes
 102.377  file is   10881 bytes
 102.378  run finished, file is   10881 bytes
```

On the rented Linux machine:

```
   0.000  file is       0 bytes
   0.501  file is    3433 bytes
  17.537  file is    6928 bytes
  34.572  file is   10423 bytes
  51.601  file is   10685 bytes
  51.851  run finished, file is   10685 bytes
```

**The steps are the frames.** The file grows once per frame, at the moment that frame finishes.

**4. The progress readout is whole separate lines, not one line rewritten.** Counting the line endings in the file test 3 wrote: on the Mac, `newlines: 147   carriage returns: 0   bytes: 10881`; on the rented Linux machine, `newlines: 146   carriage returns: 0   bytes: 10685`. No carriage returns at all.

**5. A Python script's own printing does not reach a pipe or a file until the run ends, and this is the one place the answer splits.** A script printing one line a second for fifteen seconds (`print("tick %d" % i)` and then a one-second wait) was run with `<blender> -b --factory-startup --python ticker.py 2>&1 | python3 reader.py`. On the Mac:

```
  15.711  chunk   1     111 bytes  total     111  b'tick 1\ntick 2\ntick 3\ntick 4\ntick 5\ntick 6\ntick 7\ntick 8\ntick 9\ntick 10'
  15.745  chunk   2      60 bytes  total     171  b'Blender 5.1.2 (hash ec6e62d40fa9 built 2026-05-19 01:30:33)\n'
  15.749  chunk   3      14 bytes  total     185  b'\nBlender quit\n'
  15.753  END  3 arrivals, 185 bytes total
```

All fifteen lines arrived together at the end, in one arrival. On the rented Linux machine:

```
  15.316  chunk   1     111 bytes  total     111  b'tick 1\ntick 2\ntick 3\ntick 4\ntick 5\ntick 6\ntick 7\ntick 8\ntick 9\ntick 10'
  15.349  chunk   2      60 bytes  total     171  b'Blender 5.1.2 (hash ec6e62d40fa9 built 2026-05-19 02:18:37)\n'
  15.352  chunk   3      14 bytes  total     185  b'\nBlender quit\n'
  15.360  END  3 arrivals, 185 bytes total
```

Sent to a file instead of a pipe, the file stayed at zero bytes throughout and reached its full size in one step at the end — on the Mac:

```
   0.000  file is       0 bytes
  15.911  file is     111 bytes
  16.162  file is     185 bytes
  16.162  run finished, file is     185 bytes
```

and on the rented Linux machine:

```
   0.000  file is       0 bytes
  15.280  file is     111 bytes
  15.531  file is     185 bytes
  15.531  run finished, file is     185 bytes
```

Three further runs on each machine:

- **With `PYTHONUNBUFFERED=1` set**, the result was unchanged. The first arrival on the Mac was `15.654  chunk   1     111 bytes  total     111  b'tick 1\ntick 2\ntick 3\ntick 4\ntick 5\ntick 6\ntick 7\ntick 8\ntick 9\ntick 10'` and on Linux `15.290  chunk   1     111 bytes  total     111  b'tick 1\ntick 2\ntick 3\ntick 4\ntick 5\ntick 6\ntick 7\ntick 8\ntick 9\ntick 10'`.
- **With each line printed as `print("tick %d" % i, flush=True)`**, the lines arrived one a second through the same pipe. The first four arrivals on the Mac — the remaining eleven follow the same one-a-second pattern and are cut for length:

  ```
     0.753  chunk   1       7 bytes  total       7  b'tick 1\n'
     1.755  chunk   2       7 bytes  total      14  b'tick 2\n'
     2.765  chunk   3       7 bytes  total      21  b'tick 3\n'
     3.766  chunk   4       7 bytes  total      28  b'tick 4\n'
  ```

  On the rented Linux machine the first four were:

  ```
     0.267  chunk   1       7 bytes  total       7  b'tick 1\n'
     1.267  chunk   2       7 bytes  total      14  b'tick 2\n'
     2.267  chunk   3       7 bytes  total      21  b'tick 3\n'
     3.268  chunk   4       7 bytes  total      28  b'tick 4\n'
  ```

- **With a real terminal in place of the pipe**, the plain script, with no flushing, also ticked once a second, and the version banner arrived first rather than last. The first four arrivals on the Mac:

  ```
     0.000  chunk   1       4 bytes  total       4  b'^D\x08\x08'
     0.256  chunk   2      61 bytes  total      65  b'Blender 5.1.2 (hash ec6e62d40fa9 built 2026-05-19 01:30:33)\r\n'
     0.681  chunk   3       8 bytes  total      73  b'tick 1\r\n'
     1.686  chunk   4       8 bytes  total      81  b'tick 2\r\n'
  ```

  And on the rented Linux machine:

  ```
     0.070  chunk   1      61 bytes  total      61  b'Blender 5.1.2 (hash ec6e62d40fa9 built 2026-05-19 02:18:37)\r\n'
     0.271  chunk   2       8 bytes  total      69  b'tick 1\r\n'
     1.272  chunk   3       8 bytes  total      77  b'tick 2\r\n'
     2.272  chunk   4       8 bytes  total      85  b'tick 3\r\n'
  ```

**6. Everything a render prints goes to the ordinary output stream; the error stream carried nothing.** With only the error stream fed to the reader — `<blender> -b --factory-startup scene.blend -f 1 -- --cycles-print-stats 2>&1 1>/dev/null | python3 reader.py` — the whole run gave, on the Mac, `19.010  END  0 arrivals, 0 bytes total`, and on the rented Linux machine, `17.539  END  0 arrivals, 0 bytes total`. Keeping the two streams apart in files, `<blender> -b --factory-startup scene.blend -f 1 -- --cycles-print-stats > s_out.txt 2> s_err.txt`, gave on the Mac `ordinary output: 7017 bytes    error stream: 0 bytes` and on Linux `ordinary output: 6907 bytes    error stream: 0 bytes`. **The Cycles statistics were in the ordinary output**, under headings reading `Update statistics:`, `Render statistics:` and `Mesh statistics:` — not in the error stream, which is where the help text quoted above says they go.

**7. The file written by `--log-file` fills in as the run goes.** Command: `<blender> -b --factory-startup scene.blend --log "*" --log-level info --log-file run.log -s 1 -e 2 -a > /dev/null 2>&1`. The logging-level value was read off `blender --help` rather than guessed. On the Mac:

```
   0.000  file is       0 bytes
   0.520  file is      64 bytes
   0.780  file is    6364 bytes
  20.021  file is    8163 bytes
  20.275  file is   13819 bytes
  46.143  file is   15745 bytes
  46.143  run finished, file is   15745 bytes
```

On the rented Linux machine:

```
   0.000  file is       0 bytes
   0.250  file is      64 bytes
   0.503  file is    6014 bytes
  16.534  file is    7842 bytes
  16.785  file is   13494 bytes
  33.070  file is   15322 bytes
  33.320  file is   15448 bytes
  33.321  run finished, file is   15448 bytes
```

**8. The same split shown inside one run.** So that nothing could be blamed on two runs differing, one run printed both kinds of line into one pipe: `<blender> -b --factory-startup scene.blend --python-expr "print('SCRIPT LINE PRINTED AT START')" -s 1 -e 2 -a 2>&1 | python3 reader.py`. On the Mac the render's own lines arrived throughout, and:

```
   0.563  chunk   1     172 bytes  total     172  b'00:00.341  blend            | Read blend: "/private/tmp/claude-501/-Us'
  32.296  chunk  24     126 bytes  total    3791  b"00:32.073  render           | Saved: '/tmp/r14out/f_0001.png'\n00:32.07"
  64.666  chunk  47     126 bytes  total    7286  b"01:04.443  render           | Saved: '/tmp/r14out/f_0002.png'\n01:04.44"
  64.672  chunk  48      29 bytes  total    7315  b'SCRIPT LINE PRINTED AT START\n'
  64.711  END  49 arrivals, 7329 bytes total
```

The arrivals not shown are the render's own, spread across the run. The script's line was printed within the first second and reached the reader at 64.672 seconds, after both frames were finished. **This run was done on the Mac only for 5.1** — the rented Linux machine had already been destroyed when it was added. The same run in 4.5 and in 5.2 was done on both machines, and the two machines agreed there.

**9. Nothing already printed is lost when a run is stopped part-way.** A three-frame render was sent to a file and stopped at twenty-two seconds, once with an interrupt and once with an outright kill, which cannot flush anything a program is still holding. On the Mac, before the interrupt the file held 3543 bytes ending `00:00.467  render           | Fra: 1 | Remaining: 00:16.30 | Mem: 5M | Sample 1/900`; afterwards 3800 bytes ending:

```
00:28.808  render           | Fra: 1 | Mem: 5M | Cancelled

Blender quit
```

Before the outright kill the file held 3543 bytes; afterwards it held **the same 3543 bytes**, ending on the most recent line the run had printed:

```
00:00.370  render           | Fra: 1 | Mem: 1M | Updating Device | Writing constant memory
00:00.370  render           | Fra: 1 | Mem: 1M | Sample 0/900
00:00.384  render           | Fra: 1 | Remaining: 00:13.12 | Mem: 5M | Sample 1/900
```

Nothing was lost. **This one was run on the Mac only for 5.1**, for the same reason as test 8. In 4.5 and 5.2 it was run on both machines and nothing was lost on either.

**10. The build's own help list names no switch for any of this.** Running `blender --help` and searching the whole printed list for `buffer`, `flush`, `unbuffer`, `stdout` and `stderr` gives exactly one hit in Blender 5.1.2: `Read from disk (Do not buffer).`, which is the animation player's `-m` argument and concerns reading picture files, not writing output. The same single hit, and nothing else, in the other three versions. So the installed build lists nothing the manual's argument page omits.

**11. Two things change the timing of a script's own lines, and one of them only works with the other.** Five more runs, on the Creator's Mac, in Blender 4.5.12 and Blender 5.2.1. All five used the same ticking script and the same pipe as test 5.

- **`--python-use-system-env` on its own changed nothing.** In 4.5.12:
  ```
    15.552  chunk   1     111 bytes  total     111  b'tick 1\ntick 2\ntick 3\ntick 4\ntick 5\ntick 6\ntick 7\ntick 8\ntick 9\ntick 10'
    15.578  chunk   2      65 bytes  total     176  b'Blender 4.5.12 LTS (hash 84afd5f785f7 built 2026-07-21 01:41:35)\n'
    15.582  chunk   3      14 bytes  total     190  b'\nBlender quit\n'
    15.587  END  3 arrivals, 190 bytes total
  ```
  In 5.2.1 the same, ending `16.111  END  3 arrivals, 189 bytes total`.

- **`PYTHONUNBUFFERED=1` on its own changed nothing** — that is test 5 above.

- **The two together made the fifteen lines arrive one a second.** Command: `PYTHONUNBUFFERED=1 <blender> -b --factory-startup --python-use-system-env --python ticker.py 2>&1 | python3 reader.py`. In 4.5.12, the first five arrivals — the remaining twelve follow the same one-a-second pattern and are cut for length:
  ```
     0.582  chunk   1      65 bytes  total      65  b'Blender 4.5.12 LTS (hash 84afd5f785f7 built 2026-07-21 01:41:35)\n'
     0.756  chunk   2       7 bytes  total      72  b'tick 1\n'
     1.766  chunk   3       7 bytes  total      79  b'tick 2\n'
     2.775  chunk   4       7 bytes  total      86  b'tick 3\n'
     3.786  chunk   5       7 bytes  total      93  b'tick 4\n'
  ```
  In 5.2.1 the same, first tick at 0.704 seconds and last at 14.803, `END  17 arrivals, 189 bytes total`. **The version banner also moved from last to first** in these runs.

- **Putting `stdbuf` in front of the whole command did not change the script's lines, but did move the banner.** Command: `stdbuf -o 0 <blender> -b --factory-startup --python ticker.py 2>&1 | python3 reader.py`. In 4.5.12:
  ```
     0.179  chunk   1      65 bytes  total      65  b'Blender 4.5.12 LTS (hash 84afd5f785f7 built 2026-07-21 01:41:35)\n'
    15.575  chunk   2     111 bytes  total     176  b'tick 1\ntick 2\ntick 3\ntick 4\ntick 5\ntick 6\ntick 7\ntick 8\ntick 9\ntick 10'
    15.623  chunk   3      14 bytes  total     190  b'\nBlender quit\n'
    15.627  END  3 arrivals, 190 bytes total
  ```
  **The `stdbuf -o L` runs and the 5.2.1 runs, each one on its own.** *(Corrected 2026-08-30. This bullet used to end with one sentence: "The same with `stdbuf -o L` and Blender 5.2.1: banner at 0.314 seconds, all fifteen ticks together at 15.841." That single pair of times stood for at least three separate runs — `-o L` in 4.5.12, `-o 0` in 5.2.1 and `-o L` in 5.2.1 — and did not say which run produced it, so it could not be repeated as written. The citation check of 2026-08-30 found it and wrote a replacement test out in full; it was run as entry 12 of `Audits/Blender Research/Reports/Hand tests — Open Threads questions.md` and what follows replaces that sentence. **LABEL: observed.**)*

  **On what kind of computer, and what it does not cover.** The Creator's own Mac — a MacBook Pro with an Apple M1 Pro chip, macOS 26.6.2, Apple's own graphics and no second card — with the four Blenders read off the machine that day at build hashes `84afd5f785f7`, `a3db93c5b259`, `ec6e62d40fa9` and `9e2066aef7ef`. **These runs say nothing about Linux**: the rented Linux machine that served the earlier entries had been destroyed before they were made. They say nothing about Windows either. **And other agents were running Blender on that Mac throughout that day, so the elapsed seconds run long and no timing taken on it on 2026-08-30 is a quiet machine's timing.** What the runs turn on is whether the fifteen lines arrive during the run or all at its end, which a loaded machine shows as plainly as an idle one.

  **Twelve runs, three in each version, each with its own times.** Every run is `<blender> -b --factory-startup --python ticker.py 2>&1 | python3 reader.py`, with `stdbuf -o 0` or `stdbuf -o L` in front where named. The run marked *no wrapper* has nothing in front of it and is what the other two are measured against.

  - 4.5.12, no wrapper — all fifteen ticks in one arrival of 111 bytes at 15.585 s, the banner after them at 15.623 s; 3 arrivals, 190 bytes.
  - 4.5.12, `stdbuf -o 0` — banner at 0.170 s, all fifteen ticks in one arrival at 15.526 s; 3 arrivals, 190 bytes.
  - 4.5.12, `stdbuf -o L` — banner at 0.167 s, all fifteen ticks in one arrival at 15.508 s; 3 arrivals, 190 bytes.
  - 5.0.1, no wrapper — ticks together at 15.587 s, banner after them at 15.635 s; 3 arrivals, 185 bytes.
  - 5.0.1, `stdbuf -o 0` — banner at 0.204 s, ticks together at 15.573 s; 3 arrivals, 185 bytes.
  - 5.0.1, `stdbuf -o L` — banner at 0.202 s, ticks together at 15.562 s; 3 arrivals, 185 bytes.
  - 5.1.2, no wrapper — ticks together at 15.637 s, banner after them at 15.687 s; 3 arrivals, 185 bytes.
  - 5.1.2, `stdbuf -o 0` — banner at 0.211 s, ticks together at 15.618 s; 3 arrivals, 185 bytes.
  - 5.1.2, `stdbuf -o L` — banner at 0.211 s, ticks together at 15.619 s; 3 arrivals, 185 bytes.
  - 5.2.1, no wrapper — ticks together at 15.786 s, banner after them at 15.845 s; 3 arrivals, 189 bytes.
  - 5.2.1, `stdbuf -o 0` — banner at 0.292 s, ticks together at 15.769 s; 3 arrivals, 189 bytes.
  - 5.2.1, `stdbuf -o L` — banner at 0.294 s, ticks together at 15.798 s; 3 arrivals, 189 bytes.

  The run the deleted sentence was reaching for is the last of those, and this is the whole of what the machine printed for it:
  ```
  COMMAND: stdbuf -o L /Applications/Blender.app/Contents/MacOS/Blender -b --factory-startup --python ticker.py 2>&1 | python3 reader.py
     0.294  chunk   1      64 bytes  total      64  b'Blender 5.2.1 LTS (hash 9e2066aef7ef built 2026-08-25 01:35:57)\n'
    15.798  chunk   2     111 bytes  total     175  b'tick 1\ntick 2\ntick 3\ntick 4\ntick 5\ntick 6\ntick 7\ntick 8\ntick 9\ntick 10'
    15.863  chunk   3      14 bytes  total     189  b'\nBlender quit\n'
    15.869  END  3 arrivals, 189 bytes total
  ```
  The reader shows only the first 70 bytes of an arrival, so the ticks read as ten; the arrival is 111 bytes, which is all fifteen. **Every line of all twelve runs is kept word for word** in `Audits/Blender Research/Reports/Hand tests — Open Threads questions — raw output/Mac 2026-08-30/`.

  **The calibration, run first on the same machine the same day, because a result of this shape is worth nothing without one.** Deliberately held-back output: entry 1's three-frame render passed through `holdbuf.py` reached the reader as four arrivals — 4096 bytes at 0.529 s, 4096 at 26.994 s, 4096 at 49.630 s, and a last partial 2838 bytes at 73.816 s released only when the run ended. Promptly-arriving output: `PYTHONUNBUFFERED=1 <4.5.12> -b --factory-startup --python-use-system-env --python ticker.py 2>&1 | python3 reader.py` reached it as 17 arrivals — banner at 0.346 s, then one tick a second from 0.503 s to 14.592 s. So the instrument tells the two shapes apart on this machine, and a wrapper run whose ticks had arrived one a second would have shown as such.

  **What the twelve runs came back with.** In all four versions the fifteen ticks arrived in a single arrival at about 15.5 to 15.9 seconds whether a wrapper was there or not, and in all four versions the banner arrived first, before the ticks, only in the wrapper runs — at about a fifth to a third of a second, against arriving after the ticks with no wrapper. `-o 0` and `-o L` did not differ from each other in any version.

**These five were run on the Creator's Mac only**, in 4.5.12 and 5.2.1, which behaved the same as each other. They were not run on the rented Linux machine, which had already been destroyed when they were added. *(One of the five reaches further than that line says. The `stdbuf` runs of the last bullet were run again on 2026-08-30, in all four versions and with both switches, when the sentence that used to end that bullet was replaced; 5.0.1 and 5.1.2 behaved as 4.5.12 and 5.2.1 did. The other four still stand on 4.5.12 and 5.2.1 alone, and all of them still stand on the Mac alone.)*

**What was not run.** The Windows half. It is written out in full and ready to run in `Audits/Blender Research/Reports/Hand tests — Open Threads questions.md`, under *Windows — for the runner's separate batch*, and is held for the single Windows batch this project runs across every topic. So this answer covers macOS on Apple silicon and Linux on x86-64, and leaves Windows open.

---

**Not covered.** This answer settles only that the sources are silent. It does not settle what actually happens — whether a line reaches a non-terminal reader as soon as it is printed, only when some amount of text has built up, only when Blender ends, or something else. It does not settle whether the answer differs between the two streams, between the log written by `--log-file` and ordinary printed output, or between text printed by Blender itself and text printed by a Python script it is running. It does not settle whether any of that can be changed by a setting, an argument, or an environment variable — I can only say that the arguments and environment variables the 5.1 manual lists do not include one that names buffering.

**Ambiguity.** "Its printed output" could mean everything Blender writes as a run proceeds, or only the render engine's own progress text. I answered the wider reading, since the sources are silent on both and the wider reading is the one that could not be answered by a narrower search. The narrower reading gets the same answer: nothing in the Cycles files supplied describes printing timing either.

**1. Outside the interface.** The sources do not say. I take the question to be about a run driven from outside Blender's own window already, since a reader that is not a terminal means something other than a person at the interface is reading what Blender prints — that is my reading of the question, not a statement from any source. Either way the sources say nothing about output timing, inside the interface or outside it, so there is nothing to report about how the two differ. The manual's command-line pages describe only the case of a person starting Blender in a terminal and reading it there.

**LABEL: observed** (2026-08-30). The hand test above answers this, and every run in it was Blender driven from outside its own window — started with no window at all, its output read by another program. A render's own printed lines reach a pipe and a plain file as they are produced, at the same moments as they reach a terminal; a Python script's `print()` lines do not reach a pipe or a file until the run ends, though they do reach a terminal one a second. So the behaviour asked about holds outside the interface, and it differs by *which part of Blender did the printing* rather than by whether a person is watching.

**2. Operating system.** The sources do not say. Nothing in the 5.1 manual, API documentation, release notes or Cycles files states how printed output is written on any one operating system, so there is no basis in the sources for saying whether it is the same on all of them. The operating-system-specific statements I found near this subject are on the manual's launch pages, and they are about where the console window is rather than about when a line arrives. The one I can quote is the macOS page's "to get all output printed to the terminal", which is about which executable you run.

**LABEL: observed** (2026-08-30). Tests 1 to 7 in the hand-test section above were each run twice, once on macOS 26.6.2 on Apple silicon and once on Ubuntu 22.04.4 on x86-64, on two builds carrying the same build hash `ec6e62d40fa9`, and **the two machines agreed on every one of them**. Tests 8 and 9 were run on the Mac only, because the rented machine had already been destroyed when they were added; in 4.5 and 5.2 both were run on both machines and the two agreed there. Test 10, the reading of the build's own help list, was run on the Mac only. **Windows is not covered.** It was not run — it is written out ready to run in `Audits/Blender Research/Reports/Hand tests — Open Threads questions.md` and is held for the single Windows batch this project runs across every topic. Whether the answer is the same there is open.
Source: manual, https://docs.blender.org/manual/en/5.1/advanced/command_line/launch/macos.html — read 2026-07-30.
I quote nothing from the Windows page, so nothing from it is claimed here.

---

## Things a person with Blender open could settle by hand

**Pruned 2026-08-30 by the hand tests.** Eight items stood here. **Items 1 to 7 were all run on 2026-08-30**, on the Creator's Mac and on a rented Linux machine, and their answers are in the observed section above — arrival through a pipe, the error stream on its own, arrival in a plain file, the terminal case for comparison, `--log-file`, a script's own printing, and whether the progress text is one line rewritten in place. They are not repeated here.

**One item is still open, and only in part.**

1. **The Windows half.** The items above that asked for the pipe and file tests to be repeated on Windows were run on macOS and on Linux, where the two agreed; **Windows was not run.** This project has a route to a Windows computer — a rental used four times — but the account allows one machine at a time and the way in costs about ninety minutes, so one Windows batch is run across every topic rather than each topic renting in turn. The Windows runs are written out in full and ready, in `Audits/Blender Research/Reports/Hand tests — Open Threads questions.md` under *Windows — for the runner's separate batch*, and the machine's own edition must be read and recorded before anything is claimed from it: every Windows computer this project has read turned out to be a Server edition, including one whose image was named for a desktop edition.

---

2026-08-29 — Claude Opus 5 (Claude Code) — Reports, Blender 5.1

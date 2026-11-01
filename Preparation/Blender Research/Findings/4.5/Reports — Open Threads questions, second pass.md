# Reports — Open Threads questions, second pass — Blender 4.5

**Sources consulted, and date.** Blender 4.5 manual (plain-text snapshot at `Blender Sources/manual_4.5`, including its `_images` folder), Blender 4.5 Python API documentation (`Blender Sources/api_4.5`), Blender 4.5 release notes (`Blender Sources/release_notes_4.5`), and Cycles' own files (`Blender Sources/cycles_4.5`). All read on 2026-07-30 (the snapshot date given to me).

I read only the question sheet and files inside Blender Sources/. I did not open any other part of this project.

Scope of this pass: question 14 only.

---

## 14. When a render's printed output is sent somewhere other than a terminal, what decides when each line of it becomes available to whatever is reading?

**LABEL: not in the sources.**

**Answer.** Nothing in the 4.5 manual, the 4.5 Python API documentation, the 4.5 release notes, or the Cycles files supplied here says anything about *when* a line Blender has printed becomes readable to a program that is reading Blender's output rather than a person watching a terminal. The sources describe *what* is printed and *where* it can be sent. They never describe the timing, the amount of text held back before it is handed over, or anything that changes that timing.

**A note on the reading of the question.** "Sent somewhere other than a terminal" has two plausible readings, and I searched for both:

- *(a)* Blender's ordinary printed output being captured by another program or sent to a file — the reading I take to be meant, since the question says "whatever is reading". Nothing in the sources addresses this at all.
- *(b)* Blender being told to write its log to a file with the `--log-file` option. The sources document that this option exists, but say nothing about timing either. The exact words are:

  > "--log-file <filepath>
  >
  > Set a file to output the log to."

  Source: manual, `https://docs.blender.org/manual/en/4.5/advanced/command_line/arguments.html`, section "Logging Options". Date read: 2026-07-30.

  That sentence settles that output can be directed to a file. It says nothing about when a line reaches that file, so reading *(b)* is also unanswered.

**The closest thing in the sources to a statement about output going somewhere other than a terminal.** Two statements name a destination, and neither touches timing.

- The manual, describing the ordinary case only:

  > "When Blender is manually started from a terminal,
  > Blender output is shown in the Console Window it is started from."

  Source: manual, `https://docs.blender.org/manual/en/4.5/advanced/command_line/index.html`. Date read: 2026-07-30.

- Cycles' own add-on file, naming a stream for one particular kind of output:

  > `    parser.add_argument("--cycles-print-stats",`
  > `                        help="Print rendering statistics to stderr",`
  > `                        action='store_true')`

  Source: `Blender Sources/cycles_4.5/engine.py`, lines 13–15, inside `_configure_argument_parser()`. Date read: 2026-07-30. *(Line reference corrected by the citation check of 2026-08-30: the three quoted lines are 13 to 15; line 12 is the end of the parser's own set-up. The words and the file are as given.)* This is **observed** in the file, and it settles only which stream those statistics are said to go to — not when a line on that stream becomes readable.

**Where I looked.**

Searched, case-insensitively, across the whole of `manual_4.5`, `api_4.5` and `release_notes_4.5` at once, for each of these terms:

`buffering`, `buffered`, `unbuffered`, `line buffer`, `buffer`, `flush`, `flushed`, `fflush`, `setvbuf`, `stdout`, `stderr`, `std::cout`, `std::cerr`, `standard output`, `sys.stdout`, `pipe`, `piping`, `when piped`, `tee`, `grep`, `> log`, `redirect`, `tty`, `not a terminal`, `terminal`, `console`, `System Console`, `non-interactive`, `subprocess`, `Popen`, `capture`, `as soon as`, `immediately available`, `in real time`, `real-time output`, `line by line`, `chunk`, `blender-launcher`, `print()`, `printed to`, `written to the console`, `output is shown`, `output to a file`, `appears in the console`, `latency`, `delay`, `progress bar`, `Fra:`, `Mem:`, `stat`, `print stats`, `cycles-print-stats`, `verbose`, `batch render`, `render farm`, `headless`, `X server`, `ssh`, `log`, `logging`, `log file`, `--log`, `background mode`, `PYTHON`.

Results worth recording:

- `stdout`, `stderr`, `standard output` (as a stream), `sys.stdout`, `buffering`, `unbuffered`, `fflush`, `setvbuf`, `line buffer`, `piping`, `when piped`, `in real time`, `line by line`, `blender-launcher`: **zero hits anywhere** in the 4.5 manual, API documentation or release notes.
- `tty`: only ever as part of the word "pretty" (`--debug-depsgraph-pretty`). Ruled out.
- `flush`/`flushed`: only about text objects, node editor icons, and animation edits being "flushed to other views". Nothing to do with output streams. Ruled out.
- `buffer`/`buffered`: only audio buffering (`aud` module), GPU frame buffers (`gpu.types`), compositor node output buffers, and one Preferences setting, "Console Scroll-back Lines — The number of lines, buffered in memory of the console window." That last one is about how much scroll-back the console window keeps, not about when a line is handed to a reader. Ruled out.
- `redirect`: only an icon name (`FOLDER_REDIRECT`) and a node-link option. Ruled out.
- `subprocess`: only shader compilation subprocesses in EEVEE/Preferences and one bug-fix entry. Ruled out.

Pages opened in full and read, and why each was ruled out:

- `https://docs.blender.org/manual/en/4.5/advanced/command_line/index.html` — the page about using Blender from the command line. Describes the Console Window and lists use cases including "For rendering animation" and "For automation and batch processing". Says only that output is shown in the console window it is started from. Nothing about any other destination or about timing.
- `https://docs.blender.org/manual/en/4.5/advanced/command_line/arguments.html` — read the whole "Logging Options" section (`--log`, `--log-level`, `--log-show-basename`, `--log-show-backtrace`, `--log-show-timestamp`, `--log-file`), the "Debug Options" section, the "Python Options" section, the Cycles options including `--cycles-print-stats`, and the complete "Environment Variables" list. No option or environment variable in this page is described as affecting when output becomes available; no environment variable relating to output timing is listed at all.
- `https://docs.blender.org/manual/en/4.5/advanced/command_line/render.html` — rendering from the command line, including rendering over SSH with no display. Nothing about reading Blender's output.
- `https://docs.blender.org/manual/en/4.5/advanced/command_line/launch/linux.html`, `.../launch/macos.html`, `.../launch/windows.html` — all three launch pages. All describe starting Blender from a terminal and seeing output there. None mentions sending output anywhere else.
- `https://docs.blender.org/manual/en/4.5/render/output/properties/output.html` — includes the "Primitive Render Farm" hint (shared directory, Placeholders, Overwrite off) and the tip that "When you view the System Console, you can see some of the output of the encoding process." Both are about a person watching a console; neither says when a line arrives anywhere.
- `https://docs.blender.org/manual/en/4.5/editors/preferences/system.html` — the "Console Scroll-back Lines" preference, ruled out above.
- `https://docs.blender.org/manual/en/4.5/advanced/deploying_blender.html` — deploying Blender in production. Covers installation, offline mode and bundled extensions. Nothing about output.
- `https://docs.blender.org/api/4.5/info_tips_and_tricks.html` and `https://docs.blender.org/api/4.5/info_quickstart.html` — the two API pages that discuss the terminal. They advise keeping a terminal open to see `print()` output and tracebacks. Neither says anything about what happens when that output is not going to a terminal.

Picture examined:

- `_images/advanced_command-line_launch_linux_console.png`, the manual's screenshot on the Linux launch page. **documented** by way of the manual. It shows a Konsole window in which `blender` has been run and eight lines of jack/OpenAL start-up messages have appeared. It shows output in a terminal only, so it cannot settle anything about output going elsewhere. Ruled out.

Cycles' own files:

- Searched all 399 files under `Blender Sources/cycles_4.5` for `fflush`, `stdout`, `stderr`, `setvbuf`, `printf`, `std::cout`, `std::cerr`, `isatty`, `flush`. The only hits are device-side and debug `printf` helpers inside the kernel and utility headers (for example `source/util/types_float4.h` line 121, `source/kernel/osl/services_gpu.h`), one "Flush-to-zero" comment in `source/util/half.h` line 87, and the `--cycles-print-stats` help text in `engine.py` quoted above. What is supplied here is Cycles' kernel and utility headers, its shader files, and its Python add-on files — there is no session, progress-reporting or logging code in this folder to examine. So the files cannot answer the question either.
- Read `Blender Sources/cycles_4.5/engine.py` in full at the top (lines 1–60) and searched the other add-on Python files (`__init__.py`, `properties.py`, `ui.py`, `operators.py`, `osl.py`, `presets.py`, `camera.py`, `version_update.py`) for `print`, `log`, `flush`, `report`. Nothing bears on output timing.

Release notes:

- Searched all twenty 4.5 release-note files for `log`, `logging`, `console`, `output stream`, `stdout`, `terminal`. The only hits are unrelated (a crash fix mentioning a catalog, a dialog message change, a blog link). No 4.5 release note records any change to how output is written or when it is written. Address of the set: `https://developer.blender.org/docs/release_notes/4.5/`.

---

### Answered by hand test, 2026-08-30

**LABEL: observed.** Run by Claude Opus 5 (Claude Code) on 2026-08-30, in Blender 4.5.12, on two machines. The paragraphs above still stand: the sources say nothing. What follows is what a running Blender did. The full list, with every command written out and the raw output of every run, is `Audits/Blender Research/Reports/Hand tests — Open Threads questions.md`.

**The two machines, and the answers are never merged.**

- **The Creator's own Mac** — a MacBook Pro with an Apple M1 Pro chip, eight processor cores, 16 GB of memory, macOS 26.6.2, Apple's own graphics and no second card. Blender 4.5.12 at `/Applications/Blender 4.5.12.app/Contents/MacOS/Blender`, build hash `84afd5f785f7`.
- **A rented Linux machine** — Vast.ai instance `49254257` in the United Kingdom, a container rather than a whole machine, Ubuntu 22.04.4 LTS on kernel `6.8.0-124-generic`, an Intel Core i7-11800H with sixteen visible threads against a quota of 15.36 cores, a memory ceiling of about 15.8 GB, one NVIDIA GeForce RTX 3070 Laptop GPU on driver 535.309.01 that no test used, 32 GB of disk, and no display server. Blender 4.5.12 at `/opt/blender/blender-4.5.12-linux-x64/blender`, taken from Blender's own download server as the Linux x64 build, build hash `84afd5f785f7` — the same build as the Mac's. Rented and destroyed the same day.

**What was set up.** One file, `scene.blend`: the factory startup scene — one cube, one lamp, one camera — set to Cycles on the processor, 900 samples, adaptive sampling and denoising off, 720 by 405 at 100 per cent, frames 1 to 5, saving PNG files to `/tmp/r14out/f_`. One frame takes about fifteen seconds on either machine. Every run carried `--factory-startup`, both to leave the Creator's own settings alone and because the Mac has third-party add-ons whose messages would otherwise have been read as Blender's own.

**How arrival was measured.** A reader that looks only after a run has ended cannot tell a line held back from a line printed promptly, so a small program (`reader.py`) was put on the far end of the pipe. It reads raw bytes as they arrive and prints, for each arrival, the seconds since it started, how many bytes arrived, and the first seventy bytes of them. Promptly-arriving output shows as many small arrivals timed to the run's own events; held-back output shows as few arrivals of one fixed size at long intervals. **That the instrument can show the second pattern was proved on both machines** by putting a program between Blender and the reader that deliberately held everything in 4096-byte blocks. On the Mac:

```
   0.559  chunk   1    4096 bytes  total    4096  b'Blender 4.5.12 LTS (hash 84afd5f785f7 built 2026-07-21 01:41:35)\nRead '
  24.487  chunk   2    4096 bytes  total    8192  b'Peak:0.09M | Scene, ViewLayer | Updating Integrator\nFra:1 Mem:8.30M (P'
  46.278  chunk   3    4096 bytes  total   12288  b'k 26.18M) | Time:00:00.00 | Mem:0.00M, Peak:0.00M | Scene, ViewLayer |'
  75.931  chunk   4    2709 bytes  total   14997  b', ViewLayer | Updating Mesh | Copying Attributes to device\nFra:3 Mem:8'
  75.936  END  4 arrivals, 14997 bytes total
```

On the rented Linux machine the same run gave four arrivals at 0.256, 18.103, 36.024 and 54.115 seconds, of 4096, 4096, 4096 and 2599 bytes.

**1. A render's own printed lines reach a pipe as they are produced.** Command, on both machines: `<blender> -b --factory-startup scene.blend -s 1 -e 3 -a 2>&1 | python3 reader.py`. On the Mac, sixty-two arrivals over forty-three seconds. The last eight, word for word:

```
  29.071  chunk  56     216 bytes  total   14124  b'Fra:3 Mem:8.30M (Peak 26.18M) | Time:00:00.00 | Mem:0.09M, Peak:0.09M '
  29.071  chunk  57     354 bytes  total   14478  b'Fra:3 Mem:8.30M (Peak 26.18M) | Time:00:00.00 | Mem:0.00M, Peak:0.09M '
  29.071  chunk  58     104 bytes  total   14582  b'Fra:3 Mem:8.30M (Peak 26.18M) | Time:00:00.00 | Mem:0.09M, Peak:0.09M '
  29.084  chunk  59     126 bytes  total   14708  b'Fra:3 Mem:12.83M (Peak 26.18M) | Time:00:00.01 | Remaining:00:11.67 | '
  43.302  chunk  60     208 bytes  total   14916  b'Fra:3 Mem:17.28M (Peak 26.18M) | Time:00:14.23 | Mem:4.54M, Peak:4.54M'
  43.316  chunk  61      67 bytes  total   14983  b"Saved: '/tmp/r14out/f_0003.png'\nTime: 00:14.24 (Saving: 00:00.01)\n\n"
  43.345  chunk  62      14 bytes  total   14997  b'\nBlender quit\n'
  43.349  END  62 arrivals, 14997 bytes total
```

The fifty-five arrivals before these are cut for length; they are the same shape, in bursts at each frame boundary. *(Corrected by the citation check of 2026-08-30: sixty-two arrivals in all and seven shown above, so fifty-five are cut, not the fifty-four this line had.)* On the rented Linux machine, fifty-five arrivals over forty-nine seconds, ending:

```
  32.301  chunk  52     126 bytes  total   14598  b'Fra:3 Mem:12.95M (Peak 26.30M) | Time:00:00.01 | Remaining:00:10.53 | '
  49.118  chunk  53     208 bytes  total   14806  b'Fra:3 Mem:17.40M (Peak 26.30M) | Time:00:16.83 | Mem:4.54M, Peak:4.54M'
  49.139  chunk  54      67 bytes  total   14873  b"Saved: '/tmp/r14out/f_0003.png'\nTime: 00:16.85 (Saving: 00:00.02)\n\n"
  49.167  chunk  55      14 bytes  total   14887  b'\nBlender quit\n'
  49.173  END  55 arrivals, 14887 bytes total
```

**The two machines agreed.**

**2. A pipe is no different from a terminal for those lines.** The same render was run again with a real terminal in place of the pipe, supplied by the system's own `script` command. On the Mac, fifty-six arrivals ending at 43.690 seconds; on Linux, seventy-six ending at 51.574. The arrival pattern is the same as in test 1 on each machine. The only difference in what arrived is that every line ends `\r\n` instead of `\n`, which is the terminal's doing.

**3. The same lines reach a plain file as they are produced.** Command: `<blender> -b --factory-startup scene.blend -s 1 -e 3 -a > out.txt 2>&1`, with the file's size read every quarter second from a second process while the render was still going. On the Mac:

```
   0.000  file is       0 bytes
   0.509  file is    4832 bytes
  15.260  file is    9770 bytes
  29.547  file is   14708 bytes
  44.311  file is   14997 bytes
  44.312  run finished, file is   14997 bytes
```

On the rented Linux machine:

```
   0.000  file is       0 bytes
   0.251  file is    4596 bytes
   0.501  file is    4722 bytes
  17.787  file is    9660 bytes
  35.570  file is   14598 bytes
  53.102  file is   14887 bytes
  53.102  run finished, file is   14887 bytes
```

**The steps are the frames.** The file grows once per frame, at the moment that frame finishes.

**4. The progress readout is whole separate lines, not one line rewritten.** Counting the line endings in the file test 3 wrote: on the Mac, `newlines: 136   carriage returns: 0   bytes: 14997`; on Linux, `newlines: 136   carriage returns: 0   bytes: 14887`. No carriage returns at all.

**5. A Python script's own printing does not reach a pipe or a file until the run ends, and this is the one place the answer splits.** A script printing one line a second for fifteen seconds (`print("tick %d" % i)` then a one-second wait) was run with `<blender> -b --factory-startup --python ticker.py`. Piped, on the Mac:

```
  15.584  chunk   1     111 bytes  total     111  b'tick 1\ntick 2\ntick 3\ntick 4\ntick 5\ntick 6\ntick 7\ntick 8\ntick 9\ntick 10'
  15.615  chunk   2      65 bytes  total     176  b'Blender 4.5.12 LTS (hash 84afd5f785f7 built 2026-07-21 01:41:35)\n'
  15.618  chunk   3      14 bytes  total     190  b'\nBlender quit\n'
  15.622  END  3 arrivals, 190 bytes total
```

All fifteen lines arrived together at the end, in one arrival. On the rented Linux machine, the same three arrivals at 15.229, 15.259 and 15.262 seconds. Sent to a file instead, the file stayed at zero bytes and reached its full 190 bytes in one step at 15.666 seconds on the Mac and 15.281 on Linux.

Three further runs on each machine:

- **With `PYTHONUNBUFFERED=1` set**, the result was unchanged — one arrival of 111 bytes at 15.492 seconds on the Mac, 15.226 on Linux.
- **With each line printed as `print("tick %d" % i, flush=True)`**, the lines arrived one a second through the same pipe. The first four arrivals on the Mac, word for word — the remaining eleven follow the same one-a-second pattern and are cut for length:
  ```
     0.439  chunk   1       7 bytes  total       7  b'tick 1\n'
     1.445  chunk   2       7 bytes  total      14  b'tick 2\n'
     2.453  chunk   3       7 bytes  total      21  b'tick 3\n'
     3.459  chunk   4       7 bytes  total      28  b'tick 4\n'
  ```
  On Linux the same, at 0.217, 1.217, 2.217 and 3.218 seconds.
- **With a real terminal in place of the pipe**, the plain script (no flushing) also ticked once a second, and the version banner arrived first rather than last. On the Mac:
  ```
     0.000  chunk   1       4 bytes  total       4  b'^D\x08\x08'
     0.185  chunk   2      66 bytes  total      70  b'Blender 4.5.12 LTS (hash 84afd5f785f7 built 2026-07-21 01:41:35)\r\n'
     0.444  chunk   3       8 bytes  total      78  b'tick 1\r\n'
     1.449  chunk   4       8 bytes  total      86  b'tick 2\r\n'
  ```
  On Linux the banner arrived at 0.064 seconds and the first tick at 0.214.

**6. The same split shown inside one run.** So that nothing could be blamed on two runs differing, one run printed both kinds of line into one pipe: `<blender> -b --factory-startup scene.blend --python-expr "print('SCRIPT LINE PRINTED AT START')" -s 1 -e 2 -a 2>&1 | python3 reader.py`. On the Mac the render's own lines arrived throughout, and:

```
   0.555  chunk   1     325 bytes  total     325  b'Blender 4.5.12 LTS (hash 84afd5f785f7 built 2026-07-21 01:41:35)\nRead '
  21.000  chunk  21      67 bytes  total    5107  b"Saved: '/tmp/r14out/f_0001.png'\nTime: 00:20.44 (Saving: 00:00.08)\n\n"
  46.086  chunk  27      67 bytes  total   10173  b"Saved: '/tmp/r14out/f_0002.png'\nTime: 00:25.08 (Saving: 00:00.01)\n\n"
  46.099  chunk  28      29 bytes  total   10202  b'SCRIPT LINE PRINTED AT START\n'
  46.136  END  29 arrivals, 10216 bytes total
```

The arrivals not shown are the render's own, spread across the run. The script's line was printed within the first second and reached the reader at 46.099 seconds, after both frames were finished. On the rented Linux machine the same run put it at 33.598 seconds, after the second frame's `Saved` line at 33.589.

**7. Everything a render prints goes to the ordinary output stream; the error stream carried nothing.** With only the error stream fed to the reader — `<blender> -b --factory-startup scene.blend -f 1 -- --cycles-print-stats 2>&1 1>/dev/null | python3 reader.py` — the whole run gave `END  0 arrivals, 0 bytes total` on both machines. Keeping the two streams apart in files, `<blender> -b --factory-startup scene.blend -f 1 -- --cycles-print-stats > s_out.txt 2> s_err.txt`, gave **8341 bytes of ordinary output and 0 bytes of error stream** on the Mac, and 8235 and 0 on Linux. **The Cycles statistics were in the ordinary output**, at lines beginning `Update statistics:`, `Render statistics:` and `Mesh statistics:` — not in the error stream, which is where the help text quoted above says they go.

**8. The file written by `--log-file` fills in as the run goes.** Command: `<blender> -b --factory-startup scene.blend --log "*" --log-level -1 --log-file run.log -s 1 -e 2 -a > /dev/null 2>&1`. The value `-1` was read off `blender --help`, which for 4.5 says *"Set the logging verbosity level (higher for more details) defaults to 1, use -1 to log all levels."* On the Mac the log file went 0, 1265 bytes at 0.251 seconds, 43603 at 0.509, 43715 at 28.991, finishing at 43715. On Linux, 0, 33882 bytes at 0.252 seconds, 33994 at 34.066. **Almost all of it is written at startup**: over the whole of the two-frame render the file grew by only 112 bytes on either machine, because in 4.5 the render's progress lines are not among the logging categories this file collects.

**9. Nothing already printed is lost when a run is stopped part-way.** A three-frame render was sent to a file and stopped at twenty-two seconds, once with an interrupt and once with an outright kill, which cannot flush anything a program is still holding. On the Mac, before the interrupt the file held 4832 bytes ending `Fra:1 ... | Sample 1/900`; afterwards 5218 bytes ending:

```
Fra:1 Mem:17.28M (Peak 26.18M) | Time:00:21.73 | Mem:4.54M, Peak:4.54M | Scene, ViewLayer | Cancelled

Blender quit
```

Before the outright kill the file held 9770 bytes; afterwards it held **the same 9770 bytes**, ending on the most recent line the run had printed:

```
Fra:2 Mem:8.30M (Peak 26.18M) | Time:00:00.00 | Mem:0.09M, Peak:0.09M | Scene, ViewLayer | Updating Device | Writing constant memory
Fra:2 Mem:8.30M (Peak 26.18M) | Time:00:00.00 | Mem:0.09M, Peak:0.09M | Scene, ViewLayer | Sample 0/900
Fra:2 Mem:12.83M (Peak 26.18M) | Time:00:00.03 | Remaining:00:28.09 | Mem:4.54M, Peak:4.54M | Scene, ViewLayer | Sample 1/900
```

On the rented Linux machine the same: 9660 bytes before the kill and 9660 after, ending on `Fra:2 ... | Sample 1/900`. Nothing was lost on either machine.

**10. The build's own help list names no switch for any of this.** Running `blender --help` and searching the whole printed list for `buffer`, `flush`, `unbuffer`, `stdout` and `stderr` gives exactly one hit in Blender 4.5.12: `Read from disk (Do not buffer).`, which is the animation player's `-m` argument and concerns reading picture files, not writing output. The same single hit, and nothing else, in 5.0.1, 5.1.2 and 5.2.1. So the installed build lists nothing the manual's argument page omits.

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

**Not covered.** This answer settles only that the supplied sources are silent. It does not say what Blender actually does when its output is captured — that behaviour may well be perfectly definite and simply undocumented here. It does not settle whether the two streams behave alike, whether `--log-file` output behaves like ordinary printed output, whether a render engine's progress lines behave like Python `print()` output, or whether any option or environment variable changes the timing. It also does not settle the second reading of the question in any more detail than the first.

**Outside the interface.** The sources do not say. There is no statement anywhere in the 4.5 manual, API documentation, release notes or the supplied Cycles files about the timing of printed output, either when Blender is driven by a person or when it is started and controlled by another program. The manual describes the background/no-window way of running Blender (`-b`, `--background`) and rendering over SSH, but never describes how its output reaches a reader in those cases. **The hand test of 2026-08-30 answers it, and every run in that test was Blender driven from outside its own window** — started with no window at all, its output read by another program. **LABEL: observed.** A render's own printed lines reach a pipe and a plain file as they are produced, at the same moments as they reach a terminal; a Python script's `print()` lines do not reach a pipe or a file until the run ends, though they do reach a terminal one a second. So the behaviour asked about holds outside the interface, and it differs by *which part of Blender did the printing* rather than by whether a person is watching.

**Operating system.** The sources do not say. The three launch pages (Linux, macOS, Windows) each describe seeing output in that system's own terminal, and the Windows page adds that the Console Window is toggled off by default and can be shown again with Window ‣ Toggle System Console. None of the three says anything about output going anywhere other than that window, so the sources give no basis for saying whether the behaviour asked about is the same on all three or different. **LABEL: observed** (2026-08-30): tests 1 to 9 in the section above were each run twice, once on macOS 26.6.2 on Apple silicon and once on Ubuntu 22.04.4 on x86-64, on two builds with the same build hash `84afd5f785f7`, and **the two machines agreed on every one of the nine results**. Test 10, the reading of the build's own help list, was run on the Mac only. **Windows is not covered.** It was not run — it is written out ready to run in `Audits/Blender Research/Reports/Hand tests — Open Threads questions.md` and is held for the single Windows batch this project runs across every topic. Whether the answer is the same there is open.

---

## Things a person with Blender open could settle by hand, in a few minutes

**Pruned 2026-08-30 by the hand tests.** Six items stood here. **Items 1, 3, 4, 5 and 6 were all run on 2026-08-30, on the Creator's Mac and on a rented Linux machine, and their answers are in the observed section above** — output through a pipe, the two streams together with the Cycles statistics switch, `--log-file`, a script's own printing against a render's, and whether anything in the environment or a wrapper changes the timing. Output to a file, which item 1 did not cover, was run as well and is in that section too. They are not repeated here.

**One item is still open, and only in part.**

1. **Item 2, which is wholly a Windows test.** It asked for the file-redirect run to be repeated on Windows. The same run was done on macOS and on Linux and the two agreed; **Windows was not run.** This project has a route to a Windows computer — a rental used four times — but the account allows one machine at a time and the way in costs about ninety minutes, so one Windows batch is run across every topic rather than each topic renting in turn. The Windows runs are written out in full and ready, in `Audits/Blender Research/Reports/Hand tests — Open Threads questions.md` under *Windows — for the runner's separate batch*, and the machine's own edition must be read and recorded before anything is claimed from it: every Windows computer this project has read turned out to be a Server edition, including one whose image was named for a desktop edition.

---

2026-08-30 — Claude Opus 5 (Claude Code) — Reports, Blender 4.5

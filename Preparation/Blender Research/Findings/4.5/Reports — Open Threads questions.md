# Reports — Open Threads questions (Blender 4.5)

**Sources consulted, and when.** Blender 4.5 manual (`manual_4.5`, including its `_images` folder), Blender 4.5 Python API documentation (`api_4.5`), Blender 4.5 release notes (`release_notes_4.5`), and the Cycles files supplied in `cycles_4.5`. All four are the snapshot taken on **2026-07-30**, which is the date read for everything below.

I read only the question sheet and files inside Blender Sources/. I did not open any other part of this project.

One disclosure, for completeness: after writing this file I listed the names of the files in the folder I save into, to confirm my own file name was correct. I saw sibling file names only and opened none of them; no content from any of them reached me.

This document is a first pass, and it answers question **14** only.

---

## 14. When a render's printed output is sent somewhere other than a terminal, what decides when each line of it becomes available to whatever is reading?

**LABEL: not in the sources.**

**Answer.** Nothing in the 4.5 manual, the 4.5 Python API documentation, the 4.5 release notes, or the supplied Cycles files says anything about this. The sources describe Blender's text output only as something that appears in a console window when a person starts Blender from one. They never discuss that output going anywhere else — a file, a pipe, another program — and they never state what governs the moment a printed line reaches a reader in that case. There is no statement about buffering of Blender's text output, no statement about flushing it, no statement that output behaves differently when it is not attached to a terminal, and no setting documented anywhere that changes any of this.

The closest the sources come is that the manual states plainly where the output goes when a person is at a terminal, and nothing beyond it:

> "The Console Window, also called a Terminal, is an operating system text window that displays messages about Blender's operations, status, and internal errors."

> "When Blender is manually started from a terminal, Blender output is shown in the Console Window it is started from."

Source: the manual, https://docs.blender.org/manual/en/4.5/advanced/command_line/index.html — read 2026-07-30.

Two adjacent facts I found, which I record only so it is clear they were considered and do not answer the question:

- The manual documents an argument that sends log messages to a file — "`--log-file <filepath>`" / "Set a file to output the log to." (the manual, https://docs.blender.org/manual/en/4.5/advanced/command_line/arguments.html, read 2026-07-30). It says nothing about when a line written that way becomes readable, and it covers the logging categories rather than a render's printed output generally.
- The manual documents that Blender touches one of Python's standard streams, but it is the input one, not an output one: under "`-d, --debug`" it lists "Keeps Python's `sys.stdin` rather than setting it to None" (same page, same date). There is no matching statement anywhere about `sys.stdout` or `sys.stderr`.

Because the question asks what *decides* the timing, and the sources contain no statement of that kind at all, "not in the sources" is the whole answer. Anything more specific I could write here would come from my own knowledge of how programs behave, not from these files, so I am not writing it.

**Where I looked, and what I ruled out.**

Searched (case-insensitively, across whole folders at once, not file by file):

- Across `manual_4.5`: `buffer`, `unbuffered`, `flush`, `line-buffer`, `line buffer`, `line-buffered`, `buffered output`, `pipe`, `piping`, `pipe to`, `stdout`, `stderr`, `stdin`, `terminal`, `console`, `tty`, `setvbuf`, `setbuf`, `isatty`, `fflush`, `redirect`, `file descriptor`, `capture output`, `capture the output`, `carriage return`, `same line`, `overwrit`, `streaming output`, `real time`, `as it runs`, `line by line`, `immediately`, `latency`, `block size`, `chunk`, `2>&1`, `tee`, `>>`.
- The same list across `api_4.5` and `release_notes_4.5`.
- The same list across `cycles_4.5`.

Results, and why each was ruled out:

- `stdout`, `stderr`, `unbuffered`, `line buffer`, `line-buffered`, `buffered output`, `isatty`, `setvbuf`, `setbuf`, `fflush`, `file descriptor`, `piping`, `capture output`, `capture the output`, `carriage return`, `streaming output`, `line by line`, `latency`, `block size`, `chunk` — **no hits at all** in the manual, the API documentation, or the release notes. `stdout` and `stderr` appear nowhere in any of the three.
- Every `buffer` hit in the manual is about something else: a copy-paste buffer and a vertex buffer object (https://docs.blender.org/manual/en/4.5/advanced/blender_directory_layout.html and https://docs.blender.org/manual/en/4.5/editors/preferences/system.html), an audio "Mixing Buffer" (same preferences page), the animation player's `-m` "Read from disk (Do not buffer)" (https://docs.blender.org/manual/en/4.5/advanced/command_line/arguments.html — this is the standalone animation player reading frames from disk, not text output), and, on the macOS launch page, a text editor prompt to "save the modified buffer" while editing a system file (https://docs.blender.org/manual/en/4.5/advanced/command_line/launch/macos.html). One preference reads "The number of lines, buffered in memory of the console window" (https://docs.blender.org/manual/en/4.5/editors/preferences/system.html) — that is how much scrollback Blender's own Python Console editor keeps, not the timing of output leaving Blender.
- `buffer` and `flush` in the API documentation index resolve to graphics and audio objects (`bgl.Buffer`, `gpu.types.GPUFrameBuffer`, `aud` sound buffering, `FFmpegSettings.buffersize`, `bpy.ops.ed.flush_edits`) and to a Grease Pencil modifier. None concern text output.
- `pipe` in the manual is always "pipeline", "pipes", "pipette" or "parallelepiped" — words that happen to contain the letters — and `tty` is always inside "pretty". No shell-redirection sense anywhere, and no terminal-device sense anywhere. *(Corrected by the citation check of 2026-08-30: this line had given "eyedropper" as one of the words, and "eyedropper" does not contain the letters `pipe`; the word on those pages is "pipette". The conclusion is unchanged.)*
- Pages I opened in full and ruled out: the command-line overview (https://docs.blender.org/manual/en/4.5/advanced/command_line/index.html), the full argument list (https://docs.blender.org/manual/en/4.5/advanced/command_line/arguments.html, including its Logging Options and Debug Options sections), rendering from the command line (https://docs.blender.org/manual/en/4.5/advanced/command_line/render.html), all three launch pages — Linux, macOS, Windows (https://docs.blender.org/manual/en/4.5/advanced/command_line/launch/linux.html, .../macos.html, .../windows.html), the crash troubleshooting page (https://docs.blender.org/manual/en/4.5/troubleshooting/crash.html, which describes Windows batch scripts that produce a debug log file but says nothing about when lines land in it), the Python troubleshooting page (https://docs.blender.org/manual/en/4.5/troubleshooting/python.html), the API's "Tips and Tricks" page (https://docs.blender.org/api/4.5/info_tips_and_tricks.html, whose "Use the Terminal" section is the API documentation's only discussion of where printed output shows up, and only for a terminal), and the API's "Blender as a Python Module" page (https://docs.blender.org/api/4.5/info_advanced_blender_as_bpy.html).
- Release notes: I searched every subject file for `logging`, `log`, `console`, `output`, `print`, `terminal`, `stream`. The only output-related entries are about rendered image and video files (https://developer.blender.org/docs/release_notes/4.5/rendering/) and output paths (https://developer.blender.org/docs/release_notes/4.5/core/, https://developer.blender.org/docs/release_notes/4.5/python_api/). Nothing about text output or its timing.
- Cycles files: the supplied `cycles_4.5` folder holds the add-on's Python files, the render kernel headers under `source/`, and compiled OSL shader files under `shader/`. A search of the whole folder for `fflush`, `setvbuf`, `setbuf`, `isatty`, `stdout`, `stderr` returns exactly one line, a help string in the add-on's argument parser: `help="Print rendering statistics to stderr",` (`cycles_4.5/engine.py`, line 14, inside `_configure_argument_parser`). That names a stream but says nothing about when its lines become readable. So the supplied folder shows only that Cycles has a switch whose help text names one of the two streams; nothing in it states when a line printed to either stream becomes readable.
- The manual's pictures: the pages that could plausibly have shown this are the three launch pages, and their images are screenshots of Blender started in a terminal window (`_images/advanced_command-line_launch_linux_console.png`, `_images/advanced_command-line_launch_macos_mac.png`, `_images/advanced_command-line_launch_windows_console.png`). A screenshot of a terminal cannot show what happens when output is not going to a terminal, so I did not treat them as bearing on the question.

**Ambiguity.** The question can be read two ways: what decides the timing (a mechanism), or what a user can do to change the timing (a setting or switch). The sources are silent on both, so the answer is the same either way. I answered the first reading, which I take to be the one meant.

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

**Not covered.** This answer settles only that the four supplied sources do not address the question. It does not settle what Blender 4.5 actually does when its output is redirected — the behaviour may well be consistent and easily measured, but no supplied source states it, so I have not stated it. It also does not settle whether any Blender source outside this folder (its own C++ code, developer documentation, or bug tracker) covers it; I was not given those and did not look. And it does not settle whether the `--log-file` route behaves differently from ordinary printed output, since neither is described.

**Outside the interface.** The sources do not say: the manual only ever describes output reaching a person's terminal, and says nothing about a program reading Blender's output. **The hand test of 2026-08-30 answers it, and every run in that test was Blender driven from outside its own window** — started with no window at all, its output read by another program. **LABEL: observed.** A render's own printed lines reach a pipe and a plain file as they are produced, at the same moments as they reach a terminal; a Python script's `print()` lines do not reach a pipe or a file until the run ends, though they do reach a terminal one a second. So the behaviour asked about holds outside the interface, and it differs by *which part of Blender did the printing* rather than by whether a person is watching. There is still no documented statement about the timing anywhere in the sources.

**Operating system.** The manual has separate launch pages for Linux, macOS and Windows, and I checked all three. None of them mentions output buffering or timing, and the whole-folder searches listed above turned up nothing on any of the three. So the sources say nothing about this for Linux, macOS or Windows, and state no per-system difference. **LABEL: observed** (2026-08-30): tests 1 to 9 above were each run twice, once on macOS 26.6.2 on Apple silicon and once on Ubuntu 22.04.4 on x86-64, on two builds with the same build hash `84afd5f785f7`, and **the two machines agreed on every one of the nine results**. Test 10, the reading of the build's own help list, was run on the Mac only. **Windows is not covered.** It was not run — it is written out ready to run in `Audits/Blender Research/Reports/Hand tests — Open Threads questions.md` and is held for the single Windows batch this project runs across every topic. Whether the answer is the same there is open.

---

## Things a person with Blender open could settle by hand

**Pruned 2026-08-30 by the hand tests.** Seven items stood here. **Items 1, 2, 3, 5, 6 and 7 were all run on 2026-08-30, on the Creator's Mac and on a rented Linux machine, and their answers are in the observed section above** — output through a pipe, output to a file, the two streams, `--log-file`, whether the progress readout is separate lines, and whether the build's own help list names an undocumented switch. They are not repeated here.

**One item is still open, and only in part.**

1. **Item 4's Windows half.** Items 1 and 2 were asked for Windows as well as macOS and Linux. Both were run on macOS and on Linux and the two agreed; **Windows was not run.** This project has a route to a Windows computer — a rental used four times — but the account allows one machine at a time and the way in costs about ninety minutes, so one Windows batch is run across every topic rather than each topic renting in turn. The Windows runs are written out in full and ready, in `Audits/Blender Research/Reports/Hand tests — Open Threads questions.md` under *Windows — for the runner's separate batch*, and the machine's own edition must be read and recorded before anything is claimed from it: every Windows computer this project has read turned out to be a Server edition, including one whose image was named for a desktop edition.

---

2026-08-29 — Claude Opus 5 (Claude Code) — Reports, Blender 4.5

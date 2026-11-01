# Reports — Open Threads questions — Blender 5.0

What this is: one researcher's answer to question 14 of the Reports question sheet, for Blender 5.0. The sheet's other questions are answered in other documents in this folder.

Sources consulted, all on 2026-07-30 (the snapshot date):

- The Blender 5.0 manual (plain-text snapshot), pages under https://docs.blender.org/manual/en/5.0/
- The Blender 5.0 Python API documentation (plain-text snapshot), pages under https://docs.blender.org/api/5.0/
- The Blender 5.0 release notes (plain-file snapshot), pages under https://developer.blender.org/docs/release_notes/5.0/
- The Cycles files supplied in `Blender Sources/cycles_5.0`
- One manual image, `_images/advanced_command-line_launch_linux_console.png`

I read only the question sheet and files inside Blender Sources/. I did not open any other part of this project.

---

## 14. When a render's printed output is sent somewhere other than a terminal, what decides when each line of it becomes available to whatever is reading?

**Answer: not in the sources.**

**LABEL: not in the sources**

The three documentation sets and the Cycles files supplied say nothing about when a printed line becomes available to a reader that is not a terminal. Nothing in them mentions buffering of Blender's printed output, flushing it, holding it back, or behaving differently depending on whether the destination is a terminal, a file or a pipe. The word "stdout" does not appear anywhere in the 5.0 manual, the 5.0 API documentation or the 5.0 release notes at all.

### The reading I answered, and the other reading

The question is ambiguous about what "sent somewhere other than a terminal" means. I answered the reading I think is meant: the output Blender prints is captured by the operating system — redirected to a file, or read through a pipe by another program — instead of appearing in a terminal window, and the question is what governs the delay between Blender printing a line and the reader being able to see it.

The other reading is Blender's own `--log-file` option, where Blender itself writes its log to a file rather than the console. I searched for that too, and the sources are equally silent about timing there. The manual gives the option one line and nothing more:

> `--log-file <filepath>`
>
> Set a file to output the log to.

- Source: the manual, https://docs.blender.org/manual/en/5.0/advanced/command_line/arguments.html (Logging Options)
- Date read: 2026-07-30

That line says a file can be the destination. It says nothing about when a line reaches that file.

### The nearest thing the sources do say

The manual describes the destination only in the terminal case, and only in terms of where output appears, never when:

> When Blender is manually started from a terminal, Blender output is shown in the Console Window it is started from.

- Source: the manual, https://docs.blender.org/manual/en/5.0/advanced/command_line/index.html
- Date read: 2026-07-30

The release notes state that render progress now goes through Blender's shared logging system, but they describe format and level control, not timing:

> Most of Blender now uses the same logging system,
> including operators reports and background render progress.
> The output format has changed, and log levels are easier to control.

- Source: the release notes, https://developer.blender.org/docs/release_notes/5.0/core/ (Logging)
- Date read: 2026-07-30

The same page lists the log controls that exist — `--log-level`, `--log-show-memory`, `--log-show-source`, `--log-list-categories` — and nothing written about any of them on that page mentions when a line is handed over. There is no option in that list, or in the manual's Logging Options section, for unbuffered or line-by-line output.

One adjacent option exists and I deliberately did **not** build an answer on it. The manual says of `--python-use-system-env`: "Allow Python to use system environment variables such as PYTHONPATH and the user site-packages directory" (same arguments page). Reading anything about output buffering into that would require me to supply the fact that some other environment variable controls Python's output buffering, which no supplied source states. So I stopped.

### Where I looked

Searched, case-insensitively, across the whole of `manual_5.0`, `api_5.0` and `release_notes_5.0` at once:

- `stdout` — no hits anywhere in any of the three.
- `stderr` — no hits in the manual, API docs or release notes. Two hits in the Cycles files (below).
- `unbuffered`, `buffering`, `line buffer`, `line-buffer`, `isatty`, `setvbuf`, `PYTHONUNBUFFERED` — no hits anywhere.
- `buffer` / `buffered` — many hits, all about image buffers, frame buffers, GPU buffers, the animation-player `-m` option ("Read from disk (Do not buffer)", which is about caching image frames during playback, not about printed output), copy/paste buffers and the macOS text-editor instruction "Enter Y to save the modified buffer". None about printed output.
- `flush` — hits only for the String to Curves node, text object properties, manual icon guidelines, `bmesh` selection-flush functions, and one bug-fix line about scene strip rendering flushing user edits. None about output.
- `redirect` — only a node operator option and an icon name.
- `standard output` / `standard error` / `output stream` / `text stream` — no relevant hits ("Standard output image" on a compositor page is unrelated).
- `carriage return`, `same line`, `line by line`, `real-time output`, `appears immediately`, `as it happens`, `piping`, `piped` — no relevant hits.
- `log file` / `logfile` / `log-file` — only the one `--log-file` manual line quoted above.
- `sys.stdout` / `sys.stderr` / `sys.stdin` — one hit only, under `-d, --debug`: "Keeps Python's sys.stdin rather than setting it to None". That is input, not output.

Pages I opened in full and ruled out:

- https://docs.blender.org/manual/en/5.0/advanced/command_line/index.html — describes what the console window is and why you would use the command line. Nothing on timing or non-terminal destinations.
- https://docs.blender.org/manual/en/5.0/advanced/command_line/arguments.html — the complete argument list, including the Logging, Debug and Python sections and the Environment Variables list. No buffering or flushing control, and no environment variable relating to output timing.
- https://docs.blender.org/manual/en/5.0/advanced/command_line/render.html — command-line rendering workflows. Covers argument order and example commands only; no mention of capturing or reading the output.
- https://docs.blender.org/manual/en/5.0/advanced/command_line/launch/linux.html, .../macos.html, .../windows.html — all three launch pages. They tell you how to start Blender so output is printed to a terminal. None discusses redirecting it.
- https://docs.blender.org/manual/en/5.0/troubleshooting/crash.html — mentions Windows batch scripts that obtain "the Blender debug log and system info text files", which is output ending up in a file. It says nothing about when lines arrive there.
- https://docs.blender.org/manual/en/5.0/troubleshooting/python.html — says script output "will show in Command Line Window", nothing further.
- https://developer.blender.org/docs/release_notes/5.0/core/ — the Logging section, quoted above, read in full.
- https://developer.blender.org/docs/release_notes/5.0/python_api/ — the Logging and Context Logging sections. About categories and formats, not delivery timing.

Cycles files searched (`Blender Sources/cycles_5.0`, all 403 files):

- `fflush`, `std::cout`, `std::cerr`, `endl`, `sys.stdout`, `sys.stderr`, `isatty`, `setvbuf` — no hits.
- `stderr` — two hits, neither about buffering. `cycles_5.0/engine.py`, line 14, in `_configure_argument_parser`, the help text of `--cycles-print-stats`: `help="Print rendering statistics to stderr",`. And `cycles_5.0/source/util/defines.h`, line 127, a comment about a Metal debug environment variable. Both name a destination; neither says when a line reaches a reader.
- The supplied Cycles tree contains only the add-on Python files and `source/kernel` and `source/util`. There is no session, progress or output-printing code in it, so it cannot be examined for how progress lines are written or flushed. I checked the file list to confirm this rather than assuming it.

Image ruled out: `_images/advanced_command-line_launch_linux_console.png`, which sits on the manual's Linux launch page, https://docs.blender.org/manual/en/5.0/advanced/command_line/launch/linux.html (read 2026-07-30). I opened it. It is a still screenshot of a Konsole window showing Blender's startup messages about a missing jack server. A still picture of a terminal cannot show when a line reached a non-terminal reader, and it shows no setting or option bearing on that.

---

### Answered by hand test, 2026-08-30

**LABEL: observed.** Run by Claude Opus 5 (Claude Code) on 2026-08-30, in Blender 5.0.1, on two machines. The paragraphs above still stand: the sources say nothing. What follows is what a running Blender did. The full list, with every command written out and the raw output of every run, is `Audits/Blender Research/Reports/Hand tests — Open Threads questions.md`.

**The two machines, and the answers are never merged.**

- **The Creator's own Mac** — a MacBook Pro with an Apple M1 Pro chip, eight processor cores, 16 GB of memory, macOS 26.6.2, Apple's own graphics and no second card. Blender 5.0.1 at `/Applications/Blender 5.0.1.app/Contents/MacOS/Blender`, build hash `a3db93c5b259`.
- **A rented Linux machine** — Vast.ai instance `49254257` in the United Kingdom, a container rather than a whole machine, Ubuntu 22.04.4 LTS on kernel `6.8.0-124-generic`, an Intel Core i7-11800H with sixteen visible threads against a quota of 15.36 cores, a memory ceiling of about 15.8 GB, one NVIDIA GeForce RTX 3070 Laptop GPU on driver 535.309.01 that no test used, 32 GB of disk, and no display server. Blender 5.0.1 at `/opt/blender/blender-5.0.1-linux-x64/blender`, taken from Blender's own download server as the Linux x64 build, build hash `a3db93c5b259` — the same build as the Mac's. Rented and destroyed the same day.

**What was set up.** One file, `scene.blend`: the factory startup scene — one cube, one lamp, one camera — set to Cycles on the processor, 900 samples, adaptive sampling and denoising off, 720 by 405 at 100 per cent, frames 1 to 5, saving PNG files to `/tmp/r14out/f_`. It was built with Blender 4.5.12 so that every version opens it without converting anything. One frame takes about fifteen seconds on either machine. Every run carried `--factory-startup`, both to leave the Creator's own settings alone and because the Mac has third-party add-ons whose messages would otherwise have been read as Blender's own.

**How arrival was measured.** A reader that looks only after a run has ended cannot tell a line held back from a line printed promptly, so a small program (`reader.py`) was put on the far end of the pipe. It reads raw bytes as they arrive and prints, for each arrival, the seconds since it started, how many bytes arrived, and the first seventy bytes of them. Promptly-arriving output shows as many small arrivals timed to the run's own events; held-back output shows as few arrivals of one fixed size at long intervals. **That the instrument can show the second pattern was proved on both machines**, by putting a program between Blender and the reader that deliberately held everything in 4096-byte blocks. That calibration was run with Blender 4.5.12, since it measures the reader rather than Blender. On the Mac it gave four arrivals, at 0.559, 24.487, 46.278 and 75.931 seconds, of 4096, 4096, 4096 and 2709 bytes; on the rented Linux machine, four arrivals at 0.256, 18.103, 36.024 and 54.115 seconds, of 4096, 4096, 4096 and 2599 bytes.

**1. A render's own printed lines reach a pipe as they are produced.** Command, on both machines: `<blender> -b --factory-startup scene.blend -s 1 -e 3 -a 2>&1 | python3 reader.py`. On the Mac, 82 arrivals over 40.671 seconds. The last eight, word for word:

```
  27.486  chunk  76     228 bytes  total   10387  b'00:27.282  render           | Fra: 3 | Mem: 1M | Updating Lookup Table'
  27.486  chunk  77      62 bytes  total   10449  b'00:27.282  render           | Fra: 3 | Mem: 1M | Sample 0/900\n'
  27.498  chunk  78      84 bytes  total   10533  b'00:27.295  render           | Fra: 3 | Remaining: 00:10.99 | Mem: 5M |'
  40.618  chunk  79      64 bytes  total   10597  b'00:40.415  render           | Fra: 3 | Mem: 5M | Sample 900/900\n'
  40.618  chunk  80      58 bytes  total   10655  b'00:40.415  render           | Fra: 3 | Mem: 5M | Finished\n'
  40.633  chunk  81     126 bytes  total   10781  b"00:40.429  render           | Saved: '/tmp/r14out/f_0003.png'\n00:40.42"
  40.666  chunk  82      14 bytes  total   10795  b'\nBlender quit\n'
  40.671  END  82 arrivals, 10795 bytes total
```

The 75 arrivals before these are cut for length; they are the same shape, arriving in bursts as each frame finishes. On the rented Linux machine, 56 arrivals over 52.172 seconds, ending:

```
  52.107  chunk  53     122 bytes  total   10545  b'00:52.040  render           | Fra: 3 | Mem: 5M | Sample 900/900\n00:52.'
  52.133  chunk  54      62 bytes  total   10607  b"00:52.066  render           | Saved: '/tmp/r14out/f_0003.png'\n"
  52.133  chunk  55      64 bytes  total   10671  b'00:52.066  render           | Time: 00:17.21 (Saving: 00:00.02)\n'
  52.165  chunk  56      14 bytes  total   10685  b'\nBlender quit\n'
  52.172  END  56 arrivals, 10685 bytes total
```

**The two machines agreed.**

**2. A pipe is no different from a terminal for those lines.** The same render was run again with a real terminal in place of the pipe, supplied by the system's own `script` command. On the Mac, 54 arrivals ending at 54.208 seconds; on the rented Linux machine, 71 ending at 52.312. The arrival pattern is the same as in test 1 on each machine. The only difference in what arrived is that every line ends `\r\n` instead of `\n`, which is the terminal's doing. The last four arrivals on the Mac:

```
  54.147  chunk  52     124 bytes  total   10801  b'00:53.941  render           | Fra: 3 | Mem: 5M | Sample 900/900\r\n00:53'
  54.162  chunk  53     128 bytes  total   10929  b"00:53.955  render           | Saved: '/tmp/r14out/f_0003.png'\r\n00:53.9"
  54.199  chunk  54      16 bytes  total   10945  b'\r\nBlender quit\r\n'
  54.208  END  54 arrivals, 10945 bytes total
```

**3. The same lines reach a plain file as they are produced.** Command: `<blender> -b --factory-startup scene.blend -s 1 -e 3 -a > out.txt 2>&1`, with the file's size read every quarter second from a second process while the render was still going. On the Mac:

```
   0.000  file is       0 bytes
   0.776  file is    3543 bytes
  18.509  file is    7038 bytes
  43.061  file is   10533 bytes
  58.180  file is   10795 bytes
  58.182  run finished, file is   10795 bytes
```

On the rented Linux machine:

```
   0.000  file is       0 bytes
   0.250  file is      62 bytes
   0.501  file is    3433 bytes
  17.781  file is    6928 bytes
  35.318  file is   10423 bytes
  52.852  file is   10685 bytes
  52.852  run finished, file is   10685 bytes
```

**The steps are the frames.** The file grows once per frame, at the moment that frame finishes.

**4. The progress readout is whole separate lines, not one line rewritten.** Counting the line endings in the file test 3 wrote: on the Mac, `newlines: 146   carriage returns: 0   bytes: 10795`; on the rented Linux machine, `newlines: 146   carriage returns: 0   bytes: 10685`. No carriage returns at all.

**5. A Python script's own printing does not reach a pipe or a file until the run ends, and this is the one place the answer splits.** A script printing one line a second for fifteen seconds (`print("tick %d" % i)` and then a one-second wait) was run with `<blender> -b --factory-startup --python ticker.py 2>&1 | python3 reader.py`. On the Mac:

```
  15.689  chunk   1     111 bytes  total     111  b'tick 1\ntick 2\ntick 3\ntick 4\ntick 5\ntick 6\ntick 7\ntick 8\ntick 9\ntick 10'
  15.723  chunk   2      60 bytes  total     171  b'Blender 5.0.1 (hash a3db93c5b259 built 2025-12-16 01:51:40)\n'
  15.725  chunk   3      14 bytes  total     185  b'\nBlender quit\n'
  15.731  END  3 arrivals, 185 bytes total
```

All fifteen lines arrived together at the end, in one arrival. On the rented Linux machine:

```
  15.265  chunk   1     111 bytes  total     111  b'tick 1\ntick 2\ntick 3\ntick 4\ntick 5\ntick 6\ntick 7\ntick 8\ntick 9\ntick 10'
  15.294  chunk   2      60 bytes  total     171  b'Blender 5.0.1 (hash a3db93c5b259 built 2025-12-16 01:30:59)\n'
  15.297  chunk   3      14 bytes  total     185  b'\nBlender quit\n'
  15.302  END  3 arrivals, 185 bytes total
```

Sent to a file instead of a pipe, the file stayed at zero bytes throughout and reached its full size in one step at the end — on the Mac:

```
   0.000  file is       0 bytes
  16.206  file is     185 bytes
  16.206  run finished, file is     185 bytes
```

and on the rented Linux machine:

```
   0.000  file is       0 bytes
  15.280  file is     185 bytes
  15.530  run finished, file is     185 bytes
```

Three further runs on each machine:

- **With `PYTHONUNBUFFERED=1` set**, the result was unchanged. The first arrival on the Mac was `15.927  chunk   1     111 bytes  total     111  b'tick 1\ntick 2\ntick 3\ntick 4\ntick 5\ntick 6\ntick 7\ntick 8\ntick 9\ntick 10'` and on Linux `15.236  chunk   1     111 bytes  total     111  b'tick 1\ntick 2\ntick 3\ntick 4\ntick 5\ntick 6\ntick 7\ntick 8\ntick 9\ntick 10'`.
- **With each line printed as `print("tick %d" % i, flush=True)`**, the lines arrived one a second through the same pipe. The first four arrivals on the Mac — the remaining eleven follow the same one-a-second pattern and are cut for length:

  ```
     0.626  chunk   1       7 bytes  total       7  b'tick 1\n'
     1.636  chunk   2       7 bytes  total      14  b'tick 2\n'
     2.646  chunk   3       7 bytes  total      21  b'tick 3\n'
     3.648  chunk   4       7 bytes  total      28  b'tick 4\n'
  ```

  On the rented Linux machine the first four were:

  ```
     0.225  chunk   1       7 bytes  total       7  b'tick 1\n'
     1.225  chunk   2       7 bytes  total      14  b'tick 2\n'
     2.226  chunk   3       7 bytes  total      21  b'tick 3\n'
     3.226  chunk   4       7 bytes  total      28  b'tick 4\n'
  ```

- **With a real terminal in place of the pipe**, the plain script, with no flushing, also ticked once a second, and the version banner arrived first rather than last. The first four arrivals on the Mac:

  ```
     0.000  chunk   1       4 bytes  total       4  b'^D\x08\x08'
     0.433  chunk   2      61 bytes  total      65  b'Blender 5.0.1 (hash a3db93c5b259 built 2025-12-16 01:51:40)\r\n'
     0.931  chunk   3       8 bytes  total      73  b'tick 1\r\n'
     1.940  chunk   4       8 bytes  total      81  b'tick 2\r\n'
  ```

  And on the rented Linux machine:

  ```
     0.067  chunk   1      61 bytes  total      61  b'Blender 5.0.1 (hash a3db93c5b259 built 2025-12-16 01:30:59)\r\n'
     0.225  chunk   2       8 bytes  total      69  b'tick 1\r\n'
     1.226  chunk   3       8 bytes  total      77  b'tick 2\r\n'
     2.226  chunk   4       8 bytes  total      85  b'tick 3\r\n'
  ```

**6. Everything a render prints goes to the ordinary output stream; the error stream carried nothing.** With only the error stream fed to the reader — `<blender> -b --factory-startup scene.blend -f 1 -- --cycles-print-stats 2>&1 1>/dev/null | python3 reader.py` — the whole run gave, on the Mac, `28.372  END  0 arrivals, 0 bytes total`, and on the rented Linux machine, `17.892  END  0 arrivals, 0 bytes total`. Keeping the two streams apart in files, `<blender> -b --factory-startup scene.blend -f 1 -- --cycles-print-stats > s_out.txt 2> s_err.txt`, gave on the Mac `ordinary output: 7014 bytes    error stream: 0 bytes` and on Linux `ordinary output: 6906 bytes    error stream: 0 bytes`. **The Cycles statistics were in the ordinary output**, under headings reading `Update statistics:`, `Render statistics:` and `Mesh statistics:` — not in the error stream, which is where the help text quoted above says they go.

**7. The file written by `--log-file` fills in as the run goes.** Command: `<blender> -b --factory-startup scene.blend --log "*" --log-level info --log-file run.log -s 1 -e 2 -a > /dev/null 2>&1`. The logging-level value was read off `blender --help` rather than guessed. On the Mac:

```
   0.000  file is       0 bytes
   0.513  file is     128 bytes
   1.027  file is    6428 bytes
  25.706  file is    8151 bytes
  25.966  file is   13809 bytes
  44.166  file is   15657 bytes
  44.167  run finished, file is   15657 bytes
```

On the rented Linux machine:

```
   0.000  file is       0 bytes
   0.252  file is    6073 bytes
  16.781  file is   13471 bytes
  33.565  file is   15348 bytes
  33.816  run finished, file is   15348 bytes
```

**8. The same split shown inside one run.** So that nothing could be blamed on two runs differing, one run printed both kinds of line into one pipe: `<blender> -b --factory-startup scene.blend --python-expr "print('SCRIPT LINE PRINTED AT START')" -s 1 -e 2 -a 2>&1 | python3 reader.py`. On the Mac the render's own lines arrived throughout, and:

```
   0.703  chunk   1     172 bytes  total     172  b'00:00.404  blend            | Read blend: "/private/tmp/claude-501/-Us'
  23.168  chunk  20     174 bytes  total    3839  b"00:22.869  render           | Saved: '/tmp/r14out/f_0001.png'\n00:22.86"
  44.079  chunk  45      62 bytes  total    7222  b"00:43.780  render           | Saved: '/tmp/r14out/f_0002.png'\n"
  44.086  chunk  47      29 bytes  total    7315  b'SCRIPT LINE PRINTED AT START\n'
  44.116  END  48 arrivals, 7329 bytes total
```

The arrivals not shown are the render's own, spread across the run. The script's line was printed within the first second and reached the reader at 44.086 seconds, after both frames were finished. **This run was done on the Mac only for 5.0** — the rented Linux machine had already been destroyed when it was added. The same run in 4.5 and in 5.2 was done on both machines, and the two machines agreed there.

**9. Nothing already printed is lost when a run is stopped part-way.** A three-frame render was sent to a file and stopped at twenty-two seconds, once with an interrupt and once with an outright kill, which cannot flush anything a program is still holding. On the Mac, before the interrupt the file held 3543 bytes ending `00:00.296  render           | Fra: 1 | Remaining: 00:11.44 | Mem: 5M | Sample 1/900`; afterwards 3800 bytes ending:

```
00:26.449  render           | Fra: 1 | Mem: 5M | Cancelled

Blender quit
```

Before the outright kill the file held 3543 bytes; afterwards it held **the same 3543 bytes**, ending on the most recent line the run had printed:

```
00:00.328  render           | Fra: 1 | Mem: 1M | Updating Device | Writing constant memory
00:00.328  render           | Fra: 1 | Mem: 1M | Sample 0/900
00:00.347  render           | Fra: 1 | Remaining: 00:16.72 | Mem: 5M | Sample 1/900
```

Nothing was lost. **This one was run on the Mac only for 5.0**, for the same reason as test 8. In 4.5 and 5.2 it was run on both machines and nothing was lost on either.

**10. The build's own help list names no switch for any of this.** Running `blender --help` and searching the whole printed list for `buffer`, `flush`, `unbuffer`, `stdout` and `stderr` gives exactly one hit in Blender 5.0.1: `Read from disk (Do not buffer).`, which is the animation player's `-m` argument and concerns reading picture files, not writing output. The same single hit, and nothing else, in the other three versions. So the installed build lists nothing the manual's argument page omits.

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

### Not covered

This answer settles only that the supplied sources are silent. It does not settle what Blender 5.0 actually does — output may well be held back in blocks when the destination is a pipe or file and released line by line to a terminal, or it may not, and the sources give no basis to say either way. It also does not settle whether the answer differs between the streams (whatever Blender prints as ordinary output versus errors), between Blender's own `--log-file` writing and operating-system redirection, or between what Blender's C++ side prints and what a Python script's `print()` produces. Nor does it settle whether any environment variable or command-line option not documented in these sources changes the behaviour.

### Outside the interface

The question is already about the outside-the-interface case — output being captured by another program rather than watched by a person. The sources are silent for that case, which is the case that matters, and they are equally silent for the in-window case. The manual documents only that output appears in the console window when Blender is started from a terminal by hand (https://docs.blender.org/manual/en/5.0/advanced/command_line/index.html). It offers no statement about a reader that is not a terminal, so there is nothing here that holds inside but fails outside, or the reverse — there is simply nothing.

**LABEL: observed** (2026-08-30). The hand test above answers this, and every run in it was Blender driven from outside its own window — started with no window at all, its output read by another program. A render's own printed lines reach a pipe and a plain file as they are produced, at the same moments as they reach a terminal; a Python script's `print()` lines do not reach a pipe or a file until the run ends, though they do reach a terminal one a second. So the behaviour asked about holds outside the interface, and it differs by *which part of Blender did the printing* rather than by whether a person is watching.

### Operating system

The sources do not say. Nothing in the manual, the API documentation, the release notes or the Cycles files states any per-operating-system difference in when printed output becomes available to a reader, because they do not address the subject at all. The manual's only operating-system-specific remarks about output concern where the console window comes from — that on Microsoft Windows the Console Window is created as a separate window and then toggled off, and is reopened with Window > Toggle System Console (https://docs.blender.org/manual/en/5.0/advanced/command_line/launch/windows.html) — which is about the window, not about timing. Whether the answer differs from one operating system to another is therefore not settled either way here, and a hand test is the only route to it.

**LABEL: observed** (2026-08-30). Tests 1 to 7 in the hand-test section above were each run twice, once on macOS 26.6.2 on Apple silicon and once on Ubuntu 22.04.4 on x86-64, on two builds carrying the same build hash `a3db93c5b259`, and **the two machines agreed on every one of them**. Tests 8 and 9 were run on the Mac only, because the rented machine had already been destroyed when they were added; in 4.5 and 5.2 both were run on both machines and the two agreed there. Test 10, the reading of the build's own help list, was run on the Mac only. **Windows is not covered.** It was not run — it is written out ready to run in `Audits/Blender Research/Reports/Hand tests — Open Threads questions.md` and is held for the single Windows batch this project runs across every topic. Whether the answer is the same there is open.

---

## Things a person with Blender open could settle by hand in a few minutes

**Pruned 2026-08-30 by the hand tests.** Eight items stood here. **Items 1 to 7 were all run on 2026-08-30**, on the Creator's Mac and on a rented Linux machine, and their answers are in the observed section above — output through a pipe, output to a file, the terminal case for comparison, `--log-file`, the two streams, a script's own printing, and whether an environment variable changes it. They are not repeated here.

**One item is still open, and only in part.**

1. **The Windows half.** The items above that asked for the pipe and file tests to be repeated on Windows were run on macOS and on Linux, where the two agreed; **Windows was not run.** This project has a route to a Windows computer — a rental used four times — but the account allows one machine at a time and the way in costs about ninety minutes, so one Windows batch is run across every topic rather than each topic renting in turn. The Windows runs are written out in full and ready, in `Audits/Blender Research/Reports/Hand tests — Open Threads questions.md` under *Windows — for the runner's separate batch*, and the machine's own edition must be read and recorded before anything is claimed from it: every Windows computer this project has read turned out to be a Server edition, including one whose image was named for a desktop edition.

---

2026-08-29 — Claude Opus 5 (Claude Code) — Reports, Blender 5.0

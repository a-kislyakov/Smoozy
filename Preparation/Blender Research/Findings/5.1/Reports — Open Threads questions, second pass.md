# Reports — Open Threads questions, second pass (Blender 5.1)

**Sources consulted, and on what date.** I consulted only the Blender 5.1 manual (`Blender Sources/manual_5.1`, including its `_images` folder), the Blender 5.1 Python API documentation (`Blender Sources/api_5.1`), the Blender 5.1 release notes (`Blender Sources/release_notes_5.1`), and Cycles' own files for 5.1 (`Blender Sources/cycles_5.1`). All of these were taken on 2026-07-30, and that is the date read for every finding below. I did the searching and reading on 2026-08-30.

I read only the question sheet and files inside Blender Sources/. I did not open any other part of this project.

I worked alone. I started no helper agents, so there is nobody else's work to vouch for.

Scope: question 14 only, as instructed. No other question on the sheet was answered or looked into.

---

## 14. When a render's printed output is sent somewhere other than a terminal, what decides when each line of it becomes available to whatever is reading?

**How I read the question.** I took "sent somewhere other than a terminal" to mean the ordinary cases of the output being captured rather than shown on screen: written to a file, or fed down a pipe to another program that is reading it. I took "what decides when each line becomes available" to mean the timing question — whether a line that Blender has printed can be read straight away, or is held back and delivered later in a batch, and what governs that.

The question is slightly ambiguous in one respect, and I flag it rather than choose silently. It could also be read as asking which *mechanism inside Blender* is responsible (an option, a setting, an environment variable). I answered the timing reading, because that is what "when each line becomes available to whatever is reading" says. Under the other reading the answer is the same in outcome: I found no option, setting or environment variable in the sources that governs this either. The one documented mechanism for sending output somewhere other than a terminal is quoted below, and it says nothing about timing.

**LABEL: not in the sources.**

The 5.1 manual, the 5.1 Python API documentation, the 5.1 release notes, and the Cycles 5.1 files supplied contain no statement about how Blender's printed output is buffered, whether it is delivered a line at a time or in blocks, whether Blender behaves differently when its output is a terminal versus a file or a pipe, or what a reader on the other end of a pipe sees and when. There is no mention anywhere in these sources of buffering of output, flushing of output, of a "tty", of standard output or standard error by those names, or of output redirection.

The closest thing the sources do contain is a documented way to send output somewhere other than a terminal. It is stated as a bare capability, with nothing about timing:

> "--log-file <filepath>
>
> Set a file to output the log to."

— Manual, Command Line Arguments, Logging Options. Address: https://docs.blender.org/manual/en/5.1/advanced/command_line/arguments.html — date read 2026-07-30.

And one documented entry for the option that produces that particular kind of report, which I found while searching. It says what the option logs, and it names neither a stream nor a moment: *(Corrected by the citation check of 2026-08-30: this sentence had said the manual line was a statement "about which stream a particular kind of report goes to". The quoted manual words name no stream. The only place a stream is named is the Cycles file quoted immediately below.)*

> "--cycles-print-stats
>
> Log statistics about render memory and time usage."

— Manual, Command Line Arguments, Cycles Render Options. Address: https://docs.blender.org/manual/en/5.1/advanced/command_line/arguments.html — date read 2026-07-30.

The Cycles add-on's own file names the stream for that same option:

> `    parser.add_argument("--cycles-print-stats",`
> `                        help="Print rendering statistics to stderr",`
> `                        action='store_true')`

— File `cycles_5.1/engine.py`, lines 13–15, inside the function that builds the Cycles add-on's command line argument parser. Date read 2026-07-30. Label for this quoted line on its own: **observed** (it is what the file says). I draw no conclusion from it about buffering or timing — going from "this goes to stderr" to any claim about when a reader sees it would require knowledge from outside these sources, which I do not have and have not used.

The manual does describe where output goes when Blender *is* started from a terminal, which is the case the question excludes:

> "When Blender is manually started from a terminal, Blender output is shown in the Console Window it is started from."

— Manual, Using Blender From The Command Line. Address: https://docs.blender.org/manual/en/5.1/advanced/command_line/index.html — date read 2026-07-30. This is the terminal case, so it does not answer the question.

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

**Not covered.** This answer settles only that the supplied sources are silent. It does not settle what Blender actually does — output printed by a running Blender may well reach a file or a pipe promptly, or in batches, and I cannot tell you which from these sources. It does not settle whether the behaviour differs between the parts of Blender that print (the render engine's progress reports, the logging categories reached by `--log`, and anything a Python script prints), and it does not settle whether `--log-file` writes are governed by anything different from output sent to a terminal. It also does not settle whether `--log-file` captures everything a render prints or only the logging categories; the manual does not say.

**Outside the interface.** The sources do not say. Since they say nothing about output buffering at all, they say nothing about it for a Blender started and driven by another program either. Note that the question itself is mostly a question about that case — output goes somewhere other than a terminal precisely when something else is capturing it — and the documentation does not reach it. The nearest documented facts are that Blender can be run without its interface (`-b`, "Render in the background (without UI)", from the Rendering From The Command Line page, https://docs.blender.org/manual/en/5.1/advanced/command_line/render.html — the Command Line Arguments page words the same switch differently, as "Run in background (often used for UI-less rendering)") and that its log can be pointed at a file, from the Command Line Arguments page cited above. Neither says anything about timing. *(Address corrected by the citation check of 2026-08-30: the words quoted here are on the render page, not the arguments page this line had cited.)*

**LABEL: observed** (2026-08-30). The hand test above answers this, and every run in it was Blender driven from outside its own window — started with no window at all, its output read by another program. A render's own printed lines reach a pipe and a plain file as they are produced, at the same moments as they reach a terminal; a Python script's `print()` lines do not reach a pipe or a file until the run ends, though they do reach a terminal one a second. So the behaviour asked about holds outside the interface, and it differs by *which part of Blender did the printing* rather than by whether a person is watching.

**Operating system.** The sources do not say. There is no statement in the 5.1 manual, API documentation or release notes that output buffering behaviour differs, or does not differ, between Linux, macOS and Windows. The manual's per-platform launch pages describe only how to start Blender from a terminal on each system and where the console window comes from on Windows ("the Console Window (called the Command Prompt) is first created as a separate window on the desktop", Manual, Launching from the Command Line — Windows, https://docs.blender.org/manual/en/5.1/advanced/command_line/launch/windows.html, date read 2026-07-30). That is about a terminal window, not about a file or a pipe, so it does not bear on the question.

**LABEL: observed** (2026-08-30). Tests 1 to 7 in the hand-test section above were each run twice, once on macOS 26.6.2 on Apple silicon and once on Ubuntu 22.04.4 on x86-64, on two builds carrying the same build hash `ec6e62d40fa9`, and **the two machines agreed on every one of them**. Tests 8 and 9 were run on the Mac only, because the rented machine had already been destroyed when they were added; in 4.5 and 5.2 both were run on both machines and the two agreed there. Test 10, the reading of the build's own help list, was run on the Mac only. **Windows is not covered.** It was not run — it is written out ready to run in `Audits/Blender Research/Reports/Hand tests — Open Threads questions.md` and is held for the single Windows batch this project runs across every topic. Whether the answer is the same there is open.

---

### Record of where I looked, for question 14

Everything below was run over `Blender Sources/manual_5.1`, `Blender Sources/api_5.1`, `Blender Sources/release_notes_5.1` and `Blender Sources/cycles_5.1`, whole-folder, case-insensitive, before opening any individual page.

**Terms searched across the whole manual, whole API documentation and whole release notes** (result in brackets):

- `stdout` (0 hits anywhere), `stderr` (0 in manual, 0 in API, 0 in release notes; 2 in Cycles files — see below), `stdio` (0), `standard error` (0), `standard output` (1 hit, in the Color Balance compositing node page, about a colour-grading formula — unrelated)
- `unbuffered` (0), `line buffer` (0), `setvbuf` (0), `fflush` (0), `isatty` (0), `tty` as a whole-word idea (the only hits are the letters inside "pretty", e.g. `bpy.app.debug_depsgraph_pretty`, and inside SVG figure text — no hit means a terminal device)
- `buffer` (30 files in the manual, 54 in the API documentation). I checked every manual hit and the API hits by category. They are all about other kinds of buffer: the render buffer, framebuffers and vertex buffer objects, the audio mixing buffer, the video decoder bitstream buffer, a Python `create_string_buffer` in a troubleshooting snippet, the number of lines "buffered in memory of the console window" in System Preferences, and a text-editing buffer in the macOS instructions (`Enter Y to save the modified buffer`, which is about the `nano` editor). None concern program output buffering.
- `buffered` (2 files, both covered above), `flush` / `flushed` / `flushing` (all hits are about flushing *selection* in BMesh, or flushing animation changes to other views — nothing about output)
- `pipe` / `piped` (all hits are about pipeline, pipes as geometry, "Pipeline & I/O", or a node "pipe" — none about a process pipe), `redirect` (2 hits, both about node editor redirection and a manual-writing icon list)
- `terminal` (10 manual files: the command-line index, the three per-platform launch pages, contribute/build pages, the Linux install page, and the add-on tutorial. All read or skimmed. They describe starting Blender from a terminal and seeing output there; none discuss output sent elsewhere.)
- `console` (checked alongside `terminal`; the substantive pages are the same ones, plus `editors/python_console.txt` and `editors/preferences/system.txt`)
- `log file`, `output.log`, `carriage return`, `same line`, `overwrite the line`, `in real time`, `real-time output`, `as it happens`, `line by line`, `written directly`, `chunk`, `not appear until`, `appear until`, `when the process` — all 0 hits, or hits unrelated to output (`same line` matched a Grease Pencil Line Art property)
- `subprocess` (3 hits, all about GPU shader compilation subprocesses in System Preferences and `--gpu-compilation-subprocesses`; nothing about capturing Blender's output), `Popen` (0), `os.system` (0), `nohup` (0), `daemon` (only Python threading and the 3D-mouse daemon socket)
- Shell-redirection shapes in example commands: searched for `2>&1`, `> log`, `>> `, `tee ` and `blender ... >`. The only real match is `2>&1` in the manual-translation build instructions (`contribute/translate_manual/add_language.txt`), which is about building the manual, not about Blender's render output. No example anywhere in the manual redirects a render's output to a file.
- A final combined pattern covering the concept in the phrasings documentation tends to use — buffering of output/stdout/stderr/console, line-buffered, block-buffered, unbuffered, flushing output/log, "not a tty", "is a terminal", "when connected/attached to a terminal", "piped to", "redirected to" — returned exactly one hit across all three document sets: an audio device description in `aud` ("a file output or RAM buffer output"). Unrelated.

**Pages opened and read in full or in relevant part:**

- Command Line Arguments — https://docs.blender.org/manual/en/5.1/advanced/command_line/arguments.html . I read the Render Options, Cycles Render Options, Animation Playback Options, Python Options, Logging Options, Debug Options, GPU Options, Misc Options and the whole Environment Variables list. Logging Options is where `--log`, `--log-level`, `--log-show-memory`, `--log-show-source`, `--log-show-backtrace`, `--log-file` and `--log-list-categories` live; none of their descriptions mention timing or buffering. The Environment Variables list (all of `BLENDER_USER_*`, `BLENDER_SYSTEM_*`, `BLENDER_CUSTOM_SPLASH*`, `BLENDER_OCIO`, `SPNAV_SOCKET`, `TEMP`, `TMPDIR`) contains no variable affecting output delivery, and no Python-related output variable is documented. The one hit for "Do not buffer" on this page is the animation player's `-m` argument, "Read from disk (Do not buffer)", which is about reading image frames from disk during playback, not about printed output — ruled out.
- Using Blender From The Command Line — https://docs.blender.org/manual/en/5.1/advanced/command_line/index.html . Read in full. Describes the Console Window and lists use cases including rendering animation and automation, but only in terms of a terminal.
- Rendering From The Command Line — https://docs.blender.org/manual/en/5.1/advanced/command_line/render.html . Read in full. Gives `blender -b file.blend -f 10`, `-a`, `-E CYCLES`, argument-order warnings. No mention of capturing or redirecting output.
- Launching from the Command Line, all three platform pages: Linux https://docs.blender.org/manual/en/5.1/advanced/command_line/launch/linux.html , macOS https://docs.blender.org/manual/en/5.1/advanced/command_line/launch/macos.html , Windows https://docs.blender.org/manual/en/5.1/advanced/command_line/launch/windows.html . All read in full, specifically to see whether any platform page said anything about output going somewhere other than the terminal. None does.
- Preferences — System — https://docs.blender.org/manual/en/5.1/editors/preferences/system.html , the "Console" area (the setting for how many lines are kept in memory for the console window). This is Blender's own in-window console scrollback, not a stream to a reader — ruled out.
- Python API, `bpy.types.Context` — https://docs.blender.org/api/5.1/bpy.types.Context.html , the "Logging Context Member Access" section, because it was the API documentation's only substantial use of the word logging. It says "Log statements are printed to your system's console" and nothing about when. Ruled out as not answering timing.
- Python API, Tips and Tricks — https://docs.blender.org/api/5.1/info_tips_and_tricks.html , which says you can see `print()` output as a script runs. It is written about running Blender with a terminal attached and says nothing about a file or a pipe. Ruled out.
- Python API, Gotchas (operators, threading, file paths) — skimmed for any mention of output streams; none.
- Release notes for 5.1: I searched all twenty-one subject files for logging, console, terminal, buffer, stdout, stderr, command line and background. The only command-line-related change is in Core — "Use the `--python-use-user-env` command line argument to restore access to user modules" (https://developer.blender.org/docs/release_notes/5.1/core/) — which concerns Python module search paths, not output. Nothing in Rendering, Cycles, Pipeline & I/O, Python API or Core mentions output buffering or delivery.

**Pictures opened:**

- `_images/advanced_command-line_launch_linux_console.png` (shown on the Linux launch page). I opened it. It shows a KDE Konsole window with Blender's start-up messages about a missing JACK audio server. It is output in a terminal, which is the case the question excludes, so it cannot settle the question — ruled out on that basis.
- I did not open `advanced_command-line_launch_macos_mac.png`, `advanced_command-line_launch_windows_console.png` or `advanced_command-line_launch_linux_kde.png`. The manual's own captions say they show Blender started from a macOS console window, Blender's Console Window on Windows, and a KDE menu entry configured to run Blender in a terminal. All three are the terminal case by the manual's own description, so none can settle a question about non-terminal destinations.

**Cycles' own files:**

- Searched all 406 supplied files for `stdout` (0), `stderr` (2), `flush` (0), `unbuffered` (0), `isatty` (0), `setvbuf` (0), `fwrite` (0), `printf` (12, all in maths and OSL type-printing helpers), and for buffered-output patterns (nothing).
- The two `stderr` hits are `engine.py` (the `--cycles-print-stats` help text, quoted in the answer) and `source/util/defines.h` (matched only as a substring within other text; no output-related statement).
- I listed the whole supplied Cycles tree. What is here is the add-on's Python files (`__init__.py`, `engine.py`, `operators.py`, `properties.py`, `ui.py`, `osl.py`, `camera.py`, `presets.py`, `version_update.py`) plus the render kernel headers under `source/kernel` and `source/util`. There is no session, progress, logging or output code in what was supplied, so there is nothing here that could show how printed lines are delivered. I ruled the Cycles files out on that basis, not on a guess.
- None of the Cycles Python files contains a `print(` call at all.

**What I did not do.** I did not use the web. I did not use anything I may know about Blender or about how programs buffer their output from outside these sources; where I was tempted to, I stopped, and that is why the answer is "not in the sources" rather than a confident mechanism.

---

## Things a person with Blender 5.1 open could settle in a few minutes

**Pruned 2026-08-30 by the hand tests.** Seven items stood here. **Items 1 to 6 were all run on 2026-08-30**, on the Creator's Mac and on a rented Linux machine, and their answers are in the observed section above — arrival through a pipe, the terminal case for comparison, arrival in a plain file, which stream carries what, `--log-file`, and whether anything switches the behaviour off. They are not repeated here.

**One item is still open, and only in part.**

1. **The Windows half.** The items above that asked for the pipe and file tests to be repeated on Windows were run on macOS and on Linux, where the two agreed; **Windows was not run.** This project has a route to a Windows computer — a rental used four times — but the account allows one machine at a time and the way in costs about ninety minutes, so one Windows batch is run across every topic rather than each topic renting in turn. The Windows runs are written out in full and ready, in `Audits/Blender Research/Reports/Hand tests — Open Threads questions.md` under *Windows — for the runner's separate batch*, and the machine's own edition must be read and recorded before anything is claimed from it: every Windows computer this project has read turned out to be a Server edition, including one whose image was named for a desktop edition.

---

2026-08-30 — Claude Opus 5 (Claude Code) — Reports, Blender 5.1

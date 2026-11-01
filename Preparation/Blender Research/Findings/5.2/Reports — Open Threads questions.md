# Reports — Open Threads questions — Blender 5.2

This document answers question 14 of the Reports question sheet for Blender 5.2, and no other question on that sheet. It is one research pass, and it is the first document to answer that question — not a second pass over an earlier one.

Sources consulted, all on **2026-07-30** (the snapshot date given for every file):

- Blender 5.2 manual, plain text, `Blender Sources/manual_5.2` (including the pictures in its `_images` folder)
- Blender 5.2 Python API documentation, plain text, `Blender Sources/api_5.2`
- Blender 5.2 release notes, `Blender Sources/release_notes_5.2`
- Cycles' own files as shipped, `Blender Sources/cycles_5.2`

I read only the question sheet and files inside Blender Sources/. I did not open any other part of this project.

I worked alone. I started no helper agents, so there is nobody else's reading to vouch for.

One disclosure, so the line above is exact. After saving this file I listed this folder once to check my own file name had been written correctly, and the listing showed me the names of the other findings files sitting beside it. I opened none of them, and I read nothing but their file names.

---

## 14. When a render's printed output is sent somewhere other than a terminal, what decides when each line of it becomes available to whatever is reading?

**LABEL: not in the sources.**

Nothing in the 5.2 manual, the 5.2 Python API documentation, the 5.2 release notes, or Cycles' own supplied files says anything about *when* a line Blender has printed reaches a reader that is not a terminal. The sources describe where output goes and what can be turned on or off in it. They never describe the timing of its delivery, and they never mention holding lines back, releasing them, or any condition — size, line ending, end of run — that would decide the moment a line becomes readable.

### The reading I answered, and the other reading

The question is a little ambiguous, so I say plainly which reading I took.

I answered: **Blender is started by another program, its printed text stream is sent to a file or to a pipe instead of a terminal window, and the other program is reading that stream. What governs the moment each already-printed line can be read at the far end?** That is the reading I believe is meant, because the question distinguishes a terminal from everything else, and speaks of a line "becoming available to whatever is reading".

The other reading available is: **what does Blender offer for sending its output somewhere other than a terminal in the first place** — that is, the `--log-file` option and the operating system's own redirection. I say what the sources hold on that below, because it is the nearest thing they contain, but it is not an answer to the question as I read it.

### What the sources do say, and why none of it answers the question

Each of these is documented, and each stops short of the timing question.

**Where the manual describes Blender's printed messages appearing somewhere, that somewhere is a terminal.** The command line page opens with:

> "The Console Window, also called a Terminal, is an operating system text window that displays messages about Blender's operations, status, and internal errors."

and

> "When Blender is manually started from a terminal, Blender output is shown in the Console Window it is started from."

Manual, https://docs.blender.org/manual/en/5.2/advanced/command_line/index.html, read 2026-07-30.

The case named there is the one the question excludes. The page says nothing about any other destination.

**A log can be pointed at a file, with no word on when it is written.** The manual's Logging Options list:

> "--log-file <filepath>
>
> Set a file to output the log to."

Manual, https://docs.blender.org/manual/en/5.2/advanced/command_line/arguments.html, read 2026-07-30.

That is the whole entry. It settles that a file can be the destination. It says nothing about when a line put into that file becomes readable by anything else.

**The manual documents one stream-handling detail, and it is about input, not output.** Under Debug Options:

> "-d, --debug
>
> Turn debugging on.
>
> Enables memory error detection
>
> Disables mouse grab (to interact with a debugger in some cases)
>
> Keeps Python's sys.stdin rather than setting it to None"

Manual, https://docs.blender.org/manual/en/5.2/advanced/command_line/arguments.html, read 2026-07-30.

I include this because it is the only place in the whole 5.2 manual where the handling of a standard stream is spelled out, and it is `sys.stdin` — the stream Blender reads from. There is no matching statement anywhere about the streams Blender writes to.

**Output can be suppressed, which is a statement about what is printed, not when.**

> "-q, --quiet
>
> Suppress status printing (warnings & errors are still printed)."

Manual, https://docs.blender.org/manual/en/5.2/advanced/command_line/arguments.html, read 2026-07-30.

**Cycles' own files name a stream for one thing, and stop there.** In the Cycles add-on's argument parser:

> `    parser.add_argument("--cycles-print-stats",`
> `                        help="Print rendering statistics to stderr",`
> `                        action='store_true')`

File `Blender Sources/cycles_5.2/engine.py`, lines 13–15, inside `_configure_argument_parser()`. Read 2026-07-30. *(Line reference corrected by the citation check of 2026-08-30: the three quoted lines are 13 to 15; line 12 is the end of the parser's own set-up. The words and the file are as given.)*

This is the only place in the supplied Cycles files that names a standard output stream for render-time reporting, and it names only *which* stream. The switch is then handed to a compiled module that was not supplied:

> `    if args.cycles_print_stats:`
> `        import _cycles`
> `        _cycles.enable_print_stats()`

File `Blender Sources/cycles_5.2/engine.py`, lines 34–36, inside `_parse_command_line()`. Read 2026-07-30. `_cycles` is not among the files in `cycles_5.2`, so nothing about how or when it writes can be read here. *(Line reference corrected by the citation check of 2026-08-30: the three quoted lines are 34 to 36. The words, the file and the function are as given, and so is the point that `_cycles` was not supplied.)*

The only other place in the supplied Cycles files that touches an output stream is the `maketx` sub-command, which is a texture conversion tool rather than a render:

> `        sys.stderr.write("Error: {:s}\n".format(str(ex)))`

File `Blender Sources/cycles_5.2/maketx.py`, line 83, in the `RuntimeError` handler; the success path on line 86 is `    print(out_filepath)`. Read 2026-07-30. Which stream carries what, and nothing about timing.

**Where I could confirm the sources are silent by absence.** The word `stdout` does not appear anywhere in the 5.2 sources at all — not in the manual, not in the API documentation, not in the release notes, not in any Cycles file. Neither does `stderr` in the manual, API documentation or release notes; its appearances in the 5.2 sources are the two Cycles lines quoted above, and one more that these searches, being case-insensitive, also reach: `Blender Sources/cycles_5.2/source/util/defines.h` line 130, `` * `MTLLogLevelDebug`, and `MTL_LOG_TO_STDERR` should be set to `1`. ``, inside a comment about how Metal's shader logging is turned on. It is a substring of an environment variable's name in a comment, and it bears on nothing here. *(Third appearance added by the citation check of 2026-08-30: this line had said there were only two.)* `buffering`, `unbuffered`, `line-buffered`, `fflush`, `isatty` and `PYTHONUNBUFFERED` appear nowhere in any of the 5.2 sources.

### Record of where I looked

Searches were run over the whole of `manual_5.2`, `api_5.2`, `release_notes_5.2` and `cycles_5.2` at once, case-insensitively, not file by file.

Terms searched and found nowhere in the 5.2 sources: `stdout`, `buffering`, `unbuffered`, `line-buffered`, `unbuffer`, `block-buffer`, `fflush`, `isatty`, `PYTHONUNBUFFERED`, `printf`, `fwrite`, `pseudo-terminal`, `carriage return`, `line by line`, `line-by-line`, `as soon as`, `2>&1`, `piping`, `logfile`, `subprocess`, `Popen`, `Fra:`.

Terms searched that hit only matter unrelated to output timing, each of which I opened enough of to rule out: `stderr` (only the two Cycles lines quoted above); `flush` (mesh and selection flushing in `bmesh`, text alignment in `bpy.types.TextCurve`, the `FLUSH` icon name, a geometry-node string tool); `buffer` (the render buffer on the animation output page, the GPU depth and quad buffers, the copy-paste buffer, the Python console's line buffer in Preferences, a video decoder bitstream buffer size); `pipe` (almost all of it the word "pipeline", plus curve-to-tube geometry); `redirect` (only node-editor link redirection and an icon name); `tty` (only inside longer words such as "pretty"); `console` and `System Console` (the pages listed below); `sync`/`synchron`, `delay`, `chunk`, `detach`, `immediately`, `real time`, `interactive`, `non-interactive`, `overwrit`, `same line`, `progress bar`, `ANSI`, `escape code`, `capture`, `log file`, `output to a file` — none of them produced a statement about when printed output reaches a reader.

Pages I opened in full and ruled out, with the reason:

- https://docs.blender.org/manual/en/5.2/advanced/command_line/index.html — describes the Console Window and lists use cases including rendering animation and batch processing. Terminal only; no other destination discussed.
- https://docs.blender.org/manual/en/5.2/advanced/command_line/arguments.html — the complete argument list. I read the Render, Cycles Render, Python, Network, Logging, Debug, GPU and Misc option sections. The only options touching output are `--log`, `--log-level`, `--log-show-memory`, `--log-show-source`, `--log-show-backtrace`, `--log-file`, `--log-list-categories`, `--verbose`, `-q/--quiet`, `--cycles-print-stats` and `-d/--debug`. Not one of them mentions when output is written or released.
- https://docs.blender.org/manual/en/5.2/advanced/command_line/render.html — rendering from the command line, including the note about rendering over a remote shell. Nothing on output timing.
- https://docs.blender.org/manual/en/5.2/advanced/command_line/launch/linux.html, .../launch/macos.html, .../launch/windows.html, .../launch/index.html — all four describe starting Blender so that output appears in a terminal. The Windows page adds that the Console Window is toggled off and reachable through Window > Toggle System Console. No non-terminal destination is described on any of them.
- https://docs.blender.org/manual/en/5.2/troubleshooting/startup.html and .../troubleshooting/crash.html — the Console Window's messages, the crash log file, and the Windows `blender_debug_log.cmd` batch scripts. These concern files written after or during a crash, not the timing of ordinary printed lines.
- https://docs.blender.org/manual/en/5.2/render/output/properties/output.html — reached because it mentions the System Console: the tip that encoding output is visible there and that `blender -d` shows more. Nothing on timing.
- https://docs.blender.org/api/5.2/info_tips_and_tricks.html — "Use the Terminal" and "Use Blender without its User Interface". It advises seeing `print()` output in a terminal, and for background mode suggests checking results by rendering an image, saving a file, or printing results or writing them to a file. It never says when printed text arrives anywhere.
- https://docs.blender.org/api/5.2/info_advanced_blender_as_bpy.html — searched for output, print, stdout, console, terminal and buffer; only an unrelated `print("Found:", blender_bin)` example.
- All twenty-one 5.2 release-notes files, searched together for log, output, console, terminal and print. The hits are compositor File Output nodes, node sockets, a sequencer progress bar, and bug-fix entries. Nothing about output streams or their timing. https://developer.blender.org/docs/release_notes/5.2/
- Cycles' supplied files in full: the tree holds the Python add-on, the OSL shaders, and the render kernel headers. There is no session, progress or logging code among them. Every `log` match in the headers is the mathematical `log()`. Nothing in the supplied files says anything about when output is written.

Picture I opened, from the manual, and what it showed:

- `_images/advanced_command-line_launch_linux_console.png`, shown on https://docs.blender.org/manual/en/5.2/advanced/command_line/launch/linux.html, read 2026-07-30. It is a screenshot of a Konsole window in which `blender` has been run with no redirection, showing eight lines of start-up messages about a jack audio server. **documented**: the picture shows output arriving in a terminal window. It shows no redirection and no pipe, so it cannot speak to the question at all. I opened the other two console screenshots as well, on https://docs.blender.org/manual/en/5.2/advanced/command_line/launch/macos.html and https://docs.blender.org/manual/en/5.2/advanced/command_line/launch/windows.html, read 2026-07-30. Each shows a terminal window in the same way, with no redirection visible, so neither settles anything about the question either.

---

### Answered by hand test, 2026-08-30

**LABEL: observed.** Run by Claude Opus 5 (Claude Code) on 2026-08-30, in Blender 5.2.1, on two machines. The paragraphs above still stand: the sources say nothing. What follows is what a running Blender did. The full list, with every command written out and the raw output of every run, is `Audits/Blender Research/Reports/Hand tests — Open Threads questions.md`.

**The two machines, and the answers are never merged.**

- **The Creator's own Mac** — a MacBook Pro with an Apple M1 Pro chip, eight processor cores, 16 GB of memory, macOS 26.6.2, Apple's own graphics and no second card. Blender 5.2.1 at `/Applications/Blender.app/Contents/MacOS/Blender`, build hash `9e2066aef7ef`.
- **A rented Linux machine** — Vast.ai instance `49254257` in the United Kingdom, a container rather than a whole machine, Ubuntu 22.04.4 LTS on kernel `6.8.0-124-generic`, an Intel Core i7-11800H with sixteen visible threads against a quota of 15.36 cores, a memory ceiling of about 15.8 GB, one NVIDIA GeForce RTX 3070 Laptop GPU on driver 535.309.01 that no test used, 32 GB of disk, and no display server. Blender 5.2.1 at `/opt/blender/blender-5.2.1-linux-x64/blender`, taken from Blender's own download server as the Linux x64 build, build hash `9e2066aef7ef` — the same build as the Mac's. Rented and destroyed the same day.

**What was set up.** One file, `scene.blend`: the factory startup scene — one cube, one lamp, one camera — set to Cycles on the processor, 900 samples, adaptive sampling and denoising off, 720 by 405 at 100 per cent, frames 1 to 5, saving PNG files to `/tmp/r14out/f_`. It was built with Blender 4.5.12 so that every version opens it without converting anything. One frame takes about fifteen seconds on either machine. Every run carried `--factory-startup`, both to leave the Creator's own settings alone and because the Mac has third-party add-ons whose messages would otherwise have been read as Blender's own.

**How arrival was measured.** A reader that looks only after a run has ended cannot tell a line held back from a line printed promptly, so a small program (`reader.py`) was put on the far end of the pipe. It reads raw bytes as they arrive and prints, for each arrival, the seconds since it started, how many bytes arrived, and the first seventy bytes of them. Promptly-arriving output shows as many small arrivals timed to the run's own events; held-back output shows as few arrivals of one fixed size at long intervals. **That the instrument can show the second pattern was proved on both machines**, by putting a program between Blender and the reader that deliberately held everything in 4096-byte blocks. That calibration was run with Blender 4.5.12, since it measures the reader rather than Blender. On the Mac it gave four arrivals, at 0.559, 24.487, 46.278 and 75.931 seconds, of 4096, 4096, 4096 and 2709 bytes; on the rented Linux machine, four arrivals at 0.256, 18.103, 36.024 and 54.115 seconds, of 4096, 4096, 4096 and 2599 bytes.

**1. A render's own printed lines reach a pipe as they are produced.** Command, on both machines: `<blender> -b --factory-startup scene.blend -s 1 -e 3 -a 2>&1 | python3 reader.py`. On the Mac, 54 arrivals over 98.366 seconds. The last eight, word for word:

```
  66.001  chunk  48     290 bytes  total   10761  b'01:05.659  render           | Fra: 3 | Mem: 1M | Updating Lookup Table'
  66.023  chunk  49      84 bytes  total   10845  b'01:05.682  render           | Fra: 3 | Remaining: 00:20.08 | Mem: 5M |'
  98.237  chunk  50      64 bytes  total   10909  b'01:37.895  render           | Fra: 3 | Mem: 5M | Sample 900/900\n'
  98.237  chunk  51      58 bytes  total   10967  b'01:37.896  render           | Fra: 3 | Mem: 5M | Finished\n'
  98.258  chunk  52      62 bytes  total   11029  b"01:37.917  render           | Saved: '/tmp/r14out/f_0003.png'\n"
  98.258  chunk  53      64 bytes  total   11093  b'01:37.917  render           | Time: 00:32.25 (Saving: 00:00.01)\n'
  98.360  chunk  54      14 bytes  total   11107  b'\nBlender quit\n'
  98.366  END  54 arrivals, 11107 bytes total
```

The 47 arrivals before these are cut for length; they are the same shape, arriving in bursts as each frame finishes. On the rented Linux machine, 66 arrivals over 54.260 seconds, ending:

```
  54.173  chunk  63      64 bytes  total   10713  b'00:54.101  render           | Fra: 3 | Mem: 5M | Sample 900/900\n'
  54.173  chunk  64      58 bytes  total   10771  b'00:54.101  render           | Fra: 3 | Mem: 5M | Finished\n'
  54.198  chunk  65     126 bytes  total   10897  b"00:54.125  render           | Saved: '/tmp/r14out/f_0003.png'\n00:54.12"
  54.247  chunk  66      14 bytes  total   10911  b'\nBlender quit\n'
  54.260  END  66 arrivals, 10911 bytes total
```

**The two machines agreed.**

**2. A pipe is no different from a terminal for those lines.** The same render was run again with a real terminal in place of the pipe, supplied by the system's own `script` command. On the Mac, 77 arrivals ending at 84.720 seconds; on the rented Linux machine, 66 ending at 52.598. The arrival pattern is the same as in test 1 on each machine. The only difference in what arrived is that every line ends `\r\n` instead of `\n`, which is the terminal's doing. The last four arrivals on the Mac:

```
  84.587  chunk  75     124 bytes  total   10952  b'01:24.226  render           | Fra: 3 | Mem: 5M | Sample 900/900\r\n01:24'
  84.611  chunk  76     128 bytes  total   11080  b"01:24.251  render           | Saved: '/tmp/r14out/f_0003.png'\r\n01:24.2"
  84.694  chunk  77      16 bytes  total   11096  b'\r\nBlender quit\r\n'
  84.720  END  77 arrivals, 11096 bytes total
```

**3. The same lines reach a plain file as they are produced.** Command: `<blender> -b --factory-startup scene.blend -s 1 -e 3 -a > out.txt 2>&1`, with the file's size read every quarter second from a second process while the render was still going. On the Mac:

```
   0.000  file is       0 bytes
   1.294  file is    3621 bytes
  22.367  file is    3743 bytes
  22.618  file is    7190 bytes
  44.176  file is   10759 bytes
  66.166  file is   11021 bytes
  66.416  run finished, file is   11021 bytes
```

On the rented Linux machine:

```
   0.000  file is       0 bytes
   0.503  file is    3511 bytes
  18.042  file is    7080 bytes
  35.569  file is   10649 bytes
  53.106  file is   10911 bytes
  53.107  run finished, file is   10911 bytes
```

**The steps are the frames.** The file grows once per frame, at the moment that frame finishes.

**4. The progress readout is whole separate lines, not one line rewritten.** Counting the line endings in the file test 3 wrote: on the Mac, `newlines: 149   carriage returns: 0   bytes: 11021`; on the rented Linux machine, `newlines: 149   carriage returns: 0   bytes: 10911`. No carriage returns at all.

**5. A Python script's own printing does not reach a pipe or a file until the run ends, and this is the one place the answer splits.** A script printing one line a second for fifteen seconds (`print("tick %d" % i)` and then a one-second wait) was run with `<blender> -b --factory-startup --python ticker.py 2>&1 | python3 reader.py`. On the Mac:

```
  15.845  chunk   1     111 bytes  total     111  b'tick 1\ntick 2\ntick 3\ntick 4\ntick 5\ntick 6\ntick 7\ntick 8\ntick 9\ntick 10'
  15.900  chunk   2      64 bytes  total     175  b'Blender 5.2.1 LTS (hash 9e2066aef7ef built 2026-08-25 01:35:57)\n'
  15.906  chunk   3      14 bytes  total     189  b'\nBlender quit\n'
  15.912  END  3 arrivals, 189 bytes total
```

All fifteen lines arrived together at the end, in one arrival. On the rented Linux machine:

```
  15.397  chunk   1     111 bytes  total     111  b'tick 1\ntick 2\ntick 3\ntick 4\ntick 5\ntick 6\ntick 7\ntick 8\ntick 9\ntick 10'
  15.440  chunk   2      64 bytes  total     175  b'Blender 5.2.1 LTS (hash 9e2066aef7ef built 2026-08-25 02:12:34)\n'
  15.443  chunk   3      14 bytes  total     189  b'\nBlender quit\n'
  15.453  END  3 arrivals, 189 bytes total
```

Sent to a file instead of a pipe, the file stayed at zero bytes throughout and reached its full size in one step at the end — on the Mac:

```
   0.000  file is       0 bytes
  16.195  file is     189 bytes
  16.195  run finished, file is     189 bytes
```

and on the rented Linux machine:

```
   0.000  file is       0 bytes
  15.530  file is     189 bytes
  15.530  run finished, file is     189 bytes
```

Three further runs on each machine:

- **With `PYTHONUNBUFFERED=1` set**, the result was unchanged. The first arrival on the Mac was `15.832  chunk   1     111 bytes  total     111  b'tick 1\ntick 2\ntick 3\ntick 4\ntick 5\ntick 6\ntick 7\ntick 8\ntick 9\ntick 10'` and on Linux `15.365  chunk   1     111 bytes  total     111  b'tick 1\ntick 2\ntick 3\ntick 4\ntick 5\ntick 6\ntick 7\ntick 8\ntick 9\ntick 10'`.
- **With each line printed as `print("tick %d" % i, flush=True)`**, the lines arrived one a second through the same pipe. The first four arrivals on the Mac — the remaining eleven follow the same one-a-second pattern and are cut for length:

  ```
     0.821  chunk   1       7 bytes  total       7  b'tick 1\n'
     1.825  chunk   2       7 bytes  total      14  b'tick 2\n'
     2.832  chunk   3       7 bytes  total      21  b'tick 3\n'
     3.837  chunk   4       7 bytes  total      28  b'tick 4\n'
  ```

  On the rented Linux machine the first four were:

  ```
     0.345  chunk   1       7 bytes  total       7  b'tick 1\n'
     1.346  chunk   2       7 bytes  total      14  b'tick 2\n'
     2.346  chunk   3       7 bytes  total      21  b'tick 3\n'
     3.346  chunk   4       7 bytes  total      28  b'tick 4\n'
  ```

- **With a real terminal in place of the pipe**, the plain script, with no flushing, also ticked once a second, and the version banner arrived first rather than last. The first four arrivals on the Mac:

  ```
     0.000  chunk   1       4 bytes  total       4  b'^D\x08\x08'
     0.301  chunk   2      65 bytes  total      69  b'Blender 5.2.1 LTS (hash 9e2066aef7ef built 2026-08-25 01:35:57)\r\n'
     0.835  chunk   3       8 bytes  total      77  b'tick 1\r\n'
     1.842  chunk   4       8 bytes  total      85  b'tick 2\r\n'
  ```

  And on the rented Linux machine:

  ```
     0.072  chunk   1      65 bytes  total      65  b'Blender 5.2.1 LTS (hash 9e2066aef7ef built 2026-08-25 02:12:34)\r\n'
     0.358  chunk   2       8 bytes  total      73  b'tick 1\r\n'
     1.359  chunk   3       8 bytes  total      81  b'tick 2\r\n'
     2.359  chunk   4       8 bytes  total      89  b'tick 3\r\n'
  ```

**6. Everything a render prints goes to the ordinary output stream; the error stream carried nothing.** With only the error stream fed to the reader — `<blender> -b --factory-startup scene.blend -f 1 -- --cycles-print-stats 2>&1 1>/dev/null | python3 reader.py` — the whole run gave, on the Mac, `19.870  END  0 arrivals, 0 bytes total`, and on the rented Linux machine, `18.153  END  0 arrivals, 0 bytes total`. Keeping the two streams apart in files, `<blender> -b --factory-startup scene.blend -f 1 -- --cycles-print-stats > s_out.txt 2> s_err.txt`, gave on the Mac `ordinary output: 7088 bytes    error stream: 0 bytes` and on Linux `ordinary output: 6982 bytes    error stream: 0 bytes`. **The Cycles statistics were in the ordinary output**, under headings reading `Update statistics:`, `Render statistics:` and `Mesh statistics:` — not in the error stream, which is where the help text quoted above says they go.

**7. The file written by `--log-file` fills in as the run goes.** Command: `<blender> -b --factory-startup scene.blend --log "*" --log-level info --log-file run.log -s 1 -e 2 -a > /dev/null 2>&1`. The logging-level value was read off `blender --help` rather than guessed. On the Mac:

```
   0.000  file is       0 bytes
   0.519  file is      64 bytes
   1.541  file is    6438 bytes
  21.302  file is   13966 bytes
  44.333  file is   15892 bytes
  44.592  run finished, file is   15892 bytes
```

On the rented Linux machine:

```
   0.000  file is       0 bytes
   0.250  file is      64 bytes
   0.501  file is    6086 bytes
  16.788  file is    7913 bytes
  17.038  file is   13639 bytes
  33.817  file is   15592 bytes
  34.068  run finished, file is   15592 bytes
```

**8. The same split shown inside one run.** So that nothing could be blamed on two runs differing, one run printed both kinds of line into one pipe: `<blender> -b --factory-startup scene.blend --python-expr "print('SCRIPT LINE PRINTED AT START')" -s 1 -e 2 -a 2>&1 | python3 reader.py`. On the Mac the render's own lines arrived throughout, and:

```
   1.482  chunk   1     172 bytes  total     172  b'00:00.816  blend            | Read blend: "/private/tmp/claude-501/-Us'
  30.956  chunk  39      62 bytes  total    3805  b"00:30.290  render           | Saved: '/tmp/r14out/f_0001.png'\n"
  53.824  chunk  62      62 bytes  total    7297  b"00:53.158  render           | Saved: '/tmp/r14out/f_0002.png'\n"
  53.833  chunk  64      29 bytes  total    7390  b'SCRIPT LINE PRINTED AT START\n'
  53.895  chunk  65      14 bytes  total    7404  b'\nBlender quit\n'
  53.901  END  65 arrivals, 7404 bytes total
```

The arrivals not shown are the render's own, spread across the run. The script's line was printed within the first second and reached the reader at 53.833 seconds, after both frames were finished. On the rented Linux machine:

```
   0.351  chunk   1      62 bytes  total      62  b'00:00.284  blend            | Read blend: "/root/scene.blend"\n'
  15.315  chunk  13     174 bytes  total    3807  b"00:15.247  render           | Saved: '/tmp/r14out/f_0001.png'\n00:15.24"
  30.886  chunk  35     126 bytes  total    7328  b"00:30.819  render           | Saved: '/tmp/r14out/f_0002.png'\n00:30.81"
  30.893  chunk  36      29 bytes  total    7357  b'SCRIPT LINE PRINTED AT START\n'
  30.940  END  37 arrivals, 7371 bytes total
```

**The two machines agreed.**

**9. Nothing already printed is lost when a run is stopped part-way.** A three-frame render was sent to a file and stopped at twenty-two seconds, once with an interrupt and once with an outright kill, which cannot flush anything a program is still holding. On the Mac, before the interrupt the file held 3621 bytes ending `00:00.667  render           | Fra: 1 | Remaining: 00:24.67 | Mem: 5M | Sample 1/900`; afterwards 3878 bytes ending:

```
00:23.037  render           | Fra: 1 | Mem: 5M | Cancelled

Blender quit
```

Before the outright kill the file held 7190 bytes; afterwards it held **the same 7190 bytes**, ending on the most recent line the run had printed:

```
00:17.434  render           | Fra: 2 | Mem: 1M | Updating Device | Writing constant memory
00:17.434  render           | Fra: 2 | Mem: 1M | Sample 0/900
00:17.450  render           | Fra: 2 | Remaining: 00:14.39 | Mem: 5M | Sample 1/900
```

On the rented Linux machine the same: 7080 bytes before the kill and 7080 after, ending on `00:17.688  render           | Fra: 2 | Remaining: 00:10.40 | Mem: 5M | Sample 1/900`. Nothing was lost on either machine.

**10. The build's own help list names no switch for any of this.** Running `blender --help` and searching the whole printed list for `buffer`, `flush`, `unbuffer`, `stdout` and `stderr` gives exactly one hit in Blender 5.2.1: `Read from disk (Do not buffer).`, which is the animation player's `-m` argument and concerns reading picture files, not writing output. The same single hit, and nothing else, in the other three versions. So the installed build lists nothing the manual's argument page omits.

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

**Not covered.** This answer settles only that the sources are silent. It does not say Blender has no such behaviour; it says only that the sources do not describe one. It leaves entirely open what governs that moment: whether Blender or the C library holds lines back when the destination is not a terminal, whether the amount held is a fixed size, whether a line ending releases anything, whether anything is released only when the run ends, whether Blender's own output and Python's `print()` behave alike, and whether the two streams Blender writes to behave alike. It also does not cover `--log-file`, whose own writing behaviour is undocumented in the same way. And it says nothing about a run that ends by crashing, where anything held back may never be released at all.

**Outside the interface.** The sources describe the timing of printed output for neither case, so there is nothing to compare. What the sources do carry is that Blender's printed output is a thing that exists whether or not a person is clicking: the manual names automation and batch processing as reasons to use the command line — "For automation and batch processing which require launching Blender with different arguments" (https://docs.blender.org/manual/en/5.2/advanced/command_line/index.html, read 2026-07-30) — and gives `-b`, "Render in the background (without UI)", together with `-a`, as the way to render without the interface (https://docs.blender.org/manual/en/5.2/advanced/command_line/render.html, read 2026-07-30); their long names `--background` and `--render-anim` are on the arguments page (https://docs.blender.org/manual/en/5.2/advanced/command_line/arguments.html, read 2026-07-30). *(Addresses corrected by the citation check of 2026-08-30: this line had put all of it at the render page, which carries neither the automation phrase nor the long switch names.)* But the sources describe that output only as it appears in a terminal. Whether the timing differs between a terminal and a pipe — which is the heart of the question — the sources do not say.

**LABEL: observed** (2026-08-30). The hand test above answers this, and every run in it was Blender driven from outside its own window — started with no window at all, its output read by another program. A render's own printed lines reach a pipe and a plain file as they are produced, at the same moments as they reach a terminal; a Python script's `print()` lines do not reach a pipe or a file until the run ends, though they do reach a terminal one a second. So the behaviour asked about holds outside the interface, and it differs by *which part of Blender did the printing* rather than by whether a person is watching.

**Operating system.** The sources do not say. The manual gives a separate launch page for Linux, macOS and Windows, and the differences it draws between them are how to reach the executable and, on Windows, that the Console Window is created separately and toggled off (https://docs.blender.org/manual/en/5.2/advanced/command_line/launch/windows.html, read 2026-07-30). No page distinguishes the three on how output is delivered, and since the delivery timing is not described for any of them, nothing here can be said to hold or not hold on any particular one. Guessing from the three operating systems' different natures would be inventing.

**LABEL: observed** (2026-08-30). Tests 1 to 9 in the hand-test section above were each run twice, once on macOS 26.6.2 on Apple silicon and once on Ubuntu 22.04.4 on x86-64, on two builds carrying the same build hash `9e2066aef7ef`, and **the two machines agreed on every one of them**. Test 10, the reading of the build's own help list, was run on the Mac only. **Windows is not covered.** It was not run — it is written out ready to run in `Audits/Blender Research/Reports/Hand tests — Open Threads questions.md` and is held for the single Windows batch this project runs across every topic. Whether the answer is the same there is open.

---

## Things only a running Blender 5.2 can settle

**Pruned 2026-08-30 by the hand tests.** Eight items stood here. **Items 1, 2, 3, 4, 5, 7 and 8 were all run on 2026-08-30**, on the Creator's Mac and on a rented Linux machine, and their answers are in the observed section above — arrival through a pipe, arrival in a plain file, the terminal case for comparison, the two streams, a script's own printing, what a stopped run leaves behind, and `--log-file`. Item 6, which asked whether an environment variable or a wrapper changes it, was answered too: `PYTHONUNBUFFERED=1` on its own changed nothing, `stdbuf` in front of the command changed nothing for a script's lines, but `PYTHONUNBUFFERED=1` together with `--python-use-system-env` made them arrive one a second. They are not repeated here.

**One item is still open, and only in part.**

1. **The Windows half.** This list's own opening asked for every item to be run on every operating system that matters. All eight were run on macOS and on Linux, where the two agreed; **Windows was not run.** This project has a route to a Windows computer — a rental used four times — but the account allows one machine at a time and the way in costs about ninety minutes, so one Windows batch is run across every topic rather than each topic renting in turn. The Windows runs are written out in full and ready, in `Audits/Blender Research/Reports/Hand tests — Open Threads questions.md` under *Windows — for the runner's separate batch*, and the machine's own edition must be read and recorded before anything is claimed from it: every Windows computer this project has read turned out to be a Server edition, including one whose image was named for a desktop edition.

---

2026-08-29 — Claude Opus 5 (Claude Code) — Reports, Blender 5.2

# Reports — cross-sweep questions, second pass (Blender 5.0)

**Sources consulted, and when.** Blender's 5.0 manual (`Blender Sources/manual_5.0`, including pictures in its `_images` folder), Blender's 5.0 Python API documentation (`Blender Sources/api_5.0`), Blender's 5.0 release notes (`Blender Sources/release_notes_5.0`), and Cycles' own files (`Blender Sources/cycles_5.0`). All four were taken on **2026-07-30**, and that is the date read for everything quoted below. I did the searching and reading on 2026-08-11.

I read only the question sheet and files inside Blender Sources/. I did not open any other part of this project.

Question answered: **8** only.

---

## 8. What does a render print over the whole of a run, from its first line to its last?

### The reading I took, and the other one

The question is ambiguous in one way and I say so plainly. I read "print" as *the text a render writes out as a stream of lines* — what you would see in a terminal, or capture to a file, across one whole run of Blender that renders something. That is the reading I answer.

The other reading is *what a render puts on screen inside Blender's own window* — the progress bar, the status text, the report messages. I do not answer that reading here, beyond one pointer at the end of this answer, because the words "first line to its last" only make sense of a stream of lines.

I also read "a run" as one whole launch of Blender that renders — from the moment the program starts to the moment it stops — rather than one render operation inside an already-running Blender. Again, "first line to its last" is what pushes me that way.

### LABEL: **documented**, but only in part — and the two ends the question asks about are **not in the sources**

The sources describe the *system* that does the printing, show *one worked example* of the printing, and name every switch that changes it. They do **not** state what the first line of a run is, and they do **not** state what the last line of a run is. Those two specific things are not in the sources, and the record of where I looked for them is below.

### What the sources do say

**One.** All of Blender's printing, including a render's, now comes out of a single system, and a render's progress in background mode is part of it.

> "Most of Blender now uses the same logging system,
> including operators reports and background render progress.
> The output format has changed, and log levels are easier to control."

Release notes, https://developer.blender.org/docs/release_notes/5.0/core/ — read 2026-07-30.

The Python API notes repeat it, and say out loud who is affected:

> "The format of logging output has changed,
>   including background render progress.
>   Add-ons or render farms parsing this output may need to be updated."

Release notes, https://developer.blender.org/docs/release_notes/5.0/python_api/ — read 2026-07-30. (The bracketed text is a link in the original; I have quoted it exactly as written.)

**Two.** The release notes give one worked example of the output. This is the only place in any of the four sources that shows actual printed lines from a render. Quoted exactly and in full, including the line of three dots that sits in the middle of it:

> ```
> 00:00.027  ghost.system     | Create Cocoa system
> 00:00.080  translation      | Locale en_NL used for translation
> 00:00.088  blend            | Read prefs: "/Users/blender/Library/Application Support/Blender/5.0/config/userpref.blend"
> 00:00.096  blend            | Read startup: "/Users/blender/Library/Application Support/Blender/5.0/config/startup.blend"
> 00:00.359  gpu.platform     | Using GPU "Apple M3 Max Metal API 1.2"
> 00:00.359  gpu.platform     | Using Backend "Metal"
> 00:01.041  cycles           | Added device "Apple M3 Max (GPU - 40 cores)" with id "METAL_Apple M3 Max (GPU - 40 cores)".
> ...
> 00:00.244  render           | Rendering single frame (frame 1)
> 00:00.245  render           | Start rendering: Scene, ViewLayer
> 00:00.245  render           | Engine: Cycles
> 00:26.799  render           | Mem: 2983M | Sample 1/16 
> 00:26.799  render           | Mem: 2983M | Sample 16/16
> 00:28.631  render           | Saved: '/tmp/0001.png'
> 00:28.631  render           | Time: 00:28.38 (Saving: 00:00.15)
> ```

The words introducing it are exactly: "Example output:".

Release notes, https://developer.blender.org/docs/release_notes/5.0/core/ — read 2026-07-30.

Two things about that block that the source states by showing them, and which I am not adding to:

- It is labelled an *example*, not a complete run. The line of three dots in the middle marks something left out.
- The clock on the lines after the dots runs *lower* than the clock on the lines before it (00:00.244 follows 00:01.041). The source does not explain this. I do not explain it either. What I take from it is only this: **the block is not a single unbroken run printed end to end**, so it cannot be read as "here is a whole run from first line to last".

**Three.** Reading the shape of a line off that example — **inferred**, and here is the step. Nothing in the sources defines the columns. Every line in the example has the same three parts in the same order: an elapsed time as minutes, seconds and thousandths; a category name; a vertical bar; then the message. Nine of the fourteen lines carry a different category (`ghost.system`, `translation`, `blend`, `gpu.platform`, `cycles`, `render`), and the category names line up with the names the `--log` switch takes. A reader has to supply the idea that this is a fixed format rather than a coincidence of one example, so the label is inferred, not documented.

**Four.** What the printing *contains* during the render part of the run, as shown in that example: a line saying what is being rendered ("Rendering single frame (frame 1)"), a line naming the scene and view layer ("Start rendering: Scene, ViewLayer"), a line naming the engine ("Engine: Cycles"), then repeated lines carrying memory in use and a sample count out of a total ("Mem: 2983M | Sample 1/16"), then a line naming the file written ("Saved: '/tmp/0001.png'"), then a closing line with a total time and a saving time in brackets ("Time: 00:28.38 (Saving: 00:00.15)"). All **documented** — these are the source's own words.

**Five.** Everything the sources give you to change what gets printed. All from the manual's command-line arguments page, https://docs.blender.org/manual/en/5.0/advanced/command_line/arguments.html — read 2026-07-30. Quoted exactly:

> "--log <match>
>
> Enable logging categories, taking a single comma separated argument.
>
> --log "*": everything
> --log "event": every category starting with event.
> --log "render,cycles": both render and cycles messages.
> --log "*mesh*": every category containing mesh sub-string.
> --log "*,^operator": everything except operators, with ^prefix to exclude."

> "--log-level <level>
>
> Set the logging verbosity level.
>
> fatal: Fatal errors only
> error: Errors only
> warning: Warnings
> info: Information about devices, files, configuration, operations
> debug: Verbose messages for developers
> trace: Very verbose code execution tracing"

> "--log-show-memory
>
> Show memory usage for each log message."

> "--log-show-source
>
> Show source file and function name in output."

> "--log-show-backtrace
>
> Show a back trace for each log message (debug builds only)."

> "--log-file <filepath>
>
> Set a file to output the log to."

> "--log-list-categories
>
> List all available logging categories for --log, and exit."

And the switch that turns the printing down:

> "-q, --quiet
>
> Suppress status printing (warnings & errors are still printed)."

Same page, same date.

The release notes add what these replaced:

> "Some command line flags were replaced."
>
> | Old | New |
> |-|-|
> |`--debug-cycles`|`--log cycles`|
> |`--debug-ffmpeg`|`--log video`|
> |`--verbose`|`--log-level [info\|debug\|trace]`|

Release notes, https://developer.blender.org/docs/release_notes/5.0/core/ — read 2026-07-30.

**Six.** Cycles can be asked to print its own statistics on top of all this. The manual says:

> "--cycles-print-stats
>
> Log statistics about render memory and time usage."

Manual, https://docs.blender.org/manual/en/5.0/advanced/command_line/arguments.html — read 2026-07-30.

Cycles' own file says the same switch differently, and names the stream. **Observed**, from `Blender Sources/cycles_5.0/engine.py`, lines 13 to 15, inside `_configure_argument_parser()`:

> ```
>     parser.add_argument("--cycles-print-stats",
>                         help="Print rendering statistics to stderr",
>                         action='store_true')
> ```

Read 2026-07-30. The file *says* these statistics go to the error stream. What a render actually does with them at run time is one step past what the file says, so if you want the claim "the statistics come out on the error stream while the rest of the log comes out somewhere else", that is **inferred**, and the step is: I read a help string in an argument definition and took it as a statement about where the output lands.

Where in the run these statistics appear — start, end, or throughout — is **not in the sources**.

**Seven.** When a run writes a video file rather than still images, the encoder adds its own output. Documented:

> "When you view the System Console,
> you can see some of the output of the encoding process.
> You will see even more output if you execute Blender as blender -d."

Manual, https://docs.blender.org/manual/en/5.0/render/output/properties/output.html — read 2026-07-30.

**Eight.** One of the release notes' corrective-release entries touches exactly on what a background run prints per frame. Quoted exactly, from the list headed "Blender 5.0.1":

> "* Show currently rendered frame in background mode. (blender/blender#150174)"

Release notes, https://developer.blender.org/docs/release_notes/5.0/corrective_releases/ — read 2026-07-30. The heading above the list reads: "Released on December 16th 2025, Blender 5.0.1 features 132 bug fixes:".

I report the entry and stop there. What was printed before that fix, and what is printed after it, is not stated on the page.

### The two ends of the question — first line and last line

**First line: not in the sources.** No page in the manual, the API documentation or the release notes states what Blender prints first when it starts. The example above opens with `ghost.system | Create Cocoa system`, but the block is labelled an example, not a complete run, and it was clearly taken on a Mac (Cocoa, Metal, a `/Users/…/Library/Application Support/` path), so a first line taken from it would be a first line for one machine and one platform, not a statement about runs in general.

**Last line: not in the sources.** No page states what Blender prints last. The example's last line is the `Time:` line, but that is the last line of a render, not shown as the last line of a run — the block simply stops there.

There is one picture that shows an end-of-run line, and I looked at it. The manual's page on launching from the command line on Windows says:

> "The screenshot shows the Blender Console Window on Microsoft Windows directly after starting
> Blender and then a short while later after opening a file along with the relevant messages."

Manual, https://docs.blender.org/manual/en/5.0/advanced/command_line/launch/windows.html — read 2026-07-30. The picture is `_images/advanced_command-line_launch_windows_console.png`. **Documented** — what the picture visibly shows is a Command Prompt window containing, in order: the command `blender`, a line beginning `Read new prefs:`, a line beginning `found bundled python:`, a line beginning `read blend:`, a blank line, and then `Blender quit`.

I am not carrying that forward as an answer, and here is why: the paths visible in that same picture read `Blender\2.77` and `c:\blender\2.77`. The picture is of a much older Blender than the one I am researching, and the release notes state in so many words that the output format has changed. So the picture tells you what that picture shows and nothing about a 5.0 run's last line. I looked at the matching pictures on the Linux and macOS launch pages too (`advanced_command-line_launch_linux_console.png`, `advanced_command-line_launch_macos_mac.png`); the Linux one shows audio-system warnings from a startup, the macOS one shows a 2.64 path and two startup lines. Neither shows a render, and neither shows an end of a run.

### Observed — the whole of a Blender 5.0 render run (2026-08-11)

Run in Blender 5.0.1 on **the Creator's Mac (a MacBook Pro with an Apple M1 Pro chip, 16 GB of memory, macOS 26.5.2, Apple's own graphics and no second card)** and on **a rented Linux machine (Ubuntu 22.04.5 LTS on kernel 6.8, two NVIDIA RTX 4000 Ada Generation cards of 20 GB each on driver 580.105.08, eleven cores, 49 GB of memory)**, each with a settings folder of its own so nothing touched the Creator's own Blender settings. The scene was built by Blender 5.0 itself: a cube, a sphere, an area light and a camera; Cycles on the processor; 16 samples; adaptive sampling off; 160 by 120; output PNG. **LABEL: observed.**

**The two ends the sources do not state.** `blender -b <file>.blend -o <out> -f 1 > out.txt 2> err.txt` exited 0, wrote its picture, and put **nothing at all on standard error**. Standard output held fifty-four lines. The first seven, all three sample lines and the last four, word for word, from the Creator's Mac:

```
00:00.373  blend            | Read blend: "…/scenes/base_5.0.blend"
00:00.378  render           | Rendering single frame
00:00.378  render           | Rendering frame 1
00:00.378  render           | Start rendering: Scene, ViewLayer
00:00.378  render           | Engine: Cycles
00:00.388  render           | Fra: 1 | Mem: 0M | Synchronizing object | Cube
Blender 5.0.1 (hash a3db93c5b259 built 2025-12-16 01:51:40)
…
00:00.395  render           | Fra: 1 | Mem: 2M | Sample 0/16
00:00.398  render           | Fra: 1 | Remaining: 00:00.05 | Mem: 3M | Sample 1/16
00:00.564  render           | Fra: 1 | Mem: 3M | Sample 16/16
00:00.564  render           | Fra: 1 | Mem: 3M | Finished
00:00.617  render           | Saved: '…/out/e1_5.0_0001.png'
00:00.617  render           | Time: 00:00.23 (Saving: 00:00.05)

Blender quit
```

**The first line is `blend | Read blend:` and the last is `Blender quit`, with one blank line before it.** The version banner is not first: it arrives seven lines in, in the middle of the run, and it is the one line carrying no timestamp and no category. The rented Linux machine gave the same lines in the same order, with nothing on standard error. **The two machines agreed.**

**The three-column shape read off the published example holds for every other line.** Every line but the banner has an elapsed time as minutes, seconds and thousandths, then a category name, then a vertical bar, then the message. The categories seen in a plain run are `blend` and `render`; with `--log "*"` they also include `translation` and `cycles`.

**How this compares with the example the release notes publish.** The published example's render lines read `Mem: 2983M | Sample 1/16`; what 5.0.1 actually prints is `Fra: 1 | Remaining: 00:00.05 | Mem: 3M | Sample 1/16` — a frame field and a time estimate the example does not show. The example's opening line reads `Rendering single frame (frame 1)` as one line; 5.0.1 prints `Rendering single frame` and `Rendering frame 1` as two. **The elision in the example hid nothing but stage lines**: there is no line per sample in between, only the three quoted above. So for a sixteen-sample frame exactly three lines carry a sample count, and lines 2 to 15 were never printed.

**The default level of printing.** With no logging switch at all the run gives the fifty-four lines above. `--log-level info` gives **138** lines and `--log "*"` gives **138**, on the Creator's Mac. The extra lines name the graphics device, the denoiser, the file-reading times and the translation locale.

**The full list of categories.** `--log-list-categories` prints **135** names on the Creator's Mac and **169** on the rented Linux machine — so the list is longer on Linux — and **both `render` and `cycles` are in it** on both machines.

> **Corrected by the citation check (2026-08-11).** The Linux figure read 170. That is the number of *lines* printed, of which the first is the Blender version banner; the count of categories is 169.

**Which stream the log goes to, and where Cycles' statistics land.** Both go to **standard output**. Standard error was empty in every successful run on the Mac, and in every version but 5.2 on the Linux machine. With `-- --cycles-print-stats` the statistics also arrived on standard output, in two blocks: `Update statistics:` during the Updating stages and `Render statistics:` **after the `Saved:` and `Time:` lines**, near the end of the run. Cycles' own help string names standard error; what the machines did was otherwise. Without the bare `--` the switch is taken as a file name and the run **exits 1**.

> **Corrected by the citation check (2026-08-11).** This paragraph placed the second block "after the last sample line and before `Saved:`". Repeating the run on both machines, twice each, put it after `Saved:` and after `Time:` in this version. The placing is also sent to the hand tests.

**Observed — the block this version prints, and where it lands, captured by a second pair of hands (2026-08-12).** Run on **the Creator's Mac (a MacBook Pro with an Apple M1 Pro chip, 16 GB of memory, macOS 26.5.2, Apple's own graphics and no second card)** and on **a rented Linux machine (Ubuntu 22.04.5 LTS on kernel 6.8, two NVIDIA RTX 4000 Ada Generation cards of 20 GB each on driver 580.105.08, an AMD Ryzen Threadripper PRO 3945WX with eleven cores, 49 GB of memory)**, each version with a settings folder of its own and its own copy of the scene, with `blender -b <file>.blend -o <out> -f 1 -- --cycles-print-stats`. Every run exited 0 and standard error was empty. **LABEL: observed.** The head of the block on the Creator's Mac, word for word:

```
Render statistics:
Mesh statistics:
  Geometry:
    Total memory: 24.04K (24,615)
        Sphere                           23.55K (24,117)
        Cube                             417 (417)
        Area                             81 (81)
Image statistics:
  Textures:
    Total memory: 0 (0)
Kernel statistics:
  Total render time               : Total 100.00% (0.12s), Self 4.17% (0.01s)
```

**The rented Linux machine printed the same ten lines above `Kernel statistics:`, figure for figure**; its own kernel line read `  Total render time               : Total 100.00% (0.32s), Self 4.70% (0.01s)`. **The two machines agreed on the byte counts and differed on the times.** The block then runs on through the kernel, shader and object figures, all of them times and percentages that changed from run to run. The whole block is quoted in `Preparation/Blender Research/Findings/5.0/Reports — cross-sweep questions.md`. **These figures are not Blender 4.5's**: run the same way on the same day, 4.5 printed `Total memory: 24.03K (24,607)`, `Sphere 23.55K (24,113)`, `Cube 413 (413)`.

**Where the block lands, counted by line number.** The lines of a second run were numbered. On the Creator's Mac standard output held 130 lines: `Update statistics:` at 15, `Sample 16/16` at 81, `Finished` at 82, `Saved:` at 83, `Time:` at 84, `Render statistics:` at 85. On the rented Linux machine it also held 130 lines with exactly the same numbers. **The two machines agreed, and both bear out the correction above.** Blender 4.5 run the same way put `Render statistics:` at line 78 and `Saved:` at 122 on the Mac — the other way round.

**What a run prints per frame for an animation.** A three-frame run announces the range once and each frame in turn:

```
00:00.261  render           | Rendering animation (frames 1..3)
00:00.261  render           | Rendering frame 1
…
00:00.408  render           | Rendering frame 2
…
00:00.493  render           | Rendering frame 3
```

Each frame carries its own `Start rendering`, `Engine:`, stage lines, three sample lines, `Finished`, `Saved:` and `Time:`. Nothing follows the last frame's `Time:` line but the blank line and `Blender quit`.

**What the non-Cycles engines print.** EEVEE — called `BLENDER_EEVEE` in this version — prints four progress lines of the shape `Fra: 1 | Rendering 1 / 64 samples`, at 1, 25, 50 and 64 of 64, with no memory figure and no time. **Workbench prints no progress line at all.**

**The exit status of a run that succeeded: 0**, in every successful run above, on both machines. A run given `--cycles-print-stats` without the bare `--` exits 1. A run asked for a picture of no pixels prints `00:00.207  reports          | ERROR Image too small`, writes nothing, and still **exits 0**.

**`-q` removes three lines and not the render.** Comparing the two captures line by line, it removes the `blend | Read blend:` line, the blank line before the end, and `Blender quit`. Every render line survives, including all three sample lines and the `Saved:` and `Time:` lines, and so does the version banner.

**Whether the manual's `Blender quit` picture still matches.** It does, as the last line: a 5.0.1 run ends `Blender quit`, exactly as the old screenshot shows, though everything above it is in the new format.

### NOT COVERED — what this answer does not settle

The ten gaps that stood here have been closed by the tests above, except the two below. What the sources do not say is still what the sources do not say; what a running Blender 5.0.1 does is now recorded.

- **What is printed when a render does not succeed.** That is a different question on the sheet and this answer has not gone near it.
- **The time estimate that appears when the pointer rests on the progress bar.** It needs a hover, and neither machine has any way of moving the pointer without a hand on it, so it was dropped rather than run. The on-screen side of the question is otherwise settled: with a window open the status bar shows one bar labelled `Rendering...` carrying a whole-number percentage and an X to cancel, and no progress line reaches the terminal at all.
- **Anything on Windows.** No Windows machine has run it, so this answer covers macOS and Linux and leaves Windows open.

### Outside the interface

This holds outside the interface, and in fact the clearest statements in the sources are *about* outside the interface. The release notes tie the new logging system directly to "background render progress", and the Python API notes name "render farms parsing this output" as the parties affected. The manual's own framing is the same:

> "The Console Window, also called a Terminal, is an operating system text window that displays
> messages about Blender's operations, status, and internal errors.
>
> When Blender is manually started from a terminal,
> Blender output is shown in the Console Window it is started from."

Manual, https://docs.blender.org/manual/en/5.0/advanced/command_line/index.html — read 2026-07-30. The same page lists "For rendering animation." and "For automation and batch processing which require launching Blender with different arguments." among its use cases.

Where it differs from inside: the sources do not describe a difference in the *content* of the printing between a run driven by a person and a run driven by something else, with one exception — the 5.0.1 entry "Show currently rendered frame in background mode", which is specifically about background mode and therefore implies the two were not identical. What the difference was, the sources do not say.

`--log-file <filepath>` means a run driven from outside does not need a terminal at all to keep the output.

**Observed (2026-08-11), and the difference is total.** Every line quoted in the observed section above was captured from a terminal with no window, in Blender 5.0.1 on both machines. **With a window open, no progress line reaches the terminal at all** — a render started from a script with the window up printed only `Saved:`, in both cases, on both machines. A render started **from Python with no window** printed nothing but `Saved:` either; only `-f` and `-a` print the full progress. And the render-statistics handler is handed forty-five strings with no window and one string with a window. So the printed stream this answer describes belongs to a run started with `-f` or `-a` and no window, and to no other way of starting.

### Operating system

The switches themselves are documented without any operating-system restriction. The same arguments page marks other switches as restricted where they are — "(Windows only)" on `/?` and `--qos`, "(Windows & Linux only)" on `-r, --register` — and carries no such marking on any of the `--log…` switches or on `-q`. Reading an absence of a marker as "works everywhere" is a step the reader has to supply, so that is **inferred**, not documented.

What differs by operating system is how you get at the output, and the manual documents this per platform:

- Windows: "When Blender is started on a Microsoft Windows operating system, the Console Window (called the Command Prompt) is first created as a separate window on the desktop. The main Blender window will also appear and the Console Window will then be toggled off. To display the console again, go to Window ‣ Toggle System Console." (https://docs.blender.org/manual/en/5.0/advanced/command_line/launch/windows.html, read 2026-07-30)
- macOS: "In order to run Blender you will have to specify that path to the Blender executable inside this folder, to get all output printed to the terminal." (https://docs.blender.org/manual/en/5.0/advanced/command_line/launch/macos.html, read 2026-07-30) — that is, launching the `.app` the ordinary way does not give you the output.
- Linux: run it from a terminal and the output goes there. (https://docs.blender.org/manual/en/5.0/advanced/command_line/launch/linux.html, read 2026-07-30)

The *content* of some lines is plainly platform-bound in the one example given — `Create Cocoa system`, `Using Backend "Metal"` — but the sources do not say what the equivalent lines are elsewhere, so I will not claim it.

**Observed (2026-08-11).** Settled for macOS and for Linux, and the printed content agreed line for line: the same fifty-two to fifty-four lines in the same order, the same first and last line, the same three sample lines, the same three lines removed by `-q`, the same statistics on standard output with an empty standard error, and the same animation announcements. **One difference between the machines**: the list of logging categories is 135 entries on the Creator's Mac and 169 on the rented Linux machine (corrected from 170 by the citation check, 2026-08-11 — the Linux figure had counted the version banner as a category). **Checked again on 2026-08-12 for Cycles' statistics**: the block's byte counts and its place in the run came back the same on both machines — the same six line numbers out of 130 lines — and only the times inside the block differed. **Windows is untested** — no Windows machine has run it. **Untried rather than untestable: a Windows machine can be rented, and every one this project has read reported a Server edition — see the equipment section of `Preparation/Blender Research/Method.md`.**

---

## Where I looked

This is the record, and I treat it as part of the answer.

**How I searched.** I searched the whole of `manual_5.0`, `api_5.0` and `release_notes_5.0` at once for each term, rather than opening likely files, and I searched `cycles_5.0` separately because its files are of a different kind.

**Terms searched across manual, API and release notes, and what came back:**

| Term | Result |
|---|---|
| `console` | 30 manual files. Followed up every one that could plausibly concern rendering. |
| `stdout`, `standard output` | Nothing relevant. `standard output` appears once, in a colour node page, in a different sense. |
| `stderr` | **Nothing at all** in manual, API or release notes. |
| `Fra:` | **Nothing at all**, in any of the four sources including Cycles. |
| `Saved:` | One hit only — the release-notes example above. |
| `Time:` | Two relevant hits, both the release-notes example. The rest are unrelated (a fluid page, a modifier page). |
| `Mem:` | Two hits only, both the release-notes example. |
| `Remaining` | No hits about render output. Two hits about a mesh-delete option. |
| `Peak` | One relevant hit: `memory_peak` in the API's RenderEngine page. One in the corrective releases about metadata, not about printing. |
| `Blender quit` | **Nothing at all** in text. Appears only inside a picture (the Windows one, discussed above). |
| `Append frame`, `Reading blend`, `Read blend` | Nothing. |
| `Loading render kernels`, `Synchronizing`, `Updating Scene`, `Waiting for render`, `Path Tracing` (as a status), `Denoising` (as a status) | Nothing that is printed output. `Path Tracing` and `Denoising` appear only as feature names on Cycles pages. |
| `Start rendering`, `Rendering single frame`, `Engine:` | One hit each — all three the release-notes example. |
| `progress` | 30+ files. Checked the status bar page (interface, not printing) and the two release-note hits ("background render progress"). |
| `logging`, `--log` | Five manual files. Two are unrelated to rendering (a Steam page, a translation page). The two real ones are the command-line index and the arguments page, both used above. |
| `log output`, `log line`, `log category`, `log categories`, `timestamp`, `verbosity` | Nothing beyond what is already quoted. `log category`/`log categories` returned **nothing at all**, which is why the full list of categories is a gap. |
| `exit code`, `return code`, `exit status` | **Nothing at all** in any of the three. This is why the answer says nothing about how a run ends. |
| `background render`, `render progress`, `currently rendered frame`, `Rendering frame` | The three release-note hits used above, and one unrelated API hit about placeholder files. |
| `print-stats`, `print_stats` | One manual hit, two Cycles hits — all used above. |
| `update_stats`, `update_progress` | API only: `bpy.types.RenderEngine`, `bpy.types.HydraRenderEngine`. Read the RenderEngine page. |

**Pages opened in full and what I concluded:**

- `advanced/command_line/index.txt` — describes the Console Window and why you would use it. Used above.
- `advanced/command_line/render.txt` (Rendering From The Command Line) — the obvious candidate. It gives example commands and explains argument order. **It shows no output whatsoever.** Ruled out as a source of printed lines.
- `advanced/command_line/arguments.txt` — read the Render Options, Cycles Render Options, Logging Options, Debug Options and Misc Options sections in full. This is where all the switches above come from. Ruled out as a source of example output: it describes switches, not what they produce.
- `advanced/command_line/launch/linux.txt`, `launch/macos.txt`, `launch/windows.txt` — read in full, and looked at all three of their pictures. Discussed above.
- `render/output/animation.txt` (Rendering Animations) — read in full, expecting the animation case. **It says nothing about printed output at all**; it is about workflow, file formats and recovery. Ruled out.
- `render/output/properties/output.txt` — the encoder tip, used above.
- `render/cycles/render_settings/sampling.txt` — the `console` hit here turned out to be about the *Python* console and combining sample subsets. Ruled out.
- `interface/window_system/status_bar.txt` — the on-screen progress bar. Ruled out as printing, noted as a pointer.
- `api_5.0/bpy.types.RenderEngine.txt` — read the reporting methods (`update_stats`, `update_progress`, `update_memory_stats`, `report`). These are how an engine *feeds* status; the page does not say what any of it looks like as printed text. Ruled out.
- `release_notes_5.0/core.md` — read in full. The Logging section is the single most useful thing in any source for this question.
- `release_notes_5.0/python_api.md` — read the Logging section and the Context Logging section.
- `release_notes_5.0/corrective_releases.md` — read the 5.0.1 list; picked out the one entry about background mode.
- `release_notes_5.0/rendering.md`, `cycles.md`, `pipeline_io.md` — searched for `output`, `message`, `warn`, `report`, `frame`. **Nothing about printing.** Ruled out.

**Cycles' own files.** I listed the whole tree (403 files: `__init__.py`, `camera.py`, `engine.py`, `operators.py`, `osl.py`, `presets.py`, `properties.py`, `ui.py`, `version_update.py`, a `shader` folder of compiled OSL and headers, and a `source` folder of kernel and utility headers). I searched all of it for `print(`, `logging`, `sys.stdout`, `write(`, `stderr`, `report(`, `Sample `, `Remaining`, `Rendering`. The only find relevant to this question is the `--cycles-print-stats` argument in `engine.py`, quoted above. The reporting calls in `operators.py` and `osl.py` are operator messages for denoising and shader compiling, not part of a render's own printed run, so I left them out. **The part of Cycles that would actually produce the progress lines is not in this folder** — what is here is the add-on's Python side, the shader sources, and kernel headers. That is worth stating plainly: Cycles' own files could not answer this question, and I did not stretch them to.

**What I did not do.** I did not go to the web, and did not need to. I did not open any other document in the folder I was to save into.

---

## Things a person with Blender open could settle in a few minutes

Eleven of the twelve items that stood here have been run, on both machines, and their results are written into the answer above. What is left open is below.

1. **Item 11, the manual's picture checked on Windows.** No Windows machine was rented for it, so it was not run. Everything in this answer covers macOS and Linux and leaves Windows open.

2. **The time estimate that appears when the pointer rests on the progress bar.** It needs a hover, and neither machine has any way of moving the pointer without a hand on it.

3. **What a run prints when the render does not succeed.** A different question on the sheet, and not gone near here.

4. **What a run prints when the output is a video file.** Not tested; every run above wrote still images.

---

2026-08-11 — Claude Opus 5 (Claude Code) — Reports, Blender 5.0

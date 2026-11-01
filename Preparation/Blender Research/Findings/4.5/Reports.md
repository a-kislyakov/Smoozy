# Reports — Blender 4.5

**Sources consulted, and when.** Blender 4.5 manual (plain text and its `_images` pictures), Blender 4.5 Python API documentation, Blender 4.5 release notes, and the Cycles add-on's own files, all as supplied in `Blender Sources/`. All read on 2026-07-30 (the snapshot date), work done 2026-08-04. I did not use the web.

I read only the question sheet and files inside Blender Sources/. I did not open any other part of this project.

No helper agents were used. Everything below I read myself.

---

## 1. What can cause a render to fail, and what does Blender do in each case?

**LABEL: documented in part; not in the sources for a general list.**

The sources do not carry a list of "things that make a render fail". What they carry is a scattered set of named failure situations, nearly all of them about the graphics card, plus a general page about crashes. Here is every one I found, with what the source says Blender does.

**a. The graphics card runs out of room for the scene (Cycles, CUDA / OptiX / HIP / Metal) — Blender tries system memory instead.** *documented*

> "With CUDA, OptiX, HIP and Metal devices, if the GPU memory is full Blender will automatically try to use system memory. This has a performance impact, but will usually still result in a faster render than using CPU rendering."

Note the list of four technologies; oneAPI is not among them, and the source does not say what happens there.

**This is the one failure in the list that has since been watched happening.** On 2026-08-06, on a machine with four NVIDIA GeForce RTX 4060 Ti of 16 GB each and 197 GB of system memory, a scene needing about 33 GB was rendered with a 16 GB card and the fallback ran: the card filled to its own size, the render finished, the picture was written, the process exited 0, **nothing was printed about any of it**, and the run took about ten times longer. It is recorded in full under question 6, as **Observed, 2026-08-06 — what the memory readouts say with four graphics cards, and what happens when a card is filled**. *(Pointer added by the four-card citation check, 2026-08-06: the hand-test list says that run answers questions 1, 2 and 6, and it had been written into question 6 alone.)*

**b. There is not enough memory to hold the scene for the graphics card — Blender shows an error.** *documented*

> "Error: Out of memory"
>
> "This usually means there is not enough memory to store the scene for use by the GPU."

The source gives the message but does not say what the render does next. (Citation check 2026-08-05: this used to read "There is still not enough memory", which placed this error after the system-memory fallback in item a. The page does not tie the two together, so the word was removed.)

**c. Windows limits how long a display GPU may spend on render computations, and the documented error says the driver lost its connection.** *documented*

> "The NVIDIA OpenGL driver lost connection with the display driver"
>
> "If a GPU is used for both display and rendering, Windows has a limit on the time the GPU can do render computations. If you have a particularly heavy scene, Cycles can take up too much GPU time."

**d. On Linux, a GCC version the CUDA toolkit does not support gives an error.** *documented*

> "Unsupported GNU version"
>
> "On Linux, depending on your GCC version you might get this error."

(Citation check 2026-08-05: this used to say "The CUDA kernel will not compile because the compiler version is unsupported". The quoted words never say what fails — neither "kernel" nor "compile" appears in them — so the claim was narrowed to what they carry.)

**e. The CUDA kernel will not compile because the card is newer than Blender.** *documented*

> "CUDA Error: Kernel compilation failed"
>
> "This error may happen if you have a new NVIDIA graphics card that is not yet supported by the Blender version and CUDA toolkit you have installed. In this case Blender may try to dynamically build a kernel for your graphics card and fail."

All of a–e: Blender manual, https://docs.blender.org/manual/en/4.5/render/cycles/gpu_rendering.html, read 2026-07-30.

**f. EEVEE using too much graphics memory — the application can be killed.** *documented*

> "In practice, using too much GPU memory can make the GPU driver crash, freeze, or kill the application. So be careful of what you ask."
>
> "There is no standard way of estimating if the resources will fit into the GPU memory and/or if the GPU will render them successfully."

Blender manual, https://docs.blender.org/manual/en/4.5/render/eevee/limitations/limitations.html, read 2026-07-30.

**g. Blender crashes outright — it writes a crash file.** *documented*

> "The most common causes of Blender crashes:
>
> Running out of memory.
>
> Issues with graphics hardware or drivers.
>
> Bugs in Blender."

and

> "When Blender crashes, it writes out a text file which contains information that may help identify the cause of the crash. Usually, this file is written in the Temporary Directory directory."

Per operating system, the same page says: on Windows a file named after the blend-file, e.g. `test.crash.txt`, and if the crash is in a Blender module also `blender.crash.txt`; on macOS the system Crash Reporter shows a backtrace and `.crash` files may appear in `~/Library/Logs/DiagnosticReports/`; on Linux "a file named blender.crash.txt is written to the /tmp directory."

Blender manual, https://docs.blender.org/manual/en/4.5/troubleshooting/crash.html, read 2026-07-30.

**h. Known 4.5 graphics-card stability problems named in the release notes.** *documented*

> "There are known stability issues in Blender 4.5.0 LTS. These are being worked on together with AMD to resolve in a future release.
>
> * RDNA2 GPUs have rendering artifacts in volumes on Windows. (blender/blender#139071)
> * HIP RT has stability issues that can lead to crashes while editing objects in the viewport."

and, for Intel:

> "When more than one Intel GPU is present and used for rendering, there may be crashes. A solution to this is already in Blender 5.0 alpha and planned for Blender 4.5.1, with the root cause to be fixed in a future Intel Driver release."

Release notes, https://developer.blender.org/docs/release_notes/4.5/cycles/, read 2026-07-30.

**i. When the Cycles device override returns false, the file calls `sys.exit(1)`.** *observed for the call; inferred for what the Blender process does*

Exact lines, `Blender Sources/cycles_4.5/engine.py`, lines 38–41, inside `_parse_command_line()`:

```
    if args.cycles_device:
        import _cycles
        if not _cycles.set_device_override(args.cycles_device):
            sys.exit(1)
```

What the file says is solid: when `_cycles.set_device_override(...)` returns a false value, `sys.exit(1)` is called. That this ends the whole Blender process with status 1 before any render happens is a step beyond the file — **inferred** from ordinary Python behaviour of `sys.exit`, and I did not run it.

**j. OSL compilation errors are shown in the system console.** *documented*

> "Blender will compile the OSL source file automatically. If the source is .osl, it will be compiled into .oso bytecode. Compilation errors will be shown in the system console."

Blender manual, https://docs.blender.org/manual/en/4.5/render/cycles/osl/index.html, read 2026-07-30.

**Not covered by the sources.** The sources say nothing about the failures a render farm cares about most: no camera in the scene; an output path that cannot be written (missing folder, no permission, read-only volume); the disk filling up mid-animation; a linked or texture file that has gone missing at render time; running out of ordinary system RAM during a CPU render; a corrupt or newer-version blend-file. Nor do they say what Blender's *process* does in any of these cases — whether it keeps going to the next frame, stops the whole animation, or ends with a particular status. Most of that list was settled by tests instead; see the observed block below. Still unsettled after those tests: a disk filling up mid-animation, running out of system memory during a CPU render, and a corrupt or newer-version blend-file.

**Where I looked, for the ones I am calling silent.** Whole-tree searches of the manual, the API documentation and the release notes for: "fail", "failed", "failure", "error", "out of memory", "cannot save", "unable to open", "disk full", "aborted", "no camera", "missing file", "exit code", "return code", "exit status", "error code", "non-zero", "permission". Pages opened and ruled out: the render output pages (Introduction, Output, Rendering Animations), the whole command-line section (Index, Arguments, Rendering From The Command Line, the three Launching pages), Troubleshooting (Index, Crashes, Startup), the Cycles GPU Rendering page, the EEVEE Limitations page. The only "no camera" hit anywhere was about preview thumbnails in the Save & Load preferences, nothing to do with rendering. "Report Missing Files" exists as a manual menu command but is a check you run yourself, not something a render does.

**Observed, 2026-08-05 — nine failure situations tried from a terminal.**

How all of these were run, once for all of them. On the Creator's own computer: a MacBook Pro with an Apple M1 Pro chip, macOS 26.5.2, Apple's own graphics and no other graphics card. Blender 4.5.12 LTS, at `/Applications/Blender 4.5.12.app`, started with no window (the `-b` switch) and with a throwaway settings folder of its own so nothing of the Creator's was touched. Unless said otherwise the file was the factory startup scene — one cube, one camera, one light — saved by Blender 4.5 with render engine Cycles, 16 samples, 160 by 120 pixels, frames 1 to 3, PNG output. Each run's two output streams were kept in separate files and the shell's exit status was read straight afterwards. The full commands are in `Audits/Blender Research/Reports/Hand tests.md`, entries 7 to 12, 18 and 19.

**1. No camera in the scene.** The camera object was deleted from the file before saving. Run: `-b "no camera.blend" -o "<folder>/frame_" -F PNG -f 1`. The whole of standard output:

```
Blender 4.5.12 LTS (hash 84afd5f785f7 built 2026-07-21 01:41:35)
Read blend: "/Users/artist/Documents/Smoozy hand tests/Reports/23/no camera.blend"
Error: Cannot render, no camera

Blender quit
```

Standard error was empty. Exit status 1. No image file existed in the output folder afterwards.

**2. The output folder does not exist.** Run: `-b "simple.blend" -o "<folder>/no_such_folder_45/deeper/frame_" -F PNG -f 1`, where neither `no_such_folder_45` nor `deeper` existed. Exit status 0, standard error empty, and standard output ended:

```
Saved: '/private/tmp/smoozy-test/hand-tests/Reports/007/no_such_folder_45/deeper/frame_0001.png'
Time: 00:00.14 (Saving: 00:00.04)
```

Both folders existed afterwards and held the image.

**3. The output path cannot be written.** A one-byte plain file named `blocker_45` was made, and the output path was set inside it, so no folder could be created there. Run: `-b "simple.blend" -o "<folder>/blocker_45/frame_" -F PNG -f 1`. Exit status 1. Standard output ended:

```
Fra:1 Mem:10.36M (Peak 10.95M) | Time:00:00.09 | Mem:1.48M, Peak:1.48M | Scene, ViewLayer | Finished
Error: Render error (Not a directory) cannot save: '/private/tmp/smoozy-test/hand-tests/Reports/008/blocker_45/frame_0001.png'
Time: 00:00.13


Blender quit
```

Standard error held one line and nothing else:

```
/private/tmp/smoozy-test/hand-tests/Reports/008/blocker_45/frame_0001.png: Not a directory
```

The render itself finished; the saving is what failed.

**4. An output path with a broken template in it.** Run: `-b "simple.blend" -o "<folder>/out_45/{blend_name.png" -F PNG -x 1 -a`. Exit status 1, standard error empty, and the whole of standard output:

```
Blender 4.5.12 LTS (hash 84afd5f785f7 built 2026-07-21 01:41:35)
Read blend: "/private/tmp/smoozy-test/hand-tests/Reports/018/simple.blend"
Error: Parse errors in path '/private/tmp/smoozy-test/hand-tests/Reports/018/out_45/{blend_name.png':
- Invalid or incomplete template expression '{blend_name.png'.

Blender quit
```

No frame was rendered and the output folder was empty.

**5. An animation whose output folder stops being writable partway.** A six-frame Cycles animation at 2000 samples, 320 by 240, about seven and a half seconds a frame. It was started, and once `frame_0001.png` existed the output folder was renamed away and a one-byte plain file was put in its place. Exit status 1. Standard output, at the point it went wrong:

```
Fra:2 Mem:14.83M (Peak 18.34M) | Time:00:07.41 | Mem:4.19M, Peak:4.19M | Scene, ViewLayer | Finished
Error: Render error (Not a directory) cannot save: '/private/tmp/smoozy-test/hand-tests/Reports/012/out_45/frame_0002.png'
Time: 00:07.41
```

Nothing was printed for frames 3, 4, 5 or 6. The renamed folder held one file, `frame_0001.png`. Standard error held the same one-line `: Not a directory` message as in item 3.

**6. The blend-file does not exist.** Run: `-b "<folder>/there_is_no_such_file.blend" -f 1`. Exit status 1, standard error empty, whole of standard output:

```
Blender 4.5.12 LTS (hash 84afd5f785f7 built 2026-07-21 01:41:35)
Error: Cannot read file "/private/tmp/smoozy-test/hand-tests/Reports/010/there_is_no_such_file.blend": No such file or directory

Blender quit
```

**7. A Cycles device override Cycles will not take — this is item i of the documented list above, run.** Run: `-b "simple.blend" -o "<folder>/out_45/frame_" -F PNG -f 1 -- --cycles-device NONSENSE`. Exit status 1. Standard output held one line, the version banner. Standard error held:

```

Error: NONSENSE is not a valid Cycles device.
```

No frame was rendered and the output folder was empty. The same command with `--cycles-device METAL` instead rendered normally and ended with status 0. So a Cycles device Cycles will not take ends the whole Blender process with status 1 before any render happens. (Citation check 2026-08-05: this used to say the `sys.exit(1)` in `engine.py` is what does it. The run shows the status and the missing frame; it does not show which line of code ended the process, so the sentence was narrowed to what was seen.)

**8. An image texture that is missing when the render starts.** A cube was given a material with an image texture pointing at `gone.png`, the file was saved, and `gone.png` was then moved away. Run: `-b "missing.blend" -o "<folder>/out_45/frame_" -F PNG -f 1`. Exit status 0. The image was written. Searching the whole of both streams for "error", "warn", "cannot", "unable", "missing" and "not found", case ignored, matched no line about the texture — Blender 4.5 said nothing at all about it.

**9. Turning off automatic saving of render output.** Run: `-b "simple.blend" --python-expr "import bpy; bpy.context.scene.render.save_output = False" -o "<folder>/out2_45/frame_" -F PNG -x 1 -a`. Exit status 0. All three frames were rendered and saved. Standard error held:

```
Traceback (most recent call last):
  File "<string>", line 1, in <module>
AttributeError: 'RenderSettings' object has no attribute 'save_output'
```

So there is no such setting in 4.5, and the render went ahead as if nothing had been asked.

**Observed, 2026-08-05 — two of those failures repeated with Blender's window open.** Run by the Creator on their own MacBook Pro (Apple M1 Pro, macOS 26.5.2), Blender 4.5.12, reported as screenshots.

*No camera.* The prepared file with its camera deleted was opened and F12 pressed. No Blender Render window opened at all. The scene stayed on screen and an error was shown in three places at once (quoted under question 2).

*A broken template in the output path.* A file whose output path read `//out_{blend_name.png` was rendered. **A Blender Render window did open**, showing an empty checkerboard and no image, and an error was shown (quoted under question 2). Before the render, the output path field itself was drawn with a red background, and resting the pointer on it gave a tooltip reading, word for word:

```
Output Path
Directory/name to save animations, # characters define the position and padding of frame numbers.
Value: //out_{blend_name.png
Path template error(s):
  - Invalid or incomplete template expression '{blend_name.png'.
Python: RenderSettings.filepath
bpy.data.scenes["Scene"].render.filepath
```

**Observed, 2026-08-05 — the Output panel's header, looked at by hand.** Done by the Creator on their own MacBook Pro (Apple M1 Pro, macOS 26.5.2), in Blender 4.5.

What was asked of them, the same in all four versions: open Blender at its normal startup file, go to the Properties editor, choose the Output tab, find the panel headed `Output`, and look at the header strip of that panel next to the word `Output`. If a checkbox is drawn there, rest the pointer on it and wait for the tooltip.

What came back: **there is no checkbox in that header in 4.5.** The Creator reported that the checkbox exists only in 5.2. This version was looked at and has none, which is a result rather than a silence. With no checkbox here, there was no tooltip to read. The sources for this version describe no such switch either.

The Creator works on a Mac, so this settles macOS and leaves Windows and Linux open. Their answer says nothing about the checkboxes inside the panel, and that is left open rather than filled in.

(Citation check 2026-08-05: this used to be a note about what a screenshot showed, with nothing about what was done to get the screenshot, so nobody could repeat it. The check asked for the procedure, and the record above replaces it. The steps are in `Audits/Blender Research/Reports/Hand tests.md`, entry 24.)

**Observed, 2026-08-06 — the same failures, and six more, on a second machine.**

Everything in this block was run on **a rented Linux machine — Ubuntu 22.04.5 LTS, NVIDIA GeForce RTX 4090, driver 580.105.08, twelve cores, run over SSH**. Blender 4.5.12 LTS at `/opt/blender/blender-4.5.12-linux-x64/blender`, started with `-b`, pointed at its own settings folder, with the display turned off first (`unset DISPLAY`, `unset XAUTHORITY`). The scene was the same one-cube factory scene described above, rebuilt on that machine. The machine is a rented container shared with other work, so nothing below says how long anything took or how much memory was free. Full commands in `Audits/Blender Research/Reports/Hand tests — Linux.md`.

**The Mac's results held.** No camera, a missing output folder, a blocked output path, a broken path template, a missing blend-file, a refused Cycles device, an animation that loses its output folder partway, a missing image texture, and asking for the output-saving setting — all nine came back with the same exit statuses and the same wording as on the Mac, with one exception named below. Both machines were run and both agreed.

**The one difference in wording.** When the output could not be written, Blender 4.5 on Linux printed:

```
Error: Render error (Invalid argument) cannot save: '/root/smoozy/Reports/work/L08/blocker_45/frame_0001.png'
```

On the Mac the same failure read `Render error (Not a directory) cannot save: '…'`. The bare line on standard error reads the same on both machines — the path, then `: Not a directory`.

**A genuinely read-only output folder — new, and not reachable on the Mac.** The folder was made with `chmod 0555`. On this machine the user is root, and root walks through file permissions, so the run was made twice. Run plain, the permission was ignored: exit 0 and the image written. Run again with that overriding permission dropped for the one command — `capsh --drop=cap_dac_override,cap_dac_read_search -- -c "…"` — exit status **1**, nothing written, and standard output ended:

```
Fra:1 Mem:10.47M (Peak 11.06M) | Time:00:00.16 | Mem:1.48M, Peak:1.48M | Scene, ViewLayer | Finished
Error: Render error (Invalid argument) cannot save: '/root/smoozy/Reports/work/L25/ro_45/frame_0001.png'
Time: 00:00.23
```

Standard error held one line:

```
/root/smoozy/Reports/work/L25/ro_45/frame_0001.png: Permission denied
```

**A corrupt blend-file — new.** Two were tried. A file cut short (the first 200 kB of a good one) gave exit status 1 and:

```
Error: Failed to read blend file '/root/smoozy/Reports/work/L24/truncated.blend': Missing DNA block
```

A file that was not a blend-file at all gave exit status 1 and:

```
Error: File format is not supported in file "/root/smoozy/Reports/work/L24/garbage.blend"
```

**A blend-file written by a newer Blender — new, and it is not a failure.** The same scene was saved by Blender 5.2 and opened here. Exit status **0**, the frame rendered and written. Sixteen warning lines were printed, of three kinds about missing interface regions, plus this one:

```
Warning: File written by newer Blender binary (502.44), expect loss of data!
```

**A crash, and where the crash file goes on Linux — new. Item g of the documented list above, run.** Blender was made to crash on purpose with a script whose whole content was `import ctypes`, a `print`, and `ctypes.string_at(0)`. With `simple.blend` loaded, standard output ended:

```
Blender 4.5.12 LTS (hash 84afd5f785f7 built 2026-07-21 01:31:22)
Read blend: "/root/smoozy/Reports/work/simple.blend"
Writing: /tmp/simple.crash.txt
```

The process ended with status **139**. A file `/tmp/simple.crash.txt` existed afterwards, holding a version line, a section headed `# backtrace`, and a section headed `# Python backtrace`. Run again with `--factory-startup` and no file loaded, the line read `Writing: /tmp/blender.crash.txt` and that file was written instead. So on Linux the crash file is named after the blend-file when there is one, and `blender.crash.txt` when there is not — the manual's page names only the second. The `print` that ran immediately before the crash never reached standard output.

**Which handlers fire on four more kinds of failure — new.** The probe described under question 2 was run again on this machine against five cases, not one. On a blocked save: `render_init`, `render_pre`, `render_cancel`. On a broken path template: `render_init` and `render_cancel` only — `render_pre` never fired. On a missing image texture, which succeeds: the full successful set. So `render_cancel` on failure is now seen on three different failures in this version rather than one.

**A Cycles device this card does offer.** `-- --cycles-device CUDA` and `-- --cycles-device OPTIX` were both accepted and both rendered the frame, exit 0. The refused name behaved exactly as on the Mac: exit 1, no frame, the version banner alone on standard output and `Error: NONSENSE is not a valid Cycles device.` on standard error.

**Outside the interface.** The nine cases above were all run with no window at all, so they hold when Blender is driven from outside it. Two of them were also run by the Creator with the window open and failed there too, so those two are not an artefact of running headless. A failure that stops the job leaves the process with status 1 and a success leaves status 0, which is a reading a caller outside Blender can take. Two things do not follow that rule: a missing image texture (item 8) and, in versions that have it, a disabled output setting both leave status 0. For the documented graphics-card errors in items a–e of the list above, the sources still do not say where those messages appear when there is no window; those were not among the cases run. The crash files in documented item g are written by Blender itself and the page does not condition them on the interface. Documented item f's "kill the application" is described by the source with no reference to the interface.

**Operating system.** This one is heavily operating-system-specific and the sources say so. c is Windows only. d is Linux only. a–b and e are tied to CUDA/OptiX (Windows and Linux), HIP (Windows and Linux) and Metal (macOS on Apple Silicon) as listed on the GPU Rendering page. g differs on every operating system — different file, different place, quoted above. h names Windows for the AMD RDNA2 artifacts and Ubuntu/Linux for one of the Intel issues. i and j: the sources do not say.

Two machines have now been used, and they answer different halves. **macOS** is settled by the Creator's own MacBook Pro (Apple M1 Pro, macOS 26.5.2) for the nine cases above. **Linux** is settled by the rented Linux machine for those same nine plus six more, as recorded above; both machines agreed everywhere except the one word in the saving-failure message. **Windows is untested and no Windows machine has run it**, so item c and the Windows half of item g stay open.

Documented item **d, the Linux "Unsupported GNU version", could not be reached even on the Linux machine**, and the reason is worth keeping. That error comes from the CUDA toolkit's compiler being handed an unsupported GCC, and it only arises when Blender has to build a CUDA kernel while it runs. There is no CUDA toolkit on that machine — `which nvcc` returns nothing and `/usr/local/cuda` does not exist — and this Blender ships a kernel already built for that card, which `--debug-cycles` states outright: `Testing for pre-compiled kernel …/kernel_sm_89.cubin.zst.` then `Using precompiled kernel.` So no compile is ever attempted. Installing the toolkit was not permitted on that machine. Item **e** is unreached for the same reason.

Item **a**, the fallback to system memory when the card fills, still has not been seen happen. It needs a graphics card whose memory can be exhausted while the machine's own memory is not — which the rented Linux machine can do, but only with nothing else running on it. It is written out in full as a test in `Audits/Blender Research/Reports/Hand tests — Linux.md` and has not been run. **HIP** needs an AMD card and **oneAPI** an Intel card; neither machine has one.

---

## 2. Where does Blender say that a render failed, and what does it say there?

**LABEL: documented for general reporting places; inferred that render failures use them; not in the sources for the wording of a general render-failure message.**

There are four general reporting places named in the sources, and one Python route. The quoted words do not establish that every render failure appears in each place.

**The Status Bar can show report messages briefly.** *documented; inferred for render failures*

> "Report Message
>
> Informational messages or warnings, such as after saving a file. They disappear after a short time. Click them to show the full message in the Info Editor."

Blender manual, https://docs.blender.org/manual/en/4.5/interface/window_system/status_bar.html, read 2026-07-30.

**The Info editor keeps errors, warnings and informational messages.** *documented; inferred for render failures*

> "The Info editor logs the executed operators as well as errors, warnings, and informational messages."

Blender manual, https://docs.blender.org/manual/en/4.5/editors/info_editor.html, read 2026-07-30.

The Info editor can be filtered by kind, which tells you the kinds it holds: `show_report_debug` ("Display debug reporting info"), `show_report_error` ("Display error text"), `show_report_info` ("Display general information"), `show_report_operator` ("Display the operator log"), `show_report_warning` ("Display warnings"). Python API, https://docs.blender.org/api/4.5/bpy.types.SpaceInfo.html, read 2026-07-30.

I opened the picture on the Info editor page (`_images/editors_info-editor_ui.png`, shown on the page above). It shows three rows, each a `bpy.ops....` line with a green tick at the left — operator log entries only. No error or warning row is visible in it, so it does not show me what a failure looks like there. *documented, from the picture; nothing more read into it.*

**The message carries a type, and one of the types is specifically out-of-memory.** *documented*

The full set a report can be, quoted from the enum page:

> "DEBUG: … INFO: … OPERATOR: … PROPERTY: … WARNING: … ERROR: … ERROR_INVALID_INPUT: Invalid Input. ERROR_INVALID_CONTEXT: Invalid Context. ERROR_OUT_OF_MEMORY: Out of Memory."

Python API, https://docs.blender.org/api/4.5/bpy_types_enum_items/wm_report_items.html, read 2026-07-30.

The `RenderEngine` API provides these reporting calls:

> "report(type, message)
>
> Report info, warning or error messages"

and, separately,

> "error_set(message)
>
> Set error message displaying after the render is finished"

Python API, https://docs.blender.org/api/4.5/bpy.types.RenderEngine.html, read 2026-07-30. Note that `error_set` says *when* ("after the render is finished") but not *where*.

**The console window / terminal shows operational messages and internal errors.** *documented; inferred for render failures generally*

> "The Console Window, also called a Terminal, is an operating system text window that displays messages about Blender's operations, status, and internal errors."

Blender manual, https://docs.blender.org/manual/en/4.5/advanced/command_line/index.html, read 2026-07-30. And:

> "Other messages can indicate serious errors that will most likely prevent Blender carrying out a particular task and may even make Blender non-responsive or shut down completely."

Blender manual, https://docs.blender.org/manual/en/4.5/troubleshooting/startup.html, read 2026-07-30.

The actual error *text* the sources give for renders is the Cycles graphics-card set quoted in question 1: "Error: Out of memory", "The NVIDIA OpenGL driver lost connection with the display driver", "Unsupported GNU version", "CUDA Error: Kernel compilation failed". Those are the only render error strings I found written out anywhere in the sources.

**To a Python script, through the operator's result.** *documented*

> "Operators don't have return values as you might expect, instead they return a set() which is made up of: {'RUNNING_MODAL', 'CANCELLED', 'FINISHED', 'PASS_THROUGH'}. Common return values are {'FINISHED'} and {'CANCELLED'}, the latter meaning that the operator execution was aborted without making any changes or saving an undo history entry.
>
> If operator was cancelled but there wasn't any reports from it with {'ERROR'} type, it will just return {'CANCELLED'} without raising any exceptions. However, if there are error reports, a RuntimeError will be raised after the operator finishes execution, including all error report messages, regardless of the return status (even if it was {'FINISHED'})."

Python API, https://docs.blender.org/api/4.5/bpy.ops.html, read 2026-07-30. `bpy.ops.render.render(...)` is an ordinary operator ("Render active scene", https://docs.blender.org/api/4.5/bpy.ops.render.html), so this is the route by which a render failure reaches a calling script. That the render operator in particular behaves this way is **inferred** — the step is that `bpy.ops` describes all operators and `render.render` is one of them; the page never names rendering.

**A thing the sources are conspicuously missing.** The list of things a script can be told about a render is:

> "bpy.app.handlers.render_cancel — on canceling a render job…
> bpy.app.handlers.render_complete — on completion of render job…
> bpy.app.handlers.render_init — on initialization of a render job…
> bpy.app.handlers.render_post / render_pre / render_stats / render_write"

Python API, https://docs.blender.org/api/4.5/bpy.app.handlers.html, read 2026-07-30. There is a "cancel" and a "complete". There is no "failed". *documented* that no such handler is listed; whether a failed render therefore fires `render_cancel`, or nothing at all, the sources do not say.


**Observed — a neighbour process holding the card's memory, 2026-08-06.** This is the case a rented machine actually produces: the card is not filled by this render, it is filled by somebody else's. It behaves in the opposite way to a scene too big for the card, and the difference is the finding.

*What was done, to what.* On a rented Linux machine with four NVIDIA GeForce RTX 4060 Ti of 16 GB each, driver 580.95.05, no display server, over SSH, with nothing else running. A separate small program took and held **15,616 MiB** of card 0 — almost all of it — leaving the other three cards untouched. Blender was then asked to render one frame at 1024 by 1024, 64 samples, on **card 0 alone**.

```
HOLDING 15616 MiB on card 0
card 0, 15744 MiB
card 1, 3 MiB
card 2, 3 MiB
card 3, 3 MiB
```

*What came back.* Blender **failed, loudly, and exited 1**:

```
4.5   exit 1   Out of memory in CUDA queue enqueue (integrator_shade_surface)
              RuntimeError: Error: Out of memory in CUDA queue enqueue (integrator_shade_surface)

5.2   exit 1   ERROR Out of memory in CUDA queue enqueue (integrator_shade_surface)
              ERROR Refer to the Cycles GPU rendering documentation for possible solutions:
              ERROR System is out of GPU memory
```

*Why this matters more than it looks.* When **this render's own scene** is too big for the card, Blender says nothing, exits 0, writes the picture and takes about ten times longer — it moves the work quietly. When **another process** has taken the card's memory, Blender stops and says so plainly. The card is equally full in both cases. The behaviour is opposite, and which one happens depends on who consumed the memory rather than on how much is left.

For anything renting a machine by the hour, that is the useful shape: a neighbour is loud and a heavy scene is silent, and the silent one is the expensive one.

*On what kind of computer.* The four-card rented Linux machine described above. **Run in Blender 4.5 and 5.2 only**, and settles those two on Linux with NVIDIA cards. 5.0 and 5.1 were not run for this, and Windows is untried everywhere.

**Not covered.** This does not settle: the exact words Blender uses when a render fails for any non-graphics-card reason; whether anything is shown in the Render window or Image editor itself; whether the failure text is printed to the terminal in background mode and in what shape; and whether the Blender process ends with a distinguishable status. On that last point the closest the sources come is unrelated to rendering — for custom command-line commands registered from Python, `bpy.utils.register_cli_command` says "The return value should be 0 for success, 1 on failure (specific error codes from the os module can also be used)" (https://docs.blender.org/api/4.5/bpy.utils.html). That is about your own `-c` command, not about a render, and I draw nothing about renders from it.

**Where I looked, for the silence.** Searched the whole manual, API and release notes for "exit code", "return code", "exit status", "error code", "non-zero". No hits at all for the first three. Read the Status Bar page, the Info editor page, the Wm Report Items enum, the whole `bpy.types.RenderEngine` page, `bpy.app.handlers`, `bpy.ops`, `bpy.types.Operator`, `bpy.types.SpaceInfo`, the command-line Index and Arguments pages, Troubleshooting → Startup and → Crashes, and the Cycles GPU Rendering "Error Messages" section.

**Observed, 2026-08-05 — where a failed render says so with no window, and what it says.**

Same computer and same way of running as in question 1: a MacBook Pro with an Apple M1 Pro chip, macOS 26.5.2, Apple's own graphics, Blender 4.5.12 LTS started with `-b` and its own throwaway settings folder. The two output streams were kept in separate files every time.

**Nearly every failure message went to standard output.** Across the nine runs in question 1, standard error was completely empty in five of them — items 1, 2, 4, 6 and 8. It carried something in four: the save failure, one bare line, `/private/tmp/.../blocker_45/frame_0001.png: Not a directory` (item 3); the animation that lost its output folder partway, the same one-line message (item 5); the refused Cycles device, whose whole message was on standard error and nothing on standard output but the version banner (item 7); and the run that asked for a setting this version does not have, a Python traceback (item 9). On a successful render standard error was empty and nothing was written to it at all. (Citation check 2026-08-05: this used to say standard error was empty in seven of the nine with two exceptions, which contradicted the runs recorded under question 1 — items 5 and 9 both wrote to it. Corrected against those records.)

**The exact words, quoted from the runs.** Each of these is the complete failure text that appeared:

```
Error: Cannot render, no camera
```

```
Error: Render error (Not a directory) cannot save: '/private/tmp/smoozy-test/hand-tests/Reports/008/blocker_45/frame_0001.png'
```

```
Error: Parse errors in path '/private/tmp/smoozy-test/hand-tests/Reports/018/out_45/{blend_name.png':
- Invalid or incomplete template expression '{blend_name.png'.
```

```
Error: Cannot read file "/private/tmp/smoozy-test/hand-tests/Reports/010/there_is_no_such_file.blend": No such file or directory
```

```
Error: NONSENSE is not a valid Cycles device.
```

Every one of them opens with the word `Error:` and a space.

**The process status.** Read with `echo $?` straight after each run. A render that finished and saved: 0. No camera: 1. Cannot save the output: 1. Broken path template: 1. Blend-file missing: 1. Bad Cycles device: 1. An animation that lost its output folder partway: 1. A render with a missing image texture, which finished anyway: 0.

**Which handlers fire when a render fails.** A script was loaded with `--python` that attached a printing handler to each of `render_init`, `render_pre`, `render_post`, `render_write`, `render_complete`, `render_cancel` and `render_stats`, and printed a line naming any that did not exist. All seven existed. On a render that succeeded, the lines printed, in this order, were:

```
HANDLER FIRED: render_init args: (bpy.data.scenes['Scene'], None)
HANDLER FIRED: render_pre args: (bpy.data.scenes['Scene'], None)
```

then a long run of `render_stats` lines, then:

```
HANDLER FIRED: render_post args: (bpy.data.scenes['Scene'], None)
HANDLER FIRED: render_write args: (bpy.data.scenes['Scene'], None)
HANDLER FIRED: render_complete args: (bpy.data.scenes['Scene'], None)
```

On the no-camera file, the whole of what printed was:

```
PROBE INSTALLED
HANDLER FIRED: render_init args: (bpy.data.scenes['Scene'], None)
HANDLER FIRED: render_pre args: (bpy.data.scenes['Scene'], None)
HANDLER FIRED: render_cancel args: (bpy.data.scenes['Scene'], None)
```

So this failed render — the no-camera file — fires `render_cancel`, and does not fire `render_complete`, `render_post` or `render_write`. (Citation check 2026-08-05: this used to say "a failed render". Only the no-camera failure was probed this way, so the sentence was narrowed to it. Whether the other eight failures behave the same is not settled here.)

**Whether reports can be read from Python with no window.** A script run after the failing render printed:

```
REPORTS: attribute not present in this version
```

That line is what the script prints when `bpy.context.window_manager` has no `reports` attribute. Blender 4.5 has no such list.

**Observed, 2026-08-05 — where a failed render is announced with the window open, and what it says.** Run by the Creator on their own MacBook Pro (Apple M1 Pro, macOS 26.5.2), Blender 4.5.12, reported as screenshots. In both cases the Info editor was on screen with its error and warning filters on.

**A scene with no camera, rendered with F12.** The message appeared in **three places at once**, each carrying a round white-on-red cross icon, and reading the same words:

- along the bottom of the window, on a red field: `No camera found in scene "Scene"`
- in a floating box in the middle of the screen headed `Report: Error`, below which: `No camera found in scene "Scene"`
- as an entry in the Info editor: `No camera found in scene "Scene"`

**That is not the wording the same failure prints with no window.** Headless, the same file printed `Error: Cannot render, no camera`. In the window it reads `No camera found in scene "Scene"`. Both were got from the same prepared file on the same machine on the same day.

**A broken template in the output path, rendered with Ctrl-F12.** Again in three places, with the same icon:

- along the bottom of the window: `Parse errors in path '//out_{blend_name.png': - Invalid or incomplete template expression '{blend_name.png'.`
- in a floating box headed `Report: Error`, laid out on two lines: `Parse errors in path '//out_{blend_name.png':` then, indented, `- Invalid or incomplete template expression '{blend_name.png'.`
- in the Info editor, on two lines, the same words.

Here the window and the terminal agree: headless, the same failure printed `Error: Parse errors in path '…':` and `- Invalid or incomplete template expression '…'.` The path in the Creator's file was `//out_{blend_name.png`, which is not the path used in the headless run, so the quoted path differs while the wording does not.

**A gap in what was reported.** The screenshots do not show whether either message stays in the Info editor or disappears, and they do not show what icon the Info editor entry carries beyond the cross described above. Neither was captured and neither is filled in here.

**Observed, 2026-08-06 — the same questions asked on a second machine.**

Run on the rented Linux machine, Blender 4.5.12 LTS, `-b`, its own settings folder, display turned off first. The Mac and this machine were both run and are reported separately below.

**The wordings agreed.** Every failure text quoted above came back word for word on Linux, except the saving failure, whose reason in brackets reads `Invalid argument` there and `Not a directory` on the Mac. The exit statuses agreed everywhere: 1 on failure, 0 on success.

**What the render operator hands a script — this was inferred above and is now seen.** A script was loaded with `--python` that called `bpy.ops.render.render(write_still=True)` inside a `try`. On the good file it printed:

```
RENDER RETURNED: {'FINISHED'}
```

On the no-camera file it printed:

```
RENDER RAISED: RuntimeError Error: Cannot render, no camera
```

So the general `bpy.ops` behaviour quoted above does hold for the render operator: a failing render raises a `RuntimeError` carrying the message. The same two runs with Blender's window open gave the same two results, except that the message in the raised error read `Error: No camera found in scene "Scene"` — the window's wording, not the terminal's. The failing runs with no window ended at status 1 even though the script caught the exception; the succeeding ones ended at 0.

**No readable list of reports, with a window or without.** The same script as above printed `REPORTS: attribute not present in this version` on this machine too, both with no window and with the window open. Blender 4.5 has no such list either way.

**The window wording is not a macOS oddity.** The no-camera file was rendered again with Blender's window open on the Linux machine, the render started from a script on a timer. The error raised read:

```
RuntimeError Error: No camera found in scene "Scene"
```

which is the same wording the Creator saw on their Mac, and not the `Cannot render, no camera` that the same file prints with no window. Both machines were run and both agreed.

**A gap in that Linux window run, written in rather than filled.** An Info editor was put on screen with its error, warning, information and operator filters on, and the window was photographed shortly after the failure and again twenty seconds later. **No entry appeared in the Info editor at all**, in either picture. The Creator's Mac run started the render by pressing F12; this one started it from a script. The two differ in how the render was started as well as in the machine, so nothing here says whether an Info-editor entry stays or fades, and that question is still open.

**Where the failure message goes when a log file is asked for.** Run with `--log-file <path>` and no `--log` category named, Blender 4.5 created the file and left it **empty**, and the failure still appeared on standard output exactly as usual, exit 1. (This is not what the newer versions do — see question 4.)

**A warning about standard error on Linux.** On this machine the shell arrives with `DISPLAY` set to `:0` and no authority file. Run that way, Blender 4.5 with no window wrote nothing to standard error on a successful render. Blender 5.2 in the same conditions wrote four lines there; those words come from the display libraries rather than from Blender, and every run recorded here had the display turned off first. The point that carries: on a Linux machine with a display running, standard error is not reliably empty even when nothing has gone wrong.

**Outside the interface.** Settled by the tests above for the cases run, and now on both sides of the window. With no window at all, a failed render prints its message — every message beginning `Error: ` — and leaves the process with status 1, and a successful one leaves 0. With the window open the same two failures are announced in the Status Bar, in a floating report box and in the Info editor. For the no-camera failure the wording is **not** the same in the two settings, so a caller outside Blender must not expect the window's words. Standard output carries it, except for the cases named above. On the one failure probed for it, `render_cancel` fires and `render_complete` does not. There is no readable list of reports in this version. Of the five documented places, two remain interface-only by their own description: the Status Bar report message and the Info editor are both parts of Blender's window. The console window is the one that is not — its page is written for a Blender started from a terminal. The Python route is no longer only a page: it was run on the rented Linux machine with no window and with one, and in both settings the render operator returns `{'FINISHED'}` on success and raises a `RuntimeError` carrying the message on failure. `RenderEngine.error_set` says "displaying after the render is finished" without saying where, and no test here settled that.

**Operating system.** Nothing in the report/Info/Python material names an operating system. For the console specifically the sources do differ: on Windows the console is a separate Command Prompt window that Blender toggles off, reachable again via "Window ‣ Toggle System Console" or started open with `-con, --start-console` ("(Windows only)"); on macOS and Linux you get output by running the executable from a terminal. All from https://docs.blender.org/manual/en/4.5/advanced/command_line/launch/windows.html, `.../macos.html`, `.../linux.html`, and the Arguments page, read 2026-07-30.

**Both machines have now been used and they agree on the words.** Every failure message quoted here came back identically on macOS 26.5.2 and on the rented Linux machine, apart from the one word in the saving failure recorded above. The window's own wording for the no-camera failure, `No camera found in scene "Scene"`, was seen on both. **Windows is untested and no Windows machine has run it**, so the separate Command Prompt window, `Window ‣ Toggle System Console` and `-con, --start-console` remain unchecked. The three places a failure is announced — the Status Bar, a floating report box and the Info editor — were seen on macOS only; the Linux window run did not produce an Info-editor entry, for the reason written above.

---

## 3. What does Blender report about how far through a piece of work it has got?

**LABEL: documented in part; not in the sources for how Cycles in particular reports progress; not in the sources for progress outside the interface.** (Citation check 2026-08-05: the middle clause used to read "inferred for how Cycles uses the general render-engine calls". That inference did not hold on its quote and was removed below.)

**In the window, a labelled bar with a percentage and a cancel button.** *documented*

> "Running Task
>
> Shows the progress of the currently running task (such as rendering or baking). Hovering the mouse pointer over the progress bar will display a time estimate. The task can be aborted by clicking the cancel button ()."

Blender manual, https://docs.blender.org/manual/en/4.5/interface/window_system/status_bar.html, read 2026-07-30.

I opened the picture on that page, `_images/interface_window-system_status-bar_ui-center.png`. It shows a small film-camera icon, the words "Sequence Render", a horizontal bar filled most of the way, the text "94%" sitting on the bar, and an X button at the right. So the progress readout carries a task name, a filled bar, a whole-number percentage, and a cancel control. *documented, from that picture; I read nothing into it beyond what is visible.*

**Through the API, a fraction from 0 to 1.** *documented*

> "update_progress(progress)
>
> Update progress percentage of render
>
> Parameters:
>
> progress (float in [0, 1]) – Percentage of render that's done"

And beside it, free-form status text:

> "update_stats(stats, info)
>
> Update and signal to redraw render status text
>
> Parameters:
>
> stats (string, (never None)) – Stats
>
> info (string, (never None)) – Info"

Python API, https://docs.blender.org/api/4.5/bpy.types.RenderEngine.html, read 2026-07-30. These are the calls a render engine has available for reporting progress and status. (Citation check 2026-08-05: this used to go on to say that, Cycles being a render engine, this is how Cycles' progress reaches Blender. The step does not hold on the quoted words: the page never names Cycles, and listing the calls a render engine has does not say which engines call them. Whether Cycles reports its progress this way is not settled by the sources.)

**The general job-progress calls, not render-specific.** *documented*

> "progress_begin(min, max) — Start progress report … progress_update(value) — Update the progress feedback … progress_end() — Terminate progress report"

Python API, https://docs.blender.org/api/4.5/bpy.types.WindowManager.html, read 2026-07-30.

**Not covered.** Two large gaps. First, the sources say nothing about progress being *printed* — no line format, no per-frame or per-sample text, nothing about what a terminal shows while a render runs. Second, "94%" in the picture is a progress bar for one task; the sources never say what that percentage is a percentage *of* during an animation render — one frame, or the whole frame range. `update_progress` is titled "Percentage of render that's done", which does not settle it either.

**Where I looked, for that silence.** Whole-tree search of manual, API and release notes for "progress" (every hit reviewed; the only render-relevant ones are the Status Bar page, the Preferences → Interface page which repeats it, `RenderEngine`, `WindowManager`, `HydraRenderEngine`). Also searched for "Path Tracing", "Synchronizing", "Loading render kernels", "Rendering Sample", "Updating Scene", "Waiting for render" across manual, API, release notes and the Cycles files — no hits of any kind, so no per-sample status line is written down anywhere in what I was given. Read the command-line Index, Arguments and Rendering From The Command Line pages: none mentions progress output.

**Observed, 2026-08-05 — what progress a render prints with no window.**

Same computer and same way of running as in question 1. Blender 4.5.12 LTS, `-b`, own settings folder, Cycles, 16 samples, 160 by 120 pixels, output to PNG.

**A single frame.** Run: `-b "simple.blend" -o "<folder>/frame_" -F PNG -f 1`. Standard output ran to 49 lines, all of it on standard output and none on standard error. Every line during the render had this shape, and here are the last six of them exactly as printed:

```
Fra:1 Mem:8.30M (Peak 9.10M) | Time:00:00.01 | Mem:0.09M, Peak:0.09M | Scene, ViewLayer | Sample 0/16
Fra:1 Mem:9.77M (Peak 9.77M) | Time:00:00.02 | Remaining:00:00.05 | Mem:1.48M, Peak:1.48M | Scene, ViewLayer | Sample 1/16
Fra:1 Mem:10.36M (Peak 10.95M) | Time:00:00.16 | Mem:1.48M, Peak:1.48M | Scene, ViewLayer | Sample 16/16
Fra:1 Mem:10.36M (Peak 10.95M) | Time:00:00.16 | Mem:1.48M, Peak:1.48M | Scene, ViewLayer | Finished
Saved: '/private/tmp/smoozy-test/hand-tests/Reports/001/out_45/frame_0001.png'
Time: 00:00.22 (Saving: 00:00.05)
```

Earlier lines in the same run named the stage rather than a sample count, for example:

```
Fra:1 Mem:8.21M (Peak 9.10M) | Time:00:00.01 | Mem:0.00M, Peak:0.00M | Scene, ViewLayer | Waiting for render to start
Fra:1 Mem:8.29M (Peak 9.10M) | Time:00:00.01 | Mem:0.00M, Peak:0.00M | Scene, ViewLayer | Updating Scene BVH | Building BVH 0%
Fra:1 Mem:8.30M (Peak 9.10M) | Time:00:00.01 | Mem:0.09M, Peak:0.09M | Scene, ViewLayer | Loading denoising kernels (may take a few minutes the first time)
```

So each line carries the frame number, two memory figures, an elapsed time, sometimes a remaining time, the scene and view layer, and the stage or sample count. (Citation check 2026-08-05: this used to open "So one line per change of state". Nine of the run's forty-nine lines are recorded here and nothing in them ties a line to a change of state, so that part was removed.)

**A three-frame animation.** Run: `-b "simple.blend" -o "<folder>/frame_" -F PNG -x 1 -a`, frame range 1 to 3. Exit status 0, three files written. The whole run was 137 lines. No line anywhere gave a figure for the animation as a whole. The frame number in the first field changed from `Fra:1` to `Fra:2` to `Fra:3`, and each frame ran its own sequence from `Sample 0/16` to `Sample 16/16`, `Finished`, `Saved:` and `Time:`. Nothing announced the start of a frame; the frame number in the running line is the only marker. There is no line naming the frame range.

**What the render-statistics handler is handed.** A script attached a handler to `bpy.app.handlers.render_stats` that printed the arguments with `repr`. It fired 127 times over the three-frame animation. The strings it received are the same lines that were printed, for example:

```
RENDER_STATS: ('Fra:2 Mem:9.77M (Peak 10.95M) | Time:00:00.00 | Remaining:00:00.01 | Mem:1.48M, Peak:1.48M | Scene, ViewLayer | Sample 1/16', None)
RENDER_STATS: ('Fra:3 Mem:10.36M (Peak 10.95M) | Time:00:00.08 | Mem:1.48M, Peak:1.48M | Scene, ViewLayer | Finished', None)
RENDER_STATS: ('Time: 00:00.08 (Saving: 00:00.00)', None)
```

It is handed one argument, a string, plus a second value which printed as `None`. The handler fires with no window open.

**A non-Cycles engine.** The same file with `bpy.context.scene.render.engine` set to `BLENDER_EEVEE_NEXT` printed far less. (Citation check 2026-08-05: this used to add "the only EEVEE identifier this version offers". Nothing in the record says how that was established, so it was removed.) The whole run was 11 lines and the render part was:

```
Fra:1 Mem:40.65M (Peak 40.65M) | Time:00:00.28 | Rendering 1 / 64 samples
Fra:1 Mem:40.73M (Peak 40.73M) | Time:00:00.46 | Rendering 25 / 64 samples
Fra:1 Mem:40.73M (Peak 40.73M) | Time:00:00.57 | Rendering 50 / 64 samples
Fra:1 Mem:40.73M (Peak 40.73M) | Time:00:00.63 | Rendering 64 / 64 samples
```

No scene or view-layer field, no second memory pair, no remaining time, and no per-stage lines.

**Observed, 2026-08-05 — what the window shows while a render runs.** Run by the Creator on their own MacBook Pro (Apple M1 Pro, macOS 26.5.2), Blender 4.5.12, on a Cycles scene at 1280 by 720 with 1024 samples and a frame range of 1 to 6, reported as screenshots.

**The Status Bar.** The task is named `Render...`, followed by a bar with a whole-number percentage on it and an `✕` button at its right. Percentages seen across the shots: `0%`, `8%`, `5%`, `4%`.

**Whether the percentage covers a frame or the whole range.** It restarts at each frame. In the animation shots the bar read `5%` while the render window read `Frame:2 … Sample 56/1024`, and then `4%` while the render window read `Frame:3 … Sample 47/1024`. It did not climb across the six frames.

**The Blender Render window's own status line.** One line at the top left of that window. Quoted from the shots, word for word:

```
Frame:1 | Last:00:05.66 Time:00:05.31 | Mem:1408.12M, Peak:1408.12M | Scene, ViewLayer | 3 / 66 render kernels loaded (may take a few minutes the first time)
```

```
Frame:1 | Last:00:05.66 Time:00:08.99 | Remaining:00:23.21 | Mem:1457.36M, Peak:1457.36M | Scene, ViewLayer | Sample 83/1024 (6 kernels to optimize)
```

```
Frame:1 | Last:00:05.66 Time:00:12.91 | Remaining:00:15.48 | Mem:1457.37M, Peak:1457.37M | Scene, ViewLayer | Sample 285/1024 (2 kernels to optimize)
```

```
Frame:2 | Last:00:23.72 Time:00:01.24 | Remaining:00:17.52 | Mem:1457.36M, Peak:1457.36M | Scene, ViewLayer | Sample 56/1024 (Using optimized kernels)
```

```
Frame:3 | Last:00:23.72 Time:00:00.94 | Remaining:00:16.62 | Mem:1457.36M, Peak:1457.36M | Scene, ViewLayer | Sample 47/1024 (Using optimized kernels)
```

and, once the render had finished:

```
Frame:1 | Time:00:32.33 | Mem:1457.39M, Peak: 1457.39M
```

So the fields are: frame number; the previous frame's time as `Last:`; elapsed `Time:`; sometimes `Remaining:`; two memory figures; the scene and view layer; and then a sample count or a stage. The bracketed note after the sample count changed between shots through `3 / 66 render kernels loaded (may take a few minutes the first time)`, `(6 kernels to optimize)`, `(2 kernels to optimize)` and `(Using optimized kernels)`. That is what the pictures show.

**Observed, 2026-08-06 — the same runs on a second machine.**

Run on the rented Linux machine, Blender 4.5.12 LTS, `-b`, its own settings folder, display turned off first. Both machines were run and both agreed on everything below.

**The printed lines are the same shape.** A single-frame render ran to 49 lines on standard output and nothing on standard error — the same count as the Mac — and every line carried the same fields. The last six, word for word:

```
Fra:1 Mem:8.33M (Peak 9.12M) | Time:00:00.00 | Mem:0.09M, Peak:0.09M | Scene, ViewLayer | Sample 0/16
Fra:1 Mem:9.89M (Peak 9.89M) | Time:00:00.00 | Remaining:00:00.03 | Mem:1.48M, Peak:1.48M | Scene, ViewLayer | Sample 1/16
Fra:1 Mem:10.47M (Peak 11.06M) | Time:00:00.11 | Mem:1.48M, Peak:1.48M | Scene, ViewLayer | Sample 16/16
Fra:1 Mem:10.47M (Peak 11.06M) | Time:00:00.11 | Mem:1.48M, Peak:1.48M | Scene, ViewLayer | Finished
Saved: '/root/smoozy/Reports/work/L01/out_45/frame_0001.png'
Time: 00:00.20 (Saving: 00:00.08)
```

**The three-frame animation behaved the same.** 138 lines, three files, exit 0, the frame number in the running line the only marker, and no line naming the frame range — searching the whole run for `Rendering animation` and `Rendering frame` matched nothing, which is what the Mac found too. The `render_stats` handler fired 128 times and was handed the whole printed line, frame number included.

**The engine name — this settles a question the sources never answered.** The non-Cycles run above used `BLENDER_EEVEE_NEXT`. Asked to accept the other name, this version refused it:

```
ENGINE RAISED: TypeError bpy_struct: item.attr = val: enum "BLENDER_EEVEE" not found in ('BLENDER_EEVEE_NEXT', 'BLENDER_WORKBENCH', 'CYCLES')
```

So Blender 4.5 offers exactly three engine names and `BLENDER_EEVEE_NEXT` is the EEVEE one.

**Whether a program can read the progress as it happens.** A two-frame render was piped into a reader that stamped every line with the seconds since the run began, and those stamps were set beside the elapsed time Blender itself writes inside each line. The lines arrived spread across the run rather than in one lump at the end: the line Blender stamped `Time:00:06.00` reached the reader at 6.30 seconds, and the line stamped `Time:00:06.05` on the second frame reached it at 12.45 seconds. Worth knowing beside it: in a background render Cycles printed only three sample lines per frame — `Sample 0/2000`, `Sample 1/2000` and the last one — so there is little to read between them.

**Outside the interface.** Settled by the tests above, on both sides of the window. With no window, Blender 4.5 prints running progress lines to standard output, each carrying the frame number, memory, elapsed time, a sample count and the stage. With the window open the same fields appear in the Blender Render window's status line, and a percentage — which never appears in the printed output — appears on the Status Bar and restarts at each frame. The `render_stats` handler fires and receives each of those lines as a string. What it does not carry is any figure covering a whole animation — the count is always within one frame. The documented Status Bar readout and its percentage belong to Blender's window and no percentage appeared in the printed output.

**Operating system.** Nothing in the material names an operating system, and the Status Bar and API pages carry no per-system notes. The printed progress was got on **both** machines — the Creator's MacBook Pro (Apple M1 Pro, macOS 26.5.2) and the rented Linux machine — and the two agreed on the line shape, the fields and the counts, so it holds on macOS and on Linux. **Windows is untested and no Windows machine has run it.** **Untried rather than untestable: a Windows machine can be rented, and every one this project has read reported a Server edition — see the equipment section of `Preparation/Blender Research/Method.md`.** The Creator's window tests — the Status Bar task name, the percentage and the Blender Render window's status line — were got on macOS only.

---

## 4. What does Blender write out as it works, what is that stream called, and what does it contain?

**LABEL: documented for console output and its listed contents; observed that a Cycles switch's help text names stderr; inferred that its statistics actually use that stream.**

**The place it goes, and what the manual calls it.** *documented*

> "The Console Window, also called a Terminal, is an operating system text window that displays messages about Blender's operations, status, and internal errors.
>
> When Blender is manually started from a terminal, Blender output is shown in the Console Window it is started from."

Blender manual, https://docs.blender.org/manual/en/4.5/advanced/command_line/index.html, read 2026-07-30. Elsewhere the manual also calls it "the system console" (Cycles OSL page) and "System Console" (render output page). Those are the names the documentation uses. It never uses the words "stdout" or "stderr" — I searched the whole manual, API and release notes for both, and for "standard output": no hits.

**That there is a routine "status printing", named as such.** *documented*

> "-q, --quiet
>
> Suppress status printing (warnings & errors are still printed)."

Blender manual, https://docs.blender.org/manual/en/4.5/advanced/command_line/arguments.html, read 2026-07-30. That splits the output in two: status, which can be turned off, and warnings and errors, which cannot.

**What it contains, from the examples the sources give.** *documented*

Startup messages, quoted from the Troubleshooting page:

> "found bundled python: {DIR}"
>
> "Read prefs: {DIR}/userpref.blend"

https://docs.blender.org/manual/en/4.5/troubleshooting/startup.html, read 2026-07-30.

I opened the Windows console picture (`_images/advanced_command-line_launch_windows_console.png`, on https://docs.blender.org/manual/en/4.5/advanced/command_line/launch/windows.html). Visible in it, in order: `Read new prefs: C:\Users\Aaron\AppData\Roaming\Blender Foundation\Blender\2.77\config\userpref.blend`, `found bundled python: c:\blender\2.77\python`, `read blend: C:\tmp\Vincent.blend`, then a blank line and `Blender quit`. So: preference and file loading lines, and a closing line when Blender ends. The screenshot is plainly of an older Blender (the paths say 2.77) — the manual page uses it to illustrate the console, and I take from it only the shape of what appears there. *documented, from that picture.*

I also opened the Linux console picture (`_images/advanced_command-line_launch_linux_console.png`, on https://docs.blender.org/manual/en/4.5/advanced/command_line/launch/linux.html). Everything visible in it is audio-subsystem noise — `Cannot connect to server socket err = No such file or directory`, `jack server is not running or cannot be started`, `AL lib: (WW) alc_initconfig: Failed to initialize backend "jack"`. It shows that unrelated library warnings land in the same stream.

Encoding output, when rendering to video:

> "When you view the System Console, you can see some of the output of the encoding process. You will see even more output if you execute Blender as blender -d."

https://docs.blender.org/manual/en/4.5/render/output/properties/output.html, read 2026-07-30.

Python's own output:

> "You can see the output of print() as your script runs, which is useful to view debug info.
>
> The error traceback is printed in full to the terminal which won't always generate an report message in Blender's user interface (depending on how the script is executed)."

Python API, https://docs.blender.org/api/4.5/info_tips_and_tricks.html, read 2026-07-30.

Render statistics, as a thing that gets printed:

> "bpy.app.handlers.render_stats
>
> on printing render statistics. Accepts one argument: the render stats (render/saving time plus in background mode frame/used [peak] memory)."

Python API, https://docs.blender.org/api/4.5/bpy.app.handlers.html, read 2026-07-30. This is the only place in the sources that describes the *content* of a printed render line, and it says that content differs in background mode: render/saving time always, plus frame and used/peak memory when in background.

**The one place a stream is named says stderr in the switch's help text.** *observed for the help text; inferred for the actual stream*

Exact lines, `Blender Sources/cycles_4.5/engine.py`, lines 13–15, inside `_configure_argument_parser()`:

```
    parser.add_argument("--cycles-print-stats",
                        help="Print rendering statistics to stderr",
                        action='store_true')
```

The manual describes the same switch without naming a stream:

> "--cycles-print-stats
>
> Log statistics about render memory and time usage."

https://docs.blender.org/manual/en/4.5/advanced/command_line/arguments.html, read 2026-07-30. The file establishes only that the switch's help text names stderr. That the statistics actually arrive there is one step further and I did not run it — **inferred**.

**Turning it into a controllable log.** *documented*

> "--log <match> — Enable logging categories, taking a single comma separated argument… Use "*" to log everything."
>
> "--log-level <level> — Set the logging verbosity level (higher for more details) defaults to 1, use -1 to log all levels."
>
> "--log-show-timestamp — Show a timestamp for each log message in seconds since start."
>
> "--log-file <filepath> — Set a file to output the log to."

Plus the debug switches, of which the render-relevant ones are `--debug-cycles` ("Enable debug messages from Cycles"), `--debug-jobs` ("Enable time profiling for background jobs") and `-d, --debug`. All from https://docs.blender.org/manual/en/4.5/advanced/command_line/arguments.html, read 2026-07-30.

**Not covered.** The sources do not say which stream the ordinary status and error printing uses — only Cycles' statistics are pinned to stderr, and only by a help string. They do not give the line format of anything printed during a render (no example of a per-frame or per-sample line anywhere). They do not say whether `--log-file` captures the ordinary status printing as well as the log categories. They do not say whether output is line-buffered, which matters if you want to read it as it happens.

**Where I looked, for that silence.** Whole-tree searches of manual, API and release notes for "stdout", "stderr", "standard output" (only hit was an unrelated word inside a compositing page), and for "console window", "System Console", "terminal". Read in full: command-line Index, Arguments (all 628 lines), Rendering From The Command Line, Launching from the Command Line for Windows/macOS/Linux, Troubleshooting → Startup, the API Tips and Tricks page, `bpy.app.handlers`. Grepped all Cycles Python files for "print", "stats", "debug" — `engine.py` is the only one with anything, quoted above; the rest of the Cycles folder is shader and kernel headers with nothing about output.

**Observed, 2026-08-05 — which stream carries what, and what the switches do.**

Same computer and same way of running as in question 1. Blender 4.5.12 LTS, `-b`, own settings folder. Every run sent its two streams to separate files.

**Which stream.** On a plain successful single-frame render, standard output held all 49 lines and standard error held nothing — zero bytes. The same was true of the three-frame animation, the EEVEE render, and the render with a missing texture. Standard error was used in only two of every run tried: the message when saving failed, and the message when the Cycles device was refused (both quoted in question 2).

**Where the Cycles statistics land.** Run: `-b "simple.blend" -o "<folder>/frame_" -F PNG -f 1 -- --cycles-print-stats`. Standard error was zero bytes. The statistics appeared on **standard output**, between the `Finished` line and the `Saved:` line. They begin:

```
Render statistics:
Mesh statistics:
  Geometry:
    Total memory: 575 (575)
        Cube                             413 (413)
        Light                            81 (81)
        light                            81 (81)
Image statistics:
  Textures:
    Total memory: 0 (0)
Kernel statistics:
  Total render time               : Total 100.00% (0.12s), Self 15.00% (0.02s)
    Shade Surface                   : Total 43.33% (0.05s), Self 0.00% (0.00s)
```

and end:

```
Shader statistics:
  Material                        : 0.05s (Relative cost: 1.00)
Object statistics:
  Cube                            : 0.05s (Relative cost: 1.00)
```

The whole run was 125 lines on standard output and nothing on standard error. So the help text in `engine.py` naming stderr does not describe where the statistics actually appeared here.

**What quiet mode removes.** Run: `-b -q "simple.blend" -o "<folder>/frame_" -F PNG -f 1`. Exit status 0, the frame was written, and the whole of standard output was one line:

```
Blender 4.5.12 LTS (hash 84afd5f785f7 built 2026-07-21 01:41:35)
```

Standard error was empty. So `-q` removes the file-read line, every progress line, the `Saved:` line and the `Time:` line, leaving only the version banner.

**What the logging switches carry.** Run: `--log-list-categories`. It was not accepted. Exit status 1. Standard output:

```
Error: Cannot read file "/Users/artist/Desktop/Work/Smoozy v2/--log-list-categories": No such file or directory

Blender quit
```

Standard error:

```
unknown argument, loading as file: --log-list-categories
Error: argument has no '.blend' file extension, not using as new file, exiting! /Users/artist/Desktop/Work/Smoozy v2/--log-list-categories
```

Run: `-b "simple.blend" --log "*" --log-level info -o ... -f 1`. Standard error held:

```

Error: not a number '--log-level info'.
```

The render then went ahead anyway and ended with status 0. Re-run with the numeric form the manual gives, `--log "*" --log-level -1`, it was accepted and produced 431 lines on standard output. Every log line had this shape:

```
INFO (bke.appdir): source/blender/blenkernel/intern/appdir.cc:329 test_env_path: env 'BLENDER_USER_RESOURCES' found: /private/tmp/.../cfg_45
INFO (wm.files): source/blender/windowmanager/intern/wm_files.cc:962 file_read_reports_finalize: Blender file read in 0m0.00s
INFO (translation.messages): source/blender/blentranslation/intern/messages.cc:604 init: Locale en_RS used for translation
```

— a level, a category in brackets, the source file and line, the function, then the message. The complete set of categories that appeared, out of 260 log lines, was: `bke.appdir`, `bke.layercollection`, `bke.liboverride_resync`, `bke.sound`, `blo.readfile`, `bpy.interface`, `bpy.rna`, `translation.messages`, `wm.files`, `wm.msgbus.sub`. **No line carried a `render` or a `cycles` category** — searched for and matched nothing. The render progress lines quoted in question 3 were printed alongside the log without being part of it.

**What the built-in help lists.** Run: `--help`, 434 lines on standard output, nothing on standard error. Its whole Logging Options section:

```
Logging Options:
--log <match>
	Enable logging categories, taking a single comma separated argument.
	Multiple categories can be matched using a '.*' suffix,
	so '--log "wm.*"' logs every kind of window-manager message.
	Sub-string can be matched using a '*' prefix and suffix,
	so '--log "*undo*"' logs every kind of undo-related message.
	Use "^" prefix to ignore, so '--log "*,^wm.operator.*"' logs all except for 'wm.operators.*'
	Use "*" to log everything.

--log-level <level>
	Set the logging verbosity level (higher for more details) defaults to 1,
	use -1 to log all levels.

--log-show-basename 
	Only show file name in output (not the leading path).

--log-show-backtrace 
	Show a back trace for each log message (debug builds only).

--log-show-timestamp 
	Show a timestamp for each log message in seconds since start.

--log-file <filepath>
	Set a file to output the log to.
```

and the quiet entry, quoted whole:

```
-q or --quiet 
	Suppress status printing (warnings & errors are still printed).
```

**Observed, 2026-08-06 — the same switches on a second machine, and three things the Mac never asked.**

Run on the rented Linux machine, Blender 4.5.12 LTS, `-b`, its own settings folder, display turned off first. Both machines were run.

**Which stream, and where the statistics sit — agreed, and now placed.** Standard output carried everything and standard error was empty on every ordinary run, exactly as on the Mac. With `-- --cycles-print-stats` the statistics appeared on **standard output** again, 125 lines in all, so the help text in `engine.py` naming stderr does not describe where they arrive on either machine. **Where they sit is now settled for this version:** the block opens with `Render statistics:` immediately after the `Finished` line and closes with the `Object statistics:` section, and only then come the `Saved:` and `Time:` lines.

**Quiet mode — agreed.** `-b -q` left the whole of standard output at one line, the version banner, and the frame was still written. Same as the Mac.

**The logging switches — agreed.** `--log-list-categories` is still not a switch here: run with `-b`, exit status 1, and standard output read

```
Blender 4.5.12 LTS (hash 84afd5f785f7 built 2026-07-21 01:31:22)
Error: Cannot read file "/root/--log-list-categories": No such file or directory

Blender quit
```

with `unknown argument, loading as file: --log-list-categories` on standard error. `--log-level info` was refused with `Error: not a number '--log-level info'.` on standard error and the render then went ahead and ended at 0. The numeric form `--log "*" --log-level -1` was accepted and produced 383 lines, and the complete set of categories that appeared was the same ten the Mac saw — `bke.appdir`, `bke.layercollection`, `bke.liboverride_resync`, `bke.sound`, `blo.readfile`, `bpy.interface`, `bpy.rna`, `translation.messages`, `wm.files`, `wm.msgbus.sub`. **No `render` and no `cycles` category**, on either machine.

**What `--log-file` catches — new, and nobody had run it.** Run as `-b simple.blend --log-file "<path>" -o … -f 1`, with no `--log` category named, Blender 4.5 **created the file and left it empty**, and all 49 lines still went to standard output. The same on the no-camera file: the file stayed empty and the failure was printed to standard output as usual. (Blender 5.0, 5.1 and 5.2 do the opposite — see those versions' findings.)

**Where `--debug-cycles` writes — new, and it is the one place this version uses standard error for something ordinary.** Run as `-b simple.blend --debug-cycles -o … -f 1`, the debug material went to **standard error**, 6832 bytes of it, in a shape unlike anything else Blender prints — a level letter, a date and time, a process number, a source file and line, then the message. Three lines from it, word for word:

```
I0806 04:33:57.795423 32499 device.cpp:38] CUEW initialization succeeded
I0806 04:33:57.870397 32499 device.cpp:212] Added device "NVIDIA GeForce RTX 4090" with id "CUDA_NVIDIA GeForce RTX 4090_0000:00:07".
I0806 04:33:57.875618 32499 device.cpp:554] Mapped host memory limit set to 80,647,581,696 bytes. (75.11G)
```

It ends with a block headed `Full path tracing report`, naming the devices used and giving a table of times. Nothing of it appeared on standard output.

**The built-in help — agreed on content, longer on this machine.** `--help` gave 447 lines on Linux against 434 on the Mac, and the logging entries it lists are the same six: `--log`, `--log-level`, `--log-show-basename`, `--log-show-backtrace`, `--log-show-timestamp`, `--log-file`. The quiet entry reads word for word the same on both machines:

```
-q or --quiet 
	Suppress status printing (warnings & errors are still printed).
```

**Outside the interface.** This whole answer is about the outside-the-interface case, and the tests above were all run that way. Everything Blender wrote went to standard output, apart from the two messages named in question 2. The one difference the sources name in the other direction is the `render_stats` content: "render/saving time plus **in background mode** frame/used [peak] memory". Also `-con, --start-console` is documented as "ignored if -b is set", and `-d, --debug` "Keeps Python's sys.stdin rather than setting it to None".

**Operating system.** It differs. On Windows the console is a separate Command Prompt window created at start and then toggled off, brought back with "Window ‣ Toggle System Console" or `-con, --start-console` ("(Windows only)"), and "Closing the Console Window will also close Blender, losing any unsaved work." On macOS you must run the executable inside the `.app` bundle — "In order to run Blender you will have to specify that path to the Blender executable inside this folder, to get all output printed to the terminal." On Linux you run `./blender` from a terminal, or use a desktop "Run in terminal" option. All from the three Launching pages and the command-line Index, read 2026-07-30. `--debug-ghost` is marked "(Linux only)". `--debug-wintab` reads only "Enable debug messages for Wintab" — the page names no operating system for it.

**Both machines have now been run and they agree on which stream carries what, on quiet mode, on the logging switches and on the categories.** Two small differences belong to the machine rather than to Blender: the built-in help is 447 lines on Linux and 434 on the Mac, and a windowless Blender on a Linux machine with a display running may have display-library complaints appear on its standard error — not Blender's words, and absent once the display is turned off. **Windows is untested and no Windows machine has run it**, so the separate Command Prompt window and `-con, --start-console` stay unchecked. (Citation check 2026-08-05: this used to call `--debug-wintab` Windows-specific tablet debugging, which its entry does not say.)

---

## 5. Does Blender estimate the time left, for what unit of work, and how does it describe that estimate?

**LABEL: documented that it does; not in the sources for how it words it.**

Yes, in one place, and the sentence is one line long:

> "Running Task
>
> Shows the progress of the currently running task (such as rendering or baking). Hovering the mouse pointer over the progress bar will display a time estimate."

Blender manual, https://docs.blender.org/manual/en/4.5/interface/window_system/status_bar.html, read 2026-07-30.

**For what unit of work.** For "the currently running task", which the same sentence exemplifies as "rendering or baking". The source does not say whether a render task means one frame, a whole animation, or another unit. The estimate is obtained by hovering the pointer over the bar.

**How it describes the estimate.** The source calls it "a time estimate" and stops there. It does not give the wording, the format, whether it counts down time remaining or forward to a finish time, or the units.

**Not covered.** Nothing about an estimate during an animation render — whether it covers the current frame or the whole frame range. Nothing about an estimate anywhere except that hover tooltip: not printed, not in the API, not in background mode. Nothing about how it is calculated or how it behaves with adaptive sampling, where the sample count is not known in advance.

**What the sources give instead, so it is not mistaken for an estimate.** Two things, both about time already spent or time allowed, neither a prediction: `use_stamp_render_time`, "Include the render time in image metadata" (https://docs.blender.org/api/4.5/bpy.types.RenderSettings.html), matching the manual's Metadata entry "Render Time — Includes the render time"; and Cycles' Time Limit, "Stops rendering if the time exceeds the limit, even if the desired sample count hasn't been reached yet" (https://docs.blender.org/manual/en/4.5/render/cycles/render_settings/sampling.html). Both read 2026-07-30.

**Where I looked, for the silence.** Whole-tree searches of the manual and API for "estimate" (every hit reviewed — the only relevant one is the Status Bar sentence above; the rest are about fluid, rigging, tone mapping, motion tracking and inverse kinematics), and for "remaining", "time remaining", "frames remaining", "ETA", "elapsed". "Remaining" appears once in the whole render section, in a stereoscopy page about compositing order, and nowhere in the API in a time sense. Read: Status Bar page, Preferences → Interface (Status Bar section), the whole `bpy.types.RenderEngine` page, `bpy.types.WindowManager` progress functions, Cycles Sampling page, Render Output → Metadata, the command-line Arguments page.

**Observed, 2026-08-05 — Blender does print an estimate with no window.**

Same computer and same way of running as in question 1. Blender 4.5.12 LTS, `-b`, own settings folder, Cycles.

Some of the running progress lines carry a field labelled `Remaining:`. It sits between the elapsed `Time:` field and the memory pair. From a single-frame render at 16 samples:

```
Fra:1 Mem:9.77M (Peak 9.77M) | Time:00:00.02 | Remaining:00:00.05 | Mem:1.48M, Peak:1.48M | Scene, ViewLayer | Sample 1/16
```

From a slower graphics-card render of the same file, where it appeared on several lines running and fell each time:

```
Fra:1 Mem:9.00M (Peak 24.22M) | Time:00:00.34 | Remaining:00:02.97 | Mem:0.09M, Peak:0.09M | Scene, ViewLayer | Sample 1/16
Fra:1 Mem:9.66M (Peak 24.22M) | Time:00:00.34 | Remaining:00:01.40 | Mem:0.09M, Peak:1.48M | Scene, ViewLayer | Sample 2/16
Fra:1 Mem:9.66M (Peak 24.22M) | Time:00:00.34 | Remaining:00:00.61 | Mem:0.09M, Peak:1.48M | Scene, ViewLayer | Sample 4/16
Fra:1 Mem:9.67M (Peak 24.22M) | Time:00:00.35 | Remaining:00:00.20 | Mem:0.09M, Peak:1.48M | Scene, ViewLayer | Sample 8/16
```

The word is `Remaining:`, with no space after the colon, and the figure is in minutes, seconds and hundredths. It does not appear on every line: on the runs above it was absent from the `Sample 0/16` line, from the final `Sample 16/16` line and from the `Finished` line.

**What it covers.** In the three-frame animation, a `Remaining:` field appeared inside each frame in turn — once under `Fra:1`, once under `Fra:2`, once under `Fra:3` — and the value started over each time. Here are the three, from that one run:

```
Fra:1 Mem:9.77M (Peak 9.77M) | Time:00:00.00 | Remaining:00:00.01 | Mem:1.48M, Peak:1.48M | Scene, ViewLayer | Sample 1/16
Fra:2 Mem:9.77M (Peak 10.95M) | Time:00:00.00 | Remaining:00:00.01 | Mem:1.48M, Peak:1.48M | Scene, ViewLayer | Sample 1/16
Fra:3 Mem:9.77M (Peak 10.95M) | Time:00:00.00 | Remaining:00:00.01 | Mem:1.48M, Peak:1.48M | Scene, ViewLayer | Sample 1/16
```

No line in the run gave a remaining figure for the animation as a whole. The EEVEE run printed no `Remaining:` field at all.

**Observed, 2026-08-05 — what the tooltip on the progress bar says.** Run by the Creator on their own MacBook Pro (Apple M1 Pro, macOS 26.5.2), Blender 4.5.12, on a Cycles scene at 1280 by 720 with 1024 samples and a frame range of 1 to 6, reported as screenshots.

Resting the pointer on the Status Bar progress bar brings up a box of **two lines**. Word for word, from a single-frame render:

```
Time Remaining: 00:33.73
Time Elapsed: 00:13.01.
```

and from an animation render:

```
Time Remaining: 01:15.95
Time Elapsed: 00:55.20.
```

So the wording is `Time Remaining: ` and `Time Elapsed: `, the figures are in minutes, seconds and hundredths, and the second line ends with a full stop. It gives both a forecast and a time already spent.

**What each figure sat beside.** In the animation shot, at the same moment the tooltip read `Time Remaining: 01:15.95` and `Time Elapsed: 00:55.20.`, the Blender Render window's own status line read `Frame:3 | Last:00:23.72 Time:00:07.71 | Remaining:00:10.41`. So the two `Remaining` figures on screen at that instant were 01:15.95 and 00:10.41, and the two elapsed figures were 00:55.20 and 00:07.71. In the single-frame shot the tooltip read `Time Elapsed: 00:13.01.` while the status line read `Time:00:12.91`. Those are the numbers as they appeared; nothing further is read into them here.

**Observed, 2026-08-06 — the printed estimate on a second machine.**

Run on the rented Linux machine, Blender 4.5.12 LTS, `-b`, its own settings folder, display turned off first. It agreed with the Mac in every respect. The field is spelled `Remaining:` with no space after the colon, in minutes, seconds and hundredths, and it appears on some lines and not others. From a single-frame render at 16 samples:

```
Fra:1 Mem:9.89M (Peak 9.89M) | Time:00:00.00 | Remaining:00:00.03 | Mem:1.48M, Peak:1.48M | Scene, ViewLayer | Sample 1/16
```

In the three-frame animation it appeared once inside each frame and started over each time, and the `render_stats` handler was handed each line with its frame number attached, so which frame each belongs to is visible:

```
RENDER_STATS: ('Fra:1 Mem:9.89M (Peak 9.89M) | Time:00:00.00 | Remaining:00:00.02 | Mem:1.48M, Peak:1.48M | Scene, ViewLayer | Sample 1/16', None)
RENDER_STATS: ('Fra:2 Mem:9.89M (Peak 11.06M) | Time:00:00.00 | Remaining:00:00.03 | Mem:1.48M, Peak:1.48M | Scene, ViewLayer | Sample 1/16', None)
RENDER_STATS: ('Fra:3 Mem:9.89M (Peak 11.06M) | Time:00:00.00 | Remaining:00:00.01 | Mem:1.48M, Peak:1.48M | Scene, ViewLayer | Sample 1/16', None)
```

No line in the run gave a remaining figure for the animation as a whole, and the EEVEE run printed no `Remaining:` field at all. Both machines were run and both agreed.

**Outside the interface.** Settled on both sides. An estimate is printed with no window: the `Remaining:` field on Cycles' progress lines, which restarts at each frame and so covers the current frame. With the window open there is also the two-line tooltip above, which is a different readout with different wording and, during an animation, a different figure at the same instant.

**How it describes the estimate — now answered.** The sources said only "a time estimate". Observed, it is worded `Time Remaining:` with a companion `Time Elapsed:` line, in minutes, seconds and hundredths, counting down rather than towards a finishing clock time.

**Operating system.** Nothing in the sentence or the page names an operating system, and I found no per-system note about it anywhere. The printed `Remaining:` field was got on **both** machines — the Creator's MacBook Pro (Apple M1 Pro, macOS 26.5.2) and the rented Linux machine — and they agreed on its spelling, its units, where it appears and what it covers, so it holds on macOS and on Linux. **Windows is untested and no Windows machine has run it.** The two-line tooltip was seen on macOS only.

---

## 6. What does Blender report about the memory it is using on a graphics card?

**LABEL: documented that a GPU video-memory display setting exists; inferred that it is a Status Bar item and is hidden by default; not in the sources for what it shows.**

**The one graphics-card memory readout named anywhere in the sources.** *documented*

> "show_statusbar_vram
>
> Show GPU video memory usage
>
> Type:
>
> boolean, default False"

Python API, https://docs.blender.org/api/4.5/bpy.types.PreferencesView.html, read 2026-07-30.

It sits in a run of Status Bar switches on the same page: `show_statusbar_memory` ("Show Blender memory usage", default False), `show_statusbar_scene_duration`, `show_statusbar_stats`, `show_statusbar_version` (default True), `show_statusbar_vram` (default False). So the graphics-card figure is a Status Bar item that is off unless you turn it on. That it appears in the Status Bar is **inferred** from the property's name and its company on that page — the API text itself says only "Show GPU video memory usage".

**The manual does not document it.** The manual's Status Bar page and the matching Preferences → Interface → Status Bar section both list what can be shown, and the list is: Scene Statistics, Scene Duration, System Memory, Extensions Updates, Blender Version. No video-memory entry in either. And "System Memory" is explicitly *not* the graphics card:

> "System Memory
>
> Shows an estimate of Blender's RAM consumption. On a single-instance single-machine scenario, this estimate provides a measurement against the hardware limit of the machine."

https://docs.blender.org/manual/en/4.5/interface/window_system/status_bar.html and https://docs.blender.org/manual/en/4.5/editors/preferences/interface.html, read 2026-07-30.

I opened the Status Bar right-hand picture (`_images/interface_window-system_status-bar_ui-right.png`, on the Status Bar page). It reads: `Collection | Cube | Verts:8 | Faces:6 | Tris:12 | Objects:0/3 | Mem: 29.7 MiB | v2.81.16`. So the one memory figure visible is labelled "Mem:" and given in MiB — and by the manual's own text that item is System Memory, RAM, not the graphics card. The picture also carries the version string v2.81.16, so it is an old screenshot kept in the 4.5 manual; I take from it only the label and unit shape of the RAM item, and it tells me nothing about the VRAM item, which is not in it. *documented, from that picture.*

**What a render engine hands Blender, which is not marked as graphics-card memory.** *documented*

> "update_memory_stats(*, memory_used=0.0, memory_peak=0.0)
>
> Update memory usage statistics
>
> Parameters:
>
> memory_used (float in [0, inf], (optional)) – Current memory usage in megabytes
>
> memory_peak (float in [0, inf], (optional)) – Peak memory usage in megabytes"

Python API, https://docs.blender.org/api/4.5/bpy.types.RenderEngine.html, read 2026-07-30. Two numbers, in megabytes, current and peak. Nothing on the page says whether a GPU render reports the card's memory here or the system's.

**Memory figures that do get written down, none of them marked graphics-card.** *documented*

- Printed statistics: "bpy.app.handlers.render_stats — on printing render statistics. Accepts one argument: the render stats (render/saving time plus in background mode frame/used [peak] memory)." (https://docs.blender.org/api/4.5/bpy.app.handlers.html)
- Command line: "--cycles-print-stats — Log statistics about render memory and time usage." (https://docs.blender.org/manual/en/4.5/advanced/command_line/arguments.html); and *observed*, `Blender Sources/cycles_4.5/engine.py` line 14, the same switch's help reads `"Print rendering statistics to stderr"`.
- In the saved image: "Memory — Includes the peak memory usage." (https://docs.blender.org/manual/en/4.5/render/output/properties/metadata.html), matching `use_stamp_memory`, "Include the peak memory usage in image metadata", default True (https://docs.blender.org/api/4.5/bpy.types.RenderSettings.html).

All read 2026-07-30. In none of these does the source say the figure is the graphics card's memory.

**What the sources do say about graphics-card memory otherwise — all of it about behaviour, not reporting.** *documented*

> "Typically, the GPU can only use the amount of memory that is on the GPU… With CUDA, OptiX, HIP and Metal devices, if the GPU memory is full Blender will automatically try to use system memory."
>
> "Would multiple GPUs increase available memory? Typically, no, each GPU can only access its own memory. The exception is NVIDIA GPUs connected with NVLink, where multiple GPUs can share memory at a small performance cost. This can be enabled with Distributed Memory Across Devices in the preferences."
>
> "Error: Out of memory — This usually means there is not enough memory to store the scene for use by the GPU."

https://docs.blender.org/manual/en/4.5/render/cycles/gpu_rendering.html, read 2026-07-30. And for EEVEE, flatly:

> "There is no standard way of estimating if the resources will fit into the GPU memory and/or if the GPU will render them successfully."

https://docs.blender.org/manual/en/4.5/render/eevee/limitations/limitations.html, read 2026-07-30.

**Not covered.** This does not settle what `show_statusbar_vram` actually displays: no unit, no format, no statement of whether it is Blender's own usage or the whole card's, whether it is a used figure or a used-of-total figure, whether it names the device, or what it shows when several cards are in use. It does not settle whether that figure is reachable outside the Status Bar — I found no property, operator or printed line that exposes a graphics-card memory number. And it does not settle whether the memory figures Cycles prints via `--cycles-print-stats` or stamps into metadata are card memory, system memory, or both.

**Where I looked, for that silence.** Whole-tree searches of manual, API and release notes for: "VRAM" (three hits: two EEVEE quality settings, and `show_statusbar_vram`), "GPU memory", "video memory", "device memory", "peak memory", "memory usage". Searched the release notes for "vram" and "memory" — nothing about a memory readout being added or changed in 4.5; the Status Bar entries in the 4.5 user-interface notes are about spacing, text entry, jiggling and keymap glyphs only (https://developer.blender.org/docs/release_notes/4.5/user_interface/). Read: the Status Bar manual page and its three pictures, Preferences → Interface, the full `bpy.types.PreferencesView` Status Bar block, the full `bpy.types.RenderEngine` page, `bpy.app.handlers`, Cycles GPU Rendering, EEVEE Limitations, Render Output → Metadata, `bpy.types.RenderSettings` stamp properties, the command-line Arguments page, and all Cycles Python files (grepped for memory-related words; nothing).

**Observed, 2026-08-05 — the setting exists, and nothing printed is marked as the card's memory.**

Same computer and same way of running as in question 1. Blender 4.5.12 LTS, `-b`, own settings folder.

**The setting exists and is off.** A script read the preference and printed:

```
preferences.view.show_statusbar_vram => False
preferences.view.show_statusbar_memory => False
```

so both status-bar memory switches are present in 4.5 and both start off. What the readout then shows needs the window and is entry 21 of `Audits/Blender Research/Reports/Hand tests.md`.

**A render on the graphics card.** A script turned the Metal device on and set the scene to render on it. It printed:

```
DEVICE: 'Apple M1 Pro' 'CPU' use = True
DEVICE: 'Apple M1 Pro (GPU - 14 cores)' 'METAL' use = True
SCENE DEVICE: GPU
```

The render then ran on that device and the progress lines were the same shape as a processor render, with the same two memory fields and no third:

```
Fra:1 Mem:9.66M (Peak 24.22M) | Time:00:00.34 | Remaining:00:01.40 | Mem:0.09M, Peak:1.48M | Scene, ViewLayer | Sample 2/16
Fra:1 Mem:11.63M (Peak 24.22M) | Time:00:00.43 | Mem:1.48M, Peak:1.48M | Scene, ViewLayer | Finished
```

No field, label or word anywhere in the run named the graphics card's memory. Searching the whole output for "VRAM", "video memory", "GPU memory" and "device memory" matched nothing.

**With the Cycles statistics switch added.** The same graphics-card render with `-- --cycles-print-stats` produced the statistics block quoted in question 4. Its memory figures are:

```
Mesh statistics:
  Geometry:
    Total memory: 575 (575)
Image statistics:
  Textures:
    Total memory: 0 (0)
```

Those are labelled by what the memory holds — geometry, textures — not by which device it is on. Nothing in the block names a card, a device or an amount of card memory.

**Observed, 2026-08-05 — the Status Bar offers no video-memory item.** Run by the Creator on their own MacBook Pro (Apple M1 Pro, macOS 26.5.2), Blender 4.5.12, reported as a screenshot.

Right-clicking the Status Bar opens a menu headed `Status Bar` holding **exactly five items**, each with a tick box:

```
Scene Statistics
Scene Duration
System Memory
Extensions Updates
Blender Version
```

**No item mentions video memory or GPU memory.** That is the whole menu; nothing is cut off in the picture.

The Status Bar itself, in that shot, read:

```
…llection | Cube | Verts:8 | Faces:6 | Tris:12 | Objects:1/3 | Memory: 300.0 MiB | 4.5.12
```

So the one memory figure on offer there is labelled `Memory:` and given in MiB to one decimal place — and by the manual's own text that item is System Memory, not the graphics card. (The manual's own old screenshot of this bar shows the label as `Mem:`; what this version draws is `Memory:`.)

**A gap in what was reported, since filled on another machine.** The instruction also said that, if the right-click menu had no such item, to look in Edit ▸ Preferences ▸ Interface ▸ Status Bar instead. No screenshot of that panel was returned from the Mac, and no Mac screenshot shows the readout switched on. The next block answers what it displays, on a different machine and a different card.

**Observed, 2026-08-06 — what the video-memory readout actually says.**

Run on the rented Linux machine, Blender 4.5.12 LTS, with its window open on that machine's own display, pointed at its own settings folder. A script set `bpy.context.preferences.view.show_statusbar_vram = True` and `show_statusbar_memory = True`, saved the preferences with `bpy.ops.wm.save_userpref()`, waited five seconds, photographed the window with `bpy.ops.screen.screenshot` and quit. It printed:

```
BEFORE vram = False  memory = False
AFTER  vram = True  memory = True
```

**The readout appeared, and this is what it says.** The right-hand end of the Status Bar in that picture reads, word for word:

```
Collection | Cube | Verts:8 | Faces:6 | Tris:12 | Objects:1/3 | Memory: 15.3 MiB | VRAM: 1.0/24.0 GiB | 4.5.12
```

*(Narrowed 2026-08-06 by the runner, after the check of the Linux hand tests sent this back. The check read the same bar again on a quiet card and got 1.2, 1.1, 1.1 and 1.2 in the four versions, and 1.7 and 1.8 on a busier one — so the left-hand figure is not fixed and must not be quoted as though it were. What the runs do carry, identically every time, is the label `VRAM:`, the used-of-total shape, the `GiB` units, and the right-hand figure of `24.0`, which is the whole of this card's memory. The left-hand figure moves with what is on the card, and how it is counted is still not settled — both the check's readings and this document's are on the record.)*


So the item is labelled **`VRAM:`**, and it is a **used-of-total** figure in **GiB**, to one decimal place, with a `/` between the two numbers. It sits between the system-memory item and the version. The right-hand figure, `24.0 GiB`, is this card's whole memory — `nvidia-smi` reports the card as 24564 MiB.

**Whether the left-hand figure is Blender's own use or the card's — read in 4.5 and answered in neither direction.** At the same moment as the picture above, `nvidia-smi` reported 3432 MiB used of 24564 MiB on the card, and Blender read `VRAM: 1.0/24.0 GiB`. The two do not match. The moving test was run in Blender 5.2 rather than here — a heavy Cycles render on the card, photographed five times, with `nvidia-smi` read at each shot — and there the readout moved with the card, reading `VRAM: 4.0/24.0 GiB` against 3673 MiB and `VRAM: 3.8/24.0 GiB` against 3459 MiB, and at rest `VRAM: 1.0/24.0 GiB` against 568 MiB. **What the left-hand figure counts is therefore not settled** — it tracks the card's overall use but is neither equal to it nor plainly Blender's own share. Nothing further is read into it here. The machine was shared with other work, so other processes may have held some of the card's memory at any of those moments.

**Still not reached, on either machine.** Whether the Status Bar's own right-click menu offers this item. On the Mac the menu was photographed and holds exactly five items, none of them video memory. On the Linux machine the menu could not be opened at all: there is no registered menu class to call by name — a sweep of `bpy.types` for anything with `STATUSBAR` in it returns only `STATUSBAR_HT_header`, two theme entries and the preferences panel — and there is no tool on that machine to send a right-click into the window. The preference can be switched on from Python, which is what the block above does.

**Observed, 2026-08-06 — what a render on an NVIDIA card prints about memory.**

Run on the rented Linux machine, Blender 4.5.12 LTS, `-b`, display turned off first. A script set the Cycles device type to CUDA and turned the devices on. It printed:

```
DEVICE: 'NVIDIA GeForce RTX 4090' 'CUDA' use = True
DEVICE: 'AMD Ryzen Threadripper PRO 3975WX 32-Cores' 'CPU' use = True
DEVICE: 'NVIDIA GeForce RTX 4090' 'OPTIX' use = True
SCENE DEVICE: GPU
```

**The ordinary progress lines named no card memory**, exactly as on the Mac with Metal. The two `Mem:` fields were the same shape as on a processor render and no third appeared.

**`--debug-cycles` is different, and this is the closest Blender comes to reporting card memory.** Added to the same run, it wrote to standard error in this version (see question 4) and carried these lines, word for word:

```
I0806 04:33:57.875618 32499 device.cpp:554] Mapped host memory limit set to 80,647,581,696 bytes. (75.11G)
I0806 04:33:58.084632 32547 device_impl.cpp:505] Local memory reserved 1,245,708,288 bytes. (1.16G)
```

and a block headed:

```
I0806 04:33:58.088141 32547 scene.cpp:391] System memory statistics after full device sync:
  Usage: 102,540 (100.14K)
  Peak: 28,490,634 (27.17M)
```

and, further down, the devices named outright:

```
Path tracing on: NVIDIA GeForce RTX 4090 (CUDA) [CUDA_NVIDIA GeForce RTX 4090_0000:00:07]
                 AMD Ryzen Threadripper PRO 3975WX 32-Cores (CPU) (11 threads) [CPU]

Denoising on: AMD Ryzen Threadripper PRO 3975WX 32-Cores (CPU) (12 threads) [CPU]
```

The one figure that is plainly labelled is **system** memory. `Mapped host memory limit` is the figure this build set for memory outside the card; it read the same, `80,647,581,696 bytes. (75.11G)`, in all four versions on this machine. **No line anywhere states how much of the card's memory is in use.**

**One more thing the Mac's run never saw.** On a render using the card, the `--cycles-print-stats` block loses its kernel, shader and object sections and prints one line in their place:

```
Profiling information not available (only works with CPU rendering)
```

Its memory figures — `Total memory: 575 (575)` under `Mesh statistics`, `Total memory: 0 (0)` under `Image statistics` — are the same as on a processor render and are still labelled by what the memory holds, not by which device it is on.

**Observed, 2026-08-06 — what the memory readouts say when there are two graphics cards.**

Run over SSH on a **second rented Linux machine — Ubuntu 22.04.5 LTS, two NVIDIA GeForce RTX 4090s of 24 GB each on driver 580.95.05, an AMD EPYC 7B13 with sixty-one cores, 98 GB of memory, no display server at all** — using Blender at `/opt/blender/blender-4.5.12-linux-x64/blender`, `Blender 4.5.12 LTS (hash 84afd5f785f7 built 2026-07-21 01:31:22)`, with `BLENDER_USER_RESOURCES` pointed at `/root/smoozy/two cards/res-4.5`, a folder of its own that was empty at the start. **Nothing else was running on the machine.** Label: **observed**. This is a third machine; nothing here is merged with the Creator's Mac or with the one-card rented Linux machine. The full list is `Audits/Blender Research/Reports/Hand tests — two cards and no screen.md`.

**The two cards are not joined by NVLink.** `nvidia-smi topo -m` reports `PHB` between them — `Connection traversing PCIe as well as a PCIe Host Bridge (typically the CPU)`. So the NVLink question this topic also owes is still out of reach.

**Blender reports one figure, never two, and never names a card beside a memory number.** The same Cycles scene at 1920 by 1080 and 6000 samples, rendered with both cards ticked and then with one, gave these `Finished` lines:

```
two cards: Fra:1 Mem:111.48M (Peak 174.77M) | Time:00:25.89 | Mem:6.98M, Peak:6.98M | Scene, ViewLayer | Finished
one card:  Fra:1 Mem:99.41M (Peak 162.69M) | Time:00:49.05 | Mem:1064.28M, Peak:1064.28M | Scene, ViewLayer | Finished
```

and a heavier scene — a 3000 by 3000 subdivided grid, 868 MB on disk — gave, in 4.5, `Mem:480.88M, Peak:494.94M` with two cards against `Mem:1925.82M, Peak:1925.82M` with one. **The second `Mem:` field is smaller with two cards than with one**, not larger and not doubled, and no second figure ever appears. What that field counts is not settled by this test and nothing is read into it.

**Each card holds the same amount, and about as much as one card holds on its own.** `nvidia-smi --query-gpu=index,memory.used --format=csv,noheader` sampled five times a second through the heavier scene's render, highest figure each card reached:

```
two cards, Distribute Memory across devices OFF:   card 0: 3557 MiB   card 1: 3557 MiB
two cards, Distribute Memory across devices ON:    card 0: 3557 MiB   card 1: 3557 MiB
one card:                                          card 0: 3565 MiB   card 1: 3 MiB
```

Both cards hold the same amount, and turning the distribute-memory setting on made no difference on a pair that is not joined by NVLink. Reading those equal figures as the scene being duplicated on each card rather than divided between them is a step past the measurement — **inferred**, not observed. *(Labelled by the two-card citation check, 2026-08-06: the block said the scene lands on both cards "in full", which the figures do not state on their own.)* During the lighter scene both cards held the same 2683 MiB throughout. **Blender said nothing at all about that setting**: searching both streams of every run for `peer`, `nvlink`, `interconnect` and `distribut` matched nothing but the output file's own name.

**`--debug-cycles` gives one memory block and says outright that it is system memory:**

```
scene.cpp:391] System memory statistics after full device sync:
  Usage: 15,842,126 (15.11M)
  Peak: 37,005,440 (35.29M)
```

with `Mapped host memory limit set to 101,923,811,328 bytes. (94.92G)` printed once per card, and `Local memory reserved 1,245,708,288 bytes. (1.16G)` likewise once per card. **No line anywhere states how much of either card's memory is in use.**

The same run does name both cards outright, but no memory figure sits beside either:

```
Path tracing on: NVIDIA GeForce RTX 4090 (OptiX) [CUDA_NVIDIA GeForce RTX 4090_0000:00:0a_OptiX]
                 NVIDIA GeForce RTX 4090 (OptiX) [CUDA_NVIDIA GeForce RTX 4090_0000:00:07_OptiX]
```

*(Added by the two-card citation check, 2026-08-06: this block is in the hand-test record, it is what carries the claim that a card is never named beside a memory number, and it had not been written in.)*

**`-- --cycles-print-stats` with two cards has the same shape as with one and names no device:**

```
Render statistics:
Mesh statistics:
  Geometry:
    Total memory: 6.15M (6,448,189)
        Mesh                             6.01M (6,303,089)
        Sphere                           23.55K (24,113)
        Sphere.001                       23.55K (24,113)
        Sphere.002                       23.55K (24,113)
        Sphere.003                       23.55K (24,113)
        Sphere.004                       23.55K (24,113)
        light                            81 (81)
Image statistics:
  Textures:
    Total memory: 0 (0)
Profiling information not available (only works with CPU rendering)
```

The memory figures are still labelled by what the memory holds — geometry, textures — not by which card it is on, and the profiling line is the same one a single-card render gives.

**Observed, 2026-08-06 — what the memory readouts say with four graphics cards, and what happens when a card is filled.**

Run over SSH on a **third rented Linux machine — Ubuntu 22.04.5 LTS, four NVIDIA GeForce RTX 4060 Ti of 16 GB each on driver 580.95.05, an AMD EPYC 7R43 with sixty-one cores, 197 GB of memory, no display server** — using Blender at `/opt/blender/blender-4.5.12-linux-x64/blender`, `Blender 4.5.12 LTS (hash 84afd5f785f7 built 2026-07-21 01:31:22)`, with `BLENDER_USER_RESOURCES` pointed at `/root/smoozy/four cards/res-4.5`, a folder of its own emptied and remade before use. **Nothing else was running on the machine.** Label: **observed**. This is a **fourth** machine; nothing here is merged with the Creator's Mac, the one-card rented Linux machine or the two-card rented Linux machine. The full list, with every command and every script written out, is `Audits/Blender Research/Reports/Hand tests — four cards.md`.

**Three things separate this machine from the two-card one, and all three bear on what is below.** Four cards rather than two. A **smaller** card — 16 GB against 24 GB, the smallest the project has had, so no figure here may be set beside a two-card figure and none is. And **197 GB of system memory** against 98 GB, which is what finally made it possible to fill a card while the machine's own memory was nowhere near full — the exact case this question asks about, and one no earlier machine could produce.

**The four cards are not joined by NVLink** — `nvidia-smi topo -m` reports `PHB` between every pair — so the NVLink question this topic also owes is still out of reach.

**Blender reports one figure, never four, and never names a card beside a memory number.** The same Cycles scene at 1920 by 1080 and 3000 samples, rendered with four cards ticked, with two and with one, gave these `Finished` lines:

```
four cards: Fra:1 Mem:103.60M (Peak 166.89M) | Time:00:22.74 | Mem:6.98M, Peak:38.62M | Scene, ViewLayer | Finished
two cards:  Fra:1 Mem:95.44M (Peak 158.72M) | Time:00:38.39 | Mem:6.98M, Peak:38.62M | Scene, ViewLayer | Finished
one card:   Fra:1 Mem:91.37M (Peak 154.65M) | Time:01:07.78 | Mem:384.23M, Peak:384.23M | Scene, ViewLayer | Finished
```

**The figure is far smaller with more than one card, and it does not go on shrinking.** Two cards and four cards printed exactly the same figure — the second `Mem:` field, `6.98M` against `384.23M` between four cards and one — and the same held on a heavier scene, a grid of 3000 by 3000 subdivisions at 909 MB on disk, where four cards and two cards both printed `Mem:480.88M, Peak:494.94M` in Blender 4.5 against `Mem:1224.33M, Peak:1694.79M` on one card. So the figure does not count memory divided by the number of cards. What it does count is not settled by this test and nothing is read into it. In Blender 4.5 the **first** `Mem:` field moves the other way — `103.60M` with four cards, `95.44M` with two, `91.37M` with one — and nothing is read into that either.

**A graphics card was filled while the machine's own memory held, which no earlier attempt had managed.** The scene is a plane whose surface samples a growing number of picture files of 8192 by 8192 pixels, each read through its own Mapping node so that none can be left out, added together and divided by their count; 640 by 480, 16 samples, one card ticked. The card is 16,380 MiB. What came back:

```
################ 16 textures of 8192 by 8192, one card
exit=0
Fra:1 Mem:4118.21M (Peak 4127.59M) | Time:00:05.05 | Mem:4440.80M, Peak:4440.80M | Scene, ViewLayer | Finished
Time: 00:05.33 (Saving: 00:00.10)
   card 0 4332MiB
   card 1 3MiB
   card 2 3MiB
   card 3 3MiB
################ 32 textures of 8192 by 8192, one card
exit=0
Fra:1 Mem:8214.79M (Peak 8224.17M) | Time:00:06.10 | Mem:8536.80M, Peak:8536.80M | Scene, ViewLayer | Finished
Time: 00:06.41 (Saving: 00:00.10)
   card 0 9030MiB
################ 64 textures of 8192 by 8192, one card
exit=0
Fra:1 Mem:15127.96M (Peak 16394.58M) | Time:00:19.24 | Mem:16728.81M, Peak:16728.81M | Scene, ViewLayer | Finished
Time: 00:19.90 (Saving: 00:00.10)
   card 0 15688MiB
################ 128 textures of 8192 by 8192, one card
exit=0
Fra:1 Mem:15130.27M (Peak 29196.88M) | Time:03:25.29 | Mem:33112.80M, Peak:33112.80M | Scene, ViewLayer | Finished
Time: 03:28.53 (Saving: 00:00.07)
   card 0 15658MiB
```

*(The runs above are quoted as they printed, except that the three unused cards' lines — `card 1 3MiB`, `card 2 3MiB`, `card 3 3MiB` — are given once and not repeated for each run; they read the same in every one of them. The whole output is in `Audits/Blender Research/Reports/Hand tests — four cards.md`.)*

**At 64 pictures the card is as full as it goes** — about 15.7 GB of 16.38 — and Blender's own figure already reads more than the card has. **At 128 pictures Blender asks for about 33 GB, twice the card's whole memory, and the card still sits at the same 15.7 GB.**

**Nothing failed and nothing was said.** Every run exited 0 and wrote its picture, and searching both streams of every run for `out of memory`, `failed to allocate`, `cannot allocate` and `insufficient` matched nothing. The only sign is the cost: the render time went from about twenty seconds at 64 pictures to about three and a half minutes at 128 — ten times longer for twice the data, at the same size and the same sample count.

**The one line that speaks to it at all** is printed by `--debug-cycles` before anything is loaded, and it names the machine's own memory as what the card may reach into:

```
Mapped host memory limit set to 208,234,123,264 bytes. (193.93G)
Local memory reserved 331,350,016 bytes. (316.00M)
```

**And with four cards it filled every one of them, and got slower rather than faster.** The 128-picture scene with all four ticked, in this version:

```
exit=0
Fra:1 Mem:15142.35M (Peak 29452.95M) | Time:13:08.08 | Mem:32768.09M, Peak:32772.77M | Scene, ViewLayer | Finished
Time: 13:11.85 (Saving: 00:00.08)
   card 0 15714MiB
   card 1 15630MiB
   card 2 15630MiB
   card 3 15658MiB
```

Four cards of 16 GB gave a 32 GB scene no more room: each filled to the same amount the single card reached, and the run took `13:11.85` against `03:28.53` on one card.

**So the documented fallback to system memory has been seen for the first time, and it is silent.** What was **not** reached is the `Error: Out of memory` message the manual names: on this machine a scene bigger than the card spills into 197 GB of system memory instead, and running that out as well was not attempted.

**Driven from outside — this block.** Every run above was made from outside Blender's window, and on this machine there is no other way: with no display server there is nothing for a window to open on. So the `VRAM:` Status Bar readout is still unreached, with four cards as with two.

**Operating system — this block.** Linux, on the machine named at the top of it. It says nothing about macOS — where the Creator's M1 Pro shares one pool of memory, so a card cannot be filled while system memory holds even in principle — and nothing about Windows, which no Windows machine has run.

**What could not be reached here.** The `VRAM:` Status Bar readout with two cards. That readout needs Blender's window, and this machine has no display server at all — the machine that could photograph a window had one card, and the machine that has two cards cannot show a window. So **whether the Status Bar figure names a device, adds two cards together, or shows only one is still unanswered.** So is anything about an NVLink pair.

**Outside the interface.** Settled in part. With no window, Blender 4.5 printed no graphics-card memory figure at all, on a render that was actually running on the graphics card. With the window open, the Status Bar's own right-click menu offers no video-memory item to switch on. The two `Mem:` fields and the `--cycles-print-stats` totals appeared identically whether the render used the processor or the card, and nothing labelled any of them as the card's. The one documented graphics-card readout is the `show_statusbar_vram` preference, which is present in this version and set off, and reading it needs the window.

**Operating system.** Nothing about the readout names an operating system. The surrounding graphics-card material is heavily per-system (CUDA and OptiX on Windows and Linux; HIP on Windows and Linux; oneAPI on Windows and Linux; Metal on Apple Silicon macOS 13.0 or newer — GPU Rendering page), and the memory *fallback* to system RAM is documented for CUDA, OptiX, HIP and Metal but not oneAPI. The Creator's Status Bar menu screenshot and the Metal render were got on macOS 26.5.2 with Apple's own graphics. The `VRAM:` readout, the CUDA render and the `--debug-cycles` figures above were got on the rented Linux machine with an NVIDIA GeForce RTX 4090. **Both machines were run and they agree that nothing in the ordinary printed output is marked as the card's memory.** They differ in what could be reached: only Linux could show the readout, and only the Mac could photograph the right-click menu.

**Windows is untested and no Windows machine has run it.** **More than one card is no longer out of reach** — a third machine with two NVIDIA GeForce RTX 4090s answered it on 2026-08-06 and its result is in the block above: one figure, never two, and each card holding the same amount as the other. **A card running out of memory while system memory remains is no longer out of reach either** — it was reached on 2026-08-06 on a fourth machine with four NVIDIA GeForce RTX 4060 Ti of 16 GB each and 197 GB of system memory, and what happened is in the four-card block above. *(Corrected by the four-card citation check, 2026-08-06: this line still said the test was written out and never run, which the block above it and the closing list below it both contradict.)* Still out of reach: an NVLink bridge; the `VRAM:` Status Bar readout with two cards, the two-card machine having no display server and the machine with a display having one card; and any card that is neither Apple's nor NVIDIA's, so HIP and oneAPI are unreachable. `--debug-cycles` on the Linux machine records `HIPEW initialization failed: Error opening HIP dynamic library`, which is what an absent AMD card looks like there.

---

## Things a person with Blender 4.5 open could still settle in a few minutes

Every item that was on this list has now been run — items 3, 4, 6, 7 and 8 from a terminal and the rest by the Creator with the window open, all on 2026-08-05, and the last one on a rented Linux machine on 2026-08-06. Their results are written into the answers above. **The list is empty.**

The one item that survived the Mac run — what the video-memory readout actually displays — was answered on the one-card rented Linux machine on 2026-08-06 and is recorded under question 6. It is labelled `VRAM:` and shows a used-of-total figure in GiB, with `24.0` on the right, which is that card's whole memory. The left-hand figure moves with what is on the card and what it counts is still not settled, so it is not quoted here as a fixed reading. *(Narrowed by the two-card citation check, 2026-08-06: this line quoted the moving figure as the reading, which the narrowing under question 6 forbids.)*

What is still open, and why:

- **What happens on Windows.** No Windows machine has run it.
- **What a card of another make does.** No machine the project has holds an AMD or an Intel graphics card, so HIP and oneAPI cannot be reached.
- **What the `VRAM:` Status Bar readout says when there are two cards.** Answered in part on 2026-08-06 by a third machine with two NVIDIA GeForce RTX 4090s, which showed that Blender's *printed* output gives one memory figure and never two, and that each card holds the same amount as the other — see question 6. The Status Bar readout itself could not be reached there: that machine has no display server at all, and the machine that could photograph a window had one card.
- **What an NVLink bridge changes.** The two-card machine's cards talk through the host bridge (`PHB`), not NVLink.
- **What happens when the graphics card runs out of memory while system memory remains.** **Answered on 2026-08-06** on a fourth machine — four NVIDIA GeForce RTX 4060 Ti of 16 GB each and 197 GB of system memory, which is the first machine the project has had that could produce this case at all. The card fills to its own size, the render carries on into system memory, nothing is printed, the process exits 0 and writes its picture, and the run takes about ten times longer for twice the data. See question 6. The Mac still cannot do it — the M1 Pro shares one pool — and `Error: Out of memory` was still not reached, since the fourth machine has 197 GB to spill into. *(Rewritten 2026-08-06 by the hand tests on the four-card machine; this bullet used to say the test was written out and never run.)*
- **What a disk filling up mid-animation does.** Same reason; also written out there.
- **Running out of the machine's own memory during a processor render.** Not tried on either machine.
- **What the Status Bar's right-click menu offers on Linux.** It could not be opened on that machine, for the reason recorded under question 6.
- **Whether a failure entry stays in the Info editor or fades.** Not captured on either machine.

---

2026-08-04 — Opus 5 (the tool you're running in) — Reports, Blender 4.5

2026-08-05 — Opus 5 (Claude Code) — added the observed findings from the hand tests to questions 1 to 6, brought the two standing lines up to date where a test settled them, and pruned the list above

2026-08-05 — Opus 5 (Claude Code) — added the Creator's own window tests to questions 1, 2, 3, 5 and 6, and pruned the list above again

2026-08-05 — Opus 5 (Claude Code) — citation check: narrowed nine claims to what their quotes and their own records carry, corrected one count that contradicted the runs recorded here, and sent one thinly recorded observation to the hand tests. Each change is noted where it was made.

2026-08-05 — Opus 5 (Claude Code) — closed the one item the citation check sent to the hand tests: replaced the screenshot note about the Output panel's header, under question 1, with the Creator's own answer and the steps they followed

2026-08-06 — Opus 5 (Claude Code) — added the hand tests run on a rented Linux machine with an NVIDIA card to questions 1 to 6, kept separate from the Mac's results throughout; answered what the video-memory readout displays, where a crash file goes on Linux, what the log-file switch catches, what the render operator hands a script, and six failures the Mac could not reach; brought both standing lines up to date on every question and emptied the list at the end

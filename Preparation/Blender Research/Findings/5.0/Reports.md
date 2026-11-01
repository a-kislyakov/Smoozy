# Reports — Blender 5.0

**Sources consulted, and when.** Blender's 5.0 manual (plain-text copy plus its `_images` pictures), Blender's 5.0 Python API documentation, Blender's 5.0 release notes, and Cycles' own files as shipped with Blender 5.0 — all from `Blender Sources/`, snapshot taken 2026-07-30. That snapshot date is the date read for every finding below. No web access was used.

I read only the question sheet and files inside Blender Sources/. I did not open any other part of this project.

---

## 1. What can cause a render to fail, and what does Blender do in each case?

**Label: documented** for the handful of specific failures the sources name; **not in the sources** for any general list of what makes a render fail and what Blender does about it.

The sources nowhere set out "here is what can make a render fail". What they do have is a scattering of named failures, mostly hardware- and driver-related, and in most of them the source says what the message means but not what Blender then does.

**Failures the sources name, with the exact words.**

*Graphics card runs out of memory (Cycles).* Manual, Cycles GPU Rendering, under a heading "Error Messages":

> "Error: Out of memory
>
> This usually means there is not enough memory to store the scene for use by the GPU."

The same page, in a different section, says what Blender does when the graphics card's memory is full:

> "With CUDA, OptiX, HIP and Metal devices, if the GPU memory is full Blender will automatically try to use system memory. This has a performance impact, but will usually still result in a faster render than using CPU rendering."

**This is the one failure in the list that has since been watched happening.** On 2026-08-06, on a machine with four NVIDIA GeForce RTX 4060 Ti of 16 GB each and 197 GB of system memory, a scene needing about 33 GB was rendered with a 16 GB card and the fallback ran: the card filled to its own size, the render finished, the picture was written, the process exited 0, **nothing was printed about any of it**, and the run took about ten times longer. It is recorded in full under question 6, as **Observed, 2026-08-06 — what the memory readouts say with four graphics cards, and what happens when a card is filled**. *(Pointer added by the four-card citation check, 2026-08-06: the hand-test list says that run answers questions 1, 2 and 6, and it had been written into question 6 alone.)*

(Citation check 2026-08-05: this used to say the page describes the fallback as happening "before that point is reached". The fallback sits under "Why does a scene that renders on the CPU not render on the GPU?" and the out-of-memory message under "Error Messages"; nothing on the page puts one before the other.)

And, as the answer to "Why does a scene that renders on the CPU not render on the GPU?":

> "There may be multiple causes, but the most common one is that there is not enough memory on your graphics card."

Source: manual. Address: https://docs.blender.org/manual/en/5.0/render/cycles/gpu_rendering.html — date read 2026-07-30.

*Graphics driver watchdog (NVIDIA, Windows).* Same page:

> "The NVIDIA OpenGL driver lost connection with the display driver
>
> If a GPU is used for both display and rendering, Windows has a limit on the time the GPU can do render computations. If you have a particularly heavy scene, Cycles can take up too much GPU time."

*CUDA kernel build failure.* Same page:

> "CUDA Error: Kernel compilation failed
>
> This error may happen if you have a new NVIDIA graphics card that is not yet supported by the Blender version and CUDA toolkit you have installed. In this case Blender may try to dynamically build a kernel for your graphics card and fail."

*Unsupported compiler (Linux).* Same page:

> "Unsupported GNU version
>
> On Linux, depending on your GCC version you might get this error."

(Citation check 2026-08-05: the heading used to read "(Linux, CUDA)". Neither quoted line names CUDA or says what the compiler is being used for.)

*EEVEE shadow memory exhausted.* Manual, EEVEE Light Settings:

> "The error message Shadow buffer full means that the system cannot allocate enough shadow memory."

Address: https://docs.blender.org/manual/en/5.0/render/eevee/light_settings.html — date read 2026-07-30.

*EEVEE using too much graphics memory.* Manual, EEVEE Limitations:

> "In practice, using too much GPU memory can make the GPU driver crash, freeze, or kill the application."

Address: https://docs.blender.org/manual/en/5.0/render/eevee/limitations/limitations.html — date read 2026-07-30.

*Incompatible video container and codec.* Manual, Output Properties:

> "Some containers and codecs are not compatible with each other, so if you are getting errors check that your container and codec are compatible."

Address: https://docs.blender.org/manual/en/5.0/render/output/properties/output.html — date read 2026-07-30.

*A shader node setup Cycles cannot take.* Manual, Repeat Zone shader node:

> "The number of iterations in Cycles must evaluate to a constant value."

The manual shows a picture of the resulting state, `_images/repeat-zone-dynamic-iterations-not-supported.png`, captioned "Error shown when Cycles is selected as the active Render Engine." In that picture the Repeat node carries a warning triangle and a message box reading "Iterations input has to be a constant value" — **documented**, from that picture. Address: https://docs.blender.org/manual/en/5.0/render/shader_nodes/utilities/repeat.html — date read 2026-07-30.

*A custom OSL shader that will not compile.* Manual, Open Shading Language:

> "Blender will compile the OSL source file automatically. If the source is .osl, it will be compiled into .oso bytecode. Compilation errors will be shown in the system console."

Address: https://docs.blender.org/manual/en/5.0/render/cycles/osl/index.html — date read 2026-07-30.

*Blender stopping outright.* Manual, Crashes:

> "The most common causes of Blender crashes:
>
> Running out of memory.
>
> Issues with graphics hardware or drivers.
>
> Bugs in Blender."

and what Blender does:

> "When Blender crashes, it writes out a text file which contains information that may help identify the cause of the crash. Usually, this file is written in the Temporary Directory directory."

Address: https://docs.blender.org/manual/en/5.0/troubleshooting/crash.html — date read 2026-07-30.

**What Blender does, where the sources say it at all.** Graphics-card memory full with CUDA, OptiX, HIP or Metal — falls back to system memory and carries on (quoted above). OSL compile failure — the errors appear in the system console. Cycles repeat-zone problem — an error is drawn on the node. A crash — a crash file is written. For every other failure above, the sources give the message's meaning and a remedy, and say nothing about what Blender does with the render.

The Python API shows the channel a render engine has for reporting a failure, which is the closest thing to a general answer:

> "error_set(message)
>
> Set error message displaying after the render is finished"

and

> "report(type, message)
>
> Report info, warning or error messages"

Source: Python API documentation. Address: https://docs.blender.org/api/5.0/bpy.types.RenderEngine.html — date read 2026-07-30. The report types available are listed separately and include `WARNING`, `ERROR`, `ERROR_INVALID_INPUT`, `ERROR_INVALID_CONTEXT` and `ERROR_OUT_OF_MEMORY`. Address: https://docs.blender.org/api/5.0/bpy_types_enum_items/wm_report_items.html — date read 2026-07-30.

**Where I looked, to justify the "not in the sources" half.** I searched the whole manual, the whole API documentation, the whole release notes and Cycles' own files for: "render failed", "failed", "failure", "error", "cannot render", "unable to", "could not", "disk full", "no write permission", "cannot open", "cannot create", "out of memory", "no camera", "active camera", "missing camera", "exit code", "return code", "abort". The hits are the ones quoted above plus unrelated material (mesh tools, tracking, bug-report guidance). There is no page in the 5.0 manual that enumerates render failures, and nothing anywhere about what a failed render leaves behind, what it does to the output file, whether an animation carries on to the next frame, or what a process exits with.

**Not covered by the sources.** The sources do not settle: what Blender does after a failure that is not one of the four cases above; whether a partly-written output file is left on disk; whether an animation render stops or skips to the next frame; any process exit status; failures from the file system (permissions, full disk, unwritable output path); failures from a missing camera or missing linked file. Most of that list was settled by tests instead; see the observed block below. Still unsettled after those tests: a disk filling up mid-animation, running out of system memory during a processor render, and a corrupt or newer-version blend-file.

**Observed, 2026-08-05 — nine failure situations tried from a terminal.**

How all of these were run, once for all of them. On the Creator's own computer: a MacBook Pro with an Apple M1 Pro chip, macOS 26.5.2, Apple's own graphics and no other graphics card. Blender 5.0.1, at `/Applications/Blender 5.0.1.app`, started with no window (the `-b` switch) and with a throwaway settings folder of its own so nothing of the Creator's was touched. Unless said otherwise the file was the factory startup scene — one cube, one camera, one light — saved by Blender 4.5 so every version could open it, with render engine Cycles, 16 samples, 160 by 120 pixels, frames 1 to 3, PNG output. Each run's two output streams were kept in separate files and the shell's exit status was read straight afterwards. The full commands are in `Audits/Blender Research/Reports/Hand tests.md`, entries 7 to 12, 18 and 19.

**1. No camera in the scene.** The camera object was deleted from the file before saving. Run: `-b "no camera.blend" -o "<folder>/frame_" -F PNG -f 1`. The whole of standard output:

```
00:00.264  blend            | Read blend: "/Users/artist/Documents/Smoozy hand tests/Reports/23/no camera.blend"
00:00.268  render           | Rendering single frame
00:00.268  render           | Rendering frame 1
00:00.268  reports          | ERROR Cannot render, no camera
Blender 5.0.1 (hash a3db93c5b259 built 2025-12-16 01:51:40)

Blender quit
```

Standard error was empty. Exit status 1. No image file existed in the output folder afterwards.

**2. The output folder does not exist.** Run: `-b "simple.blend" -o "<folder>/no_such_folder_50/deeper/frame_" -F PNG -f 1`, where neither folder existed. Exit status 0, standard error empty, and standard output ended:

```
00:00.471  render           | Saved: '/private/tmp/smoozy-test/hand-tests/Reports/007/no_such_folder_50/deeper/frame_0001.png'
00:00.471  render           | Time: 00:00.20 (Saving: 00:00.04)
```

Both folders existed afterwards and held the image.

**3. The output path cannot be written.** A one-byte plain file named `blocker_50` was made, and the output path was set inside it, so no folder could be created there. Run: `-b "simple.blend" -o "<folder>/blocker_50/frame_" -F PNG -f 1`. Exit status 1, standard error empty, and standard output ended:

```
00:00.363  render           | Fra: 1 | Mem: 2M | Sample 16/16
00:00.363  render           | Fra: 1 | Mem: 2M | Finished
00:00.402  image            | ERROR Couldn't create directory for file /private/tmp/smoozy-test/hand-tests/Reports/008/blocker_50/frame_0001.png: File exists
00:00.402  reports          | ERROR Render error (File exists) cannot save: '/private/tmp/smoozy-test/hand-tests/Reports/008/blocker_50/frame_0001.png'
00:00.402  render           | Time: 00:00.13
```

Two lines, under two different categories, `image` and `reports`. The render itself finished; the saving is what failed.

**4. An output path with a broken template in it.** Run: `-b "simple.blend" -o "<folder>/out_50/{blend_name.png" -F PNG -x 1 -a`. Exit status 1, standard error empty, and the whole of standard output:

```
00:00.263  blend            | Read blend: "/private/tmp/smoozy-test/hand-tests/Reports/018/simple.blend"
00:00.267  render           | Rendering animation (frames 1..3)
00:00.267  render           | Rendering frame 1
00:00.267  reports          | ERROR Parse errors in path '/private/tmp/smoozy-test/hand-tests/Reports/018/out_50/{blend_name.png':
                            | - Invalid or incomplete template expression '{blend_name.png'.
Blender 5.0.1 (hash a3db93c5b259 built 2025-12-16 01:51:40)

Blender quit
```

Note the second line of the message: the timestamp and category are left blank and the vertical bar is kept, so one report can run over several lines. No frame was rendered and the output folder was empty.

**5. An animation whose output folder stops being writable partway.** A six-frame Cycles animation at 2000 samples, 320 by 240, about seven seconds a frame. It was started, and once `frame_0001.png` existed the output folder was renamed away and a one-byte plain file was put in its place. Exit status 1. Standard output, at the point it went wrong:

```
00:07.264  render           | Saved: '/private/tmp/.../012/out_50/frame_0001.png'
00:07.264  render           | Time: 00:06.99 (Saving: 00:00.05)
00:07.264  render           | Rendering frame 2
00:14.115  image            | ERROR Couldn't create directory for file /private/tmp/.../012/out_50/frame_0002.png: File exists
00:14.115  reports          | ERROR Render error (File exists) cannot save: '/private/tmp/.../012/out_50/frame_0002.png'
00:14.115  render           | Time: 00:06.85
```

Nothing was printed for frames 3, 4, 5 or 6 — there was no further `Rendering frame` line. The renamed folder held one file, `frame_0001.png`. No part-written file was left behind.

**6. The blend-file does not exist.** Run: `-b "<folder>/there_is_no_such_file.blend" -f 1`. Exit status 1, standard error empty, whole of standard output:

```
00:00.266  reports          | ERROR Cannot read file "/private/tmp/smoozy-test/hand-tests/Reports/010/there_is_no_such_file.blend": No such file or directory
Blender 5.0.1 (hash a3db93c5b259 built 2025-12-16 01:51:40)

Blender quit
```

**7. A Cycles device override Cycles will not take.** Run: `-b "simple.blend" -o "<folder>/out_50/frame_" -F PNG -f 1 -- --cycles-device NONSENSE`. Exit status 1, standard error empty, whole of standard output:

```
00:00.348  cycles           | ERROR NONSENSE is not a valid Cycles device.
Blender 5.0.1 (hash a3db93c5b259 built 2025-12-16 01:51:40)
Error: Not freed memory blocks: 1, total unfreed memory 0.000004 MB
```

No frame was rendered and the output folder was empty. The same command with `--cycles-device METAL` instead rendered normally and ended with status 0.

**8. An image texture that is missing when the render starts.** A cube was given a material with an image texture pointing at `gone.png`, the file was saved, and `gone.png` was then moved away. Run: `-b "missing.blend" -o "<folder>/out_50/frame_" -F PNG -f 1`. Exit status 0, and the image was written. One line was printed about it:

```
00:00.279  cycles           | ERROR Image file '/private/tmp/smoozy-test/hand-tests/Reports/019/gone.png' does not exist.
```

So it is reported as an error, and the render goes ahead and succeeds anyway.

**9. Turning off automatic saving of render output.** Run: `-b "simple.blend" --python-expr "import bpy; bpy.context.scene.render.save_output = False" -o "<folder>/out2_50/frame_" -F PNG -x 1 -a`. Exit status 0. All three frames were rendered and saved. Standard error held:

```
Traceback (most recent call last):
  File "<string>", line 1, in <module>
AttributeError: 'RenderSettings' object has no attribute 'save_output'
```

So there is no such setting in 5.0, and the render went ahead as if nothing had been asked.

**Observed, 2026-08-05 — two of those failures repeated with Blender's window open.** Run by the Creator on their own MacBook Pro (Apple M1 Pro, macOS 26.5.2), Blender 5.0.1, reported as screenshots.

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

**Observed, 2026-08-05 — the Output panel's header, looked at by hand.** Done by the Creator on their own MacBook Pro (Apple M1 Pro, macOS 26.5.2), in Blender 5.0.

What was asked of them, the same in all four versions: open Blender at its normal startup file, go to the Properties editor, choose the Output tab, find the panel headed `Output`, and look at the header strip of that panel next to the word `Output`. If a checkbox is drawn there, rest the pointer on it and wait for the tooltip.

What came back: **there is no checkbox in that header in 5.0.** The Creator reported that the checkbox exists only in 5.2. This version was looked at and has none, which is a result rather than a silence. With no checkbox here, there was no tooltip to read.

The Creator works on a Mac, so this settles macOS and leaves Windows and Linux open. Their answer says nothing about the checkboxes inside the panel, and that is left open rather than filled in.

(Citation check 2026-08-05: this used to be a note about what a screenshot showed, with nothing about what was done to get the screenshot, so nobody could repeat it. The check asked for the procedure, and the record above replaces it. The steps are in `Audits/Blender Research/Reports/Hand tests.md`, entry 24.)

**Observed, 2026-08-06 — the same failures, and six more, on a second machine.**

Everything in this block was run on **a rented Linux machine — Ubuntu 22.04.5 LTS, NVIDIA GeForce RTX 4090, driver 580.105.08, twelve cores, run over SSH**. Blender Blender 5.0.1 at `/opt/blender/blender-5.0.1-linux-x64/blender`, started with `-b`, pointed at its own settings folder, with the display turned off first (`unset DISPLAY`, `unset XAUTHORITY`). The scene was the same one-cube factory scene described above, rebuilt on that machine. The machine is a rented container shared with other work, so nothing below says how long anything took or how much memory was free. Full commands in `Audits/Blender Research/Reports/Hand tests — Linux.md`.

**The Mac's results held, word for word.** No camera, a missing output folder, a blocked output path, a broken path template, a missing blend-file, a refused Cycles device, an animation that loses its output folder partway, a missing image texture, and the output-saving setting — every one came back with the same exit status and the same message text as on the Mac. Two examples, quoted from the Linux runs:

```
00:00.213  reports          | ERROR Cannot render, no camera
```

```
00:00.412  image            | ERROR Couldn't create directory for file /root/smoozy/Reports/work/L08/blocker_50/frame_0001.png: File exists
00:00.412  reports          | ERROR Render error (File exists) cannot save: '/root/smoozy/Reports/work/L08/blocker_50/frame_0001.png'
```

Both machines were run and both agreed.

**A genuinely read-only output folder — new, and not reachable on the Mac.** The folder was made with `chmod 0555`. On this machine the user is root, and root walks through file permissions, so the run was made twice. Run plain, the permission was ignored: exit 0 and the image written. Run again with that overriding permission dropped for the one command — `capsh --drop=cap_dac_override,cap_dac_read_search -- -c "…"` — exit status **1**, nothing written, standard error empty, and standard output ended:

```
00:00.469  image.write      | ERROR OpenImageIO write failed: Could not open file "/root/smoozy/Reports/work/L25/ro_50/frame_0001.png"
00:00.469  reports          | ERROR Render error (Success) cannot save: '/root/smoozy/Reports/work/L25/ro_50/frame_0001.png'
```

**The word in brackets reads `Success`** on a failure caused by a refused permission. The category `image.write` is one the blocked-folder test never produced; that one said `image`.

**A corrupt blend-file — new.** Two were tried. A file cut short (the first 200 kB of a good one) gave exit status 1 and:

```
00:00.210  reports          | ERROR Failed to read blend file '/root/smoozy/Reports/work/L24/truncated.blend': Missing DNA block
```

A file that was not a blend-file at all gave exit status 1 and:

```
00:00.213  reports          | ERROR File format is not supported in file "/root/smoozy/Reports/work/L24/garbage.blend"
```

**A blend-file written by a newer Blender — new, and it is not a failure.** The same scene was saved by Blender 5.2 and opened here. Exit status **0**, the frame rendered and written, and one line said so:

```
00:00.222  reports          | WARNING File written by newer Blender binary (502.44), expect loss of data!
```

**A crash, and where the crash file goes on Linux — new. The crash page's Linux line, run.** Blender was made to crash on purpose with a script whose whole content was `import ctypes`, a `print`, and `ctypes.string_at(0)`. With `simple.blend` loaded, the whole of standard output was:

```
00:00.206  blend            | Read blend: "/root/smoozy/Reports/work/simple.blend"
Blender 5.0.1 (hash a3db93c5b259 built 2025-12-16 01:30:59)
Writing: /tmp/simple.crash.txt
```

The process ended with status **139**. A file `/tmp/simple.crash.txt` existed afterwards, holding a version line, a section headed `# backtrace`, and a section headed `# Python backtrace`. Run again with `--factory-startup` and no file loaded, the line read `Writing: /tmp/blender.crash.txt` and that file was written instead. **So on Linux the crash file is named after the blend-file when there is one**, and `blender.crash.txt` only when there is not — the manual's page names only the second. The `print` that ran immediately before the crash never reached standard output.

**Which handlers fire on four more kinds of failure — new.** The probe described under question 2 was run again against five cases, not one. On a blocked save: `render_init`, `render_pre`, `render_cancel`. On a broken path template: `render_init` and `render_cancel` only — `render_pre` never fired at all. On a missing image texture, which succeeds: the full successful set. So `render_cancel` on failure is now seen on three different failures in this version rather than one.

**A Cycles device this card does offer.** `-- --cycles-device CUDA` and `-- --cycles-device OPTIX` were both accepted and both rendered the frame, exit 0. The refused name behaved exactly as on the Mac: exit 1, no frame, and

```
00:00.181  cycles           | ERROR NONSENSE is not a valid Cycles device.
```

on standard output, followed by `Error: Not freed memory blocks: 1, total unfreed memory 0.000004 MB`.

**Outside the interface.** Settled by the tests above for the cases run. With no window at all, a failed render prints its message and leaves the process with status 1, and a successful one leaves 0. One thing does not follow that rule: a missing image texture (item 8), which reports an error and still ends at 0. (Citation check 2026-08-05: this used to say "Two things" and named one.) Everything printed went to standard output; standard error was empty in all nine runs except the Python traceback in item 9. The OSL case is explicitly tied to "the system console", which exists whether or not Blender's own window is up, and the Cycles GPU error text is not tied to any part of the interface. For the graphics-card failures the sources describe, no test was possible on this machine.

**Operating system.** Partly OS-specific by the sources' own words. The manual states CUDA, OptiX and HIP are "supported on Windows and Linux"; Metal is "supported on Apple computers with Apple Silicon"; the NVIDIA driver-timeout failure is described as a Windows limit ("Windows has a limit on the time the GPU can do render computations"); the "Unsupported GNU version" failure is described as "On Linux". Crash-file behaviour is given separately per operating system (a `.crash.txt` beside the blend-file name on Windows, the macOS Crash Reporter and `~/Library/Logs/DiagnosticReports/` on macOS, `blender.crash.txt` in `/tmp` on Linux). The memory and codec failures are not tied to any operating system in the text.

**Two machines have now been used.** **macOS** is settled by the Creator's own MacBook Pro (Apple M1 Pro, macOS 26.5.2) for the nine cases first run; **Linux** is settled by the rented Linux machine for those same nine plus six more, recorded above. The two agreed on every message except one word in Blender 4.5's saving failure, which is not this version. **Windows is untested and no Windows machine has run it**, so the driver time-out and the Windows crash-file behaviour stay open.

**The Linux "Unsupported GNU version" could not be reached even on the Linux machine**, and the reason is worth keeping. That error comes from the CUDA toolkit's compiler being handed an unsupported GCC, and it only arises when Blender has to build a CUDA kernel while it runs. There is no CUDA toolkit on that machine — `which nvcc` returns nothing and `/usr/local/cuda` does not exist — and this Blender ships a kernel already built for that card, which `--debug-cycles` states outright: `Testing for pre-compiled kernel …/kernel_sm_89.cubin.zst.` then `Using precompiled kernel.` So no compile is ever attempted, and installing the toolkit was not permitted on that machine. The kernel-build failure for a card newer than Blender is unreachable for the same reason.

**The fallback to system memory when the card fills has still not been seen happen.** It needs a graphics card whose memory can be exhausted while the machine's own memory is not, which the rented Linux machine can do — but only with nothing else running on it, and it was shared. The test is written out in full, ready to run, under "Needs the machine to itself" in `Audits/Blender Research/Reports/Hand tests — Linux.md`. **HIP** needs an AMD card and **oneAPI** an Intel card; neither machine has one, and `--debug-cycles` on the Linux machine records `HIPEW initialization failed: Error opening HIP dynamic library`.

---

## 2. Where does Blender say that a render failed, and what does it say there?

**Label: documented** for places that can carry errors and reports; **not in the sources** for which of those places carries a failed render, or for the words a failed render actually produces.

**Places that can carry errors and reports.** The sources describe these places generally. Except where a specific failure is quoted in question 1, they do not say that a render failure appears in any one of them.

*The status bar, in the middle.* Manual, Status Bar:

> "Report Message
>
> Informational messages or warnings, such as after saving a file. They disappear after a short time. Click them to show the full message in the Info Editor."

Address: https://docs.blender.org/manual/en/5.0/interface/window_system/status_bar.html — date read 2026-07-30.

*The Info editor.* Manual, Info Editor:

> "The Info editor logs the executed operators as well as errors, warnings, and informational messages."

Address: https://docs.blender.org/manual/en/5.0/editors/info_editor.html — date read 2026-07-30. The Python API confirms errors are a separately switchable category there:

> "show_report_error
>
> Display error text"

Address: https://docs.blender.org/api/5.0/bpy.types.SpaceInfo.html — date read 2026-07-30.

*The console / terminal.* Manual, Using Blender From The Command Line:

> "The Console Window, also called a Terminal, is an operating system text window that displays messages about Blender's operations, status, and internal errors."

and

> "If Blender exits unexpectedly, the messages may indicate the cause or error."

Address: https://docs.blender.org/manual/en/5.0/advanced/command_line/index.html — date read 2026-07-30. The troubleshooting section says the same in more detail:

> "The Blender Console Window can display many different types of status and error messages. Some messages simply inform the user what Blender is doing, but have no real impact on Blender's ability to function. Other messages can indicate serious errors that will most likely prevent Blender carrying out a particular task and may even make Blender non-responsive or shut down completely."

Address: https://docs.blender.org/manual/en/5.0/troubleshooting/startup.html — date read 2026-07-30.

*The log, which in 5.0 now carries the reports too.* Release notes, Core, "Logging":

> "Most of Blender now uses the same logging system, including operators reports and background render progress."

Address: https://developer.blender.org/docs/release_notes/5.0/core/ — date read 2026-07-30. The log can be levelled down to errors alone and sent to a file — manual, Logging Options:

> "--log-level <level>
>
> Set the logging verbosity level.
>
> fatal: Fatal errors only
> error: Errors only"

and

> "--log-file <filepath>
>
> Set a file to output the log to."

Address: https://docs.blender.org/manual/en/5.0/advanced/command_line/arguments.html — date read 2026-07-30.

*After the render, from the engine itself.* Python API, RenderEngine:

> "error_set(message)
>
> Set error message displaying after the render is finished"

Address: https://docs.blender.org/api/5.0/bpy.types.RenderEngine.html — date read 2026-07-30. The documentation does not say where that message is displayed.

**What it says there.** The only failure wordings the sources give in full are the Cycles graphics-card ones already quoted in question 1 — "Error: Out of memory", "The NVIDIA OpenGL driver lost connection with the display driver", "CUDA Error: Kernel compilation failed", "Unsupported GNU version" — plus EEVEE's "Shadow buffer full" and the node-editor text "Iterations input has to be a constant value". There is no documented wording for a render that fails in general.

**Where I looked.** Searched all four sources for "render failed", "failed", "error", "warning", "report", "console", "Info editor", "status bar", "stdout", "stderr". Read in full: the Status Bar page, the Info Editor page, the Command Line index and arguments pages, the three Command Line Launching pages, the Crashes and Startup troubleshooting pages, the Cycles GPU Rendering page, `bpy.types.RenderEngine`, `bpy.types.SpaceInfo`, `bpy.app.handlers`, and the Core, Rendering, Python API and Corrective Releases release notes. I also opened the manual's own picture of the Info editor (`_images/editors_info-editor_ui.png`); it shows only three green-ticked operator lines (`bpy.ops.curve.primitive_bezier_curve_add(...)` and similar) and no error entry, so it settles nothing about how a failure looks there.


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

**Not covered.** This does not settle what a failed render's message text is; whether the same message appears in more than one of these places at once; how long it stays; or whether a failure is distinguishable from a cancellation by the text alone.

**Observed, 2026-08-05 — where a failed render says so with no window, and what it says.**

Same computer and same way of running as in question 1: a MacBook Pro with an Apple M1 Pro chip, macOS 26.5.2, Apple's own graphics, Blender 5.0.1 started with `-b` and its own throwaway settings folder. The two output streams were kept in separate files every time.

**The log carries the reports, and it all goes to standard output.** Every failure message in the nine runs of question 1 appeared on standard output, under a log category, in the shape `<elapsed time>  <category>          | <LEVEL> <message>`. Standard error was completely empty in eight of the nine — the only thing ever written to it was the Python traceback in item 9. Three different categories carried failure messages: `reports` for what an operator reported, `image` for the file-writing complaint, and `cycles` for the render engine's own.

**The exact words, quoted from the runs.** Each of these is the complete failure text that appeared:

```
00:00.268  reports          | ERROR Cannot render, no camera
```

```
00:00.402  image            | ERROR Couldn't create directory for file /private/tmp/.../blocker_50/frame_0001.png: File exists
00:00.402  reports          | ERROR Render error (File exists) cannot save: '/private/tmp/.../blocker_50/frame_0001.png'
```

```
00:00.267  reports          | ERROR Parse errors in path '/private/tmp/.../out_50/{blend_name.png':
                            | - Invalid or incomplete template expression '{blend_name.png'.
```

```
00:00.266  reports          | ERROR Cannot read file "/private/tmp/.../there_is_no_such_file.blend": No such file or directory
```

```
00:00.348  cycles           | ERROR NONSENSE is not a valid Cycles device.
```

```
00:00.279  cycles           | ERROR Image file '/private/tmp/.../gone.png' does not exist.
```

**The process status.** Read with `echo $?` straight after each run. A render that finished and saved: 0. No camera: 1. Cannot save the output: 1. Broken path template: 1. Blend-file missing: 1. Bad Cycles device: 1. An animation that lost its output folder partway: 1. A render with a missing image texture, which reported an error and finished anyway: 0.

**Which handlers fire when a render fails.** A script was loaded with `--python` that attached a printing handler to each of `render_init`, `render_pre`, `render_post`, `render_write`, `render_complete`, `render_cancel` and `render_stats`, and printed a line naming any that did not exist. All seven existed. On a render that succeeded the order was `render_init`, `render_pre`, a long run of `render_stats`, then:

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

So this failed render — the no-camera file — fires `render_cancel`, and does not fire `render_complete`, `render_post` or `render_write`. (Citation check 2026-08-05: this used to say "a failed render". Only the no-camera failure was probed this way, so the sentence was narrowed to it.)

**Whether reports can be read from Python with no window.** A script run after the failing render printed:

```
REPORTS: attribute not present in this version
```

That line is what the script prints when `bpy.context.window_manager` has no `reports` attribute. Blender 5.0 has no such list.

**Observed, 2026-08-05 — where a failed render is announced with the window open, and what it says.** Run by the Creator on their own MacBook Pro (Apple M1 Pro, macOS 26.5.2), Blender 5.0.1, reported as screenshots. In both cases the Info editor was on screen with its error and warning filters on.

**A scene with no camera, rendered with F12.** The message appeared in **three places at once**, each carrying a round white-on-red cross icon, and reading the same words:

- along the bottom of the window, on a red field: `No camera found in scene "Scene"`
- in a floating box in the middle of the screen headed `Report: Error`, below which: `No camera found in scene "Scene"`
- as an entry in the Info editor: `No camera found in scene "Scene"`

**That is not the wording the same failure prints with no window.** Headless, the same file printed `reports          | ERROR Cannot render, no camera`. In the window it reads `No camera found in scene "Scene"`. Both were got from the same prepared file on the same machine on the same day.

**A broken template in the output path, rendered with Ctrl-F12.** Again in three places, with the same icon:

- along the bottom of the window: `Parse errors in path '//out_{blend_name.png': - Invalid or incomplete template expression '{blend_name.png'.`
- in a floating box headed `Report: Error`, laid out on two lines: `Parse errors in path '//out_{blend_name.png':` then, indented, `- Invalid or incomplete template expression '{blend_name.png'.`
- in the Info editor, on two lines, the same words.

Here the window and the terminal agree on the wording: headless, the same failure printed `reports          | ERROR Parse errors in path '…':` with its second line indented under a blank timestamp. The path in the Creator's file was `//out_{blend_name.png`, which is not the path used in the headless run, so the quoted path differs while the wording does not.

**A gap in what was reported.** The screenshots do not show whether either message stays in the Info editor or disappears. That was not captured and is not filled in here.

**Observed, 2026-08-06 — the same questions asked on a second machine.**

Run on **a rented Linux machine — Ubuntu 22.04.5 LTS, NVIDIA GeForce RTX 4090, driver 580.105.08, twelve cores, run over SSH**. Blender Blender 5.0.1, `-b`, its own settings folder, display turned off first. The Mac and this machine were both run and are reported separately.

**The wordings agreed, word for word.** Every failure text quoted above came back identically on Linux, and so did the exit statuses: 1 on failure, 0 on success. Two examples are quoted under question 1.

**What the render operator hands a script — this was inferred above and is now seen.** A script was loaded with `--python` that called `bpy.ops.render.render(write_still=True)` inside a `try`. On the good file it printed:

```
RENDER RETURNED: {'FINISHED'}
```

On the no-camera file it printed:

```
RENDER RAISED: RuntimeError Error: Cannot render, no camera
```

So the general `bpy.ops` behaviour does hold for the render operator: a failing render raises a `RuntimeError` carrying the message. The same two runs with Blender's window open gave the same two results, except that the raised message read `Error: No camera found in scene "Scene"` — the window's wording, not the terminal's. The failing runs with no window ended at status 1 even though the script caught the exception; the succeeding ones ended at 0.

**No readable list of reports, with a window or without.** The same script printed `REPORTS: attribute not present in this version` on this machine too, both with no window and with the window open. Blender 5.0 has no such list either way.

**The window wording is not a macOS oddity.** The no-camera file was rendered again with Blender's window open on the Linux machine, the render started from a script on a timer. The message raised read `Error: No camera found in scene "Scene"`, which is the same wording the Creator saw on their Mac, and not the `Cannot render, no camera` that the same file prints with no window. Both machines were run and both agreed.

**A gap in that Linux window run, written in rather than filled.** An Info editor was put on screen with its error, warning, information and operator filters on, and the window was photographed shortly after the failure and again twenty seconds later. **No entry appeared in the Info editor at all**, in either picture. The Creator's Mac run started the render by pressing F12; this one started it from a script. The two differ in how the render was started as well as in the machine, so nothing here says whether an Info-editor entry stays or fades, and that question is still open.

**What `--log-file` does to a failure message — new, and it matters.** Run as `-b no_camera.blend --log-file "<path>" -f 1`, with no `--log` category named, Blender 5.0 **moved the whole log into the file**, failure and all. The whole of standard output was:

```
Blender 5.0.1 (hash a3db93c5b259 built 2025-12-16 01:30:59)

Blender quit
```

and the file held:

```
00:00.254  blend            | Read blend: "/root/smoozy/Reports/work/no_camera.blend"
00:00.261  render           | Rendering single frame
00:00.261  render           | Rendering frame 1
00:00.261  reports          | ERROR Cannot render, no camera
```

The exit status was still 1. So a caller watching standard output while a log file is asked for sees no failure message at all. (Blender 4.5 does the opposite — it creates the file, leaves it empty, and still prints everything to standard output.)

**A warning about standard error on Linux.** On this machine the shell arrives with `DISPLAY` set to `:0` and no authority file. Run that way with no window, Blender 5.2 wrote four lines to standard error on a completely successful render; 4.5, 5.0 and 5.1 wrote nothing. Those words come from the display libraries rather than from Blender, and every run recorded here had the display turned off first. The point that carries: on a Linux machine with a display running, standard error is not reliably empty even when nothing has gone wrong.

**Outside the interface.** Settled by the tests above for the cases run. With no window, a failed render's message is printed on standard output through the log, under the `reports`, `image` or `cycles` category, and the process is left with status 1 — except a missing texture, which reports an error and still leaves 0. On the one failure probed for it, `render_cancel` fires and `render_complete` does not. There is no readable list of reports in this version. The status bar and the Info editor remain interface-only by their own description; the underlying report is plainly still generated, since it appears in the log. `--log-file` is documented as "Set a file to output the log to"; whether that holds with the window open was not tested and the source does not say. (Citation check 2026-08-05: the last sentence used to read "`--log-file` writes to a file regardless", which no quote or test here carries.)

**Operating system.** The status bar, Info editor and log are described without any operating-system qualification. One console detail is OS-specific: the manual's Windows launch page says the console window appears and is then toggled off, and is brought back with "Window ‣ Toggle System Console", and `-con, --start-console` is documented as "(Windows only)"; on macOS the manual says you must run the executable inside the `.app` bundle "to get all output printed to the terminal".

**Both machines have now been run and they agree on the words.** Every failure message quoted here came back identically on macOS 26.5.2 and on the rented Linux machine (Ubuntu 22.04.5 LTS, NVIDIA GeForce RTX 4090). The window's own wording for the no-camera failure, `No camera found in scene "Scene"`, was seen on both. **Windows is untested and no Windows machine has run it**, so the separate Command Prompt window, `Window ‣ Toggle System Console` and `-con, --start-console` remain unchecked. The three places a failure is announced — the Status Bar, a floating report box and the Info editor — were seen on macOS only; the Linux window run produced no Info-editor entry, for the reason written above.

---

## 3. What does Blender report about how far through a piece of work it has got?

**Label: documented.**

**In the window.** Manual, Status Bar, under "Status Messages":

> "Running Task
>
> Shows the progress of the currently running task (such as rendering or baking). Hovering the mouse pointer over the progress bar will display a time estimate. The task can be aborted by clicking the cancel button ()."

Address: https://docs.blender.org/manual/en/5.0/interface/window_system/status_bar.html — date read 2026-07-30.

The manual's own picture of that part of the status bar, `_images/interface_window-system_status-bar_ui-center.png`, shows the whole thing: an icon, the label "Sequence Render", a bar filled about nine-tenths blue with "94%" written inside it, and an X button at the right. So the progress is reported as a named task, a filled bar and a whole-number percentage — **documented**, from that picture, in the manual at the address above, date read 2026-07-30.

**The numbers behind it.** Python API, RenderEngine:

> "update_progress(progress)
>
> Update progress percentage of render"

with the parameter given as

> "progress (float in [0, 1]) – Percentage of render that's done"

and, separately,

> "update_stats(stats, info)
>
> Update and signal to redraw render status text"

Address: https://docs.blender.org/api/5.0/bpy.types.RenderEngine.html — date read 2026-07-30. So the render engine can supply one progress value between nought and one, alongside a free-text status line. The source does not define what span of work that value covers. The generic job equivalents are `progress_begin(min, max)` / `progress_update(value)` / `progress_end()` on the window manager (https://docs.blender.org/api/5.0/bpy.types.WindowManager.html) and the drawing primitive `progress(..., factor=0.0, type='BAR')` with "factor (float in [0, 1], (optional)) – Factor, Amount of progress from 0.0f to 1.0f" and "type (enum in ['BAR', 'RING'])" (https://docs.blender.org/api/5.0/bpy.types.UILayout.html) — both date read 2026-07-30.

**Outside the window.** The release notes' worked example of 5.0 log output contains the progress lines for a render:

```
00:00.244  render           | Rendering single frame (frame 1)
00:00.245  render           | Start rendering: Scene, ViewLayer
00:00.245  render           | Engine: Cycles
00:26.799  render           | Mem: 2983M | Sample 1/16 
00:26.799  render           | Mem: 2983M | Sample 16/16
00:28.631  render           | Saved: '/tmp/0001.png'
00:28.631  render           | Time: 00:28.38 (Saving: 00:00.15)
```

Source: release notes, Core, "Logging". Address: https://developer.blender.org/docs/release_notes/5.0/core/ — date read 2026-07-30. So out here the progress information shown is which frame is being rendered, which scene and view layer, and a sample count of the form "Sample 1/16". The same example also reports the saved file and finishing time.

The release notes also state that this changed in 5.0 and that it matters to anyone reading it:

> "The format of logging output has changed, including background render progress. Add-ons or render farms parsing this output may need to be updated."

Address: https://developer.blender.org/docs/release_notes/5.0/python_api/ — date read 2026-07-30.

And in the 5.0.1 corrective release, listed among the bug fixes:

> "Show currently rendered frame in background mode. (blender/blender#150174)"

Address: https://developer.blender.org/docs/release_notes/5.0/corrective_releases/ — date read 2026-07-30.

**Not covered.** How often the progress figure is updated; whether the percentage covers a whole animation or only the current frame (the API text says "Percentage of render that's done" without saying which); what the status text says during the stages before sampling begins; whether the log emits a line for every sample or only some; and whether the sample line is Cycles-specific — the example is a Cycles render, and the sources do not say what EEVEE or Workbench put there.

**Observed, 2026-08-05 — what progress a render prints with no window.**

Same computer and same way of running as in question 1. Blender 5.0.1, `-b`, own settings folder, Cycles, 16 samples, 160 by 120 pixels, output to PNG.

**A single frame.** Run: `-b "simple.blend" -o "<folder>/frame_" -F PNG -f 1`. Standard output ran to 53 lines, standard error zero bytes. The run in full, the first six lines and the last six:

```
00:00.404  blend            | Read blend: "/private/tmp/.../001/simple.blend"
00:00.408  render           | Rendering single frame
00:00.408  render           | Rendering frame 1
00:00.409  render           | Start rendering: Scene, ViewLayer
00:00.409  render           | Engine: Cycles
00:00.421  render           | Fra: 1 | Mem: 0M | Synchronizing object | Cube
```

```
00:00.427  render           | Fra: 1 | Mem: 1M | Sample 0/16
00:00.430  render           | Fra: 1 | Remaining: 00:00.05 | Mem: 2M | Sample 1/16
00:00.589  render           | Fra: 1 | Mem: 2M | Sample 16/16
00:00.589  render           | Fra: 1 | Mem: 2M | Finished
00:00.645  render           | Saved: '/private/tmp/.../001/out_50/frame_0001.png'
00:00.645  render           | Time: 00:00.23 (Saving: 00:00.05)
```

This matches the shape the release notes give, with one difference from the example printed there: the observed lines carry `Fra: 1` before the memory figure, and the release-notes example does not.

**A three-frame animation.** Run: `-b "simple.blend" -o "<folder>/frame_" -F PNG -x 1 -a`. Exit status 0, three files written, 147 lines. The frame boundaries are announced:

```
00:00.271  render           | Rendering animation (frames 1..3)
00:00.271  render           | Rendering frame 1
00:00.415  render           | Rendering frame 2
00:00.500  render           | Rendering frame 3
```

No line anywhere gave a figure for the animation as a whole. Each frame ran its own sequence from `Sample 0/16` to `Sample 16/16`, `Finished`, `Saved:` and `Time:`.

**What the render-statistics handler is handed.** A script attached a handler to `bpy.app.handlers.render_stats` that printed the arguments with `repr`. It fired 130 times over the three-frame animation. Every string it received, at each of the three frames, was the same set — the frame number is **not** in it:

```
RENDER_STATS: ('Remaining: 00:00.01 | Mem: 2M | Sample 1/16', None)
RENDER_STATS: ('Mem: 2M | Sample 16/16', None)
RENDER_STATS: ('Mem: 2M | Finished', None)
RENDER_STATS: ('Time: 00:00.14 (Saving: 00:00.04)', None)
```

So the handler gets the message part of each log line without the timestamp, the category or the `Fra: N` field. It is handed one string plus a second value which printed as `None`, and it fires with no window open.

**A non-Cycles engine.** The same file with `bpy.context.scene.render.engine` set to `BLENDER_EEVEE` printed 14 lines in all. (Citation check 2026-08-05: this used to add "the only EEVEE identifier this version offers". Nothing in the record says how that was established, so it was removed.) The render part:

```
00:00.394  render           | Engine: EEVEE
00:00.552  render           | Fra: 1 | Rendering 1 / 64 samples
00:00.715  render           | Fra: 1 | Rendering 25 / 64 samples
00:00.819  render           | Fra: 1 | Rendering 50 / 64 samples
00:00.878  render           | Fra: 1 | Rendering 64 / 64 samples
```

No memory field, no remaining time, and no per-stage lines.

**Observed, 2026-08-05 — what the window shows while a render runs.** Run by the Creator on their own MacBook Pro (Apple M1 Pro, macOS 26.5.2), Blender 5.0.1, reported as screenshots. The scene was Cycles at 1280 by 720 with 1024 samples and a frame range of 1 to 6.

**The Status Bar.** The task is named `Rendering...`, followed by a bar with a whole-number percentage on it and an `✕` button at its right. Percentages seen across the shots: `0%`, `23%`, `25%`.

**Whether the percentage covers a frame or the whole range.** It tracks the samples within a frame. In the animation shots the bar read `25%` while the render window read `Sample 256/1024` for Frame:2 — a quarter of that frame's samples, not a quarter of six frames' worth. (Citation check 2026-08-05: this used to end "It did not climb across the six frames." The three percentages recorded are `0%`, `23%` and `25%`, none of them from past frame 2, so nothing here shows what happened over the remaining frames.)

**The Blender Render window's own status line.** One line at the top left of that window. Quoted from the shots, word for word:

```
Frame:1 | Time:00:01.22 | Mem: 1433M | 3 / 75 render kernels loaded (may take a few minutes the first time)
```

```
Frame:1 | Time:00:14.04 | Remaining: 00:20.76 | Mem: 1482M | Sample 244/1024 (2 kernels to optimize)
```

```
Frame:1 | Time:00:18.05 | Remaining: 00:16.87 | Mem: 1482M | Sample 393/1024 (Using optimized kernels)
```

```
Frame:2 | Last:00:25.90 Time:00:06.04 | Remaining: 00:17.69 | Mem: 1482M | Sample 256/1024 (Using optimized kernels)
```

So the fields are: frame number; sometimes the previous frame's time as `Last:`; elapsed `Time:`; sometimes `Remaining:`; a memory figure; and then a sample count or a stage. The bracketed note after the sample count changed between shots through `render kernels loaded (may take a few minutes the first time)`, `(2 kernels to optimize)` and `(Using optimized kernels)`. That is what the pictures show.

**Observed, 2026-08-06 — the same runs on a second machine.**

Run on **a rented Linux machine — Ubuntu 22.04.5 LTS, NVIDIA GeForce RTX 4090, driver 580.105.08, twelve cores, run over SSH**. Blender Blender 5.0.1, `-b`, its own settings folder, display turned off first. Both machines were run and both agreed on everything below.

**The printed lines are the same shape and the same count.** A single-frame render ran to 53 lines on standard output and nothing on standard error — the same count as the Mac. The closing lines, word for word:

```
00:00.216  render           | Fra: 1 | Mem: 1M | Sample 0/16
00:00.218  render           | Fra: 1 | Remaining: 00:00.03 | Mem: 2M | Sample 1/16
00:00.327  render           | Fra: 1 | Mem: 2M | Sample 16/16
00:00.327  render           | Fra: 1 | Mem: 2M | Finished
00:00.412  render           | Saved: '/root/smoozy/Reports/work/L01/out_50/frame_0001.png'
00:00.412  render           | Time: 00:00.20 (Saving: 00:00.08)
```

**The three-frame animation behaved the same.** 148 lines, three files, exit 0. It opened with `render | Rendering animation (frames 1..3)` and named each frame as it began — `Rendering frame 1`, `Rendering frame 2`, `Rendering frame 3` — and no line gave any figure for the range as a whole.

**Which frame a progress line belongs to.** The printed lines carry the frame number even though the `render_stats` string does not. From the animation:

```
00:00.231  render           | Fra: 1 | Remaining: 00:00.02 | Mem: 2M | Sample 1/16
00:00.445  render           | Fra: 2 | Remaining: 00:00.22 | Mem: 2M | Sample 1/16
00:00.577  render           | Fra: 3 | Remaining: 00:00.01 | Mem: 2M | Sample 1/16
```

**What the statistics handler is handed — the same as on the Mac.** It fired 131 times over three frames (130 on the Mac) and received the message part only, with no timestamp, no category and no frame number:

```
RENDER_STATS: ('Remaining: 00:00.02 | Mem: 2M | Sample 1/16', None)
RENDER_STATS: ('Mem: 2M | Sample 16/16', None)
RENDER_STATS: ('Mem: 2M | Finished', None)
RENDER_STATS: ('Time: 00:00.11 (Saving: 00:00.00)', None)
```

**The engine name — this settles a question no source answered.** The non-Cycles run used `BLENDER_EEVEE`. Asked to accept the older name, this version refused it:

```
ENGINE RAISED: TypeError bpy_struct: item.attr = val: enum "BLENDER_EEVEE_NEXT" not found in ('BLENDER_EEVEE', 'BLENDER_WORKBENCH', 'CYCLES')
```

Blender 4.5 does the opposite — it offers `BLENDER_EEVEE_NEXT` and refuses `BLENDER_EEVEE`. So the identifier changed after 4.5.

**Whether a program can read the progress as it happens.** A two-frame render was piped into a reader that stamped each line with the seconds since the run began, and those stamps were set beside the elapsed time Blender itself writes inside each line. The lines arrived spread across the run rather than in one lump at the end: in this version the line Blender stamped `00:06.044` reached the reader at 6.14 seconds, and the line stamped `00:11.931` reached it at 12.03 seconds. Worth knowing beside it: in a background render Cycles printed only three sample lines per frame — `Sample 0/2000`, `Sample 1/2000` and the last one — so there is little to read between them.

**Outside the interface.** Settled by the tests above. With no window, Blender 5.0 prints timestamped log lines under the `render` category, each carrying the frame number, a memory figure, sometimes a remaining time, and the sample count or stage. It names the frame range once at the start of an animation and names each frame as it begins. What it does not carry is any percentage, or any figure covering the whole animation — the sample count is always within one frame. The `render_stats` handler fires and receives the message part of each line, without the frame number.

**Operating system.** No operating-system qualification appears on any of this. The example output in the release notes happens to be from a macOS machine (it names "Create Cocoa system" and an Apple GPU earlier in the same block), but the `render` lines are not marked as OS-specific anywhere. The Creator's window tests were got on macOS 26.5.2 only. The printed lines were got on Linux as well, and the paragraph below says so. (Citation check 2026-08-06: this sentence used to say all of it was macOS-only and that neither other system could be tested, which the Linux run recorded below contradicts. Narrowed to the window tests.)

The printed progress was got on **both** machines — the Creator's MacBook Pro (Apple M1 Pro, macOS 26.5.2) and the rented Linux machine — and the two agreed on the line shape, the fields, the frame announcements and the counts, so it holds on macOS and on Linux. **Windows is untested and no Windows machine has run it.** **Untried rather than untestable: a Windows machine can be rented, and every one this project has read reported a Server edition — see the equipment section of `Preparation/Blender Research/Method.md`.** The Creator's window tests — the Status Bar task name, the percentage and the Blender Render window's status line — were got on macOS only.

---

## 4. What does Blender write out as it works, what is that stream called, and what does it contain?

**Label: documented** for what it writes and what it contains; **not in the sources** for the name of the stream, except in one narrow case.

**What it writes, and where it goes.** Manual, Using Blender From The Command Line:

> "The Console Window, also called a Terminal, is an operating system text window that displays messages about Blender's operations, status, and internal errors."

> "When Blender is manually started from a terminal, Blender output is shown in the Console Window it is started from."

Address: https://docs.blender.org/manual/en/5.0/advanced/command_line/index.html — date read 2026-07-30.

**What it is called.** In 5.0 the thing that produces most of it has a name — the logging system. Release notes, Core:

> "Most of Blender now uses the same logging system, including operators reports and background render progress. The output format has changed, and log levels are easier to control."

Address: https://developer.blender.org/docs/release_notes/5.0/core/ — date read 2026-07-30.

**What it contains.** The release notes give a worked example, which is the only complete specimen of 5.0's output in any of the sources:

```
00:00.027  ghost.system     | Create Cocoa system
00:00.080  translation      | Locale en_NL used for translation
00:00.088  blend            | Read prefs: "/Users/blender/Library/Application Support/Blender/5.0/config/userpref.blend"
00:00.096  blend            | Read startup: "/Users/blender/Library/Application Support/Blender/5.0/config/startup.blend"
00:00.359  gpu.platform     | Using GPU "Apple M3 Max Metal API 1.2"
00:00.359  gpu.platform     | Using Backend "Metal"
00:01.041  cycles           | Added device "Apple M3 Max (GPU - 40 cores)" with id "METAL_Apple M3 Max (GPU - 40 cores)".
...
00:00.244  render           | Rendering single frame (frame 1)
00:00.245  render           | Start rendering: Scene, ViewLayer
00:00.245  render           | Engine: Cycles
00:26.799  render           | Mem: 2983M | Sample 1/16 
00:26.799  render           | Mem: 2983M | Sample 16/16
00:28.631  render           | Saved: '/tmp/0001.png'
00:28.631  render           | Time: 00:28.38 (Saving: 00:00.15)
```

Same address, same date read. Reading only what is on the page: each line is a timestamp, then a category name, then a vertical bar, then the message. Categories in the example include `ghost.system`, `translation`, `blend`, `gpu.platform`, `cycles` and `render`. The render-related content is: which frame, which scene and view layer, which engine, a memory figure and a sample count per progress line, the path each frame was saved to, and total time with saving time broken out.

**How much of it there is, and where it can be sent.** Manual, Logging Options:

> "--log <match>
>
> Enable logging categories, taking a single comma separated argument."
>
> "--log "*": everything
> --log "event": every category starting with event.
> --log "render,cycles": both render and cycles messages."

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

> "--log-file <filepath>
>
> Set a file to output the log to."

> "--log-list-categories
>
> List all available logging categories for --log, and exit."

Address: https://docs.blender.org/manual/en/5.0/advanced/command_line/arguments.html — date read 2026-07-30. The release notes add the guidance that `--log-level info` "may be useful to enable in production for troubleshooting issues" (Core release notes, address above).

**The one place a stream is named — observed.** Cycles' own add-on code declares its statistics switch like this:

```
    parser.add_argument("--cycles-print-stats",
                        help="Print rendering statistics to stderr",
                        action='store_true')
```

File: `Blender Sources/cycles_5.0/engine.py`, lines 13–15, inside `_configure_argument_parser()`. Date read 2026-07-30. **Label for this specific claim: observed** — it is the text of a file I was given, not documentation. Note what it does and does not establish: the file says the help text says "stderr". That the statistics actually land on standard error when the switch is used is one step further, and that step is **inferred**, not observed.

The manual describes the same switch differently:

> "--cycles-print-stats
>
> Log statistics about render memory and time usage."

Address: https://docs.blender.org/manual/en/5.0/advanced/command_line/arguments.html — date read 2026-07-30.

**A second description of the statistics' content.** Python API, application handlers:

> "bpy.app.handlers.render_stats
>
> on printing render statistics. Accepts one argument: the render stats (render/saving time plus in background mode frame/used [peak] memory)."

Address: https://docs.blender.org/api/5.0/bpy.app.handlers.html — date read 2026-07-30.

**Not covered.** Whether Blender's general output goes to standard output or standard error is nowhere stated — the sources say "Console Window", "terminal", "output" and "log", never "stdout". The full list of logging categories is not in the sources; the manual says `--log-list-categories` prints it at runtime. Nothing says whether `--log-file` suppresses console output or duplicates it, whether lines are flushed as they are produced, what character encoding is used, or whether the format is stable within 5.x.

**Observed, 2026-08-05 — which stream carries what, and what the switches do.**

Same computer and same way of running as in question 1. Blender 5.0.1, `-b`, own settings folder. Every run sent its two streams to separate files.

**Which stream.** On a plain successful single-frame render, standard output held all 53 lines and standard error held nothing — zero bytes. The same was true of the three-frame animation, the EEVEE render, the render with a missing texture, and every one of the failure runs in question 1. The only thing ever seen on standard error in any 5.0 run was a Python traceback from a `--python-expr` that named a setting this version does not have.

**Where the Cycles statistics land.** Run: `-b "simple.blend" -o "<folder>/frame_" -F PNG -f 1 -- --cycles-print-stats`. Standard error was zero bytes. The statistics appeared on **standard output**, plainly printed rather than as log lines — no timestamp, no category, no vertical bar. They begin:

```
Render statistics:
Mesh statistics:
  Geometry:
    Total memory: 579 (579)
        Cube                             417 (417)
        Light                            81 (81)
        light                            81 (81)
Image statistics:
  Textures:
    Total memory: 0 (0)
Kernel statistics:
```

and end:

```
Shader statistics:
  Material                        : 0.03s (Relative cost: 1.00)
Object statistics:
  Cube                            : 0.03s (Relative cost: 1.00)
```

The whole run was 129 lines on standard output and nothing on standard error. So the help text in `engine.py` naming stderr does not describe where the statistics actually appeared here.

**What quiet mode removes.** Run: `-b -q "simple.blend" -o "<folder>/frame_" -F PNG -f 1`. Exit status 0, the frame was written, and standard output still held 50 lines — the whole render log, from `Rendering single frame` through every sample line to `Time: 00:00.14 (Saving: 00:00.04)`. The `blend | Read blend: "…"` line at the top was gone. Standard error was empty. So in 5.0, `-q` does not silence the render progress. (Citation check 2026-08-05: this used to say the `Read blend` line was the *only* line that disappeared. The plain run was recorded at 53 lines and this one at 50, and no comparison of the two outputs was kept, so that does not follow from what is written down.)

**The list of logging categories.** Run: `--log-list-categories`. Exit status 0, standard error empty, 135 category names on standard output, one per line. The first forty were:

```
addon
anim
anim.action
anim.data
anim.driver
anim.fcurve
anim.fmodifier
anim.keyingset
anim.motion_paths
anim.nla
asset.catalog
asset.index
asset.library
blend
blend.doversion
blend.link
blend.partial_write
blend.readfile
blend.validate
blend.writefile
bpy.interface
bpy.rna
color_management
compositor.gpu
context
cycles
depsgraph
eevee
event
geom.armature_deform
geom.attribute
geom.bmesh.convert
geom.customdata
geom.gpencil
geom.mesh
geom.mesh.convert
geom.vfont
geom.volume
ghost.ndof
```

`render` is one of the 135.

**What a render logs at information level.** Run: `-b "simple.blend" --log "*" --log-level info -o ... -f 1`. Accepted, exit status 0, standard error empty, 148 lines. The `render` category lines are the same progress lines already quoted in question 3, so the render progress is part of the log rather than printed beside it.

**What the built-in help lists.** Run: `--help`, 466 lines on standard output, nothing on standard error. Its whole Logging Options section:

```
Logging Options:
--log <match>
	Enable logging categories, taking a single comma separated argument.

	--log "*": log everything
	--log "event": logs every category starting with 'event'.
	--log "render,cycles": log both render and cycles messages.
	--log "*mesh*": log every category containing 'mesh' sub-string.
	--log "*,^operator": log everything except operators, with '^prefix' to exclude.

--log-level <level>
	Set the logging verbosity level.

	fatal: Fatal errors only
	error: Errors only
	warning: Warnings
	info: Information about devices, files, configuration, operations
	debug: Verbose messages for developers
	trace: Very verbose code execution tracing

--log-show-memory 
	Show memory usage for each log message.

--log-show-source 
	Show source file and function name in output.

--log-show-backtrace 
	Show a back trace for each log message (debug builds only).

--log-file <filepath>
	Set a file to output the log to.

--log-list-categories 
	List all available logging categories for '--log', and exit.
```

and the quiet entry, quoted whole:

```
-q or --quiet 
	Suppress status printing (warnings & errors are still printed).
```

**Observed, 2026-08-06 — the same switches on a second machine, and three things the Mac never asked.**

Run on **a rented Linux machine — Ubuntu 22.04.5 LTS, NVIDIA GeForce RTX 4090, driver 580.105.08, twelve cores, run over SSH**. Blender Blender 5.0.1, `-b`, its own settings folder, display turned off first. Both machines were run.

**Which stream, and where the statistics sit — agreed, and now placed.** Standard output carried everything and standard error was empty on every ordinary run, exactly as on the Mac. With `-- --cycles-print-stats` the statistics appeared on **standard output** again, 129 lines in all, so the help text in `engine.py` naming stderr does not describe where they arrive on either machine. **Where they sit is now settled for this version:** the block opens with `Render statistics:` **after** the closing `render | Time: …` line, at the very end of the run. (Blender 4.5 puts it earlier, between `Finished` and `Saved:`.)

**Quiet mode — agreed, and the missing two lines are now named.** `-b -q` left 50 lines on standard output against 53 without it, the same counts as the Mac, and the whole render log survived. The two runs were then compared with their timestamps stripped. **The three lines `-q` removes are:**

```
blend            | Read blend: "/root/smoozy/Reports/work/simple.blend"
```

a blank line, and

```
Blender quit
```

That is the thing all three of these documents were corrected to say was unknown.

**The logging switches — the same shape, a longer list.** `--log-list-categories` was accepted and ended at 0. It listed **169** categories here against **135** on the Mac. The Linux list carries a run of `ghost.wl.*` window-system names the Mac's would have no use for. Which exact names differ cannot be worked out, because the Mac's list was kept only as a count. The list is the same whether Blender is started with `-b` or with a window. `--log "*" --log-level info` was accepted, produced 146 lines, and the only categories that appeared on a render were `blend`, `cycles`, `render` and `translation`.

**What `--log-file` catches — new, and nobody had run it.** Run as `-b simple.blend --log-file "<path>" -o … -f 1`, with no `--log` category named, this version **moved the whole render log into the file**. Standard output dropped to three lines — the version banner, a blank line and `Blender quit` — and the file held 50 lines, from `blend | Read blend:` through every sample line to the closing `render | Time:`. On a failing render the failure message goes with it; that is recorded under question 2. **Blender 4.5 does the opposite**: it creates the file, leaves it empty, and still prints everything to standard output.

**Where `--debug-cycles` writes — new.** Run as `-b simple.blend --debug-cycles -o … -f 1`, the debug material went to **standard output**, as ordinary log lines under a `cycles` category. Standard error stayed empty. Some of what it carries:

```
00:00.228  cycles           | CUEW initialization succeeded
00:00.301  cycles           | Added device "NVIDIA GeForce RTX 4090" with id "CUDA_NVIDIA GeForce RTX 4090_0000:00:07".
00:00.303  cycles           | WARNING HIPEW initialization failed: Error opening HIP dynamic library
```

**Blender 4.5 puts the same material on standard error instead**, in a different shape entirely. So which stream carries the Cycles debug output changed after 4.5, and a caller that reads only one stream will see it in one version and not the other.

**The built-in help — agreed on content, longer on this machine.** `--help` gave 479 lines on Linux against 466 on the Mac, and the logging entries it lists are the same seven: `--log`, `--log-level`, `--log-show-memory`, `--log-show-source`, `--log-show-backtrace`, `--log-file`, `--log-list-categories`. The quiet entry reads word for word the same on both machines and in all four versions:

```
-q or --quiet 
	Suppress status printing (warnings & errors are still printed).
```

**Outside the interface.** This is chiefly an outside-the-interface answer already, and every test above was run that way. Everything Blender wrote went to standard output; standard error stayed empty. The release notes tie the console and the log together explicitly ("including background render progress") and warn that "Add-ons or render farms parsing this output may need to be updated". The sources do not say how the same output behaves when Blender is running with its own window open, beyond the Windows console note below.

**Operating system.** The output itself is not qualified by operating system. Getting at it is: on Windows the manual says the console window appears at startup and is then toggled off, reachable again through "Window ‣ Toggle System Console", with `-con, --start-console` documented "(Windows only)"; on macOS the manual says you must run `./Blender.app/Contents/MacOS/Blender` "to get all output printed to the terminal"; on Linux it is simply the terminal you started from. The example log block in the release notes is from a macOS machine.

**Both machines have now been run.** They agree on which stream carries what, on quiet mode, on the logging switches and on the shape of a log line. Two differences belong to the machine rather than to Blender: the category list is longer on Linux, and so is the built-in help. One belongs to the display rather than to Blender: on a Linux machine with a display running, a windowless Blender may have display-library complaints appear on its standard error, which vanish once the display is turned off. **Windows is untested and no Windows machine has run it**, so the separate Command Prompt window and `-con, --start-console` stay unchecked.

---

## 5. Does Blender estimate the time left, for what unit of work, and how does it describe that estimate?

**Label: documented** that an estimate exists and where; **not in the sources** for how it is worded, or for any estimate outside the window.

**That it exists.** Manual, Status Bar, under "Running Task":

> "Shows the progress of the currently running task (such as rendering or baking). Hovering the mouse pointer over the progress bar will display a time estimate."

Address: https://docs.blender.org/manual/en/5.0/interface/window_system/status_bar.html — date read 2026-07-30.

**For what unit of work.** By the sentence's own subject, the estimate belongs to "the currently running task", the same thing the progress bar is showing — and the manual's examples of such a task are "rendering or baking". It is not scoped to a frame, a sample or an animation by any wording in the sources. The manual's picture of that region (`_images/interface_window-system_status-bar_ui-center.png`) shows one such task labelled "Sequence Render" at 94 per cent; the tooltip is not visible in the picture, so it shows nothing about the estimate's wording.

**How it describes the estimate.** Not documented. The manual says only "a time estimate". It does not say whether that is time remaining, time elapsed, a finishing clock time, or in what units or format.

**What the sources do give in the way of times — and these are not estimates.** The example log output in the release notes ends a render with:

```
00:28.631  render           | Time: 00:28.38 (Saving: 00:00.15)
```

That is a completed render's actual time with the saving time broken out, reported after the fact. Address: https://developer.blender.org/docs/release_notes/5.0/core/ — date read 2026-07-30. Every log line also carries an elapsed timestamp in its first column. In image metadata there is likewise a "Render Time" field which "Includes the render time" (https://docs.blender.org/manual/en/5.0/render/output/properties/metadata.html, date read 2026-07-30). None of these is a forecast.

**Where I looked.** I searched the whole manual, the whole API documentation, the whole release notes and Cycles' own files for: "estimate", "estimated", "time remaining", "remaining", "time left", "ETA", "elapsed". Across all four sources the only hit that bears on a forecast of time is the single status-bar sentence quoted above; "remaining" appears only in unrelated modelling and sequencer text, "elapsed" only in a Grease Pencil modifier and a geometry-node description, and there is no occurrence of "time remaining", "time left" or "ETA" anywhere. There is no property, method or handler in the Python API with a name or description about time left — `bpy.types.RenderEngine` offers `update_progress` and `update_stats` and nothing time-based.

**Not covered.** The wording, unit and format of the estimate; whether it is recalculated as work proceeds; how it behaves for an animation as against a single frame; whether it exists at all for tasks other than rendering and baking; and whether any estimate is available to a caller rather than to a mouse pointer.

**Observed, 2026-08-05 — Blender does print an estimate with no window.**

Same computer and same way of running as in question 1. Blender 5.0.1, `-b`, own settings folder, Cycles.

Some of the `render` log lines carry a field labelled `Remaining:`. It sits between the frame number and the memory figure. From a single-frame render at 16 samples:

```
00:00.430  render           | Fra: 1 | Remaining: 00:00.05 | Mem: 2M | Sample 1/16
```

From a slower graphics-card render of the same file, where it appeared on several lines running and fell each time:

```
00:00.688  render           | Fra: 1 | Remaining: 00:01.43 | Mem: 1M | Sample 1/16
00:00.691  render           | Fra: 1 | Remaining: 00:00.69 | Mem: 1M | Sample 2/16
00:00.695  render           | Fra: 1 | Remaining: 00:00.17 | Mem: 1M | Sample 6/16
```

The word is `Remaining:`, with a space after the colon, and the figure is in minutes, seconds and hundredths. It does not appear on every line: it was absent from the `Sample 0/16` line, from the final `Sample 16/16` line and from the `Finished` line.

**What it covers.** In the three-frame animation a `Remaining:` field appeared three times, once per frame, and read the same small figure each time. The three, from that one run — note that they are word for word identical, because the handler is not handed the frame number, so which frame each belongs to rests on the order they printed in and not on anything in the text:

```
RENDER_STATS: ('Remaining: 00:00.01 | Mem: 2M | Sample 1/16', None)
RENDER_STATS: ('Remaining: 00:00.01 | Mem: 2M | Sample 1/16', None)
RENDER_STATS: ('Remaining: 00:00.01 | Mem: 2M | Sample 1/16', None)
```

No line in the run gave a remaining figure for the animation as a whole. The EEVEE run printed no `Remaining:` field at all.

**Observed, 2026-08-05 — what the tooltip on the progress bar says.** Run by the Creator on their own MacBook Pro (Apple M1 Pro, macOS 26.5.2), Blender 5.0.1, reported as screenshots. The scene was Cycles at 1280 by 720 with 1024 samples and a frame range of 1 to 6.

Resting the pointer on the Status Bar progress bar brings up a box of **two lines**. Word for word:

```
Time Remaining: 00:35.11
Time Elapsed: 00:21.87.
```

So the wording is `Time Remaining: ` and `Time Elapsed: `, the figures are in minutes, seconds and hundredths, and the second line ends with a full stop. It gives both a forecast and a time already spent.

**What each figure sat beside.** At the moment that tooltip was showing, the Blender Render window's own status line read `Frame:1 | Time:00:18.05 | Remaining: 00:16.87`. Those are the numbers as they appeared; nothing further is read into them here.

**Observed, 2026-08-06 — the printed estimate on a second machine.**

Run on **a rented Linux machine — Ubuntu 22.04.5 LTS, NVIDIA GeForce RTX 4090, driver 580.105.08, twelve cores, run over SSH**. Blender Blender 5.0.1, `-b`, its own settings folder, display turned off first. It agreed with the Mac in every respect: the field is spelled `Remaining:` with one space after the colon, in minutes, seconds and hundredths, sitting between the frame number and the memory figure, and it does not appear on every line.

**What it covers, and which frame each figure belongs to.** In the three-frame animation a `Remaining:` field appeared once inside each frame and the value started over each time. The printed lines carry the frame number, so which frame each belongs to is plain — this is what the equivalent Mac record could not show, since the `render_stats` string strips it:

```
00:00.231  render           | Fra: 1 | Remaining: 00:00.02 | Mem: 2M | Sample 1/16
00:00.445  render           | Fra: 2 | Remaining: 00:00.22 | Mem: 2M | Sample 1/16
00:00.577  render           | Fra: 3 | Remaining: 00:00.01 | Mem: 2M | Sample 1/16
```

No line in the run gave a remaining figure for the animation as a whole, and the EEVEE run printed no `Remaining:` field at all. Both machines were run and both agreed.

**Outside the interface.** Settled by the test above, and it is the opposite of what the release-notes example suggested. An estimate is printed with no window: the `Remaining:` field on Cycles' `render` log lines. It restarts at each frame, so what it covers is the current frame, not the whole range. The sources describe only a tooltip on the Status Bar, which does need the window; that tooltip has since been photographed by the Creator and is quoted above, worded `Time Remaining:` with a companion `Time Elapsed:` line. (Citation check 2026-08-05: this used to send the tooltip off to a test still to be done. It had already been done and written into this same question, two paragraphs up.)

**Operating system.** The sources give no operating-system qualification on this, and I found nothing suggesting one. The Creator's tooltip screenshots were got on macOS 26.5.2 only. The printed `Remaining:` field was got on Linux as well, and the paragraph below says so. (Citation check 2026-08-06: this sentence used to say all of it was macOS-only and that neither other system could be tested, which the Linux run recorded below contradicts. Narrowed to the screenshots.)

The printed `Remaining:` field was got on **both** machines — the Creator's MacBook Pro (Apple M1 Pro, macOS 26.5.2) and the rented Linux machine — and they agreed on its spelling, its units, where it appears and what it covers, so it holds on macOS and on Linux. **Windows is untested and no Windows machine has run it.** The two-line tooltip was seen on macOS only.

---

## 6. What does Blender report about the memory it is using on a graphics card?

**Label: documented** that a graphics-memory readout exists as a status-bar option; **not in the sources** for what it says, and for any graphics-card memory figure during a render.

**The one thing in the sources that is explicitly about graphics-card memory.** Python API, `bpy.types.PreferencesView`:

> "show_statusbar_vram
>
> Show GPU video memory usage
>
> Type:
>
> boolean, default False"

Address: https://docs.blender.org/api/5.0/bpy.types.PreferencesView.html — date read 2026-07-30. So Blender has a switchable status-bar readout of graphics-card memory use, off by default. Nothing in the API says what the readout looks like or what it counts.

Note what sits beside it in the same list, because the distinction matters: `show_statusbar_memory` is described as "Show Blender memory usage" — a different figure from the graphics one.

**The manual does not describe it.** The manual's Status Bar page lists what the right-hand side can show — "Scene Statistics", "Scene Duration", "System Memory", "Extensions Updates", "Blender Version" — and the System Memory entry reads:

> "Shows an estimate of Blender's RAM consumption. On a single-instance single-machine scenario, this estimate provides a measurement against the hardware limit of the machine."

Address: https://docs.blender.org/manual/en/5.0/interface/window_system/status_bar.html — date read 2026-07-30. That is main memory, not graphics memory, and no graphics-memory item appears in that list, nor in the identical list on the Interface preferences page (https://docs.blender.org/manual/en/5.0/editors/preferences/interface.html, date read 2026-07-30). I also opened the manual's picture of the right-hand side, `_images/interface_window-system_status-bar_ui-right.png`: it reads `Collection | Cube | Verts:8 | Faces:6 | Tris:12 | Objects:0/3 | Mem: 29.7 MiB | v2.81.16`. So the picture shows the main-memory item and no graphics-memory item — but the version string in the picture is `v2.81.16`, so it is an old screenshot and settles nothing about 5.0's options.

**The memory figures that a render does report are not stated to be graphics-card memory.** Four of them, all careful not to say which memory they mean:

- The log line in the release notes' example: `00:26.799  render | Mem: 2983M | Sample 1/16`. Address: https://developer.blender.org/docs/release_notes/5.0/core/ — date read 2026-07-30. The notes do not say what "Mem" counts.
- Python API, RenderEngine: "update_memory_stats(*, memory_used=0.0, memory_peak=0.0) — Update memory usage statistics", with "memory_used (float in [0, inf], (optional)) – Current memory usage in megabytes" and "memory_peak (float in [0, inf], (optional)) – Peak memory usage in megabytes". Address: https://docs.blender.org/api/5.0/bpy.types.RenderEngine.html — date read 2026-07-30. Megabytes, current and peak, of unspecified memory.
- Python API, handlers: `render_stats` accepts "the render stats (render/saving time plus in background mode frame/used [peak] memory)". Address: https://docs.blender.org/api/5.0/bpy.app.handlers.html — date read 2026-07-30.
- In image metadata, the "Memory" option "Includes the peak memory usage" (https://docs.blender.org/manual/en/5.0/render/output/properties/metadata.html); the matching API property `use_stamp_memory` reads "Include the peak memory usage in image metadata", default True (https://docs.blender.org/api/5.0/bpy.types.RenderSettings.html) — both date read 2026-07-30. The 5.0.1 corrective release names this field's label in passing: "Metadata output for "Peak Memory" displays "ddM" in Blender 5.0. (blender/blender#150524)" (https://developer.blender.org/docs/release_notes/5.0/corrective_releases/, date read 2026-07-30).

Treating any of these as a graphics-card figure would be a guess, so I do not.

**What the sources say instead, which is close but is not reporting.** Manual, EEVEE Limitations:

> "There is no standard way of estimating if the resources will fit into the GPU memory and/or if the GPU will render them successfully."

Address: https://docs.blender.org/manual/en/5.0/render/eevee/limitations/limitations.html — date read 2026-07-30. And manual, Cycles GPU Rendering, on what happens rather than what is reported:

> "With CUDA, OptiX, HIP and Metal devices, if the GPU memory is full Blender will automatically try to use system memory."

> "Typically, the GPU can only use the amount of memory that is on the GPU"

Address: https://docs.blender.org/manual/en/5.0/render/cycles/gpu_rendering.html — date read 2026-07-30.

**Cycles' own files — observed, and it is a negative.** In `Blender Sources/cycles_5.0/properties.py` the only memory-named setting is `peer_memory`, at lines 1608–1611:

```
    peer_memory: BoolProperty(
        name="Distribute memory across devices",
        description="Make more room for large scenes to fit by distributing memory across interconnected devices (e.g. via NVLink) rather than duplicating it",
```

That is a setting, not a report. Date read 2026-07-30. **Observed**. This quote establishes only that this setting exists; it says nothing about whether Cycles reports a graphics-memory amount elsewhere.

**Where I looked.** Searched all four sources for: "VRAM", "video memory", "GPU memory", "device memory", "graphics memory", "memory usage", "peak", "peak memory", "Mem:". Read in full: the Status Bar manual page and its three pictures, the Interface preferences page's Status Bar section, the Cycles GPU Rendering page, the EEVEE Limitations page, the Metadata page, `bpy.types.PreferencesView`, `bpy.types.RenderEngine`, `bpy.app.handlers`, the memory-related parts of `bpy.types.RenderSettings`, and Cycles' `properties.py` and `ui.py`. The 5.0 release notes mention VRAM only in bug-fix lines about leaks and crashes, never as something reported.

**Not covered.** What the graphics-memory status-bar readout actually displays — units, whether it is Blender's own use or the whole card's, whether it shows a total or a limit, whether it updates during a render. Whether any graphics-card memory figure reaches the log, the metadata or a caller. Whether the figure differs by backend (CUDA, OptiX, HIP, oneAPI, Metal). And whether the fallback to system memory changes what is reported.

**Observed, 2026-08-05 — the setting exists, and nothing printed is marked as the card's memory.**

Same computer and same way of running as in question 1. Blender 5.0.1, `-b`, own settings folder.

**The setting exists and is off.** A script read the preference and printed:

```
preferences.view.show_statusbar_vram => False
preferences.view.show_statusbar_memory => False
```

so both status-bar memory switches are present in 5.0 and both start off. What the readout then shows needs the window and is entry 21 of `Audits/Blender Research/Reports/Hand tests.md`.

**A render on the graphics card.** A script turned the Metal device on and set the scene to render on it. It printed:

```
DEVICE: 'Apple M1 Pro' 'CPU' use = True
DEVICE: 'Apple M1 Pro (GPU - 14 cores)' 'METAL' use = True
SCENE DEVICE: GPU
```

The render then ran on that device and the log lines were the same shape as a processor render, with the same single `Mem:` field and no second one:

```
00:00.688  render           | Fra: 1 | Remaining: 00:01.43 | Mem: 1M | Sample 1/16
00:00.848  render           | Fra: 1 | Mem: 2M | Finished
```

No field, label or word anywhere in the run named the graphics card's memory. Searching the whole output for "VRAM", "video memory", "GPU memory" and "device memory" matched nothing.

**With the Cycles statistics switch added.** The same graphics-card render with `-- --cycles-print-stats` produced the statistics block quoted in question 4. Its memory figures are:

```
Mesh statistics:
  Geometry:
    Total memory: 579 (579)
Image statistics:
  Textures:
    Total memory: 0 (0)
```

Those are labelled by what the memory holds — geometry, textures — not by which device it is on. Nothing in the block names a card, a device or an amount of card memory.

**Observed, 2026-08-05 — the Status Bar offers no video-memory item.** Run by the Creator on their own MacBook Pro (Apple M1 Pro, macOS 26.5.2), Blender 5.0.1, reported as screenshots.

Right-clicking the Status Bar opens a menu headed `Status Bar` holding **exactly five items**, each with a tick box:

```
Scene Statistics
Scene Duration
System Memory
Extensions Updates
Blender Version
```

**No item mentions video memory or GPU memory.** That is the whole menu; nothing is cut off in the picture.

The Status Bar itself, in that shot, read `: 00:00+06 (Frame 1/6) | Memory: 302.3 MiB | 5.0.1`. So the one memory figure on offer there is labelled `Memory:` and given in MiB to one decimal place — and by the manual's own text that item is System Memory, not the graphics card. (The manual's own old screenshot of this bar shows the label as `Mem:`; what this version draws is `Memory:`.)

**A gap in what was reported.** The instruction also said that, if the right-click menu had no such item, to look in Edit ▸ Preferences ▸ Interface ▸ Status Bar instead. No screenshot of that panel was returned, and no screenshot shows the readout switched on or the text it produces. So what `show_statusbar_vram` displays is still not known for this version. It is not filled in here.

**Observed, 2026-08-06 — what the video-memory readout actually says.**

Run on **a rented Linux machine — Ubuntu 22.04.5 LTS, NVIDIA GeForce RTX 4090, driver 580.105.08, twelve cores, run over SSH**, with Blender's window open on that machine's own display, pointed at its own settings folder. A script set `bpy.context.preferences.view.show_statusbar_vram = True` and `show_statusbar_memory = True`, saved the preferences with `bpy.ops.wm.save_userpref()`, waited five seconds, photographed the window with `bpy.ops.screen.screenshot` and quit. It printed:

```
BEFORE vram = False  memory = False
AFTER  vram = True  memory = True
```

**The readout appeared, and this is what it says.** The right-hand end of the Status Bar in that picture reads, word for word:

```
Collection | Cube | Verts:8 | Faces:6 | Tris:12 | Objects:1/3 | Memory: 18.9 MiB | VRAM: 1.0/24.0 GiB | 5.0.1
```

*(Narrowed 2026-08-06 by the runner, after the check of the Linux hand tests sent this back. The check read the same bar again on a quiet card and got 1.2, 1.1, 1.1 and 1.2 in the four versions, and 1.7 and 1.8 on a busier one — so the left-hand figure is not fixed and must not be quoted as though it were. What the runs do carry, identically every time, is the label `VRAM:`, the used-of-total shape, the `GiB` units, and the right-hand figure of `24.0`, which is the whole of this card's memory. The left-hand figure moves with what is on the card, and how it is counted is still not settled — both the check's readings and this document's are on the record.)*


So the item is labelled **`VRAM:`**, and it is a **used-of-total** figure in **GiB**, to one decimal place, with a `/` between the two numbers. It sits between the system-memory item and the version. The right-hand figure, `24.0 GiB`, is this card's whole memory — `nvidia-smi` reports the card as 24564 MiB. The same words appeared in all four versions.

**Whether the left-hand figure is Blender's own use or the card's — not established here.** The moving test was run in Blender 5.2 rather than in this version: a heavy Cycles render on the card (4096 by 4096, 400 samples, CUDA), photographed five times, with `nvidia-smi` read at each shot. There the readout moved, reading `VRAM: 4.0/24.0 GiB` when `nvidia-smi` reported 3673 MiB used on the card and `VRAM: 3.8/24.0 GiB` when it reported 3459 MiB, and at rest `VRAM: 1.0/24.0 GiB` when the card's whole usage was 568 MiB. So it tracks the card's overall use and is not equal to it. **What the left-hand figure counts is therefore not settled** — it is neither the card's reported total nor plainly Blender's own share. Nothing further is read into it here. The machine was shared with other work, so other processes may have held some of the card's memory at any of those moments. In that 5.2 run the system-memory item beside it switched its unit from `MiB` to `GiB` as the figure grew.

**Still not reached, on either machine.** Whether the Status Bar's own right-click menu offers this item. On the Mac the menu was photographed and holds exactly five items, none of them video memory. On the Linux machine the menu could not be opened at all: a sweep of `bpy.types` for anything with `STATUSBAR` in it returns only `STATUSBAR_HT_header`, two theme entries and the preferences panel, so there is no menu to call by name, and there is no tool on that machine to send a right-click into the window. The preference can be switched on from Python, which is what the block above does.

**Observed, 2026-08-06 — what a render on an NVIDIA card prints about memory.**

Run on the rented Linux machine, Blender Blender 5.0.1, `-b`, display turned off first. A script set the Cycles device type to CUDA and turned the devices on. It printed:

```
DEVICE: 'NVIDIA GeForce RTX 4090' 'CUDA' use = True
DEVICE: 'AMD Ryzen Threadripper PRO 3975WX 32-Cores' 'CPU' use = True
DEVICE: 'NVIDIA GeForce RTX 4090' 'OPTIX' use = True
SCENE DEVICE: GPU
```

**The ordinary progress lines named no card memory**, exactly as on the Mac with Metal. The `Mem:` field was the same shape and size as on a processor render and no second one appeared.

**`--debug-cycles` is different, and this is the closest Blender comes to reporting card memory.** Added to the same run, it wrote on standard output as `cycles` log lines in this version, and carried these, word for word:

```
00:00.306  cycles           | Mapped host memory limit set to 80,647,581,696 bytes. (75.11G)
00:00.449  cycles           | Local memory reserved 1,321,205,760 bytes. (1.23G)
```

and a block headed `System memory statistics after full device sync:` with a `Usage:` and a `Peak:` line under it, and further down the devices named outright:

```
                            | Path tracing on: NVIDIA GeForce RTX 4090 (CUDA) [CUDA_NVIDIA GeForce RTX 4090_0000:00:07]
                            | Denoising on: AMD Ryzen Threadripper PRO 3975WX 32-Cores (CPU) (12 threads) [CPU]
```

The one figure that is plainly labelled is **system** memory. `Mapped host memory limit` is the figure this build set for memory outside the card; it read the same, `80,647,581,696 bytes. (75.11G)`, in all four versions on this machine. **No line anywhere states how much of the card's memory is in use.**

**One more thing the Mac's run never saw.** On a render using the card, the `--cycles-print-stats` block loses its kernel, shader and object sections and prints one line in their place:

```
Profiling information not available (only works with CPU rendering)
```

Its memory figures are the same as on a processor render and are still labelled by what the memory holds, not by which device it is on.

**Observed, 2026-08-06 — what the memory readouts say when there are two graphics cards.**

Run over SSH on a **second rented Linux machine — Ubuntu 22.04.5 LTS, two NVIDIA GeForce RTX 4090s of 24 GB each on driver 580.95.05, an AMD EPYC 7B13 with sixty-one cores, 98 GB of memory, no display server at all** — using Blender at `/opt/blender/blender-5.0.1-linux-x64/blender`, `Blender 5.0.1 (hash a3db93c5b259 built 2025-12-16 01:30:59)`, with `BLENDER_USER_RESOURCES` pointed at `/root/smoozy/two cards/res-5.0`, a folder of its own that was empty at the start. **Nothing else was running on the machine.** Label: **observed**. This is a third machine; nothing here is merged with the Creator's Mac or with the one-card rented Linux machine. The full list is `Audits/Blender Research/Reports/Hand tests — two cards and no screen.md`.

**The two cards are not joined by NVLink.** `nvidia-smi topo -m` reports `PHB` between them — `Connection traversing PCIe as well as a PCIe Host Bridge (typically the CPU)`. So the NVLink question this topic also owes is still out of reach.

**Blender reports one figure, never two, and never names a card beside a memory number.** The same Cycles scene at 1920 by 1080 and 6000 samples, rendered with both cards ticked and then with one, gave these `Finished` lines:

```
two cards: 00:29.243  render           | Fra: 1 | Mem: 9M | Finished
one card:  00:52.871  render           | Fra: 1 | Mem: 1084M | Finished
```

**The memory figure this version prints is smaller with two cards than with one** — `Mem: 9M` against `Mem: 1084M` above — not larger and not doubled, and no second figure ever appears. What that field counts is not settled by this test and nothing is read into it. The heavier scene — a 3000 by 3000 subdivided grid, 868 MB on disk — was run in Blender 4.5 alone and its figures are recorded in 4.5's own findings; it was not run in this version. *(Narrowed by the two-card citation check, 2026-08-06: this claim was worded as "the second `Mem:` field", which is 4.5's line shape — this version prints one — and rested on a 4.5 figure, when this version's own two runs carry it.)*

**Each card holds the same amount, and about as much as one card holds on its own.** `nvidia-smi --query-gpu=index,memory.used --format=csv,noheader` sampled five times a second through the heavier scene's render, highest figure each card reached:

```
two cards, Distribute Memory across devices OFF:   card 0: 3557 MiB   card 1: 3557 MiB
two cards, Distribute Memory across devices ON:    card 0: 3557 MiB   card 1: 3557 MiB
one card:                                          card 0: 3565 MiB   card 1: 3 MiB
```

Both cards hold the same amount, and turning the distribute-memory setting on made no difference on a pair that is not joined by NVLink. Reading those equal figures as the scene being duplicated on each card rather than divided between them is a step past the measurement — **inferred**, not observed. *(Labelled by the two-card citation check, 2026-08-06: the block said the scene lands on both cards "in full", which the figures do not state on their own.)* During the lighter scene both cards held the same 2683 MiB throughout. These card-by-card measurements were taken with Blender 4.5 on this machine and settle the machine rather than the version; they were not repeated in this version. *(Said plainly by the two-card citation check, 2026-08-06: the record says which version made the measurement and this block did not.)* **Blender said nothing at all about that setting**: searching both streams of every run for `peer`, `nvlink`, `interconnect` and `distribut` matched nothing but the output file's own name.

**`--debug-cycles` gives one memory block and says outright that it is system memory:**

```
                            | System memory statistics after full device sync:
                            |   Usage: 19,485,258 (18.58M)
                            |   Peak: 33,989,961 (32.42M)
```

with `Mapped host memory limit set to 101,923,811,328 bytes. (94.92G)` printed once per card, and `Local memory reserved 1,321,205,760 bytes. (1.23G)` likewise once per card. **No line anywhere states how much of either card's memory is in use.**

**`-- --cycles-print-stats` with two cards has the same shape as with one and names no device:**

```
Render statistics:
Mesh statistics:
  Geometry:
    Total memory: 6.15M (6,448,221)
        Mesh                             6.01M (6,303,093)
        Sphere                           23.55K (24,117)
        Sphere.001                       23.55K (24,117)
        Sphere.002                       23.55K (24,117)
        Sphere.003                       23.55K (24,117)
        Sphere.004                       23.55K (24,117)
        light                            81 (81)
Image statistics:
  Textures:
    Total memory: 0 (0)
Profiling information not available (only works with CPU rendering)
```

The memory figures are still labelled by what the memory holds — geometry, textures — not by which card it is on, and the profiling line is the same one a single-card render gives.

**Observed, 2026-08-06 — what the memory readouts say with four graphics cards, and what happens when a card is filled.**

Run over SSH on a **third rented Linux machine — Ubuntu 22.04.5 LTS, four NVIDIA GeForce RTX 4060 Ti of 16 GB each on driver 580.95.05, an AMD EPYC 7R43 with sixty-one cores, 197 GB of memory, no display server** — using Blender at `/opt/blender/blender-5.0.1-linux-x64/blender`, `Blender 5.0.1 (hash a3db93c5b259 built 2025-12-16 01:30:59)`, with `BLENDER_USER_RESOURCES` pointed at `/root/smoozy/four cards/res-5.0`, a folder of its own emptied and remade before use. **Nothing else was running on the machine.** Label: **observed**. This is a **fourth** machine; nothing here is merged with the Creator's Mac, the one-card rented Linux machine or the two-card rented Linux machine. The full list, with every command and every script written out, is `Audits/Blender Research/Reports/Hand tests — four cards.md`.

**Three things separate this machine from the two-card one, and all three bear on what is below.** Four cards rather than two. A **smaller** card — 16 GB against 24 GB, the smallest the project has had, so no figure here may be set beside a two-card figure and none is. And **197 GB of system memory** against 98 GB, which is what finally made it possible to fill a card while the machine's own memory was nowhere near full — the exact case this question asks about, and one no earlier machine could produce.

**The four cards are not joined by NVLink** — `nvidia-smi topo -m` reports `PHB` between every pair — so the NVLink question this topic also owes is still out of reach.

**Blender reports one figure, never four, and never names a card beside a memory number.** The same Cycles scene at 1920 by 1080 and 3000 samples, rendered with four cards ticked, with two and with one, gave these `Finished` lines:

```
four cards: 00:25.643  render           | Fra: 1 | Mem: 9M | Finished
two cards:  00:42.704  render           | Fra: 1 | Mem: 9M | Finished
one card:   01:15.714  render           | Fra: 1 | Mem: 392M | Finished
```

**The figure is far smaller with more than one card, and it does not go on shrinking.** Two cards and four cards printed exactly the same figure — the single `Mem:` field, `9M` against `392M` between four cards and one — and the same held on a heavier scene, a grid of 3000 by 3000 subdivisions at 909 MB on disk, where four cards and two cards both printed `Mem:480.88M, Peak:494.94M` in Blender 4.5 against `Mem:1224.33M, Peak:1694.79M` on one card. So the figure does not count memory divided by the number of cards. What it does count is not settled by this test and nothing is read into it. In Blender 4.5 the **first** `Mem:` field moves the other way — `103.60M` with four cards, `95.44M` with two, `91.37M` with one — and nothing is read into that either.

**A graphics card was filled while the machine's own memory held, which no earlier attempt had managed.** The scene is a plane whose surface samples a growing number of picture files of 8192 by 8192 pixels, each read through its own Mapping node so that none can be left out, added together and divided by their count; 640 by 480, 16 samples, one card ticked. The card is 16,380 MiB. What came back:

```
################ 64 textures of 8192 by 8192, one card
exit=0
00:20.688  render           | Fra: 1 | Mem: 16735M | Finished
00:21.309  render           | Time: 00:21.04 (Saving: 00:00.09)
   card 0 15706MiB
   card 1 3MiB
   card 2 3MiB
   card 3 3MiB
################ 128 textures of 8192 by 8192, one card
exit=0
03:26.089  render           | Fra: 1 | Mem: 33119M | Finished
03:29.305  render           | Time: 03:29.04 (Saving: 00:00.08)
   card 0 15738MiB
```

*(The runs above are quoted as they printed, except that the three unused cards' lines — `card 1 3MiB`, `card 2 3MiB`, `card 3 3MiB` — are given once and not repeated for each run; they read the same in every one of them. The whole output is in `Audits/Blender Research/Reports/Hand tests — four cards.md`.)*

**At 64 pictures the card is as full as it goes** — about 15.7 GB of 16.38 — and Blender's own figure already reads more than the card has. **At 128 pictures Blender asks for about 33 GB, twice the card's whole memory, and the card still sits at the same 15.7 GB.**

**Nothing failed and nothing was said.** Every run exited 0 and wrote its picture, and searching both streams of every run for `out of memory`, `failed to allocate`, `cannot allocate` and `insufficient` matched nothing. The only sign is the cost: the render time went from about twenty seconds at 64 pictures to about three and a half minutes at 128 — ten times longer for twice the data, at the same size and the same sample count.

**The one line that speaks to it at all** is printed by `--debug-cycles` before anything is loaded, and it names the machine's own memory as what the card may reach into:

```
Mapped host memory limit set to 208,234,123,264 bytes. (193.93G)
```

*(The `Local memory reserved` line that stood beside this one was cut by the four-card citation check, 2026-08-06: it carried Blender 4.5's figure, which was never captured in this version, and a repeat on the same machine showed the figure differs by version. What it reads here is on the hand-test list.)*

The four-card form of this run was made in Blender 4.5 only and is recorded in 4.5's own findings: all four cards filled to the same amount one card reached, and the run took four times longer than on one card. It was not repeated in this version.

**So the documented fallback to system memory has been seen for the first time, and it is silent.** What was **not** reached is the `Error: Out of memory` message the manual names: on this machine a scene bigger than the card spills into 197 GB of system memory instead, and running that out as well was not attempted.

**Driven from outside — this block.** Every run above was made from outside Blender's window, and on this machine there is no other way: with no display server there is nothing for a window to open on. So the `VRAM:` Status Bar readout is still unreached, with four cards as with two.

**Operating system — this block.** Linux, on the machine named at the top of it. It says nothing about macOS — where the Creator's M1 Pro shares one pool of memory, so a card cannot be filled while system memory holds even in principle — and nothing about Windows, which no Windows machine has run.

**What could not be reached here.** The `VRAM:` Status Bar readout with two cards. That readout needs Blender's window, and this machine has no display server at all — the machine that could photograph a window had one card, and the machine that has two cards cannot show a window. So **whether the Status Bar figure names a device, adds two cards together, or shows only one is still unanswered.** So is anything about an NVLink pair.

**Outside the interface.** Settled in part. With no window, Blender 5.0 printed no graphics-card memory figure at all, on a render that was actually running on the graphics card. The `Mem:` field and the `--cycles-print-stats` totals appeared identically whether the render used the processor or the card, and nothing labelled any of them as the card's. The one documented graphics-card readout is the `show_statusbar_vram` preference, which is present in this version and set off, and reading it needs the window.

**Operating system.** The preference itself carries no operating-system qualification. The sources do not say whether the readout differs by operating system. The Creator's tests were run on macOS 26.5.2 with Apple's own graphics through Metal only, so they settle that one combination. The rented Linux machine with an NVIDIA card was run too, and the paragraph below says what it settled; every other card and system is still open. (Citation check 2026-08-06: this sentence used to end by saying there was no machine here to test the others on, which the Linux run recorded below contradicts.)

The Creator's Status Bar menu screenshot and the Metal render were got on macOS 26.5.2 with Apple's own graphics. The `VRAM:` readout, the CUDA render and the `--debug-cycles` figures above were got on the rented Linux machine with an NVIDIA GeForce RTX 4090. **Both machines were run and they agree that nothing in the ordinary printed output is marked as the card's memory.** They differ in what could be reached: only Linux could show the readout, and only the Mac could photograph the right-click menu.

**Windows is untested and no Windows machine has run it.** **More than one card is no longer out of reach** — a third machine with two NVIDIA GeForce RTX 4090s answered it on 2026-08-06 and its result is in the block above: one figure, never two, and each card holding the same amount as the other. **A card running out of memory while system memory remains is no longer out of reach either** — it was reached on 2026-08-06 on a fourth machine with four NVIDIA GeForce RTX 4060 Ti of 16 GB each and 197 GB of system memory, and what happened is in the four-card block above. *(Corrected by the four-card citation check, 2026-08-06: this line still said the test was written out and never run, which the block above it and the closing list below it both contradict.)* Still out of reach: an NVLink bridge; the `VRAM:` Status Bar readout with two cards, the two-card machine having no display server and the machine with a display having one card; and any card that is neither Apple's nor NVIDIA's, so HIP and oneAPI are unreachable. `--debug-cycles` on the Linux machine records `WARNING HIPEW initialization failed: Error opening HIP dynamic library`, which is what an absent AMD card looks like there.

---

## Things a person with Blender 5.0 open could still settle by hand in a few minutes

Every item that was on this list has now been run — the terminal half and the Creator's window half on 2026-08-05, and the last one on a rented Linux machine on 2026-08-06. Their results are written into the answers above. **The list is empty.**

The item that survived the Mac run — what the video-memory readout actually displays — was answered on the Linux machine on 2026-08-06 and is recorded under question 6. It is labelled `VRAM:` and shows a used-of-total figure in GiB, with `24.0` on the right, which is this card's whole memory. The left-hand figure moves with what is on the card, so it is not quoted here as a fixed reading. Whether the left-hand figure is Blender's own use or the whole card's is still not settled, and the paired readings that show why are recorded there.

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

2026-08-04 — Opus 5 (Claude Code) — Reports, Blender 5.0

2026-08-05 — Opus 5 (Claude Code) — added the observed findings from the hand tests to questions 1 to 6, brought the two standing lines up to date where a test settled them, and pruned the list above

2026-08-05 — Opus 5 (Claude Code) — added the Creator's own window tests to questions 1, 2, 3, 5 and 6, and pruned the list above again

2026-08-05 — Opus 5 (Claude Code) — citation check: narrowed ten claims to what their quotes and their own records carry, removed one line that contradicted a test recorded in the same question, and sent one thinly recorded observation to the hand tests. Each change is noted where it was made.

2026-08-05 — Opus 5 (Claude Code) — closed the one item the citation check sent to the hand tests: replaced the screenshot note about the Output panel's header, under question 1, with the Creator's own answer and the steps they followed
2026-08-06 — Opus 5 (Claude Code) — added the hand tests run on a rented Linux machine with an NVIDIA card to questions 1 to 6, kept separate from the Mac's results throughout; answered what the video-memory readout displays, where a crash file goes on Linux, what the log-file switch catches, which lines quiet mode removes, what the render operator hands a script, and six failures the Mac could not reach; brought both standing lines up to date on every question and emptied the list at the end

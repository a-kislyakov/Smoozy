# Reports — Blender 5.2

**Sources consulted:** Blender 5.2 manual (`Blender Sources/manual_5.2`, including its `_images` folder), Blender 5.2 Python API documentation (`Blender Sources/api_5.2`), Blender 5.2 release notes (`Blender Sources/release_notes_5.2`), and Cycles' own files as shipped with Blender 5.2 (`Blender Sources/cycles_5.2`). All were taken on **2026-07-30**, which is the date read for every finding below. I also opened `Blender Sources/requirements/requirements_2026-08-01.txt`; it never names 5.2, so I have not used it as a statement about 5.2.

I read only the question sheet and files inside Blender Sources/. I did not open any other part of this project.

I did not use any helper agents; every finding below is my own reading of the files named.

---

## 1. What can cause a render to fail, and what does Blender do in each case?

**Label: documented** for the quoted causes and responses, **inferred** for the summary drawn across them, with one **observed** item from Cycles' own files.

The sources do not give a single list of "things that make a render fail". They describe failures scattered across several pages. Here is every cause I found, with what the source says Blender does.

### a. The graphics card runs out of memory

Two different things are described, depending on which backend is in use.

> "With CUDA, OptiX, HIP and Metal devices, if the GPU memory is full Blender will automatically try to use system memory. This has a performance impact, but will usually still result in a faster render than using CPU rendering."

— Manual, https://docs.blender.org/manual/en/5.2/render/cycles/gpu_rendering.html — read 2026-07-30

Note that oneAPI (Intel) is not in that list of four.

**This is the one failure in the list that has since been watched happening.** On 2026-08-06, on a machine with four NVIDIA GeForce RTX 4060 Ti of 16 GB each and 197 GB of system memory, a scene needing about 33 GB was rendered with a 16 GB card and the fallback ran: the card filled to its own size, the render finished, the picture was written, the process exited 0, **nothing was printed about any of it**, and the run took about ten times longer. It is recorded in full under question 6, as **Observed, 2026-08-06 — what the memory readouts say with four graphics cards, and what happens when a card is filled**. *(Pointer added by the four-card citation check, 2026-08-06: the hand-test list says that run answers questions 1, 2 and 6, and it had been written into question 6 alone.)*

Separately, in the page's "Error Messages" section, a message about the same shortage:

> "Error: Out of memory
>
> This usually means there is not enough memory to store the scene for use by the GPU."

— Manual, https://docs.blender.org/manual/en/5.2/render/cycles/gpu_rendering.html — read 2026-07-30

(Citation check 2026-08-05: the line introducing that message used to read "When it is not recovered from, an error is shown", which puts the message after a failed fallback. The two passages are in different sections of the page and nothing there orders them.)

For EEVEE the source is blunter about the consequence:

> "In practice, using too much GPU memory can make the GPU driver crash, freeze, or kill the application."

and

> "There is no standard way of estimating if the resources will fit into the GPU memory and/or if the GPU will render them successfully."

— Manual, https://docs.blender.org/manual/en/5.2/render/eevee/limitations/limitations.html — read 2026-07-30

There is also a report severity reserved for this condition, `ERROR_OUT_OF_MEMORY`, described only as "Out of Memory." — API documentation, https://docs.blender.org/api/5.2/bpy_types_enum_items/wm_report_items.html — read 2026-07-30

### b. The graphics driver times out (Windows, shared display/render card)

> "If a GPU is used for both display and rendering, Windows has a limit on the time the GPU can do render computations. If you have a particularly heavy scene, Cycles can take up too much GPU time."

The heading this sits under is the error text itself: "The NVIDIA OpenGL driver lost connection with the display driver". The page offers reducing Tile Size, separate cards for display and render, or raising the time-out.

— Manual, https://docs.blender.org/manual/en/5.2/render/cycles/gpu_rendering.html — read 2026-07-30

### c. The Cycles kernel cannot be compiled for the card

> "CUDA Error: Kernel compilation failed
>
> This error may happen if you have a new NVIDIA graphics card that is not yet supported by the Blender version and CUDA toolkit you have installed. In this case Blender may try to dynamically build a kernel for your graphics card and fail."

A related one, Linux only:

> "Unsupported GNU version
>
> On Linux, depending on your GCC version you might get this error."

— Manual, https://docs.blender.org/manual/en/5.2/render/cycles/gpu_rendering.html — read 2026-07-30

### d. There is nowhere for the render to go

Blender 5.2 added a switch that turns off automatic saving of render output. The release notes state the change plainly:

> "Saving render output can now be disabled."

— Release notes, https://developer.blender.org/docs/release_notes/5.2/rendering/ — read 2026-07-30

The manual describes what happens when it is off and nothing else is set up:

> "When disabled, Blender does not write render output from this panel and instead relies on other systems. For animation renders, if the Video Sequencer is active and no alternative output is available, an error is raised. Otherwise, if compositing is enabled, active File Output Node nodes are used to save the result. If no such nodes exist, an error is reported."

and

> "Playblast operations also require an active output method. Both Render Playblast and Render Playblast on Keyframes will raise an error when render output is disabled in this panel."

— Manual, https://docs.blender.org/manual/en/5.2/render/output/properties/output.html — read 2026-07-30

Also:

> "If no valid output method is available, rendering operations will report errors."

and

> "This method relies on automatic output saving being enabled. If Save Output is disabled, the render will fail unless an alternative output method is provided."

— Manual, https://docs.blender.org/manual/en/5.2/render/output/animation.html — read 2026-07-30

### e. Incompatible video container and codec

> "Some containers and codecs are not compatible with each other, so if you are getting errors check that your container and codec are compatible. Like containers and codecs are sometimes not compatible with each other, some codecs do not work with arbitrary dimensions."

The same page also warns that a failure part-way through a direct-to-video render damages the whole output:

> "If a problem occurs while rendering, the file might become unplayable and you will have to re-render all frames from the beginning."

— Manual, https://docs.blender.org/manual/en/5.2/render/output/properties/output.html — read 2026-07-30

### f. Blender crashes outright

> "The most common causes of Blender crashes:
>
> Running out of memory.
>
> Issues with graphics hardware or drivers.
>
> Bugs in Blender."

What Blender does:

> "When Blender crashes, it writes out a text file which contains information that may help identify the cause of the crash. Usually, this file is written in the Temporary Directory directory."

— Manual, https://docs.blender.org/manual/en/5.2/troubleshooting/crash.html — read 2026-07-30

### g. Bad command line, or a Python error

> "…will exit since -ba is an unknown argument."

— Manual, https://docs.blender.org/manual/en/5.2/advanced/command_line/arguments.html — read 2026-07-30

For scripts:

> "--python-exit-code <code>
>
> Set the exit-code in [0..255] to exit if a Python exception is raised (only for scripts executed from the command line), zero disables."

— Manual, https://docs.blender.org/manual/en/5.2/advanced/command_line/arguments.html — read 2026-07-30

### h. A Cycles-specific failure I could read directly (**observed**)

Cycles' denoise-animation operator fails if a frame is missing, and what it does is visible in the file:

```
                if not os.path.isfile(filepath):
                    scene.render.filepath = original_filepath
                    err_msg = tip_("Frame '%s' not found, animation must be complete") % filepath
                    self.report({'ERROR'}, err_msg)
                    return {'CANCELLED'}
```

— Cycles' own files, `cycles_5.2/operators.py`, lines 84–88, inside `CYCLES_OT_denoise_animation.execute` — read 2026-07-30

This is a denoising operator, not a render, but it shows the shape of the mechanism: an `ERROR` report plus a `CANCELLED` result.

### What Blender does, in general

Across all of the above, I infer three groups of behaviour: it reports a message (see question 2), it falls back from GPU memory to system memory, or it crashes and writes a crash file. The sources document each quoted behaviour, but they do not present this three-part grouping. The Python side also has hooks that fire on two endings:

> "bpy.app.handlers.render_cancel
>
> on canceling a render job. Accepts one argument: the scene data-block being rendered"

> "bpy.app.handlers.render_complete
>
> on completion of render job. Accepts one argument: the scene data-block being rendered"

— API documentation, https://docs.blender.org/api/5.2/bpy.app.handlers.html — read 2026-07-30

**Not covered by the sources.** There is no handler documented for "render failed" as distinct from "render cancelled". The sources say nothing about: a missing or absent camera; a full disk; an output folder that does not exist or cannot be written to; missing or unresolvable linked files and textures; network paths; a process exit code for a failed render (the only exit code documented anywhere is `--python-exit-code`, and that is for Python exceptions). The list above is what the sources happen to mention; it is not stated to be complete. Most of that list was settled by tests instead; see the observed block below. Still unsettled after those tests: a full disk, network paths, and running out of system memory during a processor render.

**Observed, 2026-08-05 — nine failure situations tried from a terminal.**

How all of these were run, once for all of them. On the Creator's own computer: a MacBook Pro with an Apple M1 Pro chip, macOS 26.5.2, Apple's own graphics and no other graphics card. Blender 5.2.0 LTS, at `/Applications/Blender.app`, started with no window (the `-b` switch) and with a throwaway settings folder of its own so nothing of the Creator's was touched. Unless said otherwise the file was the factory startup scene — one cube, one camera, one light — saved by Blender 4.5 so every version could open it, with render engine Cycles, 16 samples, 160 by 120 pixels, frames 1 to 3, PNG output. Each run's two output streams were kept in separate files and the shell's exit status was read straight afterwards. The full commands are in `Audits/Blender Research/Reports/Hand tests.md`, entries 7 to 12, 18 and 19.

**1. No camera in the scene.** The camera object was deleted from the file before saving. Run: `-b "no camera.blend" -o "<folder>/frame_" -F PNG -f 1`. The whole of standard output:

```
00:00.403  blend            | Read blend: "/Users/artist/Documents/Smoozy hand tests/Reports/23/no camera.blend"
00:00.407  render           | Rendering single frame
00:00.407  render           | Rendering frame 1
00:00.407  reports          | ERROR Cannot render, no camera
Blender 5.2.0 LTS (hash fbe6228777e7 built 2026-07-14 01:31:22)

Blender quit
```

Standard error was empty. Exit status 1. No image file existed in the output folder afterwards.

**2. The output folder does not exist.** Run: `-b "simple.blend" -o "<folder>/no_such_folder_52/deeper/frame_" -F PNG -f 1`, where neither folder existed. Exit status 0, standard error empty, and standard output ended:

```
00:00.625  render           | Saved: '/private/tmp/smoozy-test/hand-tests/Reports/007/no_such_folder_52/deeper/frame_0001.png'
00:00.625  render           | Time: 00:00.22 (Saving: 00:00.06)
```

Both folders existed afterwards and held the image.

**3. The output path cannot be written.** A one-byte plain file named `blocker_52` was made, and the output path was set inside it, so no folder could be created there. Run: `-b "simple.blend" -o "<folder>/blocker_52/frame_" -F PNG -f 1`. Exit status 1, standard error empty, and standard output ended:

```
00:00.534  image            | ERROR Couldn't create directory for file /private/tmp/smoozy-test/hand-tests/Reports/008/blocker_52/frame_0001.png: File exists
00:00.534  reports          | ERROR Render error (File exists) cannot save: '/private/tmp/smoozy-test/hand-tests/Reports/008/blocker_52/frame_0001.png'
00:00.534  render           | Time: 00:00.13

Blender quit
```

Two lines, under two different categories, `image` and `reports`. The render itself finished; the saving is what failed.

**4. An output path with a broken template in it.** Run: `-b "simple.blend" -o "<folder>/out_52/{blend_name.png" -F PNG -x 1 -a`. Exit status 1, standard error empty, and the whole of standard output:

```
00:00.401  blend            | Read blend: "/private/tmp/smoozy-test/hand-tests/Reports/018/simple.blend"
00:00.406  render           | Rendering animation (frames 1..3)
00:00.406  render           | Rendering frame 1
00:00.406  reports          | ERROR Parse errors in path '/private/tmp/smoozy-test/hand-tests/Reports/018/out_52/{blend_name.png':
                            | - Invalid or incomplete template expression '{blend_name.png'.
Blender 5.2.0 LTS (hash fbe6228777e7 built 2026-07-14 01:31:22)

Blender quit
```

The second line of that message is the same wording as the tooltip picture the manual shows for the path field. No frame was rendered and the output folder was empty.

**5. Turning off automatic saving of render output, with nothing else set up.** Run: `-b "simple.blend" --python-expr "import bpy; bpy.context.scene.render.save_output = False" -o "<folder>/out2_52/frame_" -F PNG -x 1 -a`. The setting exists in this version and was accepted. Nothing was written to disk — the output folder was empty afterwards. Standard error was empty. The whole of standard output:

```
00:00.397  blend            | Read blend: "/private/tmp/smoozy-test/hand-tests/Reports/018/simple.blend"
00:00.401  reports          | ERROR Render output disabled in Output properties and no active compositing File Output nodes
Blender 5.2.0 LTS (hash fbe6228777e7 built 2026-07-14 01:31:22)

Blender quit
```

**The exit status was 0.** So this failure reports an error, writes nothing, and still leaves the process looking successful — unlike every other failure below.

**6. An animation whose output folder stops being writable partway.** A six-frame Cycles animation at 2000 samples, 320 by 240, about seven seconds a frame. It was started, and once `frame_0001.png` existed the output folder was renamed away and a one-byte plain file was put in its place. Exit status 1. Standard output, at the point it went wrong:

```
00:07.545  render           | Saved: '/private/tmp/.../012/out_52/frame_0001.png'
00:07.545  render           | Time: 00:07.13 (Saving: 00:00.07)
00:07.545  render           | Rendering frame 2
00:14.567  image            | ERROR Couldn't create directory for file /private/tmp/.../012/out_52/frame_0002.png: File exists
00:14.567  reports          | ERROR Render error (File exists) cannot save: /private/tmp/.../012/out_52/frame_0002.png'
00:14.567  render           | Time: 00:07.02
```

Nothing was printed for frames 3, 4, 5 or 6 — there was no further `Rendering frame` line. The renamed folder held one file, `frame_0001.png`. No part-written file was left behind.

**7. The blend-file does not exist.** Run: `-b "<folder>/there_is_no_such_file.blend" -f 1`. Exit status 1, standard error empty, whole of standard output:

```
00:00.399  reports          | ERROR Cannot read file "/private/tmp/smoozy-test/hand-tests/Reports/010/there_is_no_such_file.blend": No such file or directory
Blender 5.2.0 LTS (hash fbe6228777e7 built 2026-07-14 01:31:22)

Blender quit
```

**8. A Cycles device override Cycles will not take.** Run: `-b "simple.blend" -o "<folder>/out_52/frame_" -F PNG -f 1 -- --cycles-device NONSENSE`. Exit status 1, standard error empty, whole of standard output:

```
00:00.390  cycles           | ERROR NONSENSE is not a valid Cycles device.
Blender 5.2.0 LTS (hash fbe6228777e7 built 2026-07-14 01:31:22)
Error: Not freed memory blocks: 1, total unfreed memory 0.000004 MB
```

No frame was rendered and the output folder was empty. The same command with `--cycles-device METAL` instead rendered normally and ended with status 0.

**9. An image texture that is missing when the render starts.** A cube was given a material with an image texture pointing at `gone.png`, the file was saved, and `gone.png` was then moved away. Run: `-b "missing.blend" -o "<folder>/out_52/frame_" -F PNG -f 1`. Exit status 0, and the image was written. Two lines were printed about it:

```
00:00.413  cycles           | WARNING Image file /private/tmp/smoozy-test/hand-tests/Reports/019/gone.png does not exist.
00:00.413  cycles           | ERROR Failed to load 1 image files
```

So it is reported as a warning and then a summary error, and the render goes ahead and succeeds anyway.

**Observed, 2026-08-05 — two of those failures repeated with Blender's window open.** Run by the Creator on their own MacBook Pro (Apple M1 Pro, macOS 26.5.2), Blender 5.2.0, reported as screenshots.

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

**Observed, 2026-08-05 — the Output panel's header, looked at by hand.** Done by the Creator on their own MacBook Pro (Apple M1 Pro, macOS 26.5.2), in Blender 5.2.

What was asked of them, the same in all four versions: open Blender at its normal startup file, go to the Properties editor, choose the Output tab, find the panel headed `Output`, and look at the header strip of that panel next to the word `Output`. If a checkbox is drawn there, rest the pointer on it and wait for the tooltip.

What came back: **5.2 has a checkbox in that header, and it is the only one of the four versions that does.** In the Creator's own words, "This checkbox only exists in 5.2." Its tooltip reads, word for word:

```
Save Output
Write frames to disk for animation renders.
```

So 4.5, 5.0 and 5.1 were looked at and have no such checkbox — a result, not a silence.

The Creator works on a Mac, so this settles macOS and leaves Windows and Linux open. Their answer does not say whether the box was ticked or empty, whether the tooltip carries any line below those two, or anything about the checkboxes inside the panel. Those are left open rather than filled in.

(Citation check 2026-08-05: this used to be a note about what a screenshot showed, with nothing about what was done to get the screenshot, so nobody could repeat it — and this version's answer is the one that carries the whole finding that saving render output can be switched off. The check asked for the procedure, and the record above replaces it. The steps are in `Audits/Blender Research/Reports/Hand tests.md`, entry 24.)

**Observed, 2026-08-06 — the same failures, and six more, on a second machine.**

Everything in this block was run on **a rented Linux machine — Ubuntu 22.04.5 LTS, NVIDIA GeForce RTX 4090, driver 580.105.08, twelve cores, run over SSH**. Blender Blender 5.2.0 LTS at `/opt/blender/blender-5.2.0-linux-x64/blender`, started with `-b`, pointed at its own settings folder, with the display turned off first (`unset DISPLAY`, `unset XAUTHORITY`). The scene was the same one-cube factory scene described above, rebuilt on that machine. The machine is a rented container shared with other work, so nothing below says how long anything took or how much memory was free. Full commands in `Audits/Blender Research/Reports/Hand tests — Linux.md`.

**The Mac's results held, word for word.** No camera, a missing output folder, a blocked output path, a broken path template, a missing blend-file, a refused Cycles device, an animation that loses its output folder partway, a missing image texture, and the output-saving setting — every one came back with the same exit status and the same message text as on the Mac. Two examples, quoted from the Linux runs:

```
00:00.374  reports          | ERROR Cannot render, no camera
```

```
00:00.599  image            | ERROR Couldn't create directory for file /root/smoozy/Reports/work/L08/blocker_52/frame_0001.png: File exists
00:00.599  reports          | ERROR Render error (File exists) cannot save: '/root/smoozy/Reports/work/L08/blocker_52/frame_0001.png'
```

Both machines were run and both agreed.

**A genuinely read-only output folder — new, and not reachable on the Mac.** The folder was made with `chmod 0555`. On this machine the user is root, and root walks through file permissions, so the run was made twice. Run plain, the permission was ignored: exit 0 and the image written. Run again with that overriding permission dropped for the one command — `capsh --drop=cap_dac_override,cap_dac_read_search -- -c "…"` — exit status **1**, nothing written, standard error empty, and standard output ended:

```
00:00.638  image.write      | ERROR OpenImageIO write failed: Could not open file "/root/smoozy/Reports/work/L25/ro_52/frame_0001.png"
00:00.638  reports          | ERROR Render error (Success) cannot save: '/root/smoozy/Reports/work/L25/ro_52/frame_0001.png'
```

**The word in brackets reads `Success`** on a failure caused by a refused permission. The category `image.write` is one the blocked-folder test never produced; that one said `image`.

**A corrupt blend-file — new.** Two were tried. A file cut short (the first 200 kB of a good one) gave exit status 1 and:

```
00:00.380  reports          | ERROR Failed to read blend file '/root/smoozy/Reports/work/L24/truncated.blend': Missing DNA block
```

A file that was not a blend-file at all gave exit status 1 and:

```
00:00.378  reports          | ERROR File format is not supported in file "/root/smoozy/Reports/work/L24/garbage.blend"
```

**A blend-file written by a newer Blender — not testable in this version.** This is the newest Blender the project researches, so there is no later one to write a file with. Blender 4.5, 5.0 and 5.1 were each given a file saved by this version: all three opened it, rendered it and ended at 0, and 5.0 and 5.1 printed `File written by newer Blender binary (502.44), expect loss of data!` while 4.5 printed the same words as a plain `Warning:` line.

**A crash, and where the crash file goes on Linux — new. The crash page's Linux line, run.** Blender was made to crash on purpose with a script whose whole content was `import ctypes`, a `print`, and `ctypes.string_at(0)`. With `simple.blend` loaded, the whole of standard output was:

```
00:00.380  blend            | Read blend: "/root/smoozy/Reports/work/simple.blend"
Blender 5.2.0 LTS (hash fbe6228777e7 built 2026-07-14 01:32:04)
Writing: /tmp/simple.crash.txt
```

The process ended with status **139**. A file `/tmp/simple.crash.txt` existed afterwards, holding a version line, a section headed `# backtrace`, and a section headed `# Python backtrace`. Run again with `--factory-startup` and no file loaded, the line read `Writing: /tmp/blender.crash.txt` and that file was written instead. **So on Linux the crash file is named after the blend-file when there is one**, and `blender.crash.txt` only when there is not — the manual's page names only the second. The `print` that ran immediately before the crash never reached standard output.

**Which handlers fire on four more kinds of failure — new.** The probe described under question 2 was run again against five cases, not one. On a blocked save: `render_init`, `render_pre`, `render_cancel`. On a broken path template: `render_init` and `render_cancel` only — `render_pre` never fired at all. On a missing image texture, which succeeds: the full successful set. So `render_cancel` on failure is now seen on three different failures in this version rather than one.

**And one more failure this version alone has.** Output saving switched off with nothing else arranged behaved on Linux exactly as on the Mac: nothing written, exit status **0**, and one line —

```
00:00.373  reports          | ERROR Render output disabled in Output properties and no active compositing File Output nodes
```

The handler probe adds something the Mac run never asked: **not one render handler fires in that case**, not even `render_init`. The render never begins.

**A Cycles device this card does offer.** `-- --cycles-device CUDA` and `-- --cycles-device OPTIX` were both accepted and both rendered the frame, exit 0. The refused name behaved exactly as on the Mac: exit 1, no frame, and

```
00:00.224  cycles           | ERROR NONSENSE is not a valid Cycles device.
```

on standard output, followed by `Error: Not freed memory blocks: 1, total unfreed memory 0.000004 MB`.

**Outside the interface.** Settled by the tests above for the cases run. With no window at all, a failed render prints its message and leaves the process with status 1, and a successful one leaves 0. Two failures break that rule and both matter: a missing image texture (item 9) and, more sharply, output saving being switched off with nothing else arranged (item 5), which writes nothing at all and still ends at 0. Everything printed went to standard output; standard error was empty in all nine runs. The output-method failure (d above) was confirmed to apply when the operator is driven from the command line. The crash-file behaviour (f) is not tied to the interface. The GPU errors (a, b, c) could not be tested on this machine.

**Operating system.** Some of these are explicitly one-platform. The driver time-out (b) is described as a Windows limit. "Unsupported GNU version" is described as Linux. The CUDA/OptiX and HIP backends are documented as "supported on Windows and Linux", oneAPI likewise, and Metal as "supported on Apple computers with Apple Silicon" (Manual, https://docs.blender.org/manual/en/5.2/render/cycles/gpu_rendering.html), so the memory fallback in (a), which names CUDA, OptiX, HIP and Metal, cannot apply on a platform where those backends do not run. The crash-file locations differ per operating system — see question 2. For the rest, the sources do not say.

**Two machines have now been used.** **macOS** is settled by the Creator's own MacBook Pro (Apple M1 Pro, macOS 26.5.2) for the nine cases first run; **Linux** is settled by the rented Linux machine for those same nine plus six more, recorded above. The two agreed on every message except one word in Blender 4.5's saving failure, which is not this version. **Windows is untested and no Windows machine has run it**, so the driver time-out and the Windows crash-file behaviour stay open.

**The Linux "Unsupported GNU version" could not be reached even on the Linux machine**, and the reason is worth keeping. That error comes from the CUDA toolkit's compiler being handed an unsupported GCC, and it only arises when Blender has to build a CUDA kernel while it runs. There is no CUDA toolkit on that machine — `which nvcc` returns nothing and `/usr/local/cuda` does not exist — and this Blender ships a kernel already built for that card, which `--debug-cycles` states outright: `Testing for pre-compiled kernel …/kernel_sm_89.cubin.zst.` then `Using precompiled kernel.` So no compile is ever attempted, and installing the toolkit was not permitted on that machine. The kernel-build failure for a card newer than Blender is unreachable for the same reason.

**The fallback to system memory when the card fills has still not been seen happen.** It needs a graphics card whose memory can be exhausted while the machine's own memory is not, which the rented Linux machine can do — but only with nothing else running on it, and it was shared. The test is written out in full, ready to run, under "Needs the machine to itself" in `Audits/Blender Research/Reports/Hand tests — Linux.md`. **HIP** needs an AMD card and **oneAPI** an Intel card; neither machine has one, and `--debug-cycles` on the Linux machine records `HIPEW initialization failed: Error opening HIP dynamic library`.

---

## 2. Where does Blender say that a render failed, and what does it say there?

**Label: documented** for the quoted reporting mechanisms, **inferred** where a general report or console description is applied specifically to a failed render, with one **observed** item.

Four possible places are described. `RenderEngine.error_set` directly documents a render error displayed after rendering finishes. The status bar and Info editor quotes describe reports and errors generally, and the console quote describes Blender messages and unexpected exits generally. Treating all three as places for a failed-render message is an inference, not a direct statement in those quotes.

### The status bar, briefly

> "Report Message
>
> Informational messages or warnings, such as after saving a file. They disappear after a short time. Click them to show the full message in the Info Editor."

— Manual, https://docs.blender.org/manual/en/5.2/interface/window_system/status_bar.html — read 2026-07-30

### The Info editor, kept

> "The Info editor logs the executed operators as well as errors, warnings, and informational messages. You can select an entry by clicking it, optionally holding Shift to add it to the existing selection."

— Manual, https://docs.blender.org/manual/en/5.2/editors/info_editor.html — read 2026-07-30

The manual's picture of that editor (`_images/editors_info-editor_ui.png`, shown on the page above) shows three rows, each with a green tick icon, reading `bpy.ops.curve.primitive_bezier_curve_add(radius=1, enter_editmode=False, align='WORLD', location=(0, 0, 0))`, `bpy.ops.object.text_add(...)` and `bpy.ops.object.empty_add(type='PLAIN_AXES', ...)`. Those are operator entries, not errors; the picture does not show what a failure row looks like. **documented** (it is part of the manual), read 2026-07-30.

### The reports list, readable from Python

This is new in 5.2 and the release notes say so:

> "Blender reports are now exposed as a read-only list in WindowManager ID types. They also now have a session-wide unique identifier integer"

— Release notes, https://developer.blender.org/docs/release_notes/5.2/python_api/ — read 2026-07-30

The list itself:

> "reports
>
> Collection of reports (default None, readonly)"

— API documentation, https://docs.blender.org/api/5.2/bpy.types.WindowManager.html — read 2026-07-30

Each entry carries the text and a severity:

> "message
>
> Report message text (default "", readonly, never None)"

> "type
>
> Report type (severity) (default 'DEBUG', readonly)"

> "session_uid
>
> Unique per-session report identifier (in [-inf, inf], default 0, readonly)"

— API documentation, https://docs.blender.org/api/5.2/bpy.types.Report.html — read 2026-07-30

The severities available are `DEBUG`, `INFO`, `OPERATOR`, `PROPERTY`, `WARNING`, `ERROR`, `ERROR_INVALID_INPUT`, `ERROR_INVALID_CONTEXT`, `ERROR_OUT_OF_MEMORY`, each described by its own name ("Error.", "Invalid Input.", "Invalid Context.", "Out of Memory.").
— API documentation, https://docs.blender.org/api/5.2/bpy_types_enum_items/wm_report_items.html — read 2026-07-30

A render engine puts messages into this system with two calls:

> "report(type, message)
>
> Report info, warning or error messages"

> "error_set(message)
>
> Set error message displaying after the render is finished"

— API documentation, https://docs.blender.org/api/5.2/bpy.types.RenderEngine.html — read 2026-07-30

The second is worth noting exactly: the error is displayed *after* the render is finished, not while it runs.

### The console window

> "The Console Window, also called a Terminal, is an operating system text window that displays messages about Blender's operations, status, and internal errors."

and, in the list of what it is for:

> "If Blender exits unexpectedly, the messages may indicate the cause or error."

— Manual, https://docs.blender.org/manual/en/5.2/advanced/command_line/index.html — read 2026-07-30

### And, for a crash rather than an error, a file

Windows: "On a crash, a file is written based on the name of the currently loaded blend-file, so test.blend will create a file called test.crash.txt." and "If the crash happens in Blender module, stack trace is also written to a file named blender.crash.txt. The path to that file can be found at the end of blender_debug_out.txt file."
macOS: "After a crash, the macOS Crash Reporter shows a window with backtrace after some time, or when Blender is opened again." and "Some .crash files can also be found in ~/Library/Logs/DiagnosticReports/ with the name of format: Blender_YYYY-MM-DD-HHMMSS_MACNAME.crash".
Linux: "After a crash, a file named blender.crash.txt is written to the /tmp directory."
— Manual, https://docs.blender.org/manual/en/5.2/troubleshooting/crash.html — read 2026-07-30

### What it actually says

Only four pieces of literal failure text appear anywhere in the sources, all of them Cycles error headings from one page: "Error: Out of memory", "The NVIDIA OpenGL driver lost connection with the display driver", "Unsupported GNU version", "CUDA Error: Kernel compilation failed" (Manual, https://docs.blender.org/manual/en/5.2/render/cycles/gpu_rendering.html).

One more literal message is readable in Cycles' own files (**observed**): `sys.stderr.write("Error: {:s}\n".format(str(ex)))` — Cycles' own files, `cycles_5.2/maketx.py`, line 83, inside `maketx_file`. That is the texture-conversion command, not a render, and it writes the word "Error:" followed by the exception text to the error stream and returns `1`. Read 2026-07-30.


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

**Not covered.** The sources give no wording for a render failure message, no format for it, and no statement about whether a failed render's message is also printed to the terminal as well as shown in the interface. Whether the Info editor and the reports list are populated at all when Blender runs without a window is not stated. The Info editor page does not say the log can be saved to a file.

**Observed, 2026-08-05 — where a failed render says so with no window, and what it says.**

Same computer and same way of running as in question 1: a MacBook Pro with an Apple M1 Pro chip, macOS 26.5.2, Apple's own graphics, Blender 5.2.0 LTS started with `-b` and its own throwaway settings folder. The two output streams were kept in separate files every time.

**The log carries the reports, and it all goes to standard output.** Every failure message in the nine runs of question 1 appeared on standard output, under a log category, in the shape `<elapsed time>  <category>          | <LEVEL> <message>`. Standard error was completely empty in all nine. Three different categories carried failure messages: `reports` for what an operator reported, `image` for the file-writing complaint, and `cycles` for the render engine's own.

**The exact words, quoted from the runs.** Each of these is the complete failure text that appeared:

```
00:00.407  reports          | ERROR Cannot render, no camera
```

```
00:00.534  image            | ERROR Couldn't create directory for file /private/tmp/.../blocker_52/frame_0001.png: File exists
00:00.534  reports          | ERROR Render error (File exists) cannot save: '/private/tmp/.../blocker_52/frame_0001.png'
```

```
00:00.406  reports          | ERROR Parse errors in path '/private/tmp/.../out_52/{blend_name.png':
                            | - Invalid or incomplete template expression '{blend_name.png'.
```

```
00:00.401  reports          | ERROR Render output disabled in Output properties and no active compositing File Output nodes
```

```
00:00.399  reports          | ERROR Cannot read file "/private/tmp/.../there_is_no_such_file.blend": No such file or directory
```

```
00:00.390  cycles           | ERROR NONSENSE is not a valid Cycles device.
```

```
00:00.413  cycles           | WARNING Image file /private/tmp/.../gone.png does not exist.
00:00.413  cycles           | ERROR Failed to load 1 image files
```

The fourth of those is the wording the manual describes only as an error being "raised" or "reported" for the output-saving case.

**The process status.** Read with `echo $?` straight after each run. A render that finished and saved: 0. No camera: 1. Cannot save the output: 1. Broken path template: 1. Blend-file missing: 1. Bad Cycles device: 1. An animation that lost its output folder partway: 1. A render with a missing image texture, which reported an error and finished anyway: 0. Output saving switched off with nothing else arranged, which wrote nothing at all: **0**.

So there is a process status that separates most failures from success, and two failures that do not use it.

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

So although there is no handler named for failure, this failed render — the no-camera file — fires `render_cancel`, and does not fire `render_complete`, `render_post` or `render_write`. (Citation check 2026-08-05: this used to say "a failed render". Only the no-camera failure was probed this way, so the sentence was narrowed to it.)

**Whether the reports list is filled with no window.** This is the gap the answer above declined to guess at. A failing render was run and a script then read the list. The command was `-b "no camera.blend" -o "<folder>/frame_" -F PNG -f 1 --python "reports.py"`, where the script is:

```python
import bpy
wm = bpy.context.window_manager
if not hasattr(wm, "reports"):
    print("REPORTS: attribute not present in this version")
else:
    print("REPORTS COUNT:", len(wm.reports))
    for r in wm.reports:
        print("REPORT:", repr(r.type), repr(r.message))
```

The whole of standard output was:

```
00:00.389  blend            | Read blend: "/Users/artist/Documents/Smoozy hand tests/Reports/23/no camera.blend"
00:00.393  render           | Rendering single frame
00:00.393  render           | Rendering frame 1
00:00.393  reports          | ERROR Cannot render, no camera
Blender 5.2.0 LTS (hash fbe6228777e7 built 2026-07-14 01:31:22)
REPORTS COUNT: 0

Blender quit
```

The attribute is present in 5.2 — the same script printed `REPORTS: attribute not present in this version` in 4.5, 5.0 and 5.1 — and after this failed background render it held nothing.

**Observed, 2026-08-05 — where a failed render is announced with the window open, and what it says.** Run by the Creator on their own MacBook Pro (Apple M1 Pro, macOS 26.5.2), Blender 5.2.0, reported as screenshots. In both cases the Info editor was on screen with its error and warning filters on.

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

Run on **a rented Linux machine — Ubuntu 22.04.5 LTS, NVIDIA GeForce RTX 4090, driver 580.105.08, twelve cores, run over SSH**. Blender Blender 5.2.0 LTS, `-b`, its own settings folder, display turned off first. The Mac and this machine were both run and are reported separately.

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

**Whether the reports list fills when there is a window — the half nobody had asked.** This version has `WindowManager.reports`, and with no window it was already known to be empty after a failure. On the Linux machine it was read again in four settings: after a no-camera failure with no window, after a blocked save, after output saving switched off, and **after a no-camera failure with the window open**. Every one printed:

```
REPORTS COUNT: 0
```

An operator error unrelated to rendering — opening a blend-file that is not there — left it empty too. So the list exists and answers the question in neither setting.

**The window wording is not a macOS oddity.** The no-camera file was rendered again with Blender's window open on the Linux machine, the render started from a script on a timer. The message raised read `Error: No camera found in scene "Scene"`, which is the same wording the Creator saw on their Mac, and not the `Cannot render, no camera` that the same file prints with no window. Both machines were run and both agreed.

**A gap in that Linux window run, written in rather than filled.** An Info editor was put on screen with its error, warning, information and operator filters on, and the window was photographed shortly after the failure and again twenty seconds later. **No entry appeared in the Info editor at all**, in either picture. The Creator's Mac run started the render by pressing F12; this one started it from a script. The two differ in how the render was started as well as in the machine, so nothing here says whether an Info-editor entry stays or fades, and that question is still open.

**What `--log-file` does to a failure message — new, and it matters.** Run as `-b no_camera.blend --log-file "<path>" -f 1`, with no `--log` category named, Blender 5.2 **moved the whole log into the file**, failure and all. The whole of standard output was:

```
Blender 5.2.0 LTS (hash fbe6228777e7 built 2026-07-14 01:32:04)

Blender quit
```

and the file held:

```
00:00.370  blend            | Read blend: "/root/smoozy/Reports/work/no_camera.blend"
00:00.375  render           | Rendering single frame
00:00.375  render           | Rendering frame 1
00:00.375  reports          | ERROR Cannot render, no camera
```

The exit status was still 1. So a caller watching standard output while a log file is asked for sees no failure message at all. (Blender 4.5 does the opposite — it creates the file, leaves it empty, and still prints everything to standard output.)

**A warning about standard error on Linux.** On this machine the shell arrives with `DISPLAY` set to `:0` and no authority file. Run that way with no window, Blender 5.2 wrote four lines to standard error on a completely successful render; 4.5, 5.0 and 5.1 wrote nothing. Those words come from the display libraries rather than from Blender, and every run recorded here had the display turned off first. The point that carries: on a Linux machine with a display running, standard error is not reliably empty even when nothing has gone wrong.

**Outside the interface.** Settled by the tests above for the cases run. With no window, a failed render's message is printed on standard output through the log, under the `reports`, `image` or `cycles` category, and the process is left with status 1 in every case tried except the missing texture and the disabled output setting, both of which leave 0. On the one failure probed for it, `render_cancel` fires and `render_complete` does not. `WindowManager.reports` exists in background mode and was empty after a failure, so it is not a route to a failure message there. The status bar and Info editor remain parts of the interface; the underlying report is plainly still generated, since it appears in the log under the `reports` category.

**Operating system.** The report and console mechanisms are described without any platform condition. The crash-file behaviour is explicitly different on each of the three: separate `.crash.txt` naming and `blender_debug_log.cmd` helper scripts on Windows, the system Crash Reporter and `~/Library/Logs/DiagnosticReports/` on macOS, `/tmp/blender.crash.txt` on Linux. The console window itself behaves differently on Windows: "When Blender is started on a Microsoft Windows operating system, the Console Window (called the Command Prompt) is first created as a separate window on the desktop. The main Blender window will also appear and the Console Window will then be toggled off. To display the console again, go to Window ‣ Toggle System Console." — Manual, https://docs.blender.org/manual/en/5.2/advanced/command_line/launch/windows.html — read 2026-07-30.

**Both machines have now been run and they agree on the words.** Every failure message quoted here came back identically on macOS 26.5.2 and on the rented Linux machine (Ubuntu 22.04.5 LTS, NVIDIA GeForce RTX 4090). The window's own wording for the no-camera failure, `No camera found in scene "Scene"`, was seen on both. **Windows is untested and no Windows machine has run it**, so the separate Command Prompt window, `Window ‣ Toggle System Console` and `-con, --start-console` remain unchecked. The three places a failure is announced — the Status Bar, a floating report box and the Info editor — were seen on macOS only; the Linux window run produced no Info-editor entry, for the reason written above.

---

## 3. What does Blender report about how far through a piece of work it has got?

**Label: documented.**

### In the status bar

> "Running Task
>
> Shows the progress of the currently running task (such as rendering or baking). Hovering the mouse pointer over the progress bar will display a time estimate. The task can be aborted by clicking the cancel button ()."

— Manual, https://docs.blender.org/manual/en/5.2/interface/window_system/status_bar.html — read 2026-07-30

The manual's picture of the middle of the status bar (`_images/interface_window-system_status-bar_ui-center.png`, shown on that page) shows a small camera-clapper icon, the label `Sequence Render`, a filled bar reading `94%`, and an `✕` button. So the thing on screen is: a name for the task, a bar, a whole-number percentage, and a cancel control. **documented**, read 2026-07-30. What is visible is what I have described; I am not claiming the percentage is always whole numbers or that every task is labelled this way, because one picture cannot show that.

### As a string, to Python

> "bpy.app.handlers.render_stats
>
> on printing render statistics. Accepts one argument: the render progress as a string containing current frame, current sample, render time and saving time."

— API documentation, https://docs.blender.org/api/5.2/bpy.app.handlers.html — read 2026-07-30

This is the most specific statement in any of the sources about what a render's progress consists of: **current frame, current sample, render time, saving time**, delivered as one string. The source says the handler runs "on printing render statistics". It does not say that the handler value is a copy of another value.

### The calls an engine uses to produce it

> "update_progress(progress)
>
> Update progress percentage of render
>
> Parameters:
>
> progress (float) – Percentage of render that's done (in [0, 1])"

> "update_stats(stats, info)
>
> Update and signal to redraw render status text
>
> Parameters:
>
> stats (str) – Stats, (never None)
>
> info (str) – Info, (never None)"

— API documentation, https://docs.blender.org/api/5.2/bpy.types.RenderEngine.html — read 2026-07-30

So the underlying quantity is a single fraction between 0 and 1, plus two free-text strings.

There is also a cancellation check an engine is expected to poll:

> "test_break()
>
> Test if the render operation should been canceled, this is a fast call that should be used regularly for responsiveness"

— API documentation, https://docs.blender.org/api/5.2/bpy.types.RenderEngine.html — read 2026-07-30

### Non-render tasks

Scripts drive the same bar with three calls: `progress_begin(min, max)`, `progress_update(value)` — "value, Any value between min and max as set in progress_begin() (in [-inf, inf])" — and `progress_end()`. — API documentation, https://docs.blender.org/api/5.2/bpy.types.WindowManager.html — read 2026-07-30

Physics baking reports differently:

> "The cursor will display as a number representing the progress of the baking."

— Manual, https://docs.blender.org/manual/en/5.2/physics/baking.html — read 2026-07-30

### Changes in 5.2

Three bug-fix titles listed for 5.2 name problems with progress reporting that were put right. They are titles of fixed bugs, not statements of how progress works: (Citation check 2026-08-05: this used to open "Two of the fixed bugs… are explicit statements about what changed" and then quote three.)

> " * Render Sequence Animation stopped showing total progress"

> " * VSE "Render Audio" is missing a progress indicator"

> " * Blender 5.1.1: Is Cycles render sample progress still accessible from Python without using -f?"

— Release notes, https://developer.blender.org/docs/release_notes/5.2/bugfixes/ — read 2026-07-30

And in the sequencer notes:

> "The "Render Audio" operator now shows a realtime progress bar that can be cancelled (previously Blender would freeze until the operation finished)."

— Release notes, https://developer.blender.org/docs/release_notes/5.2/sequencer/ — read 2026-07-30

**Not covered.** The sources do not give the format of the printed statistics string, do not say how often it is emitted, and do not say what "current sample" reads as when adaptive sampling or a time limit is in play. Nothing says whether an animation render reports overall progress across the whole frame range as well as within a frame — the bugfix line about "total progress" implies such a thing exists for sequence renders, but implication is not documentation. Nothing says what progress a compositing or denoising step contributes.

**Observed, 2026-08-05 — what progress a render prints with no window.**

Same computer and same way of running as in question 1. Blender 5.2.0 LTS, `-b`, own settings folder, Cycles, 16 samples, 160 by 120 pixels, output to PNG.

**A single frame.** Run: `-b "simple.blend" -o "<folder>/frame_" -F PNG -f 1`. Standard output ran to 54 lines, standard error zero bytes. The first six lines and the last six:

```
00:00.661  blend            | Read blend: "/private/tmp/.../001/simple.blend"
00:00.666  render           | Rendering single frame
00:00.666  render           | Rendering frame 1
00:00.667  render           | Start rendering: Scene, ViewLayer
00:00.667  render           | Engine: Cycles
00:00.679  render           | Fra: 1 | Mem: 0M | Synchronizing object | Cube
```

```
00:00.685  render           | Fra: 1 | Mem: 1M | Sample 0/16
00:00.689  render           | Fra: 1 | Remaining: 00:00.04 | Mem: 2M | Sample 1/16
00:00.855  render           | Fra: 1 | Mem: 2M | Sample 16/16
00:00.855  render           | Fra: 1 | Mem: 2M | Finished
00:00.935  render           | Saved: '/private/tmp/.../001/out_52/frame_0001.png'
00:00.935  render           | Time: 00:00.26 (Saving: 00:00.08)
```

So the format of what is printed is settled: a timestamp since start, a category, a vertical bar, then a frame number, sometimes a remaining time, a memory figure, and either a stage name or a sample count. Between those two groups the run listed forty-odd preparation stages, one line each, for example `Updating Scene BVH | Building BVH 0%` and `Loading denoising kernels (may take a few minutes the first time)`.

**A three-frame animation.** Run: `-b "simple.blend" -o "<folder>/frame_" -F PNG -x 1 -a`. Exit status 0, three files written, 150 lines. The frame boundaries are announced:

```
00:00.405  render           | Rendering animation (frames 1..3)
00:00.405  render           | Rendering frame 1
00:00.573  render           | Rendering frame 2
00:00.658  render           | Rendering frame 3
```

No line anywhere gave a figure for the animation as a whole. Each frame ran its own sequence from `Sample 0/16` to `Sample 16/16`, `Finished`, `Saved:` and `Time:`. So progress is divided per frame and, within a frame, per sample, and there is no overall figure for the range.

**What the render-statistics handler is actually handed.** The API says the argument is "the render progress as a string containing current frame, current sample, render time and saving time". A script attached a handler to `bpy.app.handlers.render_stats` that printed the arguments with `repr`. It fired 133 times over the three-frame animation. Every string it received, at each of the three frames, was the same set:

```
RENDER_STATS: ('Remaining: 00:00.01 | Mem: 2M | Sample 1/16', None)
RENDER_STATS: ('Mem: 2M | Sample 16/16', None)
RENDER_STATS: ('Mem: 2M | Finished', None)
RENDER_STATS: ('Time: 00:00.16 (Saving: 00:00.06)', None)
```

The current sample is there, the render time and saving time are there on the last string of each frame, and a memory figure is there. **The current frame is not in it** — the `Fra: N` field of the printed line does not reach the handler, and the same strings arrived unchanged at frames 1, 2 and 3. The handler is also handed a second value, which printed as `None`. It fires with no window open.

**A non-Cycles engine.** The same file with `bpy.context.scene.render.engine` set to `BLENDER_EEVEE` printed 14 lines in all. (Citation check 2026-08-05: this used to add "the only EEVEE identifier this version offers". Nothing in the record says how that was established, so it was removed.) The render part:

```
00:00.602  render           | Engine: EEVEE
00:00.790  render           | Fra: 1 | Rendering 1 / 64 samples
00:00.908  render           | Fra: 1 | Rendering 25 / 64 samples
00:00.982  render           | Fra: 1 | Rendering 50 / 64 samples
00:01.021  render           | Fra: 1 | Rendering 64 / 64 samples
```

No memory field, no remaining time, and no per-stage lines. So what a render reports depends on the engine.

**Observed, 2026-08-05 — what the window shows while a render runs.** Run by the Creator on their own MacBook Pro (Apple M1 Pro, macOS 26.5.2), Blender 5.2.0, reported as screenshots. The scene was Cycles at 1280 by 720 with 1024 samples and a frame range of 1 to 6.

**The Status Bar.** The task is named `Rendering...`, followed by a bar with a whole-number percentage on it and an `✕` button at its right. Percentages seen across the shots: `73%`, `25%`.

**Whether the percentage covers a frame or the whole range.** It tracks the samples within a frame. In the shots the bar read `73%` while the render window read `Sample 748/1024`, and `25%` while it read `Sample 256/1024` — each matching that frame's own sample fraction. (Citation check 2026-08-05: this used to add that it restarts at each frame and "did not climb across the six frames". Both status lines quoted here read `Frame:1`, and no reading from any other frame was recorded, so nothing here shows what the bar does when the frame changes. The 4.5, 5.0 and 5.1 documents each have a reading from a second frame; this one does not.)

**The Blender Render window's own status line.** One line at the top left of that window. Quoted from the shots, word for word:

```
Frame:1 | Time:00:18.73 | Remaining: 00:06.83 | Mem: 1506M | Sample 748/1024 (Using optimized kernels)
```

```
Frame:1 | Last:00:27.14 Time:00:05.69 | Remaining: 00:16.52 | Mem: 1506M | Sample 256/1024 (Using optimized kernels)
```

So the fields are: frame number; sometimes the previous frame's time as `Last:`; elapsed `Time:`; sometimes `Remaining:`; a memory figure; and then a sample count or a stage. The bracketed note after the sample count changed between shots through `(2 kernels to optimize)` and `(Using optimized kernels)`. That is what the pictures show.

**Observed, 2026-08-06 — the same runs on a second machine.**

Run on **a rented Linux machine — Ubuntu 22.04.5 LTS, NVIDIA GeForce RTX 4090, driver 580.105.08, twelve cores, run over SSH**. Blender Blender 5.2.0 LTS, `-b`, its own settings folder, display turned off first. Both machines were run and both agreed on everything below.

**The printed lines are the same shape and the same count.** A single-frame render ran to 54 lines on standard output and nothing on standard error — the same count as the Mac. The closing lines, word for word:

```
00:00.544  render           | Fra: 1 | Mem: 2M | Sample 16/16
00:00.544  render           | Fra: 1 | Mem: 2M | Finished
00:00.680  render           | Saved: '/root/smoozy/Reports/work/L01/out_52/frame_0001.png'
00:00.680  render           | Time: 00:00.30 (Saving: 00:00.13)
```

**The three-frame animation behaved the same.** 151 lines, three files, exit 0. It opened with `render | Rendering animation (frames 1..3)` and named each frame as it began — `Rendering frame 1`, `Rendering frame 2`, `Rendering frame 3` — and no line gave any figure for the range as a whole.

**Which frame a progress line belongs to.** The printed lines carry the frame number even though the `render_stats` string does not. From the animation:

```
00:00.406  render           | Fra: 1 | Remaining: 00:00.03 | Mem: 2M | Sample 1/16
00:00.676  render           | Fra: 2 | Remaining: 00:00.02 | Mem: 2M | Sample 1/16
00:00.820  render           | Fra: 3 | Remaining: 00:00.01 | Mem: 2M | Sample 1/16
```

**What the statistics handler is handed — the same as on the Mac.** It fired 134 times over three frames (133 on the Mac) and received the message part only, with no timestamp, no category and no frame number:

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

**Whether a program can read the progress as it happens.** A two-frame render was piped into a reader that stamped each line with the seconds since the run began, and those stamps were set beside the elapsed time Blender itself writes inside each line. The lines arrived spread across the run rather than in one lump at the end: in this version the line Blender stamped `00:05.874` reached the reader at 5.99 seconds, and the line stamped `00:11.494` reached it at 11.61 seconds. Worth knowing beside it: in a background render Cycles printed only three sample lines per frame — `Sample 0/2000`, `Sample 1/2000` and the last one — so there is little to read between them.

**Outside the interface.** Settled by the tests above. Progress is reported with no window, as timestamped `render` log lines carrying the frame number, a memory figure, sometimes a remaining time, and the sample count or stage; the `render_stats` handler fires there and receives the message part of each line. What does not appear outside the window is any percentage, and any figure covering a whole animation. The status bar and its progress bar remain interface objects.

**Operating system.** The sources state no platform condition on any of this. The Creator's window tests were got on macOS 26.5.2 only. The printed lines were got on Linux as well, and the paragraph below says so. (Citation check 2026-08-06: this sentence used to say all of it was macOS-only and that neither other system could be tested, which the Linux run recorded below contradicts. Narrowed to the window tests.)

The printed progress was got on **both** machines — the Creator's MacBook Pro (Apple M1 Pro, macOS 26.5.2) and the rented Linux machine — and the two agreed on the line shape, the fields, the frame announcements and the counts, so it holds on macOS and on Linux. **Windows is untested and no Windows machine has run it.** **Untried rather than untestable: a Windows machine can be rented, and every one this project has read reported a Server edition — see the equipment section of `Preparation/Blender Research/Method.md`.** The Creator's window tests — the Status Bar task name, the percentage and the Blender Render window's status line — were got on macOS only.

---

## 4. What does Blender write out as it works, what is that stream called, and what does it contain?

**Label: documented**, with the naming of the stream **observed** from Cycles' own files, and one part **not in the sources**.

### What the manual calls it

The manual has no name like "stdout" or "stderr". Its name for the destination is the Console Window, or Terminal:

> "The Console Window, also called a Terminal, is an operating system text window that displays messages about Blender's operations, status, and internal errors."

> "When Blender is manually started from a terminal, Blender output is shown in the Console Window it is started from."

— Manual, https://docs.blender.org/manual/en/5.2/advanced/command_line/index.html — read 2026-07-30

### What it contains

From the same page's list of what the console is used for:

> "For Python development, to see the output of the print() function."

> "If Blender exits unexpectedly, the messages may indicate the cause or error."

> "When troubleshooting, to see the output of --debug messages."

And a warning that the window is not merely a viewer: "Closing the Console Window will also close Blender, losing any unsaved work."

Rendering to video adds to it:

> "When you view the System Console, you can see some of the output of the encoding process. You will see even more output if you execute Blender as blender -d."

— Manual, https://docs.blender.org/manual/en/5.2/render/output/properties/output.html — read 2026-07-30

The manual's Windows screenshot of the console (`_images/advanced_command-line_launch_windows_console.png`, on https://docs.blender.org/manual/en/5.2/advanced/command_line/launch/windows.html) shows the lines: `Read new prefs: C:\Users\Aaron\AppData\Roaming\Blender Foundation\Blender\2.77\config\userpref.blend`, `found bundled python: c:\blender\2.77\python`, `read blend: C:\tmp\Vincent.blend`, and later `Blender quit`. The Linux screenshot (`_images/advanced_command-line_launch_linux_console.png`) shows audio-subsystem messages such as `jack server is not running or cannot be started` and `AL lib: (WW) alc_initconfig: Failed to initialize backend "jack"`. **documented** (both are manual images), read 2026-07-30. Both screenshots show an older Blender than 5.2 — the Windows one has `2.77` in its paths — so I treat them as illustrations of the kind of line that appears, not as the 5.2 wording.

### Categorised logging can be steered

Blender 5.2 has a logging system with categories and levels:

> "--log <match>
>
> Enable logging categories, taking a single comma separated argument."
>
> `--log "*": everything`
> `--log "event": every category starting with event.`
> `--log "render,cycles": both render and cycles messages.`

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

> "--log-file <filepath>
>
> Set a file to output the log to."

> "--log-show-memory
>
> Show memory usage for each log message."

> "--log-show-source
>
> Show source file and function name in output."

> "--log-list-categories
>
> List all available logging categories for --log, and exit."

— Manual, https://docs.blender.org/manual/en/5.2/advanced/command_line/arguments.html — read 2026-07-30

Note the existence of `render` and `cycles` as named logging categories, quoted in the example above.

The same page also establishes that the ordinary running commentary is separable from the failures:

> "-q, --quiet
>
> Suppress status printing (warnings & errors are still printed)."

— Manual, https://docs.blender.org/manual/en/5.2/advanced/command_line/arguments.html — read 2026-07-30

And Cycles can be asked to add its own numbers:

> "--cycles-print-stats
>
> Log statistics about render memory and time usage."

— Manual, https://docs.blender.org/manual/en/5.2/advanced/command_line/arguments.html — read 2026-07-30

### Where the stream gets its proper name (**observed**)

The words "stdout" and "stderr" appear nowhere in the 5.2 manual, the 5.2 API documentation, or the 5.2 release notes. They appear in Cycles' own files. Cycles' argument parser says exactly which stream its statistics go to:

```
    parser.add_argument("--cycles-print-stats",
                        help="Print rendering statistics to stderr",
                        action='store_true')
```

— Cycles' own files, `cycles_5.2/engine.py`, lines 13–15, inside `_configure_argument_parser` — read 2026-07-30

And Cycles' texture-conversion command splits the two streams by hand — errors to `stderr` with a non-zero return, the result path to ordinary print:

```
    except RuntimeError as ex:
        sys.stderr.write("Error: {:s}\n".format(str(ex)))
        return 1

    print(out_filepath)
    return 0
```

— Cycles' own files, `cycles_5.2/maketx.py`, lines 82–87, inside `maketx_file` — read 2026-07-30

Blender's own API documentation says what those return values mean for a registered command: "The return value should be 0 for success, 1 on failure (specific error codes from the os module can also be used)." — API documentation, https://docs.blender.org/api/5.2/bpy.utils.html (`bpy.utils.register_cli_command`) — read 2026-07-30. That sentence is about the callback of a command registered through `register_cli_command`, and `maketx_command` is registered that way: `cli_commands.append(bpy.utils.register_cli_command("maketx", maketx_command))` — Cycles' own files, `cycles_5.2/__init__.py`, line 163 — read 2026-07-30. (Citation check 2026-08-05: the API sentence used to be applied to the returns above without that connecting line, which is in the sources and has been added.)

**Not in the sources:** which stream Blender's *own* messages go to. Nothing in the manual, the API documentation or the release notes assigns Blender's render progress, status printing, warnings or errors to standard output versus standard error. The only stream named anywhere is `stderr`, and only for Cycles' `--cycles-print-stats` and for Cycles' `maketx` errors.

**Where I looked, for that gap.** I searched the whole of `manual_5.2`, `api_5.2`, `release_notes_5.2` and Cycles' Python files, case-insensitively, for: `stdout`, `stderr`, `standard output`, `standard error`, `console`, `terminal`, `System Console`, `exit code`, `exit-code`, `return code`, `error code`, `errorlevel`, `log`, `logging`, `verbose`, `quiet`. The only hits for the stream names were the two Cycles lines quoted above (plus an unrelated compositor caption, "Standard output image."). I read in full: the Command Line index, Arguments, and all three Launch pages of the manual; the Crash page; the Output properties page; and `bpy.app.txt`, `bpy.app.handlers.txt`, `bpy.types.RenderEngine.txt`, `bpy.types.Report.txt`, `bpy.types.WindowManager.txt` and `bpy.utils.txt` in the API documentation.

**Not covered.** No line format, no timestamps, no statement of how much is written per frame, no guarantee about ordering or interleaving between the two streams, no statement about buffering, and nothing about whether `--log-file` captures everything the console would show or only the categorised log.

**Observed, 2026-08-05 — which stream carries what, and what the switches do.**

Same computer and same way of running as in question 1. Blender 5.2.0 LTS, `-b`, own settings folder. Every run sent its two streams to separate files.

**Which stream — this is the gap the answer above names.** On a plain successful single-frame render, standard output held all 54 lines and standard error held nothing — zero bytes. The same was true of the three-frame animation, the EEVEE render, the render with a missing texture, and every one of the nine failure runs in question 1, including the errors. Across every 5.2 run made, nothing was ever written to standard error. So Blender's own messages, progress and errors alike, went to standard output.

**Where the Cycles statistics land.** Run: `-b "simple.blend" -o "<folder>/frame_" -F PNG -f 1 -- --cycles-print-stats`. Standard error was zero bytes. The statistics appeared on **standard output**, plainly printed rather than as log lines — no timestamp, no category, no vertical bar. They begin:

```
Render statistics:
Mesh statistics:
  Geometry:
    Total memory: 386B (386)
        Cube                             289B (289)
        backgroundlight                  50B (50)
        Light                            47B (47)
Image statistics:
  Total image memory: 0B (0)
Kernel statistics:
  Total render time               : Total 100.00% (0.11s), Self 22.32% (0.03s)
    Shade Surface                   : Total 25.89% (0.03s), Self 0.00% (0.00s)
```

and end:

```
Shader statistics:
  Material                        : 0.03s (Relative cost: 1.00)
Object statistics:
  Cube                            : 0.03s (Relative cost: 1.00)
```

The whole run was 129 lines on standard output and nothing on standard error. So the `stderr` named in the help text of `cycles_5.2/engine.py` is not the stream the statistics actually appeared on here.

**What quiet mode removes.** Run: `-b -q "simple.blend" -o "<folder>/frame_" -F PNG -f 1`. Exit status 0, the frame was written, and standard output still held 51 lines — the whole render log, from `Rendering single frame` through every sample line to `Time: 00:00.16 (Saving: 00:00.06)`. The `blend | Read blend: "…"` line at the top was gone. Standard error was empty. So the "status printing" that `-q` suppresses does not include render progress. (Citation check 2026-08-05: this used to say the `Read blend` line was the *only* line that disappeared. The plain run was recorded at 54 lines and this one at 51, and no comparison of the two outputs was kept, so that does not follow from what is written down.)

**The list of logging categories.** Run: `--log-list-categories`. Exit status 0, standard error empty, 149 category names on standard output, one per line. `render` and `cycles` are both among them, as are `image` and `reports`, the three categories that carried messages in the tests above. Sixteen names are in this list and not in 5.0's 135: `asset.library.essentials`, `asset.remote_library`, `asset.remote_listing`, `assets.disk_file_hash_service`, `functions.mf_registry`, `geometry.compare`, `ghost.xr`, `gpu.framebuffer`, `gpu.metal`, `gpu.mipmap`, `gpu.shader.deps`, `hydra.scene_index`, `pose.paste`, `rna.node`, `system`, `wm.xr`.

**What a render logs at information level.** Run: `-b "simple.blend" --log "*" --log-level info -o ... -f 1`. Accepted, exit status 0, standard error empty, 150 lines. The `render` category lines are the same progress lines already quoted in question 3, so the render progress is part of the log rather than printed beside it.

**What the built-in help lists.** Run: `--help`, 518 lines on standard output, nothing on standard error. Its whole Logging Options section:

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

Run on **a rented Linux machine — Ubuntu 22.04.5 LTS, NVIDIA GeForce RTX 4090, driver 580.105.08, twelve cores, run over SSH**. Blender Blender 5.2.0 LTS, `-b`, its own settings folder, display turned off first. Both machines were run.

**Which stream, and where the statistics sit — agreed, and now placed.** Standard output carried everything and standard error was empty on every ordinary run, exactly as on the Mac. With `-- --cycles-print-stats` the statistics appeared on **standard output** again, 129 lines in all, so the help text in `engine.py` naming stderr does not describe where they arrive on either machine. **Where they sit is now settled for this version:** the block opens with `Render statistics:` **after** the closing `render | Time: …` line, at the very end of the run. (Blender 4.5 puts it earlier, between `Finished` and `Saved:`.)

**Quiet mode — agreed, and the missing two lines are now named.** `-b -q` left 51 lines on standard output against 54 without it, the same counts as the Mac, and the whole render log survived. The two runs were then compared with their timestamps stripped. **The three lines `-q` removes are:**

```
blend            | Read blend: "/root/smoozy/Reports/work/simple.blend"
```

a blank line, and

```
Blender quit
```

That is the thing all three of these documents were corrected to say was unknown.

**The logging switches — the same shape, a longer list.** `--log-list-categories` was accepted and ended at 0. It listed **182** categories here against **149** on the Mac. The Linux list carries a run of `ghost.wl.*` window-system names the Mac's would have no use for. Which exact names differ cannot be worked out, because the Mac's list was kept only as a count. The list is the same whether Blender is started with `-b` or with a window. `--log "*" --log-level info` was accepted, produced 148 lines, and the only categories that appeared on a render were `blend`, `cycles`, `render` and `translation`.

**What `--log-file` catches — new, and nobody had run it.** Run as `-b simple.blend --log-file "<path>" -o … -f 1`, with no `--log` category named, this version **moved the whole render log into the file**. Standard output dropped to three lines — the version banner, a blank line and `Blender quit` — and the file held 51 lines, from `blend | Read blend:` through every sample line to the closing `render | Time:`. On a failing render the failure message goes with it; that is recorded under question 2. **Blender 4.5 does the opposite**: it creates the file, leaves it empty, and still prints everything to standard output.

**Where `--debug-cycles` writes — new.** Run as `-b simple.blend --debug-cycles -o … -f 1`, the debug material went to **standard output**, as ordinary log lines under a `cycles` category. Standard error stayed empty. Some of what it carries:

```
00:00.383  cycles           | CUEW initialization succeeded
00:00.437  cycles           | Added device "NVIDIA GeForce RTX 4090" with id "CUDA_NVIDIA GeForce RTX 4090_0000:00:07".
00:00.439  cycles           | WARNING HIPEW initialization failed: Error opening HIP dynamic library
```

**Blender 4.5 puts the same material on standard error instead**, in a different shape entirely. So which stream carries the Cycles debug output changed after 4.5, and a caller that reads only one stream will see it in one version and not the other.

**The built-in help — agreed on content, longer on this machine.** `--help` gave 525 lines on Linux against 518 on the Mac, and the logging entries it lists are the same seven: `--log`, `--log-level`, `--log-show-memory`, `--log-show-source`, `--log-show-backtrace`, `--log-file`, `--log-list-categories`. The quiet entry reads word for word the same on both machines and in all four versions:

```
-q or --quiet 
	Suppress status printing (warnings & errors are still printed).
```

**Outside the interface.** Yes, this is the part that is about running outside the interface, and every test above was run that way. Everything Blender wrote went to standard output; standard error stayed empty on every run recorded here, all of which had the display turned off first. (Citation check 2026-08-06: this used to say standard error stayed empty across every 5.2 run made, which the display-noise result in question 2 and the closing paragraph below both contradict — with a display running and no authority file, a windowless 5.2 wrote four display-library lines to standard error on a successful render.) The logging arguments are command-line arguments and so exist only when Blender is started with them. What differs inside the interface is that on Windows the console is hidden by default and must be re-opened from Window ‣ Toggle System Console.

**Operating system.** The stream exists on all three, but access to it differs. On Linux and macOS the manual describes simply starting Blender from a terminal. On Windows the console is a separate window that is created and then toggled off, and is re-shown from the Window menu. Two logging-adjacent arguments are platform-restricted by their own text: `-con, --start-console` is "(Windows only)" and `--debug-ghost` is "(Linux only)". — Manual, https://docs.blender.org/manual/en/5.2/advanced/command_line/arguments.html — read 2026-07-30

**Both machines have now been run.** They agree on which stream carries what, on quiet mode, on the logging switches and on the shape of a log line. Two differences belong to the machine rather than to Blender: the category list is longer on Linux, and so is the built-in help. One belongs to the display rather than to Blender: on a Linux machine with a display running, a windowless Blender may have display-library complaints appear on its standard error, which vanish once the display is turned off. **Windows is untested and no Windows machine has run it**, so the separate Command Prompt window and `-con, --start-console` stay unchecked.

---

## 5. Does Blender estimate the time left, for what unit of work, and how does it describe that estimate?

**Label: documented**, but only barely — one sentence.

> "Hovering the mouse pointer over the progress bar will display a time estimate."

— Manual, https://docs.blender.org/manual/en/5.2/interface/window_system/status_bar.html — read 2026-07-30

That is the entire statement. The unit of work is the one the progress bar is for, which the same passage identifies: "the currently running task (such as rendering or baking)". So: one running task, not one frame and not a whole animation, unless the task happens to be one of those. How it is described is not stated — the manual does not say whether the estimate is of time remaining or time total, what units it uses, or what wording it takes.

**Elapsed time, which is a different thing, is reported in three places:**

The render statistics string, which "contain[s] current frame, current sample, render time and saving time" — API documentation, https://docs.blender.org/api/5.2/bpy.app.handlers.html (`bpy.app.handlers.render_stats`) — read 2026-07-30.

The image metadata, where "Render Time — Includes the render time." can be stamped into the output file — Manual, https://docs.blender.org/manual/en/5.2/render/output/properties/metadata.html — read 2026-07-30.

And Cycles' `--cycles-print-stats`, "Log statistics about render memory and time usage." — Manual, https://docs.blender.org/manual/en/5.2/advanced/command_line/arguments.html — read 2026-07-30.

**A time limit is not an estimate**, but it is the only other time-and-render mechanism in the sources, so for completeness:

> "Time Limit
>
> Stops rendering if the time exceeds the limit, even if the desired sample count hasn't been reached yet. Setting this value to zero disables the limit."

and

> "The time limit does not include pre-render processing time, only render time."

— Manual, https://docs.blender.org/manual/en/5.2/render/cycles/render_settings/sampling.html — read 2026-07-30

The setting itself, in Cycles' own files (**observed**):

```
    time_limit: FloatProperty(
        name="Time Limit",
        description="Limit the render time (excluding synchronization time). "
        "Zero disables the limit",
        min=0.0,
        default=0.0,
        step=100.0,
        unit='TIME_ABSOLUTE',
    )
```

— Cycles' own files, `cycles_5.2/properties.py`, lines 520–528, in the scene-level Cycles properties — read 2026-07-30

**Not covered.** The sources do not say how the estimate is calculated, how it is worded or formatted, whether it updates, whether it exists for every kind of task or only some, or whether an animation render estimates the remainder of the whole frame range or only of the current frame. Nothing states that the estimate is available anywhere except as a tooltip.

**Where I looked, for the parts that are silent.** I searched all of `manual_5.2`, `api_5.2`, `release_notes_5.2` and Cycles' Python files, case-insensitively, for: `time estimate`, `estimate`, `estimated`, `remaining`, `time remaining`, `remaining time`, `time left`, `ETA`, `elapsed`, `progress bar`, `Render Time`, `Peak`. The single relevant hit is the status-bar sentence quoted above. I read in full the Status Bar page, the Preferences ‣ Interface page's Status Bar section, the Cycles Sampling page, the Metadata page, and the Command Line Arguments page.

**Observed, 2026-08-05 — Blender does print an estimate with no window.**

Same computer and same way of running as in question 1. Blender 5.2.0 LTS, `-b`, own settings folder, Cycles.

Some of the `render` log lines carry a field labelled `Remaining:`. It sits between the frame number and the memory figure. From a single-frame render at 16 samples:

```
00:00.689  render           | Fra: 1 | Remaining: 00:00.04 | Mem: 2M | Sample 1/16
```

From a slower graphics-card render of the same file, where it appeared on several lines running and fell each time:

```
00:01.011  render           | Fra: 1 | Remaining: 00:02.43 | Mem: 1M | Sample 1/16
00:01.014  render           | Fra: 1 | Remaining: 00:01.15 | Mem: 1M | Sample 2/16
00:01.017  render           | Fra: 1 | Remaining: 00:00.50 | Mem: 1M | Sample 4/16
00:01.022  render           | Fra: 1 | Remaining: 00:00.17 | Mem: 1M | Sample 8/16
```

The word is `Remaining:`, with a space after the colon, and the figure is in minutes, seconds and hundredths. It does not appear on every line: it was absent from the `Sample 0/16` line, from the final `Sample 16/16` line and from the `Finished` line.

**What it covers.** In the three-frame animation a `Remaining:` field appeared three times, once per frame, and read the same small figure each time. The three, as the `render_stats` handler received them in that one run — note that they are word for word identical, because the frame number does not reach the handler, so which frame each belongs to rests on the order they printed in and not on anything in the text:

```
RENDER_STATS: ('Remaining: 00:00.01 | Mem: 2M | Sample 1/16', None)
RENDER_STATS: ('Remaining: 00:00.01 | Mem: 2M | Sample 1/16', None)
RENDER_STATS: ('Remaining: 00:00.01 | Mem: 2M | Sample 1/16', None)
```

No line in the run gave a remaining figure for the animation as a whole. The EEVEE run printed no `Remaining:` field at all, so it is not offered by every engine.

**Observed, 2026-08-05 — what the tooltip on the progress bar says.** Run by the Creator on their own MacBook Pro (Apple M1 Pro, macOS 26.5.2), Blender 5.2.0, reported as screenshots. The scene was Cycles at 1280 by 720 with 1024 samples and a frame range of 1 to 6.

Resting the pointer on the Status Bar progress bar brings up a box of **two lines**. Word for word:

```
Time Remaining: 00:09.19
Time Elapsed: 00:24.92.
```

So the wording is `Time Remaining: ` and `Time Elapsed: `, the figures are in minutes, seconds and hundredths, and the second line ends with a full stop. It gives both a forecast and a time already spent.

**What each figure sat beside.** At the moment that tooltip was showing, the Blender Render window's own status line read `Frame:1 | Time:00:18.73 | Remaining: 00:06.83`. Those are the numbers as they appeared; nothing further is read into them here.

**Observed, 2026-08-06 — the printed estimate on a second machine.**

Run on **a rented Linux machine — Ubuntu 22.04.5 LTS, NVIDIA GeForce RTX 4090, driver 580.105.08, twelve cores, run over SSH**. Blender Blender 5.2.0 LTS, `-b`, its own settings folder, display turned off first. It agreed with the Mac in every respect: the field is spelled `Remaining:` with one space after the colon, in minutes, seconds and hundredths, sitting between the frame number and the memory figure, and it does not appear on every line.

**What it covers, and which frame each figure belongs to.** In the three-frame animation a `Remaining:` field appeared once inside each frame and the value started over each time. The printed lines carry the frame number, so which frame each belongs to is plain — this is what the equivalent Mac record could not show, since the `render_stats` string strips it:

```
00:00.406  render           | Fra: 1 | Remaining: 00:00.03 | Mem: 2M | Sample 1/16
00:00.676  render           | Fra: 2 | Remaining: 00:00.02 | Mem: 2M | Sample 1/16
00:00.820  render           | Fra: 3 | Remaining: 00:00.01 | Mem: 2M | Sample 1/16
```

No line in the run gave a remaining figure for the animation as a whole, and the EEVEE run printed no `Remaining:` field at all. Both machines were run and both agreed.

**Outside the interface.** Settled by the test above, and it is the opposite of what the sources suggested. An estimate is printed with no window: the `Remaining:` field on Cycles' `render` log lines. It restarts at each frame, so what it covers is the current frame, not the whole range. It also reaches Python, inside the string the `render_stats` handler is handed. The sources describe only a tooltip on the Status Bar, which does need the window; that tooltip has since been photographed by the Creator and is quoted above, worded `Time Remaining:` with a companion `Time Elapsed:` line. (Citation check 2026-08-05: this used to send the tooltip off to a test still to be done. It had already been done and written into this same question, two paragraphs up.)

**Operating system.** The sources state no platform condition. The Creator's tooltip screenshots were got on macOS 26.5.2 only. The printed `Remaining:` field was got on Linux as well, and the paragraph below says so. (Citation check 2026-08-06: this sentence used to say all of it was macOS-only and that neither other system could be tested, which the Linux run recorded below contradicts. Narrowed to the screenshots.)

The printed `Remaining:` field was got on **both** machines — the Creator's MacBook Pro (Apple M1 Pro, macOS 26.5.2) and the rented Linux machine — and they agreed on its spelling, its units, where it appears and what it covers, so it holds on macOS and on Linux. **Windows is untested and no Windows machine has run it.** The two-line tooltip was seen on macOS only.

---

## 6. What does Blender report about the memory it is using on a graphics card?

**Label: documented**, and thin.

There is one setting that is unambiguously about graphics-card memory, and it is in the API documentation rather than the manual:

> "show_statusbar_vram
>
> Show GPU video memory usage (default False)"

— API documentation, https://docs.blender.org/api/5.2/bpy.types.PreferencesView.html — read 2026-07-30

So: Blender can show GPU video memory usage in the status bar, and it does not do so unless switched on.

Notably, the manual's own list of what the status bar can show does not include it. The manual lists exactly five items — Scene Statistics, Scene Duration, System Memory, Extensions Updates, Blender Version — and the memory one is system RAM, not the card:

> "System Memory
>
> Shows an estimate of Blender's RAM consumption. On a single-instance single-machine scenario, this estimate provides a measurement against the hardware limit of the machine."

— Manual, https://docs.blender.org/manual/en/5.2/interface/window_system/status_bar.html and https://docs.blender.org/manual/en/5.2/editors/preferences/interface.html — read 2026-07-30

The manual's picture of the right-hand end of the status bar (`_images/interface_window-system_status-bar_ui-right.png`, on the Status Bar page) shows `Collection | Cube | Verts:8 | Faces:6 | Tris:12 | Objects:0/3 | Mem: 29.7 MiB | v2.81.16`. So the memory readout there is labelled `Mem:` and given in MiB to one decimal place — but the version string in that same picture reads `v2.81.16`, so the screenshot predates 5.2, and no video-memory figure appears in it. **documented** (manual image), read 2026-07-30. I take from it only what is visible: the label and unit style used for the system-memory readout in that screenshot. It shows nothing about the VRAM readout.

**Memory reporting that is not stated to be graphics-card memory.** Three mechanisms report memory during a render, and none of them says whether the figure is system memory, card memory, or both:

> "update_memory_stats(*, memory_used=0.0, memory_peak=0.0)
>
> Update memory usage statistics
>
> Parameters:
>
> memory_used (float) – Current memory usage in megabytes (in [0, inf], optional)
>
> memory_peak (float) – Peak memory usage in megabytes (in [0, inf], optional)"

— API documentation, https://docs.blender.org/api/5.2/bpy.types.RenderEngine.html — read 2026-07-30

> "--cycles-print-stats
>
> Log statistics about render memory and time usage."

— Manual, https://docs.blender.org/manual/en/5.2/advanced/command_line/arguments.html — read 2026-07-30

> "Memory
>
> Includes the peak memory usage."

(one of the metadata items that can be stamped into the rendered file) — Manual, https://docs.blender.org/manual/en/5.2/render/output/properties/metadata.html — read 2026-07-30

**What the sources say about knowing GPU memory at all.** For EEVEE, explicitly, it cannot be predicted:

> "There is no standard way of estimating if the resources will fit into the GPU memory and/or if the GPU will render them successfully."

— Manual, https://docs.blender.org/manual/en/5.2/render/eevee/limitations/limitations.html — read 2026-07-30

And for Cycles, the closest the manual comes to a statement about card memory during a render is the fallback behaviour and the out-of-memory error, both quoted under question 1.

**Not covered.** The sources do not say what the VRAM readout looks like — no units, no format, no label, no picture of it. They do not say whether the figure is Blender's own consumption, the whole device's consumption, or the device's capacity. They do not say whether it updates during a render, whether it covers every enabled render device when more than one card is in use, or which backends (CUDA, OptiX, HIP, oneAPI, Metal) it works with. There is no Python property documented anywhere that hands a script a GPU memory figure — `show_statusbar_vram` is a preference that turns a display on, not a reading. The metadata "Memory" item and `update_memory_stats` are not said to be graphics-card memory, and I will not assume they are.

**Where I looked, for those gaps.** I searched all of `manual_5.2`, `api_5.2`, `release_notes_5.2` and Cycles' Python files, case-insensitively, for: `VRAM`, `video memory`, `GPU memory`, `device memory`, `graphics memory`, `memory usage`, `used memory`, `peak`, `Mem:`, `statistics`, `stats`. The only statement tying a *report* to the graphics card is `show_statusbar_vram`. Everything else found was either a setting that *consumes* GPU memory (EEVEE volume resolution, Cycles tile size, denoising on GPU) or a memory figure not attributed to the card. I read in full: the Status Bar page, the Preferences ‣ Interface page, the Metadata page, the Cycles GPU Rendering page, the Cycles Performance page, the EEVEE Limitations page, the Command Line Arguments page, `bpy.types.PreferencesView.txt` around the status-bar settings, and `bpy.types.RenderEngine.txt` in full. In Cycles' own files I searched `properties.py` and `ui.py` for `memory`; the only memory-named things there are the Performance ‣ Memory panel (which holds `tile_size` only) and the `peer_memory` preference, "Make more room for large scenes to fit by distributing memory across interconnected devices (e.g. via NVLink) rather than duplicating it" (`cycles_5.2/properties.py`, lines 1664–1666) — a setting, not a report.

**Observed, 2026-08-05 — the setting exists, and nothing printed is marked as the card's memory.**

Same computer and same way of running as in question 1. Blender 5.2.0 LTS, `-b`, own settings folder.

**The setting exists and is off.** A script read the preference and printed:

```
preferences.view.show_statusbar_vram => False
preferences.view.show_statusbar_memory => False
```

so both status-bar memory switches are present in 5.2 and both start off. What the readout then shows needs the window and is entry 21 of `Audits/Blender Research/Reports/Hand tests.md`.

**A render on the graphics card.** A script turned the Metal device on and set the scene to render on it. It printed:

```
DEVICE: 'Apple M1 Pro' 'CPU' use = True
DEVICE: 'Apple M1 Pro (GPU - 14 cores)' 'METAL' use = True
SCENE DEVICE: GPU
```

The render then ran on that device and the log lines were the same shape as a processor render, with the same single `Mem:` field and no second one:

```
00:01.011  render           | Fra: 1 | Remaining: 00:02.43 | Mem: 1M | Sample 1/16
00:01.166  render           | Fra: 1 | Mem: 2M | Finished
```

No field, label or word anywhere in the run named the graphics card's memory. Searching the whole output for "VRAM", "video memory", "GPU memory" and "device memory" matched nothing.

A separate run of the same file that selected the device through the Cycles command-line switch instead — `-- --cycles-device METAL` — printed a much larger figure on the same field:

```
00:00.992  render           | Fra: 1 | Mem: 1458M | Finished
```

Both figures carry the same label, `Mem:`, and the same unit, `M`. Nothing in either run says what memory the figure counts.

**With the Cycles statistics switch added.** The same graphics-card render with `-- --cycles-print-stats` produced the statistics block quoted in question 4. Its memory figures are:

```
Mesh statistics:
  Geometry:
    Total memory: 386B (386)
Image statistics:
  Total image memory: 0B (0)
```

Those are labelled by what the memory holds — geometry, images — not by which device it is on. They carry a unit letter, `B`, and a second figure in brackets. Nothing in the block names a card, a device or an amount of card memory.

**Observed, 2026-08-05 — the Status Bar offers no video-memory item.** Run by the Creator on their own MacBook Pro (Apple M1 Pro, macOS 26.5.2), Blender 5.2.0, reported as screenshots.

Right-clicking the Status Bar opens a menu headed `Status Bar` holding **exactly five items**, each with a tick box:

```
Scene Statistics
Scene Duration
System Memory
Extensions Updates
Blender Version
```

**No item mentions video memory or GPU memory.** That is the whole menu; nothing is cut off in the picture.

The Status Bar itself, in that shot, read `Duration: 00:00+06 (Frame 1/6) | Memory: 28.7 MiB | 5.2.0`. So the one memory figure on offer there is labelled `Memory:` and given in MiB to one decimal place — and by the manual's own text that item is System Memory, not the graphics card. (The manual's own old screenshot of this bar shows the label as `Mem:`; what this version draws is `Memory:`.)

**A gap in what was reported.** The instruction also said that, if the right-click menu had no such item, to look in Edit ▸ Preferences ▸ Interface ▸ Status Bar instead. No screenshot of that panel was returned, and no screenshot shows the readout switched on or the text it produces. So what `show_statusbar_vram` displays is still not known for this version. It is not filled in here.

**Observed, 2026-08-06 — what the video-memory readout actually says.**

Run on **a rented Linux machine — Ubuntu 22.04.5 LTS, NVIDIA GeForce RTX 4090, driver 580.105.08, twelve cores, run over SSH**, with Blender's window open on that machine's own display, pointed at its own settings folder. A script set `bpy.context.preferences.view.show_statusbar_vram = True` and `show_statusbar_memory = True`, saved the preferences with `bpy.ops.wm.save_userpref()`, waited five seconds, photographed the window with `bpy.ops.screen.screenshot` and quit. It printed:

```
BEFORE vram = False  memory = False
AFTER  vram = True  memory = True
```

**The readout appeared, and this is what it says.** The right-hand end of the Status Bar in that picture reads, word for word:

```
Collection | Cube | Verts:8 | Faces:6 | Tris:12 | Objects:1/3 | Memory: 20.9 MiB | VRAM: 1.0/24.0 GiB | 5.2.0
```

*(Narrowed 2026-08-06 by the runner, after the check of the Linux hand tests sent this back. The check read the same bar again on a quiet card and got 1.2, 1.1, 1.1 and 1.2 in the four versions, and 1.7 and 1.8 on a busier one — so the left-hand figure is not fixed and must not be quoted as though it were. What the runs do carry, identically every time, is the label `VRAM:`, the used-of-total shape, the `GiB` units, and the right-hand figure of `24.0`, which is the whole of this card's memory. The left-hand figure moves with what is on the card, and how it is counted is still not settled — both the check's readings and this document's are on the record.)*


So the item is labelled **`VRAM:`**, and it is a **used-of-total** figure in **GiB**, to one decimal place, with a `/` between the two numbers. It sits between the system-memory item and the version. The right-hand figure, `24.0 GiB`, is this card's whole memory — `nvidia-smi` reports the card as 24564 MiB. The same words appeared in all four versions.

**Whether the left-hand figure is Blender's own use or the card's — the moving test was run in this version.** A heavy Cycles render was started on the card (4096 by 4096, 400 samples, CUDA) and the window was photographed five times during it, with `nvidia-smi` read at each shot. The readout moved: `VRAM: 4.0/24.0 GiB` when `nvidia-smi` reported 3673 MiB used on the card, and `VRAM: 3.8/24.0 GiB` when it reported 3459 MiB. So it tracks the card's overall use and is not equal to it. At rest it read `VRAM: 1.0/24.0 GiB` at a moment when `nvidia-smi` put the card's entire usage at 568 MiB. **What the left-hand figure counts is therefore not settled** — it is neither the card's reported total nor plainly Blender's own share. Nothing further is read into it here. The machine was shared with other work, so other processes may have held some of the card's memory at any of those moments. The system-memory item beside it switched its unit from `MiB` to `GiB` as the figure grew.

**Still not reached, on either machine.** Whether the Status Bar's own right-click menu offers this item. On the Mac the menu was photographed and holds exactly five items, none of them video memory. On the Linux machine the menu could not be opened at all: a sweep of `bpy.types` for anything with `STATUSBAR` in it returns only `STATUSBAR_HT_header`, two theme entries and the preferences panel, so there is no menu to call by name, and there is no tool on that machine to send a right-click into the window. The preference can be switched on from Python, which is what the block above does.

**Observed, 2026-08-06 — what a render on an NVIDIA card prints about memory.**

Run on the rented Linux machine, Blender Blender 5.2.0 LTS, `-b`, display turned off first. A script set the Cycles device type to CUDA and turned the devices on. It printed:

```
DEVICE: 'NVIDIA GeForce RTX 4090' 'CUDA' use = True
DEVICE: 'AMD Ryzen Threadripper PRO 3975WX 32-Cores' 'CPU' use = True
DEVICE: 'NVIDIA GeForce RTX 4090' 'OPTIX' use = True
SCENE DEVICE: GPU
```

**The ordinary progress lines named no card memory**, exactly as on the Mac with Metal. The `Mem:` field was the same shape and size as on a processor render and no second one appeared.

**`--debug-cycles` is different, and this is the closest Blender comes to reporting card memory.** Added to the same run, it wrote on standard output as `cycles` log lines in this version, and carried these, word for word:

```
00:00.443  cycles           | Mapped host memory limit set to 80,647,581,696 bytes. (75.11G)
00:00.631  cycles           | Local memory reserved 1,132,462,080 bytes. (1.05G)
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

Run over SSH on a **second rented Linux machine — Ubuntu 22.04.5 LTS, two NVIDIA GeForce RTX 4090s of 24 GB each on driver 580.95.05, an AMD EPYC 7B13 with sixty-one cores, 98 GB of memory, no display server at all** — using Blender at `/opt/blender/blender-5.2.0-linux-x64/blender`, `Blender 5.2.0 LTS (hash fbe6228777e7 built 2026-07-14 01:32:04)`, with `BLENDER_USER_RESOURCES` pointed at `/root/smoozy/two cards/res-5.2`, a folder of its own that was empty at the start. **Nothing else was running on the machine.** Label: **observed**. This is a third machine; nothing here is merged with the Creator's Mac or with the one-card rented Linux machine. The full list is `Audits/Blender Research/Reports/Hand tests — two cards and no screen.md`.

**The two cards are not joined by NVLink.** `nvidia-smi topo -m` reports `PHB` between them — `Connection traversing PCIe as well as a PCIe Host Bridge (typically the CPU)`. So the NVLink question this topic also owes is still out of reach.

**Blender reports one figure, never two, and never names a card beside a memory number.** The same Cycles scene at 1920 by 1080 and 6000 samples, rendered with both cards ticked and then with one, gave these `Finished` lines:

```
two cards: 00:31.079  render           | Fra: 1 | Mem: 8M | Finished
one card:  00:57.508  render           | Fra: 1 | Mem: 1101M | Finished
```

**The memory figure this version prints is smaller with two cards than with one** — `Mem: 8M` against `Mem: 1101M` above — not larger and not doubled, and no second figure ever appears. What that field counts is not settled by this test and nothing is read into it. The heavier scene — a 3000 by 3000 subdivided grid, 868 MB on disk — was run in Blender 4.5 alone and its figures are recorded in 4.5's own findings; it was not run in this version. *(Narrowed by the two-card citation check, 2026-08-06: this claim was worded as "the second `Mem:` field", which is 4.5's line shape — this version prints one — and rested on a 4.5 figure, when this version's own two runs carry it.)*

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
                            |   Usage: 15,868,742 (15.13M)
                            |   Peak: 31,145,248 (29.70M)
```

with `Mapped host memory limit set to 101,923,811,328 bytes. (94.92G)` printed once per card, and `Local memory reserved 1,132,462,080 bytes. (1.05G)` likewise once per card. **No line anywhere states how much of either card's memory is in use.**

**`-- --cycles-print-stats` with two cards has the same shape as with one and names no device:**

```
Render statistics:
Mesh statistics:
  Geometry:
    Total memory: 4.18M (4,381,164)
        Suzanne                          4.08M (4,282,453)
        Sphere                           16.02K (16,405)
        Sphere.001                       16.02K (16,405)
        Sphere.002                       16.02K (16,405)
        Sphere.003                       16.02K (16,405)
        Sphere.004                       16.02K (16,405)
Image statistics:
  Textures:
    Total memory: 0 (0)
Profiling information not available (only works with CPU rendering)
```

The memory figures are still labelled by what the memory holds — geometry, textures — not by which card it is on, and the profiling line is the same one a single-card render gives.

**Observed, 2026-08-06 — what the memory readouts say with four graphics cards, and what happens when a card is filled.**

Run over SSH on a **third rented Linux machine — Ubuntu 22.04.5 LTS, four NVIDIA GeForce RTX 4060 Ti of 16 GB each on driver 580.95.05, an AMD EPYC 7R43 with sixty-one cores, 197 GB of memory, no display server** — using Blender at `/opt/blender/blender-5.2.0-linux-x64/blender`, `Blender 5.2.0 LTS (hash fbe6228777e7 built 2026-07-14 01:32:04)`, with `BLENDER_USER_RESOURCES` pointed at `/root/smoozy/four cards/res-5.2`, a folder of its own emptied and remade before use. **Nothing else was running on the machine.** Label: **observed**. This is a **fourth** machine; nothing here is merged with the Creator's Mac, the one-card rented Linux machine or the two-card rented Linux machine. The full list, with every command and every script written out, is `Audits/Blender Research/Reports/Hand tests — four cards.md`.

**Three things separate this machine from the two-card one, and all three bear on what is below.** Four cards rather than two. A **smaller** card — 16 GB against 24 GB, the smallest the project has had, so no figure here may be set beside a two-card figure and none is. And **197 GB of system memory** against 98 GB, which is what finally made it possible to fill a card while the machine's own memory was nowhere near full — the exact case this question asks about, and one no earlier machine could produce.

**The four cards are not joined by NVLink** — `nvidia-smi topo -m` reports `PHB` between every pair — so the NVLink question this topic also owes is still out of reach.

**Blender reports one figure, never four, and never names a card beside a memory number.** The same Cycles scene at 1920 by 1080 and 3000 samples, rendered with four cards ticked, with two and with one, gave these `Finished` lines:

```
four cards: 00:26.687  render           | Fra: 1 | Mem: 8M | Finished
two cards:  00:43.612  render           | Fra: 1 | Mem: 8M | Finished
one card:   01:20.385  render           | Fra: 1 | Mem: 397M | Finished
```

**The figure is far smaller with more than one card, and it does not go on shrinking.** Two cards and four cards printed exactly the same figure — the single `Mem:` field, `8M` against `397M` between four cards and one — and the same held on a heavier scene, a grid of 3000 by 3000 subdivisions at 909 MB on disk, where four cards and two cards both printed `Mem:480.88M, Peak:494.94M` in Blender 4.5 against `Mem:1224.33M, Peak:1694.79M` on one card. So the figure does not count memory divided by the number of cards. What it does count is not settled by this test and nothing is read into it. In Blender 4.5 the **first** `Mem:` field moves the other way — `103.60M` with four cards, `95.44M` with two, `91.37M` with one — and nothing is read into that either.

**A graphics card was filled while the machine's own memory held, which no earlier attempt had managed.** The scene is a plane whose surface samples a growing number of picture files of 8192 by 8192 pixels, each read through its own Mapping node so that none can be left out, added together and divided by their count; 640 by 480, 16 samples, one card ticked. The card is 16,380 MiB. What came back:

```
################ 64 textures of 8192 by 8192, one card
exit=0
00:22.555  render           | Fra: 1 | Mem: 16741M | Finished
00:24.325  render           | Time: 00:23.88 (Saving: 00:00.10)
   card 0 15682MiB
   card 1 3MiB
   card 2 3MiB
   card 3 3MiB
################ 128 textures of 8192 by 8192, one card
exit=0
03:28.690  render           | Fra: 1 | Mem: 33125M | Finished
03:33.172  render           | Time: 03:32.71 (Saving: 00:00.10)
   card 0 15714MiB
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

**Outside the interface.** Settled in part. With no window, Blender 5.2 printed no graphics-card memory figure at all, on a render that was actually running on the graphics card. The `Mem:` field and the `--cycles-print-stats` totals appeared with the same labels whether the render used the processor or the card, and nothing labelled any of them as the card's. The one documented graphics-card readout, `show_statusbar_vram`, is present in this version and set off, and reading it needs the window.

**Operating system.** The sources state no platform condition on `show_statusbar_vram`. They do restrict the backends that would be doing the work: CUDA and OptiX "supported on Windows and Linux", HIP "supported on Windows and Linux", oneAPI "supported on Windows and Linux", Metal "supported on Apple computers with Apple Silicon" (Manual, https://docs.blender.org/manual/en/5.2/render/cycles/gpu_rendering.html, read 2026-07-30). Whether the status-bar figure works on all of them is not said. The Creator's tests were run on macOS 26.5.2 with Apple's own graphics through Metal only, so they settle that one combination. The rented Linux machine with an NVIDIA card was run too, and the paragraph below says what it settled; every other card and system is still open. (Citation check 2026-08-06: this sentence used to end by saying there was no machine here to test the others on, which the Linux run recorded below contradicts.)

The Creator's Status Bar menu screenshot and the Metal render were got on macOS 26.5.2 with Apple's own graphics. The `VRAM:` readout, the CUDA render and the `--debug-cycles` figures above were got on the rented Linux machine with an NVIDIA GeForce RTX 4090. **Both machines were run and they agree that nothing in the ordinary printed output is marked as the card's memory.** They differ in what could be reached: only Linux could show the readout, and only the Mac could photograph the right-click menu.

**Windows is untested and no Windows machine has run it.** **More than one card is no longer out of reach** — a third machine with two NVIDIA GeForce RTX 4090s answered it on 2026-08-06 and its result is in the block above: one figure, never two, and each card holding the same amount as the other. **A card running out of memory while system memory remains is no longer out of reach either** — it was reached on 2026-08-06 on a fourth machine with four NVIDIA GeForce RTX 4060 Ti of 16 GB each and 197 GB of system memory, and what happened is in the four-card block above. *(Corrected by the four-card citation check, 2026-08-06: this line still said the test was written out and never run, which the block above it and the closing list below it both contradict.)* Still out of reach: an NVLink bridge; the `VRAM:` Status Bar readout with two cards, the two-card machine having no display server and the machine with a display having one card; and any card that is neither Apple's nor NVIDIA's, so HIP and oneAPI are unreachable. `--debug-cycles` on the Linux machine records `WARNING HIPEW initialization failed: Error opening HIP dynamic library`, which is what an absent AMD card looks like there.

---

## Things a person with Blender 5.2 open could still settle in a few minutes

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

2026-08-04 — Opus 5 (Claude Code) — Reports, Blender 5.2

2026-08-05 — Opus 5 (Claude Code) — added the observed findings from the hand tests to questions 1 to 6, brought the two standing lines up to date where a test settled them, and pruned the list above

2026-08-05 — Opus 5 (Claude Code) — added the Creator's own window tests to questions 1, 2, 3, 5 and 6, and pruned the list above again

2026-08-05 — Opus 5 (Claude Code) — citation check: added a missing connecting citation in question 4, narrowed eight claims to what their quotes and their own records carry, removed one line that contradicted a test recorded in the same question, and sent one thinly recorded observation to the hand tests. Each change is noted where it was made.

2026-08-05 — Opus 5 (Claude Code) — closed the one item the citation check sent to the hand tests: replaced the screenshot note about the Output panel's header, under question 1, with the Creator's own answer, the tooltip they quoted, and the steps they followed
2026-08-06 — Opus 5 (Claude Code) — added the hand tests run on a rented Linux machine with an NVIDIA card to questions 1 to 6, kept separate from the Mac's results throughout; answered what the video-memory readout displays, where a crash file goes on Linux, what the log-file switch catches, which lines quiet mode removes, what the render operator hands a script, and six failures the Mac could not reach; brought both standing lines up to date on every question and emptied the list at the end

# Reports — Blender 5.2 — second pass (questions 1, 2, 3)

**Sources consulted, and when.** Blender 5.2 manual, Blender 5.2 Python API documentation, Blender 5.2 release notes, and the Cycles add-on's own files, all as supplied in `Blender Sources/` and all taken on **2026-07-30**. That snapshot date is the date read for every finding below. Manual pictures in the `_images` folder were opened where named.

**I read only the question sheet and files inside Blender Sources/. I did not open any other part of this project.**

I did the work alone. No helper agents were started, so there is nobody else's reading to vouch for.

A note on how this pass was run, since it is the point of it: every question below carries a full record of what was searched, with the search terms and the files opened, including the searches that found nothing. Searching was done across the whole manual and the whole API documentation at once, not file by file.

---

## 1. What can cause a render to fail, and what does Blender do in each case?

**Short answer.** The sources name eight distinct things that can make a render fail or stop, plus one more that comes from Cycles' own files. They are scattered across the manual rather than gathered in one place, and — this matters — for most of them the sources say only that "an error is raised" or name the error message, without saying what happens to the render job afterwards.

### The causes, one at a time

**(a) No valid way to save the output.** This is new behaviour in 5.2, and it is the failure the sources are clearest about.

Release note: "Saving render output can now be disabled. (blender/blender@342f37566f)"
— Release notes, https://developer.blender.org/docs/release_notes/5.2/rendering/, read 2026-07-30

Manual: "When disabled, Blender does not write render output from this panel and instead relies on other systems. For animation renders, if the Video Sequencer is active and no alternative output is available, an error is raised. Otherwise, if compositing is enabled, active File Output Node nodes are used to save the result. If no such nodes exist, an error is reported."

And: "Playblast operations also require an active output method. Both Render Playblast and Render Playblast on Keyframes will raise an error when render output is disabled in this panel."
— Manual, https://docs.blender.org/manual/en/5.2/render/output/properties/output.html, read 2026-07-30

Manual, saying the same thing from the other side: "If no valid output method is available, rendering operations will report errors."

And: "This method relies on automatic output saving being enabled. If Save Output is disabled, the render will fail unless an alternative output method is provided."
— Manual, https://docs.blender.org/manual/en/5.2/render/output/animation.html, read 2026-07-30

The switch itself, in the Python API: "save_output — Write frames to disk for animation renders (default True)"
— API documentation, https://docs.blender.org/api/5.2/bpy.types.RenderSettings.html, read 2026-07-30

LABEL: **documented**. What Blender does: raises or reports an error. The sources do not say the error's wording, nor whether the job stops at once.

**(b) A broken template in the output path.** "Note — Depending on the path, template errors may prevent certain actions. For example, if there are errors in the render output path, then rendering an animation will fail with an error message indicating the path errors."
— Manual, https://docs.blender.org/manual/en/5.2/files/file_paths.html, read 2026-07-30

That page also shows the path field turning red and a tooltip listing the errors. The tooltip picture (`_images/files_file_paths-invalid_path_template_tooltip.png`, named on that same page) reads, for the example path `//my_render_dir/{blend_name.png`:

> Syntax error(s):
> - Invalid or incomplete template expression '{blend_name.png'.

LABEL: **documented** (the picture is part of the manual). Note the tooltip is what the *path field* says before you render; the wording of the message that a failed animation render produces is not given.

**(c) Not enough memory on the graphics card.** Two documented behaviours here, and they are different from each other.

First, an automatic fallback: "With CUDA, OptiX, HIP and Metal devices, if the GPU memory is full Blender will automatically try to use system memory. This has a performance impact, but will usually still result in a faster render than using CPU rendering."

Second, an error: under the heading "Error Messages", "Error: Out of memory — This usually means there is not enough memory to store the scene for use by the GPU."
— Manual, https://docs.blender.org/manual/en/5.2/render/cycles/gpu_rendering.html, read 2026-07-30

LABEL: **documented**. The fallback list names CUDA, OptiX, HIP and Metal — oneAPI is not in that list. What Blender does when the fallback is unavailable or also fails is not said beyond showing that message.

Release notes record this area was worked on: "Out of GPU Memory error on file that renders in 5.0.1" is listed among fixed issues.
— Release notes, https://developer.blender.org/docs/release_notes/5.2/bugfixes/, read 2026-07-30

**(d) A graphics card too new for the installed CUDA kernels.** "CUDA Error: Kernel compilation failed — This error may happen if you have a new NVIDIA graphics card that is not yet supported by the Blender version and CUDA toolkit you have installed. In this case Blender may try to dynamically build a kernel for your graphics card and fail."
— Manual, https://docs.blender.org/manual/en/5.2/render/cycles/gpu_rendering.html, read 2026-07-30

LABEL: **documented**. What Blender does: tries to build a kernel, and the build fails. Whether the render then falls back to the CPU or stops is not said.

**(e) An unsupported compiler version, on Linux only.** "Unsupported GNU version — On Linux, depending on your GCC version you might get this error." The page then describes setting `CYCLES_CUDA_EXTRA_CFLAGS` or editing `host_config.h`, and says the fix "will allow Cycles to successfully compile the CUDA rendering kernel the first time it attempts to use your GPU for rendering."
— Manual, https://docs.blender.org/manual/en/5.2/render/cycles/gpu_rendering.html, read 2026-07-30

LABEL: **documented**. Linux only, by the source's own words.

**(f) The display driver timing the render out, on Windows.** "The NVIDIA OpenGL driver lost connection with the display driver — If a GPU is used for both display and rendering, Windows has a limit on the time the GPU can do render computations. If you have a particularly heavy scene, Cycles can take up too much GPU time. Reducing Tile Size in the Performance panel may alleviate the issue, but the only real solution is to use separate graphics cards for display and rendering."
— Manual, https://docs.blender.org/manual/en/5.2/render/cycles/gpu_rendering.html, read 2026-07-30

LABEL: **documented**. Windows only, and NVIDIA, by the source's own words. What Blender does is not stated — the message quoted is the driver's, not Blender's.

**(g) Not enough shadow memory in EEVEE.** "Tip — The error message Shadow buffer full means that the system cannot allocate enough shadow memory. Increasing the Shadow Pool Size or the Resolution Limit on some lights can fix the issue. Otherwise, the only workaround is to disable shadow casting on some lights."
— Manual, https://docs.blender.org/manual/en/5.2/render/eevee/light_settings.html, read 2026-07-30

The other end of the same message: "Increasing the size might fix the Shadow buffer full error."
— Manual, https://docs.blender.org/manual/en/5.2/render/eevee/render_settings/performance.html, read 2026-07-30

LABEL: **documented**. The sources call this an error message; they do not say the render stops.

**(h) Running out of memory altogether, which crashes Blender.** "The most common causes of Blender crashes: Running out of memory. Issues with graphics hardware or drivers. Bugs in Blender."

What Blender does: "When Blender crashes, it writes out a text file which contains information that may help identify the cause of the crash. Usually, this file is written in the Temporary Directory directory."
— Manual, https://docs.blender.org/manual/en/5.2/troubleshooting/crash.html, read 2026-07-30

LABEL: **documented**. A crash is not a reported render failure — Blender is gone. The page names the crash file per system: on Windows `test.crash.txt` beside the loaded blend-file's name and `blender.crash.txt` for the Blender module; on macOS the system Crash Reporter plus files in `~/Library/Logs/DiagnosticReports/`; on Linux `blender.crash.txt` written to `/tmp`.

**(i) An unusable Cycles device given on the command line.** From Cycles' own files:

```
    if args.cycles_device:
        import _cycles
        if not _cycles.set_device_override(args.cycles_device):
            sys.exit(1)
```
— `Blender Sources/cycles_5.2/engine.py`, lines 38–41, inside `_parse_command_line()`, which is called from `init()` at line 53. Read 2026-07-30.

LABEL: **observed** for what the file says. That Blender therefore quits, and quits with status 1, is a step past the file — that step is **inferred**: `sys.exit(1)` is Python's way of ending the process with that status, but whether Blender's embedded interpreter lets it end the whole application is not shown by this file.

### What is not here

The sources never say, for any of these, whether a failed frame stops an animation render or is skipped, whether the remaining frames are attempted, or what is left on disk. There is no documented text for a general "render failed" message. Searching all four sources for the phrases *render failed*, *failed to render*, *could not render*, *unable to render*, *rendering failed*, *stopped rendering*, *aborting*, *render was cancelled*, *render is cancelled* returned nothing at all in the manual, the API documentation or the release notes (the sole hit anywhere was an unrelated code comment in `cycles_5.2/source/kernel/film/read.h`, line 144, about the adaptive sampler). Nor is there anything about writing failing: *unable to write*, *cannot write*, *failed to save*, *could not save*, *cannot open*, *cannot be saved*, *no write permission*, *disk full* and *out of disk* all returned zero hits across all four sources.

**Ambiguity flagged.** "Fail" could mean "ends with an error" or could include "was cancelled by the user". I have answered the first reading. On the second: cancelling is documented and is treated as its own thing, not as an error — the Status Bar has a cancel button ("The task can be aborted by clicking the cancel button ()") and Python has a dedicated handler, `bpy.app.handlers.render_cancel`, "on canceling a render job".

**Not covered by the sources.** What Blender does *after* raising each error — stop, skip, continue, what exit status it leaves, what partial files remain. Whether these are all the causes; the manual has no single list of them, so this is a gathered list. Whether EEVEE's "Shadow buffer full" and Cycles' driver-timeout case stop a render at all. The first of those was settled by tests instead; see the observed block below. The last two need a graphics card the Creator does not have, and remain open.

**Observed, 2026-08-05 — nine failure situations tried from a terminal.**

How all of these were run, once for all of them. On the Creator's own computer: a MacBook Pro with an Apple M1 Pro chip, macOS 26.5.2, Apple's own graphics and no other graphics card. Blender 5.2.0 LTS, at `/Applications/Blender.app`, started with no window (the `-b` switch) and with a throwaway settings folder of its own. Unless said otherwise the file was the factory startup scene — one cube, one camera, one light — saved by Blender 4.5 so every version could open it, with render engine Cycles, 16 samples, 160 by 120 pixels, frames 1 to 3, PNG output. Each run's two output streams were kept in separate files and the shell's exit status was read straight afterwards. The full commands are in `Audits/Blender Research/Reports/Hand tests.md`, entries 7 to 12, 18 and 19. The same nine runs, with their whole output, are also written into `Preparation/Blender Research/Findings/5.2/Reports.md`.

**Cause (a), no valid way to save the output — the wording the sources withhold.** Run: `-b "simple.blend" --python-expr "import bpy; bpy.context.scene.render.save_output = False" -o "<folder>/out2_52/frame_" -F PNG -x 1 -a`. Nothing was written to disk. Standard error was empty. The whole of standard output:

```
00:00.397  blend            | Read blend: "/private/tmp/.../018/simple.blend"
00:00.401  reports          | ERROR Render output disabled in Output properties and no active compositing File Output nodes
Blender 5.2.0 LTS (hash fbe6228777e7 built 2026-07-14 01:31:22)

Blender quit
```

The exit status was **0**. So the error is reported at once, before any frame is rendered, and the job still leaves the process looking successful.

**Cause (b), a broken template in the output path.** Run: `-b "simple.blend" -o "<folder>/out_52/{blend_name.png" -F PNG -x 1 -a`. Exit status 1, standard error empty, nothing written. The message, which the answer above says the sources do not give for a render:

```
00:00.406  reports          | ERROR Parse errors in path '/private/tmp/.../018/out_52/{blend_name.png':
                            | - Invalid or incomplete template expression '{blend_name.png'.
```

Its second line is word for word what the manual's tooltip picture shows for the path field. The render stopped before frame 1 — no `Sample` line was printed and the output folder was empty.

**Cause (i), the Cycles device override — the inference above, run.** Run: `-b "simple.blend" -o "<folder>/out_52/frame_" -F PNG -f 1 -- --cycles-device NONSENSE`. Exit status 1, standard error empty, whole of standard output:

```
00:00.390  cycles           | ERROR NONSENSE is not a valid Cycles device.
Blender 5.2.0 LTS (hash fbe6228777e7 built 2026-07-14 01:31:22)
Error: Not freed memory blocks: 1, total unfreed memory 0.000004 MB
```

No frame was rendered. So a Cycles device Cycles will not take ends the whole Blender process with status 1 before any render happens. The same command with `--cycles-device METAL` rendered normally and ended with status 0. (Citation check 2026-08-05: this used to say the `sys.exit(1)` in `engine.py` is what does it. The run shows the status and the missing frame; it does not show which line of code ended the process — which is the very step the label at (i) above is careful to say the file cannot carry.)

**What the sources never name at all, tried anyway.** Six more:

*No camera in the scene.* Exit status 1, nothing written, whole message: `00:00.407  reports          | ERROR Cannot render, no camera`

*The output folder does not exist.* Blender made it and rendered into it. Exit status 0, no message of any kind.

*The output path cannot be written* — a plain file was put where the folder would have to be. Exit status 1, nothing written, two lines:

```
00:00.534  image            | ERROR Couldn't create directory for file /private/tmp/.../blocker_52/frame_0001.png: File exists
00:00.534  reports          | ERROR Render error (File exists) cannot save: '/private/tmp/.../blocker_52/frame_0001.png'
```

*The blend-file does not exist.* Exit status 1: `00:00.399  reports          | ERROR Cannot read file "…": No such file or directory`

*A missing image texture.* Exit status 0, the frame was written anyway, two lines:

```
00:00.413  cycles           | WARNING Image file /private/tmp/.../gone.png does not exist.
00:00.413  cycles           | ERROR Failed to load 1 image files
```

*An animation whose output folder stops being writable partway.* This is the question the answer above marks as never stated: whether a failed frame stops the job. A six-frame animation was started and its output folder was replaced with a plain file after frame 1 was written. Blender printed the two error lines for frame 2 and then **stopped** — no further `Rendering frame` line appeared, frames 3 to 6 were never attempted, and exactly one file existed afterwards, `frame_0001.png`. No part-written file was left. Exit status 1.

**Observed, 2026-08-05 — failures repeated with Blender's window open.** Run by the Creator on their own MacBook Pro (Apple M1 Pro, macOS 26.5.2), Blender 5.2.0, reported as screenshots.

*Cause (b), a broken template in the output path.* A file whose output path read `//out_{blend_name.png` was rendered with Ctrl-F12. **A Blender Render window did open**, showing an empty checkerboard and no image. The error appeared in three places at once (quoted under question 2). Before the render, the path field itself was drawn with a red background, and resting the pointer on it gave a tooltip reading, word for word:

```
Output Path
Directory/name to save animations, # characters define the position and padding of frame numbers.
Value: //out_{blend_name.png
Path template error(s):
  - Invalid or incomplete template expression '{blend_name.png'.
Python: RenderSettings.filepath
bpy.data.scenes["Scene"].render.filepath
```

The error line itself is word for word what the manual's own tooltip picture shows. The line above it is not: the manual's picture reads `Syntax error(s):` and this build's live tooltip reads `Path template error(s):`. (Citation check 2026-08-05: this used to say the whole wording was the same as the picture's; the two differ on that one line.)

*A cause the sources never name.* A scene with its camera deleted was rendered with F12. No Blender Render window opened, and an error was shown in three places (quoted under question 2).

**Observed, 2026-08-05 — cause (a)'s switch in the Output panel's header, looked at by hand.** Done by the Creator on their own MacBook Pro (Apple M1 Pro, macOS 26.5.2), in Blender 5.2.

What was asked of them, the same in all four versions: open Blender at its normal startup file, go to the Properties editor, choose the Output tab, find the panel headed `Output`, and look at the header strip of that panel next to the word `Output`. If a checkbox is drawn there, rest the pointer on it and wait for the tooltip.

What came back: **5.2 has a checkbox in that header, and it is the only one of the four versions that does.** In the Creator's own words, "This checkbox only exists in 5.2." Its tooltip reads, word for word:

```
Save Output
Write frames to disk for animation renders.
```

So 4.5, 5.0 and 5.1 were looked at and have no such checkbox — a result, not a silence.

The Creator works on a Mac, so this settles macOS and leaves Windows and Linux open. Their answer does not say whether the box was ticked or empty, whether the tooltip carries any line below those two, or anything about the checkboxes inside the panel. Those are left open rather than filled in.

(Citation check 2026-08-05: this used to be a note about what a screenshot showed, with nothing about what was done to get the screenshot, so nobody could repeat it — and it is the record that carries the whole finding that saving render output can be switched off. The check asked for the procedure, and the record above replaces it. The steps are in `Audits/Blender Research/Reports/Hand tests.md`, entry 24.)

**Observed, 2026-08-06 — the same causes, and six more, on a second machine.**

Run on **a rented Linux machine — Ubuntu 22.04.5 LTS, NVIDIA GeForce RTX 4090, driver 580.105.08, twelve cores, run over SSH**. Blender 5.2.0 LTS, `-b`, its own settings folder, display turned off first. Both machines were run; the whole set is in `Preparation/Blender Research/Findings/5.2/Reports.md`.

**The nine held, word for word.** Every message and every exit status came back identically on Linux, including the one this version alone has:

```
00:00.373  reports          | ERROR Render output disabled in Output properties and no active compositing File Output nodes
```

nothing written, exit status **0**. The handler probe adds something new about it: **not one render handler fires in that case**, not even `render_init`. The render never begins.

**A genuinely read-only output folder — new.** On that machine the user is root, and root walks through file permissions, so the run was made twice. Run plain, the permission was ignored and the file was written. Run again with the overriding permission dropped for the one command, exit status 1, nothing written, and:

```
00:00.638  image.write      | ERROR OpenImageIO write failed: Could not open file "/root/smoozy/Reports/work/L25/ro_52/frame_0001.png"
00:00.638  reports          | ERROR Render error (Success) cannot save: '/root/smoozy/Reports/work/L25/ro_52/frame_0001.png'
```

**The word in brackets reads `Success`** on a failure caused by a refused permission.

**A corrupt blend-file — new.** A file cut short gave exit 1 and `reports | ERROR Failed to read blend file '…': Missing DNA block`. A file that was not a blend-file gave exit 1 and `reports | ERROR File format is not supported in file "…"`.

**A crash, and the crash file on Linux — new, and it refines cause (h).** Blender was made to crash on purpose. With a file loaded it printed `Writing: /tmp/simple.crash.txt` and wrote that file; with `--factory-startup` and no file it printed `Writing: /tmp/blender.crash.txt`. Process status **139** both times. So on Linux the crash file is named after the blend-file when there is one — the manual's page names only the second form.

**Cause (c), the graphics-card memory fallback, was still not reached.** It needs a card whose memory can be exhausted while the machine's own memory is not. The rented machine can do it, but only with nothing else running on it, and it was shared. The test is written out ready to run under "Needs the machine to itself" in `Audits/Blender Research/Reports/Hand tests — Linux.md`. Causes (e) and the kernel-build failure were not reached either: there is no CUDA toolkit on that machine — `which nvcc` returns nothing — and this build ships a kernel already made for that card, so no compile is ever attempted.

**Outside the interface.** Settled by the tests above for the cases run. All nine were run with `-b` and no window at all. Every message went to standard output; standard error was empty in all nine. A failure that stops the job leaves the process with status 1 and a success leaves 0 — with two exceptions, both worth knowing: a missing image texture, and cause (a) itself, output saving switched off with nothing else arranged, which writes nothing and still leaves 0. The GPU causes (c)–(f) could not be tested on this machine.

**Operating system.** Three of these are explicitly limited: (e) is Linux only, (f) is Windows only (and NVIDIA), and the crash-file behaviour in (h) differs per system as quoted above. Cause (c)'s fallback is limited by device backend rather than by system, but the backends are themselves tied to systems — the same page says CUDA, OptiX, HIP and oneAPI are "supported on Windows and Linux", and Metal "is supported on Apple computers with Apple Silicon", with "macOS 13.0 or newer is required to support all features". For the rest, the sources do not say.

**Two machines have now been used.** macOS is settled by the Creator's own MacBook Pro (Apple M1 Pro, macOS 26.5.2); Linux by the rented Linux machine, for the same nine causes plus six more recorded above. The two agreed everywhere. **Windows is untested and no Windows machine has run it**, so cause (f) and the Windows crash-file behaviour stay open. Cause (e), the Linux "Unsupported GNU version", **could not be reached even on the Linux machine**: it needs a CUDA kernel to be built while Blender runs, and there is no CUDA toolkit on that machine while the build ships a kernel already made for the card. **HIP** needs an AMD card and **oneAPI** an Intel card; neither machine has one.

### Where I looked

Searched the whole manual, the whole API documentation, the whole release notes and the whole Cycles folder.

- Terms searched across all four sources: `fail`, `render fail` / `fail render` (within 20 characters either way), `render failed`, `failed to render`, `could not render`, `unable to render`, `rendering failed`, `stopped rendering`, `aborting`, `render was cancelled`, `render is cancelled`, `out of memory`, `unable to write`, `cannot write`, `failed to save`, `could not save`, `cannot open`, `cannot be saved`, `no write permission`, `disk full`, `out of disk`, `exit code`, `exit status`, `return code`, `nonzero`, `non-zero`.
- Terms searched across the manual: `no camera`, `cannot render`, `unable to`, `not writable`, `read-only`, `disk`, `permission`, `aborted`, `abort`, `cancel`, `press esc`, `error message`.
- Terms searched within the manual's `render/` and `compositing/` folders: `error`, `cannot`, `can't`, `invalid`.
- Files opened in full or in the relevant part: `render/cycles/gpu_rendering.txt`, `render/output/animation.txt`, `render/output/properties/output.txt`, `render/output/introduction.txt`, `render/introduction.txt`, `files/file_paths.txt`, `render/eevee/light_settings.txt`, `troubleshooting/crash.txt`, `troubleshooting/index.txt`, `troubleshooting/gpu/index.txt`, `troubleshooting/startup.txt`, `troubleshooting/3d_view.txt`, `advanced/command_line/render.txt`, `advanced/command_line/arguments.txt`, `compositing/types/output/warning.txt`, `render/shader_nodes/utilities/repeat.txt`, `render/cycles/render_settings/performance.txt`.
- Pictures opened: `_images/files_file_paths-invalid_path_template_tooltip.png`, `_images/repeat-zone-dynamic-iterations-not-supported.png`.
- Cycles files opened: `engine.py`, `__init__.py`, `operators.py`, `osl.py`, `version_update.py`, and `properties.py` around the render-passes settings; `properties.py`, `ui.py`, `engine.py` and `__init__.py` searched for `error`, `report`, `progress`, `fail`, `warn`, `stat`, `profil`, `verbose`, `log`.
- Release notes: `index.md`, `rendering.md`, `cycles.md`, `user_interface.md`, `sequencer.md`, `python_api.md` read; the whole folder searched for `progress`, `report`, `error`, `fail`, `status bar`, `console`.
- Ruled out and why: `compositing/types/output/warning.txt` — the Warning node's `Error` severity is described as "Indicates an invalid configuration that is likely to produce incorrect results", which is a message a node group author writes, not a render failure. `render/shader_nodes/utilities/repeat.txt` and its picture — the node-level message "Iterations input has to be a constant value" is a shading limitation shown on a node, and the page does not say a render fails because of it. `cycles_5.2/operators.py` — the errors there ("Frame '%s' not found, animation must be complete") belong to the Denoise Animation and Merge Images operators, not to rendering. `troubleshooting/3d_view.txt` and `troubleshooting/startup.txt` — their failures are about selection and about Blender not starting. `cycles_5.2/maketx.py` — its `sys.stderr.write("Error: …")` belongs to texture conversion, not rendering.

---

## 2. Where does Blender say that a render failed, and what does it say there?

**Short answer.** The sources document four places messages appear — the Status Bar, the Info editor, Python's report list, and the system console or log — and one place for crashes, the crash file. What they say is documented only in fragments: a handful of exact error strings and a severity list. There is no documented wording for a general render-failure message.

### The places

**The Status Bar, briefly.** "Report Message — Informational messages or warnings, such as after saving a file. They disappear after a short time. Click them to show the full message in the Info Editor."
— Manual, https://docs.blender.org/manual/en/5.2/interface/window_system/status_bar.html, read 2026-07-30

**The Info editor, kept.** "The Info editor logs the executed operators as well as errors, warnings, and informational messages. You can select an entry by clicking it, optionally holding Shift to add it to the existing selection."
— Manual, https://docs.blender.org/manual/en/5.2/editors/info_editor.html, read 2026-07-30

That page's own picture, `_images/editors_info-editor_ui.png`, shows three logged operator calls (`bpy.ops.curve.primitive_bezier_curve_add(...)`, `bpy.ops.object.text_add(...)`, `bpy.ops.object.empty_add(...)`), each with a green tick icon. No error or warning entry is visible in it, so the picture does not show what a failure entry looks like.

The API documentation adds that the editor does not log everything: "Blender records operators you run and displays them in the Info editor." and "Not all operators get registered for display, zooming the view for example isn't so useful to repeat so it's excluded from the output."
— API documentation, https://docs.blender.org/api/5.2/info_api_reference.html, read 2026-07-30

**Python's report list — and this is new in 5.2.** Release note: "Blender reports are now exposed as a read-only list in WindowManager ID types. They also now have a session-wide unique identifier integer (blender/blender!156439, blender/blender@79e02e1405)."
— Release notes, https://developer.blender.org/docs/release_notes/5.2/python_api/, read 2026-07-30

The list: "reports — Collection of reports (default None, readonly)", of type "bpy_prop_collection[Report]"
— API documentation, https://docs.blender.org/api/5.2/bpy.types.WindowManager.html, read 2026-07-30

What one entry holds: "Report entry"; "message — Report message text (default "", readonly, never None)"; "session_uid — Unique per-session report identifier (in [-inf, inf], default 0, readonly)"; "type — Report type (severity) (default 'DEBUG', readonly)", typed `Literal[Wm Report Items]`.
— API documentation, https://docs.blender.org/api/5.2/bpy.types.Report.html, read 2026-07-30

**What severities exist.** The full list, verbatim: `DEBUG` ("Debug."), `INFO` ("Info."), `OPERATOR` ("Operator."), `PROPERTY` ("Property."), `WARNING` ("Warning."), `ERROR` ("Error."), `ERROR_INVALID_INPUT` ("Invalid Input."), `ERROR_INVALID_CONTEXT` ("Invalid Context."), `ERROR_OUT_OF_MEMORY` ("Out of Memory.")
— API documentation, https://docs.blender.org/api/5.2/bpy_types_enum_items/wm_report_items.html, read 2026-07-30

That there is a dedicated `ERROR_OUT_OF_MEMORY` severity is worth marking: running out of memory is the one failure with its own machine-readable kind.

**What a render engine can put there.** "report(type, message) — Report info, warning or error messages", with `type` a "set[Literal[Wm Report Items]]" and `message` a "Report Message, (never None)".

And separately: "error_set(message) — Set error message displaying after the render is finished".
— API documentation, https://docs.blender.org/api/5.2/bpy.types.RenderEngine.html, read 2026-07-30

`error_set` is the closest thing in the sources to a render-failure message, and note its wording: the message displays *after the render is finished*. Where it displays is not said.

**The system console or terminal.** Two documented cases:

"Compilation errors will be shown in the system console."
— Manual, https://docs.blender.org/manual/en/5.2/render/cycles/osl/index.html, read 2026-07-30

"Tip — When you view the System Console, you can see some of the output of the encoding process. You will see even more output if you execute Blender as blender -d."
— Manual, https://docs.blender.org/manual/en/5.2/render/output/properties/output.html, read 2026-07-30

On Windows the console is a separate window: "To display the console again, go to Window ‣ Toggle System Console."
— Manual, https://docs.blender.org/manual/en/5.2/advanced/command_line/launch/windows.html, read 2026-07-30

**The log, if you turn it on.** "--log <match> — Enable logging categories, taking a single comma separated argument", with the given example `--log "render,cycles": both render and cycles messages`. Verbosity is set by "--log-level <level>", whose levels are listed as "fatal: Fatal errors only", "error: Errors only", "warning: Warnings", "info: Information about devices, files, configuration, operations", "debug: Verbose messages for developers", "trace: Very verbose code execution tracing". Output can be diverted: "--log-file <filepath> — Set a file to output the log to." And the categories can be listed: "--log-list-categories — List all available logging categories for --log, and exit."
— Manual, https://docs.blender.org/manual/en/5.2/advanced/command_line/arguments.html, read 2026-07-30

**The crash file, if Blender dies.** Quoted under question 1(h).

**The operator's return value.** `bpy.ops.render.render(...)` "Returns: Result of the operator call.", of type "set[Literal[Operator Return Items]]".
— API documentation, https://docs.blender.org/api/5.2/bpy.ops.render.html, read 2026-07-30

The possible values are `RUNNING_MODAL` ("Keep the operator running with blender."), `CANCELLED` ("The operator exited without doing anything, so no undo entry should be pushed."), `FINISHED` ("The operator exited after completing its action."), `PASS_THROUGH` ("Do nothing and pass the event on."), `INTERFACE` ("Handled but not executed (popup menus).").
— API documentation, https://docs.blender.org/api/5.2/bpy_types_enum_items/operator_return_items.html, read 2026-07-30

There is no `FAILED` among them. `CANCELLED` is defined as the operator having done nothing, which is not the same as a render that started and then broke.

LABEL for everything above: **documented**.

### What it actually says there

The sources give only these exact strings, all quoted under question 1: `Error: Out of memory`; `CUDA Error: Kernel compilation failed`; `Shadow buffer full`; `Unsupported GNU version`; `The NVIDIA OpenGL driver lost connection with the display driver` (this last is the driver's own message, quoted as a heading). Plus, from a picture, the path-field tooltip `Syntax error(s): - Invalid or incomplete template expression '{blend_name.png'.`

For the failures in question 1(a) — no valid output method — the sources say an error "is raised" or "is reported" and give no wording at all.

### What is missing, and I looked hard for it

There is **no render-failure handler in Python**. The complete list of handlers on `bpy.app.handlers` is: `animation_playback_post`, `animation_playback_pre`, `annotation_post`, `annotation_pre`, `blend_import_post`, `blend_import_pre`, `composite_cancel`, `composite_post`, `composite_pre`, `depsgraph_update_post`, `depsgraph_update_pre`, `exit_pre`, `frame_change_post`, `frame_change_pre`, `load_factory_preferences_post`, `load_factory_startup_post`, `load_post`, `load_post_fail`, `load_pre`, `object_bake_cancel`, `object_bake_complete`, `object_bake_pre`, `redo_post`, `redo_pre`, `render_cancel`, `render_complete`, `render_init`, `render_post`, `render_pre`, `render_stats`, `render_write`, `save_post`, `save_post_fail`, `save_pre`, `translation_update_post`, `undo_post`, `undo_pre`, `version_update`, `xr_session_start_pre`, `persistent`.
— API documentation, https://docs.blender.org/api/5.2/bpy.app.handlers.html, read 2026-07-30

The render handlers are described as: "render_cancel — on canceling a render job", "render_complete — on completion of render job", "render_init — on initialization of a render job", "render_post — on render (after)", "render_pre — on render (before)", "render_write — on writing a render frame (directly after the frame is written)", each "Accepts one argument: the scene data-block being rendered".

LABEL for the list itself: **documented**. That the list contains no render-failure handler while it *does* contain `load_post_fail` ("on failure to load a new blend file (after)") and `save_post_fail` ("on failure to save a blend file (after)") is a reading of the complete list, so: **inferred** — the step is that this page is the complete set of handlers, which its structure implies but does not state in words.

**Not covered.** The exact wording of any message produced by a failed render. Whether a failed render produces a Status Bar report at all, or only a console line. Where `RenderEngine.error_set`'s message is displayed. Whether the Info editor keeps a failure entry or whether it disappears like a Status Bar message. What exit status a failed render leaves — searched for and not found (see question 1's record). Whether `WindowManager.reports` is populated during a render or only after.

**Observed, 2026-08-05 — where a failed render says so with no window, and what it says.**

Same computer and same way of running as in question 1: a MacBook Pro with an Apple M1 Pro chip, macOS 26.5.2, Apple's own graphics, Blender 5.2.0 LTS started with `-b` and its own throwaway settings folder.

**The place is the log, on standard output.** Every failure message in the nine runs above appeared on standard output, under a log category, in the shape `<elapsed time>  <category>          | <LEVEL> <message>`. Standard error was empty in all nine, and in every other 5.2 run made. Three categories carried failure messages: `reports`, `image` and `cycles`. So operator reports do reach the log in background mode — the category is literally called `reports`.

**The wordings, which the answer above says are missing.** Quoted from the runs, complete:

```
00:00.407  reports          | ERROR Cannot render, no camera
00:00.401  reports          | ERROR Render output disabled in Output properties and no active compositing File Output nodes
00:00.399  reports          | ERROR Cannot read file "…": No such file or directory
00:00.534  reports          | ERROR Render error (File exists) cannot save: '…'
00:00.534  image            | ERROR Couldn't create directory for file …: File exists
00:00.390  cycles           | ERROR NONSENSE is not a valid Cycles device.
00:00.413  cycles           | WARNING Image file … does not exist.
00:00.413  cycles           | ERROR Failed to load 1 image files
00:00.406  reports          | ERROR Parse errors in path '…':
                            | - Invalid or incomplete template expression '…'.
```

The last shows how a report that runs over more than one line is laid out: the timestamp and category are left blank and the vertical bar is kept.

**What exit status a failed render leaves — the item the answer above lists as searched for and not found.** Read with `echo $?` straight after each run. Success: 0. No camera: 1. Cannot save: 1. Broken path template: 1. Blend-file missing: 1. Bad Cycles device: 1. Animation that lost its output folder: 1. Missing image texture, which finished anyway: 0. Output saving disabled with nothing else arranged, which wrote nothing: 0.

**Whether `WindowManager.reports` is populated in background mode — the gap the answer above names.** A failing render was run and a script then read the list. Command: `-b "no camera.blend" -o "<folder>/frame_" -F PNG -f 1 --python "reports.py"`, where the script is:

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

The whole of standard output:

```
00:00.389  blend            | Read blend: "/Users/artist/Documents/Smoozy hand tests/Reports/23/no camera.blend"
00:00.393  render           | Rendering single frame
00:00.393  render           | Rendering frame 1
00:00.393  reports          | ERROR Cannot render, no camera
Blender 5.2.0 LTS (hash fbe6228777e7 built 2026-07-14 01:31:22)
REPORTS COUNT: 0

Blender quit
```

The attribute is present — the same script printed `REPORTS: attribute not present in this version` when run in Blender 4.5, 5.0 and 5.1 — and after this failed background render it held nothing.

**Which handlers fire, given there is no failure handler in the list.** A script attached a printing handler to `render_init`, `render_pre`, `render_post`, `render_write`, `render_complete`, `render_cancel` and `render_stats`. On a successful render the order was `render_init`, `render_pre`, many `render_stats`, then `render_post`, `render_write`, `render_complete`. On the no-camera file the whole of what printed was:

```
PROBE INSTALLED
HANDLER FIRED: render_init args: (bpy.data.scenes['Scene'], None)
HANDLER FIRED: render_pre args: (bpy.data.scenes['Scene'], None)
HANDLER FIRED: render_cancel args: (bpy.data.scenes['Scene'], None)
```

So this failure — the no-camera file — fires `render_cancel` and not `render_complete`. (Citation check 2026-08-05: this used to say "a failure". Only the no-camera failure was probed this way, so the sentence was narrowed to it.)

**Observed, 2026-08-05 — where a failed render is announced with the window open, and what it says.** Run by the Creator on their own MacBook Pro (Apple M1 Pro, macOS 26.5.2), Blender 5.2.0, reported as screenshots. The Info editor was on screen with its error and warning filters on.

**A scene with no camera.** The message appeared in **three places at once**, each carrying a round white-on-red cross icon:

- along the bottom of the window, on a red field: `No camera found in scene "Scene"`
- in a floating box in the middle of the screen headed `Report: Error`: `No camera found in scene "Scene"`
- as an entry in the Info editor: `No camera found in scene "Scene"`

**The window's wording differs from the terminal's.** Headless, the same file printed `reports          | ERROR Cannot render, no camera`. In the window it reads `No camera found in scene "Scene"`. Same file, same machine, same day.

**A broken template in the output path.** Again in three places, with the same icon:

- along the bottom of the window: `Parse errors in path '//out_{blend_name.png': - Invalid or incomplete template expression '{blend_name.png'.`
- in a floating box headed `Report: Error`, on two lines: `Parse errors in path '//out_{blend_name.png':` then, indented, `- Invalid or incomplete template expression '{blend_name.png'.`
- in the Info editor, on two lines, the same words.

Here the two settings agree. So the answer above, which said the sources give no wording for a render-failure message and no statement about whether the terminal and the interface match, now has both: for one failure they match and for the other they do not.

**The Info editor shows a failure entry.** Both messages are visible as Info-editor entries in the screenshots, each with the red cross icon. Whether the entry then stays or disappears was not captured, and is not filled in here. (Citation check 2026-08-05: the heading used to read "does keep a failure entry", which claims the very thing the second sentence says was not captured.)

**`WindowManager.reports` with a window.** Not covered by these screenshots. Headless, it was present and empty after a failure. What it holds when there is a window is still unknown.

**Observed, 2026-08-06 — the two things this answer left hanging, on a second machine.**

Run on **a rented Linux machine — Ubuntu 22.04.5 LTS, NVIDIA GeForce RTX 4090, driver 580.105.08, twelve cores, run over SSH**. Blender 5.2.0 LTS, its own settings folder.

**Whether `WindowManager.reports` fills when there is a window.** It was read in four settings: after a no-camera failure with no window, after a blocked save, after output saving switched off, and **after a no-camera failure with the window open**. Every one printed:

```
REPORTS COUNT: 0
```

An operator error unrelated to rendering left it empty too. So the list exists and answers the question in neither setting.

**Which handlers fire on four more kinds of failure.** The probe was run against five cases, not one. Blocked save: `render_init`, `render_pre`, `render_cancel`. Broken path template: `render_init` and `render_cancel` only — `render_pre` never fired. Missing texture, which succeeds: the full successful set. Output saving switched off: **nothing at all**.

**What the render operator hands a script.** Success returns `{'FINISHED'}`; failure raises `RuntimeError Error: Cannot render, no camera` with no window and `RuntimeError Error: No camera found in scene "Scene"` with one.

**What `--log-file` does to a failure message.** It moves the whole log into the file. On the no-camera file, standard output held only the version banner, a blank line and `Blender quit`, while the file held the four log lines ending `reports | ERROR Cannot render, no camera`. The exit status was still 1.

**Whether an Info-editor entry stays.** Not settled. On the Linux machine, with an Info editor on screen and its filters on, **no entry appeared at all** when the render was started from a script rather than by hand, so nothing follows either way.

**Outside the interface.** Settled by the tests above. In background mode a failed render's message is printed to standard output through the log, under `reports`, `image` or `cycles`; on the one failure probed for it, `render_cancel` fires; the process is left with status 1 in every case tried except a missing texture and disabled output saving, both of which leave 0; and `WindowManager.reports` exists but was empty. The Status Bar and the Info editor are still parts of the Blender window — the underlying report is plainly generated, since it appears in the log, but whether those two show it is untested. `--log-file` exists specifically to put messages in a file.

**Operating system.** The Status Bar, Info editor, report severities, handler list and log arguments are documented without any system qualification. One difference is documented: on Windows the system console is a separate window that must be shown with `Window ‣ Toggle System Console`, whereas the Linux and macOS pages describe running Blender from a terminal. The crash-file locations differ per system, as quoted under question 1(h). Otherwise the sources do not say.

**Both machines have now been run and they agree on the words.** Every failure message came back identically on macOS 26.5.2 and on the rented Linux machine, and the window's own `No camera found in scene "Scene"` was seen on both. **Windows is untested and no Windows machine has run it.** **Untried rather than untestable: a Windows machine can be rented, and every one this project has read reported a Server edition — see the equipment section of `Preparation/Blender Research/Method.md`.** On Linux the crash file goes to `/tmp` and is named after the blend-file when one is loaded, which is narrower than the page's own wording.

### Where I looked

- Terms searched across the manual and API documentation: `system console`, `toggle system console`, `terminal window`, `command prompt`, `error message`, `error messages`, `reports an error`, `report an error`, `reported in the`, `shown in the Info`, `reports are shown`, `reports is displayed`, `shown in the status bar`, `Info editor`, `stderr`, `stdout`, `standard error`, `standard output`, `status bar`, `statusbar`, `render_cancel`, `render_complete`, `render_stats`.
- API files opened: `bpy.app.handlers.txt` (all handlers listed), `bpy.types.RenderEngine.txt`, `bpy.types.Report.txt`, `bpy.types.WindowManager.txt`, `bpy.types.Operator.txt` (the `report` method), `bpy.ops.render.txt`, `bpy.app.txt`, `bpy_types_enum_items/wm_report_items.txt`, `bpy_types_enum_items/operator_return_items.txt`, `bpy_types_enum_items/wm_job_type_items.txt`, `info_api_reference.txt`, `info_tips_and_tricks.txt`, `bpy.types.RenderSettings.txt`.
- Manual files opened: `interface/window_system/status_bar.txt`, `editors/info_editor.txt`, `interface/window_system/topbar.txt` (Render menu), `editors/preferences/interface.txt` (the "Render In" preference), `advanced/command_line/arguments.txt`, `advanced/command_line/launch/windows.txt`, `render/cycles/osl/index.txt`, `render/output/properties/output.txt`, `troubleshooting/crash.txt`.
- Pictures opened: `_images/editors_info-editor_ui.png`.
- Ruled out and why: the Image editor pages (`editors/image/`) — searched for `error` and `fail` and returned nothing, so the render window is not documented as a place failures are announced. `interface/window_system/` as a whole — same search, only the Status Bar's report message. `bpy.ops.render.view_cancel` — "Cancel showing the render view", which is about hiding the render view, not about a render failing.

---

## 3. What does Blender report about how far through a piece of work it has got?

**Short answer.** For someone watching the window: a progress bar in the middle of the Status Bar, with a percentage on it, a name for the task, a cancel button, and a time estimate on hover. For a script: a percentage from 0 to 1, a free-text status line, and a statistics string whose contents are named but whose format is not.

### In the window

"Status Messages — The middle of the Status Bar displays information about in-progress operations."

"Running Task — Shows the progress of the currently running task (such as rendering or baking). Hovering the mouse pointer over the progress bar will display a time estimate. The task can be aborted by clicking the cancel button ()."
— Manual, https://docs.blender.org/manual/en/5.2/interface/window_system/status_bar.html, read 2026-07-30

(The empty parentheses are where an icon sat in the original page.)

That page names the picture `_images/interface_window-system_status-bar_ui-center.png`. I opened it. It shows, left to right: a small camera-like icon, the text **Sequence Render**, a horizontal bar filled roughly nine-tenths blue with **94%** written on it, and an **✕** button at the right end.

LABEL: **documented** (the picture is part of the manual). What is visible is visible: the task is named, the fill is proportional, the number is a whole-number percentage with a `%` sign, and the cancel control sits at the right. What is *not* visible and must not be read in: whether a render shows the same layout, what a render's task name is, and what the hover time estimate looks like.

Baking reports differently again: "Blender will become unresponsive during most baking operations. The cursor will display as a number representing the progress of the baking."
— Manual, https://docs.blender.org/manual/en/5.2/physics/baking.html, read 2026-07-30

LABEL: **documented**.

### To a script

Three separate things, and they report different amounts.

**A percentage.** "update_progress(progress) — Update progress percentage of render", parameter "progress (float) – Percentage of render that's done (in [0, 1])".

**A status line.** "update_stats(stats, info) — Update and signal to redraw render status text", parameters "stats (str) – Stats, (never None)" and "info (str) – Info, (never None)".
— API documentation, https://docs.blender.org/api/5.2/bpy.types.RenderEngine.html, read 2026-07-30

Both belong to `RenderEngine`, so they are how an engine *reports* progress rather than how a watching script *reads* it.

**A statistics string, pushed to a handler.** "render_stats — on printing render statistics. Accepts one argument: the render progress as a string containing current frame, current sample, render time and saving time."
— API documentation, https://docs.blender.org/api/5.2/bpy.app.handlers.html, read 2026-07-30

This is the richest thing in the sources on the question: four named quantities — **current frame**, **current sample**, **render time**, **saving time** — arriving as one string, per statistics print. The string's format is not given.

**A generic progress reporter for scripts.** "progress_begin(min, max) — Start progress report", with "min (float) – min, any value in range [0,9999]" and "max (float) – max, any value in range [min+1,9998]"; "progress_update(value)", with "value (float) – value, Any value between min and max as set in progress_begin()"; "progress_end() — Terminate progress report".
— API documentation, https://docs.blender.org/api/5.2/bpy.types.WindowManager.html, read 2026-07-30

**Whether a job is running at all, though not how far.** "bpy.app.is_job_running(job_type) — Check whether a job of the given type is running", with "job_type (str) – job type in Wm Job Type Items" and returning "Whether a job of the given type is currently running".
— API documentation, https://docs.blender.org/api/5.2/bpy.app.html, read 2026-07-30

The job types, verbatim: `RENDER` ("Regular rendering."), `RENDER_PREVIEW` ("Rendering previews."), `OBJECT_BAKE` ("Object Baking."), `COMPOSITE` ("Compositing."), `SHADER_COMPILATION` ("Shader compilation.")
— API documentation, https://docs.blender.org/api/5.2/bpy_types_enum_items/wm_job_type_items.html, read 2026-07-30

LABEL for all of the above: **documented**.

### What the release notes say about progress in 5.2

Three entries. The first is an explicit statement of change; the other two are titles of fixed bugs, so they name a problem that existed rather than describing how progress works: (Citation check 2026-08-05: this used to call all three "explicit statements of change".)

"The "Render Audio" operator now shows a realtime progress bar that can be cancelled (previously Blender would freeze until the operation finished). (blender/blender!155623)"
— Release notes, https://developer.blender.org/docs/release_notes/5.2/sequencer/, read 2026-07-30

Among the fixed issues: "Render Sequence Animation stopped showing total progress" and "Blender 5.1.1: Is Cycles render sample progress still accessible from Python without using -f?"
— Release notes, https://developer.blender.org/docs/release_notes/5.2/bugfixes/, read 2026-07-30

The second of those is a fixed issue reported as a question, and it names sample progress from Python as the subject. It is evidence that Cycles sample progress is reachable from Python in 5.2; it is not a statement of how.

LABEL: **documented**.

### One observation about Cycles' own files

Cycles' Python files do not call `update_progress` or `update_stats` anywhere. Searching all of `cycles_5.2/*.py` for `progress` returns exactly one line, and it is a note that something is missing:

```
        # TODO: support cancel and progress reports.
```
— `Blender Sources/cycles_5.2/operators.py`, line 97, inside `CYCLES_OT_denoise_animation.execute`. Read 2026-07-30.

LABEL: **observed**. The step *not* to take: this does not mean Cycles reports no progress. `CyclesRender.render()` in `__init__.py` (declared at line 73) hands straight off to `engine.render(self, depsgraph)` at line 74, and `engine.py` hands off to a compiled module `_cycles` which is not in this folder. (Citation check 2026-08-05: the line number used to read 71, which is `engine.reset(...)` in a different method.) So Cycles' progress reporting is simply out of reach of these files. The one line quoted is about the Denoise Animation operator only.

**Not covered.** The format of any progress text for a render — not the Status Bar's task name for a render, not the render status line, not the `render_stats` string. How often progress updates. Whether progress is per frame, per sample, per animation, or all three at once (the `render_stats` description names both a current frame and a current sample, which suggests both, but the string's structure is not given). Whether the Status Bar percentage for an animation covers the whole frame range or the current frame. What the hover time estimate says or how it is computed.

**Observed, 2026-08-05 — what progress a render prints with no window, and what the handler is handed.**

Same computer and same way of running as in question 1. Blender 5.2.0 LTS, `-b`, own settings folder, Cycles, 16 samples, 160 by 120 pixels, PNG output.

**A single frame.** Run: `-b "simple.blend" -o "<folder>/frame_" -F PNG -f 1`. 54 lines on standard output, nothing on standard error. First six and last six:

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

Between those two groups the run listed forty-odd preparation stages, one line each, such as `Updating Scene BVH | Building BVH 0%` and `Loading denoising kernels (may take a few minutes the first time)`. This is the format the answer above records as not given anywhere.

**Whether the percentage covers the frame or the range.** Run: `-b "simple.blend" -o "<folder>/frame_" -F PNG -x 1 -a`, frames 1 to 3. Exit status 0, three files, 150 lines. The frame boundaries are announced:

```
00:00.405  render           | Rendering animation (frames 1..3)
00:00.405  render           | Rendering frame 1
00:00.573  render           | Rendering frame 2
00:00.658  render           | Rendering frame 3
```

No percentage of any kind appeared in the whole run, and no figure covered the animation as a whole. The sample count started over at each frame. So outside the window, progress is per frame and, within a frame, per sample.

**What `render_stats` is handed — measured against the API's own description.** The API says the argument is "the render progress as a string containing current frame, current sample, render time and saving time". A handler that printed its arguments with `repr` fired 133 times over the three-frame animation. Every string it received, at each of the three frames, was the same set:

```
RENDER_STATS: ('Remaining: 00:00.01 | Mem: 2M | Sample 1/16', None)
RENDER_STATS: ('Mem: 2M | Sample 16/16', None)
RENDER_STATS: ('Mem: 2M | Finished', None)
RENDER_STATS: ('Time: 00:00.16 (Saving: 00:00.06)', None)
```

The current sample is there; the render time and saving time are there, on the last string of each frame; a memory figure is there as well. **The current frame is not.** The `Fra: N` field of the printed line does not reach the handler, and the strings were identical at frames 1, 2 and 3. The handler receives one string and a second value that printed as `None`, and it fires with no window open.

**A different engine reports differently.** The same file with the engine set to `BLENDER_EEVEE` printed 14 lines in all:

```
00:00.602  render           | Engine: EEVEE
00:00.790  render           | Fra: 1 | Rendering 1 / 64 samples
00:00.908  render           | Fra: 1 | Rendering 25 / 64 samples
00:00.982  render           | Fra: 1 | Rendering 50 / 64 samples
00:01.021  render           | Fra: 1 | Rendering 64 / 64 samples
```

No memory field, no remaining time, no per-stage lines.

**Observed, 2026-08-05 — what the window shows while a render runs.** Run by the Creator on their own MacBook Pro (Apple M1 Pro, macOS 26.5.2), Blender 5.2.0, reported as screenshots. The scene was Cycles at 1280 by 720 with 1024 samples and a frame range of 1 to 6.

**The Status Bar.** The task is named `Rendering...`, followed by a bar carrying a whole-number percentage and an `✕` button. Percentages seen: `73%` and `25%`.

**Whether the percentage covers the frame or the range — the question the answer above leaves open.** It tracks the samples within a frame. The bar read `25%` while the render window read `Sample 256/1024`, and `73%` while it read `Sample 748/1024`. Nothing on screen gave a figure across the whole six-frame range. (Citation check 2026-08-05: this used to open "It restarts at each frame and tracks the samples within it." Both status lines quoted below read `Frame:1`, and no reading from another frame was recorded, so nothing here shows what the bar does when the frame changes.)

**The Blender Render window's own status line**, quoted from the shots word for word:

```
Frame:1 | Time:00:18.73 | Remaining: 00:06.83 | Mem: 1506M | Sample 748/1024 (Using optimized kernels)
```

```
Frame:1 | Last:00:27.14 Time:00:05.69 | Remaining: 00:16.52 | Mem: 1506M | Sample 256/1024 (Using optimized kernels)
```

**The hover time estimate — also left open above.** Resting the pointer on the progress bar gives a box of two lines:

```
Time Remaining: 00:09.19
Time Elapsed: 00:24.92.
```

and, in another shot:

```
Time Remaining: 00:17.44
Time Elapsed: 00:05.81.
```

**Observed, 2026-08-06 — the same progress runs on a second machine.**

Run on **a rented Linux machine — Ubuntu 22.04.5 LTS, NVIDIA GeForce RTX 4090, driver 580.105.08, twelve cores, run over SSH**. Blender 5.2.0 LTS, `-b`, display turned off first. It agreed with the Mac on the line shape, the fields, the frame announcements and the counts. Two things it adds:

**Which frame a progress line belongs to.** The printed lines carry the frame number even though the `render_stats` string does not:

```
00:00.406  render           | Fra: 1 | Remaining: 00:00.03 | Mem: 2M | Sample 1/16
00:00.676  render           | Fra: 2 | Remaining: 00:00.02 | Mem: 2M | Sample 1/16
00:00.820  render           | Fra: 3 | Remaining: 00:00.01 | Mem: 2M | Sample 1/16
```

**Whether a program can read the progress as it happens.** A two-frame render was piped into a reader that stamped each line with the seconds since the run began. The lines arrived spread across the run: the line Blender stamped `00:05.874` reached the reader at 5.99 seconds, and the line stamped `00:11.494` reached it at 11.61 seconds. In a background render Cycles printed only three sample lines per frame, so there is little to read between them.

**Outside the interface.** Settled by the tests above. Progress is reported with no window: timestamped `render` log lines on standard output, carrying the frame number, a memory figure, sometimes a remaining time, and either a stage name or a sample count. The `render_stats` handler does fire in background mode and receives the message part of each of those lines. What does not appear outside the window is any percentage, and any figure covering the whole animation. The Status Bar route is part of the Blender window. `RenderEngine.update_progress` and `update_stats` are engine-side and were not reached by these tests.

**Operating system.** Nothing in any of the material above carries a system qualification. The printed progress was got on **both** machines — the Creator's MacBook Pro (Apple M1 Pro, macOS 26.5.2) and the rented Linux machine — and the two agreed, so it holds on macOS and on Linux. **Windows is untested and no Windows machine has run it.** The Creator's window tests were got on macOS only.

### Where I looked

- Terms searched across the manual and API documentation: `progress`, `progress_begin`, `progress_update`, `progress_end`, `percentage`, `elapsed`, `remaining`, `remaining time`, `time remaining`, `time estimate`, `estimated time`, `estimate`, `render status`, `status text`, `render info`, `Rendering .* sample`, `Mem:`, `Time:`, `Peak:`, `Render Time`, `status bar`, `statusbar`.
- The whole release-notes folder searched for `progress`, `report`, `error`, `fail`, `status bar`, `console`; `sample` and `frame` searched within `cycles.md`.
- The whole Cycles folder searched for `progress`, `stat`, `profil`, `verbose`, `log`.
- Manual files opened: `interface/window_system/status_bar.txt`, `physics/baking.txt`, `render/output/introduction.txt`, `render/introduction.txt`, `interface/window_system/topbar.txt`, `editors/preferences/interface.txt`, `advanced/command_line/arguments.txt`, `glossary/index.txt` (checked the `progress` hit — it is about mip-maps, not progress reporting), `compositing/introduction.txt` (checked — "as the clip progresses", unrelated).
- API files opened: `bpy.types.RenderEngine.txt`, `bpy.app.handlers.txt`, `bpy.types.WindowManager.txt`, `bpy.app.txt`, `bpy_types_enum_items/wm_job_type_items.txt`, `info_gotchas_internal_data_and_python_objects.txt` (its "see progress in the viewport" passage is about script authors redrawing, not about what Blender reports).
- Pictures opened: `_images/interface_window-system_status-bar_ui-center.png`.
- Ruled out and why: the many `progress` hits in `physics/fluid/...` are about simulation baking showing progress in the status bar and Esc pausing or aborting it — the same Status Bar mechanism, not a separate report. The `PreferencesView` hits in the API are translation-completion percentages for languages. The `NodeSocket…Percentage` hits are socket subtypes. `--cycles-print-stats` (manual: "Log statistics about render memory and time usage."; Cycles' own `engine.py` line 13–14: "Print rendering statistics to stderr") was found and set aside deliberately — it concerns statistics about memory and time, not how far through the work Blender has got, and belongs to a different question on the sheet.

---

## Things a person with Blender 5.2 open could still settle by hand in a few minutes

Every item that was on this list has now been run — items 6, 7, 8, 9, 11, 12 and the background half of 10 from a terminal, and items 1 to 5 by the Creator with the window open, all on 2026-08-05. Their results are written into the answers above.

One of the two smaller things the pictures did not reach has since been answered on a rented Linux machine: **`WindowManager.reports` is empty when there is a window too**, after four different failures. The other is still open: whether a failure entry **stays** in the Info editor or disappears. The Linux window run produced no Info-editor entry at all when the render was started from a script, so it settles nothing either way.

What could not be tested on either machine, and why: **EEVEE's "Shadow buffer full"** and **Cycles' graphics-card out-of-memory case** both need a card whose memory can be exhausted while the machine's own memory is not. The rented Linux machine can do it but was shared, so the test is written out ready to run under "Needs the machine to itself" in `Audits/Blender Research/Reports/Hand tests — Linux.md`. **Cycles' Windows driver-timeout case** needs Windows, and no Windows machine has run it.

---

2026-08-04 — Opus 5 (Claude Code) — Reports, Blender 5.2

2026-08-05 — Opus 5 (Claude Code) — added the observed findings from the hand tests to questions 1, 2 and 3, brought the two standing lines up to date where a test settled them, and pruned the list above

2026-08-05 — Opus 5 (Claude Code) — added the Creator's own window tests to questions 1, 2 and 3, and pruned the list above again

2026-08-05 — Opus 5 (Claude Code) — citation check: corrected one wrong line number, narrowed six claims to what their quotes and their own records carry, and sent one thinly recorded observation to the hand tests. Each change is noted where it was made.

2026-08-05 — Opus 5 (Claude Code) — closed the one item the citation check sent to the hand tests: replaced the screenshot note about the Output panel's header, under question 1, with the Creator's own answer, the tooltip they quoted, and the steps they followed

2026-08-06 — Opus 5 (Claude Code) — added the hand tests run on a rented Linux machine with an NVIDIA card to questions 1, 2 and 3, kept separate from the Mac's results; answered whether the reports list fills with a window, which handlers fire on four more failures, where a crash file goes on Linux, what the log-file switch does to a failure message, and a read-only folder and a corrupt file; brought both standing lines up to date and pruned the list above

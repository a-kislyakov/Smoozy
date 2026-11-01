# The machine — added questions, second pass (Blender 4.5)

**Sources consulted, and the date read.** Blender's manual for 4.5, Blender's Python API documentation for 4.5, Blender's release notes for 4.5, and Cycles' own files as shipped with 4.5 — all read 2026-07-30. Blender's requirements page — read 2026-08-01. Two pictures from the manual (named in the answer) were opened as well.

**I read only the question sheet and files inside `Blender Sources/`. I did not open any other part of this project.**

Scope: question 7 only. Every other question on the sheet was left alone.

---

## 7. Can anything a render does wait for a person, and what happens on a computer with nobody at it?

### The question was ambiguous, and this is the reading I answered

"Anything a render does" can be read narrowly — only the picture-making itself — or widely — the whole run, from starting Blender and loading the file through to the finished frame being written. **I answered the wide reading**, because on the narrow reading the answer is a single flat "nothing in the sources", and because everything I found that waits sits at the edges of the render rather than inside it. Where a finding belongs to the wide reading only, I say so.

The narrow reading would be: "once the picture is actually being computed, does that computation ever stop and wait for a person?" On that reading the answer is **not in the sources** — see the record of looking at the end.

### Short answer

Yes. Several things in Blender wait for a person, and two of them stand directly in a render's path: a warning box about running the file's scripts, and a warning box about unsaved changes. Both are things Blender draws in its own window. The sources also describe a mode with no window at all, meant for exactly this — rendering on a machine nobody is sitting at — and give command-line switches that pre-answer the script warning. **What the sources never say is what becomes of these boxes when there is no window to draw them in.** That gap is the important part of this answer.

---

### (a) What waits for a person — the script warning

Blender does not run a file's Python scripts unless told to, and when it declines it puts up a box.

> "The ability to include Python scripts within blend-files is valuable for advanced tasks such as rigging and automation. However, it poses a security risk since Python does not restrict what a script can do. Therefore, you should only run scripts from sources you know and trust. Automatic execution is disabled by default; however, some blend-files need this to function properly."

> "When a blend-file tries to execute a script and is not allowed, a dialog will appear. In it, you can choose to Allow Execution or to Ignore the scripts."

Source: the manual, https://docs.blender.org/manual/en/4.5/advanced/scripting/security.html — read 2026-07-30.

The same page carries a picture of that box (`_images/advanced_scripting_security_autorun-scripts-dialog.png`, on the page above). I opened it. It shows a warning triangle, the words "For security reasons, automatic execution of Python scripts in this file was disabled:", a named item "Text 'Text.py'", the line "This may lead to unexpected behavior", an unticked box "Permanently allow execution of scripts", and two buttons, "Allow Execution" and "Ignore". Label for what the picture shows: **documented** (it is part of the manual). The picture shows two buttons and no timer, no countdown, no default choice marked.

This matters to a render because scripts are part of how scenes are built:

> "Python expressions can be used to Drive values and are often used in more advanced rigs and animations."

Same page. And:

> "By default Blender will restrict execution of Python scripts. If using a Scripted Expression Driver Type that doesn't follow the Simple Expressions subset, you will have to open the file as Trusted Source, or set Auto Run Python Scripts in Preferences ‣ Save & Load ‣ Blender Files."

Source: the manual, https://docs.blender.org/manual/en/4.5/animation/drivers/troubleshooting.html — read 2026-07-30.

One kind of render-time script is explicitly exempt:

> "There are other ways scripts in a blend-file may execute that require user interaction (therefore will run even when auto execution is off), but you should be aware that this is the case since it is not necessarily obvious."
> "Running a script in the Text editor."
> "Rendering with Freestyle, because Freestyle uses scripts to control line styles."

Source: the manual, https://docs.blender.org/manual/en/4.5/advanced/scripting/security.html — read 2026-07-30.

The manual then addresses the exact situation of this question by name:

> "You may want to perform batch rendering or some other task from the command line, running Blender without an interface. In this case, the Preferences are still used but you may want to override them:"
> "Enable with -y or --enable-autoexec"
> "Disable with -Y or --disable-autoexec"
> "To render an animation in background mode, allowing drivers and other scripts to run:"
> "blender --background --enable-autoexec my_movie.blend --render-anim"
> "These command-line arguments can be used to start a regular Blender instance and will still override the Preferences."

Same page. The switches themselves:

> "-Y, --disable-autoexec"
> "Disable automatic Python script execution (Python-drivers & startup scripts), (default)."

Source: the manual, https://docs.blender.org/manual/en/4.5/advanced/command_line/arguments.html — read 2026-07-30.

**LABEL: documented** — that the box exists, what it says, what its two buttons are, that automatic execution is off by default, and that `-y`/`--enable-autoexec` overrides the preference including when there is no interface.

**What is NOT documented, and it is the crux:** nowhere do the sources say whether that box appears, is skipped, is answered for you, or blocks, when Blender is run with no interface. The manual's advice to override the preference from the command line is advice, not a statement about what happens if you don't. I will not guess.

---

### (b) What waits for a person — unsaved changes

> "By default, when exiting Blender or loading a new blend-file, if you have unsaved changes, a pop-up will ask you to either confirm discarding those changes, or save them. This behavior can be disabled with the Save Prompt option in the Save & Load section of the Preferences."

Source: the manual, https://docs.blender.org/manual/en/4.5/files/blend/open_save.html — read 2026-07-30.

The preference itself:

> "Save – Save Prompt"
> "Asks for confirmation before closing or opening a new blend-file if the current file has unsaved changes."

Source: the manual, https://docs.blender.org/manual/en/4.5/editors/preferences/save_load.html — read 2026-07-30.

**LABEL: documented.** This is a wide-reading finding: it fires on exit or on loading another file, not during the picture-making. It is relevant because a run that renders several files in turn, or that quits when finished, passes through both of those moments.

---

### (c) What waits for a person — the file browser

> "The most common way to use this editor is through modal operators (like opening or saving a blend-file). The File Browser will appear in a new window, wait for you to select a file, and then close again."

Source: the manual, https://docs.blender.org/manual/en/4.5/editors/file_browser.html — read 2026-07-30.

The API says the same from the other side:

> "Opens a file selector with an operator."
> "This method is used from the operators invoke callback which must then return {'RUNNING_MODAL'}."
> "Accepting the file selector will run the operators execute callback."

Source: the Python API documentation, `WindowManager.fileselect_add`, https://docs.blender.org/api/4.5/bpy.types.WindowManager.html — read 2026-07-30.

**LABEL: documented.** The words "wait for you to select a file" are as plain a statement of waiting for a person as the sources contain. Nothing in the render pages sends a render through the file browser; this is a wide-reading finding about how Blender behaves generally.

---

### (d) A still picture is not written unless you ask — and in the interface, a person is what asks

> "While rendering stills will allow you to view and save the image from the render buffer when it is complete, animations are a series of images, or frames, and are automatically saved directly out to a drive after being rendered."

Source: the manual, https://docs.blender.org/manual/en/4.5/render/output/animation.html — read 2026-07-30.

Driven from outside, the asking is done by a switch or an argument instead:

> "-f, --render-frame <frame>"
> "Render frame <frame> and save it."

Source: the manual, https://docs.blender.org/manual/en/4.5/advanced/command_line/arguments.html — read 2026-07-30.

> "write_still (boolean, (optional)) – Write Image, Save the rendered image to the output path (used only when animation is disabled)"

Source: the Python API documentation, `bpy.ops.render.render`, https://docs.blender.org/api/4.5/bpy.ops.render.html — read 2026-07-30.

**LABEL: documented** for each quoted sentence. **inferred** for the joining-up: that a single-frame render left in the buffer is a render outcome which depends on a person unless something else asks for it to be written. The step: the first quote says stills "allow you to view and save"; the third says the writing happens only when `write_still` is set; therefore with neither a person nor that flag, nothing lands on disk. A reader has to supply that last move, so this is inferred, not documented.

---

### (e) What does NOT wait — writing over existing frames, and errors

Writing over files that are already there is a setting, not a question:

> "Overwrite"
> "Overwrite existing files when rendering."
> "Placeholders"
> "Create empty placeholder frames while rendering."

Source: the manual, https://docs.blender.org/manual/en/4.5/render/output/properties/output.html — read 2026-07-30.

> "use_overwrite"
> "Overwrite existing files while rendering"
> "Type:"
> "boolean, default True"

Source: the Python API documentation, `RenderSettings`, https://docs.blender.org/api/4.5/bpy.types.RenderSettings.html — read 2026-07-30.

Blender does have a warn-before-overwriting mechanism, but it belongs to the file-saving operators, not to the render operator:

> "check_existing (boolean, (optional)) – Check Existing, Check and warn on overwriting existing files"

Source: the Python API documentation, `bpy.ops.wm`, https://docs.blender.org/api/4.5/bpy.ops.wm.html — read 2026-07-30. I checked the whole render operator page and no operator on it has this argument.

**LABEL for "a render does not ask before overwriting frames": inferred.** The step: the render settings offer a plain on/off switch whose default is on and whose description is "Overwrite existing files while rendering", and the warn-on-overwrite argument that other operators carry is absent from every render operator. Absence of an argument is not a statement, so this is inferred.

Errors and warnings during work are reported, not asked about:

> "Report Message"
> "Informational messages or warnings, such as after saving a file."
> "They disappear after a short time."
> "Click them to show the full message in the Info Editor."

Source: the manual, https://docs.blender.org/manual/en/4.5/interface/window_system/status_bar.html — read 2026-07-30.

Missing external files behave the same way — a message, not a box:

> "This option is useful to check if there are links to unpacked files that no longer exist. After selecting this option, a warning message will appear in the Info editor's header."

Source: the manual, https://docs.blender.org/manual/en/4.5/interface/window_system/topbar.html — read 2026-07-30. (Note the contrast on the same page: "Find Missing Files … A File Browser will show up." That one does wait; but a person has to choose it from a menu.)

---

### (f) What does NOT wait — the render itself, in the interface

> "Keep User Interface:"
> "The user interface does not change and the render is computed in the background."

Source: the manual, https://docs.blender.org/manual/en/4.5/editors/preferences/interface.html — read 2026-07-30.

> "Running Task"
> "Shows the progress of the currently running task (such as rendering or baking). Hovering the mouse pointer over the progress bar will display a time estimate. The task can be aborted by clicking the cancel button ()."

Source: the manual, https://docs.blender.org/manual/en/4.5/interface/window_system/status_bar.html — read 2026-07-30.

> "Unlike rendering with Cycles, the UI might become irresponsive when using the GPU compositor for some slower GPUs."

Source: the manual, https://docs.blender.org/manual/en/4.5/compositing/limitations.html — read 2026-07-30.

**LABEL: documented.** A render is a task that runs alongside the interface, that a person can cancel but is not required to attend, and — said outright — a Cycles render does not make Blender unresponsive.

---

### (g) What does NOT wait — Cycles' own operators

Cycles ships as an add-on, so I read its files.

**LABEL: observed.**

- What I did: read `operators.py` in full and searched every Python file in the folder for the words `invoke`, `popup`, `dialog`, `confirm`, `prompt` and `modal`.
- What I did it to: `Blender Sources/cycles_4.5/operators.py`, and all `.py` files at the top of `Blender Sources/cycles_4.5`.
- What came back: `operators.py` defines exactly three operator classes — `CYCLES_OT_use_shading_nodes`, `CYCLES_OT_denoise_animation` and `CYCLES_OT_merge_images`. Each defines only `poll` and/or `execute`; none defines `invoke`. The only way any of them speaks to a person is through `self.report`, e.g. in `CYCLES_OT_denoise_animation.execute`: `self.report({'ERROR'}, err_msg)` and `self.report({'INFO'}, "Denoising completed")`. The search for `invoke`/`popup`/`dialog`/`confirm`/`prompt`/`modal` across the folder's Python files returned no operator-related hit at all; the only matches were the unrelated `scale_popup=3` in `ui.py` line 2435 and various uses of the word "background" meaning the sky, not the mode.
- Machine and date: macOS (Darwin 25.5.0), Apple hardware, 2026-08-06.

Cycles is also told, at start-up, whether Blender has an interface. From `Blender Sources/cycles_4.5/engine.py`, inside `def init():`, line 52:

> `    _cycles.init(path, user_path, bpy.app.background)`

**LABEL: observed** for the line. What Cycles then does with that flag is inside a compiled part I was not given, so I make no claim about it.

---

### (h) A computer with nobody at it — what the sources do say

There is a mode with no interface, and the manual names rendering as its purpose:

> "-b, --background"
> "Run in background (often used for UI-less rendering)."
> "The audio device is disabled in background-mode by default and can be re-enabled by passing in -setaudio Default afterwards."

Source: the manual, https://docs.blender.org/manual/en/4.5/advanced/command_line/arguments.html — read 2026-07-30.

> "bpy.app.background"
> "Boolean, True when blender is running without a user interface (started with -b)"

Source: the Python API documentation, https://docs.blender.org/api/4.5/bpy.app.html — read 2026-07-30.

And it does not need a screen or a logged-in desktop:

> "One advantage of using the command line is that we do not need a graphical display (no need for X server on Linux for example) and consequently we can render via a remote shell (typically SSH)."

Source: the manual, https://docs.blender.org/manual/en/4.5/advanced/command_line/render.html — read 2026-07-30.

The manual describes several unattended machines rendering together, with no mention of anyone being present:

> "Primitive Render Farm"
> "An easy way to get multiple machines to share the rendering workload is to:"
> "Set up a shared directory over a network file system."
> "Disable Overwrite, enable Placeholders in the Render Output panel."
> "Start as many machines as you wish rendering to that directory."

Source: the manual, https://docs.blender.org/manual/en/4.5/render/output/properties/output.html — read 2026-07-30.

**LABEL: documented.** A render on an unattended machine is a supported, described way to work; the frames are written straight to disk; nothing in these passages mentions a person being needed once it is going.

---

### (i) A computer with nobody at it — driving Blender by script

When Blender is driven by Python rather than by a person, the parts of an operator that talk to a person are skipped by default:

> "When calling an operator you may want to pass the execution context."
> "This determines the context that is given for the operator to run in, and whether invoke() is called or only execute()."
> "EXEC_DEFAULT is used by default, running only the execute() method, but you may want the operator to take user interaction with INVOKE_DEFAULT which will also call invoke() if existing."

Source: the Python API documentation, https://docs.blender.org/api/4.5/bpy.ops.html — read 2026-07-30.

And a script cannot sit waiting on a menu it puts up:

> "Note that they will not block the scripts execution, so the caller can't wait for user input."

Source: the Python API documentation, `WindowManager.popup_menu`, https://docs.blender.org/api/4.5/bpy.types.WindowManager.html — read 2026-07-30.

While a script runs, Blender attends to nothing else:

> "While a script executes, Blender waits for it to finish and is effectively locked until it's done; while in this state Blender won't redraw or respond to user input."

Source: the Python API documentation, https://docs.blender.org/api/4.5/info_gotchas_internal_data_and_python_objects.html — read 2026-07-30.

**LABEL: documented** for all three quotes.

One further point, and I label it carefully. The debug switch is described like this:

> "-d, --debug"
> "Turn debugging on."
> "Enables memory error detection"
> "Disables mouse grab (to interact with a debugger in some cases)"
> "Keeps Python's sys.stdin rather than setting it to None"

Source: the manual, https://docs.blender.org/manual/en/4.5/advanced/command_line/arguments.html — read 2026-07-30.

**LABEL: inferred.** The step: the switch is described as *keeping* Python's standard input "rather than setting it to None", so without the switch Blender sets it to None; a Python script that tries to read typed input therefore has nothing to read from. The sources do not say what happens when a script tries anyway, and I do not claim it. What is documented is only that ordinary runs have no standard input for Python.

---

### (j) A computer with nobody at it — what the sources do NOT say

**LABEL: not in the sources** for every item here. The record of looking is at the end.

- Whether the script-permission box appears, is suppressed, is answered for you, or blocks, when Blender runs with `-b`.
- Whether the unsaved-changes pop-up can fire when there is no interface.
- Whether any box in Blender ever times out or has a default answer that is taken after a while. I found no mention of a timeout on any dialog anywhere.
- What Blender does if it needs a person and there is none — wait forever, fail, carry on. Not stated for any case.
- Whether the Windows crash pop-up (below) appears when there is no interface.
- Whether a machine that is locked, logged out, or asleep changes anything.
- Whether a render on graphics cards needs a logged-in graphical session. The one nearby statement is about not needing a display for the command line generally, quoted above, and it does not mention graphics cards.

---

### (k) Run by hand: what a computer actually did, Blender 4.5

**LABEL: observed.** Added 2026-08-06 from the hand tests. The full record of every command, with all of the machine's output word for word, is in `Audits/Blender Research/The machine/Hand tests — added questions.md`; the entry numbers below are that list's.

**On what kind of computer.** A rented Linux machine: Ubuntu 22.04.5 LTS, one NVIDIA GeForce RTX 4080 SUPER with 16 GB on driver 580.95.05, an AMD EPYC 7B12 with twenty-four cores, 39 GB of memory, reached over SSH, and **with no display server at all** — no X server, no Wayland, `DISPLAY` empty, no X socket, no X or Wayland process. Blender 4.5.12 LTS at `/opt/blender/4.5/blender`, **pointed at a settings folder of its own, `/root/the-machine/config/4.5`, made before it was used**, never at the real one; that folder reported `use_scripts_auto_execute = False` as its starting value. The machine was **shared with other topics' tests throughout**, so no time below is a clean measurement. Date: 2026-08-06.

**The script warning of part (a) does not block a run with no window.** *(Entry 3.)* A file with a scripted-expression driver reading `__import__('math').floor(4.5)` was rendered twice with no window. Refusing, word for word:

```
	BPY_driver_exec: restricted access disallows name '__import__', enable auto-execution to support

Error in PyDriver: expression failed: __import__('math').floor(4.5)
For target: (type=Object, name="Cube", property=location, property_index=0)

Read blend: "/root/the-machine/files/4.5/driver2.blend"
Saved: '/root/the-machine/runs/4.5/e3r_a/d_0001.png'
Time: 00:10.55 (Saving: 00:00.12)
```

exit status 0. Allowing wrote a frame too, exit status 0, and the two frames are different pictures — compared as pictures, not as files: `PIXELS_DIFFER max_channel_difference=0.517647`. The run can also read its own state: `AUTOEXEC_FAIL True` with `AUTOEXEC_FAIL_MESSAGE "Driver '__import__('math').floor(4.5)'"` when refused, `False` and an empty message when allowed. A registered text data-block behaves the same way, printing `scripts disabled for "…/regtext.blend", skipping 'Text.py'` and carrying on. *(Entry 4.)*

**The unsaved-changes pop-up of part (b) does not fire with no window.** *(Entries 6 and 7.)* A file was changed and never saved, and Blender reached its own exit: it exited by itself with status 0, asked nothing, and the file on disk had the same checksum before and after. A second file loaded over unsaved changes printed `SECOND FILE LOADED` and carried straight on.

**The still-picture reading of part (d) holds.** *(Entry 11.)* `bpy.ops.render.render()` with an output path set left the output folder empty; `bpy.ops.render.render(write_still=True)` wrote `s_.png` there. So the inference in (d) is what a computer does.

**Overwriting frames, part (e).** *(Entry 12.)* A frame was rendered, then rendered again to the same path: nothing was asked, the written-time moved, and the picture was the same picture. A third run with Overwrite off printed `skipping existing frame "…"` and `Info: No frames rendered, skipped to not overwrite` and left the file alone — which is the control showing the switch was being read.

**Missing files, part (e).** *(Entry 14.)* A file whose material used an image was rendered after that image was deleted. It ended by itself with exit status 0 and wrote its frame, and **said nothing at all about the missing file**.

**[Citation check, 2026-08-07: repeated, and a control added.** Repeating this in 4.5 on that machine gave the same answer. But as written the test has no control showing the deleted image was doing anything in the render, so the silence could equally have come from a texture that never reached the picture. The control was added on the repeat: the same file rendered once with the image present and once after deleting it came back `PIXELS_DIFFER max_channel_difference=0.694118`. The control belongs in the record for all four versions and is sent to the hand tests.**]**

**[Hand tests, 2026-08-07: the control was added to the file this entry used, and it failed — see (n).** On that file, in all four versions, the two pictures are the same picture, because the textured material sits in a material slot no face uses. **The silence recorded here is silence about a texture that reached nothing.** With that put right, 4.5 prints `GPUTexture: Blender Texture Not Loaded!`, so a missing texture **is** mentioned when it is in the picture. The record above is left as written; the half that holds, and now holds on a test that could have failed, is that nothing waited.**]**

**Cycles' own files, part (g), and the running thing.** *(Entry 18.)* Cycles could not be reached on this machine on this day: asked for through OptiX and CUDA it printed `CUDA cuInit: Unknown error` and `Error: Found no Cycles device of the specified type` and gave the prompt back with exit status 1. The machine's own `nvidia-smi` reported `Failed to initialize NVML: Driver/library version mismatch` throughout, so this is the machine, not Blender. The card was repaired afterwards and the entry was run again — see (l). **One thing this run settles on its own**: when the device asked for is not there, Blender says so and stops rather than quietly using the processor.

**Standard input, part (i), is not what the inference says.** *(Entry 9.)* With and without `--debug`, both runs printed:

```
STDIN IS <_io.TextIOWrapper name='<stdin>' mode='r' encoding='utf-8'>
```

The inference is left standing as written, and this observation sits beside it. What follows is the one thing found that waits: *(Entry 10)* a script calling `input()` failed at once with nothing on standard input (`EOFError: EOF when reading a line`), and with something holding standard input open it was **still sitting there sixty seconds later**, stopped only by a time limit.

**One more thing that holds a render.** *(Entry 16.)* A script added to `bpy.app.handlers.render_pre` that waits forty-five seconds held the whole run for those forty-five seconds — `HANDLER STARTED`, then `HANDLER FINISHED`, then the frame saved, then `RENDER RETURNED`, 55 seconds in all where the same scene rendered alone in nine to fifteen.

**Freestyle.** *(Entry 20.)* The same scene saved with Freestyle on and off, both rendered with `--disable-autoexec`: both ended by themselves, neither said anything about scripts, and `PIXELS_DIFFER max_channel_difference=0.658824`.

**An unwritable output folder.** *(Entry 15.)* `/root/the-machine/readonly/x_0001.png: Read-only file system` and `Error: Render error (Read-only file system) cannot save: '…'`, exit status 1, nothing written, no waiting.

**[Citation check, 2026-08-07: a repeat on the same machine disagreed on one word, and this is sent back to the hand tests.** Repeating this in 4.5 on that machine, with the folder made read-only by a read-only bind mount, gave `Error: Render error (Invalid argument) cannot save: …`. The first line matched, and so did everything the entry rests on — an error, nothing written, nobody waited for, exit status 1. The record does not say how its folder was made read-only, so the two runs may not be the same test. The original is left exactly as written; neither run is being called right.**]**

**[Hand tests, 2026-08-07: run again both ways of making a folder unwritable — see (n).** The two ways gave the same words as each other, so **that is not what the two runs differed over**. What moves the bracketed word on this machine is which graphics library Blender gets, and **both wordings came back on demand**, so neither run was mistaken. The record above is left as written and the disagreement is not resolved. Everything the entry rests on held in all eight runs: an error, nothing written, nobody waited for, exit status 1.**]**

**Losing the connection.** *(Entry 17.)* An animation render started over SSH kept going after the connection was killed: 3 frames written at the first look, 12 ninety seconds later, the process alive both times.

**Not covered by any of this.** All of it is Linux on a machine with no display server. Nothing here says what happens on macOS or Windows, and nothing here says what any of it does when Blender has a window.

---

### (l) Run by hand: Cycles on the graphics card, with no display server, Blender 4.5

**LABEL: observed.** Added 2026-08-07. This is entry 18 of `Audits/Blender Research/The machine/Hand tests — added questions.md`, run a second time after the card was repaired. It is the one thing (k) could not settle.

**On what kind of computer.** The same rented Linux machine as (k), with its graphics card now working: Ubuntu 22.04.5 LTS, one NVIDIA GeForce RTX 4080 SUPER with 16 GB, an AMD EPYC 7B12 with twenty-four cores, 39 GB of memory, reached over SSH, and **still with no display server at all** — `DISPLAY` empty, `WAYLAND_DISPLAY` empty, `XDG_SESSION_TYPE=tty`, no X socket, no X or Wayland process. The machine's **kernel driver and its driver library now agree on 580.173.02**; they had drifted apart on the day of (k), which is why that entry failed then. Blender 4.5.12 LTS at `/opt/blender/4.5/blender`, **pointed at a fresh settings folder of its own, `/root/the-machine/config-card/4.5`, made before it was used**, never at the real one; that folder reported `use_scripts_auto_execute = False` as its starting value. `nvidia-smi` reported no processes and no memory in use on the card immediately before and immediately after, so nothing else was sharing it. Date: 2026-08-07.

**What was done.** The factory cube, camera and light at 64 by 64, rendered with no window twice, once through each way the card can be reached: `--engine CYCLES … -- --cycles-device OPTIX`, then the same with `CUDA`.

**What came back. Both runs rendered on the card, wrote their picture, and gave the prompt back by themselves with exit status 0.** Nothing waited, nothing was asked. Through OptiX, word for word around the lines that matter, with the repeated per-sample lines cut:

```
Fra:1 Mem:8.24M (Peak 9.12M) | Time:00:01.53 | Mem:0.00M, Peak:0.00M | Scene, ViewLayer | Loading render kernels (may take a few minutes the first time)
Fra:1 Mem:8.33M (Peak 35.41M) | Time:00:02.96 | Mem:0.01M, Peak:0.01M | Scene, ViewLayer | Updating Scene BVH | Building OptiX acceleration structure
Fra:1 Mem:15.84M (Peak 35.41M) | Time:00:02.98 | Mem:660.10M, Peak:660.10M | Scene, ViewLayer | Sample 0/4096
Saved: '/root/the-machine/runs/4.5/e18card_OPTIX/c_0001.png'
Time: 00:03.42 (Saving: 00:00.10)


Blender quit
--- EXIT STATUS: 0
--- SECONDS: 5
```

and through CUDA:

```
Fra:1 Mem:8.33M (Peak 35.41M) | Time:00:01.54 | Mem:0.00M, Peak:0.00M | Scene, ViewLayer | Updating Scene BVH | Packing BVH nodes
Fra:1 Mem:15.84M (Peak 35.41M) | Time:00:01.55 | Mem:660.09M, Peak:660.09M | Scene, ViewLayer | Sample 0/4096
Saved: '/root/the-machine/runs/4.5/e18card_CUDA/c_0001.png'
Time: 00:01.98 (Saving: 00:00.10)


Blender quit
--- EXIT STATUS: 0
--- SECONDS: 2
```

Both wrote a picture — 4609 bytes through OptiX, 4607 through CUDA. The same happened in 5.0, 5.1 and 5.2 on the same machine and the same day.

**So: Cycles renders on a graphics card on a computer with no display server.** No run named the card itself; what names the path is `Building OptiX acceleration structure` in one and `Packing BVH nodes` in the other, and both reported about 660 megabytes of separate memory for the render. A run that finished at all got the device it asked for, because (k) shows Blender stops with an error rather than quietly using the processor when the device is not there.

**[Citation check, 2026-08-07: the label on the last sentence is corrected to inferred.** "A run that finished at all got the device it asked for" is not something either run printed; it is a step taken from a different run, so it does not belong under this section's **observed** label. This check ran the control the hand tests did not — on the repaired machine, asking 4.5 for a device this computer has not got gave `Error: Found no Cycles device of the specified type`, exit status 1 and nothing written — so the step does hold. The label was the fault, not the reasoning.**]**

**Two smaller things.** Neither Cycles run printed the `libEGL warning` or `EGL Error (0x3009)` lines that every EEVEE run on this machine prints. And loading the render kernels for the first time, in a settings folder never used before, took about a second and a half, not the "few minutes" Blender's own message warns of.

**[Citation check, 2026-08-07: the Cycles half repeated cleanly; the contrast did not, and it is sent back to the hand tests.** Repeating all eight card runs on that machine today and searching every log for error, warning and failed returned nothing, exactly as recorded, and the kernel line came at one and a half to two seconds in every run. But an EEVEE background render on the same machine today printed **no `libEGL` line and no `EGL Error` line at all**, so the claim that every EEVEE run prints them is what a repeat did not reproduce. The one known difference between the two days is that the graphics card's driver was mismatched then and now reads 580.173.02 on both halves. Neither run is being called right.**]**

**[Hand tests, 2026-08-07: both runs were reproduced, and the contrast is gone — see (n).** With the card's own graphics library, **EEVEE on this machine prints none of those lines either**, and reports it is drawing on the card; with that library hidden and only Mesa's left, all four versions print them word for word and report `llvmpipe`. **So neither run was mistaken, and what decides it is which graphics library Blender gets, not whether there is a display server.** There is now nothing to set the Cycles runs against — the Cycles half stands on its own. The disagreement is recorded, not resolved.**]**

**Not covered by this.** Nothing about macOS or Windows, nothing about more than one card, and no comparison with the same scene rendered on the processor. The times are recorded, not measured.

---

### (m) Run by hand on the Creator's Mac: a second computer, and a window, Blender 4.5

**LABEL: observed.** Added 2026-08-07 from the second sitting of the hand tests, which was run on the Creator's own Mac. The full record of every command, with all of the machine's output word for word, is in `Audits/Blender Research/The machine/Hand tests — added questions.md`; the entry numbers below are that list's. It is the second half of the same hand-test list whose Linux half is already recorded above.

**On what kind of computer.** **The Creator's own Mac: a MacBook Pro with an Apple M1 Pro chip, running macOS 26.5.2, with Apple's own graphics and no second card of any kind.** Blender 4.5.12 LTS at `/Applications/Blender 4.5.12.app/Contents/MacOS/Blender`. **It was pointed at a settings folder of its own, `/Users/artist/Documents/Smoozy hand tests/The machine/config-mac2/4.5`, made before it was used**, never at the Creator's own settings folder; a stand-in `HOME` does not move Blender's user folder on macOS, so `BLENDER_USER_RESOURCES` is what was used, and Blender was asked where its user folder actually was and answered with that path. The starting value of Auto Run Python Scripts in that folder was checked rather than assumed and reported `use_scripts_auto_execute = False`. Nothing else was running on the machine, but **no time and no memory was measured here** — this round's timings come from the Linux machine, and the seconds quoted below are only ever used to show where a wait sat, never as a measurement. Date: 2026-08-07.

**(a) Everything the Linux half found on the render path was found again here, and nothing was found that Linux did not find.** *(Entries 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 19, 20 — nineteen entries, every one of them run in this version.)* Run with no window, each of these ended by itself and asked nothing: a plain still render and a plain three-frame animation, both writing their frames; a blocked scripted-expression driver; a blocked registered text data-block; Blender given a file and no render to do; a file changed and never saved, reaching Blender's own exit; a second file loaded over unsaved changes; a save made with a render result in memory; a still render nobody asked to be written; a frame written over a frame already on disk; two Blenders sharing one output folder; a file whose texture had been deleted; an output folder that could not be written to; a script hooked into the render; a render whose shell was killed under it; Render Audio aimed at a file already there; and Freestyle with automatic script execution switched off. **The two machines agreed on every one of these**, and where a number came back it was the same number: `PIXELS_DIFFER max_channel_difference=0.517647` for the blocked driver, `PIXELS_IDENTICAL max_channel_difference=0.000000` for the frame written twice, `PIXELS_DIFFER max_channel_difference=0.658824` for Freestyle, and the same sound file to the byte and the same checksum.

**(b) The one thing that waits, waits here too.** *(Entry 10.)* A script calling `input()` with nothing on standard input failed at once, in all four versions:

```
ASKING NOW
type something: Traceback (most recent call last):
  File "<string>", line 1, in <module>
EOFError: EOF when reading a line
```

With something holding standard input open and never sending anything, **Blender was still sitting there sixty seconds later** in all four versions, and only a time limit stopped it. The Mac has no `timeout` command of its own, so the limit was kept by starting the pipeline in a process group of its own and ending the group after sixty seconds.

**(c) Standard input is a real stream on macOS too.** *(Entry 9.)* With and without `--debug`, all four versions printed the same thing, and it was not `None`:

```
STDIN IS <_io.TextIOWrapper name='<stdin>' mode='r' encoding='utf-8'>
```

So the inference this project carried is now contradicted by observation on two operating systems rather than one.

**(d) A render survives the shell it was started from being killed, and a hang-up signal as well.** *(Entry 17.)* The Linux half cut an SSH connection; the Mac has no SSH connection, so the equivalent was done twice. First the shell the render was started from was killed outright after forty-five seconds: **the Blender process was still there and frames kept appearing.** Then, because closing a terminal window on a Mac sends a hang-up signal to the whole group rather than killing a parent, the same render was started again and a hang-up signal was sent to the whole group after thirty seconds: **Blender survived that too, and frames kept appearing.** Both halves behaved the same way in all four versions.

**(e) Two places where what was printed differed between the two machines, and neither changes an answer.** The Mac printed **no `libEGL warning` or `EGL Error` lines at all** — those belong to a machine reaching for a display server that is not there, and this machine has a screen. And where the Linux record says Blender 4.5 prints no Freestyle line of its own, **4.5 on the Mac printed a whole run of them** (`Freestyle: Mesh loading`, `Freestyle: View map creation`, `Freestyle: Visibility computations …`, `Freestyle: Stroke rendering`), and 5.0, 5.1 and 5.2 on the Mac printed both `Rendering freestyle` and that same run. Nothing is read into either difference.

**(f) One wording that cannot be trusted, on either machine.** *(Entry 15.)* An output folder that cannot be written to gives an error, writes nothing, waits for nobody, and comes back with exit status 1 — the same on both machines. But the reason Blender puts in brackets is only right in 4.5, which said `Read-only file system` on Linux and `Permission denied` on the Mac. In 5.0, 5.1 and 5.2 the bracketed word has nothing to do with the failure — `Success` on Linux, `Undefined error: 0` on the Mac — and the real reason is in the separate `OpenImageIO write failed` line beside it.

**[Citation check, 2026-08-07: a repeat disagreed with the 4.5 half of this, and it is sent back to the hand tests.** Repeating the entry in 4.5 on the rented Linux machine, with the folder made read-only by a read-only bind mount, gave `Error: Render error (Invalid argument) cannot save: …`, not `Read-only file system`. The line above it — `…x_0001.png: Read-only file system` — did match, and so did everything the entry rests on: an error, nothing written, nobody waited for, exit status 1. So the bracketed word may be untrustworthy in 4.5 as well, or the two runs may not be the same test, since neither record says how its folder was made unwritable. The original is left exactly as written; neither run is being called right.**]**

**[Hand tests, 2026-08-07: run again on the Linux machine both ways — see (n).** The way the folder is made unwritable is **not** what the two runs differed over. **The bracketed word is untrustworthy in 4.5 as well**, and both wordings were reproduced on demand under a stated graphics condition. The Mac half was not re-run: it needs the Creator's Mac, which is not going anywhere. The disagreement is recorded, not resolved.**]**

**(g) One thing the Mac corrects in the Linux write-up.** *(Entry 7.)* On Linux the picture from the second-file test landed at the loaded file's own default path, and that was put down to `--render-output` having been given after `--python-expr`. On the Mac `--render-output` was given **before** `--python-expr` and the picture still landed at the file's own default path. So the reason is not the order of the arguments: **loading a second file replaces the output path with the one stored in that file.** It does not touch what the entry asks.

**(h) The window. This is ground neither machine had reached before, and it is most of what this sitting adds.** *(Entries 22, 23, 24, 25, 26 and 27, every one of them run in this version.)* Each was a Blender started with a window and driven by a script that takes Blender's own screenshots and quits at the end, so nothing was clicked at a screen position and nothing was left open on the Creator's desk.

**The automatic-execution warning is a dialog in the middle of the main window, and it reports rather than blocks.** *(Entry 22.)* The same `driver2.blend` was opened with a window. `bpy.app.autoexec_fail` reported `True`, so the block really did fire. The screenshots show a dialog in the middle of the main window — **not a warning in a header** — reading, word for word off the picture:

```
For security reasons, automatic execution of Python scripts in this file was disabled:
Driver '__import__('math').floor(4.5)'
This may lead to unexpected behavior
[ ] Permanently allow execution of scripts
        Allow Execution            Ignore
```

**Nobody answered it, and the render ran anyway.** The screenshot taken while rendering shows the same unanswered dialog with the status bar underneath reading `Render...` and a progress bar, and the output folder went from empty to holding the frame. The first-run Quick Set Up panel was on screen underneath it, also unanswered, and did not hold anything up either.

**A file chooser opened on top of a running animation render stopped nothing.** *(Entry 24.)* An animation render was started, then `bpy.ops.wm.save_as_mainfile('INVOKE_DEFAULT')` was called, which opens a file chooser in a window of its own. It was never answered. The frames written, before the chooser opened, a moment after, and thirty seconds later again: **12, 34, 107** in 4.5; 13, 35, 109 in 5.0; 12, 33, 105 in 5.1 and in 5.2. The count roughly trebled while a window sat there waiting for a person.

**A still render nobody asks to be saved still writes nothing — but the picture opens in a window of its own regardless.** *(Entry 23.)* With `write_still` off the output folder was empty before and after; with it on the file was there. **In both halves, and in all four versions, a second window appeared** whose screen is named `temp` and whose only area is an `IMAGE_EDITOR`, and it was still open when the render had finished. So a picture on screen is not proof of a file on disk.

**The splash screen and the first-run set-up do not hold up a render asked for on the command line. But a Blender started with a window never quits.** *(Entry 25.)* Pointed at a settings folder made moments before and never used, and given a blend-file and `--render-frame 1` but no `--background`, Blender wrote its frame within about a second of reading the file, with nothing dismissed by anybody. **Then it sat there.** In all four versions it was still on screen when a two-minute limit ran out and had to be stopped — `--- EXIT STATUS: 137`. So this is not a way to render on a machine with nobody at it; anything doing it has to end the process itself.

**A `--background` render does register with the Mac's window server.** *(Entry 26.)* The Mac's own list of registered programs was read every two seconds for the whole of a forty-frame background render. Blender was in it throughout, word for word:

```
"Blender" ASN:0x0-0xbd94d89:
    bundleID="org.blenderfoundation.blender"
    pid = 67004 !signalled type="Foreground" flavor=3 Version="4.5.12 2026-07-21" fileType="APPL" creator="????" Arch=ARM64
"LSDisplayName"="Blender"
"Hidden"=false
"visible"=[ NULL ]
"isFrontmost"=[ NULL ]
```

It never became the front program — the program the Mac showed as in front was never Blender, in any look, in any version — and nothing was drawn on screen. But **`--background` is not a promise that the operating system knows nothing about Blender.**

**And one thing behaves differently when a menu asks for it than when a script does.** *(Entry 27, set beside entry 19.)* `bpy.ops.sound.mixdown` was called with a window open. Given a path, even through `INVOKE_DEFAULT`, it returned `{'FINISHED'}`, opened no chooser, and wrote straight over a file already sitting there — the same as with no window at all. **Given no path, which is what choosing it from a menu does, it returned `{'RUNNING_MODAL'}`, opened a file chooser in a window of its own, and left the existing file exactly as it was** — same size, same checksum, before and after. All four versions did the same. **What decides which of the two you get is whether the path was named, not whether there is a window and not which operating system.**

**(i) One entry on this machine was not run, and the reason is not Blender.** *(Entry 21 — whether the screen locking or the display going to sleep interrupts a render.)* It needs a machine nobody else is using, put to sleep on purpose and then left untouched for several minutes. **The only machine this project has with a screen is the Creator's own computer**, and this sitting had no standing to take the Creator's own desk over that way. So it was not attempted, and it is written into the hand-test list's "Cannot be run" section with that reason. The commands are still written out there in full for whoever can run it. **Nothing here says what a locking screen or a sleeping display does to a render.** The nearest thing that was answered is (d) above, which is a different and smaller event.

**Not covered by any of this.** Every result above is from macOS on the Creator's own machine, with a screen attached and somebody's session running. Nothing here says what happens on Windows, which no Windows machine has run. **Untried rather than untestable: a Windows machine can be rented, and every one this project has read reported a Server edition — see the equipment section of `Preparation/Blender Research/Method.md`.** Nothing here says what the machine's own screen locking, display sleeping or power settings do to a render — that is entry 21, and it was not run. Nothing here is a measurement: no time and no memory was taken on this machine. And nothing here touches a graphics card of the kind the Linux machine has, because this one has none.

---

### (n) Run by hand a third time: the three tests the citation check sent back, Blender 4.5

**LABEL: observed.** Added 2026-08-07. These are entries 28, 29 and 30 of `Audits/Blender Research/The machine/Hand tests — added questions.md` — the three items the citation check sent back that needed the rented Linux machine, run in a third sitting because that machine was about to be destroyed. The full record is in that list.

**On what kind of computer.** The same rented Linux machine as (k) and (l), named in full: **Ubuntu 22.04.5 LTS, one NVIDIA GeForce RTX 4080 SUPER with 16 GB, an AMD EPYC 7B12 with twenty-four cores, 39 GB of memory, reached over SSH, and with no display server at all** — `DISPLAY` empty, `WAYLAND_DISPLAY` empty, `XDG_SESSION_TYPE=tty`, no X socket, no X or Wayland process. Its card is working and its **kernel driver and driver library agree on 580.173.02**, with no other work on the card or the machine. Blender 4.5.12 LTS at `/opt/blender/4.5/blender`, **pointed at a settings folder of its own, `/root/again-the-machine/config/4.5`, made before it was used**, which reported `use_scripts_auto_execute = False` as its starting value. All work under `/root/again-the-machine/`. Date: 2026-08-07.

**1. The `libEGL` lines are about which graphics library Blender gets, not about a missing display server.** *(Entry 28.)* `plain.blend` rendered with no window, and the **whole** log searched:

```
Blender 4.5.12 LTS (hash 84afd5f785f7 built 2026-07-21 01:31:22)
Read blend: "/root/again-the-machine/files/4.5/plain.blend"
Fra:1 Mem:39.30M (Peak 39.33M) | Time:00:01.91 | Rendering 64 / 64 samples
Saved: '/root/again-the-machine/runs/4.5/e28/e_0001.png'
Time: 00:02.02 (Saving: 00:00.10)


Blender quit
--- EXIT STATUS: 0
--- how many lines of the whole log contain libEGL:
0
--- how many lines of the whole log contain EGL:
0
```

Asked what it was drawing with, it answered `GPU_RENDERER NVIDIA GeForce RTX 4080 SUPER/PCIe/SSE2` on `4.6.0 NVIDIA 580.173.02`. **So EEVEE gets a graphics context on the card on a machine with no display server, and says nothing about it.** Then the same render with the NVIDIA graphics library hidden and only Mesa's left, nothing else changed — **and the lines came straight back**, word for word:

```
libEGL warning: egl: failed to create dri2 screen
EGL Error (0x3009): EGL_BAD_MATCH: Arguments are inconsistent (for example, a valid context requires buffers not supplied by a valid surface).
EGL Error (0x3009): EGL_BAD_MATCH: Arguments are inconsistent (for example, a valid context requires buffers not supplied by a valid surface).
EGL Error (0x3009): EGL_BAD_MATCH: Arguments are inconsistent (for example, a valid context requires buffers not supplied by a valid surface).
```

with `GPU_RENDERER llvmpipe (LLVM 15.0.7, 256 bits)`. The same happened in the other three versions. **So neither of the two disagreeing runs was mistaken about what it saw.** The mismatched driver of 2026-08-06 cannot be put back, so **nothing here shows what that machine was doing on that day, and the disagreement is recorded, not resolved** — the record in (k) and (l) is left exactly as written.

**2. An unwritable output folder: how the folder is made unwritable is not what the two runs differed over.** *(Entry 29.)* Two folders made two ways — a tmpfs remounted read-only, which is how (k)'s folder was made, and a read-only bind mount, which is how the citation check's was made — each probed first and each refusing a write with `Read-only file system`. **Both ways, this version printed the same two lines:**

```
/root/again-the-machine/readonly-tmpfs/x_0001.png: Read-only file system
Error: Render error (Invalid argument) cannot save: '/root/again-the-machine/readonly-tmpfs/x_0001.png'
Time: 00:02.15


Blender quit
--- EXIT STATUS: 1
```

With only Mesa's graphics library, both ways, it printed `Error: Render error (Read-only file system) cannot save: …` instead — **(k)'s wording, brought back on demand**. 5.0, 5.1 and 5.2 said `Success` on both ways and both graphics paths, which is what both earlier runs found. **The disagreement is recorded, not resolved.** And the part the answer rests on did not move in any of the eight runs: an error printed, nothing written, nobody waited for, exit status 1. **The bracketed word cannot be trusted to say what went wrong in any version now, 4.5 included.**

**3. A missing texture is not silent — and the entry that said so was measuring nothing.** *(Entry 30.)* The control was added to the file (k) actually used, and **it failed**: in all four versions the picture with the image and the picture without it are the same picture, `PIXELS_IDENTICAL max_channel_difference=0.000000`. The file was asked why:

```
SLOTS ['Material', 'M']
MATERIAL M
  LINK Image Texture Color -> Principled BSDF Base Color
IMAGE 'tex.png' source FILE size (0, 0) has_data False file //tex.png
```

The cube carries **two** material slots; the one holding the image was appended as slot 1 and every face is on slot 0. **So the deleted image was never in the picture.** The recipe was rebuilt with that one thing changed, and then the control fires, identically in all four versions:

```
--- as built vs red (does the texture reach the picture at all?):
PIXELS_DIFFER max_channel_difference=0.701961
--- as built vs deleted:
PIXELS_DIFFER max_channel_difference=0.666667
--- red vs deleted (THE CONTROL):
PIXELS_DIFFER max_channel_difference=0.694118
```

and with the texture in the picture **this version does say something**, word for word, at the top of the run:

```
GPUTexture: Blender Texture Not Loaded!
Blender 4.5.12 LTS (hash 84afd5f785f7 built 2026-07-21 01:31:22)
Read blend: "/root/again-the-machine/files/4.5/mt2/missingtex2.blend"
Saved: '/root/again-the-machine/runs/4.5/e30e/m_0001.png'
Time: 00:02.16 (Saving: 00:00.10)


Blender quit
--- EXIT STATUS: 0
```

Both runs with an image present were searched for the words texture, not loaded and failed and returned nothing, so the message is tied to the missing file. **(k)'s own record stands and is not touched, but its conclusion that a missing texture is "not even mentioned" is not what happens when the texture is in the picture.** What did not change, and now rests on a test that could have failed: **nothing waited** — every run ended by itself with exit status 0 and wrote its frame.

**Not covered by any of this.** All of it is Linux, on a machine with no display server. **The Mac half of two of these three entries was not run**: the citation check asked for it, and the Creator's Mac is not going anywhere, so it was left. These messages came from EEVEE; Cycles was not asked.

---

## Outside the interface

Everything that waits for a person, in every case I found, is something Blender draws in its own window: a dialog, a pop-up, a file browser. The sources describe a mode with no window (`-b`), name rendering as its main use, and say a graphical display is not needed for it. They also give two switches (`-y` / `-Y`) whose stated purpose is to settle the script-permission question from the command line, and the manual introduces them with the words "running Blender without an interface … the Preferences are still used but you may want to override them".

But the sources never close the loop. **They do not say what happens to a dialog when there is no window to put it in.** Nor do they say it is impossible. This was the single biggest gap in this answer.

**It is now closed for Linux, by test rather than by the sources — see (k).** On a computer with no display server, driven from outside its own window, Blender 4.5 did not wait for anybody on any part of the render path that was tried. The script warning became text on the terminal and the render carried straight on; the unsaved-changes pop-up never fired; a second file loaded over unsaved changes did not stop anything; an existing output file was written over silently; a missing texture was not even mentioned; an unwritable output folder gave an error and exit status 1 rather than a question; and Render Audio wrote straight over a file already there. **Two things do hold a run driven from outside**: a script reading from standard input while something keeps standard input open, which waits for ever, and a script hooked into the render, which holds it for as long as it runs. What is still open from outside: everything on macOS and Windows, and everything about what these same things do when Blender has a window.

Two things that do differ outside the interface, and are documented:

- A single-frame render must be told to write itself — `-f` on the command line, or `write_still` in Python — where inside the interface a person saves it from the buffer.
- Operators called from Python run only their `execute` part by default, so the parts that ask a person anything are skipped unless `INVOKE_DEFAULT` is passed.

**A second operating system has now been tried, and it agreed — see (m).** Every one of the nineteen entries the Linux machine ran outside Blender's own window was run again on the Creator's own Mac, in all four versions, and **the two machines agreed on every one of them**, down to the same figures where a figure came back. So what is said above about a run driven from outside is no longer true of one operating system only. **The two things that hold such a run hold on macOS too**: a script reading from standard input while something keeps standard input open, which waits for ever, and a script hooked into the render, which holds the render for as long as it runs. Standard input is a real stream there as well, not nothing. **A render on the Mac also outlives the shell it was started from, and outlives a hang-up signal to its whole process group**, which is what closing a terminal window really sends. Still open from outside: everything on Windows, which no Windows machine has run.

**Three of these results were run a third time, and one of them changes ((n)).** "A missing texture was not even mentioned" is **not** what happens when the texture is in the picture: 4.5 prints `GPUTexture: Blender Texture Not Loaded!` before anything else, and the run still ends by itself with exit status 0 and writes its frame. The entry that reported silence was rendering a file whose textured material sat in a slot no face used, so it was measuring nothing. **So a run driven from outside has one more thing to watch for on its output, beyond an exit status.** Nothing else moves: an unwritable output folder still gives an error rather than a question, writes nothing, waits for nobody and comes back with exit status 1, in every version and however the folder is made unwritable — though **the reason it names in brackets cannot be trusted in any version now**. And one thing said above about that machine is not a rule about Blender: the `libEGL` lines are not what a machine without a display server prints when it reaches for one. With the card's own graphics library, EEVEE renders there with no display server at all and prints nothing.

**And the window has now been reached, on the Mac — the thing every version of this answer has been leaving open ((m)).** With a window open, the automatic-execution warning is a **dialog in the middle of the main window**, not a warning in a header, and it **reports rather than blocks**: a render started, finished and wrote its frame with the dialog unanswered, and with the first-run Quick Set Up unanswered underneath it. A **file chooser opened on top of a running animation render stopped no frames at all**. A still render nobody asks to be saved still writes nothing — **but the picture opens in a window of its own either way**. A render asked for on the command line with a window open is written at once, **but that Blender then never quits**. A `--background` render **does register with the Mac's window server** for the whole run, though it never comes to the front and draws nothing. And Render Audio is the one place where the menu's way and a script's way part company: **with no path named it opens a chooser and writes nothing; with a path named it writes straight over an existing file without asking** — and that, not the window, is what decides.

## Operating system

Some of this is stated to differ by operating system, and some is not stated at all.

Stated:

- **Windows, and Eevee:** "Headless rendering is not supported on headless Windows systems." — the manual, https://docs.blender.org/manual/en/4.5/render/eevee/limitations/limitations.html, read 2026-07-30. This is the one hard operating-system limit I found on rendering with nobody at the machine.
- **Windows, and crashes:** "Blender on Windows now shows a crash popup with recovery options" — the release notes, https://developer.blender.org/docs/release_notes/4.5/core/, read 2026-07-30. A pop-up on a crash is a thing that could wait for a person, on Windows only. Whether it appears without an interface is not stated.
- **Windows, and the GPU compositor:** "Timeout Detection Recovery (TDR) resets the GPU driver if a GPU compositing task takes longer than the default 2 second limit." — the manual, https://docs.blender.org/manual/en/4.5/compositing/limitations.html, read 2026-07-30.
- **Linux:** "no need for X server on Linux for example" — the manual, https://docs.blender.org/manual/en/4.5/advanced/command_line/render.html, read 2026-07-30.
- **macOS:** the sources say nothing about any of the above for macOS, and nothing has been tested there either.

**Linux is now settled by test as well, and only Linux (k).** Everything in (k) was got on Ubuntu 22.04.5 LTS on a machine with no display server. It says nothing about macOS, where the same tests have not been run, and nothing about Windows, which no Windows machine has run.

One more, not an operating-system difference but a housekeeping point for an unattended machine: "Closing the Console Window will also close Blender, losing any unsaved work." — the manual, https://docs.blender.org/manual/en/4.5/advanced/command_line/index.html, read 2026-07-30.

Blender's requirements page, read 2026-08-01, https://www.blender.org/download/requirements/, names Blender 4.5 LTS three times (for the last Intel/macOS 11.2 release, for supported graphics cards, and for macOS support). None of those mentions rendering without a person, headless operation, or dialogs. It contributes nothing to this question.

**macOS has now been tried as well ((m)), and it agreed with Linux on everything the two machines both ran.** The Creator's own Mac — a MacBook Pro with an Apple M1 Pro chip on macOS 26.5.2, with Apple's own graphics and no second card — gave the same answer as the rented Linux machine on all nineteen entries that were run on both, in all four versions. **It also reached the one thing Linux could not: a window.** Two differences between the two machines showed in what was printed, and neither changes an answer: the Mac prints no `libEGL` or `EGL Error` lines, because it has a display server; and Blender 4.5 on the Mac prints a run of `Freestyle:` progress lines that the Linux record does not show.

**The first of those two has since been taken back, by test on the Linux machine ((n)).** The `libEGL` and `EGL Error` lines are **not** the mark of a machine without a display server. On that same machine, with its own card's graphics library, EEVEE renders with no display server at all and prints none of them; with that library hidden and only Mesa's software one left, all four versions print them word for word. **So it is which graphics library Blender gets that decides, not whether there is a display server**, and the Mac's silence is not evidence either way. The Freestyle difference is untouched and stands.

**Windows is still untouched** — no Windows machine has run it — so every statement above that names Windows stays untested. **And one thing on the Mac was not run**: whether the screen locking or the display going to sleep interrupts a render. That needs the Creator's own machine to itself for several minutes, and this sitting had no standing to take it; the test is written out in full in `Audits/Blender Research/The machine/Hand tests — added questions.md`, entry 21.

## Not covered

- The narrow reading of the question — whether the picture-making itself ever pauses for a person — is not settled. I found no statement either way.
- The fate of any dialog when there is no interface. Not settled, and this is the gap that matters most.
- Whether anything waits *forever*. No source mentions a timeout, a default answer, or a fallback for any dialog.
- Third-party add-ons. The sources describe Blender's and Cycles' behaviour; an add-on could put up anything it likes and nothing here constrains that. The manual does note, in a different context, that "Blender cannot prevent third-party add-ons from violating this rule" (about online access), which is a warning against assuming add-ons behave. *(Citation check, 2026-08-07: address added, which the finding did not give — `manual_4.5/editors/preferences/system.txt`.)*
- What happens if the output path cannot be written, the disk fills, or a linked file is missing at render time. I found the general reporting mechanism (status bar, Info editor) but no statement about a render specifically.
- An "Online Access" prompt exists — the API describes a preference "The user has been shown the 'Online Access' prompt and made a choice" (https://docs.blender.org/api/4.5/bpy.types.PreferencesExtensions.html and https://docs.blender.org/api/4.5/bpy.types.PreferencesFilePaths.html, read 2026-07-30) — but the manual never describes the prompt itself, when it appears, or whether it blocks. `--online-mode` and `--offline-mode` exist as command-line overrides. I could not settle whether this prompt can stand in a render's way.
- The manual mentions a "Quick Set Up Process" in the splash screen "When starting Blender for the first time or updating to a new version" (https://docs.blender.org/manual/en/4.5/interface/window_system/splash.html, read 2026-07-30) but never describes it. I opened the splash picture on that page (`_images/interface_window-system_splash_current.png`); it shows an ordinary splash screen with New File, Recent Files, Open, Recover Last Session and links, and no set-up questions. The picture is labelled 3.6.1 in its corner, so I take nothing from it about 4.5 beyond the general shape of the screen.

**Several of the gaps listed just above have since been closed by test, and only by test — see (m).** With a window open on the Creator's own Mac, the automatic-execution warning is a dialog in the middle of the main window, not a warning in a header, and it **reports rather than blocks**: a render started, finished and wrote its frame with it unanswered. A window that *is* waiting for a person — a file chooser — did not stop a running animation render either. A still render nobody asks to be saved still writes nothing, though the picture opens in a window of its own. **The sources still say none of this**; what has changed is that it has been watched happening. What is left genuinely open in this section: everything on Windows, and what the machine's own screen locking or display sleeping does to a render, which was not run because it needs the Creator's own computer to itself.

---

## Record of where I looked

**Files searched.** The whole of `Blender Sources/manual_4.5`, `Blender Sources/api_4.5`, `Blender Sources/release_notes_4.5`, `Blender Sources/cycles_4.5`, and `Blender Sources/requirements/requirements_2026-08-01.txt`. Searches were run across all of each, not file by file.

**Terms searched, plain and industry, and words around them.** "background mode", "headless", "unattended", "nobody", "without a user", "user input", "no user interface", "dialog", "pop-up", "popup", "will appear", "confirmation", "prompt", "wait for", "waits", "blocking", "block until", "user interaction", "requires user", "user confirmation", "asks you", "ask you", "you will be asked", "press any key", "click OK", "to continue", "acknowledge", "hangs", "freezes", "not responding", "stdin", "lock interface", "batch", "X server", "display server", "remote desktop", "SSH", "logged in", "screen saver", "power management", "idle", "splash", "online access", "allow online", "offline mode", "Trusted Source", "autoexec", "missing file", "Find Missing", "Report Missing", "check_existing", "invoke_confirm", "invoke_props_dialog", "invoke_popup", "modal", "quit", "exit", "pause", "resume", "interrupt", "abort", "Esc".

**Pages opened in full.** Scripting & Security; Command Line Arguments; Rendering From The Command Line; Using Blender From The Command Line; Output (render properties); Rendering Animations; Output Introduction; Drivers Troubleshooting; Preferences ‣ Save & Load; Preferences ‣ Interface; Preferences ‣ System; Opening & Saving; File Browser; Info Editor; Status Bar; Topbar; Splash Screen; Deploying Blender in Production; EEVEE Limitations; Compositing Limitations; Recovering Data; Crashes; Python Errors; Configuration Introduction. In the API: `bpy.app`, `bpy.app.handlers`, `bpy.app.timers`, `bpy.ops` (execution context), `bpy.ops.render`, `bpy.ops.wm` (the `use_scripts` and `check_existing` arguments), `bpy.types.WindowManager`, `bpy.types.RenderSettings`, `bpy.types.PreferencesFilePaths`, `bpy.types.PreferencesExtensions`, `bpy_types_enum_items/operator_type_flag_items`, `info_gotchas_operators`, `info_gotchas_internal_data_and_python_objects`, `info_tips_and_tricks`. In the release notes: `core.md`, `python_api.md`, `user_interface.md`, and a search of `bugfixes.md`. All of Cycles' Python files. The requirements page in full for the terms above.

**Pictures opened.** `_images/advanced_scripting_security_autorun-scripts-dialog.png` (from https://docs.blender.org/manual/en/4.5/advanced/scripting/security.html) and `_images/interface_window-system_splash_current.png` (from https://docs.blender.org/manual/en/4.5/interface/window_system/splash.html). Both read 2026-07-30.

**What I ruled out, and why.**

- *A render pausing mid-picture.* Searched the whole render section for "pause", "resume", "interrupt", "abort", "cancel the render", "Esc". The only hits are about a person choosing to stop and later restart at a chosen frame, and about the animation player's playback controls. Nothing describes the render stopping of its own accord and waiting.
- *A timeout or default answer on any dialog.* Searched "to continue", "press any key", "click OK", "acknowledge", "confirmation", "prompt". The only timeout in the whole documentation set is the network one ("Time Out — The time (in seconds) that online operations may wait before timing out") and the Windows GPU driver one (TDR). Neither is about a dialog.
- *A machine being locked, asleep, or logged out.* Searched "logged in", "screen saver", "power management", "idle", "session". Nothing. Blender's documentation does not discuss the state of the machine's login session at all.
- *Waiting inside Cycles.* Read every Python file Cycles ships and searched them for the interactive words listed above. Nothing. The rest of Cycles in this folder is compiled shader objects (`.oso`) and headers, which cannot answer a behaviour question, so I did not read them.
- *The requirements page.* Read in full for these terms; it covers hardware and operating-system versions only, and its three mentions of 4.5 are all about hardware support.

---

## Things a person at a computer could settle in a few minutes

**Pruned 2026-08-06 after the hand tests, again 2026-08-07 after the card tests, and again 2026-08-07 after the Mac sitting.** Everything a terminal could reach was run on the rented Linux machine; everything needing a window was then run on the Creator's own Mac. **All of it is answered in (k), (l) and (m), and those items are gone from this list.** The full list, with every command written out, is `Audits/Blender Research/The machine/Hand tests — added questions.md`.

**What is left is two things, and neither is about Blender being unwilling.**

1. **Does the screen locking, or the display going to sleep, interrupt a render? — Not run, and it needs the Creator.**
   The only machine this project has with a screen is the Creator's own computer. This test puts that screen to sleep on purpose and then needs the machine left untouched for several minutes, and the Mac sitting had no standing to take the Creator's own desk over that way. It is written out in full as entry 21 of the hand-test list. *What to look at:* whether any frame file carries a written-time from inside the stretch when the screen was asleep, whether the process is still alive at the end, and the last line of the log. **The nearest thing that was answered is that a render on that Mac survived the shell it was started from being killed, and survived a hang-up signal to its whole process group — which is a different and smaller event.**

2. **Everything on Windows. — Cannot be run.**
   No Windows machine has run it, so the EEVEE headless-Windows statement, the Windows crash pop-up, and the two Windows-only command-line arguments all stay untested rather than answered. Each is written out in the hand-test list, ready for the next Windows rental.

---

2026-08-06 — Opus 5 (Claude Code) — The machine, Blender 4.5

2026-08-07 — Opus 5 (Claude Code) — the card tests for The machine, added questions

2026-08-07 — Opus 5 (Claude Code) — the Mac hand tests for The machine, added questions

2026-08-07 — Opus 5 (Claude Code) — the tests the citation check sent back for The machine, added questions

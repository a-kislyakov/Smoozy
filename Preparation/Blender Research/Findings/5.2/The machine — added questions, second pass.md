# The machine — added questions, second pass (Blender 5.2)

**Sources consulted, and when.** Blender's manual for 5.2, Blender's Python API documentation for 5.2, the 5.2 release notes, and Cycles' own files for 5.2 — all read on 2026-07-30 (the snapshot date). Blender's requirements page, read 2026-08-01. One picture from the manual (`advanced_scripting_security_autorun-scripts-dialog.png`) was opened and looked at.

I read only the question sheet and files inside Blender Sources/. I did not open any other part of this project.

Only question 7 was answered. The rest of the sheet was left alone.

---

## 7. Can anything a render does wait for a person, and what happens on a computer with nobody at it?

**The question was ambiguous, and I have said so rather than choosing quietly.** "A render" can mean the rendering operation alone, or the whole act of getting a picture out of Blender — starting the program, loading the file, rendering, writing the result. I answered the wider reading, because the narrower one is contained in it, and I have marked which findings belong to which. If only the narrow reading was meant, read section A.1 and section B and ignore A.2 and A.3.

**LABEL: mixed — see each finding below.** The parts of the answer that describe what waits for a person in the interface are **documented**. The central question — what becomes of those waits when nobody is there, in particular whether they appear at all when Blender is run with `-b` — is **not in the sources**. My record of looking for it is in section D.

---

### A. Things that wait for a person

#### A.1 — Inside a render itself

**A rendered still is not written out on its own. It sits in a buffer until a person saves it.** This is the one place in the render itself where the interface stops and waits.

> "By default, Blender automatically saves animation frames using the settings defined in the Output panel, while still renders must be saved manually from the render buffer or by using a Compositor File Output node."

Source: the manual, https://docs.blender.org/manual/en/5.2/render/output/animation.html — read 2026-07-30.

> "By default, animation renders are saved automatically, while still renders are not. Still images must be saved manually from the Image Editor or written using a File Output Node."

Source: the manual, https://docs.blender.org/manual/en/5.2/render/output/properties/output.html — read 2026-07-30.

LABEL: **documented**.

This wait is removed from outside the interface. The command line argument that renders one frame says it saves it:

> "-f, --render-frame <frame>
>
> Render frame <frame> and save it."

Source: the manual, https://docs.blender.org/manual/en/5.2/advanced/command_line/arguments.html — read 2026-07-30.

And the render operator has a switch for the same thing:

> "write_still (bool) – Write Image, Save the rendered image to the output path (used only when animation is disabled) (optional)"

Source: the Python API documentation, https://docs.blender.org/api/5.2/bpy.ops.render.html — read 2026-07-30.

LABEL: **documented**.

**Nothing else in the render itself asks a question.** Where a render cannot write its result, the sources say it reports an error — it does not stop and ask:

> "If no valid output method is available, rendering operations will report errors."

Source: the manual, https://docs.blender.org/manual/en/5.2/render/output/animation.html — read 2026-07-30.

> "For animation renders, if the Video Sequencer is active and no alternative output is available, an error is raised. Otherwise, if compositing is enabled, active File Output Node nodes are used to save the result. If no such nodes exist, an error is reported."

Source: the manual, https://docs.blender.org/manual/en/5.2/render/output/properties/output.html — read 2026-07-30.

LABEL: **documented**.

Writing over an existing file is a setting, not a question. There is no documented "this file exists, overwrite?" prompt:

> "Image Sequence – Overwrite Image Multi-Layer EXR
>
> Overwrite existing files when rendering."

Source: the manual, https://docs.blender.org/manual/en/5.2/render/output/properties/output.html — read 2026-07-30.

LABEL: **documented**.

Inside the interface the render runs as a task with a progress bar. A person **may** stop it; they are never **required** to do anything:

> "Shows the progress of the currently running task (such as rendering or baking). Hovering the mouse pointer over the progress bar will display a time estimate. The task can be aborted by clicking the cancel button ()."

Source: the manual, https://docs.blender.org/manual/en/5.2/interface/window_system/status_bar.html — read 2026-07-30.

And with the default preference, the interface is not taken over at all:

> "Keep User Interface:
>
> The user interface does not change and the render is computed in the background."

Source: the manual, https://docs.blender.org/manual/en/5.2/editors/preferences/interface.html — read 2026-07-30.

LABEL: **documented**.

**Cycles' own tools never open a dialog.** Cycles ships three operators. All three define only an `execute` method; none defines `invoke`, none calls a window-manager popup, none runs a modal loop. The full list, from the file:

> ```
> classes = (
>     CYCLES_OT_use_shading_nodes,
>     CYCLES_OT_denoise_animation,
>     CYCLES_OT_merge_images
> )
> ```

File: `Blender Sources/cycles_5.2/operators.py`, lines 151–155 (the `classes` tuple at the end of the file). The three class definitions above it, at lines 14, 30 and 113, each contain a `def execute(self, context):` and nothing else that takes input. Read 2026-07-30. *(Citation check, 2026-08-07: line numbers corrected. The tuple is at 151–155, not 145–149; `CYCLES_OT_denoise_animation` is at line 30, not 29; `CYCLES_OT_merge_images` is at line 113, not 110. The quoted tuple itself is word for word right, and so is everything the finding says about the three classes.)*

LABEL: **observed** — I read the file and listed what is in it. What this does **not** settle: it is the step beyond the file to say "therefore Cycles never blocks", because Cycles' actual rendering is done by the compiled `_cycles` module, whose source I was not given in a form I read.

Cycles also quits rather than asks when it is given a device it cannot use on the command line:

> ```
>     if args.cycles_device:
>         import _cycles
>         if not _cycles.set_device_override(args.cycles_device):
>             sys.exit(1)
> ```

File: `Blender Sources/cycles_5.2/engine.py`, lines 37–41, inside `_parse_command_line()`. Read 2026-07-30.

LABEL: **observed**.

#### A.2 — Around a render, at file load

**If a blend-file wants to run a Python script and is not allowed to, something appears and offers a person two choices.**

> "When a blend-file tries to execute a script and is not allowed, a dialog will appear. In it, you can choose to Allow Execution or to Ignore the scripts."

Source: the manual, https://docs.blender.org/manual/en/5.2/advanced/scripting/security.html — read 2026-07-30.

The manual's own caption for the picture beside that sentence describes it differently — as a warning in a header, not a modal dialog:

> "An Auto-run warning in the Info editor's header."

Same page, same date. I opened the picture itself (`_images/advanced_scripting_security_autorun-scripts-dialog.png`, part of the same manual page). It shows a bar reading "For security reasons, automatic execution of Python scripts in this file was disabled:", naming a text block, with a checkbox "Permanently allow execution of scripts" and two buttons, "Allow Execution" and "Ignore". LABEL for what the picture shows: **documented**. What the picture does **not** show: whether it blocks anything. A bar with buttons and a modal dialog look different in a screenshot only by what surrounds them, and the surroundings are cropped away.

So the sources contradict themselves on whether this waits: the sentence says "dialog", the caption says "header". I have not resolved it and I am not going to guess.

The same page says this is the setting to reach for when running without an interface, and that the command line overrides it:

> "You may want to perform batch rendering or some other task from the command line, running Blender without an interface. In this case, the Preferences are still used but you may want to override them:
>
> Enable with -y or --enable-autoexec
>
> Disable with -Y or --disable-autoexec"

and the worked example:

> "blender --background --enable-autoexec my_movie.blend --render-anim"

Source: the manual, https://docs.blender.org/manual/en/5.2/advanced/scripting/security.html — read 2026-07-30. LABEL: **documented**.

Note that one route by which a blend-file runs scripts is rendering itself, and that route is not covered by the auto-run setting at all:

> "There are other ways scripts in a blend-file may execute that require user interaction (therefore will run even when auto execution is off), but you should be aware that this is the case since it is not necessarily obvious."
>
> "Rendering with Freestyle, because Freestyle uses scripts to control line styles."

Same page, same date. LABEL: **documented**. This is about which scripts run, not about waiting; I include it because it is the one place the manual ties script execution to a render.

#### A.3 — Around a render, elsewhere

These wait for a person, and each is a place an unattended pipeline could stop if it happens to go through that step.

**Unsaved changes when quitting or loading another file.**

> "By default, when exiting Blender or loading a new blend-file, if you have unsaved changes, a pop-up will ask you to either confirm discarding those changes, or save them.
>
> This behavior can be disabled with the Save Prompt option in the Save & Load section of the Preferences."

Source: the manual, https://docs.blender.org/manual/en/5.2/files/blend/open_save.html — read 2026-07-30. LABEL: **documented**.

**Saving a file that holds modified images — new in 5.2.**

> "When saving a file with unsaved images, a new dialog box now asks if the user wants to also save the modified images. (blender/blender@37518ed6cc)"
>
> "A new user preference was added to allow specifying image saving behavior when saving a file with modified images. (blender/blender@bc57513da3)"

Source: the release notes, https://developer.blender.org/docs/release_notes/5.2/user_interface/ — read 2026-07-30.

The preference itself:

> "Ask Every Time:
>
> Displays a confirmation dialog when saving the blend-file, allowing modified images to be saved individually."

Source: the manual, https://docs.blender.org/manual/en/5.2/editors/preferences/save_load.html — read 2026-07-30. The same panel lists "Always Save" and "Never Save" as the other two choices. LABEL: **documented**. This matters because the manual's own render-result list explicitly includes renders among the images this covers: "This affects images edited within Blender, such as painted textures, generated images, Render Results, and Compositor Viewer node results that have unsaved changes."

**Starting Blender for the first time, or after an update.**

> "When you start Blender for the first time or update to a new version, the interactive region of the Splash Screen is replaced with a few initial preferences to configure how you interact with Blender."

Source: the manual, https://docs.blender.org/manual/en/5.2/getting_started/configuration/defaults.html — read 2026-07-30. LABEL: **documented**.

**Opening Blender for the first time on macOS.**

> "Depending on the Security and Privacy preferences of your Mac, macOS will request your approval before opening Blender for the first time."

Source: the manual, https://docs.blender.org/manual/en/5.2/getting_started/installing/macos.html — read 2026-07-30. LABEL: **documented**. This is the operating system waiting, not Blender.

**After a crash on macOS.**

> "After a crash, the macOS Crash Reporter shows a window with backtrace after some time, or when Blender is opened again."

Source: the manual, https://docs.blender.org/manual/en/5.2/troubleshooting/crash.html — read 2026-07-30. LABEL: **documented**. On Windows and Linux the same page describes files being written, with no window mentioned.

#### A.4 — The general machinery for waiting

Blender's way of waiting for a person is the modal operator, and the API documentation states plainly that it does not advance without events:

> "Modal operators run every time a new event is detected, such as a mouse click or key press. Conversely, when no new events are detected, the modal operator will not run."

Source: the Python API documentation, https://docs.blender.org/api/5.2/bpy.types.Operator.html — read 2026-07-30. LABEL: **documented**.

The things that open and wait are all reached through the window manager — `fileselect_add`, `invoke_props_dialog`, `invoke_confirm`, `invoke_popup`, `invoke_props_popup`, `invoke_search_popup`, `modal_handler_add`. For the file selector:

> "This method is used from the operators invoke callback which must then return {'RUNNING_MODAL'}.
>
> Accepting the file selector will run the operators execute callback."

And for the confirmation popup:

> "Operator confirmation popup (only to let user confirm the execution, no operator properties shown)"

Source for both: the Python API documentation, https://docs.blender.org/api/5.2/bpy.types.WindowManager.html — read 2026-07-30. LABEL: **documented**.

Crucially, whether any of these is reached depends on how an operator is called, and the default from Python is the path that does **not** call `invoke`:

> "EXEC_DEFAULT is used by default, running only the execute() method, but you may want the operator to take user interaction with INVOKE_DEFAULT which will also call invoke() if existing."

Source: the Python API documentation, https://docs.blender.org/api/5.2/bpy.ops.html — read 2026-07-30. LABEL: **documented**. So a script that calls `bpy.ops.render.render()` plainly gets the non-interactive path by default.

One popup is documented as explicitly not waiting:

> "Note that they will not block the scripts execution, so the caller can't wait for user input."

(about `popup_menu`.) Source: the Python API documentation, https://docs.blender.org/api/5.2/bpy.types.WindowManager.html — read 2026-07-30. LABEL: **documented**.

There is also an operator flag whose whole purpose is to make the operator wait for a person:

> "DEPENDS_ON_CURSOR:
>
> Depends on Cursor.
>
> The initial cursor location is used, when running from a menus or buttons the user is prompted to place the cursor before beginning the operation."

Source: the Python API documentation, https://docs.blender.org/api/5.2/bpy_types_enum_items/operator_type_flag_items.html — read 2026-07-30. LABEL: **documented**. Nothing in the sources says any render operator carries this flag.

Finally, anything Python does during a render holds everything up while it runs:

> "While a script executes, Blender waits for it to finish and is effectively locked until it's done; while in this state Blender won't redraw or respond to user input."

Source: the Python API documentation, https://docs.blender.org/api/5.2/info_gotchas_internal_data_and_python_objects.html — read 2026-07-30. LABEL: **documented**. Render-time script hooks exist and are documented — `render_init`, `render_pre`, `render_post`, `render_write`, `render_stats`, `render_complete`, `render_cancel` (source: https://docs.blender.org/api/5.2/bpy.app.handlers.html, read 2026-07-30). Putting these two together — that a script hooked into a render can hold the render up indefinitely — is **inferred**. The step: the handlers are described as running at points in a render job, and script execution is described as locking Blender until finished; therefore a handler that does not finish leaves the render stopped. The sources do not say this in one place.

---

### B. What happens on a computer with nobody at it

**Blender has a mode with no interface, and it is the documented way to render this way.**

> "-b, --background
>
> Run in background (often used for UI-less rendering).
>
> The audio device is disabled in background-mode by default and can be re-enabled by passing in -setaudio Default afterwards."

Source: the manual, https://docs.blender.org/manual/en/5.2/advanced/command_line/arguments.html — read 2026-07-30.

> "Boolean, True when blender is running without a user interface (started with -b)"

(about `bpy.app.background`.) Source: the Python API documentation, https://docs.blender.org/api/5.2/bpy.app.html — read 2026-07-30. LABEL: **documented**.

**In that mode there is no graphical interface at all.** The manual states this in passing, in a code comment in its own add-on tutorial:

> "# Note that in background mode (no GUI available), keyconfigs are not available either,
> # so we have to check this to avoid nasty errors in background case."

Source: the manual, https://docs.blender.org/manual/en/5.2/advanced/scripting/addon_tutorial.html — read 2026-07-30. LABEL: **documented**.

**No screen is needed, and the machine can be somewhere else entirely.**

> "One advantage of using the command line is that we do not need a graphical display (no need for X server on Linux for example) and consequently we can render via a remote shell (typically SSH)."

Source: the manual, https://docs.blender.org/manual/en/5.2/advanced/command_line/render.html — read 2026-07-30. LABEL: **documented**.

**One named exception, and it is an operating-system one.**

> "Headless Rendering
>
> Headless rendering is not supported on headless Windows systems."

Source: the manual, https://docs.blender.org/manual/en/5.2/render/eevee/limitations/limitations.html — read 2026-07-30. LABEL: **documented**. Not covered: the page does not define "headless Windows system", and does not say what happens when you try — whether it errors, renders wrong, or crashes.

**Where output goes when nobody is watching.**

> "The Console Window, also called a Terminal, is an operating system text window that displays messages about Blender's operations, status, and internal errors.
>
> When Blender is manually started from a terminal, Blender output is shown in the Console Window it is started from."

and, on the same page:

> "Closing the Console Window will also close Blender, losing any unsaved work."

Source: the manual, https://docs.blender.org/manual/en/5.2/advanced/command_line/index.html — read 2026-07-30. LABEL: **documented**. The second line matters for an unattended machine: whatever holds the terminal open also holds the render open.

There is an argument to send that output to a file instead:

> "--log-file <filepath>
>
> Set a file to output the log to."

Source: the manual, https://docs.blender.org/manual/en/5.2/advanced/command_line/arguments.html — read 2026-07-30. LABEL: **documented**.

**A script cannot read from the keyboard by default.**

> "-d, --debug
>
> Turn debugging on.
>
> Enables memory error detection
>
> Disables mouse grab (to interact with a debugger in some cases)
>
> Keeps Python's sys.stdin rather than setting it to None"

Source: the manual, https://docs.blender.org/manual/en/5.2/advanced/command_line/arguments.html — read 2026-07-30. LABEL for the quote: **documented**. LABEL for the consequence — that without `-d` a Python script cannot sit waiting on keyboard input, because there is no input stream to wait on — **inferred**. The step: the argument is described as *keeping* `sys.stdin` "rather than setting it to None", so without it `sys.stdin` is `None`; a `None` stream cannot be read from and so cannot block. Not covered: the sentence does not say whether this applies only in background mode or always, and it does not say what a script that tries gets back.

**The manual's own advice for unattended rendering.**

> "Rendering takes all available CPU time; you should render overnight, when the computer is not needed, or set Blender to a low priority while rendering, and work on other things (be careful with the RAM space!)."

and, for recovering from a machine that dies unattended:

> "Your computer accidentally turns off in the middle of rendering your movie!
>
> Unless your animation renders in a few minutes, it is best to render the animation as separate image files. Instead of rendering directly to a compressed movie file, use a lossless format (e.g. PNG).
>
> This allows you an easy recovery if there is a problem and you have to re-start the rendering, since the frames you have already rendered will still be in the output directory.
>
> Just disable the Overwrite option to start rendering where you left off."

Source for both: the manual, https://docs.blender.org/manual/en/5.2/render/output/animation.html — read 2026-07-30. LABEL: **documented**.

And the arrangement for several unattended machines at once:

> "Primitive Render Farm
>
> An easy way to get multiple machines to share the rendering workload is to:
>
> Set up a shared directory over a network file system.
>
> Disable Overwrite, enable Placeholders in the Render Output panel.
>
> Start as many machines as you wish rendering to that directory."

Source: the manual, https://docs.blender.org/manual/en/5.2/render/output/properties/output.html — read 2026-07-30. LABEL: **documented**. Note the setting names as the manual gives them: "Image Sequence – Overwrite … Overwrite existing files when rendering" and "Image Sequence – Overwrite … Create empty placeholder frames while rendering" (both headings read "Image Sequence – Overwrite" on this page, which appears to be an error in the manual; the second describes Placeholders).

**One thing the operating system can do to an unattended render.**

> "Timeout Detection Recovery (TDR) resets the GPU driver if a GPU compositing task takes longer than the default 2 second limit. This is especially common with older GPUs when using expensive nodes like Bokeh Blur node. To prevent unexpected resets, it is recommended to increase the TDR timeout."

Source: the manual, https://docs.blender.org/manual/en/5.2/compositing/limitations.html — read 2026-07-30. LABEL: **documented**. Not covered: this page is about the GPU compositor, not about Cycles or EEVEE rendering, and the sources do not say whether the same limit applies to them.

One 5.2 change touches running the graphics parts of Blender with no interface:

> "- Added `gpu.init()` to initialize gpu backend when running Blender in background mode `--background`. (blender/blender@a6f60f8657493e34386880e30f08331dc7913b3c)"

Source: the release notes, https://developer.blender.org/docs/release_notes/5.2/python_api/ — read 2026-07-30. LABEL: **documented** as a statement of change. Not covered: the note does not say what needed it or what fails without it.

---

### B2. Run by hand: what a computer actually did, Blender 5.2

**LABEL: observed.** Added 2026-08-06 from the hand tests. The full record of every command, with all of the machine's output word for word, is in `Audits/Blender Research/The machine/Hand tests — added questions.md`; the entry numbers below are that list's.

**On what kind of computer.** A rented Linux machine: Ubuntu 22.04.5 LTS, one NVIDIA GeForce RTX 4080 SUPER with 16 GB on driver 580.95.05, an AMD EPYC 7B12 with twenty-four cores, 39 GB of memory, reached over SSH, and **with no display server at all** — no X server, no Wayland, `DISPLAY` empty, no X socket, no X or Wayland process. Blender 5.2.0 LTS at `/opt/blender/5.2/blender`, **pointed at a settings folder of its own, `/root/the-machine/config/5.2`, made before it was used**, never at the real one; that folder reported `use_scripts_auto_execute = False` as its starting value. The machine was **shared with other topics' tests throughout**, so no time below is a clean measurement. Date: 2026-08-06.

**The one wait inside a render — A.1 — does not exist outside the interface.** *(Entry 11.)* `bpy.ops.render.render()` with the output path set and printed left the output folder empty; `bpy.ops.render.render(write_still=True)` wrote `s_.png` there. Word for word:

```
OUTPUT PATH IS /root/the-machine/runs/5.2/e11a/s_
RENDER RETURNED
--- FILES:
/root/the-machine/runs/5.2/e11a:
total 0
```

**The thing A.2 could not settle — dialog or header, blocking or not — does not block with no window.** *(Entry 3.)* A file was made holding the factory cube with a scripted-expression driver on its X position reading `__import__('math').floor(4.5)`, and rendered twice with no window. Refusing, word for word:

```
	BPY_driver_exec: restricted access disallows name '__import__', enable auto-execution to support

Error in PyDriver: expression failed: __import__('math').floor(4.5)
For target: (type=Object, name="Cube", property=location, property_index=0)

00:14.878  render           | Saved: '/root/the-machine/runs/5.2/e3r_a/d_0001.png'
```

exit status 0. Allowing wrote a frame too, exit status 0, and the pictures — compared as pictures, not as files — came back `PIXELS_DIFFER max_channel_difference=0.517647`. A run can also read its own state: refused, `AUTOEXEC_FAIL True` with `AUTOEXEC_FAIL_MESSAGE "Driver '__import__('math').floor(4.5)'"` and `CUBE_X 0.0`; allowed, `AUTOEXEC_FAIL False`, empty message, `CUBE_X 4.0`. A registered text data-block behaves the same way, printing `scripts disabled for "…/regtext.blend", skipping 'Text.py'` and carrying on. *(Entry 4.)* **Whether it blocks in a window is still not settled**; that needs a screen.

**None of the waits in A.3 fired with no window.** *(Entries 6, 7, 8.)* A file changed and never saved, reaching Blender's own exit, exited by itself with status 0, asked nothing, and left the file on disk with the same checksum. A second file loaded over unsaved changes printed `SECOND FILE LOADED` and carried on. And the new-in-5.2 one: a still was rendered so a Render Result was in memory, the file was changed, and `bpy.ops.wm.save_mainfile()` called, with Save Modified Images at its starting value of Ask Every Time. It printed `Info: Saved "c.blend"` and `SAVE RETURNED`, ended with exit status 0, and the file's written-time moved. **Nothing was asked.**

**The inference at the end of A.4 is what a computer does.** *(Entry 16.)* A script waiting forty-five seconds was added to `bpy.app.handlers.render_pre`, then a still rendered. `HANDLER STARTED` was printed, then `HANDLER FINISHED` forty-five seconds later, then the frame saved, then `RENDER RETURNED`; 57 seconds in all, where the same scene rendered alone in ten to fifteen. So a handler that does not finish does leave the render stopped.

**The inference in section B about standard input is not what a computer does.** *(Entry 9.)* With and without `-d`, Blender printed the same thing, and it was not `None`:

```
STDIN IS <_io.TextIOWrapper name='<stdin>' mode='r' encoding='utf-8'>
```

The inference is left standing as the researcher wrote it, and this sits beside it. What follows is the one thing found that waits: *(Entry 10)* a script calling `input()` failed at once with `EOFError: EOF when reading a line` when nothing was on standard input, and with something holding standard input open it was **still sitting there sixty seconds later**, stopped only by a time limit.

**Errors are reported and the run ends, and the exit status carries it.** *(Entry 15.)* An output folder that cannot be written to, word for word:

```
00:10.846  image.write      | ERROR OpenImageIO write failed: Could not open file "/root/the-machine/readonly/x_0001.png"
00:10.846  reports          | ERROR Render error (Success) cannot save: '/root/the-machine/readonly/x_0001.png'

Blender quit
--- EXIT STATUS: 1
```

Nothing was written, nothing was asked. A missing texture, by contrast, produced **no message at all** *(Entry 14)*: the run ended by itself with exit status 0 and wrote its frame, and the whole output was searched for any word about the missing file.

**Writing over frames, and the manual's several-machine arrangement.** *(Entries 12 and 13.)* A frame rendered twice to the same path: nothing asked, the written-time moved, `PIXELS_IDENTICAL max_channel_difference=0.000000`. A third run with Overwrite off printed `00:00.529  reports          | No frames rendered, skipped to not overwrite` and left it alone — the control that shows the switch was read. Two Blenders started at once into one folder with Overwrite off and Placeholders on both ended by themselves with exit status 0, all forty frames present and none empty, each skipping what the other had taken. **That is two Blenders on one machine sharing one ordinary folder, not the manual's several machines sharing a network folder.**

**Freestyle.** *(Entry 20.)* The same scene saved with `scene.render.use_freestyle` and `view_layer.use_freestyle` on and off, both rendered with `--disable-autoexec`: both ended by themselves, neither said anything about scripts, the Freestyle run printed `00:11.482  render           | Rendering freestyle` as a step of its own, and `PIXELS_DIFFER max_channel_difference=0.658824`.

**Render Audio, driven from a script.** *(Entry 19.)* `bpy.ops.sound.mixdown(filepath=…)` aimed at a path where a file already sat returned `MIXDOWN RETURNED {'FINISHED'}`, ended in about a second with exit status 0, and wrote straight over it.

**A render survives the connection it was started from being cut.** *(Entry 17.)* Started over an SSH connection killed after forty-five seconds, the process was still there on a fresh connection with 3 frames written, and ninety seconds later with 10.

**EEVEE renders with no display server; Cycles could not be tried on this day.** *(Entries 1 and 18.)* Every render above used EEVEE, which is what the factory settings choose here. Cycles was asked for through OptiX and through CUDA and printed `ERROR CUDA cuInit: Unknown error` and `ERROR Found no Cycles device of the specified type`, then gave the prompt back in about a second with exit status 1. The machine's own `nvidia-smi` reported `Failed to initialize NVML: Driver/library version mismatch` throughout, **so this is a fault in the machine, not a finding about Blender**. The card was repaired afterwards and the entry was run again — see B3. **One thing this run settles on its own**: when the device asked for is not there, Blender says so and stops rather than quietly using the processor.

**Not covered by any of this.** Every result above is from Linux on a machine with no display server. Nothing here says what happens on macOS or on Windows, and nothing here says what any of these things do when Blender has a window. Nothing here touches the machine's own screen locking or sleeping, because this machine has no screen.

**[Citation check, 2026-08-07: two notes on the entries above.** *The unwritable output folder:* repeating that entry in 4.5 on the same machine, with the folder made read-only by a read-only bind mount, gave a different word in the brackets from the one 4.5's own record gives, so the bracketed reason goes back to the hand tests for every version — the behaviour it rests on (an error, nothing written, nobody waited for, exit status 1) matched. *The missing texture:* as written that test has no control showing the deleted image was doing anything in the render. The control was added on a repeat in 4.5 and came back `PIXELS_DIFFER max_channel_difference=0.694118`; the control belongs in this version's record too and is sent to the hand tests.**]**

**[Hand tests, 2026-08-07: both went back to the machine, in this version — see B5.** *The unwritable output folder:* the folder was made unwritable both ways and **the two ways gave the same words as each other**, so that is not what the two runs differed over; this version said `Success` on both ways and on both graphics paths, as both earlier runs found. *The missing texture:* the control was added to the file this entry used, **and it failed** — the two pictures are the same picture, because the textured material sits in a slot no face uses. With that put right the control fires, and **this version then prints `ERROR Failed to create GPU texture from Blender image`**, so a missing texture is mentioned when it is in the picture. Both records above are left exactly as written.**]**

---

### B3. Run by hand: Cycles on the graphics card, with no display server, Blender 5.2

**LABEL: observed.** Added 2026-08-07. This is entry 18 of `Audits/Blender Research/The machine/Hand tests — added questions.md`, run a second time after the card was repaired. It is the one thing B2 could not settle.

**On what kind of computer.** The same rented Linux machine as B2, with its graphics card now working: Ubuntu 22.04.5 LTS, one NVIDIA GeForce RTX 4080 SUPER with 16 GB, an AMD EPYC 7B12 with twenty-four cores, 39 GB of memory, reached over SSH, and **still with no display server at all** — `DISPLAY` empty, `WAYLAND_DISPLAY` empty, `XDG_SESSION_TYPE=tty`, no X socket, no X or Wayland process. The machine's **kernel driver and its driver library now agree on 580.173.02**; they had drifted apart on the day of B2, which is why that entry failed then. Blender 5.2.0 LTS at `/opt/blender/5.2/blender`, **pointed at a fresh settings folder of its own, `/root/the-machine/config-card/5.2`, made before it was used**, never at the real one; that folder reported `use_scripts_auto_execute = False` as its starting value. `nvidia-smi` reported no processes and no memory in use on the card immediately before and immediately after, so nothing else was sharing it. Date: 2026-08-07.

**What was done.** The factory cube, camera and light at 64 by 64, rendered with no window twice, once through each way the card can be reached: `--engine CYCLES … -- --cycles-device OPTIX`, then the same with `CUDA`.

**What came back. Both runs rendered on the card, wrote their picture, and gave the prompt back by themselves with exit status 0.** Nothing waited, nothing was asked. Through OptiX, word for word around the lines that matter, with the repeated per-sample lines cut:

```
00:00.504  render           | Engine: Cycles
00:02.086  render           | Fra: 1 | Mem: 1M | Loading render kernels (may take a few minutes the first time)
00:02.228  render           | Fra: 1 | Mem: 1M | Updating Scene BVH | Building OptiX acceleration structure
00:02.239  render           | Fra: 1 | Mem: 683M | Sample 0/4096
00:02.707  render           | Saved: '/root/the-machine/runs/5.2/e18card_OPTIX/c_0001.png'
00:02.707  render           | Time: 00:02.20 (Saving: 00:00.16)

Blender quit
--- EXIT STATUS: 0
--- SECONDS: 3
```

and through CUDA:

```
00:02.088  render           | Fra: 1 | Mem: 1M | Updating Scene BVH | Packing BVH nodes
00:02.102  render           | Fra: 1 | Mem: 683M | Sample 0/4096
00:02.540  render           | Saved: '/root/the-machine/runs/5.2/e18card_CUDA/c_0001.png'
00:02.540  render           | Time: 00:02.03 (Saving: 00:00.16)

Blender quit
--- EXIT STATUS: 0
--- SECONDS: 3
```

Both wrote a picture — 4663 bytes through OptiX, 4664 through CUDA. The same happened in 4.5, 5.0 and 5.1 on the same machine and the same day.

**So: Cycles renders on a graphics card on a computer with no display server.** No run named the card itself; what names the path is `Building OptiX acceleration structure` in one and `Packing BVH nodes` in the other, and both reported 683 megabytes of separate memory for the render. A run that finished at all got the device it asked for, because B2 shows Blender stops with an error rather than quietly using the processor when the device is not there.

**[Citation check, 2026-08-07: the label on that last sentence is corrected to inferred.** It is not something either run printed; it is a step taken from a different run, so it does not belong under this section's **observed** label. This check repeated both runs on the repaired machine and got the same figures, and ran the control the hand tests did not: asking for a device this computer has not got gave `Found no Cycles device of the specified type`, exit status 1 and nothing written. So the step holds; the label was the fault.**]**

**Two smaller things.** Neither Cycles run printed the `libEGL warning` or `EGL Error (0x3009)` lines that every EEVEE run on this machine prints, so those lines belong to EEVEE reaching for a display server rather than to Blender starting up here. And loading the render kernels for the first time, in a settings folder never used before, took about a second and a half, not the "few minutes" Blender's own message warns of.

**[Citation check, 2026-08-07: the Cycles half repeated cleanly; the contrast did not, and it is sent back to the hand tests.** Repeating all eight card runs on that machine today and searching every log for error, warning and failed returned nothing, exactly as recorded. But an EEVEE background render on the same machine today printed **no `libEGL` line and no `EGL Error` line at all**, so the sentence about every EEVEE run is what a repeat did not reproduce. The one known difference between the two days is that the graphics card's driver was mismatched then and now reads 580.173.02 on both halves. Neither run is being called right.**]**

**[Hand tests, 2026-08-07: both runs were reproduced, and the contrast is gone — see B5.** With the card's own graphics library, **EEVEE on this machine prints none of those lines either**, and reports it is drawing on the card; with that library hidden and only Mesa's left, all four versions print them word for word and report `llvmpipe`. **So neither run was mistaken, and what decides it is which graphics library Blender gets, not whether there is a display server.** There is now nothing to set these Cycles runs against — what the Cycles half says on its own still stands, and **the conclusion that those lines belong to EEVEE reaching for a display server should not be carried forward**. The disagreement is recorded, not resolved.**]**

**Not covered by this.** Nothing about macOS or Windows, nothing about more than one card, and no comparison with the same scene rendered on the processor. The times are recorded, not measured.

---

### B4. Run by hand on the Creator's Mac: a second computer, and a window, Blender 5.2

**LABEL: observed.** Added 2026-08-07 from the second sitting of the hand tests, which was run on the Creator's own Mac. The full record of every command, with all of the machine's output word for word, is in `Audits/Blender Research/The machine/Hand tests — added questions.md`; the entry numbers below are that list's. It is the second half of the same hand-test list whose Linux half is already recorded above.

**On what kind of computer.** **The Creator's own Mac: a MacBook Pro with an Apple M1 Pro chip, running macOS 26.5.2, with Apple's own graphics and no second card of any kind.** Blender 5.2.0 LTS at `/Applications/Blender.app/Contents/MacOS/Blender`. **It was pointed at a settings folder of its own, `/Users/artist/Documents/Smoozy hand tests/The machine/config-mac2/5.2`, made before it was used**, never at the Creator's own settings folder; a stand-in `HOME` does not move Blender's user folder on macOS, so `BLENDER_USER_RESOURCES` is what was used, and Blender was asked where its user folder actually was and answered with that path. The starting value of Auto Run Python Scripts in that folder was checked rather than assumed and reported `use_scripts_auto_execute = False`. Nothing else was running on the machine, but **no time and no memory was measured here** — this round's timings come from the Linux machine, and the seconds quoted below are only ever used to show where a wait sat, never as a measurement. Date: 2026-08-07.

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

**[Hand tests, 2026-08-07: run again on the Linux machine both ways — see B5.** The way the folder is made unwritable is **not** what the two runs differed over: both ways give the same words. **The bracketed word is untrustworthy in 4.5 as well** — it moves with which graphics library Blender gets, and both wordings were reproduced on demand. The Mac half was not re-run: it needs the Creator's Mac, which is not going anywhere. The disagreement is recorded, not resolved.**]**

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

**[Citation check, 2026-08-07: the block quoted just above is 4.5's run, not this version's.** It reads `Version="4.5.12 2026-07-21"` and 4.5's process number. What this version's own run of the entry gave is not in this document, and this check cannot supply it without inventing it. The claim that a `--background` render registers with the Mac's window server is not being called wrong — only that the evidence printed here is another version's. Sent to the hand tests.**]**

**And one thing behaves differently when a menu asks for it than when a script does.** *(Entry 27, set beside entry 19.)* `bpy.ops.sound.mixdown` was called with a window open. Given a path, even through `INVOKE_DEFAULT`, it returned `{'FINISHED'}`, opened no chooser, and wrote straight over a file already sitting there — the same as with no window at all. **Given no path, which is what choosing it from a menu does, it returned `{'RUNNING_MODAL'}`, opened a file chooser in a window of its own, and left the existing file exactly as it was** — same size, same checksum, before and after. All four versions did the same. **What decides which of the two you get is whether the path was named, not whether there is a window and not which operating system.**

**(i) One entry on this machine was not run, and the reason is not Blender.** *(Entry 21 — whether the screen locking or the display going to sleep interrupts a render.)* It needs a machine nobody else is using, put to sleep on purpose and then left untouched for several minutes. **The only machine this project has with a screen is the Creator's own computer**, and this sitting had no standing to take the Creator's own desk over that way. So it was not attempted, and it is written into the hand-test list's "Cannot be run" section with that reason. The commands are still written out there in full for whoever can run it. **Nothing here says what a locking screen or a sleeping display does to a render.** The nearest thing that was answered is (d) above, which is a different and smaller event.

**Not covered by any of this.** Every result above is from macOS on the Creator's own machine, with a screen attached and somebody's session running. Nothing here says what happens on Windows, which no Windows machine has run. **Untried rather than untestable: a Windows machine can be rented, and every one this project has read reported a Server edition — see the equipment section of `Preparation/Blender Research/Method.md`.** Nothing here says what the machine's own screen locking, display sleeping or power settings do to a render — that is entry 21, and it was not run. Nothing here is a measurement: no time and no memory was taken on this machine. And nothing here touches a graphics card of the kind the Linux machine has, because this one has none.

---

### B5. Run by hand a third time: the three tests the citation check sent back, Blender 5.2

**LABEL: observed.** Added 2026-08-07. These are entries 28, 29 and 30 of `Audits/Blender Research/The machine/Hand tests — added questions.md` — the three items the citation check sent back that needed the rented Linux machine, run in a third sitting because that machine was about to be destroyed. The full record is in that list.

**On what kind of computer.** The same rented Linux machine as B2 and B3, named in full: **Ubuntu 22.04.5 LTS, one NVIDIA GeForce RTX 4080 SUPER with 16 GB, an AMD EPYC 7B12 with twenty-four cores, 39 GB of memory, reached over SSH, and with no display server at all** — `DISPLAY` empty, `WAYLAND_DISPLAY` empty, `XDG_SESSION_TYPE=tty`, no X socket, no X or Wayland process. Its card is working and its **kernel driver and driver library agree on 580.173.02**, with no other work on the card or the machine. Blender 5.2.0 LTS at `/opt/blender/5.2/blender`, **pointed at a settings folder of its own, `/root/again-the-machine/config/5.2`, made before it was used**, which reported `use_scripts_auto_execute = False` as its starting value. All work under `/root/again-the-machine/`. Date: 2026-08-07.

**1. The `libEGL` lines are about which graphics library Blender gets, not about a missing display server.** *(Entry 28.)* `plain.blend` rendered with no window, and the **whole** log searched:

```
00:00.482  blend            | Read blend: "/root/again-the-machine/files/5.2/plain.blend"
00:00.492  render           | Start rendering: Scene, ViewLayer
00:00.492  render           | Engine: EEVEE
Blender 5.2.0 LTS (hash fbe6228777e7 built 2026-07-14 01:32:04)
00:02.797  render           | Saved: '/root/again-the-machine/runs/5.2/e28/e_0001.png'
00:02.797  render           | Time: 00:02.30 (Saving: 00:00.16)

Blender quit
--- EXIT STATUS: 0
--- how many lines of the whole log contain libEGL:
0
--- how many lines of the whole log contain EGL:
0
```

Asked what it was drawing with, it answered `GPU_RENDERER NVIDIA GeForce RTX 4080 SUPER/PCIe/SSE2` on `4.6.0 NVIDIA 580.173.02`. **So EEVEE gets a graphics context on the card on a machine with no display server, and says nothing about it.** Then the same render with the NVIDIA graphics library hidden and only Mesa's left, nothing else changed — **and the lines came straight back**, after the engine line:

```
00:00.505  render           | Engine: EEVEE
libEGL warning: egl: failed to create dri2 screen
EGL Error (0x3009): EGL_BAD_MATCH: Arguments are inconsistent (for example, a valid context requires buffers not supplied by a valid surface).
EGL Error (0x3009): EGL_BAD_MATCH: Arguments are inconsistent (for example, a valid context requires buffers not supplied by a valid surface).
EGL Error (0x3009): EGL_BAD_MATCH: Arguments are inconsistent (for example, a valid context requires buffers not supplied by a valid surface).
```

— the whole-log count of lines containing `EGL` was **4** — with `GPU_RENDERER llvmpipe (LLVM 15.0.7, 256 bits)`. The same happened in the other three versions. **So neither of the two disagreeing runs was mistaken about what it saw**, and **the conclusion in B3 that those lines belong to EEVEE reaching for a display server should not be carried forward**. The mismatched driver of 2026-08-06 cannot be put back, so **nothing here shows what that machine was doing on that day, and the disagreement is recorded, not resolved** — B2 and B3 are left exactly as written.

**2. An unwritable output folder: how the folder is made unwritable is not what the two runs differed over.** *(Entry 29.)* Two folders made two ways — a tmpfs remounted read-only, which is how B2's folder was made, and a read-only bind mount, which is how the citation check's was made — each probed first and each refusing a write with `Read-only file system`. **Both ways, this version printed the same two lines:**

```
00:02.198  image.write      | ERROR OpenImageIO write failed: Could not open file "/root/again-the-machine/readonly-tmpfs/x_0001.png"
00:02.198  reports          | ERROR Render error (Success) cannot save: '/root/again-the-machine/readonly-tmpfs/x_0001.png'
00:02.198  render           | Time: 00:01.71

Blender quit
--- EXIT STATUS: 1
```

With only Mesa's graphics library this version **did not move at all** — `Success` again, both ways. **`Success` is what both earlier runs found here; the disagreement was in 4.5 alone**, whose bracketed word does move with the graphics library, and **both of its wordings were reproduced on demand**. **The disagreement is recorded, not resolved.** In all eight runs the part the answer rests on held: an error printed, nothing written, nobody waited for, exit status 1. **The bracketed word cannot be trusted to say what went wrong in any version; the real reason is in the `OpenImageIO write failed` line above it.**

**3. A missing texture is not silent — and the entry that said so was measuring nothing.** *(Entry 30.)* The control was added to the file B2 used, and **it failed**: in all four versions the picture with the image and the picture without it are the same picture, `PIXELS_IDENTICAL max_channel_difference=0.000000`. The file was asked why:

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

and with the texture in the picture **this version does say something**, word for word:

```
00:00.488  blend            | Read blend: "/root/again-the-machine/files/5.2/mt2/missingtex2.blend"
00:00.497  render           | Start rendering: Scene, ViewLayer
00:00.497  render           | Engine: EEVEE
00:02.462  gpu.texture      | ERROR Failed to create GPU texture from Blender image
Blender 5.2.0 LTS (hash fbe6228777e7 built 2026-07-14 01:32:04)
00:02.765  render           | Saved: '/root/again-the-machine/runs/5.2/e30e/m_0001.png'
00:02.765  render           | Time: 00:02.26 (Saving: 00:00.16)

Blender quit
--- EXIT STATUS: 0
```

Both runs with an image present were searched for the words texture, not loaded and failed and returned nothing, so the message is tied to the missing file. **B2's own record stands and is not touched, but its "no message at all" is not what happens when the texture is in the picture** — and this version marks it `ERROR`, in a column of its own. What did not change, and now rests on a test that could have failed: **nothing waited** — every run ended by itself with exit status 0 and wrote its frame.

**Not covered by any of this.** All of it is Linux, on a machine with no display server. **The Mac half of two of these three entries was not run**: the citation check asked for it, and the Creator's Mac is not going anywhere, so it was left. These messages came from EEVEE; Cycles was not asked.

---

### C. The two lines required on every answer

**1. Outside the interface.** This is most of the answer, so the short version: the single wait that lives inside a render — a still sitting unsaved in the buffer — does not hold outside the interface, because `-f` is documented as saving the frame and `write_still` is documented as saving the image. The waits in section A.3 are all reached through interface actions (quitting, saving, first start) and none of them is documented as part of a render. The waits in section A.4 are all reached through `invoke`, and Python's default is `EXEC_DEFAULT`, which does not call `invoke`.

What I cannot tell you from the sources, and it is the heart of the question: **the sources never state that a dialog which would appear in the interface is suppressed, auto-answered, or turned into an error when Blender runs with `-b`.** They state that there is no GUI in background mode. They do not join the two thoughts.

**It has now been tested, on Linux, and the result is in B2.** Driven from outside its own window on a computer with no display server, Blender 5.2 did not wait for anybody on any part of the render path that was tried: the auto-run warning became text on the terminal and the render carried straight on, none of the waits in A.3 fired — **including the new-in-5.2 modified-images dialog with its preference at Ask Every Time** — an existing output file was written over silently, a missing texture was not even mentioned, an unwritable output folder gave an error and exit status 1 rather than a question, and Render Audio wrote straight over a file already there. **Two things do hold a run driven from outside**: a script that reads from standard input while something keeps standard input open, which waits for ever, and a script hooked into the render, which holds the render for as long as it runs — the inference at the end of A.4, confirmed. Standard input is a real stream in an ordinary background run, not `None`. **Cycles can use a graphics card there** — that was the one thing B2 could not settle, and B3 settles it. Still open from outside: everything on macOS and Windows, and everything about what these same things do when Blender has a window.

**Three of those results were run a third time, and one of them changes (B5).** "A missing texture was not even mentioned" is **not** what happens when the texture is in the picture: Blender 5.2 prints `ERROR Failed to create GPU texture from Blender image`, in a column of its own, and the run still ends by itself with exit status 0 and writes its frame. The entry that reported silence was rendering a file whose textured material sat in a slot no face used, so it was measuring nothing. **So a run driven from outside has one more thing to watch for on its output, beyond an exit status.** Nothing else moves: an unwritable output folder still gives an error rather than a question, writes nothing, waits for nobody and comes back with exit status 1, however the folder is made unwritable — though **the reason it names in brackets cannot be trusted in any version**, this one having said `Success` throughout. And one thing said above about that machine is not a rule about Blender: the `libEGL` lines are not what a machine without a display server prints when it reaches for one. With the card's own graphics library, EEVEE renders there with no display server at all and prints nothing.

**A second operating system has now been tried, and it agreed — see B4.** Every one of the nineteen entries the Linux machine ran outside Blender's own window was run again on the Creator's own Mac, in all four versions, and **the two machines agreed on every one of them**, down to the same figures where a figure came back. So what is said above about a run driven from outside is no longer true of one operating system only. **The two things that hold such a run hold on macOS too**: a script reading from standard input while something keeps standard input open, which waits for ever, and a script hooked into the render, which holds the render for as long as it runs. Standard input is a real stream there as well, not nothing. **A render on the Mac also outlives the shell it was started from, and outlives a hang-up signal to its whole process group**, which is what closing a terminal window really sends. Still open from outside: everything on Windows, which no Windows machine has run.

**And the window has now been reached, on the Mac — the thing every version of this answer has been leaving open (B4).** With a window open, the automatic-execution warning is a **dialog in the middle of the main window**, not a warning in a header, and it **reports rather than blocks**: a render started, finished and wrote its frame with the dialog unanswered, and with the first-run Quick Set Up unanswered underneath it. A **file chooser opened on top of a running animation render stopped no frames at all**. A still render nobody asks to be saved still writes nothing — **but the picture opens in a window of its own either way**. A render asked for on the command line with a window open is written at once, **but that Blender then never quits**. A `--background` render **does register with the Mac's window server** for the whole run, though it never comes to the front and draws nothing. And Render Audio is the one place where the menu's way and a script's way part company: **with no path named it opens a chooser and writes nothing; with a path named it writes straight over an existing file without asking** — and that, not the window, is what decides.

**2. Operating system.** Three named differences, all quoted above:
- EEVEE headless rendering "is not supported on headless Windows systems" — Windows only.
- The macOS Crash Reporter shows a window after a crash; the Windows and Linux entries on the same page describe files only.
- macOS may "request your approval before opening Blender for the first time".

Two smaller ones, from the arguments page (same address and date as above): `-con, --start-console` is marked "(Windows only)", and `--no-window-frame` "(Linux only)". Both concern windows, so neither applies in background mode.

On Linux specifically, the manual states the interface needs X11 or Wayland — "On Linux Blender supports both X11 and Wayland for official releases" (https://docs.blender.org/manual/en/5.2/getting_started/installing/linux_windowing_environment.html, read 2026-07-30) — while the command-line render page states no display is needed. Everything else in this answer is written without an operating system attached, and the sources do not distinguish.

**Linux is now settled by test as well, and only Linux (B2).** Everything in B2 was got on Ubuntu 22.04.5 LTS on a machine with neither X11 nor Wayland running, and EEVEE rendered there in every run, which is the command-line render page's claim borne out. It says nothing about macOS, where the same tests have not been run, and nothing about Windows, which no Windows machine has run.

**macOS has now been tried as well (B4), and it agreed with Linux on everything the two machines both ran.** The Creator's own Mac — a MacBook Pro with an Apple M1 Pro chip on macOS 26.5.2, with Apple's own graphics and no second card — gave the same answer as the rented Linux machine on all nineteen entries that were run on both, in all four versions. **It also reached the one thing Linux could not: a window.** Two differences between the two machines showed in what was printed, and neither changes an answer: the Mac prints no `libEGL` or `EGL Error` lines, because it has a display server; and Blender 4.5 on the Mac prints a run of `Freestyle:` progress lines that the Linux record does not show.

**The first of those two has since been taken back, by test on the Linux machine (B5).** The `libEGL` and `EGL Error` lines are **not** the mark of a machine without a display server. On that same machine, with its own card's graphics library, EEVEE renders with no display server at all and prints none of them; with that library hidden and only Mesa's software one left, all four versions print them word for word. **So it is which graphics library Blender gets that decides, not whether there is a display server**, and the Mac's silence is not evidence either way. The Freestyle difference is untouched and stands.

**Windows is still untouched** — no Windows machine has run it — so every statement above that names Windows stays untested. **And one thing on the Mac was not run**: whether the screen locking or the display going to sleep interrupts a render. That needs the Creator's own machine to itself for several minutes, and this sitting had no standing to take it; the test is written out in full in `Audits/Blender Research/The machine/Hand tests — added questions.md`, entry 21.

---

### D. Where I looked, and what I ruled out

Searches were run across the whole of `manual_5.2`, `api_5.2`, `release_notes_5.2`, `cycles_5.2` and `requirements/` at once, not file by file.

**Terms searched (plain, industry, and neighbouring words):** background, `--background`, `-b`, headless, unattended, "no interface", "without a user interface", "no user interface", "UI-less", "no GUI", interactive, "user interaction", "requires user", "user input", "input from the user", "wait for", "waits for", "waiting for", "user is not present", attended, prompt, confirm, confirmation, dialog, "a dialog will appear", popup, pop-up, ask, asked, modal, blocking, "blocks Blender", "block Blender", stdin, keypress, "key press", "press any key", "render job", "background job", "progress bar", freeze, hang, "not responding", unresponsive, timeout, watchdog, TDR, "Lock Interface", "screen saver", screensaver, "sleep mode", idle, "remote desktop", SSH, "logged in", "log out", daemon, service, "X server", "display server", Wayland, Xvfb, "virtual display", "no display", "render farm", "batch render", "batch process", overnight, "network render", "first time", "first run", "first launch", "missing file", "Report Missing", "Find Missing", splash, "Allow Online Access", cancel, Esc, INVOKE_DEFAULT, EXEC_DEFAULT, invoke_confirm, invoke_props_dialog, fileselect_add, modal_handler_add, compile.

**Pages opened in full or in the relevant part:**
- Manual: `advanced/command_line/index`, `advanced/command_line/arguments` (all of it), `advanced/command_line/render`, `advanced/scripting/security`, `advanced/scripting/addon_tutorial` (register section), `advanced/deploying_blender` (all of it — a page about running Blender in studios, which would be the natural home for unattended-machine advice, and has none), `render/introduction`, `render/output/introduction`, `render/output/animation`, `render/output/properties/output`, `render/output/audio/introduction`, `render/eevee/limitations/limitations` and its index, `render/cycles/gpu_rendering` (FAQ and limitations), `compositing/limitations`, `interface/window_system/status_bar`, `interface/window_system/topbar` (Render and Window menus), `editors/preferences/interface` (Temporary Editors), `editors/preferences/save_load`, `editors/preferences/system` (Network), `files/blend/open_save`, `getting_started/configuration/defaults`, `getting_started/installing/macos`, `getting_started/installing/linux_windowing_environment`, `troubleshooting/crash`, `troubleshooting/recover`.
- API: `bpy.app`, `bpy.app.handlers`, `bpy.ops`, `bpy.ops.render`, `bpy.types.Operator`, `bpy.types.WindowManager`, `bpy.types.RenderEngine` (background-mode note), `bpy_types_enum_items/operator_type_flag_items`, `info_gotchas_operators`, `info_gotchas_internal_data_and_python_objects`, `info_tips_and_tricks`, `info_advanced_blender_as_bpy`.
- Release notes: `index`, `rendering`, `user_interface`, `python_api`, `cycles`, `core`, and a search of `bugfixes`.
- Cycles: `operators.py` (all of it), `engine.py` (command line and init), `properties.py` (searched), `ui.py` (searched).
- One picture: `_images/advanced_scripting_security_autorun-scripts-dialog.png`.

**Ruled out, and why:**
- **The requirements page** (https://www.blender.org/download/requirements/, read 2026-08-01). It never names 5.2 anywhere, so by its own limits it cannot speak for 5.2 at all. Separately, I read all of it: it says nothing about background mode, headless machines, dialogs, or unattended running. Its only near-miss is a "Displays" section about screen resolution, which is about a person sitting at a monitor.
- **Anything about sleep, screen savers, power settings, session locking or remote desktop.** Searched for all of these across every source. Zero hits anywhere. The sources do not discuss what an operating system's own idle behaviour does to a running render.
- **Any statement that dialogs are suppressed in background mode.** I searched every occurrence of "background mode" and "background-mode" across all four sources — there are 20 of them, and I read every one. They cover: audio being disabled, dependency graphs not being built, keyconfigs being absent, `context.space_data` being None, node group operators not re-registering, the quit handler being passed False, the GPU module not importing by default, and `gpu.init()`. **None of them mentions dialogs, popups, confirmations, or waiting.** This is the silence at the centre of the question, and I hunted it.
- **`--command` / sub-commands.** `-c, --command` "implies --background mode" but the sources describe only extension package management under it, nothing about rendering or prompts.
- **Freestyle.** Checked because the security page names rendering with Freestyle as a route by which scripts run. Nothing there about waiting.
- **Missing files.** "Report Missing Files" and "Find Missing Files" exist only as things a person chooses from a menu. Nothing says a render stops and asks about them.
- **Kernel compilation.** Cycles' `kernel_optimization_level` describes waiting on the computer, not on a person: "Optimized kernels are requested at the start of a render. If optimized kernels are not available, rendering will proceed using generic kernels until the optimized set is available in the cache." (`Blender Sources/cycles_5.2/properties.py`, lines 1694–1697.) It proceeds; it does not wait. **observed.**

---

### E. Things a person with Blender open could settle in a few minutes

**Pruned 2026-08-06 after the hand tests, again 2026-08-07 after the card tests, and again 2026-08-07 after the Mac sitting.** Everything a terminal could reach was run on the rented Linux machine; everything needing a window was then run on the Creator's own Mac. **All of it is answered in B2, B3 and B4, and those items are gone from this list.** The full list, with every command written out, is `Audits/Blender Research/The machine/Hand tests — added questions.md`.

**What is left is two things, and neither is about Blender being unwilling.**

1. **Does the screen locking, or the display going to sleep, interrupt a render? — Not run, and it needs the Creator.**
   The only machine this project has with a screen is the Creator's own computer. This test puts that screen to sleep on purpose and then needs the machine left untouched for several minutes, and the Mac sitting had no standing to take the Creator's own desk over that way. It is written out in full as entry 21 of the hand-test list. *What to look at:* whether any frame file carries a written-time from inside the stretch when the screen was asleep, whether the process is still alive at the end, and the last line of the log. **The nearest thing that was answered is that a render on that Mac survived the shell it was started from being killed, and survived a hang-up signal to its whole process group — which is a different and smaller event.**

2. **Everything on Windows. — Cannot be run.**
   No Windows machine has run it, so the EEVEE headless-Windows statement, the Windows crash pop-up, and the two Windows-only command-line arguments all stay untested rather than answered. Each is written out in the hand-test list, ready for the next Windows rental.

---

2026-08-06 — Opus 5 (Claude Code) — The machine, Blender 5.2

2026-08-07 — Opus 5 (Claude Code) — the card tests for The machine, added questions

2026-08-07 — Opus 5 (Claude Code) — the Mac hand tests for The machine, added questions

2026-08-07 — Opus 5 (Claude Code) — the tests the citation check sent back for The machine, added questions

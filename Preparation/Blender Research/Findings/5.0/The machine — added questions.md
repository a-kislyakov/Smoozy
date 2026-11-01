# The machine — added questions (Blender 5.0)

Sources consulted: Blender's 5.0 manual, Blender's 5.0 Python API documentation, the Blender 5.0 release notes, and Cycles' own files as shipped with 5.0 — all read on 2026-07-30. Blender's requirements page was available (read 2026-08-01) but carried nothing on this question and is not cited below.

I read only the question sheet and files inside Blender Sources/. I did not open any other part of this project.

Scope: question 7 only. Questions 1–6 were not touched.

---

## 7. Can anything a render does wait for a person, and what happens on a computer with nobody at it?

**The question can be read two ways, and both are answered below.**

The first reading, which most of this answers: can any step in producing a render stop and refuse to go on until a human does something, and what do the sources say about rendering on a machine with nobody sitting at it. That is 7a to 7f.

The second reading: "a computer with nobody at it" could mean the *computer's own* behaviour when nobody is present — the screen locking, the machine going to sleep, the person being logged out. That is 7g, and the sources say nothing at all about it.

**LABEL: mixed.** Broken out per part below.

---

### 7a. One thing puts a question to a person: the script-execution warning

**LABEL: documented.**

Blender refuses, by default, to run Python that is stored inside a blend-file, and when a file tries to it puts up a box with two buttons. The sources describe the box and the two choices in it. They do not say whether Blender stops and does nothing else until the box is answered, so "it waits" is not written down anywhere — tests 1 to 3 below are aimed at that.

> "Automatic execution is disabled by default; however, some blend-files need this to function properly.
>
> When a blend-file tries to execute a script and is not allowed, a dialog will appear. In it, you can choose to Allow Execution or to Ignore the scripts."

Source: the manual, "Scripting & Security", https://docs.blender.org/manual/en/5.0/advanced/scripting/security.html — read 2026-07-30.

The picture that page shows next to those words is `_images/advanced_scripting_security_autorun-scripts-dialog.png` (part of the same manual page, read 2026-07-30). What is visible in it: the text "For security reasons, automatic execution of Python scripts in this file was disabled:", a named item "Text 'Text.py'", the line "This may lead to unexpected behavior", a tick-box "Permanently allow execution of scripts", and two buttons, "Allow Execution" and "Ignore".

The same page names the two kinds of stored script this catches:

> "Registered Text-Blocks
>
> A text data-block can have its Register option enabled which means it will load on start.
>
> Animation Drivers
>
> Python expressions can be used to Drive values and are often used in more advanced rigs and animations."

Same page and date.

The default is off, confirmed in the API documentation:

> "use_scripts_auto_execute
>
> Allow any .blend file to run scripts automatically (unsafe with blend files from an untrusted source)
>
> Type:
>
> boolean, default False"

Source: the API documentation, https://docs.blender.org/api/5.0/bpy.types.PreferencesFilePaths.html — read 2026-07-30.

**Where this sits relative to the render, exactly.** The manual says the box appears when "a blend-file tries to execute a script and is not allowed". It does not say the box appears *during* a render. A driver written as a Python expression is evaluated when the scene is evaluated, and a scene is evaluated to render it — but the manual does not spell that chain out, so treating this as "a render waiting" rather than "loading the file waiting" would be my step, not the source's. What the source does say plainly is that the same page's own worked example is a rendering one:

> "To render an animation in background mode, allowing drivers and other scripts to run:
>
> blender --background --enable-autoexec my_movie.blend --render-anim"

Same page and date. So the manual itself treats the setting as something you must get right before an unattended animation render, and gives the switch that gets it right.

**One render feature the manual explicitly excuses from this.** The same page lists Freestyle line rendering under a heading for scripts that run *without* the warning box:

> "Manual Execution
>
> There are other ways scripts in a blend-file may execute that require user interaction (therefore will run even when auto execution is off), but you should be aware that this is the case since it is not necessarily obvious.
>
> Running a script in the Text editor.
>
> Rendering with Freestyle, because Freestyle uses scripts to control line styles."

Same page and date. What those words carry: Freestyle's scripts run even when automatic execution is switched off. That no warning box therefore appears is one step further — the box is described as appearing when a script is *not allowed*, and these are described as running anyway — so that part is **inferred**, not documented. The page's reasoning ("require user interaction") is its own.

**How to stop it waiting.** Three ways, all documented on that page and in the command-line list:

> "You may want to perform batch rendering or some other task from the command line, running Blender without an interface. In this case, the Preferences are still used but you may want to override them:
>
> Enable with -y or --enable-autoexec
>
> Disable with -Y or --disable-autoexec"

Source: the manual, https://docs.blender.org/manual/en/5.0/advanced/scripting/security.html — read 2026-07-30.

> "-Y, --disable-autoexec
>
> Disable automatic Python script execution (Python-drivers & startup scripts), (default)."

Source: the manual, "Command Line Arguments", https://docs.blender.org/manual/en/5.0/advanced/command_line/arguments.html — read 2026-07-30.

And in the file-opening options, per file:

> "use_scripts (boolean, (optional)) – Trusted Source, Allow .blend file to execute scripts automatically, default available from system preferences"

Source: the API documentation, https://docs.blender.org/api/5.0/bpy.ops.wm.html — read 2026-07-30 (this wording appears on `bpy.ops.wm.open_mainfile`, `bpy.ops.wm.recover_last_session` and `bpy.ops.wm.revert_mainfile`).

**Not covered.** The manual never says what Blender does with this warning when there is no interface to draw it in — whether it silently ignores the scripts, refuses to load, or something else. That is a real gap and it matters, because it is the difference between an unattended render finishing with wrong values and an unattended render hanging.

---

### 7b. The render job itself is not described as waiting for anybody

**LABEL: mixed.** **Documented** for what is quoted below: that a render shows as a running task with a progress bar and a cancel button, that the interface can be left alone while a render runs, and that the setting governing writing over existing files is a plain switch. **Not in the sources** for the other half of this section: that no question is put to the person mid-render is a silence, established by the search recorded at the end of this document, not something anybody wrote down.

Inside Blender's own window, a render is a background task with a progress bar and a cancel button, not something that stops and asks:

> "Running Task
>
> Shows the progress of the currently running task (such as rendering or baking). Hovering the mouse pointer over the progress bar will display a time estimate. The task can be aborted by clicking the cancel button ()."

Source: the manual, "Status Bar", https://docs.blender.org/manual/en/5.0/interface/window_system/status_bar.html — read 2026-07-30.

And the preference that decides what the window does while rendering has an option that leaves it alone entirely:

> "Render In
>
> When rendering, the user interface can do any of:
>
> Keep User Interface:
>
> The user interface does not change and the render is computed in the background."

Source: the manual, "Interface Preferences", https://docs.blender.org/manual/en/5.0/editors/preferences/interface.html — read 2026-07-30.

Nothing on any render page describes a confirmation, a prompt, or a question put to the person mid-render. In particular, the setting that governs writing over files already on disk is a plain on/off switch with no question attached:

> "Image Sequence – Overwrite Image Multi-Layer EXR
>
> Overwrite existing files when rendering."

Source: the manual, "Output Properties", https://docs.blender.org/manual/en/5.0/render/output/properties/output.html — read 2026-07-30. (Saving and exporting operators, by contrast, do carry a warn-before-overwriting option — "check_existing (boolean, (optional)) – Check Existing, Check and warn on overwriting existing files", API documentation, https://docs.blender.org/api/5.0/bpy.ops.wm.html, read 2026-07-30 — but that belongs to those operators, not to rendering output.)

**Not covered.** Absence of a described prompt is not the same as a documented promise that no prompt exists. The manual nowhere states "rendering never asks the user anything". This is the strongest thing the sources support, not a guarantee.

---

### 7c. What *can* wait for a person, near a render but not part of it

**LABEL: documented.** These are the person-waiting things I did find, so that they can be recognised and avoided. None of them is described as happening because of a render.

The file browser, which several operations open:

> "The most common way to use this editor is through modal operators (like opening or saving a blend-file). The File Browser will appear in a new window, wait for you to select a file, and then close again."

Source: the manual, "File Browser", https://docs.blender.org/manual/en/5.0/editors/file_browser.html — read 2026-07-30.

Unsaved-changes questions:

> "By default, when exiting Blender or loading a new blend-file, if you have unsaved changes, a pop-up will ask you to either confirm discarding those changes, or save them.
>
> This behavior can be disabled with the Save Prompt option in the Save & Load section of the Preferences."

Source: the manual, "Opening & Saving Files", https://docs.blender.org/manual/en/5.0/files/blend/open_save.html — read 2026-07-30.

Saving over a file written by a newer Blender:

> "Trying to overwrite it (with a simple 'Save' operation) will also show a confirmation popup, as this could make that loss of data permanent."

Source: the manual, "Compatibility", https://docs.blender.org/manual/en/5.0/files/blend/compatibility.html — read 2026-07-30.

The first-run set-up on the opening screen:

> "When starting Blender for the first time or updating to a new version, the 'interactive region' contains a Quick Set Up Process."

Source: the manual, "Splash Screen", https://docs.blender.org/manual/en/5.0/interface/window_system/splash.html — read 2026-07-30.

And, generally, the whole category of operations that demand input:

> "Modal operators exist as a concept in between Tools and regular operators. They require some sort of interactive input."

Source: the manual, "Operators", https://docs.blender.org/manual/en/5.0/interface/operators.html — read 2026-07-30.

**Not covered.** This is what the search recorded below turned up, not a complete list — the sources nowhere set out everything that can put a question on screen. And nothing here says whether a render can reach any of them.

---

### 7d. Driven from Python, operators do not take the person-waiting path unless asked to

**LABEL: documented.**

> "EXEC_DEFAULT is used by default, running only the execute() method, but you may want the operator to take user interaction with INVOKE_DEFAULT which will also call invoke() if existing."

Source: the API documentation, "Operators (bpy.ops)", https://docs.blender.org/api/5.0/bpy.ops.html — read 2026-07-30.

The two ways a command can put a question on screen and hold there are both described as things reached through that other path:

> "invoke_confirm(operator, event, ...) — Operator confirmation popup (only to let user confirm the execution, no operator properties shown)"

> "invoke_props_dialog(operator, ...) — Operator dialog (non-autoexec popup) invoke (show operator properties and only execute it on click on OK button)"

Source: the API documentation, https://docs.blender.org/api/5.0/bpy.types.WindowManager.html — read 2026-07-30.

**Not covered.** This says what the default calling style is. It does not say that a badly written add-on cannot find some other way to block.

---

### 7e. Cycles' own files contain nothing that waits for a person

**LABEL: observed.**

What I did: read every Python file shipped in Cycles for 5.0 (`__init__.py`, `camera.py`, `engine.py`, `operators.py`, `osl.py`, `presets.py`, `properties.py`, `ui.py`, `version_update.py`) and searched all of them for the constructs that make a command wait — `def invoke`, `invoke_props`, `invoke_confirm`, `wm.invoke`, `modal`, `popup`, `prompt`.

What came back: one hit only, and it is a drawing option, not a question put to anybody —

> `sub.template_icon_view(shading, "studio_light", scale_popup=3)`

File: `cycles_5.0/ui.py`, line 2436. Read 2026-07-30.

Cycles' three commands are all defined with a plain `execute` and no `invoke`:

> `class CYCLES_OT_use_shading_nodes(Operator):`
> `class CYCLES_OT_denoise_animation(Operator):`
> `class CYCLES_OT_merge_images(Operator):`

File: `cycles_5.0/operators.py`, lines 14, 30 and 113. Read 2026-07-30. *(Citation check, 2026-08-07: corrected — `CYCLES_OT_merge_images` is at line 113, not 108.)*

What the file says is that no waiting step is defined in it. That Cycles therefore never stops a render to ask something is a step beyond the file — it would be **inferred**, and I do not take it, because the bulk of Cycles is not Python and I was not given that part in readable form. The heavy lifting sits behind calls into `_cycles` (for example `_cycles.denoise(...)` at `cycles_5.0/operators.py` line 100), which I cannot read. *(Citation check, 2026-08-07: corrected — the call is at line 100; line 96 is the comment `# Run denoiser`.)*

**Not covered.** Only Cycles' Python layer. Its compiled part, and every other render engine, are outside what I could examine.

---

### 7f. What the sources say about a computer with nobody at it

**LABEL: documented.**

Rendering unattended is treated as normal and is written about directly.

> "Rendering takes all available CPU time; you should render overnight, when the computer is not needed, or set Blender to a low priority while rendering, and work on other things (be careful with the RAM space!)."

Source: the manual, "Rendering Animations", https://docs.blender.org/manual/en/5.0/render/output/animation.html — read 2026-07-30.

Started from outside, Blender needs no screen and no person at the machine at all:

> "One advantage of using the command line is that we do not need a graphical display (no need for X server on Linux for example) and consequently we can render via a remote shell (typically SSH)."

Source: the manual, "Rendering From The Command Line", https://docs.blender.org/manual/en/5.0/advanced/command_line/render.html — read 2026-07-30.

> "-b, --background
>
> Run in background (often used for UI-less rendering)."

Source: the manual, https://docs.blender.org/manual/en/5.0/advanced/command_line/arguments.html — read 2026-07-30.

> "bpy.app.background
>
> Boolean, True when blender is running without a user interface (started with -b)"

Source: the API documentation, https://docs.blender.org/api/5.0/bpy.app.html — read 2026-07-30.

Several unattended machines working the same job is described as a supported way to work:

> "Primitive Render Farm
>
> An easy way to get multiple machines to share the rendering workload is to:
>
> Set up a shared directory over a network file system.
>
> Disable Overwrite, enable Placeholders in the Render Output panel.
>
> Start as many machines as you wish rendering to that directory."

Source: the manual, https://docs.blender.org/manual/en/5.0/render/output/properties/output.html — read 2026-07-30.

And the standing advice for a machine that may be interrupted with nobody there to notice is to write single frames, not a movie file:

> "Your computer accidentally turns off in the middle of rendering your movie!
>
> Unless your animation renders in a few minutes, it is best to render the animation as separate image files. Instead of rendering directly to a compressed movie file, use a lossless format (e.g. PNG).
>
> This allows you an easy recovery if there is a problem and you have to re-start the rendering, since the frames you have already rendered will still be in the output directory.
>
> Just disable the Overwrite option to start rendering where you left off."

Source: the manual, https://docs.blender.org/manual/en/5.0/render/output/animation.html — read 2026-07-30.

> "It's not recommended to render directly to a video format in the first instance. If a problem occurs while rendering, the file might become unplayable and you will have to re-render all frames from the beginning."

Source: the manual, https://docs.blender.org/manual/en/5.0/render/output/properties/output.html — read 2026-07-30.

Nobody at the machine also means nobody watching the output. 5.0 changed what that output looks like:

> "Most of Blender now uses the same logging system, including operators reports and background render progress. The output format has changed, and log levels are easier to control."

Source: the release notes, https://developer.blender.org/docs/release_notes/5.0/core/ — read 2026-07-30.

> "The format of logging output has changed, including background render progress. Add-ons or render farms parsing this output may need to be updated."

Source: the release notes, https://developer.blender.org/docs/release_notes/5.0/python_api/ — read 2026-07-30.

A related 5.0 corrective-release entry:

> "Show currently rendered frame in background mode. (blender/blender#150174)"

Source: the release notes, https://developer.blender.org/docs/release_notes/5.0/corrective_releases/ — read 2026-07-30.

**One documented limit on a machine with nobody at it, and it is specific.** EEVEE:

> "Headless Rendering
>
> Headless rendering is not supported on headless Windows systems."

Source: the manual, "Limitations" (EEVEE), https://docs.blender.org/manual/en/5.0/render/eevee/limitations/limitations.html — read 2026-07-30.

**And three failure modes the release notes name, each of which would strand an unattended render.** These are the release notes' own statements about things that were wrong and were fixed, quoted as such:

> "Windows graphical crash report appears in background mode [66ef7f56214] - Backported to 4.5.1"

> "Linux Freeze when force cancelling a CLI render [d86de2cde26]"

> "Python API: Filebrowser crash in background mode [7e51e78696e]"

Source: the release notes, https://developer.blender.org/docs/release_notes/5.0/bugfixes/ — read 2026-07-30.

**One reading of the first of those, labelled separately. LABEL: inferred.** The release notes record that a graphical crash report appearing in background mode was fixed. Reading that as *nothing graphical should appear in background mode* is a step a reader has to supply: the notes say a particular fault was mended, not what the rule is. It is the closest the sources come to the point, and it is still an inference.

**Not covered.** The sources say nothing about the machine itself while nobody is at it — screen locking, going to sleep, the user being logged out or logged in at all, power settings, or whether a render survives any of those. Nothing about running Blender as a service that starts without anybody signing in. Nothing about what a render does if the drive it is writing to fills up or disappears. That silence is the second reading of the question, and it is answered as its own part below.

---

### 7g. The second reading — what the computer itself does with nobody at it

**LABEL: not in the sources.**

The preamble to this question sets out the other reading: not what Blender does, but what the *computer* does when nobody is present, and whether a render survives it. The screen locking. The machine going to sleep. The person being logged out, or never signed in at all. Power settings.

Nothing was found on any of it. The search is recorded at the end of this document and covered *screensaver*, *screen saver*, *sleep*, *idle*, *log out*, *logged in*, *session*, *power management*, *as a service* and *daemon*, across the whole 5.0 manual and the whole 5.0 API documentation, plus the release notes named there.

**Not covered.** All of it. This reading has no answer from the sources at all, in either direction, and only a running computer can settle it.

---

### 7h. Run by hand: what a computer actually did, Blender 5.0

**LABEL: observed.** Added 2026-08-06 from the hand tests. The full record of every command, with all of the machine's output word for word, is in `Audits/Blender Research/The machine/Hand tests — added questions.md`; the entry numbers below are that list's.

**On what kind of computer.** A rented Linux machine: Ubuntu 22.04.5 LTS, one NVIDIA GeForce RTX 4080 SUPER with 16 GB on driver 580.95.05, an AMD EPYC 7B12 with twenty-four cores, 39 GB of memory, reached over SSH, and **with no display server at all** — no X server, no Wayland, `DISPLAY` empty, no X socket, no X or Wayland process. Blender 5.0.1 at `/opt/blender/5.0/blender`, **pointed at a settings folder of its own, `/root/the-machine/config/5.0`, made before it was used**, never at the real one; that folder reported `use_scripts_auto_execute = False` as its starting value. The machine was **shared with other topics' tests throughout**, so no time below is a clean measurement and none is used as one. Date: 2026-08-06.

**(a) The script-execution warning of 7a does not stop a background render.** *(Entry 3.)* A blend-file was made holding the factory cube with a scripted-expression driver on its X position reading `__import__('math').floor(4.5)`, refused unless automatic execution is allowed, and rendered twice with no window. Refusing, word for word:

```
00:00.266  blend            | Read blend: "/root/the-machine/files/5.0/driver2.blend"
	BPY_driver_exec: restricted access disallows name '__import__', enable auto-execution to support

Error in PyDriver: expression failed: __import__('math').floor(4.5)
For target: (type=Object, name="Cube", property=location, property_index=0)

00:00.277  render           | Rendering single frame
00:11.570  render           | Saved: '/root/the-machine/runs/5.0/e3r_a/d_0001.png'
00:11.570  render           | Time: 00:11.29 (Saving: 00:00.10)
```

exit status 0. Allowing wrote a frame too, exit status 0. The pictures were compared as pictures, not as files: `PIXELS_DIFFER max_channel_difference=0.517647`. So the warning does not appear as anything a background run has to answer; it becomes text, and the render carries on with a scene whose driver did not run.

**(b) A run can read its own state.** *(Entry 3. Bears on the three values 5.1 and 5.2 quote from `bpy.app`.)* Refused: `AUTOEXEC_FAIL True`, `AUTOEXEC_FAIL_MESSAGE "Driver '__import__('math').floor(4.5)'"`, `CUBE_X 0.0`. Allowed: `AUTOEXEC_FAIL False`, empty message, `CUBE_X 4.0`.

**(c) A refused registered text data-block behaves the same way.** *(Entry 4.)* The run printed `scripts disabled for "/root/the-machine/files/5.0/regtext.blend", skipping 'Text.py'` and went on to write its frame with exit status 0; allowing it, the text's own line `REGISTERED TEXT RAN` appeared.

**(d) None of the person-waiting things in 7c fired with no window.** *(Entries 6, 7, 8.)* A file changed and never saved, reaching Blender's own exit, exited by itself with status 0, asked nothing, and left the file on disk with the same checksum. A second file loaded over unsaved changes printed `SECOND FILE LOADED` and carried on. A save made with a render result in memory printed `Info: Saved "c.blend"`, moved the file's written-time, and asked nothing.

**(e) Writing over an existing frame, and the control that shows the switch was read.** *(Entry 12. Bears on 7b.)* A frame was rendered, then rendered again to the same path: nothing was asked, the written-time moved, and the picture was `PIXELS_IDENTICAL max_channel_difference=0.000000` with the first. A third run with Overwrite off printed `00:00.464  reports          | No frames rendered, skipped to not overwrite` and left the file's written-time alone.

**(f) A still render writes nothing unless asked.** *(Entry 11.)* `bpy.ops.render.render()` with the output path set and printed left the folder empty; `bpy.ops.render.render(write_still=True)` wrote `s_.png`.

**(g) An output folder that cannot be written to gives an error and an exit status, and does not wait.** *(Entry 15. Bears on 7f's "Not covered", which names this as a silence.)* Word for word:

```
00:08.913  image.write      | ERROR OpenImageIO write failed: Could not open file "/root/the-machine/readonly/x_0001.png"
00:08.913  reports          | ERROR Render error (Success) cannot save: '/root/the-machine/readonly/x_0001.png'
00:08.913  render           | Time: 00:08.62

Blender quit
--- EXIT STATUS: 1
```

Nothing was written. This is the one place in the whole set where a failed unattended run told the outside world about it through its exit status.

**(h) A missing texture is not mentioned at all.** *(Entry 14.)* A file whose material used an image was rendered after that image was deleted. It ended by itself with exit status 0 and wrote its frame, and nothing whatever was printed about the missing file.

**(i) A script inside Blender can hold an unattended run open for ever.** *(Entry 10.)* With nothing on standard input, `input()` failed at once with `EOFError: EOF when reading a line`. With something holding standard input open and never sending anything, the same command was **still sitting there sixty seconds later**, and only a time limit stopped it. And *(Entry 9)*, with and without `--debug`, Blender printed the same thing, which is not nothing:

```
STDIN IS <_io.TextIOWrapper name='<stdin>' mode='r' encoding='utf-8'>
```

**(j) A script hooked into the render holds the render for as long as it runs.** *(Entry 16.)* A script waiting forty-five seconds was added to `bpy.app.handlers.render_pre`: `HANDLER STARTED`, then `HANDLER FINISHED`, then the frame saved, then `RENDER RETURNED`, 57 seconds in all where the same scene rendered alone in nine to fourteen.

**(k) Two Blenders share one output folder without being asked anything.** *(Entry 13.)* Two runs of the same animation were started at once into one folder with Overwrite off and Placeholders on. Both ended by themselves with exit status 0, all forty frames were there and none was empty, and each skipped what the other had taken — for example `05:04.322  render           | Skipping existing frame "…/f_0040.png"` in one while the other wrote it. **This is not the arrangement the manual describes**, which has several machines sharing a folder over a network file system; this was two Blenders on one machine sharing one ordinary folder.

**(l) Freestyle changes the picture with automatic script execution switched off.** *(Entry 20.)* The same scene saved with `scene.render.use_freestyle` and `view_layer.use_freestyle` on and off, both rendered with `--disable-autoexec`: both ended by themselves, neither said anything about scripts, the Freestyle run printed `00:11.185  render           | Rendering freestyle` as a step of its own, and `PIXELS_DIFFER max_channel_difference=0.658824`.

**(m) A render survives the connection it was started from being cut.** *(Entry 17. This is the second half of the researcher's own hand test 8.)* An animation render was started over an SSH connection which was then killed from the other end after forty-five seconds. On a fresh connection the process was still there with 4 frames written; ninety seconds later it was still there with 10.

**(n) EEVEE renders on a computer with no display server, and Cycles could not be tried on this day.** *(Entries 1 and 18.)* Every render above used EEVEE, which is what the factory settings choose here. Cycles was asked for through OptiX and through CUDA and printed `00:00.366  cycles           | ERROR CUDA cuInit: Unknown error` and `00:00.371  reports          | ERROR Found no Cycles device of the specified type`, then gave the prompt back in about a second with exit status 1. The machine's own `nvidia-smi` reported `Failed to initialize NVML: Driver/library version mismatch` throughout, **so this is a fault in the machine, not a finding about Blender**. The card was repaired afterwards and the entry was run again; the answer is in 7i. **One thing this run settles on its own, and 7i leans on it**: when the device asked for is not there, Blender says so and stops with exit status 1 rather than quietly using the processor instead.

**(o) What an unattended run prints.** *(Entry 2. This is what the researcher's own item 7 asked for and correctly refused to call a test.)* A three-frame animation render, word for word:

```
00:00.264  blend            | Read blend: "/root/the-machine/files/5.0/plain.blend"
00:00.275  render           | Rendering animation (frames 1..3)
00:00.275  render           | Rendering frame 1
00:00.276  render           | Start rendering: Scene, ViewLayer
00:00.276  render           | Engine: EEVEE
Blender 5.0.1 (hash a3db93c5b259 built 2025-12-16 01:30:59)
00:09.195  render           | Saved: '/root/the-machine/runs/5.0/e2/a_0001.png'
00:09.195  render           | Time: 00:08.91 (Saving: 00:00.11)
00:09.195  render           | Rendering frame 2
00:09.195  render           | Start rendering: Scene, ViewLayer
00:09.195  render           | Engine: EEVEE
00:17.416  render           | Saved: '/root/the-machine/runs/5.0/e2/a_0002.png'
00:17.416  render           | Time: 00:08.22 (Saving: 00:00.00)
00:17.416  render           | Rendering frame 3
00:17.416  render           | Start rendering: Scene, ViewLayer
00:17.416  render           | Engine: EEVEE
00:25.946  render           | Saved: '/root/the-machine/runs/5.0/e2/a_0003.png'
00:25.946  render           | Time: 00:08.52 (Saving: 00:00.00)
Blender quit
```

That is a record, not a test, exactly as the researcher said.

**Not covered by any of this.** Every result above is from Linux on a machine with no display server. Nothing here says what happens on macOS or on Windows, and nothing here says what any of these things do when Blender has a window. Nothing here touches 7g — the machine's own screen locking, sleeping or logging out — because this machine has no screen to lock.

**[Citation check, 2026-08-07: two notes on the entries above.** *(g), the unwritable output folder:* repeating the same entry in 4.5 on that machine, with the folder made read-only by a read-only bind mount, gave a different word in the brackets from the one 4.5's own record gives, so the bracketed reason goes back to the hand tests for every version — the behaviour it rests on (an error, nothing written, nobody waited for, exit status 1) matched. *(h), the missing texture:* as written the test has no control showing the deleted image was doing anything in the render. The control was added on a repeat in 4.5 and came back `PIXELS_DIFFER max_channel_difference=0.694118`, so the silence is real there; the control belongs in this version's record too and is sent to the hand tests.**]**

**[Hand tests, 2026-08-07: both went back to the machine, in this version — see 7k.** *The unwritable output folder:* the folder was made unwritable both ways and **the two ways gave the same words as each other**, so that is not what the two runs differed over; this version said `Success` in the brackets on both ways and on both graphics paths, as both earlier runs found. *The missing texture:* the control was added to the file this entry used, **and it failed** — the two pictures are the same picture, because the textured material sits in a slot no face uses. With that put right the control fires, and **this version then prints `ERROR Failed to create GPU texture from Blender image`**, so a missing texture is mentioned when it is in the picture. Both records above are left exactly as written.**]**

---

### 7i. Run by hand: Cycles on the graphics card, with no display server, Blender 5.0

**LABEL: observed.** Added 2026-08-07. This is entry 18 of `Audits/Blender Research/The machine/Hand tests — added questions.md`, run a second time after the card was repaired. It is the one thing 7h could not settle. The full record, with all of the machine's output word for word, is in that list.

**On what kind of computer.** The same rented Linux machine as 7h, with its graphics card now working: Ubuntu 22.04.5 LTS, one NVIDIA GeForce RTX 4080 SUPER with 16 GB, an AMD EPYC 7B12 with twenty-four cores, 39 GB of memory, reached over SSH, and **still with no display server at all** — `DISPLAY` empty, `WAYLAND_DISPLAY` empty, `XDG_SESSION_TYPE=tty`, no X socket, no X or Wayland process. The machine's **kernel driver and its driver library now agree on 580.173.02** (they had drifted apart on the day of 7h, which is why that entry failed then), and `nvidia-smi` answers instead of failing. Blender 5.0.1 at `/opt/blender/5.0/blender`, **pointed at a fresh settings folder of its own, `/root/the-machine/config-card/5.0`, made before it was used**, never at the real one; that folder reported `use_scripts_auto_execute = False` as its starting value. `nvidia-smi` reported no processes and no memory in use on the card both immediately before and immediately after, so nothing else was sharing it. Date: 2026-08-07.

**What was done.** The factory cube, camera and light at 64 by 64, saved by this version, rendered with no window twice — once through each of the two ways this card can be reached:

```
/opt/blender/5.0/blender --background …/plain.blend --engine CYCLES --render-output …/e18card_OPTIX/c_ --render-frame 1 -- --cycles-device OPTIX
/opt/blender/5.0/blender --background …/plain.blend --engine CYCLES --render-output …/e18card_CUDA/c_  --render-frame 1 -- --cycles-device CUDA
```

**What came back. Both runs rendered on the card, wrote their picture, and gave the prompt back by themselves with exit status 0.** Nothing waited and nothing was asked. Through OptiX, word for word, with the repeated per-sample lines cut where marked and nothing else removed:

```
00:00.274  blend            | Read blend: "/root/the-machine/files/5.0/plain.blend"
00:00.283  render           | Rendering single frame
00:00.285  render           | Rendering frame 1
00:00.285  render           | Start rendering: Scene, ViewLayer
00:00.285  render           | Engine: Cycles
00:01.813  render           | Fra: 1 | Mem: 1M | Synchronizing object | Light
Blender 5.0.1 (hash a3db93c5b259 built 2025-12-16 01:30:59)
00:01.817  render           | Fra: 1 | Mem: 1M | Loading render kernels (may take a few minutes the first time)
00:03.282  render           | Fra: 1 | Mem: 1M | Updating Scene BVH | Building OptiX acceleration structure
00:03.310  render           | Fra: 1 | Mem: 688M | Sample 0/4096
   (the per-sample lines are cut here)
00:03.535  render           | Fra: 1 | Remaining: 00:02.17 | Mem: 688M | Finished
00:03.771  render           | Saved: '/root/the-machine/runs/5.0/e18card_OPTIX/c_0001.png'
00:03.771  render           | Time: 00:03.48 (Saving: 00:00.10)

Blender quit
--- EXIT STATUS: 0
--- SECONDS: 4
--- FILES:
total 8
-rw-r--r-- 1 root root 4561 Aug  7 00:31 c_0001.png
```

and through CUDA, word for word around the lines that matter:

```
00:00.287  render           | Engine: Cycles
00:01.454  render           | Fra: 1 | Mem: 0M | Loading render kernels (may take a few minutes the first time)
00:01.507  render           | Fra: 1 | Mem: 1M | Updating Scene BVH | Packing BVH nodes
00:01.523  render           | Fra: 1 | Mem: 688M | Sample 0/4096
00:01.985  render           | Saved: '/root/the-machine/runs/5.0/e18card_CUDA/c_0001.png'
00:01.985  render           | Time: 00:01.69 (Saving: 00:00.10)

Blender quit
--- EXIT STATUS: 0
--- SECONDS: 2
--- FILES:
total 8
-rw-r--r-- 1 root root 4561 Aug  7 00:31 c_0001.png
```

**(a) Cycles renders on a graphics card on a computer with no display server.** Both ways worked, on a machine with no X server, no Wayland and no screen. The same happened in 4.5, 5.1 and 5.2, all on the same machine and the same day.

**(b) What names the device, and how far it goes.** Neither run named the card itself. What the OptiX run printed is `Building OptiX acceleration structure`, and what the CUDA run printed in the same place is `Packing BVH nodes`, so the two ran by different paths. Both reported 688 megabytes of separate memory for the render beside Blender's own nought to one. **A run that finished at all is a run that got the device it asked for**, because 7h (n) shows that when the device is not there Blender says `ERROR Found no Cycles device of the specified type` and stops with exit status 1 rather than quietly using the processor.

**[Citation check, 2026-08-07: the label on that last sentence is corrected to inferred.** It is not something either run printed; it is a step taken from a different run, so it does not belong under this section's **observed** label. This check repeated both runs in 5.0 on the repaired machine — both reported `Mem: 688M` as recorded — and ran the control the hand tests did not: asking for a device this computer has not got gave `Found no Cycles device of the specified type`, exit status 1 and nothing written. So the step holds; the label was the fault.**]**

**(c) A Cycles run printed no display-server complaints at all.** Every EEVEE run on this machine prints `libEGL warning: egl: failed to create dri2 screen` and three `EGL Error (0x3009)` lines before rendering. **Neither of these two Cycles runs printed any of them.** The whole record of all eight runs of this entry was searched for the words error, warning and failed, and nothing matched.

**[Citation check, 2026-08-07: the Cycles half repeated cleanly; the contrast did not, and it is sent back to the hand tests.** Repeating all eight card runs on that machine today and searching every log for error, warning and failed returned nothing, exactly as recorded. But an EEVEE background render on the same machine today printed **no `libEGL` line and no `EGL Error` line at all**, so the first sentence is what a repeat did not reproduce. The one known difference between the two days is that the graphics card's driver was mismatched then and now reads 580.173.02 on both halves. Neither run is being called right.**]**

**[Hand tests, 2026-08-07: both runs were reproduced, and the contrast is gone — see 7k (a).** With the card's own graphics library, **EEVEE on this machine prints none of those lines either**, and reports it is drawing on the card; with that library hidden and only Mesa's left, all four versions print them word for word and report `llvmpipe`. **So neither run was mistaken, and what decides it is which graphics library Blender gets, not whether there is a display server.** There is now nothing to set these Cycles runs against — what the Cycles half says on its own still stands. **The first sentence should be read as a record of what EEVEE did on that machine on 2026-08-06, not as a rule about EEVEE.** The disagreement is recorded, not resolved.**]**

**(d) Loading the render kernels for the first time was quick.** Blender's own line warns it "may take a few minutes the first time". In a settings folder never used before, on this machine, it took about a second and a half.

**Not covered by this.** It says nothing about macOS or Windows, nothing about a machine with more than one card, and nothing about how the picture compares with the same scene rendered on the processor — that was not tried. The times above are recorded, not measured; nothing rests on them.

---

### 7j. Run by hand on the Creator's Mac: a second computer, and a window, Blender 5.0

**LABEL: observed.** Added 2026-08-07 from the second sitting of the hand tests, which was run on the Creator's own Mac. The full record of every command, with all of the machine's output word for word, is in `Audits/Blender Research/The machine/Hand tests — added questions.md`; the entry numbers below are that list's. It is the second half of the same hand-test list whose Linux half is already recorded above.

**On what kind of computer.** **The Creator's own Mac: a MacBook Pro with an Apple M1 Pro chip, running macOS 26.5.2, with Apple's own graphics and no second card of any kind.** Blender 5.0.1 at `/Applications/Blender 5.0.1.app/Contents/MacOS/Blender`. **It was pointed at a settings folder of its own, `/Users/artist/Documents/Smoozy hand tests/The machine/config-mac2/5.0`, made before it was used**, never at the Creator's own settings folder; a stand-in `HOME` does not move Blender's user folder on macOS, so `BLENDER_USER_RESOURCES` is what was used, and Blender was asked where its user folder actually was and answered with that path. The starting value of Auto Run Python Scripts in that folder was checked rather than assumed and reported `use_scripts_auto_execute = False`. Nothing else was running on the machine, but **no time and no memory was measured here** — this round's timings come from the Linux machine, and the seconds quoted below are only ever used to show where a wait sat, never as a measurement. Date: 2026-08-07.

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

**[Hand tests, 2026-08-07: run again on the Linux machine both ways — see 7k (b).** The way the folder is made unwritable is **not** what the two runs differed over: both ways give the same words. **The bracketed word is untrustworthy in 4.5 as well** — it moves with which graphics library Blender gets, and both wordings were reproduced on demand. The Mac half was not re-run: it needs the Creator's Mac, which is not going anywhere. The disagreement is recorded, not resolved.**]**

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

### 7k. Run by hand a third time: the three tests the citation check sent back, Blender 5.0

**LABEL: observed.** Added 2026-08-07. These are entries 28, 29 and 30 of `Audits/Blender Research/The machine/Hand tests — added questions.md` — the three items the citation check sent back that needed the rented Linux machine, run in a third sitting because that machine was about to be destroyed. The full record is in that list.

**On what kind of computer.** The same rented Linux machine as 7h and 7i, named in full: **Ubuntu 22.04.5 LTS, one NVIDIA GeForce RTX 4080 SUPER with 16 GB, an AMD EPYC 7B12 with twenty-four cores, 39 GB of memory, reached over SSH, and with no display server at all** — `DISPLAY` empty, `WAYLAND_DISPLAY` empty, `XDG_SESSION_TYPE=tty`, no X socket, no X or Wayland process. Its card is working and its **kernel driver and driver library agree on 580.173.02**; `nvidia-smi` reported no processes on the card, and the machine was not shared with other work this sitting. Blender 5.0.1 at `/opt/blender/5.0/blender`, **pointed at a settings folder of its own, `/root/again-the-machine/config/5.0`, made before it was used**, never at the real one; it reported `use_scripts_auto_execute = False` as its starting value. All work under `/root/again-the-machine/`, with the prepared files rebuilt by the same builder the earlier sittings used, checked by checksum. Date: 2026-08-07.

**(a) EEVEE renders here on a computer with no display server without printing anything — and prints the `libEGL` lines when it is given a different graphics library.** *(Entry 28. The first of the two results the citation check could not reproduce. Bears on 7h (n), 7i (c) and 7j (e).)*

`plain.blend` rendered with no window, and the **whole** log searched rather than glanced at. What came back, word for word, the whole run:

```
00:00.272  blend            | Read blend: "/root/again-the-machine/files/5.0/plain.blend"
00:00.284  render           | Rendering single frame
00:00.284  render           | Rendering frame 1
00:00.285  render           | Start rendering: Scene, ViewLayer
00:00.285  render           | Engine: EEVEE
00:02.155  render           | Fra: 1 | Rendering 1 / 64 samples
Blender 5.0.1 (hash a3db93c5b259 built 2025-12-16 01:30:59)
00:02.181  render           | Fra: 1 | Rendering 25 / 64 samples
00:02.203  render           | Fra: 1 | Rendering 50 / 64 samples
00:02.216  render           | Fra: 1 | Rendering 64 / 64 samples
00:02.323  render           | Saved: '/root/again-the-machine/runs/5.0/e28/e_0001.png'
00:02.323  render           | Time: 00:02.03 (Saving: 00:00.10)

Blender quit
--- EXIT STATUS: 0
--- how many lines of the whole log contain libEGL:
0
--- how many lines of the whole log contain EGL:
0
```

Asked what it was drawing with, this version answered:

```
GPU_VENDOR NVIDIA Corporation
GPU_RENDERER NVIDIA GeForce RTX 4080 SUPER/PCIe/SSE2
GPU_VERSION 4.6.0 NVIDIA 580.173.02
GPU_BACKEND OPENGL
```

Then the same render with the machine's NVIDIA graphics library hidden and only Mesa's left (`__EGL_VENDOR_LIBRARY_FILENAMES=/usr/share/glvnd/egl_vendor.d/50_mesa.json`), nothing else changed. **The lines came straight back**, after the engine line:

```
00:00.275  render           | Engine: EEVEE
libEGL warning: egl: failed to create dri2 screen
EGL Error (0x3009): EGL_BAD_MATCH: Arguments are inconsistent (for example, a valid context requires buffers not supplied by a valid surface).
EGL Error (0x3009): EGL_BAD_MATCH: Arguments are inconsistent (for example, a valid context requires buffers not supplied by a valid surface).
EGL Error (0x3009): EGL_BAD_MATCH: Arguments are inconsistent (for example, a valid context requires buffers not supplied by a valid surface).
```

— the whole-log count of lines containing `EGL` was **4** — and asked the same question it now answered `GPU_RENDERER llvmpipe (LLVM 15.0.7, 256 bits)`. The render finished by itself with exit status 0 and wrote its frame either way. The same happened in 4.5, 5.1 and 5.2 on the same machine the same day.

**What this settles.** The words are exactly the words 7h records, and they came back on demand. **So neither run was mistaken about what it saw.** What decides it on this machine is **which graphics library Blender gets**: with the NVIDIA one, EEVEE draws on the card and says nothing; with only Mesa's, it draws in software and prints those four lines. **The reading offered in 7j (e) — that these lines belong to a machine reaching for a display server that is not there — is not what was observed here.**

**What this does not settle.** The mismatched driver of 2026-08-06 cannot be put back, so **nothing here shows what that machine was doing that day**. **The disagreement stands recorded, not resolved, and 7h is left exactly as written.**

**(b) An output folder that cannot be written to: how the folder is made unwritable is not what the two runs differed over.** *(Entry 29. The second of the two results the citation check could not reproduce. Bears on 7h (g) and 7j (f).)*

Two unwritable folders were made two different ways and each probed before use — **way A, a tmpfs mounted and then remounted read-only, which is how 7h (g)'s folder was made**, and **way B, a read-only bind mount, which is how the citation check's folder was made**. Both refused a write with `Read-only file system`. The same render was aimed into each. **Both ways, this version printed the same two lines**, word for word:

```
00:02.271  image.write      | ERROR OpenImageIO write failed: Could not open file "/root/again-the-machine/readonly-tmpfs/x_0001.png"
00:02.271  reports          | ERROR Render error (Success) cannot save: '/root/again-the-machine/readonly-tmpfs/x_0001.png'
00:02.271  render           | Time: 00:01.98

Blender quit
--- EXIT STATUS: 1
```

Nothing was written, nobody was waited for, and the exit status was 1. **So the way the folder is made unwritable is not what the two earlier runs differed over.** With only Mesa's graphics library, this version **did not move at all** — `ERROR Render error (Success) cannot save: …` again, both ways. **`Success` is what both earlier runs found in this version; there was never a disagreement here.** The disagreement was in 4.5, whose bracketed word does move with the graphics library, and **both of its wordings were reproduced on demand**, so neither run there was mistaken either. **The disagreement stands recorded, not resolved.**

**What this adds.** The bracketed word cannot be trusted to say what went wrong in any version now — 4.5's moves with something that has nothing to do with the failure, and this version's has said `Success` throughout. **The real reason is in the `OpenImageIO write failed` line above it.** And what the answer rests on did not move in any of the eight runs: an error, nothing written, nobody waited for, exit status 1.

**(c) A missing texture is not silent after all — and 7h (h) was measuring nothing.** *(Entry 30. The control the citation check asked for.)*

On the file 7h (h) actually used: rendered once with the image in place, the image deleted, rendered again, and **the two frames compared as pictures**:

```
--- the texture file before the first render:
-rw-r--r-- 1 root root 177 Aug  7 05:41 files/5.0/tex.png
--- now delete it:
ls: cannot access 'files/5.0/tex.png': No such file or directory
--- THE CONTROL, the two pictures compared:
PIXELS_IDENTICAL max_channel_difference=0.000000
```

**The control failed.** The file was asked what it contains, and this version answered:

```
SLOTS ['Material', 'M']
MATERIAL M
  LINK Principled BSDF BSDF -> Material Output Surface
  LINK Image Texture Color -> Principled BSDF Base Color
MATERIAL Material
  LINK Principled BSDF BSDF -> Material Output Surface
IMAGE 'tex.png' source FILE size (0, 0) has_data False file //tex.png
```

The cube carries **two** material slots. The one holding the image was appended as slot 1, and every face is on slot 0. **So the deleted image was never in the picture, and 7h (h)'s silence is silence about a texture that reached nothing.**

The recipe was rebuilt with **one thing changed** — the textured material put in the slot the faces use — and rendered three times: with the image as built, with the image replaced by a plain red one at the same path, and with the image deleted:

```
SLOTS AFTER ['M']
FACE MATERIAL INDEXES [0]
--- as built vs red (does the texture reach the picture at all?):
PIXELS_DIFFER max_channel_difference=0.701961
--- as built vs deleted:
PIXELS_DIFFER max_channel_difference=0.666667
--- red vs deleted (THE CONTROL):
PIXELS_DIFFER max_channel_difference=0.694118
```

**The control fires**, and identically in all four versions. And with the texture in the picture, **this version does say something about the missing file**, word for word, the whole run:

```
00:00.285  blend            | Read blend: "/root/again-the-machine/files/5.0/mt2/missingtex2.blend"
00:00.298  render           | Rendering single frame
00:00.298  render           | Rendering frame 1
00:00.298  render           | Start rendering: Scene, ViewLayer
00:00.298  render           | Engine: EEVEE
00:02.069  gpu.texture      | ERROR Failed to create GPU texture from Blender image
00:02.119  render           | Fra: 1 | Rendering 1 / 64 samples
Blender 5.0.1 (hash a3db93c5b259 built 2025-12-16 01:30:59)
00:02.140  render           | Fra: 1 | Rendering 25 / 64 samples
00:02.158  render           | Fra: 1 | Rendering 50 / 64 samples
00:02.168  render           | Fra: 1 | Rendering 64 / 64 samples
00:02.275  render           | Saved: '/root/again-the-machine/runs/5.0/e30e/m_0001.png'
00:02.275  render           | Time: 00:01.97 (Saving: 00:00.10)

Blender quit
--- EXIT STATUS: 0
```

**And the message is tied to the missing file, not to the file being open.** Both runs with an image present were searched for the words texture, not loaded and failed, and returned nothing. **This version marks it `ERROR`, in a column of its own named `gpu.texture`** — which is more than 4.5 does, where the same event prints `GPUTexture: Blender Texture Not Loaded!` with no marking at all.

**What this settles.** **7h (h)'s own record stands and is not touched** — it is a true record of what that file did — but it does not answer the question it was written for, and its conclusion that a missing texture is not mentioned is **not what happens on a file where the texture is in the picture**. So an unattended run does have something to watch for here. **What did not change is the half that matters most: nothing waited.** On both files, with the image and without it, the run ended by itself with exit status 0 and wrote its frame — and that half now rests on a test that could have failed.

**One thing this does not settle.** These messages came from EEVEE. Whether Cycles says the same about a missing image was not asked.

**Not covered by any of this.** All of it is Linux, on a machine with no display server. **The Mac half of two of these three entries was not run**: the citation check asked for it, and the Creator's Mac is not going anywhere, so it was left; the reasons are in the hand-test list. Nothing here says what happens on Windows.

---

### Two things about how far this answer reaches, plus one warning about sharing a card

**GPU sharing, since it comes up on a machine left alone. LABEL: documented.** Two places where the *machine*, not a person, cuts a computation short. The first is about a Cycles render. The second is about a compositing task, which is a different thing and is quoted as such:

> "The NVIDIA OpenGL driver lost connection with the display driver
>
> If a GPU is used for both display and rendering, Windows has a limit on the time the GPU can do render computations. If you have a particularly heavy scene, Cycles can take up too much GPU time."

Source: the manual, "GPU Rendering", https://docs.blender.org/manual/en/5.0/render/cycles/gpu_rendering.html — read 2026-07-30.

> "TDR on Windows
>
> Timeout Detection Recovery (TDR) resets the GPU driver if a GPU compositing task takes longer than the default 2 second limit."

Source: the manual, "Limitations" (compositing), https://docs.blender.org/manual/en/5.0/compositing/limitations.html — read 2026-07-30.

**Outside the interface.** This is where most of the answer lives, and it holds better there than inside. Started from outside its own window with `-b`, Blender runs "without a user interface" and needs no graphical display, so the person-waiting things listed in 7c have nowhere to appear. The script-execution warning of 7a is the one to settle before starting: the manual gives `--enable-autoexec` and `--disable-autoexec` precisely for command-line rendering and says "the Preferences are still used but you may want to override them", but it never says what happens to the warning box itself when there is no window. Two further differences the sources name outright: headless rendering with EEVEE is "not supported on headless Windows systems", which is the manual's own wording and is not the same as saying it fails; and the shape of the progress output changed in 5.0, so anything reading that output has to be updated.

**That last gap is now closed for Linux, by test rather than by the sources — see 7h.** On a computer with no display server, driven from outside its own window, Blender 5.0 did not wait for anybody on any part of the render path that was tried. The warning of 7a became text on the terminal and the render carried straight on; none of the person-waiting things of 7c fired; an existing output file was written over silently; a missing texture was not even mentioned; an unwritable output folder gave an error and exit status 1 rather than a question; and two Blenders shared one output folder without being asked anything. **Two things do hold a run driven from outside**: a script that reads from standard input while something keeps standard input open, which waits for ever, and a script hooked into the render, which holds the render for as long as it runs. **Cycles can use a graphics card there** — the card on the machine used could not be reached on the day 7h was run, but it was repaired and the test was run again; see 7i. What is still open from outside: everything on macOS and Windows, and everything about what these same things do when Blender has a window.

**A second operating system has now been tried, and it agreed — see 7j.** Every one of the nineteen entries the Linux machine ran outside Blender's own window was run again on the Creator's own Mac, in all four versions, and **the two machines agreed on every one of them**, down to the same figures where a figure came back. So what is said above about a run driven from outside is no longer true of one operating system only. **The two things that hold such a run hold on macOS too**: a script reading from standard input while something keeps standard input open, which waits for ever, and a script hooked into the render, which holds the render for as long as it runs. Standard input is a real stream there as well, not nothing. **A render on the Mac also outlives the shell it was started from, and outlives a hang-up signal to its whole process group**, which is what closing a terminal window really sends. Still open from outside: everything on Windows, which no Windows machine has run.

**Three of these results were run a third time, and one of them changes (7k).** "A missing texture was not even mentioned" is **not** what happens when the texture is in the picture: Blender 5.0 prints `ERROR Failed to create GPU texture from Blender image`, in a column of its own, and the run still ends by itself with exit status 0 and writes its frame. The entry that reported silence was rendering a file whose textured material sat in a slot no face used, so it was measuring nothing. **So a run driven from outside has one more thing to watch for on its output, beyond an exit status.** Nothing else moves: an unwritable output folder still gives an error rather than a question, writes nothing, waits for nobody and comes back with exit status 1, however the folder is made unwritable — though **the reason it names in brackets cannot be trusted in any version**, this one having said `Success` throughout. And one thing said above about that machine is not a rule about Blender: the `libEGL` lines are not what a machine without a display server prints when it reaches for one. With the card's own graphics library, EEVEE renders there with no display server at all and prints nothing.

**And the window has now been reached, on the Mac — the thing every version of this answer has been leaving open (7j).** With a window open, the automatic-execution warning is a **dialog in the middle of the main window**, not a warning in a header, and it **reports rather than blocks**: a render started, finished and wrote its frame with the dialog unanswered, and with the first-run Quick Set Up unanswered underneath it. A **file chooser opened on top of a running animation render stopped no frames at all**. A still render nobody asks to be saved still writes nothing — **but the picture opens in a window of its own either way**. A render asked for on the command line with a window open is written at once, **but that Blender then never quits**. A `--background` render **does register with the Mac's window server** for the whole run, though it never comes to the front and draws nothing. And Render Audio is the one place where the menu's way and a script's way part company: **with no path named it opens a chooser and writes nothing; with a path named it writes straight over an existing file without asking** — and that, not the window, is what decides.

**Operating system.** Not uniform, and the sources name the differences.
- Windows: EEVEE headless rendering is not supported on headless Windows systems (manual, EEVEE Limitations). Windows has a time limit on how long a card used for display may spend computing a render, and a separate 2-second reset for GPU compositing tasks (manual, GPU Rendering; manual, compositing Limitations). A graphical crash report appearing in background mode was a fault fixed for 5.0 and backported to 4.5.1 (release notes, bugfixes).
- Linux: no display server is needed for command-line rendering — "no need for X server on Linux for example" (manual, Rendering From The Command Line). A freeze when force-cancelling a command-line render was fixed for 5.0 (release notes, bugfixes).
- macOS: the sources name no macOS-specific behaviour on this question, and nothing has been tested there either.
Everything else above is written without naming an operating system, and I have not assumed it applies everywhere.

**Linux is now settled by test as well, and only Linux (7h).** Everything in 7h was got on Ubuntu 22.04.5 LTS on a machine with no display server. It says nothing about macOS, where the same tests have not been run, and nothing about Windows, which no Windows machine has run. So where an answer above says the sources attach no operating system to it, that is still true of the sources; what has changed is that one operating system has now been tried.

**macOS has now been tried as well (7j), and it agreed with Linux on everything the two machines both ran.** The Creator's own Mac — a MacBook Pro with an Apple M1 Pro chip on macOS 26.5.2, with Apple's own graphics and no second card — gave the same answer as the rented Linux machine on all nineteen entries that were run on both, in all four versions. **It also reached the one thing Linux could not: a window.** Two differences between the two machines showed in what was printed, and neither changes an answer: the Mac prints no `libEGL` or `EGL Error` lines, because it has a display server; and Blender 4.5 on the Mac prints a run of `Freestyle:` progress lines that the Linux record does not show.

**The first of those two has since been taken back, by test on the Linux machine (7k).** The `libEGL` and `EGL Error` lines are **not** the mark of a machine without a display server. On that same machine, with its own card's graphics library, EEVEE renders with no display server at all and prints none of them; with that library hidden and only Mesa's software one left, all four versions print them word for word. **So it is which graphics library Blender gets that decides, not whether there is a display server**, and the Mac's silence is not evidence either way. The Freestyle difference is untouched and stands.

**Windows is still untouched** — no Windows machine has run it — so every statement above that names Windows stays untested. **And one thing on the Mac was not run**: whether the screen locking or the display going to sleep interrupts a render. That needs the Creator's own machine to itself for several minutes, and this sitting had no standing to take it; the test is written out in full in `Audits/Blender Research/The machine/Hand tests — added questions.md`, entry 21.

---

## Record of where I looked

**Searched across the whole 5.0 manual and the whole 5.0 API documentation** (and, where noted, the 5.0 release notes and Cycles' files) for these words and phrases: *background mode*, *in background*, *background-mode*, *UI-less*, *headless*, *unattended*, *without a user interface*, *no interface*, *dialog*, *popup*, *pop-up*, *prompt*, *confirm*, *modal*, *interactive*, *user input*, *user interaction*, *requires user*, *waits for*, *wait*, *waiting*, *blocking*, *pause*, *resume*, *hangs*, *unresponsive*, *not responding*, *timeout*, *time out*, *freeze*, *Auto Run*, *Trusted Source*, *Allow Execution*, *Ignore the scripts*, *Overwrite*, *Placeholders*, *Lock Interface*, *Display Mode*, *Render In*, *Temporary Editors*, *Save Prompt*, *unsaved changes*, *Quit*, *splash*, *Find Missing Files*, *missing file*, *Allow Online Access*, *online access*, *screensaver*, *screen saver*, *sleep*, *idle*, *log out*, *logged in*, *session*, *power management*, *X server*, *DISPLAY*, *EGL*, *Wayland*, *remote desktop*, *virtual machine*, *as a service*, *daemon*, *batch render*, *render farm*, *overnight*, *while rendering*, *during rendering*, *during a render*, *nobody*, *no user is*.

**Pages opened in full:** the manual's Scripting & Security, Command Line Arguments, Rendering From The Command Line, Operators, File Browser, Splash Screen, Status Bar (relevant part), Interface Preferences (Temporary Editors), Save & Load Preferences (relevant parts), Opening & Saving Files (relevant parts), Compatibility (relevant part), Rendering Animations, Output Introduction, Output Properties (relevant parts), Rendering introduction, EEVEE Limitations, compositing Limitations, Cycles GPU Rendering, Deploying Blender in Production, Configuration Introduction, Drivers Troubleshooting, Crashes, Startup, Topbar (relevant parts). The API documentation's `bpy.ops` overview, `bpy.app`, `bpy.app.handlers` (relevant parts), `bpy.ops.render`, `bpy.ops.wm` (relevant parts), `bpy.types.WindowManager` (relevant parts), `bpy.types.PreferencesFilePaths` (relevant part), `info_tips_and_tricks`. The 5.0 release notes for core, python_api, compositor, bugfixes and corrective_releases (relevant parts). Cycles' `operators.py` in full, and all of Cycles' Python files by search.

**Pictures opened:** `_images/advanced_scripting_security_autorun-scripts-dialog.png`, from the manual's Scripting & Security page.

**Ruled out and why.**
- The requirements page (https://www.blender.org/download/requirements/, read 2026-08-01) — searched it for *background*, *headless*, *display*, *remote*, *monitor*. It describes hardware and named operating-system versions only, and says nothing about interaction, prompts, or unattended operation. It does not name 5.0 in the sections that would matter here, so it could not speak for 5.0 even if it did.
- The manual's physics baking pages, which do describe pausing and resuming with keys pressed by a person, were ruled out: they are about baking simulations, not about rendering, and the question asks about a render.
- The animation player pages (`-a` as a player) were ruled out: the manual says that mode is "ignored if -b is set", so it cannot apply to an unattended command-line render.
- Nothing was found anywhere on screen locking, sleeping, logging out, running without a logged-in user, or power settings. On those, the answer is genuinely **not in the sources**.

---

## Things a person with Blender open could settle by hand in a few minutes

**Pruned 2026-08-06 after the hand tests, again 2026-08-07 after the card tests, and again 2026-08-07 after the Mac sitting.** Everything a terminal could reach was run on the rented Linux machine; everything needing a window was then run on the Creator's own Mac. **All of it is answered in 7h, 7i and 7j, and those items are gone from this list.** The full list, with every command written out, is `Audits/Blender Research/The machine/Hand tests — added questions.md`.

**What is left is two things, and neither is about Blender being unwilling.**

1. **Does the screen locking, or the display going to sleep, interrupt a render? — Not run, and it needs the Creator.**
   The only machine this project has with a screen is the Creator's own computer. This test puts that screen to sleep on purpose and then needs the machine left untouched for several minutes, and the Mac sitting had no standing to take the Creator's own desk over that way. It is written out in full as entry 21 of the hand-test list. *What to look at:* whether any frame file carries a written-time from inside the stretch when the screen was asleep, whether the process is still alive at the end, and the last line of the log. **The nearest thing that was answered is that a render on that Mac survived the shell it was started from being killed, and survived a hang-up signal to its whole process group — which is a different and smaller event.**

2. **Everything on Windows. — Cannot be run.**
   No Windows machine has run it, so the EEVEE headless-Windows statement, the Windows crash pop-up, and the two Windows-only command-line arguments all stay untested rather than answered. Each is written out in the hand-test list, ready for the next Windows rental.

---

2026-08-06 — Opus 5 (Claude Code) — The machine, Blender 5.0

2026-08-07 — Opus 5 (Claude Code) — the card tests for The machine, added questions

2026-08-07 — Opus 5 (Claude Code) — the Mac hand tests for The machine, added questions

2026-08-07 — Opus 5 (Claude Code) — the tests the citation check sent back for The machine, added questions

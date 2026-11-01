# The machine — added questions (Blender 5.2)

**Sources consulted, and when.** Blender 5.2's manual, Blender 5.2's Python API documentation, the Blender 5.2 release notes, and Cycles' own files taken from Blender 5.2 — all read 2026-07-30. Blender's requirements page, read 2026-08-01; it does not name 5.2 anywhere, so it cannot speak for this version, and in any case it says nothing on this question.

I read only the question sheet and files inside Blender Sources/. I did not open any other part of this project.

Scope of this document: question 7 only. Questions 1–6 were left alone.

---

## 7. Can anything a render does wait for a person, and what happens on a computer with nobody at it?

**A note on the question.** It can be read two ways. I answer the first.

- The reading I take: on the path from asking for a render to the finished frames sitting on disk, is there any point at which Blender stops and will not go on until a person does something — and what becomes of that on a machine nobody is sitting at?
- The other reading: is there a feature by which a render can be deliberately told to pause and wait for a person. I found one thing that fits that reading and report it too, at (5).

**LABEL: mixed — each finding below carries its own label.** In short: several waits are documented, all of them in Blender's own window; and the sources are silent on the one thing that matters most, which is what those same waits do when Blender is run with no window.

---

### (1) A blend-file that wants to run Python stops and asks — documented

The manual names animation drivers among the things that count as automatic execution. **inferred** — that this puts the wait on the render path: a reader has to supply that a render evaluates drivers. The quoted words also do not say at what moment the dialog appears, whether on opening the file or when the driver is first used.

Exact words, Blender 5.2 manual, "Scripting & Security":

> "Automatic execution is disabled by default; however, some blend-files need this to function properly."

> "When a blend-file tries to execute a script and is not allowed, a dialog will appear.
> In it, you can choose to Allow Execution or to Ignore the scripts."

The same page lists what counts as automatic execution:

> "Registered Text-Blocks
>
> A text data-block can have its Register option enabled which means it will load on start.
>
> Animation Drivers
>
> Python expressions can be used to Drive values and are often used in more advanced rigs and animations."

- Source: Blender 5.2 manual — https://docs.blender.org/manual/en/5.2/advanced/scripting/security.html
- Date read: 2026-07-30

I opened the picture the manual shows at that point (`_images/advanced_scripting_security_autorun-scripts-dialog.png`, shown on the page above and again on the drivers troubleshooting page; read 2026-07-30). **documented** — what is visible in it: a warning panel reading "For security reasons, automatic execution of Python scripts in this file was disabled:", naming the offending item ("Text 'Text.py'"), then "This may lead to unexpected behavior", a checkbox "Permanently allow execution of scripts", and two buttons, "Allow Execution" and "Ignore". Nothing in the picture shows a timeout or a default choice.

The drivers page ties this to getting a scene to evaluate at all:

> "By default Blender will restrict execution of Python scripts."

> "If using a Scripted Expression Driver Type that doesn't follow
> the Simple Expressions
> subset, you will have to open the file as Trusted Source,
> or set Auto Run Python Scripts in Preferences ‣ Save & Load ‣ Blender Files."

- Source: Blender 5.2 manual — https://docs.blender.org/manual/en/5.2/animation/drivers/troubleshooting.html
- Date read: 2026-07-30

The defaults are stated outright in the API documentation:

> "use_scripts_auto_execute
>
> Allow any .blend file to run scripts automatically (unsafe with blend files from an untrusted source) (default False)"

- Source: Blender 5.2 Python API — https://docs.blender.org/api/5.2/bpy.types.PreferencesFilePaths.html
- Date read: 2026-07-30

and on the command line:

> "-Y, --disable-autoexec
>
> Disable automatic Python script execution (Python-drivers & startup scripts), (default)."

- Source: Blender 5.2 manual — https://docs.blender.org/manual/en/5.2/advanced/command_line/arguments.html
- Date read: 2026-07-30

**The manual's own answer for an unattended render is to settle this in advance, on the command line.** Exact words:

> "You may want to perform batch rendering or some other task from the command line,
> running Blender without an interface.
> In this case, the Preferences are still used but you may want to override them:
>
> Enable with -y or --enable-autoexec
>
> Disable with -Y or --disable-autoexec"

> "To render an animation in background mode, allowing drivers and other scripts to run:
>
> blender --background --enable-autoexec my_movie.blend --render-anim"

- Source: Blender 5.2 manual — https://docs.blender.org/manual/en/5.2/advanced/scripting/security.html
- Date read: 2026-07-30

There is also a way to find out after the fact that scripts were blocked, rather than being stopped by a dialog. Exact words:

> "bpy.app.autoexec_fail
>
> Boolean, True when auto-execution of scripts failed (read-only)."

> "bpy.app.autoexec_fail_message
>
> String, message describing the auto-execution failure (read-only)."

> "bpy.app.autoexec_fail_quiet
>
> Boolean, True when auto-execution failure should be quiet, set after the warning is shown once for the current blend file (read-only)."

- Source: Blender 5.2 Python API — https://docs.blender.org/api/5.2/bpy.app.html
- Date read: 2026-07-30

One more line from the security page, because it cuts the other way and is easy to misread. Exact words:

> "There are other ways scripts in a blend-file may execute that require user
> interaction (therefore will run even when auto execution is off),
> but you should be aware that this is the case since it is not necessarily obvious.
>
> Running a script in the Text editor.
>
> Rendering with Freestyle, because Freestyle uses scripts to control line styles."

- Source: Blender 5.2 manual — https://docs.blender.org/manual/en/5.2/advanced/scripting/security.html
- Date read: 2026-07-30

So the manual says those line-style scripts run even when automatic execution is off. **inferred** — that no permission dialog stands in their way: the quote says they run, not that nothing asks. The same sentence calls these ways ones that "require user interaction", which could be read as a person starting the render or as a wait for a person; the quoted words do not settle which.

---

### (2) A still render is not written to disk unless something asks for it — documented

This is not a dialog, but it is a render that ends needing a person.

Exact words:

> "By default, animation renders are saved automatically, while still renders are
> not. Still images must be saved manually from the Image Editor
> or written using a File Output Node."

- Source: Blender 5.2 manual — https://docs.blender.org/manual/en/5.2/render/output/properties/output.html
- Date read: 2026-07-30

And on the companion page:

> "While rendering stills will allow you to view and save the image from the render buffer when
> it is complete, animations are a series of images, or frames, and are typically saved directly
> to disk as they are rendered."

- Source: Blender 5.2 manual — https://docs.blender.org/manual/en/5.2/render/output/animation.html
- Date read: 2026-07-30

Two things remove the person from that step, both documented. First, the render operator has a switch:

> "write_still (bool) – Write Image, Save the rendered image to the output path (used only when animation is disabled) (optional)"

- Source: Blender 5.2 Python API — https://docs.blender.org/api/5.2/bpy.ops.render.html
- Date read: 2026-07-30

Second, the command line frame argument says it saves:

> "-f, --render-frame <frame>
>
> Render frame <frame> and save it."

- Source: Blender 5.2 manual — https://docs.blender.org/manual/en/5.2/advanced/command_line/arguments.html
- Date read: 2026-07-30

---

### (3) Two prompts sit around a render rather than inside it — documented

The quotes tie each one to a moment outside the render itself: exiting Blender or loading another file, and saving the blend-file.

Unsaved changes on exit or on loading another file:

> "By default, when exiting Blender or loading a new blend-file, if you have unsaved changes,
> a pop-up will ask you to either confirm discarding those changes, or save them.
>
> This behavior can be disabled with the Save Prompt option in the Save & Load section
> of the Preferences."

- Source: Blender 5.2 manual — https://docs.blender.org/manual/en/5.2/files/blend/open_save.html
- Date read: 2026-07-30

Its default:

> "Ask for confirmation when quitting with unsaved changes (default True)"

- Source: Blender 5.2 Python API — https://docs.blender.org/api/5.2/bpy.types.PreferencesView.html
- Date read: 2026-07-30

Modified images when the blend-file is saved. This one is new in this version. Release note, exact words:

> "* When saving a file with unsaved images, a new dialog box now asks if the user wants to also save the modified images. (blender/blender@37518ed6cc)"
> "* A new user preference was added to allow specifying image saving behavior when saving a file with modified images. (blender/blender@bc57513da3)"

- Source: Blender 5.2 release notes — https://developer.blender.org/docs/release_notes/5.2/user_interface/
- Date read: 2026-07-30

The manual describes the preference, and names Render Results among what it covers:

> "Save Modified Images
>
> Controls how modified images are handled when saving the blend-file.
>
> This affects images edited within Blender, such as painted textures,
> generated images, Render Results, and Compositor Viewer node results
> that have unsaved changes."

> "Ask Every Time:
>
> Displays a confirmation dialog when saving the blend-file, allowing
> modified images to be saved individually."

- Source: Blender 5.2 manual — https://docs.blender.org/manual/en/5.2/editors/preferences/save_load.html
- Date read: 2026-07-30

Its default is to ask:

> "save_modified_images
>
> How modified images should be handled when saving the .blend file (default 'ASK')"

- Source: Blender 5.2 Python API — https://docs.blender.org/api/5.2/bpy.types.PreferencesFilePaths.html
- Date read: 2026-07-30

The save operator carries the dialog as a switch of its own, off by default in the signature the documentation prints:

> "bpy.ops.wm.save_mainfile(*, filepath='', hide_props_region=True, check_existing=True, … , exit=False, incremental=False, show_save_modified_images_dialog=False)"

> "show_save_modified_images_dialog (bool) – Show Save Modified Images Dialog, Show a popup dialog to save modified images before saving the blend file (optional)"

- Source: Blender 5.2 Python API — https://docs.blender.org/api/5.2/bpy.ops.wm.html
- Date read: 2026-07-30

---

### (4) Calls made from Python default to the non-asking path — documented

Exact words:

> "This determines the context that is given for the operator to run in, and whether
> invoke() is called or only execute().
>
> EXEC_DEFAULT is used by default, running only the execute() method, but you may
> want the operator to take user interaction with INVOKE_DEFAULT which will also
> call invoke() if existing."

- Source: Blender 5.2 Python API — https://docs.blender.org/api/5.2/bpy.ops.html
- Date read: 2026-07-30

The manual's definition of the operators that do wait:

> "Modal Operators
>
> Modal operators exist as a concept in between Tools
> and regular operators.
> They require some sort of interactive input.
>
> Esc, RMB
>
> Cancels a modal operator.
>
> Return, LMB
>
> Confirms the action of a modal operator."

- Source: Blender 5.2 manual — https://docs.blender.org/manual/en/5.2/interface/operators.html
- Date read: 2026-07-30

**inferred** — a render driven from Python by calling `bpy.ops.render.render(...)` without naming an execution context takes the `execute()` path, so whatever asking `invoke()` might do is not done. The step a reader has to supply: that `bpy.ops.render.render` is an ordinary operator to which the quoted default applies. The documentation does not say so about that operator specifically; its own entry reads "Undocumented, consider contributing."

---

### (5) Cycles has a setting named for pausing viewport preview renders — observed

From Cycles' own files.

`Blender Sources/cycles_5.2/properties.py`, lines 399–404, inside `class CyclesRenderSettings`:

```
    preview_pause: BoolProperty(
        name="Pause Preview",
        description="Pause all viewport preview renders",
        default=False,
        update=update_pause,
    )
```

`Blender Sources/cycles_5.2/ui.py`, line 2549, drawn only when the viewport shading is 'RENDERED':

```
            layout.prop(cscene, "preview_pause", icon='PLAY' if cscene.preview_pause else 'PAUSE', text="")
```

`Blender Sources/cycles_5.2/__init__.py`, lines 86–97, in `view_update`:

```
        if not self.session:
            # When starting a new render session in viewport (by switching
            # viewport to Rendered shading) unpause the render. The way to think
            # of it is: artist requests render, so we start to render.
```

- Date read: 2026-07-30

**observed** — the setting exists, its own description limits it to "viewport preview renders", and it is a switch a person flips. Nothing equivalent appears for a final render. Note the line I am not crossing: what a paused viewport render actually does while paused is a step beyond what these files say, and I did not take it.

Also observed, from the same folder: **no file in Cycles' Python code defines an `invoke` or a `modal` method, and none calls `invoke_props_dialog`, `invoke_confirm`, `invoke_popup` or `fileselect_add`.** I searched every `.py` file in `Blender Sources/cycles_5.2/` for each of those terms and got no matches. The operator classes there — `CYCLES_OT_use_shading_nodes`, `CYCLES_OT_denoise_animation`, `CYCLES_OT_merge_images` in `operators.py`, and the four preset operators in `presets.py` — define `poll` and `execute` only. (That last listing is stated from the same search rather than quoted: no lines of `operators.py` or `presets.py` appear above, and no line numbers are given for them.) **inferred** from that: Cycles' own add-on code contributes no dialog and no waiting step of its own to a render. The step a reader has to supply: that a dialog or a wait in a Blender add-on has to come through one of those named entry points. Cycles' compiled part (`source/`) is C++ and I did not read it for this.

---

### (6) Cycles is handed the value that says whether Blender is running without a user interface — observed

`Blender Sources/cycles_5.2/engine.py`, line 52, inside `def init()`:

```
    _cycles.init(path, user_path, bpy.app.background)
```

and the value it is handed, from the API documentation:

> "bpy.app.background
>
> Boolean, True when blender is running without a user interface (started with -b)"

- Sources: `Blender Sources/cycles_5.2/engine.py`, line 52 (read 2026-07-30); Blender 5.2 Python API — https://docs.blender.org/api/5.2/bpy.app.html (read 2026-07-30)

**observed** for the line; **documented** for what the value means. What Cycles then does differently with it is not in these files and I did not guess.

---

### (7) What the sources say about a computer with nobody at it

All **documented** unless marked.

No screen and no person is an intended way to run:

> "-b, --background
>
> Run in background (often used for UI-less rendering)."

- Source: Blender 5.2 manual — https://docs.blender.org/manual/en/5.2/advanced/command_line/arguments.html

> "One advantage of using the command line is that we do not need a graphical display
> (no need for X server on Linux for example)
> and consequently we can render via a remote shell (typically SSH)."

- Source: Blender 5.2 manual — https://docs.blender.org/manual/en/5.2/advanced/command_line/render.html

Blender itself distinguishes an unattended exit from a person's exit:

> "bpy.app.handlers.exit_pre
>
> just before Blender shuts down, while all data is still valid. Accepts one boolean argument. True indicates either that a user has been using Blender and exited, or that Blender is exiting in a circumstance that should be treated as if that were the case. False indicates that Blender is running in background mode, or is exiting due to failed command line arguments, etc."

- Source: Blender 5.2 Python API — https://docs.blender.org/api/5.2/bpy.app.handlers.html

Standard input is normally taken away from Python, and only a debug flag gives it back:

> "-d, --debug
>
> Turn debugging on.
>
> Enables memory error detection
>
> Disables mouse grab (to interact with a debugger in some cases)
>
> Keeps Python's sys.stdin rather than setting it to None"

- Source: Blender 5.2 manual — https://docs.blender.org/manual/en/5.2/advanced/command_line/arguments.html

**inferred** from that line: without `-d`, a script cannot sit waiting on typed input, because there is nothing to read from. The step a reader has to supply: that "setting it to None" removes the object a Python script would call to read a typed line. The manual does not spell out the consequence.

Two arguments exist to make an unattended run end rather than sit:

> "--python-exit-code <code>
>
> Set the exit-code in [0..255] to exit if a Python exception is raised
> (only for scripts executed from the command line), zero disables."

> "--debug-exit-on-error
>
> Immediately exit when internal errors are detected."

- Source: Blender 5.2 manual — https://docs.blender.org/manual/en/5.2/advanced/command_line/arguments.html

The manual's advice for renders left running is to make them restartable rather than attended:

> "Images can also be used for rendering animations which has a couple advantages.
> For example, when rendering animations to image file formats the render job can be canceled
> and resumed at the last rendered frame by changing the frame range."

- Source: Blender 5.2 manual — https://docs.blender.org/manual/en/5.2/render/output/introduction.html

> "Rendering takes all available CPU time; you should render overnight,
> when the computer is not needed, or set Blender to a low priority while rendering,
> and work on other things (be careful with the RAM space!)."

> "Your computer accidentally turns off in the middle of rendering your movie!
>
> Unless your animation renders in a few minutes,
> it is best to render the animation as separate image files.
> Instead of rendering directly to a compressed movie file, use a lossless format (e.g. PNG).
>
> This allows you an easy recovery if there is a problem and you have to re-start the rendering,
> since the frames you have already rendered will still be in the output directory.
>
> Just disable the Overwrite option to start rendering where you left off."

- Source: Blender 5.2 manual — https://docs.blender.org/manual/en/5.2/render/output/animation.html

And the manual's own note on several unattended machines sharing one job:

> "Hint
>
> Primitive Render Farm
>
> An easy way to get multiple machines to share the rendering workload is to:
>
> Set up a shared directory over a network file system.
>
> Disable Overwrite, enable Placeholders in the Render Output panel.
>
> Start as many machines as you wish rendering to that directory."

- Source: Blender 5.2 manual — https://docs.blender.org/manual/en/5.2/render/output/properties/output.html

Their defaults:

> "use_overwrite
>
> Overwrite existing files while rendering (default True)"

> "use_placeholder
>
> Create empty placeholder files while rendering frames (similar to Unix 'touch') (default False)"

- Source: Blender 5.2 Python API — https://docs.blender.org/api/5.2/bpy.types.RenderSettings.html

All the above read 2026-07-30.

---

### (8) With the interface up, a render opens a window by default — documented

Relevant because the default is a new window, and a machine with nobody at it has nobody to act on one.

> "render_display_type
>
> Default location where rendered images will be displayed in (default 'WINDOW')"
>
> "WINDOW
> New Window – Images are rendered in a new window."

- Source: Blender 5.2 Python API — https://docs.blender.org/api/5.2/bpy.types.PreferencesView.html

> "Render In
>
> When rendering, the user interface can do any of:
>
> Keep User Interface:
>
> The user interface does not change and the render is computed in the background."

- Source: Blender 5.2 manual — https://docs.blender.org/manual/en/5.2/editors/preferences/interface.html

There is an operator to close it without a person: `bpy.ops.render.view_cancel()`, "Cancel showing the render view" (Blender 5.2 Python API — https://docs.blender.org/api/5.2/bpy.ops.render.html). All the above read 2026-07-30.

The sources do **not** say whether that window blocks anything, or whether it closes by itself.

---

### (9) The core of the question — not in the sources

**Nothing in the Blender 5.2 manual, the Blender 5.2 Python API documentation, the Blender 5.2 release notes, or Cycles' own 5.2 files says what any of these dialogs does when Blender is run with `-b`.** Not whether the auto-run dialog appears, is skipped, is answered for you, or is replaced by a message. Not whether the unsaved-changes prompt or the save-modified-images dialog appears. Not whether a render started without an interface can be blocked by anything at all.

Nothing found in the searches below shows Blender asking anything when a file it needs is missing, when the output folder cannot be written, or when a render has finished and nobody is there. What a render *does* in those cases, beyond whether it asks, was not searched for and is not claimed here either way.

The record of where I looked is in "Where I looked" below.

---

### (10) Run by hand: what a computer actually did, Blender 5.2

**LABEL: observed.** Added 2026-08-06 from the hand tests. The full record of every command, with all of the machine's output word for word, is in `Audits/Blender Research/The machine/Hand tests — added questions.md`; the entry numbers below are that list's.

**On what kind of computer.** A rented Linux machine: Ubuntu 22.04.5 LTS, one NVIDIA GeForce RTX 4080 SUPER with 16 GB on driver 580.95.05, an AMD EPYC 7B12 with twenty-four cores, 39 GB of memory, reached over SSH, and **with no display server at all** — no X server, no Wayland, `DISPLAY` empty, no X socket, no X or Wayland process. Blender 5.2.0 LTS at `/opt/blender/5.2/blender`, **pointed at a settings folder of its own, `/root/the-machine/config/5.2`, made before it was used**, never at the real one; that folder reported `use_scripts_auto_execute = False` as its starting value. The machine was **shared with other topics' tests throughout**, so no time below is a clean measurement and none is used as one. Date: 2026-08-06.

**(a) The dialog of finding (1) does not appear as anything a run with no window has to answer.** *(Entry 3. This is the heart of (9).)* A file was made holding the factory cube with a scripted-expression driver on its X position reading `__import__('math').floor(4.5)`, and rendered twice with no window. Refusing, word for word:

```
00:00.478  blend            | Read blend: "/root/the-machine/files/5.2/driver2.blend"
	BPY_driver_exec: restricted access disallows name '__import__', enable auto-execution to support

Error in PyDriver: expression failed: __import__('math').floor(4.5)
For target: (type=Object, name="Cube", property=location, property_index=0)

00:00.486  render           | Rendering single frame
00:14.878  render           | Saved: '/root/the-machine/runs/5.2/e3r_a/d_0001.png'
00:14.878  render           | Time: 00:14.39 (Saving: 00:00.16)
```

exit status 0. Allowing wrote a frame too, exit status 0. The pictures were compared as pictures, not as files: `PIXELS_DIFFER max_channel_difference=0.517647`. So it becomes text on the terminal and the render carries straight on with a scene whose driver did not run.

**(b) Whether the driver is on the render path — the step finding (1) marks inferred — is what a computer does here.** *(Entry 3.)* The two probe runs report where the cube ended up:

```
--- refused
AUTOEXEC_FAIL True
AUTOEXEC_FAIL_MESSAGE "Driver '__import__('math').floor(4.5)'"
CUBE_X 0.0
--- allowed
AUTOEXEC_FAIL False
AUTOEXEC_FAIL_MESSAGE ''
CUBE_X 4.0
```

and the two pictures differ, so the refused driver changed what was rendered. That does not make the manual say it; it records that on this machine the driver was worked out as part of getting a picture, and that a run can read `bpy.app.autoexec_fail` to find out that scripts were refused.

**(c) A refused registered text data-block behaves the same way.** *(Entry 4.)* The run printed `scripts disabled for "/root/the-machine/files/5.2/regtext.blend", skipping 'Text.py'` and went on to write its frame with exit status 0; allowing it, the text's own line `REGISTERED TEXT RAN` appeared first.

**(d) A still render writes nothing unless asked.** *(Entry 11. Bears on finding (2).)* `bpy.ops.render.render()` with the output path set and printed left the folder empty; `bpy.ops.render.render(write_still=True)` wrote `s_.png`. Word for word:

```
OUTPUT PATH IS /root/the-machine/runs/5.2/e11a/s_
RENDER RETURNED
--- FILES:
/root/the-machine/runs/5.2/e11a:
total 0
```

**(e) Neither of the two prompts in finding (3) fires with no window.** *(Entries 6, 7, 8.)* A file changed and never saved, reaching Blender's own exit, exited by itself with status 0, asked nothing, and left the file on disk with the same checksum. A second file loaded over unsaved changes printed `SECOND FILE LOADED` and carried on. And the new-in-5.2 one, the save with modified images: a still was rendered, the file changed, and `bpy.ops.wm.save_mainfile()` called. Word for word:

```
00:12.629  render           | Time: 00:12.14 (Saving: 00:00.00)
Info: Saved "c.blend"
SAVE RETURNED

Blender quit
--- EXIT STATUS: 0
--- written-time before: 2026-08-06 23:17:32.272490469 +0000
--- written-time after:  2026-08-06 23:17:43.926660575 +0000
```

**With the Save Modified Images preference at its starting value of Ask Every Time, nothing was asked.**

**(f) Writing over a frame already on disk, with the control that shows the switch was read.** *(Entry 12.)* Rendered, then rendered again to the same path: nothing was asked, the written-time moved, and the picture was `PIXELS_IDENTICAL max_channel_difference=0.000000` with the first. A third run with Overwrite off and Placeholders off printed `00:00.529  reports          | No frames rendered, skipped to not overwrite` and left the written-time alone.

**(g) The two cases finding (9) names as unsearched.** *(Entries 14 and 15.)* A missing texture: the run ended by itself with exit status 0 and wrote its frame, and nothing whatever was printed about the missing file. An output folder that cannot be written to, word for word:

```
00:10.846  image.write      | ERROR OpenImageIO write failed: Could not open file "/root/the-machine/readonly/x_0001.png"
00:10.846  reports          | ERROR Render error (Success) cannot save: '/root/the-machine/readonly/x_0001.png'
00:10.846  render           | Time: 00:10.51

Blender quit
--- EXIT STATUS: 1
```

Nothing was written, nothing was asked. **This is the one place in the whole set where a failed unattended run told the outside world about it through its exit status**, which is what the researcher's own test 3 asked.

**(h) Blender ends by itself after a background run.** *(Entries 1, 2, 5. The researcher's own test 6.)* A still render, an animation render, and a run given a file and no render to do all gave the prompt back on their own with exit status 0.

**(i) A script inside Blender can hold an unattended run open for ever.** *(Entries 9 and 10. This is the one thing found that waits.)* With and without `--debug`, Blender printed the same thing, which is not nothing:

```
STDIN IS <_io.TextIOWrapper name='<stdin>' mode='r' encoding='utf-8'>
```

so the inference in finding (7) is not what a computer does here; it is left standing as the researcher wrote it, and this sits beside it. A script calling `input()` then failed at once with `EOFError: EOF when reading a line` when nothing was on standard input — and with something holding standard input open and never sending anything, the same command was **still sitting there sixty seconds later**, stopped only by a time limit.

**(j) A script hooked into the render holds the render for as long as it runs.** *(Entry 16.)* A script waiting forty-five seconds was added to `bpy.app.handlers.render_pre`: `HANDLER STARTED`, then `HANDLER FINISHED`, then the frame saved, then `RENDER RETURNED`, 57 seconds in all where the same scene rendered alone in ten to fifteen.

**(k) Two Blenders share one output folder without being asked anything.** *(Entry 13.)* Two runs of the same forty-frame animation started at once into one folder with Overwrite off and Placeholders on. Both ended by themselves with exit status 0, all forty frames were there and none was empty, and each skipped what the other had taken. **This is not the arrangement the manual describes**, which has several machines sharing a folder over a network file system.

**(l) Freestyle changes the picture with automatic script execution switched off.** *(Entry 20. Bears on the last paragraph of finding (1).)* The same scene saved with `scene.render.use_freestyle` and `view_layer.use_freestyle` on and off, both rendered with `--disable-autoexec`: both ended by themselves, neither said anything about scripts, the Freestyle run printed `00:11.482  render           | Rendering freestyle` as a step of its own, and `PIXELS_DIFFER max_channel_difference=0.658824`.

**(m) Render Audio, driven from a script, writes straight over a file already there.** *(Entry 19.)* `bpy.ops.sound.mixdown(filepath=…)` aimed at a path where a file already sat returned `MIXDOWN RETURNED {'FINISHED'}`, ended in about a second with exit status 0, and replaced the file — 24 bytes became 8302 and the checksum changed. No window, nothing asked.

**(n) A render survives the connection it was started from being cut.** *(Entry 17.)* An animation render was started over an SSH connection which was then killed from the other end after forty-five seconds. On a fresh connection the process was still there with 3 frames written; ninety seconds later it was still there with 10. (The first attempt at this for 5.2 was spoiled by another of my own scripts emptying the same folder; it was run again in a folder of its own, and the figures above are from that second run.)

**(o) EEVEE renders on a computer with no display server; Cycles could not be tried on this day.** *(Entries 1 and 18.)* Every render above used EEVEE, which is what the factory settings choose here — Blender reported its own list of engines as `['BLENDER_EEVEE']`. Every run also printed `libEGL warning: egl: failed to create dri2 screen` and three `EGL Error (0x3009): EGL_BAD_MATCH` lines before rendering, and rendered anyway. Cycles was asked for through OptiX and through CUDA and printed `00:00.560  cycles           | ERROR CUDA cuInit: Unknown error` and `00:00.565  reports          | ERROR Found no Cycles device of the specified type`, then gave the prompt back in about a second with exit status 1. The machine's own `nvidia-smi` reported `Failed to initialize NVML: Driver/library version mismatch` throughout, **so this is a fault in the machine, not a finding about Blender**. The card was repaired afterwards and the entry was run again; the answer is in (11). **One thing this run settles on its own, and (11) leans on it**: when the device asked for is not there, Blender says so and stops with exit status 1 rather than quietly using the processor instead.

**Not covered by any of this.** Every result above is from Linux on a machine with no display server. Nothing here says what happens on macOS or on Windows, and nothing here says what any of these things do when Blender has a window — including finding (8)'s render window, which cannot exist here. Nothing here touches the machine's own screen locking or sleeping, because this machine has no screen.

**[Citation check, 2026-08-07: two notes on the entries above.** *The unwritable output folder:* repeating that entry in 4.5 on the same machine, with the folder made read-only by a read-only bind mount, gave a different word in the brackets from the one 4.5's own record gives, so the bracketed reason goes back to the hand tests for every version — the behaviour it rests on (an error, nothing written, nobody waited for, exit status 1) matched. *The missing texture:* as written that test has no control showing the deleted image was doing anything in the render. The control was added on a repeat in 4.5 and came back `PIXELS_DIFFER max_channel_difference=0.694118`; the control belongs in this version's record too and is sent to the hand tests.**]**

**[Hand tests, 2026-08-07: both went back to the machine, in this version — see (13).** *The unwritable output folder:* the folder was made unwritable both ways and **the two ways gave the same words as each other**, so that is not what the two runs differed over; this version said `Success` in the brackets on both ways and on both graphics paths, as both earlier runs found. *The missing texture:* the control was added to the file this entry used, **and it failed** — the two pictures are the same picture, because the textured material sits in a slot no face uses. With that put right the control fires, and **this version then prints `ERROR Failed to create GPU texture from Blender image`**, so a missing texture is mentioned when it is in the picture. Both records above are left exactly as written.**]**

---

### (11) Run by hand: Cycles on the graphics card, with no display server, Blender 5.2

**LABEL: observed.** Added 2026-08-07. This is entry 18 of `Audits/Blender Research/The machine/Hand tests — added questions.md`, run a second time after the card was repaired. It is the one thing (10) could not settle. The full record, with all of the machine's output word for word, is in that list.

**On what kind of computer.** The same rented Linux machine as (10), with its graphics card now working: Ubuntu 22.04.5 LTS, one NVIDIA GeForce RTX 4080 SUPER with 16 GB, an AMD EPYC 7B12 with twenty-four cores, 39 GB of memory, reached over SSH, and **still with no display server at all** — `DISPLAY` empty, `WAYLAND_DISPLAY` empty, `XDG_SESSION_TYPE=tty`, no X socket, no X or Wayland process. The machine's **kernel driver and its driver library now agree on 580.173.02** (they had drifted apart on the day of (10), which is why that entry failed then), and `nvidia-smi` answers instead of failing. Blender 5.2.0 LTS at `/opt/blender/5.2/blender`, **pointed at a fresh settings folder of its own, `/root/the-machine/config-card/5.2`, made before it was used**, never at the real one; that folder reported `use_scripts_auto_execute = False` as its starting value. `nvidia-smi` reported no processes and no memory in use on the card both immediately before and immediately after, so nothing else was sharing it. Date: 2026-08-07.

**What was done.** The factory cube, camera and light at 64 by 64, saved by this version, rendered with no window twice — once through each of the two ways this card can be reached:

```
/opt/blender/5.2/blender --background …/plain.blend --engine CYCLES --render-output …/e18card_OPTIX/c_ --render-frame 1 -- --cycles-device OPTIX
/opt/blender/5.2/blender --background …/plain.blend --engine CYCLES --render-output …/e18card_CUDA/c_  --render-frame 1 -- --cycles-device CUDA
```

**What came back. Both runs rendered on the card, wrote their picture, and gave the prompt back by themselves with exit status 0.** Nothing waited and nothing was asked. Through OptiX, word for word, with the repeated per-sample lines cut where marked and nothing else removed:

```
00:00.495  blend            | Read blend: "/root/the-machine/files/5.2/plain.blend"
00:00.504  render           | Rendering single frame
00:00.504  render           | Rendering frame 1
00:00.504  render           | Start rendering: Scene, ViewLayer
00:00.504  render           | Engine: Cycles
00:02.085  render           | Fra: 1 | Mem: 1M | Synchronizing object | Cube
Blender 5.2.0 LTS (hash fbe6228777e7 built 2026-07-14 01:32:04)
00:02.086  render           | Fra: 1 | Mem: 1M | Loading render kernels (may take a few minutes the first time)
00:02.228  render           | Fra: 1 | Mem: 1M | Updating Scene BVH | Building OptiX acceleration structure
00:02.239  render           | Fra: 1 | Mem: 683M | Sample 0/4096
   (the per-sample lines are cut here)
00:02.707  render           | Saved: '/root/the-machine/runs/5.2/e18card_OPTIX/c_0001.png'
00:02.707  render           | Time: 00:02.20 (Saving: 00:00.16)

Blender quit
--- EXIT STATUS: 0
--- SECONDS: 3
--- FILES:
total 8
-rw-r--r-- 1 root root 4663 Aug  7 00:32 c_0001.png
```

and through CUDA, word for word around the lines that matter:

```
00:00.504  render           | Engine: Cycles
00:01.949  render           | Fra: 1 | Mem: 0M | Loading render kernels (may take a few minutes the first time)
00:02.088  render           | Fra: 1 | Mem: 1M | Updating Scene BVH | Packing BVH nodes
00:02.102  render           | Fra: 1 | Mem: 683M | Sample 0/4096
00:02.540  render           | Saved: '/root/the-machine/runs/5.2/e18card_CUDA/c_0001.png'
00:02.540  render           | Time: 00:02.03 (Saving: 00:00.16)

Blender quit
--- EXIT STATUS: 0
--- SECONDS: 3
--- FILES:
total 8
-rw-r--r-- 1 root root 4664 Aug  7 00:32 c_0001.png
```

**(a) Cycles renders on a graphics card on a computer with no display server.** Both ways worked, on a machine with no X server, no Wayland and no screen. The same happened in 4.5, 5.0 and 5.1, all on the same machine and the same day.

**(b) What names the device, and how far it goes.** Neither run named the card itself. What the OptiX run printed is `Building OptiX acceleration structure`, and what the CUDA run printed in the same place is `Packing BVH nodes`, so the two ran by different paths. Both reported 683 megabytes of separate memory for the render beside Blender's own nought to one. **A run that finished at all is a run that got the device it asked for**, because (10) (o) shows that when the device is not there Blender says `ERROR Found no Cycles device of the specified type` and stops with exit status 1 rather than quietly using the processor.

**[Citation check, 2026-08-07: the label on that last sentence is corrected to inferred.** It is not something either run printed; it is a step taken from a different run, so it does not belong under this section's **observed** label. This check repeated both runs on the repaired machine and got the same figures, and ran the control the hand tests did not: asking for a device this computer has not got gave `Found no Cycles device of the specified type`, exit status 1 and nothing written. So the step holds; the label was the fault.**]**

**(c) A Cycles run printed no display-server complaints at all.** (10) (o) records that every EEVEE run on this machine prints `libEGL warning: egl: failed to create dri2 screen` and three `EGL Error (0x3009): EGL_BAD_MATCH` lines before rendering. **Neither of these two Cycles runs printed any of them.** The whole record of all eight runs of this entry was searched for the words error, warning and failed, and nothing matched. So those lines belong to EEVEE reaching for a display server, not to Blender starting up on this machine.

**[Citation check, 2026-08-07: the Cycles half repeated cleanly; the contrast did not, and it is sent back to the hand tests.** Repeating all eight card runs on that machine today and searching every log for error, warning and failed returned nothing, exactly as recorded. But an EEVEE background render on the same machine today printed **no `libEGL` line and no `EGL Error` line at all**, so the sentence about every EEVEE run is what a repeat did not reproduce. The one known difference between the two days is that the graphics card's driver was mismatched then and now reads 580.173.02 on both halves. Neither run is being called right.**]**

**[Hand tests, 2026-08-07: both runs were reproduced, and the contrast is gone — see (13) (a).** With the card's own graphics library, **EEVEE on this machine prints none of those lines either**, and reports it is drawing on the card; with that library hidden and only Mesa's left, all four versions print them word for word and report `llvmpipe`. **So neither run was mistaken, and what decides it is which graphics library Blender gets, not whether there is a display server.** There is now nothing to set these Cycles runs against — what the Cycles half says on its own still stands. **The conclusion that those lines belong to EEVEE reaching for a display server should not be carried forward.** The disagreement is recorded, not resolved.**]**

**(d) Loading the render kernels for the first time was quick.** Blender's own line warns it "may take a few minutes the first time". In a settings folder never used before, on this machine, it took about a second and a half.

**Not covered by this.** It says nothing about macOS or Windows, nothing about a machine with more than one card, and nothing about how the picture compares with the same scene rendered on the processor — that was not tried. The times above are recorded, not measured; nothing rests on them.

---

### (12) Run by hand on the Creator's Mac: a second computer, and a window, Blender 5.2

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

**[Hand tests, 2026-08-07: run again on the Linux machine both ways — see (13) (b).** The way the folder is made unwritable is **not** what the two runs differed over: both ways give the same words. **The bracketed word is untrustworthy in 4.5 as well** — it moves with which graphics library Blender gets, and both wordings were reproduced on demand. The Mac half was not re-run: it needs the Creator's Mac, which is not going anywhere. The disagreement is recorded, not resolved.**]**

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

### (13) Run by hand a third time: the three tests the citation check sent back, Blender 5.2

**LABEL: observed.** Added 2026-08-07. These are entries 28, 29 and 30 of `Audits/Blender Research/The machine/Hand tests — added questions.md` — the three items the citation check sent back that needed the rented Linux machine, run in a third sitting because that machine was about to be destroyed. The full record is in that list.

**On what kind of computer.** The same rented Linux machine as (10) and (11), named in full: **Ubuntu 22.04.5 LTS, one NVIDIA GeForce RTX 4080 SUPER with 16 GB, an AMD EPYC 7B12 with twenty-four cores, 39 GB of memory, reached over SSH, and with no display server at all** — `DISPLAY` empty, `WAYLAND_DISPLAY` empty, `XDG_SESSION_TYPE=tty`, no X socket, no X or Wayland process. Its card is working and its **kernel driver and driver library agree on 580.173.02**; `nvidia-smi` reported no processes on the card, and the machine was not shared with other work this sitting. Blender 5.2.0 LTS at `/opt/blender/5.2/blender`, **pointed at a settings folder of its own, `/root/again-the-machine/config/5.2`, made before it was used**, never at the real one; it reported `use_scripts_auto_execute = False` as its starting value. All work under `/root/again-the-machine/`, with the prepared files rebuilt by the same builder the earlier sittings used, checked by checksum. Date: 2026-08-07.

**(a) EEVEE renders here on a computer with no display server without printing anything — and prints the `libEGL` lines when it is given a different graphics library.** *(Entry 28. The first of the two results the citation check could not reproduce. Bears on (10) (o), (11) (c) and (12) (e).)*

`plain.blend` rendered with no window, and the **whole** log searched rather than glanced at. What came back, word for word, the whole run:

```
00:00.482  blend            | Read blend: "/root/again-the-machine/files/5.2/plain.blend"
00:00.491  render           | Rendering single frame
00:00.491  render           | Rendering frame 1
00:00.492  render           | Start rendering: Scene, ViewLayer
00:00.492  render           | Engine: EEVEE
00:02.545  render           | Fra: 1 | Rendering 1 / 64 samples
Blender 5.2.0 LTS (hash fbe6228777e7 built 2026-07-14 01:32:04)
00:02.585  render           | Fra: 1 | Rendering 25 / 64 samples
00:02.613  render           | Fra: 1 | Rendering 50 / 64 samples
00:02.628  render           | Fra: 1 | Rendering 64 / 64 samples
00:02.797  render           | Saved: '/root/again-the-machine/runs/5.2/e28/e_0001.png'
00:02.797  render           | Time: 00:02.30 (Saving: 00:00.16)

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
00:00.505  render           | Engine: EEVEE
libEGL warning: egl: failed to create dri2 screen
EGL Error (0x3009): EGL_BAD_MATCH: Arguments are inconsistent (for example, a valid context requires buffers not supplied by a valid surface).
EGL Error (0x3009): EGL_BAD_MATCH: Arguments are inconsistent (for example, a valid context requires buffers not supplied by a valid surface).
EGL Error (0x3009): EGL_BAD_MATCH: Arguments are inconsistent (for example, a valid context requires buffers not supplied by a valid surface).
```

— the whole-log count of lines containing `EGL` was **4** — and asked the same question it now answered `GPU_RENDERER llvmpipe (LLVM 15.0.7, 256 bits)`. The render finished by itself with exit status 0 and wrote its frame either way. The same happened in 4.5, 5.0 and 5.1 on the same machine the same day.

**What this settles.** The words are exactly the words (10) (o) records, and they came back on demand. **So neither run was mistaken about what it saw.** What decides it on this machine is **which graphics library Blender gets**: with the NVIDIA one, EEVEE draws on the card and says nothing; with only Mesa's, it draws in software and prints those four lines. **The conclusion drawn in (11) (c) — that those lines belong to EEVEE reaching for a display server — is not what was observed here**: a machine with no display server at all printed nothing while getting a graphics context on the card.

**What this does not settle.** The mismatched driver of 2026-08-06 cannot be put back, so **nothing here shows what that machine was doing that day**. **The disagreement stands recorded, not resolved, and (10) (o) is left exactly as written.**

**(b) An output folder that cannot be written to: how the folder is made unwritable is not what the two runs differed over.** *(Entry 29. The second of the two results the citation check could not reproduce. Bears on (10) (g) and (12) (f).)*

Two unwritable folders were made two different ways and each probed before use — **way A, a tmpfs mounted and then remounted read-only, which is how (10) (g)'s folder was made**, and **way B, a read-only bind mount, which is how the citation check's folder was made**. Both refused a write with `Read-only file system`. The same render was aimed into each. **Both ways, this version printed the same two lines**, word for word:

```
00:02.198  image.write      | ERROR OpenImageIO write failed: Could not open file "/root/again-the-machine/readonly-tmpfs/x_0001.png"
00:02.198  reports          | ERROR Render error (Success) cannot save: '/root/again-the-machine/readonly-tmpfs/x_0001.png'
00:02.198  render           | Time: 00:01.71

Blender quit
--- EXIT STATUS: 1
```

Nothing was written, nobody was waited for, and the exit status was 1. **So the way the folder is made unwritable is not what the two earlier runs differed over.** With only Mesa's graphics library, this version **did not move at all** — `ERROR Render error (Success) cannot save: …` again, both ways. **`Success` is what both earlier runs found in this version; there was never a disagreement here.** The disagreement was in 4.5, whose bracketed word does move with the graphics library, and **both of its wordings were reproduced on demand**, so neither run there was mistaken either. **The disagreement stands recorded, not resolved.**

**What this adds.** The bracketed word cannot be trusted to say what went wrong in any version now — 4.5's moves with something that has nothing to do with the failure, and this version's has said `Success` throughout. **The real reason is in the `OpenImageIO write failed` line above it.** And what the answer rests on did not move in any of the eight runs: an error, nothing written, nobody waited for, exit status 1.

**(c) A missing texture is not silent after all — and the missing-texture half of (10) (g) was measuring nothing.** *(Entry 30. The control the citation check asked for.)*

On the file that entry actually used: rendered once with the image in place, the image deleted, rendered again, and **the two frames compared as pictures**:

```
--- the texture file before the first render:
-rw-r--r-- 1 root root 250 Aug  7 05:41 files/5.2/tex.png
--- now delete it:
ls: cannot access 'files/5.2/tex.png': No such file or directory
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

The cube carries **two** material slots. The one holding the image was appended as slot 1, and every face is on slot 0. **So the deleted image was never in the picture, and that entry's silence is silence about a texture that reached nothing.**

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
00:00.488  blend            | Read blend: "/root/again-the-machine/files/5.2/mt2/missingtex2.blend"
00:00.497  render           | Rendering single frame
00:00.497  render           | Rendering frame 1
00:00.497  render           | Start rendering: Scene, ViewLayer
00:00.497  render           | Engine: EEVEE
00:02.462  gpu.texture      | ERROR Failed to create GPU texture from Blender image
00:02.532  render           | Fra: 1 | Rendering 1 / 64 samples
Blender 5.2.0 LTS (hash fbe6228777e7 built 2026-07-14 01:32:04)
00:02.560  render           | Fra: 1 | Rendering 25 / 64 samples
00:02.584  render           | Fra: 1 | Rendering 50 / 64 samples
00:02.597  render           | Fra: 1 | Rendering 64 / 64 samples
00:02.765  render           | Saved: '/root/again-the-machine/runs/5.2/e30e/m_0001.png'
00:02.765  render           | Time: 00:02.26 (Saving: 00:00.16)

Blender quit
--- EXIT STATUS: 0
```

**And the message is tied to the missing file, not to the file being open.** Both runs with an image present were searched for the words texture, not loaded and failed, and returned nothing. **This version marks it `ERROR`, in a column of its own named `gpu.texture`** — which is more than 4.5 does, where the same event prints `GPUTexture: Blender Texture Not Loaded!` with no marking at all.

**What this settles, and it bears directly on (9).** (9) says the sources are silent on what an unattended render does about things going wrong. **The record's own answer for a missing texture — that nothing whatever is said — was got from a file where the texture reached nothing, and is not what happens when it does.** That entry's record stands and is not touched, but it does not answer the question it was written for. **What did not change is the half that matters most: nothing waited.** On both files, with the image and without it, the run ended by itself with exit status 0 and wrote its frame — and that half now rests on a test that could have failed.

**One thing this does not settle.** These messages came from EEVEE. Whether Cycles says the same about a missing image was not asked.

**Not covered by any of this.** All of it is Linux, on a machine with no display server. **The Mac half of two of these three entries was not run**: the citation check asked for it, and the Creator's Mac is not going anywhere, so it was left; the reasons are in the hand-test list. Nothing here says what happens on Windows.

---

## Not covered

- Every wait I found is documented as a thing that happens in Blender's own window. **Whether those same waits happen, are suppressed, or fail outright when Blender runs with `-b` is exactly what the sources do not say** — and that is the part that matters for an unattended machine. Do not read finding (1) as "a background render will hang"; read it as "the sources establish that this dialog exists and do not establish what becomes of it without a window".
- I did not establish what a wait does over time: no source mentions a timeout, a default answer after a delay, or a way to answer a dialog from outside.
- Add-ons other than Cycles are not covered. Any enabled add-on could add its own dialog; nothing in these sources rules that out.
- Cycles' compiled part (`Blender Sources/cycles_5.2/source/`) is C++ and I read only its file names and a couple of type headers while searching. A wait implemented there would not show up in the Python files I checked.
- The requirements page does not name 5.2, so it does not speak for this version. It says nothing on this subject anyway.
- Nothing here covers whether the *machine* (screen lock, sleep, power settings, a logged-out session) affects a running render. No source I searched mentions any of that; see below.
- The Freestyle line at (1) says those scripts "will run even when auto execution is off". It does not say whether that also holds without an interface.

**Several of the gaps listed just above have since been closed by test, and only by test — see (12).** With a window open on the Creator's own Mac, the automatic-execution warning is a dialog in the middle of the main window, not a warning in a header, and it **reports rather than blocks**: a render started, finished and wrote its frame with it unanswered. A window that *is* waiting for a person — a file chooser — did not stop a running animation render either. A still render nobody asks to be saved still writes nothing, though the picture opens in a window of its own. **The sources still say none of this**; what has changed is that it has been watched happening. What is left genuinely open in this section: everything on Windows, and what the machine's own screen locking or display sleeping does to a render, which was not run because it needs the Creator's own computer to itself.

## Outside the interface

This is the question's own weak point, and I will not paper over it.

- What is established: Blender is meant to be driven from outside — `-b` exists and is described as "often used for UI-less rendering"; a render can be run over a remote shell with no graphical display; the auto-run decision has command-line overrides (`-y` / `-Y`) that the manual says "are still used" over the Preferences; calls into Blender from Python take the non-asking `execute()` path by default; standard input is normally taken away from Python; and Blender knows internally which mode it is in (`bpy.app.background`, and the `exit_pre` handler's boolean).
- What is not established **from the sources**: whether any of the dialogs at (1) and (3) appear at all with no window, and if they do not, what Blender does instead. Neither the manual, the API documentation, the release notes, nor Cycles' files answer this. **An answer that only holds inside the interface is of no use here, and that is what most of this document is.** The one strong, outside-the-interface fact from the sources is (1)'s documented command-line example, which shows the intended way to settle the script question before a background render ever starts.
- **It is now established for Linux by test — see (10).** On a computer with no display server, driven from outside its own window, Blender 5.2 did not wait for anybody on any part of the render path that was tried: the dialog of (1) became text on the terminal and the render carried straight on; neither prompt of (3) fired, **including the new-in-5.2 one about modified images, with its preference at Ask Every Time**; an existing output file was written over silently; a missing texture was not even mentioned; an unwritable output folder gave an error and exit status 1 rather than a question; Render Audio wrote straight over a file already there; and two Blenders shared one output folder without being asked anything. **Two things do hold a run driven from outside**: a script that reads from standard input while something keeps standard input open, which waits for ever, and a script hooked into the render, which holds the render for as long as it runs. Standard input in an ordinary background run is a real stream, not nothing, so the inference in (7) is not what a computer does here. **Cycles can use a graphics card there** — that was the one thing (10) could not settle, and (11) settles it. Still open from outside: everything on macOS and Windows, and everything about what these same things do when Blender has a window — including (8)'s render window.
- Finding (5), Cycles' pause, is described in its own file as applying to "viewport preview renders" — which do not exist without a window. Finding (8), the render window, likewise.

**A second operating system has now been tried, and it agreed — see (12).** Every one of the nineteen entries the Linux machine ran outside Blender's own window was run again on the Creator's own Mac, in all four versions, and **the two machines agreed on every one of them**, down to the same figures where a figure came back. So what is said above about a run driven from outside is no longer true of one operating system only. **The two things that hold such a run hold on macOS too**: a script reading from standard input while something keeps standard input open, which waits for ever, and a script hooked into the render, which holds the render for as long as it runs. Standard input is a real stream there as well, not nothing. **A render on the Mac also outlives the shell it was started from, and outlives a hang-up signal to its whole process group**, which is what closing a terminal window really sends. Still open from outside: everything on Windows, which no Windows machine has run.

**Three of these results were run a third time, and one of them changes ((13)).** "A missing texture is not mentioned at all" is **not** what happens when the texture is in the picture: Blender 5.2 prints `ERROR Failed to create GPU texture from Blender image`, in a column of its own, and the run still ends by itself with exit status 0 and writes its frame. The entry that reported silence was rendering a file whose textured material sat in a slot no face used, so it was measuring nothing. **So a run driven from outside has one more thing to watch for on its output, beyond an exit status** — which narrows the silence (9) records. Nothing else moves: an unwritable output folder still gives an error rather than a question, writes nothing, waits for nobody and comes back with exit status 1, however the folder is made unwritable — though **the reason it names in brackets cannot be trusted in any version**, this one having said `Success` throughout. And one thing said above about that machine is not a rule about Blender: the `libEGL` lines are not what a machine without a display server prints when it reaches for one. With the card's own graphics library, EEVEE renders there with no display server at all and prints nothing.

**And the window has now been reached, on the Mac — the thing every version of this answer has been leaving open ((12)).** With a window open, the automatic-execution warning is a **dialog in the middle of the main window**, not a warning in a header, and it **reports rather than blocks**: a render started, finished and wrote its frame with the dialog unanswered, and with the first-run Quick Set Up unanswered underneath it. A **file chooser opened on top of a running animation render stopped no frames at all**. A still render nobody asks to be saved still writes nothing — **but the picture opens in a window of its own either way**. A render asked for on the command line with a window open is written at once, **but that Blender then never quits**. A `--background` render **does register with the Mac's window server** for the whole run, though it never comes to the front and draws nothing. And Render Audio is the one place where the menu's way and a script's way part company: **with no path named it opens a chooser and writes nothing; with a path named it writes straight over an existing file without asking** — and that, not the window, is what decides.

## Operating system

- Almost everything quoted here is stated without naming an operating system: the auto-run dialog, the save prompts, the output defaults, the command-line arguments for autoexec and background mode, the Python execution-context default. On its face it holds everywhere.
- One difference is stated outright and it bears on unattended rendering: **"Headless rendering is not supported on headless Windows systems."** (Blender 5.2 manual, EEVEE limitations — https://docs.blender.org/manual/en/5.2/render/eevee/limitations/limitations.html, read 2026-07-30.) That is on the EEVEE page and I did not find an equivalent statement for Cycles.
- The no-display point is written for Linux: "no need for X server on Linux for example". The manual gives no equivalent statement for Windows or macOS.
- Blender's window-open behaviour on Windows is described separately: "When Blender is started on a Microsoft Windows operating system, the Console Window (called the Command Prompt) is first created as a separate window on the desktop. The main Blender window will also appear and the Console Window will then be toggled off." (https://docs.blender.org/manual/en/5.2/advanced/command_line/launch/windows.html, read 2026-07-30.) The manual does not say whether that console window ever waits for anything.
- Beyond those, the sources do not say.
- **Linux is now settled by test as well, and only Linux (10).** Everything in (10) was got on Ubuntu 22.04.5 LTS on a machine with no display server. It says nothing about macOS, where the same tests have not been run, and nothing about Windows, which no Windows machine has run. So where an answer above says it holds everywhere on its face, that is still only what the sources say; what has changed is that one operating system has now been tried.

**macOS has now been tried as well ((12)), and it agreed with Linux on everything the two machines both ran.** The Creator's own Mac — a MacBook Pro with an Apple M1 Pro chip on macOS 26.5.2, with Apple's own graphics and no second card — gave the same answer as the rented Linux machine on all nineteen entries that were run on both, in all four versions. **It also reached the one thing Linux could not: a window.** Two differences between the two machines showed in what was printed, and neither changes an answer: the Mac prints no `libEGL` or `EGL Error` lines, because it has a display server; and Blender 4.5 on the Mac prints a run of `Freestyle:` progress lines that the Linux record does not show.

**The first of those two has since been taken back, by test on the Linux machine ((13)).** The `libEGL` and `EGL Error` lines are **not** the mark of a machine without a display server. On that same machine, with its own card's graphics library, EEVEE renders with no display server at all and prints none of them; with that library hidden and only Mesa's software one left, all four versions print them word for word. **So it is which graphics library Blender gets that decides, not whether there is a display server**, and the Mac's silence is not evidence either way. The Freestyle difference is untouched and stands.

**Windows is still untouched** — no Windows machine has run it — so every statement above that names Windows stays untested. **And one thing on the Mac was not run**: whether the screen locking or the display going to sleep interrupts a render. That needs the Creator's own machine to itself for several minutes, and this sitting had no standing to take it; the test is written out in full in `Audits/Blender Research/The machine/Hand tests — added questions.md`, entry 21.

---

## Where I looked

Everything below was searched across the whole of `Blender Sources/manual_5.2`, `Blender Sources/api_5.2`, `Blender Sources/release_notes_5.2` and `Blender Sources/cycles_5.2` at once, unless noted.

**Terms searched, and what came back**

- `background mode`, `--background`, `UI-less`, `without a user interface`, `no user interface`, `non-interactive` — five manual/API pages plus one release note. All read. Only `arguments.txt`, `security.txt`, `render.txt`, `info_tips_and_tricks.txt`, `bpy.app.txt`, `bpy.app.handlers.txt` and `info_advanced_blender_as_bpy.txt` had anything on point.
- `dialog`, `popup`, `pop-up` — fifteen manual pages for "dialog", forty-odd for "popup". Ruled out by reading: all the "popup" hits in `interface/`, `sculpt_paint/`, `grease_pencil/`, `editors/`, `modeling/`, `animation/` are about menus, colour pickers, data-block selectors and node tools, none of them on a render path. The ones that mattered were `advanced/scripting/security.txt`, `files/blend/open_save.txt`, `editors/preferences/save_load.txt`.
- `dialog|pop-up|popup|confirm|prompt|ask` restricted to `manual_5.2/render/` and `manual_5.2/compositing/` — **four hits in the entire render and compositing sections of the manual, and none of them is a wait**: "Frequently Asked Questions", "So be careful of what you ask", a material-assignment pop-up, and "Confirm the range of your animation". This is the strongest part of the silence proof at (9).
- `wait for user`, `user input`, `waits for`, `unattended`, `blocks until`, `press any key`, `acknowledge`, `message box`, `requires confirmation`, `asks you`, `ask the user`, `will ask` — nothing across the manual or the API except the unsaved-changes pop-up and a glTF exporter note.
- `waiting` — three hits, all ruled out: two are the `DEPENDS_ON_CURSOR` cursor setting on `bpy.types.Operator` and `bpy.types.Macro`, one is a print statement in the threading example.
- `pause`, `paused`, `resume` — in the manual, only the animation player, video-strip playback, fluid baking, and walk-navigation. **The Cycles viewport pause at (5) is not in the manual or the API documentation at all**; I searched `preview_pause` and `Pause Preview` across `api_5.2` and got nothing. It exists only in Cycles' own files.
- `interactive` — many hits; read the ones in `interface/operators.txt`, `advanced/command_line/arguments.txt`, `advanced/scripting/`, and the compositor release note. Only `interface/operators.txt` was on point (modal operators).
- `modal` — `interface/operators.txt`, `bpy_types_enum_items/operator_type_flag_items.txt` (read in full; its `BLOCKING` flag means "Block anything else from using the cursor", which is about the mouse, not about waiting, so ruled out), and three bug-fix lines in the release notes.
- `auto-run`, `auto run`, `autorun`, `Allow Execution`, `Trusted Source`, `autoexec`, `use_scripts_auto_execute` — led to finding (1) and its defaults.
- `headless`, `X server`, `Xorg`, `display server`, `remote desktop`, `SSH`, `no display`, `DISPLAY` — `render/eevee/limitations/limitations.txt` and `advanced/command_line/render.txt`; both quoted above. "Xorg" hits were inside SVG diagram files for the video sequencer and are unrelated.
- `screen saver`, `screensaver`, `lock screen`, `locked screen`, `sleep`, `idle`, `power` — **nothing at all** for the first five. `sleep` has no hits anywhere in the manual or API. `idle` hits are all displacement and tracking settings. `power` hits are maths nodes, brush falloff, and the "stable power supply" line in `render/output/animation.txt`. So: the sources say nothing about a machine sleeping, locking, or logging out under a render.
- `no user`, `without a user`, `user is present` — the `no user` hits are about data-block user counts (`bpy.types.ID`, `bpy.types.BlendData`, material assignment), a different meaning entirely; ruled out.
- `will quit`, `then exit`, `exits after`, `quit after`, `exit code` — only `--python-exit-code`, the Windows registration arguments, and the Ctrl-C tip in `info_tips_and_tricks.txt`. **Nothing states whether or when Blender quits by itself after a background render**, which is why that is in the not-covered list.
- `stdin` — one hit, the `-d` line quoted above.

**Pages opened and read in full**

`advanced/scripting/security.txt`, `advanced/command_line/arguments.txt` (all 691 lines), `advanced/command_line/render.txt`, `advanced/command_line/launch/windows.txt` and `.../macos.txt` (in part), `advanced/deploying_blender.txt`, `advanced/operators.txt`, `animation/drivers/troubleshooting.txt`, `interface/operators.txt`, `interface/window_system/splash.txt`, `interface/window_system/topbar.txt` (render and window menus), `editors/preferences/save_load.txt` (in part), `editors/preferences/interface.txt` (Temporary Editors), `files/blend/open_save.txt` (in part), `getting_started/configuration/introduction.txt`, `render/output/introduction.txt`, `render/output/animation.txt`, `render/output/properties/output.txt`, `render/cycles/gpu_rendering.txt` (FAQ and error messages), `render/eevee/limitations/limitations.txt` (in part), `troubleshooting/recover.txt`.

API: `bpy.app.txt`, `bpy.app.handlers.txt`, `bpy.ops.txt`, `bpy.ops.render.txt`, `bpy.ops.wm.txt` (the save/open/quit entries), `bpy.types.PreferencesFilePaths.txt`, `bpy.types.PreferencesView.txt`, `bpy.types.RenderSettings.txt` (the lock/overwrite/placeholder entries), `bpy.types.WindowManager.txt` (in part), `info_tips_and_tricks.txt`, `info_gotchas_threading.txt`, `info_gotchas_operators.txt`, `info_advanced_blender_as_bpy.txt` (in part).

Release notes: `index.md` folder listing, `user_interface.md`, `python_api.md`, and grep across all twenty-one files.

Cycles: `engine.py`, `operators.py`, `properties.py` (the pause entries), `ui.py` (the pause draw line), `__init__.py` (the view_update comment), plus a term-by-term search of every `.py` file in the folder.

Pictures: `manual_5.2/_images/advanced_scripting_security_autorun-scripts-dialog.png`, opened and described at (1).

**Ruled out and why**

- `render/cycles/gpu_rendering.txt`, "Why is Blender unresponsive during rendering?" — this is about the graphics card being unable to draw the interface and render at the same time. It is not a wait for a person, so it does not answer this question.
- `troubleshooting/recover.txt` and the Auto Save pages — about recovering after a crash, not about waiting.
- `interface/window_system/splash.txt` — the splash screen is dismissed by a click, but the manual never says it blocks anything, and nothing connects it to a render. Left out on purpose.
- The requirements page — searched for `5.2`, `background`, `headless`, `unattended`, `dialog`, `prompt`. No hits. It never names 5.2 and so cannot speak for it.

---

## Things only a running Blender can settle

**Pruned 2026-08-06 after the hand tests, again 2026-08-07 after the card tests, and again 2026-08-07 after the Mac sitting.** Everything a terminal could reach was run on the rented Linux machine; everything needing a window was then run on the Creator's own Mac. **All of it is answered in (10), (11) and (12), and those items are gone from this list.** The full list, with every command written out, is `Audits/Blender Research/The machine/Hand tests — added questions.md`.

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
